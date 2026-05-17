---
name: components-technical-drawing
description: TechnicalDrawing declarations from components
---

# TechnicalDrawing

## Methods

- `raycast(ray: THREE.Ray, viewport?: DrawingViewport | null)`
- `alignTo(drawingPoints: THREE.Vector3[], worldPoints: THREE.Vector3[])`
- `toDrawingSpace(ls: THREE.LineSegments, drawing: TechnicalDrawing)`
- `addProjectionLines(ls: THREE.LineSegments, layer?: string)`
- `addProjectionFromItems()`
- `orientTo(direction: THREE.Vector3)`
- `dispose()`
