use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3::exceptions::PyValueError;

#[pyclass]
struct WagonNode {
    data: PyObject,
    next: Option<Box<WagonNode>>
}

#[pymethods]
impl WagonNode {
    #[new]
    pub fn new(data: PyObject, next: Option<Box<WagonNode>>) -> Self {
        Self { data, next} 
    }
}

#[pyclass]
struct LinkedList {
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

    pub fn remove_head(&mut self) -> Option<Box<WagonNode>> {
        let old_head = self.head.take();

        if let Some(mut node) = old_head {
            self.head = node.next.take();
            self.count -= 1;
            return Some(node);
        } else {
            return None;
        }
    }

    pub fn insert(&mut self, value: PyObject, index: Option<usize>) -> PyResult<()> {
        let idx = index.unwrap_or(self.count);

        if idx > self.count {
            return Err(PyValueError::new_err("Index out of bounds"))
        }

        let new_node = Box::new(WagonNode {
            data: value,
            next: None
        });

        if idx == 0 {
            new_node.next = self.head.take();
            self.head = Some(new_node);
        } else {
            let mut current_node = self.head.as_mut();
            for node in 0..(idx - 1) {
                match current_node {
                    Some(node) => current_node = node.next.as_mut(),
                    None => return Err(PyValueError::new_arr("Corrupted List")),
                }
            }

            let next = current_node.as_mut().unwrap().next.take();
            new_node.next = next;
            current_node.as_mut().unwrap().next = Some(new_node)
        }
        self.count += 1;
        return  Ok(());
    }

    pub fn get(&mut self, index: usize) -> Option<PyObject> {

    }

    pub fn remove(&mut self, node: WagonNode) -> Box<Option<WagonNode>> {

    }

    pub fn search(&self, value: i32) -> i32 {

    }

    pub fn to_list(&self, py: Python<'py>) -> Option<Bound<&'py PyList>> {
        if self.head.is_none() {
            return None;
        } else {
            let mut new_list: Vec<PyObject> = Vec::new();

            let mut current: WagonNode = self.head.as_mut().unwrap();
            while current.next.is_some() {
                new_list.push(&current.data);
                current = current.next.as_mut().unwrap();
            }

            return PyList::new(py, &new_list);
        }
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.count = 0;
    }

    pub fn __len__(&mut self) -> usize {
        return self.count;
    }
}