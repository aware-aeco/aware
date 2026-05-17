---
name: core-frontend-map-tile
description: MapTile declarations from core-frontend
---

# MapTile

## Methods

- `getRangeCorners(result: Point3d[])`
- `getSizeProjectionCorners()`
- `markUsed(args: TileDrawArgs)`
- `tileFromQuadId(quadId: QuadId)`
- `addBoundingGraphic(builder: GraphicBuilder, color: ColorDef)`
- `getContentClip()`
- `setNotFound()`
- `getGraphic(_system: RenderSystem, _texture: RenderTexture)`
- `isOccluded(viewingSpace: ViewingSpace)`
- `_loadChildren(resolve: (children: Tile[] | undefined) => void, _reject: (error: Error) => void)`
- `computeRangeCorners(corners: Point3d[], normal: Vector3d, chordHeight: number, result?: Point3d[], heightRange?: Range1d)`
- `isRegionCulled(args: TileDrawArgs)`
- `isContentCulled(args: TileDrawArgs)`
- `produceGraphics()`
- `getClipShape()`
- `_collectStatistics(stats: RenderMemory.Statistics)`
- `adjustHeights(minHeight: number, maxHeight: number)`
- `getProjection(heightRange?: Range1d)`
- `selectSecondaryTiles(args: TileDrawArgs, context: TraversalSelectionContext)`
- `forceSelectRealityTile()`
- `minimumVisibleFactor()`
- `getDrapeTextures()`
- `setContent(content: TerrainTileContent)`
- `freeMemory()`
- `disposeContents()`
- `getRangeCorners(result: Point3d[])`
- `getSizeProjectionCorners()`
- `markUsed(args: TileDrawArgs)`
- `tileFromQuadId(quadId: QuadId)`
- `addBoundingGraphic(builder: GraphicBuilder, color: ColorDef)`
- `getContentClip()`
- `setNotFound()`
- `getGraphic(_system: RenderSystem, _texture: RenderTexture)`
- `isOccluded(viewingSpace: ViewingSpace)`
- `_loadChildren(resolve: (children: Tile[] | undefined) => void, _reject: (error: Error) => void)`
- `computeRangeCorners(corners: Point3d[], normal: Vector3d, chordHeight: number, result?: Point3d[], heightRange?: Range1d)`
- `isRegionCulled(args: TileDrawArgs)`
- `isContentCulled(args: TileDrawArgs)`
- `produceGraphics()`
- `getClipShape()`
- `_collectStatistics(stats: RenderMemory.Statistics)`
- `adjustHeights(minHeight: number, maxHeight: number)`
- `getProjection(heightRange?: Range1d)`
- `selectSecondaryTiles(args: TileDrawArgs, context: TraversalSelectionContext)`
- `forceSelectRealityTile()`
- `minimumVisibleFactor()`
- `getDrapeTextures()`
- `setContent(content: TerrainTileContent)`
- `freeMemory()`
- `disposeContents()`
