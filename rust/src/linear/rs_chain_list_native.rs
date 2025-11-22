use std::collections::VecDeque;

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::PyObject;
use pyo3::types::PyList;

/// ---------------------------------------------------------------------------------
/// Implementation of ChainLink helper class & Slot Enum
/// ---------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Slot {
    Empty,
    Occupied(ChainLink),
}

#[derive(Debug, Clone)]
struct ChainLink {
    data: PyObject,
    next: usize,
    previous: usize
}

/// ---------------------------------------------------------------------------------
/// Implementation of main ChainList-class -> Array-based Linked List 
/// ---------------------------------------------------------------------------------

#[pyclass]
pub struct ChainList {
    capacity: usize,
    list_size: usize,
    head: usize,
    tail: usize,
    list_array: Vec<Slot>,
    free_list: VecDeque<usize>,
}

impl ChainList {
    fn is_full(&self) -> bool {
        // Internal helper-method to determine if the current ChainList is at capacity.
        self.list_size >= self.capacity
    }
}

#[pymethods]
impl ChainList {
    #[new]
    pub fn new(capacity: Option<usize>) -> Self {
        let cap = capacity.unwrap_or(1024);
        Self {
            capacity: cap,
            list_size: 0,
            head: 0,
            tail: 0,
            list_array: vec![Slot::Empty; cap],
            free_list: VecDeque::new(),
        }
    }

    pub fn insert(&mut self, py: Python, value: PyObject, index: Option<usize>) -> PyResult<bool> {
        // Insert Python data into the ChainList at specified index.
        if self.is_full() {
            return Err(PyValueError::new_err(format!("ChainList at maximum capacity {}! Value {} not inserted", self.capacity, value)));
        }

        // Retrieve the self.head index.
        let mut head_idx = self.head - 1;

        // Check if an unused Slot exists in free_list -> If True, use that Slot instead.
        if !self.free_list.is_empty() {
            head_idx = self.free_list.pop_back();
        }
    }

    pub fn capacity(&self) -> PyResult<usize> {
        // Return the current maximum capacity of the internal Rust Vectors.
        Ok(self.capacity)
    }

    pub fn size(&self) -> PyResult<usize> {
        // Return the current size of the internal Rust Vectors.
        Ok(self.list_size)
    }

    pub fn percentage(&self) -> PyResult<f64> {
        // Returns a floating-point number indicating the percentage of internal space occupied. 
        let percent = (self.list_size as f64 / self.capacity as f64) * 100.0;
        Ok(percent)
    }

    pub fn is_empty(&self) -> PyResult<bool> {
        // Check whether the current ChainList array is currenly empty.
        Ok(self.list_size <= 0)
    }

    pub fn clear(&mut self) -> PyResult<()> {
        // Clear all internal class variables -> Resetting the entire data structure.
        self.list_size = 0;
        self.head = 0;
        self.tail = 0;
        self.list_array = vec![Slot::Empty; self.capacity];
        self.free_list = Vec::new();
        Ok(())
    }
}


