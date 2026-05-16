---
name: yard-geom-point3d
description: Geom::Point3d API reference (YARD)
---

# Geom::Point3d API reference

The Point3d class allows you to work with a point in 3D space. The point is basically just a series of values representing x, y and z coordinates.

## Methods

- `initialize` — The new method is used to create a new 3D point.
- `linear_combination` — The linear_combination method is used to create a new point as a linear combination of two points.
- `+` — The #+ operator is a fast way to add to the current x, y and z values of a vector.
- `-` — The '-' operator is a fast way to subtract from the current x, y and z values of a point.
- `<` — The comparison is performed in the order x, y and z coordinates.
- `==` — The == method is used to compare two points for equality.
- `[]` — The [] method is used to retrieve the value of the point at the specified index.
- `[]=` — The []= method is used to set the x, y, or z value of the point based on the specific index of the value.
- `clone` — The clone method is used to create another point identical to the point being cloned.
- `distance` — The distance method is used to compute the distance from a point to another point.
- `distance_to_line` — This function returns a `Float` value, not a `Length`.
- `distance_to_plane` — This function returns a `Float` value, not a `Length`.
- `inspect` — The inspect method is used to format a 3D point as a string.
- `offset` — The offset method is used to offset a point by a vector and return a new point. The length of the vector must not be zero.
- `offset!` — The offset! method is used to offset a point by a vector. The point itself is modified.
- `on_line?` — The on_line? method is used to determine if the point is on a line.
- `on_plane?` — The on_plane? method is used to determine if the point is on a plane.
- `project_to_line` — The project_to_line method is used to retrieve the point on a line that is closest to this point.
- `project_to_plane` — The project_to_plane method is used to retrieve the point on a plane that is closest to the point.
- `set!` — The #set! method is used to set the values of the Point3d.
- `to_a` — The to_a method is used to convert the point to an array of 3 numbers
- `to_s` — The to_s method is used to retrieve a string representation of a point.
- `transform` — Apply a Transformation to a point, returning a new point. The original vector is unchanged by this method.
- `transform!` — Apply a Transformation to a point. The point itself is modified.
- `vector_to` — The vector_to team method retrieves the vector between points.
- `x` — The #x method retrieves the x value of the 3D point.
- `x=` — The #x= method is used to set the x value of a 3D point.
- `y` — The #y method retrieves the y value of the 3D point.
- `y=` — The #y= method is used to set the y value of a 3D point.
- `z` — The #z method retrieves the z value of the 3D point.
- `z=` — The #z= method is used to set the z value of a 3D point.
