use std::fs;
// use feagi_data_structures::{sensor_definition}; // Removed in latest feagi_data_structures
// use crate::{get_sensor_variants, SensorVariant}; // Unused

// TODO this file is unfinished PYI generation. Work on it later

#[allow(dead_code)]
pub fn generate_pyi(_template_path: &str, _output_path: &str) {
    // Read the template file
    let _template_content = fs::read_to_string(_template_path)
        .expect("Failed to read feagi_data_processing.pyi.template");

    // Insert the SensorCorticalType into the template
    //let final_content = insert_sensor_cortical_type(template_content, sensor_class_def);

    // Write the final .pyi file
    //fs::write(output_path, final_content)
    //    .expect("Failed to write feagi_data_processing.pyi");

    println!("Generated feagi_data_processing.pyi with SensorCorticalType");
}

#[allow(dead_code)]
fn insert_sensor_cortical_type(template: String, sensor_class_def: String) -> String {
    // Find the insertion point - after CoreCorticalType but before CorticalGroupingIndex
    let insertion_marker = "    class CoreCorticalType:\n        \"\"\"Enum representing core cortical area types.\"\"\"\n        Death: 'CoreCorticalType'\n        Power: 'CoreCorticalType'\n    \n";

    if let Some(pos) = template.find(insertion_marker) {
        let end_pos = pos + insertion_marker.len();
        let mut result = String::new();
        result.push_str(&template[..end_pos]);
        result.push_str(&sensor_class_def);
        result.push_str(&template[end_pos..]);
        result
    } else {
        // Fallback: try to find a different marker
        let fallback_marker = "    class CorticalGroupingIndex:";
        if let Some(pos) = template.find(fallback_marker) {
            let mut result = String::new();
            result.push_str(&template[..pos]);
            result.push_str(&sensor_class_def);
            result.push_str("    ");
            result.push_str(&template[pos..]);
            result
        } else {
            // If we can't find the insertion point, just append at the end of the genome module
            let genome_end_marker = "\n# IO Data module";
            if let Some(pos) = template.find(genome_end_marker) {
                let mut result = String::new();
                result.push_str(&template[..pos]);
                result.push('\n');
                result.push_str(&sensor_class_def);
                result.push_str(&template[pos..]);
                result
            } else {
                // Last resort: just return template with sensor class appended
                let mut result = template;
                result.push('\n');
                result.push_str(&sensor_class_def);
                result
            }
        }
    }
}


// Commented out until updated for new feagi_data_structures API
// fn generate_sensor_cortical_type_class() -> String {
//     let variants = get_sensor_variants();
//     let mut class_def = String::new();
//
//     class_def.push_str("    class SensorCorticalType:\n");
//     class_def.push_str("        \"\"\"Enum representing different types of sensor cortical areas.\n");
//     class_def.push_str("        \n");
//     class_def.push_str("        This enum defines all the available sensor types that can be used\n");
//     class_def.push_str("        in FEAGI for processing input data from various sensors and devices.\n");
//     class_def.push_str("        Each sensor type has specific characteristics and use cases.\n");
//     class_def.push_str("        \"\"\"\n");
//     class_def.push_str("        \n");
//
//     // Add each variant as a class attribute with docstrings
//     for variant in &variants {
//         class_def.push_str(&format!("        {}: 'SensorCorticalType'\n", variant.name));
//         if let Some(doc) = &variant.doc {
//             class_def.push_str(&format!("        \"\"\"{}.\"\"\"\n", doc));
//         }
//         class_def.push_str("        \n");
//     }
//
//     class_def
// }