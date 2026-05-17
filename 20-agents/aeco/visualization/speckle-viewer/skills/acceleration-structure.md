---
name: viewer-acceleration-structure
description: AccelerationStructure declarations from viewer
---

# AccelerationStructure

## Methods

- `buildBVH(indices: Uint16Array | Uint32Array | undefined, position: Float32Array | Float64Array | undefined, options?: BVHOptions, transform?: Matrix4)`
- `raycast(ray: Ray, materialOrSide?: Side | Material | Material[])`
- `raycastFirst(ray: Ray, materialOrSide?: Side | Material | Material[])`
- `shapecast()`
- `transformInput(input: T)`
- `transformOutput(output: T)`
- `getBoundingBox(target?: Box3)`
- `getVertexAtIndex(index: number)`
