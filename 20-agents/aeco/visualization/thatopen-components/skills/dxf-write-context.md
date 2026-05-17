---
name: components-dxf-write-context
description: DxfWriteContext declarations from components
---

# DxfWriteContext

## Methods

- `writeLine(x1: number, y1: number, x2: number, y2: number, layer?: string, aciColor?: number)`
- `writePairs(positions: ArrayLike<number>, layer?: string, aciColor?: number)`
- `writeText(text: string, x: number, y: number, height: number, options?: DxfTextOptions)`
- `writeMeshTriangles(triangles: number[], layer?: string, aciColor?: number)`
- `hexToAci(hex: number)`
- `textAngle(dx: number, dz: number)`
