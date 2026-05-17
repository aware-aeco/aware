---
name: viewer-world-tree
description: WorldTree declarations from viewer
---

# WorldTree

## Methods

- `getRenderTree()`
- `getRenderTree(subtreeId: string)`
- `isRoot(node: TreeNode)`
- `isSubtreeRoot(node: TreeNode)`
- `parse(model: Model<NodeData>)`
- `addSubtree(node: TreeNode)`
- `addNode(node: TreeNode, parent: TreeNode | null)`
- `removeNode(node: TreeNode, removeChildren: boolean)`
- `hasNodeId(id: string, subtreeId?: number)`
- `hasInstanceId(id: string, subtreeId?: number)`
- `hasId(id: string, subtreeId?: number)`
- `findAll(predicate: SearchPredicate, node?: TreeNode)`
- `findId(id: string, subtreeId?: number)`
- `findSubtree(id: string)`
- `getAncestors(node: TreeNode)`
- `getInstances(subtreeId: string)`
- `getDuplicates(subtreeId: string)`
- `walk(predicate: SearchPredicate, node?: TreeNode)`
- `walkAsync(predicate: SearchPredicate, node?: TreeNode)`
- `purge(subtreeId?: string)`
