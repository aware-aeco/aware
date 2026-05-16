---
name: three-triangle
description: Triangle declarations from three
---

# Triangle

## Methods

- `setFromPointsAndIndices(points: Vector3[], i0: number, i1: number, i2: number)`
- `setFromAttributeAndIndices()`
- `clone()`
- `copy(triangle: Triangle)`
- `getArea()`
- `getMidpoint(target: Vector3)`
- `getNormal(target: Vector3)`
- `getPlane(target: Plane)`
- `getBarycoord(point: Vector3, target: Vector3)`
- `getInterpolation(point: Vector3, v1: Vector2, v2: Vector2, v3: Vector2, target: Vector2)`
- `getInterpolation(point: Vector3, v1: Vector3, v2: Vector3, v3: Vector3, target: Vector3)`
- `getInterpolation(point: Vector3, v1: Vector4, v2: Vector4, v3: Vector4, target: Vector4)`
- `containsPoint(point: Vector3)`
- `intersectsBox(box: Box3)`
- `isFrontFacing(direction: Vector3)`
- `closestPointToPoint(point: Vector3, target: Vector3)`
- `equals(triangle: Triangle)`
- `getNormal(a: Vector3, b: Vector3, c: Vector3, target: Vector3)`
- `getBarycoord(point: Vector3, a: Vector3, b: Vector3, c: Vector3, target: Vector3)`
- `containsPoint(point: Vector3, a: Vector3, b: Vector3, c: Vector3)`
- `getInterpolation()`
- `getInterpolation()`
- `getInterpolation()`
- `getInterpolatedAttribute()`
- `getInterpolatedAttribute()`
- `getInterpolatedAttribute()`
- `isFrontFacing(a: Vector3, b: Vector3, c: Vector3, direction: Vector3)`
