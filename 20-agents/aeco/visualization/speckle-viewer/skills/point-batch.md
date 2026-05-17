---
name: viewer-point-batch
description: PointBatch declarations from viewer
---

# PointBatch

## Methods

- `setDrawRanges(ranges: BatchUpdateRange[])`
- `resetDrawRanges()`
- `getCurrentIndexBuffer()`
- `getNextIndexBuffer()`
- `shuffleMaterialOrder(a: DrawGroup, b: DrawGroup)`
- `updateGradientIndexBufferData(start: number, end: number, value: number)`
- `buildBatch()`
- `makePointGeometry(index: Uint32Array | Uint16Array, position: Float64Array | Float32Array, color: Float32Array)`
- `getRenderView(index: number)`
- `getMaterialAtIndex(index: number)`
