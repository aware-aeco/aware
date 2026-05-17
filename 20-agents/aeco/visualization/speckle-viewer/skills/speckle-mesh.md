---
name: viewer-speckle-mesh
description: SpeckleMesh declarations from viewer
---

# SpeckleMesh

## Methods

- `setBatchMaterial(material: Material)`
- `setBatchObjects(batchObjects: BatchObject[], transformStorage: TransformStorage)`
- `setOverrideMaterial(material: Material)`
- `setOverrideBatchMaterial(material: Material)`
- `restoreBatchMaterial()`
- `getCachedMaterial(material: Material, copy?: boolean)`
- `restoreMaterial()`
- `updateMaterialTransformsUniform(material: Material)`
- `updateTransformsUniform()`
- `buildTAS()`
- `getBatchObjectMaterial(batchObject: BatchObject)`
- `raycast(raycaster: SpeckleRaycaster, intersects: Array<Intersection>)`
