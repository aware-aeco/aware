---
name: viewer-primitive-batch
description: PrimitiveBatch declarations from viewer
---

# PrimitiveBatch

## Methods

- `getCount()`
- `setBatchMaterial(material: Material)`
- `onUpdate()`
- `setVisibleRange(ranges: BatchUpdateRange[])`
- `getVisibleRange()`
- `getOpaque()`
- `getDepth()`
- `getTransparent()`
- `getStencil()`
- `setBatchBuffers(ranges: BatchUpdateRange[])`
- `cleanMaterials()`
- `getCurrentIndexBuffer()`
- `getNextIndexBuffer()`
- `shuffleMaterialOrder(a: DrawGroup, b: DrawGroup)`
- `updateGradientIndexBufferData(start: number, end: number, value: number)`
- `updateGradientIndexBuffer(rangeMin?: number, rangeMax?: number)`
- `setDrawRanges(ranges: BatchUpdateRange[])`
- `resetDrawRanges()`
- `buildBatch()`
- `getRenderView(index: number)`
- `getMaterialAtIndex(index: number)`
- `getMaterial(rv: NodeRenderView)`
- `purge()`
