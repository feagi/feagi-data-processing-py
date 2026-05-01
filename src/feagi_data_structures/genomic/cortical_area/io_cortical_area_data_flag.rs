use crate::py_error::PyFeagiError;
use crate::{__base_py_class_shared, wrap_flat_enum, wrap_layered_enum};
use feagi_data_structures::genomic::cortical_area::io_cortical_area_configuration_flag::{
    FrameChangeHandling, IOCorticalAreaConfigurationFlag, PercentageNeuronPositioning,
};
use pyo3::prelude::*;
use pyo3::{pyclass, pymethods, PyResult};

//region IOCorticalAreaConfigurationFlag

wrap_layered_enum!(
    PyIOCorticalAreaConfigurationFlag,
    IOCorticalAreaConfigurationFlag,
    "IOCorticalAreaConfigurationFlag"
);

#[pymethods]
#[allow(non_snake_case)]
impl PyIOCorticalAreaConfigurationFlag {
    //region Constructors

    /// Create a Boolean data flag (for on/off signals).
    #[staticmethod]
    pub fn Boolean() -> Self {
        PyIOCorticalAreaConfigurationFlag {
            inner: IOCorticalAreaConfigurationFlag::Boolean,
        }
    }

    /// Create a Percentage data flag (single 0-1 value).
    #[staticmethod]
    pub fn Percentage(
        frame_change_handling: PyFrameChangeHandling,
        percentage_neuron_positioning: PyPercentageNeuronPositioning,
    ) -> Self {
        PyIOCorticalAreaConfigurationFlag {
            inner: IOCorticalAreaConfigurationFlag::Percentage(
                frame_change_handling.inner,
                percentage_neuron_positioning.inner,
            ),
        }
    }

    /// Create a Percentage2D data flag (2D percentage values).
    #[staticmethod]
    pub fn Percentage2D(
        frame_change_handling: PyFrameChangeHandling,
        percentage_neuron_positioning: PyPercentageNeuronPositioning,
    ) -> Self {
        PyIOCorticalAreaConfigurationFlag {
            inner: IOCorticalAreaConfigurationFlag::Percentage2D(
                frame_change_handling.inner,
                percentage_neuron_positioning.inner,
            ),
        }
    }

    /// Create a Percentage3D data flag (3D percentage values).
    #[staticmethod]
    pub fn Percentage3D(
        frame_change_handling: PyFrameChangeHandling,
        percentage_neuron_positioning: PyPercentageNeuronPositioning,
    ) -> Self {
        PyIOCorticalAreaConfigurationFlag {
            inner: IOCorticalAreaConfigurationFlag::Percentage3D(
                frame_change_handling.inner,
                percentage_neuron_positioning.inner,
            ),
        }
    }

    /// Create a Percentage4D data flag (4D percentage values).
    #[staticmethod]
    pub fn Percentage4D(
        frame_change_handling: PyFrameChangeHandling,
        percentage_neuron_positioning: PyPercentageNeuronPositioning,
    ) -> Self {
        PyIOCorticalAreaConfigurationFlag {
            inner: IOCorticalAreaConfigurationFlag::Percentage4D(
                frame_change_handling.inner,
                percentage_neuron_positioning.inner,
            ),
        }
    }

    /// Create a SignedPercentage data flag (single -1 to 1 value).
    #[staticmethod]
    pub fn SignedPercentage(
        frame_change_handling: PyFrameChangeHandling,
        percentage_neuron_positioning: PyPercentageNeuronPositioning,
    ) -> Self {
        PyIOCorticalAreaConfigurationFlag {
            inner: IOCorticalAreaConfigurationFlag::SignedPercentage(
                frame_change_handling.inner,
                percentage_neuron_positioning.inner,
            ),
        }
    }

    /// Create a SignedPercentage2D data flag (2D signed percentage values).
    #[staticmethod]
    pub fn SignedPercentage2D(
        frame_change_handling: PyFrameChangeHandling,
        percentage_neuron_positioning: PyPercentageNeuronPositioning,
    ) -> Self {
        PyIOCorticalAreaConfigurationFlag {
            inner: IOCorticalAreaConfigurationFlag::SignedPercentage2D(
                frame_change_handling.inner,
                percentage_neuron_positioning.inner,
            ),
        }
    }

    /// Create a SignedPercentage3D data flag (3D signed percentage values).
    #[staticmethod]
    pub fn SignedPercentage3D(
        frame_change_handling: PyFrameChangeHandling,
        percentage_neuron_positioning: PyPercentageNeuronPositioning,
    ) -> Self {
        PyIOCorticalAreaConfigurationFlag {
            inner: IOCorticalAreaConfigurationFlag::SignedPercentage3D(
                frame_change_handling.inner,
                percentage_neuron_positioning.inner,
            ),
        }
    }

    /// Create a SignedPercentage4D data flag (4D signed percentage values, e.g., quaternion).
    #[staticmethod]
    pub fn SignedPercentage4D(
        frame_change_handling: PyFrameChangeHandling,
        percentage_neuron_positioning: PyPercentageNeuronPositioning,
    ) -> Self {
        PyIOCorticalAreaConfigurationFlag {
            inner: IOCorticalAreaConfigurationFlag::SignedPercentage4D(
                frame_change_handling.inner,
                percentage_neuron_positioning.inner,
            ),
        }
    }

    /// Create a CartesianPlane data flag (for 2D image/grid data).
    #[staticmethod]
    pub fn CartesianPlane(frame_change_handling: PyFrameChangeHandling) -> Self {
        PyIOCorticalAreaConfigurationFlag {
            inner: IOCorticalAreaConfigurationFlag::CartesianPlane(frame_change_handling.inner),
        }
    }

    /// Create a Misc data flag (for miscellaneous/custom data).
    #[staticmethod]
    pub fn Misc(frame_change_handling: PyFrameChangeHandling) -> Self {
        PyIOCorticalAreaConfigurationFlag {
            inner: IOCorticalAreaConfigurationFlag::Misc(frame_change_handling.inner),
        }
    }

    //endregion

    //region Conversion Methods

    /// Convert to a 16-bit data type configuration flag.
    ///
    /// Returns:
    ///     int: The packed configuration flag as a 16-bit unsigned integer.
    pub fn to_data_type_configuration_flag(&self) -> u16 {
        self.inner.to_data_type_configuration_flag()
    }

    /// Create from a 16-bit data type configuration flag.
    ///
    /// Args:
    ///     flag: The 16-bit configuration flag.
    ///
    /// Returns:
    ///     IOCorticalAreaConfigurationFlag: The decoded data flag.
    ///
    /// Raises:
    ///     ValueError: If the flag represents an invalid configuration.
    #[staticmethod]
    pub fn try_from_data_type_configuration_flag(flag: u16) -> PyResult<Self> {
        let data_flag =
            IOCorticalAreaConfigurationFlag::try_from_data_type_configuration_flag(flag)
                .map_err(PyFeagiError::from)?;
        Ok(PyIOCorticalAreaConfigurationFlag { inner: data_flag })
    }

    //endregion
}

//endregion

//region PercentageNeuronPositioning

wrap_flat_enum!(
    PyPercentageNeuronPositioning,
    PercentageNeuronPositioning,
    "PercentageNeuronPositioning"
);

#[pymethods]
#[allow(non_snake_case)]
impl PyPercentageNeuronPositioning {
    /// Create a Linear neuron positioning mode.
    #[staticmethod]
    pub fn Linear() -> Self {
        PyPercentageNeuronPositioning {
            inner: PercentageNeuronPositioning::Linear,
        }
    }

    /// Create a Fractional neuron positioning mode (default).
    #[staticmethod]
    pub fn Fractional() -> Self {
        PyPercentageNeuronPositioning {
            inner: PercentageNeuronPositioning::Fractional,
        }
    }
}

//endregion

//region FrameChangeHandling

wrap_flat_enum!(
    PyFrameChangeHandling,
    FrameChangeHandling,
    "FrameChangeHandling"
);

#[pymethods]
#[allow(non_snake_case)]
impl PyFrameChangeHandling {
    /// Create an Absolute frame change handling mode (default).
    #[staticmethod]
    pub fn Absolute() -> Self {
        PyFrameChangeHandling {
            inner: FrameChangeHandling::Absolute,
        }
    }

    /// Create an Incremental frame change handling mode.
    #[staticmethod]
    pub fn Incremental() -> Self {
        PyFrameChangeHandling {
            inner: FrameChangeHandling::Incremental,
        }
    }
}
//endregion
