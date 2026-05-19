---
name: allplan-handlemodificationservice
description: This skill encodes the allplan 2025.0 surface of the HandleModificationService namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: HandleModificationService.
---

# HandleModificationService

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## HandleModificationService (class)

Implementation of the handle modification service

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/HandleModificationService/)

### Constructors
- `HandleModificationService( coord_input: CoordinateInput, build_ele_list: list[BuildingElement], control_props_list: list[BuildingElementControlProperties], asso_ref_ele: BaseElementAdapter | None = None, use_system_angle: bool = True, )` — initialize

### Methods
#### `get_local_handle_point(input_pnt: Point3D) -> Point3D | None`

get the local handle point

**Remarks:** get the local handle point

**Parameters:**
- `input_pnt` (Point3D) — new handle point

**Returns:** `Point3D | None` — handle point in the local element coordinate system, None if not exist

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/HandleModificationService/#HandleModificationService.HandleModificationService.get_local_handle_point)

#### `is_handle_draw_lock() -> bool`

get the handle draw lock state

**Remarks:** get the handle draw lock state

**Returns:** `bool` — handle draw lock state

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/HandleModificationService/#HandleModificationService.HandleModificationService.is_handle_draw_lock)

#### `new_handle_point_input(mouse_msg: int, pnt: Point2D, msg_info: Any) -> Point3D`

Input the new handle point

**Remarks:** Input the new handle point

**Parameters:**
- `mouse_msg` (int) — mouse message ID
- `pnt` (Point2D) — input point in Allplan view coordinates
- `msg_info` (Any) — additional mouse message info

**Returns:** `Point3D` — new handle point

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/HandleModificationService/#HandleModificationService.HandleModificationService.new_handle_point_input)

#### `process_mouse_msg(mouse_msg: int, pnt: Point2D, msg_info: Any) -> bool`

Handles the process mouse message event

**Remarks:** Handles the process mouse message event

**Parameters:**
- `mouse_msg` (int) — mouse message ID
- `pnt` (Point2D) — input point in Allplan view coordinates
- `msg_info` (Any) — additional mouse message info

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/HandleModificationService/#HandleModificationService.HandleModificationService.process_mouse_msg)

#### `reset_value()`

reset the handle value

**Remarks:** reset the handle value

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/HandleModificationService/#HandleModificationService.HandleModificationService.reset_value)

#### `set_handle_draw_lock(handle_draw_lock: bool)`

set the handle draw lock state

**Remarks:** set the handle draw lock state

**Parameters:**
- `handle_draw_lock` (bool) — handle draw lock state

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/HandleModificationService/#HandleModificationService.HandleModificationService.set_handle_draw_lock)

#### `start( handle_list: list[HandleProperties], placement_mat: Matrix3D, input_doc: DocumentAdapter, view_world_proj: ViewWorldProjection, create_view_input_controls: bool, )`

start the handle modification

**Remarks:** start the handle modification

**Parameters:**
- `handle_list` (list[HandleProperties]) — list with the handles
- `placement_mat` (Matrix3D) — placement matrix of the PythonPart
- `input_doc` (DocumentAdapter) — input document
- `view_world_proj` (ViewWorldProjection) — view world projection
- `create_view_input_controls` (bool) — create the input controls in the view

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/HandleModificationService/#HandleModificationService.HandleModificationService.start)

#### `start_new_handle_point_input( str_table_service: StringTableService, handle_placement_geo: list[Any] | None = None, )`

start the new handle point input

**Remarks:** start the new handle point input

**Parameters:**
- `str_table_service` (StringTableService) — string table service
- `handle_placement_geo` (list[Any] | None) — additional geometry element for the point input (hidden elements, preview elements)

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/HandleModificationService/#HandleModificationService.HandleModificationService.start_new_handle_point_input)

#### `stop()`

stop the handle modification

**Remarks:** stop the handle modification

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/HandleModificationService/#HandleModificationService.HandleModificationService.stop)

### Properties
- `handle_prop` (HandleProperties | None, get) — get the handle property

