---
name: three-ray
description: Ray declarations from three
---

# Ray

## Methods

- `clone()`
- `copy(ray: Ray)`
- `at(t: number, target: Vector3)`
- `lookAt(v: Vector3)`
- `recast(t: number)`
- `closestPointToPoint(point: Vector3, target: Vector3)`
- `distanceToPoint(point: Vector3)`
- `distanceSqToPoint(point: Vector3)`
- `distanceSqToSegment()`
- `intersectSphere(sphere: Sphere, target: Vector3)`
- `intersectsSphere(sphere: Sphere)`
- `distanceToPlane(plane: Plane)`
- `intersectPlane(plane: Plane, target: Vector3)`
- `intersectsPlane(plane: Plane)`
- `intersectBox(box: Box3, target: Vector3)`
- `intersectsBox(box: Box3)`
- `intersectTriangle(a: Vector3, b: Vector3, c: Vector3, backfaceCulling: boolean, target: Vector3)`
- `applyMatrix4(matrix4: Matrix4)`
- `equals(ray: Ray)`
- `isIntersectionBox(b: any)`
- `isIntersectionPlane(p: any)`
- `isIntersectionSphere(s: any)`
