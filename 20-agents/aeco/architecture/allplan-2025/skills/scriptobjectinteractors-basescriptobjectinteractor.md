---
name: allplan-scriptobjectinteractors-basescriptobjectinteractor
description: This skill encodes the allplan 2025.0 surface of the ScriptObjectInteractors.BaseScriptObjectInteractor namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BaseScriptObjectInteractor.
---

# ScriptObjectInteractors.BaseScriptObjectInteractor

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## BaseScriptObjectInteractor (interface)

implementation of the base script object interactor

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/BaseScriptObjectInteractor/)

### Methods
#### `create_selection_query( ele_filter: list[GUID] | BaseFilterObject | SelectionQuery | None, ) -> SelectionQuery`

create the selection query

**Remarks:** create the selection query

**Parameters:**
- `ele_filter` (list[GUID] | BaseFilterObject | SelectionQuery | None) — element filter as a list of accepted element type GUIDs or a callable or a selection query

**Returns:** `SelectionQuery` — selection query or None

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/BaseScriptObjectInteractor/#ScriptObjectInteractors.BaseScriptObjectInteractor.BaseScriptObjectInteractor.create_selection_query)

#### `on_cancel_function() -> OnCancelFunctionResult`

Handles the cancel function event (e.g. by ESC, ...)

**Remarks:** Handles the cancel function event (e.g. by ESC, ...)

**Returns:** `True` — cancel the input

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/BaseScriptObjectInteractor/#ScriptObjectInteractors.BaseScriptObjectInteractor.BaseScriptObjectInteractor.on_cancel_function)

#### `on_mouse_leave()`

Handles the mouse leave event

**Remarks:** Handles the mouse leave event

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/BaseScriptObjectInteractor/#ScriptObjectInteractors.BaseScriptObjectInteractor.BaseScriptObjectInteractor.on_mouse_leave)

#### `on_preview_draw()`

Handles the preview draw event

**Remarks:** Handles the preview draw event

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/BaseScriptObjectInteractor/#ScriptObjectInteractors.BaseScriptObjectInteractor.BaseScriptObjectInteractor.on_preview_draw)

#### `process_mouse_msg(mouse_msg: int, pnt: Point2D, msg_info: AddMsgInfo) -> bool`

Handles the process mouse message event

**Remarks:** Handles the process mouse message event

**Parameters:**
- `mouse_msg` (int) — mouse message ID
- `pnt` (Point2D) — input point in Allplan view coordinates
- `msg_info` (AddMsgInfo) — additional mouse message info

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/BaseScriptObjectInteractor/#ScriptObjectInteractors.BaseScriptObjectInteractor.BaseScriptObjectInteractor.process_mouse_msg)

#### `start_input(coord_input: CoordinateInput)`

start the input

**Remarks:** start the input

**Parameters:**
- `coord_input` (CoordinateInput) — API object for the coordinate input, element selection, ... in the Allplan view

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/BaseScriptObjectInteractor/#ScriptObjectInteractors.BaseScriptObjectInteractor.BaseScriptObjectInteractor.start_input)

