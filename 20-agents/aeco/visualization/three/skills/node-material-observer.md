---
name: three-node-material-observer
description: NodeMaterialObserver declarations from three
---

# NodeMaterialObserver

## Methods

- `firstInitialization(renderObject: RenderObject)`
- `needsVelocity(renderer: Renderer)`
- `getRenderObjectData(renderObject: RenderObject)`
- `getAttributesData(attributes: Record<string, BufferAttribute>)`
- `containsNode(builder: NodeBuilder)`
- `getGeometryData(geometry: BufferGeometry)`
- `getMaterialData(material: Material)`
- `equals(renderObject: RenderObject, lightsData: Light[], renderId: number)`
- `getLightsData(materialLights: Light[])`
- `getLights(lightsNode: LightsNode, renderId: number)`
- `needsRefresh(renderObject: RenderObject, nodeFrame: NodeFrame)`
