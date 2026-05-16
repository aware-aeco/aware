---
name: xeokit-sdk-tree-view-plugin
description: TreeViewPlugin declarations from xeokit-sdk
---

# TreeViewPlugin

## Methods

- `addModel()`
- `removeModel(modelId: string)`
- `showNode(objectId: string)`
- `unShowNode()`
- `expandToDepth(depth: number)`
- `collapse()`
- `withNodeTree(node: TreeViewNode, callback: (node: TreeViewNode) => void)`
- `destroy()`
- `on(event: "contextmenu", callback: (data: { event: MouseEvent, viewer: Viewer, treeViewPlugin: TreeViewPlugin, treeViewNode: TreeViewNode }) => void)`
- `on(event: "nodeTitleClicked", callback: (data: { event: MouseEvent, viewer: Viewer, treeViewPlugin: TreeViewPlugin, treeViewNode: TreeViewNode }) => void)`
