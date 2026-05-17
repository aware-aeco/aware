---
name: components-i-d-s-facet
description: IDSFacet declarations from components
---

# IDSFacet

## Methods

- `addCheckResult(check: IDSCheck, checks: IDSCheck[])`
- `getItemChecks(collector: ModelIdDataMap<IDSItemCheckResult>, modelId: string, item: FRAGS.ItemData, skipIfFails: boolean)`
- `getEntities(modelIds: RegExp[], collector: ModelIdMap)`
- `test()`
- `serialize(type: "applicability" | "requirement")`
