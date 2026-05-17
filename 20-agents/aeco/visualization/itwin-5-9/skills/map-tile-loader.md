---
name: core-frontend-map-tile-loader
description: MapTileLoader declarations from core-frontend
---

# MapTileLoader

## Methods

- `isTileAvailable(quadId: QuadId)`
- `getFeatureIndex(layerModelId: Id64String)`
- `getRequestChannel(_tile: Tile)`
- `requestTileContent(tile: MapTile, isCanceled: () => boolean)`
- `forceTileLoad(tile: MapTile)`
- `loadTileContent(tile: MapTile, data: TileRequest.ResponseData, system: RenderSystem, isCanceled?: () => boolean)`
- `loadPolyfaces()`
- `getChildHeightRange(quadId: QuadId, rectangle: MapCartoRectangle, parent: MapTile)`
- `loadChildren(_tile: RealityTile)`
- `isTileAvailable(quadId: QuadId)`
- `getFeatureIndex(layerModelId: Id64String)`
- `getRequestChannel(_tile: Tile)`
- `requestTileContent(tile: MapTile, isCanceled: () => boolean)`
- `forceTileLoad(tile: MapTile)`
- `loadTileContent(tile: MapTile, data: TileRequest.ResponseData, system: RenderSystem, isCanceled?: () => boolean)`
- `loadPolyfaces()`
- `getChildHeightRange(quadId: QuadId, rectangle: MapCartoRectangle, parent: MapTile)`
- `loadChildren(_tile: RealityTile)`
