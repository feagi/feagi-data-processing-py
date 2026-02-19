/*
 * PyO3 wrapper for AgentConfig
 */

use super::py_agent_type::{AgentTypeCompat, PyAgentType};
use base64::Engine;
use pyo3::prelude::*;
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct AgentDescriptorCompat {
    pub manufacturer: String,
    pub agent_name: String,
    pub agent_version: u32,
}

#[derive(Clone, Debug)]
pub struct VisionCapabilityCompat {
    pub modality: String,
    pub width: usize,
    pub height: usize,
    pub channels: usize,
    pub cortical_area: Option<String>,
    pub unit: Option<String>,
    pub group: Option<u8>,
}

#[derive(Clone, Debug)]
pub struct MotorCapabilityCompat {
    pub value: Value,
}

#[derive(Clone, Debug)]
pub struct AgentConfigCompat {
    pub agent_id: String,
    pub agent_type: AgentTypeCompat,
    pub registration_endpoint: String,
    pub sensory_endpoint: String,
    pub motor_endpoint: String,
    pub visualization_endpoint: String,
    pub control_endpoint: String,
    pub heartbeat_interval_secs: Option<f64>,
    pub connection_timeout_ms: Option<u64>,
    pub registration_retries: Option<u32>,
    pub sensory_send_hwm: i32,
    pub sensory_linger_ms: i32,
    pub sensory_immediate: bool,
    pub descriptor: Option<AgentDescriptorCompat>,
    pub auth_token: Option<[u8; 32]>,
    pub vision_capability: Option<VisionCapabilityCompat>,
    pub motor_capability: Option<MotorCapabilityCompat>,
    pub custom_capabilities: Vec<(String, Value)>,
}

impl AgentConfigCompat {
    pub(crate) fn new(agent_id: String, agent_type: AgentTypeCompat) -> Self {
        Self {
            agent_id,
            agent_type,
            registration_endpoint: String::new(),
            sensory_endpoint: String::new(),
            motor_endpoint: String::new(),
            visualization_endpoint: String::new(),
            control_endpoint: String::new(),
            heartbeat_interval_secs: None,
            connection_timeout_ms: None,
            registration_retries: None,
            sensory_send_hwm: 1,
            sensory_linger_ms: 0,
            sensory_immediate: true,
            descriptor: None,
            auth_token: None,
            vision_capability: None,
            motor_capability: None,
            custom_capabilities: Vec::new(),
        }
    }
}

#[pyclass(name = "PyAgentConfig")]
#[derive(Clone)]
pub struct PyAgentConfig {
    inner: AgentConfigCompat,
}

#[pymethods]
impl PyAgentConfig {
    #[new]
    fn new(agent_id: String, agent_type: PyAgentType) -> Self {
        PyAgentConfig {
            inner: AgentConfigCompat::new(agent_id, agent_type.inner()),
        }
    }

    fn with_feagi_endpoints(
        &mut self,
        host: String,
        registration_port: u16,
        sensory_port: u16,
        motor_port: u16,
        visualization_port: u16,
        control_port: u16,
    ) -> PyResult<()> {
        self.inner.registration_endpoint = format!("tcp://{}:{}", host, registration_port);
        self.inner.sensory_endpoint = format!("tcp://{}:{}", host, sensory_port);
        self.inner.motor_endpoint = format!("tcp://{}:{}", host, motor_port);
        self.inner.visualization_endpoint = format!("tcp://{}:{}", host, visualization_port);
        self.inner.control_endpoint = format!("tcp://{}:{}", host, control_port);
        Ok(())
    }

    fn with_registration_endpoint(&mut self, endpoint: String) -> PyResult<()> {
        self.inner.registration_endpoint = endpoint;
        Ok(())
    }

    fn with_sensory_endpoint(&mut self, endpoint: String) -> PyResult<()> {
        self.inner.sensory_endpoint = endpoint;
        Ok(())
    }

    fn with_motor_endpoint(&mut self, endpoint: String) -> PyResult<()> {
        self.inner.motor_endpoint = endpoint;
        Ok(())
    }

    fn with_heartbeat_interval(&mut self, interval: f64) -> PyResult<()> {
        self.inner.heartbeat_interval_secs = Some(interval);
        Ok(())
    }

    fn with_connection_timeout_ms(&mut self, timeout: u64) -> PyResult<()> {
        self.inner.connection_timeout_ms = Some(timeout);
        Ok(())
    }

    fn with_registration_retries(&mut self, retries: u32) -> PyResult<()> {
        self.inner.registration_retries = Some(retries);
        Ok(())
    }

    fn with_sensory_socket_config(
        &mut self,
        hwm: i32,
        linger_ms: i32,
        immediate: bool,
    ) -> PyResult<()> {
        self.inner.sensory_send_hwm = hwm;
        self.inner.sensory_linger_ms = linger_ms;
        self.inner.sensory_immediate = immediate;
        Ok(())
    }

    #[pyo3(signature = (modality, width, height, channels, cortical_area))]
    fn with_vision_capability(
        &mut self,
        modality: String,
        width: usize,
        height: usize,
        channels: usize,
        cortical_area: String,
    ) -> PyResult<()> {
        self.inner.vision_capability = Some(VisionCapabilityCompat {
            modality,
            width,
            height,
            channels,
            cortical_area: Some(cortical_area),
            unit: None,
            group: None,
        });
        Ok(())
    }

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
        self.inner.vision_capability = Some(VisionCapabilityCompat {
            modality,
            width,
            height,
            channels,
            cortical_area: None,
            unit: Some(unit),
            group: Some(group),
        });
        Ok(())
    }

    #[pyo3(signature = (modality, output_count, cortical_areas))]
    fn with_motor_capability(
        &mut self,
        modality: String,
        output_count: usize,
        cortical_areas: Vec<String>,
    ) -> PyResult<()> {
        self.inner.motor_capability = Some(MotorCapabilityCompat {
            value: serde_json::json!({
                "modality": modality,
                "output_count": output_count,
                "source_cortical_areas": cortical_areas,
            }),
        });
        Ok(())
    }

    #[pyo3(signature = (modality, output_count, unit, group))]
    fn with_motor_unit(
        &mut self,
        modality: String,
        output_count: usize,
        unit: String,
        group: u8,
    ) -> PyResult<()> {
        self.inner.motor_capability = Some(MotorCapabilityCompat {
            value: serde_json::json!({
                "modality": modality,
                "output_count": output_count,
                "unit": unit,
                "group": group,
            }),
        });
        Ok(())
    }

    #[pyo3(signature = (modality, output_count, source_units))]
    fn with_motor_units(
        &mut self,
        modality: String,
        output_count: usize,
        source_units: Vec<(String, u8)>,
    ) -> PyResult<()> {
        self.inner.motor_capability = Some(MotorCapabilityCompat {
            value: serde_json::json!({
                "modality": modality,
                "output_count": output_count,
                "source_units": source_units,
            }),
        });
        Ok(())
    }

    fn with_custom_capability(&mut self, key: String, value_json: String) -> PyResult<()> {
        let value: Value = serde_json::from_str(&value_json).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid JSON: {}", e))
        })?;
        self.inner.custom_capabilities.push((key, value));
        Ok(())
    }

    fn with_agent_descriptor(
        &mut self,
        manufacturer: String,
        agent_name: String,
        agent_version: u32,
    ) -> PyResult<()> {
        self.inner.descriptor = Some(AgentDescriptorCompat {
            manufacturer,
            agent_name,
            agent_version,
        });
        Ok(())
    }

    fn with_auth_token_base64(&mut self, auth_token_b64: String) -> PyResult<()> {
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(auth_token_b64.as_bytes())
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid auth token base64: {}",
                    e
                ))
            })?;
        if decoded.len() != 32 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Auth token must decode to exactly 32 bytes, got {}",
                decoded.len()
            )));
        }
        let mut token = [0_u8; 32];
        token.copy_from_slice(&decoded);
        self.inner.auth_token = Some(token);
        Ok(())
    }

    fn validate(&self) -> PyResult<()> {
        self.validate_internal()
            .map_err(|err| PyErr::new::<pyo3::exceptions::PyValueError, _>(err))
    }

    fn __repr__(&self) -> String {
        format!("PyAgentConfig(agent_id={})", self.inner.agent_id)
    }
}

#[cfg(test)]
mod tests {
    use super::{AgentConfigCompat, AgentDescriptorCompat, AgentTypeCompat, VisionCapabilityCompat};

    fn valid_config() -> AgentConfigCompat {
        let mut cfg = AgentConfigCompat::new("agent-1".to_string(), AgentTypeCompat::Both);
        cfg.registration_endpoint = "tcp://127.0.0.1:30001".to_string();
        cfg.sensory_endpoint = "tcp://127.0.0.1:5558".to_string();
        cfg.motor_endpoint = "tcp://127.0.0.1:5564".to_string();
        cfg.connection_timeout_ms = Some(1000);
        cfg.registration_retries = Some(3);
        cfg.heartbeat_interval_secs = Some(1.0);
        cfg.descriptor = Some(AgentDescriptorCompat {
            manufacturer: "Neuraville".to_string(),
            agent_name: "agent-1".to_string(),
            agent_version: 1,
        });
        cfg.auth_token = Some([7_u8; 32]);
        cfg.vision_capability = Some(VisionCapabilityCompat {
            modality: "vision".to_string(),
            width: 4,
            height: 4,
            channels: 1,
            cortical_area: Some("iv00".to_string()),
            unit: None,
            group: None,
        });
        cfg
    }

    #[test]
    fn validate_internal_accepts_complete_config() {
        let cfg = valid_config();
        let py_cfg = crate::feagi_agent_sdk::py_agent_config::PyAgentConfig { inner: cfg };
        assert!(py_cfg.validate_internal().is_ok());
    }

    #[test]
    fn validate_internal_rejects_missing_auth_token() {
        let mut cfg = valid_config();
        cfg.auth_token = None;
        let py_cfg = crate::feagi_agent_sdk::py_agent_config::PyAgentConfig { inner: cfg };
        assert!(py_cfg
            .validate_internal()
            .expect_err("expected validation failure")
            .contains("auth token"));
    }

    #[test]
    fn validate_internal_rejects_incomplete_vision_targeting() {
        let mut cfg = valid_config();
        cfg.vision_capability = Some(VisionCapabilityCompat {
            modality: "vision".to_string(),
            width: 4,
            height: 4,
            channels: 1,
            cortical_area: None,
            unit: Some("vision".to_string()),
            group: None,
        });
        let py_cfg = crate::feagi_agent_sdk::py_agent_config::PyAgentConfig { inner: cfg };
        assert!(py_cfg
            .validate_internal()
            .expect_err("expected validation failure")
            .contains("unit+group"));
    }
}

impl PyAgentConfig {
    pub fn inner(&self) -> &AgentConfigCompat {
        &self.inner
    }

    pub fn validate_internal(&self) -> Result<(), String> {
        if self.inner.agent_id.trim().is_empty() {
            return Err("agent_id cannot be empty".to_string());
        }
        if self.inner.registration_endpoint.trim().is_empty() {
            return Err("registration_endpoint must be set".to_string());
        }
        if matches!(
            self.inner.agent_type,
            AgentTypeCompat::Sensory | AgentTypeCompat::Both
        ) && self.inner.sensory_endpoint.trim().is_empty()
        {
            return Err("sensory_endpoint must be set for sensory/both agents".to_string());
        }
        if matches!(
            self.inner.agent_type,
            AgentTypeCompat::Motor | AgentTypeCompat::Both
        ) && self.inner.motor_endpoint.trim().is_empty()
        {
            return Err("motor_endpoint must be set for motor/both agents".to_string());
        }
        if self.inner.connection_timeout_ms.is_none() {
            return Err("connection_timeout_ms must be explicitly set".to_string());
        }
        if self.inner.registration_retries.is_none() {
            return Err("registration_retries must be explicitly set".to_string());
        }
        if self.inner.heartbeat_interval_secs.is_none() {
            return Err("heartbeat_interval_secs must be explicitly set".to_string());
        }
        if self.inner.descriptor.is_none() {
            return Err("agent descriptor must be explicitly set".to_string());
        }
        if self.inner.auth_token.is_none() {
            return Err("auth token must be explicitly set".to_string());
        }
        if let Some(descriptor) = &self.inner.descriptor {
            if descriptor.manufacturer.trim().is_empty() {
                return Err("descriptor.manufacturer cannot be empty".to_string());
            }
            if descriptor.agent_name.trim().is_empty() {
                return Err("descriptor.agent_name cannot be empty".to_string());
            }
            if descriptor.agent_version == 0 {
                return Err("descriptor.agent_version must be > 0".to_string());
            }
        }
        if matches!(
            self.inner.agent_type,
            AgentTypeCompat::Sensory | AgentTypeCompat::Both
        ) && self.inner.vision_capability.is_none()
        {
            return Err(
                "vision capability must be set for sensory/both agents when using tuple API"
                    .to_string(),
            );
        }
        if let Some(vision) = &self.inner.vision_capability {
            if vision.modality.trim().is_empty() {
                return Err("vision.modality cannot be empty".to_string());
            }
            if vision.cortical_area.is_none() && (vision.unit.is_none() || vision.group.is_none()) {
                return Err(
                    "vision capability must provide either target_cortical_area or unit+group"
                        .to_string(),
                );
            }
        }
        if let Some(motor) = &self.inner.motor_capability {
            if !motor.value.is_object() {
                return Err("motor capability must be a JSON object".to_string());
            }
        }
        Ok(())
    }
}

