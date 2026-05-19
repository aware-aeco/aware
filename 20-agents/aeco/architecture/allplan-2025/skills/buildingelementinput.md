---
name: allplan-buildingelementinput
description: This skill encodes the allplan 2025.0 surface of the BuildingElementInput namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BuildingElementInput.
---

# BuildingElementInput

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## BuildingElementInput (class)

Definition of class BuildingElementInput

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementInput/)

### Constructors
- `BuildingElementInput(coord_input: CoordinateInput, path: str)` — Initialization of class BuildingElementInput

### Methods
#### `close_palette()`

Close the palette

**Remarks:** Close the palette

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementInput/#BuildingElementInput.BuildingElementInput.close_palette)

#### `execute_pre_element_delete() -> bool`

execute the pre element delete

**Remarks:** execute the pre element delete

**Returns:** `bool` — pre element delete handled state

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementInput/#BuildingElementInput.BuildingElementInput.execute_pre_element_delete)

#### `get_point_from_insert_matrix() -> Point3D`

Get input point from the insert matrix

**Remarks:** Get input point from the insert matrix

**Returns:** `Point3D` — point from the insert matrix

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementInput/#BuildingElementInput.BuildingElementInput.get_point_from_insert_matrix)

#### `is_visualeditor_running( _power_management: bool, cancel_by_menu_function: bool ) -> bool`

check for running visual editor

**Remarks:** check for running visual editor

**Parameters:**
- `_power_management` (bool) — checking for power management
- `cancel_by_menu_function` (bool) — cancel is execute due to menu function start

**Returns:** `bool` — Visual Editor is running

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementInput/#BuildingElementInput.BuildingElementInput.is_visualeditor_running)

#### `modify_element_property(page: int, name: str, value: Any)`

Modify property of element

**Remarks:** Modify property of element

**Parameters:**
- `page` (int) — the page of the property
- `name` (str) — the name of the property.
- `value` (Any) — new value for property.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementInput/#BuildingElementInput.BuildingElementInput.modify_element_property)

#### `on_cancel_function() -> bool`

Cancel the input function

**Remarks:** Cancel the input function

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementInput/#BuildingElementInput.BuildingElementInput.on_cancel_function)

#### `on_control_event(event_id: int)`

On control event

**Remarks:** On control event

**Parameters:**
- `event_id` (int) — event id of control.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementInput/#BuildingElementInput.BuildingElementInput.on_control_event)

#### `on_input_undo() -> bool`

Process the input undo event

**Remarks:** Process the input undo event

**Returns:** `bool` — message was processed: True/False

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementInput/#BuildingElementInput.BuildingElementInput.on_input_undo)

#### `on_mouse_leave()`

Handles the mouse leave event

**Remarks:** Handles the mouse leave event

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementInput/#BuildingElementInput.BuildingElementInput.on_mouse_leave)

#### `on_preview_draw()`

Handles the preview draw event

**Remarks:** Handles the preview draw event

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementInput/#BuildingElementInput.BuildingElementInput.on_preview_draw)

#### `on_shortcut_control_input(value: int) -> bool`

Handles the input inside the shortcut control

**Remarks:** Handles the input inside the shortcut control

**Parameters:**
- `value` (int) — shortcut value

**Returns:** `bool` — message was processed: True/False

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementInput/#BuildingElementInput.BuildingElementInput.on_shortcut_control_input)

#### `on_value_input_control_enter() -> bool`

Process the enter inside the value input control

**Remarks:** Process the enter inside the value input control

**Returns:** `bool` — message was processed: True/False

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementInput/#BuildingElementInput.BuildingElementInput.on_value_input_control_enter)

#### `process_mouse_msg(mouse_msg: int, pnt: Point2D, msg_info: AddMsgInfo) -> bool`

Handles the process mouse message event

**Remarks:** Handles the process mouse message event

**Parameters:**
- `mouse_msg` (int) — the mouse message.
- `pnt` () — the input point.
- `msg_info` (object) — additional message info.

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementInput/#BuildingElementInput.BuildingElementInput.process_mouse_msg)

#### `reset_param_values()`

Reset to original parameter values from PYP file

**Remarks:** Reset to original parameter values from PYP file

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementInput/#BuildingElementInput.BuildingElementInput.reset_param_values)

#### `save_load_favorite(is_save: bool, file_name: str)`

Save or load a favorite

**Remarks:** Save or load a favorite

**Parameters:**
- `is_save` (bool) — True = save, False = load
- `file_name` (str) — Name of the favorite file

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementInput/#BuildingElementInput.BuildingElementInput.save_load_favorite)

#### `set_is_only_update_locks(is_only_update: bool)`

set the is only update locks

**Remarks:** set the is only update locks

**Parameters:**
- `is_only_update` (bool) — only update the PythonPart, no user interaction

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementInput/#BuildingElementInput.BuildingElementInput.set_is_only_update_locks)

#### `start_input( file_name: str, parameter_data: list[str], msg_info: AddMsgInfo | None, is_modification_mode: bool, modify_uuid_list: ModificationElementList, geo_matrix: Matrix3D, local_placement_matrix: Matrix3D, asso_ref_ele: BaseElementAdapter, is_only_update: bool, is_marked_for_delete: bool = False, ) -> tuple[bool, Any]`

Start the input function

**Remarks:** Start the input function

**Parameters:**
- `file_name` (str) — file name of the pyp file
- `parameter_data` (list[str]) — parameter data of the selected PythonPart
- `msg_info` (AddMsgInfo | None) — additional mouse message info
- `is_modification_mode` (bool) — is started in modification mode
- `modify_uuid_list` (ModificationElementList) — list with the UUIDs of the modified elements
- `geo_matrix` (Matrix3D) — placement matrix
- `local_placement_matrix` (Matrix3D) — local placement matrix
- `asso_ref_ele` (BaseElementAdapter) — reference element of the associative view
- `is_only_update` (bool) — only update the PythonPart, no user interaction
- `is_marked_for_delete` (bool) — is marked for delete state

**Returns:** `tuple[bool, Any]` — (successfully started, started script object)

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementInput/#BuildingElementInput.BuildingElementInput.start_input)

#### `start_next_input() -> bool`

start the next input if multi placement is active

**Remarks:** start the next input if multi placement is active

**Returns:** `bool` — next input is started: True/False

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementInput/#BuildingElementInput.BuildingElementInput.start_next_input)

#### `start_standard_pythonpart(update_palette: bool) -> bool`

start the standard PythonPart input

**Remarks:** start the standard PythonPart input

**Parameters:**
- `update_palette` (bool) — update palette state

**Returns:** `bool` — start state

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementInput/#BuildingElementInput.BuildingElementInput.start_standard_pythonpart)

