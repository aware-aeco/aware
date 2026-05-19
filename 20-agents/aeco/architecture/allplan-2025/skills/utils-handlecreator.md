---
name: allplan-utils-handlecreator
description: This skill encodes the allplan 2025.0 surface of the Utils.HandleCreator namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: HandleCreator.
---

# Utils.HandleCreator

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## HandleCreator (class)

implementation of the handle creator

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/Utils/HandleCreator/)

### Methods
#### `move( handle_list: list[HandleProperties], name: str, placement_point: Point3D, info_text: str = "", )`

create a handle for a move

**Remarks:** create a handle for a move

**Parameters:**
- `handle_list` (list[HandleProperties]) — handle list
- `name` (str) — handle parameter name
- `placement_point` (Point3D) — placement move handle
- `info_text` (str) — info text

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/HandleCreator/#Utils.HandleCreator.HandleCreator.move)

#### `move_in_direction( handle_list: list[HandleProperties], name: str, placement_point: Point3D, angle: Angle, info_text: str = "", )`

create a handle for a move in the arrow direction

**Remarks:** create a handle for a move in the arrow direction

**Parameters:**
- `handle_list` (list[HandleProperties]) — handle list
- `name` (str) — handle parameter name
- `placement_point` (Point3D) — placement move handle
- `angle` (Angle) — handle angle
- `info_text` (str) — info text

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/HandleCreator/#Utils.HandleCreator.HandleCreator.move_in_direction)

#### `move_xyz( handle_list: list[HandleProperties], name: str, handle_point: Point3D, ref_point: Point3D, has_input_field: bool = True, input_field_above: bool = True, show_handles: bool = True, )`

create a handle with a move in x/y/z direction

**Remarks:** create a handle with a move in x/y/z direction

**Parameters:**
- `handle_list` (list[HandleProperties]) — handle list
- `name` (str) — handle parameter name
- `handle_point` (Point3D) — handle point
- `ref_point` (Point3D) — reference point
- `has_input_field` (bool) — has input field state
- `input_field_above` (bool) — input field above the dimension line state
- `show_handles` (bool) — show the handles state

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/HandleCreator/#Utils.HandleCreator.HandleCreator.move_xyz)

#### `point( handle_list: list[HandleProperties], name: str, handle_point: Point3D, index: int | None = None, info_text: str = "", )`

create a handle with a point to move

**Remarks:** create a handle with a point to move

**Parameters:**
- `handle_list` (list[HandleProperties]) — handle list
- `name` (str) — handle parameter name
- `handle_point` (Point3D) — handle point
- `index` (int | None) — point index
- `info_text` (str) — info text

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/HandleCreator/#Utils.HandleCreator.HandleCreator.point)

#### `point_distance( handle_list: list[HandleProperties], name: str, handle_point: Point3D, ref_point: Point3D, has_input_field: bool = True, input_field_above: bool = True, show_handles: bool = True, center_point: Point3D | None = None, )`

create a handle with a point distance

**Remarks:** create a handle with a point distance

**Parameters:**
- `handle_list` (list[HandleProperties]) — handle list
- `name` (str) — handle parameter name
- `handle_point` (Point3D) — handle point
- `ref_point` (Point3D) — reference point
- `has_input_field` (bool) — has input field state
- `input_field_above` (bool) — input field above the dimension line state
- `show_handles` (bool) — show the handles state
- `center_point` (Point3D | None) — center point for arc

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/HandleCreator/#Utils.HandleCreator.HandleCreator.point_distance)

#### `point_list_2d( handle_list: list[HandleProperties], name: str, handle_points: list[Point2D], info_text: str = "", )`

create a handle with a point to move

**Remarks:** create a handle with a point to move

**Parameters:**
- `handle_list` (list[HandleProperties]) — handle list
- `name` (str) — handle parameter name
- `handle_points` (list[Point2D]) — handle points
- `info_text` (str) — info text

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/HandleCreator/#Utils.HandleCreator.HandleCreator.point_list_2d)

