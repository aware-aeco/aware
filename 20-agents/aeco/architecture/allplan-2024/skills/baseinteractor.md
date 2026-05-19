---
name: allplan-baseinteractor
description: This skill encodes the allplan 2024.0 surface of the BaseInteractor namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BaseInteractor.
---

# BaseInteractor

Auto-generated from vendor docs for allplan 2024.0. 1 types in this namespace.

## BaseInteractor (interface)

base class for a PythonPart interactor

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BaseInteractor/BaseInteractor/)

### Methods
#### `modify_element_property(page, name, value)`

Modify property of element

**Remarks:** Modify property of element

**Parameters:**
- `page` (int) — the page of the property
- `name` (str) — the name of the property.
- `value` (Any) — new value for property.

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BaseInteractor/BaseInteractor/#BaseInteractor.BaseInteractor.modify_element_property)

#### `on_cancel_function()`

Handles the cancel function event (e.g. by ESC, ...)

**Remarks:** Handles the cancel function event (e.g. by ESC, ...)

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BaseInteractor/BaseInteractor/#BaseInteractor.BaseInteractor.on_cancel_function)

#### `on_control_event(event_id)`

Handles the on control event

**Remarks:** Handles the on control event

**Parameters:**
- `event_id` (int) — event id of button control.

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BaseInteractor/BaseInteractor/#BaseInteractor.BaseInteractor.on_control_event)

#### `on_mouse_leave()`

Handles the mouse leave event

**Remarks:** Handles the mouse leave event

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BaseInteractor/BaseInteractor/#BaseInteractor.BaseInteractor.on_mouse_leave)

#### `on_preview_draw()`

Handles the preview draw event

**Remarks:** Handles the preview draw event

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BaseInteractor/BaseInteractor/#BaseInteractor.BaseInteractor.on_preview_draw)

#### `on_value_input_control_enter()`

Handles the enter inside the value input control event

**Remarks:** Handles the enter inside the value input control event

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BaseInteractor/BaseInteractor/#BaseInteractor.BaseInteractor.on_value_input_control_enter)

#### `process_mouse_msg(mouse_msg, pnt, msg_info)`

Handles the process mouse message event

**Remarks:** Handles the process mouse message event

**Parameters:**
- `mouse_msg` (int) — the mouse message.
- `pnt` (object) — the input point.
- `msg_info` (object) — additional message info.

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BaseInteractor/BaseInteractor/#BaseInteractor.BaseInteractor.process_mouse_msg)

