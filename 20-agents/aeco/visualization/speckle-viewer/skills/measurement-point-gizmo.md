---
name: viewer-measurement-point-gizmo
description: MeasurementPointGizmo declarations from viewer
---

# MeasurementPointGizmo

## Methods

- `enable(normalIndicator: boolean, line: boolean, point: boolean, text: boolean)`
- `frameUpdate(camera: Camera, size: Vector2)`
- `updateNormalIndicator(position: Vector3, normal: Vector3)`
- `updatePoint(position: Vector3)`
- `updateLine(points: Vector3[])`
- `updateText(value: string, position?: Vector3, quaternion?: Quaternion, scale?: Vector3)`
- `updateStyle()`
- `raycast(raycaster: Raycaster, intersects: Array<Intersection>)`
- `updateClippingPlanes(planes: Plane[])`
