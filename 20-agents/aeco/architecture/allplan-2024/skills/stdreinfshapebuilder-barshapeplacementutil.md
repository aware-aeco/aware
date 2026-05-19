---
name: allplan-stdreinfshapebuilder-barshapeplacementutil
description: This skill encodes the allplan 2024.0 surface of the StdReinfShapeBuilder.BarShapePlacementUtil namespace — 2 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BarShapePlacementUtil, Functions.
---

# StdReinfShapeBuilder.BarShapePlacementUtil

Auto-generated from vendor docs for allplan 2024.0. 2 types in this namespace.

## BarShapePlacementUtil (class)

Implementation of the bar shape placement utilities

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/BarShapePlacementUtil/)

### Methods
#### `add_shape(shape_id, shape)`

Add a shape to the shape list

**Remarks:** Add a shape to the shape list

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.add_shape)

#### `get_placement(reinf_def, param_dict, diameter, local_angles)`

Calculate the local placement line inside the local X/Y coordinate system of the shapes

**Remarks:** Calculate the local placement line inside the local X/Y coordinate system of the shapes

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.get_placement)

#### `get_placement_at_shape_side(shape_id, side_number, ref_pnt_fac, b_above_side, placement_diameter, local_angles)`

Calculate the local placement line at a shape side inside the local X/Y coordinate system of the shapes

**Remarks:** Calculate the local placement line at a shape side inside the local X/Y coordinate system of the shapes

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.get_placement_at_shape_side)

#### `get_placement_at_shape_side_intersection(shape_id1, side_number1, shape_id2, side_number2, shape_id3, side_number3, b_above_side3, placement_diameter, local_angles)`

Calculate exactly the ref_pnt_fac between line intersaction shape 1-3 and shape 2-3 Return: ref_pnt_fac

**Remarks:** Calculate exactly the ref_pnt_fac between line intersaction shape 1-3 and shape 2-3 Return: ref_pnt_fac

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.get_placement_at_shape_side_intersection)

#### `get_placement_from_bending_roller(shape_id, side_number, b_roller_start_point, placement_base_line, b_placment_start_point, placement_diameter, local_angles)`

Get the placement cover from the bending roller of a defined side number Return: Local placement cover to a placement base line

**Remarks:** Get the placement cover from the bending roller of a defined side number Return: Local placement cover to a placement base line

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.get_placement_from_bending_roller)

#### `get_placement_from_side_intersection(shape_id1, side_number1, b_above_side1, shape_id2, side_number2, b_above_side2, placement_base_line, b_placment_start_point, placement_diameter, local_angles)`

Get the placement cover from the side intersection of two defined side numbers. Return: Local placement cover to a placement base line

**Remarks:** Get the placement cover from the side intersection of two defined side numbers. Return: Local placement cover to a placement base line

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.get_placement_from_side_intersection)

#### `get_placement_in_corner(shape_id, corner_number, placement_diameter, local_angles)`

Calculate the local placement point inside the local X/Y coordinate system of the shape corner Return: local position of placement start

**Remarks:** Calculate the local placement point inside the local X/Y coordinate system of the shape corner Return: local position of placement start

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.get_placement_in_corner)

#### `get_placement_in_side_corners(shape_id, side_number, placement_diameter, local_angles)`

Calculate the local placement line from the left to the right side corner inside the local X/Y coordinate system of the shapes

**Remarks:** Calculate the local placement line from the left to the right side corner inside the local X/Y coordinate system of the shapes

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.get_placement_in_side_corners)

#### `get_placement_in_side_intersection(shape_id1, side_number1, b_above_side1, shape_id2, side_number2, b_above_side2, placement_diameter, local_angles)`

Get the placement point from the side intersection of two defined side numbers. Return: Local placement cover to a placement base line

**Remarks:** Get the placement point from the side intersection of two defined side numbers. Return: Local placement cover to a placement base line

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.get_placement_in_side_intersection)

#### `get_placent_line_cover_from_side(shape_id, side_number, b_above_side)`

Get the placement line cover from a shape side by side number Return: 3D placement line and the cover

**Remarks:** Get the placement line cover from a shape side by side number Return: 3D placement line and the cover

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.get_placent_line_cover_from_side)

#### `get_side_length(shape_id, side_number)`

Check for an existing shape by an shape_id Return: Shape exist: True/False

**Remarks:** Check for an existing shape by an shape_id Return: Shape exist: True/False

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.get_side_length)

#### `is_shape_created(shape_id)`

Check for an existing shape by an shape_id Return: Shape exist: True/False Parameter: shape_id ID of the shape

**Remarks:** Check for an existing shape by an shape_id Return: Shape exist: True/False Parameter: shape_id ID of the shape

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/BarShapePlacementUtil/#StdReinfShapeBuilder.BarShapePlacementUtil.BarShapePlacementUtil.is_shape_created)

## Functions (static-class)

Module-level functions of StdReinfShapeBuilder.BarShapePlacementUtil

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/_functions/)

### Methods
#### `get_placement_end_from_bending_roller(shape, side_number, bending_roller, base_line, placement_diameter, local_angles)`

Calculate the placement start point from the bending roller point at the end of a shape side. The point is local to the end of the base line ----------------x----E base line <--- Return: local position of placement end from E to x

**Remarks:** Calculate the placement start point from the bending roller point at the end of a shape side. The point is local to the end of the base line ----------------x----E base line <--- Return: local position of placement end from E to x

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/_functions/#StdReinfShapeBuilder.BarShapePlacementUtil.get_placement_end_from_bending_roller)

#### `get_placement_inside_bending_roller(shape, corner_number, bending_roller, placement_diameter, local_angles, global_point=False)`

Calculate the position of a placement inside the bending roller | | X -------- Return: local position of placement start as Point3D

**Remarks:** Calculate the position of a placement inside the bending roller | | X -------- Return: local position of placement start as Point3D

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/_functions/#StdReinfShapeBuilder.BarShapePlacementUtil.get_placement_inside_bending_roller)

#### `get_placement_inside_side_intersection(shape1, side_number1, above_side1, shape2, side_number2, above_side2, placement_diameter, local_angles, global_point=False)`

Calculate the position of a placement inside the intersection of two shape sides Return: local placement point as Point2D

**Remarks:** Calculate the position of a placement inside the intersection of two shape sides Return: local placement point as Point2D

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/_functions/#StdReinfShapeBuilder.BarShapePlacementUtil.get_placement_inside_side_intersection)

#### `get_placement_start_from_bending_roller(shape, side_number, bending_roller, base_line, placement_diameter, local_angles)`

Calculate the placement start point from the bending roller point at the start of a shape side. The point is local to the start of the base line | | -------- S----x---------------- base line ---> Return: local position of placement start

**Remarks:** Calculate the placement start point from the bending roller point at the start of a shape side. The point is local to the start of the base line | | -------- S----x---------------- base line ---> Return: local position of placement start

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/BarShapePlacementUtil/_functions/#StdReinfShapeBuilder.BarShapePlacementUtil.get_placement_start_from_bending_roller)

