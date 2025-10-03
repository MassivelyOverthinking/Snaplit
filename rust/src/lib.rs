mod linear;

use pyo3::prelude::*;
use linear::linked_list::LinkedList;

#[pymodule]
fn linkedlist(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<LinkedList>()?;
    Ok(())
}
