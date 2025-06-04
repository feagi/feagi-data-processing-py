use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::types::PyBytes;
use feagi_core_data_structures_and_processing::byte_structures::FeagiByteStructureType;
use feagi_core_data_structures_and_processing::byte_structures::feagi_byte_structure::FeagiByteStructure;

#[pyclass]
#[pyo3(name = "FeagiByteStructure")]
#[derive(Clone)]
pub struct PyFeagiByteStructure{
    pub inner: FeagiByteStructure,
}

#[pymethods]
impl PyFeagiByteStructure {
    
    /// Create a new FeagiByteStructure from bytes
    #[new]
    pub fn new<'py>(py: Python<'py>, bytes: Bound<'py, PyBytes>) -> PyResult<Self> {
        let bytes_vec = bytes.as_bytes().to_vec();
        match FeagiByteStructure::create_from_bytes(bytes_vec) {
            Ok(inner) => Ok(PyFeagiByteStructure { inner }),
            Err(e) => Err(PyValueError::new_err(format!("{:?}", e))),
        }
    }
    
    /// Try to get the structure type
    pub fn try_get_structure_type(&self) -> PyResult<u8> {
        match self.inner.try_get_structure_type() {
            Ok(structure_type) => Ok(structure_type as u8),
            Err(e) => Err(PyValueError::new_err(format!("{:?}", e))),
        }
    }
    
    /// Get the version number
    pub fn get_version(&self) -> u8 {
        self.inner.get_version()
    }
    
    /// Get the data as a bytes copy
    pub fn get_data_as_bytes(&self) -> Vec<u8> {
        self.inner.borrow_data_as_slice().to_vec()
    }
    
    /// Ensure capacity of at least the specified size
    pub fn ensure_capacity_of_at_least(&mut self, size: usize) -> PyResult<()> {
        match self.inner.ensure_capacity_of_at_least(size) {
            Ok(()) => Ok(()),
            Err(e) => Err(PyValueError::new_err(format!("{:?}", e))),
        }
    }
    
    /// Shed wasted capacity to free up memory
    pub fn shed_wasted_capacity(&mut self) {
        self.inner.shed_wasted_capacity();
    }
    
    /// Reset write index (truncate to 0 length)
    pub fn reset_write_index(&mut self) {
        self.inner.reset_write_index();
    }
    
    /// Get count of wasted capacity
    pub fn get_wasted_capacity_count(&self) -> usize {
        self.inner.get_wasted_capacity_count()
    }
    
    /// Get utilized capacity as a percentage
    pub fn get_utilized_capacity_percentage(&self) -> f32 {
        self.inner.get_utilized_capacity_percentage()
    }

    // Return as copy of Python Bytes
    pub fn copy_as_bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        Ok(PyBytes::new(py, self.inner.borrow_data_as_slice()))
    }

    /*
    /// String representation for debugging
    pub fn __repr__(&self) -> String {
        format!(
            "FeagiByteStructure(type={}, version={}, len={}, capacity={})",
            self.try_get_structure_type().unwrap_or(0),
            self.get_version(),
            self.len(),
            self.capacity()
        )
    }
    
    /// String representation
    pub fn __str__(&self) -> String {
        self.__repr__()
    }

     */
}
