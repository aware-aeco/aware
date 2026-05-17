---
name: core-frontend-scene-compositor
description: SceneCompositor declarations from core-frontend
---

# SceneCompositor

## Methods

- `preDraw()`
- `draw(_commands: RenderCommands)`
- `drawForReadPixels(_commands: RenderCommands, sceneOverlays: GraphicList, worldOverlayDecorations: GraphicList | undefined, viewOverlayDecorations: GraphicList | undefined)`
- `readPixels(rect: ViewRect, selector: Pixel.Selector)`
- `readDepthAndOrder(rect: ViewRect)`
- `readFeatureIds(rect: ViewRect)`
- `updateSolarShadows(context: SceneContext | undefined)`
- `drawPrimitive(primitive: Primitive, exec: ShaderProgramExecutor, outputsToPick: boolean)`
- `forceBufferChange()`
- `create(target: Target)`
- `collectStatistics(stats: RenderMemory.Statistics)`
- `preDraw()`
- `draw(_commands: RenderCommands)`
- `drawForReadPixels(_commands: RenderCommands, sceneOverlays: GraphicList, worldOverlayDecorations: GraphicList | undefined, viewOverlayDecorations: GraphicList | undefined)`
- `readPixels(rect: ViewRect, selector: Pixel.Selector)`
- `readDepthAndOrder(rect: ViewRect)`
- `readFeatureIds(rect: ViewRect)`
- `updateSolarShadows(context: SceneContext | undefined)`
- `drawPrimitive(primitive: Primitive, exec: ShaderProgramExecutor, outputsToPick: boolean)`
- `forceBufferChange()`
- `create(target: Target)`
- `collectStatistics(stats: RenderMemory.Statistics)`
