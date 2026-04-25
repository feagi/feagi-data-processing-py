use crate::{
    check_for_segment, get_motor_variants, read_source_file, replace_code_segment, save_source_file,
};

#[allow(dead_code)]
pub fn update_connector_agent_source_file(file_path: &str) {
    let checking_flag: &str = "//BUILDRS_ON";
    let motor_start_comment: &str = "//BUILDRS_MOTOR_DEVICE_START";
    let motor_end_comment: &str = "//BUILDRS_MOTOR_DEVICE_END";
    let _sensor_start_comment: &str = "//BUILDRS_SENSOR_DEVICE_START";
    let _sensor_end_comment: &str = "//BUILDRS_SENSOR_DEVICE_END";

    let source_file_string = read_source_file(file_path);
    check_for_segment(&source_file_string, checking_flag); // Errors out here if not enabled!

    let motor_registration_functions = generate_motor_registration_functions();
    let source_file_string = replace_code_segment(
        source_file_string,
        motor_start_comment,
        motor_end_comment,
        motor_registration_functions,
    );

    //let sensor_registration_functions = generate_sensor_registration_functions();
    //let source_file_string = replace_code_segment(source_file_string, sensor_start_comment, sensor_end_comment, sensor_registration_functions);

    save_source_file(source_file_string, file_path);
}

#[allow(dead_code)]
fn generate_motor_registration_functions() -> String {
    let variants = get_motor_variants();
    let mut functions = String::new();

    for variant in &variants {
        functions.push_str("    //region ");
        functions.push_str(&variant.snake_case_name);
        functions.push('\n');

        functions.push_str(&motor_unique_functions(
            &variant.snake_case_name,
            &variant.accepted_wrapped_io_data_type,
        ));
        functions.push_str(&motor_shared_functions(
            &variant.snake_case_name,
            &variant.accepted_wrapped_io_data_type,
        ));

        functions.push_str("    //endregion\n\n");
    }

    functions
}

/*
fn generate_sensor_registration_functions() -> String {
    let variants = get_sensor_variants();
    let mut functions = String::new();

    for variant in &variants {
        functions.push_str("    //region ");
        functions.push_str(&variant.snake_case_name);
        functions.push('\n');
        functions.push_str(&generate_sensor_functions_for_coder_type(
            &variant.snake_case_name,
            &variant.default_coder_type,
            &variant.accepted_wrapped_io_data_type
        ));
        functions.push_str("    //endregion\n\n");
    }

    functions
}


 */

//region Motor

#[allow(dead_code)]
fn motor_shared_functions(snake_case_name: &str, accepted_wrapped_io_data_type: &str) -> String {
    format!(
        r#"
    pub fn motor_{snake_case_name}_read_preprocessed_cached_value(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
    ) -> PyResult<Py{accepted_wrapped_io_data_type}>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;

        let unwrapped: {accepted_wrapped_io_data_type} = self.inner.get_motor_cache().{snake_case_name}_read_preprocessed_cached_value(group, channel).map_err(PyFeagiError::from)?;
        Ok(unwrapped.into())
    }}

    pub fn motor_{snake_case_name}_read_postprocessed_cached_value(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
    ) -> PyResult<Py{accepted_wrapped_io_data_type}>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;

        let unwrapped: {accepted_wrapped_io_data_type} = self.inner.get_motor_cache().{snake_case_name}_read_postprocessed_cached_value(group, channel).map_err(PyFeagiError::from)?;
        Ok(unwrapped.into())
    }}

    /*
    pub fn motor_{snake_case_name}_get_single_stage_properties(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        stage_index: PyObject,
    ) -> PyResult<PyPipelineStageProperties>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let stage_index: PipelineStagePropertyIndex = PyPipelineStagePropertyIndex::try_get_from_py_object(py, stage_index).map_err(PyFeagiError::from)?;

        let boxed_stage: Box<dyn PipelineStageProperties + Sync + Send> = self.inner.get_motor_cache().{snake_case_name}_get_single_stage_properties(group, channel, stage_index).map_err(PyFeagiError::from)?;
        Ok(boxed_stage.into())
    }}

    pub fn motor_{snake_case_name}_get_all_stage_properties(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
    ) -> PyResult<Vec<PyPipelineStageProperties>>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;

        let boxed_stages: Vec<Box<dyn PipelineStageProperties + Sync + Send>> = self.inner.get_motor_cache().{snake_case_name}_get_all_stage_properties(group, channel).map_err(PyFeagiError::from)?;
        let mut output: Vec<PyPipelineStageProperties> = Vec::with_capacity(boxed_stages.len());
        for boxed_stage in boxed_stages {{
            output.push(boxed_stage.into());
        }}

        Ok(output)
    }}

    pub fn motor_{snake_case_name}_update_single_stage_properties(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        stage_index: PyObject,
        updating_property: PyObject,
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let stage_index: PipelineStagePropertyIndex = PyPipelineStagePropertyIndex::try_get_from_py_object(py, stage_index).map_err(PyFeagiError::from)?;
        let updating_property: Box<dyn PipelineStageProperties + Sync + Send> = extract_pipeline_stage_properties_from_py(py, updating_property).map_err(PyFeagiError::from)?;

        self.inner.get_motor_cache().{snake_case_name}_update_single_stage_properties(group, channel, stage_index, updating_property).map_err(PyFeagiError::from)?;
        Ok(())
    }}

    pub fn motor_{snake_case_name}_update_all_stage_properties(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        updated_pipeline_stage_properties: Vec<PyObject>
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let mut stages: Vec<Box<dyn PipelineStageProperties + Sync + Send>> = Vec::with_capacity(updated_pipeline_stage_properties.len());
        for py_stage in updated_pipeline_stage_properties {{
            let stage: Box<dyn PipelineStageProperties + Sync + Send> = extract_pipeline_stage_properties_from_py(py, py_stage).map_err(PyFeagiError::from)?;
            stages.push(stage);
        }}

        self.inner.get_motor_cache().{snake_case_name}_update_all_stage_properties(group, channel, stages).map_err(PyFeagiError::from)?;
        Ok(())
    }}

    pub fn motor_{snake_case_name}_replace_single_stage(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        stage_index: PyObject,
        replacing_property: PyObject,
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let stage_index: PipelineStagePropertyIndex = PyPipelineStagePropertyIndex::try_get_from_py_object(py, stage_index).map_err(PyFeagiError::from)?;
        let replacing_property: Box<dyn PipelineStageProperties + Sync + Send> = replacing_property.into().map_err(PyFeagiError::from)?;

        self.inner.get_motor_cache().{snake_case_name}_replace_single_stage(group, channel, stage_index, replacing_property).map_err(PyFeagiError::from)?;
        Ok(())
    }}

    pub fn motor_{snake_case_name}_replace_all_stages(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        new_pipeline_stage_properties: Vec<PyObject>
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let mut stages: Vec<Box<dyn PipelineStageProperties + Sync + Send>> = Vec::with_capacity(new_pipeline_stage_properties.len());
        for py_stage in new_pipeline_stage_properties {{
            let stage: Box<dyn PipelineStageProperties + Sync + Send> = extract_pipeline_stage_properties_from_py(py, py_stage).map_err(PyFeagiError::from)?;
            stages.push(stage);
        }}

        self.inner.get_motor_cache().{snake_case_name}_replace_all_stages(group, channel, stages).map_err(PyFeagiError::from)?;
        Ok(())
    }}

    /*
    pub fn motor_{snake_case_name}_try_register_motor_callback(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        callback: PyObject,
    ) -> PyResult<PyFeagiSignalIndex>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;

        // Note: Callback handling would need special implementation for Python callbacks
        // This is a placeholder that would need proper PyO3 callback wrapping
        let signal_index = self.inner.get_motor_cache().{snake_case_name}_try_register_motor_callback(
            group,
            channel,
            |_| {{ /* Python callback wrapper */ }}
        ).map_err(PyFeagiError::from)?;

        Ok(signal_index.into())
    }}
    */
    "#,
    )
}

#[allow(dead_code)]
fn motor_unique_functions(snake_case_name: &str, accepted_wrapped_io_data_type: &str) -> String {
    // This function generates registration functions based on the coder type.

    let percentage_1d = format!(
        r#"
    pub fn motor_{snake_case_name}_register(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        number_of_channels: PyObject,
        frame_change_handling: PyObject,
        z_neuron_depth: PyObject,
        percentage_neuron_positioning: PyObject
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
        let frame_change_handling: FrameChangeHandling = PyFrameChangeHandling::try_get_from_py_object(py, frame_change_handling).map_err(PyFeagiError::from)?;
        let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;
        let percentage_neuron_positioning: PercentageNeuronPositioning = PyPercentageNeuronPositioning::try_get_from_py_object(py, percentage_neuron_positioning).map_err(PyFeagiError::from)?;

        self.inner.get_motor_cache().{snake_case_name}_register(group, number_of_channels, frame_change_handling, z_neuron_depth, percentage_neuron_positioning).map_err(PyFeagiError::from)?;
        Ok(())
    }}
"#,
    );

    let _misc_data_functions = format!(
        r#"
    pub fn motor_{snake_case_name}_try_register(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        number_of_channels: PyObject,
        misc_dimensions: PyObject,
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
        let misc_dimensions: MiscDataDimensions = PyMiscDataDimensions::try_get_from_py_object(py, misc_dimensions).map_err(PyFeagiError::from)?;

        self.inner.motor_{snake_case_name}_try_register(group, number_of_channels, misc_dimensions).map_err(PyFeagiError::from)?;
        Ok(())
    }}
"#,
    );

    // Match on coder type to generate appropriate function
    // Currently all types use the same template, but each can be customized independently
    match accepted_wrapped_io_data_type {
        "Percentage" => percentage_1d,
        "Percentage_3D" => percentage_1d,    // TODO
        "SignedPercentage" => percentage_1d, // TODO
        "MiscData" => percentage_1d,
        "GazeProperties" => percentage_1d,
        // Default case for any future types
        _ => {
            println!(
                "cargo:warning=Unknown coder type '{}', using default template",
                accepted_wrapped_io_data_type
            );
            percentage_1d
        }
    }
}

//endregion

#[allow(dead_code)]
fn generate_sensor_functions_for_coder_type(
    snake_case_name: &str,
    coder_type: &str,
    accepted_wrapped_io_data_type: &str,
) -> String {
    // This function generates registration functions based on the coder type.

    let percentage_functions = format!(
        r#"
    pub fn sensor_{snake_case_name}_try_register(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        number_of_channels: PyObject,
        z_neuron_depth: PyObject
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
        let z_neuron_depth: NeuronDepth = PyNeuronDepth::try_get_from_py_object(py, z_neuron_depth).map_err(PyFeagiError::from)?;

        self.inner.sensor_{snake_case_name}_try_register(group, number_of_channels, z_neuron_depth).map_err(PyFeagiError::from)?;
        Ok(())
    }}

    pub fn sensor_{snake_case_name}_try_write(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        data: PyObject,
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let data: {accepted_wrapped_io_data_type} = Py{accepted_wrapped_io_data_type}::try_get_from_py_object(py, data).map_err(PyFeagiError::from)?;

        self.inner.sensor_{snake_case_name}_try_write(group, channel, data).map_err(PyFeagiError::from)?;
        Ok(())
    }}

    pub fn sensor_{snake_case_name}_try_read_postprocessed_cache_value(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
    ) -> PyResult<Py{accepted_wrapped_io_data_type}>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;

        let unwrapped: {accepted_wrapped_io_data_type} = self.inner.sensor_{snake_case_name}_try_read_postprocessed_cache_value(group, channel).map_err(PyFeagiError::from)?;
        Ok(unwrapped.into())
    }}

    /*
    pub fn sensor_{snake_case_name}_try_get_single_stage_properties(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        stage_index: PyObject,
    ) -> PyResult<PyPipelineStageProperties>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let stage_index: PipelineStagePropertyIndex = PyPipelineStagePropertyIndex::try_get_from_py_object(py, stage_index).map_err(PyFeagiError::from)?;

        let boxed_stage: Box<dyn PipelineStageProperties + Sync + Send> = self.inner.sensor_{snake_case_name}_try_get_single_stage_properties(group, channel, stage_index).map_err(PyFeagiError::from)?;
        Ok(boxed_stage.into())
    }}

    pub fn sensor_{snake_case_name}_try_get_all_stage_properties(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
    ) -> PyResult<Vec<PyPipelineStageProperties>>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;

        let boxed_stages: Vec<Box<dyn PipelineStageProperties + Sync + Send>> = self.inner.sensor_{snake_case_name}_try_get_all_stage_properties(group, channel).map_err(PyFeagiError::from)?;
        let mut output: Vec<PyPipelineStageProperties> = Vec::with_capacity(boxed_stages.len());
        for boxed_stage in boxed_stages {{
            output.push(boxed_stage.into());
        }}

        Ok(output)
    }}

    pub fn sensor_{snake_case_name}_try_update_single_stage_properties(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        stage_index: PyObject,
        updating_property: PyObject,
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let stage_index: PipelineStagePropertyIndex = PyPipelineStagePropertyIndex::try_get_from_py_object(py, stage_index).map_err(PyFeagiError::from)?;
        let updating_property: Box<dyn PipelineStageProperties + Sync + Send> = extract_pipeline_stage_properties_from_py(py, updating_property).map_err(PyFeagiError::from)?;

        self.inner.sensor_{snake_case_name}_try_update_single_stage_properties(group, channel, stage_index, updating_property).map_err(PyFeagiError::from)?;
        Ok(())
    }}

    pub fn sensor_{snake_case_name}_try_update_all_stage_properties(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        updated_pipeline_stage_properties: Vec<PyObject>
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let mut stages: Vec<Box<dyn PipelineStageProperties + Sync + Send>> = Vec::with_capacity(updated_pipeline_stage_properties.len());
        for py_stage in updated_pipeline_stage_properties {{
            let stage: Box<dyn PipelineStageProperties + Sync + Send> = extract_pipeline_stage_properties_from_py(py, py_stage).map_err(PyFeagiError::from)?;
            stages.push(stage);
        }}

        self.inner.sensor_{snake_case_name}_try_update_all_stage_properties(group, channel, stages).map_err(PyFeagiError::from)?;
        Ok(())
    }}

    pub fn sensor_{snake_case_name}_try_replace_single_stage(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        stage_index: PyObject,
        replacing_property: PyObject,
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let stage_index: PipelineStagePropertyIndex = PyPipelineStagePropertyIndex::try_get_from_py_object(py, stage_index).map_err(PyFeagiError::from)?;
        let replacing_property: Box<dyn PipelineStageProperties + Sync + Send> = replacing_property.into().map_err(PyFeagiError::from)?;

        self.inner.sensor_{snake_case_name}_try_replace_single_stage(group, channel, stage_index, replacing_property).map_err(PyFeagiError::from)?;
        Ok(())
    }}

    pub fn sensor_{snake_case_name}_try_replace_all_stages(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        new_pipeline_stage_properties: Vec<PyObject>
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let mut stages: Vec<Box<dyn PipelineStageProperties + Sync + Send>> = Vec::with_capacity(new_pipeline_stage_properties.len());
        for py_stage in new_pipeline_stage_properties {{
            let stage: Box<dyn PipelineStageProperties + Sync + Send> = extract_pipeline_stage_properties_from_py(py, py_stage).map_err(PyFeagiError::from)?;
            stages.push(stage);
        }}

        self.inner.sensor_{snake_case_name}_try_replace_all_stages(group, channel, stages).map_err(PyFeagiError::from)?;
        Ok(())
    }}
    */

"#,
    );

    let misc_data_functions = format!(
        r#"
    pub fn sensor_{snake_case_name}_try_register(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        number_of_channels: PyObject,
        misc_dimensions: PyMiscDataDimensions,
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
        let misc_dimensions: MiscDataDimensions = misc_dimensions.into();

        self.inner.sensor_{snake_case_name}_try_register(group, number_of_channels, misc_dimensions).map_err(PyFeagiError::from)?;
        Ok(())
    }}

    pub fn sensor_{snake_case_name}_try_write(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        data: PyObject,
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let data: {accepted_wrapped_io_data_type} = Py{accepted_wrapped_io_data_type}::try_get_from_py_object(py, data).map_err(PyFeagiError::from)?;

        self.inner.sensor_{snake_case_name}_try_write(group, channel, data).map_err(PyFeagiError::from)?;
        Ok(())
    }}

    pub fn sensor_{snake_case_name}_try_read_postprocessed_cache_value(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
    ) -> PyResult<Py{accepted_wrapped_io_data_type}>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;

        let unwrapped: {accepted_wrapped_io_data_type} = self.inner.sensor_{snake_case_name}_try_read_postprocessed_cache_value(group, channel).map_err(PyFeagiError::from)?;
        Ok(unwrapped.into())
    }}

    /*
    pub fn sensor_{snake_case_name}_try_get_single_stage_properties(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        stage_index: PyObject,
    ) -> PyResult<PyPipelineStageProperties>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let stage_index: PipelineStagePropertyIndex = PyPipelineStagePropertyIndex::try_get_from_py_object(py, stage_index).map_err(PyFeagiError::from)?;

        let boxed_stage: Box<dyn PipelineStageProperties + Sync + Send> = self.inner.sensor_{snake_case_name}_try_get_single_stage_properties(group, channel, stage_index).map_err(PyFeagiError::from)?;
        Ok(boxed_stage.into())
    }}

    pub fn sensor_{snake_case_name}_try_get_all_stage_properties(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
    ) -> PyResult<Vec<PyPipelineStageProperties>>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;

        let boxed_stages: Vec<Box<dyn PipelineStageProperties + Sync + Send>> = self.inner.sensor_{snake_case_name}_try_get_all_stage_properties(group, channel).map_err(PyFeagiError::from)?;
        let mut output: Vec<PyPipelineStageProperties> = Vec::with_capacity(boxed_stages.len());
        for boxed_stage in boxed_stages {{
            output.push(boxed_stage.into());
        }}

        Ok(output)
    }}

    pub fn sensor_{snake_case_name}_try_update_single_stage_properties(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        stage_index: PyObject,
        updating_property: PyObject,
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let stage_index: PipelineStagePropertyIndex = PyPipelineStagePropertyIndex::try_get_from_py_object(py, stage_index).map_err(PyFeagiError::from)?;
        let updating_property: Box<dyn PipelineStageProperties + Sync + Send> = extract_pipeline_stage_properties_from_py(py, updating_property).map_err(PyFeagiError::from)?;

        self.inner.sensor_{snake_case_name}_try_update_single_stage_properties(group, channel, stage_index, updating_property).map_err(PyFeagiError::from)?;
        Ok(())
    }}

    pub fn sensor_{snake_case_name}_try_update_all_stage_properties(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        updated_pipeline_stage_properties: Vec<PyObject>
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let mut stages: Vec<Box<dyn PipelineStageProperties + Sync + Send>> = Vec::with_capacity(updated_pipeline_stage_properties.len());
        for py_stage in updated_pipeline_stage_properties {{
            let stage: Box<dyn PipelineStageProperties + Sync + Send> = extract_pipeline_stage_properties_from_py(py, py_stage).map_err(PyFeagiError::from)?;
            stages.push(stage);
        }}

        self.inner.sensor_{snake_case_name}_try_update_all_stage_properties(group, channel, stages).map_err(PyFeagiError::from)?;
        Ok(())
    }}

    pub fn sensor_{snake_case_name}_try_replace_single_stage(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        stage_index: PyObject,
        replacing_property: PyObject,
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let stage_index: PipelineStagePropertyIndex = PyPipelineStagePropertyIndex::try_get_from_py_object(py, stage_index).map_err(PyFeagiError::from)?;
        let replacing_property: Box<dyn PipelineStageProperties + Sync + Send> = replacing_property.into().map_err(PyFeagiError::from)?;

        self.inner.sensor_{snake_case_name}_try_replace_single_stage(group, channel, stage_index, replacing_property).map_err(PyFeagiError::from)?;
        Ok(())
    }}

    pub fn sensor_{snake_case_name}_try_replace_all_stages(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        new_pipeline_stage_properties: Vec<PyObject>
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let mut stages: Vec<Box<dyn PipelineStageProperties + Sync + Send>> = Vec::with_capacity(new_pipeline_stage_properties.len());
        for py_stage in new_pipeline_stage_properties {{
            let stage: Box<dyn PipelineStageProperties + Sync + Send> = extract_pipeline_stage_properties_from_py(py, py_stage).map_err(PyFeagiError::from)?;
            stages.push(stage);
        }}

        self.inner.sensor_{snake_case_name}_try_replace_all_stages(group, channel, stages).map_err(PyFeagiError::from)?;
        Ok(())
    }}
    */
"#,
    );

    let image_frame_functions = format!(
        r#"
    pub fn sensor_{snake_case_name}_try_register(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        number_of_channels: PyObject,
        image_properties: PyImageFrameProperties,
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let number_of_channels: CorticalChannelCount = PyCorticalChannelCount::try_get_from_py_object(py, number_of_channels).map_err(PyFeagiError::from)?;
        let image_properties: ImageFrameProperties = image_properties.into();

        self.inner.sensor_{snake_case_name}_try_register(group, number_of_channels, image_properties).map_err(PyFeagiError::from)?;
        Ok(())
    }}

    pub fn sensor_{snake_case_name}_try_write(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        data: PyObject,
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let data: {accepted_wrapped_io_data_type} = Py{accepted_wrapped_io_data_type}::try_get_from_py_object(py, data).map_err(PyFeagiError::from)?;

        self.inner.sensor_{snake_case_name}_try_write(group, channel, data).map_err(PyFeagiError::from)?;
        Ok(())
    }}

    pub fn sensor_{snake_case_name}_try_read_postprocessed_cache_value(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
    ) -> PyResult<Py{accepted_wrapped_io_data_type}>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;

        let unwrapped: {accepted_wrapped_io_data_type} = self.inner.sensor_{snake_case_name}_try_read_postprocessed_cache_value(group, channel).map_err(PyFeagiError::from)?;
        Ok(unwrapped.into())
    }}

    /*
    pub fn sensor_{snake_case_name}_try_get_single_stage_properties(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        stage_index: PyObject,
    ) -> PyResult<PyPipelineStageProperties>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let stage_index: PipelineStagePropertyIndex = PyPipelineStagePropertyIndex::try_get_from_py_object(py, stage_index).map_err(PyFeagiError::from)?;

        let boxed_stage: Box<dyn PipelineStageProperties + Sync + Send> = self.inner.sensor_{snake_case_name}_try_get_single_stage_properties(group, channel, stage_index).map_err(PyFeagiError::from)?;
        Ok(boxed_stage.into())
    }}

    pub fn sensor_{snake_case_name}_try_get_all_stage_properties(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
    ) -> PyResult<Vec<PyPipelineStageProperties>>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;

        let boxed_stages: Vec<Box<dyn PipelineStageProperties + Sync + Send>> = self.inner.sensor_{snake_case_name}_try_get_all_stage_properties(group, channel).map_err(PyFeagiError::from)?;
        let mut output: Vec<PyPipelineStageProperties> = Vec::with_capacity(boxed_stages.len());
        for boxed_stage in boxed_stages {{
            output.push(boxed_stage.into());
        }}

        Ok(output)
    }}

    pub fn sensor_{snake_case_name}_try_update_single_stage_properties(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        stage_index: PyObject,
        updating_property: PyObject,
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let stage_index: PipelineStagePropertyIndex = PyPipelineStagePropertyIndex::try_get_from_py_object(py, stage_index).map_err(PyFeagiError::from)?;
        let updating_property: Box<dyn PipelineStageProperties + Sync + Send> = extract_pipeline_stage_properties_from_py(py, updating_property).map_err(PyFeagiError::from)?;

        self.inner.sensor_{snake_case_name}_try_update_single_stage_properties(group, channel, stage_index, updating_property).map_err(PyFeagiError::from)?;
        Ok(())
    }}

    pub fn sensor_{snake_case_name}_try_update_all_stage_properties(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        updated_pipeline_stage_properties: Vec<PyObject>
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let mut stages: Vec<Box<dyn PipelineStageProperties + Sync + Send>> = Vec::with_capacity(updated_pipeline_stage_properties.len());
        for py_stage in updated_pipeline_stage_properties {{
            let stage: Box<dyn PipelineStageProperties + Sync + Send> = extract_pipeline_stage_properties_from_py(py, py_stage).map_err(PyFeagiError::from)?;
            stages.push(stage);
        }}

        self.inner.sensor_{snake_case_name}_try_update_all_stage_properties(group, channel, stages).map_err(PyFeagiError::from)?;
        Ok(())
    }}

    pub fn sensor_{snake_case_name}_try_replace_single_stage(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        stage_index: PyObject,
        replacing_property: PyObject,
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let stage_index: PipelineStagePropertyIndex = PyPipelineStagePropertyIndex::try_get_from_py_object(py, stage_index).map_err(PyFeagiError::from)?;
        let replacing_property: Box<dyn PipelineStageProperties + Sync + Send> = replacing_property.into().map_err(PyFeagiError::from)?;

        self.inner.sensor_{snake_case_name}_try_replace_single_stage(group, channel, stage_index, replacing_property).map_err(PyFeagiError::from)?;
        Ok(())
    }}

    pub fn sensor_{snake_case_name}_try_replace_all_stages(
        &mut self,
        py: Python<'_>,
        group: PyObject,
        channel: PyObject,
        new_pipeline_stage_properties: Vec<PyObject>
    ) -> PyResult<()>
    {{
        let group: CorticalUnitIndex = PyCorticalUnitIndex::try_get_from_py_object(py, group).map_err(PyFeagiError::from)?;
        let channel: CorticalChannelIndex = PyCorticalChannelIndex::try_get_from_py_object(py, channel).map_err(PyFeagiError::from)?;
        let mut stages: Vec<Box<dyn PipelineStageProperties + Sync + Send>> = Vec::with_capacity(new_pipeline_stage_properties.len());
        for py_stage in new_pipeline_stage_properties {{
            let stage: Box<dyn PipelineStageProperties + Sync + Send> = extract_pipeline_stage_properties_from_py(py, py_stage).map_err(PyFeagiError::from)?;
            stages.push(stage);
        }}

        self.inner.sensor_{snake_case_name}_try_replace_all_stages(group, channel, stages).map_err(PyFeagiError::from)?;
        Ok(())
    }}
    */

"#,
    );

    // Match on coder type to generate appropriate function
    match coder_type {
        // Percentage types
        "Percentage_Absolute_Linear" => percentage_functions,
        "Percentage_Absolute_Fractional" => percentage_functions,
        "Percentage_Incremental_Linear" => percentage_functions,
        "Percentage_Incremental_Fractional" => percentage_functions,

        // Percentage2D types
        "Percentage2D_Absolute_Linear" => percentage_functions,
        "Percentage2D_Absolute_Fractional" => percentage_functions,
        "Percentage2D_Incremental_Linear" => percentage_functions,
        "Percentage2D_Incremental_Fractional" => percentage_functions,

        // Percentage3D types
        "Percentage3D_Absolute_Linear" => percentage_functions,
        "Percentage3D_Absolute_Fractional" => percentage_functions,
        "Percentage3D_Incremental_Linear" => percentage_functions,
        "Percentage3D_Incremental_Fractional" => percentage_functions,

        // Percentage4D types
        "Percentage4D_Absolute_Linear" => percentage_functions,
        "Percentage4D_Absolute_Fractional" => percentage_functions,
        "Percentage4D_Incremental_Linear" => percentage_functions,
        "Percentage4D_Incremental_Fractional" => percentage_functions,

        // SignedPercentage types
        "SignedPercentage_Absolute_Linear" => percentage_functions,
        "SignedPercentage_Absolute_Fractional" => percentage_functions,
        "SignedPercentage_Incremental_Linear" => percentage_functions,
        "SignedPercentage_Incremental_Fractional" => percentage_functions,

        // SignedPercentage2D types
        "SignedPercentage2D_Absolute_Linear" => percentage_functions,
        "SignedPercentage2D_Absolute_Fractional" => percentage_functions,
        "SignedPercentage2D_Incremental_Linear" => percentage_functions,
        "SignedPercentage2D_Incremental_Fractional" => percentage_functions,

        // SignedPercentage3D types
        "SignedPercentage3D_Absolute_Linear" => percentage_functions,
        "SignedPercentage3D_Absolute_Fractional" => percentage_functions,
        "SignedPercentage3D_Incremental_Linear" => percentage_functions,
        "SignedPercentage3D_Incremental_Fractional" => percentage_functions,

        // SignedPercentage4D types
        "SignedPercentage4D_Absolute_Linear" => percentage_functions,
        "SignedPercentage4D_Absolute_Fractional" => percentage_functions,
        "SignedPercentage4D_Incremental_Linear" => percentage_functions,
        "SignedPercentage4D_Incremental_Fractional" => percentage_functions,

        // MiscData types
        "MiscData_Absolute" => misc_data_functions,
        "MiscData_Incremental" => misc_data_functions,

        // ImageFrame types
        "ImageFrame_Absolute" => image_frame_functions,
        "ImageFrame_Incremental" => image_frame_functions,

        // Default case for any future types
        _ => {
            println!(
                "cargo:warning=Unknown sensor coder type '{}', using default template",
                coder_type
            );
            percentage_functions
        }
    }
}
