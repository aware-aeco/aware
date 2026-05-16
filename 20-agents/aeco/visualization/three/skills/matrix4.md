---
name: three-matrix4
description: Matrix4 declarations from three
---

# Matrix4

## Methods

- `identity()`
- `clone()`
- `copy(m: Matrix4)`
- `copyPosition(m: Matrix4)`
- `setFromMatrix3(m: Matrix3)`
- `extractBasis(xAxis: Vector3, yAxis: Vector3, zAxis: Vector3)`
- `makeBasis(xAxis: Vector3, yAxis: Vector3, zAxis: Vector3)`
- `extractRotation(m: Matrix4)`
- `makeRotationFromEuler(euler: Euler)`
- `makeRotationFromQuaternion(q: Quaternion)`
- `lookAt(eye: Vector3, target: Vector3, up: Vector3)`
- `multiply(m: Matrix4)`
- `premultiply(m: Matrix4)`
- `multiplyMatrices(a: Matrix4, b: Matrix4)`
- `multiplyScalar(s: number)`
- `determinant()`
- `transpose()`
- `setPosition(v: Vector3)`
- `setPosition(x: number, y: number, z: number)`
- `invert()`
- `scale(v: Vector3)`
- `getMaxScaleOnAxis()`
- `makeTranslation(v: Vector3)`
- `makeTranslation(x: number, y: number, z: number)`
- `makeRotationX(theta: number)`
- `makeRotationY(theta: number)`
- `makeRotationZ(theta: number)`
- `makeRotationAxis(axis: Vector3, angle: number)`
- `makeScale(x: number, y: number, z: number)`
- `makeShear(xy: number, xz: number, yx: number, yz: number, zx: number, zy: number)`
- `compose(position: Vector3, quaternion: Quaternion, scale: Vector3)`
- `decompose(position: Vector3, quaternion: Quaternion, scale: Vector3)`
- `makePerspective()`
- `makeOrthographic()`
- `equals(matrix: Matrix4)`
- `fromArray(array: ArrayLike<number>, offset?: number)`
- `toArray()`
