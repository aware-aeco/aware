---
name: allplan-utils-rotationutil
description: This skill encodes the allplan 2024.0 surface of the Utils.RotationUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: RotationUtil.
---

# Utils.RotationUtil

Auto-generated from vendor docs for allplan 2024.0. 1 types in this namespace.

## RotationUtil (class)

Implementation of the rotation utility

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/Utils/RotationUtil/RotationUtil/)

### Constructors
- `RotationUtil(angle_x, angle_y, angle_z)` — Set the model angles

### Methods
#### `__repr__()`

create a string from the object data

**Remarks:** create a string from the object data

**Returns:** `object` — string with the object data

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/RotationUtil/RotationUtil/#Utils.RotationUtil.RotationUtil.__repr__)

#### `get_rotation_matrix(axis_point=Point3D())`

get the rotation matrix

**Remarks:** get the rotation matrix

**Parameters:**
- `axis_point` (Point3D) — axis point. Defaults to AllplanGeo.Point3D().

**Returns:** `Matrix3D` — rotation axis

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/RotationUtil/RotationUtil/#Utils.RotationUtil.RotationUtil.get_rotation_matrix)

### Properties
- `angle_x` (float, get) — Get the x angle in degree
- `angle_y` (float, get) — Get the y angle in degree
- `angle_z` (float, get) — Get the y angle in degree

