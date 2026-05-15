---
parent: tekla-structures-geometry3d
description: Full API reference for Tekla.Structures.Geometry3d types, Point, Vector, Line, Plane, Arc, CoordinateSystem, Matrix, AABB, intersection, distance utilities.
---

## Core Types

### Point
> Represents a single position in 3D space. The most fundamental type in the Tekla API.

**Constructors:**
- `Point()`
- `Point(double X, double Y, double Z)`
- `Point(double X, double Y)`
- `Point(Point Point)` -- copy constructor

**Fields:**
- `double X` -- the X coordinate
- `double Y` -- the Y coordinate
- `double Z` -- the Z coordinate

**Methods:**
- `int CompareTo(object obj)` -- compares to another point
- `void Translate(double X, double Y, double Z)` -- translates the point in-place
- `void Zero()` -- sets all coordinates to zero

**Static Methods:**
- `static bool AreEqual(Point Point1, Point Point2)` -- checks if two points are equal

---

### Vector : Point
> Defines a direction and magnitude. Inherits X, Y, Z fields from Point.

**Constructors:**
- `Vector()`
- `Vector(double X, double Y, double Z)`
- `Vector(Point Point)` -- creates vector from point coordinates

**Instance Methods:**
- `Vector Cross(Vector Vector)` -- cross product with another vector
- `double Dot(Vector Vector)` -- dot product with another vector
- `double GetAngleBetween(Vector Vector)` -- angle in radians between vectors
- `double GetLength()` -- returns the length (magnitude)
- `Vector GetNormal()` -- returns a new normalized copy (unit vector), does NOT mutate
- `double Normalize()` -- normalizes in-place, returns original length
- `double Normalize(double NewLength)` -- normalizes to specified length in-place

**Static Methods:**
- `static Vector Cross(Vector Vector1, Vector Vector2)` -- cross product of two vectors
- `static double Dot(Vector Vector1, Vector Vector2)` -- dot product of two vectors

---

### Line
> An infinite line in 3D space defined by an origin point and direction vector.

**Constructors:**
- `Line()`
- `Line(Point p1, Point p2)` -- line through two points
- `Line(Point Point, Vector Direction)` -- line from point and direction
- `Line(LineSegment LineSegment)` -- infinite line through a line segment

**Properties:**
- `Vector Direction { get; set; }` -- direction vector of the line
- `Point Origin { get; set; }` -- origin point of the line

---

### LineSegment (implements ICurve)
> A finite segment of a line between two points.

**Constructors:**
- `LineSegment()`
- `LineSegment(Point Point1, Point Point2)`

**Properties:**
- `Point EndPoint { get; }` -- end point (alias for Point2)
- `Point Point1 { get; set; }` -- start point
- `Point Point2 { get; set; }` -- end point
- `Point StartPoint { get; }` -- start point (alias for Point1)

**Methods:**
- `ICurve Clone()` -- creates a copy
- `Vector GetDirectionVector()` -- returns direction from Point1 to Point2
- `double Length()` -- returns the length of the segment

---

### GeometricPlane
> A 3D geometric plane defined by an origin point and a normal vector.

**Constructors:**
- `GeometricPlane()`
- `GeometricPlane(Point Origin, Vector Normal)` -- from origin and normal
- `GeometricPlane(Point Origin, Vector Xaxis, Vector Yaxis)` -- from origin and two axes (normal = cross product)
- `GeometricPlane(CoordinateSystem CoordSys)` -- from coordinate system

**Properties:**
- `Vector Normal { get; set; }` -- normal vector of the plane
- `Point Origin { get; set; }` -- origin point of the plane

**Methods:**
- `Vector GetNormal()` -- returns the normal vector

---

### CoordinateSystem
> A coordinate system defined by an origin, X-axis, and Y-axis. Z-axis = cross(X, Y).

**Constructors:**
- `CoordinateSystem()`
- `CoordinateSystem(Point Origin, Vector AxisX, Vector AxisY)`

**Properties:**
- `Vector AxisX { get; set; }` -- X-axis of the coordinate system
- `Vector AxisY { get; set; }` -- Y-axis of the coordinate system
- `Point Origin { get; set; }` -- origin of the coordinate system

---

### Matrix
> A 4x3 transformation matrix for coordinate transformations.

**Constructors:**
- `Matrix()`
- `Matrix(Matrix m)` -- copy constructor

**Properties:**
- `double Item { get; set; }` -- indexer for matrix elements

**Methods:**
- `Matrix GetTranspose()` -- returns transposed copy
- `Point Transform(Point p)` -- transforms a single point
- `IEnumerable<Point> Transform(IEnumerable<Point> Points)` -- transforms multiple points
- `void Transpose()` -- transposes in-place

---

### MatrixFactory (static)
> Factory methods for generating transformation matrices.

**Static Methods:**
- `static Matrix ByCoordinateSystems(CoordinateSystem CoordSys1, CoordinateSystem CoordSys2)` -- matrix to transform from CoordSys1 to CoordSys2
- `static Matrix FromCoordinateSystem(CoordinateSystem CoordSys)` -- matrix to transform FROM the given coordinate system to global
- `static Matrix Rotate(double Angle, Vector Axis)` -- rotation matrix around an axis (angle in radians)
- `static Matrix ToCoordinateSystem(CoordinateSystem CoordSys)` -- matrix to transform from global TO the given coordinate system

---

### Arc (implements ICurve)
> Represents an arc geometry defined by center, start, end, and normal.

**Constructors:**
- `Arc(Point startPoint, Point endPoint, Point pointOnArc)` -- arc through 3 points
- `Arc(Point centerPoint, Point startPoint, Vector normal, double deltaAngleRadians)` -- from center, start, normal, and sweep angle
- `Arc(Point centerPoint, Vector startDirection, Vector startTangent, double radius, double deltaAngleRadians)` -- from center, directions, radius, and sweep angle

**Properties:**
- `double Angle { get; }` -- angle of the arc in radians
- `Point ArcMiddlePoint { get; }` -- point at the middle of the arc
- `Point CenterPoint { get; }` -- center point
- `Point EndPoint { get; }` -- end point
- `double Length { get; }` -- arc length
- `Vector Normal { get; }` -- normal vector (axis of rotation)
- `double Radius { get; }` -- radius
- `Vector StartDirection { get; }` -- unit vector from center to start point (X-axis of arc CS)
- `Point StartPoint { get; }` -- start point
- `Vector StartTangent { get; }` -- unit vector tangent at start point (Y-axis of arc CS)

**Methods:**
- `ICurve Clone()` -- creates a copy

---

### AABB (implements IBoundingVolume)
> Axis-aligned 3D bounding box. Edges are parallel to the global coordinate axes.

**Constructors:**
- `AABB()`
- `AABB(Point MinPoint, Point MaxPoint)`
- `AABB(IEnumerable<Point> Points)` -- computes bounding box from point cloud
- `AABB(AABB AABB)` -- copy constructor

**Properties:**
- `Point MaxPoint { get; set; }` -- maximum corner point
- `Point MinPoint { get; set; }` -- minimum corner point

**Methods:**
- `bool Collide(AABB Other)` -- tests collision with another AABB
- `Point GetCenterPoint()` -- returns center point
- `Point[] GetCornerPoints()` -- returns all 8 corner points
- `bool IsInside(Point Point)` -- tests if point is inside
- `bool IsInside(LineSegment LineSegment)` -- tests if line segment is inside

---

### OBB (sealed, implements IBoundingVolume)
> Oriented 3D bounding box. Can be rotated unlike AABB. Defined by center, 3 orthogonal axes, and 3 half-extents.

**Constructors:**
- `OBB()`
- `OBB(Point center, Vector axis0, Vector axis1, Vector axis2, double extent0, double extent1, double extent2)`
- `OBB(Point center, Vector[] axis, double[] extent)`
- `OBB(OBB obb)` -- copy constructor

**Properties:**
- `Vector Axis0 { get; }` -- first axis (unit-length, orthogonal to Axis1 and Axis2)
- `Vector Axis1 { get; }` -- second axis (unit-length, orthogonal to Axis0 and Axis2)
- `Vector Axis2 { get; }` -- third axis (unit-length, orthogonal to Axis0 and Axis1)
- `Point Center { get; set; }` -- center point of the box
- `double Extent0 { get; set; }` -- half-extent along Axis0
- `double Extent1 { get; set; }` -- half-extent along Axis1
- `double Extent2 { get; set; }` -- half-extent along Axis2

**Closest Point Methods:**
- `Point ClosestPointTo(Point point)`
- `Point ClosestPointTo(Line line)`
- `Point ClosestPointTo(LineSegment lineSegment)`

**Distance Methods:**
- `double DistanceTo(Point point)`
- `double DistanceTo(Line line)`
- `double DistanceTo(LineSegment lineSegment)`

**Intersection Methods:**
- `Point[] IntersectionPointsWith(LineSegment lineSegment)` -- all intersection points
- `Point[] IntersectionPointsWith(Line line)`
- `LineSegment IntersectionWith(LineSegment lineSegment)` -- intersection segment
- `LineSegment IntersectionWith(Line line)`
- `bool Intersects(Line line)`
- `bool Intersects(LineSegment lineSegment)`
- `bool Intersects(GeometricPlane geometricPlane)`
- `bool Intersects(OBB obb)` -- OBB-OBB intersection test

**Vertex Methods:**
- `Point[] ComputeVertices()` -- returns all 8 corner vertices

**Shortest Segment Methods:**
- `LineSegment ShortestSegmentTo(Point point)`
- `LineSegment ShortestSegmentTo(LineSegment lineSegment)`
- `LineSegment ShortestSegmentTo(Line line)`

**Setters:**
- `void SetAxis(Vector axis0, Vector axis1, Vector axis2)`
- `void SetAxis(Vector[] axis)`
- `void SetExtent(double extent0, double extent1, double extent2)`
- `void SetExtent(double[] extent)`

---

### Polycurve (implements ICurve, IEnumerable\<ICurve\>)
> A composite curve made of line segments and arcs.

**Constructors:**
- `Polycurve()`
- `Polycurve(ICurve geometry)` -- wrap a single curve
- `Polycurve(PolyLine polyLine)` -- convert from polyline

**Properties:**
- `Point EndPoint { get; }` -- end point of the polycurve
- `double Length { get; }` -- total length of all segments
- `Point StartPoint { get; }` -- start point

**Methods:**
- `ICurve Clone()` -- creates a copy
- `IEnumerator<ICurve> GetEnumerator()` -- iterate individual curve segments

---

### PolycurveGeometryBuilder
> Builder for constructing Polycurve objects by appending/prepending segments and arcs. All append/prepend methods return the builder for method chaining.

**Constructors:**
- `PolycurveGeometryBuilder()`
- `PolycurveGeometryBuilder(Polycurve geometries)` -- start from existing polycurve

**Append Methods (return `PolycurveGeometryBuilder`):**
- `Append(LineSegment segment)` -- append a line segment
- `Append(Arc arc)` -- append an arc
- `Append(Polycurve otherPolycurve)` -- append another polycurve
- `AppendArc(Point middlePoint, Point endPoint)` -- append arc through middle and end points
- `AppendSegment(Point endPoint)` -- append line segment to end point
- `AppendTangentArc(Point endPoint)` -- append tangent-continuous arc
- `AppendTangentSegment(double length)` -- append tangent-continuous line segment

**Prepend Methods (return `PolycurveGeometryBuilder`):**
- `Prepend(LineSegment segment)` -- prepend a line segment
- `Prepend(Arc arc)` -- prepend an arc
- `PrependArc(Point startPoint, Point middlePoint)` -- prepend arc through start and middle points
- `PrependSegment(Point startPoint)` -- prepend line segment from start point
- `PrependTangentArc(Point startPoint)` -- prepend tangent-continuous arc
- `PrependTangentSegment(double length)` -- prepend tangent-continuous line segment

**Replace Methods (return `PolycurveGeometryBuilder`):**
- `Replace(int index, LineSegment segment)`
- `Replace(int index, Arc arc)`

**Build Method:**
- `Polycurve GetPolycurve()` -- returns the built Polycurve

---

### PolyLine
> A line consisting of one or more connected line segments defined by a list of points.

**Constructors:**
- `PolyLine(IEnumerable Points)` -- note: uses non-generic IEnumerable (ArrayList of Points)

**Properties:**
- `ArrayList Points { get; set; }` -- the points forming the polyline (ArrayList, cast to Point)

**Methods:**
- `double Length()` -- total length of all segments

---

### ICurve (interface)
> Base interface for 3D curves. Implemented by LineSegment, Arc, and Polycurve.

**Properties:**
- `Point EndPoint { get; }`
- `double Length { get; }`
- `Point StartPoint { get; }`

**Methods:**
- `ICurve Clone()`

---

### IBoundingVolume (interface)
> Marker interface for any generic 3D bounding volume. Implemented by AABB and OBB.

---

## Static Utility Classes

### Distance (static)
> Calculates distances between geometric objects.

- `static double PointToLine(Point Point, Line Line)` -- shortest distance from point to infinite line
- `static double PointToLineSegment(Point Point, LineSegment LineSegment)` -- shortest distance from point to line segment
- `static double PointToPlane(Point Point, GeometricPlane Plane)` -- signed distance from point to plane
- `static double PointToPoint(Point Point1, Point Point2)` -- distance between two points

---

### Intersection (static)
> Calculates intersections between geometric objects. Returns null when no intersection exists.

- `static LineSegment LineSegmentToObb(LineSegment lineSegment, OBB obb)` -- intersection of segment with OBB
- `static Point LineSegmentToPlane(LineSegment lineSegment, GeometricPlane plane)` -- intersection point of segment with plane (null if parallel)
- `static LineSegment LineToLine(Line line1, Line line2)` -- shortest segment between two lines (zero-length if they intersect)
- `static LineSegment LineToObb(Line line, OBB obb)` -- intersection of line with OBB
- `static Point LineToPlane(Line line, GeometricPlane plane)` -- intersection point of line with plane (null if parallel)
- `static Line PlaneToPlane(GeometricPlane plane1, GeometricPlane plane2)` -- intersection line of two planes (null if parallel)

---

### Projection (static)
> Projects geometric objects onto other geometric objects.

- `static LineSegment LineSegmentToPlane(LineSegment LineSegment, GeometricPlane Plane)` -- project segment onto plane
- `static Line LineToPlane(Line Line, GeometricPlane Plane)` -- project line onto plane
- `static Point PointToLine(Point Point, Line Line)` -- project point onto line (closest point on the line)
- `static Point PointToPlane(Point Point, GeometricPlane Plane)` -- project point onto plane

---

### Parallel (static)
> Tests parallelism of geometric objects. All methods have overloads with and without a tolerance parameter.

- `static bool LineSegmentToLineSegment(LineSegment, LineSegment [, double Tolerance])`
- `static bool LineSegmentToPlane(LineSegment, GeometricPlane [, double Tolerance])`
- `static bool LineToLine(Line, Line [, double Tolerance])`
- `static bool LineToPlane(Line, GeometricPlane [, double Tolerance])`
- `static bool PlaneToPlane(GeometricPlane, GeometricPlane [, double Tolerance])`
- `static bool VectorToPlane(Vector, GeometricPlane [, double Tolerance])`
- `static bool VectorToVector(Vector, Vector [, double Tolerance])`

---

## Faceted BREP Types

### FacetedBrep
> Defines a faceted boundary representation solid using indexed vertices and face wires.

**Constructors:**
- `FacetedBrep(Vector[] vertices, int[][] outerWires, IDictionary<int, int[][]> innerWires)`
- `FacetedBrep(Vector[] vertices, int[][] outerWires, IDictionary<int, int[][]> innerWires, IList<IndirectPolymeshEdge> edges)`

**Properties:**
- `ICollection<FacetedBrepFace> Faces { get; }` -- the faces
- `IList<IndirectPolymeshEdge> GetEdges { get; }` -- the edges
- `IDictionary<int, int[][]> InnerWires { get; }` -- inner wires (holes)
- `int[][] OuterWires { get; }` -- outer wires (boundaries)
- `IList<Vector> Vertices { get; }` -- vertex positions

**Methods:**
- `bool CheckForTwoManifold()` -- validates topology
- `int[] GetInnerFace(int faceIndex)` -- inner face vertex indices
- `int GetInnerFaceCount(int faceIndex)` -- number of inner faces for a face
- `int[] GetOuterFace(int faceIndex)` -- outer face vertex indices

---

### FacetedBrepWithNormals : FacetedBrep
> Faceted BREP with per-vertex normal vectors.

**Constructors:**
- `FacetedBrepWithNormals(Vector[] vertices, int[][] outerWires, IDictionary<int, int[][]> innerWires, Vector[] normals)`

**Properties:**
- `Vector[] Normals { get; set; }` -- vertex normal unit vectors

---

### FacetedBrepFace
> A face of a faceted BREP. Managed internally by FacetedBrep (not directly instantiated).

**Properties:**
- `bool HasHoles { get; }` -- whether the face has holes
- `IList<FacetedBrepFaceHole> Holes { get; }` -- list of holes
- `bool IsReadOnly { get; }` -- whether read-only
- `IList<int> VerticeIndexes { get; }` -- vertex indices
- `IList<Vector> Vertices { get; }` -- vertex positions

---

### FacetedBrepFaceHole
> A hole on a faceted BREP face. Managed internally by FacetedBrepFace.

**Properties:**
- `int Count { get; }` -- number of vertices
- `bool IsReadOnly { get; }` -- whether read-only
- `IList<int> VerticeIndexes { get; }` -- vertex indices
- `IList<Vector> Vertices { get; }` -- vertex positions

---

### IndirectPolymeshEdge
> A single global edge in a solid using integer indexes to reference vertices.

**Constructors:** `IndirectPolymeshEdge()`

**Properties:**
- `PolymeshEdgeTypeEnum EdgeType { get; set; }` -- type of edge
- `int EndPoint { get; set; }` -- end point index
- `int ShellIndex { get; set; }` -- index of the shell the edge belongs to
- `int StartPoint { get; set; }` -- start point index

---

### PolymeshEdgeTypeEnum

| Value | Int | Description |
|-------|-----|-------------|
| VISIBLE_EDGE | 1 | Edge is visible |
| INVISIBLE_EDGE | 2 | Edge is hidden in visualizations |

---

### GeometryConstants
> Holds constant values used internally by geometry classes.

**Constructors:** `GeometryConstants()`

---

## Compatibility Conversions (Tekla.Structures.Geometry3d.Compatibility)

These static helper classes convert between the legacy Geometry3d types and the modern `Trimble.Geometry` types. Found in `Tekla.Structures.Geometry3d.Compatibility.dll`.

### ArcConversions (static)
- `static Arc ToArc(Arc3 arc)` -- Arc3 to Geometry3d Arc
- `static Arc3 ToArc3(Arc arc)` -- Geometry3d Arc to Arc3

### BoxConversions (static)
- `static AABB ToAABB(AxisAlignedBox3 axisAlignedBox)` -- AxisAlignedBox3 to AABB
- `static AxisAlignedBox3 ToAxisAlignedBox3(AABB axisAlignedBoundingBox)` -- AABB to AxisAlignedBox3
- `static Box3 ToBox3(OBB boundingBox)` -- OBB to Box3
- `static OBB ToOBB(Box3 box)` -- Box3 to OBB

### CoordinateSystemConversions (static)
- `static CoordinateSystem ToCoordinateSystem(CoordinateSystem3 coordinateSystem)` -- CoordinateSystem3 to Geometry3d CS
- `static CoordinateSystem3 ToCoordinateSystem3(CoordinateSystem coordinateSystem)` -- Geometry3d CS to CoordinateSystem3

### LineConversions (static)
- `static Line ToLine(Line3 line)` -- Line3 to Geometry3d Line
- `static Line3 ToLine3(Line line)` -- Geometry3d Line to Line3
- `static List<Line3> ToLine3List(IEnumerable<Line> lines)` -- batch convert
- `static List<Line> ToLineList(IEnumerable<Line3> lines)` -- batch convert

### PlaneConversions (static)
- `static Vector Normal(Plane plane)` -- get normal from legacy Plane
- `static Face3 ToFace3(Face face)` -- Solid Face to Face3
- `static Face3 ToFace3(ICollection<Point> pointList)` -- point list to Face3
- `static Face3 ToFace3(Polygon polygon)` -- Polygon to Face3
- `static Face3 ToFace3(Polygon polygon, UnitVector3 normal)` -- Polygon to Face3 with normal
- `static IEnumerable<Face3> ToFace3Array(IEnumerable<List<Point>> pointsList)` -- batch convert
- `static GeometricPlane ToGeometricPlane(Plane plane)` -- legacy Plane to GeometricPlane
- `static Plane3 ToPlane3(Plane plane)` -- legacy Plane to Plane3
- `static Plane3 ToPlane3(GeometricPlane geometricPlane)` -- GeometricPlane to Plane3
- `static List<Plane3> ToPlane3List(IEnumerable<GeometricPlane> geometricPlanes)` -- batch convert

### PolycurveConversions (static)
- `static ComplexGeometry ToComplexGeometry(Polycurve polycurve)` -- Polycurve to ComplexGeometry
- `static Polycurve ToPolycurve(IComplexGeometry complexGeometry)` -- ComplexGeometry to Polycurve

### SegmentConversions (static)
- `static LineSegment ToLineSegment(Segment3 segment)` -- Segment3 to LineSegment
- `static List<LineSegment> ToLineSegmentList(IEnumerable<Segment3> segments)` -- batch convert
- `static PolySegment3 ToPolySegment3(PolyLine polyLine)` -- PolyLine to PolySegment3
- `static PolySegment3 ToPolySegment3(Polygon polygon)` -- Polygon to PolySegment3
- `static Segment3 ToSegment3(LineSegment lineSegment)` -- LineSegment to Segment3
- `static List<Segment3> ToSegment3List(IEnumerable<LineSegment> lineSegments)` -- batch convert

### VectorConversions (static)
- `static Point ToPoint(UnitVector3 vector)` -- UnitVector3 to Point
- `static Point ToPoint(Vector3 vector)` -- Vector3 to Point
- `static ArrayList ToPointArrayList(IEnumerable<Vector3> vectors)` -- batch to ArrayList
- `static ArrayList ToPointArrayList(ArrayList vectors)` -- batch to ArrayList
- `static List<Point> ToPointList(ArrayList vectors)` -- batch to List<Point>
- `static List<Point> ToPointList(IEnumerable<Vector3> vectors)` -- batch to List<Point>
- `static UnitVector3 ToUnitVector3(Point point)` -- Point to UnitVector3
- `static Vector ToVector(UnitVector3 vector)` -- UnitVector3 to Vector
- `static Vector ToVector(Offset offset)` -- Offset to Vector
- `static Vector ToVector(Vector3 vector)` -- Vector3 to Vector
- `static Vector3 ToVector3(Offset offset)` -- Offset to Vector3
- `static Vector3 ToVector3(Point point)` -- Point to Vector3
- `static List<Vector3> ToVector3List(IEnumerable<Point> points, Matrix matrix)` -- batch with transform
- `static List<Vector3> ToVector3List(ArrayList points)` -- batch from ArrayList
- `static List<Vector3> ToVector3List(Loop loop)` -- loop vertices to Vector3 list
- `static List<Vector3> ToVector3List(IEnumerable<Point> points)` -- batch convert
- `static bool TryToUnitVector3(Point point, out UnitVector3 unitVector)` -- safe conversion

### SolidEx
> Extended solid with face-based topology queries.

**Constructors:**
- `SolidEx(ISolid solid)` -- from Tekla solid
- `SolidEx(IEnumerable<Face3> faces)` -- from face list

**Properties:**
- `List<Face3> Faces { get; set; }`

**Methods:**
- `void AddEdgeFaces(Face3 face)`
- `void Clean()`
- `Nullable<Segment3> FindEdge(Vector3 point)`
- `Face3 FindFace(Segment3 segment)`
- `bool TryGetAdjacentFace(Face3 face, Segment3 edge, out Face3 adjacentFace)`

### SolidExtensions (static)
> Extension methods for solid geometry queries.

- `static bool ContainsPartOf(Face3 face, Segment3 segment)`
- `static Nullable<Segment3> FindEdgeThatContainsPoint(Solid solid, Vector3 point)`
- `static IEnumerable<Face3> FindFacesThatContainCollinearEdges(Solid solid, IEnumerable<Segment3> edges)`
- `static IEnumerable<Face3> GetFaces(Solid solid)` -- get all faces as Face3
- `static IList<Face3> GetIntersectionFaces(Solid solid, Plane3 plane3)` -- faces cut by plane
- `static bool TryFindEdgeContainingPoint(Face3 face, Vector3 point, out Nullable<Segment3> edge)`
- `static bool TryGetClosestEdgeTo(Solid solid, Vector3 point, out Segment3 closestEdge)`
- `static bool TryGetClosestEdgeTo(IEnumerable<Face3> faces, Vector3 point, out Segment3 closestEdge)`
- `static bool TryGetFace(Solid solid, UnitVector3 normal, Vector3 pointOnFace, out Face3 face)`
- `static bool TryGetFace(IEnumerable<Face3> faces, UnitVector3 normal, Vector3 pointOnFace, out Face3 face)`

### StructuresExtensions (static)
> Utility extensions for copying coordinates and translating point collections.

- `static void CopyCoordinates(Vector target, Vector3 vector)` -- copy Vector3 coords to Vector
- `static void CopyCoordinates(Point target, Vector3 vector)` -- copy Vector3 coords to Point
- `static void Translate(ArrayList contourPoints, Vector3 translation)` -- translate all points in ArrayList

### Vector3Comparer : Comparer\<Vector3\>
> Compares Vector3 values along a specified direction.

**Constructors:** `Vector3Comparer(UnitVector3 direction)`

**Methods:**
- `int Compare(Vector3 x, Vector3 y)` -- compare two vectors along the direction

---

## Common Usage Patterns

### Coordinate system transformation
```csharp
var partCS = new CoordinateSystem(part.GetCoordinateSystem());
var toLocal = MatrixFactory.ToCoordinateSystem(partCS);
var toGlobal = MatrixFactory.FromCoordinateSystem(partCS);
Point localPoint = toLocal.Transform(globalPoint);
Point backToGlobal = toGlobal.Transform(localPoint);
```

### Transform between two coordinate systems
```csharp
var cs1 = new CoordinateSystem(origin1, axisX1, axisY1);
var cs2 = new CoordinateSystem(origin2, axisX2, axisY2);
Matrix transform = MatrixFactory.ByCoordinateSystems(cs1, cs2);
Point transformed = transform.Transform(point);
```

### Rotation around an axis
```csharp
double angleRad = Math.PI / 4; // 45 degrees
var axis = new Vector(0, 0, 1); // rotate around Z
Matrix rotation = MatrixFactory.Rotate(angleRad, axis);
Point rotated = rotation.Transform(originalPoint);
```

### Finding intersection of line with plane
```csharp
var line = new Line(point1, point2);
var plane = new GeometricPlane(planeOrigin, planeNormal);
Point intersection = Intersection.LineToPlane(line, plane);
if (intersection != null) // null means line is parallel to plane
{
    // use intersection point
}
```

### Distance calculations
```csharp
double d = Distance.PointToPoint(p1, p2);
double dToLine = Distance.PointToLine(point, line);
double dToPlane = Distance.PointToPlane(point, plane); // signed distance
```

### Projecting a point onto a plane
```csharp
Point projected = Projection.PointToPlane(point, plane);
Point closestOnLine = Projection.PointToLine(point, line);
```

### Parallelism check
```csharp
bool isParallel = Parallel.VectorToVector(v1, v2);
bool isParallelWithTolerance = Parallel.VectorToVector(v1, v2, 0.001);
bool lineParallelToPlane = Parallel.LineToPlane(line, plane);
```

### Vector operations
```csharp
var v1 = new Vector(1, 0, 0);
var v2 = new Vector(0, 1, 0);
Vector cross = v1.Cross(v2);         // (0, 0, 1)
double dot = v1.Dot(v2);             // 0.0
double angle = v1.GetAngleBetween(v2); // PI/2 radians
Vector unit = v1.GetNormal();         // non-mutating normalized copy
```

### Building a polycurve with segments and arcs
```csharp
var builder = new PolycurveGeometryBuilder();
builder.AppendSegment(new Point(0, 0, 0));
builder.AppendSegment(new Point(1000, 0, 0));
builder.AppendArc(new Point(1250, 250, 0), new Point(1000, 500, 0));
builder.AppendSegment(new Point(0, 500, 0));
Polycurve result = builder.GetPolycurve();
```

### Bounding box operations
```csharp
// Axis-aligned bounding box from points
var points = new List<Point> { p1, p2, p3, p4 };
var aabb = new AABB(points);
bool inside = aabb.IsInside(testPoint);
Point center = aabb.GetCenterPoint();

// Oriented bounding box
var obb = new OBB(center, axis0, axis1, axis2, extent0, extent1, extent2);
bool intersects = obb.Intersects(otherObb);
double dist = obb.DistanceTo(point);
Point closest = obb.ClosestPointTo(line);
```

### Plane-plane intersection
```csharp
var plane1 = new GeometricPlane(origin1, normal1);
var plane2 = new GeometricPlane(origin2, normal2);
Line intersectionLine = Intersection.PlaneToPlane(plane1, plane2);
// null if planes are parallel
```
