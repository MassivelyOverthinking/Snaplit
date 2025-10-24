use std::collections::VecDeque;
use pyo3::{exceptions::PyValueError, types::PyTuple};
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::PyObject;
use rustc_hash::{FxHashMap, FxHashSet};

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct HyperNode {
    id: usize,
    payload: PyObject,
}

#[allow(dead_code)]
impl HyperNode {
    fn new(id: usize, payload: PyObject) -> Self {
        Self {
            id: id,
            payload: payload,
        }
    }
}

#[pyclass]
pub struct HyperGraph {
    nodes: FxHashMap<usize, HyperNode>,
    hyperedges: FxHashSet<FxHashSet<usize>>,
    next_id: usize,
}

#[pymethods]
impl HyperGraph {
    #[new]
    pub fn new() -> Self {
        Self {
            nodes: FxHashMap::default(),
            hyperedges: FxHashSet::default(),
            next_id: 1,
        }
    }
}