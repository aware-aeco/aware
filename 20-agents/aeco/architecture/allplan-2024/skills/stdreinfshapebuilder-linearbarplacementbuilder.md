---
name: allplan-stdreinfshapebuilder-linearbarplacementbuilder
description: This skill encodes the allplan 2024.0 surface of the StdReinfShapeBuilder.LinearBarPlacementBuilder namespace — 2 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions, StartEndPlacementRule.
---

# StdReinfShapeBuilder.LinearBarPlacementBuilder

Auto-generated from vendor docs for allplan 2024.0. 2 types in this namespace.

## Functions (static-class)

Module-level functions of StdReinfShapeBuilder.LinearBarPlacementBuilder

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/LinearBarPlacementBuilder/_functions/)

### Methods
#### `calculate_length_of_regions(value_list, from_point, to_point, concrete_cover_left, concrete_cover_right)`

Create the real region length of a placement Return: Bar placement ploints as list of tuples (start point, end point)

**Remarks:** Create the real region length of a placement Return: Bar placement ploints as list of tuples (start point, end point)

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/LinearBarPlacementBuilder/_functions/#StdReinfShapeBuilder.LinearBarPlacementBuilder.calculate_length_of_regions)

#### `check_placement_length_by_distance(length, distance)`

Helper function for the length checking of a placement Return: adapted length of the placement

**Remarks:** Helper function for the length checking of a placement Return: adapted length of the placement

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/LinearBarPlacementBuilder/_functions/#StdReinfShapeBuilder.LinearBarPlacementBuilder.check_placement_length_by_distance)

#### `create_linear_bar_placement_from_by_dist_count(position, shape, from_point, direction_point, concrete_cover, bar_distance, bar_count, global_move=True)`

Create a linear placement defined by a from point, a direction point, a bar count and a distance Return: Bar placement

**Remarks:** Create a linear placement defined by a from point, a direction point, a bar count and a distance Return: Bar placement

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/LinearBarPlacementBuilder/_functions/#StdReinfShapeBuilder.LinearBarPlacementBuilder.create_linear_bar_placement_from_by_dist_count)

#### `create_linear_bar_placement_from_to_by_count(position, shape, from_point, to_point, concrete_cover_left, concrete_cover_right, bar_count, global_move=True, remove_count_left=0, remove_count_right=0)`

Create a linear placement defined by two points and a bar count Return: Bar placement

**Remarks:** Create a linear placement defined by two points and a bar count Return: Bar placement

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/LinearBarPlacementBuilder/_functions/#StdReinfShapeBuilder.LinearBarPlacementBuilder.create_linear_bar_placement_from_to_by_count)

#### `create_linear_bar_placement_from_to_by_dist(position, shape, from_point, to_point, concrete_cover_left, concrete_cover_right, bar_distance, start_end_rule=AdditionalCover, global_move=True)`

Create a linear placement defined by two points and a bar distance Return: Bar placement

**Remarks:** Create a linear placement defined by two points and a bar distance Return: Bar placement

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/LinearBarPlacementBuilder/_functions/#StdReinfShapeBuilder.LinearBarPlacementBuilder.create_linear_bar_placement_from_to_by_dist)

## StartEndPlacementRule (enum)

Class with the definition of the handle input directions

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/LinearBarPlacementBuilder/StartEndPlacementRule/)

### Values
- `AdaptDistance` = `4`
- `AdditionalCover` = `1`
- `AdditionalCoverLeft` = `2`
- `AdditionalCoverRight` = `3`

