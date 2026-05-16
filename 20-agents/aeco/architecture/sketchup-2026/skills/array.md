---
name: yard-array
description: Array API reference (YARD)
---

# Array API reference

The SketchUp Array class adds additional methods to the standard Ruby Array class. Specifically, it contains methods allowing an array to behave just as a Geom::Vector3d or Geom::Point3d object (which can be thought of as arrays of 3 coordinate values). Therefore, you can use the Array class in place of a Geom::Point3d or Geom::Vector3d as a way to pass coordinate values.

## Methods

- `cross` — The #cross method is used to compute the cross product between two vectors.
- `distance` — The #distance method is used to compute the distance between two points.
- `distance_to_line` — The #distance_to_line method is used to compute the distance from a Geom::Point3d object to a line.
- `distance_to_plane` — The #distance_to_plane method is used to compute the distance from a Geom::Point3d object to a plane.
- `dot` — The #dot method is used to compute the dot product between two vectors.
- `normalize` — The arguments and return value will be converted to a floating point value. (Unlike in the Geom::Vector3d#normalize! method.)
- `normalize!` — The #normalize! method is used to normalize a vector in place (setting its length to 1).
- `offset` — The #offset method is used to offset a point by a vector. it returns a new array rather than modifying the original in place.
- `offset!` — The #offset! method is used to offset a point by a vector. The array is modified in place.
- `on_line?` — The #on_line? method is used to determine if a Geom::Point3d object is on a line.
- `on_plane?` — The #on_plane? method is used to determine if a Geom::Point3d object is on a plane (to within SketchUp's standard floating point tolerance).
- `project_to_line` — The #project_to_line method is used to retrieve the projection of a Geom::Point3d object onto a line.
- `project_to_plane` — The #project_to_plane method retrieves the projection of a Geom::Point3d onto a plane.
- `transform` — The #transform method is used to apply a Geom::Transformation or Geom::Transformation2d object to a Geom::Point3d or Geom::Point2d object defined by an Array object.
- `transform!` — This method modifies the original.
- `vector_to` — The #vector_to method is used to create an array as a vector from one point to a second point.
- `x` — The #x method retrieves the x coordinate.
- `x=` — The #x= method sets the x coordinate.
- `y` — The #y method retrieves the y coordinate.
- `y=` — The #y= method sets the y coordinate.
- `z` — The #z method retrieves the z coordinate.
- `z=` — The #z= method sets the z coordinate.
