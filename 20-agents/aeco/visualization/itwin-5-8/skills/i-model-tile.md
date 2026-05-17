---
name: core-frontend-i-model-tile
description: IModelTile declarations from core-frontend
---

# IModelTile

## Methods

- `requestContent()`
- `readContent(data: TileRequest.ResponseData, system: RenderSystem, isCanceled?: () => boolean)`
- `setContent(content: IModelTileContent)`
- `_loadChildren(resolve: (children: Tile[]) => void, reject: (error: Error) => void)`
- `addRangeGraphic(builder: GraphicBuilder, type: TileBoundingBoxes)`
- `pruneChildren(olderThan: BeTimePoint)`
- `selectTiles(selected: Tile[], args: TileDrawArgs, numSkipped: number)`
- `clearLayers()`
- `requestContent()`
- `readContent(data: TileRequest.ResponseData, system: RenderSystem, isCanceled?: () => boolean)`
- `setContent(content: IModelTileContent)`
- `_loadChildren(resolve: (children: Tile[]) => void, reject: (error: Error) => void)`
- `addRangeGraphic(builder: GraphicBuilder, type: TileBoundingBoxes)`
- `pruneChildren(olderThan: BeTimePoint)`
- `selectTiles(selected: Tile[], args: TileDrawArgs, numSkipped: number)`
- `clearLayers()`
