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

impl ChainLink {
    fn new(data: PyObject, next: usize, previous: usize) -> Self {
        Self {
            data: data,
            next: next,
            previous: previous,
        }
    }
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
    next_index: usize,
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
            next_index: 0,
            list_array: vec![Slot::Empty; cap],
            free_list: VecDeque::new(),
        }
    }

    pub fn prepend(&mut self, py: Python, value: PyObject) -> PyResult<bool> {
        // Check if the internal ChainList array is full -> Raise Error if True.
        if self.is_full() {
            return Err(PyValueError::new_err(format!("ChainList at maximum capacity: {}! Unable to add value.", self.capacity)));
        }

        let mut next_index: usize = 0;
        let mut previous_index: usize = 0;

        let chain_value = ChainLink::new(
            value,
            next_index,
            previous_index
        );

        if let Some(free_index) = self.free_list.pop_back() {
            self.list_array[free_index] = Slot::Occupied(chain_value);
            self.list_size += 1;
            self.head = free_index;
            Ok(true)
        } else {
            let new_idx = self.next_index;
            self.list_array[new_idx] = Slot::Occupied(chain_value);
            self.list_size += 1;
            self.head = new_idx;
            Ok(true)
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
        self.next_index = 0;
        self.list_array = vec![Slot::Empty; self.capacity];
        self.free_list = VecDeque::new();
        Ok(())
    }
}


