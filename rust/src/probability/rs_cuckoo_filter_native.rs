use pyo3::prelude::*;
use pyo3::PyObject;

struct CuckooBucket {
    entries: Vec<Option<u16>>,
}

impl CuckooBucket {
    fn new(bucket_size: usize) -> Self {
        Self {
            entries: vec![None; bucket_size],
        }
    }

    fn insert(&mut self, fingerprint: u16) -> bool {
        for slot in self.entries.iter_mut() {
            if slot.is_none() {
                *slot = Some(fingerprint);
                return true;
            }
        }
        false
    }

    fn delete(&mut self, fingerprint: u16) -> bool {
        for slot in self.entries.iter_mut() {
            if let Some(value) = slot {
                if *value == fingerprint {
                    *slot = None;
                    return true;
                }
            }
        }
        false
    }

    fn contains(&self, fingerprint: u16) -> bool {
        let result = self.entries.contains(&Some(fingerprint));
        result
    }

    fn swap(&mut self, fingerprint: u16) -> bool {
        
    }

    fn is_full(&self) -> bool {
        if self.entries.iter().all(|x| x.is_none()) {
            return false;
        } else {
            return true;
        }
    }
}

#[pyclass]
pub struct CuckooFilter {
    buckets: Vec<CuckooBucket>,
    size: usize,
    fingerprint_size: usize,
    bucket_count: usize,
    retries: usize
}

#[pymethods]
impl CuckooFilter {
    #[new]
    pub fn new()
}