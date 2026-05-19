---
name: allplan-handlemodificationservice
description: This skill encodes the allplan 2024.0 surface of the HandleModificationService namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: HandleModificationService.
---

# HandleModificationService

Auto-generated from vendor docs for allplan 2024.0. 1 types in this namespace.

## HandleModificationService (class)

Implementation of the handle modification service

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandleModificationService/HandleModificationService/)

### Constructors
- `HandleModificationService(coord_input, build_ele_list, control_props_list, asso_ref_ele=None)` — initialize

### Methods
#### `get_local_handle_point(input_pnt)`

get the local handle point

**Remarks:** get the local handle point

**Parameters:**
- `input_pnt` (Point3D) — new handle point

**Returns:** `Optional[Point3D]` — handle point in the local element coordinate system, None if not exist

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandleModificationService/HandleModificationService/#HandleModificationService.HandleModificationService.get_local_handle_point)

#### `new_handle_point_input(mouse_msg, pnt, msg_info)`

Input the new handle point

**Remarks:** Input the new handle point

**Parameters:**
- `mouse_msg` (int) — the mouse message.
- `pnt` (object) — the input point.
- `msg_info` (object) — additional message info.

**Returns:** `Point3D` — new handle point

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandleModificationService/HandleModificationService/#HandleModificationService.HandleModificationService.new_handle_point_input)

#### `process_mouse_msg(mouse_msg, pnt, msg_info)`

Handles the process mouse message event

**Remarks:** Handles the process mouse message event

**Parameters:**
- `mouse_msg` (int) — the mouse message.
- `pnt` (object) — the input point.
- `msg_info` (object) — additional message info.

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandleModificationService/HandleModificationService/#HandleModificationService.HandleModificationService.process_mouse_msg)

#### `reset_value()`

reset the handle value

**Remarks:** reset the handle value

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandleModificationService/HandleModificationService/#HandleModificationService.HandleModificationService.reset_value)

#### `start(handle_list, placement_mat, input_doc, view_world_proj, create_view_input_controls)`

start the handle modification

**Remarks:** start the handle modification

**Parameters:**
- `handle_list` (List[HandleProperties]) — list with the handles
- `placement_mat` (Matrix3D) — placement matrix of the PythonPart
- `create_view_input_controls` (bool) — create the input controls in the view

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandleModificationService/HandleModificationService/#HandleModificationService.HandleModificationService.start)

#### `start_new_handle_point_input(str_table_service)`

start the new handle point input

**Remarks:** start the new handle point input

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandleModificationService/HandleModificationService/#HandleModificationService.HandleModificationService.start_new_handle_point_input)

#### `stop()`

stop the handle modification

**Remarks:** stop the handle modification

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandleModificationService/HandleModificationService/#HandleModificationService.HandleModificationService.stop)

### Properties
- `handle_prop` (HandleProperties, get) — get the handle property

