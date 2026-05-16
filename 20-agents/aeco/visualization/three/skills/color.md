---
name: three-color
description: Color declarations from three
---

# Color

## Methods

- `setFromVector3(vector: Vector3)`
- `setScalar(scalar: number)`
- `setHex(hex: number, colorSpace?: string)`
- `setRGB(r: number, g: number, b: number, colorSpace?: string)`
- `setHSL(h: number, s: number, l: number, colorSpace?: string)`
- `setStyle(style: string, colorSpace?: string)`
- `setColorName(style: string, colorSpace?: string)`
- `clone()`
- `copy(color: Color)`
- `copySRGBToLinear(color: Color)`
- `copyLinearToSRGB(color: Color)`
- `convertSRGBToLinear()`
- `convertLinearToSRGB()`
- `getHex(colorSpace?: string)`
- `getHexString(colorSpace?: string)`
- `getHSL(target: HSL, colorSpace?: string)`
- `getRGB(target: RGB, colorSpace?: string)`
- `getStyle(colorSpace?: string)`
- `offsetHSL(h: number, s: number, l: number)`
- `add(color: Color)`
- `addColors(color1: Color, color2: Color)`
- `addScalar(s: number)`
- `applyMatrix3(m: Matrix3)`
- `sub(color: Color)`
- `multiply(color: Color)`
- `multiplyScalar(s: number)`
- `lerp(color: Color, alpha: number)`
- `lerpColors(color1: Color, color2: Color, alpha: number)`
- `lerpHSL(color: Color, alpha: number)`
- `equals(color: Color)`
- `fromArray(array: number[] | ArrayLike<number>, offset?: number)`
- `toArray(array?: number[], offset?: number)`
- `toArray(xyz: ArrayLike<number>, offset?: number)`
- `toJSON()`
- `fromBufferAttribute(attribute: BufferAttribute | InterleavedBufferAttribute, index: number)`
