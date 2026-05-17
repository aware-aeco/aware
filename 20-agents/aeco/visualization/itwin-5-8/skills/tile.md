---
name: core-frontend-tile
description: Tile declarations from core-frontend
---

# Tile

## Methods

- `_loadChildren(resolve: (children: Tile[] | undefined) => void, reject: (error: Error) => void)`
- `requestContent(isCanceled: () => boolean)`
- `readContent(data: TileRequest.ResponseData, system: RenderSystem, isCanceled?: () => boolean)`
- `freeMemory()`
- `disposeContents()`
- `dispose()`
- `setNotFound()`
- `setIsReady()`
- `setLeaf()`
- `computeLoadPriority(_viewports: Iterable<Viewport>, _users: Iterable<TileUser>)`
- `produceGraphics()`
- `setGraphic(graphic: RenderGraphic | undefined)`
- `setContent(content: TileContent)`
- `_collectStatistics(_stats: RenderMemory.Statistics)`
- `collectStatistics(stats: RenderMemory.Statistics, includeChildren?: boolean)`
- `loadChildren()`
- `disposeChildren()`
- `isRegionCulled(args: TileDrawArgs)`
- `isContentCulled(args: TileDrawArgs)`
- `isFrustumCulled(box: Frustum, args: TileDrawArgs, testClipIntersection: boolean, sphere?: BoundingSphere)`
- `computeVisibility(args: TileDrawArgs)`
- `meetsScreenSpaceError(args: TileDrawArgs)`
- `extendRangeForContent(range: Range3d, matrix: Matrix4d, treeTransform: Transform, frustumPlanes?: FrustumPlanes)`
- `countDescendants()`
- `drawGraphics(args: TileDrawArgs)`
- `getRangeGraphic(context: SceneContext)`
- `addRangeGraphic(builder: GraphicBuilder, type: TileBoundingBoxes)`
- `getSizeProjectionCorners()`
- `clearLayers()`
- `_loadChildren(resolve: (children: Tile[] | undefined) => void, reject: (error: Error) => void)`
- `requestContent(isCanceled: () => boolean)`
- `readContent(data: TileRequest.ResponseData, system: RenderSystem, isCanceled?: () => boolean)`
- `freeMemory()`
- `disposeContents()`
- `dispose()`
- `setNotFound()`
- `setIsReady()`
- `setLeaf()`
- `computeLoadPriority(_viewports: Iterable<Viewport>, _users: Iterable<TileUser>)`
- `produceGraphics()`
- `setGraphic(graphic: RenderGraphic | undefined)`
- `setContent(content: TileContent)`
- `_collectStatistics(_stats: RenderMemory.Statistics)`
- `collectStatistics(stats: RenderMemory.Statistics, includeChildren?: boolean)`
- `loadChildren()`
- `disposeChildren()`
- `isRegionCulled(args: TileDrawArgs)`
- `isContentCulled(args: TileDrawArgs)`
- `isFrustumCulled(box: Frustum, args: TileDrawArgs, testClipIntersection: boolean, sphere?: BoundingSphere)`
- `computeVisibility(args: TileDrawArgs)`
- `meetsScreenSpaceError(args: TileDrawArgs)`
- `extendRangeForContent(range: Range3d, matrix: Matrix4d, treeTransform: Transform, frustumPlanes?: FrustumPlanes)`
- `countDescendants()`
- `drawGraphics(args: TileDrawArgs)`
- `getRangeGraphic(context: SceneContext)`
- `addRangeGraphic(builder: GraphicBuilder, type: TileBoundingBoxes)`
- `getSizeProjectionCorners()`
- `clearLayers()`
