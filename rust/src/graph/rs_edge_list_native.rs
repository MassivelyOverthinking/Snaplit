use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyList, PyTuple};
use pyo3::PyObject;
use std::collections::VecDeque;

/// ---------------------------------------------------------------------------------
/// Implementation of EdgeNode structure/class & related operations
/// ---------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Slot {
    Empty,
    Occupied(EdgeNode),
}

#[derive(Debug, Clone)]
struct EdgeNode {
    data: PyObject,
    index: usize,
    edge_count: usize,
}

impl EdgeNode {
    fn new(data: PyObject, index: usize) -> Self {
        Self {
            data: data,
            index: index,
            edge_count: 0,
        }
    }
}

/// ---------------------------------------------------------------------------------
/// Implementation of EdgeList structure/class & related operations
/// ---------------------------------------------------------------------------------

#[pyclass]
pub struct EdgeList {
    capacity: usize,
    nodes: Vec<Slot>,
    vertices: Vec<(usize, usize, f64)>,
    free_list: VecDeque<usize>,
    next: usize,
    size: usize,
}

impl EdgeList {
    fn is_full(&self) -> bool {
        return self.size >= self.capacity;
    }

    fn sort_by_weight(&mut self) {
        self.vertices.sort_by(|x, y| x.2.partial_cmp(&y.2).unwrap());
    }
}

#[pymethods]
impl EdgeList {
    #[new]
    pub fn new(capacity: Option<usize>) -> Self {
        let cap = capacity.unwrap_or(1024);
        Self {
            capacity: cap,
            nodes: vec![Slot::Empty; cap],
            vertices: Vec::new(),
            free_list: VecDeque::new(),
            next: 0,
            size: 0
        }
    }

    pub fn insert(&mut self, item: PyObject) -> PyResult<bool> {
        // Raise ValueError if Edge List is currently full.
        if self.is_full() {
            return Err(PyValueError::new_err(
                format!("Edge list at maximum capacity: {}! Unable to add value", self.capacity)
            ));
        }

        // Get the next available index.
        let index = self.free_list.pop_back().unwrap_or(self.next);

        // Instantiate new EdgeNode-class.
        let new_node = EdgeNode::new(item, index);

        // Match stmt to determine value at index.
        match &mut self.nodes[index] {
            // If Slot::Occupied -> Return False as the space is already taken.
            Slot::Occupied(_) => {
                return Ok(false);
            },
            // If Slot::Empty -> Add node-instance to available Slot & Increment size.
            Slot::Empty => {
                self.nodes[index] = Slot::Occupied(new_node);
                self.size += 1;
                self.next += 1;
                return Ok(true);
            }
        }
    }

    pub fn get(&self, py: Python, index: usize) -> PyResult<PyObject> {
        // Raise ValueError if specified Index is out of bounds.
        let size = self.size;
        if index > size {
            return Err(PyValueError::new_err(
                format!("Index out of bounds! Edge List currently contains {} entries", size)
            ));
        }

        // Match stmt to correctly handle internal EdgeNode values.
        match &self.nodes[index] {
            // If Slot::Occupied -> Clone & return data.
            Slot::Occupied(node) => {
                return Ok(node.data.clone_ref(py));
            },
            // If Slot::Empty -> Raise ValueError to indicate issue retrieving data.
            Slot::Empty => {
                return Err(PyValueError::new_err(
                    format!("Retrieval Error! No value found at index: {}", index)
                ));
            }
        }
    }

    pub fn contains(&self, py: Python, item: PyObject) -> PyResult<bool> {
        // Iterate through internal array list.
        for index in 0..=self.size {
            // Match stmt to retrieve internal EdgeNode-instance.
            match &self.nodes[index] {
                // If Slot::Occupied -> Compare item with internal value & Return bool accordingly.
                Slot::Occupied(node) => {
                    if node.data.as_ref(py).eq(item.as_ref(py))? {
                        return Ok(true);
                    }
                },
                // If Slot::Empty -> Continue to next loop iteration.
                Slot::Empty => {
                    continue;
                }
            }
        }
        // Default = Value was not found so return False. 
        Ok(false)
    }

    pub fn nodes<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        // Instantialize a new Rust Vectors.
        let mut elements = Vec::new();

        // Iterate through available slots in nodes-array.
        for index in 0..=self.size {
            // Match stmt to retreive internal values.
            match &self.nodes[index] {
                // If Slot::Occupied -> Add the data to elements list.
                Slot::Occupied(node) => {
                    elements.push(node.data.clone_ref(py));
                },
                // If Slot::Empty -> Continue to next loop iteration.
                Slot::Empty => {
                    continue;
                }
            }
        }
        // Convert & return the final PyList.
        Ok(PyList::new(py, elements))
    }

    pub fn edges<'py>(&mut self, py: Python<'py>, sort: Option<bool>) -> PyResult<&'py PyList> {
        // Instantialize a new Rust Vectors.
        let mut elements: Vec<PyObject> = Vec::new();

        // Unwrap sort option -> Use it to sort list.
        let sort = sort.unwrap_or(false);

        // Iterate through available slots in nodes-array.
        for vertex in &self.vertices {
            // Extract necessary variables to construct Tuple.
            let from_node = vertex.0;
            let to_node = vertex.1;
            let weight = vertex.2;

            // Add element to internal Rust Vector.
            elements.push((from_node, to_node, weight).into_py(py));
        }

        // Sort value by weight if specified.
        if sort {
            self.sort_by_weight();
        }

        // Convert & return finalized PyList-instance. 
        Ok(PyList::new(py, elements))
    }

    pub fn capacity(&self) -> PyResult<usize> {
        // Return the current maximum capacity of the internal Rust Vectors.
        Ok(self.capacity)
    }

    pub fn size(&self) -> PyResult<usize> {
        // Return the current node size of EdgeList instance.
        Ok(self.size)
    }

    pub fn edge_count(&self) -> PyResult<usize> {
        // Return the current number of edges in EdgeList instance.
        Ok(self.vertices.len())
    }

    pub fn is_emtpy(&self) -> PyResult<bool> {
        // check if EdgeList currently contains no Node-instances.
        Ok(self.size == 0)
    }

    pub fn clear(&mut self) -> PyResult<()> {
        // Clear all internal attributes -> Resetting the entire data structure.
        self.nodes = vec![Slot::Empty; self.capacity];
        self.vertices = Vec::new();
        self.size = 0;
        Ok(())
    }
}