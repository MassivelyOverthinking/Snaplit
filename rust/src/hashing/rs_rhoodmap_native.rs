use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyList, PyTuple};
use pyo3::PyObject;
use rustc_hash::{FxHashMap, FxHasher};
use core::hash;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::hashing::rs_snapmap_native::SnapMap;

/// ---------------------------------------------------------------------------------
/// Implementation of Enum types & Conversion of Python objects -> Rust data types
/// ---------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Slot {
    Emtpy,
    Occupied(RobinBucket),
}

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
/// Implementation of Robin Bucket structure/class & related operations
/// ---------------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct RobinBucket {
    key: PyObject,
    value: PyObject,
    hash: usize,
    distance: usize,
}

impl RobinBucket {
    fn new(py: Python, key: PyObject, value: PyObject, hash: usize) -> Self {
        Self { 
            key: key,
            value: value,
            hash: hash,
            distance: 0,
        }
    }
}

/// ---------------------------------------------------------------------------------
/// Implementation of RhoodMap structure/class & related operations
/// ---------------------------------------------------------------------------------

#[pyclass]
pub struct RhoodMap {
    capacity: usize,
    map_size: usize,
    series: Vec<Slot>,
}

impl RhoodMap {
    fn python_to_rust(py: Python, item: &PyObject) -> PyResult<Hashable> {
        // Converts Python native data types -> Rust native data types
        if let Ok(i) = item.extract::<i64>(py) {
            return Ok(Hashable::Int(i));
        } else if let Ok(f) = item.extract::<f64>(py) {
            return Ok(Hashable::Float(f.to_bits()));
        } else if let Ok(s) = item.extract::<String>(py)  {
            return Ok(Hashable::Str(s));
        } else if let Ok(b) = item.extract::<bool>(py) {
            return Ok(Hashable::Bool(b));
        } else {
            return Err(PyValueError::new_err("Unsupported data type for Rust conversion"));
        }
    }

    fn generate_hash<T: Hash>(&self, key: &T) -> usize {
        // Generates the intial Hash index for Robin Hood insertion
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash_value = hasher.finish();
        let map_capacity = self.capacity;
        return (hash_value as usize) % map_capacity;
    }
}

#[pymethods]
impl RhoodMap {
    #[new]
    pub fn new(capacity: Option<usize>) -> Self {
        let rhm_cap = capacity.unwrap_or(1024);
        Self {
            capacity: rhm_cap,
            map_size: 0,
            series: vec![Slot::Emtpy; rhm_cap],
        }
    }

    pub fn insert(&mut self, py: Python, key: PyObject, value: PyObject) -> PyResult<bool> {
        let mut key = key;
        let mut value = value;

        if self.map_size >= self.capacity {
            return Err(PyValueError::new_err(format!("Maximum capacity ({}) reached! Unable to insert key-value", self.capacity)));
        }

        // Convert key to Rust data type & produce hash-value.
        let rust_hash = Self::python_to_rust(py, &key)?;
        let index = Self::generate_hash(&self, &rust_hash);

        // Generate the new Bucket to insert.
        let bucket = RobinBucket::new(py, key, value, index);

        if matches!(self.series[index], Slot::Emtpy) {
            self.series[index] = Slot::Occupied(bucket);
            self.map_size += 1;
            return Ok(true);
        }

    }
}
