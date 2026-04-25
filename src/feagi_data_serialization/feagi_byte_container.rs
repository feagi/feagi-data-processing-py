use crate::feagi_data_structures::neurons_voxels::xyzp::PyCorticalMappedXYZPNeuronVoxels;
use crate::py_error::PyFeagiError;
use feagi_data_structures::neuron_voxels::xyzp::CorticalMappedXYZPNeuronVoxels;
use feagi_serialization::{FeagiByteContainer, FeagiSerializable};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::pyclass;
use pyo3::types::PyBytes;
//create_pyclass!(PyFeagiByteContainer, FeagiByteContainer, "FeagiByteContainer");

#[pyclass]
#[derive(Clone)]
#[pyo3(name = "FeagiByteContainer")]
pub struct PyFeagiByteContainer {
    pub(crate) inner: FeagiByteContainer,
}

#[pymethods]
impl PyFeagiByteContainer {
    fn as_any<'py>(slf: Bound<'py, Self>) -> Py<PyAny> {
        slf.unbind().into_any()
    }

    //region Constructors

    #[new]
    pub fn new() -> Self {
        PyFeagiByteContainer {
            inner: FeagiByteContainer::new_empty(),
        }
    }

    //endregion

    //region Direct Data Access
    // some differences here since references don't apply in python

    pub fn copy_out_as_byte_vector<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        Ok(PyBytes::new(py, self.inner.get_byte_ref()))
    }

    pub fn load_bytes_and_verify<'py>(
        &mut self,
        _py: Python<'py>,
        bytes: Bound<'py, PyBytes>,
    ) -> PyResult<()> {
        let byte_arr: Vec<u8> = bytes.as_bytes().to_vec();
        self.inner
            .try_write_data_by_ownership_to_container_and_verify(byte_arr)
            .map_err(PyFeagiError::from)?;
        Ok(())
    }

    //endregion

    //region Get Properties

    #[getter]
    pub fn valid(&self) -> bool {
        self.inner.is_valid()
    }

    #[getter]
    pub fn number_contained_structures(&self) -> PyResult<usize> {
        Ok(self
            .inner
            .try_get_number_contained_structures()
            .map_err(PyFeagiError::from)?)
    }

    #[getter]
    pub fn number_of_bytes_used(&self) -> usize {
        self.inner.get_number_of_bytes_used()
    }

    #[getter]
    pub fn number_of_bytes_allocated(&self) -> usize {
        self.inner.get_number_of_bytes_allocated()
    }

    #[getter]
    pub fn increment_counter(&self) -> PyResult<u16> {
        Ok(self
            .inner
            .get_increment_counter()
            .map_err(PyFeagiError::from)?)
    }

    //endregion

    //region Extracting Struct Data

    pub fn try_create_new_struct_from_index(&self, py: Python, index: u8) -> PyResult<Py<PyAny>> {
        let feagi_serializable = self
            .inner
            .try_create_new_struct_from_index(index)
            .map_err(PyFeagiError::from)?;
        let voxels: CorticalMappedXYZPNeuronVoxels =
            feagi_serializable.try_into().map_err(PyFeagiError::from)?; // TODO support other types too!
        let py_voxels = PyCorticalMappedXYZPNeuronVoxels::python_etc_child_constructor(py, voxels)?;
        Ok(py_voxels.into_any())
    }

    //endregion

    //region Adding Struct Data

    /// Add a serializable structure to the container.
    ///
    /// This overwrites any existing data in the container and wraps the structure
    /// in a FeagiByteContainer with version 2 header.
    ///
    /// Args:
    ///     struct_obj: A PyFeagiSerializable object (e.g., CorticalMappedXYZPNeuronVoxels)
    ///     increment_value: Optional increment counter value (defaults to 0)
    ///
    /// Returns:
    ///     None on success
    pub fn add_struct(
        &mut self,
        struct_obj: &Bound<PyAny>,
        increment_value: Option<u16>,
    ) -> PyResult<()> {
        // Try to cast to PyCorticalMappedXYZPNeuronVoxels
        if let Ok(py_voxels) = struct_obj.cast::<PyCorticalMappedXYZPNeuronVoxels>() {
            // Get a reference to the inner structure
            let voxels_ref: &dyn FeagiSerializable = &py_voxels.borrow().inner;
            self.inner
                .overwrite_byte_data_with_multiple_struct_data(
                    vec![voxels_ref],
                    increment_value.unwrap_or(0),
                )
                .map_err(PyFeagiError::from)?;
            Ok(())
        } else {
            Err(PyErr::new::<PyValueError, _>(
                "Unsupported structure type. Currently only CorticalMappedXYZPNeuronVoxels is supported."
            ))
        }
    }

    //endregion
}
