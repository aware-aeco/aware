---
name: yard-geom-bounds2d
description: Geom::Bounds2d API reference (YARD)
---

# Geom::Bounds2d API reference

The bounds2d class represents an axis aligned bounding box represented by two Point2d objects, upper left and lower right positions. The units utilized in the creation and modification Bounds2d are inches.

## Methods

- `initialize` — The #initialize method creates a new Geom::Bounds2d.
- `==` — The #== method checks to see if the two Geom::Bounds2ds are equal. This checks whether the point values are the same
- `height` — The #height method returns the height of the Geom::Bounds2d.
- `lower_right` — The #lower_right method returns the Point2d of the lower right corner of the Geom::Bounds2d.
- `set!` — The #set! method sets the Geom::Bounds2d to match another one. The argument is anything that can be converted into a Geom::Bounds2d.
- `to_a` — The #to_a method returns an array which contains the Point2d that define the Geom::Bounds2d.
- `upper_left` — The #upper_left method returns the Point2d of the upper left corner of the Geom::Bounds2d.
- `width` — The #width method returns the width of the Geom::Bounds2d.
