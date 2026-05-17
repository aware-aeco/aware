---
name: viewer-speckle-instanced-mesh
description: SpeckleInstancedMesh declarations from viewer
---

# SpeckleInstancedMesh

## Methods

- `setBatchMaterial(material: Material)`
- `setBatchObjects(batchObjects: BatchObject[])`
- `setOverrideMaterial(material: Material)`
- `setOverrideBatchMaterial(material: Material)`
- `restoreBatchMaterial()`
- `getCachedMaterial(material: Material, copy?: boolean)`
- `restoreMaterial()`
- `buildTAS()`
- `updateDrawGroups(transformBuffer: Float32Array, gradientBuffer: Float32Array, objectIndexBuffer?: Float32Array)`
- `updateTransformsUniform()`
- `updateMaterialTransformsUniform(material: Material)`
- `getBatchObjectMaterial(batchObject: BatchObject)`
- `raycast(raycaster: SpeckleRaycaster, intersects: Array<Intersection>)`
