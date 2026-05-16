---
name: three-web-g-l-renderer
description: WebGLRenderer declarations from three
---

# WebGLRenderer

## Methods

- `getContext()`
- `getContextAttributes()`
- `forceContextLoss()`
- `forceContextRestore()`
- `getPixelRatio()`
- `setPixelRatio(value: number)`
- `getSize(target: Vector2)`
- `setSize(width: number, height: number, updateStyle?: boolean)`
- `getDrawingBufferSize(target: Vector2)`
- `setDrawingBufferSize(width: number, height: number, pixelRatio: number)`
- `setEffects(effects: Effect[] | null)`
- `getCurrentViewport(target: Vector4)`
- `getViewport(target: Vector4)`
- `setViewport(x: Vector4 | number, y?: number, width?: number, height?: number)`
- `getScissor(target: Vector4)`
- `setScissor(x: Vector4 | number, y?: number, width?: number, height?: number)`
- `getScissorTest()`
- `setScissorTest(enable: boolean)`
- `setOpaqueSort(method: ((a: any, b: any) => number) | null)`
- `setTransparentSort(method: ((a: any, b: any) => number) | null)`
- `getClearColor(target: Color)`
- `setClearColor(color: ColorRepresentation, alpha?: number)`
- `getClearAlpha()`
- `setClearAlpha(alpha: number)`
- `clear(color?: boolean, depth?: boolean, stencil?: boolean)`
- `clearColor()`
- `clearDepth()`
- `clearStencil()`
- `setNodesHandler(nodesHandler: NodesHandler)`
- `dispose()`
- `renderBufferDirect()`
- `setAnimationLoop(callback: XRFrameRequestCallback | null)`
- `render(scene: Object3D, camera: Camera)`
- `getActiveCubeFace()`
- `getActiveMipmapLevel()`
- `getRenderTarget()`
- `setRenderTarget()`
- `readRenderTargetPixels()`
- `readRenderTargetPixelsAsync()`
- `copyFramebufferToTexture(texture: Texture, position?: Vector2 | null, level?: number)`
- `copyTextureToTexture()`
- `initRenderTarget(target: WebGLRenderTarget)`
- `initTexture(texture: Texture)`
- `resetState()`
