---
name: allplan-stdreinfshapebuilder-corbelreinfshapebuilder
description: This skill encodes the allplan 2025.0 surface of the StdReinfShapeBuilder.CorbelReinfShapeBuilder namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions.
---

# StdReinfShapeBuilder.CorbelReinfShapeBuilder

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## Functions (static-class)

Module-level functions of StdReinfShapeBuilder.CorbelReinfShapeBuilder

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/CorbelReinfShapeBuilder/)

### Methods
#### `column_corbel_shape_type1( column_width: float, column_thickness: float, corbel_width: float, corbel_top: float, model_angles: RotationAngles, shape_props: ReinforcementShapeProperties, concrete_cover: float, ) -> BendingShape`

Create a 3-dimensional rebar shape for the column corbel. The created shape has 5 legs in total. The corbel is assumed to be at the right side of the columns rectangular cross-section. The shape's legs are created in following directions: Z+, X+, Y+, X-, Z-. This means, that from above the generated shape will look like Ↄ In addition, following assumptions are made:

**Remarks:** Create a 3-dimensional rebar shape for the column corbel. The created shape has 5 legs in total. The corbel is assumed to be at the right side of the columns rectangular cross-section. The shape's legs are created in following directions: Z+, X+, Y+, X-, Z-. This means, that from above the generated shape will look like Ↄ In addition, following assumptions are made:

**Parameters:**
- `column_width` (float) — Width (X-dimension) of the column's cross-section
- `column_thickness` (float) — Thickness (Y-dimension) of the column's cross-section
- `corbel_width` (float) — Corbel width (X-dimension)
- `corbel_top` (float) — Elevation (Z-coordinate) of the corbel's top surface
- `model_angles` (RotationAngles) — Angles to rotate the shape from local to global coordinate system.
- `shape_props` (ReinforcementShapeProperties) — Shape properties
- `concrete_cover` (float) — Concrete cover at the shape sides

**Returns:** `BendingShape` — Bar shape in world coordinates

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/CorbelReinfShapeBuilder/#StdReinfShapeBuilder.CorbelReinfShapeBuilder.column_corbel_shape_type1)

