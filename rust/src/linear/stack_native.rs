use pyo3::prelude::{self, *};
use pyo3::exceptions::PyValueError;
use pyo3::PyObject;
use pyo3::types::PyList;

#[pyclass]
struct TowerNode {
    data: PyObject,
    next: Option<Box<TowerNode>>,
}

pub struct Stack {
    top: Option<Box<TowerNode>>,
    count: usize,
}

#[pymethods]
impl Stack {
    #[new]
    pub fn new() -> Self {
        Self {
            top: None,
            count: 0
        }
    }

    pub fn push(&mut self, value: PyObject) {
        let new_node = Box::new(TowerNode {
            data: value,
            next: self.top.take(),
        });

        self.top = Some(new_node);
        self.count += 1;
    }

    pub fn peek(&self, py: Python) -> Option<PyObject> {
        if let Some(node) = self.top.as_ref() {
            return Some(node.data.clone_ref(py));
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<PyObject> {
        self.top.take().map(|mut node| {
            self.top = node.next.take();
            self.count -= 1;

            return node.data;
        })
    }

    pub fn size(&self) -> usize {
        return self.count;
    }

    pub fn contains(&self, py: Python, value: PyObject) -> bool {
        let mut current_node = self.top.as_ref();

        while let Some(node) = current_node {
            if node.data.as_ref(py).eq(value.as_ref(py)).unwrap_or(false) {
                return true;
            }

            current_node = node.next.as_ref();
        }
        return false;
    }

    pub fn copy() {

    }

    pub fn is_empty(&self) -> bool {
        if self.count == 0 {
            return true;
        } else {
            return false;
        }
    }

    pub fn to_list<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        let mut elements = Vec::new();
        let mut current_node = self.top.as_ref();

        while let Some(node) = current_node  {
            elements.push(node.data.clone_ref(py));
            current_node = node.next.as_ref();
        }
        let list_bound = PyList::new(py, elements);
        Ok(list_bound)
    }

    pub fn reverse() {

    }

    pub fn clear(&mut self) {
        self.top = None;
        self.count = 0;
    }

    pub fn __len__(&self) -> usize {
        return self.size();
    }
}