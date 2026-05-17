---
name: viewer-text-batch
description: TextBatch declarations from viewer
---

# TextBatch

## Methods

- `getCount()`
- `setBatchMaterial(material: Material)`
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
- `alignmentXToAnchorX(value: number)`
- `alignmentYToAnchorY(value: number)`
- `buildBatch()`
- `getRenderView(index: number)`
- `getMaterialAtIndex(index: number)`
- `getMaterial(rv: NodeRenderView)`
- `purge()`
