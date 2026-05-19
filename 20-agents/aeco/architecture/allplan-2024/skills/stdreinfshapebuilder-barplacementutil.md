---
name: allplan-stdreinfshapebuilder-barplacementutil
description: This skill encodes the allplan 2024.0 surface of the StdReinfShapeBuilder.BarPlacementUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions.
---

# StdReinfShapeBuilder.BarPlacementUtil

Auto-generated from vendor docs for allplan 2024.0. 1 types in this namespace.

## Functions (static-class)

Module-level functions of StdReinfShapeBuilder.BarPlacementUtil

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/BarPlacementUtil/_functions/)

### Methods
#### `get_placement_end_from_bending_roller(shape, side_number, bending_roller, base_line, placement_diameter, local_angles)`

Calculate the placement start point from the bending roller point at the end of a shape side. The point is local to the end of the base line ----------------x----E base line <--- Return: local position of placement end from E to x

**Remarks:** Calculate the placement start point from the bending roller point at the end of a shape side. The point is local to the end of the base line ----------------x----E base line <--- Return: local position of placement end from E to x

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/BarPlacementUtil/_functions/#StdReinfShapeBuilder.BarPlacementUtil.get_placement_end_from_bending_roller)

#### `get_placement_inside_bending_roller(shape, corner_number, bending_roller, placement_diameter, local_angles, global_point=False)`

Calculate the position of a placement inside the bending roller | | X -------- Return: local position of placement start as Point3D

**Remarks:** Calculate the position of a placement inside the bending roller | | X -------- Return: local position of placement start as Point3D

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/BarPlacementUtil/_functions/#StdReinfShapeBuilder.BarPlacementUtil.get_placement_inside_bending_roller)

#### `get_placement_inside_side_intersection(shape1, side_number1, above_side1, shape2, side_number2, above_side2, placement_diameter, local_angles, global_point=False)`

Calculate the position of a placement inside the intersection of two shape sides Return: local placement point as Point2D

**Remarks:** Calculate the position of a placement inside the intersection of two shape sides Return: local placement point as Point2D

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/BarPlacementUtil/_functions/#StdReinfShapeBuilder.BarPlacementUtil.get_placement_inside_side_intersection)

#### `get_placement_start_from_bending_roller(shape, side_number, bending_roller, base_line, placement_diameter, local_angles)`

Calculate the placement start point from the bending roller point at the start of a shape side. The point is local to the start of the base line | | -------- S----x---------------- base line ---> Return: local position of placement start

**Remarks:** Calculate the placement start point from the bending roller point at the start of a shape side. The point is local to the start of the base line | | -------- S----x---------------- base line ---> Return: local position of placement start

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/BarPlacementUtil/_functions/#StdReinfShapeBuilder.BarPlacementUtil.get_placement_start_from_bending_roller)

