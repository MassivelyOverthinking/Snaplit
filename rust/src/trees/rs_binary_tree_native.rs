use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::PyObject;
use std::cmp::Ordering;

struct LeafNode {
    value: PyObject,
    left: Option<Box<LeafNode>>,
    right: Option<Box<LeafNode>>,
    count: usize,
}

impl LeafNode {
    fn new(data: PyObject) -> Self {
        Self {
            value: data,
            left: None,
            right: None,
            count: 1
        }
    }
}

#[pyclass]
pub struct BinarySearchTree {
    root: Option<Box<LeafNode>>,
    size: usize,
    allow_duplicates: bool,
}

impl BinarySearchTree {
    fn comparison(py: Python, x: PyObject, y: PyObject) -> PyResult<Ordering> {
        let x_ref = x.as_ref(py);
        let y_ref = y.as_ref(py);

        if x_ref.lt(y_ref)? {
            Ok(Ordering::Less)
        } else if x_ref.gt(y_ref)? {
            Ok(Ordering::Greater)
        } else if x_ref.eq(y_ref)? {
            Ok(Ordering::Equal)
        } else {
            Err(PyValueError::new_err("Cannot compare Python Objects"))
        }
    }

    fn node_height(node: &Option<Box<LeafNode>>) -> usize {
        if let Some(n) = node {
            let left_height = Self::node_height(&n.left);
            let right_height = Self::node_height(&n.right);
            1 + left_height.max(right_height)
        } else {
            0
        }
    }
}

#[pymethods]
impl BinarySearchTree {
    #[new]
    pub fn new(allow_duplicates: bool) -> Self {
        Self {
            root: None,
            size: 0,
            allow_duplicates: allow_duplicates,
        }
    }

    pub fn add(&mut self, py: Python, value: PyObject) -> PyResult<()> {
        let mut current_node = &mut self.root;

        while let Some(node) = current_node {
            match Self::comparison(py, value.clone(), node.value.clone())? {
                Ordering::Less => {
                    current_node = &mut node.left;
                }
                Ordering::Greater => {
                    current_node = &mut node.right;
                }
                Ordering::Equal => {
                    if self.allow_duplicates {
                        node.count += 1;
                        self.size += 1;
                    }
                    return Ok(());
                }
            }
        }
        *current_node = Some(Box::new(LeafNode::new(value)));
        self.size += 1;

        Ok(())
    }

    pub fn remove(&mut self, py: Python) -> PyResult<()> {
        
    }

    pub fn peek_root(&self, py: Python) -> PyResult<PyObject> {
        match self.root.as_ref() {
            Some(node) => Ok(node.value.clone_ref(py)),
            None => Err(PyValueError::new_err("No elements currently available in the BST"))
        }
    }

    pub fn contains(&self, py: Python, value: PyObject) -> PyResult<bool> {
        if self.is_empty() {
            return Ok(false);
        }

        let mut current_node = self.root.as_ref();
        while let Some(node) = current_node {
            match Self::comparison(py, value.clone(), node.value.clone())? {
                Ordering::Less => current_node = node.left.as_ref(),
                Ordering::Greater => current_node = node.right.as_ref(),
                Ordering::Equal => return Ok(true)
            }
        }
        return Ok(false);
    }

    pub fn min(&self, py: Python) -> PyResult<PyObject> {
        if self.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in the BST"));
        }

        let current_node = self.root.as_ref();
        while let Some(node) = current_node {
            if node.left.is_none() {
                return Ok(node.value.clone_ref(py));
            }
        }
        Err(PyValueError::new_err("Invalid Tree Structure"))
    }

    pub fn max(&self, py: Python) -> PyResult<PyObject> {
        if self.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in the BST"));
        }

        let current_node = self.root.as_ref();
        while let Some(node) = current_node {
            if node.right.is_none() {
                return Ok(node.value.clone_ref(py));
            }
        }
        Err(PyValueError::new_err("Invalid Tree Structure"))
    }

    pub fn height(&self) -> usize {
        Self::node_height(&self.root)
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn clear(&mut self) {
        self.root = None;
        self.size = 0;
    }
}