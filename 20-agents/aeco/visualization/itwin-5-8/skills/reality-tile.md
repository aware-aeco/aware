---
name: core-frontend-reality-tile
description: RealityTile declarations from core-frontend
---

# RealityTile

## Methods

- `setContent(content: RealityTileContent)`
- `freeMemory()`
- `markUsed(args: TileDrawArgs)`
- `markDisplayed()`
- `isOccluded(_viewingSpace: ViewingSpace)`
- `requestContent(isCanceled: () => boolean)`
- `_loadChildren(resolve: (children: Tile[] | undefined) => void, reject: (error: Error) => void)`
- `readContent(data: TileRequest.ResponseData, system: RenderSystem, isCanceled?: () => boolean)`
- `computeLoadPriority(viewports: Iterable<Viewport>, users: Iterable<TileUser>)`
- `getContentClip()`
- `selectSecondaryTiles(_args: TileDrawArgs, _context: TraversalSelectionContext)`
- `preloadRealityTilesAtDepth(depth: number, context: TraversalSelectionContext, args: TileDrawArgs)`
- `preloadProtectedTiles(args: TileDrawArgs, context: TraversalSelectionContext)`
- `addBoundingGraphic(builder: GraphicBuilder, color: ColorDef)`
- `reproject(rootReprojection: Transform)`
- `allChildrenIncluded(tiles: Tile[])`
- `getLoadedRealityChildren(args: TileDrawArgs)`
- `forceSelectRealityTile()`
- `minimumVisibleFactor()`
- `selectRealityTiles(context: TraversalSelectionContext, args: TileDrawArgs, traversalDetails: TraversalDetails)`
- `selectRealityChildrenAsFallback(context: TraversalSelectionContext, args: TileDrawArgs, traversalDetails: TraversalDetails)`
- `selectRealityChildren(context: TraversalSelectionContext, args: TileDrawArgs, traversalDetails: TraversalDetails)`
- `purgeContents(olderThan: BeTimePoint, useProtectedTiles: boolean)`
- `computeVisibilityFactor(args: TileDrawArgs)`
- `getSizeProjectionCorners()`
- `loadAdditiveRefinementChildren(resolve: (children: Tile[]) => void)`
- `produceGraphics()`
- `disposeContents()`
- `collectTileGeometry(collector: TileGeometryCollector)`
- `setContent(content: RealityTileContent)`
- `freeMemory()`
- `markUsed(args: TileDrawArgs)`
- `markDisplayed()`
- `isOccluded(_viewingSpace: ViewingSpace)`
- `requestContent(isCanceled: () => boolean)`
- `_loadChildren(resolve: (children: Tile[] | undefined) => void, reject: (error: Error) => void)`
- `readContent(data: TileRequest.ResponseData, system: RenderSystem, isCanceled?: () => boolean)`
- `computeLoadPriority(viewports: Iterable<Viewport>, users: Iterable<TileUser>)`
- `getContentClip()`
- `selectSecondaryTiles(_args: TileDrawArgs, _context: TraversalSelectionContext)`
- `preloadRealityTilesAtDepth(depth: number, context: TraversalSelectionContext, args: TileDrawArgs)`
- `preloadProtectedTiles(args: TileDrawArgs, context: TraversalSelectionContext)`
- `addBoundingGraphic(builder: GraphicBuilder, color: ColorDef)`
- `reproject(rootReprojection: Transform)`
- `allChildrenIncluded(tiles: Tile[])`
- `getLoadedRealityChildren(args: TileDrawArgs)`
- `forceSelectRealityTile()`
- `minimumVisibleFactor()`
- `selectRealityTiles(context: TraversalSelectionContext, args: TileDrawArgs, traversalDetails: TraversalDetails)`
- `selectRealityChildrenAsFallback(context: TraversalSelectionContext, args: TileDrawArgs, traversalDetails: TraversalDetails)`
- `selectRealityChildren(context: TraversalSelectionContext, args: TileDrawArgs, traversalDetails: TraversalDetails)`
- `purgeContents(olderThan: BeTimePoint, useProtectedTiles: boolean)`
- `computeVisibilityFactor(args: TileDrawArgs)`
- `getSizeProjectionCorners()`
- `loadAdditiveRefinementChildren(resolve: (children: Tile[]) => void)`
- `produceGraphics()`
- `disposeContents()`
- `collectTileGeometry(collector: TileGeometryCollector)`
