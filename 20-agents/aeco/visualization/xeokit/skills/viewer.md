---
name: xeokit-sdk-viewer
description: Viewer declarations from xeokit-sdk
---

# Viewer

## Methods

- `on(event: string, callback: Function)`
- `fire(event: string, value: any)`
- `log(msg: string)`
- `error(msg: string)`
- `addPlugin(plugin: Plugin)`
- `beginSnapshot()`
- `getSnapshot(params: { width?: number, height?: number, format?: "jpeg" | "png" | "bmp", includeGizmos?: boolean })`
- `getSnapshotWithPlugins(params: { width?: number, height?: number, format?: "jpeg" | "png" | "bmp", includeGizmos?: boolean })`
- `endSnapshot()`
- `destroy()`
