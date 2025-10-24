use std::collections::VecDeque;
use pyo3::{exceptions::PyValueError, types::PyTuple};
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::PyObject;
use rustc_hash::{FxHashMap, FxHashSet};

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct HyperNode {
    NodeId: usize,
    payload: PyObject,
}

#[allow(dead_code)]
impl HyperNode {
    fn new(id: usize, payload: PyObject) -> Self {
        Self {
            NodeId: id,
            payload: payload,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct HyperEdge {
    EdgeId: String,
    vertices: FxHashSet<usize>,
}

impl HyperEdge {
    fn new(id: String) -> Self {
        Self {
            EdgeId: id, 
            vertices: FxHashSet::default(),
        }
    }
}

impl HyperGraph {
    fn node_removal(&mut self, id: usize) {
        for (_, item) in self.hyperedges.iter_mut() {
            if item.vertices.contains(&id) {
                item.vertices.remove(&id);
            }
        }
    }
}

#[pyclass]
pub struct HyperGraph {
    nodes: FxHashMap<usize, HyperNode>,
    hyperedges: FxHashMap<String, HyperEdge>,
    next_id: usize,
}

#[pymethods]
impl HyperGraph {
    #[new]
    pub fn new() -> Self {
        Self {
            nodes: FxHashMap::default(),
            hyperedges: FxHashMap::default(),
            next_id: 1,
        }
    }

    pub fn insert(&mut self, payload: PyObject) -> PyResult<bool> {
        let new_node = HyperNode::new(self.next_id, payload);

        if self.nodes.contains_key(&self.next_id) {
            return Ok(false);
        } else {
            self.nodes.insert(self.next_id, new_node);
            self.next_id += 1;
            Ok(true)
        }
    }

    pub fn remove(&mut self, py: Python, key: usize) -> PyResult<PyObject> {
        if self.nodes.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in Graph"));
        }

        match self.nodes.remove(&key) {
            Some(value) => {
                Self::node_removal(self, key);
                return Ok(value.payload.clone_ref(py));
            },
            None => return Err(PyValueError::new_err("Value not found in graph")),
        }
    }

    pub fn extract(&self, py: Python, key: usize) -> PyResult<PyObject> {
        if self.nodes.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in Graph"));
        }

        let node = self.nodes.get(&key);
        match node {
            Some(value) => return Ok(value.payload.clone_ref(py)),
            None => return Err(PyValueError::new_err("Value not found in Graph")),
        }
    }

    pub fn keys<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        if self.nodes.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in Graph"));
        }

        let mut elements = Vec::new();

        for (id,_) in self.nodes.iter() {
            elements.push(id);
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

    pub fn update(&mut self, py: Python, payload: PyObject, id: usize) -> PyResult<()> {
        if self.nodes.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in Graph"));
        }

        let new_node = match self.nodes.get_mut(&id) {
            Some(node_value) => node_value,
            None => return Err(PyValueError::new_err("Value not found in graph")),
        };

        new_node.payload = payload.clone_ref(py);
        Ok(())
    }

    pub fn add_edge(&mut self, id: String, vertices: Option<Vec<usize>>) -> PyResult<()> {
        if self.nodes.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in Graph"));
        }

        if self.hyperedges.contains_key(&id) {
            return Err(PyValueError::new_err(format!("Edge {} already exists!", id)));
        }

        let mut new_edge = HyperEdge::new(id.clone());

        if let Some(vs) = vertices {
            for node_id in vs {
                if !self.nodes.contains_key(&node_id) {
                    return Err(PyValueError::new_err(format!("Node with ID {} does not exist in Graph", node_id)));
                }
                new_edge.vertices.insert(node_id);
            }
        }

        self.hyperedges.insert(id, new_edge);

        Ok(())
    }
}