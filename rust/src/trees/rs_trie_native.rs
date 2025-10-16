use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::PyObject;

#[derive(Clone)]
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
        
        let mut current_node = &mut self.root;
        let py_str: &str = value.extract(py)?;
        let last_index: usize = py_str.chars().count() - 1;
        
        for (index, item) in py_str.chars().enumerate() {
            current_node = match current_node.children.entry(item) {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => {
                    let mut new_node = TrieNode::new(py.None().into_py(py));
                    if index == last_index {
                        new_node.value = Some(value.clone());
                        new_node.terminal = true;
                    }
                    self.size += 1;
                    entry.insert(Box::new(new_node))
                }
            };
        }
        Ok(())
    }
}