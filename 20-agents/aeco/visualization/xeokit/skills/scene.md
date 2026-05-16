---
name: xeokit-sdk-scene
description: Scene declarations from xeokit-sdk
---

# Scene

## Methods

- `on(event: "modelLoaded", callback: (modelId: string) => void, scope?: any)`
- `on(event: "modelUnloaded", callback: (modelId: string) => void, scope?: any)`
- `on(event: "rendering", callback: (renderEvent: { sceneId: number; pass: number; }) => void, scope?: any)`
- `on(event: "rendered", callback: (renderEvent: { sceneId: number; pass: number; }) => void, scope?: any)`
- `on(event: "objectVisibility" | "objectXRayed" | "objectHighlighted" | "objectSelected", callback: (entity: VBOSceneModel | Mesh | Node) => void, scope?: any)`
- `on(event: "tick", callback: (tickEvent: TickEvent) => void, scope?: any)`
- `on(event: "sectionPlaneCreated", callback: (measurement: SectionPlane) => void)`
- `on(event: "sectionPlaneDestroyed", callback: (measurement: SectionPlane) => void)`
- `doOcclusionTest()`
- `render(forceRender?: boolean)`
- `pick()`
- `clear()`
- `clearLights()`
- `clearSectionPlanes()`
- `clearBitmaps()`
- `clearLines()`
- `getAABB(ids: string[])`
- `setObjectsVisible(ids: string[], visible: boolean)`
- `setObjectsCollidable(ids: string[], collidable: boolean)`
- `setObjectsCulled(ids: string[], culled: boolean)`
- `setObjectsSelected(ids: string[], selected: boolean)`
- `setObjectsHighlighted(ids: string[], highlighted: boolean)`
- `setObjectsXRayed(ids: string[], xrayed: boolean)`
- `setObjectsEdges(ids: string[], edges: boolean)`
- `setObjectsColorized(ids: string[], colorize: number[])`
- `setObjectsOpacity(ids: string[], opacity: number)`
- `setObjectsPickable(ids: string[], pickable: boolean)`
- `setObjectsOffset(ids: string[], offset: number[])`
- `withObjects(ids: string[], callback: Function)`
- `destroy()`
