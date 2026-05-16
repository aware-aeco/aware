---
name: yard-geom-transformation
description: Geom::Transformation API reference (YARD)
---

# Geom::Transformation API reference

Transformations are a standard construct in the 3D world for representing the position, rotation, and sizing of a given entity. In the SketchUp world, Sketchup::ComponentInstance and Sketchup::Group have a .transformation method that reports their current state and various methods (.move!, transformation=, etc.) that allow them to be manipulated.

## Methods

- `initialize` — You can use this method or one of the more specific methods for creating specific kinds of Transformations.
- `axes` — The axes method creates a transformation that goes from world coordinates to an arbitrary coordinate system defined by an origin and three axis vectors.
- `interpolate` — The interpolate method is used to create a new transformation that is the result of interpolating between two other transformations.
- `rotation` — The rotation method is used to create a transformation that does rotation about an axis.
- `scaling` — The scaling method is used to create a transformation that does scaling.
- `translation` — The translation method is used to create a transformation that does translation.
- `*` — The #* method is used to do matrix multiplication using the transform.
- `clone` — The #clone method is used to create a copy of a transformation.
- `identity?` — As of SketchUp 2018, this now looks at the data to determine if the transformation is identity. Prior to SU2018, this only looks at the flag to see if the transform has not been modified. If the transform has been changed, this will return false even if it is really the identity.
- `inverse` — As of SketchUp 2026, this will raise an error if the Geom::Transformation is not invertible. Prior to 2026 this would silently attempt to invert the transformation possibly returning in an invalid transformation.
- `invert!` — As of SketchUp 2026, this will raise an error if the Geom::Transformation is not invertible. Prior to 2026 this would silently attempt to invert the transformation possibly creating in an invalid transformation.
- `origin` — The #origin method retrieves the origin of a rigid transformation.
- `set!` — The #set! method is used to set this transformation to match another one.
- `to_a` — The #to_a method retrieves a 16 element array which contains the values that define the transformation.
- `xaxis` — The #xaxis method retrieves the x axis of a rigid transformation.
- `yaxis` — The #yaxis method retrieves the y axis of a rigid transformation.
- `zaxis` — The #zaxis method retrieves the z axis of a rigid transformation.
