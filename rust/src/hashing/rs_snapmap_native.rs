use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::PyObject;

#[derive(Debug, Clone)]
struct CuckooBucket {
    slots: Vec<PyObject>,
}

impl CuckooBucket {
    fn new(py: Python, slot_num: usize) -> Self {
        let none_obj = py.None().into_py(py);
        Self {
            slots: vec![none_obj; slot_num],
        }
    }
}

#[pyclass]
pub struct SnapMap {
    capacity: usize,
    bucket_size: usize,
    first_layer: Vec<CuckooBucket>,
    second_layer: Vec<CuckooBucket>,
}

impl SnapMap {
    fn generate_map_capacity(capacity: usize, size: usize) -> usize {
        return capacity / size as usize;
    }
}

#[pymethods]
impl SnapMap {
    #[new]
    pub fn new(py: Python, capacity: Option<usize>, bucket_size: Option<usize>) -> Self {
        let sm_cap = capacity.unwrap_or(1024);
        let sm_buc = bucket_size.unwrap_or(4);
        let final_size = Self::generate_map_capacity(sm_cap, sm_buc);
        Self {
            capacity: sm_cap,
            bucket_size: sm_buc,
            first_layer: vec![CuckooBucket::new(py, sm_buc); final_size],
            second_layer: vec![CuckooBucket::new(py, sm_buc); final_size],
        }
    }
}
