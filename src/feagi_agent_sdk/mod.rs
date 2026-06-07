/*
 * PyO3 bindings for feagi-agent-sdk
 *
 * Exposes the Rust AgentClient to Python as PyAgentClient
 */

pub mod py_agent_client;
pub mod py_agent_config;
pub mod py_agent_type;
pub mod py_recovery;

pub use py_agent_client::PyAgentClient;
pub use py_agent_config::PyAgentConfig;
pub use py_agent_type::PyAgentType as AgentType;

use feagi_data_structures::{motor_cortical_units, sensor_cortical_units};
use pyo3::prelude::*;
use std::sync::Once;
use std::{collections::BTreeMap, str};

static INIT: Once = Once::new();

/// Initialize Rust tracing (call once from Python)
#[pyfunction]
fn init_rust_logging() {
    INIT.call_once(|| {
        use tracing_subscriber::{fmt, EnvFilter};

        // Default to INFO level if RUST_LOG not set
        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

        fmt()
            .with_env_filter(filter)
            .with_target(false)
            .with_thread_ids(false)
            .with_file(false)
            .try_init()
            // Avoid panicking if another module already installed a global subscriber.
            // This can happen if both connector_core and agent_sdk call init_rust_logging()
            // within the same Python process.
            .ok();
    });
}

fn build_cortical_subtype_friendly_names() -> BTreeMap<String, BTreeMap<u8, String>> {
    let mut names: BTreeMap<String, BTreeMap<u8, String>> = BTreeMap::new();

    macro_rules! collect_sensor_subtype_names {
        (
            SensoryCorticalUnit {
                $(
                    $(#[doc = $doc:expr])?
                    $variant_name:ident => {
                        friendly_name: $friendly_name:expr,
                        accepted_wrapped_io_data_type: $accepted_wrapped_io_data_type:expr,
                        cortical_id_unit_reference: $cortical_id_unit_reference:expr,
                        number_cortical_areas: $number_cortical_areas:expr,
                        $(default_firing_threshold: $default_firing_threshold:expr,)?
                        $(default_mp_charge_accumulation: $default_mp_charge_accumulation:expr,)?
                        cortical_type_parameters: {
                            $($param_name:ident: $param_type:ty),* $(,)?
                        },
                        $(allowed_frame_change_handling: [$($allowed_frame:ident),* $(,)?],)?
                        cortical_area_properties: {
                            $($area_index:tt => ($cortical_area_type_expr:expr, relative_position: [$rel_x:expr, $rel_y:expr, $rel_z:expr], channel_dimensions_default: [$dim_default_x:expr, $dim_default_y:expr, $dim_default_z:expr], channel_dimensions_min: [$dim_min_x:expr, $dim_min_y:expr, $dim_min_z:expr], channel_dimensions_max: [$dim_max_x:expr, $dim_max_y:expr, $dim_max_z:expr])),* $(,)?
                        }
                    }
                ),* $(,)?
            }
        ) => {
            $(
                let subtype_key = str::from_utf8(&$cortical_id_unit_reference)
                    .expect("sensor cortical subtype reference must be ascii")
                    .to_string();
                names
                    .entry(subtype_key)
                    .or_default()
                    .insert(b'i', $friendly_name.to_string());
            )*
        };
    }

    macro_rules! collect_motor_subtype_names {
        (
            MotorCorticalUnit {
                $(
                    $(#[doc = $doc:expr])?
                    $variant_name:ident => {
                        friendly_name: $friendly_name:expr,
                        accepted_wrapped_io_data_type: $accepted_wrapped_io_data_type:expr,
                        cortical_id_unit_reference: $cortical_id_unit_reference:expr,
                        number_cortical_areas: $number_cortical_areas:expr,
                        cortical_type_parameters: {
                            $($param_name:ident: $param_type:ty),* $(,)?
                        },
                        $(allowed_frame_change_handling: [$($allowed_frame:ident),* $(,)?],)?
                        cortical_area_properties: {
                            $($area_index:tt => ($cortical_area_type_expr:expr, relative_position: [$rel_x:expr, $rel_y:expr, $rel_z:expr], channel_dimensions_default: [$dim_default_x:expr, $dim_default_y:expr, $dim_default_z:expr], channel_dimensions_min: [$dim_min_x:expr, $dim_min_y:expr, $dim_min_z:expr], channel_dimensions_max: [$dim_max_x:expr, $dim_max_y:expr, $dim_max_z:expr])),* $(,)?
                        }
                    }
                ),* $(,)?
            }
        ) => {
            $(
                let subtype_key = str::from_utf8(&$cortical_id_unit_reference)
                    .expect("motor cortical subtype reference must be ascii")
                    .to_string();
                names
                    .entry(subtype_key)
                    .or_default()
                    .insert(b'o', $friendly_name.to_string());
            )*
        };
    }

    sensor_cortical_units!(collect_sensor_subtype_names);
    motor_cortical_units!(collect_motor_subtype_names);
    names
}

/// Return canonical cortical subtype friendly names grouped by category byte.
///
/// Key format:
/// - outer key: 3-byte cortical subtype as ASCII string (e.g. `"ptr"`)
/// - inner key: category byte (`105` for sensory/input, `111` for motor/output)
#[pyfunction]
fn cortical_subtype_friendly_names() -> BTreeMap<String, BTreeMap<u8, String>> {
    build_cortical_subtype_friendly_names()
}

/// Register the feagi_agent module with Python
pub fn register_module(py: Python, parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let submodule = PyModule::new(py, "feagi_agent")?;

    // Register types
    submodule.add_class::<PyAgentClient>()?;
    submodule.add_class::<PyAgentConfig>()?;
    submodule.add_class::<AgentType>()?;

    // Register recovery primitives (HealthWatcher, ReconnectPolicy, etc.)
    py_recovery::register(&submodule)?;

    // Register functions
    submodule.add_function(wrap_pyfunction!(init_rust_logging, &submodule)?)?;
    submodule.add_function(wrap_pyfunction!(
        cortical_subtype_friendly_names,
        &submodule
    )?)?;

    parent_module.add_submodule(&submodule)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::build_cortical_subtype_friendly_names;

    #[test]
    fn cortical_subtype_friendly_names_exports_known_subtypes() {
        let names = build_cortical_subtype_friendly_names();

        let ptr = names.get("ptr").expect("expected 'ptr' subtype");
        assert_eq!(
            ptr.get(&b'o').map(String::as_str),
            Some("Spatial Pointer"),
            "motor spatial-pointer name should match template",
        );
        assert!(
            !ptr.contains_key(&b'i'),
            "'ptr' should not have a sensory category mapping",
        );

        let svi = names.get("svi").expect("expected 'svi' subtype");
        assert_eq!(
            svi.get(&b'i').map(String::as_str),
            Some("Segmented Vision"),
            "sensory segmented-vision name should match template",
        );
        assert!(
            !svi.contains_key(&b'o'),
            "'svi' should not have a motor category mapping",
        );
    }

    #[test]
    fn cortical_subtype_friendly_names_handles_shared_subtypes_per_category() {
        let names = build_cortical_subtype_friendly_names();
        let shared = names.get("mis").expect("expected shared 'mis' subtype");

        assert_eq!(
            shared.get(&b'i').map(String::as_str),
            Some("Miscellaneous Sensor"),
        );
        assert_eq!(
            shared.get(&b'o').map(String::as_str),
            Some("Miscellaneous Motor"),
        );
    }
}
