---
name: core-frontend-reality-tile-loader
description: RealityTileLoader declarations from core-frontend
---

# RealityTileLoader

## Methods

- `computeTilePriority(tile: Tile, viewports: Iterable<Viewport>, _users: Iterable<TileUser>)`
- `loadChildren(tile: RealityTile)`
- `getRequestChannel(tile: Tile)`
- `requestTileContent(tile: Tile, isCanceled: () => boolean)`
- `getBatchIdMap()`
- `forceTileLoad(_tile: Tile)`
- `processSelectedTiles(selected: Tile[], _args: TileDrawArgs)`
- `loadTileContent(tile: Tile, data: TileRequest.ResponseData, system: RenderSystem, isCanceled?: () => boolean)`
- `loadGeometryFromStream(tile: RealityTile, streamBuffer: ByteStream, system: RenderSystem)`
- `computeTileLocationPriority(tile: Tile, viewports: Iterable<Viewport>, location: Transform)`
- `computeTilePriority(tile: Tile, viewports: Iterable<Viewport>, _users: Iterable<TileUser>)`
- `loadChildren(tile: RealityTile)`
- `getRequestChannel(tile: Tile)`
- `requestTileContent(tile: Tile, isCanceled: () => boolean)`
- `getBatchIdMap()`
- `forceTileLoad(_tile: Tile)`
- `processSelectedTiles(selected: Tile[], _args: TileDrawArgs)`
- `loadTileContent(tile: Tile, data: TileRequest.ResponseData, system: RenderSystem, isCanceled?: () => boolean)`
- `loadGeometryFromStream(tile: RealityTile, streamBuffer: ByteStream, system: RenderSystem)`
- `computeTileLocationPriority(tile: Tile, viewports: Iterable<Viewport>, location: Transform)`
