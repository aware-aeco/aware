---
name: allplan-scriptobjectinteractors-singleelementselectinteractor
description: This skill encodes the allplan 2025.0 surface of the ScriptObjectInteractors.SingleElementSelectInteractor namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions, SingleElementSelectInteractor, SingleElementSelectResult.
---

# ScriptObjectInteractors.SingleElementSelectInteractor

Auto-generated from vendor docs for allplan 2025.0. 3 types in this namespace.

## Functions (static-class)

Module-level functions of ScriptObjectInteractors.SingleElementSelectInteractor

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/SingleElementSelectInteractor/)

## SingleElementSelectInteractor (class)

implementation of the interactor for the single element selection

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/SingleElementSelectInteractor/SingleElementSelectInteractor/)

### Constructors
- `SingleElementSelectInteractor( interactor_result: SingleElementSelectResult, ele_filter: list[GUID] | SingleElementSelectInteractor() | SingleElementSelectInteractor() | SingleElementSelectInteractor()` — initialize

### Methods
#### `process_mouse_msg(mouse_msg: int, pnt: Point2D, msg_info: AddMsgInfo) -> bool`

Handles the process mouse message event

**Remarks:** Handles the process mouse message event

**Parameters:**
- `mouse_msg` (int) — mouse message ID
- `pnt` (Point2D) — input point in Allplan view coordinates
- `msg_info` (AddMsgInfo) — additional mouse message info

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/SingleElementSelectInteractor/SingleElementSelectInteractor/#ScriptObjectInteractors.SingleElementSelectInteractor.SingleElementSelectInteractor.process_mouse_msg)

#### `start_input(coord_input: CoordinateInput)`

start the input

**Remarks:** start the input

**Parameters:**
- `coord_input` (CoordinateInput) — API object for the coordinate input, element selection, ... in the Allplan view

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/SingleElementSelectInteractor/SingleElementSelectInteractor/#ScriptObjectInteractors.SingleElementSelectInteractor.SingleElementSelectInteractor.start_input)

## SingleElementSelectResult (class)

implementation of the single element selection result

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/SingleElementSelectInteractor/SingleElementSelectResult/)

### Properties
- `input_point` (Point3D, get) — Input point
- `sel_element` (BaseElementAdapter, get) — Selected element
- `sel_geo_ele` (Any, get) — Geometry of the selected element

