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

    fn get_height(node: &Option<Box<AVLNode>>) -> usize {
        return node.as_ref().map_or(0, |n| n.height);
    }

    fn update_height(node: &mut Box<AVLNode>) {
        let left_height = Self::get_height(&node.left);
        let right_height = Self::get_height(&node.right);
        node.height = 1 + left_height.max(right_height);
    }

    fn balance_factor(node: &Box<AVLNode>) -> isize {
        let left = Self::get_height(&node.left) as isize;
        let right = Self::get_height(&node.right) as isize;
        left - right
    }

    fn right_rotation(_py: Python, node: &mut Option<Box<AVLNode>>) {
        if let Some(mut x_node) = node.take() {
            if let Some(mut y_node) = x_node.left.take() {
                let t2 = y_node.right.take();

                y_node.right = Some(x_node);
                y_node.right.as_mut().unwrap().left = t2;

                Self::update_height(y_node.right.as_mut().unwrap());
                Self::update_height(&mut y_node);

                *node = Some(y_node);
            } else {
                *node = Some(x_node);
            }
        }
    }

    fn left_rotation(_py: Python, node: &mut Option<Box<AVLNode>>) {
        if let Some(mut x_node) = node.take() {
            if let Some(mut y_node) = x_node.right.take() {
                let t2 = y_node.left.take();

                y_node.left = Some(x_node);
                y_node.left.as_mut().unwrap().right = t2;

                Self::update_height(y_node.left.as_mut().unwrap());
                Self::update_height(&mut y_node);

                *node = Some(y_node);
            } else {
                *node = Some(x_node);
            }
        }
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

    fn insert(py: Python, node: Option<Box<AVLNode>>, value: &PyObject, duplicate: bool) -> PyResult<Option<Box<AVLNode>>> {
        if let Some(mut n_node) = node {
            match Self::comparison(py, value.clone(), n_node.value.clone())? {
                Ordering::Less => {
                    n_node.left = Self::insert(py, n_node.left, value, duplicate)?;
                }
                Ordering::Greater => {
                    n_node.right = Self::insert(py, n_node.right, value, duplicate)?;
                }
                Ordering::Equal => {
                    if duplicate {
                        n_node.count += 1;
                        return Ok(Some(n_node));
                    } else {
                        return Ok(Some(n_node));
                    }
                }
            }
            Self::update_height(&mut n_node);
            let balance = Self::balance_factor(&n_node);

            let cmp_left = n_node.left.as_ref().map(|left| {
                Self::comparison(py, value.clone(), left.value.clone())
            }).transpose()?;

            let cmp_right = n_node.right.as_ref().map(|right| {
                Self::comparison(py, value.clone(), right.value.clone())
            }).transpose()?;

            if balance > 1 && cmp_left == Some(Ordering::Less) {
                let mut boxed_node = Some(n_node);
                Self::right_rotation(py, &mut boxed_node);
                return Ok(boxed_node);
            }

            if balance < -1 && cmp_right == Some(Ordering::Greater) {
                let mut boxed_node = Some(n_node);
                Self::left_rotation(py, &mut boxed_node);
                return Ok(boxed_node);
            }

            if balance > 1 && cmp_left == Some(Ordering::Greater) {
                let mut left_subtree = n_node.left.take();
                Self::left_rotation(py, &mut left_subtree);
                n_node.left = left_subtree;

                let mut boxed_node = Some(n_node);
                Self::right_rotation(py, &mut boxed_node);
                return Ok(boxed_node);
            }

            if balance > 1 && cmp_right == Some(Ordering::Less) {
                let mut right_subtree = n_node.right.take();
                Self::right_rotation(py, &mut right_subtree);
                n_node.right = right_subtree;

                let mut boxed_node = Some(n_node);
                Self::left_rotation(py, &mut boxed_node);
                return Ok(boxed_node);
            }

            Ok(Some(n_node))
        } else {
            Ok(Some(Box::new(AVLNode::new(value.clone()))))
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
        self.root = AVLTree::insert(py, self.root.take(), &value, self.allow_duplicates)?;
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