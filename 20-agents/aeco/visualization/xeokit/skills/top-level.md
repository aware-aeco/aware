---
name: xeokit-sdk-top-level
description: TopLevel declarations from xeokit-sdk
---

# TopLevel

## Methods

- `touchPointSelector(viewer: Viewer, pointerCircle: PointerCircle, ray2WorldPos: Ray2WorldPos<T>)`
- `addMousePressListener(element: HTMLElement, onChange: (canvasPos: Vec2 | null) => void)`
- `addTouchPressListener(element: HTMLElement, cameraControl: CameraControl, pointerCircle: PointerCircle, onChange: (canvasPos: Vec2 | null) => void)`
- `buildBoxGeometry(cfg: buildBoxGeometryConfiguration)`
- `buildBoxLinesGeometry(cfg: buildBoxLinesGeometryConfiguration)`
- `buildBoxLinesGeometryFromAABB(cfg: buildBoxLinesGeometryFromAABBConfiguration)`
- `buildCylinderGeometry(cfg: buildCylinderGeometryConfiguration)`
- `buildGridGeometry(cfg: buildGridGeometryConfiguration)`
- `buildLineGeometry(cfg: buildLineGeometryConfiguration)`
- `buildPlaneGeometry(cfg: buildPlaneGeometryConfiguration)`
- `buildPolylineGeometry(cfg: buildPolylineGeometryConfiguration)`
- `buildPolylineGeometryFromCurve(cfg: buildPolylineGeometryFromCurveConfiguration)`
- `buildSphereGeometry(cfg: buildSphereGeometryConfiguration)`
- `buildTorusGeometry(cfg: buildTorusGeometryConfiguration)`
- `buildVectorTextGeometry(cfg: buildVectorTextGeometryConfiguration)`
- `load3DSGeometry(scene: Scene, cfg?: load3DSGeometryConfiguration)`
- `loadOBJGeometry(scene: Scene, cfg?: LoadObjGeometryConfiguration)`
