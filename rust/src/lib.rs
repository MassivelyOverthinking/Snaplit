// Imports
use pyo3::prelude::*;

mod linear;

use linear::linked_list_native::LinkedList;
use linear::stack_native::Stack;

// Final export to Python

#[pymodule]
pub fn rust_snaplit(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<LinkedList>()?;
    m.add_class::<Stack>()?;
    Ok(())
}
