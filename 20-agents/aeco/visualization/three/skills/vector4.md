---
name: three-vector4
description: Vector4 declarations from three
---

# Vector4

## Methods

- `setScalar(scalar: number)`
- `setX(x: number)`
- `setY(y: number)`
- `setZ(z: number)`
- `setW(w: number)`
- `setComponent(index: number, value: number)`
- `getComponent(index: number)`
- `clone()`
- `copy(v: Vector4Like)`
- `add(v: Vector4Like)`
- `addScalar(scalar: number)`
- `addVectors(a: Vector4Like, b: Vector4Like)`
- `addScaledVector(v: Vector4Like, s: number)`
- `sub(v: Vector4Like)`
- `subScalar(s: number)`
- `subVectors(a: Vector4Like, b: Vector4Like)`
- `multiply(v: Vector4Like)`
- `multiplyScalar(s: number)`
- `applyMatrix4(m: Matrix4)`
- `divide(v: Vector4Like)`
- `divideScalar(s: number)`
- `setAxisAngleFromQuaternion(q: QuaternionLike)`
- `setAxisAngleFromRotationMatrix(m: Matrix4)`
- `setFromMatrixPosition(m: Matrix4)`
- `min(v: Vector4Like)`
- `max(v: Vector4Like)`
- `clamp(min: Vector4Like, max: Vector4Like)`
- `clampScalar(min: number, max: number)`
- `floor()`
- `ceil()`
- `round()`
- `roundToZero()`
- `negate()`
- `dot(v: Vector4Like)`
- `lengthSq()`
- `length()`
- `manhattanLength()`
- `normalize()`
- `setLength(length: number)`
- `lerp(v: Vector4Like, alpha: number)`
- `lerpVectors(v1: Vector4Like, v2: Vector4Like, alpha: number)`
- `equals(v: Vector4Like)`
- `fromArray(array: number[] | ArrayLike<number>, offset?: number)`
- `toArray(array?: number[], offset?: number)`
- `toArray(array?: Vector4Tuple, offset?: 0)`
- `toArray(array: ArrayLike<number>, offset?: number)`
- `fromBufferAttribute(attribute: BufferAttribute, index: number)`
- `random()`
