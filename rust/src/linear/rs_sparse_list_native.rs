use std::collections::VecDeque;

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::PyObject;
use pyo3::types::{IntoPyDict, PyDict, PyList};

/// ---------------------------------------------------------------------------------
/// Implementation of Arraylist main class & general methods/operations
/// ---------------------------------------------------------------------------------

#[pyclass]
pub struct Sparselist {
    capacity: usize,
    size: usize,
    next: usize,
    threshold: f64,
    none: PyObject,
    array: Vec<PyObject>,
    free: VecDeque<usize>,
}

impl Sparselist {
    fn restructure(&mut self, py: Python) {
        let new_capacity = self.capacity * 2;
        let mut elements = vec![self.none.clone(); new_capacity];

        for index in 0..(self.size + self.free.len()) {
            let item = self.array[index].clone_ref(py);
            if item.as_ref(py).eq(self.none.as_ref(py)).expect("Failed to comapre!") {
                elements[index] = item;
            }
        }

        self.array = elements;
        self.capacity = new_capacity;
    }
}

#[pymethods]
impl Sparselist {
    #[new]
    pub fn new(py: Python, capacity: Option<usize>, threshold: Option<f64>) -> Self {
        let cap = capacity.unwrap_or(128);
        let thresh = threshold.unwrap_or(80.0);
        let none = py.None();
        Self {
            capacity: cap,
            size: 0,
            next: 0,
            threshold: thresh,
            none: none.clone(),
            array: vec![none; cap],
            free: VecDeque::new(),
        }
    }

    pub fn add(&mut self, py: Python, value: PyObject) -> PyResult<bool> {
        // Grow the internal Rust Vector if the load factor is too high.
        if self.percentage()? >= self.threshold {
            self.restructure(py);
        }

        // Check if there is an available index & insert if True.
        if let Some(index) = self.free.pop_back() {
            self.array[index] = value;
            self.size += 1;
            return Ok(true);
        } else {
            // Else -> Insert in the next available index.
            self.array[self.next] = value;
            self.next += 1;
            self.size += 1;
            return Ok(true);
        }
    }

    pub fn contains(&self, py: Python, value: PyObject) -> PyResult<bool> {
        // Iterate over internal array to extract values.
        for index in 0..(self.size + self.free.len()) {
            // Extract internal value at specified index.
            let item = self.array[index].clone_ref(py);
            // If Item is equal to specified value -> Return True.
            if item.as_ref(py).eq(value.as_ref(py))? {
                return Ok(true);
            }
        }
        // DEFAULT = No correct value found -> Return False.
        Ok(false)
    }

    pub fn values<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        // Initiate a new Rust Vectors to store values
        let mut elements = Vec::new();

        // Iterate over internal array & add values != None.
        for index in 0..(self.size + self.free.len()) {
            // Extract reference to the internal value.
            let item = self.array[index].clone_ref(py);
            // If the extracted value is not 'None' -> Add to 'Elements'.
            if item.as_ref(py).eq(self.none.as_ref(py))? {
                elements.push(item);
            }
        }

        // Convert & return finalized PyList.
        Ok(PyList::new(py, elements))
    }

    pub fn info<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        // Extract the necessary metrics from internal variables
        let percentage = self.percentage()?;
        let values = self.values(py)?.into();

        // Contruct a Rust Vector consisting of individual Tuples(String, Object).
        let key_vals: Vec<(&str, PyObject)> = vec![
            ("type", "SparseList".to_object(py)),
            ("capacity", self.capacity.to_object(py)),
            ("size", self.size.to_object(py)),
            ("percentage", percentage.to_object(py)),
            ("values", values),
        ];

        // Convert Vector to Python Dictionary and return value.
        let dict = key_vals.into_py_dict(py);
        Ok(dict)
    }

    pub fn capacity(&self) -> PyResult<usize> {
        // Return the current capacity of the internal array.
        Ok(self.capacity)
    }

    pub fn size(&self) -> PyResult<usize> {
        // Return the current number of entries present in internal array.
        Ok(self.size)
    }

    pub fn percentage(&self) -> PyResult<f64> {
        // Return the current percentage of the internal array that is occupied by entries.
        Ok((self.size as f64 / self.capacity as f64) * 100.0)
    }

    pub fn clear(&mut self) -> PyResult<()> {
        // Clear all internal variables & reset them to original values.
        self.size = 0;
        self.next = 0;
        self.array = vec![self.none.clone(); self.capacity];
        self.free.clear();
        Ok(())
    }
}
