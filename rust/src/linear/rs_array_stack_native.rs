use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::PyObject;
use pyo3::types::PyList;

#[pyclass]
pub struct ArrayStack {
    capacity: usize,
    stack: Vec<PyObject>,
}

#[pymethods]
impl ArrayStack {
    #[new]
    pub fn new(size: Option<usize>) -> Self {
        Self {
            capacity: size.unwrap_or(0),
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self, value: PyObject) -> PyResult<()> {
        if self.stack.len() >= self.capacity && self.capacity != 0 {
            return Err(PyValueError::new_err("Stack is at max capacity"));
        }

        self.stack.push(value);
        Ok(())
    }

    pub fn pop(&mut self, py: Python) -> PyResult<PyObject> {
        if self.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in Stack"));
        }

        let result = self.stack.pop().unwrap().clone_ref(py);
        Ok(result)
    }

    pub fn peek(&self, py: Python) -> PyResult<PyObject> {
        if self.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in Stack"));
        }

        let result = self.stack.last().unwrap().clone_ref(py);
        Ok(result)
    }

    pub fn size(&self) -> usize {
        self.stack.len()
    }

    pub fn swap() {

    }

    pub fn contains(&self, py: Python, value: PyObject) -> bool {
        if self.is_empty() {
            return false;
        } else {
            for item in self.stack.iter() {
                if item.as_ref(py).eq(value.as_ref(py)).unwrap_or(false) {
                    return true;
                }
            }
        }
        false
    }

    pub fn copy() {

    }

    pub fn is_empty(&self) -> bool {
        return self.stack.len() == 0;
    }

    pub fn is_full(&self) -> bool {
        if self.capacity == self.stack.len() && self.capacity != 0 {
            return true;
        } else {
            return false;
        }
    }

    pub fn reverse() {

    }

    pub fn to_list() {

    }

    pub fn update_top() {

    }

    pub fn clear(&mut self) {
        self.stack.clear();
    }

}