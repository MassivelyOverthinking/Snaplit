use pyo3::prelude::*;
use pyo3::PyObject;

#[pyclass]
pub struct Dequeue {
    head: usize,
    tail: usize,
    count: usize,
    total: usize,
    array: Vec<PyObject>,
}

#[pymethods]
impl Dequeue {
    #[new]
    pub fn new(py: Python, size: usize) -> Self {
        let none_obj = py.None().into_py(py);

        Self {
            head: 0,
            tail: 0,
            count: 0,
            total: size,
            array: vec![none_obj; size],
        }
    }

    pub fn enqueue(&mut self) {
        
    }

    pub fn dequeue(&mut self) {

    }
}