---
name: viewer-selection-extension
description: SelectionExtension declarations from viewer
---

# SelectionExtension

## Methods

- `getSelectedObjects()`
- `getSelectedNodes()`
- `selectObjects(ids: Array<string>, multiSelect?: boolean)`
- `unselectObjects(ids: Array<string>)`
- `clearSelection(nodes?: Array<TreeNode>)`
- `onObjectClicked(selection: SelectionEvent | null)`
- `onObjectDoubleClick(selectionInfo: SelectionEvent | null)`
- `onPointerMove()`
- `applySelection()`
- `removeSelection(rvs?: Array<NodeRenderView>)`
- `applyHover(renderView: NodeRenderView | null)`
- `removeHover()`
