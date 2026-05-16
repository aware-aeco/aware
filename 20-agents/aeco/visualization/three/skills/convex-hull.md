---
name: three-convex-hull
description: ConvexHull declarations from three
---

# ConvexHull

## Methods

- `addAdjoiningFace(eyeVertex: VertexNode, horizonEdge: HalfEdge)`
- `addNewFaces(eyeVertex: VertexNode, horizon: HalfEdge[])`
- `addVertexToFace(vertex: VertexNode, face: Face)`
- `addVertexToHull(eyeVertex: VertexNode)`
- `cleanup()`
- `compute()`
- `computeExtremes()`
- `computeHorizon(eyePoint: Vector3, crossEdge: HalfEdge, face: Face, horizon: HalfEdge[])`
- `computeInitialHull()`
- `containsPoint(point: Vector3)`
- `deleteFaceVertices(face: Face, absorbingFace: Face)`
- `intersectRay(ray: Ray, target: Vector3)`
- `intersectsRay(ray: Ray)`
- `makeEmpty()`
- `nextVertexToAdd()`
- `reindexFaces()`
- `removeAllVerticesFromFace(face: Face)`
- `removeVertexFromFace(vertex: VertexNode, face: Face)`
- `resolveUnassignedPoints(newFaces: Face[])`
- `setFromPoints(points: Vector3[])`
- `setFromObject(object: Object3D)`
