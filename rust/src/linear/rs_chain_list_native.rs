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
    previous: usize,
    index: usize,
}

impl ChainLink {
    fn new(data: PyObject, next: usize, previous: usize, index: usize) -> Self {
        Self {
            data: data,
            next: next,
            previous: previous,
            index: index,
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

    pub fn prepend(&mut self, value: PyObject) -> PyResult<bool> {
        // Check if the internal ChainList array is full -> Raise Error if True.
        if self.is_full() {
            return Err(PyValueError::new_err(format!("ChainList at maximum capacity: {}! Unable to add value.", self.capacity)));
        }

        // Take the necessary values to create new ChainLink instance.
        let next_index: usize = self.head;
        let previous_index: usize = self.tail;

        // Check is there is currently a free Slot available in free_list.
        if let Some(free_index) = self.free_list.pop_back() {
            // Create new ChainLink-class
            let chain_value = ChainLink::new(
                value,
                next_index,
                previous_index,
                free_index,
            );

            self.list_array[free_index] = Slot::Occupied(chain_value);
            self.list_size += 1;
            self.head = free_index;
            return Ok(true);
        } else {
            // If no free Slot -> Simply add at next available array index.
            let new_idx = self.next_index;

            // Create new ChainLink-class
            let chain_value = ChainLink::new(
                value,
                next_index,
                previous_index,
                new_idx
            );

            self.list_array[new_idx] = Slot::Occupied(chain_value);
            self.next_index += 1;
            self.list_size += 1;
            self.head = new_idx;
            return Ok(true);
        }
    }

    pub fn append(&mut self, value: PyObject) -> PyResult<bool> {
        // Check if the internal ChainList array is full -> Raise Error if True.
        if self.is_full() {
            return Err(PyValueError::new_err(format!("ChainList at maximum capacity: {}! Unable to add value.", self.capacity)));
        }

        // Take the necessary values to create new ChainLink instance.
        let next_index: usize = self.tail;
        let previous_index: usize = self.head;

        // Check is there is currently a free Slot available in free_list.
        if let Some(free_index) = self.free_list.pop_back() {

            // Create new ChainLink-class
            let chain_value = ChainLink::new(
                value,
                next_index,
                previous_index,
                free_index,
            );

            self.list_array[free_index] = Slot::Occupied(chain_value);
            self.list_size += 1;
            self.tail = free_index;
            return Ok(true);
        } else {
            // If no free Slot -> Simply add at next available array index.
            let new_idx = self.next_index;

            // Create new ChainLink-class
            let chain_value = ChainLink::new(
                value,
                next_index,
                previous_index,
                new_idx,
            );

            self.list_array[new_idx] = Slot::Occupied(chain_value);
            self.next_index += 1;
            self.list_size += 1;
            self.tail = new_idx;
            return Ok(true);
        }
    }

    pub fn contains(&self, py: Python, value: PyObject) -> PyResult<bool> {
        // Iterate over internal list_array to determine if specified value is present.
        for index in 0..=self.next_index {
            // Utilise Match stmt to retrieve internal instances.
            match &self.list_array[index] {
                // If Slot::Occupied -> Check value & return if it matches!
                Slot::Occupied(link) => {
                    if link.data.as_ref(py).eq(value.as_ref(py))? {
                        return Ok(true);
                    }
                },
                // If Slot::Empty -> Continue to next loope iteration.
                Slot::Empty => {
                    continue;
                }
            }
        }
        // DEFAULT = Entire list array has been checked & no correct value was found. 
        Ok(false)
    }

    pub fn get(&self, py: Python, index: usize) -> PyResult<PyObject> {
        // Raise a ValueError if the speficied index if out of bounds.
        let list_size = self.list_size;
        if index >= list_size {
            return Err(PyValueError::new_err(
                format!("Index out of bounds! Current size is {} entries", list_size)
            ));
        }

        // Get the starting index -> Logical Linked List order.
        let mut current_idx = self.head;

        // Iterate through Logical List order -> Node by Node.
        for i in 0..=index {
            // Utilise Match stmt to retrieve internal instances.
            match &self.list_array[current_idx] {
                // If Slot::Occupied -> Check if the current index matches to extract internal value.
                Slot::Occupied(link) => {
                    if i == index {
                        return Ok(link.data.clone_ref(py));
                    } else {
                        current_idx = link.next;
                    }
                },
                // If Slot::Empty -> Raise ValueError due to unexpected traversal mistake.
                Slot::Empty => {
                    return Err(PyValueError::new_err(
                        format!("Traversal Error! Link at index {} doesn't exist", current_idx)
                    ));
                }
            }
        }
        // DEFAULT = Debugging element to ensure this code-part is never reached!
        unreachable!("Failed to correctl compute the 'Get' function.")
    }

    pub fn remove(&mut self, py: Python, index: usize) -> PyResult<PyObject> {
        // Check if the specified Index is out of bounds.
        let list_size = self.list_size;
        if index >= list_size {
            return Err(PyValueError::new_err(
                format!("Index out of bounds! Current size is {} entries", list_size)
            ));
        }

        // Retrieve 'Head' index to begin Logical order traversal.
        let mut current_idx = self.head;

        // Iterate through Logical order -> Until index is found.
        for _ in 0..index {
            // Match stmt to update current_idx value.
            match &self.list_array[current_idx] {
                // If Slot::Occupied -> Update current_idx with next index.
                Slot::Occupied(link) => {
                    current_idx = link.next;
                },
                // If Slot::Empty -> Raise ValueError as unexpected incident took place.
                Slot::Empty => {
                    return Err(PyValueError::new_err(
                        format!("Traversal Error! Link at index {} doesn't exist", current_idx)
                    ));
                }
            }
        }

        // Extract the internal values from ChainLink at final index.
        let (data, next_val, prev_val) = match &self.list_array[current_idx] {
            // If Slot::Occupied -> ChainLink exists.
            Slot::Occupied(link) => (
                link.data.clone_ref(py),
                link.next,
                link.previous,
            ),
            // If Slot::empty -> ChainLink doesn't exist - Unforseen error.
            Slot::Empty => {
                return Err(PyValueError::new_err(
                    format!("Traversal Error! Link at index {} doesn't exist", current_idx)
                ));
            }
        };

        // Update the internal 'next' variable inside previous node. 
        if let Slot::Occupied(previous) = &mut self.list_array[prev_val] {
            previous.next = next_val;
        } else {
            // Unexpected error finding ChainLink-class.
            return Err(PyValueError::new_err(
                format!("Removal Error! Issues retrieving previous index: {}", prev_val)
            ));
        }

        // Update the internal 'previous' variable inside next node. 
        if let Slot::Occupied(next) = &mut self.list_array[next_val] {
            next.previous = prev_val;
        } else {
            // Unexpected error finding ChainLink-class.
            return Err(PyValueError::new_err(
                format!("Removal Error! Issues retrieving next index: {}", next_val)
            ));
        }

        // Check if the removed node was current 'head' / 'tail'
        // If True -> Update internal values to preserve cyclical nature.
        if current_idx == self.head {
            self.head = next_val;
        }
        if current_idx == self.tail {
            self.tail = prev_val;
        }

        // Finally convert the Slot at current index to Empty.
        // Decrement list_size variable by 1 & push the final index to free_list.
        self.list_array[current_idx] = Slot::Empty;
        self.list_size -= 1;
        self.free_list.push_front(current_idx);

        // Return Python data.
        Ok(data)
    }

    pub fn search(&self, py: Python, value: PyObject) -> PyResult<Option<usize>> {
        // Iterate over internal list_array to determine if specified value is present.
        for index in 0..=self.next_index {
            // Utilise Match stmt to retrieve internal instances.
            match &self.list_array[index] {
                // If Slot::Occupied -> Check value & return link-index if it matches!
                Slot::Occupied(link) => {
                    if link.data.as_ref(py).eq(value.as_ref(py))? {
                        return Ok(Some(index));
                    }
                },
                // If Slot::Empty -> Continue to next loope iteration.
                Slot::Empty => {
                    continue;
                }
            }
        }
        // DEFAULT = Entire list array has been checked & no correct value was found.
        // Return 'None'. 
        Ok(None)
    }

    pub fn update(&mut self, index: usize, value: PyObject) -> PyResult<bool> {
        // Check if the specified Index is out of bounds.
        if index >= self.list_size {
            return Err(PyValueError::new_err(
                format!("Index out of bounds! Current size is {} entries", self.list_size)
            ));
        }

        // Initiate index value.
        let mut int_index = self.head;

        // Iterate over all values present in internal List array.
        for _ in 0..=self.list_size {
            // Use Match stmt to determine if a values exists.
            match &mut self.list_array[int_index] {
                // If Slot::Occupied -> Check if internal value matches & then update 'Data' variable.
                Slot::Occupied(link) => {
                    if int_index == index {
                        link.data = value;
                        return Ok(true);
                    } else {
                        int_index = link.next;
                    }
                },
                // If Slot::Empty -> Continue to next loop iteration.
                Slot::Empty => {
                    return Err(PyValueError::new_err(
                        format!("Traversal Error! Link at index {} doesn't exist", int_index)
                    ));
                }
            }
        }
        // DEFAULT = Didn't find the index value so return False.
        Ok(false)
    }

    pub fn to_list<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        // Initiate Rust Vectors & initial index value.
        let mut elements: Vec<PyObject> = Vec::new();
        let mut index = self.head;

        // Iterate over all values present in internal List array.
        for _ in 0..=self.list_size {
            // Use Match stmt to determine if a values exists.
            match &self.list_array[index] {
                // If Slot::Occupied -> Copy the link's internal 'Data' value & add to Vectors.
                Slot::Occupied(link) => {
                    elements.push(link.data.clone_ref(py));
                    index = link.next;
                },
                // If Slot::Empty -> Continue to next loop iteration.
                Slot::Empty => {
                    continue;
                }
            }
        }
        // Convert the Rust Vectors to Python-native list type. 
        Ok(PyList::new(py, elements))
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


