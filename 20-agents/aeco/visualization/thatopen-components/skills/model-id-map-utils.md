---
name: components-model-id-map-utils
description: ModelIdMapUtils declarations from components
---

# ModelIdMapUtils

## Methods

- `join(maps: ModelIdMap[])`
- `intersect(maps: ModelIdMap[])`
- `clone(source: ModelIdMap)`
- `remove(target: ModelIdMap, source: ModelIdMap, clone?: boolean)`
- `add(target: ModelIdMap, source: ModelIdMap, clone?: boolean)`
- `append(target: ModelIdMap, modelId: string, ...localIds: number[])`
- `isEqual(a: ModelIdMap, b: ModelIdMap)`
- `isEmpty(map: ModelIdMap)`
- `toRaw(map: ModelIdMap)`
- `fromRaw()`
