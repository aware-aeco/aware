---
name: three-object3-d
description: Object3D declarations from three
---

# Object3D

## Methods

- `onBeforeShadow()`
- `onAfterShadow()`
- `onBeforeRender()`
- `onAfterRender()`
- `applyMatrix4(matrix: Matrix4)`
- `applyQuaternion(quaternion: Quaternion)`
- `setRotationFromAxisAngle(axis: Vector3, angle: number)`
- `setRotationFromEuler(euler: Euler)`
- `setRotationFromMatrix(m: Matrix4)`
- `setRotationFromQuaternion(q: Quaternion)`
- `rotateOnAxis(axis: Vector3, angle: number)`
- `rotateOnWorldAxis(axis: Vector3, angle: number)`
- `rotateX(angle: number)`
- `rotateY(angle: number)`
- `rotateZ(angle: number)`
- `translateOnAxis(axis: Vector3, distance: number)`
- `translateX(distance: number)`
- `translateY(distance: number)`
- `translateZ(distance: number)`
- `localToWorld(vector: Vector3)`
- `worldToLocal(vector: Vector3)`
- `lookAt(vector: Vector3)`
- `lookAt(x: number, y: number, z: number)`
- `add(...object: Object3D[])`
- `remove(...object: Object3D[])`
- `removeFromParent()`
- `clear()`
- `attach(object: Object3D)`
- `getObjectById(id: number)`
- `getObjectByName(name: string)`
- `getObjectByProperty(name: string, value: any)`
- `getObjectsByProperty(name: string, value: any, optionalTarget?: Object3D[])`
- `getWorldPosition(target: Vector3)`
- `getWorldQuaternion(target: Quaternion)`
- `getWorldScale(target: Vector3)`
- `getWorldDirection(target: Vector3)`
- `raycast(raycaster: Raycaster, intersects: Intersection[])`
- `traverse(callback: (object: Object3D) => any)`
- `traverseVisible(callback: (object: Object3D) => any)`
- `traverseAncestors(callback: (object: Object3D) => any)`
- `updateMatrix()`
- `updateMatrixWorld(force?: boolean)`
- `updateWorldMatrix(updateParents: boolean, updateChildren: boolean)`
- `toJSON(meta?: JSONMeta)`
- `clone(recursive?: boolean)`
- `copy(object: Object3D, recursive?: boolean)`
