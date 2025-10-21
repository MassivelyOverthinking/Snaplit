use std::collections::VecDeque;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::PyObject;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone)]
struct GraphNode {
    id: usize,
    payload: PyObject,
    neighbours: FxHashSet<usize>,
}

#[allow(dead_code)]
impl GraphNode {
    fn new(id: usize, payload: PyObject) -> Self {
        Self {
            id: id,
            payload: payload,
            neighbours: FxHashSet::default(),
        }
    }
}

#[pyclass]
pub struct BaseGraph {
    nodes: FxHashMap<usize, GraphNode>,
    next_id: usize,
    count: usize,
}

impl BaseGraph {
    fn removal(&mut self, id: usize) {
        for (_, item) in self.nodes.iter_mut() {
            if item.neighbours.contains(&id) {
                item.neighbours.remove(&id);
            }
        }
    }
}

#[pymethods]
impl BaseGraph {
    #[new]
    pub fn new() -> Self {
        Self {
            nodes: FxHashMap::default(),
            next_id: 1,
            count: 0,
        }
    }

    pub fn insert(&mut self, item: PyObject) -> PyResult<bool> {
        let new_node = GraphNode::new(self.next_id, item);

        if self.nodes.contains_key(&self.next_id) {
            return Ok(false)
        } else {
            self.nodes.insert(self.next_id, new_node);
            self.next_id += 1;
            self.count += 1;
            Ok(true)
        }
    }

    pub fn remove(&mut self, py: Python, key: usize) -> PyResult<PyObject> {
        if self.nodes.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in Graph"));
        }

        match self.nodes.remove(&key) {
            Some(value) => {
                self.count -= 1;
                Self::removal(self, key);
                return Ok(value.payload.clone_ref(py))
            },
            None => return Err(PyValueError::new_err("Value not found in Graph")),
        }
    }

    pub fn extract(&mut self, py: Python, key: usize) -> PyResult<PyObject> {
        let node = self.nodes.get(&key);
        match node {
            Some(value) => return Ok(value.payload.clone_ref(py)),
            None => return Err(PyValueError::new_err("Value not found in Graph"))
        }
        
    }

    pub fn keys<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        if self.nodes.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in Graph"));
        }

        let mut elements = Vec::new();

        for item in self.nodes.keys().into_iter() {
            elements.push(item);
        }

        let final_list = PyList::new(py, elements);
        Ok(final_list.into())
    }

    pub fn contains(&self, key: usize) -> PyResult<bool> {
        if self.nodes.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in Graph"));
        }

        let result = self.nodes.contains_key(&key);
        Ok(result)
    }

    pub fn update(&mut self, py: Python, item: PyObject, index: usize) -> PyResult<()> {
        if self.nodes.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in Graph"));
        }

        let new_node = match self.nodes.get_mut(&index) {
            Some(node_value) => node_value,
            None => return Err(PyValueError::new_err("Index not found in present Graph"))
        };

        new_node.payload = item.clone_ref(py);
        Ok(())
    }

    pub fn add_edge(&mut self, x: usize, y: usize) -> PyResult<()> {
        if self.nodes.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in Graph"));
        }

        if !self.nodes.contains_key(&x) {
            return Err(PyValueError::new_err("X node not found in current Graph"));
        }

        if !self.nodes.contains_key(&y) {
            return Err(PyValueError::new_err("Y node not found in current Graph"));
        }
        
        let x_node = self.nodes.get_mut(&x).unwrap();
        x_node.neighbours.insert(y);
        
        let y_node = self.nodes.get_mut(&y).unwrap();
        y_node.neighbours.insert(x);
        
        Ok(())
    }

    pub fn is_connected(&self, x: usize, y: usize) -> PyResult<bool> {
        if self.nodes.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in Graph"));
        }

        if !self.nodes.contains_key(&x) {
            return Err(PyValueError::new_err("X node not found in current Graph"));
        }

        if !self.nodes.contains_key(&y) {
            return Err(PyValueError::new_err("Y node not found in current Graph"));
        }

        let x_node = self.nodes.get(&x).unwrap();
        let y_node = self.nodes.get(&y).unwrap();
        if x_node.neighbours.contains(&y) && y_node.neighbours.contains(&x) {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn neighbours<'py>(&self, py: Python<'py>, index: usize) -> PyResult<&'py PyList> {
        if self.nodes.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in Graph"));
        }

        let new_node = match self.nodes.get(&index) {
            Some(value) => value,
            None => return Err(PyValueError::new_err("Index not found in present Graph")),
        };

        let final_list = PyList::new(py, new_node.neighbours.clone());
        Ok(final_list.into())
    }

    pub fn edges<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        if self.nodes.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in Graph"));
        }

        let mut elements = Vec::new();
        
        for (num, item) in self.nodes.iter() {
            elements.push((num, item.payload.clone_ref(py)).to_object(py));
        }

        let final_list = PyList::new(py, elements);
        Ok((final_list).into())
    }

    pub fn bfs_list<'py>(&self, py: Python<'py>, start_id: usize) -> PyResult<&'py PyList> {
        if self.nodes.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in Graph"));
        }

        if !self.nodes.contains_key(&start_id) {
            return Err(PyValueError::new_err("Index value not found in Graph"));
        }

        let mut visited = FxHashSet::default();
        let mut id_queue = VecDeque::new();
        let mut results = Vec::new();

        visited.insert(start_id);
        id_queue.push_back(start_id);

        while let Some(current_id) = id_queue.pop_front() {
            results.push(current_id);

            let node = self.nodes.get(&current_id).ok_or_else(|| {
                PyValueError::new_err("Corrupted Graph structure: Node missing during BFS")
            })?;

            for neigh_id in &node.neighbours {
                if !visited.contains(neigh_id) {
                    visited.insert(*neigh_id);
                    id_queue.push_back(*neigh_id);
                }
            }
        }

        let final_list = PyList::new(py, results);
        Ok((final_list).into())
    }

    pub fn size(&self) -> PyResult<usize> {
        Ok(self.count)
    }

    pub fn clear(&mut self) -> PyResult<()> {
        if self.nodes.is_empty() {
            return Err(PyValueError::new_err("Graph is already empty"))
        }

        self.nodes.clear();
        self.next_id = 1;
        self.count = 0;
        Ok(())
    }
}