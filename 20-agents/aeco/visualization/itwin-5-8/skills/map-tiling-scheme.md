---
name: core-frontend-map-tiling-scheme
description: MapTilingScheme declarations from core-frontend
---

# MapTilingScheme

## Methods

- `longitudeToXFraction(longitude: number)`
- `xFractionToLongitude(xFraction: number)`
- `yFractionToLatitude(yFraction: number)`
- `latitudeToYFraction(latitude: number)`
- `getNumberOfXTilesAtLevel(level: number)`
- `getNumberOfYTilesAtLevel(level: number)`
- `getNumberOfXChildrenAtLevel(level: number)`
- `getNumberOfYChildrenAtLevel(level: number)`
- `tileXToFraction(x: number, level: number)`
- `tileYToFraction(y: number, level: number)`
- `xFractionToTileX(xFraction: number, level: number)`
- `yFractionToTileY(yFraction: number, level: number)`
- `tileXToLongitude(x: number, level: number)`
- `tileYToLatitude(y: number, level: number)`
- `tileXYToFraction(x: number, y: number, level: number, result?: Point2d)`
- `tileXYToCartographic(x: number, y: number, level: number, result: Cartographic, height?: number)`
- `tileXYToRectangle(x: number, y: number, level: number, result?: MapCartoRectangle)`
- `tileBordersNorthPole(row: number, level: number)`
- `tileBordersSouthPole(row: number, level: number)`
- `cartographicToTileXY(carto: Cartographic, level: number, result?: Point2d)`
- `fractionToCartographic(xFraction: number, yFraction: number, result: Cartographic, height?: number)`
- `cartographicToFraction(latitudeRadians: number, longitudeRadians: number, result: Point2d)`
- `computeMercatorFractionToDb(ecefToDb: Transform, bimElevationOffset: number, iModel: IModelConnection, applyTerrain: boolean)`
- `yFractionFlip(fraction: number)`
- `longitudeToXFraction(longitude: number)`
- `xFractionToLongitude(xFraction: number)`
- `yFractionToLatitude(yFraction: number)`
- `latitudeToYFraction(latitude: number)`
- `getNumberOfXTilesAtLevel(level: number)`
- `getNumberOfYTilesAtLevel(level: number)`
- `getNumberOfXChildrenAtLevel(level: number)`
- `getNumberOfYChildrenAtLevel(level: number)`
- `tileXToFraction(x: number, level: number)`
- `tileYToFraction(y: number, level: number)`
- `xFractionToTileX(xFraction: number, level: number)`
- `yFractionToTileY(yFraction: number, level: number)`
- `tileXToLongitude(x: number, level: number)`
- `tileYToLatitude(y: number, level: number)`
- `tileXYToFraction(x: number, y: number, level: number, result?: Point2d)`
- `tileXYToCartographic(x: number, y: number, level: number, result: Cartographic, height?: number)`
- `tileXYToRectangle(x: number, y: number, level: number, result?: MapCartoRectangle)`
- `tileBordersNorthPole(row: number, level: number)`
- `tileBordersSouthPole(row: number, level: number)`
- `cartographicToTileXY(carto: Cartographic, level: number, result?: Point2d)`
- `fractionToCartographic(xFraction: number, yFraction: number, result: Cartographic, height?: number)`
- `cartographicToFraction(latitudeRadians: number, longitudeRadians: number, result: Point2d)`
- `computeMercatorFractionToDb(ecefToDb: Transform, bimElevationOffset: number, iModel: IModelConnection, applyTerrain: boolean)`
- `yFractionFlip(fraction: number)`
