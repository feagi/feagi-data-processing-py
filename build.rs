mod rust_build_scripts;
use std::fs;
use heck::ToSnakeCase;

// NOTE: This crate uses a Cargo alias so existing code can refer to the
// `feagi-structures` package as `feagi_data_structures`.
use feagi_data_structures::motor_cortical_units;

fn main() {
    println!("cargo:rerun-if-changed=feagi_data_processing.pyi.template");
    println!("cargo:rerun-if-changed=src/feagi_connector_core/connector_agent.rs");
    
    // NOTE: Code generation currently disabled - uncomment below to re-enable
    // let _template_path = "feagi_data_processing.pyi.template";
    // let _pyi_output_path = "feagi_data_processing.pyi";
    // let _io_cache_path = "src/feagi_connector_core/caching/io_cache.rs";
    // rust_build_scripts::io_cache_template_writer::update_connector_agent_source_file(_io_cache_path);
}


// TODO: Rename to feagi_data_libraries?
// TODO add macro(s) / funcs for going from PyObject to index types?
// TODO: confirm func for building inheritance?

#[allow(dead_code)]
fn read_source_file(file_path: &str) -> String {
    let content = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Failed to read {}", file_path));
    content
}

#[allow(dead_code)]
fn save_source_file(data: String, file_path: &str) {
    // Write the updated content back to the file
    fs::write(file_path, &data)
        .unwrap_or_else(|_| panic!("Failed to write {}", file_path));
}

#[allow(dead_code)]
fn check_for_segment(source_string: &str, checking: &str) {
    _ = source_string.find(checking)
        .unwrap_or_else(|| panic!("Could not find '{}' requirement in source file!", checking));
}

#[allow(dead_code)]
fn replace_code_segment(source_string: String, start_marker: &str, end_marker: &str, replacing_string: String) -> String {
    // Read the file
    let content = source_string;

    // Find the positions of the markers
    let start_pos = content.find(start_marker)
        .unwrap_or_else(|| panic!("Could not find {} marker in source file!", start_marker));
    let end_pos = content.find(end_marker)
        .unwrap_or_else(|| panic!("Could not find {} marker in source file!", end_marker));

    // Ensure the markers are in the correct order
    if start_pos >= end_pos {
        panic!("Markers are in the wrong order in source file!");
    }

    // Calculate the position after the start marker (including the newline)
    let replace_start = start_pos + start_marker.len();

    // Find the newline after the start marker
    let replace_start = if content[replace_start..].starts_with('\r') {
        replace_start + 2 // Skip \r\n
    } else if content[replace_start..].starts_with('\n') {
        replace_start + 1 // Skip \n
    } else {
        replace_start
    };

    // Find the position before the end marker (including any leading whitespace on that line)
    let replace_end = content[..end_pos].rfind('\n').map(|pos| pos + 1).unwrap_or(end_pos);

    // Build the new content
    let mut new_content = String::new();
    new_content.push_str(&content[..replace_start]);
    new_content.push_str(&replacing_string);
    new_content.push_str(&content[replace_end..]);

    new_content
}



//region Collect Sensor / Motor context

#[allow(unused_macros)]
macro_rules! collect_motor_variants {
    (
        MotorCorticalUnit {
            $(
                #[doc = $doc:expr]
                $cortical_type_key_name:ident => {
                    friendly_name: $friendly_name:expr,
                    accepted_wrapped_io_data_type: $accepted_wrapped_io_data_type:ident,
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
        vec![
            $(
                MotorVariant {
                    name: stringify!($cortical_type_key_name).to_string(),
                    doc: Some(($doc).to_string()),
                    friendly_name: $friendly_name.to_string(),
                    snake_case_name: stringify!($cortical_type_key_name).to_snake_case(),
                    accepted_wrapped_io_data_type: stringify!($accepted_wrapped_io_data_type).to_string()
                }
            ),*
        ]
    };
}

#[allow(unused_macros)]
macro_rules! collect_sensor_variants {
    (
        SensorCorticalType {
            $(
                #[doc = $doc:expr]
                $variant:ident => {
                    friendly_name: $friendly_name:expr,
                    snake_case_identifier: $snake_case_identifier:expr,
                    base_ascii: $base_ascii:expr,
                    channel_dimension_range: $channel_dimension_range:expr,
                    default_coder_type: $default_coder_type:ident,
                    wrapped_data_type: $wrapped_data_type:expr,
                    data_type: $data_type:ident,
                }$(,)?
            )*
        }
    ) => {
        vec![
            $(
                SensorVariant {
                    name: stringify!($variant).to_string(),
                    doc: Some($doc.to_string()),
                    friendly_name: $friendly_name.to_string(),
                    snake_case_name: $snake_case_identifier.to_string(),
                    accepted_wrapped_io_data_type: stringify!($data_type).to_string()
                }
            ),*
        ]
    };
}

#[allow(dead_code)]
#[derive(Debug)]
struct MotorVariant {
    name: String,
    doc: Option<String>,
    friendly_name: String,
    snake_case_name: String,
    accepted_wrapped_io_data_type: String,
}

#[allow(dead_code)]
#[derive(Debug)]
struct SensorVariant {
    name: String,
    doc: Option<String>,
    friendly_name: String,
    snake_case_identifier: String,
    default_coder_type: String,
    rust_data_type: String,
}
/*
fn get_sensor_variants() -> Vec<SensorVariant> {
     sensor_definition!(collect_sensor_variants)
}
*/

#[allow(dead_code)]
fn get_motor_variants() -> Vec<MotorVariant> {
     motor_cortical_units!(collect_motor_variants)
}


//endregion