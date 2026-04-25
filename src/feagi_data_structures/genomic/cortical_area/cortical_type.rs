use crate::feagi_data_structures::genomic::cortical_area::PyCorticalID;
use crate::{__base_py_class_shared, wrap_flat_enum, wrap_layered_enum};
use feagi_data_structures::genomic::cortical_area::{
    CoreCorticalType, CorticalAreaType, CustomCorticalType, MemoryCorticalType,
};
use pyo3::prelude::*;
use pyo3::{pyclass, pymethods};

wrap_layered_enum!(PyCorticalAreaType, CorticalAreaType, "CorticalAreaType");

#[pymethods]
#[allow(non_snake_case)]
impl PyCorticalAreaType {
    #[staticmethod]
    pub fn Core(core_cortical_type: PyCoreCorticalType) -> Self {
        PyCorticalAreaType {
            inner: CorticalAreaType::Core(core_cortical_type.into()),
        }
    }

    #[staticmethod]
    pub fn Custom(custom_cortical_type: PyCustomCorticalType) -> Self {
        PyCorticalAreaType {
            inner: CorticalAreaType::Custom(custom_cortical_type.into()),
        }
    }

    #[staticmethod]
    pub fn Memory(memory_cortical_type: PyMemoryCorticalType) -> Self {
        PyCorticalAreaType {
            inner: CorticalAreaType::Memory(memory_cortical_type.into()),
        }
    }
}

//region Core
wrap_flat_enum!(PyCoreCorticalType, CoreCorticalType, "CoreCorticalType");

#[pymethods]
#[allow(non_snake_case)]
impl PyCoreCorticalType {
    #[staticmethod]
    pub fn Death() -> Self {
        PyCoreCorticalType {
            inner: CoreCorticalType::Death,
        }
    }

    #[staticmethod]
    pub fn Power() -> Self {
        PyCoreCorticalType {
            inner: CoreCorticalType::Power,
        }
    }

    pub fn to_cortical_id(&self) -> PyCorticalID {
        self.inner.to_cortical_id().into()
    }
}

//endregion

//region Custom

wrap_flat_enum!(
    PyCustomCorticalType,
    CustomCorticalType,
    "CustomCorticalType"
);

#[pymethods]
#[allow(non_snake_case)]
impl PyCustomCorticalType {
    #[staticmethod]
    pub fn LeakyIntegrateFire() -> Self {
        PyCustomCorticalType {
            inner: CustomCorticalType::LeakyIntegrateFire,
        }
    }
}

//endregion

//region Memory

wrap_flat_enum!(
    PyMemoryCorticalType,
    MemoryCorticalType,
    "MemoryCorticalType"
);

#[pymethods]
#[allow(non_snake_case)]
impl PyMemoryCorticalType {
    #[staticmethod]
    pub fn Memory() -> Self {
        PyMemoryCorticalType {
            inner: MemoryCorticalType::Memory,
        }
    }
}

//endregion
