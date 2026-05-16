---
name: xeokit-sdk-meta-scene
description: MetaScene declarations from xeokit-sdk
---

# MetaScene

## Methods

- `on(event: string, callback: ()=> void)`
- `fire(event: string, value: any)`
- `off(event: any)`
- `createMetaModel()`
- `destroyMetaModel(id: string)`
- `getObjectIDsByType(type: string)`
- `getObjectIDsInSubtree(id: string, includeTypes?: string[], excludeTypes?: string[])`
- `withMetaObjectsInSubtree(id: string, callback: ()=> void)`
