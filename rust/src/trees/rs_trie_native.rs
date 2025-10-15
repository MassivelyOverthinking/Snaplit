use std::cmp::Ordering;
use std::collections::HashMap;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::PyObject;

struct TrieNode {
    value: Option<PyObject>,
    children: HashMap<char, Box<TrieNode>>,
    terminal: bool,
}

impl TrieNode {
    fn new(data: PyObject) -> Self {
        Self {
            value: Some(data),
            children: HashMap::new(),
            terminal: false,
        }
    }
}

#[pyclass]
pub struct Trie {
    root: TrieNode,
    size: usize,
}

impl Trie {

}

#[pymethods]
impl Trie {
    #[new]
    pub fn new(py: Python) -> Self {
        Self {
            root: TrieNode::new(py.None().into_py(py)),
            size: 0
        }
    }

    pub fn insert(&mut self, py: Python, value: PyObject) -> PyResult<()> {
        
    }
}