---
name: three-octree
description: Octree declarations from three
---

# Octree

## Methods

- `addTriangle(triangle: Triangle)`
- `calcBox()`
- `split(level: number)`
- `build()`
- `getRayTriangles(ray: Ray, triangles: Triangle[])`
- `triangleCapsuleIntersect(capsule: Capsule, triangle: Triangle)`
- `triangleBoxIntersect(box: Box3, triangle: Triangle)`
- `triangleSphereIntersect(sphere: Sphere, triangle: Triangle)`
- `getSphereTriangles(sphere: Sphere, triangles: Triangle[])`
- `getBoxTriangles(box: Box3, triangles: Triangle[])`
- `getCapsuleTriangles(capsule: Capsule, triangles: Triangle[])`
- `boxIntersect(box: Box3)`
- `sphereIntersect(sphere: Sphere)`
- `capsuleIntersect(capsule: Capsule)`
- `rayIntersect(ray: Ray)`
- `fromGraphNode(group: Object3D)`
- `clear()`
