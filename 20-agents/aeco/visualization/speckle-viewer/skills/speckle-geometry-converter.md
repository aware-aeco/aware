---
name: viewer-speckle-geometry-converter
description: SpeckleGeometryConverter declarations from viewer
---

# SpeckleGeometryConverter

## Methods

- `getSpeckleType(node: NodeData)`
- `convertNodeToGeometryData(node: NodeData)`
- `disposeNodeGeometryData(node: NodeData)`
- `View3DToGeometryData(node: NodeData)`
- `TransformToGeometryData(node: NodeData)`
- `BlockInstanceToGeometryData(node: NodeData)`
- `RevitInstanceToGeometryData(node: NodeData)`
- `InstanceProxyToGeometyData(node: NodeData)`
- `PointcloudToGeometryData(node: NodeData)`
- `BrepToGeometryData(node: NodeData)`
- `RegionGeometyData(node: NodeData)`
- `MeshToGeometryData(node: NodeData)`
- `TextToGeometryData(node: NodeData)`
- `PointToGeometryData(node: NodeData)`
- `LineToGeometryData(node: NodeData)`
- `PolylineToGeometryData(node: NodeData)`
- `BoxToGeometryData(node: NodeData)`
- `PolycurveToGeometryData(node: NodeData)`
- `CurveToGeometryData(node: NodeData)`
- `CircleToGeometryData(node: NodeData)`
- `ArcToGeometryData(node: NodeData)`
- `EllipseToGeometryData(node: NodeData)`
- `getCircularCurvePoints()`
- `PointToVector3()`
- `PointToFloatArray()`
- `FlattenVector3Array(input: Vector3[] | Vector2[])`
- `unpackColors(int32Colors: ChunkArray, tolinear?: boolean)`
- `srgbToLinear(x: number)`
- `chunkArrayHasData(chunks: Array<DataChunk>)`
