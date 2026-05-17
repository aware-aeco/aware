---
name: components-fragments-manager
description: FragmentsManager declarations from components
---

# FragmentsManager

## Methods

- `getWorker()`
- `dispose()`
- `init()`
- `raycast()`
- `getPositions(items: ModelIdMap)`
- `getBBoxes(items: ModelIdMap)`
- `highlight(style: FRAGS.MaterialDefinition, items?: ModelIdMap)`
- `getData(items: ModelIdMap, config?: Partial<FRAGS.ItemsDataConfig>)`
- `resetHighlight(items?: ModelIdMap)`
- `guidsToModelIdMap(guids: Iterable<string>)`
- `modelIdMapToGuids(modelIdMap: ModelIdMap)`
- `applyBaseCoordinateSystem(object: THREE.Object3D | THREE.Vector3, originalCoordinateSystem?: THREE.Matrix4)`
