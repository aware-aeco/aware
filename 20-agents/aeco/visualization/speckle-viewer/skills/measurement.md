---
name: viewer-measurement
description: Measurement declarations from viewer
---

# Measurement

## Methods

- `frameUpdate(camera: Camera | null, size: Vector2, _bounds?: Box3)`
- `update()`
- `raycast(_raycaster: Raycaster, _intersects: Array<Intersection>)`
- `highlight(_value: boolean)`
- `updateClippingPlanes(_planes: Plane[])`
- `locationUpdated(point?: Vector3, normal?: Vector3, ndcScreen?: Vector2)`
- `locationSelected(point?: Vector3, normal?: Vector3, ndcScreen?: Vector2)`
- `toMeasurementData()`
- `fromMeasurementData(data: MeasurementData)`
