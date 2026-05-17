---
name: viewer-batch
description: Batch declarations from viewer
---

# Batch

## Methods

- `getCount()`
- `setBatchMaterial(material: Material)`
- `setBatchBuffers(range: BatchUpdateRange[])`
- `setVisibleRange(range: BatchUpdateRange[])`
- `getVisibleRange()`
- `setDrawRanges(ranges: BatchUpdateRange[])`
- `resetDrawRanges()`
- `buildBatch()`
- `getRenderView(index: number)`
- `getMaterialAtIndex(index: number)`
- `getMaterial(rv: NodeRenderView)`
- `getOpaque()`
- `getTransparent()`
- `getStencil()`
- `getDepth()`
- `onUpdate(deltaTime?: number)`
- `purge()`
