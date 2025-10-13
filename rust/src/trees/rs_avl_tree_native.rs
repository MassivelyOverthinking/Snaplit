use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::PyObject;
use std::cmp::Ordering;
use std::collections::VecDeque;

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

    fn left_rotation(py: Python, node: &Option<Box<AVLNode>>) {

    }

    fn right_rotation(py: Python, node: &Option<Box<AVLNode>>) {
        
    }

    fn inorder_traversal(py: Python, node: &Option<Box<AVLNode>>, acc: &mut Vec<PyObject>, duplicate: bool) {
        if let Some(ref boxed_node) = node {

            Self::inorder_traversal(py, &boxed_node.left, acc, duplicate);
            
            if duplicate {
                for _ in 0..boxed_node.count {
                    acc.push(boxed_node.value.clone_ref(py));
                }
            } else {
                acc.push(boxed_node.value.clone_ref(py));
            }

            Self::inorder_traversal(py, &boxed_node.right, acc, duplicate);
        }
    }

    fn preorder_traversal(py: Python, node: &Option<Box<AVLNode>>, acc: &mut Vec<PyObject>, duplicate: bool) {
        if let Some(ref boxed_node) = node {
            
            if duplicate {
                for _ in 0..boxed_node.count {
                    acc.push(boxed_node.value.clone_ref(py));
                }
            } else {
                acc.push(boxed_node.value.clone_ref(py));
            }

            Self::preorder_traversal(py, &boxed_node.left, acc, duplicate);
            Self::preorder_traversal(py, &boxed_node.right, acc, duplicate);
        }
    }

    fn postorder_traversal(py: Python, node: &Option<Box<AVLNode>>, acc: &mut Vec<PyObject>, duplicate: bool) {
        if let Some(ref boxed_node) = node {

            Self::postorder_traversal(py, &boxed_node.left, acc, duplicate);
            Self::postorder_traversal(py, &boxed_node.right, acc, duplicate);
            
            if duplicate {
                for _ in 0..boxed_node.count {
                    acc.push(boxed_node.value.clone_ref(py));
                }
            } else {
                acc.push(boxed_node.value.clone_ref(py));
            }
        }
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
        let mut current_node = &mut self.root;

        while let Some(node) = current_node {
            match Self::comparison(py, value.clone(), node.value.clone())? {
                Ordering::Less => {
                    current_node = &mut node.left
                }
                Ordering::Greater => {
                    current_node = &mut node.right
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

        *current_node = Some(Box::new(AVLNode::new(value)));
        self.size += 1;
        Ok(())

    }

    pub fn remove(&mut self, py: Python, value: PyObject) -> PyResult<PyObject> {
        
    }

    pub fn prune(&mut self, _py: Python) -> PyResult<()> {

    }

    pub fn peek_root(&self, py: Python) -> PyResult<PyObject> {
        match self.root.as_ref() {
            Some(node) => Ok(node.value.clone_ref(py)),
            None => Err(PyValueError::new_err("No elements currently available in AVL Tree"))
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

    pub fn extend(&mut self, py: Python, iterable: &PyList) -> PyResult<()> {
        for item in iterable.iter() {
            let obj = item.extract()?;
            self.add(py, obj)?;
        }
        Ok(())
    }

    pub fn min(&self, py: Python) -> PyResult<PyObject> {
        if self.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in AVL Tree"));
        }

        let mut current_node = self.root.as_ref();
        while let Some(node) = current_node {
            if node.left.is_none() {
                return Ok(node.value.clone_ref(py));
            }
            current_node = node.left.as_ref();
        }
        Err(PyValueError::new_err("Invalid tree structure"))
    }

    pub fn max(&self, py: Python) -> PyResult<PyObject> {
        if self.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in AVL Tree"));
        }

        let mut current_node = self.root.as_ref();
        while let Some(node) = current_node {
            if node.right.is_none() {
                return Ok(node.value.clone_ref(py));
            }
            current_node = node.right.as_ref();
        }
        Err(PyValueError::new_err("Invalid Tree structure"))
    }

    pub fn at_depth(&self, py: Python, value: PyObject) -> PyResult<usize> {
        
    }

    pub fn height(&self) -> usize {

    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn inorder_list<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        if self.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in AVL Tree"));
        }

        let mut elements = Vec::with_capacity(self.size);
        Self::inorder_traversal(py, &self.root, &mut elements, self.allow_duplicates);
        Ok(PyList::new(py, elements))
    }

    pub fn preorder_list<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        if self.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in AVL Tree"));
        }

        let mut elements = Vec::with_capacity(self.size);
        Self::preorder_traversal(py, &self.root, &mut elements, self.allow_duplicates);
        Ok(PyList::new(py, elements))
    }

    pub fn postorder_list<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        if self.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in AVL Tree"));
        }

        let mut elements = Vec::with_capacity(self.size);
        Self::postorder_traversal(py, &self.root, &mut elements, self.allow_duplicates);
        Ok(PyList::new(py, elements))
    }

    pub fn bfs_list<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        if self.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in AVL Tree"));
        }

        let mut results = Vec::new();
        let mut queue = VecDeque::new();

        if let Some(ref root_node) = self.root {
            queue.push_back(root_node);
        }

        while let Some(current_node) = queue.pop_front() {
            if self.allow_duplicates {
                for _ in 0..current_node.count {
                    results.push(current_node.value.clone_ref(py));
                }
            } else {
                results.push(current_node.value.clone_ref(py));
            }

            if let Some(ref left_node) = current_node.left {
                queue.push_back(left_node);
            }
            if let Some(ref right_node) = current_node.right {
                queue.push_back(right_node);
            }
        }
        Ok(PyList::new(py, results))
    }

    pub fn copy(&mut self, py: Python) -> PyResult<PyObject> {
        if self.is_empty() {
            return Err(PyValueError::new_err("No elements currently available in AVL Tree"));
        }

        let mut new_tree = AVLTree::new(self.allow_duplicates);
        let tree_list = self.bfs_list(py)?;

        for item in tree_list.iter() {
            let obj = item.extract()?;
            new_tree.add(py, obj)?;
        }
        Py::new(py, new_tree).map(|py_obj| py_obj.to_object(py))
    }

    pub fn clear(&mut self) {
        self.root = None;
        self.size = 0;
    }
 
}