---
name: components-annotation-system
description: AnnotationSystem declarations from components
---

# AnnotationSystem

## Methods

- `_buildGroup(item: TSystem["item"], style: TSystem["style"])`
- `pickHandle(drawing: TechnicalDrawing, ray: THREE.Ray, threshold?: number)`
- `_onAfterPersist(_item: TSystem["item"], _group: THREE.Group)`
- `_onDispose()`
- `_updatePreview()`
- `add(drawing: TechnicalDrawing, data: TSystem["data"])`
- `update(drawing: TechnicalDrawing, uuids: string[], changes: Partial<TSystem["data"]>)`
- `pick(ray: THREE.Ray, threshold?: number)`
- `delete(drawing: TechnicalDrawing, uuids: string[])`
- `clear(drawings?: TechnicalDrawing[])`
- `dispose()`
- `_trackDrawing(drawing: TechnicalDrawing)`
- `_resolveStyle(styleName: string)`
- `_getMaterial(styleName: string)`
- `_getMeshMaterial(styleName: string)`
- `_disposeGroup(group: THREE.Group)`
- `_clearPreview()`
- `_persist(drawing: TechnicalDrawing, item: TSystem["item"])`
- `_redraw(item: TSystem["item"], group: THREE.Group)`
