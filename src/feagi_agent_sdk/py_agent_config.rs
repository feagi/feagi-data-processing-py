/*
 * PyO3 wrapper for AgentConfig
 */

use pyo3::prelude::*;
use super::py_agent_type::PyAgentType;

fn parse_sensory_unit(unit: &str) -> PyResult<feagi_io::SensoryUnit> {
    match unit {
        "infrared" => Ok(feagi_io::SensoryUnit::Infrared),
        "proximity" => Ok(feagi_io::SensoryUnit::Proximity),
        "shock" => Ok(feagi_io::SensoryUnit::Shock),
        "battery" => Ok(feagi_io::SensoryUnit::Battery),
        "servo" => Ok(feagi_io::SensoryUnit::Servo),
        "analog_gpio" => Ok(feagi_io::SensoryUnit::AnalogGpio),
        "digital_gpio" => Ok(feagi_io::SensoryUnit::DigitalGpio),
        "misc_data" => Ok(feagi_io::SensoryUnit::MiscData),
        "text_english_input" => Ok(feagi_io::SensoryUnit::TextEnglishInput),
        "count_input" => Ok(feagi_io::SensoryUnit::CountInput),
        "vision" => Ok(feagi_io::SensoryUnit::Vision),
        "segmented_vision" => Ok(feagi_io::SensoryUnit::SegmentedVision),
        "accelerometer" => Ok(feagi_io::SensoryUnit::Accelerometer),
        "gyroscope" => Ok(feagi_io::SensoryUnit::Gyroscope),
        _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Unsupported sensory unit: {}", unit),
        )),
    }
}

fn parse_motor_unit(unit: &str) -> PyResult<feagi_io::MotorUnit> {
    match unit {
        "rotary_motor" => Ok(feagi_io::MotorUnit::RotaryMotor),
        "positional_servo" => Ok(feagi_io::MotorUnit::PositionalServo),
        "gaze" => Ok(feagi_io::MotorUnit::Gaze),
        "misc_data" => Ok(feagi_io::MotorUnit::MiscData),
        "text_english_output" => Ok(feagi_io::MotorUnit::TextEnglishOutput),
        "count_output" => Ok(feagi_io::MotorUnit::CountOutput),
        "object_segmentation" => Ok(feagi_io::MotorUnit::ObjectSegmentation),
        "simple_vision_output" => Ok(feagi_io::MotorUnit::SimpleVisionOutput),
        "dynamic_image_processing" => Ok(feagi_io::MotorUnit::DynamicImageProcessing),
        _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Unsupported motor unit: {}", unit),
        )),
    }
}

#[pyclass(name = "PyAgentConfig")]
#[derive(Clone)]
pub struct PyAgentConfig {
    inner: feagi_agent::AgentConfig,
}

#[pymethods]
impl PyAgentConfig {
    #[new]
    fn new(agent_id: String, agent_type: PyAgentType) -> Self {
        let inner = feagi_agent::AgentConfig::new(agent_id, agent_type.inner());
        PyAgentConfig { inner }
    }
    
    /// Set FEAGI host and ports (required for all endpoints)
    fn with_feagi_endpoints(
        &mut self,
        host: String,
        registration_port: u16,
        sensory_port: u16,
        motor_port: u16,
        visualization_port: u16,
        control_port: u16,
    ) -> PyResult<()> {
        self.inner = self.inner.clone()
            .with_feagi_endpoints(host, registration_port, sensory_port, motor_port, visualization_port, control_port);
        Ok(())
    }
    
    /// Set registration endpoint
    fn with_registration_endpoint(&mut self, endpoint: String) -> PyResult<()> {
        self.inner = self.inner.clone().with_registration_endpoint(endpoint);
        Ok(())
    }
    
    /// Set sensory data endpoint
    fn with_sensory_endpoint(&mut self, endpoint: String) -> PyResult<()> {
        self.inner = self.inner.clone().with_sensory_endpoint(endpoint);
        Ok(())
    }
    
    /// Set motor data endpoint
    fn with_motor_endpoint(&mut self, endpoint: String) -> PyResult<()> {
        self.inner = self.inner.clone().with_motor_endpoint(endpoint);
        Ok(())
    }
    
    /// Set heartbeat interval in seconds (0 to disable)
    fn with_heartbeat_interval(&mut self, interval: f64) -> PyResult<()> {
        self.inner = self.inner.clone().with_heartbeat_interval(interval);
        Ok(())
    }
    
    /// Set connection timeout in milliseconds
    fn with_connection_timeout_ms(&mut self, timeout: u64) -> PyResult<()> {
        self.inner = self.inner.clone().with_connection_timeout_ms(timeout);
        Ok(())
    }
    
    /// Set number of registration retries
    fn with_registration_retries(&mut self, retries: u32) -> PyResult<()> {
        self.inner = self.inner.clone().with_registration_retries(retries);
        Ok(())
    }
    
    /// Set sensory socket configuration (high water mark, linger, immediate)
    fn with_sensory_socket_config(&mut self, hwm: i32, linger_ms: i32, immediate: bool) -> PyResult<()> {
        self.inner = self.inner.clone().with_sensory_socket_config(hwm, linger_ms, immediate);
        Ok(())
    }
    
    /// Add vision capability
    #[pyo3(signature = (modality, width, height, channels, cortical_area))]
    fn with_vision_capability(
        &mut self,
        modality: String,
        width: usize,
        height: usize,
        channels: usize,
        cortical_area: String,
    ) -> PyResult<()> {
        self.inner = self.inner.clone().with_vision_capability(
            modality,
            (width, height),
            channels,
            cortical_area,
        );
        Ok(())
    }

    /// Add vision capability using semantic unit + group (preferred FEAGI 2.0 contract).
    #[pyo3(signature = (modality, width, height, channels, unit, group))]
    fn with_vision_unit(
        &mut self,
        modality: String,
        width: usize,
        height: usize,
        channels: usize,
        unit: String,
        group: u8,
    ) -> PyResult<()> {
        let unit_enum = parse_sensory_unit(unit.as_str())?;
        self.inner = self.inner.clone().with_vision_unit(
            modality,
            (width, height),
            channels,
            unit_enum,
            group,
        );
        Ok(())
    }
    
    /// Add motor capability
    #[pyo3(signature = (modality, output_count, cortical_areas))]
    fn with_motor_capability(
        &mut self,
        modality: String,
        output_count: usize,
        cortical_areas: Vec<String>,
    ) -> PyResult<()> {
        self.inner = self.inner.clone().with_motor_capability(modality, output_count, cortical_areas);
        Ok(())
    }

    /// Add motor capability using semantic unit + group (preferred FEAGI 2.0 contract).
    #[pyo3(signature = (modality, output_count, unit, group))]
    fn with_motor_unit(
        &mut self,
        modality: String,
        output_count: usize,
        unit: String,
        group: u8,
    ) -> PyResult<()> {
        let unit_enum = parse_motor_unit(unit.as_str())?;
        self.inner = self.inner.clone().with_motor_unit(modality, output_count, unit_enum, group);
        Ok(())
    }

    /// Add multiple motor units using semantic unit + group pairs.
    ///
    /// Expects source_units as a list of (unit, group) tuples.
    #[pyo3(signature = (modality, output_count, source_units))]
    fn with_motor_units(
        &mut self,
        modality: String,
        output_count: usize,
        source_units: Vec<(String, u8)>,
    ) -> PyResult<()> {
        let mut specs = Vec::with_capacity(source_units.len());
        for (unit, group) in source_units {
            let unit_enum = parse_motor_unit(unit.as_str())?;
            specs.push(feagi_io::MotorUnitSpec { unit: unit_enum, group });
        }
        self.inner = self.inner.clone().with_motor_units(modality, output_count, specs);
        Ok(())
    }
    
    /// Add custom capability (takes JSON string)
    fn with_custom_capability(&mut self, key: String, value_json: String) -> PyResult<()> {
        let value: serde_json::Value = serde_json::from_str(&value_json)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Invalid JSON: {}", e)
            ))?;
        self.inner = self.inner.clone().with_custom_capability(key, value);
        Ok(())
    }
    
    /// Validate configuration
    fn validate(&self) -> PyResult<()> {
        self.inner.validate()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }
    
    fn __repr__(&self) -> String {
        format!("PyAgentConfig(agent_id={})", self.inner.agent_id)
    }
}

impl PyAgentConfig {
    pub fn inner(&self) -> &feagi_agent::AgentConfig {
        &self.inner
    }
    
    #[allow(dead_code)]
    pub fn into_inner(self) -> feagi_agent::AgentConfig {
        self.inner
    }
}

