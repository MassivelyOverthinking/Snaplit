use std::cmp::Ordering;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::PyObject;

pub enum Colour {
    Red,
    Black,
}

struct ColourNode {
    value: PyObject,
    left: Option<Box<ColourNode>>,
    right: Option<Box<ColourNode>>,
    colour: Colour,
    count: usize,
}

impl ColourNode {
    fn new(data: PyObject) -> Self {
        Self {
            value: data,
            left: None,
            right: None,
            colour: Colour::Black,
            count: 1
        }
    }
}

#[pyclass]
pub struct RedBlackTree {
    root: Option<Box<ColourNode>>,
    size: usize,
    allow_duplicates: bool,
}

impl RedBlackTree {
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
            Err(PyValueError::new_err("Cannnot compare specified Python objects"))
        }
    }
}

#[pymethods]
impl RedBlackTree {
    #[new]
    pub fn new(allow_duplicates: bool) -> Self {
        Self {
            root: None,
            size: 0,
            allow_duplicates: allow_duplicates,
        }
    }
}