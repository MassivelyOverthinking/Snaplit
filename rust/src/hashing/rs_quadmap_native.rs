use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyList, PyTuple};
use pyo3::PyObject;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// ---------------------------------------------------------------------------------
/// Implementation Hashable Enum & Conversion of Python objects -> Rust data types
/// ---------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Slot {
    Empty,
    Tombstone,
    Occupied((PyObject, PyObject)),
}

enum Hashable {
    Int(i64),
    Float(u64),
    Str(String),
    Bool(bool),
}

impl Hash for Hashable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Hashable::Int(i) => i.hash(state),
            Hashable::Float(f) => f.hash(state),
            Hashable::Str(s) => s.hash(state),
            Hashable::Bool(b) => b.hash(state),
        }
    }
}

/// ---------------------------------------------------------------------------------
/// Implementation of QuadMap-class & related operations
/// ---------------------------------------------------------------------------------

#[pyclass]
pub struct QuadMap {
    capacity: usize,
    map_size: usize,
    series: Vec<Slot>
}

impl QuadMap {
    fn python_to_rust(py: Python, item: &PyObject) -> PyResult<Hashable> {
        // Converts Python native data types -> Rust native data types
        if let Ok(i) = item.extract::<i64>(py) {
            return Ok(Hashable::Int(i));
        } else if let Ok(f) = item.extract::<f64>(py) {
            return Ok(Hashable::Float(f.to_bits()));
        } else if let Ok(s) = item.extract::<String>(py)  {
            return Ok(Hashable::Str(s));
        } else if let Ok(b) = item.extract::<bool>(py) {
            return Ok(Hashable::Bool(b));
        } else {
            return Err(PyValueError::new_err("Unsupported data type for Rust conversion"));
        }
    }

    fn generate_hash<T: Hash>(&self, key: &T) -> usize {
        // Generates the intial Hash index for Vector insertion
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash_value = hasher.finish();
        let map_capacity = self.capacity;
        return (hash_value as usize) % map_capacity;
    }
}

#[pymethods]
impl QuadMap {
    #[new]
    pub fn new(capacity: Option<usize>) -> Self {
        let qm_cap = capacity.unwrap_or(1024);
        Self {
            capacity: qm_cap,
            map_size: 0,
            series: vec![Slot::Empty; qm_cap],
        }
    }

    pub fn insert(&mut self, py: Python, key: PyObject, value: PyObject) -> PyResult<bool> {
        // Check if the current map size is above the internal capacity values -> Map if full!
        let cap = self.capacity;
        if self.map_size >= cap {
            return Err(PyValueError::new_err(format!("Maximum capacity {} reached! Unable to insert key-value", cap)));
        }

        // Convert key-value Rust data-type & produce hash-value for indexing.
        let rust_hash = Self::python_to_rust(py, &key)?;
        let hash = Self::generate_hash(&self, &rust_hash); 

        // Iterate over the internal Series array -> Probe Chaining
        for quad_idx in 0..cap {

            // Calculate the index for loop. 
            let index = (hash + quad_idx * quad_idx) % cap;
            // Match the Slot at specified index in internal Series array.
            match &mut self.series[index] {
                // If Slot::Empty -> Insert key-value tuple & increment map_size.
                Slot::Empty => {
                    self.series[index] = Slot::Occupied((key.clone_ref(py), value.clone_ref(py)));
                    self.map_size+= 1;
                    return Ok(true)
                },
                // If Slot::Tombstone -> Insert key-value tuple & increment map_size.
                Slot::Tombstone => {
                    self.series[index] = Slot::Occupied((key.clone_ref(py), value.clone_ref(py)));
                    self.map_size += 1;
                    return Ok(true)
                },
                // If Slot::Occupied -> Continue to next loop iteration (Begin Probe Chain).
                Slot::Occupied(tuple) => {
                    if tuple.0.as_ref(py).eq(key.as_ref(py))? {
                        tuple.1 = value;
                        return Ok(false)
                    }
                },
            }
        }
        // DEFAULT = Return a PyValueEror if the value was nuable to be inserted.
        return Err(PyValueError::new_err(format!("Could not insert key {} into QuadMap", key)));
    }

    pub fn remove(&mut self, py: Python, key: PyObject) -> PyResult<PyObject> {
        // Get maximum capacity number to utilise. 
        let cap = self.capacity;

        // Convert key Rust data-type & produce hash-value for indexing.
        let rust_hash = Self::python_to_rust(py, &key)?;
        let hash = Self::generate_hash(&self, &rust_hash);

        // Iterate over the internal Series array -> Probe Chaining
        for quad_idx in 0..cap {
            // Calculate the index for loop. 
            let index = (hash + quad_idx * quad_idx) % cap;
            // Match the Slot at specified index in internal Series array.
            match &mut self.series[index] {
                // If Slot::Tombstone -> Continue to next loop iteration.
                Slot::Tombstone => {
                    continue;
                },
                // If Slot::Empty -> Raise ValueError (No entry with key-value found!).
                Slot::Empty => {
                    return Err(PyValueError::new_err(format!("Could not locate key {} in QuadMap", key)));
                },
                // If Slot::Occupied -> Remove & return value.
                // Set the Slot at the removed value index to Slot::Tombstone. 
                Slot::Occupied(tuple) => {
                    if tuple.0.as_ref(py).eq(key.as_ref(py))? {
                        let removed_value = tuple.1.clone_ref(py);
                        self.series[index] = Slot::Tombstone;
                        self.map_size -= 1;
                        return Ok(removed_value);
                    }
                }
            }
        }
        // Default = Raise ValueError (No entry with key-value found!)
        return Err(PyValueError::new_err(format!("Could not locate key {} in QuadMap", key)));
    }

    pub fn update(&mut self, py: Python, key: PyObject, new_value: PyObject) -> PyResult<bool> {
        // Get maximum capacity number to utilise. 
        let cap = self.capacity;

        // Convert key Rust data-type & produce hash-value for indexing.
        let rust_hash = Self::python_to_rust(py, &key)?;
        let hash = Self::generate_hash(&self, &rust_hash);

        // Iterate over entire Series-array starting from 'Index'.
        for quad_idx in 0..cap {

            // Calculate the index for loop. 
            let index = (hash + quad_idx * quad_idx) % cap;
            // Match internal Slots.
            match &mut self.series[index] {
                // If Slot::Occupied -> Check to see if the correct key is present.
                // If correct -> Update the value variable & return 'True'.
                Slot::Occupied(tuple) => {
                    if tuple.0.as_ref(py).eq(key.as_ref(py))? {
                        tuple.1 = new_value;
                        return Ok(true)
                    }
                },
                // If Slot::Tombstone -> Continue Quadratic Probing.
                Slot::Tombstone => {
                    continue;
                },
                // If Slot::Empty -> Value is not found. Return 'False'.
                Slot::Empty => {
                    return Ok(false);
                }
            }
        }
        // DEFAULT = Iterated over entire Series-array & key-value is not found.
        Ok(false)
    }

    pub fn get(&self, py: Python, key: PyObject) -> PyResult<PyObject> {
        // Get maximum capacity number to utilise.
        let cap = self.capacity;

        // Convert key Rust data-type & produce hash-value for indexing.
        let rust_hash = Self::python_to_rust(py, &key)?;
        let hash = Self::generate_hash(&self, &rust_hash);

        // Iterate through intern Series array.
        for quad_idx in 0..cap {

            // Calculate the index for loop. 
            let index = (hash + quad_idx * quad_idx) % cap;
            // Match internal Slots.
            match &self.series[index] {
                // If Slot::Occupied -> Check to see if the correct key is present.
                Slot::Occupied(tuple) => {
                    if tuple.0.as_ref(py).eq(key.as_ref(py))? {
                        let result = tuple.1.clone_ref(py);
                        return Ok(result)
                    }
                },
                // If Slot::Tombstone -> Continnue Quadratic Probe Chain.
                Slot::Tombstone => {
                    continue;
                },
                // If Slot::Empty -> Value is not found. Return 'None'.
                Slot::Empty => {
                    return Ok(py.None());
                }
            }
        }
        // DEFAULT = Return None if the value was not found (Iterated over entire Map).
        return Ok(py.None())
    }

    pub fn contains(&self, py: Python, key: PyObject) -> PyResult<bool> {
        // Get maximum capacity number to utilise.
        let cap = self.capacity;

        // Convert key Rust data-type & produce hash-value for indexing.
        let rust_hash = Self::python_to_rust(py, &key)?;
        let hash = Self::generate_hash(&self, &rust_hash);

        // Utilise loop structure to check internal Series-array for specified key-value.
        for quad_idx in 0..cap {

            // Calculate the index for loop. 
            let index = (hash + quad_idx * quad_idx) % cap;
            // Match internal Slots.
            match &self.series[index] {
                // If Slot::Occupied -> Check to see if the correct key is present.
                Slot::Occupied(tuple) => {
                    if tuple.0.as_ref(py).eq(key.as_ref(py))? {
                        return Ok(true);
                    }
                },
                // If Slot::Tombstone -> Continue Quadratic Probe Chain.
                Slot::Tombstone => {
                    continue;
                },
                // If Slot::Empty -> Value is not found. Return 'False'.
                Slot::Empty => {
                    return Ok(false);
                }
            }
        }
        // DEFAULT = If iterated over the entire Series-array return 'False'.
        Ok(false)
    }

    pub fn from_keys<'py>(&self, py: Python<'py>, iterable: &PyAny) -> PyResult<&'py PyList> {
        // Initiate new Rust Vectors to store value.
        let mut elements = Vec::new();

        // Iterate over all keys elements in 'iterable' parameter.
        for key_object in iterable.iter()? {
            // Extract the key-value from behind Result-type.
            let key = key_object?;
            // Utilise internal .get() method to retrieve values.
            let value = self.get(py, key.to_object(py))?;

            // Check if the retrieved value is 'None' before adding to elements list.
            if !value.is_none(py) {
                elements.push(value);
            }
        }
        // Create a final PyList-object and return to user.
        Ok(PyList::new(py, &elements))
    }

    pub fn keys<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        // Initialize a temporery vector array to store key-values in.
        let mut elements = Vec::new();

        // Iterate through internal Series-array & append keys to 'elements' vector.
        for slot in self.series.iter() {
            // Match Slot Enum type.
            match slot {
                // If Slot::Occupied -> Push the '0' value from Tuple to list.
                Slot::Occupied(tuple) => {
                    elements.push(&tuple.0);
                },
                // If Slot::Tombstone -> Continue to next iteration of the loop.
                Slot::Tombstone => {
                    continue;
                },
                // If Slot::Empty -> Continue to next iteration of the loop.
                Slot::Empty => {
                    continue;
                }
            }
        }
        // Convert the 'Elements' Vector to a Python native list structure.
        Ok(PyList::new(py, elements))
    }

    pub fn values<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        // Initialize a temporery vector array to store values in.
        let mut elements = Vec::new();

        // Iterate through internal Series-array & append values to 'elements' vector.
        for slot in self.series.iter() {
            // Match Slot Enum type.
            match slot {
                // If Slot::Occupied -> Push the '1' value from Tuple to list.
                Slot::Occupied(tuple) => {
                    elements.push(&tuple.1);
                },
                // If Slot::Tombstone -> Continue to next iteration of the loop.
                Slot::Tombstone => {
                    continue;
                },
                // If Slot::Empty -> Continue to next iteration of the loop.
                Slot::Empty => {
                    continue;
                }
            }
        }
        // Convert the 'Elements' Vector to a Python native list structure.
        Ok(PyList::new(py, elements))
    }

    pub fn items<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        // Initialize a temporery vector array to store key-value pairs in.
        let mut elements = Vec::new();

        // Iterate through internal Series-array & append key-value pairs to 'elements' vector.
        for slot in self.series.iter() {
            // Match Slot Enum type.
            match slot {
                // If Slot::Occupied -> Push both (0, 1) value from Tuple to list.
                Slot::Occupied(tuple) => {
                    elements.push((&tuple.0, &tuple.1));
                },
                // If Slot::Tombstone -> Continue to next iteration of the loop.
                Slot::Tombstone => {
                    continue;
                },
                // If Slot::Empty -> Continue to next iteration of the loop.
                Slot::Empty => {
                    continue;
                }
            }
        }
        // Convert the 'Elements' Vector to a Python native list structure.
        Ok(PyList::new(py, elements))
    }

    pub fn copy(&self, py: Python<'_>) -> PyResult<PyObject> {
        // Initiate a new, empty QuadMap-class.
        let mut new_map = QuadMap::new(Some(self.capacity));

        // Iterate through all stored items.
        for tuple in self.items(py)?.iter() {
            // Downcast entry to PyTuple to extract key & value pairs safely.
            let tup = tuple.downcast::<PyTuple>()?;
            let key = tup.get_item(0)?;
            let val = tup.get_item(1)?;

            // Convert &PyAny -> PyObject to safely add to new QuadMap instance.
            new_map.insert(py, key.to_object(py), val.to_object(py))?;
        }
        // Convert the new, fully loaded QuadMap to a PyObject. 
        Ok(Py::new(py, new_map)?.into_py(py))
    }

    pub fn info<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        // Extract teh necessary metrcis from internal variables & methods
        let percentage = self.percentage()?;
        let keys = self.keys(py)?.into();
        let values = self.values(py)?.into();

        // Construct a Rust Vector consisting of indvidual Tuples (String, PyObject).
        let key_vals: Vec<(&str, PyObject)> = vec![
            ("type", "QuadMap".to_object(py)),
            ("capacity", self.capacity.to_object(py)),
            ("size", self.map_size.to_object(py)),
            ("percentage", percentage.to_object(py)),
            ("keys", keys),
            ("values", values)
        ];

        // Convert Vector to Python Dictionary and return value.
        let py_dict = key_vals.into_py_dict(py);
        Ok(py_dict)
    }

    pub fn capacity(&self) -> PyResult<usize> {
        // Return the total entry capacity of the QuadMap.
        Ok(self.capacity)
    }

    pub fn size(&self) -> PyResult<usize> {
        // Return the current entry number of the QuadMap.
        Ok(self.map_size)
    }

    pub fn percentage(&self) -> PyResult<f64> {
        // Return the percent of the internal Series array that is currently occupied.
        let percent = (self.map_size as f64 / self.capacity as f64) * 100.0;
        Ok(percent)
    }

    pub fn is_empty(&self) -> PyResult<bool> {
        // Check if the internal Series-array contains no current entries. 
        Ok(self.map_size <= 0)
    }

    pub fn clear(&mut self) -> PyResult<()> {
        // Set internal values in Series-array to Slot::Empty & reset variable 'map_size' to 0.
        self.map_size = 0;
        self.series = vec![Slot::Empty; self.capacity];
        Ok(())
    }
}