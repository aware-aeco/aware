---
name: yard-sketchup-dimension-linear
description: Sketchup::DimensionLinear API reference (YARD)
---

# Sketchup::DimensionLinear API reference

The DimensionLinear class represents linear dimensions.

## Methods

- `aligned_text_position` — The #aligned_text_position method returns the text position for dimensions with aligned text (i.e. has_aligned_text? returns true). Valid values are class constants:
- `aligned_text_position=` — The #aligned_text_position= method is used to set the text position for dimensions with aligned text (i.e. has_aligned_text? returns true). Valid values are class constants:
- `end` — The end method returns the point or entity the dimension is referencing at its end.
- `end=` — The end= method is used to set the end point of the dimension and/or the entity it is referencing.
- `end_attached_to` — The #end_attached_to method will return the attached end point via an array containing the InstancePath and Geom::Point3d.
- `end_attached_to=` — The #end_attached_to= method will attach the ending point to the InstancePath and Geom::Point3d.
- `offset_vector` — The offset_vector method returns the parallel offset vector from the reference line to the dimension line measured from the 'start' reference point.
- `offset_vector=` — The offset_vector= method is used to set the parallel offset vector from the reference line to the dimension line measured from the 'start' reference point.
- `start` — The start method returns the point or entity the dimension is referencing at its start.
- `start=` — The start= method is used to set the start point of the dimension and/or the entity it is referencing.
- `start_attached_to` — The #start_attached_to method will return the attached start point via an array containing the InstancePath and Geom::Point3d.
- `start_attached_to=` — The #start_attached_to= method will attach the starting point to the InstancePath and Geom::Point3d.
- `text_position` — The #text_position method returns the position of the text along the dimension line. Valid values are class constants:
- `text_position=` — The #text_position= method is used to set the position of the text along the dimension line. Valid values are class constants:
