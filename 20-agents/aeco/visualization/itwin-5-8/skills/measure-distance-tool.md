---
name: core-frontend-measure-distance-tool
description: MeasureDistanceTool declarations from core-frontend
---

# MeasureDistanceTool

## Methods

- `allowView(vp: Viewport)`
- `isCompatibleViewport(vp: Viewport | undefined, isSelectedViewChange: boolean)`
- `isValidLocation(_ev: BeButtonEvent, _isButtonEvent: boolean)`
- `requireWriteableTarget()`
- `onPostInstall()`
- `onCleanup()`
- `onUnsuspend()`
- `showPrompt()`
- `setupAndPromptForNextAction()`
- `testDecorationHit(id: string)`
- `getSnapPoints()`
- `getDecorationGeometry(_hit: HitDetail)`
- `displayDynamicDistance(context: DecorateContext, points: Point3d[], adjustedPoints: Point3d[])`
- `displayDelta(context: DecorateContext, seg: any)`
- `createDecorations(context: DecorateContext, isSuspended: boolean)`
- `decorate(context: DecorateContext)`
- `decorateSuspended(context: DecorateContext)`
- `onMouseMotion(ev: BeButtonEvent)`
- `reportMeasurements()`
- `updateTotals()`
- `getMarkerToolTip(distance: number, slope: number, start: Point3d, end: Point3d, delta?: Vector3d)`
- `updateSelectedMarkerToolTip(seg: any, ev: BeButtonEvent, reopenToolTip: boolean)`
- `acceptNewSegments()`
- `getReferenceAxes(vp?: Viewport)`
- `onDataButtonDown(ev: BeButtonEvent)`
- `onResetButtonUp(ev: BeButtonEvent)`
- `onUndoPreviousStep()`
- `onRestartTool()`
- `allowView(vp: Viewport)`
- `isCompatibleViewport(vp: Viewport | undefined, isSelectedViewChange: boolean)`
- `isValidLocation(_ev: BeButtonEvent, _isButtonEvent: boolean)`
- `requireWriteableTarget()`
- `onPostInstall()`
- `onCleanup()`
- `onUnsuspend()`
- `showPrompt()`
- `setupAndPromptForNextAction()`
- `testDecorationHit(id: string)`
- `getSnapPoints()`
- `getDecorationGeometry(_hit: HitDetail)`
- `displayDynamicDistance(context: DecorateContext, points: Point3d[], adjustedPoints: Point3d[])`
- `displayDelta(context: DecorateContext, seg: any)`
- `createDecorations(context: DecorateContext, isSuspended: boolean)`
- `decorate(context: DecorateContext)`
- `decorateSuspended(context: DecorateContext)`
- `onMouseMotion(ev: BeButtonEvent)`
- `reportMeasurements()`
- `updateTotals()`
- `getMarkerToolTip(distance: number, slope: number, start: Point3d, end: Point3d, delta?: Vector3d)`
- `updateSelectedMarkerToolTip(seg: any, ev: BeButtonEvent, reopenToolTip: boolean)`
- `acceptNewSegments()`
- `getReferenceAxes(vp?: Viewport)`
- `onDataButtonDown(ev: BeButtonEvent)`
- `onResetButtonUp(ev: BeButtonEvent)`
- `onUndoPreviousStep()`
- `onRestartTool()`
