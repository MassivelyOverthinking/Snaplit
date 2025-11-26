use std::cmp::Ordering;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::PyObject;
use pyo3::types::PyList;

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