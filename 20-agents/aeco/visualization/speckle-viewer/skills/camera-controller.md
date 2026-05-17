---
name: viewer-camera-controller
description: CameraController declarations from viewer
---

# CameraController

## Methods

- `default()`
- `on(eventType: T, listener: (arg: CameraEventPayload[T]) => void)`
- `getTarget()`
- `getPosition()`
- `toggleControls()`
- `setCameraView(objectIds: string[] | undefined, transition: boolean | undefined, fit?: number)`
- `setCameraView(view: CanonicalView | SpeckleView | InlineView | PolarView, transition: boolean | undefined, fit?: number)`
- `setCameraView(bounds: Box3, transition: boolean | undefined, fit?: number)`
- `onEarlyUpdate(_delta?: number)`
- `onLateUpdate()`
- `onResize()`
- `setPerspectiveCameraOn()`
- `setOrthoCameraOn()`
- `toggleCameras()`
- `setupOrthoCamera()`
- `setupPerspectiveCamera()`
- `disableRotations()`
- `enableRotations()`
- `updateCameraPlanes(targetVolume?: Box3, offsetScale?: number)`
- `computeNearCameraPlaneEmpiric(targetVolume?: Box3, offsetScale?: number)`
- `computeNearCameraPlaneAccurate(targetVolume?: Box3, offsetScale?: number, fallback?: number)`
- `updateFarCameraPlane()`
- `getClosestGeometryDistance(fallback?: number)`
- `zoom(objectIds?: string[], fit?: number, transition?: boolean)`
- `zoomExtents(fit?: number, transition?: boolean)`
- `zoomToBox(box: Box3, fit?: number, transition?: boolean)`
- `fitToRadius(radius: number)`
- `isSpeckleView(view: CanonicalView | SpeckleView | InlineView | PolarView)`
- `isCanonicalView(view: CanonicalView | SpeckleView | InlineView | PolarView)`
- `isInlineView(view: CanonicalView | SpeckleView | InlineView | PolarView)`
- `isPolarView(view: CanonicalView | SpeckleView | InlineView | PolarView)`
- `isBox3(view: CanonicalView | SpeckleView | InlineView | PolarView | Box3)`
- `setView(view: CanonicalView | SpeckleView | InlineView | PolarView, transition?: boolean)`
- `setViewSpeckle(view: SpeckleView, transition?: boolean)`
- `setViewCanonical(side: string, transition?: boolean)`
- `setViewInline(view: InlineView, transition?: boolean)`
