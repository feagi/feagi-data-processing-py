use pyo3::{pyclass, pymethods, PyResult, PyObject};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyBytes, PyList};
use feagi_core_data_structures_and_processing::byte_structures::feagi_byte_structure::FeagiByteStructure;
use feagi_core_data_structures_and_processing::byte_structures::{FeagiByteStructureCompatible, FeagiByteStructureType};
use crate::byte_structures::{PyFeagiByteStructureCompatible, PyFeagiByteStructureType as PyFBSType};
use crate::neuron_data::neuron_mappings::PyCorticalMappedXYZPNeuronData;

/// Helper function to convert a Box<dyn FeagiByteStructureCompatible> to the appropriate Python object
pub fn convert_compatible_to_python(py: Python, boxed_object: Box<dyn FeagiByteStructureCompatible>, structure_type: FeagiByteStructureType) -> PyResult<PyObject> {
    match structure_type {
        FeagiByteStructureType::JSON => {
            // Assuming you have a PyJsonStructure wrapper
            // let json_struct = boxed_object.downcast::<JsonStructure>().map_err(|_| PyValueError::new_err("Failed to downcast to JsonStructure"))?;
            // Ok(PyJsonStructure { inner: *json_struct }.into_py(py))
            Err(PyValueError::new_err("JSON structure conversion not yet implemented"))
        },
        FeagiByteStructureType::NeuronCategoricalXYZP => {
            // Convert the boxed trait object back to concrete type
            // We'll create it from a byte structure instead
            let temp_byte_struct = boxed_object.as_new_feagi_byte_structure().map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;
            let py_byte_struct = PyFeagiByteStructure { inner: temp_byte_struct };
            let neuron_data = PyCorticalMappedXYZPNeuronData::new_from_feagi_byte_structure(py_byte_struct)?;
            let parent = PyFeagiByteStructureCompatible::new();
            let py_obj = Py::new(py, (neuron_data, parent))?;
            Ok(py_obj.into())
        },
        FeagiByteStructureType::MultiStructHolder => {
            Err(PyValueError::new_err("Cannot convert multistruct holder to single compatible object"))
        },
        _ => {
            Err(PyValueError::new_err(format!("Unsupported structure type for conversion: {:?}", structure_type)))
        }
    }
}

#[pyclass]
#[pyo3(name = "FeagiByteStructure")]
#[derive(Clone)]
pub struct PyFeagiByteStructure{
    pub inner: FeagiByteStructure,
}

#[pymethods]
impl PyFeagiByteStructure {
    
    //region Constructors
    /// Create a new FeagiByteStructure from bytes
    #[new]
    pub fn create_from_bytes<'py>(py: Python<'py>, bytes: Bound<'py, PyBytes>) -> PyResult<Self> {
        let bytes_vec = bytes.as_bytes().to_vec();
        match FeagiByteStructure::create_from_bytes(bytes_vec) {
            Ok(inner) => Ok(PyFeagiByteStructure { inner }),
            Err(e) => Err(PyValueError::new_err(format!("{:?}", e))),
        }
    }
    
    #[staticmethod]
    pub fn create_from_2_existing(a: PyFeagiByteStructure, b: PyFeagiByteStructure) -> PyResult<Self> {
        let result = FeagiByteStructure::create_from_2_existing(&a.inner, &b.inner);
        match result {
            Ok(inner) => Ok(PyFeagiByteStructure { inner }),
            Err(e) => Err(PyValueError::new_err(format!("{:?}", e))),
        }
    }
    
    #[staticmethod]
    pub fn create_from_multiple_existing(existing_list: Bound<'_, PyList>) -> PyResult<Self> {
        // First collect all the borrowed items to keep them alive
        let borrowed_items: Vec<_> = existing_list
            .iter()
            .map(|item| {
                let bound_item = item.downcast::<PyFeagiByteStructure>().unwrap();
                bound_item.borrow()
            })
            .collect();

        // Now create references from the borrowed items
        let rust_structs: Vec<&FeagiByteStructure> = borrowed_items
            .iter()
            .map(|borrowed| &borrowed.inner)
            .collect();

        let result = FeagiByteStructure::create_from_multiple_existing(rust_structs);
        match result {
            Ok(inner) => Ok(PyFeagiByteStructure { inner }),
            Err(e) => Err(PyValueError::new_err(format!("{:?}", e))),
        }
    }
    
    #[staticmethod]
    pub fn create_from_compatible(object: Bound<'_, PyFeagiByteStructureCompatible>) -> PyResult<Self> {
        object.borrow().as_new_feagi_byte_structure()
    }

    //endregion
    
    //region Get Properties
    
    #[getter]
    pub fn structure_type(&self) -> PyResult<PyFBSType> {
        let result = self.inner.try_get_structure_type();
        match result {
            Ok(inner) => Ok(PyFBSType::from_base(inner)),
            Err(e) => Err(PyValueError::new_err(format!("{:?}", e))),
        }
    }
    
    #[getter]
    pub fn version(&self) -> PyResult<u8> {
        let result = self.inner.try_get_version();
        match result {
            Ok(inner) => Ok(inner),
            Err(e) => Err(PyValueError::new_err(format!("{:?}", e))),
        }
    }
    
    #[getter]
    pub fn is_multistruct(&self) -> PyResult<bool> {
        let result = self.inner.is_multistruct();
        match result {
            Ok(inner) => Ok(inner),
            Err(e) => Err(PyValueError::new_err(format!("{:?}", e))),
        }
    }

    #[getter]
    pub fn contained_structure_count(&self) -> PyResult<usize> {
        let result = self.inner.contained_structure_count();
        match result {
            Ok(inner) => Ok(inner),
            Err(e) => Err(PyValueError::new_err(format!("{:?}", e))),
        }
    }
    
    pub fn get_ordered_object_types<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
         let result = self.inner.get_ordered_object_types();
         match result {
             Ok(types) => {
                 let py_types: Vec<PyFBSType> = types
                     .into_iter()
                     .map(|t| PyFBSType::from_base(t))
                     .collect();
                 PyList::new(py, py_types)
             },
             Err(e) => Err(PyValueError::new_err(format!("{:?}", e))),
         }
     }
    
    pub fn copy_out_single_byte_structure_from_multistruct(&self, index: usize) -> PyResult<PyFeagiByteStructure> {
        let result = self.inner.copy_out_single_byte_structure_from_multistruct(index);
        match result {
            Ok(inner_struct) => Ok(PyFeagiByteStructure { inner: inner_struct }),
            Err(e) => Err(PyValueError::new_err(format!("{:?}", e))),
        }
    }
    
    pub fn copy_out_single_object_from_single_struct<'py>(&self, py: Python<'py>) -> PyResult<PyObject> {
        let result = self.inner.copy_out_single_object_from_single_struct();
        match result {
            Ok(boxed_object) => {
                let structure_type = self.inner.try_get_structure_type().map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;
                convert_compatible_to_python(py, boxed_object, structure_type)
            },
            Err(e) => Err(PyValueError::new_err(format!("{:?}", e))),
        }
    }
    
    pub fn copy_out_single_object_from_multistruct<'py>(&self, py: Python<'py>, index: usize) -> PyResult<PyObject> {
        let result = self.inner.copy_out_single_object_from_multistruct(index);
        match result {
            Ok(boxed_object) => {
                // For multistruct, we need to get the structure type of the specific index
                let temp_struct = self.inner.copy_out_single_byte_structure_from_multistruct(index).map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;
                let structure_type = temp_struct.try_get_structure_type().map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;
                convert_compatible_to_python(py, boxed_object, structure_type)
            },
            Err(e) => Err(PyValueError::new_err(format!("{:?}", e))),
        }
    }
    
    pub fn copy_out_as_byte_vector<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        Ok(PyBytes::new(py, self.inner.borrow_data_as_slice()))
    }
    
    //endregion

    //region Interactions with Internal Vector

    /// Get count of wasted capacity
    pub fn get_wasted_capacity_count(&self) -> usize {
        self.inner.get_wasted_capacity_count()
    }

    /// Get utilized capacity as a percentage
    pub fn get_utilized_capacity_percentage(&self) -> f32 {
        self.inner.get_utilized_capacity_percentage()
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
    
    //endregion
    

}
