use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::PyObject;
use pyo3::types::PyList;

#[pyclass]
struct WagonNode {
    data: PyObject,
    next: Option<Box<WagonNode>>
}

#[pyclass]
pub struct LinkedList {
    head: Option<Box<WagonNode>>,
    count: usize
}

#[pymethods]
impl LinkedList {
    #[new]
    pub fn new() -> Self {
        Self {
            head: None,
            count: 0 
        }
    }

    pub fn prepend(&mut self, value: PyObject) {
        let new_node = Box::new(WagonNode {
            data: value,
            next: self.head.take()
        });

        self.head = Some(new_node);
        self.count += 1;
    }

    pub fn append(&mut self, value: PyObject) {
        let new_node = Box::new(WagonNode {
            data: value,
            next: None
        });

        if self.head.is_none() {
            self.head = Some(new_node);
        } else {
            let mut current = self.head.as_mut().unwrap();

            while current.next.is_some() {
                current = current.next.as_mut().unwrap();
            }

            current.next = Some(new_node);
        }
        self.count += 1;
    }

    pub fn remove_head(&mut self) -> Option<PyObject> {
        if let Some(mut node) = self.head.take() {
            self.head = node.next.take();
            self.count -= 1;
            return Some(node.data);
        } else {
            return None;
        }
    }

    pub fn insert(&mut self, value: PyObject, index: Option<usize>) -> PyResult<()> {
        let idx = index.unwrap_or(self.count);

        if idx > self.count {
            return Err(PyValueError::new_err("Index out of bounds"))
        }

        if idx == 0 {
            let new_node = Box::new(WagonNode {
                data: value,
                next: self.head.take(),
            });
            self.head = Some(new_node);
        } else {
            let mut current_node = self.head.as_mut();
            for _ in 0..(idx - 1) {
                match current_node {
                    Some(node) => current_node = node.next.as_mut(),
                    None => return Err(PyValueError::new_err("Corrupted List")),
                }
            }

            if let Some(node) = current_node {
                let next_node = node.next.take();
                let new_node = Box::new(WagonNode {
                    data: value,
                    next: next_node,
                });
                node.next = Some(new_node);
            }
        }
        self.count += 1;
        Ok(())
    }

    pub fn get(&self, py: Python, index: usize) -> PyResult<PyObject> {
        if index >= self.count {
            return Err(PyValueError::new_err("Index out of bounds"))
        }

        let mut counter = 0;
        let mut current_node = self.head.as_ref();

        while let Some(node) = current_node {
            if counter == index {
                return Ok(node.data.clone_ref(py));
            }
            current_node = node.next.as_ref();
            counter += 1;
        }
        Err(PyValueError::new_err("Index not found"))
    }

    pub fn contains(&self, py: Python, value: PyObject) -> bool {
        let mut current_node = self.head.as_ref();

        while let Some(node) = current_node {
            if node.data.as_ref(py).eq(value.as_ref(py)).unwrap_or(false) {
                return true;
            }
            current_node = node.next.as_ref();
        }
        false
    }

    pub fn pop(&mut self, index: Option<usize>) -> PyResult<PyObject> {
        let idx = index.unwrap_or(self.count.checked_sub(1).ok_or_else(|| PyValueError::new_err("List is Empty"))?);

        self.remove(idx).ok_or_else(|| PyValueError::new_err("Index out of bounds"))
    }

    pub fn remove(&mut self, index: usize) -> Option<PyObject> {
        if index >= self.count {
            return None;
        }

        if index == 0 {
            return self.remove_head();
        }

        let mut current_node = self.head.as_mut()?;
        for _ in 0..(index - 1) {
            current_node = current_node.next.as_mut()?;
        }

        let removed_node = current_node.next.take();
        if let Some(mut node) = removed_node {
            current_node.next = node.next.take();
            self.count -= 1;
            Some(node.data)
        } else {
            None
        }
    }

    pub fn search(&self, py: Python, value: PyObject) -> Option<usize> {
        let mut current_node = self.head.as_ref();
        let mut index = 0;

        while let Some(node) = current_node {
            if node.data.as_ref(py).eq(value.as_ref(py)).unwrap_or(false) {
                return Some(index);
            }
            current_node = node.next.as_ref();
            index += 1;
        }
        None
    }

    pub fn update(&mut self, value: PyObject, index: usize) -> PyResult<()> {
        if index >= self.count {
            return Err(PyValueError::new_err("Index out of bounds"))
        }

        let mut counter = 0;
        let mut current_node = self.head.as_mut();

        while let Some(node) = current_node {
            if counter == index {
                node.data = value;
                return Ok(());
            }
            current_node = node.next.as_mut();
            counter += 1;
        }
        Err(PyValueError::new_err("No data found at index"))
    }

    pub fn to_list<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        let mut elements = Vec::new();
        let mut current_node = self.head.as_ref();

        while let Some(node) = current_node {
            elements.push(node.data.clone_ref(py));
            current_node = node.next.as_ref();
        }
        let list_bound = PyList::new(py, elements);
        Ok(list_bound)
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.count = 0;
    }

    pub fn __len__(&self) -> usize {
        self.count
    }

    pub fn __getitem__(&self, py: Python, index: usize) -> PyResult<PyObject> {
        self.get(py, index)
    }

    pub fn __setitem__(&mut self, value: PyObject, index: usize) -> PyResult<()> {
        self.update(value, index)
    }

    pub fn __delitem__(&mut self, index: usize) ->PyResult<()> {
        match self.remove(index) {
            Some(_) => Ok(()),
            None => Err(PyValueError::new_err("Index out of bounds")),
        }
    }

    pub fn __contains__(&self, py: Python, value: PyObject) -> bool {
        self.contains(py, value)
    }
}

