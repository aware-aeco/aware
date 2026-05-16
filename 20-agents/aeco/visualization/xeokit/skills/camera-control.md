---
name: xeokit-sdk-camera-control
description: CameraControl declarations from xeokit-sdk
---

# CameraControl

## Methods

- `on(event: "rightClick", callback: (e: {event: MouseEvent; pagePos: number[]; canvasPos: number[]; }) => void, scope?: any)`
- `on(event: "hover", callback: (e: PickResult) => void, scope?: any)`
- `on(event: "hoverOff", callback: (e: {canvasPos: number[]; }) => void, scope?: any)`
- `on(event: "hoverEnter", callback: (e: PickResult) => void, scope?: any)`
- `on(event: "hoverOut", callback: (e: {entity: Entity; }) => void, scope?: any)`
- `on(event: "picked", callback: (e: PickResult) => void, scope?: any)`
- `on(event: "pickedSurface", callback: (e: PickResult) => void, scope?: any)`
- `on(event: "pickedNothing", callback: (e?: {canvasPos: number[]; }) => void, scope?: any)`
- `on(event: "doublePicked", callback: (e: PickResult) => void, scope?: any)`
- `on(event: "doublePickedSurface", callback: (e: PickResult) => void, scope?: any)`
- `on(event: "doublePickedNothing", callback: (e?: {canvasPos: number[]; }) => void, scope?: any)`
- `enablePivotSphere(cfg: {size: number, material: Material})`
- `disablePivotSphere()`
- `setCursorStyle(action: string, style: string)`
- `getCursorStyle(action: string)`
