---
name: three-euler
description: Euler declarations from three
---

# Euler

## Methods

- `clone()`
- `copy(euler: Euler)`
- `setFromRotationMatrix(m: Matrix4, order?: EulerOrder, update?: boolean)`
- `setFromQuaternion(q: Quaternion, order?: EulerOrder, update?: boolean)`
- `setFromVector3(v: Vector3, order?: EulerOrder)`
- `reorder(newOrder: EulerOrder)`
- `equals(euler: Euler)`
- `fromArray(array: EulerTuple)`
- `toArray(array?: Partial<EulerTuple>, offset?: number)`
- `_onChange(callback: () => void)`
