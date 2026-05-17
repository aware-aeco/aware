---
name: viewer-batcher
description: Batcher declarations from viewer
---

# Batcher

## Methods

- `makeBatches(worldTree: WorldTree, renderTree: RenderTree, speckleType: SpeckleType[], batchType?: GeometryType)`
- `update(deltaTime: number)`
- `render(renderer: WebGLRenderer)`
- `saveVisiblity()`
- `applyVisibility(ranges: Record<string, BatchUpdateRange>)`
- `getVisibility(objectVisibility: ObjectVisibility)`
- `getTransparent()`
- `getStencil()`
- `getOpaque()`
- `getDepth()`
- `overrideMaterial(ranges: Record<string, BatchUpdateRange>, material: Material)`
- `overrideBatchMaterial(ranges: Record<string, BatchUpdateRange>, material: Material)`
- `restoreMaterial(ranges: Record<string, BatchUpdateRange>)`
- `restoreBatchMaterial(ranges: Record<string, BatchUpdateRange>)`
- `purgeBatches(subtreeId: string)`
- `getBatches(subtreeId?: string, geometryType?: K)`
- `getBatches(subtreeId?: string, geometryType?: Array<K>)`
- `getBatch(rv: NodeRenderView)`
- `getRenderView(batchId: string, index: number)`
- `getRenderViewMaterial(batchId: string, index: number)`
- `resetBatchesDrawRanges()`
- `isolateBatch(batchId: string)`
