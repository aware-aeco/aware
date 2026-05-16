---
name: three-sphere
description: Sphere declarations from three
---

# Sphere

## Methods

- `setFromPoints(points: Vector3[], optionalCenter?: Vector3)`
- `clone()`
- `copy(sphere: Sphere)`
- `expandByPoint(point: Vector3)`
- `isEmpty()`
- `makeEmpty()`
- `containsPoint(point: Vector3)`
- `distanceToPoint(point: Vector3)`
- `intersectsSphere(sphere: Sphere)`
- `intersectsBox(box: Box3)`
- `intersectsPlane(plane: Plane)`
- `clampPoint(point: Vector3, target: Vector3)`
- `getBoundingBox(target: Box3)`
- `applyMatrix4(matrix: Matrix4)`
- `translate(offset: Vector3)`
- `equals(sphere: Sphere)`
- `union(sphere: Sphere)`
- `empty()`
- `toJSON()`
- `fromJSON(json: SphereJSON)`
