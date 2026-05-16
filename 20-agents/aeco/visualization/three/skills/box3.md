---
name: three-box3
description: Box3 declarations from three
---

# Box3

## Methods

- `setFromArray(array: ArrayLike<number>)`
- `setFromBufferAttribute(bufferAttribute: BufferAttribute)`
- `setFromPoints(points: Vector3[])`
- `setFromCenterAndSize(center: Vector3, size: Vector3)`
- `setFromObject(object: Object3D, precise?: boolean)`
- `clone()`
- `copy(box: Box3)`
- `makeEmpty()`
- `isEmpty()`
- `getCenter(target: Vector3)`
- `getSize(target: Vector3)`
- `expandByPoint(point: Vector3)`
- `expandByVector(vector: Vector3)`
- `expandByScalar(scalar: number)`
- `expandByObject(object: Object3D, precise?: boolean)`
- `containsPoint(point: Vector3)`
- `containsBox(box: Box3)`
- `getParameter(point: Vector3, target: Vector3)`
- `intersectsBox(box: Box3)`
- `intersectsSphere(sphere: Sphere)`
- `intersectsPlane(plane: Plane)`
- `intersectsTriangle(triangle: Triangle)`
- `clampPoint(point: Vector3, target: Vector3)`
- `distanceToPoint(point: Vector3)`
- `getBoundingSphere(target: Sphere)`
- `intersect(box: Box3)`
- `union(box: Box3)`
- `applyMatrix4(matrix: Matrix4)`
- `translate(offset: Vector3)`
- `equals(box: Box3)`
- `empty()`
- `isIntersectionBox(b: any)`
- `isIntersectionSphere(s: any)`
- `toJSON()`
- `fromJSON(json: Box3JSON)`
