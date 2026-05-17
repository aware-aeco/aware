---
name: core-frontend-spatial-view-state
description: SpatialViewState declarations from core-frontend
---

# SpatialViewState

## Methods

- `createBlank(iModel: IModelConnection, origin: XYAndZ, extents: XYAndZ, rotation?: Matrix3d)`
- `createFromProps(props: ViewStateProps, iModel: IModelConnection)`
- `toProps()`
- `isSpatialView()`
- `equals(other: this)`
- `createAuxCoordSystem(acsName: string)`
- `markModelSelectorChanged()`
- `computeFitRange(options?: ComputeSpatialViewFitRangeOptions)`
- `getViewedExtents()`
- `toJSON()`
- `preload(hydrateRequest: HydrateViewStateRequestProps)`
- `postload(hydrateResponse: HydrateViewStateResponseProps)`
- `viewsModel(modelId: Id64String)`
- `clearViewedModels()`
- `addViewedModel(id: Id64String)`
- `removeViewedModel(id: Id64String)`
- `forEachModel(func: (model: GeometricModelState) => void)`
- `getModelTreeRefs()`
- `createScene(context: SceneContext)`
- `attachToViewport(args: AttachToViewportArgs)`
- `detachFromViewport()`
- `setTileTreeReferencesDeactivated(modelIds: Id64String | Id64String[] | undefined, deactivated: boolean | undefined, which: "all" | "animated" | "primary" | "section" | number[])`
- `collectMaskRefs(modelIds: OrderedId64Iterable, maskTreeRefs: TileTreeReference[], maskRange: Range3d)`
- `getModelsNotInMask(maskModels: OrderedId64Iterable | undefined, useVisible: boolean)`
- `createBlank(iModel: IModelConnection, origin: XYAndZ, extents: XYAndZ, rotation?: Matrix3d)`
- `createFromProps(props: ViewStateProps, iModel: IModelConnection)`
- `toProps()`
- `isSpatialView()`
- `equals(other: this)`
- `createAuxCoordSystem(acsName: string)`
- `markModelSelectorChanged()`
- `computeFitRange(options?: ComputeSpatialViewFitRangeOptions)`
- `getViewedExtents()`
- `toJSON()`
- `preload(hydrateRequest: HydrateViewStateRequestProps)`
- `postload(hydrateResponse: HydrateViewStateResponseProps)`
- `viewsModel(modelId: Id64String)`
- `clearViewedModels()`
- `addViewedModel(id: Id64String)`
- `removeViewedModel(id: Id64String)`
- `forEachModel(func: (model: GeometricModelState) => void)`
- `getModelTreeRefs()`
- `createScene(context: SceneContext)`
- `attachToViewport(args: AttachToViewportArgs)`
- `detachFromViewport()`
- `setTileTreeReferencesDeactivated(modelIds: Id64String | Id64String[] | undefined, deactivated: boolean | undefined, which: "all" | "animated" | "primary" | "section" | number[])`
- `collectMaskRefs(modelIds: OrderedId64Iterable, maskTreeRefs: TileTreeReference[], maskRange: Range3d)`
- `getModelsNotInMask(maskModels: OrderedId64Iterable | undefined, useVisible: boolean)`
