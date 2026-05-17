---
name: core-frontend-render-target
description: RenderTarget declarations from core-frontend
---

# RenderTarget

## Methods

- `pickOverlayDecoration(_pt: XAndY)`
- `cssPixelsToDevicePixels(cssPixels: number, floor?: boolean)`
- `adjustPixelSizeForLOD(cssPixelSize: number)`
- `assignFrameStatsCollector(_collector: FrameStatsCollector)`
- `updateSolarShadows(_context: SceneContext | undefined)`
- `getPlanarClassifier(_id: string)`
- `createPlanarClassifier(_properties?: ActiveSpatialClassifier)`
- `getTextureDrape(_id: Id64String)`
- `createGraphicBuilder(options: CustomGraphicBuilderOptions | ViewportGraphicBuilderOptions)`
- `reset(_realityMapLayerChanged?: boolean)`
- `changeScene(scene: Scene)`
- `changeDynamics(foreground: GraphicList | undefined, overlay: GraphicList | undefined)`
- `changeDecorations(decorations: Decorations)`
- `changeRenderPlan(plan: RenderPlan)`
- `drawFrame(sceneMilSecElapsed?: number)`
- `overrideFeatureSymbology(_ovr: FeatureSymbology.Overrides)`
- `setHiliteSet(_hilited: HiliteSet)`
- `setFlashed(_elementId: Id64String, _intensity: number)`
- `onBeforeRender(_viewport: Viewport, _setSceneNeedRedraw: (redraw: boolean) => void)`
- `setViewRect(_rect: ViewRect, _temporary: boolean)`
- `onResized()`
- `updateViewRect()`
- `readPixels(rect: ViewRect, selector: Pixel.Selector, receiver: Pixel.Receiver, excludeNonLocatable: boolean, excludedElements?: Iterable<Id64String>)`
- `readImageBuffer(_args?: ReadImageBufferArgs)`
- `readImageToCanvas(_overlayCanvas?: HTMLCanvasElement)`
- `collectStatistics(_stats: RenderMemory.Statistics)`
- `setRenderToScreen(_toScreen: boolean)`
- `queryVisibleTileFeatures(_options: QueryTileFeaturesOptions, _iModel: IModelConnection, callback: QueryVisibleFeaturesCallback)`
- `pickOverlayDecoration(_pt: XAndY)`
- `cssPixelsToDevicePixels(cssPixels: number, floor?: boolean)`
- `adjustPixelSizeForLOD(cssPixelSize: number)`
- `assignFrameStatsCollector(_collector: FrameStatsCollector)`
- `updateSolarShadows(_context: SceneContext | undefined)`
- `getPlanarClassifier(_id: string)`
- `createPlanarClassifier(_properties?: ActiveSpatialClassifier)`
- `getTextureDrape(_id: Id64String)`
- `createGraphicBuilder(options: CustomGraphicBuilderOptions | ViewportGraphicBuilderOptions)`
- `reset(_realityMapLayerChanged?: boolean)`
- `changeScene(scene: Scene)`
- `changeDynamics(foreground: GraphicList | undefined, overlay: GraphicList | undefined)`
- `changeDecorations(decorations: Decorations)`
- `changeRenderPlan(plan: RenderPlan)`
- `drawFrame(sceneMilSecElapsed?: number)`
- `overrideFeatureSymbology(_ovr: FeatureSymbology.Overrides)`
- `setHiliteSet(_hilited: HiliteSet)`
- `setFlashed(_elementId: Id64String, _intensity: number)`
- `onBeforeRender(_viewport: Viewport, _setSceneNeedRedraw: (redraw: boolean) => void)`
- `setViewRect(_rect: ViewRect, _temporary: boolean)`
- `onResized()`
- `updateViewRect()`
- `readPixels(rect: ViewRect, selector: Pixel.Selector, receiver: Pixel.Receiver, excludeNonLocatable: boolean, excludedElements?: Iterable<Id64String>)`
- `readImageBuffer(_args?: ReadImageBufferArgs)`
- `readImageToCanvas(_overlayCanvas?: HTMLCanvasElement)`
- `collectStatistics(_stats: RenderMemory.Statistics)`
- `setRenderToScreen(_toScreen: boolean)`
- `queryVisibleTileFeatures(_options: QueryTileFeaturesOptions, _iModel: IModelConnection, callback: QueryVisibleFeaturesCallback)`
