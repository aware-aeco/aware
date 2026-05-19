---
name: grasshopper-grasshopper-kernel-types-transforms
description: This skill encodes the grasshopper 7.0 surface of the Grasshopper.Kernel.Types.Transforms namespace — 8 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Generic, Identity, Orientation, Projection, Rotation, Scale, Translation, ITransform.
---

# Grasshopper.Kernel.Types.Transforms

Auto-generated from vendor docs for grasshopper 7.0. 8 types in this namespace.

## Generic (class)

Implements a generic matrix transformation.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_Transforms_Generic.htm)

### Constructors
- `public Generic()` — Initializes a new instance of the Generic class
- `public Generic(Transform transform)` — Initializes a new instance of the Generic class

### Methods
#### `public ITransform Duplicate()`



**Returns:** `ITransform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Generic_Duplicate.htm)

#### `public bool Read(GH_IReader reader)`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Generic_Read.htm)

#### `public ITransform Reverse()`



**Returns:** `ITransform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Generic_Reverse.htm)

#### `public Transform ToMatrix()`



**Returns:** `Transform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Generic_ToMatrix.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Generic_ToString.htm)

#### `public bool Write(GH_IWriter writer)`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Generic_Write.htm)

### Properties
- `Name` (String, get) — 
- `Transform` (Transform, get/set) — 

## ITransform (interface)

Basic interface for a single Grasshopper transformation.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_Transforms_ITransform.htm)

### Methods
#### `ITransform Duplicate()`



**Returns:** `ITransform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_ITransform_Duplicate.htm)

#### `bool Read(GH_IReader reader)`

This method is called whenever the instance is required to deserialize itself.

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader) — Reader object to deserialize from.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_GH_ISerializable_Read.htm)

#### `ITransform Reverse()`



**Returns:** `ITransform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_ITransform_Reverse.htm)

#### `Transform ToMatrix()`



**Returns:** `Transform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_ITransform_ToMatrix.htm)

#### `string ToString()`



**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_ITransform_ToString.htm)

#### `bool Write(GH_IWriter writer)`

This method is called whenever the instance is required to serialize itself.

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter) — Writer object to serialize with.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_GH_IO_GH_ISerializable_Write.htm)

### Properties
- `Name` (String, get) — 

## Identity (class)

Implements a null-transform.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_Transforms_Identity.htm)

### Constructors
- `public Identity()` — Initializes a new instance of the Identity class

### Methods
#### `public ITransform Duplicate()`



**Returns:** `ITransform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Identity_Duplicate.htm)

#### `public bool Read(GH_IReader reader)`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Identity_Read.htm)

#### `public ITransform Reverse()`



**Returns:** `ITransform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Identity_Reverse.htm)

#### `public Transform ToMatrix()`



**Returns:** `Transform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Identity_ToMatrix.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Identity_ToString.htm)

#### `public bool Write(GH_IWriter writer)`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Identity_Write.htm)

### Properties
- `Name` (String, get) — 

## Orientation (class)

Implements an orientation (ChangeBasis) transformation.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_Transforms_Orientation.htm)

### Constructors
- `public Orientation()` — Initializes a new instance of the Orientation class
- `public Orientation(Plane source, Plane target)` — Initializes a new instance of the Orientation class

### Methods
#### `public ITransform Duplicate()`



**Returns:** `ITransform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Orientation_Duplicate.htm)

#### `public bool Read(GH_IReader reader)`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Orientation_Read.htm)

#### `public ITransform Reverse()`



**Returns:** `ITransform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Orientation_Reverse.htm)

#### `public Transform ToMatrix()`



**Returns:** `Transform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Orientation_ToMatrix.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Orientation_ToString.htm)

#### `public bool Write(GH_IWriter writer)`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Orientation_Write.htm)

### Properties
- `Name` (String, get) — 
- `SourceFrame` (Plane, get/set) — 
- `TargetFrame` (Plane, get/set) — 

## Projection (class)

Implements a projection transformation.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_Transforms_Projection.htm)

### Constructors
- `public Projection()` — Initializes a new instance of the Projection class
- `public Projection(Plane plane)` — Initializes a new instance of the Projection class

### Methods
#### `public ITransform Duplicate()`



**Returns:** `ITransform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Projection_Duplicate.htm)

#### `public bool Read(GH_IReader reader)`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Projection_Read.htm)

#### `public ITransform Reverse()`



**Returns:** `ITransform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Projection_Reverse.htm)

#### `public Transform ToMatrix()`



**Returns:** `Transform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Projection_ToMatrix.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Projection_ToString.htm)

#### `public bool Write(GH_IWriter writer)`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Projection_Write.htm)

### Properties
- `Name` (String, get) — 
- `ProjectionPlane` (Plane, get/set) — 

## Rotation (class)

Implements a rotation transformation.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_Transforms_Rotation.htm)

### Constructors
- `public Rotation()` — Initializes a new instance of the Rotation class
- `public Rotation(Point3d center, Vector3d dir0, Vector3d dir1)` — Initializes a new instance of the Rotation class
- `public Rotation(Point3d center, Vector3d axis, double angle)` — Initializes a new instance of the Rotation class

### Methods
#### `public ITransform Duplicate()`



**Returns:** `ITransform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Rotation_Duplicate.htm)

#### `public bool Read(GH_IReader reader)`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Rotation_Read.htm)

#### `public ITransform Reverse()`



**Returns:** `ITransform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Rotation_Reverse.htm)

#### `public Transform ToMatrix()`



**Returns:** `Transform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Rotation_ToMatrix.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Rotation_ToString.htm)

#### `public bool Write(GH_IWriter writer)`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Rotation_Write.htm)

### Properties
- `Name` (String, get) — 
- `RotationAngle` (Double, get/set) — 
- `RotationAxis` (Vector3d, get/set) — 
- `RotationCenter` (Point3d, get/set) — 

## Scale (class)

Implements a scaling transformation.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_Transforms_Scale.htm)

### Constructors
- `public Scale()` — Initializes a new instance of the Scale class
- `public Scale(Plane frame, double scale)` — Initializes a new instance of the Scale class
- `public Scale(Point3d point, double scale)` — Initializes a new instance of the Scale class
- `public Scale(Plane frame, double xscale, double yscale, double zscale)` — Initializes a new instance of the Scale class

### Methods
#### `public ITransform Duplicate()`



**Returns:** `ITransform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Scale_Duplicate.htm)

#### `public bool Read(GH_IReader reader)`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Scale_Read.htm)

#### `public ITransform Reverse()`



**Returns:** `ITransform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Scale_Reverse.htm)

#### `public Transform ToMatrix()`



**Returns:** `Transform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Scale_ToMatrix.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Scale_ToString.htm)

#### `public bool Write(GH_IWriter writer)`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Scale_Write.htm)

### Properties
- `Name` (String, get) — 
- `ScaleXFactor` (Double, get/set) — 
- `ScaleYFactor` (Double, get/set) — 
- `ScaleZFactor` (Double, get/set) — 
- `ScalingFrame` (Plane, get/set) — 

## Translation (class)

Implements a translation (Move) transformation.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Types_Transforms_Translation.htm)

### Constructors
- `public Translation()` — Initializes a new instance of the Translation class
- `public Translation(Vector3d translation)` — Initializes a new instance of the Translation class

### Methods
#### `public ITransform Duplicate()`



**Returns:** `ITransform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Translation_Duplicate.htm)

#### `public bool Read(GH_IReader reader)`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Translation_Read.htm)

#### `public ITransform Reverse()`



**Returns:** `ITransform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Translation_Reverse.htm)

#### `public Transform ToMatrix()`



**Returns:** `Transform` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Translation_ToMatrix.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Translation_ToString.htm)

#### `public bool Write(GH_IWriter writer)`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Types_Transforms_Translation_Write.htm)

### Properties
- `Name` (String, get) — 
- `TranslationVector` (Vector3d, get/set) — 

