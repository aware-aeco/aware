---
name: three-plane
description: Plane declarations from three
---

# Plane

## Methods

- `setComponents(x: number, y: number, z: number, w: number)`
- `setFromNormalAndCoplanarPoint(normal: Vector3, point: Vector3)`
- `setFromCoplanarPoints(a: Vector3, b: Vector3, c: Vector3)`
- `clone()`
- `copy(plane: Plane)`
- `normalize()`
- `negate()`
- `distanceToPoint(point: Vector3)`
- `distanceToSphere(sphere: Sphere)`
- `projectPoint(point: Vector3, target: Vector3)`
- `intersectLine(line: Line3, target: Vector3, clampToLine?: boolean)`
- `intersectsLine(line: Line3)`
- `intersectsBox(box: Box3)`
- `intersectsSphere(sphere: Sphere)`
- `coplanarPoint(target: Vector3)`
- `applyMatrix4(matrix: Matrix4, optionalNormalMatrix?: Matrix3)`
- `translate(offset: Vector3)`
- `equals(plane: Plane)`
- `isIntersectionLine(l: any)`
