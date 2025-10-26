use pyo3::prelude::*;
use pyo3::PyObject;

impl XORFilter {
    
    fn mix(&self, base: u64, salt: u64) -> u64 {
        let mut x = base ^ (self.seed.wrapping_add(salt).rotate_left(13));

        x ^= x >> 23;
        x = x.wrapping_mul(0x2127599bf4325c37);
        x ^= x >> 47;
        
        return x;
    }
}

#[pyclass]
pub struct XORFilter {
    fingerprints: Vec<u8>,
    seed: u64,
    size: usize,
}

#[pymethods]
impl XORFilter {
    #[new]
    pub fn new(seed: Option<u64>) -> Self {
        let filter_seed = seed.unwrap_or(42);
        Self {
            fingerprints: Vec::new(),
            seed: filter_seed,
            size: 0
        }
    }
}