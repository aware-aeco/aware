---
name: three-spherical-harmonics3
description: SphericalHarmonics3 declarations from three
---

# SphericalHarmonics3

## Methods

- `zero()`
- `add(sh: SphericalHarmonics3)`
- `addScaledSH(sh: SphericalHarmonics3, s: number)`
- `scale(s: number)`
- `lerp(sh: SphericalHarmonics3, alpha: number)`
- `equals(sh: SphericalHarmonics3)`
- `copy(sh: SphericalHarmonics3)`
- `clone()`
- `fromArray(array: number[] | ArrayLike<number>, offset?: number)`
- `toArray(array?: number[], offset?: number)`
- `toArray(array: ArrayLike<number>, offset?: number)`
- `getAt(normal: Vector3, target: Vector3)`
- `getIrradianceAt(normal: Vector3, target: Vector3)`
- `getBasisAt(normal: Vector3, shBasis: number[])`
