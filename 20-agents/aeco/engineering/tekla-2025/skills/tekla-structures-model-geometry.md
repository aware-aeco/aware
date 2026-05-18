---
name: tekla-tekla-structures-model-geometry
description: This skill encodes the tekla 2025.0 surface of the Tekla.Structures.Model.Geometry namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Rotation3D.
---

# Tekla.Structures.Model.Geometry

Auto-generated from vendor docs for tekla 2025.0. 1 types in this namespace.

## Rotation3D (class)

Represents 3d rotation of object.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/18fb56dd-64f5-6410-d628-9b71ef5585aa)

### Constructors
- `public Rotation3D()` — Initializes a new instance of the Rotation3D class.
- `public Rotation3D(Vector axisX, Vector axisY)` — Initializes a new instance of the Rotation3D class.

### Methods
#### `public bool Equals(Rotation3D other, double tolerance)`

Compares with another rotation.

**Parameters:**
- `other` (Tekla.Structures.Model.Geometry.Rotation3D) — The other rotation.
- `tolerance` (System.Double) — Comparison tolerance.

**Returns:** `Boolean` — True if found equal, false otherwise.

[Docs](https://developer.tekla.com/topic/en/18/43/979fb96b-2c44-9fc5-e000-56947aa3efa9)

#### `public static Rotation3D FromZRotation(Angle angle)`

Creates Rotation3D which represents rotation around the Z axis.

**Parameters:**
- `angle` (Tekla.Structures.Datatype.Angle) — The rotation andle.

**Returns:** `Rotation3D` — Rotation3D instance whcih represents the Z rotation.

[Docs](https://developer.tekla.com/topic/en/18/43/f5b0d717-aa22-02fa-cefa-41307adec2f5)

### Properties
- `AxisX` (Vector, get) — Gets the X direction.
- `AxisY` (Vector, get) — Gets the Y direction.
- `AxisZ` (Vector, get) — Gets the Z direction.

