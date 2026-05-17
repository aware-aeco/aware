---
name: core-frontend-aux-coord-system-state
description: AuxCoordSystemState declarations from core-frontend
---

# AuxCoordSystemState

## Methods

- `fromProps(props: AuxCoordSystemProps, iModel: IModelConnection)`
- `createNew(acsName: string, iModel: IModelConnection)`
- `toJSON()`
- `isValidForView(view: ViewState)`
- `getOrigin(result?: Point3d)`
- `setOrigin(val: XYAndZ | XAndY)`
- `getRotation(result?: Matrix3d)`
- `setRotation(val: Matrix3d)`
- `drawGrid(context: DecorateContext)`
- `isOriginInView(drawOrigin: Point3d, viewport: Viewport, adjustOrigin: boolean)`
- `display(context: DecorateContext, options: ACSDisplayOptions)`
- `fromProps(props: AuxCoordSystemProps, iModel: IModelConnection)`
- `createNew(acsName: string, iModel: IModelConnection)`
- `toJSON()`
- `isValidForView(view: ViewState)`
- `getOrigin(result?: Point3d)`
- `setOrigin(val: XYAndZ | XAndY)`
- `getRotation(result?: Matrix3d)`
- `setRotation(val: Matrix3d)`
- `drawGrid(context: DecorateContext)`
- `isOriginInView(drawOrigin: Point3d, viewport: Viewport, adjustOrigin: boolean)`
- `display(context: DecorateContext, options: ACSDisplayOptions)`
