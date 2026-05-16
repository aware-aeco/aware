---
name: three-node-class
description: NodeClass declarations from three
---

# NodeClass

## Methods

- `onUpdate(callback: (this: this, frame: NodeFrame) => unknown, updateType: NodeUpdateType)`
- `update(frame: NodeFrame)`
- `onFrameUpdate(callback: (this: this, frame: NodeFrame) => void)`
- `onRenderUpdate(callback: (this: this, frame: NodeFrame) => void)`
- `onObjectUpdate(callback: (this: this, frame: NodeFrame) => void)`
- `onReference(callback: (this: this, frame: NodeBuilder | NodeFrame) => unknown)`
- `updateReference(state: NodeBuilder | NodeFrame)`
- `isGlobal(builder: NodeBuilder)`
- `getChildren()`
- `dispose()`
- `traverse(callback: (node: Node) => void)`
- `getCacheKey(force?: boolean, ignores?: Set<Node>)`
- `customCacheKey()`
- `getScope()`
- `getHash(builder: NodeBuilder)`
- `getUpdateType()`
- `getUpdateBeforeType()`
- `getUpdateAfterType()`
- `getElementType(builder: NodeBuilder)`
- `getMemberType(builder: NodeBuilder, name: string)`
- `getNodeType(builder: NodeBuilder, output?: string)`
- `generateNodeType(builder: NodeBuilder, output?: string)`
- `getShared(builder: NodeBuilder)`
- `getArrayCount(builder: NodeBuilder)`
- `setup(builder: NodeBuilder)`
- `analyze(builder: NodeBuilder, output?: Node | null)`
- `generate(builder: NodeBuilder, output?: string | null)`
- `updateBefore(frame: NodeFrame)`
- `updateAfter(frame: NodeFrame)`
- `before(node: Node)`
- `build(builder: NodeBuilder, output?: (string | Node) | null)`
- `getSerializeChildren()`
- `serialize(json: NodeJSONIntermediateOutputData)`
- `deserialize(json: NodeJSONInputData)`
- `toJSON(meta?: NodeJSONMeta | string)`
