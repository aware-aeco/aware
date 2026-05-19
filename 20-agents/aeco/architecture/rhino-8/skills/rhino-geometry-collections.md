---
name: rhino-rhino-geometry-collections
description: This skill encodes the rhino 8.0 surface of the Rhino.Geometry.Collections namespace — 24 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BrepCurveList, BrepEdgeList, BrepFaceList, BrepLoopList, BrepSurfaceList, BrepVertexList, BrepTrimList, MeshFaceList, and 16 more types.
---

# Rhino.Geometry.Collections

Auto-generated from vendor docs for rhino 8.0. 24 types in this namespace.

## BrepCurveList (class)

Provides access to all the underlying curves in a Brep object.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_BrepCurveList.htm)

### Methods
#### `public int Add(Curve curve)`

Adds a curve

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — A copy of the curve is added to this brep

**Returns:** `Int32` — Index that should be used to reference the geometry. -1 is returned if the input is not acceptable.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepCurveList_Add.htm)

#### `public IEnumerator<Curve> GetEnumerator()`

Get an enumerator that visits all curves.

**Returns:** `IEnumerator<Curve>` — The enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepCurveList_GetEnumerator.htm)

### Properties
- `Count` (Int32, get) — Gets the number of curves in this list.
- `Item` (Curve, get) — Gets the Curve at the given index. The index must be valid or an IndexOutOfRangeException will be thrown.

## BrepEdgeList (class)

Provides access to all the Edges in a Brep object.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_BrepEdgeList.htm)

### Methods
#### `public BrepEdge Add(BrepVertex startVertex, BrepVertex endVertex, int curve3dIndex, double edgeTolerance)`

Create and add a new edge to this list

**Parameters:**
- `startVertex` (Rhino.Geometry.BrepVertex) — [Missing <param name="startVertex"/> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(Rhino.Geometry.BrepVertex,Rhino.Geometry.BrepVertex,System.Int32,System.Double)"]
- `endVertex` (Rhino.Geometry.BrepVertex) — [Missing <param name="endVertex"/> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(Rhino.Geometry.BrepVertex,Rhino.Geometry.BrepVertex,System.Int32,System.Double)"]
- `curve3dIndex` (System.Int32) — [Missing <param name="curve3dIndex"/> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(Rhino.Geometry.BrepVertex,Rhino.Geometry.BrepVertex,System.Int32,System.Double)"]
- `edgeTolerance` (System.Double) — [Missing <param name="edgeTolerance"/> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(Rhino.Geometry.BrepVertex,Rhino.Geometry.BrepVertex,System.Int32,System.Double)"]

**Returns:** `BrepEdge` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(Rhino.Geometry.BrepVertex,Rhino.Geometry.BrepVertex,System.Int32,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepEdgeList_Add_1.htm)

#### `public BrepEdge Add(BrepVertex startVertex, BrepVertex endVertex, int curve3dIndex, Interval subDomain, double edgeTolerance)`

Create and add a new edge to this list

**Parameters:**
- `startVertex` (Rhino.Geometry.BrepVertex) — [Missing <param name="startVertex"/> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(Rhino.Geometry.BrepVertex,Rhino.Geometry.BrepVertex,System.Int32,Rhino.Geometry.Interval,System.Double)"]
- `endVertex` (Rhino.Geometry.BrepVertex) — [Missing <param name="endVertex"/> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(Rhino.Geometry.BrepVertex,Rhino.Geometry.BrepVertex,System.Int32,Rhino.Geometry.Interval,System.Double)"]
- `curve3dIndex` (System.Int32) — [Missing <param name="curve3dIndex"/> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(Rhino.Geometry.BrepVertex,Rhino.Geometry.BrepVertex,System.Int32,Rhino.Geometry.Interval,System.Double)"]
- `subDomain` (Rhino.Geometry.Interval) — [Missing <param name="subDomain"/> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(Rhino.Geometry.BrepVertex,Rhino.Geometry.BrepVertex,System.Int32,Rhino.Geometry.Interval,System.Double)"]
- `edgeTolerance` (System.Double) — [Missing <param name="edgeTolerance"/> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(Rhino.Geometry.BrepVertex,Rhino.Geometry.BrepVertex,System.Int32,Rhino.Geometry.Interval,System.Double)"]

**Returns:** `BrepEdge` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(Rhino.Geometry.BrepVertex,Rhino.Geometry.BrepVertex,System.Int32,Rhino.Geometry.Interval,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepEdgeList_Add.htm)

#### `public BrepEdge Add(int curve3dIndex)`

Create and add a new edge to this list

**Parameters:**
- `curve3dIndex` (System.Int32) — [Missing <param name="curve3dIndex"/> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(System.Int32)"]

**Returns:** `BrepEdge` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepEdgeList_Add_2.htm)

#### `public BrepEdge Add(int startVertexIndex, int endVertexIndex, int curve3dIndex, double edgeTolerance)`

Create and add a new edge to this list

**Parameters:**
- `startVertexIndex` (System.Int32) — [Missing <param name="startVertexIndex"/> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(System.Int32,System.Int32,System.Int32,System.Double)"]
- `endVertexIndex` (System.Int32) — [Missing <param name="endVertexIndex"/> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(System.Int32,System.Int32,System.Int32,System.Double)"]
- `curve3dIndex` (System.Int32) — [Missing <param name="curve3dIndex"/> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(System.Int32,System.Int32,System.Int32,System.Double)"]
- `edgeTolerance` (System.Double) — [Missing <param name="edgeTolerance"/> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(System.Int32,System.Int32,System.Int32,System.Double)"]

**Returns:** `BrepEdge` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(System.Int32,System.Int32,System.Int32,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepEdgeList_Add_4.htm)

#### `public BrepEdge Add(int startVertexIndex, int endVertexIndex, int curve3dIndex, Interval subDomain, double edgeTolerance)`

Create and add a new edge to this list

**Parameters:**
- `startVertexIndex` (System.Int32) — [Missing <param name="startVertexIndex"/> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(System.Int32,System.Int32,System.Int32,Rhino.Geometry.Interval,System.Double)"]
- `endVertexIndex` (System.Int32) — [Missing <param name="endVertexIndex"/> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(System.Int32,System.Int32,System.Int32,Rhino.Geometry.Interval,System.Double)"]
- `curve3dIndex` (System.Int32) — [Missing <param name="curve3dIndex"/> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(System.Int32,System.Int32,System.Int32,Rhino.Geometry.Interval,System.Double)"]
- `subDomain` (Rhino.Geometry.Interval) — [Missing <param name="subDomain"/> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(System.Int32,System.Int32,System.Int32,Rhino.Geometry.Interval,System.Double)"]
- `edgeTolerance` (System.Double) — [Missing <param name="edgeTolerance"/> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(System.Int32,System.Int32,System.Int32,Rhino.Geometry.Interval,System.Double)"]

**Returns:** `BrepEdge` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.BrepEdgeList.Add(System.Int32,System.Int32,System.Int32,Rhino.Geometry.Interval,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepEdgeList_Add_3.htm)

#### `public IEnumerator<BrepEdge> GetEnumerator()`

Gets an enumerator that visits all edges.

**Returns:** `IEnumerator<BrepEdge>` — The enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepEdgeList_GetEnumerator.htm)

#### `public int MergeAllEdges(double angleTolerance)`

Merges all possible Brep edges. Edges across kinks cannot be merged. A pair of adjacent edges in a Brep are mergeable if the angle between them is less than tolerance and the valence of the shared vertex is 2.

**Parameters:**
- `angleTolerance` (System.Double) — The maximum allowable difference of angle in radian between adjacent edges that can be merged.

**Returns:** `Int32` — The number of edges merged.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepEdgeList_MergeAllEdges.htm)

#### `public int MergeEdge(int edgeIndex, double angleTolerance)`

Merge adjacent edges to a specified edge recursively. A pair of adjacent edges in a Brep are mergeable if the angle between them is less than tolerance and the valence of the shared vertex is 2.

**Parameters:**
- `edgeIndex` (System.Int32) — >Index of edge to merge.
- `angleTolerance` (System.Double) — The maximum allowable difference of angle in radian between adjacent edges that can be merged.

**Returns:** `Int32` — The number of edges merged.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepEdgeList_MergeEdge.htm)

#### `public int RemoveNakedMicroEdges(double tolerance)`

Finds any naked edges with the same start and end vertex and an arc-length less than tolerance and attempts to remove them by removing trims and extending the adjacent to meet.

**Parameters:**
- `tolerance` (System.Double) — The tolerance. When in doubt, use the document's model absolute tolerance.

**Returns:** `Int32` — The number of naked micro edges that were removed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepEdgeList_RemoveNakedMicroEdges.htm)

#### `public int RemoveNakedMicroEdges(double tolerance, bool cleanUp)`

Finds any naked edges with the same start and end vertex and an arc-length less than tolerance and attempts to remove them by removing trims and extending the adjacent to meet.

**Parameters:**
- `tolerance` (System.Double) — The tolerance. When in doubt, use the document's model absolute tolerance.
- `cleanUp` (System.Boolean) — If true, then the method cleans up the Brep by setting tolerances, boxes, flags, and then compacts. If false, then the caller should do this at some point.

**Returns:** `Int32` — The number of naked micro edges that were removed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepEdgeList_RemoveNakedMicroEdges_1.htm)

#### `public int SplitEdgeAtParameters(int edgeIndex, IEnumerable<double> edgeParameters)`

Splits an edge at the specified parameters.

**Remarks:** This function leaves deleted stuff in the brep. Call Brep.Compact() to remove deleted stuff.

**Parameters:**
- `edgeIndex` (System.Int32) — The index of the edge to be addressed.
- `edgeParameters` (System.Collections.Generic.IEnumerable<Double>) — The parameter along that edge.

**Returns:** `Int32` — Number of splits applied to the edge.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepEdgeList_SplitEdgeAtParameters.htm)

#### `public bool SplitKinkyEdge(int edgeIndex, double kinkToleranceRadians)`

Splits the edge into G1 pieces.

**Remarks:** This function leaves deleted stuff in the brep. Call Brep.Compact() to remove deleted stuff.

**Parameters:**
- `edgeIndex` (System.Int32) — Index of edge to test and split.
- `kinkToleranceRadians` (System.Double) — The split tolerance in radians.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepEdgeList_SplitKinkyEdge.htm)

### Properties
- `Count` (Int32, get) — Gets the number of brep edges.
- `Item` (BrepEdge, get) — Gets the BrepEdge at the given index. The index must be valid or an IndexOutOfRangeException will be thrown.

## BrepFaceList (class)

Provides access to all the Faces in a Brep object.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_BrepFaceList.htm)

### Methods
#### `public BrepFace Add(int surfaceIndex)`

Create and add a new face to this list. An incomplete face is added. The caller must create and fill in the loops used by the face.

**Parameters:**
- `surfaceIndex` (System.Int32) — index of surface in brep's Surfaces list

**Returns:** `BrepFace` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.BrepFaceList.Add(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepFaceList_Add_1.htm)

#### `public BrepFace Add(Surface surface)`

Add a new face to a brep. This creates a complete face with new vertices at the surface corners, new edges along the surface boundary, etc. The loop of the returned face has four trims that correspond to the south, east, north, and west side of the surface in that order. If you use this version of Add to add an exiting brep, then you are responsible for using a tool like JoinEdges() to hook the new face to its neighbors.

**Parameters:**
- `surface` (Rhino.Geometry.Surface) — surface is copied

**Returns:** `BrepFace` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.BrepFaceList.Add(Rhino.Geometry.Surface)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepFaceList_Add.htm)

#### `public BrepFace AddConeFace(BrepVertex vertex, BrepEdge edge, bool revEdge)`

Add a new face to the brep whose surface geometry is a ruled cone with the edge as the base and the vertex as the apex point.

**Parameters:**
- `vertex` (Rhino.Geometry.BrepVertex) — The apex of the cone will be at this vertex. The north side of the surface's parameter space will be a singular point at the vertex.
- `edge` (Rhino.Geometry.BrepEdge) — The south side of the face's surface will run along this edge.
- `revEdge` (System.Boolean) — true if the new face's outer boundary orientation along the edge is opposite the orientation of edge.

**Returns:** `BrepFace` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.BrepFaceList.AddConeFace(Rhino.Geometry.BrepVertex,Rhino.Geometry.BrepEdge,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepFaceList_AddConeFace.htm)

#### `public BrepFace AddRuledFace(BrepEdge edgeA, bool revEdgeA, BrepEdge edgeB, bool revEdgeB)`

Add a new face to the brep whose surface geometry is a ruled surface between two edges.

**Parameters:**
- `edgeA` (Rhino.Geometry.BrepEdge) — The south side of the face's surface will run along edgeA.
- `revEdgeA` (System.Boolean) — true if the new face's outer boundary orientation along edgeA is opposite the orientation of edgeA.
- `edgeB` (Rhino.Geometry.BrepEdge) — The north side of the face's surface will run along edgeA
- `revEdgeB` (System.Boolean) — true if the new face's outer boundary orientation along edgeB is opposite the orientation of edgeB

**Returns:** `BrepFace` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.BrepFaceList.AddRuledFace(Rhino.Geometry.BrepEdge,System.Boolean,Rhino.Geometry.BrepEdge,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepFaceList_AddRuledFace.htm)

#### `public uint ClearPerFaceColors()`

Removes all per face color overrides on the active level.

**Remarks:** Per face colors are a mutable property on BrepFace and are set with PerFaceColor.

**Returns:** `UInt32` — Number of changed faces.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepFaceList_ClearPerFaceColors.htm)

#### `public Brep ExtractFace(int faceIndex)`

Extracts a face from a Brep.

**Parameters:**
- `faceIndex` (System.Int32) — A face index

**Returns:** `Brep` — A brep. This can be null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepFaceList_ExtractFace.htm)

#### `public void Flip(bool onlyReversedFaces)`

Flips the orientation of faces.

**Parameters:**
- `onlyReversedFaces` (System.Boolean) — If true, clears all BrepFace.OrientationIsReversed flags by calling BrepFace.Transpose() on each face with a true OrientationIsReversed setting. If false, all of the faces are flipped regardless of their orientation.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepFaceList_Flip.htm)

#### `public IEnumerator<BrepFace> GetEnumerator()`

Gets an enumerators that yields BrepFace objects.

**Returns:** `IEnumerator<BrepFace>` — The enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepFaceList_GetEnumerator.htm)

#### `public void RemoveAt(int faceIndex)`

Deletes a face at a specified index.

**Parameters:**
- `faceIndex` (System.Int32) — The index of the mesh face.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepFaceList_RemoveAt.htm)

#### `public bool RemoveSlits()`

Remove slit trims and slit boundaries from each face.

**Returns:** `Boolean` — true if any slits were removed

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepFaceList_RemoveSlits.htm)

#### `public bool ShrinkFaces()`

Shrinks all the underlying surfaces in this Brep. Sometimes the surfaces extend far beyond the trimming boundaries of the Brep Face. This function will remove those portions of the surfaces that are not used.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepFaceList_ShrinkFaces.htm)

#### `public bool SplitBipolarFaces()`

Splits surfaces with two singularities, like spheres, so the results have at most one singularity.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepFaceList_SplitBipolarFaces.htm)

#### `public bool SplitClosedFaces(int minimumDegree)`

Splits closed surfaces so they are not closed.

**Parameters:**
- `minimumDegree` (System.Int32) — If the degree of the surface < min_degree, the surface is not split. In some cases, minimumDegree = 2 is useful to preserve piecewise linear surfaces.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepFaceList_SplitClosedFaces.htm)

#### `public bool SplitFaceAtTangents(int faceIndex)`

Splits the face of a Brep at tangent locations.

**Parameters:**
- `faceIndex` (System.Int32) — The index of the face to split.

**Returns:** `Boolean` — True if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepFaceList_SplitFaceAtTangents.htm)

#### `public bool SplitFacesAtTangents()`

Splits all of the faces of a Brep at tangent locations.

**Returns:** `Boolean` — True if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepFaceList_SplitFacesAtTangents.htm)

#### `public bool SplitKinkyFace(int faceIndex, double kinkTolerance)`

Splits a single face into G1 pieces.

**Remarks:** This function leaves deleted stuff in the brep. Call Brep.Compact() to remove deleted stuff.

**Parameters:**
- `faceIndex` (System.Int32) — The index of the face to split.
- `kinkTolerance` (System.Double) — Tolerance (in radians) to use for crease detection.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepFaceList_SplitKinkyFace.htm)

#### `public bool SplitKinkyFaces()`

Splits any faces with creases into G1 pieces.

**Remarks:** If you need to detect whether splitting occurred, compare the before and after values of Faces.Count

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepFaceList_SplitKinkyFaces.htm)

#### `public bool SplitKinkyFaces(double kinkTolerance)`

Splits any faces with creases into G1 pieces.

**Remarks:** If you need to detect whether splitting occurred, compare the before and after values of Faces.Count

**Parameters:**
- `kinkTolerance` (System.Double) — Tolerance (in radians) to use for crease detection.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepFaceList_SplitKinkyFaces_1.htm)

#### `public bool SplitKinkyFaces(double kinkTolerance, bool compact)`

Splits any faces with creases into G1 pieces.

**Remarks:** If you need to detect whether splitting occurred, compare the before and after values of Faces.Count

**Parameters:**
- `kinkTolerance` (System.Double) — Tolerance (in radians) to use for crease detection.
- `compact` (System.Boolean) — If true, the Brep will be compacted if possible.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepFaceList_SplitKinkyFaces_2.htm)

#### `public bool StandardizeFaceSurface(int faceIndex)`

Standardizes the relationship between a BrepFace and the 3d surface it uses. When done, the face will be the only face that references its 3d surface, and the orientations of the face and 3d surface will be the same.

**Parameters:**
- `faceIndex` (System.Int32) — The index of the face.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepFaceList_StandardizeFaceSurface.htm)

#### `public void StandardizeFaceSurfaces()`

Standardize all faces in the brep.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepFaceList_StandardizeFaceSurfaces.htm)

### Properties
- `Count` (Int32, get) — Gets the number of brep faces.
- `HasPerFaceColors` (Boolean, get) — True if one or more faces on the active level have per face color overrides.
- `Item` (BrepFace, get) — Gets the BrepFace at the given index. The index must be valid or an IndexOutOfRangeException will be thrown.

## BrepLoopList (class)

Provides access to all the Loops in a Brep object.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_BrepLoopList.htm)

### Methods
#### `public BrepLoop Add(BrepLoopType loopType)`

Create a new empty boundary loop. The new loop will not be part of a face and will not include any trim curves.

**Parameters:**
- `loopType` (Rhino.Geometry.BrepLoopType) — [Missing <param name="loopType"/> documentation for "M:Rhino.Geometry.Collections.BrepLoopList.Add(Rhino.Geometry.BrepLoopType)"]

**Returns:** `BrepLoop` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.BrepLoopList.Add(Rhino.Geometry.BrepLoopType)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepLoopList_Add.htm)

#### `public BrepLoop Add(BrepLoopType loopType, BrepFace face)`

Create a new boundary loop on a face. After you get this BrepLoop, you still need to create the vertices, edges, and trims that define the loop.

**Parameters:**
- `loopType` (Rhino.Geometry.BrepLoopType) — [Missing <param name="loopType"/> documentation for "M:Rhino.Geometry.Collections.BrepLoopList.Add(Rhino.Geometry.BrepLoopType,Rhino.Geometry.BrepFace)"]
- `face` (Rhino.Geometry.BrepFace) — [Missing <param name="face"/> documentation for "M:Rhino.Geometry.Collections.BrepLoopList.Add(Rhino.Geometry.BrepLoopType,Rhino.Geometry.BrepFace)"]

**Returns:** `BrepLoop` — New loop that needs to be filled in

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepLoopList_Add_1.htm)

#### `public BrepLoop AddOuterLoop(int faceIndex)`

Create a new outer boundary loop that runs along the sides of the face's surface. All the necessary trims, edges, and vertices are created and added to the brep.

**Parameters:**
- `faceIndex` (System.Int32) — index of face that needs an outer boundary that runs along the sides of its surface.

**Returns:** `BrepLoop` — New outer boundary loop that is complete.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepLoopList_AddOuterLoop.htm)

#### `public BrepLoop AddPlanarFaceLoop(int faceIndex, BrepLoopType loopType, IEnumerable<Curve> boundaryCurves)`

Add a planar trimming loop to a planar face

**Parameters:**
- `faceIndex` (System.Int32) — index of planar face. The underlying surface must be a PlaneSurface
- `loopType` (Rhino.Geometry.BrepLoopType) — type of loop to add. If loopType is Unknown, then the loop direction is tested and the new loops type will be set to Outer or Inner. If the loopType is Outer, then the direction of the new loop is tested and flipped if it is clockwise. If the loopType is Inner, then the direction of the new loop is tested and flipped if it is counter-clockwise.
- `boundaryCurves` (System.Collections.Generic.IEnumerable<Curve>) — list of 3d curves that form a simple (no self intersections) closed curve. These curves define the 3d edge geometry and should be near the planar surface.

**Returns:** `BrepLoop` — new loop if successful

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepLoopList_AddPlanarFaceLoop.htm)

#### `public IEnumerator<BrepLoop> GetEnumerator()`

Gets an enumerator that visits all edges.

**Returns:** `IEnumerator<BrepLoop>` — The enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepLoopList_GetEnumerator.htm)

### Properties
- `Count` (Int32, get) — Gets the number of brep loops.
- `Item` (BrepLoop, get) — Gets the BrepLoop at the given index. The index must be valid or an IndexOutOfRangeException will be thrown.

## BrepSurfaceList (class)

Provides access to all the underlying surfaces in a Brep object.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_BrepSurfaceList.htm)

### Methods
#### `public IEnumerator<Surface> GetEnumerator()`

Gets an enumerator that visits all surfaces.

**Returns:** `IEnumerator<Surface>` — The enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepSurfaceList_GetEnumerator.htm)

### Properties
- `Count` (Int32, get) — Gets the number of surfaces in a brep.
- `Item` (Surface, get) — Gets the Surface at the given index. The index must be valid or an IndexOutOfRangeException will be thrown.

## BrepTrimList (class)

Provides access to all the Trims in a Brep object

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_BrepTrimList.htm)

### Methods
#### `public BrepTrim Add(bool rev3d, BrepEdge edge, int curve2dIndex)`

Add a new trim that will be part of an inner, outer, or slit loop to the brep

**Parameters:**
- `rev3d` (System.Boolean) — true if the edge and trim have opposite directions
- `edge` (Rhino.Geometry.BrepEdge) — 3d edge associated with this trim
- `curve2dIndex` (System.Int32) — index of 2d trimming curve

**Returns:** `BrepTrim` — new trim

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepTrimList_Add_1.htm)

#### `public BrepTrim Add(bool rev3d, BrepLoop loop, int curve2dIndex)`

Add a new trim that will be part of an inner, outer, or slit loop to the brep

**Parameters:**
- `rev3d` (System.Boolean) — true if the edge and trim have opposite directions
- `loop` (Rhino.Geometry.BrepLoop) — trim is appended to this loop
- `curve2dIndex` (System.Int32) — index of 2d trimming curve

**Returns:** `BrepTrim` — new trim

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepTrimList_Add_2.htm)

#### `public BrepTrim Add(BrepEdge edge, bool rev3d, BrepLoop loop, int curve2dIndex)`

Add a new trim that will be part of an inner, outer, or slit loop to the brep.

**Parameters:**
- `edge` (Rhino.Geometry.BrepEdge) — 3d edge associated with this trim
- `rev3d` (System.Boolean) — true if the edge and trim have opposite directions
- `loop` (Rhino.Geometry.BrepLoop) — trim is appended to this loop
- `curve2dIndex` (System.Int32) — index of 2d trimming curve

**Returns:** `BrepTrim` — new trim

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepTrimList_Add.htm)

#### `public BrepTrim Add(int curve2dIndex)`

Add a new trim that will be part of an inner, outer, or slit loop to the brep.

**Remarks:** You should set the trim's Tolerance, TrimType, and IsoStatus values. In general, you should try to use the Add( edge, bRev3d, loop, c2i ) version of NewTrim. If you want to add a singular trim, use AddSingularTrim. If you want to add a curve-on-surface trim, use AddCurveOnFace. If you want to add a point-on-surface trim, use AddPointOnFace.

**Parameters:**
- `curve2dIndex` (System.Int32) — index of 2d trimming curve

**Returns:** `BrepTrim` — New Trim

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepTrimList_Add_3.htm)

#### `public BrepTrim AddCurveOnFace(BrepFace face, BrepEdge edge, bool rev3d, int curve2dIndex)`

Add a new curve on face to the brep

**Remarks:** You should set the trim's tolerance and iso values.

**Parameters:**
- `face` (Rhino.Geometry.BrepFace) — face that curve lies on
- `edge` (Rhino.Geometry.BrepEdge) — 3d edge associated with this curve on surface
- `rev3d` (System.Boolean) — true if the 3d edge and the 2d parameter space curve have opposite directions.
- `curve2dIndex` (System.Int32) — index of 2d curve in face's parameter space

**Returns:** `BrepTrim` — new trim that represents the curve on surface

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepTrimList_AddCurveOnFace.htm)

#### `public BrepTrim AddSingularTrim(BrepVertex vertex, BrepLoop loop, IsoStatus iso, int curve2dIndex)`

Add a new singular trim to the brep.

**Parameters:**
- `vertex` (Rhino.Geometry.BrepVertex) — vertex along collapsed surface edge
- `loop` (Rhino.Geometry.BrepLoop) — trim is appended to this loop
- `iso` (Rhino.Geometry.IsoStatus) — [Missing <param name="iso"/> documentation for "M:Rhino.Geometry.Collections.BrepTrimList.AddSingularTrim(Rhino.Geometry.BrepVertex,Rhino.Geometry.BrepLoop,Rhino.Geometry.IsoStatus,System.Int32)"]
- `curve2dIndex` (System.Int32) — index of 2d trimming curve

**Returns:** `BrepTrim` — new trim

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepTrimList_AddSingularTrim.htm)

#### `public IEnumerator<BrepTrim> GetEnumerator()`

Gets an enumerator that visits all edges.

**Returns:** `IEnumerator<BrepTrim>` — The enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepTrimList_GetEnumerator.htm)

#### `public bool MatchEnds()`

Matches the endpoints of all trims in the Brep.

**Returns:** `Boolean` — true if any trim's 2d curve is changed, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepTrimList_MatchEnds.htm)

#### `public bool MatchEnds(BrepLoop loop)`

Match the endpoints of all trims in a loop.

**Parameters:**
- `loop` (Rhino.Geometry.BrepLoop) — The Brep loop.

**Returns:** `Boolean` — true if any trim's 2d curve is changed, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepTrimList_MatchEnds_1.htm)

#### `public bool MatchEnds(BrepTrim trim0, BrepTrim trim1)`

Match the end of a trim to the start of the next trim.

**Parameters:**
- `trim0` (Rhino.Geometry.BrepTrim) — The Brep trim.
- `trim1` (Rhino.Geometry.BrepTrim) — Brep trim that comes immediately after trim0 in the same loop.

**Returns:** `Boolean` — true if either trim's 2d curve is changed, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepTrimList_MatchEnds_2.htm)

#### `public bool MatchEnds(int trimIndex)`

Match the endpoints of a trim to the next and previous trim.

**Parameters:**
- `trimIndex` (System.Int32) — [Missing <param name="trimIndex"/> documentation for "M:Rhino.Geometry.Collections.BrepTrimList.MatchEnds(System.Int32)"]

**Returns:** `Boolean` — true if any trim's 2d curve is changed, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepTrimList_MatchEnds_3.htm)

### Properties
- `Count` (Int32, get) — Gets the number of brep trims.
- `Item` (BrepTrim, get) — Gets the BrepTrim at the given index. The index must be valid or an IndexOutOfRangeException will be thrown.

## BrepVertexList (class)

Provides access to all the Vertices in a Brep object

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_BrepVertexList.htm)

### Methods
#### `public BrepVertex Add()`

Create and add a new vertex to this list

**Returns:** `BrepVertex` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.BrepVertexList.Add"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepVertexList_Add.htm)

#### `public BrepVertex Add(Point3d point, double vertexTolerance)`

Create and add a new vertex to this list

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — [Missing <param name="point"/> documentation for "M:Rhino.Geometry.Collections.BrepVertexList.Add(Rhino.Geometry.Point3d,System.Double)"]
- `vertexTolerance` (System.Double) — Use RhinoMath.UnsetTolerance if you are unsure

**Returns:** `BrepVertex` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.BrepVertexList.Add(Rhino.Geometry.Point3d,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepVertexList_Add_1.htm)

#### `public BrepVertex AddPointOnFace(BrepFace face, double s, double t)`

Adds a new point on face to the brep

**Remarks:** If a vertex is a point on a face, then brep.Edges[edge_index] will be an edge with no 3d curve. This edge will have a single trim with type BrepTrimType.CurveOnSurface. There will be a loop containing this single trim.

**Parameters:**
- `face` (Rhino.Geometry.BrepFace) — face that vertex lies on
- `s` (System.Double) — surface parameters
- `t` (System.Double) — surface parameters

**Returns:** `BrepVertex` — new vertex that represents the point on face

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepVertexList_AddPointOnFace.htm)

#### `public IEnumerator<BrepVertex> GetEnumerator()`

Gets an enumerator that visits all surfaces.

**Returns:** `IEnumerator<BrepVertex>` — The enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_BrepVertexList_GetEnumerator.htm)

### Properties
- `Count` (Int32, get) — Gets the number of brep vertices.
- `Item` (BrepVertex, get) — Gets the BrepVertex at the given index. The index must be valid or an IndexOutOfRangeException will be thrown.

## MeshFaceList (class)

Provides access to the faces and Face related functionality of a Mesh.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_MeshFaceList.htm)

### Methods
#### `public int AddFace(int vertex1, int vertex2, int vertex3)`

Appends a new triangular face to the end of the mesh face list.

**Parameters:**
- `vertex1` (System.Int32) — Index of first face corner.
- `vertex2` (System.Int32) — Index of second face corner.
- `vertex3` (System.Int32) — Index of third face corner.

**Returns:** `Int32` — The index of the newly added triangle.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_AddFace_1.htm)

#### `public int AddFace(int vertex1, int vertex2, int vertex3, int vertex4)`

Appends a new quadrangular face to the end of the mesh face list.

**Parameters:**
- `vertex1` (System.Int32) — Index of first face corner.
- `vertex2` (System.Int32) — Index of second face corner.
- `vertex3` (System.Int32) — Index of third face corner.
- `vertex4` (System.Int32) — Index of fourth face corner.

**Returns:** `Int32` — The index of the newly added quad.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_AddFace_2.htm)

#### `public int AddFace(MeshFace face)`

Appends a new mesh face to the end of the mesh face list.

**Parameters:**
- `face` (Rhino.Geometry.MeshFace) — Face to add.

**Returns:** `Int32` — The index of the newly added face.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_AddFace.htm)

#### `public int[] AddFaces(IEnumerable<MeshFace> faces)`

Appends a list of faces to the end of the mesh face list.

**Parameters:**
- `faces` (System.Collections.Generic.IEnumerable<MeshFace>) — Faces to add.

**Returns:** `Int32[]` — Indices of the newly created faces

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_AddFaces.htm)

#### `public int[] AdjacentFaces(int faceIndex)`

Gets all faces that share a topological edge with a given face.

**Parameters:**
- `faceIndex` (System.Int32) — A face index.

**Returns:** `Int32[]` — All indices that share an edge.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_AdjacentFaces.htm)

#### `public void Clear()`

Clears the Face list on the mesh.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_Clear.htm)

#### `public int ConvertNonPlanarQuadsToTriangles(double planarTolerance, double angleToleranceRadians, int splitMethod)`

Splits non-planar quads into two triangles based on given parameters.

**Remarks:** If both planarTolerance = Rhino.RhinoMath.UnsetValue and angleToleranceRadians = Rhino.RhinoMath.UnsetValue, then all quads are split.

**Parameters:**
- `planarTolerance` (System.Double) — If planarTolerance >= 0, then a quad is split if its vertices are not coplanar. If both planarTolerance = Rhino.RhinoMath.UnsetValue and angleToleranceRadians >= 0.0, then the planarity test is skipped.
- `angleToleranceRadians` (System.Double) — If angleToleranceRadians >= 0.0, then a quad is split if the angle between opposite corner normals is > angleToleranceRadians. The corner normal is the normal to the triangle formed by two adjacent edges and the diagonal connecting their endpoints. A quad has four corner normals. If both angleToleranceRadians = Rhino.RhinoMath.UnsetValue and planarTolerance >= 0.0, then the corner normal angle test is skipped.
- `splitMethod` (System.Int32) — 0 default Currently divides along the short diagonal. This may be changed as better methods are found or preferences change. By passing zero, you let the developers of this code decide what's best for you over time. 1 divide along the short diagonal 2 divide along the long diagonal 3 minimize resulting area 4 maximize resulting area 5 minimize angle between triangle normals 6 maximize angle between triangle normals

**Returns:** `Int32` — Number of quads that were converted to triangles.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_ConvertNonPlanarQuadsToTriangles.htm)

#### `public bool ConvertQuadsToTriangles()`

Splits all quads along the short diagonal.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_ConvertQuadsToTriangles.htm)

#### `public bool ConvertTrianglesToQuads(double angleToleranceRadians, double minimumDiagonalLengthRatio)`

Joins adjacent triangles into quads if the resulting quad is 'nice'.

**Parameters:**
- `angleToleranceRadians` (System.Double) — Used to compare adjacent triangles' face normals. For two triangles to be considered, the angle between their face normals has to be <= angleToleranceRadians. When in doubt use RhinoMath.PI/90.0 (2 degrees).
- `minimumDiagonalLengthRatio` (System.Double) — ( <= 1.0) For two triangles to be considered the ratio of the resulting quad's diagonals (length of the shortest diagonal)/(length of longest diagonal). has to be >= minimumDiagonalLengthRatio. When in doubt us .875.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_ConvertTrianglesToQuads.htm)

#### `public int CullDegenerateFaces()`

Attempts to removes degenerate faces from the mesh. Degenerate faces are faces that contains such a combination of indices, that their final shape collapsed in a line or point.Before returning, this method also attempts to repair faces by juggling vertex indices.

**Returns:** `Int32` — The number of degenerate faces that were removed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_CullDegenerateFaces.htm)

#### `public int DeleteFaces(IEnumerable<int> faceIndexes)`

Removes a collection of faces from the mesh without affecting the remaining geometry.

**Parameters:**
- `faceIndexes` (System.Collections.Generic.IEnumerable<Int32>) — An array containing all the face indices to be removed.

**Returns:** `Int32` — The number of faces deleted on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_DeleteFaces.htm)

#### `public int DeleteFaces(IEnumerable<int> faceIndexes, bool compact)`

Removes a collection of faces from the mesh without affecting the remaining geometry.

**Parameters:**
- `faceIndexes` (System.Collections.Generic.IEnumerable<Int32>) — An array containing all the face indices to be removed.
- `compact` (System.Boolean) — If true, removes vertices that are no longer referenced.

**Returns:** `Int32` — The number of faces deleted on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_DeleteFaces_1.htm)

#### `public void Destroy()`

Releases all memory allocated to store faces. The list capacity will be 0 after this call. Subsequent calls can add new items.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_Destroy.htm)

#### `public Mesh ExtractDuplicateFaces()`

Extracts, or removes, duplicate faces.

**Returns:** `Mesh` — A mesh containing the extracted duplicate faces if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_ExtractDuplicateFaces.htm)

#### `public Mesh ExtractFaces(IEnumerable<int> faceIndices)`

Extracts, or removes, faces.

**Parameters:**
- `faceIndices` (System.Collections.Generic.IEnumerable<Int32>) — The face indices to be extracted.

**Returns:** `Mesh` — A mesh containing the extracted faces if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_ExtractFaces.htm)

#### `public IndexPair[] GetClashingFacePairs(int maxPairCount)`

Gets an array of pairs of mesh faces that clash.

**Parameters:**
- `maxPairCount` (System.Int32) — If >0, then at most this many pairs will be added to the output array. If <=0, then all clashing pairs will be added to the output array.

**Returns:** `IndexPair[]` — Array of pairs of mesh face indices.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_GetClashingFacePairs.htm)

#### `public int[] GetConnectedFaces(int faceIndex)`

Find all connected face indices

**Parameters:**
- `faceIndex` (System.Int32) — face index to start from

**Returns:** `Int32[]` — list of connected face indices

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_GetConnectedFaces.htm)

#### `public int[] GetConnectedFaces(int faceIndex, double angleRadians, bool greaterThanAngle)`

Find all connected face indices where adjacent face normals meet the criteria of angleRadians and greaterThanAngle

**Parameters:**
- `faceIndex` (System.Int32) — face index to start from
- `angleRadians` (System.Double) — angle to use for comparison of what is connected
- `greaterThanAngle` (System.Boolean) — If true angles greater than or equal to are considered connected. If false, angles less than or equal to are considered connected.

**Returns:** `Int32[]` — list of connected face indices

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_GetConnectedFaces_1.htm)

#### `public int[] GetConnectedFacesToEdges(int startFaceIndex, bool treatNonmanifoldLikeUnwelded)`

Uses startFaceIndex and finds all connected face indexes up to unwelded or naked edges. If treatNonmanifoldLikeUnwelded is true then non-manifold edges will be considered as unwelded or naked

**Parameters:**
- `startFaceIndex` (System.Int32) — Initial face index
- `treatNonmanifoldLikeUnwelded` (System.Boolean) — True means non-manifold edges will be handled like unwelded edges, False means they aren't considered

**Returns:** `Int32[]` — Array of connected face indexes

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_GetConnectedFacesToEdges.htm)

#### `public int[] GetDuplicateFaces()`

Finds all of the duplicate faces.

**Returns:** `Int32[]` — The indexes that are duplicates of other indexes if successful. If there are no duplicate, then an empty array is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_GetDuplicateFaces.htm)

#### `public IEnumerator<MeshFace> GetEnumerator()`

Gets an enumerator that yields all faces in this collection.

**Returns:** `IEnumerator<MeshFace>` — The enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_GetEnumerator.htm)

#### `public MeshFace GetFace(int index)`

Returns the mesh face at the given index.

**Parameters:**
- `index` (System.Int32) — Index of face to get. Must be larger than or equal to zero and smaller than the Face Count of the mesh.

**Returns:** `MeshFace` — The mesh face at the given index on success or MeshFace.Unset if the index is out of range.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_GetFace.htm)

#### `public double GetFaceAspectRatio(int index)`

Returns the mesh face at the given index.

**Parameters:**
- `index` (System.Int32) — Index of face to get. Must be larger than or equal to zero and smaller than the Face Count of the mesh.

**Returns:** `Double` — The mesh face at the given index on success or MeshFace.Unset if the index is out of range.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_GetFaceAspectRatio.htm)

#### `public BoundingBox GetFaceBoundingBox(int faceIndex)`

Gets the bounding box of a face.

**Parameters:**
- `faceIndex` (System.Int32) — A face index.

**Returns:** `BoundingBox` — A new bounding box, or Empty on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_GetFaceBoundingBox.htm)

#### `public Point3d GetFaceCenter(int faceIndex)`

Gets the center point of a face. For a triangular face, this is the centroid or barycenter.For a quad, this is the average of four comer points.

**Parameters:**
- `faceIndex` (System.Int32) — A face index.

**Returns:** `Point3d` — The center point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_GetFaceCenter.htm)

#### `public bool GetFaceVertices(int faceIndex, out Point3f a, out Point3f b, out Point3f c, out Point3f d)`

Gets the 3D location of the vertices forming a face.

**Parameters:**
- `faceIndex` (System.Int32) — A face index.
- `a` (Rhino.Geometry.Point3f) — A first point. This out argument is assigned during the call.
- `b` (Rhino.Geometry.Point3f) — A second point. This out argument is assigned during the call.
- `c` (Rhino.Geometry.Point3f) — A third point. This out argument is assigned during the call.
- `d` (Rhino.Geometry.Point3f) — A fourth point. This out argument is assigned during the call.

**Returns:** `Boolean` — true if the operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_GetFaceVertices.htm)

#### `public int[] GetTopologicalVertices(int faceIndex)`

Gets the topology vertex indices of a face.

**Parameters:**
- `faceIndex` (System.Int32) — A face index.

**Returns:** `Int32[]` — An array of integers.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_GetTopologicalVertices.htm)

#### `public bool GetZeroAreaFaces(out int[] whollyDegenerateFaces, out int[] partiallyDegenerateFaces)`

Examines and adds face indexes to whollyDegenerateFaces if the face is a triangle with zero area or a quad both triangles have zero area. Face indexes are added to partiallyDegenerateFaces when a quad has one triangle with zero area.

**Parameters:**
- `whollyDegenerateFaces` (System.Int32[]) — Array of indexes for faces, both triangles and quads, that have zero area.
- `partiallyDegenerateFaces` (System.Int32[]) — Array of indexes for quad faces, that have one triangle with zero area.

**Returns:** `Boolean` — Returns true if the mesh has wholly or partially degenerate faces, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_GetZeroAreaFaces.htm)

#### `public bool HasNakedEdges(int faceIndex)`

Returns true if at least one of the face edges are not topologically connected to any other faces.

**Parameters:**
- `faceIndex` (System.Int32) — A face index.

**Returns:** `Boolean` — true if that face makes the mesh open, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_HasNakedEdges.htm)

#### `public void Insert(int index, MeshFace face)`

Inserts a mesh face at a defined index in this list.

**Parameters:**
- `index` (System.Int32) — An index.
- `face` (Rhino.Geometry.MeshFace) — A face.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_Insert.htm)

#### `public bool IsHidden(int faceIndex)`

Gets a value indicating whether a face is hidden. A face is hidden if, and only if, at least one of its vertices is hidden.

**Parameters:**
- `faceIndex` (System.Int32) — A face index.

**Returns:** `Boolean` — true if hidden, false if fully visible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_IsHidden.htm)

#### `public bool MergeAdjacentFaces(int edgeIndex)`

Merges two triangular mesh faces that share an edge into one quadrangular face.

**Parameters:**
- `edgeIndex` (System.Int32) — The common topological edge index.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_MergeAdjacentFaces.htm)

#### `public void RemoveAt(int index)`

Removes a face from the mesh.

**Parameters:**
- `index` (System.Int32) — The index of the face that will be removed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_RemoveAt.htm)

#### `public void RemoveAt(int index, bool compact)`

Removes a face from the mesh.

**Parameters:**
- `index` (System.Int32) — The index of the face that will be removed.
- `compact` (System.Boolean) — If true, removes vertices that are no longer referenced.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_RemoveAt_1.htm)

#### `public int RemoveZeroAreaFaces(ref int fixedFaceCount)`

Deletes or fixes mesh faces that have zero area.

**Parameters:**
- `fixedFaceCount` (System.Int32) — Number of fixed partially degenerate faces.

**Returns:** `Int32` — Number of removed wholly degenerate faces.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_RemoveZeroAreaFaces.htm)

#### `public bool SetFace(int index, int vertex1, int vertex2, int vertex3)`

Sets a triangular face at a specific index of the mesh.

**Parameters:**
- `index` (System.Int32) — A position in the list.
- `vertex1` (System.Int32) — The first vertex index.
- `vertex2` (System.Int32) — The second vertex index.
- `vertex3` (System.Int32) — The third vertex index.

**Returns:** `Boolean` — true if the operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_SetFace_1.htm)

#### `public bool SetFace(int index, int vertex1, int vertex2, int vertex3, int vertex4)`

Sets a quadrangular face at a specific index of the mesh.

**Parameters:**
- `index` (System.Int32) — A position in the list.
- `vertex1` (System.Int32) — The first vertex index.
- `vertex2` (System.Int32) — The second vertex index.
- `vertex3` (System.Int32) — The third vertex index.
- `vertex4` (System.Int32) — The fourth vertex index.

**Returns:** `Boolean` — true if the operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_SetFace_2.htm)

#### `public bool SetFace(int index, MeshFace face)`

Sets a face at a specific index of the mesh.

**Parameters:**
- `index` (System.Int32) — A position in the list.
- `face` (Rhino.Geometry.MeshFace) — A face.

**Returns:** `Boolean` — true if the operation succeeded, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_SetFace.htm)

#### `public int[] ToIntArray(bool asTriangles)`

Copies all of the face indices to a linear array of indices per face. Note that this includes indices from invalid faces too.

**Parameters:**
- `asTriangles` (System.Boolean) — If set to true as triangles.

**Returns:** `Int32[]` — The int array. This method never returns null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_ToIntArray.htm)

#### `public int[] ToIntArray(bool asTriangles, ref List<int> replacedIndices)`

Copies all of the faces to a linear array of indices. Clean-up of vertex indices if replacedIndices is a valid List<int> ///

**Parameters:**
- `asTriangles` (System.Boolean) — If set to true as triangles.
- `replacedIndices` (System.Collections.Generic.List<Int32>) — List is populated with vertex indices that were replaced with 0. If replacedIndices is null there will be no cleanup

**Returns:** `Int32[]` — The int array. This method never returns null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceList_ToIntArray_1.htm)

### Properties
- `Capacity` (Int32, get/set) — Gets or sets the total number of mesh triangles and quads the internal data structure can hold without resizing.
- `Count` (Int32, get/set) — Gets or sets the number of mesh faces. When getting this can includes invalid faces.
- `Item` (MeshFace, get/set) — Returns the mesh face at the given index.
- `QuadCount` (Int32, get) — Gets the number of faces that are valid quads (4 corners).
- `TriangleCount` (Int32, get) — Gets the number of faces that are valid triangles (3 corners).

## MeshFaceNormalList (class)

Provides access to the Face normals of a Mesh.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_MeshFaceNormalList.htm)

### Methods
#### `public int AddFaceNormal(double x, double y, double z)`

Appends a face normal to the list of mesh face normals.

**Parameters:**
- `x` (System.Double) — X component of face normal.
- `y` (System.Double) — Y component of face normal.
- `z` (System.Double) — Z component of face normal.

**Returns:** `Int32` — The index of the newly added face normal.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceNormalList_AddFaceNormal_2.htm)

#### `public int AddFaceNormal(float x, float y, float z)`

Appends a face normal to the list of mesh face normals.

**Parameters:**
- `x` (System.Single) — X component of face normal.
- `y` (System.Single) — Y component of face normal.
- `z` (System.Single) — Z component of face normal.

**Returns:** `Int32` — The index of the newly added face normal.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceNormalList_AddFaceNormal_3.htm)

#### `public int AddFaceNormal(Vector3d normal)`

Appends a face normal to the list of mesh face normals.

**Parameters:**
- `normal` (Rhino.Geometry.Vector3d) — New face normal.

**Returns:** `Int32` — The index of the newly added face normal.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceNormalList_AddFaceNormal.htm)

#### `public int AddFaceNormal(Vector3f normal)`

Appends a face normal to the list of mesh face normals.

**Parameters:**
- `normal` (Rhino.Geometry.Vector3f) — New face normal.

**Returns:** `Int32` — The index of the newly added face normal.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceNormalList_AddFaceNormal_1.htm)

#### `public void Clear()`

Clears the Face Normal list on the mesh.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceNormalList_Clear.htm)

#### `public bool ComputeFaceNormals()`

Computes all the face normals for this mesh based on the physical shape of the mesh.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceNormalList_ComputeFaceNormals.htm)

#### `public void Destroy()`

Releases all memory allocated to store face normals. The list capacity will be 0 after this call. Subsequent calls can add new items.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceNormalList_Destroy.htm)

#### `public IEnumerator<Vector3f> GetEnumerator()`

Gets an enumerator that yields all normals (vectors) in this collection.

**Returns:** `IEnumerator<Vector3f>` — The enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceNormalList_GetEnumerator.htm)

#### `public bool SetFaceNormal(int index, double x, double y, double z)`

Sets a face normal vector at an index using three double-precision numbers.

**Parameters:**
- `index` (System.Int32) — An index.
- `x` (System.Double) — A x component.
- `y` (System.Double) — A y component.
- `z` (System.Double) — A z component.

**Returns:** `Boolean` — true on success; false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceNormalList_SetFaceNormal_2.htm)

#### `public bool SetFaceNormal(int index, float x, float y, float z)`

Sets a face normal vector at an index using three single-precision numbers.

**Parameters:**
- `index` (System.Int32) — An index.
- `x` (System.Single) — A x component.
- `y` (System.Single) — A y component.
- `z` (System.Single) — A z component.

**Returns:** `Boolean` — true on success; false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceNormalList_SetFaceNormal_3.htm)

#### `public bool SetFaceNormal(int index, Vector3d normal)`

Sets a face normal vector at an index using a single-precision vector.

**Parameters:**
- `index` (System.Int32) — An index.
- `normal` (Rhino.Geometry.Vector3d) — A normal vector.

**Returns:** `Boolean` — true on success; false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceNormalList_SetFaceNormal.htm)

#### `public bool SetFaceNormal(int index, Vector3f normal)`

Sets a face normal vector at an index using a single-precision vector.

**Parameters:**
- `index` (System.Int32) — An index.
- `normal` (Rhino.Geometry.Vector3f) — A normal vector.

**Returns:** `Boolean` — true on success; false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceNormalList_SetFaceNormal_1.htm)

#### `public bool UnitizeFaceNormals()`

Unitizes all the existing face normals.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshFaceNormalList_UnitizeFaceNormals.htm)

### Properties
- `Capacity` (Int32, get/set) — Gets or sets the total number of face normals the internal data structure can hold without resizing.
- `Count` (Int32, get/set) — Gets or sets the number of mesh face normals.
- `Item` (Vector3f, get/set) — Gets or sets the face normal at the given face index. The index must be valid or an IndexOutOfRangeException will be thrown.

## MeshNgonList (class)

Provides access to the ngons and ngon-related functionality of a Mesh. See also the Ngons property for Ngon functionality details.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_MeshNgonList.htm)

### Methods
#### `public int AddNgon(MeshNgon ngon)`

Appends a new ngon to the end of the mesh ngon list.

**Parameters:**
- `ngon` (Rhino.Geometry.MeshNgon) — Ngon to add.

**Returns:** `Int32` — The index of the newly added ngon.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_AddNgon.htm)

#### `public int[] AddNgons(IEnumerable<MeshNgon> ngons)`

Appends a list of ngons to the end of the mesh ngon list.

**Parameters:**
- `ngons` (System.Collections.Generic.IEnumerable<MeshNgon>) — Ngons to add.

**Returns:** `Int32[]` — Indices of the newly created ngons

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_AddNgons.htm)

#### `public int AddPlanarNgons(double planarTolerance)`

Add an ngon for each group of connected coplanar faces.

**Parameters:**
- `planarTolerance` (System.Double) — 3d distance tolerance for coplanar test.

**Returns:** `Int32` — Number of ngons added to the mesh.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_AddPlanarNgons.htm)

#### `public int AddPlanarNgons(double planarTolerance, int minimumNgonVertexCount, int minimumNgonFaceCount, bool allowHoles)`

Add an ngon for each group of connected coplanar faces.

**Parameters:**
- `planarTolerance` (System.Double) — 3d distance tolerance for coplanar test.
- `minimumNgonVertexCount` (System.Int32) — Minimum number of vertices for an ngon.
- `minimumNgonFaceCount` (System.Int32) — Minimum number of faces for an ngon.
- `allowHoles` (System.Boolean) — Determines whether the ngon can have inner boundaries.

**Returns:** `Int32` — Number of ngons added to the mesh.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_AddPlanarNgons_1.htm)

#### `public void Clear()`

Clears the Ngon list on the mesh.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_Clear.htm)

#### `public IEnumerator<MeshNgon> GetEnumerator()`

Gets an enumerator that yields all ngons in this collection.

**Returns:** `IEnumerator<MeshNgon>` — The enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_GetEnumerator.htm)

#### `public MeshNgon GetNgon(int index)`

Returns the mesh ngon at the given index.

**Parameters:**
- `index` (System.Int32) — Index of ngon to get. Must be larger than or equal to zero and smaller than the Ngon Count of the mesh.

**Returns:** `MeshNgon` — The mesh ngon at the given index. This ngon can be MeshNgon.Empty.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_GetNgon.htm)

#### `public int[] GetNgonBoundary(IEnumerable<int> ngonFaceIndexList)`

Get a list of vertices that form the boundary of a face set. This is often use to get ngon boundaries.

**Parameters:**
- `ngonFaceIndexList` (System.Collections.Generic.IEnumerable<Int32>) — List of mesh face indices.

**Returns:** `Int32[]` — List of mesh vertex indices that form the boundary of the face set.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_GetNgonBoundary.htm)

#### `public BoundingBox GetNgonBoundingBox(int index)`

Gets the bounding box of an ngon.

**Parameters:**
- `index` (System.Int32) — A ngon index.

**Returns:** `BoundingBox` — A new bounding box, or Empty on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_GetNgonBoundingBox_1.htm)

#### `public BoundingBox GetNgonBoundingBox(MeshNgon ngon)`

Gets the bounding box of an ngon.

**Parameters:**
- `ngon` (Rhino.Geometry.MeshNgon) — An ngon.

**Returns:** `BoundingBox` — A new bounding box, or Empty on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_GetNgonBoundingBox.htm)

#### `public Point3d GetNgonCenter(int index)`

Gets the center point of an ngon. This the average of the corner points.

**Parameters:**
- `index` (System.Int32) — A ngon index.

**Returns:** `Point3d` — The center point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_GetNgonCenter_1.htm)

#### `public Point3d GetNgonCenter(MeshNgon ngon)`

Gets the center point of an ngon. This the average of the corner points.

**Parameters:**
- `ngon` (Rhino.Geometry.MeshNgon) — An ngon.

**Returns:** `Point3d` — The center point.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_GetNgonCenter.htm)

#### `public int GetNgonEdgeCount(int index)`

Gets the complete edge count of an ngon.

**Parameters:**
- `index` (System.Int32) — Ngon index.

**Returns:** `Int32` — Complete edge count or zero on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_GetNgonEdgeCount.htm)

#### `public int GetNgonOuterEdgeCount(int index)`

Gets the outer edge count of an ngon.

**Parameters:**
- `index` (System.Int32) — Ngon index.

**Returns:** `Int32` — Outer edge count or zero on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_GetNgonOuterEdgeCount.htm)

#### `public void Insert(int index, MeshNgon ngon)`

Inserts a mesh ngon at a defined index in this list.

**Parameters:**
- `index` (System.Int32) — An ngon index.
- `ngon` (Rhino.Geometry.MeshNgon) — An ngon.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_Insert.htm)

#### `public uint IsValid(int index)`

Tests an ngon to see if the vertex and face references are valid and pass partial boundary validity checks.

**Remarks:** If the return value is > MeshNgon.BoundaryVertexCount, then the ngon has either inner boundaries or duplicate vertices.

**Parameters:**
- `index` (System.Int32) — The index of the ngon to test.

**Returns:** `UInt32` — 0 if the ngon is not valid, otherwise the number of boundary edges.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_IsValid.htm)

#### `public uint IsValid(int index, TextLog textLog)`

Tests an ngon to see if the vertex and face references are valid and pass partial boundary validity checks.

**Remarks:** If the return value is > MeshNgon.BoundaryVertexCount, then the ngon has either inner boundaries or duplicate vertices.

**Parameters:**
- `index` (System.Int32) — The index of the ngon to test.
- `textLog` (Rhino.FileIO.TextLog) — A text log for collecting information about problems.

**Returns:** `UInt32` — 0 if the ngon is not valid, otherwise the number of boundary edges.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_IsValid_1.htm)

#### `public Point3d[] NgonBoundaryVertexList(MeshNgon ngon, bool bAppendStartPoint)`

Get an ngon's boundary points.

**Parameters:**
- `ngon` (Rhino.Geometry.MeshNgon) — ngon.
- `bAppendStartPoint` (System.Boolean) — If true, the first point in the list is also appended to the end of the list to create a closed polyline.

**Returns:** `Point3d[]` — A list of ngon boundary points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_NgonBoundaryVertexList.htm)

#### `public bool NgonHasHoles(int index)`

Determines whether a ngon has holes.

**Remarks:** A slit, for example, will give an edge count that differs from outer edge count despite the lack of true "holes" i.e. interior edges that are not shared by more than one face of the ngon in question.

**Parameters:**
- `index` (System.Int32) — Ngon index.

**Returns:** `Boolean` — true for holes (or malformed ngon, see remarks), false for no holes.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_NgonHasHoles.htm)

#### `public int NgonIndexFromFaceIndex(int meshFaceIndex)`

Returns the index of a mesh ngon the face belongs to.

**Parameters:**
- `meshFaceIndex` (System.Int32) — Index of a mesh face.

**Returns:** `Int32` — The index of the mesh ngon the face belongs to or -1 if the face does not belong to an ngon.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_NgonIndexFromFaceIndex.htm)

#### `public int Orientation(int index, bool permitHoles)`

Determine if the ngon's boundary orientation matches that of the set of faces it is made from.

**Parameters:**
- `index` (System.Int32) — Ngon index.
- `permitHoles` (System.Boolean) — true if the ngon is permitted to have interior holes, false otherwise.

**Returns:** `Int32` — 1: The ngon does not have holes, the ngon's faces are compatibly oriented, and the ngon's outer boundary orientation matches the faces' orientation. -1: The ngon does not have holes, the ngon's faces are compatibly oriented, and the ngon's outer boundary orientation is opposite the faces' orientation. 0: Otherwise.The ngon may be invalid, have holes, the ngon's faces may not be compatibly oriented, the ngons edges may not have a consistent orientation with respect to the faces, or some other issue.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_Orientation.htm)

#### `public int RemoveAllNgons()`

Removes all ngons from the mesh.

**Returns:** `Int32` — The number deleted ngons.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_RemoveAllNgons.htm)

#### `public void RemoveAt(int index)`

Removes an ngon from the mesh.

**Parameters:**
- `index` (System.Int32) — The index of the ngon.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_RemoveAt.htm)

#### `public int RemoveNgons(IEnumerable<int> indices)`

Remove one or more ngons from the mesh.

**Parameters:**
- `indices` (System.Collections.Generic.IEnumerable<Int32>) — An array of ngon indices.

**Returns:** `Int32` — The number of deleted ngons.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_RemoveNgons.htm)

#### `public void ReverseOuterBoundary(int index)`

Reverse the order of the m_vi[] array for an ngon

**Parameters:**
- `index` (System.Int32)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_ReverseOuterBoundary.htm)

#### `public void SetNgon(int index, MeshNgon ngon)`

Set an ngon in this list.

**Parameters:**
- `index` (System.Int32) — An ngon index.
- `ngon` (Rhino.Geometry.MeshNgon) — An ngon.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshNgonList_SetNgon.htm)

### Properties
- `Count` (Int32, get/set) — Gets or sets the number of mesh ngons.
- `Item` (MeshNgon, get/set) — Returns the mesh ngon at the given index.
- `UnsignedCount` (UInt32, get/set) — Gets or sets the number of mesh ngons.

## MeshTextureCoordinateList (class)

Provides access to the Vertex Texture coordinates of a Mesh.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_MeshTextureCoordinateList.htm)

### Methods
#### `public int Add(double s, double t)`

Adds a new texture coordinate to the end of the Texture list.

**Parameters:**
- `s` (System.Double) — S component of new texture coordinate.
- `t` (System.Double) — T component of new texture coordinate.

**Returns:** `Int32` — The index of the newly added texture coordinate.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTextureCoordinateList_Add_2.htm)

#### `public int Add(Point2f tc)`

Adds a new texture coordinate to the end of the Texture list.

**Parameters:**
- `tc` (Rhino.Geometry.Point2f) — Texture coordinate to add.

**Returns:** `Int32` — The index of the newly added texture coordinate.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTextureCoordinateList_Add.htm)

#### `public int Add(Point3d tc)`

Adds a new texture coordinate to the end of the Texture list.

**Parameters:**
- `tc` (Rhino.Geometry.Point3d) — Texture coordinate to add.

**Returns:** `Int32` — The index of the newly added texture coordinate.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTextureCoordinateList_Add_1.htm)

#### `public int Add(float s, float t)`

Adds a new texture coordinate to the end of the Texture list.

**Parameters:**
- `s` (System.Single) — S component of new texture coordinate.
- `t` (System.Single) — T component of new texture coordinate.

**Returns:** `Int32` — The index of the newly added texture coordinate.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTextureCoordinateList_Add_3.htm)

#### `public bool AddRange(Point2f[] textureCoordinates)`

Appends an array of texture coordinates.

**Parameters:**
- `textureCoordinates` (Rhino.Geometry.Point2f[]) — Texture coordinates to append.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTextureCoordinateList_AddRange.htm)

#### `public void Clear()`

Clears the Texture Coordinate list on the mesh.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTextureCoordinateList_Clear.htm)

#### `public void Destroy()`

Releases all memory allocated to store texture coordinates. The list capacity will be 0 after this call. Subsequent calls can add new items.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTextureCoordinateList_Destroy.htm)

#### `public IEnumerator<Point2f> GetEnumerator()`

Gets an enumerator that yields all texture coordinates in this collection.

**Returns:** `IEnumerator<Point2f>` — The enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTextureCoordinateList_GetEnumerator.htm)

#### `public bool NormalizeTextureCoordinates()`

Scales the texture coordinates so the texture domains are [0,1] and eliminate any texture rotations.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTextureCoordinateList_NormalizeTextureCoordinates.htm)

#### `public bool ReverseTextureCoordinates(int direction)`

Reverses one coordinate direction of the texture coordinates. The region of the bitmap the texture uses does not change. Either Us or Vs direction is flipped.

**Parameters:**
- `direction` (System.Int32) — 0 = first texture coordinate is reversed.1 = second texture coordinate is reversed.

**Returns:** `Boolean` — true if operation succeeded; otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTextureCoordinateList_ReverseTextureCoordinates.htm)

#### `public bool SetTextureCoordinate(int index, double s, double t)`

Sets or adds a texture coordinate to the Texture Coordinate List. If [index] is less than [Count], the existing coordinate at [index] will be modified.If [index] equals [Count], a new coordinate is appended to the end of the coordinate list.If [index] is larger than [Count], the function will return false.

**Parameters:**
- `index` (System.Int32) — Index of texture coordinate to set.
- `s` (System.Double) — S component of texture coordinate.
- `t` (System.Double) — T component of texture coordinate.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTextureCoordinateList_SetTextureCoordinate_2.htm)

#### `public bool SetTextureCoordinate(int index, Point2f tc)`

Sets or adds a texture coordinate to the Texture Coordinate List. If [index] is less than [Count], the existing coordinate at [index] will be modified.If [index] equals [Count], a new coordinate is appended to the end of the coordinate list.If [index] is larger than [Count], the function will return false.

**Parameters:**
- `index` (System.Int32) — Index of texture coordinate to set.
- `tc` (Rhino.Geometry.Point2f) — Texture coordinate point.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTextureCoordinateList_SetTextureCoordinate.htm)

#### `public bool SetTextureCoordinate(int index, Point3f tc)`

Sets or adds a texture coordinate to the Texture Coordinate List. If [index] is less than [Count], the existing coordinate at [index] will be modified.If [index] equals [Count], a new coordinate is appended to the end of the coordinate list.If [index] is larger than [Count], the function will return false.

**Parameters:**
- `index` (System.Int32) — Index of texture coordinate to set.
- `tc` (Rhino.Geometry.Point3f) — Texture coordinate point.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTextureCoordinateList_SetTextureCoordinate_1.htm)

#### `public bool SetTextureCoordinate(int index, float s, float t)`

Sets or adds a texture coordinate to the Texture Coordinate List. If [index] is less than [Count], the existing coordinate at [index] will be modified.If [index] equals [Count], a new coordinate is appended to the end of the coordinate list.If [index] is larger than [Count], the function will return false.

**Parameters:**
- `index` (System.Int32) — Index of texture coordinate to set.
- `s` (System.Single) — S component of texture coordinate.
- `t` (System.Single) — T component of texture coordinate.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTextureCoordinateList_SetTextureCoordinate_3.htm)

#### `public bool SetTextureCoordinates(Point2f[] textureCoordinates)`

Sets all texture coordinates in one go.

**Parameters:**
- `textureCoordinates` (Rhino.Geometry.Point2f[]) — Texture coordinates to assign to the mesh.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTextureCoordinateList_SetTextureCoordinates.htm)

#### `public bool SetTextureCoordinates(TextureMapping mapping)`

Set all texture coordinates based on a texture mapping function

**Parameters:**
- `mapping` (Rhino.Render.TextureMapping) — The new mapping type.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTextureCoordinateList_SetTextureCoordinates_1.htm)

#### `public float[] ToFloatArray()`

Copies all vertices to a linear array of float in u,v order

**Returns:** `Single[]` — The float array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTextureCoordinateList_ToFloatArray.htm)

#### `public bool TransposeTextureCoordinates()`

Transposes texture coordinates. The region of the bitmap the texture uses does not change. All texture coordinates rows (Us) become columns (Vs), and vice versa.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTextureCoordinateList_TransposeTextureCoordinates.htm)

### Properties
- `Capacity` (Int32, get/set) — Gets or sets the total number of texture coordinates the internal data structure can hold without resizing.
- `Count` (Int32, get/set) — Gets or sets the number of texture coordinates.
- `Item` (Point2f, get/set) — Gets or sets the texture coordinate at the given index. The index must be valid or an IndexOutOfRangeException will be thrown.

## MeshTopologyEdgeList (class)

Represents an entry point to the list of edges in a mesh topology.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_MeshTopologyEdgeList.htm)

### Methods
#### `public bool CollapseEdge(int topologyEdgeIndex)`

Replaces a mesh edge with a vertex at its center and update adjacent faces as needed.

**Parameters:**
- `topologyEdgeIndex` (System.Int32) — An index of a topology edge in TopologyEdges.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyEdgeList_CollapseEdge.htm)

#### `public Line EdgeLine(int topologyEdgeIndex)`

Gets the 3d line along an edge.

**Parameters:**
- `topologyEdgeIndex` (System.Int32) — The topology edge index.

**Returns:** `Line` — Line along edge. If input is not valid, an Invalid Line is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyEdgeList_EdgeLine.htm)

#### `public int[] GetConnectedFaces(int topologyEdgeIndex)`

Gets indices of faces connected to an edge.

**Parameters:**
- `topologyEdgeIndex` (System.Int32) — An index of a topology edge that is queried.

**Returns:** `Int32[]` — An array of face indices the edge borders. This might be empty on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyEdgeList_GetConnectedFaces.htm)

#### `public int[] GetConnectedFaces(int topologyEdgeIndex, out bool[] faceOrientationMatchesEdgeDirection)`

Gets indices of faces connected to an edge.

**Parameters:**
- `topologyEdgeIndex` (System.Int32) — An index of a topology edge that is queried.
- `faceOrientationMatchesEdgeDirection` (System.Boolean[]) — An array of Boolean values that explains whether each face direction matches the direction of the specified edge.

**Returns:** `Int32[]` — An array of face indices the edge borders. This might be empty on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyEdgeList_GetConnectedFaces_1.htm)

#### `public int GetEdgeIndex(int topologyVertex1, int topologyVertex2)`

Returns index of edge that connects topological vertices. returns -1 if no edge is found.

**Parameters:**
- `topologyVertex1` (System.Int32) — The first topology vertex index.
- `topologyVertex2` (System.Int32) — The second topology vertex index.

**Returns:** `Int32` — The edge index.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyEdgeList_GetEdgeIndex.htm)

#### `public int[] GetEdgesForFace(int faceIndex)`

Gets indices of edges that surround a given face.

**Parameters:**
- `faceIndex` (System.Int32) — A face index.

**Returns:** `Int32[]` — A new array of indices to the topological edges that are connected with the specified face.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyEdgeList_GetEdgesForFace.htm)

#### `public int[] GetEdgesForFace(int faceIndex, out bool[] sameOrientation)`

Gets indices of edges that surround a given face.

**Parameters:**
- `faceIndex` (System.Int32) — A face index.
- `sameOrientation` (System.Boolean[]) — Same length as returned edge index array. For each edge, the sameOrientation value tells you if the edge orientation matches the face orientation (true), or is reversed (false) compared to it.

**Returns:** `Int32[]` — A new array of indices to the topological edges that are connected with the specified face.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyEdgeList_GetEdgesForFace_1.htm)

#### `public IndexPair GetTopologyVertices(int topologyEdgeIndex)`

Gets the two topology vertices for a given topology edge.

**Parameters:**
- `topologyEdgeIndex` (System.Int32) — An index of a topology edge.

**Returns:** `IndexPair` — The pair of vertex indices the edge connects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyEdgeList_GetTopologyVertices.htm)

#### `public bool IsEdgeUnwelded(int topologyEdgeIndex)`

Determines if the mesh edge is unwelded, or if the mesh faces that share the edge have unique vertex indices.

**Parameters:**
- `topologyEdgeIndex` (System.Int32) — An index of a topology edge in TopologyEdges.

**Returns:** `Boolean` — true if the edge is unwelded, false if the edge is welded.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyEdgeList_IsEdgeUnwelded.htm)

#### `public bool IsHidden(int topologyEdgeIndex)`

Returns true if the topological edge is hidden. The mesh topology edge is hidden only if either of its mesh topology vertices is hidden.

**Parameters:**
- `topologyEdgeIndex` (System.Int32) — An index of a topology edge in TopologyEdges.

**Returns:** `Boolean` — true if mesh topology edge is hidden.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyEdgeList_IsHidden.htm)

#### `public bool IsNgonInterior(int topologyEdgeIndex)`

Returns true if the topological edge is an interior ngon edge

**Parameters:**
- `topologyEdgeIndex` (System.Int32) — An index of a topology edge in TopologyEdges.

**Returns:** `Boolean` — true if mesh topology edge is an interior ngon edge.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyEdgeList_IsNgonInterior.htm)

#### `public bool IsSwappableEdge(int topologyEdgeIndex)`

Determines if a mesh edge index is valid input for SwapEdge(Int32).

**Parameters:**
- `topologyEdgeIndex` (System.Int32) — An index of a topology edge in TopologyEdges.

**Returns:** `Boolean` — true if edge can be swapped.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyEdgeList_IsSwappableEdge.htm)

#### `public bool SplitEdge(int topologyEdgeIndex, double t)`

Divides a mesh edge to create two or more triangles

**Parameters:**
- `topologyEdgeIndex` (System.Int32) — Edge to divide
- `t` (System.Double) — Parameter along edge. This is the same as getting an EdgeLine and calling PointAt(t) on that line

**Returns:** `Boolean` — true if successful

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyEdgeList_SplitEdge_1.htm)

#### `public bool SplitEdge(int topologyEdgeIndex, Point3d point)`

Divides a mesh edge to create two or more triangles

**Parameters:**
- `topologyEdgeIndex` (System.Int32) — Edge to divide
- `point` (Rhino.Geometry.Point3d) — Location to perform the split

**Returns:** `Boolean` — true if successful

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyEdgeList_SplitEdge.htm)

#### `public bool SwapEdge(int topologyEdgeIndex)`

If the edge is shared by two triangular face, then the edge is swapped.

**Parameters:**
- `topologyEdgeIndex` (System.Int32) — An index of a topology edge in TopologyEdges.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyEdgeList_SwapEdge.htm)

### Properties
- `Count` (Int32, get) — Gets the amount of edges in this list.

## MeshTopologyVertexList (class)

Provides access to the mesh topology vertices of a mesh. Topology vertices are sets of vertices in the MeshVertexList that can topologically be considered the same vertex.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_MeshTopologyVertexList.htm)

### Methods
#### `public int ConnectedEdge(int topologyVertexIndex, int edgeAtVertexIndex)`

Gets a particular edge that is connected to a topological vertex. Call TopologyVertices.SortVertices before this if you are interested in ordered edges.

**Parameters:**
- `topologyVertexIndex` (System.Int32) — Index of a topology vertex in Mesh.TopologyVertices.
- `edgeAtVertexIndex` (System.Int32) — Index of the edge at the vertex.

**Returns:** `Int32` — The index of the connected edge.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyVertexList_ConnectedEdge.htm)

#### `public int[] ConnectedEdges(int topologyVertexIndex)`

Gets all edges that are connected to a given vertex. Call TopologyVertices.SortVertices before this if you are interested in ordered edges.

**Parameters:**
- `topologyVertexIndex` (System.Int32) — Index of a topology vertex in Mesh.TopologyVertices.

**Returns:** `Int32[]` — Indices of all edges around vertex that are connected to this topological vertex. null if no faces are connected to this vertex.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyVertexList_ConnectedEdges.htm)

#### `public int ConnectedEdgesCount(int topologyVertexIndex)`

Gets the count of edges that are connected to a given vertex.

**Parameters:**
- `topologyVertexIndex` (System.Int32) — Index of a topology vertex in Mesh.TopologyVertices.

**Returns:** `Int32` — The amount of edges at this vertex. This can be 0.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyVertexList_ConnectedEdgesCount.htm)

#### `public int[] ConnectedFaces(int topologyVertexIndex)`

Gets all faces that are connected to a given vertex.

**Parameters:**
- `topologyVertexIndex` (System.Int32) — Index of a topology vertex in Mesh.TopologyVertices.

**Returns:** `Int32[]` — Indices of all faces in Mesh.Faces that are connected to this topological vertex. null if no faces are connected to this vertex.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyVertexList_ConnectedFaces.htm)

#### `public int[] ConnectedTopologyVertices(int topologyVertexIndex)`

Gets all topological vertices that are connected to a given vertex.

**Parameters:**
- `topologyVertexIndex` (System.Int32) — index of a topology vertex in Mesh.TopologyVertices.

**Returns:** `Int32[]` — Indices of all topological vertices that are connected to this topological vertex. null if no vertices are connected to this vertex.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyVertexList_ConnectedTopologyVertices.htm)

#### `public int[] ConnectedTopologyVertices(int topologyVertexIndex, bool sorted)`

Gets all topological vertices that are connected to a given vertex.

**Parameters:**
- `topologyVertexIndex` (System.Int32) — index of a topology vertex in Mesh.TopologyVertices.
- `sorted` (System.Boolean) — if true, the vertices are returned in a radially sorted order.

**Returns:** `Int32[]` — Indices of all topological vertices that are connected to this topological vertex. null if no vertices are connected to this vertex.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyVertexList_ConnectedTopologyVertices_1.htm)

#### `public IEnumerator<Point3f> GetEnumerator()`

Gets an enumerator that yields all topology vertices in this collection.

**Returns:** `IEnumerator<Point3f>` — The enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyVertexList_GetEnumerator.htm)

#### `public int[] IndicesFromFace(int faceIndex)`

Returns TopologyVertexIndices for a given mesh face index.

**Parameters:**
- `faceIndex` (System.Int32) — The index of a face to query.

**Returns:** `Int32[]` — An array of vertex indices.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyVertexList_IndicesFromFace.htm)

#### `public bool IsHidden(int topologyVertexIndex)`

Returns true if the topological vertex is hidden. The mesh topology vertex is hidden if and only if all the ON_Mesh vertices it represents is hidden.

**Parameters:**
- `topologyVertexIndex` (System.Int32) — index of a topology vertex in Mesh.TopologyVertices.

**Returns:** `Boolean` — true if mesh topology vertex is hidden.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyVertexList_IsHidden.htm)

#### `public int[] MeshVertexIndices(int topologyVertexIndex)`

Gets all indices of the mesh vertices that a given topology vertex represents.

**Parameters:**
- `topologyVertexIndex` (System.Int32) — Index of a topology vertex in Mesh.TopologyVertices to query.

**Returns:** `Int32[]` — Indices of all vertices that in Mesh.Vertices that a topology vertex represents.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyVertexList_MeshVertexIndices.htm)

#### `public bool SortEdges()`

Sorts the edge list for the mesh topology vertex list so that the edges are in radial order when you call ConnectedTopologyVertices. A non-manifold edge is treated as a boundary edge with respect to sorting. If any boundary or non-manifold edges end at the vertex, then the first edge will be a boundary or non-manifold edge.

**Returns:** `Boolean` — true on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyVertexList_SortEdges.htm)

#### `public bool SortEdges(int topologyVertexIndex)`

Sorts the edge list for as single mesh topology vertex so that the edges are in radial order when you call ConnectedTopologyVertices. A non-manifold edge is treated as a boundary edge with respect to sorting. If any boundary or non-manifold edges end at the vertex, then the first edge will be a boundary or non-manifold edge.

**Parameters:**
- `topologyVertexIndex` (System.Int32) — index of a topology vertex in Mesh.TopologyVertices>

**Returns:** `Boolean` — true on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyVertexList_SortEdges_1.htm)

#### `public int TopologyVertexIndex(int vertexIndex)`

Gets the topology vertex index for an existing mesh vertex in the mesh's VertexList.

**Parameters:**
- `vertexIndex` (System.Int32) — Index of a vertex in the Mesh.Vertices.

**Returns:** `Int32` — Index of a topology vertex in the Mesh.TopologyVertices.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshTopologyVertexList_TopologyVertexIndex.htm)

### Properties
- `Count` (Int32, get) — Gets or sets the number of mesh topology vertices.
- `Item` (Point3f, get/set) — Gets or sets the vertex at the given index. Setting a location adjusts all vertices in the mesh's vertex list that are defined by this topological vertex The index must be valid or an IndexOutOfRangeException will be thrown.

## MeshVertexColorList (class)

Provides access to the vertex colors of a mesh object.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_MeshVertexColorList.htm)

### Methods
#### `public int Add(Color color)`

Adds a new vertex color to the end of the color list.

**Parameters:**
- `color` (System.Drawing.Color) — Color to append.

**Returns:** `Int32` — The index of the newly added color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexColorList_Add.htm)

#### `public int Add(int red, int green, int blue)`

Adds a new vertex color to the end of the color list.

**Parameters:**
- `red` (System.Int32) — Red component of color, must be in the 0~255 range.
- `green` (System.Int32) — Green component of color, must be in the 0~255 range.
- `blue` (System.Int32) — Blue component of color, must be in the 0~255 range.

**Returns:** `Int32` — The index of the newly added color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexColorList_Add_1.htm)

#### `public bool AddRange(IEnumerable<Color> colors)`

Adds an enumerable of colors to the to the vertex color list. For the Mesh to be valid, the number of colors must match the number of vertices.

**Parameters:**
- `colors` (System.Collections.Generic.IEnumerable<Color>) — Colors to append.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexColorList_AddRange.htm)

#### `public bool AppendColors(Color[] colors)`

Appends a collection of colors to the vertex color list. For the Mesh to be valid, the number of colors must match the number of vertices.

**Parameters:**
- `colors` (System.Drawing.Color[]) — Colors to append.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexColorList_AppendColors.htm)

#### `public void Clear()`

Clears the vertex color list on the mesh.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexColorList_Clear.htm)

#### `public bool CreateMonotoneMesh(Color baseColor)`

Constructs a valid vertex color list consisting of a single color.

**Parameters:**
- `baseColor` (System.Drawing.Color) — Color to apply to every vertex.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexColorList_CreateMonotoneMesh.htm)

#### `public void Destroy()`

Releases all memory allocated to store vertex colors. The list capacity will be 0 after this call. Subsequent calls can add new items.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexColorList_Destroy.htm)

#### `public IEnumerator<Color> GetEnumerator()`

Gets an enumerator that yields all colors in this collection.

**Returns:** `IEnumerator<Color>` — The enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexColorList_GetEnumerator.htm)

#### `public bool SetColor(int index, Color color)`

Sets or adds a vertex to the Vertex List. If [index] is less than [Count], the existing vertex at [index] will be modified.If [index] equals [Count], a new vertex is appended to the end of the vertex list.If [index] is larger than [Count], the function will return false.

**Parameters:**
- `index` (System.Int32) — Index of vertex color to set. If index equals Count, then the color will be appended.
- `color` (System.Drawing.Color) — Color to set, Alpha channels will be ignored.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexColorList_SetColor_1.htm)

#### `public bool SetColor(int index, int red, int green, int blue)`

Sets or adds a vertex color to the color List. If [index] is less than [Count], the existing vertex at [index] will be modified.If [index] equals [Count], a new vertex is appended to the end of the vertex list.If [index] is larger than [Count], the function will return false.

**Parameters:**
- `index` (System.Int32) — Index of vertex color to set. If index equals Count, then the color will be appended.
- `red` (System.Int32) — Red component of vertex color. Value must be in the 0~255 range.
- `green` (System.Int32) — Green component of vertex color. Value must be in the 0~255 range.
- `blue` (System.Int32) — Blue component of vertex color. Value must be in the 0~255 range.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexColorList_SetColor_2.htm)

#### `public bool SetColor(MeshFace face, Color color)`

Sets a color at the three or four vertex indices of a specified face.

**Parameters:**
- `face` (Rhino.Geometry.MeshFace) — A face to use to retrieve indices.
- `color` (System.Drawing.Color) — A color.

**Returns:** `Boolean` — true on success; false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexColorList_SetColor.htm)

#### `public bool SetColors(Color[] colors)`

Sets all the vertex colors in one go. For the Mesh to be valid, the number of colors must match the number of vertices.

**Parameters:**
- `colors` (System.Drawing.Color[]) — Colors to set.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexColorList_SetColors.htm)

#### `public int[] ToARGBArray()`

Return colors as an array of integers with packed ARGB values

**Returns:** `Int32[]` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.MeshVertexColorList.ToARGBArray"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexColorList_ToARGBArray.htm)

### Properties
- `Capacity` (Int32, get/set) — Gets or sets the total number of vertex colors the internal data structure can hold without resizing.
- `Count` (Int32, get/set) — Gets or sets the number of mesh colors.
- `Item` (Color, get/set) — Gets or sets the vertex color at the given index. The index must be valid or an IndexOutOfRangeException will be thrown.
- `Tag` (MappingTag, get/set) — Gets or sets a mapping information for the mesh associated with these vertex colors.

## MeshVertexList (class)

Provides access to the vertices and vertex-related functionality of a mesh.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_MeshVertexList.htm)

### Methods
#### `public int Add(double x, double y, double z)`

Adds a new vertex to the end of the Vertex list.

**Parameters:**
- `x` (System.Double) — X component of new vertex coordinate.
- `y` (System.Double) — Y component of new vertex coordinate.
- `z` (System.Double) — Z component of new vertex coordinate.

**Returns:** `Int32` — The index of the newly added vertex.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_Add_2.htm)

#### `public int Add(Point3d vertex)`

Adds a new vertex to the end of the Vertex list.

**Parameters:**
- `vertex` (Rhino.Geometry.Point3d) — Location of new vertex.

**Returns:** `Int32` — The index of the newly added vertex.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_Add.htm)

#### `public int Add(Point3f vertex)`

Adds a new vertex to the end of the Vertex list.

**Parameters:**
- `vertex` (Rhino.Geometry.Point3f) — Location of new vertex.

**Returns:** `Int32` — The index of the newly added vertex.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_Add_1.htm)

#### `public int Add(float x, float y, float z)`

Adds a new vertex to the end of the Vertex list.

**Parameters:**
- `x` (System.Single) — X component of new vertex coordinate.
- `y` (System.Single) — Y component of new vertex coordinate.
- `z` (System.Single) — Z component of new vertex coordinate.

**Returns:** `Int32` — The index of the newly added vertex.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_Add_3.htm)

#### `public void AddVertices(IEnumerable<Point3d> vertices)`

Adds a series of new vertices to the end of the vertex list. This overload accepts double-precision points.

**Parameters:**
- `vertices` (System.Collections.Generic.IEnumerable<Point3d>) — A list, an array or any enumerable set of Point3d.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_AddVertices.htm)

#### `public void AddVertices(IEnumerable<Point3f> vertices)`

Adds a series of new vertices to the end of the vertex list. This overload accepts single-precision points.

**Parameters:**
- `vertices` (System.Collections.Generic.IEnumerable<Point3f>) — A list, an array or any enumerable set of Point3f.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_AddVertices_1.htm)

#### `public int Align(double distance, IEnumerable<bool> whichVertices = null)`

Moves mesh vertices that belong to naked edges to neighboring vertices, within the specified distance. This forces unaligned mesh vertices to the same location and is helpful to clean meshes for 3D printing.See the Copy_AlignMeshVertices Rhino command for more information.

**Parameters:**
- `distance` (System.Double) — Distance that should not be exceed when modifying the mesh.
- `whichVertices` (System.Collections.Generic.IEnumerable<Boolean>) — If not null, defines which vertices should be considered for adjustment.

**Returns:** `Int32` — If the operation succeeded, the number of moved vertices, or -1 on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_Align_1.htm)

#### `public static int Align(IEnumerable<Mesh> meshes, double distance, IEnumerable<IEnumerable<bool>> whichVertices = null)`

Moves mesh vertices that belong to naked edges to neighboring vertices, within the specified distance. This forces unaligned mesh vertices to the same location and is helpful to clean meshes for 3D printing.See the Copy_AlignMeshVertices Rhino command for more information.

**Parameters:**
- `meshes` (System.Collections.Generic.IEnumerable<Mesh>) — The enumerable of meshes that need to have vertices adjusted.
- `distance` (System.Double) — Distance that should not be exceed when modifying the mesh.
- `whichVertices` (System.Collections.Generic.IEnumerable<IEnumerable<Boolean>>) — If not null, defines which vertices should be considered for adjustment. If this parameter is non-null, then all items within it have to be non-null as well, defining for each mesh, which vertices to adjust.

**Returns:** `Int32` — If the operation succeeded, the number of moved vertices, or -1 on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_Align.htm)

#### `public void Clear()`

Clears the Vertex list on the mesh.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_Clear.htm)

#### `public bool CombineIdentical(bool ignoreNormals, bool ignoreAdditional)`

Merges identical vertices.

**Parameters:**
- `ignoreNormals` (System.Boolean) — If true, vertex normals will not be taken into consideration when comparing vertices.
- `ignoreAdditional` (System.Boolean) — If true, texture coordinates, colors, and principal curvatures will not be taken into consideration when comparing vertices.

**Returns:** `Boolean` — true if the mesh is changed, in which case the mesh will have fewer vertices than before.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_CombineIdentical.htm)

#### `public int CullUnused()`

Removes all vertices that are currently not used by the Face list.

**Returns:** `Int32` — The number of unused vertices that were removed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_CullUnused.htm)

#### `public void Destroy()`

Releases all memory allocated to store faces. The list capacity will be 0 after this call. Subsequent calls can add new items.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_Destroy.htm)

#### `public int[] GetConnectedVertices(int vertexIndex)`

Gets indices of all vertices that form "edges" with a given vertex index.

**Parameters:**
- `vertexIndex` (System.Int32) — The index of a vertex to query.

**Returns:** `Int32[]` — An array of vertex indices that are connected with the specified vertex.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_GetConnectedVertices.htm)

#### `public IEnumerator<Point3f> GetEnumerator()`

Gets an enumerator that yields all mesh vertices (points) in this collection.

**Returns:** `IEnumerator<Point3f>` — The enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_GetEnumerator.htm)

#### `public int[] GetTopologicalIndenticalVertices(int vertexIndex)`

Gets a list of other vertices which are "topologically" identical to this vertex.

**Parameters:**
- `vertexIndex` (System.Int32) — A vertex index in the mesh.

**Returns:** `Int32[]` — Array of indices of vertices that are topologically the same as this vertex. The array includes vertexIndex. Returns null on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_GetTopologicalIndenticalVertices.htm)

#### `public int[] GetVertexFaces(int vertexIndex)`

Gets a list of all of the faces that share a given vertex.

**Parameters:**
- `vertexIndex` (System.Int32) — The index of a vertex in the mesh.

**Returns:** `Int32[]` — An array of indices of faces on success, null on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_GetVertexFaces.htm)

#### `public void Hide(int vertexIndex)`

Hides the vertex at the given index.

**Parameters:**
- `vertexIndex` (System.Int32) — Index of vertex to hide.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_Hide.htm)

#### `public void HideAll()`

Hides all vertices in the mesh.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_HideAll.htm)

#### `public bool IsHidden(int vertexIndex)`

Gets a value indicating whether or not a vertex is hidden.

**Parameters:**
- `vertexIndex` (System.Int32) — Index of vertex to query.

**Returns:** `Boolean` — true if the vertex is hidden, false if it is not.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_IsHidden.htm)

#### `public Point3d Point3dAt(int index)`

Get double precision location at a given index

**Parameters:**
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.Geometry.Collections.MeshVertexList.Point3dAt(System.Int32)"]

**Returns:** `Point3d` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.MeshVertexList.Point3dAt(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_Point3dAt.htm)

#### `public bool Remove(IEnumerable<int> indices, bool shrinkFaces)`

Removes the vertices at the given indices and all faces that reference those vertices.

**Parameters:**
- `indices` (System.Collections.Generic.IEnumerable<Int32>) — Vertex indices to remove.
- `shrinkFaces` (System.Boolean) — If true, quads that reference the deleted vertex will be converted to triangles.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_Remove.htm)

#### `public bool Remove(int index, bool shrinkFaces)`

Removes the vertex at the given index and all faces that reference that index.

**Parameters:**
- `index` (System.Int32) — Index of vertex to remove.
- `shrinkFaces` (System.Boolean) — If true, quads that reference the deleted vertex will be converted to triangles.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_Remove_1.htm)

#### `public bool SetVertex(int index, double x, double y, double z)`

Sets or adds a vertex to the Vertex List. If [index] is less than [Count], the existing vertex at [index] will be modified.If [index] equals [Count], a new vertex is appended to the end of the vertex list.If [index] is larger than [Count], the function will return false.

**Parameters:**
- `index` (System.Int32) — Index of vertex to set.
- `x` (System.Double) — X component of vertex location.
- `y` (System.Double) — Y component of vertex location.
- `z` (System.Double) — Z component of vertex location.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_SetVertex_2.htm)

#### `public bool SetVertex(int index, double x, double y, double z, bool updateNormals)`

Sets or adds a vertex to the Vertex List. If [index] is less than [Count], the existing vertex at [index] will be modified.If [index] equals [Count], a new vertex is appended to the end of the vertex list.If [index] is larger than [Count], the function will return false.

**Parameters:**
- `index` (System.Int32) — Index of vertex to set.
- `x` (System.Double) — X component of vertex location.
- `y` (System.Double) — Y component of vertex location.
- `z` (System.Double) — Z component of vertex location.
- `updateNormals` (System.Boolean) — Set to true if you'd like the vertex and face normals impacted by the change updated.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_SetVertex_3.htm)

#### `public bool SetVertex(int index, Point3d vertex)`

Sets or adds a vertex to the Vertex List. If [index] is less than [Count], the existing vertex at [index] will be modified.If [index] equals [Count], a new vertex is appended to the end of the vertex list.If [index] is larger than [Count], the function will return false.

**Parameters:**
- `index` (System.Int32) — Index of vertex to set.
- `vertex` (Rhino.Geometry.Point3d) — Vertex location.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_SetVertex.htm)

#### `public bool SetVertex(int index, Point3f vertex)`

Sets or adds a vertex to the Vertex List. If [index] is less than [Count], the existing vertex at [index] will be modified.If [index] equals [Count], a new vertex is appended to the end of the vertex list.If [index] is larger than [Count], the function will return false.

**Parameters:**
- `index` (System.Int32) — Index of vertex to set.
- `vertex` (Rhino.Geometry.Point3f) — Vertex location.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_SetVertex_1.htm)

#### `public bool SetVertex(int index, float x, float y, float z)`

Sets or adds a vertex to the Vertex List. If [index] is less than [Count], the existing vertex at [index] will be modified.If [index] equals [Count], a new vertex is appended to the end of the vertex list.If [index] is larger than [Count], the function will return false.

**Parameters:**
- `index` (System.Int32) — Index of vertex to set.
- `x` (System.Single) — X component of vertex location.
- `y` (System.Single) — Y component of vertex location.
- `z` (System.Single) — Z component of vertex location.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_SetVertex_4.htm)

#### `public void Show(int vertexIndex)`

Shows the vertex at the given index.

**Parameters:**
- `vertexIndex` (System.Int32) — Index of vertex to show.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_Show.htm)

#### `public void ShowAll()`

Shows all vertices in the mesh.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_ShowAll.htm)

#### `public float[] ToFloatArray()`

Copies all vertices to a linear array of float in x,y,z order

**Returns:** `Single[]` — The float array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_ToFloatArray.htm)

#### `public Point3d[] ToPoint3dArray()`

Copies all vertices to a new array of Point3d.

**Returns:** `Point3d[]` — A new array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_ToPoint3dArray.htm)

#### `public Point3f[] ToPoint3fArray()`

Copies all vertices to a new array of Point3f.

**Returns:** `Point3f[]` — A new array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexList_ToPoint3fArray.htm)

### Properties
- `Capacity` (Int32, get/set) — Gets or sets the total number of vertices the internal data structure can hold without resizing.
- `Count` (Int32, get/set) — Gets or sets the number of mesh vertices.
- `Item` (Point3f, get/set) — Gets or sets the vertex at the given index. The index must be valid or an IndexOutOfRangeException will be thrown.
- `UseDoublePrecisionVertices` (Boolean, get/set) — Set to true if the vertices should be stored in double precision

## MeshVertexNormalList (class)

Provides access to the Vertex Normals of a Mesh.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_MeshVertexNormalList.htm)

### Methods
#### `public int Add(double x, double y, double z)`

Adds a new vertex normal at the end of the list.

**Parameters:**
- `x` (System.Double) — X component of new vertex normal.
- `y` (System.Double) — Y component of new vertex normal.
- `z` (System.Double) — Z component of new vertex normal.

**Returns:** `Int32` — The index of the newly added vertex normal.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexNormalList_Add_2.htm)

#### `public int Add(float x, float y, float z)`

Adds a new vertex normal at the end of the list.

**Parameters:**
- `x` (System.Single) — X component of new vertex normal.
- `y` (System.Single) — Y component of new vertex normal.
- `z` (System.Single) — Z component of new vertex normal.

**Returns:** `Int32` — The index of the newly added vertex normal.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexNormalList_Add_3.htm)

#### `public int Add(Vector3d normal)`

Adds a new vertex normal at the end of the list.

**Parameters:**
- `normal` (Rhino.Geometry.Vector3d) — new vertex normal.

**Returns:** `Int32` — The index of the newly added vertex normal.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexNormalList_Add.htm)

#### `public int Add(Vector3f normal)`

Adds a new vertex normal at the end of the list.

**Parameters:**
- `normal` (Rhino.Geometry.Vector3f) — new vertex normal.

**Returns:** `Int32` — The index of the newly added vertex normal.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexNormalList_Add_1.htm)

#### `public bool AddRange(Vector3f[] normals)`

Appends a collection of normal vectors.

**Parameters:**
- `normals` (Rhino.Geometry.Vector3f[]) — Normals to append.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexNormalList_AddRange.htm)

#### `public void Clear()`

Clears the vertex normal collection on the mesh.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexNormalList_Clear.htm)

#### `public bool ComputeNormals()`

Computes the vertex normals based on the physical shape of the mesh.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexNormalList_ComputeNormals.htm)

#### `public void Destroy()`

Releases all memory allocated to store vertex normals. The list capacity will be 0 after this call. Subsequent calls can add new items.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexNormalList_Destroy.htm)

#### `public void Flip()`

Reverses direction of all vertex normals This is the same as Mesh.Flip(true, false, false)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexNormalList_Flip.htm)

#### `public IEnumerator<Vector3f> GetEnumerator()`

Gets an enumerator that yields all normals (vectors) in this collection.

**Returns:** `IEnumerator<Vector3f>` — The enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexNormalList_GetEnumerator.htm)

#### `public bool SetNormal(int index, double x, double y, double z)`

Sets or adds a vertex normal to the list. If [index] is less than [Count], the existing vertex normal at [index] will be modified.If [index] equals [Count], a new vertex normal is appended to the end of the list.If [index] is larger than [Count], the function will return false.

**Parameters:**
- `index` (System.Int32) — Index of vertex normal to set.
- `x` (System.Double) — X component of vertex normal.
- `y` (System.Double) — Y component of vertex normal.
- `z` (System.Double) — Z component of vertex normal.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexNormalList_SetNormal_2.htm)

#### `public bool SetNormal(int index, float x, float y, float z)`

Sets or adds a normal to the list. If [index] is less than [Count], the existing vertex normal at [index] will be modified.If [index] equals [Count], a new vertex normal is appended to the end of the list.If [index] is larger than [Count], the function will return false.

**Parameters:**
- `index` (System.Int32) — Index of vertex normal to set.
- `x` (System.Single) — X component of vertex normal.
- `y` (System.Single) — Y component of vertex normal.
- `z` (System.Single) — Z component of vertex normal.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexNormalList_SetNormal_3.htm)

#### `public bool SetNormal(int index, Vector3d normal)`

Sets or adds a vertex normal to the list. If [index] is less than [Count], the existing vertex normal at [index] will be modified.If [index] equals [Count], a new vertex normal is appended to the end of the list.If [index] is larger than [Count], the function will return false.

**Parameters:**
- `index` (System.Int32) — Index of vertex normal to set.
- `normal` (Rhino.Geometry.Vector3d) — The new normal at the index.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexNormalList_SetNormal.htm)

#### `public bool SetNormal(int index, Vector3f normal)`

Sets or adds a vertex normal to the list. If [index] is less than [Count], the existing vertex normal at [index] will be modified.If [index] equals [Count], a new vertex normal is appended to the end of the vertex list.If [index] is larger than [Count], the function will return false.

**Parameters:**
- `index` (System.Int32) — Index of vertex normal to set.
- `normal` (Rhino.Geometry.Vector3f) — The new normal at the index.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexNormalList_SetNormal_1.htm)

#### `public bool SetNormals(Vector3f[] normals)`

Sets all normal vectors in one go. This method destroys the current normal array if it exists.

**Parameters:**
- `normals` (Rhino.Geometry.Vector3f[]) — Normals for the entire mesh.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexNormalList_SetNormals.htm)

#### `public float[] ToFloatArray()`

Copies all vertex normals to a linear array of float in x,y,z order

**Returns:** `Single[]` — The float array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexNormalList_ToFloatArray.htm)

#### `public bool UnitizeNormals()`

Unitizes all vertex normals.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexNormalList_UnitizeNormals.htm)

### Properties
- `Capacity` (Int32, get/set) — Gets or sets the amount of vertex normals that the list can hold without resizing.
- `Count` (Int32, get/set) — Gets or sets the number of mesh vertex normals.
- `Item` (Vector3f, get/set) — Gets or sets the vertex at the given index. The index must be valid or an IndexOutOfRangeException will be thrown.

## MeshVertexStatusList (class)

Provides access to status information relative to components of a mesh.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_MeshVertexStatusList.htm)

### Methods
#### `public void Add(bool hidden)`

Adds a new flag at the end of the list.

**Parameters:**
- `hidden` (System.Boolean) — True if vertex is hidden.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexStatusList_Add.htm)

#### `public void AddRange(IEnumerable<bool> values)`

Appends an array, a list or any enumerable of flags to the end of the list.

**Parameters:**
- `values` (System.Collections.Generic.IEnumerable<Boolean>) — Hidden values to append.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexStatusList_AddRange.htm)

#### `public void Clear()`

Clears the hidden vertex list on the mesh. This results in a fully visible mesh.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexStatusList_Clear.htm)

#### `public bool Contains(bool hidden)`

Determines if some vertices are hidden or some are shown.

**Parameters:**
- `hidden` (System.Boolean) — The value to be checked. True means some vertex is hidden.

**Returns:** `Boolean` — True if the array contains the specified value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexStatusList_Contains.htm)

#### `public void CopyTo(bool[] array, int arrayIndex)`

Copies to an array, starting at an index.

**Parameters:**
- `array` (System.Boolean[]) — The array to be copied into.
- `arrayIndex` (System.Int32) — The starting index in the array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexStatusList_CopyTo.htm)

#### `public void Destroy()`

Releases all memory allocated to store hidden vertices. The list capacity will be 0 after this call. Vertices will be immediately considered visible.Subsequent calls can add new items.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexStatusList_Destroy.htm)

#### `public IEnumerator<bool> GetEnumerator()`

Gets an enumerator that yields all flags in this collection.

**Returns:** `IEnumerator<Boolean>` — The enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_MeshVertexStatusList_GetEnumerator.htm)

### Properties
- `Capacity` (Int32, get/set) — Gets or sets the total number of hidden vertex information the internal data structure can hold without resizing.
- `Count` (Int32, get/set) — Gets or sets the number of hidden vertices. For this to be a valid part of a mesh, this count should be the same as the one of mesh vertices.
- `HiddenCount` (Int32, get) — Gets a value indicating how many vertices have been set to hidden.
- `Item` (Boolean, get/set) — Gets or sets the hidden value at the given index. The index must be valid or an IndexOutOfRangeException will be thrown.

## NurbsCurveKnotList (class)

Provides access to the knot vector of a NURBS curve.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_NurbsCurveKnotList.htm)

### Methods
#### `public bool ClampEnd(CurveEnd end)`

Clamp end knots. Does not modify control point locations.

**Parameters:**
- `end` (Rhino.Geometry.CurveEnd) — Curve end to clamp.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurveKnotList_ClampEnd.htm)

#### `public bool Contains(double item)`

Returns an indication of the presence of a value in the knot list.

**Parameters:**
- `item` (System.Double) — The item.

**Returns:** `Boolean` — true if present, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurveKnotList_Contains.htm)

#### `public void CopyTo(double[] array, int arrayIndex)`

Copies the list to an array.

**Parameters:**
- `array` (System.Double[]) — The array to copy to.
- `arrayIndex` (System.Int32) — The index into copy will begin.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurveKnotList_CopyTo.htm)

#### `public bool CreatePeriodicKnots(double knotSpacing)`

Compute a clamped, uniform, periodic knot vector based on the current degree and control point count. Does not change values of control vertices.

**Parameters:**
- `knotSpacing` (System.Double) — Spacing of subsequent knots.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurveKnotList_CreatePeriodicKnots.htm)

#### `public bool CreateUniformKnots(double knotSpacing)`

Compute a clamped, uniform knot vector based on the current degree and control point count. Does not change values of control vertices.

**Parameters:**
- `knotSpacing` (System.Double) — Spacing of subsequent knots.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurveKnotList_CreateUniformKnots.htm)

#### `public void EnsurePrivateCopy()`

If you want to keep a copy of this class around by holding onto it in a variable after a command completes, call EnsurePrivateCopy to make sure that this class is not tied to the document. You can call this function as many times as you want.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurveKnotList_EnsurePrivateCopy.htm)

#### `public bool EpsilonEquals(NurbsCurveKnotList other, double epsilon)`

Checks that all values in the other list are sequentially equal within epsilon to the values in this list.

**Parameters:**
- `other` (Rhino.Geometry.Collections.NurbsCurveKnotList) — The other list.
- `epsilon` (System.Double) — The epsilon value.

**Returns:** `Boolean` — True if values are, orderly, equal within epsilon. False otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurveKnotList_EpsilonEquals.htm)

#### `public int IndexOf(double item)`

Returns the first item in the list.

**Parameters:**
- `item` (System.Double) — The value.

**Returns:** `Int32` — The index, or -1 if no index is found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurveKnotList_IndexOf.htm)

#### `public bool InsertKnot(double value)`

Inserts a knot and update control point locations. Does not change parameterization or locus of curve.

**Parameters:**
- `value` (System.Double) — Knot value to insert.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurveKnotList_InsertKnot.htm)

#### `public bool InsertKnot(double value, int multiplicity)`

Inserts a knot and update control point locations. Does not change parameterization or locus of curve.

**Parameters:**
- `value` (System.Double) — Knot value to insert.
- `multiplicity` (System.Int32) — Multiplicity of knot to insert.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurveKnotList_InsertKnot_1.htm)

#### `public int KnotMultiplicity(int index)`

Get knot multiplicity.

**Parameters:**
- `index` (System.Int32) — Index of knot to query.

**Returns:** `Int32` — The multiplicity (valence) of the knot.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurveKnotList_KnotMultiplicity.htm)

#### `public bool RemoveKnotAt(double t)`

Remove a knot from a curve and adjusts the remaining control points to maintain curve position as closely as possible.

**Parameters:**
- `t` (System.Double) — The parameter on the curve that is closest to the knot to be removed.

**Returns:** `Boolean` — true if successful, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurveKnotList_RemoveKnotAt.htm)

#### `public bool RemoveKnots(int index0, int index1)`

Remove knots from a curve and adjusts the remaining control points to maintain curve position as closely as possible. The knots from Knots[index0] through Knots[index1 - 1] will be removed.

**Parameters:**
- `index0` (System.Int32) — The starting knot index, where Degree-1 < index0 < index1 <= Points.Count-1.
- `index1` (System.Int32) — The ending knot index, where Degree-1 < index0 < index1 <= Points.Count-1.

**Returns:** `Boolean` — true if successful, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurveKnotList_RemoveKnots.htm)

#### `public int RemoveMultipleKnots(int minimumMultiplicity, int maximumMultiplicity, double tolerance)`

Remove multiple knots from this curve.

**Parameters:**
- `minimumMultiplicity` (System.Int32) — Remove knots with multiplicity > minimumKnotMultiplicity.
- `maximumMultiplicity` (System.Int32) — Remove knots with multiplicity < maximumKnotMultiplicity.
- `tolerance` (System.Double) — When you remove knots, the shape of the curve is changed. If tolerance is RhinoMath.UnsetValue, any amount of change is permitted. If tolerance is >=0, the maximum distance between the input and output curve is restricted to be <= tolerance.

**Returns:** `Int32` — number of knots removed on success. 0 if no knots were removed

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurveKnotList_RemoveMultipleKnots.htm)

#### `public double SuperfluousKnot(bool start)`

Computes the knots that are superfluous because they are not used in NURBs evaluation. These make it appear so that the first and last curve spans are different from interior spans. http://wiki.mcneel.com/developer/onsuperfluousknot

**Parameters:**
- `start` (System.Boolean) — true if the query targets the first knot. Otherwise, the last knot.

**Returns:** `Double` — A component.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurveKnotList_SuperfluousKnot.htm)

### Properties
- `Count` (Int32, get) — Total number of knots in this curve.
- `IsClampedEnd` (Boolean, get) — Gets a value indicating whether or not the knot vector is clamped at the end of the curve. Clamped curves are coincident with the first and last control-point. This requires fully multiple knots.
- `IsClampedStart` (Boolean, get) — Gets a value indicating whether or not the knot vector is clamped at the start of the curve. Clamped curves start at the first control-point. This requires fully multiple knots.
- `Item` (Double, get/set) — Gets or sets the knot vector value at the given index.
- `KnotStyle` (KnotStyle, get) — Gets the style of the knot vector.

## NurbsCurvePointList (class)

Provides access to the control points of a NURBS curve.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_NurbsCurvePointList.htm)

### Methods
#### `public bool ChangeEndWeights(double w0, double w1)`

Use a combination of scaling and reparameterization to change the end weights to the specified values.

**Remarks:** The domain, Euclidean locations of the control points, and locus of the curve do not change, but the weights, homogeneous CV values and internal knot values may change. If w0 and w1 are 1 and the curve is not rational, the curve is not changed.

**Parameters:**
- `w0` (System.Double) — Weight for first control point.
- `w1` (System.Double) — Weight for last control point.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurvePointList_ChangeEndWeights.htm)

#### `public bool Contains(ControlPoint item)`

Determines if this list contains an item.

**Parameters:**
- `item` (Rhino.Geometry.ControlPoint) — The exact item to search for.

**Returns:** `Boolean` — A boolean value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurvePointList_Contains.htm)

#### `public Polyline ControlPolygon()`

Constructs a polyline through all the control points. Note that periodic curves generate a closed polyline with fewer points than control-points.

**Returns:** `Polyline` — A polyline connecting all control points.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurvePointList_ControlPolygon.htm)

#### `public void CopyTo(ControlPoint[] array, int arrayIndex)`

Copied the list to an array.

**Parameters:**
- `array` (Rhino.Geometry.ControlPoint[]) — The array to copy to.
- `arrayIndex` (System.Int32) — The index in which the copy will begin.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurvePointList_CopyTo.htm)

#### `public void EnsurePrivateCopy()`

If you want to keep a copy of this class around by holding onto it in a variable after a command completes, call EnsurePrivateCopy to make sure that this class is not tied to the document. You can call this function as many times as you want.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurvePointList_EnsurePrivateCopy.htm)

#### `public bool EpsilonEquals(NurbsCurvePointList other, double epsilon)`

Check that all values in other are within epsilon of the values in this

**Parameters:**
- `other` (Rhino.Geometry.Collections.NurbsCurvePointList) — [Missing <param name="other"/> documentation for "M:Rhino.Geometry.Collections.NurbsCurvePointList.EpsilonEquals(Rhino.Geometry.Collections.NurbsCurvePointList,System.Double)"]
- `epsilon` (System.Double) — [Missing <param name="epsilon"/> documentation for "M:Rhino.Geometry.Collections.NurbsCurvePointList.EpsilonEquals(Rhino.Geometry.Collections.NurbsCurvePointList,System.Double)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.NurbsCurvePointList.EpsilonEquals(Rhino.Geometry.Collections.NurbsCurvePointList,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurvePointList_EpsilonEquals.htm)

#### `public bool GetPoint(int index, out Point3d point)`

Gets a world 3-D, or Euclidean, control point at the given index. The 4-D representation is (x, y, z, 1.0).

**Parameters:**
- `index` (System.Int32) — Index of control point to get.
- `point` (Rhino.Geometry.Point3d) — Coordinate of control point.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurvePointList_GetPoint.htm)

#### `public bool GetPoint(int index, out Point4d point)`

Gets a homogeneous control point at the given index, where the 4-D representation is (x, y, z, w). The world 3-D, or Euclidean, representation is (x/w, y/w, z/w).

**Remarks:** For expert use only. If you do not understand homogeneous coordinates, then use the override that returns world 3-D, or Euclidean, coordinates.

**Parameters:**
- `index` (System.Int32) — Index of control point to get.
- `point` (Rhino.Geometry.Point4d) — Coordinate and weight of control point.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurvePointList_GetPoint_1.htm)

#### `public double GetWeight(int index)`

Gets the weight of a control point at the given index. Note, if the curve is non-rational, the weight will be 1.0.

**Parameters:**
- `index` (System.Int32) — Index of control point to get.

**Returns:** `Double` — The control point weight if successful, Rhino.Math.UnsetValue otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurvePointList_GetWeight.htm)

#### `public int IndexOf(ControlPoint item)`

Gets the index of a control point, or -1.

**Parameters:**
- `item` (Rhino.Geometry.ControlPoint) — The exact item to search for.

**Returns:** `Int32` — The index.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurvePointList_IndexOf.htm)

#### `public bool MakeNonRational()`

Converts the curve to a Non-rational NURBS curve. Non-rational curves have unweighted control points.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurvePointList_MakeNonRational.htm)

#### `public bool MakeRational()`

Converts the curve to a Rational NURBS curve. Rational NURBS curves have weighted control points.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurvePointList_MakeRational.htm)

#### `public bool SetPoint(int index, double x, double y, double z)`

Sets a world 3-D, or Euclidean, control point at the given index. The 4-D representation is (x, y, z, 1.0).

**Parameters:**
- `index` (System.Int32) — Index of control point to set.
- `x` (System.Double) — X coordinate of control point.
- `y` (System.Double) — Y coordinate of control point.
- `z` (System.Double) — Z coordinate of control point.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurvePointList_SetPoint_3.htm)

#### `public bool SetPoint(int index, double x, double y, double z, double weight)`

Sets a homogeneous control point at the given index, where the 4-D representation is (x, y, z, w). The world 3-D, or Euclidean, representation is (x/w, y/w, z/w).

**Remarks:** For expert use only. If you do not understand homogeneous coordinates, then use an override that accepts world 3-D, or Euclidean, coordinates as input.

**Parameters:**
- `index` (System.Int32) — Index of control point to set.
- `x` (System.Double) — X coordinate of control point.
- `y` (System.Double) — Y coordinate of control point.
- `z` (System.Double) — Z coordinate of control point.
- `weight` (System.Double) — Weight of control point.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurvePointList_SetPoint_4.htm)

#### `public bool SetPoint(int index, Point3d point)`

Sets a world 3-D, or Euclidean, control point at the given index. The 4-D representation is (x, y, z, 1.0).

**Parameters:**
- `index` (System.Int32) — Index of control point to set.
- `point` (Rhino.Geometry.Point3d) — Coordinate of control point.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurvePointList_SetPoint.htm)

#### `public bool SetPoint(int index, Point3d point, double weight)`

Sets a world 3-D, or Euclidean, control point and weight at a given index. The 4-D representation is (x*w, y*w, z*w, w).

**Parameters:**
- `index` (System.Int32) — Index of control point to set.
- `point` (Rhino.Geometry.Point3d) — Coordinates of the control point.
- `weight` (System.Double) — Weight of control point.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurvePointList_SetPoint_1.htm)

#### `public bool SetPoint(int index, Point4d point)`

Sets a homogeneous control point at the given index, where the 4-D representation is (x, y, z, w). The world 3-D, or Euclidean, representation is (x/w, y/w, z/w).

**Remarks:** For expert use only. If you do not understand homogeneous coordinates, then use an override that accepts world 3-D, or Euclidean, coordinates as input.

**Parameters:**
- `index` (System.Int32) — Index of control point to set.
- `point` (Rhino.Geometry.Point4d) — Coordinate and weight of control point.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurvePointList_SetPoint_2.htm)

#### `public bool SetWeight(int index, double weight)`

Sets the weight of a control point at the given index Note, if the curve is non-rational, it will be converted to rational.

**Parameters:**
- `index` (System.Int32) — Index of control point to set.
- `weight` (System.Double) — The control point weight.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurvePointList_SetWeight.htm)

#### `public bool UVNDirectionsAt(int index, out Vector3d uDir, out Vector3d vDir, out Vector3d nDir)`

Calculates the U, V, and N directions of a NURBS curve control point similar to the method used by Rhino's MoveUVN command.

**Parameters:**
- `index` (System.Int32) — Index of control point.
- `uDir` (Rhino.Geometry.Vector3d) — The U direction.
- `vDir` (Rhino.Geometry.Vector3d) — The V direction.
- `nDir` (Rhino.Geometry.Vector3d) — The N direction.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurvePointList_UVNDirectionsAt.htm)

#### `public bool ValidateSpacing(double closeTolerance, double stackTolerance, out int[] closeIndices, out int[] stackedIndices)`

Simple check of distance between adjacent control points

**Parameters:**
- `closeTolerance` (System.Double) — tolerance to use for determining if control points are 'close'
- `stackTolerance` (System.Double) — tolerance to use for determining if control points are 'stacked'
- `closeIndices` (System.Int32[]) — indices of 'close' points are returned in this array
- `stackedIndices` (System.Int32[]) — indices of 'stacked' points are returned in this array

**Returns:** `Boolean` — true if close or stacked indices are found

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsCurvePointList_ValidateSpacing.htm)

### Properties
- `ControlPolygonLength` (Double, get) — Gets the length of the polyline connecting all control points.
- `Count` (Int32, get) — Gets the number of control points in this curve.
- `Item` (ControlPoint, get/set) — Gets or sets the control point location at the given index.
- `PointSize` (Int32, get) — Returns the control point size, or the number of doubles per control point. For rational curves, PointSize = Curve.Dimension + 1. For non-rational curves, PointSize = Curve.Dimension.

## NurbsSurfaceKnotList (class)

Provides access to the knot vector of a NURBS surface.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_NurbsSurfaceKnotList.htm)

### Methods
#### `public bool CreatePeriodicKnots(double knotSpacing)`

Compute a clamped, uniform, periodic knot vector based on the current degree and control point count. Does not change values of control vertices.

**Parameters:**
- `knotSpacing` (System.Double) — Spacing of subsequent knots.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfaceKnotList_CreatePeriodicKnots.htm)

#### `public bool CreateUniformKnots(double knotSpacing)`

Compute a clamped, uniform knot vector based on the current degree and control point count. Does not change values of control vertices.

**Parameters:**
- `knotSpacing` (System.Double) — Spacing of subsequent knots.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfaceKnotList_CreateUniformKnots.htm)

#### `public void EnsurePrivateCopy()`

If you want to keep a copy of this class around by holding onto it in a variable after a command completes, call EnsurePrivateCopy to make sure that this class is not tied to the document. You can call this function as many times as you want.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfaceKnotList_EnsurePrivateCopy.htm)

#### `public bool EpsilonEquals(NurbsSurfaceKnotList other, double epsilon)`

Check that all values in other are within epsilon of the values in this

**Parameters:**
- `other` (Rhino.Geometry.Collections.NurbsSurfaceKnotList) — [Missing <param name="other"/> documentation for "M:Rhino.Geometry.Collections.NurbsSurfaceKnotList.EpsilonEquals(Rhino.Geometry.Collections.NurbsSurfaceKnotList,System.Double)"]
- `epsilon` (System.Double) — [Missing <param name="epsilon"/> documentation for "M:Rhino.Geometry.Collections.NurbsSurfaceKnotList.EpsilonEquals(Rhino.Geometry.Collections.NurbsSurfaceKnotList,System.Double)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.NurbsSurfaceKnotList.EpsilonEquals(Rhino.Geometry.Collections.NurbsSurfaceKnotList,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfaceKnotList_EpsilonEquals.htm)

#### `public bool InsertKnot(double value)`

Inserts a knot and update control point locations. Does not change parameterization or locus of curve.

**Parameters:**
- `value` (System.Double) — Knot value to insert.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfaceKnotList_InsertKnot.htm)

#### `public bool InsertKnot(double value, int multiplicity)`

Inserts a knot and update control point locations. Does not change parameterization or locus of curve.

**Parameters:**
- `value` (System.Double) — Knot value to insert.
- `multiplicity` (System.Int32) — Multiplicity of knot to insert.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfaceKnotList_InsertKnot_1.htm)

#### `public int KnotMultiplicity(int index)`

Get knot multiplicity.

**Parameters:**
- `index` (System.Int32) — Index of knot to query.

**Returns:** `Int32` — The multiplicity (valence) of the knot.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfaceKnotList_KnotMultiplicity.htm)

#### `public bool RemoveKnots(int index0, int index1)`

Remove knots from the knot vector and adjusts the remaining control points to maintain surface position as closely as possible. The knots from Knots[index0] through Knots[index1 - 1] will be removed.

**Parameters:**
- `index0` (System.Int32) — The starting knot index, where Degree-1 < index0 < index1 <= Points.Count-1.
- `index1` (System.Int32) — The ending knot index, where Degree-1 < index0 < index1 <= Points.Count-1.

**Returns:** `Boolean` — true if successful, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfaceKnotList_RemoveKnots.htm)

#### `public bool RemoveKnotsAt(double u, double v)`

Remove knots from the surface and adjusts the remaining control points to maintain surface position as closely as possible.

**Parameters:**
- `u` (System.Double) — The u parameter on the surface that is closest to the knot to be removed.
- `v` (System.Double) — The v parameter on the surface that is closest to the knot to be removed.

**Returns:** `Boolean` — true if successful, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfaceKnotList_RemoveKnotsAt.htm)

#### `public int RemoveMultipleKnots(int minimumMultiplicity, int maximumMultiplicity, double tolerance)`

Remove multiple knots from this surface

**Parameters:**
- `minimumMultiplicity` (System.Int32) — Remove knots with multiplicity > minimumKnotMultiplicity
- `maximumMultiplicity` (System.Int32) — Remove knots with multiplicity < maximumKnotMultiplicity
- `tolerance` (System.Double) — When you remove knots, the shape of the surface is changed. If tolerance is RhinoMath.UnsetValue, any amount of change is permitted. If tolerance is >=0, the maximum distance between the input and output surface is restricted to be <= tolerance.

**Returns:** `Int32` — number of knots removed on success. 0 if no knots were removed

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfaceKnotList_RemoveMultipleKnots.htm)

#### `public double SuperfluousKnot(bool start)`

Computes the knots that are superfluous because they are not used in NURBs evaluation. These make it appear so that the first and last surface spans are different from interior spans. http://wiki.mcneel.com/developer/onsuperfluousknot

**Parameters:**
- `start` (System.Boolean) — true if the query targets the first knot. Otherwise, the last knot.

**Returns:** `Double` — A component.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfaceKnotList_SuperfluousKnot.htm)

### Properties
- `ClampedAtEnd` (Boolean, get) — Determines if a knot vector is clamped.
- `ClampedAtStart` (Boolean, get) — Determines if a knot vector is clamped.
- `Count` (Int32, get) — Gets the total number of knots in this curve.
- `Item` (Double, get/set) — Gets or sets the knot vector value at the given index.
- `KnotStyle` (KnotStyle, get) — Gets the style of the knot vector.

## NurbsSurfacePointList (class)

Provides access to the control points of a NURBS surface.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_NurbsSurfacePointList.htm)

### Methods
#### `public void EnsurePrivateCopy()`

If you want to keep a copy of this class around by holding onto it in a variable after a command completes, call EnsurePrivateCopy to make sure that this class is not tied to the document. You can call this function as many times as you want.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfacePointList_EnsurePrivateCopy.htm)

#### `public bool EpsilonEquals(NurbsSurfacePointList other, double epsilon)`

Check that all values in other are within epsilon of the values in this

**Parameters:**
- `other` (Rhino.Geometry.Collections.NurbsSurfacePointList) — [Missing <param name="other"/> documentation for "M:Rhino.Geometry.Collections.NurbsSurfacePointList.EpsilonEquals(Rhino.Geometry.Collections.NurbsSurfacePointList,System.Double)"]
- `epsilon` (System.Double) — [Missing <param name="epsilon"/> documentation for "M:Rhino.Geometry.Collections.NurbsSurfacePointList.EpsilonEquals(Rhino.Geometry.Collections.NurbsSurfacePointList,System.Double)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.NurbsSurfacePointList.EpsilonEquals(Rhino.Geometry.Collections.NurbsSurfacePointList,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfacePointList_EpsilonEquals.htm)

#### `public ControlPoint GetControlPoint(int u, int v)`

Gets the control point at the given (u, v) index.

**Parameters:**
- `u` (System.Int32) — Index of control point in the surface U direction.
- `v` (System.Int32) — Index of control point in the surface V direction.

**Returns:** `ControlPoint` — The control point at the given (u, v) index.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfacePointList_GetControlPoint.htm)

#### `public Point2d GetGrevillePoint(int u, int v)`

Gets the 2-D Greville point associated with the control point at the given (u, v) index.

**Parameters:**
- `u` (System.Int32) — Index of control point in the surface U direction.
- `v` (System.Int32) — Index of control point in the surface V direction.

**Returns:** `Point2d` — A Surface UV coordinate on success, Point2d.Unset on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfacePointList_GetGrevillePoint.htm)

#### `public bool GetPoint(int u, int v, out Point3d point)`

Gets a world 3-D, or Euclidean, control point at the given (u, v) index. The 4-D representation is (x, y, z, 1.0).

**Parameters:**
- `u` (System.Int32) — Index of control point in the surface U direction.
- `v` (System.Int32) — Index of control point in the surface V direction.
- `point` (Rhino.Geometry.Point3d) — Coordinate of control point.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfacePointList_GetPoint.htm)

#### `public bool GetPoint(int u, int v, out Point4d point)`

Gets a homogeneous control point at the given (u, v) index, where the 4-D representation is (x, y, z, w). The world 3-D, or Euclidean, representation is (x/w, y/w, z/w).

**Remarks:** For expert use only. If you do not understand homogeneous coordinates, then use the override that returns world 3-D, or Euclidean, coordinates.

**Parameters:**
- `u` (System.Int32) — Index of control point in the surface U direction.
- `v` (System.Int32) — Index of control point in the surface V direction.
- `point` (Rhino.Geometry.Point4d) — Coordinate and weight of control point.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfacePointList_GetPoint_1.htm)

#### `public double GetWeight(int u, int v)`

Gets the weight of a control point at the given (u, v) index. Note, if the surface is non-rational, the weight will be 1.0.

**Parameters:**
- `u` (System.Int32) — Index of control-point along surface U direction.
- `v` (System.Int32) — Index of control-point along surface V direction.

**Returns:** `Double` — The control point weight if successful, Rhino.Math.UnsetValue otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfacePointList_GetWeight.htm)

#### `public bool SetControlPoint(int u, int v, ControlPoint cp)`

Sets the control point at the given (u, v) index.

**Parameters:**
- `u` (System.Int32) — Index of control point in the surface U direction.
- `v` (System.Int32) — Index of control point in the surface V direction.
- `cp` (Rhino.Geometry.ControlPoint) — The control point to set.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfacePointList_SetControlPoint.htm)

#### `[ObsoleteAttribute("Use one of the SetPoint() overrides")] public bool SetControlPoint(int u, int v, Point3d cp)`

Sets the control point at the given (u, v) index.

**Parameters:**
- `u` (System.Int32) — Index of control point in the surface U direction.
- `v` (System.Int32) — Index of control point in the surface V direction.
- `cp` (Rhino.Geometry.Point3d) — The control point location to set (weight is assumed to be 1.0).

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfacePointList_SetControlPoint_1.htm)

#### `public bool SetPoint(int u, int v, double x, double y, double z)`

Sets a world 3-D, or Euclidean, control point at the given (u, v) index. The 4-D representation is (x, y, z, 1.0).

**Parameters:**
- `u` (System.Int32) — Index of control point in the surface U direction.
- `v` (System.Int32) — Index of control point in the surface V direction.
- `x` (System.Double) — X coordinate of control point.
- `y` (System.Double) — Y coordinate of control point.
- `z` (System.Double) — Z coordinate of control point.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfacePointList_SetPoint_3.htm)

#### `public bool SetPoint(int u, int v, double x, double y, double z, double weight)`

Sets a homogeneous control point at the given (u, v) index, where the 4-D representation is (x, y, z, w). The world 3-D, or Euclidean, representation is (x/w, y/w, z/w).

**Remarks:** For expert use only. If you do not understand homogeneous coordinates, then use an override that accepts world 3-D, or Euclidean, coordinates as input.

**Parameters:**
- `u` (System.Int32) — Index of control point in the surface U direction.
- `v` (System.Int32) — Index of control point in the surface V direction.
- `x` (System.Double) — X coordinate of control point.
- `y` (System.Double) — Y coordinate of control point.
- `z` (System.Double) — Z coordinate of control point.
- `weight` (System.Double) — Weight of control point.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfacePointList_SetPoint_4.htm)

#### `public bool SetPoint(int u, int v, Point3d point)`

Sets a world 3-D, or Euclidean, control point at the given (u, v) index. The 4-D representation is (x, y, z, 1.0).

**Parameters:**
- `u` (System.Int32) — Index of control point in the surface U direction.
- `v` (System.Int32) — Index of control point in the surface V direction.
- `point` (Rhino.Geometry.Point3d) — Coordinate of control point.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfacePointList_SetPoint.htm)

#### `public bool SetPoint(int u, int v, Point3d point, double weight)`

Sets a world 3-D, or Euclidean, control point and weight at a given index. The 4-D representation is (x*w, y*w, z*w, w).

**Parameters:**
- `u` (System.Int32) — Index of control point in the surface U direction.
- `v` (System.Int32) — Index of control point in the surface V direction.
- `point` (Rhino.Geometry.Point3d) — Coordinates of the control point.
- `weight` (System.Double) — Weight of control point.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfacePointList_SetPoint_1.htm)

#### `public bool SetPoint(int u, int v, Point4d point)`

Sets a homogeneous control point at the given (u, v) index, where the 4-D representation is (x, y, z, w). The world 3-D, or Euclidean, representation is (x/w, y/w, z/w).

**Remarks:** For expert use only. If you do not understand homogeneous coordinates, then use an override that accepts world 3-D, or Euclidean, coordinates as input.

**Parameters:**
- `u` (System.Int32) — Index of control point in the surface U direction.
- `v` (System.Int32) — Index of control point in the surface V direction.
- `point` (Rhino.Geometry.Point4d) — Coordinate and weight of control point.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfacePointList_SetPoint_2.htm)

#### `public bool SetWeight(int u, int v, double weight)`

Sets the weight of a control point at the given (u, v) index. Note, if the surface is non-rational, it will be converted to rational.

**Parameters:**
- `u` (System.Int32) — Index of control-point along surface U direction.
- `v` (System.Int32) — Index of control-point along surface V direction.
- `weight` (System.Double) — The control point weight.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfacePointList_SetWeight.htm)

#### `public bool UVNDirectionsAt(int u, int v, out Vector3d uDir, out Vector3d vDir, out Vector3d nDir)`

Calculates the U, V, and N directions of a NURBS surface control point similar to the method used by Rhino's MoveUVN command.

**Parameters:**
- `u` (System.Int32) — Index of control-point along surface U direction.
- `v` (System.Int32) — Index of control-point along surface V direction.
- `uDir` (Rhino.Geometry.Vector3d) — The U direction.
- `vDir` (Rhino.Geometry.Vector3d) — The V direction.
- `nDir` (Rhino.Geometry.Vector3d) — The N direction.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfacePointList_UVNDirectionsAt.htm)

#### `public bool ValidateSpacing(double closeTolerance, double stackTolerance, out IndexPair[] closeIndices, out IndexPair[] stackedIndices)`

Simple check of distance between adjacent control points

**Parameters:**
- `closeTolerance` (System.Double) — [Missing <param name="closeTolerance"/> documentation for "M:Rhino.Geometry.Collections.NurbsSurfacePointList.ValidateSpacing(System.Double,System.Double,Rhino.IndexPair[]@,Rhino.IndexPair[]@)"]
- `stackTolerance` (System.Double) — [Missing <param name="stackTolerance"/> documentation for "M:Rhino.Geometry.Collections.NurbsSurfacePointList.ValidateSpacing(System.Double,System.Double,Rhino.IndexPair[]@,Rhino.IndexPair[]@)"]
- `closeIndices` (Rhino.IndexPair[]) — [Missing <param name="closeIndices"/> documentation for "M:Rhino.Geometry.Collections.NurbsSurfacePointList.ValidateSpacing(System.Double,System.Double,Rhino.IndexPair[]@,Rhino.IndexPair[]@)"]
- `stackedIndices` (Rhino.IndexPair[]) — [Missing <param name="stackedIndices"/> documentation for "M:Rhino.Geometry.Collections.NurbsSurfacePointList.ValidateSpacing(System.Double,System.Double,Rhino.IndexPair[]@,Rhino.IndexPair[]@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.NurbsSurfacePointList.ValidateSpacing(System.Double,System.Double,Rhino.IndexPair[]@,Rhino.IndexPair[]@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_NurbsSurfacePointList_ValidateSpacing.htm)

### Properties
- `CountU` (Int32, get) — Gets the number of control points in the U direction of this surface.
- `CountV` (Int32, get) — Gets the number of control points in the V direction of this surface.
- `PointSize` (Int32, get) — Returns the control point size, or the number of doubles per control point. For rational curves, PointSize = Surface.Dimension + 1. For non-rational curves, PointSize = Surface.Dimension.

## SubDEdgeList (class)

All edges in a SubD

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_SubDEdgeList.htm)

### Methods
#### `public SubDEdge Add(SubDEdgeTag tag, SubDVertex v0, SubDVertex v1)`

Add a new edge to the list.

**Parameters:**
- `tag` (Rhino.Geometry.SubDEdgeTag) — The type of edge tag, such as smooth or corner.
- `v0` (Rhino.Geometry.SubDVertex) — First vertex.
- `v1` (Rhino.Geometry.SubDVertex) — Second vertex.

**Returns:** `SubDEdge` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.SubDEdgeList.Add(Rhino.Geometry.SubDEdgeTag,Rhino.Geometry.SubDVertex,Rhino.Geometry.SubDVertex)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_SubDEdgeList_Add.htm)

#### `public SubDEdge Find(int id)`

Find an edge in this SubD with a given id

**Parameters:**
- `id` (System.Int32) — [Missing <param name="id"/> documentation for "M:Rhino.Geometry.Collections.SubDEdgeList.Find(System.Int32)"]

**Returns:** `SubDEdge` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.SubDEdgeList.Find(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_SubDEdgeList_Find.htm)

#### `public SubDEdge Find(uint id)`

Find an edge in this SubD with a given id

**Parameters:**
- `id` (System.UInt32) — [Missing <param name="id"/> documentation for "M:Rhino.Geometry.Collections.SubDEdgeList.Find(System.UInt32)"]

**Returns:** `SubDEdge` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.SubDEdgeList.Find(System.UInt32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_SubDEdgeList_Find_1.htm)

#### `public IEnumerator<SubDEdge> GetEnumerator()`

Implementation of IEnumerable

**Returns:** `IEnumerator<SubDEdge>` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.SubDEdgeList.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_SubDEdgeList_GetEnumerator.htm)

#### `public void SetEdgeTags(IEnumerable<int> edgeIndices, SubDEdgeTag tag)`

Set edge tags for a list of edges. Useful for adding creases to SubDs

**Parameters:**
- `edgeIndices` (System.Collections.Generic.IEnumerable<Int32>) — list of indices for the edges to set tags on
- `tag` (Rhino.Geometry.SubDEdgeTag) — The type of edge tag

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_SubDEdgeList_SetEdgeTags_1.htm)

#### `public void SetEdgeTags(IEnumerable<SubDEdge> edges, SubDEdgeTag tag)`

Set edge tags for a list of edges. Useful for adding creases to SubDs

**Parameters:**
- `edges` (System.Collections.Generic.IEnumerable<SubDEdge>) — list of edges to set a specific tag on
- `tag` (Rhino.Geometry.SubDEdgeTag) — The type of edge tag

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_SubDEdgeList_SetEdgeTags.htm)

### Properties
- `Count` (Int32, get) — Gets the number of SubD edges.

## SubDFaceList (class)

All faces in a SubD

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_SubDFaceList.htm)

### Methods
#### `public uint ClearPerFaceColors()`

Removes all per face color overrides on the active level.

**Remarks:** Per face colors are a mutable property on SubDFace and are set with PerFaceColor.

**Returns:** `UInt32` — Number of changed faces.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_SubDFaceList_ClearPerFaceColors.htm)

#### `public SubDFace Find(int id)`

Find a face in this SubD with a given id

**Parameters:**
- `id` (System.Int32) — [Missing <param name="id"/> documentation for "M:Rhino.Geometry.Collections.SubDFaceList.Find(System.Int32)"]

**Returns:** `SubDFace` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.SubDFaceList.Find(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_SubDFaceList_Find.htm)

#### `public SubDFace Find(uint id)`

Find a face in this SubD with a given id

**Parameters:**
- `id` (System.UInt32) — [Missing <param name="id"/> documentation for "M:Rhino.Geometry.Collections.SubDFaceList.Find(System.UInt32)"]

**Returns:** `SubDFace` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.SubDFaceList.Find(System.UInt32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_SubDFaceList_Find_1.htm)

#### `public IEnumerator<SubDFace> GetEnumerator()`

Implementation of IEnumerable

**Returns:** `IEnumerator<SubDFace>` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.SubDFaceList.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_SubDFaceList_GetEnumerator.htm)

### Properties
- `Count` (Int32, get) — Gets the number of SubD faces.
- `HasPerFaceColors` (Boolean, get) — True if one or more faces on the active level have per face color overrides.

## SubDVertexList (class)

Provides access to all the vertices and vertex-related functionality of a SubD

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_Collections_SubDVertexList.htm)

### Methods
#### `public SubDVertex Add(SubDVertexTag tag, Point3d vertex)`

Add a new vertex to the end of the Vertex list.

**Parameters:**
- `tag` (Rhino.Geometry.SubDVertexTag) — The type of vertex tag, such as smooth or corner.
- `vertex` (Rhino.Geometry.Point3d) — Location of new vertex.

**Returns:** `SubDVertex` — The newly added vertex.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_SubDVertexList_Add.htm)

#### `public SubDVertex Find(int id)`

Find a vertex in this SubD with a given id

**Parameters:**
- `id` (System.Int32) — [Missing <param name="id"/> documentation for "M:Rhino.Geometry.Collections.SubDVertexList.Find(System.Int32)"]

**Returns:** `SubDVertex` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.SubDVertexList.Find(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_SubDVertexList_Find.htm)

#### `public SubDVertex Find(uint id)`

Find a vertex in this SubD with a given id

**Parameters:**
- `id` (System.UInt32) — [Missing <param name="id"/> documentation for "M:Rhino.Geometry.Collections.SubDVertexList.Find(System.UInt32)"]

**Returns:** `SubDVertex` — [Missing <returns> documentation for "M:Rhino.Geometry.Collections.SubDVertexList.Find(System.UInt32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_SubDVertexList_Find_1.htm)

#### `public void SetVertexTags(IEnumerable<int> vertexIndices, SubDVertexTag tag)`

Set vertex tags for a list of vertices. Useful for adding creases to SubDs

**Parameters:**
- `vertexIndices` (System.Collections.Generic.IEnumerable<Int32>) — list of indices for the vertices to set tags on
- `tag` (Rhino.Geometry.SubDVertexTag) — The type of vertex tag

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_SubDVertexList_SetVertexTags_1.htm)

#### `public void SetVertexTags(IEnumerable<SubDVertex> vertices, SubDVertexTag tag)`

Set vertex tags for a list of vertices. Useful for adding creases to SubDs

**Parameters:**
- `vertices` (System.Collections.Generic.IEnumerable<SubDVertex>) — list of vertices to set a specific tag on
- `tag` (Rhino.Geometry.SubDVertexTag) — The type of vertex tag

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Geometry_Collections_SubDVertexList_SetVertexTags.htm)

### Properties
- `Count` (Int32, get) — Gets the number of SubD vertices.
- `First` (SubDVertex, get) — First vertex in this linked list of vertices

