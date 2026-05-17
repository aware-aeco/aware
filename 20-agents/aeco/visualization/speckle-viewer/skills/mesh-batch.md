---
name: viewer-mesh-batch
description: MeshBatch declarations from viewer
---

# MeshBatch

## Methods

- `getCurrentIndexBuffer()`
- `getNextIndexBuffer()`
- `shuffleMaterialOrder(a: DrawGroup, b: DrawGroup)`
- `updateGradientIndexBufferData(start: number, end: number, value: number)`
- `setDrawRanges(ranges: BatchUpdateRange[])`
- `resetDrawRanges()`
- `buildBatch()`
- `makeMeshGeometry(indices: Uint32Array | Uint16Array, position: Float64Array | Float32Array, normals: Float32Array, batchIndices: Float32Array, color?: Float32Array)`
- `getRenderView(index: number)`
- `getMaterialAtIndex(index: number)`
