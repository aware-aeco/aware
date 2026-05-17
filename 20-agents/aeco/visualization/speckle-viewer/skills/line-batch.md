---
name: viewer-line-batch
description: LineBatch declarations from viewer
---

# LineBatch

## Methods

- `getCount()`
- `setBatchMaterial(material: SpeckleLineMaterial)`
- `onUpdate(deltaTime: number)`
- `onRender(renderer: WebGLRenderer)`
- `setVisibleRange(ranges: BatchUpdateRange[])`
- `getVisibleRange()`
- `getOpaque()`
- `getDepth()`
- `getTransparent()`
- `getStencil()`
- `setBatchBuffers(ranges: BatchUpdateRange[])`
- `setDrawRanges(ranges: BatchUpdateRange[])`
- `resetDrawRanges()`
- `buildBatch()`
- `getRenderView(index: number)`
- `getMaterialAtIndex(index: number)`
- `getMaterial(rv: NodeRenderView)`
- `purge()`
