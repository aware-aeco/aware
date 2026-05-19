---
name: allplan-scriptobjectinteractors-pointinteractor
description: This skill encodes the allplan 2025.0 surface of the ScriptObjectInteractors.PointInteractor namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions, PointInteractor, PointInteractorResult.
---

# ScriptObjectInteractors.PointInteractor

Auto-generated from vendor docs for allplan 2025.0. 3 types in this namespace.

## Functions (static-class)

Module-level functions of ScriptObjectInteractors.PointInteractor

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/PointInteractor/)

## PointInteractor (class)

implementation of the interactor for the point input

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/PointInteractor/PointInteractor/)

### Constructors
- `PointInteractor( interactor_result: PointInteractorResult, is_first_input: bool, request_text: str, preview_function: Callable[[], None] | None = None, default_input_value: float | None = None, abscissa_element: CURVES = None, )` — initialize

### Methods
#### `on_mouse_leave()`

Handles the mouse leave event

**Remarks:** Handles the mouse leave event

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/PointInteractor/PointInteractor/#ScriptObjectInteractors.PointInteractor.PointInteractor.on_mouse_leave)

#### `on_preview_draw()`

Handles the preview draw event

**Remarks:** Handles the preview draw event

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/PointInteractor/PointInteractor/#ScriptObjectInteractors.PointInteractor.PointInteractor.on_preview_draw)

#### `process_mouse_msg(mouse_msg: int, pnt: Point2D, msg_info: AddMsgInfo) -> bool`

Handles the process mouse message event

**Remarks:** Handles the process mouse message event

**Parameters:**
- `mouse_msg` (int) — mouse message ID
- `pnt` (Point2D) — input point in Allplan view coordinates
- `msg_info` (AddMsgInfo) — additional mouse message info

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/PointInteractor/PointInteractor/#ScriptObjectInteractors.PointInteractor.PointInteractor.process_mouse_msg)

#### `start_input(coord_input: CoordinateInput)`

start the input

**Remarks:** start the input

**Parameters:**
- `coord_input` (CoordinateInput) — API object for the coordinate input, element selection, ... in the Allplan view

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/PointInteractor/PointInteractor/#ScriptObjectInteractors.PointInteractor.PointInteractor.start_input)

## PointInteractorResult (class)

Implementation of the point input result

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/PointInteractor/PointInteractorResult/)

### Properties
- `input_point` (Point3D, get) — Input point
- `input_value` (float, get) — Value input in the input control in the dialog line

