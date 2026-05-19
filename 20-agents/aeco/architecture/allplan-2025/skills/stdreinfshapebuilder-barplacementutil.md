---
name: allplan-stdreinfshapebuilder-barplacementutil
description: This skill encodes the allplan 2025.0 surface of the StdReinfShapeBuilder.BarPlacementUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions.
---

# StdReinfShapeBuilder.BarPlacementUtil

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## Functions (static-class)

Module-level functions of StdReinfShapeBuilder.BarPlacementUtil

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/BarPlacementUtil/)

### Methods
#### `get_placement_end_from_bending_roller( shape: BendingShape, side_number: int, bending_roller: float, base_line: Line2D | Line3D, placement_diameter: float, local_angles: RotationAngles, ) -> float`

For the placement of longitudinal bars, located on one of the legs of a transverse bar (e.g. a stirrup), this function calculates the X-component between the edge of the concrete cross-section and the end point of the placement of the longitudinal bars

**Remarks:** For the placement of longitudinal bars, located on one of the legs of a transverse bar (e.g. a stirrup), this function calculates the X-component between the edge of the concrete cross-section and the end point of the placement of the longitudinal bars

**Parameters:**
- `shape` (BendingShape) — Bending shape of the transverse bar
- `side_number` (int) — Number of the leg, on which the longitudinal bars are located (starting from 1)
- `bending_roller` (float) — Bending roller if the transverse bar
- `base_line` (Line2D | Line3D) — The outline of the concrete cross section
- `placement_diameter` (float) — Bar diameter of the longitudinal bars
- `local_angles` (RotationAngles) — Model angles to get the local x/y coordinate system for the calculation

**Returns:** `float` — Calculated distance to the placement end

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/BarPlacementUtil/#StdReinfShapeBuilder.BarPlacementUtil.get_placement_end_from_bending_roller)

#### `get_placement_inside_bending_roller( shape: BendingShape, corner_number: int, bending_roller: float, placement_diameter: float, local_angles: RotationAngles, global_point: bool = False, ) -> Point3D`

Calculate the position of a placement inside the bending roller

**Remarks:** Calculate the position of a placement inside the bending roller

**Parameters:**
- `shape` (BendingShape) — Bending shape of the transverse bar
- `corner_number` (int) — Number of the transverse bar's corner, in which the placement point should be calculated (starting from 1)
- `bending_roller` (float) — Bending roller of the transverse bar
- `placement_diameter` (float) — Bar diameter of the longitudinal bar
- `local_angles` (RotationAngles) — Model angles to get the local x/y coordinate system for the calculation
- `global_point` (bool) — If set to True, the result will be a point in global coordinates. Otherwise, the point in local coordinate system will be returned.

**Returns:** `Point3D` — Local point of the longitudinal bar placement

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/BarPlacementUtil/#StdReinfShapeBuilder.BarPlacementUtil.get_placement_inside_bending_roller)

#### `get_placement_inside_side_intersection( shape1: BendingShape, side_number1: int, above_side1: bool, shape2: BendingShape, side_number2: int, above_side2: bool, placement_diameter: float, local_angles: RotationAngles, global_point: bool = False, ) -> Point2D | Point3D`

Calculate the position of a longitudinal bar placed at an intersection of two legs of transverse bars (e.g. two stirrups)

**Remarks:** Calculate the position of a longitudinal bar placed at an intersection of two legs of transverse bars (e.g. two stirrups)

**Parameters:**
- `shape1` (BendingShape) — Shape of the first transverse rebar
- `side_number1` (int) — Number of the leg in the first shape to consider in the calculation (starting from 1)
- `above_side1` (bool) — When set to True, the point will be calculated above the leg of the first shape. Otherwise below.
- `shape2` (BendingShape) — Shape of the second transverse rebar
- `side_number2` (int) — Number of the leg in the second shape to consider in the calculation (starting from 1)
- `above_side2` (bool) — When set to True, the point will be calculated above the leg of the second shape. Otherwise below.
- `placement_diameter` (float) — Bar diameter of the longitudinal bar
- `local_angles` (RotationAngles) — Model angles to get the local x/y coordinate system for the calculation
- `global_point` (bool) — If set to True, the result will be a 3D point in global coordinates. Otherwise, a 2D point in local coordinate system will be returned.

**Returns:** `Point2D | Point3D` — local placement point as Point2D

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/BarPlacementUtil/#StdReinfShapeBuilder.BarPlacementUtil.get_placement_inside_side_intersection)

#### `get_placement_start_from_bending_roller( shape: BendingShape, side_number: int, bending_roller: float, base_line: Line2D | Line3D, placement_diameter: float, local_angles: RotationAngles, ) -> float`

For the placement of longitudinal bars, located on one of the legs of a transverse bar (e.g. a stirrup), this function calculates the X-component between the edge of the concrete cross-section and the start point of the placement of the longitudinal bars

**Remarks:** For the placement of longitudinal bars, located on one of the legs of a transverse bar (e.g. a stirrup), this function calculates the X-component between the edge of the concrete cross-section and the start point of the placement of the longitudinal bars

**Parameters:**
- `shape` (BendingShape) — Bending shape of the transverse bar
- `side_number` (int) — Number of the leg, on which the longitudinal bars are located (starting from 1)
- `bending_roller` (float) — Bending roller if the transverse bar
- `base_line` (Line2D | Line3D) — The outline of the concrete cross section
- `placement_diameter` (float) — Bar diameter of the longitudinal bars
- `local_angles` (RotationAngles) — Model angles to get the local x/y coordinate system for the calculation

**Returns:** `float` — Calculated distance to the placement start

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/BarPlacementUtil/#StdReinfShapeBuilder.BarPlacementUtil.get_placement_start_from_bending_roller)

