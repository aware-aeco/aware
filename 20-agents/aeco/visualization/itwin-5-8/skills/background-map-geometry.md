---
name: core-frontend-background-map-geometry
description: BackgroundMapGeometry declarations from core-frontend
---

# BackgroundMapGeometry

## Methods

- `getCartesianRange(iModel: IModelConnection, result?: Range3d)`
- `getCartesianTransitionDistance(iModel: IModelConnection)`
- `dbToCartographicFromGcs(db: XYAndZ[])`
- `dbToWGS84CartographicFromGcs(db: XYAndZ[])`
- `dbToCartographic(db: XYAndZ, result?: Cartographic)`
- `cartographicToDbFromGcs(cartographic: Cartographic[])`
- `cartographicToDbFromWgs84Gcs(cartographic: Cartographic[])`
- `cartographicToDb(cartographic: Cartographic, result?: Point3d)`
- `getEarthEllipsoid(radiusOffset?: number)`
- `getPlane(offset?: number)`
- `getRayIntersection(ray: Ray3d, positiveOnly: boolean)`
- `getPointHeight(point: Point3d)`
- `getFrustumIntersectionDepthRange(frustum: Frustum, bimRange: Range3d, heightRange?: Range1d, gridPlane?: Plane3dByOriginAndUnitNormal, doGlobalScope?: boolean)`
- `addFrustumDecorations(builder: GraphicBuilder, frustum: Frustum)`
- `getCartesianRange(iModel: IModelConnection, result?: Range3d)`
- `getCartesianTransitionDistance(iModel: IModelConnection)`
- `dbToCartographicFromGcs(db: XYAndZ[])`
- `dbToWGS84CartographicFromGcs(db: XYAndZ[])`
- `dbToCartographic(db: XYAndZ, result?: Cartographic)`
- `cartographicToDbFromGcs(cartographic: Cartographic[])`
- `cartographicToDbFromWgs84Gcs(cartographic: Cartographic[])`
- `cartographicToDb(cartographic: Cartographic, result?: Point3d)`
- `getEarthEllipsoid(radiusOffset?: number)`
- `getPlane(offset?: number)`
- `getRayIntersection(ray: Ray3d, positiveOnly: boolean)`
- `getPointHeight(point: Point3d)`
- `getFrustumIntersectionDepthRange(frustum: Frustum, bimRange: Range3d, heightRange?: Range1d, gridPlane?: Plane3dByOriginAndUnitNormal, doGlobalScope?: boolean)`
- `addFrustumDecorations(builder: GraphicBuilder, frustum: Frustum)`
