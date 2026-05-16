---
name: xeokit-sdk-annotations-plugin
description: AnnotationsPlugin declarations from xeokit-sdk
---

# AnnotationsPlugin

## Methods

- `createAnnotation()`
- `destroyAnnotation(id: string)`
- `clear()`
- `on(event: "annotationCreated", callback: (annotationId: string)=> void)`
- `on(event: "annotationDestroyed", callback: (annotationId: string)=> void)`
- `on(event: "markerMouseEnter", callback: (annotation: Annotation)=> void)`
- `on(event: "markerMouseLeave", callback: (annotation: Annotation)=> void)`
- `on(event: "markerClicked", callback: (annotation: Annotation)=> void)`
