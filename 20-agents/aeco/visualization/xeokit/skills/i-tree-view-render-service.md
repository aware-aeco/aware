---
name: xeokit-sdk-i-tree-view-render-service
description: ITreeViewRenderService declarations from xeokit-sdk
---

# ITreeViewRenderService

## Methods

- `createRootNode()`
- `createNodeElement(node: TreeViewNode, expandHandler: genericHandler, checkHandler: genericHandler, contextmenuHandler: genericHandler, titleClickHandler: genericHandler)`
- `createDisabledNodeElement(rootName: string)`
- `addChildren(element: HTMLElement, nodes: Array<HTMLElement>)`
- `expand(element: HTMLElement, expandHandler: genericHandler, collapseHandler: genericHandler)`
- `collapse(element: HTMLElement, expandHandler: genericHandler, collapseHandler: genericHandler)`
- `isExpanded(element: HTMLElement)`
- `getId(element: HTMLElement)`
- `getIdFromCheckbox(element: HTMLElement)`
- `getSwitchElement(nodeId: string)`
- `isChecked(element: HTMLElement)`
- `setCheckbox(nodeId: string, checked: boolean, indeterminate?: boolean)`
- `setXRayed(nodeId: string, xrayed: boolean)`
- `setHighlighted(nodeId: string, highlighted: boolean)`
