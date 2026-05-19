---
name: allplan-stdreinfshapebuilder-rotationangles
description: This skill encodes the allplan 2025.0 surface of the StdReinfShapeBuilder.RotationAngles namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: RotationAngles.
---

# StdReinfShapeBuilder.RotationAngles

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## RotationAngles (class)

Class describing the rotation of a geometrical object around the three axes (X,Y,Z) in order to transform it from local to global coordinate system or the other way around.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/RotationAngles/)

### Constructors
- `RotationAngles(angle_x: float, angle_y: float, angle_z: float)` — Construct the object by setting all three angles. The angles are given in degrees.

### Methods
#### `__repr__() -> str`

Create a string from the object data

**Remarks:** Create a string from the object data

**Returns:** `str` — string with the object data

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/RotationAngles/#StdReinfShapeBuilder.RotationAngles.RotationAngles.__repr__)

#### `change_rotation() -> RotationAngles`

Inverse rotation direction

**Remarks:** Inverse rotation direction

**Returns:** `RotationAngles` — RotationAngles with the inverted rotation direction

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/RotationAngles/#StdReinfShapeBuilder.RotationAngles.RotationAngles.change_rotation)

#### `get_rotation_matrix() -> Matrix3D`

Get a 3D transformation matrix (4x4) describing the rotation around the origin (0,0,0).

**Remarks:** Get a 3D transformation matrix (4x4) describing the rotation around the origin (0,0,0).

**Returns:** `Matrix3D` — Transformation matrix describing the rotation

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/RotationAngles/#StdReinfShapeBuilder.RotationAngles.RotationAngles.get_rotation_matrix)

### Properties
- `angle_x` (float, get) — Rotation angle around the X axis (in degrees)
- `angle_y` (float, get) — Rotation angle around the Y axis (in degrees)
- `angle_z` (float, get) — Rotation angle around the Z axis (in degrees)

