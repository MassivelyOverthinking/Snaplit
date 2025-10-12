use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::PyObject;
use std::cmp::Ordering;

struct AVLNode {
    value: PyObject,
    left: Option<Box<AVLNode>>,
    right: Option<Box<AVLNode>>,
    height: usize,
    count: usize,
}

impl AVLNode {
    fn new(data: PyObject) -> Self {
        Self {
            value: data,
            left: None,
            right: None,
            height: 1,
            count: 1
        }
    }
}

#[pyclass]
pub struct AVLTree {
    root: Option<Box<AVLNode>>,
    size: usize,
    allow_duplicates: bool,
}

impl AVLTree {
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
            Err(PyValueError::new_err("Cannot compare specified Python objects"))
        }
    }

    fn get_height(node: &Option<Box<AVLNode>>) -> i32 {
        match node {
            Some(ref boxed_node) => boxed_node.height as i32,
            None => 0,
        }
    }

    fn balance_factor(node: &AVLNode) {
        Self::get_height(&node.left) - Self::get_height(&node.right);
    }
}

#[pymethods]
impl AVLTree {
    #[new]
    pub fn new(allow_duplicates: bool) -> Self {
        Self {
            root: None,
            size: 0,
            allow_duplicates: allow_duplicates,
        }
    }

    pub fn add(&mut self, py: Python, value: PyObject) -> PyResult<()> {
        
    }

    pub fn remove(&mut self, py: Python, value: PyObject) -> PyResult<PyObject> {
        
    }

    pub fn prune(&mut self, _py: Python) -> PyResult<()> {

    }

    pub fn peek_root(&self, py: Python) -> PyResult<PyObject> {

    }

    pub fn contains(&self, py: Python, value: PyObject) -> PyResult<bool> {

    }

    pub fn extend(&mut self, py: Python, iterable: &PyList) -> PyResult<()> {

    }

    pub fn min(&self, py: Python) -> PyResult<usize> {

    }

    pub fn max(&self, py: Python) -> PyResult<usize> {
        
    }

    pub fn at_depth(&self, py: Python, value: PyObject) -> PyResult<usize> {

    }

    pub fn height(&self) -> usize {

    }

    pub fn size(&self) -> usize {
        
    }

    pub fn is_empty(&self) -> bool {
        
    }

    pub fn inorder_list<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {

    }

    pub fn preorder_list<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        
    }

    pub fn postorder_list<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        
    }

    pub fn bfs_list<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        
    }

    pub fn copy(&mut self, py: Python) -> PyResult<AVLTree> {

    }

    pub fn clear(&mut self) {

    }
 
}