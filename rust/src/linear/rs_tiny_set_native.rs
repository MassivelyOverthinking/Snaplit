use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::PyObject;
use pyo3::types::{IntoPyDict, PyDict, PyList};

/// ---------------------------------------------------------------------------------
/// Implementation of TinySet main class & general methods/operations
/// ---------------------------------------------------------------------------------


#[pyclass]
pub struct TinySet {
    capacity: usize,
    size: usize,
    none: PyObject,
    threshold: f64,
    array: Vec<PyObject>,
}

impl TinySet {
    fn shift_upwards(&mut self, end_index: usize) {
        if end_index >= self.array.len() - 1 {
            return;
        }

        for index in (end_index + 1..self.array.len()).rev() {
            self.array[index] = self.array[index - 1].clone();
        }

        self.array[end_index] = self.none.clone();
    }

    fn shift_downwards(&mut self, start_index: usize) {
        let size = self.size;
        if start_index >= size {
            return;
        }

        for index in start_index..size - 1 {
            self.array[index] = self.array[index + 1].clone();
        }

        self.array[size - 1] = self.none.clone();
    }

    fn restructure(&mut self, py: Python) {
        let capacity = self.capacity * 2;
        let mut new_vector = vec![self.none.clone(); capacity];

        for index in 0..self.size {
            new_vector[index] = self.array[index].clone_ref(py);
        }

        self.array = new_vector;
        self.capacity = capacity;
    }
}

#[pymethods]
impl TinySet {
    #[new]
    pub fn new(py: Python, capacity: Option<usize>, threshold: Option<f64>) -> Self {
        let cap = capacity.unwrap_or(128);
        let factor = threshold.unwrap_or(80.0);
        let none = py.None();
        Self {
            capacity: cap,
            size: 0,
            none: none.clone(),
            threshold: factor,
            array: vec![none; cap],
        }
    }

    pub fn add(&mut self, py: Python, value: PyObject) -> PyResult<bool> {
        // Grow if necessary method.
        let load_factor = self.percentage()?;
        if load_factor >= self.threshold {
            self.restructure(py);
        }

        // The starting index for iteration.
        let mut current_index: usize = 0;

        // Iterate through the internal array.
        for index in 0..self.array.len() {
            // Get reference to the value-object.
            let item = self.array[index].as_ref(py);

            // If value == item -> Duplicate value so terminate.
            if value.as_ref(py).eq(item)? {
                return Ok(false);
            // If value < item -> Arrived at correct index for insertion.
            } else if value.as_ref(py).lt(item)? {
                // Get the index & terminate loop.
                current_index = index;
                break;
            }
        }

        // Shift all value upwards & return.
        self.shift_upwards(current_index);
        self.array[current_index] = value;
        self.size += 1;
        Ok(true)
    }

    pub fn pop(&mut self, py: Python) -> PyResult<PyObject> {
        // Remove the current value at index 0.
        let removed_value = self.array[0].clone_ref(py);

        // Shift all values in internal array downwards & return removed value.
        self.shift_downwards(0);
        self.size -= 1;
        Ok(removed_value)
    }

    pub fn remove(&mut self, py: Python, value: PyObject) -> PyResult<PyObject> {
        // Get a reference to the value-object.
        let value_ref = value.as_ref(py);
        
        // Iterate over internal array.
        for index in 0..self.size {
            // retrieve a reference to the internal value at index.
            let item = self.array[index].as_ref(py);
            // If value == item -> Remove the value and shift downwards.
            if value_ref.eq(item)? {
                let removed_value = self.array[index].clone_ref(py);
                self.shift_downwards(index);
                self.size -= 1;
                return Ok(removed_value);

            // If value < item -> Break loop iteration.
            } else if value_ref.lt(item)? {
                break;
            }
        }
        // DEFAULT = No value found in the TinySet structure.
        return Err(PyValueError::new_err(
            format!("Value: {} not found in TinySet!", value)
        ));
    }

    pub fn contains(&self, py: Python, target: PyObject) -> PyResult<bool> {
        // Binary Search -> Set 'left' & 'right' variables.
        let mut left: usize = 0;
        let mut right: usize = match self.size.checked_sub(1) {
            Some(r) => r,
            None => return Err(PyValueError::new_err("TinySet is currently empty! No entries to search through."))
        };

        // Continue to loop while 'left' & 'right' are distinct.
        while left <= right {
            // Get middle index.
            let mid = left + (right - left) / 2;

            // Retrieve the Object at middle index.
            let mid_obj = &self.array[mid];

            // If the mid-object equals the traget -> Return True.
            // If the mid-object is greater -> Search upper part of array.
            // If the mid-object is lesser -> Search lower part of array.
            if mid_obj.as_ref(py).eq(target.as_ref(py))? {
                return Ok(true);
            } else if mid_obj.as_ref(py).gt(target.as_ref(py))? {
                right = mid.checked_sub(1).unwrap_or(0);
            } else {
                left = mid + 1;
            }
        }
        // DEFAULT = Target not found, so return False.
        Ok(false)
    }

    pub fn get(&self, py: Python, index: usize) -> PyResult<PyObject> {
        // Retrieve internal array size.
        let size = self.size;
        // Raise Error if the index is out of bounds.
        if index >= size {
            return Err(PyValueError::new_err(
                format!("Index out of bounds! TinySet currently holds {} entries", size)
            ));
        }
        // Return a copy of the value at index.
        Ok(self.array[index].clone_ref(py))
    }

    pub fn range<'py>(&self, py: Python<'py>, start: usize, end: usize) -> PyResult<&'py PyList> {
        // Raise Error if any of the indexes are out of bounds!
        let size = self.size;
        if start > size || end > size {
            return Err(PyValueError::new_err(
                format!("Index out of bounds! TinySet currently holds {} entries", size)
            ));
        }

        // Create new storage Vectors.
        let mut elements = Vec::new();

        // Iterate through & clone values.
        for index in start..=end {
            elements.push(self.array[index].clone_ref(py));
        }

        // Convert to Python-native list and return.
        Ok(PyList::new(py, elements))
    }

    pub fn first(&self, py: Python) -> PyResult<PyObject> {
        // If the current array is empty -> Raise ValueError.
        if self.is_empty()? {
            return Err(PyValueError::new_err(
                format!("TinySet is currently empty! No values to inspect")
            ));
        }

        // Return the value at index 0.
        Ok(self.array[0].clone_ref(py))
    }

    pub fn last(&self, py: Python) -> PyResult<PyObject> {
        // If the current array is empty -> Raise ValueError.
        if self.is_empty()? {
            return Err(PyValueError::new_err(
                format!("TinySet is currently empty! No values to inspect")
            ));
        }

        // Return the value at final array index.
        Ok(self.array[self.size - 1].clone_ref(py))
    }

    pub fn values<'py>(&self, py: Python<'py>) -> PyResult<&'py PyList> {
        // Initiate new Rust Vectors to stores values.
        let mut elements = Vec::new();

        // Iterate through internal array & clone values.
        for i in 0..=self.size {
            elements.push(self.array[i].clone_ref(py));
        }

        // Convert & return the new PyList instance. 
        Ok(PyList::new(py, elements))
    }

    pub fn copy(&self, py: Python<'_>) -> PyResult<PyObject> {
        // Instantiate new new TinySet-class.
        let mut new_set = TinySet::new(py, Some(self.capacity), Some(self.threshold));

        // Iterate through the entire list and add values.
        for index in 0..self.size - 1 {
            let _ = new_set.add(py, self.array[index].clone_ref(py));
        }

        // Convert the TinySet into a Python-native object. 
        Ok(Py::new(py, new_set)?.into_py(py))
    }

    pub fn info<'py>(&self, py: Python<'py>) -> PyResult<&'py PyDict> {
        // Extract the necessary metrics from internal variables
        let percentage = self.percentage()?;
        let values = self.values(py)?.into();

        // Contruct a Rust Vector consisting of individual Tuples(String, Object).
        let key_vals: Vec<(&str, PyObject)> = vec![
            ("type", "TinySet".to_object(py)),
            ("capacity", self.capacity.to_object(py)),
            ("size", self.size.to_object(py)),
            ("percentage", percentage.to_object(py)),
            ("values", values),
        ];

        // Convert Vector to Python Dictionary and return value.
        let dict = key_vals.into_py_dict(py);
        Ok(dict)
    }

    pub fn capacity(&self) -> PyResult<usize> {
        // Return the maximum number of possible entries in internal array. 
        Ok(self.capacity)
    }

    pub fn size(&self) -> PyResult<usize> {
        // Return the current number of entries present in internal array.
        Ok(self.size)
    }

    pub fn percentage(&self) -> PyResult<f64> {
        // Returns a floating-point number indicating the percentage of internal space occupied.
        let percent = (self.size as f64 / self.capacity as f64) * 100.0;
        Ok(percent)
    }

    pub fn is_empty(&self) -> PyResult<bool> {
        // Return True if there currently is 0 entries in internal array.
        Ok(self.size <= 0)
    }

    pub fn clear(&mut self, py: Python) -> PyResult<()> {
        // Reset all internal variables & vectors.
        self.size = 0;
        self.array = vec![py.None(); self.capacity];
        Ok(())
    }
}