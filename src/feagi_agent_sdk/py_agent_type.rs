/*
 * PyO3 wrapper for AgentType enum
 */

use pyo3::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentTypeCompat {
    Sensory,
    Motor,
    Both,
}

impl AgentTypeCompat {
    pub fn as_str(self) -> &'static str {
        match self {
            AgentTypeCompat::Sensory => "sensory",
            AgentTypeCompat::Motor => "motor",
            AgentTypeCompat::Both => "both",
        }
    }
}

#[pyclass(name = "AgentType")]
#[derive(Clone, Debug)]
pub struct PyAgentType {
    inner: AgentTypeCompat,
}

#[pymethods]
impl PyAgentType {
    #[new]
    fn new(agent_type: &str) -> PyResult<Self> {
        let inner = match agent_type.to_lowercase().as_str() {
            "sensory" => AgentTypeCompat::Sensory,
            "motor" => AgentTypeCompat::Motor,
            "both" => AgentTypeCompat::Both,
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid agent type: {}. Must be 'sensory', 'motor', or 'both'",
                    agent_type
                )));
            }
        };
        Ok(PyAgentType { inner })
    }

    #[classattr]
    const SENSORY: &'static str = "sensory";

    #[classattr]
    const MOTOR: &'static str = "motor";

    #[classattr]
    const BOTH: &'static str = "both";

    #[staticmethod]
    fn sensory() -> Self {
        PyAgentType {
            inner: AgentTypeCompat::Sensory,
        }
    }

    #[staticmethod]
    fn motor() -> Self {
        PyAgentType {
            inner: AgentTypeCompat::Motor,
        }
    }

    #[staticmethod]
    fn both() -> Self {
        PyAgentType {
            inner: AgentTypeCompat::Both,
        }
    }

    fn __repr__(&self) -> String {
        format!("AgentType({})", self.inner.as_str())
    }

    fn __str__(&self) -> String {
        self.inner.as_str().to_string()
    }
}

impl PyAgentType {
    pub fn inner(&self) -> AgentTypeCompat {
        self.inner
    }
}
