use std::collections::{HashMap, HashSet};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::PyObject;

struct GraphNode {
    id: usize,
    payload: PyObject,
    neighbours: HashSet<usize>,
}

#[allow(dead_code)]
impl GraphNode {
    fn new(id: usize, payload: PyObject) -> Self {
        Self {
            id: id,
            payload: payload,
            neighbours: HashSet::new(),
        }
    }
}

#[pyclass]
pub struct BaseGraph {
    node: HashMap<usize, GraphNode>,
    next_id: usize,
}

#[pymethods]
impl BaseGraph {
    #[new]
    pub fn new() -> Self {
        Self {
            node: HashMap::new(),
            next_id: 1,
        }
    }
}