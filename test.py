
import feagi_data_processing as fdp
import numpy as np

a = fdp.brain_input.vision.

a = fdp.data_vision.cropping_utils.CornerPoints((0,1), (3,4))
print(a.lower_right())

b = fdp.data_vision.peripheral_segmentation.SegmentedVisionCenterProperties((0.2, 0.3), (0.4, 0.6))
print(b.calculate_pixel_coordinates_of_center_corners((1000, 1000)).lower_right())
print("pause")