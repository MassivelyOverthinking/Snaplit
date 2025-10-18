use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::PyObject;
use crate::other::rs_bit_array::BitArray;

#[pyclass]
pub struct BloomFilter {
    probability: u64,
    size: usize,
    hash_count: usize,
    array: BitArray,
}