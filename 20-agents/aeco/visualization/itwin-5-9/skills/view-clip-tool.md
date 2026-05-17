---
name: core-frontend-view-clip-tool
description: ViewClipTool declarations from core-frontend
---

# ViewClipTool

## Methods

- `enumAsOrientationMessage(str: string)`
- `requireWriteableTarget()`
- `isCompatibleViewport(vp: Viewport | undefined, isSelectedViewChange: boolean)`
- `onPostInstall()`
- `onUnsuspend()`
- `onRestartTool()`
- `showPrompt()`
- `setupAndPromptForNextAction()`
- `onResetButtonUp(_ev: BeButtonEvent)`
- `getPlaneInwardNormal(orientation: ContextRotationId, viewport: Viewport)`
- `enableClipVolume(viewport: Viewport)`
- `setViewClip(viewport: Viewport, clip?: ClipVector)`
- `doClipToConvexClipPlaneSet(viewport: Viewport, planes: ConvexClipPlaneSet)`
- `doClipToPlane(viewport: Viewport, origin: Point3d, normal: Vector3d, clearExistingPlanes: boolean)`
- `doClipToShape(viewport: Viewport, xyPoints: Point3d[], transform?: Transform, zLow?: number, zHigh?: number)`
- `doClipToRange(viewport: Viewport, range: Range3d, transform?: Transform)`
- `doClipClear(viewport: Viewport)`
- `getClipRayTransformed(origin: Point3d, direction: Vector3d, transform?: Transform)`
- `getOffsetValueTransformed(offset: number, transform?: Transform)`
- `addClipPlanesLoops(builder: GraphicBuilder, loops: GeometryQuery[], outline: boolean)`
- `drawClip(context: DecorateContext, clip: ClipVector, viewExtents?: Range3d, options?: DrawClipOptions)`
- `drawClipShape(context: DecorateContext, shape: ClipShape, extents: Range1d, color: ColorDef, weight: number, id?: string)`
- `getClipShapePoints(shape: ClipShape, z: number)`
- `getClipShapeExtents(shape: ClipShape, viewRange: Range3d)`
- `isSingleClipShape(clip: ClipVector)`
- `drawClipPlanesLoops(context: DecorateContext, loops: GeometryQuery[], color: ColorDef, weight: number, dashed?: boolean, fill?: ColorDef, id?: string)`
- `isSingleConvexClipPlaneSet(clip: ClipVector)`
- `isSingleClipPlane(clip: ClipVector)`
- `areClipsEqual(clipA: ClipVector, clipB: ClipVector)`
- `hasClip(viewport: Viewport)`
- `enumAsOrientationMessage(str: string)`
- `requireWriteableTarget()`
- `isCompatibleViewport(vp: Viewport | undefined, isSelectedViewChange: boolean)`
- `onPostInstall()`
- `onUnsuspend()`
- `onRestartTool()`
- `showPrompt()`
- `setupAndPromptForNextAction()`
- `onResetButtonUp(_ev: BeButtonEvent)`
- `getPlaneInwardNormal(orientation: ContextRotationId, viewport: Viewport)`
- `enableClipVolume(viewport: Viewport)`
- `setViewClip(viewport: Viewport, clip?: ClipVector)`
- `doClipToConvexClipPlaneSet(viewport: Viewport, planes: ConvexClipPlaneSet)`
- `doClipToPlane(viewport: Viewport, origin: Point3d, normal: Vector3d, clearExistingPlanes: boolean)`
- `doClipToShape(viewport: Viewport, xyPoints: Point3d[], transform?: Transform, zLow?: number, zHigh?: number)`
- `doClipToRange(viewport: Viewport, range: Range3d, transform?: Transform)`
- `doClipClear(viewport: Viewport)`
- `getClipRayTransformed(origin: Point3d, direction: Vector3d, transform?: Transform)`
- `getOffsetValueTransformed(offset: number, transform?: Transform)`
- `addClipPlanesLoops(builder: GraphicBuilder, loops: GeometryQuery[], outline: boolean)`
- `drawClip(context: DecorateContext, clip: ClipVector, viewExtents?: Range3d, options?: DrawClipOptions)`
- `drawClipShape(context: DecorateContext, shape: ClipShape, extents: Range1d, color: ColorDef, weight: number, id?: string)`
- `getClipShapePoints(shape: ClipShape, z: number)`
- `getClipShapeExtents(shape: ClipShape, viewRange: Range3d)`
- `isSingleClipShape(clip: ClipVector)`
- `drawClipPlanesLoops(context: DecorateContext, loops: GeometryQuery[], color: ColorDef, weight: number, dashed?: boolean, fill?: ColorDef, id?: string)`
- `isSingleConvexClipPlaneSet(clip: ClipVector)`
- `isSingleClipPlane(clip: ClipVector)`
- `areClipsEqual(clipA: ClipVector, clipB: ClipVector)`
- `hasClip(viewport: Viewport)`
