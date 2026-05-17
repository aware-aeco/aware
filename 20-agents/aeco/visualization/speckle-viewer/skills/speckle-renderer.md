---
name: viewer-speckle-renderer
description: SpeckleRenderer declarations from viewer
---

# SpeckleRenderer

## Methods

- `create(container: HTMLElement)`
- `update(deltaTime: number)`
- `resetPipeline()`
- `render()`
- `resize(width?: number, height?: number)`
- `addRenderTree(renderTree: RenderTree)`
- `removeRenderTree(subtreeId: string)`
- `cancelRenderTree(subtreeId: string)`
- `setMaterial(rvs: NodeRenderView[], material: Material)`
- `setMaterial(rvs: NodeRenderView[], material: RenderMaterial & DisplayStyle & MaterialOptions)`
- `setMaterial(rvs: NodeRenderView[], material: FilterMaterial)`
- `getMaterial(rv: NodeRenderView)`
- `getBatchMaterial(rv: NodeRenderView)`
- `resetMaterials()`
- `getBatch(id: string)`
- `updateClippingPlanes()`
- `updateShadowCatcher(force?: boolean)`
- `setSunLightConfiguration(config: SunLightConfiguration)`
- `queryHits(results: Array<ExtendedIntersection>)`
- `queryHitIds(results: Array<ExtendedIntersection>)`
- `renderViewFromIntersection(intersection: ExtendedIntersection)`
- `boxFromObjects(objectIds: string[])`
- `screenToNDC(clientX: number, clientY: number, width?: number, height?: number)`
- `NDCToScreen(clientX: number, clientY: number, width?: number, height?: number)`
- `debugShowBatches()`
- `getBatchIds()`
- `getBatchSize(batchId: string)`
- `isolateBatch(batchId: string)`
- `getObjects()`
- `getObject(rv: NodeRenderView)`
- `enableLayers(layers: ObjectLayers[], value: boolean)`
