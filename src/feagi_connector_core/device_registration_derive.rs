//! Derive motor and sensory cortical IDs from device_registrations JSON.
//!
//! Must match feagi-api's auto_create_cortical_areas_from_device_registrations
//! so controller verification expectations align with FEAGI's auto-created areas.

use feagi_data_structures::genomic::cortical_area::descriptors::CorticalUnitIndex;
use feagi_data_structures::genomic::{MotorCorticalUnit, SensoryCorticalUnit};
use serde_json::Map;
use std::collections::HashSet;

fn build_io_config_map() -> Result<Map<String, serde_json::Value>, String> {
    use feagi_data_structures::genomic::cortical_area::io_cortical_area_configuration_flag::{
        FrameChangeHandling, PercentageNeuronPositioning,
    };
    let mut config = Map::new();
    config.insert(
        "frame_change_handling".to_string(),
        serde_json::to_value(FrameChangeHandling::Absolute)
            .map_err(|e| format!("Failed to serialize FrameChangeHandling: {}", e))?,
    );
    config.insert(
        "percentage_neuron_positioning".to_string(),
        serde_json::to_value(PercentageNeuronPositioning::Linear)
            .map_err(|e| format!("Failed to serialize PercentageNeuronPositioning: {}", e))?,
    );
    Ok(config)
}

/// Derive motor cortical IDs from device_registrations JSON.
///
/// Parses output_units_and_decoder_properties and returns base64 cortical IDs
/// that FEAGI will create when auto_create_missing_cortical_areas is enabled.
pub fn derive_motor_cortical_ids_from_device_registrations(
    device_registrations: &serde_json::Value,
) -> Result<HashSet<String>, String> {
    let output_units = device_registrations
        .get("output_units_and_decoder_properties")
        .and_then(|v| v.as_object())
        .ok_or_else(|| {
            "device_registrations missing output_units_and_decoder_properties".to_string()
        })?;

    let mut cortical_ids: HashSet<String> = HashSet::new();

    for (motor_unit_key, unit_defs) in output_units {
        let motor_unit: MotorCorticalUnit = serde_json::from_value::<MotorCorticalUnit>(
            serde_json::Value::String(motor_unit_key.clone()),
        )
        .map_err(|e| {
            format!(
                "Unable to parse MotorCorticalUnit key '{}': {}",
                motor_unit_key, e
            )
        })?;

        let unit_defs_arr = unit_defs
            .as_array()
            .ok_or_else(|| "Motor unit definitions must be an array".to_string())?;

        for entry in unit_defs_arr {
            let pair = entry
                .as_array()
                .ok_or_else(|| "Motor unit definition entries must be arrays".to_string())?;
            let unit_def = pair
                .first()
                .ok_or_else(|| "Motor unit definition entry missing unit_def".to_string())?;
            let group_u64 = unit_def
                .get("cortical_unit_index")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| {
                    "Motor unit definition missing cortical_unit_index".to_string()
                })?;
            let group_u8: u8 = group_u64.try_into().map_err(|_| {
                "Motor unit cortical_unit_index out of range for u8".to_string()
            })?;
            let group: CorticalUnitIndex = group_u8.into();

            let device_count = unit_def
                .get("device_grouping")
                .and_then(|v| v.as_array())
                .map(|a| a.len())
                .unwrap_or(0);
            if device_count == 0 {
                return Err(format!(
                    "device_grouping is empty for motor unit '{}' group {}",
                    motor_unit_key, group_u8
                ));
            }

            let config = build_io_config_map()
                .map_err(|e| format!("Failed to build motor IO config map: {}", e))?;
            let unit_cortical_ids = motor_unit
                .get_cortical_id_vector_from_index_and_serde_io_configuration_flags(group, config)
                .map_err(|e| format!("Failed to derive cortical IDs: {}", e))?;
            for cortical_id in unit_cortical_ids {
                cortical_ids.insert(cortical_id.as_base_64());
            }
        }
    }

    Ok(cortical_ids)
}

/// Derive sensory cortical IDs from device_registrations JSON.
///
/// Parses input_units_and_encoder_properties and returns base64 cortical IDs
/// that FEAGI will create when auto_create_missing_cortical_areas is enabled.
/// Returns empty set if no input units (e.g. motor-only agent).
pub fn derive_sensory_cortical_ids_from_device_registrations(
    device_registrations: &serde_json::Value,
) -> Result<HashSet<String>, String> {
    let input_units = device_registrations
        .get("input_units_and_encoder_properties")
        .and_then(|v| v.as_object());

    let Some(input_units) = input_units else {
        return Ok(HashSet::new());
    };

    let mut cortical_ids: HashSet<String> = HashSet::new();

    for (sensory_unit_key, unit_defs) in input_units {
        let sensory_unit: SensoryCorticalUnit = serde_json::from_value::<SensoryCorticalUnit>(
            serde_json::Value::String(sensory_unit_key.clone()),
        )
        .map_err(|e| {
            format!(
                "Unable to parse SensoryCorticalUnit key '{}': {}",
                sensory_unit_key, e
            )
        })?;

        let unit_defs_arr = unit_defs
            .as_array()
            .ok_or_else(|| "Sensory unit definitions must be an array".to_string())?;

        for entry in unit_defs_arr {
            let pair = entry
                .as_array()
                .ok_or_else(|| "Sensory unit definition entries must be arrays".to_string())?;
            let unit_def = pair
                .first()
                .ok_or_else(|| {
                    "Sensory unit definition entry missing unit_def".to_string()
                })?;
            let group_u64 = unit_def
                .get("cortical_unit_index")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| {
                    "Sensory unit definition missing cortical_unit_index".to_string()
                })?;
            let group_u8: u8 = group_u64.try_into().map_err(|_| {
                "Sensory unit cortical_unit_index out of range for u8".to_string()
            })?;
            let group: CorticalUnitIndex = group_u8.into();

            let device_count = unit_def
                .get("device_grouping")
                .and_then(|v| v.as_array())
                .map(|a| a.len())
                .unwrap_or(0);
            if device_count == 0 {
                return Err(format!(
                    "device_grouping is empty for sensory unit '{}' group {}",
                    sensory_unit_key, group_u8
                ));
            }

            let config = build_io_config_map()
                .map_err(|e| format!("Failed to build sensory IO config map: {}", e))?;
            let unit_cortical_ids = sensory_unit
                .get_cortical_id_vector_from_index_and_serde_io_configuration_flags(group, config)
                .map_err(|e| format!("Failed to derive sensory cortical IDs: {}", e))?;
            for cortical_id in unit_cortical_ids {
                cortical_ids.insert(cortical_id.as_base_64());
            }
        }
    }

    Ok(cortical_ids)
}
