---
name: viewer-top-level
description: TopLevel declarations from viewer
---

# TopLevel

## Methods

- `getRelativeOffset(referenceBox: Box3, offsetAmount?: number)`
- `makePerspectiveProjection(screenSize: Vector2, fov: number, near: number, far: number)`
- `getResourceUrls(url: string, authToken?: string)`
- `isAcceleratedBatchType(batch: Batch)`
- `isNoneBatchUpdateRange(range: BatchUpdateRange)`
- `isAllBatchUpdateRange(range: BatchUpdateRange, totalCount?: number)`
- `getConversionFactor(from: any, to?: string)`
- `normaliseName(unit: any)`
- `isPerspectiveCamera(camera: Camera)`
- `isOrthographicCamera(camera: Camera)`
- `computeOrthographicSize(distance: number, fov: number, aspect: number)`
- `intersectObjectWithRay(object: any, raycaster: any, includeInvisible: any)`
- `getPointer(event: any)`
- `onPointerDown(event: any)`
- `onPointerHover(event: any)`
- `onPointerMove(event: any)`
- `onPointerUp(event: any)`
- `applyMixins(derivedCtor: any, constructors: any[])`
