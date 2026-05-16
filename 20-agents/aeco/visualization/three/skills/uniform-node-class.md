---
name: three-uniform-node-class
description: UniformNodeClass declarations from three
---

# UniformNodeClass

## Methods

- `setName(name: string)`
- `label(name: string)`
- `setGroup(group: UniformGroupNode)`
- `getGroup()`
- `getUniformHash(builder: NodeBuilder)`
- `onUpdate(callback: (frame: NodeFrame, self: this) => TValue | undefined, updateType: NodeUpdateType)`
- `getInputType(builder: NodeBuilder)`
- `generate(builder: NodeBuilder, output: string | null)`
