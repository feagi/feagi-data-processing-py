
import feagi_data_processing as fdp
import numpy as np

print("start")

# cortical areas and their neurons
cortical_id_a = fdp.cortical_data.CorticalID("AAAAAA")
neuron_a_1 = fdp.neuron_data.neurons.NeuronXYZP(1,2,3,0.5)
neuron_a_2 = fdp.neuron_data.neurons.NeuronXYZP(4,5,6,0.2)
neurons_a = fdp.neuron_data.neuron_arrays.NeuronXYZPArrays(2)
neurons_a.add_neuron(neuron_a_1)
neurons_a.add_neuron(neuron_a_2)

cortical_id_b = fdp.cortical_data.CorticalID("BBBBBB")
neuron_b_1 = fdp.neuron_data.neurons.NeuronXYZP(8,9,10,0.5)
neuron_b_2 = fdp.neuron_data.neurons.NeuronXYZP(11,12,13,0.2)
neurons_b = fdp.neuron_data.neuron_arrays.NeuronXYZPArrays(2)
neurons_b.add_neuron(neuron_b_1)
neurons_b.add_neuron(neuron_b_2)

cortical_id_c  = fdp.cortical_data.CorticalID("CCCCCC")
neurons_c_x = np.asarray([1,2,3], dtype=np.uint32)
neurons_c_y = np.asarray([4,5,6], dtype=np.uint32)
neurons_c_z = np.asarray([7,8,9], dtype=np.uint32)
neurons_c_p = np.asarray([0.1,0.2,0.3], dtype=np.float32)
neurons_c = fdp.neuron_data.neuron_arrays.NeuronXYZPArrays.new_from_numpy(neurons_c_x, neurons_c_y, neurons_c_z, neurons_c_p)
copy_back_c = neurons_c.copy_as_tuple_of_numpy()


# list_of_neurons = neurons_a.copy_as_neuron_xyzp_vec() # example, getting as vector

generated_mapped_neuron_data = fdp.neuron_data.neuron_mappings.CorticalMappedXYZPNeuronData()
generated_mapped_neuron_data.insert(cortical_id_a, neurons_a)
generated_mapped_neuron_data.insert(cortical_id_b, neurons_b)
generated_mapped_neuron_data.insert(cortical_id_c, neurons_c)

for (c_id, neurons) in generated_mapped_neuron_data.iter_easy():
    print("breakpoint")

byte_data = generated_mapped_neuron_data.as_new_feagi_byte_structure()
bytes = byte_data.copy_as_bytes()

received_byte_data = fdp.byte_structures.FeagiByteStructure(bytes)
received_cortical_mappings = fdp.neuron_data.neuron_mappings.CorticalMappedXYZPNeuronData.from_feagi_byte_structure(received_byte_data)
for (c_id, neurons) in received_cortical_mappings.iter_easy():
    print("breakpoint")

print("pause")
#fake_image_source = np.zeros((2000,2000,3), dtype=np.float32)
#[1,1,1] = 1

#image_source_frame = fdp.brain_input.vision.single_frame.ImageFrame.from_array(fake_image_source)
#image_segment_center_properties = fdp.brain_input.vision.peripheral_segmentation.SegmentedVisionCenterProperties(
#    (0.5, 0.5), (0.5, 0.5)
#)
#image_segment_resolutions = fdp.brain_input.vision.peripheral_segmentation.SegmentedVisionTargetResolutions(
#    (5, 5),
#    (5, 5),
#    (5, 5),
#    (5, 5),
#    (5, 5),
#    (5, 5),
#    (5, 5),
#    (5, 5),
#    (10, 10),
#)

#image_segmented = fdp.brain_input.vision.peripheral_segmentation.SegmentedVisionFrame(image_source_frame, image_segment_center_properties, image_segment_resolutions)
#bytes = image_segmented.direct_export_as_byte_neuron_potential_categorical_xyz(0)


#{"cortical_ID": (list(int x y z), float potential)}


print("pause")