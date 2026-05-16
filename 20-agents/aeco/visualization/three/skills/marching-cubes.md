---
name: three-marching-cubes
description: MarchingCubes declarations from three
---

# MarchingCubes

## Methods

- `begin()`
- `end()`
- `init(resolution: number)`
- `addBall(ballx: number, bally: number, ballz: number, strength: number, subtract: number, colors?: Color)`
- `addPlaneX(strength: number, subtract: number)`
- `addPlaneY(strength: number, subtract: number)`
- `addPlaneZ(strength: number, subtract: number)`
- `setCell(x: number, y: number, z: number, value: number)`
- `getCell(x: number, y: number, z: number)`
- `blur(intensity: number)`
- `reset()`
- `update()`
- `render(renderCallback: any)`
- `generateGeometry()`
- `generateBufferGeometry()`
