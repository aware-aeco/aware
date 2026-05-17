---
name: viewer-materials
description: Materials declarations from viewer
---

# Materials

## Methods

- `renderMaterialFromNode(materialNode: TreeNode | null, geometryNode: TreeNode | null)`
- `displayStyleFromNode(node: TreeNode | null)`
- `colorMaterialFromNode(node: TreeNode | null)`
- `fastCopy(from: Material, to: Material)`
- `isMaterialInstance(material: Material | FilterMaterial | RenderMaterial | DisplayStyle)`
- `isFilterMaterial(material: Material | FilterMaterial | RenderMaterial | DisplayStyle)`
- `isRenderMaterial(materialData: Material | FilterMaterial | RenderMaterial | DisplayStyle | MaterialOptions)`
- `isDisplayStyle(materialData: Material | FilterMaterial | RenderMaterial | DisplayStyle | MaterialOptions)`
- `getMaterialHash(renderView: NodeRenderView, materialData?: RenderMaterial | DisplayStyle | MaterialOptions | null)`
- `isTransparent(material: Material)`
- `isOpaque(material: Material)`
- `isSpeckleMaterial(material: Material)`
- `createDefaultMaterials()`
- `getMaterial(hash: number, material: RenderMaterial | DisplayStyle | MinimalMaterial | null, type: GeometryType)`
- `getGhostMaterial(renderView: NodeRenderView, filterMaterial?: FilterMaterial)`
- `getGradientMaterial(renderView: NodeRenderView, filterMaterial?: FilterMaterial)`
- `getColoredMaterial(renderView: NodeRenderView, filterMaterial?: FilterMaterial)`
- `getHiddenMaterial(renderView: NodeRenderView, filterMaterial?: FilterMaterial)`
- `getFilterMaterial(renderView: NodeRenderView, filterMaterial: FilterMaterial)`
- `getDataMaterial(renderView: NodeRenderView, materialData: RenderMaterial & DisplayStyle & MaterialOptions)`
- `getFilterMaterialOptions(filterMaterial: FilterMaterial)`
- `purge()`
