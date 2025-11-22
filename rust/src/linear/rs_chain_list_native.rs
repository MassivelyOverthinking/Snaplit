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
    free_list: Vec<usize>,
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
            free_list: Vec::new(),
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

    pub fn clear(&mut self) -> PyResult<()> {
        // Clear all internal class variables -> Resetting the entire data structure.
        self.list_size = 0;
        self.head = 0;
        self.tail = 0;
        self.list_array = vec![Slot::Empty; self.capacity];
        Ok(())
    }
}


