---
name: rhino-rhino-geometry-morphs
description: This skill encodes the rhino 8.0 surface of the Rhino.Geometry.Morphs namespace — 8 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BendSpaceMorph, FlowSpaceMorph, MaelstromSpaceMorph, SplopSpaceMorph, SporphSpaceMorph, StretchSpaceMorph, TaperSpaceMorph, TwistSpaceMorph.
---

# Rhino.Geometry.Morphs

Auto-generated from vendor docs for rhino 8.0. 8 types in this namespace.

## BendSpaceMorph (class)

Deforms objects by bending along a spine arc.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Morphs_BendSpaceMorph.htm)

### Constructors
- `public BendSpaceMorph(Point3d start, Point3d end, Point3d point, bool straight, bool symmetric)` — Constructs a bend space morph.
- `public BendSpaceMorph(Point3d start, Point3d end, Point3d point, double angle, bool straight, bool symmetric)` — Constructs a bend space morph.

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_BendSpaceMorph_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_BendSpaceMorph_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_BendSpaceMorph_Finalize.htm)

#### `public bool Morph(GeometryBase geometry)`

Apply the space morph to geometry.

**Parameters:**
- `geometry` (Rhino.Geometry.GeometryBase) — Geometry to morph.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_SpaceMorph_Morph.htm)

#### `public bool Morph(ref Plane plane)`

Apply the space morph to a plane.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — Plane to morph.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_SpaceMorph_Morph_1.htm)

#### `public override Point3d MorphPoint(Point3d point)`

Morphs an Euclidean point.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — A point that will be morphed by this object.

**Returns:** `Point3d` — Resulting morphed point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_BendSpaceMorph_MorphPoint.htm)

### Properties
- `IsValid` (Boolean, get) — Returns true if the space morph definition is valid, false otherwise.
- `PreserveStructure` (Boolean, get/set) — true if the morph should be done in a way that preserves the structure of the geometry. In particular, for NURBS objects, true means that only the control points are moved. The PreserveStructure value does not affect the way meshes and points are morphed. The default is false.
- `QuickPreview` (Boolean, get/set) — true if the morph should be done as quickly as possible because the result is being used for some type of dynamic preview. If QuickPreview is true, the tolerance may be ignored. The QuickPreview value does not affect the way meshes and points are morphed. The default is false.
- `Tolerance` (Double, get/set) — The desired accuracy of the morph. This value is primarily used for deforming surfaces and breps. The default is 0.0 and any value <= 0.0 is ignored by morphing functions. The Tolerance value does not affect the way meshes and points are morphed.

## FlowSpaceMorph (class)

Re-aligns objects from a base curve to a target curve.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Morphs_FlowSpaceMorph.htm)

### Constructors
- `public FlowSpaceMorph(Curve curve0, Curve curve1, bool preventStretching)` — Constructs a flow space morph.
- `public FlowSpaceMorph(Curve curve0, Curve curve1, bool reverseCurve0, bool reverseCurve1, bool preventStretching)` — Constructs a flow space morph.

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_FlowSpaceMorph_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_FlowSpaceMorph_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_FlowSpaceMorph_Finalize.htm)

#### `public bool Morph(GeometryBase geometry)`

Apply the space morph to geometry.

**Parameters:**
- `geometry` (Rhino.Geometry.GeometryBase) — Geometry to morph.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_SpaceMorph_Morph.htm)

#### `public bool Morph(ref Plane plane)`

Apply the space morph to a plane.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — Plane to morph.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_SpaceMorph_Morph_1.htm)

#### `public override Point3d MorphPoint(Point3d point)`

Morphs an Euclidean point.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — A point that will be morphed by this object.

**Returns:** `Point3d` — Resulting morphed point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_FlowSpaceMorph_MorphPoint.htm)

### Properties
- `IsValid` (Boolean, get) — Returns true if the space morph definition is valid, false otherwise.
- `PreserveStructure` (Boolean, get/set) — true if the morph should be done in a way that preserves the structure of the geometry. In particular, for NURBS objects, true means that only the control points are moved. The PreserveStructure value does not affect the way meshes and points are morphed. The default is false.
- `QuickPreview` (Boolean, get/set) — true if the morph should be done as quickly as possible because the result is being used for some type of dynamic preview. If QuickPreview is true, the tolerance may be ignored. The QuickPreview value does not affect the way meshes and points are morphed. The default is false.
- `Tolerance` (Double, get/set) — The desired accuracy of the morph. This value is primarily used for deforming surfaces and breps. The default is 0.0 and any value <= 0.0 is ignored by morphing functions. The Tolerance value does not affect the way meshes and points are morphed.

## MaelstromSpaceMorph (class)

Deforms objects in a spiral as if they were caught in a whirlpool.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Morphs_MaelstromSpaceMorph.htm)

### Constructors
- `public MaelstromSpaceMorph(Plane plane, double radius0, double radius1, double angle)` — Constructs a maelstrom space morph.

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_MaelstromSpaceMorph_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_MaelstromSpaceMorph_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_MaelstromSpaceMorph_Finalize.htm)

#### `public bool Morph(GeometryBase geometry)`

Apply the space morph to geometry.

**Parameters:**
- `geometry` (Rhino.Geometry.GeometryBase) — Geometry to morph.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_SpaceMorph_Morph.htm)

#### `public bool Morph(ref Plane plane)`

Apply the space morph to a plane.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — Plane to morph.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_SpaceMorph_Morph_1.htm)

#### `public override Point3d MorphPoint(Point3d point)`

Morphs an Euclidean point.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — A point that will be morphed by this object.

**Returns:** `Point3d` — Resulting morphed point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_MaelstromSpaceMorph_MorphPoint.htm)

### Properties
- `IsValid` (Boolean, get) — Returns true if the space morph definition is valid, false otherwise.
- `PreserveStructure` (Boolean, get/set) — true if the morph should be done in a way that preserves the structure of the geometry. In particular, for NURBS objects, true means that only the control points are moved. The PreserveStructure value does not affect the way meshes and points are morphed. The default is false.
- `QuickPreview` (Boolean, get/set) — true if the morph should be done as quickly as possible because the result is being used for some type of dynamic preview. If QuickPreview is true, the tolerance may be ignored. The QuickPreview value does not affect the way meshes and points are morphed. The default is false.
- `Tolerance` (Double, get/set) — The desired accuracy of the morph. This value is primarily used for deforming surfaces and breps. The default is 0.0 and any value <= 0.0 is ignored by morphing functions. The Tolerance value does not affect the way meshes and points are morphed.

## SplopSpaceMorph (class)

Rotates, scales, and wraps objects on a surface.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Morphs_SplopSpaceMorph.htm)

### Constructors
- `public SplopSpaceMorph(Plane plane, Surface surface, Point2d surfaceParam)` — Constructs a flow space morph.
- `public SplopSpaceMorph(Plane plane, Surface surface, Point2d surfaceParam, double scale)` — Constructs a flow space morph.
- `public SplopSpaceMorph(Plane plane, Surface surface, Point2d surfaceParam, double scale, double angle)` — Constructs a flow space morph.

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_SplopSpaceMorph_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_SplopSpaceMorph_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_SplopSpaceMorph_Finalize.htm)

#### `public bool Morph(GeometryBase geometry)`

Apply the space morph to geometry.

**Parameters:**
- `geometry` (Rhino.Geometry.GeometryBase) — Geometry to morph.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_SpaceMorph_Morph.htm)

#### `public bool Morph(ref Plane plane)`

Apply the space morph to a plane.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — Plane to morph.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_SpaceMorph_Morph_1.htm)

#### `public override Point3d MorphPoint(Point3d point)`

Morphs an Euclidean point.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — A point that will be morphed by this object.

**Returns:** `Point3d` — Resulting morphed point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_SplopSpaceMorph_MorphPoint.htm)

### Properties
- `IsValid` (Boolean, get) — Returns true if the space morph definition is valid, false otherwise.
- `PreserveStructure` (Boolean, get/set) — true if the morph should be done in a way that preserves the structure of the geometry. In particular, for NURBS objects, true means that only the control points are moved. The PreserveStructure value does not affect the way meshes and points are morphed. The default is false.
- `QuickPreview` (Boolean, get/set) — true if the morph should be done as quickly as possible because the result is being used for some type of dynamic preview. If QuickPreview is true, the tolerance may be ignored. The QuickPreview value does not affect the way meshes and points are morphed. The default is false.
- `Tolerance` (Double, get/set) — The desired accuracy of the morph. This value is primarily used for deforming surfaces and breps. The default is 0.0 and any value <= 0.0 is ignored by morphing functions. The Tolerance value does not affect the way meshes and points are morphed.

## SporphSpaceMorph (class)

Deforms an object from a source surface to a target surface.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Morphs_SporphSpaceMorph.htm)

### Constructors
- `public SporphSpaceMorph(Surface surface0, Surface surface1)` — Constructs a Sporph space morph.
- `public SporphSpaceMorph(Surface surface0, Surface surface1, Point2d surface0Param, Point2d surface1Param)` — Constructs a Sporph space morph.

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_SporphSpaceMorph_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_SporphSpaceMorph_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_SporphSpaceMorph_Finalize.htm)

#### `public bool Morph(GeometryBase geometry)`

Apply the space morph to geometry.

**Parameters:**
- `geometry` (Rhino.Geometry.GeometryBase) — Geometry to morph.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_SpaceMorph_Morph.htm)

#### `public bool Morph(ref Plane plane)`

Apply the space morph to a plane.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — Plane to morph.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_SpaceMorph_Morph_1.htm)

#### `public override Point3d MorphPoint(Point3d point)`

Morphs an Euclidean point.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — A point that will be morphed by this object.

**Returns:** `Point3d` — Resulting morphed point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_SporphSpaceMorph_MorphPoint.htm)

### Properties
- `ConstrainNormal` (Vector3d, get/set) — Specifies how the normal direction of the base surface is mapped onto the target surface. To use the target surface normal, set to Unset.
- `IsValid` (Boolean, get) — Returns true if the space morph definition is valid, false otherwise.
- `PreserveStructure` (Boolean, get/set) — true if the morph should be done in a way that preserves the structure of the geometry. In particular, for NURBS objects, true means that only the control points are moved. The PreserveStructure value does not affect the way meshes and points are morphed. The default is false.
- `QuickPreview` (Boolean, get/set) — true if the morph should be done as quickly as possible because the result is being used for some type of dynamic preview. If QuickPreview is true, the tolerance may be ignored. The QuickPreview value does not affect the way meshes and points are morphed. The default is false.
- `Tolerance` (Double, get/set) — The desired accuracy of the morph. This value is primarily used for deforming surfaces and breps. The default is 0.0 and any value <= 0.0 is ignored by morphing functions. The Tolerance value does not affect the way meshes and points are morphed.

## StretchSpaceMorph (class)

Deforms objects toward or away from a specified axis.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Morphs_StretchSpaceMorph.htm)

### Constructors
- `public StretchSpaceMorph(Point3d start, Point3d end, Point3d point)` — Constructs a stretch space morph.
- `public StretchSpaceMorph(Point3d start, Point3d end, double length)` — Constructs a stretch space morph.

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_StretchSpaceMorph_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_StretchSpaceMorph_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_StretchSpaceMorph_Finalize.htm)

#### `public bool Morph(GeometryBase geometry)`

Apply the space morph to geometry.

**Parameters:**
- `geometry` (Rhino.Geometry.GeometryBase) — Geometry to morph.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_SpaceMorph_Morph.htm)

#### `public bool Morph(ref Plane plane)`

Apply the space morph to a plane.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — Plane to morph.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_SpaceMorph_Morph_1.htm)

#### `public override Point3d MorphPoint(Point3d point)`

Morphs an Euclidean point.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — A point that will be morphed by this object.

**Returns:** `Point3d` — Resulting morphed point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_StretchSpaceMorph_MorphPoint.htm)

### Properties
- `IsValid` (Boolean, get) — Returns true if the space morph definition is valid, false otherwise.
- `PreserveStructure` (Boolean, get/set) — true if the morph should be done in a way that preserves the structure of the geometry. In particular, for NURBS objects, true means that only the control points are moved. The PreserveStructure value does not affect the way meshes and points are morphed. The default is false.
- `QuickPreview` (Boolean, get/set) — true if the morph should be done as quickly as possible because the result is being used for some type of dynamic preview. If QuickPreview is true, the tolerance may be ignored. The QuickPreview value does not affect the way meshes and points are morphed. The default is false.
- `Tolerance` (Double, get/set) — The desired accuracy of the morph. This value is primarily used for deforming surfaces and breps. The default is 0.0 and any value <= 0.0 is ignored by morphing functions. The Tolerance value does not affect the way meshes and points are morphed.

## TaperSpaceMorph (class)

Deforms objects toward or away from a specified axis.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Morphs_TaperSpaceMorph.htm)

### Constructors
- `public TaperSpaceMorph(Point3d start, Point3d end, double startRadius, double endRadius, bool bFlat, bool infiniteTaper)` — Constructs a taper space morph.

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_TaperSpaceMorph_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_TaperSpaceMorph_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_TaperSpaceMorph_Finalize.htm)

#### `public bool Morph(GeometryBase geometry)`

Apply the space morph to geometry.

**Parameters:**
- `geometry` (Rhino.Geometry.GeometryBase) — Geometry to morph.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_SpaceMorph_Morph.htm)

#### `public bool Morph(ref Plane plane)`

Apply the space morph to a plane.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — Plane to morph.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_SpaceMorph_Morph_1.htm)

#### `public override Point3d MorphPoint(Point3d point)`

Morphs an Euclidean point.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — A point that will be morphed by this object.

**Returns:** `Point3d` — Resulting morphed point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_TaperSpaceMorph_MorphPoint.htm)

### Properties
- `IsValid` (Boolean, get) — Returns true if the space morph definition is valid, false otherwise.
- `PreserveStructure` (Boolean, get/set) — true if the morph should be done in a way that preserves the structure of the geometry. In particular, for NURBS objects, true means that only the control points are moved. The PreserveStructure value does not affect the way meshes and points are morphed. The default is false.
- `QuickPreview` (Boolean, get/set) — true if the morph should be done as quickly as possible because the result is being used for some type of dynamic preview. If QuickPreview is true, the tolerance may be ignored. The QuickPreview value does not affect the way meshes and points are morphed. The default is false.
- `Tolerance` (Double, get/set) — The desired accuracy of the morph. This value is primarily used for deforming surfaces and breps. The default is 0.0 and any value <= 0.0 is ignored by morphing functions. The Tolerance value does not affect the way meshes and points are morphed.

## TwistSpaceMorph (class)

Deforms objects by rotating them around an axis.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Morphs_TwistSpaceMorph.htm)

### Constructors
- `public TwistSpaceMorph()` — Constructs a twist space morph.

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_TwistSpaceMorph_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_TwistSpaceMorph_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_TwistSpaceMorph_Finalize.htm)

#### `public bool Morph(GeometryBase geometry)`

Apply the space morph to geometry.

**Parameters:**
- `geometry` (Rhino.Geometry.GeometryBase) — Geometry to morph.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_SpaceMorph_Morph.htm)

#### `public bool Morph(ref Plane plane)`

Apply the space morph to a plane.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — Plane to morph.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_SpaceMorph_Morph_1.htm)

#### `public override Point3d MorphPoint(Point3d point)`

Morphs an Euclidean point. This method is abstract.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — A point that will be morphed by this function.

**Returns:** `Point3d` — Resulting morphed point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Morphs_TwistSpaceMorph_MorphPoint.htm)

### Properties
- `InfiniteTwist` (Boolean, get/set) — If true, the deformation is constant throughout the object, even if the axis is shorter than the object. If false, the deformation takes place only the length of the axis.
- `PreserveStructure` (Boolean, get/set) — true if the morph should be done in a way that preserves the structure of the geometry. In particular, for NURBS objects, true means that only the control points are moved. The PreserveStructure value does not affect the way meshes and points are morphed. The default is false.
- `QuickPreview` (Boolean, get/set) — true if the morph should be done as quickly as possible because the result is being used for some type of dynamic preview. If QuickPreview is true, the tolerance may be ignored. The QuickPreview value does not affect the way meshes and points are morphed. The default is false.
- `Tolerance` (Double, get/set) — The desired accuracy of the morph. This value is primarily used for deforming surfaces and breps. The default is 0.0 and any value <= 0.0 is ignored by morphing functions. The Tolerance value does not affect the way meshes and points are morphed.
- `TwistAngleRadians` (Double, get/set) — Twist angle in radians.
- `TwistAxis` (Line, get/set) — Axis to rotate about.

