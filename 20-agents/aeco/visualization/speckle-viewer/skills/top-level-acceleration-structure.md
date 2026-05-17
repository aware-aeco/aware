---
name: viewer-top-level-acceleration-structure
description: TopLevelAccelerationStructure declarations from viewer
---

# TopLevelAccelerationStructure

## Methods

- `refit()`
- `raycast(ray: Ray, tasOnly?: boolean, materialOrSide?: Side | Material | Material[])`
- `raycastFirst(ray: Ray, tasOnly?: boolean, materialOrSide?: Side | Material | Material[])`
- `shapecast(callbacks: ExtendedShapeCastCallbacks)`
- `closestPointToPoint(point: Vector3)`
- `closestPointToPointHalfplane(point: Vector3, planeNormal: Vector3, fallback?: number, target?: HitPointInfo, minThreshold?: number, maxThreshold?: number)`
- `getBoundingBox(target: Box3)`
