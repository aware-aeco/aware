---
name: allplan-scriptobjectinteractors-archoffsetpointinteractor
description: This skill encodes the allplan 2025.0 surface of the ScriptObjectInteractors.ArchOffsetPointInteractor namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions, ArchOffsetPointInteractor, ArchOffsetPointInteractorResult.
---

# ScriptObjectInteractors.ArchOffsetPointInteractor

Auto-generated from vendor docs for allplan 2025.0. 3 types in this namespace.

## ArchOffsetPointInteractor (class)

Implementation of the interactor for point input on an architectural element with an additional offset input after picking the point.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/ArchOffsetPointInteractor/ArchOffsetPointInteractor/)

### Constructors
- `ArchOffsetPointInteractor( interactor_result: ArchOffsetPointInteractorResult, request_text: str, preview_function: Callable[[], None] | ArchOffsetPointInteractor()` — initialize

### Methods
#### `on_mouse_leave()`

Handles the mouse leave event

**Remarks:** Handles the mouse leave event

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/ArchOffsetPointInteractor/ArchOffsetPointInteractor/#ScriptObjectInteractors.ArchOffsetPointInteractor.ArchOffsetPointInteractor.on_mouse_leave)

#### `on_preview_draw()`

Handles the preview draw event

**Remarks:** Handles the preview draw event

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/ArchOffsetPointInteractor/ArchOffsetPointInteractor/#ScriptObjectInteractors.ArchOffsetPointInteractor.ArchOffsetPointInteractor.on_preview_draw)

#### `process_mouse_msg(mouse_msg: int, pnt: Point2D, msg_info: AddMsgInfo) -> bool`

Handles the process mouse message event

**Remarks:** Handles the process mouse message event

**Parameters:**
- `mouse_msg` (int) — mouse message ID
- `pnt` (Point2D) — input point in Allplan view coordinates
- `msg_info` (AddMsgInfo) — additional mouse message info

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/ArchOffsetPointInteractor/ArchOffsetPointInteractor/#ScriptObjectInteractors.ArchOffsetPointInteractor.ArchOffsetPointInteractor.process_mouse_msg)

#### `start_input(coord_input: CoordinateInput)`

start the input

**Remarks:** start the input

**Parameters:**
- `coord_input` (CoordinateInput) — API object for the coordinate input, element selection, ... in the Allplan view

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/ArchOffsetPointInteractor/ArchOffsetPointInteractor/#ScriptObjectInteractors.ArchOffsetPointInteractor.ArchOffsetPointInteractor.start_input)

## ArchOffsetPointInteractorResult (class)

Implementation of the result of a point input on an architectural element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/ArchOffsetPointInteractor/ArchOffsetPointInteractorResult/)

### Properties
- `input_point` (Point3D, get) — Selected point
- `sel_geo_ele` (CURVES, get) — Selected geometry element
- `sel_wall` (BaseElementAdapter, get) — Selected wall
- `show_simple_preview` (bool, get) — Whether a simplified preview should be shown

## Functions (static-class)

Module-level functions of ScriptObjectInteractors.ArchOffsetPointInteractor

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/ArchOffsetPointInteractor/)

