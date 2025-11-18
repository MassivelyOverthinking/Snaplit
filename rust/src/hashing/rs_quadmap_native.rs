use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyList, PyTuple};
use pyo3::PyObject;
use rustc_hash::{FxHashMap, FxHasher};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// ---------------------------------------------------------------------------------
/// Implementation Hashable Enum & Conversion of Python objects -> Rust data types
/// ---------------------------------------------------------------------------------

enum Hashable {
    Int(i64),
    Float(u64),
    Str(String),
    Bool(bool),
}

impl Hash for Hashable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Hashable::Int(i) => i.hash(state),
            Hashable::Float(f) => f.hash(state),
            Hashable::Str(s) => s.hash(state),
            Hashable::Bool(b) => b.hash(state),
        }
    }
}

/// ---------------------------------------------------------------------------------
/// Implementation of QuadMap-class & related operations
/// ---------------------------------------------------------------------------------

#[pyclass]
pub struct QuadMap {
    capacity: usize,
    map_size: usize,
}

#[pymethods]
impl QuadMap {
    #[new]
    pub fn new(capacity: Option<usize>) -> Self {
        let qm_cap = capacity.unwrap_or(1024);
        Self {
            capacity: qm_cap,
            map_size: 0
        }
    }
}