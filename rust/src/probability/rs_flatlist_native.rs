use pyo3::basic::CompareOp;
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

    fn get_num_list(&self) -> usize {
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
        let top_lvl = self.get_num_list();

        for lvl in 0..=top_lvl {
            self.list[lvl].push(new_node.clone());
            self.list[lvl].sort_by(|x, y| Flatlist::py_compare(py, &x.payload, &y.payload));
        }

        self.id_map.insert(id, top_lvl);
        Ok(true)
    }
}