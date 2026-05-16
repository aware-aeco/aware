---
name: three-vector2
description: Vector2 declarations from three
---

# Vector2

## Methods

- `setScalar(scalar: number)`
- `setX(x: number)`
- `setY(y: number)`
- `setComponent(index: number, value: number)`
- `getComponent(index: number)`
- `clone()`
- `copy(v: Vector2Like)`
- `add(v: Vector2Like)`
- `addScalar(s: number)`
- `addVectors(a: Vector2Like, b: Vector2Like)`
- `addScaledVector(v: Vector2Like, s: number)`
- `sub(v: Vector2Like)`
- `subScalar(s: number)`
- `subVectors(a: Vector2Like, b: Vector2Like)`
- `multiply(v: Vector2Like)`
- `multiplyScalar(scalar: number)`
- `divide(v: Vector2Like)`
- `divideScalar(s: number)`
- `applyMatrix3(m: Matrix3)`
- `min(v: Vector2Like)`
- `max(v: Vector2Like)`
- `clamp(min: Vector2Like, max: Vector2Like)`
- `clampScalar(min: number, max: number)`
- `clampLength(min: number, max: number)`
- `floor()`
- `ceil()`
- `round()`
- `roundToZero()`
- `negate()`
- `dot(v: Vector2Like)`
- `cross(v: Vector2Like)`
- `lengthSq()`
- `length()`
- `manhattanLength()`
- `normalize()`
- `angle()`
- `angleTo(v: Vector2)`
- `distanceTo(v: Vector2Like)`
- `distanceToSquared(v: Vector2Like)`
- `manhattanDistanceTo(v: Vector2Like)`
- `setLength(length: number)`
- `lerp(v: Vector2Like, alpha: number)`
- `lerpVectors(v1: Vector2Like, v2: Vector2Like, alpha: number)`
- `equals(v: Vector2Like)`
- `fromArray(array: number[] | ArrayLike<number>, offset?: number)`
- `toArray(array?: number[], offset?: number)`
- `toArray(array?: Vector2Tuple, offset?: 0)`
- `toArray(array: ArrayLike<number>, offset?: number)`
- `fromBufferAttribute(attribute: BufferAttribute, index: number)`
- `rotateAround(center: Vector2Like, angle: number)`
- `random()`
