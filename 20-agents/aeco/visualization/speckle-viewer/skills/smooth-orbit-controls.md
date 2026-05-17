---
name: viewer-smooth-orbit-controls
description: SmoothOrbitControls declarations from viewer
---

# SmoothOrbitControls

## Methods

- `fromPositionAndTarget(position: Vector3, target: Vector3)`
- `jumpToGoal()`
- `fitToSphere(sphere: Sphere)`
- `getPosition()`
- `getTarget()`
- `getCurrentPosition()`
- `getCurrentTarget()`
- `isStationary()`
- `applyOptions(_options: SmoothOrbitControlsOptions)`
- `computeMinMaxRadius()`
- `setOrbit(goalTheta?: number, goalPhi?: number, goalRadius?: number)`
- `setRadius(radius: number)`
- `setFieldOfView(fov: number)`
- `setDamperDecayTime(decayMilliseconds: number)`
- `setTarget(x: number, y: number, z: number)`
- `adjustOrbit(deltaTheta: number, deltaPhi: number, deltaZoom: number)`
- `update(delta?: number)`
- `polarFromPivotal(position: Vector3)`
- `positionFromPivotal(origin: Vector3, quaternion: Quaternion)`
- `getPivotalOrigin(pivotPoint: Vector3, position: Vector3, quaternion: Quaternion)`
- `moveCamera()`
- `positionFromSpherical(spherical: Spherical, origin?: Vector3)`
- `quaternionFromSpherical(spherical: Spherical)`
- `userAdjustOrbit(deltaTheta: number, deltaPhi: number, deltaZoom: number)`
- `enableInteraction()`
- `disableInteraction()`
- `wrapAngle(radians: number)`
- `pixelLengthToSphericalAngle(pixelLength: number)`
- `twoTouchDistance(touchOne: Pointer, touchTwo: Pointer)`
- `handleSinglePointerMove(dx: number, dy: number)`
- `initializePan()`
- `movePan(dx: number, dy: number)`
- `filterOrbitToCursorHits(hit: ExtendedMeshIntersection)`
- `onTouchChange(event: PointerEvent)`
- `onMouseDown(event: MouseEvent)`
- `dispose()`
