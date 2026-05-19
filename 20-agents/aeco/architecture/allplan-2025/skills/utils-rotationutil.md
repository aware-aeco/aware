---
name: allplan-utils-rotationutil
description: This skill encodes the allplan 2025.0 surface of the Utils.RotationUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: RotationUtil.
---

# Utils.RotationUtil

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## RotationUtil (class)

Class used to describe the rotation around the three axes (X,Y,Z) in order to transform a geometry from local to global coordinate system or the other way around.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/Utils/RotationUtil/)

### Constructors
- `RotationUtil(angle_x: float, angle_y: float, angle_z: float)` — Construct the object by setting all three angles. The angles are given in degrees.

### Methods
#### `__repr__() -> str`

create a string from the object data

**Remarks:** create a string from the object data

**Returns:** `str` — string with the object data

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/RotationUtil/#Utils.RotationUtil.RotationUtil.__repr__)

#### `get_rotation_matrix(axis_point: Point3D = Point3D()) -> Matrix3D`

Get a 3D transformation matrix (4x4) describing the rotation around the given point by all three angles: x, y and z.

**Remarks:** Get a 3D transformation matrix (4x4) describing the rotation around the given point by all three angles: x, y and z.

**Parameters:**
- `axis_point` (Point3D) — Rotation point. Defaults to the origin (0,0,0).

**Returns:** `Matrix3D` — Transformation matrix describing the rotation

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/RotationUtil/#Utils.RotationUtil.RotationUtil.get_rotation_matrix)

### Properties
- `angle_x` (float, get) — Rotation angle around the X axis (in degrees)
- `angle_y` (float, get) — Rotation angle around the Y axis (in degrees)
- `angle_z` (float, get) — Rotation angle around the Z axis (in degrees)

