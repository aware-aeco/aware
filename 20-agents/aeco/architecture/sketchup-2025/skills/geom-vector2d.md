---
name: yard-geom-vector2d
description: Geom::Vector2d API reference (YARD)
---

# Geom::Vector2d API reference

The Vector2d class represents vectors in a 2 dimensional space.

## Methods

- `initialize` — The new method creates a new Geom::Vector2d.
- `%` — The #% method returns the dot product between two Geom::Vector2d. This is an alias of the dot method.
- `*` — The #* method returns the cross product between two Geom::Vector2d. This is an alias of the cross method.
- `+` — The #+ method adds a Geom::Vector2d to this one.
- `-` — The #- method subtracts a Geom::Vector2d from this one.
- `==` — The #== method returns whether two Geom::Vector2d are equal within tolerance.
- `[]` — The #[] method returns the value of the Geom::Vector2d at the specified index.
- `[]=` — The #[]= method sets the x or y value of the Geom::Vector2d based on the specific index of the value.
- `angle_between` — The #angle_between method computes the angle in radians between the Geom::Vector2d and another Geom::Vector2d.
- `clone` — The #clone method makes a copy of the Geom::Vector2d. This method is equivalent to vec2 = Geom::Vector2d.new(vec).
- `cross` — The #* method returns the cross product between two Geom::Vector2d. This is an alias of the cross method.
- `dot` — The #% method returns the dot product between two Geom::Vector2d. This is an alias of the dot method.
- `inspect` — The #inspect method formats the Geom::Vector2d as a string.
- `length` — The #length method returns the length of the Geom::Vector2d.
- `length=` — The #length= method sets the length of the Geom::Vector2d. The new length must not be 0.
- `normalize` — The #normalize method returns a Geom::Vector2d that is a unit vector of the Geom::Vector2d.
- `normalize!` — The #normalize! method converts a Geom::Vector2d vector into a unit vector. Another way to do this is vector.length = 1
- `parallel?` — The #parallel? method determines if the Geom::Vector2d is parallel to another Geom::Vector2d to within tolerance.
- `perpendicular?` — The #perpendicular? method determines if the Geom::Vector2d is perpendicular to another Geom::Vector2d to within tolerance.
- `reverse` — The #reverse method returns a new Geom::Vector2d that is the reverse of the Geom::Vector2d, leaving the original unchanged.
- `reverse!` — The #reverse! method reverses the Geom::Vector2d in place.
- `same_direction?` — The #same_direction? method determines if the Geom::Vector2d is parallel to and in the same direction as another Geom::Vector2d within tolerance.
- `set!` — The #set! method sets the values of the Geom::Vector2d.
- `to_a` — The #to_a method retrieves the coordinates of the Geom::Vector2d in an Array.
- `to_s` — The #to_s method returns a string representation of the Geom::Vector2d.
- `transform` — The #transform method applies a transformation to a vector, returning a new vector. The original vector is unchanged by this method.
- `transform!` — The #transform! method applies a transformation to a vector. The vector itself is modified.
- `unit_vector?` — The #unit_vector? method returns whether the Geom::Vector2d is a unit vector. This is equivalent to vector.length == 1.0
- `valid?` — The #valid? method verifies if a Geom::Vector2d is valid. A Geom::Vector2d is valid if its length is not zero.
- `x` — The #x method retrieves the x value of the Geom::Vector2d.
- `x=` — The #x= method sets the x coordinate of the Geom::Vector2d.
- `y` — The #y method retrieves the y value of the Geom::Vector2d.
- `y=` — The #y= method sets the y coordinate of the Geom::Vector2d.
