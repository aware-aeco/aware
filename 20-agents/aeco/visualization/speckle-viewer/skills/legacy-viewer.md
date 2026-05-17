---
name: viewer-legacy-viewer
description: LegacyViewer declarations from viewer
---

# LegacyViewer

## Methods

- `init()`
- `getRenderer()`
- `setSectionBox(boxData?: SpeckleViewer.ViewerState.SectionBoxData, offset?: number)`
- `getCurrentSectionBox()`
- `toggleSectionBox()`
- `sectionBoxOff()`
- `sectionBoxOn()`
- `selectObjects(objectIds: string[])`
- `resetSelection()`
- `hideObjects(objectIds: string[], stateKey?: string | undefined, includeDescendants?: boolean, ghost?: boolean)`
- `showObjects(objectIds: string[], stateKey?: string | undefined, includeDescendants?: boolean)`
- `isolateObjects(objectIds: string[], stateKey?: string | undefined, includeDescendants?: boolean, ghost?: boolean)`
- `unIsolateObjects(objectIds: string[], stateKey?: string | undefined, includeDescendants?: boolean)`
- `highlightObjects(objectIds: string[])`
- `resetHighlight()`
- `setColorFilter(property: PropertyInfo, ghost?: boolean)`
- `removeColorFilter()`
- `setUserObjectColors()`
- `resetFilters()`
- `getDataTree()`
- `getWorldTree()`
- `query(query: T)`
- `queryAsync(query: Query)`
- `zoom(objectIds?: string[], fit?: number, transition?: boolean)`
- `setOrthoCameraOn()`
- `setPerspectiveCameraOn()`
- `toggleCameraProjection()`
- `setLightConfiguration(config: SunLightConfiguration)`
- `getViews()`
- `setView(view: CanonicalView | SpeckleView | InlineView | PolarView, transition?: boolean)`
- `screenshot()`
- `explode(time: number)`
- `getObjects(id: string)`
- `loadObjectAsync(url: string, token?: string | undefined, enableCaching?: boolean, zoomToObject?: boolean)`
- `diff(urlA: string, urlB: string, mode: VisualDiffMode, authToken?: string)`
- `undiff()`
- `setDiffTime(_diffResult: DiffResult, time: number)`
- `setVisualDiffMode(_diffResult: DiffResult, mode: VisualDiffMode)`
- `enableMeasurements(value: boolean)`
- `setMeasurementOptions(options: MeasurementOptions)`
- `removeMeasurement()`
- `dispose()`
