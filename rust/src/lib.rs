// Imports
use pyo3::prelude::*;
// rust mods
mod linear;
mod trees;

// imports from rust folders
use linear::linked_list_native::LinkedList;
use linear::stack_native::Stack;
use linear::queue_native::Queue;
use linear::ring_buffer_native::RingBuffer;
use linear::circular_buffer_native::CircularBuffer;
use linear::priority_queue_native::PriorityQueue;

use trees::rs_binary_tree_native::BinarySearchTree;

// Final export to Python
#[pymodule]
pub fn rust_snaplit(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<LinkedList>()?;
    m.add_class::<Stack>()?;
    m.add_class::<Queue>()?;
    m.add_class::<RingBuffer>()?;
    m.add_class::<CircularBuffer>()?;
    m.add_class::<PriorityQueue>()?;
    m.add_class::<BinarySearchTree>()?;
    Ok(())
}
