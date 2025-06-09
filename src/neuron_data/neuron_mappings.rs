use feagi_core_data_structures_and_processing::byte_structures::{FeagiByteStructureCompatible, FeagiByteStructureType};
use pyo3::{pyclass, pymethods, PyResult, Py};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use numpy::PyArray1;
use feagi_core_data_structures_and_processing::neuron_data::neuron_mappings::*;
use crate::byte_structures::feagi_byte_structure::PyFeagiByteStructure;
use crate::byte_structures::{PyFeagiByteStructureCompatible, PyFeagiByteStructureType};
use crate::cortical_data::{PyCorticalID};
use super::neuron_arrays::{PyNeuronXYZPArrays, tuple_nd_array_to_tuple_np_array};

#[pyclass(extends=PyFeagiByteStructureCompatible)]
#[derive(Clone)]
#[pyo3(name = "CorticalMappedXYZPNeuronData")]
pub struct PyCorticalMappedXYZPNeuronData { // HashMap<CorticalID, NeuronYXCPArrays>
    pub inner: CorticalMappedXYZPNeuronData
}

#[pymethods]
impl PyCorticalMappedXYZPNeuronData {
    
    //region Definitions for base class

    #[getter]
    pub fn struct_type(&self) -> PyFeagiByteStructureType {
        PyFeagiByteStructureType::NeuronCategoricalXYZP
    }

    pub fn version(&self) -> u8 { self.inner.get_version() } // This is a overridden placeholder

    #[staticmethod]
    pub fn new_from_feagi_byte_structure(byte_structure: PyFeagiByteStructure) -> PyResult<PyCorticalMappedXYZPNeuronData> where Self: Sized {
        let result = CorticalMappedXYZPNeuronData::new_from_feagi_byte_structure(&byte_structure.inner);
        match result {
            Ok(inner) => Ok(PyCorticalMappedXYZPNeuronData { inner }),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    pub fn as_new_feagi_byte_structure(&self) -> PyResult<PyFeagiByteStructure> {
        let result = self.inner.as_new_feagi_byte_structure();
        match result {
            Ok(result) => Ok(PyFeagiByteStructure { inner: result }),
            Err(error) => Err(PyValueError::new_err(error.to_string())),
        }
    }
    
    //endregion
    
    
    #[new]
    pub fn new() -> (PyCorticalMappedXYZPNeuronData, PyFeagiByteStructureCompatible) {
        (
            PyCorticalMappedXYZPNeuronData {
                inner: CorticalMappedXYZPNeuronData::new()
            },
            PyFeagiByteStructureCompatible::new()
        )
    }

    pub fn insert(&mut self, cortical_id: PyCorticalID, data: PyNeuronXYZPArrays) -> PyResult<()> {
        if self.inner.contains(cortical_id.inner.clone()) { // TODO fix clone
            return Err(PyValueError::new_err(format!("Cortical ID of {} already exists in this CorticalMappedNeuronData object!", cortical_id.as_str())));
        }
        self.inner.insert(cortical_id.inner, data.inner);
        Ok(())
    }

    pub fn contains(&self, cortical_id: PyCorticalID) -> PyResult<bool> {
        Ok(self.inner.contains(cortical_id.inner))
    }
    
    pub fn get(&self, cortical_id: PyCorticalID) -> PyResult<PyNeuronXYZPArrays> {
        let result = self.inner.borrow(&cortical_id.inner);
        if result.is_none() {
            return Err(PyValueError::new_err(format!("Cortical ID {} does not have a mapping to neuron XYZP data!", cortical_id.as_str()))); 
        }
        Ok(PyNeuronXYZPArrays {inner: result.unwrap().clone()})
    }
    
    fn __iter__(&self) -> PyResult<PyCorticalMappedXYZPNeuronDataFullIter> {
        let items: Vec<(PyCorticalID, PyNeuronXYZPArrays)> = self
            .inner
            .mappings
            .iter()
            .map(|(k, v)| (PyCorticalID { inner: k.clone() }, PyNeuronXYZPArrays { inner: v.clone() }))
            .collect();
        Ok(PyCorticalMappedXYZPNeuronDataFullIter { items, index: 0 })
    }

    fn iter_easy(&self, py: Python<'_>) -> PyResult<PyCorticalMappedXYZPNeuronDataEasyIter> {
        let mut items: Vec<(String, (Py<PyArray1<u32>>, Py<PyArray1<u32>>, Py<PyArray1<u32>>, Py<PyArray1<f32>>))> = Vec::new();
        
        for (k, v) in self.inner.mappings.iter() {
            let cortical_id_str = k.as_str().to_string();
            let nd_arrays = v.copy_as_tuple_of_nd_arrays();
            let bound_arrays = tuple_nd_array_to_tuple_np_array(nd_arrays, py)?;
            let np_arrays = (
                bound_arrays.0.unbind(),
                bound_arrays.1.unbind(),
                bound_arrays.2.unbind(),
                bound_arrays.3.unbind(),
            );
            items.push((cortical_id_str, np_arrays));
        }
        
        Ok(PyCorticalMappedXYZPNeuronDataEasyIter { items, index: 0 })
    }
    
    

}


#[pyclass]
pub struct PyCorticalMappedXYZPNeuronDataFullIter {
    items: Vec<(PyCorticalID, PyNeuronXYZPArrays)>,
    index: usize,
}

#[pymethods]
impl PyCorticalMappedXYZPNeuronDataFullIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self) -> Option<(PyCorticalID, PyNeuronXYZPArrays)> {
        if self.index >= self.items.len() {
            None
        } else {
            let pair = self.items[self.index].clone();
            self.index += 1;
            Some(pair)
        }
    }
}

#[pyclass]
pub struct PyCorticalMappedXYZPNeuronDataEasyIter {
    items: Vec<(String, (Py<PyArray1<u32>>, Py<PyArray1<u32>>, Py<PyArray1<u32>>, Py<PyArray1<f32>>))>,
    index: usize,
}

#[pymethods]
impl PyCorticalMappedXYZPNeuronDataEasyIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self) -> Option<(String, (Py<PyArray1<u32>>, Py<PyArray1<u32>>, Py<PyArray1<u32>>, Py<PyArray1<f32>>))> {
        if self.index >= self.items.len() {
            None
        } else {
            let pair = self.items.swap_remove(self.index);
            Some(pair)
        }
    }
}
    
