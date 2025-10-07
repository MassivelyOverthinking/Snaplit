use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::PyObject;


#[derive(Clone, Copy)]
pub enum HeapType {
    Min,
    Max,
}

struct  HeapItem {
    priority: i32,
    value: PyObject,
}

#[pyclass]
pub struct PriorityQueue {
    count: i32,
    priority: HeapType,
    array: Vec<HeapItem>,
}

#[pymethods]
impl PriorityQueue {
    #[new]
    pub fn new(priority_type: &str) -> PyResult<Self> {
        let heap_type = match priority_type.to_lowercase().as_str() {
            "min" => HeapType::Min,
            "max" => HeapType::Max,
            _ => return Err(PyValueError::new_err("Priority Type must be 'min' or 'max'")),
        };


        Ok(Self {
            count: 0,
            priority: heap_type,
            array: Vec::new(),
        })
    }

    pub fn heapify_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent_idx = (index - 1) / 2;

            let should_swap = match self.priority {
                HeapType::Min => self.array[index].priority < self.array[parent_idx].priority,
                HeapType::Max => self.array[index].priority > self.array[parent_idx].priority,
            };

            if should_swap {
                self.array.swap(index, parent_idx);
                index = parent_idx;
            } else {
                break;
            }
        }
    }

    pub fn heapify_down(&mut self) {
        
    }
}