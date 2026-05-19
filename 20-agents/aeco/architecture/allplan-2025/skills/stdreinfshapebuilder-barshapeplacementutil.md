---
name: allplan-stdreinfshapebuilder-barshapeplacementutil
description: This skill encodes the allplan 2025.0 surface of the StdReinfShapeBuilder.BarShapePlacementUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BarShapePlacementUtil.
---

# StdReinfShapeBuilder.BarShapePlacementUtil

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## BarShapePlacementUtil (class)

Implementation of the bar shape placement utility.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/)

### Constructors
- `BarShapePlacementUtil()` — Constructor

### Methods
#### `add_shape(shape_id: int | str, shape: BendingShape)`

Add a shape to the shape list

**Remarks:** Add a shape to the shape list

**Parameters:**
- `shape_id` (int | str) — ID of the reference shape
- `shape` (BendingShape) — Shape

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.add_shape)

#### `get_placement(reinf_def, param_dict, diameter, local_angles)`

Calculate the local placement line inside the local X/Y coordinate system of the shapes

**Remarks:** Calculate the local placement line inside the local X/Y coordinate system of the shapes

**Parameters:**
- `reinf_def` (object) — Reinforcement definition
- `param_dict` (object) — Parameter dictionary
- `local_angles` (object) — Rotation from global to local coordinate system

**Returns:** `object` — Placement line in local coordinate system

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.get_placement)

#### `get_placement_at_shape_side( shape_id: int | str, side_number: int, ref_pnt_fac: float, b_above_side: bool, placement_diameter: float, local_angles: RotationAngles, ) -> tuple[Line2D, float, float]`

Calculate the local placement line at a given leg of the reference shape in its local coordinate system

**Remarks:** Calculate the local placement line at a given leg of the reference shape in its local coordinate system

**Parameters:**
- `shape_id` (int | str) — ID of the reference shape
- `side_number` (int) — Number of the leg of the reference shape, beginning with 1
- `ref_pnt_fac` (float) — Factor for the reference point calculation. (-1 = at the side from left to right)
- `b_above_side` (bool) — When set to True, the placement line is created above the leg
- `placement_diameter` (float) — Diameter of the longitudinal bar
- `local_angles` (RotationAngles) — Rotation from global to local coordinate system

**Returns:** `Line2D` — Placement line in local coordinate system

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.get_placement_at_shape_side)

#### `get_placement_at_shape_side_intersection( shape_id1: int | str, side_number1: int, shape_id2: int | str, side_number2: int, shape_id3: int | str, side_number3: int, b_above_side3: bool, placement_diameter: float, local_angles: RotationAngles, ) -> tuple[Line2D, float, float]`

Calculates the placement line on the specified leg of the third reference shape, starting at the intersection with the leg of the first reference shape and ending a the intersection with leg of the second reference shape.

**Remarks:** Calculates the placement line on the specified leg of the third reference shape, starting at the intersection with the leg of the first reference shape and ending a the intersection with leg of the second reference shape.

**Parameters:**
- `shape_id1` (int | str) — ID of the first reference shape
- `side_number1` (int) — Number of the leg of the first reference shape, beginning with 1
- `shape_id2` (int | str) — ID of the second reference shape
- `side_number2` (int) — Number of the leg of the second reference shape, beginning with 1
- `shape_id3` (int | str) — ID of the third reference shape
- `side_number3` (int) — Number of the leg of the third reference shape, beginning with 1
- `b_above_side3` (bool) — When True, the placement line will be calculated above the leg, otherwise below
- `placement_diameter` (float) — Diameter of the longitudinal bar
- `local_angles` (RotationAngles) — Rotation from global to local coordinate system

**Returns:** `Line2D` — Placement line in local coordinate system

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.get_placement_at_shape_side_intersection)

#### `get_placement_from_bending_roller( shape_id: int | str, side_number: int, b_roller_start_point: bool, placement_base_line: Line2D | Line3D, b_placment_start_point: bool, placement_diameter: float, local_angles: RotationAngles, ) -> float`

Get the placement cover from the bending roller of a defined side number

**Remarks:** Get the placement cover from the bending roller of a defined side number

**Parameters:**
- `shape_id` (int | str) — ID of the reference shape
- `side_number` (int) — Number of the shape leg
- `b_roller_start_point` (bool) — True = roller at the start point / False = roller at the end point
- `placement_base_line` (Line2D | Line3D) — Base line of the placement
- `b_placment_start_point` (bool) — True = placement start point / False = placement end point
- `placement_diameter` (float) — Diameter of the longitudinal bar
- `local_angles` (RotationAngles) — Rotation from global to local coordinate system

**Returns:** `float` — Local placement cover to a placement base line

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.get_placement_from_bending_roller)

#### `get_placement_from_side_intersection( shape_id1: int | str, side_number1: int, b_above_side1: bool, shape_id2: int | str, side_number2: int, b_above_side2: bool, placement_base_line: Line2D, b_placment_start_point: bool, placement_diameter: float, local_angles: RotationAngles, ) -> float`

Get the placement cover from the side intersection of two defined side numbers.

**Remarks:** Get the placement cover from the side intersection of two defined side numbers.

**Parameters:**
- `shape_id1` (int | str) — ID of the first shape
- `side_number1` (int) — Number of the first shape leg
- `b_above_side1` (bool) — Cover above the first side: True/False
- `shape_id2` (int | str) — ID of the second shape
- `side_number2` (int) — Number of the second shape leg
- `b_above_side2` (bool) — Cover above the second side: True/False
- `placement_base_line` (Line2D) — Base line of the placement
- `b_placment_start_point` (bool) — True = placement start point / False = placement end point
- `placement_diameter` (float) — Diameter of the longitudinal bar
- `local_angles` (RotationAngles) — Rotation from global to local coordinate system

**Returns:** `float` — Local placement cover to a placement base line

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.get_placement_from_side_intersection)

#### `get_placement_in_corner( shape_id: int | str, corner_number: int, placement_diameter: float, local_angles: RotationAngles, ) -> Point3D`

Calculate the placement point for a longitudinal bar in the corner of the given reference shape

**Remarks:** Calculate the placement point for a longitudinal bar in the corner of the given reference shape

**Parameters:**
- `shape_id` (int | str) — ID of the reference shape
- `corner_number` (int) — Number of the corner in the reference shape, beginning with 1
- `placement_diameter` (float) — Diameter of the longitudinal bar
- `local_angles` (RotationAngles) — Rotation from global to local coordinate system

**Returns:** `Point3D` — Position of corner bar in the local coordinate system of the reference shape.

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.get_placement_in_corner)

#### `get_placement_in_side_corners( shape_id: int | str, side_number: int, placement_diameter: float, local_angles: RotationAngles, ) -> tuple[Line2D, float, float]`

Calculate the placement line on the entire length of the specified leg of the reference shape. Note, that the calculation cannot be done in the first leg!

**Remarks:** Calculate the placement line on the entire length of the specified leg of the reference shape. Note, that the calculation cannot be done in the first leg!

**Parameters:**
- `shape_id` (int | str) — ID of the reference shape
- `side_number` (int) — Number of the shape's leg, beginning with 1
- `placement_diameter` (float) — Diameter of the longitudinal bar
- `local_angles` (RotationAngles) — Rotation from global to local coordinate system

**Returns:** `Line2D` — Placement line in local coordinate system

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.get_placement_in_side_corners)

#### `get_placement_in_side_intersection( shape_id1: int | str, side_number1: int, b_above_side1: bool, shape_id2: int | str, side_number2: int, b_above_side2: bool, placement_diameter: float, local_angles: RotationAngles, ) -> Point2D | Point3D`

Get the placement point at the intersection of two legs.

**Remarks:** Get the placement point at the intersection of two legs.

**Parameters:**
- `shape_id1` (int | str) — ID of the first reference shape
- `side_number1` (int) — Leg number of the first reference shape, beginning with 1
- `b_above_side1` (bool) — True will calculate the placement point above the first leg, False below
- `shape_id2` (int | str) — ID of the second reference shape
- `side_number2` (int) — Leg number of the second reference shape, beginning with 1
- `b_above_side2` (bool) — True will calculate the placement point above the second leg, False below
- `placement_diameter` (float) — Diameter of the longitudinal bar
- `local_angles` (RotationAngles) — Rotation from global to local coordinate system

**Returns:** `Point2D | Point3D` — Local placement point

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.get_placement_in_side_intersection)

#### `get_placent_line_cover_from_side( shape_id: int | str, side_number: int, b_above_side: bool ) -> tuple[Line3D, float]`

Get the placement line cover from a shape leg by side number

**Remarks:** Get the placement line cover from a shape leg by side number

**Parameters:**
- `shape_id` (int | str) — ID of the reference shape
- `side_number` (int) — Number of the shape leg
- `b_above_side` (bool) — Cover above the side: True/False

**Returns:** `Line3D` — Placement line

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.get_placent_line_cover_from_side)

#### `get_side_length(shape_id: int | str, side_number: int) -> float`

Gets the leg's length of the given shape

**Remarks:** Gets the leg's length of the given shape

**Parameters:**
- `shape_id` (int | str) — ID of the reference shape
- `side_number` (int) — Number of the leg

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.get_side_length)

#### `is_shape_created(shape_id: int | str) -> bool`

Check if the shape of given id exists in the list

**Remarks:** Check if the shape of given id exists in the list

**Parameters:**
- `shape_id` (int | str) — ID of the reference shape

**Returns:** `bool` — True if shape exists, False otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.is_shape_created)

