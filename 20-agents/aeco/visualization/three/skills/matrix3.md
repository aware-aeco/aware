---
name: three-matrix3
description: Matrix3 declarations from three
---

# Matrix3

## Methods

- `identity()`
- `copy(m: Matrix3)`
- `extractBasis(xAxis: Vector3, yAxis: Vector3, zAxis: Vector3)`
- `setFromMatrix4(m: Matrix4)`
- `multiply(m: Matrix3)`
- `premultiply(m: Matrix3)`
- `multiplyMatrices(a: Matrix3, b: Matrix3)`
- `multiplyScalar(s: number)`
- `determinant()`
- `invert()`
- `transpose()`
- `getNormalMatrix(matrix4: Matrix4)`
- `transposeIntoArray(r: number[])`
- `setUvTransform(tx: number, ty: number, sx: number, sy: number, rotation: number, cx: number, cy: number)`
- `scale(sx: number, sy: number)`
- `rotate(theta: number)`
- `translate(tx: number, ty: number)`
- `makeTranslation(v: Vector2)`
- `makeTranslation(x: number, y: number)`
- `makeRotation(theta: number)`
- `makeScale(x: number, y: number)`
- `equals(matrix: Matrix3)`
- `fromArray(array: ArrayLike<number>, offset?: number)`
- `toArray()`
- `clone()`
