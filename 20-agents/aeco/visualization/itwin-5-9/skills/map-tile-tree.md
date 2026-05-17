---
name: core-frontend-map-tile-tree
description: MapTileTree declarations from core-frontend
---

# MapTileTree

## Methods

- `getImageryTreeState(imageryTreeId: string)`
- `cloneImageryTreeState()`
- `tileFromQuadId(quadId: QuadId)`
- `collectClassifierGraphics(args: TileDrawArgs, selectedTiles: RealityTile[])`
- `clearImageryTreesAndClassifiers()`
- `createPlanarChild(params: TileParams, quadId: QuadId, corners: Point3d[], normal: Vector3d, rectangle: MapCartoRectangle, chordHeight: number, heightRange?: Range1d)`
- `createGlobeChild(params: TileParams, quadId: QuadId, _rangeCorners: Point3d[], rectangle: MapCartoRectangle, ellipsoidPatch: EllipsoidPatch, heightRange?: Range1d)`
- `getChildHeightRange(quadId: QuadId, rectangle: MapCartoRectangle, parent: MapTile)`
- `getBaseRealityDepth(sceneContext: SceneContext)`
- `doCreateGlobeChildren(tile: Tile)`
- `doReprojectChildren(tile: Tile)`
- `getCornerRays(rectangle: MapCartoRectangle)`
- `pointAboveEllipsoid(point: Point3d)`
- `getCachedReprojectedPoints(gridPoints: Point3d[])`
- `loadReprojectionCache(tile: MapTile)`
- `getPlanarChildCorners(tile: MapTile, columnCount: number, rowCount: number, resolve: (childCorners: Point3d[][]) => void)`
- `reportTileVisibility(args: TileDrawArgs, selected: RealityTile[])`
- `getFractionalTileCorners(quadId: QuadId)`
- `getTileRectangle(quadId: QuadId)`
- `getLayerIndex(imageryTreeId: Id64String)`
- `getLayerTransparency(imageryTreeId: Id64String)`
- `getImageryTreeState(imageryTreeId: string)`
- `cloneImageryTreeState()`
- `tileFromQuadId(quadId: QuadId)`
- `collectClassifierGraphics(args: TileDrawArgs, selectedTiles: RealityTile[])`
- `clearImageryTreesAndClassifiers()`
- `createPlanarChild(params: TileParams, quadId: QuadId, corners: Point3d[], normal: Vector3d, rectangle: MapCartoRectangle, chordHeight: number, heightRange?: Range1d)`
- `createGlobeChild(params: TileParams, quadId: QuadId, _rangeCorners: Point3d[], rectangle: MapCartoRectangle, ellipsoidPatch: EllipsoidPatch, heightRange?: Range1d)`
- `getChildHeightRange(quadId: QuadId, rectangle: MapCartoRectangle, parent: MapTile)`
- `getBaseRealityDepth(sceneContext: SceneContext)`
- `doCreateGlobeChildren(tile: Tile)`
- `doReprojectChildren(tile: Tile)`
- `getCornerRays(rectangle: MapCartoRectangle)`
- `pointAboveEllipsoid(point: Point3d)`
- `getCachedReprojectedPoints(gridPoints: Point3d[])`
- `loadReprojectionCache(tile: MapTile)`
- `getPlanarChildCorners(tile: MapTile, columnCount: number, rowCount: number, resolve: (childCorners: Point3d[][]) => void)`
- `reportTileVisibility(args: TileDrawArgs, selected: RealityTile[])`
- `getFractionalTileCorners(quadId: QuadId)`
- `getTileRectangle(quadId: QuadId)`
- `getLayerIndex(imageryTreeId: Id64String)`
- `getLayerTransparency(imageryTreeId: Id64String)`
