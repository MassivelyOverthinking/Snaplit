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
    free: VecDeque<PyObject>,
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
