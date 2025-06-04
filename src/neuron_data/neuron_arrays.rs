use pyo3::{pyclass, pymethods, PyResult};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use numpy::{PyArray1, PyReadonlyArray1};
use feagi_core_data_structures_and_processing::neuron_data::neuron_arrays::*;
use ndarray::Array1;
use pyo3::types::{PyBytes, PyList, PyTuple};
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

    #[staticmethod]
    pub fn new_from_numpy(x: PyReadonlyArray1<u32>, y: PyReadonlyArray1<u32>, z: PyReadonlyArray1<u32>, p: PyReadonlyArray1<f32>) -> PyResult<Self> {
        let x_nd = x.as_array().to_owned();
        let y_nd = y.as_array().to_owned();
        let z_nd = z.as_array().to_owned();
        let p_nd = p.as_array().to_owned();
        match NeuronXYZPArrays::new_from_ndarrays(x_nd, y_nd, z_nd, p_nd) {
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

    pub fn copy_as_tuple_of_numpy<'py>(&self, py: Python<'py>) -> PyResult<(Bound<'py, PyArray1<u32>>, Bound<'py, PyArray1<u32>>, Bound<'py, PyArray1<u32>>, Bound<'py, PyArray1<f32>>)> {
        let nd_arrays_tuple = self.inner.copy_as_tuple_of_nd_arrays();
        Ok((
            PyArray1::from_array(py, &nd_arrays_tuple.0),
            PyArray1::from_array(py, &nd_arrays_tuple.1),
            PyArray1::from_array(py, &nd_arrays_tuple.2),
            PyArray1::from_array(py, &nd_arrays_tuple.3)
        ))
    }

    pub fn copy_as_neuron_xyzp_vec<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let items = self.inner.copy_as_neuron_xyzp_vec();

        let py_objects: Vec<PyObject> = items
            .into_iter()
            .map(|item| Py::new(py, PyNeuronXYZP{inner: item}).map(|obj| obj.into()))
            .collect::<PyResult<_>>()?;
        
        PyList::new(py, py_objects)
    }
}

pub fn tuple_nd_array_to_tuple_np_array<'py>(input: (Array1<u32>, Array1<u32>, Array1<u32>, Array1<f32>), py: Python<'py>)
    -> PyResult<(Bound<'py, PyArray1<u32>>, Bound<'py, PyArray1<u32>>, Bound<'py, PyArray1<u32>>, Bound<'py, PyArray1<f32>>)> {

    Ok((
        PyArray1::from_array(py, &input.0),
        PyArray1::from_array(py, &input.1),
        PyArray1::from_array(py, &input.2),
        PyArray1::from_array(py, &input.3)
    ))
    }