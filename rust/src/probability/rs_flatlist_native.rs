use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::PyObject;
use rand::Rng;
use rustc_hash::FxHashMap;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct FlatNode {
    id: usize,
    payload: PyObject,
}

impl FlatNode {
    fn new(id: usize, payload: PyObject) -> Self {
        Self {
            id: id,
            payload: payload,
        }
    }
}

impl Flatlist {
    fn coin_toss(&self) -> bool {
        let mut rng = rand::thread_rng();
        let n = rng.gen_bool(self.probability);
        return n;
    }

    fn get_top_level(&self) -> usize {
        let mut level = 0;

        while level < self.size - 1 && self.coin_toss() {
            level += 1;
        }
        level
    }

    fn py_compare(py: Python, x_value: &PyObject, y_value: &PyObject) -> Ordering {
        let x = x_value.as_ref(py);
        let y = y_value.as_ref(py);

        // Try X < Y
        if let Ok(obj) = x.rich_compare(y, CompareOp::Lt) {
            if obj.is_true().unwrap_or(false) {
                return Ordering::Less
            }
        }

        // Try X == Y
        if let Ok(obj) = x.rich_compare(y, CompareOp::Eq) {
            if obj.is_true().unwrap_or(false) {
                return Ordering::Equal
            }
        }

        // Try X > Y
        if let Ok(obj) = x.rich_compare(y, CompareOp::Gt) {
            if obj.is_true().unwrap_or(false) {
                return Ordering::Greater
            }
        }
        return Ordering::Equal
    }

}

#[pyclass]
pub struct Flatlist {
    size: usize,
    probability: f64,
    nex_id: usize,
    id_map: FxHashMap<usize, usize>,
    list: Vec<Vec<FlatNode>>,
}

#[pymethods]
impl Flatlist {
    #[new]
    pub fn new(num_list: Option<usize>, probability: Option<f64>) -> Self {
        let rs_num = num_list.unwrap_or(4);
        let rs_prob = probability.unwrap_or(0.5);
        Self {
            size: rs_num,
            probability: rs_prob,
            nex_id: 1,
            id_map: FxHashMap::default(),
            list: vec![Vec::new(); rs_num],
        }
    }

    pub fn insert(&mut self, py: Python, payload: PyObject) -> PyResult<bool> {
        let id = self.nex_id;
        let new_node = FlatNode::new(id, payload);
        let top_lvl = self.get_top_level();

        for lvl in 0..=top_lvl {
            self.list[lvl].push(new_node.clone());
            self.list[lvl].sort_by(|x, y| Flatlist::py_compare(py, &x.payload, &y.payload));
        }

        self.id_map.insert(id, top_lvl);
        self.nex_id += 1;
        Ok(true)
    }

    pub fn remove(&mut self, py: Python, key: PyObject) -> PyResult<PyObject> {
        let mut removed_node = None;

        for level in self.list.iter_mut().rev() {
            level.retain(|node| {
                if node.payload.as_ref(py).eq(key.as_ref(py)).unwrap_or(false) {
                    removed_node = Some(node.payload.clone());
                    false
                } else {
                    true
                }
            });
        }
        
        match removed_node {
            Some(value) => return Ok(value),
            None => return Err(PyValueError::new_err(format!("No value with key {} found in list!", key)))
        }
    }

    pub fn contains(&self, py:Python, key: PyObject) -> PyResult<bool> {
        for level in self.list.iter().rev() {
            
            let mut index = 0;
            while index < level.len() {
                let comparison = Flatlist::py_compare(py, &level[index].payload, &key);

                match comparison {
                    Ordering::Less => index += 1,
                    Ordering::Equal => return Ok(true),
                    Ordering::Greater => break,
                }
            }
        }
        Ok(false)
    }
    
}