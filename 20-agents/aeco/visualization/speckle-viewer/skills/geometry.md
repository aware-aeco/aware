---
name: viewer-geometry
description: Geometry declarations from viewer
---

# Geometry

## Methods

- `updateRTEGeometry(geometry: BufferGeometry, doublePositions: Float64Array | Float32Array)`
- `mergeGeometryAttribute(attributes: ChunkArray[], target: Float32Array | Float64Array)`
- `mergeIndexAttribute(indexAttributes: ChunkArray[], positionAttributes: ChunkArray[])`
- `mergeGeometryData(geometries: GeometryData[])`
- `transformGeometryData(geometryData: GeometryData, m: Matrix4 | null)`
- `transformArray(array: number[] | Float32Array | Float64Array, m: Matrix4 | null, offset?: number, count?: number)`
- `isMatrix4Identity(matrix: Matrix4)`
- `unpackColors(int32Colors: number[])`
- `DoubleToHighLowVector(input: Vector3, low: Vector3, high: Vector3)`
- `DoubleToHighLowBuffer(input: ArrayLike<number>, position_low: number[] | Float32Array, position_high: number[] | Float32Array)`
- `needsRTE(bounds: Box3)`
- `getFP32ProjectionDelta(point: Vector3, projection: Matrix4, screenSize: Vector2, relativeOffset?: number)`
- `computeVertexNormalsBuffer(buffer: number[], position: number[], index: number[])`
- `computeVertexNormalsBufferVirtual(buffer: number[], position: ChunkArray, index: ChunkArray, flip?: boolean)`
- `flipNormalsBuffer(buffer: Float32Array)`
- `computeVertexNormals(buffer: BufferGeometry, positions: Float64Array | Float32Array)`
- `triangulatePolygon(points: Vector2[])`
