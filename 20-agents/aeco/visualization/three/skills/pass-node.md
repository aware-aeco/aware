---
name: three-pass-node
description: PassNode declarations from three
---

# PassNode

## Methods

- `setResolutionScale(resolution: number)`
- `getResolutionScale()`
- `setResolution(resolution: number)`
- `getResolution()`
- `setLayers(layers: Layers)`
- `getLayers()`
- `setMRT(mrt: MRTNode | null)`
- `getMRT()`
- `getTexture(name: string)`
- `getPreviousTexture(name: string)`
- `toggleTexture(name: string)`
- `getTextureNode(name?: string)`
- `getPreviousTextureNode(name?: string)`
- `getViewZNode(name?: string)`
- `getLinearDepthNode(name?: string)`
- `compileAsync(renderer: Renderer)`
- `setSize(width: number, height: number)`
- `setScissor(x: number, y: number, width: number, height: number)`
- `setScissor(x: Vector4)`
- `setViewport(x: number, y: number, width: number, height: number)`
- `setViewport(x: Vector4)`
- `setPixelRatio(pixelRatio: number)`
- `dispose()`
