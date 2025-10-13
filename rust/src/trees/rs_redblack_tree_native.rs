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