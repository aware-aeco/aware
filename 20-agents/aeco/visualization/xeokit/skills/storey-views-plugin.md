---
name: xeokit-sdk-storey-views-plugin
description: StoreyViewsPlugin declarations from xeokit-sdk
---

# StoreyViewsPlugin

## Methods

- `gotoStoreyCamera()`
- `showStoreyObjects()`
- `withStoreyObjects(storeyId: string, callback: Function)`
- `createStoreyMap()`
- `pickStoreyMap()`
- `getStoreyContainingWorldPos(worldPos: number[], modelId: null | string)`
- `getStoreyInVerticalRange(worldPos: number[], modelId: null | string)`
- `isPositionAboveOrBelowBuilding(worldPos: number[], modelId: null | string)`
- `worldPosToStoreyMap(storeyMap: StoreyMap, worldPos: number[], imagePos: number[])`
- `worldDirToStoreyMap(storeyMap: StoreyMap, worldDir: number[], imageDir: number[])`
- `on(event: "storeys", callback: (storyes: {[key: string]: Storey })=> void)`
