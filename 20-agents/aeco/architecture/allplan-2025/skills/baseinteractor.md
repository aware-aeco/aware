---
name: allplan-baseinteractor
description: This skill encodes the allplan 2025.0 surface of the BaseInteractor namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BaseInteractor.
---

# BaseInteractor

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## BaseInteractor (interface)

base class for a PythonPart interactor

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseInteractor/)

### Constructors
- `BaseInteractor( coord_input: CoordinateInput, pyp_path: str, global_str_table_service: StringTableService, build_ele_list: list[BuildingElement], build_ele_composite: BuildingElementComposite, control_props_list: list[BuildingElementControlProperties], modify_uuid_list: list[str], )` — Constructor

### Methods
#### `execute_load_favorite(_file_name: str) -> None`

Handles the execute load favorite event. This event is triggered after pressing "Load favorite" button in the property palette and selecting the favorite file in the file dialog Implementing this method is necessary only, when the load/save/restore favorite buttons are shown in the property palette, i.e. when the tag ShowFavoriteButtons in the .pyp file is set to True

**Remarks:** Handles the execute load favorite event. This event is triggered after pressing "Load favorite" button in the property palette and selecting the favorite file in the file dialog Implementing this method is necessary only, when the load/save/restore favorite buttons are shown in the property palette, i.e. when the tag ShowFavoriteButtons in the .pyp file is set to True

**Parameters:**
- `_file_name` (str) — full path and name of the selected favorite file

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseInteractor/#BaseInteractor.BaseInteractor.execute_load_favorite)

#### `execute_save_favorite(_file_name: str) -> None`

Handles the execute save favorite event. This event is triggered after pressing "Save as a favorite" button in the property palette and selecting the location of the favorite file in the file dialog. Implementing this method is necessary only, when the load/save/restore favorite buttons are shown in the property palette, i.e. when the tag ShowFavoriteButtons in the .pyp file is set to True

**Remarks:** Handles the execute save favorite event. This event is triggered after pressing "Save as a favorite" button in the property palette and selecting the location of the favorite file in the file dialog. Implementing this method is necessary only, when the load/save/restore favorite buttons are shown in the property palette, i.e. when the tag ShowFavoriteButtons in the .pyp file is set to True

**Parameters:**
- `_file_name` (str) — full path and name of the selected favorite file

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseInteractor/#BaseInteractor.BaseInteractor.execute_save_favorite)

#### `modify_element_property(page: int, name: str, value: Any)`

Handles the modify element property event. This event is triggered with each modification of the element property done in the property palette or by using a handle.

**Remarks:** Handles the modify element property event. This event is triggered with each modification of the element property done in the property palette or by using a handle.

**Parameters:**
- `page` (int) — Page of the modified property
- `name` (str) — Name of the modified property.
- `value` (Any) — New value of the modified property.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseInteractor/#BaseInteractor.BaseInteractor.modify_element_property)

#### `on_cancel_by_menu_function() -> None`

Handles the event of terminating the PythonPart by calling another function in Allplan UI. Implement this method, when some actions must be introduced, before the PythonPart is eventually terminated.

**Remarks:** Handles the event of terminating the PythonPart by calling another function in Allplan UI. Implement this method, when some actions must be introduced, before the PythonPart is eventually terminated.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseInteractor/#BaseInteractor.BaseInteractor.on_cancel_by_menu_function)

#### `on_cancel_function() -> bool`

Handles the cancel function event This event is triggered when the ESC button is hit during the runtime of the PythonPart.

**Remarks:** Handles the cancel function event This event is triggered when the ESC button is hit during the runtime of the PythonPart.

**Returns:** `bool` — True when the PythonPart framework should terminate the PythonPart, False otherwise.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseInteractor/#BaseInteractor.BaseInteractor.on_cancel_function)

#### `on_control_event(event_id: int) -> bool`

Handles the on control event. Called when an event is triggered by a palette control (ex. button).

**Remarks:** Handles the on control event. Called when an event is triggered by a palette control (ex. button).

**Parameters:**
- `event_id` (int) — event id of the clicked button control

**Returns:** `bool` — palette update state

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseInteractor/#BaseInteractor.BaseInteractor.on_control_event)

#### `on_input_undo() -> bool`

Process the input undo event This event is triggered, when during coordinate input the user hits the undo button in the dialog line. Implementing this method is necessary only, when the undo button is shown to the user, i.e. the CoordinateInput.EnableUndoStep method was used.

**Remarks:** Process the input undo event This event is triggered, when during coordinate input the user hits the undo button in the dialog line. Implementing this method is necessary only, when the undo button is shown to the user, i.e. the CoordinateInput.EnableUndoStep method was used.

**Returns:** `bool` — message was processed: True/False

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseInteractor/#BaseInteractor.BaseInteractor.on_input_undo)

#### `on_mouse_leave()`

Handles the mouse leave event. This event is triggered, when the mouse leaves the viewport.

**Remarks:** Handles the mouse leave event. This event is triggered, when the mouse leaves the viewport.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseInteractor/#BaseInteractor.BaseInteractor.on_mouse_leave)

#### `on_preview_draw()`

Handles the preview draw event. This event is triggered, when an input in the dialog line is done (e.g. input of a coordinate).

**Remarks:** Handles the preview draw event. This event is triggered, when an input in the dialog line is done (e.g. input of a coordinate).

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseInteractor/#BaseInteractor.BaseInteractor.on_preview_draw)

#### `on_shortcut_control_input(_value: int) -> bool`

Handles the input inside the shortcut control. This event is triggered, when a shortcut is hit inside a shortcut input control in the dialog line. A shortcut input control is a control in the dialog line, where the user can hit only certain keys, like e.g. ROTATION_ANGLE_STEP where only + and - keys can be hit. Implementing this method makes sense only, when this kind of input control is used!

**Remarks:** Handles the input inside the shortcut control. This event is triggered, when a shortcut is hit inside a shortcut input control in the dialog line. A shortcut input control is a control in the dialog line, where the user can hit only certain keys, like e.g. ROTATION_ANGLE_STEP where only + and - keys can be hit. Implementing this method makes sense only, when this kind of input control is used!

**Parameters:**
- `_value` (int) — value associated with the hit shortcut key

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseInteractor/#BaseInteractor.BaseInteractor.on_shortcut_control_input)

#### `on_value_input_control_enter() -> bool`

Handles the value input control enter event. This event is triggered, when enter key is hit during the input inside the input control located in the dialog line.

**Remarks:** Handles the value input control enter event. This event is triggered, when enter key is hit during the input inside the input control located in the dialog line.

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseInteractor/#BaseInteractor.BaseInteractor.on_value_input_control_enter)

#### `process_mouse_msg(mouse_msg: int, pnt: Point2D, msg_info: Any) -> bool`

Handles the process mouse message event. This event is triggered with each message sent by the mouse, which can be a mouse move, mouse click, zoom out, etc. The message is sent only during mouse actions inside a viewport.

**Remarks:** Handles the process mouse message event. This event is triggered with each message sent by the mouse, which can be a mouse move, mouse click, zoom out, etc. The message is sent only during mouse actions inside a viewport.

**Parameters:**
- `mouse_msg` (int) — The mouse message.
- `pnt` (Point2D) — The input point in view coordinates. The origin is the mid point of the viewport
- `msg_info` (Any) — additional message info.

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseInteractor/#BaseInteractor.BaseInteractor.process_mouse_msg)

#### `reset_param_values(_build_ele_list: list[BuildingElement]) -> None`

Handles the reset parameter values event. This event is triggered after pressing "Restore basic setting" button in the property palette. Implementing this method is necessary only, when the load/save/restore favorite buttons are shown in the property palette, i.e. when the tag ShowFavoriteButtons in the .pyp file is set to True

**Remarks:** Handles the reset parameter values event. This event is triggered after pressing "Restore basic setting" button in the property palette. Implementing this method is necessary only, when the load/save/restore favorite buttons are shown in the property palette, i.e. when the tag ShowFavoriteButtons in the .pyp file is set to True

**Parameters:**
- `_build_ele_list` (list[BuildingElement]) — list with building elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseInteractor/#BaseInteractor.BaseInteractor.reset_param_values)

#### `set_active_palette_page_index(_active_page_index: int) -> None`

Handles the event of changing pages inside the property palette

**Remarks:** Handles the event of changing pages inside the property palette

**Parameters:**
- `_active_page_index` (int) — index of the active page, starting from 0

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseInteractor/#BaseInteractor.BaseInteractor.set_active_palette_page_index)

#### `start_next_input()`

start the next input Overload this member function to execute the needed steps after the execution of a script object interactor (e. g. after an element selection)

**Remarks:** start the next input Overload this member function to execute the needed steps after the execution of a script object interactor (e. g. after an element selection)

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseInteractor/#BaseInteractor.BaseInteractor.start_next_input)

#### `update_after_favorite_read() -> None`

Called after reading the favorite data Implementing this method is necessary only, when the execute_load_favorite method was implemented and additional actions must be performed after the favorite file has been loaded.

**Remarks:** Called after reading the favorite data Implementing this method is necessary only, when the execute_load_favorite method was implemented and additional actions must be performed after the favorite file has been loaded.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseInteractor/#BaseInteractor.BaseInteractor.update_after_favorite_read)

### Properties
- `script_object_interactor` (BaseScriptObjectInteractor | None, get/set) — get the script object interactor

