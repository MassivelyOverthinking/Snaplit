use std::cmp::Ordering;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::PyObject;
use pyo3::types::{IntoPyDict, PyDict, PyList};

/// ---------------------------------------------------------------------------------
/// Implementation of ChainLink helper class & Slot Enum
/// ---------------------------------------------------------------------------------


#[pyclass]
pub struct TinySet {
    capacity: usize,
    size: usize,
    array: Vec<PyObject>,
}

impl TinySet {
    fn comparison(py: Python, x: PyObject, y: PyObject) -> PyResult<Ordering> {
        let x_ref = x.as_ref(py);
        let y_ref = y.as_ref(py);

        if x_ref.lt(y_ref)? {
            Ok(Ordering::Less)
        } else if x_ref.gt(y_ref)? {
            Ok(Ordering::Greater)
        } else if x_ref.eq(y_ref)? {
            Ok(Ordering::Equal)
        } else {
            Err(PyValueError::new_err("Cannot compare Python Objects"))
        }
    }
}

#[pymethods]
impl TinySet {
    #[new]
    pub fn new(py: Python, capacity: Option<usize>) -> Self {
        let cap = capacity.unwrap_or(128);
        Self {
            capacity: cap,
            size: 0,
            array: vec![py.None(); cap],
        }
    }

    pub fn contains(&self, py: Python, target: PyObject) -> PyResult<bool> {
        // Binary Search -> Set 'left' & 'right' variables.
        let mut left: usize = 0;
        let mut right: usize = match self.size.checked_sub(1) {
            Some(r) => r,
            None => return Err(PyValueError::new_err("TinySet is currently empty! No entries to search through."))
        };

        // Continue to loop while 'left' & 'right' are distinct.
        while left <= right {
            // Get middle index.
            let mid = left + (right - left) / 2;

            // Retrieve the Object at middle index.
            let mid_obj = &self.array[mid];

            // If the mid-object equals the traget -> Return True.
            // If the mid-object is greater -> Search upper part of array.
            // If the mid-object is lesser -> Search lower part of array.
            if mid_obj.as_ref(py).eq(target.as_ref(py))? {
                return Ok(true);
            } else if mid_obj.as_ref(py).gt(target.as_ref(py))? {
                right = mid.checked_sub(1).unwrap_or(0);
            } else {
                left = mid + 1;
            }
        }
        // DEFAULT = Target not found, so return False.
        Ok(false)
    }

    pub fn update(&mut self, py: Python, target: PyObject, value: PyObject) -> PyResult<bool> {
        // Binary Search -> Set 'left' & 'right' variables.
        let mut left: usize = 0;
        let mut right: usize = match self.size.checked_sub(1) {
            Some(r) => r,
            None => return Err(PyValueError::new_err("TinySet is currently empty! No entries to search through."))
        };

        // Continue to loop while 'left' & 'right' are distinct.
        while left <= right {
            // Get middle index.
            let mid = left + (right - left) / 2;

            // Retrieve the Object at middle index.
            let mid_obj = &self.array[mid];

            // If the mid-object equals the traget -> Update value & Return True.
            // If the mid-object is greater -> Search upper part of array.
            // If the mid-object is lesser -> Search lower part of array.
            if mid_obj.as_ref(py).eq(target.as_ref(py))? {
                self.array[mid] = value;
                return Ok(true);
            } else if mid_obj.as_ref(py).gt(target.as_ref(py))? {
                right = mid.checked_sub(1).unwrap_or(0);
            } else {
                left = mid + 1;
            }
        }
        // DEFAULT = Target not found, so return False.
        Ok(false)
    }

    pub fn values<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        // Initiate new Rust Vectors to stores values.
        let mut elements = Vec::new();

        // Iterate through internal array & clone values.
        for i in 0..=self.size {
            elements.push(self.array[i].clone_ref(py));
        }

        // Convert & return the new PyList instance. 
        Ok(PyList::new(py, elements))
    }

    pub fn info<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        // Extract the necessary metrics from internal variables
        let percentage = self.percentage()?;
        let values = self.values(py)?.into();

        // Contruct a Rust Vector consisting of individual Tuples(String, Object).
        let key_vals: Vec<(&str, PyObject)> = vec![
            ("type", "TinySet".to_object(py)),
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
        // Return the maximum number of possible entries in internal array. 
        Ok(self.capacity)
    }

    pub fn size(&self) -> PyResult<usize> {
        // Return the current number of entries present in internal array.
        Ok(self.size)
    }

    pub fn percentage(&self) -> PyResult<f64> {
        // Returns a floating-point number indicating the percentage of internal space occupied.
        let percent = (self.size as f64 / self.capacity as f64) * 100.0;
        Ok(percent)
    }

    pub fn is_empty(&self) -> PyResult<bool> {
        // Return True if there currently is 0 entries in internal array.
        Ok(self.size <= 0)
    }

    pub fn clear(&mut self, py: Python) -> PyResult<()> {
        // Reset all internal variables & vectors.
        self.size = 0;
        self.array = vec![py.None(); self.capacity];
        Ok(())
    }
}