---
name: rhino-rhino-geometry-meshrefinements
description: This skill encodes the rhino 8.0 surface of the Rhino.Geometry.MeshRefinements namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: RefinementSettings, CreaseEdges, LoopFormula.
---

# Rhino.Geometry.MeshRefinements

Auto-generated from vendor docs for rhino 8.0. 3 types in this namespace.

## CreaseEdges (enum)

Defines the way naked edges are handled.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_MeshRefinements_CreaseEdges.htm)

### Values
- `NakedFixed` = `0` — Naked edges will not move or be modified.
- `NakedSmooth` = `1` — The naked edge will tend toward a spline.
- `CornerFixedOtherCreased` = `2` — Corners (2-sided vertices) will be fixed, while other naked vertices will tend toward a spline.
- `Auto` = `3` — Unwelded vertices become creases, and welded are smooth.

## LoopFormula (enum)

Enumerates the alternative Loop implementations.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_MeshRefinements_LoopFormula.htm)

### Values
- `Loop` = `0` — The original Loop formula.
- `Warren` = `1` — Warren's improved formula.
- `WarrenWeimer` = `2` — The formula as explained by Weimer, improving over Warren's.

## RefinementSettings (class)

Defines the way a mesh refinement modifier works.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Geometry_MeshRefinements_RefinementSettings.htm)

### Constructors
- `public RefinementSettings()` — Creates a default operation settings object.

### Properties
- `ContinueRequest` (CancellationToken, get/set) — A token to request computation termination.
- `HasPull` (Boolean, get) — Gets a value indicating whether this subdivision should create a result on a specific surface.
- `Level` (Int32, get/set) — The level of subdivision to achieve.
- `NakedEdgeMode` (CreaseEdges, get/set) — Set this property to define how naked edges should be treated.

