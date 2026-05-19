---
name: allplan-basescriptobject
description: This skill encodes the allplan 2025.0 surface of the BaseScriptObject namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions, BaseScriptObject, BaseScriptObjectData.
---

# BaseScriptObject

Auto-generated from vendor docs for allplan 2025.0. 3 types in this namespace.

## BaseScriptObject (interface)

implementation of the script object base class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseScriptObject/BaseScriptObject/)

### Constructors
- `BaseScriptObject(script_object_data: BaseScriptObjectData)` — Initialization of class TextExample

### Methods
#### `execute() -> CreateElementResult`

execute the script

**Remarks:** execute the script

**Returns:** `CreateElementResult` — created result

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseScriptObject/BaseScriptObject/#BaseScriptObject.BaseScriptObject.execute)

#### `modify_element_property(_name: str, _value: Any) -> bool`

Modify property of element

**Remarks:** Modify property of element

**Parameters:**
- `_name` (str) — the name of the property.
- `_value` (Any) — new value for property.

**Returns:** `bool` — palette update state

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseScriptObject/BaseScriptObject/#BaseScriptObject.BaseScriptObject.modify_element_property)

#### `move_handle( handle_prop: HandleProperties, input_pnt: Point3D ) -> CreateElementResult`

Modify the element geometry by handles

**Remarks:** Modify the element geometry by handles

**Parameters:**
- `handle_prop` (HandleProperties) — handle properties
- `input_pnt` (Point3D) — input point

**Returns:** `CreateElementResult` — created element result

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseScriptObject/BaseScriptObject/#BaseScriptObject.BaseScriptObject.move_handle)

#### `on_cancel_function() -> OnCancelFunctionResult`

Handles the cancel function event (e.g. by ESC, ...)

**Remarks:** Handles the cancel function event (e.g. by ESC, ...)

**Returns:** `True` — cancel the input

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseScriptObject/BaseScriptObject/#BaseScriptObject.BaseScriptObject.on_cancel_function)

#### `on_control_event(event_id: int) -> bool`

Handles the on control event Called when an event is triggered by a palette control (ex. button).

**Remarks:** Handles the on control event Called when an event is triggered by a palette control (ex. button).

**Parameters:**
- `event_id` (int) — event id of the clicked button control

**Returns:** `bool` — True, when palette should be updated. False otherwise.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseScriptObject/BaseScriptObject/#BaseScriptObject.BaseScriptObject.on_control_event)

#### `on_input_undo() -> bool`

Process the input undo event

**Remarks:** Process the input undo event

**Returns:** `bool` — message was processed: True/False

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseScriptObject/BaseScriptObject/#BaseScriptObject.BaseScriptObject.on_input_undo)

#### `on_shortcut_control_input(_value: int) -> bool`

Handles the input inside the shortcut control

**Remarks:** Handles the input inside the shortcut control

**Parameters:**
- `_value` (int) — shortcut value

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseScriptObject/BaseScriptObject/#BaseScriptObject.BaseScriptObject.on_shortcut_control_input)

#### `on_value_input_control_enter() -> bool`

Process the enter inside the value input control

**Remarks:** Process the enter inside the value input control

**Returns:** `bool` — message was processed: True/False

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseScriptObject/BaseScriptObject/#BaseScriptObject.BaseScriptObject.on_value_input_control_enter)

#### `set_text_for_palette_modification(text: str)`

set an input text for a modification by palette

**Remarks:** set an input text for a modification by palette

**Parameters:**
- `text` (str) — input text

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseScriptObject/BaseScriptObject/#BaseScriptObject.BaseScriptObject.set_text_for_palette_modification)

#### `start_input()`

start the input Overload this member function in the case where a script object interactor (e. g. for an element selection) needs to be started before the script execution

**Remarks:** start the input Overload this member function in the case where a script object interactor (e. g. for an element selection) needs to be started before the script execution

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseScriptObject/BaseScriptObject/#BaseScriptObject.BaseScriptObject.start_input)

#### `start_next_input()`

start the next input Overload this member function to execute the needed steps after the execution of a script object interactor (e. g. after an element selection)

**Remarks:** start the next input Overload this member function to execute the needed steps after the execution of a script object interactor (e. g. after an element selection)

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseScriptObject/BaseScriptObject/#BaseScriptObject.BaseScriptObject.start_next_input)

### Properties
- `document` (DocumentAdapter, get) — get the document
- `script_object_interactor` (BaseScriptObjectInteractor | None, get/set) — get the script object interactor

## BaseScriptObjectData (class)

implementation of the data class for the script object interactor

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseScriptObject/BaseScriptObjectData/)

### Properties
- `control_props_util` (ControlPropertiesUtil, get) — Utility for altering the properties of palette controls
- `coord_input` (CoordinateInput, get) — Object representing user's input in Allplan Viewport
- `is_only_update` (bool, get) — True, when the script execution was triggered by a PythonPart update
- `modification_ele_list` (ModificationElementList, get) — List with UUIDs of modified elements

## Functions (static-class)

Module-level functions of BaseScriptObject

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BaseScriptObject/)

