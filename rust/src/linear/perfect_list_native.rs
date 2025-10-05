use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::PyObject;
use pyo3::types::PyList;

#[pyclass]
struct WagonNode {
    data: PyObject,
    next: Option<Box<WagonNode>>,
    previous: Option<Box<WagonNode>>,
}

#[pyclass]
pub struct PerfectList {
    head: Option<Box<WagonNode>>,
    tail: Option<Box<WagonNode>>,
    count: usize,
}

#[pymethods]
impl PerfectList {
    #[new]
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            count: 0 
        }
    }

    pub fn prepend(&mut self, value: PyObject) {
        let new_node = Box::new(WagonNode {
            data: value,
            next: self.head.take(),
            previous: self.tail.take()
        });

        self.head = Some(new_node);
        self.count += 1;
    }

    pub fn append(&mut self, value: PyObject) {
        let new_node = Box::new(WagonNode {
            data: value,
            next: self.head.take(),
            previous: self.tail.take(),
        });

        self.tail = Some(new_node);
        self.count += 1;
    }

    pub fn remove_head(&mut self) -> Option<PyObject> {
        if let Some(mut node) = self.head.take() {
            self.head = node.next.take();
            self.tail = node.previous.take();
            self.count -= 1;
            Some(node.data)
        } else {
            None
        }
    }

    pub fn insert(&mut self, value: PyObject, index: Option<usize>) -> PyResult<()> {
        
    }

    pub fn get(&mut self, py: Python, index: usize) -> PyRef<PyObject> {
        
    }

    pub fn contains(&mut self, py: Python, value: PyObject) -> bool {
        
    }

    pub fn pop(&mut self, index: Option<usize>) -> PyResult<PyObject> {
        
    }

    pub fn remove(&mut self, index: usize) -> Option<PyObject> {
        
    }

    pub fn search(&self, py: Python, value: PyObject) -> Option<usize> {
        
    }

    pub fn update(&mut self, value: PyObject, index: usize) -> PyResult<()> {
        
    }

    pub fn to_list<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.tail = None;
        self.count = 0;
    }

    pub fn __len__(&self) -> usize {
        self.count
    }

    pub fn __getitem__(&self, py: Python, index: usize) -> PyResult<PyObject> {
        self.get(py, index)
    }

    pub fn __setitem__(&mut self, value: PyObject, index: usize) -> PyResult<()> {
        self.update(value, index)
    }

    pub fn __delitem__(&mut self, index: usize) -> PyResult<()> {
        match self.remove(index) {
            Some(_) => Ok(()),
            None => Err(PyValueError::new_err("Index out of bounds")),
        }
    }

    pub fn __contains__(&self, py: Python, value: PyObject) -> bool {
        self.contains(py, value)
    }
}