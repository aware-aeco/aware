---
name: three-node-manager
description: NodeManager declarations from three
---

# NodeManager

## Methods

- `updateGroup(nodeUniformsGroup: NodeUniformsGroup)`
- `getForRenderCacheKey(renderObject: RenderObject)`
- `getForRender(renderObject: RenderObject, useAsync?: boolean)`
- `getForRenderAsync(renderObject: RenderObject)`
- `getForRenderDeferred(renderObject: RenderObject)`
- `delete(object: unknown)`
- `getForCompute(computeNode: ComputeNode)`
- `getEnvironmentNode(scene: Scene)`
- `getBackgroundNode(scene: Scene)`
- `getFogNode(scene: Scene)`
- `getCacheKey(scene: Scene, lightsNode: LightsNode)`
- `updateBackground(scene: Scene)`
- `getCacheNode(type: string, object: object, callback: () => Node | undefined, forceUpdate?: boolean)`
- `updateFog(scene: Scene)`
- `updateEnvironment(scene: Scene)`
- `getNodeFrame()`
- `getNodeFrameForRender(renderObject: RenderObject)`
- `getOutputCacheKey()`
- `getOutputNode(outputTarget: Texture)`
- `updateBefore(renderObject: RenderObject)`
- `updateAfter(renderObject: RenderObject)`
- `updateForCompute(computeNode: ComputeNode)`
- `updateForRender(renderObject: RenderObject)`
- `needsRefresh(renderObject: RenderObject)`
