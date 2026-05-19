---
name: allplan-buildingelementinput
description: This skill encodes the allplan 2024.0 surface of the BuildingElementInput namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BuildingElementInput.
---

# BuildingElementInput

Auto-generated from vendor docs for allplan 2024.0. 1 types in this namespace.

## BuildingElementInput (class)

Definition of class BuildingElementInput

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementInput/BuildingElementInput/)

### Constructors
- `BuildingElementInput(coord_input, path)` — Initialization of class BuildingElementInput

### Methods
#### `calculate_update_matrix(geo_matrix, local_placement_matrix)`

Calculate matrix for update

**Remarks:** Calculate matrix for update

**Parameters:**
- `geo_matrix` (Matrix3D) — geometry matrix
- `local_placement_matrix` (Matrix3D) — local placement matrix

**Returns:** `Matrix3D` — original insert matrix of the PythonPart

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementInput/BuildingElementInput/#BuildingElementInput.BuildingElementInput.calculate_update_matrix)

#### `close_palette()`

Close the palette

**Remarks:** Close the palette

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementInput/BuildingElementInput/#BuildingElementInput.BuildingElementInput.close_palette)

#### `execute_pre_element_delete()`

execute the pre element delete

**Remarks:** execute the pre element delete

**Returns:** `bool` — pre element delete handled state

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementInput/BuildingElementInput/#BuildingElementInput.BuildingElementInput.execute_pre_element_delete)

#### `get_point_from_insert_matrix()`

Get input point from the insert matrix

**Remarks:** Get input point from the insert matrix

**Returns:** `Point3D` — point from the insert matrix

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementInput/BuildingElementInput/#BuildingElementInput.BuildingElementInput.get_point_from_insert_matrix)

#### `initialize_geometry_expand(msg_info)`

initialize the geometry expand

**Remarks:** initialize the geometry expand

**Parameters:**
- `msg_info` (AddMsgInfo) — message info

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementInput/BuildingElementInput/#BuildingElementInput.BuildingElementInput.initialize_geometry_expand)

#### `is_visualeditor_running(_power_management, cancel_by_menu_function)`

check for running visual editor

**Remarks:** check for running visual editor

**Parameters:**
- `_power_management` (bool) — checking for power management
- `cancel_by_menu_function` (bool) — cancel is execute due to menu function start

**Returns:** `bool` — Visual Editor is running

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementInput/BuildingElementInput/#BuildingElementInput.BuildingElementInput.is_visualeditor_running)

#### `modify_element_property(page, name, value)`

Modify property of element

**Remarks:** Modify property of element

**Parameters:**
- `page` (int) — the page of the property
- `name` (str) — the name of the property.
- `value` (Any) — new value for property.

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementInput/BuildingElementInput/#BuildingElementInput.BuildingElementInput.modify_element_property)

#### `on_cancel_function()`

Cancel the input function

**Remarks:** Cancel the input function

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementInput/BuildingElementInput/#BuildingElementInput.BuildingElementInput.on_cancel_function)

#### `on_control_event(event_id)`

On control event

**Remarks:** On control event

**Parameters:**
- `event_id` (int) — event id of control.

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementInput/BuildingElementInput/#BuildingElementInput.BuildingElementInput.on_control_event)

#### `on_mouse_leave()`

Handles the mouse leave event

**Remarks:** Handles the mouse leave event

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementInput/BuildingElementInput/#BuildingElementInput.BuildingElementInput.on_mouse_leave)

#### `on_preview_draw()`

Handles the preview draw event

**Remarks:** Handles the preview draw event

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementInput/BuildingElementInput/#BuildingElementInput.BuildingElementInput.on_preview_draw)

#### `on_value_input_control_enter()`

Process the enter inside the value input control

**Remarks:** Process the enter inside the value input control

**Returns:** `bool` — message was processed: True/False

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementInput/BuildingElementInput/#BuildingElementInput.BuildingElementInput.on_value_input_control_enter)

#### `process_mouse_msg(mouse_msg, pnt, msg_info)`

Handles the process mouse message event

**Remarks:** Handles the process mouse message event

**Parameters:**
- `mouse_msg` (int) — the mouse message.
- `pnt` (object) — the input point.
- `msg_info` (object) — additional message info.

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementInput/BuildingElementInput/#BuildingElementInput.BuildingElementInput.process_mouse_msg)

#### `save_load_favorite(is_save, file_name)`

Save or load a favorite

**Remarks:** Save or load a favorite

**Parameters:**
- `is_save` (bool) — True = save, False = load
- `file_name` (str) — Name of the favorite file

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementInput/BuildingElementInput/#BuildingElementInput.BuildingElementInput.save_load_favorite)

#### `start_input(file_name, parameter_data, msg_info, is_modification_mode, modify_uuid_list, geo_matrix, local_placement_matrix, asso_ref_ele, only_update, is_marked_for_delete=False)`

Start the input function

**Remarks:** Start the input function

**Parameters:**
- `file_name` (str) — file name of the pyp file
- `parameter_data` (list[str]) — parameter data of the selected PythonPart
- `msg_info` (AddMsgInfo) — additional mouse message info
- `is_modification_mode` (bool) — is started in modification mode
- `modify_uuid_list` (ModificationElementList) — list with the UUIDs of the modified elements
- `geo_matrix` (Matrix3D) — placement matrix
- `local_placement_matrix` (Matrix3D) — local placement matrix
- `asso_ref_ele` (BaseElementAdapter) — reference element of the associative view
- `only_update` (bool) — only update the PythonPart, no user interaction
- `is_marked_for_delete` (bool) — is marked for delete state

**Returns:** `tuple[bool, Any]` — (successfully started, started script object)

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementInput/BuildingElementInput/#BuildingElementInput.BuildingElementInput.start_input)

#### `start_interactor()`

start the interactor

**Remarks:** start the interactor

**Returns:** `tuple[bool, object]` — created interactor state, interactor object

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementInput/BuildingElementInput/#BuildingElementInput.BuildingElementInput.start_interactor)

#### `start_next_input()`

start the next input if multi placement is active

**Remarks:** start the next input if multi placement is active

**Returns:** `bool` — next input is started: True/False

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementInput/BuildingElementInput/#BuildingElementInput.BuildingElementInput.start_next_input)

