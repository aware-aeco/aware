---
name: yard-geom-transformation2d
description: Geom::Transformation2d API reference (YARD)
---

# Geom::Transformation2d API reference

The #initialize method creates a new Geom::Transformation2d. You can use this method or one of the more specific methods for creating specific kinds of Geom::Transformation2d.

## Methods

- `initialize` — The #initialize method creates a new Geom::Transformation2d. You can use this method or one of the more specific methods for creating specific kinds of Geom::Transformation2d.
- `rotation` — The rotation method is used to create a transformation that does rotation about a point.
- `scaling` — The scaling method is used to create a transformation that does scaling.
- `translation` — The translation method is used to create a transformation that does translation.
- `*` — The #* method is used to do matrix multiplication using the transform.
- `==` — The #== method checks to see if the two Geom::Transformation2ds are equal. This checks whether the values of the transformations are the same.
- `clone` — The #clone method creates a copy of the Geom::Transformation2d.
- `identity?` — The #identity? method determines if the Geom::Transformation2d is the IDENTITY_2D transform.
- `inverse` — The #inverse method is used to retrieve the inverse of a transformation.
- `invert!` — The #invert! method sets the transformation to its inverse.
- `set!` — The #set! method sets the Geom::Transformation2d to match another one. The argument is anything that can be converted into a Geom::Transformation2d.
- `to_a` — The #to_a method returns a 6 element array which contains the values that define the Transformation2d.
