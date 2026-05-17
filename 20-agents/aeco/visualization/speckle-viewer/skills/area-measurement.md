---
name: viewer-area-measurement
description: AreaMeasurement declarations from viewer
---

# AreaMeasurement

## Methods

- `frameUpdate(camera: Camera, size: Vector2, bounds: Box3)`
- `locationUpdated(point: Vector3, normal: Vector3)`
- `locationSelected()`
- `addPoint(point: Vector3)`
- `removePoint()`
- `autoFinish()`
- `snap(ndcPoint: Vector2, _intersection: ExtendedMeshIntersection, outPoint: Vector3, outNormal: Vector3)`
- `update()`
- `raycast(raycaster: Raycaster, intersects: Array<Intersection>)`
- `highlight(value: boolean)`
- `updateClippingPlanes(planes: Plane[])`
- `toMeasurementData()`
- `fromMeasurementData(data: MeasurementData)`
