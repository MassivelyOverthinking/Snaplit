use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::PyObject;

struct LeafNode {
    value: PyObject,
    left: Option<Box<LeafNode>>,
    right: Option<Box<LeafNode>>,
    count: usize,
}

impl LeafNode {
    fn new(value: PyObject) -> Self {
        Self {
            value: value,
            left: None,
            right: None,
            count: 1
        }
    }
}

#[pyclass]
pub struct BinarySearchTree {
    root: Option<LeafNode>,
    size: usize,
    allow_duplicates: bool,
}

#[pymethods]
impl BinarySearchTree {
    #[new]
    pub fn new(allow_duplicates: bool) -> Self {
        Self {
            root: None,
            size: 0,
            allow_duplicates: allow_duplicates,
        }
    }
}