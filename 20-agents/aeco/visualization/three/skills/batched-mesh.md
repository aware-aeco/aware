---
name: three-batched-mesh
description: BatchedMesh declarations from three
---

# BatchedMesh

## Methods

- `computeBoundingBox()`
- `computeBoundingSphere()`
- `dispose()`
- `setCustomSort()`
- `getColorAt(instanceId: number, color: Color)`
- `getColorAt(instanceId: number, color: Vector4)`
- `getColorAt(instanceId: number, color: Color | Vector4)`
- `getMatrixAt(instanceId: number, target: Matrix4)`
- `getVisibleAt(instanceId: number)`
- `getGeometryRangeAt()`
- `getGeometryIdAt(instanceId: number)`
- `setColorAt(instanceId: number, color: Color | Vector4)`
- `setMatrixAt(instanceId: number, matrix: Matrix4)`
- `setVisibleAt(instanceId: number, visible: boolean)`
- `setGeometryIdAt(instanceId: number, geometryId: number)`
- `addGeometry(geometry: BufferGeometry, reservedVertexRange?: number, reservedIndexRange?: number)`
- `addInstance(geometryId: number)`
- `deleteGeometry(geometryId: number)`
- `deleteInstance(instanceId: number)`
- `setGeometryAt(geometryId: number, geometry: BufferGeometry)`
- `optimize()`
- `setGeometrySize(maxVertexCount: number, maxIndexCount: number)`
- `setInstanceCount(maxInstanceCount: number)`
- `getBoundingBoxAt(geometryId: number, target: Box3)`
- `getBoundingSphereAt(geometryId: number, target: Sphere)`
