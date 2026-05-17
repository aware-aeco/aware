---
name: core-frontend-frame-buffer
description: FrameBuffer declarations from core-frontend
---

# FrameBuffer

## Methods

- `getColor(ndx: number)`
- `getColorTargets(useMSBuffers: boolean, ndx: number)`
- `create(colorTextures: TextureHandle[], depthBuffer?: DepthBuffer, colorMsBuffers?: RenderBufferMultiSample[], msFilters?: GL.MultiSampling.Filter[], depthBufferMs?: DepthBuffer)`
- `bind(bindAttachments?: boolean, bindMS?: boolean)`
- `unbind()`
- `suspend()`
- `markTargetsDirty()`
- `blitMsBuffersToTextures(blitDepth: boolean, ndx?: number)`
- `invalidate(invDepth: boolean, invStencil: boolean, withMultiSampling: boolean, indices?: number[])`
- `getColor(ndx: number)`
- `getColorTargets(useMSBuffers: boolean, ndx: number)`
- `create(colorTextures: TextureHandle[], depthBuffer?: DepthBuffer, colorMsBuffers?: RenderBufferMultiSample[], msFilters?: GL.MultiSampling.Filter[], depthBufferMs?: DepthBuffer)`
- `bind(bindAttachments?: boolean, bindMS?: boolean)`
- `unbind()`
- `suspend()`
- `markTargetsDirty()`
- `blitMsBuffersToTextures(blitDepth: boolean, ndx?: number)`
- `invalidate(invDepth: boolean, invStencil: boolean, withMultiSampling: boolean, indices?: number[])`
