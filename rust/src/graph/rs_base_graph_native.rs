use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::PyObject;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone)]
struct GraphNode {
    id: usize,
    payload: PyObject,
    neighbours: FxHashSet<usize>,
}

#[allow(dead_code)]
impl GraphNode {
    fn new(id: usize, payload: PyObject) -> Self {
        Self {
            id: id,
            payload: payload,
            neighbours: FxHashSet::default(),
        }
    }
}

#[pyclass]
pub struct BaseGraph {
    nodes: FxHashMap<usize, GraphNode>,
    next_id: usize,
    count: usize,
}

#[pymethods]
impl BaseGraph {
    #[new]
    pub fn new() -> Self {
        Self {
            nodes: FxHashMap::default(),
            next_id: 1,
            count: 0,
        }
    }

    pub fn insert(&mut self, item: PyObject) -> PyResult<bool> {
        let new_node = GraphNode::new(self.next_id, item);

        if self.nodes.contains_key(&self.next_id) {
            return Ok(false)
        } else {
            self.nodes.insert(self.next_id, new_node);
            self.next_id += 1;
            self.count += 1;
            Ok(true)
        }
    }

    pub fn remove(&mut self, py: Python, key: usize) -> PyResult<PyObject> {
        if self.nodes.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in Graph"));
        }

        match self.nodes.remove(&key) {
            Some(value) => {
                self.count -= 1;
                return Ok(value.payload.clone_ref(py))
            },
            None => return Err(PyValueError::new_err("Value not found in Graph")),
        }
    }

    pub fn extract(&mut self, py: Python, key: usize) -> PyResult<PyObject> {
        let node = self.nodes.get(&key);
        match node {
            Some(value) => return Ok(value.payload.clone_ref(py)),
            None => return Err(PyValueError::new_err("Value not found in Graph"))
        }
        
    }

    pub fn keys<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        if self.nodes.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in Graph"));
        }

        let mut elements = Vec::new();

        for item in self.nodes.keys().into_iter() {
            elements.push(item);
        }

        let final_list = PyList::new(py, elements);
        Ok(final_list.into())
    }

    pub fn contains(&self, key: usize) -> PyResult<bool> {
        if self.nodes.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in Graph"));
        }

        let result = self.nodes.contains_key(&key);
        Ok(result)
    }

    pub fn size(&self) -> PyResult<usize> {
        Ok(self.count)
    }

    pub fn clear(&mut self) -> PyResult<()> {
        if self.nodes.is_empty() {
            return Err(PyValueError::new_err("Graph is already empty"))
        }

        self.nodes.clear();
        self.next_id = 1;
        self.count = 0;
        Ok(())
    }
}