use crate::{__base_py_class_shared, wrap_flat_enum};
use feagi_serialization::FeagiByteStructureType;
use pyo3::prelude::*;
use pyo3::{pyclass, pymethods};

wrap_flat_enum!(
    PyFeagiByteStructureType,
    FeagiByteStructureType,
    "FeagiByteStructureType"
);

#[pymethods]
#[allow(non_snake_case)]
impl PyFeagiByteStructureType {
    #[staticmethod]
    pub fn JSON() -> Self {
        PyFeagiByteStructureType {
            inner: FeagiByteStructureType::JSON,
        }
    }

    #[staticmethod]
    pub fn NeuronCategoricalXYZP() -> Self {
        PyFeagiByteStructureType {
            inner: FeagiByteStructureType::NeuronCategoricalXYZP,
        }
    }
}
