---
name: viewer-viewer
description: Viewer declarations from viewer
---

# Viewer

## Methods

- `createExtension(type: Constructor<T>)`
- `getExtension(type: Constructor<T>)`
- `hasExtension(type: Constructor<T>)`
- `getContainer()`
- `getCanvas()`
- `getRenderer()`
- `resize()`
- `requestRender(flags?: UpdateFlags)`
- `frame()`
- `update()`
- `render()`
- `init()`
- `on(eventType: T, listener: (arg: ViewerEventPayload[T]) => void)`
- `getObjectProperties(resourceURL?: string | null, bypassCache?: boolean)`
- `getDataTree()`
- `getWorldTree()`
- `query(query: T)`
- `setLightConfiguration(config: SunLightConfiguration)`
- `getViews()`
- `screenshot()`
- `loadObject(loader: Loader, zoomToObject?: boolean)`
- `cancelLoad(resource: string, unload?: boolean)`
- `unloadObject(resource: string)`
- `unloadAll()`
- `dispose()`
