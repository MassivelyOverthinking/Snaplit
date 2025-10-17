use std::collections::hash_map::Entry;
use std::collections::HashMap;
use pyo3::exceptions::PyValueError;
use pyo3::{prelude::*, PyTypeInfo};
use pyo3::types::{PyList, PyString};
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
    words_count: usize,
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
            words_count: 0,
            size: 0
        }
    }

    pub fn insert(&mut self, py: Python, value: PyObject) -> PyResult<()> {
        let py_any = value.as_ref(py);
        if !py_any.is_instance(PyString::type_object(py))? {
            return Err(PyValueError::new_err("Trie class only supports Strings"));
        }
        
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
        self.words_count += 1;
        Ok(())
    }

    pub fn contains(&self, py: Python, value: PyObject) -> PyResult<bool> {
        let py_any = value.as_ref(py);
        if !py_any.is_instance(PyString::type_object(py))? {
            return Err(PyValueError::new_err("Trie class only supports Strings"));
        }

        let mut current_node = &self.root;
        let py_str: &str = value.extract(py)?;

        for item in py_str.chars() {
            match current_node.children.get(&item) {
                Some(node_value) => {
                    current_node = node_value;
                }
                None => return Ok(false)
            }
        }
        Ok(current_node.terminal)
    }

    pub fn base_keys(&self, py: Python) -> PyResult<Py<PyList>> {
        if self.size == 0 {
            return Err(PyValueError::new_err("no keys currently available in Trie's root node"));
        }

        let chars: Vec<String> = self.root.children.keys().map(|ch| ch.to_string()).collect();
        Ok(PyList::new(py, chars).into())
    }

    pub fn node_size(&self) -> PyResult<usize> {
        Ok(self.size)
    }

    pub fn word_size(&self) -> PyResult<usize> {
        Ok(self.words_count)
    }

    pub fn is_empty(&self) -> PyResult<bool> {
        Ok(self.size == 0)
    }

    pub fn clear(&mut self) -> PyResult<()> {
        self.root.children.clear();
        self.words_count = 0;
        self.size = 0;
        Ok(())
    }
}