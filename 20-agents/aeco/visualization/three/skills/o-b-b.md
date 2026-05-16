---
name: three-o-b-b
description: OBB declarations from three
---

# OBB

## Methods

- `copy(obb: OBB)`
- `clone()`
- `getSize(result: Vector3)`
- `clampPoint(point: Vector3, result: Vector3)`
- `containsPoint(point: Vector3)`
- `intersectsBox3(box3: Box3)`
- `intersectsSphere(sphere: Sphere)`
- `intersectsOBB(obb: OBB, epsilon?: number)`
- `intersectsPlane(plane: Plane)`
- `intersectRay(ray: Ray, result: Vector3)`
- `intersectsRay(ray: Ray)`
- `fromBox3(box3: Box3)`
- `equals(obb: OBB)`
- `applyMatrix4(matrix: Matrix4)`
