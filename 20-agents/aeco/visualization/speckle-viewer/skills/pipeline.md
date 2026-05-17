---
name: viewer-pipeline
description: Pipeline declarations from viewer
---

# Pipeline

## Methods

- `onBeforePipelineRender()`
- `onAfterPipelineRender()`
- `getPass(name: string)`
- `setClippingPlanes(planes: Plane[])`
- `update(camera: PerspectiveCamera | OrthographicCamera)`
- `reset()`
- `render()`
- `resize(width: number, height: number)`
- `haltonNumber(base: number, index: number)`
- `generateHaltonJiters(length: number)`
- `createRenderTarget(options?: WebGLRenderTargetOptions, width?: number, height?: number)`
- `createMultipleRenderTarget(count: number, options?: WebGLRenderTargetOptions, width?: number, height?: number)`
