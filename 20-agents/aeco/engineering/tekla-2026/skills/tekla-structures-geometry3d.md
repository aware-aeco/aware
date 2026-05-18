---
name: tekla-tekla-structures-geometry3d
description: This skill encodes the tekla 2026.0 surface of the Tekla.Structures.Geometry3d namespace — 27 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AABB, Arc, CoordinateSystem, Distance, FacetedBrep, FacetedBrepFaceHole, FacetedBrepFace, FacetedBrepWithNormals, and 19 more types.
---

# Tekla.Structures.Geometry3d

Auto-generated from vendor docs for tekla 2026.0. 27 types in this namespace.

## AABB (class)

The AABB class represents an axis-aligned 3d bounding box.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/1f5576c5-859c-2d9b-55f4-5b4d66bbf320)

### Constructors
- `public AABB()` — Instantiates a new axis-aligned 3d bounding box with the maximum point initialized to the smallest possible value and the minimum point to the largest possible value.
- `public AABB(AABB AABB)` — Instantiates a new axis-aligned 3d bounding box which is a copy of the given axis-aligned 3d bounding box.
- `public AABB(IEnumerable<Point> Points)` — Constructs AABB from given point list.
- `public AABB(Point MinPoint, Point MaxPoint)` — Instantiates a new axis-aligned 3d bounding box with the given minimum and maximum points.

### Methods
#### `public bool Collide(AABB Other)`

Checks if the current axis-aligned 3d bounding box collides with another given axis-aligned 3d bounding box. Both axis-aligned 3d bounding boxes need to be in the same coordinate system or in the same workplane, so that they are defined using the same axes.

**Parameters:**
- `Other` (Tekla.Structures.Geometry3d.AABB) — Another axis-aligned 3d bounding box to be used in the collision check.

**Returns:** `Boolean` — True if the axis-aligned 3d bounding boxes collide.

[Docs](https://developer.tekla.com/topic/en/18/47/057b4309-6593-9b8a-581e-8233518f597a)

#### `public Point GetCenterPoint()`

Returns the geometric center point of the current axis-aligned 3d bounding box.

**Returns:** `Point` — The geometric center point of the current axis-aligned 3d bounding box.

[Docs](https://developer.tekla.com/topic/en/18/47/015fb65b-ad44-3e33-e891-0d9351e340cf)

#### `public Point[] GetCornerPoints()`

Returns all corner points of the current axis-aligned 3d bounding box.

**Returns:** `.Point.` — All corner points of the current axis-aligned 3d bounding box.

[Docs](https://developer.tekla.com/topic/en/18/47/5f6f6ca4-13aa-d956-047a-f84112c028bb)

#### `public bool IsInside(LineSegment LineSegment)`

Checks if the given line segment is inside the current axis-aligned 3d bounding box.

**Parameters:**
- `LineSegment` (Tekla.Structures.Geometry3d.LineSegment) — The line segment to be checked.

**Returns:** `Boolean` — True if the point is inside the current axis-aligned 3d bounding box.

[Docs](https://developer.tekla.com/topic/en/18/47/31e46219-4fba-b1da-4b5b-2ab276eef609)

#### `public bool IsInside(Point Point)`

Checks if the given point is inside the current axis-aligned 3d bounding box.

**Parameters:**
- `Point` (Tekla.Structures.Geometry3d.Point) — The point to be checked.

**Returns:** `Boolean` — True if the point is inside the current axis-aligned 3d bounding box.

[Docs](https://developer.tekla.com/topic/en/18/47/29c49520-1316-8e73-c29e-f4c1dd49711d)

### Properties
- `MaxPoint` (Point, get/set) — The maximum point of the axis-aligned 3d bounding box.
- `MinPoint` (Point, get/set) — The minimum point of the axis-aligned 3d bounding box.

## Arc (class)

Represents an arc geometry

[Vendor docs](https://developer.tekla.com/topic/en/18/47/90601fc6-a64e-ea9f-c0e5-46edcf1909e9)

### Constructors
- `public Arc(Point startPoint, Point endPoint, Point pointOnArc)` — Constructs a new arc geometry
- `public Arc(Point centerPoint, Point startPoint, Vector normal, double deltaAngleRadians)` — Constructs a new arc geometry
- `public Arc(Point centerPoint, Vector startDirection, Vector startTangent, double radius, double deltaAngleRadians)` — Constructs a new arc geometry

### Methods
#### `public ICurve Clone()`

Returns a copy of itself

**Returns:** `ICurve` — The copy

[Docs](https://developer.tekla.com/topic/en/18/47/1f4dbf08-4da7-7c11-7c55-159f1f5b86e8)

#### `public bool Equals(ICurve other)`

Checks for equality with another curve

**Parameters:**
- `other` (Tekla.Structures.Geometry3d.ICurve) — Other curve

**Returns:** `Boolean` — True if the other curve is an arc and is equal to this

[Docs](https://developer.tekla.com/topic/en/18/47/1c2a7907-359a-8324-3650-cb35ab367763)

#### `public override bool Equals(Object other)`

Checks for equality with another arc

**Parameters:**
- `other` (System.Object) — Other arc

**Returns:** `Boolean` — True if equal, false otherwise

[Docs](https://developer.tekla.com/topic/en/18/47/88eada87-789f-9424-44aa-230246e3fdae)

#### `public override int GetHashCode()`

Get HashCode override

**Returns:** `Int32` — 

[Docs](https://developer.tekla.com/topic/en/18/47/e1176a87-c480-02f4-813f-7255afde47b3)

### Properties
- `Angle` (Double, get) — Gets angle of the arc in radians.
- `ArcMiddlePoint` (Point, get) — Gets the point located at the middle of the arc
- `CenterPoint` (Point, get) — Gets or sets center point of the arc.
- `EndPoint` (Point, get) — Gets or sets end point of the arc.
- `Length` (Double, get) — Returns the length of the arc
- `Normal` (Vector, get) — Gets normal of the arc, which defines the axis of rotation of the radial vector pointing to the start point of the arc.
- `Radius` (Double, get) — Gets the radius of the arc.
- `StartDirection` (Vector, get) — Gets the unit vector which points from the center point to the start point of the arc, which defines the X axis of the arc coordinate system.
- `StartPoint` (Point, get) — Gets or sets start point of the arc.
- `StartTangent` (Vector, get) — Gets the unit vector tangent to the start point which defines the Y axis of the arc coordinate system

## CoordinateSystem (class)

The CoordinateSystem class defines a coordinate system in space. The system is defined by an origin, an X-axis and a Y-axis. The Z-axis is the cross product of the X-axis and the Y-axis.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/62ca8a5c-4cfe-1fe0-5009-75b994b06a07)

### Constructors
- `public CoordinateSystem()` — Instantiates a coordinate system in the current work plane.
- `public CoordinateSystem(Point Origin, Vector AxisX, Vector AxisY)` — Instantiates a coordinate system with the given origin, X-axis and Y-axis.

### Properties
- `AxisX` (Vector, get/set) — The X-axis of the coordinate system.
- `AxisY` (Vector, get/set) — The Y-axis of the coordinate system.
- `Origin` (Point, get/set) — The origin of the coordinate system.

## Distance (class)

The Distance class contains methods for calculating the distance between geometric objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/521b8d2d-b375-34c3-3af2-2baa10d02611)

### Methods
#### `public static double PointToLine(Point Point, Line Line)`

Returns the distance between the given point and line.

**Parameters:**
- `Point` (Tekla.Structures.Geometry3d.Point) — The point to be used.
- `Line` (Tekla.Structures.Geometry3d.Line) — The line to be used.

**Returns:** `Double` — The distance between the given point and line.

[Docs](https://developer.tekla.com/topic/en/18/47/11a2eb0e-0732-164f-31c8-c85b56929f4d)

#### `public static double PointToLineSegment(Point Point, LineSegment LineSegment)`

Returns the distance between the given point and line segment.

**Parameters:**
- `Point` (Tekla.Structures.Geometry3d.Point) — The point to be used.
- `LineSegment` (Tekla.Structures.Geometry3d.LineSegment) — The line segment to be used.

**Returns:** `Double` — The distance between the given point and line segment.

[Docs](https://developer.tekla.com/topic/en/18/47/9e10cbbd-9168-5dcc-391e-c6fc3cfd130b)

#### `public static double PointToPlane(Point Point, GeometricPlane Plane)`

Returns the distance between the given point and plane.

**Parameters:**
- `Point` (Tekla.Structures.Geometry3d.Point) — The point to be used.
- `Plane` (Tekla.Structures.Geometry3d.GeometricPlane) — The plane to be used.

**Returns:** `Double` — The distance between the given point and plane.

[Docs](https://developer.tekla.com/topic/en/18/47/95fcd6fe-bccf-7fce-b43e-63613ed787be)

#### `public static double PointToPoint(Point Point1, Point Point2)`

Returns the distance between the given two points.

**Parameters:**
- `Point1` (Tekla.Structures.Geometry3d.Point) — The first point to be used.
- `Point2` (Tekla.Structures.Geometry3d.Point) — The second point to be used.

**Returns:** `Double` — The distance between the given points.

[Docs](https://developer.tekla.com/topic/en/18/47/6b98cdad-9a85-b545-9c21-d101f7354eb3)

## FacetedBrep (class)

The FacetedBrep class defines a faceted BREP.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/de56c80a-cf25-a6c3-42ee-195ee7615a12)

### Constructors
- `public FacetedBrep(Vector[] vertices, int[][] outerWires, IDictionary<int, int[][]> innerWires)` — Initializes a new instance of the FacetedBrep class.
- `public FacetedBrep(Vector[] vertices, int[][] outerWires, IDictionary<int, int[][]> innerWires, IList<IndirectPolymeshEdge> edges)` — Initializes a new instance of the FacetedBrep class.

### Methods
#### `public bool CheckForTwoManifold()`

Gets a value indicating whether this instance is 2-manifold.

**Returns:** `Boolean` — true if this instance is 2-manifold; otherwise false.

[Docs](https://developer.tekla.com/topic/en/18/47/7ed60d58-765f-eb09-d6e3-4fab3bbfd756)

#### `public int[] GetInnerFace(int faceIndex)`

Gets the inner face at the specified index.

**Parameters:**
- `faceIndex` (System.Int32) — The index of the face.

**Returns:** `.Int32.` — The array of vertex indexes for the inner face.

[Docs](https://developer.tekla.com/topic/en/18/47/cff69feb-f110-d653-a99a-5ce21f4e314e)

#### `public int GetInnerFaceCount(int faceIndex)`

Gets the inner face count of the outer face at the specified index.

**Parameters:**
- `faceIndex` (System.Int32) — The index of the face.

**Returns:** `Int32` — The count of inner faces for the outer face.

[Docs](https://developer.tekla.com/topic/en/18/47/885fcb90-953a-2c8f-0aa9-a636087c2f0a)

#### `public int[] GetOuterFace(int faceIndex)`

Gets the outer face at the specified index.

**Parameters:**
- `faceIndex` (System.Int32) — The index of the face.

**Returns:** `.Int32.` — The array of vertex indexes for the outer face.

[Docs](https://developer.tekla.com/topic/en/18/47/43194089-a2f0-b88b-ec45-94b6039e0393)

### Properties
- `Faces` (ICollection<FacetedBrepFace>, get) — Gets the faces.
- `GetEdges` (IList<IndirectPolymeshEdge>, get) — Gets the edges.
- `InnerWires` (IDictionary<Int32,..Int32..>, get) — Gets the inner wires.
- `OuterWires` (..Int32.., get) — Gets the outer wires.
- `Vertices` (IList<Vector>, get) — Gets the vertices.

## FacetedBrepFace (class)

The FacetedBrepFace class defines a face of a faceted BREP. A FacetedBrepFace cannot be instantiated directly. They are managed internally by a FacetedBrep.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/932c1e79-b0cd-465e-1e16-210095e65415)

### Properties
- `HasHoles` (Boolean, get) — Gets a value indicating whether this instance has holes.
- `Holes` (IList<FacetedBrepFaceHole>, get) — Gets the holes.
- `IsReadOnly` (Boolean, get) — Gets a value indicating whether this instance is read only.
- `VerticeIndexes` (IList<Int32>, get) — Gets the vertex indexes.
- `Vertices` (IList<Vector>, get) — Gets the vertices.

## FacetedBrepFaceHole (class)

The FacetedBrepFaceHole class defines a hole on a faceted BREP face. A FacetedBrepFaceHole cannot be instantiated directly. They are managed internally by a FacetedBrepFace. You can look also at the code example for the class FacetedBrepFace to see more examples.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5e278d3b-6ae3-35bd-fd33-6bb3fac88cfd)

### Properties
- `Count` (Int32, get) — Gets the count;
- `IsReadOnly` (Boolean, get) — Gets a value indicating whether this instance is read only.
- `VerticeIndexes` (IList<Int32>, get) — Gets the vertice indexes.
- `Vertices` (IList<Vector>, get) — Gets the vertices.

## FacetedBrepWithNormals (class)

The FacetedBRepWithNormals class defines a faceted BREP that has also vertex normal vectors.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9ce06f7c-25e8-3267-435d-f3943a722c9e)

### Constructors
- `public FacetedBrepWithNormals(Vector[] vertices, int[][] outerWires, IDictionary<int, int[][]> innerWires, Vector[] normals)` — Initializes a new instance of the FacetedBrepWithNormals class.

### Methods
#### `public bool CheckForTwoManifold()`

Gets a value indicating whether this instance is 2-manifold.

**Returns:** `Boolean` — true if this instance is 2-manifold; otherwise false.

[Docs](https://developer.tekla.com/topic/en/18/47/7ed60d58-765f-eb09-d6e3-4fab3bbfd756)

#### `public int[] GetInnerFace(int faceIndex)`

Gets the inner face at the specified index.

**Parameters:**
- `faceIndex` (System.Int32) — The index of the face.

**Returns:** `.Int32.` — The array of vertex indexes for the inner face.

[Docs](https://developer.tekla.com/topic/en/18/47/cff69feb-f110-d653-a99a-5ce21f4e314e)

#### `public int GetInnerFaceCount(int faceIndex)`

Gets the inner face count of the outer face at the specified index.

**Parameters:**
- `faceIndex` (System.Int32) — The index of the face.

**Returns:** `Int32` — The count of inner faces for the outer face.

[Docs](https://developer.tekla.com/topic/en/18/47/885fcb90-953a-2c8f-0aa9-a636087c2f0a)

#### `public int[] GetOuterFace(int faceIndex)`

Gets the outer face at the specified index.

**Parameters:**
- `faceIndex` (System.Int32) — The index of the face.

**Returns:** `.Int32.` — The array of vertex indexes for the outer face.

[Docs](https://developer.tekla.com/topic/en/18/47/43194089-a2f0-b88b-ec45-94b6039e0393)

### Properties
- `Faces` (ICollection<FacetedBrepFace>, get) — Gets the faces.
- `GetEdges` (IList<IndirectPolymeshEdge>, get) — Gets the edges.
- `InnerWires` (IDictionary<Int32,..Int32..>, get) — Gets the inner wires.
- `Normals` (.Vector., get/set) — Gets or sets the vertex normal unit vectors
- `OuterWires` (..Int32.., get) — Gets the outer wires.
- `Vertices` (IList<Vector>, get) — Gets the vertices.

## GeometricPlane (class)

The GeometricPlane class represents a 3d geometric plane.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/cfcb353e-74cf-bffb-3b89-9b22f9d5e2e4)

### Constructors
- `public GeometricPlane()` — Instantiates an XY-plane with the origin at (0,0,0).
- `public GeometricPlane(CoordinateSystem CoordSys)` — Instantiates a plane defined by the given coordinate system.
- `public GeometricPlane(Point Origin, Vector Normal)` — Instantiates a plane defined by the given origin point and normal vector.
- `public GeometricPlane(Point Origin, Vector Xaxis, Vector Yaxis)` — Instantiates a plane defined by the given origin, X-axis vector and Y-axis vector.

### Methods
#### `public Vector GetNormal()`

Returns a normalized normal vector of the plane.

**Returns:** `Vector` — The normalized normal vector of the plane.

[Docs](https://developer.tekla.com/topic/en/18/47/0d4be4d7-ed58-feb0-756c-76428dd49e0d)

### Properties
- `Normal` (Vector, get/set) — The normal vector of the plane.
- `Origin` (Point, get/set) — The origin point of the plane.

## GeometryConstants (class)

The Constants class of Geometry3d holds certain constant values that are used internally by the other geometry classes.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/b5a32e82-3d8b-f5cc-5abd-24b7c92d8db4)

### Constructors
- `public GeometryConstants()` — Initializes a new instance of the GeometryConstants class

## IBoundingVolume (interface)

The BoundingVolume interface represents any generic 3D bounding volume.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/1114b1ad-5f17-f45d-5879-ccce0b9366ef)

## ICurve (interface)

Interface that represents an abstract curved geometry defined by a set of points. This interface is intended to be the base class of 3D curves.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/101740ff-cebe-b199-47f4-7dbaa88045d6)

### Methods
#### `ICurve Clone()`

Returns a deep copy of the geometry

**Returns:** `ICurve` — Copy of self

[Docs](https://developer.tekla.com/topic/en/18/47/cfb5dd66-0d7e-3aec-4fb1-38b1615347b5)

### Properties
- `EndPoint` (Point, get) — Gets the end point of the curve. May be the same as StartPoint if the curve is closed.
- `Length` (Double, get) — Gets the length of the curve
- `StartPoint` (Point, get) — Gets the start point of the curve

## IndirectPolymeshEdge (class)

The IndirectEdge class represents a single global edge in a solid using integer indexes to reference to vertices instead of directly using geometric (coordinate) values.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/bfb0abb5-fc13-1e3b-c2e3-3709696304f8)

### Constructors
- `public IndirectPolymeshEdge()` — Initializes a new instance of the IndirectPolymeshEdge class

### Properties
- `EdgeType` (PolymeshEdgeTypeEnum, get/set) — Type of the edge.
- `EndPoint` (Int32, get/set) — End point of the edge.
- `ShellIndex` (Int32, get/set) — The index of the shell the edge belongs to.
- `StartPoint` (Int32, get/set) — Start point of the edge.

## Intersection (class)

The Intersection class contains methods for calculating intersections between geometric objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/35693308-f07a-d279-aaf9-a56b86cd7d5f)

### Methods
#### `public static LineSegment LineSegmentToObb(LineSegment lineSegment, OBB obb)`

Returns a new line segment which is an intersection of the given line segment and the oriented bounding box or null if the line segment and oriented bounding box do not intersect.

**Parameters:**
- `lineSegment` (Tekla.Structures.Geometry3d.LineSegment) — Line segment to be used.
- `obb` (Tekla.Structures.Geometry3d.OBB) — Oriented bounding box to be used.

**Returns:** `LineSegment` — The intersection line segment or null if no intersection was found.

[Docs](https://developer.tekla.com/topic/en/18/47/e8e3d881-f138-39fc-4d4e-11b314c5c563)

#### `public static Point LineSegmentToPlane(LineSegment lineSegment, GeometricPlane plane)`

Returns a new point which is an intersection of the given line segment and plane or null if the line segment and the plane are parallel or do not intersect.

**Parameters:**
- `lineSegment` (Tekla.Structures.Geometry3d.LineSegment) — The line segment to be used.
- `plane` (Tekla.Structures.Geometry3d.GeometricPlane) — The plane to be used.

**Returns:** `Point` — The intersection point or null if no intersection was found.

[Docs](https://developer.tekla.com/topic/en/18/47/0225ef4f-26b0-b781-547c-eaaa48b705ce)

#### `public static LineSegment LineToLine(Line line1, Line line2)`

Returns a new line segment which is the shortest path between the given lines or null if the lines are parallel. If the resulting line segment has a length of 0.0, the given lines actually intersect in 3d space.

**Parameters:**
- `line1` (Tekla.Structures.Geometry3d.Line) — The first line to be used.
- `line2` (Tekla.Structures.Geometry3d.Line) — The second line to be used.

**Returns:** `LineSegment` — The shortest line segment between the given lines or null if the lines are parallel.

[Docs](https://developer.tekla.com/topic/en/18/47/36ab76a6-451c-6982-4ab2-eaf927e916d9)

#### `public static LineSegment LineToObb(Line line, OBB obb)`

Returns a new line segment which is an intersection of the given line and the oriented bounding box or null if the line and oriented bounding box do not intersect.

**Parameters:**
- `line` (Tekla.Structures.Geometry3d.Line) — Line to be used.
- `obb` (Tekla.Structures.Geometry3d.OBB) — Oriented bounding box to be used.

**Returns:** `LineSegment` — The intersection line segment or null if no intersection was found.

[Docs](https://developer.tekla.com/topic/en/18/47/9fbd9b8f-a9f9-17d7-4ae9-4b3c53c84b9f)

#### `public static Point LineToPlane(Line line, GeometricPlane plane)`

Returns a new point which is an intersection of the given line and plane or null if the line and the plane are parallel.

**Parameters:**
- `line` (Tekla.Structures.Geometry3d.Line) — The line to be used.
- `plane` (Tekla.Structures.Geometry3d.GeometricPlane) — The plane to be used.

**Returns:** `Point` — The intersection point or null if no intersection was found.

[Docs](https://developer.tekla.com/topic/en/18/47/0600c85a-e9a9-ce7a-694e-d76674556e8c)

#### `public static Line PlaneToPlane(GeometricPlane plane1, GeometricPlane plane2)`

Returns a new line which is an intersection of the given two planes or null if the planes are parallel.

**Parameters:**
- `plane1` (Tekla.Structures.Geometry3d.GeometricPlane) — The first plane to be used.
- `plane2` (Tekla.Structures.Geometry3d.GeometricPlane) — The second plane to be used.

**Returns:** `Line` — The intersection line or null if no intersection was found.

[Docs](https://developer.tekla.com/topic/en/18/47/cc70bcb0-7baf-4576-7fad-72dfacb97b61)

## Line (class)

The Line class represents a single infinite line in 3D space. See LineSegment for the implementation of a segment of a line.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/abc6b564-46d8-8c73-982a-a898f4337cc2)

### Constructors
- `public Line()` — Instantiates a line with an undefined direction.
- `public Line(LineSegment LineSegment)` — Instantiates a line defined by the given line segment.
- `public Line(Point p1, Point p2)` — Instantiates a line defined by the given points.
- `public Line(Point Point, Vector Direction)` — Instantiates a line defined by the given point and direction vector.

### Properties
- `Direction` (Vector, get/set) — The direction vector of the line.
- `Origin` (Point, get/set) — The origin of the line.

## LineSegment (class)

The LineSegment class represents a single finite segment of a line in 3D space. See Line for the implementation of a straight line.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/e075bf75-c88c-4d32-eb59-077962e93594)

### Constructors
- `public LineSegment()` — Instantiates a line segment with both the starting point and the end point zeroed.
- `public LineSegment(Point Point1, Point Point2)` — Instantiates a line segment with the given points.

### Methods
#### `public ICurve Clone()`

Creates a copy of itself

**Returns:** `ICurve` — The copy

[Docs](https://developer.tekla.com/topic/en/18/47/c81ac600-994d-2a96-c537-414a626db1ed)

#### `public bool Equals(ICurve other)`

Returns true if the other curve is a line segment equal to this

**Parameters:**
- `other` (Tekla.Structures.Geometry3d.ICurve) — The curve to check equality against

**Returns:** `Boolean` — True if the curves are equal

[Docs](https://developer.tekla.com/topic/en/18/47/77485a19-6d04-fc4e-4cf9-0e369a926157)

#### `public override bool Equals(Object o)`

Returns true if the objects are equal.

**Parameters:**
- `o` (System.Object) — The object that equality is wished to be checked with.

**Returns:** `Boolean` — True if the objects are equal.

[Docs](https://developer.tekla.com/topic/en/18/47/08148a1d-6e76-8b28-eeae-52e13634094c)

#### `public Vector GetDirectionVector()`

Returns a new unit direction vector of a line segment.

**Returns:** `Vector` — The unit direction vector of the line segment.

[Docs](https://developer.tekla.com/topic/en/18/47/7a0004f3-fada-07e1-a7c0-a897501b2a08)

#### `public override int GetHashCode()`

Returns a hash code for a line segment. Notice, in extremely rare cases, you might not get the same hash code for two line segments even though they are considered equal! This should, however, happen only in extremely rare cases!

**Returns:** `Int32` — The hash code for the line segment.

[Docs](https://developer.tekla.com/topic/en/18/47/39347eea-0647-bafb-9893-b512589edf21)

#### `public double Length()`

Returns the length of a line segment.

**Returns:** `Double` — The length of the line segment.

[Docs](https://developer.tekla.com/topic/en/18/47/6abe1629-ed9b-50a9-0761-d7d75ee187bc)

### Properties
- `EndPoint` (Point, get) — Gets the end point of the line segment
- `Point1` (Point, get/set) — Gets the start point of the line segment
- `Point2` (Point, get/set) — Gets the end point of the line segment
- `StartPoint` (Point, get) — Gets the start point of the line segment

## Matrix (class)

The Matrix class represents a 4x3 matrix.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/ec2ec811-7205-62fe-315b-c5935a5e20b8)

### Constructors
- `public Matrix()` — Creates a new indentity matrix.
- `public Matrix(Matrix m)` — Creates a new matrix which is a copy of the given matrix.

### Methods
#### `public Matrix GetTranspose()`

Returns a new matrix which is a transpose of the current matrix. The transposed matrix is an inversion of the current matrix, if the current matrix was a valid rotation matrix.

**Returns:** `Matrix` — The new transposed matrix.

[Docs](https://developer.tekla.com/topic/en/18/47/a4b57ab7-e31f-2043-abe8-e73b6acdd4b3)

#### `public override string ToString()`

Returns a string that represents the current matrix.

**Returns:** `String` — The string that represents the current matrix.

[Docs](https://developer.tekla.com/topic/en/18/47/da738b67-6880-722e-9732-116eae2286dc)

#### `public IEnumerable<Point> Transform(IEnumerable<Point> Points)`

Transforms the given points using the current matrix.

**Parameters:**
- `Points` (System.Collections.Generic.IEnumerable<Point>) — The points to be transformed.

**Returns:** `IEnumerable<Point>` — The new transformed points.

[Docs](https://developer.tekla.com/topic/en/18/47/abb1fdf5-d970-ae5f-7429-843be1235ce8)

#### `public Point Transform(Point p)`

Transforms the given point using the current matrix.

**Parameters:**
- `p` (Tekla.Structures.Geometry3d.Point) — The point to be transformed.

**Returns:** `Point` — The new transformed point.

[Docs](https://developer.tekla.com/topic/en/18/47/473f82dc-4f16-3fc1-2b7c-7c673fcd2f1a)

#### `public void Transpose()`

Transposes a matrix. The resulting matrix is an inversion of the current matrix, if the current matrix was a valid rotation matrix.

[Docs](https://developer.tekla.com/topic/en/18/47/8801696e-5b19-3242-507c-28b3318ee3b4)

### Properties
- `Item` (object, get/set) — Sets or gets the matrix element values.

## MatrixFactory (class)

The MatrixFactory class provides a convenient way to generate different kinds of transformation matrices.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/ef97ef94-ca37-e418-1974-02453a5183fe)

### Methods
#### `public static Matrix ByCoordinateSystems(CoordinateSystem CoordSys1, CoordinateSystem CoordSys2)`

Returns a coordinate transformation matrix defined by two coordinate systems. With the returned matrix points can be transformed from the first coordinate system to the second coordinate system. The ByCoordinateSystems method is meant for transforming points between coordinate systems asked in the same work plane.

**Parameters:**
- `CoordSys1` (Tekla.Structures.Geometry3d.CoordinateSystem) — The coordinate system to start from.
- `CoordSys2` (Tekla.Structures.Geometry3d.CoordinateSystem) — The target coordinate system.

**Returns:** `Matrix` — The transformation matrix defined by the two coordinate systems.

[Docs](https://developer.tekla.com/topic/en/18/47/164a690d-7f09-985b-d7a7-cc0259e6bc53)

#### `public static Matrix FromCoordinateSystem(CoordinateSystem CoordSys)`

Returns a coordinate transformation matrix defined by the given coordinate system. With the returned matrix points can be transformed from the given coordinate system to the current work plane coordinate system.

**Parameters:**
- `CoordSys` (Tekla.Structures.Geometry3d.CoordinateSystem) — The coordinate system to transform points from.

**Returns:** `Matrix` — The transformation matrix defined by the given coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/eeb1ab4c-20e3-6a13-15f0-a1f5947c8dce)

#### `public static Matrix Rotate(double Angle, Vector Axis)`

Returns a rotation matrix in a Clockwise rotation around the given rotation axis, defined by the given angle and the given rotation axis.

**Parameters:**
- `Angle` (System.Double) — The rotation angle (in radians).
- `Axis` (Tekla.Structures.Geometry3d.Vector) — The rotation axis.

**Returns:** `Matrix` — The new rotation matrix.

[Docs](https://developer.tekla.com/topic/en/18/47/9421178e-d986-44f4-f2f4-e2d753bd2fa9)

#### `public static Matrix ToCoordinateSystem(CoordinateSystem CoordSys)`

Returns a coordinate transformation matrix defined by the given coordinate system. With the returned matrix points can be transformed from the current work plane coordinate system to the given coordinate system.

**Parameters:**
- `CoordSys` (Tekla.Structures.Geometry3d.CoordinateSystem) — The coordinate system to transform points to.

**Returns:** `Matrix` — The transformation matrix defined by the given coordinate system.

[Docs](https://developer.tekla.com/topic/en/18/47/7dbba383-7bd0-e7a1-c409-9b0e9aa50f36)

## OBB (class)

The OBB class represents an oriented 3d bounding box.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5c37ba3e-c3a3-b597-5998-714e46e499c8)

### Constructors
- `public OBB()` — Initializes a new instance of the OBB class.
- `public OBB(OBB obb)` — Initializes a new instance of the OBB class.
- `public OBB(Point center, Vector[] axis, double[] extent)` — Initializes a new instance of the OBB class.
- `public OBB(Point center, Vector axis0, Vector axis1, Vector axis2, double extent0, double extent1, double extent2)` — Initializes a new instance of the OBB class.

### Methods
#### `public Point ClosestPointTo(Line line)`

Calculates the closest point in OBB to given Line.

**Parameters:**
- `line` (Tekla.Structures.Geometry3d.Line) — Line to calculate against.

**Returns:** `Point` — Closest point in OBB to given Line.

[Docs](https://developer.tekla.com/topic/en/18/47/a25d70d5-3938-345a-a199-ab5b4b770e43)

#### `public Point ClosestPointTo(LineSegment lineSegment)`

Calculates the closest point in OBB to given LineSegment.

**Parameters:**
- `lineSegment` (Tekla.Structures.Geometry3d.LineSegment) — LineSegment to calculate against.

**Returns:** `Point` — Closest point in OBB to given LineSegment.

[Docs](https://developer.tekla.com/topic/en/18/47/6edbf96c-c1c5-3fea-7f13-bed5db6c5ee4)

#### `public Point ClosestPointTo(Point point)`

Calculates the closest point in OBB to given point.

**Parameters:**
- `point` (Tekla.Structures.Geometry3d.Point) — Point to calculate against.

**Returns:** `Point` — Closest point in OBB to given point.

[Docs](https://developer.tekla.com/topic/en/18/47/8ec45629-7b3a-29a2-0392-2cc29be33c8e)

#### `public Point[] ComputeVertices()`

Calculates the corner points of the OBB.

**Returns:** `.Point.` — Array of corner points.

[Docs](https://developer.tekla.com/topic/en/18/47/dcd2474b-161b-da47-6359-bf876385b237)

#### `public double DistanceTo(Line line)`

Calculates the distance from OBB to given Line.

**Parameters:**
- `line` (Tekla.Structures.Geometry3d.Line) — Line to calculate against.

**Returns:** `Double` — Distance from OBB to given Line.

[Docs](https://developer.tekla.com/topic/en/18/47/2610518f-52c2-ea1b-3c7a-1c7180057ce3)

#### `public double DistanceTo(LineSegment lineSegment)`

Calculates the distance from OBB to given LineSegment.

**Parameters:**
- `lineSegment` (Tekla.Structures.Geometry3d.LineSegment) — LineSegment to calculate against.

**Returns:** `Double` — Distance from OBB to given LineSegment.

[Docs](https://developer.tekla.com/topic/en/18/47/c9a3589b-b8a8-2d5b-e527-e9b42c43650e)

#### `public double DistanceTo(Point point)`

Calculates the distance from OBB to given point.

**Parameters:**
- `point` (Tekla.Structures.Geometry3d.Point) — Point to calculate against.

**Returns:** `Double` — Distance from OBB to given point.

[Docs](https://developer.tekla.com/topic/en/18/47/8c9b771f-0733-4f40-dbdc-140832c3130e)

#### `public bool Equals(OBB other)`

Tests for the exact equality of two OBBs.

**Parameters:**
- `other` (Tekla.Structures.Geometry3d.OBB) — OBB to compare.

**Returns:** `Boolean` — True if the OBBs are equal, otherwise false.

[Docs](https://developer.tekla.com/topic/en/18/47/4352acdc-5e16-9feb-5113-64ff906e73ce)

#### `public override bool Equals(Object obj)`

Tests for the exact equality of two OBBs.

**Parameters:**
- `obj` (System.Object) — Object to compare.

**Returns:** `Boolean` — True if the OBBs are equal, otherwise false.

[Docs](https://developer.tekla.com/topic/en/18/47/7ff7e1d3-fb35-30cd-ac4e-322fe65a6109)

#### `public override int GetHashCode()`

Gets the hash code for the obb.

**Returns:** `Int32` — Hash code for the obb.

[Docs](https://developer.tekla.com/topic/en/18/47/67b6f680-a672-b7bf-bc9d-dce1437c1108)

#### `public Point[] IntersectionPointsWith(Line line)`

Calculates the intersection points between OBB and given Line.

**Parameters:**
- `line` (Tekla.Structures.Geometry3d.Line) — Line to calculate against.

**Returns:** `.Point.` — Array of intersection points.

[Docs](https://developer.tekla.com/topic/en/18/47/f70ed589-fe37-2ab8-ee91-77aac4ebc562)

#### `public Point[] IntersectionPointsWith(LineSegment lineSegment)`

Calculates the intersection points between OBB and given LineSegment.

**Parameters:**
- `lineSegment` (Tekla.Structures.Geometry3d.LineSegment) — LineSegment to calculate against.

**Returns:** `.Point.` — Array of intersection points.

[Docs](https://developer.tekla.com/topic/en/18/47/c248e24c-b2c3-85e3-60a7-9874f25e9c99)

#### `public LineSegment IntersectionWith(Line line)`

Creates an intersection between the OBB and the given Line.

**Parameters:**
- `line` (Tekla.Structures.Geometry3d.Line) — Line to calculate against

**Returns:** `LineSegment` — Intersection as LineSegment or null if no intersection was found.

[Docs](https://developer.tekla.com/topic/en/18/47/18b7c3cd-6ec9-5406-83dc-b059faf6195a)

#### `public LineSegment IntersectionWith(LineSegment lineSegment)`

Creates an intersection between the OBB and the given LineSegment.

**Parameters:**
- `lineSegment` (Tekla.Structures.Geometry3d.LineSegment) — LineSegment to calculate against

**Returns:** `LineSegment` — Intersection as LineSegment or null if no intersection was found.

[Docs](https://developer.tekla.com/topic/en/18/47/bfb9225e-4d44-5202-e089-bf6922640275)

#### `public bool Intersects(GeometricPlane geometricPlane)`

Tests if current OBB intersects with the given GeometricPlane

**Parameters:**
- `geometricPlane` (Tekla.Structures.Geometry3d.GeometricPlane) — GeometricPlane to test against

**Returns:** `Boolean` — True if the OBB intersects with the given GeometricPlane, otherwise false

[Docs](https://developer.tekla.com/topic/en/18/47/7d52dbda-9f4a-3724-a90a-8657f81ef392)

#### `public bool Intersects(Line line)`

Tests if current OBB intersects with the given Line

**Parameters:**
- `line` (Tekla.Structures.Geometry3d.Line) — Line to test against

**Returns:** `Boolean` — True if the OBB intersects with the given Line, otherwise false

[Docs](https://developer.tekla.com/topic/en/18/47/d5afac1a-997b-13dd-e4e1-dcae5d52a536)

#### `public bool Intersects(LineSegment lineSegment)`

Tests if current OBB intersects with the given LineSegment

**Parameters:**
- `lineSegment` (Tekla.Structures.Geometry3d.LineSegment) — LineSegment to test against

**Returns:** `Boolean` — True if the OBB intersects with the given LineSegment, otherwise false

[Docs](https://developer.tekla.com/topic/en/18/47/de6988ef-d6f1-80fb-e617-712faf099d59)

#### `public bool Intersects(OBB obb)`

Tests if current OBB intersects with the given OBB

**Parameters:**
- `obb` (Tekla.Structures.Geometry3d.OBB) — OBB to test against

**Returns:** `Boolean` — True if the OBBs intersect, otherwise false

[Docs](https://developer.tekla.com/topic/en/18/47/d81fab1c-b77c-982c-c180-b511b40252e0)

#### `public void SetAxis(Vector[] axis)`

Sets the Axis to the OBB.

**Parameters:**
- `axis` (.Tekla.Structures.Geometry3d.Vector.) — Three element array of unit-length vectors parallel the axis of the OBB.

[Docs](https://developer.tekla.com/topic/en/18/47/72ab5a44-1b83-4e60-f54c-304d9dff66b6)

#### `public void SetAxis(Vector axis0, Vector axis1, Vector axis2)`

Sets the Axis to the OBB.

**Parameters:**
- `axis0` (Tekla.Structures.Geometry3d.Vector) — Unit-length vector parallel to one axis of the OBB.
- `axis1` (Tekla.Structures.Geometry3d.Vector) — Unit-length vector parallel to the second axis of the OBB.
- `axis2` (Tekla.Structures.Geometry3d.Vector) — Unit-length vector parallel to the third axis of the OBB.

[Docs](https://developer.tekla.com/topic/en/18/47/ef0df18d-695b-93ff-004a-6ff160d3a5b8)

#### `public void SetExtent(double[] extent)`

Sets the extents to the OBB.

**Parameters:**
- `extent` (.System.Double.) — Three element array of the OBB extents (half-lengths of the edges).

[Docs](https://developer.tekla.com/topic/en/18/47/6521170d-c8ab-0b5b-a4a1-67857790cde3)

#### `public void SetExtent(double extent0, double extent1, double extent2)`

Sets the extents to the OBB.

**Parameters:**
- `extent0` (System.Double) — The extent (half-length) of the side parallel to axis one.
- `extent1` (System.Double) — The extent (half-length) of the side parallel to axis two.
- `extent2` (System.Double) — The extent (half-length) of the side parallel to axis three.

[Docs](https://developer.tekla.com/topic/en/18/47/ae6c99a9-697a-e284-dcb5-bd8116fffcaa)

#### `public LineSegment ShortestSegmentTo(Line line)`

Calculates the shortest LineSegment from OBB to the given Line.

**Parameters:**
- `line` (Tekla.Structures.Geometry3d.Line) — Line to calculate against

**Returns:** `LineSegment` — Shortest LineSegment from OBB to the given Line.

[Docs](https://developer.tekla.com/topic/en/18/47/b6b25124-7c30-8e80-196d-f86f4f86762e)

#### `public LineSegment ShortestSegmentTo(LineSegment lineSegment)`

Calculates the shortest LineSegment from OBB to the given LineSegment.

**Parameters:**
- `lineSegment` (Tekla.Structures.Geometry3d.LineSegment) — LineSegment to calculate against

**Returns:** `LineSegment` — Shortest LineSegment from OBB to the given LineSegment.

[Docs](https://developer.tekla.com/topic/en/18/47/3aed389b-03cd-c05e-cc2d-0ea7863d558e)

#### `public LineSegment ShortestSegmentTo(Point point)`

Calculates the shortest LineSegment from OBB to the given point.

**Parameters:**
- `point` (Tekla.Structures.Geometry3d.Point) — Point to calculate against.

**Returns:** `LineSegment` — Shortest LineSegment from OBB to the given point.

[Docs](https://developer.tekla.com/topic/en/18/47/4670fbf1-45ee-bde7-32c2-d7edbf307843)

### Properties
- `Axis0` (Vector, get) — Gets an axis parallel to one side of the OBB, a unit-length vector orthogonal to Axis1 and Axis2.
- `Axis1` (Vector, get) — Gets an axis parallel to the second side of the OBB, a unit-length vector orthogonal to Axis0 and Axis2.
- `Axis2` (Vector, get) — Gets an axis parallel to the third side of the OBB, a unit-length vector orthogonal to Axis0 and Axis1.
- `Center` (Point, get/set) — Gets or sets the center point of the box.
- `Extent0` (Double, get/set) — Gets or sets the extent parallel to Axis0.
- `Extent1` (Double, get/set) — Gets or sets the extent parallel to Axis1.
- `Extent2` (Double, get/set) — Gets or sets the extent parallel to Axis2.

## Parallel (class)

The Parallel class contains methods for testing the parallelism of geometric objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/500ff3fe-9fc3-17dd-9776-c01150876d46)

### Methods
#### `public static bool LineSegmentToLineSegment(LineSegment LineSegment1, LineSegment LineSegment2)`

Returns true if the given line segments are parallel.

**Parameters:**
- `LineSegment1` (Tekla.Structures.Geometry3d.LineSegment) — The first line segment to be used.
- `LineSegment2` (Tekla.Structures.Geometry3d.LineSegment) — The second line segment to be used.

**Returns:** `Boolean` — True if the given line segments are parallel.

[Docs](https://developer.tekla.com/topic/en/18/47/3fefb26c-3308-e52b-ae2b-cba9ac001741)

#### `public static bool LineSegmentToLineSegment(LineSegment LineSegment1, LineSegment LineSegment2, double Tolerance)`

Returns true if the given line segments are parallel within the given angular tolerance.

**Parameters:**
- `LineSegment1` (Tekla.Structures.Geometry3d.LineSegment) — The first line segment to be used.
- `LineSegment2` (Tekla.Structures.Geometry3d.LineSegment) — The second line segment to be used.
- `Tolerance` (System.Double) — The angular tolerance (in radians) to be used.

**Returns:** `Boolean` — True if the given line segments are parallel within the given tolerance.

[Docs](https://developer.tekla.com/topic/en/18/47/44ea8e3b-9814-ec3b-a4e5-516c578a1494)

#### `public static bool LineSegmentToPlane(LineSegment LineSegment, GeometricPlane Plane)`

Returns true if the given line segment and plane are parallel.

**Parameters:**
- `LineSegment` (Tekla.Structures.Geometry3d.LineSegment) — The line segment to be used.
- `Plane` (Tekla.Structures.Geometry3d.GeometricPlane) — The plane to be used.

**Returns:** `Boolean` — True if the given line segment and plane are parallel.

[Docs](https://developer.tekla.com/topic/en/18/47/e9314182-a9a9-2302-1209-a29b53163c93)

#### `public static bool LineSegmentToPlane(LineSegment LineSegment, GeometricPlane Plane, double Tolerance)`

Returns true if the given line segment and plane are parallel within the given angular tolerance.

**Parameters:**
- `LineSegment` (Tekla.Structures.Geometry3d.LineSegment) — The line segment to be used.
- `Plane` (Tekla.Structures.Geometry3d.GeometricPlane) — The plane to be used.
- `Tolerance` (System.Double) — The angular tolerance (in radians) to be used.

**Returns:** `Boolean` — True if the given line segment and plane are parallel within the given tolerance.

[Docs](https://developer.tekla.com/topic/en/18/47/870d4a57-1b49-825b-2e31-a04a7bf43c35)

#### `public static bool LineToLine(Line Line1, Line Line2)`

Returns true if the given lines are parallel.

**Parameters:**
- `Line1` (Tekla.Structures.Geometry3d.Line) — The first line to be used.
- `Line2` (Tekla.Structures.Geometry3d.Line) — The second line to be used.

**Returns:** `Boolean` — True if the given lines are parallel.

[Docs](https://developer.tekla.com/topic/en/18/47/df4f30d9-ee2a-cee1-18c1-9b3d2d69ed89)

#### `public static bool LineToLine(Line Line1, Line Line2, double Tolerance)`

Returns true if the given lines are parallel within the given angular tolerance.

**Parameters:**
- `Line1` (Tekla.Structures.Geometry3d.Line) — The first line to be used.
- `Line2` (Tekla.Structures.Geometry3d.Line) — The second line to be used.
- `Tolerance` (System.Double) — The angular tolerance (in radians) to be used.

**Returns:** `Boolean` — True if the given lines are parallel within the given tolerance.

[Docs](https://developer.tekla.com/topic/en/18/47/0fbab871-aea2-72ba-a7dc-2e2a912050d8)

#### `public static bool LineToPlane(Line Line, GeometricPlane Plane)`

Returns true if the given line and plane are parallel.

**Parameters:**
- `Line` (Tekla.Structures.Geometry3d.Line) — The line to be used.
- `Plane` (Tekla.Structures.Geometry3d.GeometricPlane) — The plane to be used.

**Returns:** `Boolean` — True if the given line and plane are parallel.

[Docs](https://developer.tekla.com/topic/en/18/47/31959918-df5e-12cc-cd38-e1260cf7357a)

#### `public static bool LineToPlane(Line Line, GeometricPlane Plane, double Tolerance)`

Returns true if the given line and plane are parallel within the given angular tolerance.

**Parameters:**
- `Line` (Tekla.Structures.Geometry3d.Line) — The line to be used.
- `Plane` (Tekla.Structures.Geometry3d.GeometricPlane) — The plane to be used.
- `Tolerance` (System.Double) — The angular tolerance (in radians) to be used.

**Returns:** `Boolean` — True if the given line and plane are parallel within the given tolerance.

[Docs](https://developer.tekla.com/topic/en/18/47/e754eecb-6a64-d122-aad1-384160b958c9)

#### `public static bool PlaneToPlane(GeometricPlane Plane1, GeometricPlane Plane2)`

Returns true if the given planes are parallel.

**Parameters:**
- `Plane1` (Tekla.Structures.Geometry3d.GeometricPlane) — The first plane to be used.
- `Plane2` (Tekla.Structures.Geometry3d.GeometricPlane) — The second plane to be used.

**Returns:** `Boolean` — True if the given planes are parallel.

[Docs](https://developer.tekla.com/topic/en/18/47/6a5598cf-b34d-d891-95df-3e37bd53fcd8)

#### `public static bool PlaneToPlane(GeometricPlane Plane1, GeometricPlane Plane2, double Tolerance)`

Returns true if the given planes are parallel within the given angular tolerance.

**Parameters:**
- `Plane1` (Tekla.Structures.Geometry3d.GeometricPlane) — The first plane to be used.
- `Plane2` (Tekla.Structures.Geometry3d.GeometricPlane) — The second plane to be used.
- `Tolerance` (System.Double) — The angular tolerance (in radians) to be used.

**Returns:** `Boolean` — True if the given planes are parallel within the given tolerance.

[Docs](https://developer.tekla.com/topic/en/18/47/b4d81bf6-d30f-693a-2cc4-a848e0ed328f)

#### `public static bool VectorToPlane(Vector Vector, GeometricPlane Plane)`

Returns true if the given vector and plane are parallel.

**Parameters:**
- `Vector` (Tekla.Structures.Geometry3d.Vector) — The vector to be used.
- `Plane` (Tekla.Structures.Geometry3d.GeometricPlane) — The plane to be used.

**Returns:** `Boolean` — True if the given vector and plane are parallel.

[Docs](https://developer.tekla.com/topic/en/18/47/2c8b8357-5e18-facf-dece-f437f6f1e98e)

#### `public static bool VectorToPlane(Vector Vector, GeometricPlane Plane, double Tolerance)`

Returns true if the given vector and plane are parallel within the given angular tolerance.

**Parameters:**
- `Vector` (Tekla.Structures.Geometry3d.Vector) — The vector to be used.
- `Plane` (Tekla.Structures.Geometry3d.GeometricPlane) — The plane to be used.
- `Tolerance` (System.Double) — The angular tolerance (in radians) to be used.

**Returns:** `Boolean` — True if the given vector and plane are parallel within the given tolerance.

[Docs](https://developer.tekla.com/topic/en/18/47/feb7a799-8d7c-dae3-c162-4e1d83c5131c)

#### `public static bool VectorToVector(Vector Vector1, Vector Vector2)`

Returns true if the given vectors are parallel.

**Parameters:**
- `Vector1` (Tekla.Structures.Geometry3d.Vector) — The first vector to be used.
- `Vector2` (Tekla.Structures.Geometry3d.Vector) — The second vector to be used.

**Returns:** `Boolean` — True if the given vectors are parallel.

[Docs](https://developer.tekla.com/topic/en/18/47/8c02559b-3211-e5a0-95fa-43312faa6df2)

#### `public static bool VectorToVector(Vector Vector1, Vector Vector2, double Tolerance)`

Returns true if the given vectors are parallel within the given angular tolerance.

**Parameters:**
- `Vector1` (Tekla.Structures.Geometry3d.Vector) — The first vector to be used.
- `Vector2` (Tekla.Structures.Geometry3d.Vector) — The second vector to be used.
- `Tolerance` (System.Double) — The angular tolerance (in radians) to be used.

**Returns:** `Boolean` — True if the given vectors are parallel within the given tolerance.

[Docs](https://developer.tekla.com/topic/en/18/47/f77a5938-933c-2e87-c3cc-6e8132ee9c0f)

## Point (class)

The Point class represents a single position in 3D space.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c111c59a-fcae-cf59-bb15-632fb811ce4a)

### Constructors
- `public Point()` — Instantiates a point with zero members.
- `public Point(Point Point)` — Instantiates a point with the given other point.
- `public Point(double X, double Y)` — Instantiates a point with the given coordinates and zero Z-coordinate.
- `public Point(double X, double Y, double Z)` — Instantiates a point with the given coordinates.

### Methods
#### `public static bool AreEqual(Point Point1, Point Point2)`

Compares two points and tells whether they are equal.

**Parameters:**
- `Point1` (Tekla.Structures.Geometry3d.Point) — The first point to be compared.
- `Point2` (Tekla.Structures.Geometry3d.Point) — The second point to be compared.

**Returns:** `Boolean` — True if the points are equal, false otherwise.

[Docs](https://developer.tekla.com/topic/en/18/47/2b8178e8-b6d0-0e73-b572-399ae181e2b4)

#### `public int CompareTo(Object obj)`

Compares two points. To use binarysearch somekind of sorting should be used.

**Parameters:**
- `obj` (System.Object) — The point to be compared.

**Returns:** `Int32` — 0 if both are equal, -1 if this point is before, 1 if this point is after.

[Docs](https://developer.tekla.com/topic/en/18/47/2f202828-05fd-220f-92b1-5b64372a4f7e)

#### `public override bool Equals(Object obj)`

Returns true if the current object and the given object are equal.

**Parameters:**
- `obj` (System.Object) — The object we wish to check the equality with.

**Returns:** `Boolean` — True if the given object equals the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/dbca17f2-6730-e2c4-583c-90c74c72e0b0)

#### `public override int GetHashCode()`

Returns a hash code for the point. Notice, in extremely rare cases, you might not get the same hash code for two points even though they are considered equal! This should, however, happen only in extremely rare cases!

**Returns:** `Int32` — The hash code for the point.

[Docs](https://developer.tekla.com/topic/en/18/47/9f84f024-170b-5181-770d-5ed2f56332cd)

#### `public string ToString(CultureInfo cultureInfo)`

Formats the point into a string, using the provided culture.

**Parameters:**
- `cultureInfo` (System.Globalization.CultureInfo)

**Returns:** `String` — The string that represents the point.

[Docs](https://developer.tekla.com/topic/en/18/47/176a7423-000a-5fb0-dab9-659b87bf1505)

#### `public override string ToString()`

Formats the point into a string, using the current culture.

**Returns:** `String` — The string that represents the point.

[Docs](https://developer.tekla.com/topic/en/18/47/63db29a6-32f7-6696-31eb-a6084dcb62cd)

#### `public virtual void Translate(double X, double Y, double Z)`

Translates the point using the given vector.

**Parameters:**
- `X` (System.Double) — The X-translation to be used.
- `Y` (System.Double) — The Y-translation to be used.
- `Z` (System.Double) — The Z-translation to be used.

[Docs](https://developer.tekla.com/topic/en/18/47/7ad7ecb9-acf3-7ed2-f2bc-aa52b99ceae6)

#### `public void Zero()`

Zeros all the members of the point.

[Docs](https://developer.tekla.com/topic/en/18/47/9bf63e0c-fe71-40ae-e467-306f0a41f315)

## PolyLine (class)

The PolyLine class represents a line that consists of one or more line segments. To create a polyline, you have to give a list of the points that will form the polyline. The first point in the list will be connected with the second point in the list, the second point in the list will be connected with the third point in the list, etc.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5f792268-8861-1d47-978b-1380f7d4e1c0)

### Constructors
- `public PolyLine(IEnumerable Points)` — Instantiates a polyline with the given Points.

### Methods
#### `public override bool Equals(Object O)`

Returns true if the objects are equal.

**Parameters:**
- `O` (System.Object) — The object that equality is wished to be checked with.

**Returns:** `Boolean` — True if the objects are equal.

[Docs](https://developer.tekla.com/topic/en/18/47/b7ce0d5e-662d-c157-fbd6-40a9c01d7f72)

#### `public override int GetHashCode()`

Returns a hash code for a polyline. Notice, in extremely rare cases, you might not get the same hash code for two polylines even though they are considered equal! This should, however, happen only in extremely rare cases!

**Returns:** `Int32` — The hash code for the polyline.

[Docs](https://developer.tekla.com/topic/en/18/47/d2a8500c-8cab-16c4-78b7-881c3633b65b)

#### `public double Length()`

Returns the length of a polyline.

**Returns:** `Double` — The length of the polyline.

[Docs](https://developer.tekla.com/topic/en/18/47/9d429505-b15d-8a4d-bf62-1923a263ec03)

### Properties
- `Points` (ArrayList, get/set) — The points the polyline consists of.

## Polycurve (class)

Represents 3D polycurve geometry, which itself is composed of one or more connected curves. The class offers facilities for iteration through the polycurve, and implements the ICurve interface. Once this class is constructed, it represents an immutable polycurve. The curves that can be obtained during iteration are mere copies of the curves in the polycurve, and mutating them won't affect the polycurve. Please use the PolycurveGeometryBuilder class to build and manipulate this objects of this class. Since it implements IEnumerable, the curves can be iterated using foreach loops, and it supports all of the LINQ operations, and it is possible to build arbitrarily complex queries over the curves of this class.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/30425d6a-467a-1cf9-a182-0d5859a9b75d)

### Constructors
- `public Polycurve()` — Initializes a new instance of the Polycurve class.
- `public Polycurve(ICurve geometry)` — Initializes a new instance of the Polycurve class from a curve geometry.
- `public Polycurve(PolyLine polyLine)` — Initializes a new instance of the Polycurve class from a PolyLine. The lines are converted to line segments.

### Methods
#### `public ICurve Clone()`

Creates a deep copy of the polycurve

**Returns:** `ICurve` — The copy

[Docs](https://developer.tekla.com/topic/en/18/47/99934b85-1d06-7519-e2f4-088739ec4ff2)

#### `public bool Equals(ICurve other)`

Determines whether the polycurve equals another polycurve This function only checks for equality among polycurves

**Parameters:**
- `other` (Tekla.Structures.Geometry3d.ICurve) — The other polycurve

**Returns:** `Boolean` — True if equal, false otherwise

[Docs](https://developer.tekla.com/topic/en/18/47/db9385bc-cef2-213d-644b-0c6bd15583d1)

#### `public IEnumerator<ICurve> GetEnumerator()`

This method implements the IEnumerable interface

**Returns:** `IEnumerator<ICurve>` — Enumerator of the geometry list

[Docs](https://developer.tekla.com/topic/en/18/47/5f3b450c-d314-020c-dafa-3ae3b17c7a54)

### Properties
- `EndPoint` (Point, get) — Gets the end point of the polycurve
- `Length` (Double, get) — Gets the total length of the polycurve
- `StartPoint` (Point, get) — Gets the start point of the polycurve

## PolycurveGeometryBuilder (class)

The polycurve geometry builder class is in charge of building the geometry list of a polycurve. All the geometries that the builder connects must be connected on the correct end points. The builder has a fluent interface, and can be used as follows: Copyusing Tekla.Structures.Geometry3d; using Tekla.Structures.Model; public class Example { static void Main(string[] args) { var segment = new LineSegment(new Point(0, 0, 0), new Point(1000, 0, 0)); var arc = new Arc(new Point(1000, 0, 0), new Point(2000, 1000, 0), new Point(1707.1, 292.89, 0)); var geometryList = new PolycurveGeometrySolver().Append(arc).Append(segment).GetPolycurve(); var polycurve = new ControlPolycurve(); polycurve.Geometry = geometryList; polycurve.Insert(); } }

[Vendor docs](https://developer.tekla.com/topic/en/18/47/8704f981-6637-a70e-e44f-13a39aebd86c)

### Constructors
- `public PolycurveGeometryBuilder()` — Creates an empty geometry builder. This builder contains no geometries
- `public PolycurveGeometryBuilder(Polycurve geometries)` — Constructs a geometry builder from an existing geometry list

### Methods
#### `public PolycurveGeometryBuilder Append(Arc arc)`

Appends an arc to the geometry list

**Parameters:**
- `arc` (Tekla.Structures.Geometry3d.Arc) — Geometry to append

**Returns:** `PolycurveGeometryBuilder` — The builder itself

[Docs](https://developer.tekla.com/topic/en/18/47/130ffd77-0137-e50c-a906-f72980c4e529)

#### `public PolycurveGeometryBuilder Append(LineSegment segment)`

Appends a line segment to the geometry list

**Parameters:**
- `segment` (Tekla.Structures.Geometry3d.LineSegment) — Geometry to append

**Returns:** `PolycurveGeometryBuilder` — The builder itself

[Docs](https://developer.tekla.com/topic/en/18/47/b9618faf-1dc6-8997-d4a8-d97065f19276)

#### `public PolycurveGeometryBuilder Append(Polycurve otherPolycurve)`

Appends another polycurve to the geometry list

**Parameters:**
- `otherPolycurve` (Tekla.Structures.Geometry3d.Polycurve) — Polycurve to add to the list

**Returns:** `PolycurveGeometryBuilder` — The builder itself

[Docs](https://developer.tekla.com/topic/en/18/47/7c1cfb82-2a9a-472b-d0be-af5b1a424a15)

#### `public PolycurveGeometryBuilder AppendArc(Point middlePoint, Point endPoint)`

Appends an arc using three points. The first point is taken as the last point of the last element of the polycurve.

**Remarks:** This function requires that there is at least one curve in the polycurve

**Parameters:**
- `middlePoint` (Tekla.Structures.Geometry3d.Point) — Arc middle point
- `endPoint` (Tekla.Structures.Geometry3d.Point) — Arc end point

**Returns:** `PolycurveGeometryBuilder` — The builder itself

[Docs](https://developer.tekla.com/topic/en/18/47/6cff5ac1-35c8-1833-85dd-f61c704782f1)

#### `public PolycurveGeometryBuilder AppendSegment(Point endPoint)`

Appends an line segment. The first point is taken as the last point of the last element of the polycurve.

**Remarks:** This function requires that there is at least one curve in the polycurve

**Parameters:**
- `endPoint` (Tekla.Structures.Geometry3d.Point) — Segment end point

**Returns:** `PolycurveGeometryBuilder` — The builder itself

[Docs](https://developer.tekla.com/topic/en/18/47/877b5f08-530d-c3c6-3b27-b0388d1a8eaf)

#### `public PolycurveGeometryBuilder AppendTangentArc(Point endPoint)`

Appends an arc tangent to the last curve of the curve list.

**Remarks:** This function requires that there is at least one curve in the polycurve

**Parameters:**
- `endPoint` (Tekla.Structures.Geometry3d.Point) — End point of the arc

**Returns:** `PolycurveGeometryBuilder` — The builder itself

[Docs](https://developer.tekla.com/topic/en/18/47/1bfd7bf5-b45f-3e6c-5823-2d488e032604)

#### `public PolycurveGeometryBuilder AppendTangentSegment(double length)`

Appends a line segment of the given length tangent to the last curve of the list.

**Remarks:** This function requires that there is at least one curve in the polycurve

**Parameters:**
- `length` (System.Double) — Length of the segment to append

**Returns:** `PolycurveGeometryBuilder` — The builder itself

[Docs](https://developer.tekla.com/topic/en/18/47/b4d39724-45c4-0bdb-e9ac-f1e454a30e12)

#### `public Polycurve GetPolycurve()`

Gets the final geometry after all the modifications made by the builder

**Returns:** `Polycurve` — The final geometry list

[Docs](https://developer.tekla.com/topic/en/18/47/79f07950-e9ab-986e-5c7f-a6682de823c2)

#### `public PolycurveGeometryBuilder Prepend(Arc arc)`

Prepends an arc to the geometry list

**Parameters:**
- `arc` (Tekla.Structures.Geometry3d.Arc) — Geometry to prepend

**Returns:** `PolycurveGeometryBuilder` — The builder itself

[Docs](https://developer.tekla.com/topic/en/18/47/5bbbff95-4bb2-04cd-3de9-6877cfa3dd1c)

#### `public PolycurveGeometryBuilder Prepend(LineSegment segment)`

Prepends a line segment to the geometry list

**Parameters:**
- `segment` (Tekla.Structures.Geometry3d.LineSegment) — Geometry to prepend

**Returns:** `PolycurveGeometryBuilder` — The builder itself

[Docs](https://developer.tekla.com/topic/en/18/47/8d3c332e-9191-e060-cb8a-91ecf8387ef6)

#### `public PolycurveGeometryBuilder PrependArc(Point startPoint, Point middlePoint)`

Prepends an arc using three points. The last point is taken as the start point of the first element of the polycurve.

**Remarks:** This function requires that there is at least one curve in the polycurve

**Parameters:**
- `startPoint` (Tekla.Structures.Geometry3d.Point) — Arc start point
- `middlePoint` (Tekla.Structures.Geometry3d.Point) — Arc middle point

**Returns:** `PolycurveGeometryBuilder` — The builder itself

[Docs](https://developer.tekla.com/topic/en/18/47/99a847f2-de10-f995-b26a-bf5f1e9bf440)

#### `public PolycurveGeometryBuilder PrependSegment(Point startPoint)`

Prepends an line segment. The last point is taken as the start point of the first element of the polycurve.

**Remarks:** This function requires that there is at least one curve in the polycurve

**Parameters:**
- `startPoint` (Tekla.Structures.Geometry3d.Point) — Segment start point

**Returns:** `PolycurveGeometryBuilder` — The builder itself

[Docs](https://developer.tekla.com/topic/en/18/47/78fce10c-2023-b722-d326-028bf12faf56)

#### `public PolycurveGeometryBuilder PrependTangentArc(Point startPoint)`

Prepends an arc tangent to the first curve of the curve list.

**Remarks:** This function requires that there is at least one curve in the polycurve

**Parameters:**
- `startPoint` (Tekla.Structures.Geometry3d.Point) — Start point of the arc

**Returns:** `PolycurveGeometryBuilder` — The builder itself

[Docs](https://developer.tekla.com/topic/en/18/47/accf12eb-0a7e-54e3-c8b0-313bfe92481e)

#### `public PolycurveGeometryBuilder PrependTangentSegment(double length)`

Prepends a line segment of the given length tangent to the first curve of the list.

**Remarks:** This function requires that there is at least one curve in the polycurve

**Parameters:**
- `length` (System.Double) — Length of the segment to append

**Returns:** `PolycurveGeometryBuilder` — The builder itself

[Docs](https://developer.tekla.com/topic/en/18/47/d67e7fa4-7550-0712-9f0b-52a08631d0d5)

#### `public PolycurveGeometryBuilder Replace(int index, Arc arc)`

Replaces the geometry at a given index

**Parameters:**
- `index` (System.Int32) — Index of the geometry to replace
- `arc` (Tekla.Structures.Geometry3d.Arc) — Geometry to insert in place of the existing one

**Returns:** `PolycurveGeometryBuilder` — The builder itself

[Docs](https://developer.tekla.com/topic/en/18/47/e8c5f5f1-3c92-6664-1907-0637abfd65f9)

#### `public PolycurveGeometryBuilder Replace(int index, LineSegment segment)`

Replaces the geometry at a given index

**Parameters:**
- `index` (System.Int32) — Index of the geometry to replace
- `segment` (Tekla.Structures.Geometry3d.LineSegment) — Geometry to insert in place of the existing one

**Returns:** `PolycurveGeometryBuilder` — The builder itself

[Docs](https://developer.tekla.com/topic/en/18/47/901804e0-459b-b57b-6bb5-03b6eb1eb3c7)

## PolymeshEdgeTypeEnum (enum)

The type of the polymesh edge. This should always correspond 1:1 with geometry::IndirectEdge_s::EdgeType_e on the Tekla Structures Core side

[Vendor docs](https://developer.tekla.com/topic/en/18/47/51c45a07-942f-59e1-2f66-5f88f94356ee)

### Values
- `VISIBLE_EDGE` = `1` — Explicitly states that this edge is visible
- `INVISIBLE_EDGE` = `2` — For hiding edges in visualizations

## Projection (class)

The Projection class contains methods for calculating the projection of geometric objects on other geometric objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/47b33428-8930-f48d-da1c-415d61d50d9e)

### Methods
#### `public static LineSegment LineSegmentToPlane(LineSegment LineSegment, GeometricPlane Plane)`

Returns a new line segment which is a projection of the given line segment onto the given plane.

**Parameters:**
- `LineSegment` (Tekla.Structures.Geometry3d.LineSegment) — The line segment to be used.
- `Plane` (Tekla.Structures.Geometry3d.GeometricPlane) — The plane to be used.

**Returns:** `LineSegment` — The new projection line segment.

[Docs](https://developer.tekla.com/topic/en/18/47/d3a1c359-b82a-1e76-5f3f-31076224f4b5)

#### `public static Line LineToPlane(Line Line, GeometricPlane Plane)`

Returns a new line which is a projection of the given line onto the given plane.

**Parameters:**
- `Line` (Tekla.Structures.Geometry3d.Line) — The line to be used.
- `Plane` (Tekla.Structures.Geometry3d.GeometricPlane) — The plane to be used.

**Returns:** `Line` — The new projection line.

[Docs](https://developer.tekla.com/topic/en/18/47/85fc7afb-9440-5632-948e-1f98201f5291)

#### `public static Point PointToLine(Point Point, Line Line)`

Returns a new point which is a projection of the given point onto the given line.

**Parameters:**
- `Point` (Tekla.Structures.Geometry3d.Point) — The point to be used.
- `Line` (Tekla.Structures.Geometry3d.Line) — The line to be used.

**Returns:** `Point` — The new projection point.

[Docs](https://developer.tekla.com/topic/en/18/47/6a36026c-9003-b1f1-fdd5-109453c21397)

#### `public static Point PointToPlane(Point Point, GeometricPlane Plane)`

Returns a new point which is a projection of the given point onto the given plane.

**Parameters:**
- `Point` (Tekla.Structures.Geometry3d.Point) — The point to be used.
- `Plane` (Tekla.Structures.Geometry3d.GeometricPlane) — The plane to be used.

**Returns:** `Point` — The new projection point.

[Docs](https://developer.tekla.com/topic/en/18/47/68645083-3c17-6e0a-9b43-15f056dcfedd)

## Vector (class)

The Vector class defines a direction and magnitude from the current origin.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/b0d6ce02-e6ee-a703-e1ab-ae8dede8728b)

### Constructors
- `public Vector()` — Instantiates a zero length vector.
- `public Vector(Point Point)` — Instatiates a new vector with the given point.
- `public Vector(double X, double Y, double Z)` — Instantiates a vector with the given coordinates.

### Methods
#### `public int CompareTo(Object obj)`

Compares two points. To use binarysearch somekind of sorting should be used.

**Parameters:**
- `obj` (System.Object) — The point to be compared.

**Returns:** `Int32` — 0 if both are equal, -1 if this point is before, 1 if this point is after.

[Docs](https://developer.tekla.com/topic/en/18/47/2f202828-05fd-220f-92b1-5b64372a4f7e)

#### `public Vector Cross(Vector Vector)`

Returns a new cross product vector of the current vector and the given vector.

**Parameters:**
- `Vector` (Tekla.Structures.Geometry3d.Vector) — The vector to be used.

**Returns:** `Vector` — The new cross product vector.

[Docs](https://developer.tekla.com/topic/en/18/47/71f0684a-6601-0fcd-277d-b3c480c84463)

#### `public static Vector Cross(Vector Vector1, Vector Vector2)`

Returns a new cross product vector of the given two vectors.

**Parameters:**
- `Vector1` (Tekla.Structures.Geometry3d.Vector) — The first vector to be used.
- `Vector2` (Tekla.Structures.Geometry3d.Vector) — The second vector to be used.

**Returns:** `Vector` — The new cross product vector.

[Docs](https://developer.tekla.com/topic/en/18/47/5212d3aa-0c71-d610-2f74-a107e4fbe503)

#### `public double Dot(Vector Vector)`

Returns a dot product of the current vector and the given vector.

**Parameters:**
- `Vector` (Tekla.Structures.Geometry3d.Vector) — The vector to be used.

**Returns:** `Double` — The dot product of the vectors.

[Docs](https://developer.tekla.com/topic/en/18/47/5bf3f7da-a1b6-c2b0-1047-eb52545ea5a3)

#### `public static double Dot(Vector Vector1, Vector Vector2)`

Returns a dot product of the given two vectors.

**Parameters:**
- `Vector1` (Tekla.Structures.Geometry3d.Vector) — The first vector to be used.
- `Vector2` (Tekla.Structures.Geometry3d.Vector) — The second vector to be used.

**Returns:** `Double` — The dot product of the vectors.

[Docs](https://developer.tekla.com/topic/en/18/47/c68f1d16-b657-bfec-7779-59fa3745dfef)

#### `public override bool Equals(Object obj)`

Returns true if the current object and the given object are equal.

**Parameters:**
- `obj` (System.Object) — The object we wish to check the equality with.

**Returns:** `Boolean` — True if the given object equals the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/dbca17f2-6730-e2c4-583c-90c74c72e0b0)

#### `public double GetAngleBetween(Vector Vector)`

Gets the angle (in radians) between the current vector and the given vector.

**Parameters:**
- `Vector` (Tekla.Structures.Geometry3d.Vector) — The vector to be used.

**Returns:** `Double` — The angle between the vectors in radians.

[Docs](https://developer.tekla.com/topic/en/18/47/b55375f9-5252-e2e9-2589-5775e1bd3010)

#### `public override int GetHashCode()`

Returns a hash code for the point. Notice, in extremely rare cases, you might not get the same hash code for two points even though they are considered equal! This should, however, happen only in extremely rare cases!

**Returns:** `Int32` — The hash code for the point.

[Docs](https://developer.tekla.com/topic/en/18/47/9f84f024-170b-5181-770d-5ed2f56332cd)

#### `public double GetLength()`

Gets the length (magnitude) of a vector.

**Returns:** `Double` — The vector's length.

[Docs](https://developer.tekla.com/topic/en/18/47/91f7b218-3ef9-3c5e-6123-73dbe4d4b036)

#### `public Vector GetNormal()`

Returns a new normalized equivalent of the current vector.

**Returns:** `Vector` — The normalized equivalent of the current vector.

[Docs](https://developer.tekla.com/topic/en/18/47/63f639b4-79da-d1b8-ec4e-8017837710d6)

#### `public double Normalize(double NewLength)`

Normalizes the vector using the given length.

**Parameters:**
- `NewLength` (System.Double) — The length to be used.

**Returns:** `Double` — The normalized vector's length.

[Docs](https://developer.tekla.com/topic/en/18/47/816a0090-f142-ebd4-31fd-1cf78d4ddd53)

#### `public double Normalize()`

Normalizes the vector using the length 1.0 (the length of a unit vector).

**Returns:** `Double` — The normalized vector's length.

[Docs](https://developer.tekla.com/topic/en/18/47/54721df5-3066-b8b0-a27e-1cad586e287a)

#### `public string ToString(CultureInfo cultureInfo)`

Formats the point into a string, using the provided culture.

**Parameters:**
- `cultureInfo` (System.Globalization.CultureInfo)

**Returns:** `String` — The string that represents the point.

[Docs](https://developer.tekla.com/topic/en/18/47/176a7423-000a-5fb0-dab9-659b87bf1505)

#### `public override string ToString()`

Formats the 3D vector into a string with fixed decimals, in the following way: "(X, Y, Z)".

**Returns:** `String` — The string that represents the vector.

[Docs](https://developer.tekla.com/topic/en/18/47/af9ca1e1-e366-d66a-9c2b-24f2ea504c02)

#### `public virtual void Translate(double X, double Y, double Z)`

Translates the point using the given vector.

**Parameters:**
- `X` (System.Double) — The X-translation to be used.
- `Y` (System.Double) — The Y-translation to be used.
- `Z` (System.Double) — The Z-translation to be used.

[Docs](https://developer.tekla.com/topic/en/18/47/7ad7ecb9-acf3-7ed2-f2bc-aa52b99ceae6)

#### `public void Zero()`

Zeros all the members of the point.

[Docs](https://developer.tekla.com/topic/en/18/47/9bf63e0c-fe71-40ae-e467-306f0a41f315)

