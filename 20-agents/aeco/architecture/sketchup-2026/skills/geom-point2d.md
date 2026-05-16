---
name: yard-geom-point2d
description: Geom::Point2d API reference (YARD)
---

# Geom::Point2d API reference

The Point2d class allows you to work with a point in 2D space. Point2d is a series of values representing x and y coordinates.

## Methods

- `initialize` — The new method creates a new Geom::Point2d.
- `+` — The #+ operator is a simple way to add to the current x and y values of the Geom::Point2d, or to set the values of the Geom::Point2d by adding a Vector2d to the Geom::Point2d.
- `-` — The #- operator is a simple way to subtract from the current x and y values of the Geom::Point2d.
- `==` — The #== method compares two points for equality. This uses the standard SketchUp tolerance to determine if two points are the same.
- `[]` — The #[] method returns the value of the Geom::Point2d at the specified index.
- `[]=` — The #[]= method sets the x or y value of the Geom::Point2d based on the specific index of the value.
- `clone` — The #clone method creates another point identical to the Geom::Point2d being cloned.
- `distance` — The #distance method computes the distance from the Geom::Point2d to another Geom::Point2d.
- `inspect` — The #inspect method formats the Geom::Point2d as a string.
- `offset` — The #offset method offsets the Geom::Point2d by a Vector2d and returns a new Geom::Point2d. If distance is provided, it must be non-zero.
- `offset!` — The #offset! method offsets the Geom::Point2d by a Vector2d. The Geom::Point2d itself is modified. The length of the vector must not be zero.
- `set!` — The #set! method sets the values of the Geom::Point2d.
- `to_a` — The #to_a method converts the Geom::Point2d to an array of 2 numbers.
- `to_s` — The #to_s method returns a string representation of the Geom::Point2d.
- `transform` — The #transform method applies a transformation to a point, returning a new point. The original point is unchanged by this method.
- `transform!` — The #transform! method applies a transformation to a point. The point itself is modified.
- `vector_to` — The #vector_to method returns the vector between points.
- `x` — The #x method returns the x value of the Geom::Point2d.
- `x=` — The #x= method sets the x value of the Geom::Point2d.
- `y` — The #y method returns the y value of the Geom::Point2d.
- `y=` — The #y= method sets the y value of the Geom::Point2d.
