---
name: core-frontend-imagery-tile-loader
description: ImageryTileLoader declarations from core-frontend
---

# ImageryTileLoader

## Methods

- `computeTilePriority(tile: Tile)`
- `addLogoCards(cards: HTMLTableElement, vp: ScreenViewport)`
- `addAttributions(cards: HTMLTableElement, vp: ScreenViewport)`
- `getToolTip(strings: string[], quadId: QuadId, carto: Cartographic, tree: ImageryMapTileTree)`
- `getMapFeatureInfo(featureInfos: MapLayerFeatureInfo[], quadId: QuadId, carto: Cartographic, tree: ImageryMapTileTree, hit: HitDetail, options?: MapFeatureInfoOptions)`
- `generateChildIds(tile: ImageryMapTile, resolveChildren: (childIds: QuadId[]) => void)`
- `loadChildren(_tile: RealityTile)`
- `requestTileContent(tile: Tile, _isCanceled: () => boolean)`
- `getRequestChannel(_tile: Tile)`
- `loadTileContent(tile: Tile, data: TileRequest.ResponseData, system: RenderSystem)`
- `computeTilePriority(tile: Tile)`
- `addLogoCards(cards: HTMLTableElement, vp: ScreenViewport)`
- `addAttributions(cards: HTMLTableElement, vp: ScreenViewport)`
- `getToolTip(strings: string[], quadId: QuadId, carto: Cartographic, tree: ImageryMapTileTree)`
- `getMapFeatureInfo(featureInfos: MapLayerFeatureInfo[], quadId: QuadId, carto: Cartographic, tree: ImageryMapTileTree, hit: HitDetail, options?: MapFeatureInfoOptions)`
- `generateChildIds(tile: ImageryMapTile, resolveChildren: (childIds: QuadId[]) => void)`
- `loadChildren(_tile: RealityTile)`
- `requestTileContent(tile: Tile, _isCanceled: () => boolean)`
- `getRequestChannel(_tile: Tile)`
- `loadTileContent(tile: Tile, data: TileRequest.ResponseData, system: RenderSystem)`
