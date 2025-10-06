// Imports
use pyo3::prelude::*;

// rust mods
mod linear;

// imports from rust folders
use linear::linked_list_native::LinkedList;
use linear::stack_native::Stack;
use linear::queue_native::Queue;

// Final export to Python
#[pymodule]
pub fn rust_snaplit(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<LinkedList>()?;
    m.add_class::<Stack>()?;
    m.add_class::<Queue>()?;
    Ok(())
}
