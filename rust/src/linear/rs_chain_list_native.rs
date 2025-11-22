use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::PyObject;
use pyo3::types::PyList;

/// ---------------------------------------------------------------------------------
/// Implementation of ChainLink helper class -> Operates like internal List Node
/// ---------------------------------------------------------------------------------

struct ChainLink {
    data: PyObject,
    next: usize,
    previous: usize
}

/// ---------------------------------------------------------------------------------
/// Implementation of main ChainList-class -> Array-based Linked List 
/// ---------------------------------------------------------------------------------

#[pyclass]
pub struct ChainList {
    capacity: usize,
    list_size: usize,
    head: usize,
    tail: usize,
    list_array: Vec<PyObject>,
}

#[pymethods]
impl ChainList {
    #[new]
    pub fn new(py: Python, capacity: Option<usize>) -> Self {
        let cap = capacity.unwrap_or(1024);
        let py_none = py.None();
        Self {
            capacity: cap,
            list_size: 0,
            head: 0,
            tail: 0,
            list_array: vec![py_none; cap],
        }
    }
}


