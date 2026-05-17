---
name: viewer-base-g-pass
description: BaseGPass declarations from viewer
---

# BaseGPass

## Methods

- `setClearColor(color: number, alpha: number)`
- `setClearFlags(flags: number)`
- `setLayers(layers: ObjectLayers[])`
- `enableLayer(layer: ObjectLayers, value: boolean)`
- `setVisibility(objectVisibility: ObjectVisibility | null, visibilityFunction?: (renderer: SpeckleRenderer) => Record<string, BatchUpdateRange>)`
- `setJitter(value: boolean)`
- `applyLayers(camera: Camera | null)`
- `clear(renderer: WebGLRenderer)`
- `render(renderer: WebGLRenderer, camera?: PerspectiveCamera | OrthographicCamera | null, scene?: Scene)`
- `setSize(width: number, height: number)`
