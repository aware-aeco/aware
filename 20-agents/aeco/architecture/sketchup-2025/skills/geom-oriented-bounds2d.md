---
name: yard-geom-oriented-bounds2d
description: Geom::OrientedBounds2d API reference (YARD)
---

# Geom::OrientedBounds2d API reference

The OrientedBounds2d class is a bounding box represented by four Point2d objects, upper left, upper right, lower left and lower right positions.

## Methods

- `==` — The #== method checks to see if the two Geom::OrientedBounds2ds are equal. This checks whether the point values are the same.
- `lower_left` — The #lower_left method returns the Point2d of the lower left corner of the Geom::OrientedBounds2d.
- `lower_right` — The #lower_right method returns the Point2d of the lower right corner of the Geom::OrientedBounds2d.
- `to_a` — The #to_a method returns an array which contains the Point2d that define the Geom::OrientedBounds2d.
- `upper_left` — The #upper_left method returns the Point2d of the upper left corner of the Geom::OrientedBounds2d.
- `upper_right` — The #upper_right method returns the Point2d of the upper right corner of the Geom::OrientedBounds2d.
