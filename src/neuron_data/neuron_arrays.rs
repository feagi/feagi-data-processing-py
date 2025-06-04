use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use feagi_core_data_structures_and_processing::neuron_data::neuron_arrays::*;
use pyo3::types::{PyBytes, PyList};
use super::neurons::PyNeuronXYZP;


#[pyclass]
#[derive(Clone)]
#[pyo3(name = "NeuronXYZPArrays")]
pub struct PyNeuronXYZPArrays {
    pub inner: NeuronXYZPArrays,
}

#[pymethods]
impl PyNeuronXYZPArrays {
    #[new]
    pub fn new(maximum_number_of_neurons_possibly_needed: usize) -> PyResult<Self> {
        let result = NeuronXYZPArrays::new(maximum_number_of_neurons_possibly_needed);
        match result {
            Ok(inner) => Ok(PyNeuronXYZPArrays {inner}),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    #[staticmethod]
    pub fn new_from_resolution(resolution: (usize, usize, usize))  -> PyResult<Self> {
        let result = NeuronXYZPArrays::new_from_resolution(resolution);
        match result {
            Ok(inner) => Ok(PyNeuronXYZPArrays {inner}),
            Err(e) => Err(PyValueError::new_err(e.to_string()))
        }
    }

    pub fn get_max_neuron_capacity_without_reallocating(&self) -> PyResult<usize> {
        let result = self.inner.get_max_neuron_capacity_without_reallocating();
        Ok(result)
    }

    pub fn expand_to_new_max_count_if_required(&mut self, new_max_neuron_count: usize) -> PyResult<()> {
        self.inner.expand_to_new_max_count_if_required(new_max_neuron_count);
        Ok(())
    }

    pub fn reset_indexes(&mut self) -> PyResult<()> {
        self.inner.reset_indexes();
        Ok(())
    }

    pub fn get_number_of_neurons_used(&self) -> PyResult<usize> {
        let result = self.inner.get_number_of_neurons_used();
        Ok(result)
    }

    pub fn add_neuron(&mut self, neuron: PyNeuronXYZP) -> PyResult<()> {
        self.inner.add_neuron(&neuron.inner);
        Ok(())
    }

    pub fn copy_as_neuron_xyzp_vec<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let items = self.inner.copy_as_neuron_xyzp_vec();

        let py_objects: Vec<PyObject> = items
            .into_iter()
            .map(|item| Py::new(py, PyNeuronXYZP{inner: item}).map(|obj| obj.into()))
            .collect::<PyResult<_>>()?;
        
        PyList::new(py, py_objects)
    }


    // TODO add a way to copy data out

}

