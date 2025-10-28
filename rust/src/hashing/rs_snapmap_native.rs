use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
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
/// Implementation of Cuckoo Bucket structure/class & related operations
/// ---------------------------------------------------------------------------------
 

#[derive(Debug, Clone)]
struct CuckooBucket {
    capacity: usize,
    slots: Vec<(PyObject, PyObject)>,
    index: FxHashMap<u64, usize>,
}

impl CuckooBucket {
    fn new(slot_num: usize) -> Self {
        Self {
            capacity: slot_num,
            slots: Vec::new(),
            index: FxHashMap::default(),
        }
    }

    fn is_full(&self) -> bool {
        if self.slots.len() >= self.capacity {
            true
        } else {
            false
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

    const MAX_EVICTIONS: usize = 100;

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
    pub fn new(capacity: Option<usize>, bucket_size: Option<usize>) -> Self {
        let sm_cap = capacity.unwrap_or(1024);
        let sm_buc = bucket_size.unwrap_or(4);
        let final_size = Self::generate_map_capacity(sm_cap, sm_buc);
        Self {
            capacity: sm_cap,
            bucket_size: sm_buc,
            first_layer: vec![CuckooBucket::new(sm_buc); final_size],
            second_layer: vec![CuckooBucket::new(sm_buc); final_size],
        }
    }

    pub fn insert(&mut self, py: Python, key: PyObject, value: PyObject) -> PyResult<bool> {
        let mut key = key;
        let mut value = value;

        // Try inserting key-value pair in Map-structure (100 attempts)
        for _ in 0..Self::MAX_EVICTIONS {

            // Convert key to Rust data type & produce 2 hash-values
            let rust_hash = SnapMap::python_to_rust(py, &key)?;

            let idx1 = SnapMap::generate_first_hash(&self, &rust_hash);
            let idx2 = SnapMap::generate_second_hash(&self, &rust_hash);

            // Compute a new hash value for indexing in CuckooBucket
            let mut h = DefaultHasher::new();
            rust_hash.hash(&mut h);
            let idx_value = h.finish();

            // Extract mutable references to the 2 Buckets
            let first_bucket = &mut self.first_layer[idx1];
            let second_bucket = &mut self.second_layer[idx2];

            // Attempt to insert key-value pair in first layer
            if !first_bucket.is_full() {
                first_bucket.slots.push((key.clone_ref(py), value.clone_ref(py)));
                let position = first_bucket.slots.len() - 1;
                first_bucket.index.insert(idx_value, position);
                return Ok(true);
            } 

            // Attempt to insert key-value pair in second layer
            if !second_bucket.is_full() {
                second_bucket.slots.push((key.clone_ref(py), value.clone_ref(py)));
                let position = second_bucket.slots.len() - 1;
                second_bucket.index.insert(idx_value, position);
                return Ok(true);
            }

            // If both insertions fail - Push out oldest key-value pair and forcibly insert new pair.
            let evicted_pair = first_bucket.slots.pop().expect("Slot should be full!");
            first_bucket.slots.push((key, value));
            let position = first_bucket.slots.len() - 1;
            first_bucket.index.insert(idx_value, position);

            // Reassign the eviced key and value to retry
            key = evicted_pair.0;
            value = evicted_pair.1;
        }

        // If all 100 insertion attempts fail return false (**Rehash**)
        Ok(false)
    }

    pub fn contains(&self, py: Python, key: PyObject) -> PyResult<bool> {
        // Convert key to Rust data type & produce 2 hash-values
        let rust_hash = SnapMap::python_to_rust(py, &key)?;

        let idx1 = SnapMap::generate_first_hash(&self, &rust_hash);
        let idx2 = SnapMap::generate_second_hash(&self, &rust_hash);

        // Compute a new hash value for indexing in CuckooBucket
        let mut h = DefaultHasher::new();
        rust_hash.hash(&mut h);
        let idx_value = h.finish();

        // Check if value exists in first layer
        if self.first_layer[idx1].index.contains_key(&idx_value) {
            return Ok(true);
        }

        // Check if value exists in second layer
        if self.second_layer[idx2].index.contains_key(&idx_value) {
            return Ok(true);
        }

        // If key doesn't exist in both layers return false to user.
        return Ok(false);
    }
}
