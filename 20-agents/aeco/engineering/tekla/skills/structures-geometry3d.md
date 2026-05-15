---
name: tekla-structures-geometry3d
description: "Tekla Open API - Geometry3d namespace. Points, vectors, lines, planes, arcs, coordinate systems, matrix transformations, bounding boxes, intersection/distance/projection/parallelism utilities, polycurves, and faceted BREPs. This skill should be used when working with 3D geometry, spatial calculations, coordinate transforms, or geometric queries in Tekla."
---

# Tekla.Structures.Geometry3d API Reference (v2025)

## Overview

The Geometry3d namespace provides all 3D math primitives and spatial query utilities for the Tekla Open API. These types are used everywhere -- model object positions, coordinate system transformations, solid geometry analysis, drawing view geometry, and more.

**Assemblies:**
- `Tekla.Structures.dll` -- core Geometry3d types
- `Tekla.Structures.Geometry3d.Compatibility.dll` -- conversion helpers between legacy and modern geometry types

---

## Critical Patterns

- **Point.X/Y/Z are fields, not properties** -- access directly: `point.X`, not `point.GetX()`
- **Vector inherits from Point** -- Vector has X, Y, Z fields from Point
- **Normalize() modifies in-place** -- `vector.Normalize()` mutates the vector and returns original length. Use `vector.GetNormal()` for a non-mutating copy.
- **Angles are in radians** -- `Arc.Angle`, `GetAngleBetween()`, `MatrixFactory.Rotate()` all use radians
- **PolyLine uses ArrayList** -- legacy API; cast elements to `Point`
- **MatrixFactory direction matters** -- `ToCoordinateSystem` = global-to-local, `FromCoordinateSystem` = local-to-global
- **Intersection returns null** -- if no intersection exists, methods return `null` (not an exception)
- **OBB extents are half-widths** -- `Extent0` is half the box width along `Axis0`

---

## Inheritance Hierarchy

```
Point
 └── Vector

ICurve
 ├── LineSegment
 ├── Arc
 └── Polycurve

IBoundingVolume
 ├── AABB
 └── OBB

FacetedBrep
 └── FacetedBrepWithNormals
```

> Detailed API reference (all types, methods, properties, constructors) is available in the supporting file `tekla-structures-geometry3d-api-reference.md`.

