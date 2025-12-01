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
}
