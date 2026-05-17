---
name: web-ifc-properties
description: Properties declarations from web-ifc
---

# Properties

## Methods

- `getItemProperties(modelID: number, id: number, recursive?: boolean, inverse?: boolean)`
- `getPropertySets(modelID: number, elementID?: number, recursive?: boolean, includeTypeProperties?: boolean)`
- `setPropertySets(modelID: number, elementID: number | number[], psetID: number | number[])`
- `getTypeProperties(modelID: number, elementID?: number, recursive?: boolean)`
- `getMaterialsProperties(modelID: number, elementID?: number, recursive?: boolean, includeTypeMaterials?: boolean)`
- `setMaterialsProperties(modelID: number, elementID: number | number[], materialID: number | number[])`
- `getSpatialStructure(modelID: number, includeProperties?: boolean)`
