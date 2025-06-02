
import feagi_data_processing as fdp
import numpy as np

print("start")

cortical_id = fdp.cortical_data.CorticalID("abcdef")

fdp.




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