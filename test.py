
import feagi_data_processing as fdp
import numpy as np

print("start")





cortical_id = fdp.cortical_data.CorticalID("abcdef")

neuron_1 = fdp.neuron_data.neurons.NeuronXYZP(1,2,3,0.5)
neuron_2 = fdp.neuron_data.neurons.NeuronXYZP(4,5,6,0.1)



created_neuron_arr = fdp.neuron_data.neuron_arrays.NeuronXYZPArrays(2)
created_neuron_arr.add_neuron(neuron_1)
created_neuron_arr.add_neuron(neuron_2)
print(created_neuron_arr.get_number_of_neurons_used())
list_of_neurons = created_neuron_arr.copy_as_neuron_xyzp_vec()

mapped_neuron_data = fdp.neuron_data.neuron_mappings.CorticalMappedXYZPNeuronData()
mapped_neuron_data.insert(cortical_id, created_neuron_arr)

byte_data = mapped_neuron_data.as_new_feagi_byte_structure()

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