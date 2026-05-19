---
name: allplan-scriptobjectinteractors-archpointinteractor
description: This skill encodes the allplan 2025.0 surface of the ScriptObjectInteractors.ArchPointInteractor namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions, ArchPointInteractor, ArchPointInteractorResult.
---

# ScriptObjectInteractors.ArchPointInteractor

Auto-generated from vendor docs for allplan 2025.0. 3 types in this namespace.

## ArchPointInteractor (class)

Implementation of the interactor for point input on an architectural element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/ArchPointInteractor/ArchPointInteractor/)

### Constructors
- `ArchPointInteractor( interactor_result: ArchPointInteractorResult, ele_filter: list[GUID] | ArchPointInteractor() | ArchPointInteractor() | ArchPointInteractor() | ArchPointInteractor()` — initialize

### Methods
#### `get_outline_segment_and_point(sel_ele: BaseElementAdapter, input_pnt: Point3D)`

get the outline segment and point related to the input point (maybe an inner tier segment was selected)

**Remarks:** get the outline segment and point related to the input point (maybe an inner tier segment was selected)

**Parameters:**
- `sel_ele` (BaseElementAdapter) — selected element
- `input_pnt` (Point3D) — input point

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/ArchPointInteractor/ArchPointInteractor/#ScriptObjectInteractors.ArchPointInteractor.ArchPointInteractor.get_outline_segment_and_point)

#### `on_mouse_leave()`

Handles the mouse leave event

**Remarks:** Handles the mouse leave event

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/ArchPointInteractor/ArchPointInteractor/#ScriptObjectInteractors.ArchPointInteractor.ArchPointInteractor.on_mouse_leave)

#### `on_preview_draw()`

Handles the preview draw event

**Remarks:** Handles the preview draw event

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/ArchPointInteractor/ArchPointInteractor/#ScriptObjectInteractors.ArchPointInteractor.ArchPointInteractor.on_preview_draw)

#### `process_mouse_msg(mouse_msg: int, pnt: Point2D, msg_info: AddMsgInfo) -> bool`

Handles the process mouse message event

**Remarks:** Handles the process mouse message event

**Parameters:**
- `mouse_msg` (int) — mouse message ID
- `pnt` (Point2D) — input point in Allplan view coordinates
- `msg_info` (AddMsgInfo) — additional mouse message info

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/ArchPointInteractor/ArchPointInteractor/#ScriptObjectInteractors.ArchPointInteractor.ArchPointInteractor.process_mouse_msg)

#### `start_input(coord_input: CoordinateInput)`

start the input

**Remarks:** start the input

**Parameters:**
- `coord_input` (CoordinateInput) — API object for the coordinate input, element selection, ... in the Allplan view

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/ArchPointInteractor/ArchPointInteractor/#ScriptObjectInteractors.ArchPointInteractor.ArchPointInteractor.start_input)

## ArchPointInteractorResult (class)

Implementation of the result of a point input on an architectural element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/ArchPointInteractor/ArchPointInteractorResult/)

### Properties
- `input_point` (Point3D, get) — input point
- `sel_geo_ele` (Line2D | None, get) — Selected geometry element
- `sel_model_ele` (BaseElementAdapter, get) — Selected model element
- `sel_model_ele_geo` (Polygon2D | Polyline2D, get) — Geometry of the selected model element

## Functions (static-class)

Module-level functions of ScriptObjectInteractors.ArchPointInteractor

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/ArchPointInteractor/)

