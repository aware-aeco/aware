---
name: core-frontend-accu-draw-hint-builder
description: AccuDrawHintBuilder declarations from core-frontend
---

# AccuDrawHintBuilder

## Methods

- `setOrigin(origin: Point3d)`
- `setMatrix(matrix: Matrix3d)`
- `setRotation(rowMatrix: Matrix3d)`
- `setXAxis(xAxis: Vector3d)`
- `setXAxis2(xAxis: Vector3d)`
- `setNormal(normal: Vector3d)`
- `setModePolar()`
- `setModeRectangular()`
- `setDistance(distance: number)`
- `setAngle(angle: number)`
- `activate()`
- `deactivate()`
- `processHintsImmediate()`
- `sendHints(activate?: boolean)`
- `setLastPoint(ev: BeButtonEvent)`
- `getBoresite(spacePt: Point3d, vp: Viewport, checkAccuDraw?: boolean, checkACS?: boolean)`
- `projectPointToPlaneInView(spacePt: Point3d, planePt: Point3d, planeNormal: Vector3d, vp: Viewport, checkAccuDraw?: boolean, checkACS?: boolean)`
- `projectPointToLineInView(spacePt: Point3d, linePt: Point3d, lineDirection: Vector3d, vp: Viewport, checkAccuDraw?: boolean, checkACS?: boolean)`
- `getCurrentRotation(vp: Viewport, checkAccuDraw: boolean, checkACS: boolean, matrix?: Matrix3d)`
- `getSnapRotation(snap: SnapDetail, matrix?: Matrix3d)`
- `getContextRotation(id: ContextRotationId, vp: Viewport)`
- `setOrigin(origin: Point3d)`
- `setMatrix(matrix: Matrix3d)`
- `setRotation(rowMatrix: Matrix3d)`
- `setXAxis(xAxis: Vector3d)`
- `setXAxis2(xAxis: Vector3d)`
- `setNormal(normal: Vector3d)`
- `setModePolar()`
- `setModeRectangular()`
- `setDistance(distance: number)`
- `setAngle(angle: number)`
- `activate()`
- `deactivate()`
- `processHintsImmediate()`
- `sendHints(activate?: boolean)`
- `setLastPoint(ev: BeButtonEvent)`
- `getBoresite(spacePt: Point3d, vp: Viewport, checkAccuDraw?: boolean, checkACS?: boolean)`
- `projectPointToPlaneInView(spacePt: Point3d, planePt: Point3d, planeNormal: Vector3d, vp: Viewport, checkAccuDraw?: boolean, checkACS?: boolean)`
- `projectPointToLineInView(spacePt: Point3d, linePt: Point3d, lineDirection: Vector3d, vp: Viewport, checkAccuDraw?: boolean, checkACS?: boolean)`
- `getCurrentRotation(vp: Viewport, checkAccuDraw: boolean, checkACS: boolean, matrix?: Matrix3d)`
- `getSnapRotation(snap: SnapDetail, matrix?: Matrix3d)`
- `getContextRotation(id: ContextRotationId, vp: Viewport)`
