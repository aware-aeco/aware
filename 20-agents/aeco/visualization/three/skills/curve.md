---
name: three-curve
description: Curve declarations from three
---

# Curve

## Methods

- `getPointAt(u: number)`
- `getTangentAt(u: number)`
- `getPoint(t: number, optionalTarget?: TVector)`
- `getPointAt(u: number, optionalTarget?: TVector)`
- `getPoints(divisions?: number)`
- `getSpacedPoints(divisions?: number)`
- `getLength()`
- `getLengths(divisions?: number)`
- `updateArcLengths()`
- `getUtoTmapping(u: number, distance: number)`
- `getTangent(t: number, optionalTarget?: TVector)`
- `getTangentAt(u: number, optionalTarget?: TVector)`
- `computeFrenetFrames()`
- `clone()`
- `copy(source: Curve<TVector>)`
- `toJSON()`
- `fromJSON(json: CurveJSON)`
