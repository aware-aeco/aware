---
name: three-quaternion
description: Quaternion declarations from three
---

# Quaternion

## Methods

- `clone()`
- `copy(q: QuaternionLike)`
- `setFromEuler(euler: Euler, update?: boolean)`
- `setFromAxisAngle(axis: Vector3Like, angle: number)`
- `setFromRotationMatrix(m: Matrix4)`
- `setFromUnitVectors(vFrom: Vector3, vTo: Vector3Like)`
- `angleTo(q: Quaternion)`
- `rotateTowards(q: Quaternion, step: number)`
- `identity()`
- `invert()`
- `conjugate()`
- `dot(v: Quaternion)`
- `lengthSq()`
- `length()`
- `normalize()`
- `multiply(q: Quaternion)`
- `premultiply(q: Quaternion)`
- `multiplyQuaternions(a: Quaternion, b: Quaternion)`
- `slerp(qb: Quaternion, t: number)`
- `slerpQuaternions(qa: Quaternion, qb: Quaternion, t: number)`
- `equals(v: Quaternion)`
- `fromArray(array: number[] | ArrayLike<number>, offset?: number)`
- `toArray(array?: number[], offset?: number)`
- `toArray(array?: QuaternionTuple, offset?: 0)`
- `toArray(array: ArrayLike<number>, offset?: number)`
- `toJSON()`
- `fromBufferAttribute(attribute: BufferAttribute | InterleavedBufferAttribute, index: number)`
- `_onChange(callback: () => void)`
- `slerpFlat()`
- `multiplyQuaternionsFlat()`
- `random()`
