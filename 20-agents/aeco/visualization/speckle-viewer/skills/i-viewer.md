---
name: viewer-i-viewer
description: IViewer declarations from viewer
---

# IViewer

## Methods

- `init()`
- `resize()`
- `on(eventType: T, handler: (arg: ViewerEventPayload[T]) => void)`
- `requestRender(flags?: UpdateFlags)`
- `setLightConfiguration(config: LightConfiguration)`
- `getViews()`
- `loadObject(loader: Loader, zoomToObject?: boolean)`
- `cancelLoad(url: string, unload?: boolean)`
- `unloadObject(url: string)`
- `unloadAll()`
- `screenshot()`
- `getObjectProperties(resourceURL?: string, bypassCache?: boolean)`
- `getWorldTree()`
- `query(query: T)`
- `getRenderer()`
- `getContainer()`
- `getCanvas()`
- `createExtension(type: Constructor<T>)`
- `getExtension(type: Constructor<T>)`
- `hasExtension(type: Constructor<T>)`
- `dispose()`
