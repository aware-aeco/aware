---
name: viewer-render-tree
description: RenderTree declarations from viewer
---

# RenderTree

## Methods

- `buildRenderTree(geometryConverter: GeometryConverter, countCallback?: (count: number) => void)`
- `computeTransform(node: TreeNode)`
- `getInstances()`
- `getDuplicates()`
- `getRenderableRenderViews(...types: SpeckleType[])`
- `getRenderableNodes(...types: SpeckleType[])`
- `getRenderViewsForNode(node: TreeNode)`
- `getRenderViewNodesForNode(node: TreeNode)`
- `getRenderViewsForNodeId(id: string, subtreeId?: number)`
- `getAtomicParent(node: TreeNode)`
- `purge()`
- `cancelBuild()`
