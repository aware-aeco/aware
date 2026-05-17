---
name: viewer-speckle-batched-text
description: SpeckleBatchedText declarations from viewer
---

# SpeckleBatchedText

## Methods

- `setBatchMaterial(material: Material)`
- `setBatchObjects(batchObjects: BatchObject[], textObjects: Text[])`
- `getCachedMaterial(material: Material, copy?: boolean)`
- `buildTAS()`
- `updateTransformsUniform()`
- `updateMaterialTransformsUniform(material: Material)`
- `setGradientTexture(texture: Texture)`
- `getBatchObjectMaterial(batchObject: BatchObject)`
- `onBeforeRender(renderer: SpeckleWebGLRenderer, scene: Scene, camera: Camera, geometry: BufferGeometry, material: Material, group: unknown)`
- `raycast(raycaster: SpeckleRaycaster, intersects: Array<Intersection>)`
- `updateBounds()`
- `addText(text: Text)`
- `_prepareForRender(material: SpeckleTextMaterial)`
