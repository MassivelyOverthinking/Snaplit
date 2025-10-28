use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::PyObject;
use rustc_hash::FxHasher;
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
/// Implementation of Cuckoo Bucket structure/class & related operations
/// ---------------------------------------------------------------------------------
 

#[derive(Debug, Clone)]
struct CuckooBucket {
    slots: Vec<PyObject>,
}

impl CuckooBucket {
    fn new(py: Python, slot_num: usize) -> Self {
        let none_obj = py.None().into_py(py);
        Self {
            slots: vec![none_obj; slot_num],
        }
    }
}

/// ---------------------------------------------------------------------------------
/// Implementation of SnapMap structure/class & related operations
/// ---------------------------------------------------------------------------------

#[pyclass]
pub struct SnapMap {
    capacity: usize,
    bucket_size: usize,
    first_layer: Vec<CuckooBucket>,
    second_layer: Vec<CuckooBucket>,
}

impl SnapMap {
    fn generate_map_capacity(capacity: usize, size: usize) -> usize {
        return capacity / size as usize;
    }

    fn python_to_rust(py: Python, item: &PyObject) -> PyResult<Hashable> {
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

    fn generate_first_hash<T: Hash>(&self, key: &T) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash_value = hasher.finish();
        let map_capacity = self.first_layer.len();
        return (hash_value as usize) % map_capacity;
    }

    fn generate_second_hash<T: Hash>(&self, key: &T) -> usize {
        let mut hasher = FxHasher::default();
        key.hash(&mut hasher);
        let hash_value = hasher.finish();
        let map_capacity = self.second_layer.len();
        return (hash_value as usize) % map_capacity;
    }
}

#[pymethods]
impl SnapMap {
    #[new]
    pub fn new(py: Python, capacity: Option<usize>, bucket_size: Option<usize>) -> Self {
        let sm_cap = capacity.unwrap_or(1024);
        let sm_buc = bucket_size.unwrap_or(4);
        let final_size = Self::generate_map_capacity(sm_cap, sm_buc);
        Self {
            capacity: sm_cap,
            bucket_size: sm_buc,
            first_layer: vec![CuckooBucket::new(py, sm_buc); final_size],
            second_layer: vec![CuckooBucket::new(py, sm_buc); final_size],
        }
    }
}
