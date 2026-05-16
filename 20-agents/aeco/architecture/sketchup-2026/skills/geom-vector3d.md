---
name: yard-geom-vector3d
description: Geom::Vector3d API reference (YARD)
---

# Geom::Vector3d API reference

The Vector3d class is used to represent vectors in a 3 dimensional space. Vectors in SketchUp have a direction and a length, but not a starting point.

## Methods

- `initialize` — The new method is used to create a new vector.
- `linear_combination` — The Geom#linear_combination method is used to create a new vector as a linear combination of other vectors. This method is generally used to get a vector at some percentage between two vectors.
- `%` — The #% method is used to compute the dot product between two vectors.
- `*` — The #* method is used to compute the cross product between two vectors.
- `+` — The #+ method is used to add a vector to this one.
- `-` — The #- method is used to subtract a vector from this one.
- `<` — The #< compare method is used to compare two vectors to determine if the left-hand vector is less than the right-hand vector.
- `==` — The #== method is used to determine if two vectors are equal to within tolerance.
- `[]` — The #[] method is used to access the coordinates of a vector as if it was an Array. The index must be 0, 1 or 2.
- `[]=` — The #[]= method is used to set the coordinates of a vector as if it was an Array. The value of i must be 0, 1 or 2.
- `angle_between` — The #angle_between method is used to compute the angle (in radians) between this vector and another vector.
- `axes` — The #axes method is used to compute an arbitrary set of axes with the given vector as the z-axis direction.
- `clone` — The #clone method is used to make a copy of a vector.
- `cross` — The #cross method is used to compute the cross product between two vectors.
- `dot` — The #dot method is used to compute the dot product between two vectors.
- `inspect` — The #inspect method is used to inspect the contents of a vector as a friendly string.
- `length` — The #length method is used to retrieve the length of the vector.
- `length=` — The #length= method is used to set the length of the vector. The length must be greater than 0.
- `normalize` — The #normalize method is used to return a vector that is a unit vector of another.
- `normalize!` — The #normalize! method is used to convert a vector into a unit vector, in place.
- `parallel?` — The #parallel? method determines if two Geom::Vector3ds are parallel within a tolerance. Two vectors are parallel if there exists a scalar multiple between them.
- `perpendicular?` — The #perpendicular? method determines if two Geom::Vector3ds are perpendicular within a tolerance. Two vectors are considered perpendicular if their dot product is zero.
- `reverse` — The #reverse method is used to return a new vector that is the reverse of this vector, while leaving the original unchanged.
- `reverse!` — The #reverse! method is used to reverse the vector in place.
- `samedirection?` — The #samedirection? method is used to determine if this vector is parallel to and in the same direction as another vector to within tolerance.
- `set!` — The #set! method is used to set the coordinates of the vector.
- `to_a` — The #to_a method retrieves the coordinates of the vector in an Array[x, y, z].
- `to_s` — The #to_s method is used to format the vector as a String.
- `transform` — The #transform method applies a Transformation to a vector, returning a new vector. The original vector is unchanged by this method.
- `transform!` — The #transform! method applies a Transformation to a vector. The vector itself is modified.
- `unitvector?` — The #unitvector? method is used to see if the vector is a unit vector.
- `valid?` — The #valid? method is used to verify if a vector is valid. A vector is valid if its length is not zero.
- `x` — The #x method is used to retrieve the x coordinate of the vector.
- `x=` — The #x= method is used to set the x coordinate of the vector.
- `y` — The #y method is used to retrieve the y coordinate of the vector.
- `y=` — Set the #y= coordinate of the vector.
- `z` — Get the #z coordinate of the vector.
- `z=` — Set the #z= coordinate of the vector.
