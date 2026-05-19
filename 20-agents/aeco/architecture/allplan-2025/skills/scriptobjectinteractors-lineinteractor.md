---
name: allplan-scriptobjectinteractors-lineinteractor
description: This skill encodes the allplan 2025.0 surface of the ScriptObjectInteractors.LineInteractor namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions, LineInteractor, LineInteractorResult.
---

# ScriptObjectInteractors.LineInteractor

Auto-generated from vendor docs for allplan 2025.0. 3 types in this namespace.

## Functions (static-class)

Module-level functions of ScriptObjectInteractors.LineInteractor

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/LineInteractor/)

## LineInteractor (class)

Implementation of the interactor for line input

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/LineInteractor/LineInteractor/)

### Constructors
- `LineInteractor( interactor_result: LineInteractorResult, is_first_input: bool, prompt: str, allow_pick_up: bool = False, uvs_to_world: bool = False, preview_function: Callable[[Line3D], list] | LineInteractor() | LineInteractor()` — initialize

### Methods
#### `draw_preview(line: Line3D) -> None`

Draw the preview during the input. If a function generating preview elements from a 3D line is defined, use it. Otherwise preview a red 3D line.

**Remarks:** Draw the preview during the input. If a function generating preview elements from a 3D line is defined, use it. Otherwise preview a red 3D line.

**Parameters:**
- `line` (Line3D) — line to preview

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/LineInteractor/LineInteractor/#ScriptObjectInteractors.LineInteractor.LineInteractor.draw_preview)

#### `get_input_line() -> Line3D | get_input_line(mouse_msg: int, pnt: Point2D, msg_info: AddMsgInfo) -> Line3D`

Construct the input line from the mouse message. This overload is meant to be used inside events, where no mouse message is available ().

**Remarks:** Construct the input line from the mouse message. This overload is meant to be used inside events, where no mouse message is available ().

**Returns:** `Line3D` — The input line as follows:

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/LineInteractor/LineInteractor/#ScriptObjectInteractors.LineInteractor.LineInteractor.get_input_line)

#### `on_cancel_function() -> OnCancelFunctionResult`

Handles the event of hitting ESC during the input Cancels the input, when ESC is hit during the start point input. Otherwise restarts the start point input.

**Remarks:** Handles the event of hitting ESC during the input Cancels the input, when ESC is hit during the start point input. Otherwise restarts the start point input.

**Returns:** `OnCancelFunctionResult` — False during the start point input, True otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/LineInteractor/LineInteractor/#ScriptObjectInteractors.LineInteractor.LineInteractor.on_cancel_function)

#### `on_mouse_leave()`

Handles the event of mouse leaving the viewport

**Remarks:** Handles the event of mouse leaving the viewport

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/LineInteractor/LineInteractor/#ScriptObjectInteractors.LineInteractor.LineInteractor.on_mouse_leave)

#### `on_preview_draw()`

Handles the preview draw event

**Remarks:** Handles the preview draw event

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/LineInteractor/LineInteractor/#ScriptObjectInteractors.LineInteractor.LineInteractor.on_preview_draw)

#### `process_mouse_msg(mouse_msg: int, pnt: Point2D, msg_info: AddMsgInfo) -> bool`

Handles the process mouse message event

**Remarks:** Handles the process mouse message event

**Parameters:**
- `mouse_msg` (int) — mouse message ID
- `pnt` (Point2D) — input point in Allplan view coordinates
- `msg_info` (AddMsgInfo) — additional mouse message info

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/LineInteractor/LineInteractor/#ScriptObjectInteractors.LineInteractor.LineInteractor.process_mouse_msg)

#### `start_end_point_input()`

Start the input of the line's end point

**Remarks:** Start the input of the line's end point

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/LineInteractor/LineInteractor/#ScriptObjectInteractors.LineInteractor.LineInteractor.start_end_point_input)

#### `start_input(coord_input: CoordinateInput)`

Start the input of the line's start point

**Remarks:** Start the input of the line's start point

**Parameters:**
- `coord_input` (CoordinateInput) — API object for the coordinate input, element selection, ... in the Allplan view

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/LineInteractor/LineInteractor/#ScriptObjectInteractors.LineInteractor.LineInteractor.start_input)

## LineInteractorResult (class)

implementation of the line interactor result

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/LineInteractor/LineInteractorResult/)

### Properties
- `element` (BaseElementAdapter, get) — If line was picked from an existing element, this is its element adapter
- `input_line` (Line3D, get) — Input line
- `input_value` (float, get) — Value from the input value control in the dialog line

