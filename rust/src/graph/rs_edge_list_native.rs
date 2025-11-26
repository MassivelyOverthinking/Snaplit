use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyList, PyTuple};
use pyo3::PyObject;
use std::collections::VecDeque;
use std::mem::replace;

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
    id: usize,
    data: PyObject,
    index: usize,
    edge_count: usize,
}

impl EdgeNode {
    fn new(id: usize, data: PyObject, index: usize) -> Self {
        Self {
            id: id,
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
    next_id: usize,
    size: usize,
}

impl EdgeList {
    fn is_full(&self) -> bool {
        return self.size >= self.capacity;
    }

    fn sort_by_weight(&mut self) {
        self.vertices.sort_by(|x, y| x.2.partial_cmp(&y.2).unwrap());
    }

    fn remove_all_edges(&mut self, id: usize) -> bool {

        let before = self.vertices.len();

        self.vertices.retain(|(x, y)| *x != id && *y != id);

        before != self.vertices.len()
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
            next_id: 1,
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
        let new_node = EdgeNode::new(self.next_id, item, index);

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
                self.next_id += 1;

                // Do not increment next-variable if the index was aquired from free_list.
                if index == self.next {
                    self.next += 1;
                }

                return Ok(true);
            }
        }
    }

    pub fn remove(&mut self, py: Python, value: PyObject) -> PyResult<PyObject> {
        // Get the sum of internal size + free_list size to ensure full traversal. 
        let size = self.vertices.len();

        let mut removal_idx: Option<usize> = None;

        for index in 0..=size {
            match &mut self.nodes[index] {
                Slot::Occupied(node) => {
                    if value.as_ref(py).eq(node.data.as_ref(py))? {
                        removal_idx = Some(index);
                        break;
                    }
                },
                Slot::Empty => {
                    continue;
                }
            }
        }

        let idx = removal_idx.ok_or_else(|| {
            PyValueError::new_err("Value Error! Node not found in list.")
        })?;

        let removed_val = match replace(&mut self.nodes[idx], Slot::Empty) {
            Slot::Occupied(node) => node.data,
            Slot::Empty => {
                return Err(PyValueError::new_err(
                    format!("Value Error! Unable to extract internal data value from node")
                ));
            }
        };

        self.free_list.push_front(idx);
        self.size -= 1;

        self.remove_all_edges(idx);

        Ok(removed_val)
    }

    pub fn extract(&self, py: Python, id: usize) -> PyResult<PyObject> {
        // Get the sum of internal size + free_list size to ensure full traversal. 
        let size = self.size + self.free_list.len();

        // Iterate over internal nodes list. 
        for i in 0..=size {
            // Match stmt to correctly handle internal EdgeNode values.
            match &self.nodes[i] {
                // If Slot::Occupied -> Clone & return data.
                Slot::Occupied(node) => {
                    if node.id == id {
                        return Ok(node.data.clone_ref(py));
                    }
                },
                // If Slot::Empty -> Continue to next loop iteration..
                Slot::Empty => {
                    continue;
                }
            }
        }
        // DEFAULT = Node with ID not found in EdgeList.
        Ok(py.None())
    }

    pub fn contains(&self, py: Python, item: PyObject) -> PyResult<bool> {
        // Get the sum of internal size + free_list size to ensure full traversal. 
        let size = self.size + self.free_list.len();

        // Iterate through internal array list.
        for index in 0..=size {
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

    pub fn update(&mut self, id: usize, value: PyObject) -> PyResult<bool> {
        // Get the sum of internal size + free_list size to ensure full traversal. 
        let size = self.size / self.free_list.len();

        // Traverse internal nodes array to fidn correct node.
        for index in 0..=size {
            // Match stmt to extract values.
            match &mut self.nodes[index] {
                // If Slot::Occupied -> Update the internal node data variable.
                Slot::Occupied(node) => {
                    if id == node.id {
                        node.data = value;
                        return Ok(true);
                    }
                },
                // If Slot::Empty -> Continue to next loop itertion.
                Slot::Empty => {
                    continue;
                }
            }
        }
        // DEFAULT = Return false as node with correct ID was not found.
        Ok(false)
    }

    pub fn nodes<'py>(&self, py: Python<'py>, with_id: bool) -> PyResult<&'py PyList> {
        // Instantialize a new Rust Vectors.
        let mut elements = Vec::new();

        // Get the sum of internal size + free_list size to ensure full traversal. 
        let size = self.size + self.free_list.len();

        // Iterate through available slots in nodes-array.
        for index in 0..=size {
            // Match stmt to retreive internal values.
            match &self.nodes[index] {
                // If Slot::Occupied -> Add the data to elements list.
                Slot::Occupied(node) => {
                    // If 'with_id' is True -> Return a Tuple of data & ID.
                    if with_id {
                        let result = (node.data.clone_ref(py), node.id).into_py(py);
                        elements.push(result);
                    }
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
        self.next_id = 0;
        Ok(())
    }
}