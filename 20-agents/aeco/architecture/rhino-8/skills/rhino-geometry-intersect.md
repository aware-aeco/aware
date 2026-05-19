---
name: rhino-rhino-geometry-intersect
description: This skill encodes the rhino 8.0 surface of the Rhino.Geometry.Intersect namespace — 15 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: CurveIntersections, Intersection, IntersectionEvent, MeshIntersectionCache, MeshClash, RayShootEvent, MeshInterference, CircleCircleIntersection, and 7 more types.
---

# Rhino.Geometry.Intersect

Auto-generated from vendor docs for rhino 8.0. 15 types in this namespace.

## ArcArcIntersection (enum)

Represents all possible cases of a Arc|Arc intersection event.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Intersect_ArcArcIntersection.htm)

### Values
- `None` = `0` — Arcs do not intersect.
- `Single` = `1` — Arcs touch at a one point.
- `Multiple` = `2` — Arcs intersect at two points.
- `Overlap` = `3` — Arcs are cocircular and overlap.

## CircleCircleIntersection (enum)

Represents all possible cases of a Circle|Circle intersection event.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Intersect_CircleCircleIntersection.htm)

### Values
- `None` = `0` — Circles do not intersect.
- `Single` = `1` — Circles touch at a one point.
- `Multiple` = `2` — Circles intersect at two points.
- `Overlap` = `3` — Circles are identical.

## CurveIntersections (class)

Maintains an ordered list of Curve Intersection results.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Intersect_CurveIntersections.htm)

### Methods
#### `public void CopyTo(IntersectionEvent[] array, int arrayIndex)`

Copies all intersection results into another array, departing at an index in the target array.

**Parameters:**
- `array` (Rhino.Geometry.Intersect.IntersectionEvent[]) — The target array. This value cannot be null.
- `arrayIndex` (System.Int32) — Zero-based index in which to start the copy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_CurveIntersections_CopyTo.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_CurveIntersections_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_CurveIntersections_Dispose_1.htm)

#### `protected override void Finalize()`

Destructor.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_CurveIntersections_Finalize.htm)

#### `public IEnumerator<IntersectionEvent> GetEnumerator()`

Returns an enumerator that is capable of yielding all IntersectionEvents in the collection.

**Returns:** `IEnumerator<IntersectionEvent>` — The constructed enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_CurveIntersections_GetEnumerator.htm)

### Properties
- `Count` (Int32, get) — Gets the number of recorded intersection events.
- `Item` (IntersectionEvent, get) — Gets the intersection event data at the given index.

## Intersection (class)

Provides static methods for the computation of intersections, projections, sections and similar.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Intersect_Intersection.htm)

### Methods
#### `public static ArcArcIntersection ArcArc(Arc arcA, Arc arcB, out Point3d intersectionPoint1, out Point3d intersectionPoint2)`

Intersects two arcs using exact calculations.

**Parameters:**
- `arcA` (Rhino.Geometry.Arc) — First arc to intersect.
- `arcB` (Rhino.Geometry.Arc) — Second arc to intersect.
- `intersectionPoint1` (Rhino.Geometry.Point3d) — First intersection point.
- `intersectionPoint2` (Rhino.Geometry.Point3d) — Second intersection point.

**Returns:** `ArcArcIntersection` — The intersection type.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_ArcArc.htm)

#### `public static bool BrepBrep(Brep brepA, Brep brepB, double tolerance, bool joinCurves, out Curve[] intersectionCurves, out Point3d[] intersectionPoints)`

Intersects two Breps.

**Parameters:**
- `brepA` (Rhino.Geometry.Brep) — First Brep for intersection.
- `brepB` (Rhino.Geometry.Brep) — Second Brep for intersection.
- `tolerance` (System.Double) — Intersection tolerance.
- `joinCurves` (System.Boolean) — If true, join the resulting curves where possible.
- `intersectionCurves` (Rhino.Geometry.Curve[]) — The intersection curves will be returned here.
- `intersectionPoints` (Rhino.Geometry.Point3d[]) — The intersection points will be returned here.

**Returns:** `Boolean` — true if the operation was success, false otherwise. Check the output arrays to determine how the input intersects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_BrepBrep_1.htm)

#### `public static bool BrepBrep(Brep brepA, Brep brepB, double tolerance, out Curve[] intersectionCurves, out Point3d[] intersectionPoints)`

Intersects two Breps.

**Parameters:**
- `brepA` (Rhino.Geometry.Brep) — First Brep for intersection.
- `brepB` (Rhino.Geometry.Brep) — Second Brep for intersection.
- `tolerance` (System.Double) — Intersection tolerance.
- `intersectionCurves` (Rhino.Geometry.Curve[]) — The intersection curves will be returned here.
- `intersectionPoints` (Rhino.Geometry.Point3d[]) — The intersection points will be returned here.

**Returns:** `Boolean` — true if the operation was success, false otherwise. Check the output arrays to determine how the input intersects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_BrepBrep.htm)

#### `public static bool BrepPlane(Brep brep, Plane plane, double tolerance, bool joinCurves, out Curve[] intersectionCurves, out Point3d[] intersectionPoints)`

Intersects a Brep with an (infinite) plane.

**Parameters:**
- `brep` (Rhino.Geometry.Brep) — Brep to intersect.
- `plane` (Rhino.Geometry.Plane) — Plane to intersect with.
- `tolerance` (System.Double) — Tolerance to use for intersections.
- `joinCurves` (System.Boolean) — If true, join the resulting curves where possible.
- `intersectionCurves` (Rhino.Geometry.Curve[]) — The intersection curves will be returned here.
- `intersectionPoints` (Rhino.Geometry.Point3d[]) — The intersection points will be returned here.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_BrepPlane_1.htm)

#### `public static bool BrepPlane(Brep brep, Plane plane, double tolerance, out Curve[] intersectionCurves, out Point3d[] intersectionPoints)`

Intersects a Brep with an (infinite) plane.

**Parameters:**
- `brep` (Rhino.Geometry.Brep) — Brep to intersect.
- `plane` (Rhino.Geometry.Plane) — Plane to intersect with.
- `tolerance` (System.Double) — Tolerance to use for intersections.
- `intersectionCurves` (Rhino.Geometry.Curve[]) — The intersection curves will be returned here.
- `intersectionPoints` (Rhino.Geometry.Point3d[]) — The intersection points will be returned here.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_BrepPlane.htm)

#### `public static bool BrepSurface(Brep brep, Surface surface, double tolerance, bool joinCurves, out Curve[] intersectionCurves, out Point3d[] intersectionPoints)`

Intersects a Brep and a Surface.

**Parameters:**
- `brep` (Rhino.Geometry.Brep) — A brep to be intersected.
- `surface` (Rhino.Geometry.Surface) — A surface to be intersected.
- `tolerance` (System.Double) — A tolerance value.
- `joinCurves` (System.Boolean) — If true, join the resulting curves where possible.
- `intersectionCurves` (Rhino.Geometry.Curve[]) — The intersection curves array argument. This out reference is assigned during the call.
- `intersectionPoints` (Rhino.Geometry.Point3d[]) — The intersection points array argument. This out reference is assigned during the call.

**Returns:** `Boolean` — true if the operation was success, false otherwise. Check the output arrays to determine how the input intersects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_BrepSurface_1.htm)

#### `public static bool BrepSurface(Brep brep, Surface surface, double tolerance, out Curve[] intersectionCurves, out Point3d[] intersectionPoints)`

Intersects a Brep and a Surface.

**Parameters:**
- `brep` (Rhino.Geometry.Brep) — A brep to be intersected.
- `surface` (Rhino.Geometry.Surface) — A surface to be intersected.
- `tolerance` (System.Double) — A tolerance value.
- `intersectionCurves` (Rhino.Geometry.Curve[]) — The intersection curves array argument. This out reference is assigned during the call.
- `intersectionPoints` (Rhino.Geometry.Point3d[]) — The intersection points array argument. This out reference is assigned during the call.

**Returns:** `Boolean` — true if the operation was success, false otherwise. Check the output arrays to determine how the input intersects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_BrepSurface.htm)

#### `public static CircleCircleIntersection CircleCircle(Circle circleA, Circle circleB, out Point3d intersectionPoint1, out Point3d intersectionPoint2)`

Intersects two circles using exact calculations.

**Parameters:**
- `circleA` (Rhino.Geometry.Circle) — First circle to intersect.
- `circleB` (Rhino.Geometry.Circle) — Second circle to intersect.
- `intersectionPoint1` (Rhino.Geometry.Point3d) — First intersection point.
- `intersectionPoint2` (Rhino.Geometry.Point3d) — Second intersection point.

**Returns:** `CircleCircleIntersection` — The intersection type.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_CircleCircle.htm)

#### `public static bool CurveBrep(Curve curve, Brep brep, double tolerance, out Curve[] overlapCurves, out Point3d[] intersectionPoints)`

Intersects a curve with a Brep. This function returns the 3D points of intersection and 3D overlap curves. If an error occurs while processing overlap curves, this function will return false, but it will still provide partial results.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — Curve for intersection.
- `brep` (Rhino.Geometry.Brep) — Brep for intersection.
- `tolerance` (System.Double) — Fitting and near miss tolerance.
- `overlapCurves` (Rhino.Geometry.Curve[]) — The overlap curves will be returned here.
- `intersectionPoints` (Rhino.Geometry.Point3d[]) — The intersection points will be returned here.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_CurveBrep.htm)

#### `public static bool CurveBrep(Curve curve, Brep brep, double tolerance, out Curve[] overlapCurves, out Point3d[] intersectionPoints, out double[] curveParameters)`

Intersects a curve with a Brep. This function returns the 3D points of intersection, curve parameters at the intersection locations, and 3D overlap curves. If an error occurs while processing overlap curves, this function will return false, but it will still provide partial results.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — Curve for intersection.
- `brep` (Rhino.Geometry.Brep) — Brep for intersection.
- `tolerance` (System.Double) — Fitting and near miss tolerance.
- `overlapCurves` (Rhino.Geometry.Curve[]) — The overlap curves will be returned here.
- `intersectionPoints` (Rhino.Geometry.Point3d[]) — The intersection points will be returned here.
- `curveParameters` (System.Double[]) — The intersection curve parameters will be returned here.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_CurveBrep_1.htm)

#### `public static bool CurveBrep(Curve curve, Brep brep, double tolerance, double angleTolerance, out double[] t)`

Intersect a curve with a Brep. This function returns the intersection parameters on the curve.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — Curve.
- `brep` (Rhino.Geometry.Brep) — Brep.
- `tolerance` (System.Double) — Absolute tolerance for intersections.
- `angleTolerance` (System.Double) — Angle tolerance in radians.
- `t` (System.Double[]) — Curve parameters at intersections.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_CurveBrep_2.htm)

#### `public static bool CurveBrepFace(Curve curve, BrepFace face, double tolerance, out Curve[] overlapCurves, out Point3d[] intersectionPoints)`

Intersects a curve with a Brep face.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — A curve.
- `face` (Rhino.Geometry.BrepFace) — A brep face.
- `tolerance` (System.Double) — Fitting and near miss tolerance.
- `overlapCurves` (Rhino.Geometry.Curve[]) — A overlap curves array argument. This out reference is assigned during the call.
- `intersectionPoints` (Rhino.Geometry.Point3d[]) — A points array argument. This out reference is assigned during the call.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_CurveBrepFace.htm)

#### `public static CurveIntersections CurveCurve(Curve curveA, Curve curveB, double tolerance, double overlapTolerance)`

Finds the intersections between two curves.

**Parameters:**
- `curveA` (Rhino.Geometry.Curve) — First curve for intersection.
- `curveB` (Rhino.Geometry.Curve) — Second curve for intersection.
- `tolerance` (System.Double) — Intersection tolerance. If the curves approach each other to within tolerance, an intersection is assumed.
- `overlapTolerance` (System.Double) — The tolerance with which the curves are tested.

**Returns:** `CurveIntersections` — A collection of intersection events.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_CurveCurve.htm)

#### `public static CurveIntersections CurveCurveValidate(Curve curveA, Curve curveB, double tolerance, double overlapTolerance, out int[] invalidIndices, out TextLog textLog)`

Finds the intersections between two curves.

**Parameters:**
- `curveA` (Rhino.Geometry.Curve) — First curve for intersection.
- `curveB` (Rhino.Geometry.Curve) — Second curve for intersection.
- `tolerance` (System.Double) — Intersection tolerance. If the curves approach each other to within tolerance, an intersection is assumed.
- `overlapTolerance` (System.Double) — The tolerance with which the curves are tested.
- `invalidIndices` (System.Int32[]) — The indices in the resulting CurveIntersections collection that are invalid.
- `textLog` (Rhino.FileIO.TextLog) — A text log that contains tails about the invalid intersection events.

**Returns:** `CurveIntersections` — A collection of intersection events.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_CurveCurveValidate.htm)

#### `public static CurveIntersections CurveLine(Curve curve, Line line, double tolerance, double overlapTolerance)`

Intersects a curve and an infinite line.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — Curve for intersection.
- `line` (Rhino.Geometry.Line) — Infinite line to intersect.
- `tolerance` (System.Double) — Intersection tolerance. If the curves approach each other to within tolerance, an intersection is assumed.
- `overlapTolerance` (System.Double) — The tolerance with which the curves are tested.

**Returns:** `CurveIntersections` — A collection of intersection events.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_CurveLine.htm)

#### `public static CurveIntersections CurvePlane(Curve curve, Plane plane, double tolerance)`

Intersects a curve with an (infinite) plane.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — Curve to intersect.
- `plane` (Rhino.Geometry.Plane) — Plane to intersect with.
- `tolerance` (System.Double) — Tolerance to use during intersection.

**Returns:** `CurveIntersections` — A list of intersection events or null if no intersections were recorded.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_CurvePlane.htm)

#### `public static CurveIntersections CurveSelf(Curve curve, double tolerance)`

Finds the places where a curve intersects itself.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — Curve for self-intersections.
- `tolerance` (System.Double) — Intersection tolerance. If the curve approaches itself to within tolerance, an intersection is assumed.

**Returns:** `CurveIntersections` — A collection of intersection events.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_CurveSelf.htm)

#### `public static CurveIntersections CurveSurface(Curve curve, Interval curveDomain, Surface surface, double tolerance, double overlapTolerance)`

Intersects a sub-curve and a surface.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — Curve for intersection.
- `curveDomain` (Rhino.Geometry.Interval) — Domain of sub-curve to take into consideration for Intersections.
- `surface` (Rhino.Geometry.Surface) — Surface for intersection.
- `tolerance` (System.Double) — Intersection tolerance. If the curve approaches the surface to within tolerance, an intersection is assumed.
- `overlapTolerance` (System.Double) — The tolerance with which the curves are tested.

**Returns:** `CurveIntersections` — A collection of intersection events.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_CurveSurface.htm)

#### `public static CurveIntersections CurveSurface(Curve curve, Surface surface, double tolerance, double overlapTolerance)`

Intersects a curve and a surface.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — Curve for intersection.
- `surface` (Rhino.Geometry.Surface) — Surface for intersection.
- `tolerance` (System.Double) — Intersection tolerance. If the curve approaches the surface to within tolerance, an intersection is assumed.
- `overlapTolerance` (System.Double) — The tolerance with which the curves are tested.

**Returns:** `CurveIntersections` — A collection of intersection events.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_CurveSurface_1.htm)

#### `public static CurveIntersections CurveSurfaceValidate(Curve curve, Interval curveDomain, Surface surface, double tolerance, double overlapTolerance, out int[] invalidIndices, out TextLog textLog)`

Intersects a sub-curve and a surface.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — Curve for intersection.
- `curveDomain` (Rhino.Geometry.Interval) — Domain of sub-curve to take into consideration for Intersections.
- `surface` (Rhino.Geometry.Surface) — Surface for intersection.
- `tolerance` (System.Double) — Intersection tolerance. If the curve approaches the surface to within tolerance, an intersection is assumed.
- `overlapTolerance` (System.Double) — The tolerance with which the curves are tested.
- `invalidIndices` (System.Int32[]) — The indices in the resulting CurveIntersections collection that are invalid.
- `textLog` (Rhino.FileIO.TextLog) — A text log that contains tails about the invalid intersection events.

**Returns:** `CurveIntersections` — A collection of intersection events.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_CurveSurfaceValidate.htm)

#### `public static CurveIntersections CurveSurfaceValidate(Curve curve, Surface surface, double tolerance, double overlapTolerance, out int[] invalidIndices, out TextLog textLog)`

Intersects a curve and a surface.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — Curve for intersection.
- `surface` (Rhino.Geometry.Surface) — Surface for intersection.
- `tolerance` (System.Double) — Intersection tolerance. If the curve approaches the surface to within tolerance, an intersection is assumed.
- `overlapTolerance` (System.Double) — The tolerance with which the curves are tested.
- `invalidIndices` (System.Int32[]) — The indices in the resulting CurveIntersections collection that are invalid.
- `textLog` (Rhino.FileIO.TextLog) — A text log that contains tails about the invalid intersection events.

**Returns:** `CurveIntersections` — A collection of intersection events.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_CurveSurfaceValidate_1.htm)

#### `public static bool LineBox(Line line, BoundingBox box, double tolerance, out Interval lineParameters)`

Intersects an infinite line and an axis aligned bounding box.

**Parameters:**
- `line` (Rhino.Geometry.Line) — Line for intersection.
- `box` (Rhino.Geometry.BoundingBox) — BoundingBox to intersect.
- `tolerance` (System.Double) — If tolerance > 0.0, then the intersection is performed against a box that has each side moved out by tolerance.
- `lineParameters` (Rhino.Geometry.Interval) — The chord from line.PointAt(lineParameters.T0) to line.PointAt(lineParameters.T1) is the intersection.

**Returns:** `Boolean` — true if the line intersects the box, false if no intersection occurs.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_LineBox.htm)

#### `public static bool LineBox(Line line, Box box, double tolerance, out Interval lineParameters)`

Intersects an infinite line with a box volume.

**Parameters:**
- `line` (Rhino.Geometry.Line) — Line for intersection.
- `box` (Rhino.Geometry.Box) — Box to intersect.
- `tolerance` (System.Double) — If tolerance > 0.0, then the intersection is performed against a box that has each side moved out by tolerance.
- `lineParameters` (Rhino.Geometry.Interval) — The chord from line.PointAt(lineParameters.T0) to line.PointAt(lineParameters.T1) is the intersection.

**Returns:** `Boolean` — true if the line intersects the box, false if no intersection occurs.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_LineBox_1.htm)

#### `public static LineCircleIntersection LineCircle(Line line, Circle circle, out double t1, out Point3d point1, out double t2, out Point3d point2)`

Intersects a line with a circle using exact calculations.

**Parameters:**
- `line` (Rhino.Geometry.Line) — Line for intersection.
- `circle` (Rhino.Geometry.Circle) — Circle for intersection.
- `t1` (System.Double) — Parameter on line for first intersection.
- `point1` (Rhino.Geometry.Point3d) — Point on circle closest to first intersection.
- `t2` (System.Double) — Parameter on line for second intersection.
- `point2` (Rhino.Geometry.Point3d) — Point on circle closest to second intersection.

**Returns:** `LineCircleIntersection` — If Single is returned, only t1 and point1 will have valid values. If Multiple is returned, t2 and point2 will also be filled out.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_LineCircle.htm)

#### `public static LineCylinderIntersection LineCylinder(Line line, Cylinder cylinder, out Point3d intersectionPoint1, out Point3d intersectionPoint2)`

Intersects a line with a cylinder using exact calculations.

**Parameters:**
- `line` (Rhino.Geometry.Line) — Line for intersection.
- `cylinder` (Rhino.Geometry.Cylinder) — Cylinder for intersection.
- `intersectionPoint1` (Rhino.Geometry.Point3d) — First intersection point.
- `intersectionPoint2` (Rhino.Geometry.Point3d) — Second intersection point.

**Returns:** `LineCylinderIntersection` — If None is returned, the first point is the point on the line closest to the cylinder and the second point is the point on the cylinder closest to the line. If Single is returned, the first point is the point on the line and the second point is the same point on the cylinder.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_LineCylinder.htm)

#### `public static bool LineLine(Line lineA, Line lineB, out double a, out double b)`

Find the unique closest-points pair between two infinite lines, if it exists.

**Parameters:**
- `lineA` (Rhino.Geometry.Line) — First line.
- `lineB` (Rhino.Geometry.Line) — Second line.
- `a` (System.Double) — Parameter on lineA that is closest to lineB.
- `b` (System.Double) — Parameter on lineB that is closest to lineA. The shortest distance between the lines is the chord from lineA.PointAt(a) to lineB.PointAt(b)

**Returns:** `Boolean` — true if points are found and false if the lines are numerically parallel. Numerically parallel means the 2x2 matrix: +AoA -AoB-AoB +BoB is numerically singular, where A = (lineA.To - lineA.From) and B = (lineB.To-lineB.From).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_LineLine.htm)

#### `public static bool LineLine(Line lineA, Line lineB, out double a, out double b, double tolerance, bool finiteSegments)`

Intersects two lines.

**Remarks:** If the lines are exactly parallel, meaning the system of equations used to find a and b has no numerical solution, then false is returned. If the lines are nearly parallel, which is often numerically true even if you think the lines look exactly parallel, then the closest points are found and true is returned. So, if you care about weeding out "parallel" lines, then you need to do something like the following:

**Parameters:**
- `lineA` (Rhino.Geometry.Line) — First line for intersection.
- `lineB` (Rhino.Geometry.Line) — Second line for intersection.
- `a` (System.Double) — Parameter on lineA that is closest to LineB. The shortest distance between the lines is the chord from lineA.PointAt(a) to lineB.PointAt(b)
- `b` (System.Double) — Parameter on lineB that is closest to LineA. The shortest distance between the lines is the chord from lineA.PointAt(a) to lineB.PointAt(b)
- `tolerance` (System.Double) — If tolerance > 0.0, then an intersection is reported only if the distance between the points is <= tolerance. If tolerance <= 0.0, then the closest point between the lines is reported.
- `finiteSegments` (System.Boolean) — If true, the input lines are treated as finite segments. If false, the input lines are treated as infinite lines.

**Returns:** `Boolean` — true if a closest point can be calculated and the result passes the tolerance parameter test; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_LineLine_1.htm)

#### `public static bool LinePlane(Line line, Plane plane, out double lineParameter)`

Intersects a line and a plane. This function only returns true if the intersection result is a single point (i.e. if the line is coincident with the plane then no intersection is assumed).

**Parameters:**
- `line` (Rhino.Geometry.Line) — Line for intersection.
- `plane` (Rhino.Geometry.Plane) — Plane to intersect.
- `lineParameter` (System.Double) — Parameter on line where intersection occurs. If the parameter is not within the {0, 1} Interval then the finite segment does not intersect the plane.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_LinePlane.htm)

#### `public static LineSphereIntersection LineSphere(Line line, Sphere sphere, out Point3d intersectionPoint1, out Point3d intersectionPoint2)`

Intersects a line with a sphere using exact calculations.

**Parameters:**
- `line` (Rhino.Geometry.Line) — Line for intersection.
- `sphere` (Rhino.Geometry.Sphere) — Sphere for intersection.
- `intersectionPoint1` (Rhino.Geometry.Point3d) — First intersection point.
- `intersectionPoint2` (Rhino.Geometry.Point3d) — Second intersection point.

**Returns:** `LineSphereIntersection` — If None is returned, the first point is the point on the line closest to the sphere and the second point is the point on the sphere closest to the line. If Single is returned, the first point is the point on the line and the second point is the same point on the sphere.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_LineSphere.htm)

#### `public static Point3d[] MeshLine(Mesh mesh, Line line)`

Finds the intersections of a mesh and a line.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — A mesh to intersect
- `line` (Rhino.Geometry.Line) — The line to intersect with the mesh

**Returns:** `Point3d[]` — An array of points: one for each face that was passed by the faceIds out reference. Empty if no items are found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_MeshLine.htm)

#### `public static Point3d[] MeshLine(Mesh mesh, Line line, out int[] faceIds)`

Finds the intersections of a mesh and a line. The points are not necessarily sorted.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — A mesh to intersect
- `line` (Rhino.Geometry.Line) — The line to intersect with the mesh
- `faceIds` (System.Int32[]) — The indices of the intersecting faces. This out reference is assigned during the call. Empty if nothing is found.

**Returns:** `Point3d[]` — An array of points: one for each face that was passed by the faceIds out reference. Empty if no items are found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_MeshLine_1.htm)

#### `public static Point3d[] MeshLineSorted(Mesh mesh, Line line, out int[] faceIds)`

Finds the intersections of a mesh and a line. Points are sorted along the line.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — A mesh to intersect
- `line` (Rhino.Geometry.Line) — The line to intersect with the mesh
- `faceIds` (System.Int32[]) — The indices of the intersecting faces. This out reference is assigned during the call. Empty if nothing is found.

**Returns:** `Point3d[]` — An array of points: one for each face that was passed by the faceIds out reference. Empty if no items are found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_MeshLineSorted.htm)

#### `public static bool MeshMesh(IEnumerable<Mesh> meshes, double tolerance, out Polyline[] intersections, bool overlapsPolylines, out Polyline[] overlapsPolylinesResult, bool overlapsMesh, out Mesh overlapsMeshResult, TextLog textLog, CancellationToken cancel, IProgress<double> progress)`

Intersects meshes. Overlaps and perforations are provided in the output list.

**Parameters:**
- `meshes` (System.Collections.Generic.IEnumerable<Mesh>) — The mesh input list. This cannot be null. Null entries are tolerated.
- `tolerance` (System.Double) — A tolerance value. If negative, the positive value will be used. WARNING! Good tolerance values are in the magnitude of 10^-7, or RhinoMath.SqrtEpsilon*10.
- `intersections` (Rhino.Geometry.Polyline[]) — Returns the intersections.
- `overlapsPolylines` (System.Boolean) — If true, overlaps are computed and returned.
- `overlapsPolylinesResult` (Rhino.Geometry.Polyline[]) — If requested, overlaps are returned here.
- `overlapsMesh` (System.Boolean) — If true, an overlaps mesh is computed and returned.
- `overlapsMeshResult` (Rhino.Geometry.Mesh) — If requested, overlaps are returned here.
- `textLog` (Rhino.FileIO.TextLog) — A text log, or null.
- `cancel` (System.Threading.CancellationToken) — A cancellation token to stop the computation at a given point.
- `progress` (System.IProgress<Double>) — A progress reporter to inform the user about progress, or null. The reported value is indicative.

**Returns:** `Boolean` — True, if the operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_MeshMesh.htm)

#### `public static Polyline[] MeshMeshAccurate(Mesh meshA, Mesh meshB, double tolerance)`

Intersects two meshes. Overlaps and near misses are handled. This is an old method kept for compatibility.

**Parameters:**
- `meshA` (Rhino.Geometry.Mesh) — First mesh for intersection.
- `meshB` (Rhino.Geometry.Mesh) — Second mesh for intersection.
- `tolerance` (System.Double) — A tolerance value. If negative, the positive value will be used. WARNING! Good tolerance values are in the magnitude of 10^-7, or RhinoMath.SqrtEpsilon*10.

**Returns:** `Polyline[]` — An array of intersection and overlaps polylines.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_MeshMeshAccurate.htm)

#### `public static Line[] MeshMeshFast(Mesh meshA, Mesh meshB)`

This is an old overload kept for compatibility. Overlaps and near misses are ignored.

**Parameters:**
- `meshA` (Rhino.Geometry.Mesh) — First mesh for intersection.
- `meshB` (Rhino.Geometry.Mesh) — Second mesh for intersection.

**Returns:** `Line[]` — An array of intersection line segments, or null if no intersections were found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_MeshMeshFast.htm)

#### `public static bool MeshMeshPredicate(IEnumerable<Mesh> meshes, double tolerance, out int[] pairs, TextLog textLog)`

Determines if meshes intersect or overlap.

**Parameters:**
- `meshes` (System.Collections.Generic.IEnumerable<Mesh>) — The mesh input list. This cannot be null. Null entries are tolerated.
- `tolerance` (System.Double) — A tolerance value. If negative, the positive value will be used. WARNING! Good tolerance values are in the magnitude of 10^-7, or RhinoMath.SqrtEpsilon*10.
- `pairs` (System.Int32[]) — An array containing pairs of meshes that intersect.
- `textLog` (Rhino.FileIO.TextLog) — A text log, or null.

**Returns:** `Boolean` — True, if meshes intersect or overlap, otherwise false. False is also returned on error, but textLog will start with Error: and contain the error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_MeshMeshPredicate.htm)

#### `public static Polyline[] MeshPlane(Mesh mesh, IEnumerable<Plane> planes)`

Intersects a mesh with a collection of infinite planes.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — Mesh to intersect.
- `planes` (System.Collections.Generic.IEnumerable<Plane>) — Planes to intersect with.

**Returns:** `Polyline[]` — An array of polylines describing the intersection loops, or null if no intersections could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_MeshPlane_3.htm)

#### `public static Polyline[] MeshPlane(Mesh mesh, MeshIntersectionCache cache, IEnumerable<Plane> planes, double tolerance)`

Intersects a mesh with a collection of infinite planes.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — Mesh to intersect.
- `cache` (Rhino.Geometry.Intersect.MeshIntersectionCache) — Intersection cache for the mesh.
- `planes` (System.Collections.Generic.IEnumerable<Plane>) — Planes to intersect with.
- `tolerance` (System.Double) — Intersection tolerance.

**Returns:** `Polyline[]` — An array of polylines describing the intersection loops, or null if no intersections could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_MeshPlane_1.htm)

#### `public static Polyline[] MeshPlane(Mesh mesh, MeshIntersectionCache cache, Plane plane, double tolerance)`

Intersects a mesh with an infinite plane.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — Mesh to intersect.
- `cache` (Rhino.Geometry.Intersect.MeshIntersectionCache) — Intersection cache for mesh.
- `plane` (Rhino.Geometry.Plane) — Plane to intersect with.
- `tolerance` (System.Double) — Intersection tolerance.

**Returns:** `Polyline[]` — An array of polylines describing the intersection loops, or null if no intersections could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_MeshPlane.htm)

#### `public static Polyline[] MeshPlane(Mesh mesh, Plane plane)`

Intersects a mesh with an infinite plane.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — Mesh to intersect.
- `plane` (Rhino.Geometry.Plane) — Plane to intersect with.

**Returns:** `Polyline[]` — An array of polylines describing the intersection loops, or null if no intersections could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_MeshPlane_2.htm)

#### `public static Point3d[] MeshPolyline(Mesh mesh, PolylineCurve curve, out int[] faceIds)`

Finds the intersection of a mesh and a polyline. Starting from version 7, points are always sorted along the polyline.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — A mesh to intersect.
- `curve` (Rhino.Geometry.PolylineCurve) — A polyline curves to intersect.
- `faceIds` (System.Int32[]) — The indices of the intersecting faces. This out reference is assigned during the call.

**Returns:** `Point3d[]` — An array of points: one for each face that was passed by the faceIds out reference.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_MeshPolyline.htm)

#### `public static Point3d[] MeshPolylineSorted(Mesh mesh, PolylineCurve curve, out int[] faceIds)`

Finds the intersection of a mesh and a polyline. Points are guaranteed to be sorted along the polyline.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — A mesh to intersect.
- `curve` (Rhino.Geometry.PolylineCurve) — A polyline curves to intersect.
- `faceIds` (System.Int32[]) — The indices of the intersecting faces. This out reference is assigned during the call.

**Returns:** `Point3d[]` — An array of points: one for each face that was passed by the faceIds out reference.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_MeshPolylineSorted.htm)

#### `public static double MeshRay(Mesh mesh, Ray3d ray)`

Finds the first intersection of a ray with a mesh.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — A mesh to intersect.
- `ray` (Rhino.Geometry.Ray3d) — A ray to be casted.

**Returns:** `Double` — >= 0.0 parameter along ray if successful. < 0.0 if no intersection found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_MeshRay.htm)

#### `public static double MeshRay(Mesh mesh, Ray3d ray, out int[] meshFaceIndices)`

Finds the first intersection of a ray with a mesh.

**Remarks:** The ray may intersect more than one face in cases where the ray hits the edge between two faces or the vertex corner shared by multiple faces, or more than one face is coplanar or happens to be intersecting at the same location as the ray.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — A mesh to intersect.
- `ray` (Rhino.Geometry.Ray3d) — A ray to be casted.
- `meshFaceIndices` (System.Int32[]) — faces on mesh that ray intersects.

**Returns:** `Double` — >= 0.0 parameter along ray if successful. < 0.0 if no intersection found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_MeshRay_1.htm)

#### `public static bool PlaneBoundingBox(Plane plane, BoundingBox boundingBox, out Polyline polyline)`

Intersects a plane and a bounding box.

**Remarks:** Intersects the four bounding box infinite lines in the direction of the max coordinate with the plane. Point and single line intersections are ignored.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — The plane.
- `boundingBox` (Rhino.Geometry.BoundingBox) — The bounding box.
- `polyline` (Rhino.Geometry.Polyline) — The output polyline if successful.

**Returns:** `Boolean` — True if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_PlaneBoundingBox.htm)

#### `public static PlaneCircleIntersection PlaneCircle(Plane plane, Circle circle, out double firstCircleParameter, out double secondCircleParameter)`

Intersects a plane with a circle using exact calculations.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — Plane to intersect.
- `circle` (Rhino.Geometry.Circle) — Circe to intersect.
- `firstCircleParameter` (System.Double) — First intersection parameter on circle if successful or RhinoMath.UnsetValue if not.
- `secondCircleParameter` (System.Double) — Second intersection parameter on circle if successful or RhinoMath.UnsetValue if not.

**Returns:** `PlaneCircleIntersection` — The type of intersection that occurred.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_PlaneCircle.htm)

#### `public static bool PlanePlane(Plane planeA, Plane planeB, out Line intersectionLine)`

Intersects two planes and return the intersection line. If the planes are parallel or coincident, no intersection is assumed.

**Parameters:**
- `planeA` (Rhino.Geometry.Plane) — First plane for intersection.
- `planeB` (Rhino.Geometry.Plane) — Second plane for intersection.
- `intersectionLine` (Rhino.Geometry.Line) — If this function returns true, the intersectionLine parameter will return the line where the planes intersect.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_PlanePlane.htm)

#### `public static bool PlanePlanePlane(Plane planeA, Plane planeB, Plane planeC, out Point3d intersectionPoint)`

Intersects three planes to find the single point they all share.

**Parameters:**
- `planeA` (Rhino.Geometry.Plane) — First plane for intersection.
- `planeB` (Rhino.Geometry.Plane) — Second plane for intersection.
- `planeC` (Rhino.Geometry.Plane) — Third plane for intersection.
- `intersectionPoint` (Rhino.Geometry.Point3d) — Point where all three planes converge.

**Returns:** `Boolean` — true on success, false on failure. If at least two out of the three planes are parallel or coincident, failure is assumed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_PlanePlanePlane.htm)

#### `public static PlaneSphereIntersection PlaneSphere(Plane plane, Sphere sphere, out Circle intersectionCircle)`

Intersects a plane with a sphere using exact calculations.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — Plane to intersect.
- `sphere` (Rhino.Geometry.Sphere) — Sphere to intersect.
- `intersectionCircle` (Rhino.Geometry.Circle) — Intersection result.

**Returns:** `PlaneSphereIntersection` — If None is returned, the intersectionCircle has a radius of zero and the center point is the point on the plane closest to the sphere.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_PlaneSphere.htm)

#### `public static Point3d[] ProjectPointsToBreps(IEnumerable<Brep> breps, IEnumerable<Point3d> points, Vector3d direction, double tolerance)`

Projects points onto breps.

**Parameters:**
- `breps` (System.Collections.Generic.IEnumerable<Brep>) — The breps projection targets.
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — The points to project.
- `direction` (Rhino.Geometry.Vector3d) — The direction to project.
- `tolerance` (System.Double) — The tolerance used for intersections.

**Returns:** `Point3d[]` — Array of projected points, or null in case of any error or invalid input.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_ProjectPointsToBreps.htm)

#### `public static Point3d[] ProjectPointsToBrepsEx(IEnumerable<Brep> breps, IEnumerable<Point3d> points, Vector3d direction, double tolerance, out int[] indices)`

Projects points onto breps.

**Parameters:**
- `breps` (System.Collections.Generic.IEnumerable<Brep>) — The breps projection targets.
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — The points to project.
- `direction` (Rhino.Geometry.Vector3d) — The direction to project.
- `tolerance` (System.Double) — The tolerance used for intersections.
- `indices` (System.Int32[]) — Return points[i] is a projection of points[indices[i]]

**Returns:** `Point3d[]` — Array of projected points, or null in case of any error or invalid input.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_ProjectPointsToBrepsEx.htm)

#### `public static Point3d[] ProjectPointsToMeshes(IEnumerable<Mesh> meshes, IEnumerable<Point3d> points, Vector3d direction, double tolerance)`

Projects points onto meshes.

**Parameters:**
- `meshes` (System.Collections.Generic.IEnumerable<Mesh>) — the meshes to project on to.
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — the points to project.
- `direction` (Rhino.Geometry.Vector3d) — the direction to project.
- `tolerance` (System.Double) — Projection tolerances used for culling close points and for line-mesh intersection.

**Returns:** `Point3d[]` — Array of projected points, or null in case of any error or invalid input.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_ProjectPointsToMeshes.htm)

#### `public static Point3d[] ProjectPointsToMeshesEx(IEnumerable<Mesh> meshes, IEnumerable<Point3d> points, Vector3d direction, double tolerance, out int[] indices)`

Projects points onto meshes.

**Parameters:**
- `meshes` (System.Collections.Generic.IEnumerable<Mesh>) — the meshes to project on to.
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — the points to project.
- `direction` (Rhino.Geometry.Vector3d) — the direction to project.
- `tolerance` (System.Double) — Projection tolerances used for culling close points and for line-mesh intersection.
- `indices` (System.Int32[]) — Return points[i] is a projection of points[indices[i]]

**Returns:** `Point3d[]` — Array of projected points, or null in case of any error or invalid input.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_ProjectPointsToMeshesEx.htm)

#### `public static RayShootEvent[] RayShoot(IEnumerable<GeometryBase> geometry, Ray3d ray, int maxReflections)`

Computes point intersections that occur when shooting a ray to a collection of surfaces and Breps.

**Parameters:**
- `geometry` (System.Collections.Generic.IEnumerable<GeometryBase>) — The collection of surfaces and Breps to intersect. Trims are ignored on Breps.
- `ray` (Rhino.Geometry.Ray3d) — >A ray used in intersection.
- `maxReflections` (System.Int32) — The maximum number of reflections. This value should be any value between 1 and 1000, inclusive.

**Returns:** `RayShootEvent[]` — An array of RayShootEvent structs if successful, or an empty array on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_RayShoot_1.htm)

#### `public static Point3d[] RayShoot(Ray3d ray, IEnumerable<GeometryBase> geometry, int maxReflections)`

Computes point intersections that occur when shooting a ray to a collection of surfaces and Breps.

**Parameters:**
- `ray` (Rhino.Geometry.Ray3d) — A ray used in intersection.
- `geometry` (System.Collections.Generic.IEnumerable<GeometryBase>) — Only Surface and Brep objects are currently supported. Trims are ignored on Breps.
- `maxReflections` (System.Int32) — The maximum number of reflections. This value should be any value between 1 and 1000, inclusive.

**Returns:** `Point3d[]` — An array of points: one for each surface or Brep face that was hit, or an empty array on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_RayShoot.htm)

#### `public static SphereSphereIntersection SphereSphere(Sphere sphereA, Sphere sphereB, out Circle intersectionCircle)`

Intersects two spheres using exact calculations.

**Parameters:**
- `sphereA` (Rhino.Geometry.Sphere) — First sphere to intersect.
- `sphereB` (Rhino.Geometry.Sphere) — Second sphere to intersect.
- `intersectionCircle` (Rhino.Geometry.Circle) — If intersection is a point, then that point will be the center, radius 0.

**Returns:** `SphereSphereIntersection` — The intersection type.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_SphereSphere.htm)

#### `public static bool SurfaceSurface(Surface surfaceA, Surface surfaceB, double tolerance, out Curve[] intersectionCurves, out Point3d[] intersectionPoints)`

Intersects two Surfaces.

**Parameters:**
- `surfaceA` (Rhino.Geometry.Surface) — First Surface for intersection.
- `surfaceB` (Rhino.Geometry.Surface) — Second Surface for intersection.
- `tolerance` (System.Double) — Intersection tolerance.
- `intersectionCurves` (Rhino.Geometry.Curve[]) — The intersection curves will be returned here.
- `intersectionPoints` (Rhino.Geometry.Point3d[]) — The intersection points will be returned here.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_Intersection_SurfaceSurface.htm)

### Properties
- `MeshIntersectionsTolerancesCoefficient` (Double, get) — Offers a requested adjustment coefficient for mesh-mesh intersections tolerances. The value can be used to multiply the document absolute tolerance.This is only a UI value; it is up to developer to honor (or not) this request, depending on application needs.

## IntersectionEvent (class)

Provides all the information for a single Curve Intersection event.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Intersect_IntersectionEvent.htm)

### Constructors
- `public IntersectionEvent()` — Initializes a new instance of the IntersectionEvent class

### Methods
#### `public static bool CompareEquivalent(IntersectionEvent eventA, IntersectionEvent eventB, double relativePointTolerance)`

Compare intersection events.

**Parameters:**
- `eventA` (Rhino.Geometry.Intersect.IntersectionEvent) — The first intersection event to compare.
- `eventB` (Rhino.Geometry.Intersect.IntersectionEvent) — The second intersection event to compare.
- `relativePointTolerance` (System.Double) — The comparison tolerance. If RhinoMath.UnsetValue, then RhinoMath.SqrtEpsilon is used.

**Returns:** `Boolean` — true if the two inputs represent the same intersection, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_IntersectionEvent_CompareEquivalent.htm)

#### `public static bool CompareEquivalent(IntersectionEvent eventA, IntersectionEvent eventB, double relativePointTolerance, TextLog log)`

Compare intersection events.

**Parameters:**
- `eventA` (Rhino.Geometry.Intersect.IntersectionEvent) — The first intersection event to compare.
- `eventB` (Rhino.Geometry.Intersect.IntersectionEvent) — The second intersection event to compare.
- `relativePointTolerance` (System.Double) — The comparison tolerance. If RhinoMath.UnsetValue, then RhinoMath.SqrtEpsilon is used.
- `log` (Rhino.FileIO.TextLog) — If not null and false is returned, then a description of the error is appended to log.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Geometry.Intersect.IntersectionEvent.CompareEquivalent(Rhino.Geometry.Intersect.IntersectionEvent,Rhino.Geometry.Intersect.IntersectionEvent,System.Double,Rhino.FileIO.TextLog)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_IntersectionEvent_CompareEquivalent_1.htm)

#### `public void SurfaceOverlapParameter(out Interval uDomain, out Interval vDomain)`

If this instance records a Curve|Surface intersection event, and the intersection type if overlap, then use this function to get the U and V domains on the surface where the overlap occurs.

**Parameters:**
- `uDomain` (Rhino.Geometry.Interval) — Domain along surface U direction for overlap event.
- `vDomain` (Rhino.Geometry.Interval) — Domain along surface V direction for overlap event.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_IntersectionEvent_SurfaceOverlapParameter.htm)

#### `public void SurfacePointParameter(out double u, out double v)`

If this instance records a Curve|Surface intersection event, and the intersection type is point, then use this function to get the U and V parameters on the surface where the intersection occurs.

**Parameters:**
- `u` (System.Double) — Parameter on surface u direction where the intersection occurs.
- `v` (System.Double) — Parameter on surface v direction where the intersection occurs.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_IntersectionEvent_SurfacePointParameter.htm)

### Properties
- `IsOverlap` (Boolean, get) — All curve intersection events are either a single point or an overlap.
- `IsPoint` (Boolean, get) — All curve intersection events are either a single point or an overlap.
- `OverlapA` (Interval, get) — Gets the interval on curve A where the overlap occurs. If the intersection type is not overlap, this value is meaningless.
- `OverlapB` (Interval, get) — Gets the interval on curve B where the overlap occurs. If the intersection type is not overlap, this value is meaningless.
- `ParameterA` (Double, get) — Gets the parameter on Curve A where the intersection occurred. If the intersection type is overlap, then this will return the start of the overlap region.
- `ParameterB` (Double, get) — Gets the parameter on Curve B where the intersection occurred. If the intersection type is overlap, then this will return the start of the overlap region.
- `PointA` (Point3d, get) — Gets the point on Curve A where the intersection occurred. If the intersection type is overlap, then this will return the start of the overlap region.
- `PointA2` (Point3d, get) — Gets the end point of the overlap on Curve A. If the intersection type is not overlap, this value is meaningless.
- `PointB` (Point3d, get) — Gets the point on Curve B (or Surface B) where the intersection occurred. If the intersection type is overlap, then this will return the start of the overlap region.
- `PointB2` (Point3d, get) — Gets the end point of the overlap on Curve B (or Surface B). If the intersection type is not overlap, this value is meaningless.

## LineCircleIntersection (enum)

Represents all possible cases of a Line|Circle intersection event.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Intersect_LineCircleIntersection.htm)

### Values
- `None` = `0` — No intersections.
- `Single` = `1` — One intersection.
- `Multiple` = `2` — Two intersections.

## LineCylinderIntersection (enum)

Represents all possible cases of a Line|Cylinder intersection event.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Intersect_LineCylinderIntersection.htm)

### Values
- `None` = `0` — No intersections.
- `Single` = `1` — One intersection.
- `Multiple` = `2` — Two intersections.
- `Overlap` = `3` — Line lies on cylinder.

## LineSphereIntersection (enum)

Represents all possible cases of a Line|Sphere intersection event.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Intersect_LineSphereIntersection.htm)

### Values
- `None` = `0` — No intersections.
- `Single` = `1` — One intersection.
- `Multiple` = `2` — Two intersections.

## MeshClash (class)

Represents a particular instance of a clash or intersection between two meshes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Intersect_MeshClash.htm)

### Methods
#### `public static Mesh[] FindDetail(RhinoObject objA, RhinoObject objB, double distance)`

Finds all of the mesh faces on each of two Rhino objects that interfere within a clash distance. This function uses the object's mesh to calculate the interferences. Acceptable object types include: BrepObject, ExtrusionObject, MeshObject, and SubDObject.

**Parameters:**
- `objA` (Rhino.DocObjects.RhinoObject) — The first Rhino object.
- `objB` (Rhino.DocObjects.RhinoObject) — The second Rhino object.
- `distance` (System.Double) — The largest distance at which a clash can occur.

**Returns:** `Mesh[]` — The resulting meshes are sub-meshes of the input meshes if successful, or an empty array on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_MeshClash_FindDetail.htm)

#### `public static Mesh[] FindDetail(RhinoObject objA, RhinoObject objB, double distance, MeshType meshType, MeshingParameters meshingParameters)`

Finds all of the mesh faces on each of two Rhino objects that interfere within a clash distance. This function uses the object's mesh to calculate the interferences. Acceptable object types include: BrepObject, ExtrusionObject, MeshObject, and SubDObject.

**Parameters:**
- `objA` (Rhino.DocObjects.RhinoObject) — The first Rhino object.
- `objB` (Rhino.DocObjects.RhinoObject) — The second Rhino object.
- `distance` (System.Double) — The largest distance at which a clash can occur.
- `meshType` (Rhino.Geometry.MeshType) — The type of mesh to be used for the calculation.
- `meshingParameters` (Rhino.Geometry.MeshingParameters) — The meshing parameters used to generate meshes for the calculation.

**Returns:** `Mesh[]` — The resulting meshes are sub-meshes of the input meshes if successful, or an empty array on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_MeshClash_FindDetail_1.htm)

#### `public static MeshClash[] Search(IEnumerable<Mesh> setA, IEnumerable<Mesh> setB, double distance, int maxEventCount)`

Searches for locations where the distance from a mesh in one set of meshes is less than distance to another mesh in a second set of meshes.

**Parameters:**
- `setA` (System.Collections.Generic.IEnumerable<Mesh>) — The first set of meshes.
- `setB` (System.Collections.Generic.IEnumerable<Mesh>) — The second set of meshes.
- `distance` (System.Double) — The largest distance at which there is a clash. All values smaller than this cause a clash as well.
- `maxEventCount` (System.Int32) — The maximum number of clash objects.

**Returns:** `MeshClash[]` — An array of clash objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_MeshClash_Search_4.htm)

#### `public static MeshInterference[] Search(IEnumerable<RhinoObject> setA, IEnumerable<RhinoObject> setB, double distance)`

Searches for locations where the distance from a RhinoObject, in one set of objects, is less than the specified distance to another RhinoObject in a second set of objects. This function uses the object's mesh to calculate the interferences. Acceptable object types include: BrepObject, ExtrusionObject, MeshObject, and SubDObject.

**Parameters:**
- `setA` (System.Collections.Generic.IEnumerable<RhinoObject>) — The first set of Rhino objects.
- `setB` (System.Collections.Generic.IEnumerable<RhinoObject>) — The second set of Rhino objects.
- `distance` (System.Double) — The largest distance at which a clash can occur.

**Returns:** `MeshInterference[]` — An array of mesh interference object if successful, or an empty array on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_MeshClash_Search_2.htm)

#### `public static MeshInterference[] Search(IEnumerable<RhinoObject> setA, IEnumerable<RhinoObject> setB, double distance, MeshType meshType, MeshingParameters meshingParameters)`

Searches for locations where the distance from a RhinoObject, in one set of objects, is less than the specified distance to another RhinoObject in a second set of objects. This function uses the object's mesh to calculate the interferences. Acceptable object types include: BrepObject, ExtrusionObject, MeshObject, and SubDObject.

**Parameters:**
- `setA` (System.Collections.Generic.IEnumerable<RhinoObject>) — The first set of Rhino objects.
- `setB` (System.Collections.Generic.IEnumerable<RhinoObject>) — The second set of Rhino objects.
- `distance` (System.Double) — The largest distance at which a clash can occur.
- `meshType` (Rhino.Geometry.MeshType) — The type of mesh to be used for the calculation.
- `meshingParameters` (Rhino.Geometry.MeshingParameters) — The meshing parameters used to generate meshes for the calculation.

**Returns:** `MeshInterference[]` — An array of mesh interference object if successful, or an empty array on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_MeshClash_Search_3.htm)

#### `public static MeshClash[] Search(Mesh meshA, IEnumerable<Mesh> setB, double distance, int maxEventCount)`

Searches the locations where the distance from the first mesh to a mesh in the second set of meshes is less than the provided value.

**Parameters:**
- `meshA` (Rhino.Geometry.Mesh) — The first mesh.
- `setB` (System.Collections.Generic.IEnumerable<Mesh>) — The second set of meshes.
- `distance` (System.Double) — The largest distance at which there is a clash. All values smaller than this cause a clash as well.
- `maxEventCount` (System.Int32) — The maximum number of clash objects.

**Returns:** `MeshClash[]` — An array of clash objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_MeshClash_Search_1.htm)

#### `public static MeshClash[] Search(Mesh meshA, Mesh meshB, double distance, int maxEventCount)`

Searches the locations where the distance from the first mesh to the second mesh is less than the provided value.

**Parameters:**
- `meshA` (Rhino.Geometry.Mesh) — The first mesh.
- `meshB` (Rhino.Geometry.Mesh) — The second mesh.
- `distance` (System.Double) — The largest distance at which there is a clash. All values smaller than this cause a clash as well.
- `maxEventCount` (System.Int32) — The maximum number of clash objects.

**Returns:** `MeshClash[]` — An array of clash objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_MeshClash_Search.htm)

### Properties
- `ClashPoint` (Point3d, get) — If valid, then the sphere centered at ClashPoint of ClashRadius distance intersects the clashing meshes.
- `ClashRadius` (Double, get) — Gets the clash, or intersection, radius.
- `MeshA` (Mesh, get) — Gets the first mesh.
- `MeshB` (Mesh, get) — Gets the second mesh.

## MeshInterference (struct)

Represents an element which is part of a clash or intersection between two meshes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Intersect_MeshInterference.htm)

### Properties
- `HitPoints` (Point3d[], get/set) — Array of hit points where the objects of IndexA and IndexB interfere.
- `IndexA` (Int32, get/set) — The index of the first clashing, or interfering object.
- `IndexB` (Int32, get/set) — The index of the second clashing, or interfering object.

## MeshIntersectionCache (class)

Provides a mechanism for lazily evaluating mesh data.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Intersect_MeshIntersectionCache.htm)

### Constructors
- `public MeshIntersectionCache()` — Provides a mechanism for lazily evaluating mesh data. The implementation is private and subject to change.

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_MeshIntersectionCache_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Disposes the mesh intersection cache.

**Parameters:**
- `disposing` (System.Boolean) — If set to true dispose was called explicitly, otherwise specify false if calling from a finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_MeshIntersectionCache_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Intersect_MeshIntersectionCache_Finalize.htm)

## PlaneCircleIntersection (enum)

Represents all possible cases of a Plane|Circle intersection event.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Intersect_PlaneCircleIntersection.htm)

### Values
- `None` = `0` — No intersections. Either because radius is too small or because circle plane is parallel but not coincident with the intersection plane.
- `Tangent` = `1` — Tangent (one point) intersection.
- `Secant` = `2` — Secant (two point) intersection.
- `Parallel` = `3` — Circle and plane are planar but not coincident. Parallel indicates no intersection took place.
- `Coincident` = `4` — Circle and plane are co-planar, they intersect everywhere.

## PlaneSphereIntersection (enum)

Represents all possible cases of a Plane|Sphere intersection event.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Intersect_PlaneSphereIntersection.htm)

### Values
- `None` = `0` — No intersections.
- `Point` = `1` — Tangent intersection.
- `Circle` = `2` — Circular intersection.

## RayShootEvent (struct)

Represents an element which is part of a ray shoot.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Intersect_RayShootEvent.htm)

### Properties
- `BrepFaceIndex` (Int32, get/set) — If GeometryIndex references a Brep, then the index of the Brep face that was hit. If GeometryIndex references a surface, than RhinoMath.UnsetIntIndex.
- `GeometryIndex` (Int32, get/set) — The index of the surface or Brep that was hit.
- `Point` (Point3d, get/set) — The 3d hit point.

## SphereSphereIntersection (enum)

Represents all possible cases of a Sphere|Sphere intersection event.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Intersect_SphereSphereIntersection.htm)

### Values
- `None` = `0` — Spheres do not intersect.
- `Point` = `1` — Spheres touch at a single point.
- `Circle` = `2` — Spheres intersect at a circle.
- `Overlap` = `3` — Spheres are identical.

