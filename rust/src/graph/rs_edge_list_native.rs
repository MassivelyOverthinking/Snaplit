use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyList, PyTuple};
use pyo3::PyObject;
use rustc_hash::{FxHashMap, FxHasher};
use core::net;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// ---------------------------------------------------------------------------------
/// Implementation of EdgeNode structure/class & related operations
/// ---------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Slot {
    Empty,
    Occupied(EdgeNode),
}

#[derive(Debug, Clone)]
struct EdgeNode {
    data: PyObject,
    index: usize,
    edge_count: usize,
}

impl EdgeNode {
    fn new(data: PyObject, index: usize) -> Self {
        Self {
            data: data,
            index: index,
            edge_count: 0,
        }
    }
}

/// ---------------------------------------------------------------------------------
/// Implementation of EdgeList structure/class & related operations
/// ---------------------------------------------------------------------------------

#[pyclass]
pub struct EdgeList {
    capacity: usize,
    nodes: Vec<Slot>,
    vertices: Vec<(usize, usize, f64)>,
    size: usize,
}

#[pymethods]
impl EdgeList {
    #[new]
    pub fn new(capacity: Option<usize>) -> Self {
        let cap = capacity.unwrap_or(1024);
        Self {
            capacity: cap,
            nodes: vec![Slot::Empty; cap],
            vertices: Vec::new(),
            size: 0
        }
    }

    pub fn capacity(&self) -> PyResult<usize> {
        // Return the current maximum capacity of the internal Rust Vectors.
        Ok(self.capacity)
    }

    pub fn size(&self) -> PyResult<usize> {
        // Return the current node size of EdgeList instance.
        Ok(self.size)
    }

    pub fn edge_count(&self) -> PyResult<usize> {
        // Return the current number of edges in EdgeList instance.
        Ok(self.vertices.len())
    }

    pub fn clear(&mut self) -> PyResult<()> {
        // Clear all internal attributes -> Resetting the entire data structure.
        self.nodes = vec![Slot::Empty; self.capacity];
        self.vertices = Vec::new();
        self.size = 0;
        Ok(())
    }
}