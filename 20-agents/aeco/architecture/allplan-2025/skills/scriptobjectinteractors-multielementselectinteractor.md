---
name: allplan-scriptobjectinteractors-multielementselectinteractor
description: This skill encodes the allplan 2025.0 surface of the ScriptObjectInteractors.MultiElementSelectInteractor namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions, MultiElementSelectInteractor, MultiElementSelectInteractorResult.
---

# ScriptObjectInteractors.MultiElementSelectInteractor

Auto-generated from vendor docs for allplan 2025.0. 3 types in this namespace.

## Functions (static-class)

Module-level functions of ScriptObjectInteractors.MultiElementSelectInteractor

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/MultiElementSelectInteractor/)

## MultiElementSelectInteractor (class)

Implementation of the interactor for selecting multiple elements

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/MultiElementSelectInteractor/MultiElementSelectInteractor/)

### Constructors
- `MultiElementSelectInteractor( interactor_result: MultiElementSelectInteractorResult, ele_filter: list[GUID] | MultiElementSelectInteractor() | MultiElementSelectInteractor() | MultiElementSelectInteractor()` — Initialize the multiple element selection interactor

### Methods
#### `__del__()`

Default destructor Remove the selection function from the stack, if it is running.

**Remarks:** Default destructor Remove the selection function from the stack, if it is running.

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/MultiElementSelectInteractor/MultiElementSelectInteractor/#ScriptObjectInteractors.MultiElementSelectInteractor.MultiElementSelectInteractor.__del__)

#### `close_selection()`

close the selection

**Remarks:** close the selection

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/MultiElementSelectInteractor/MultiElementSelectInteractor/#ScriptObjectInteractors.MultiElementSelectInteractor.MultiElementSelectInteractor.close_selection)

#### `process_mouse_msg( _mouse_msg: int, _pnt: Point2D, _msg_info: AddMsgInfo ) -> bool`

Handles the process mouse message event

**Remarks:** Handles the process mouse message event

**Parameters:**
- `_mouse_msg` (int) — mouse message ID
- `_pnt` (Point2D) — input point in Allplan view coordinates
- `_msg_info` (AddMsgInfo) — additional mouse message info

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/MultiElementSelectInteractor/MultiElementSelectInteractor/#ScriptObjectInteractors.MultiElementSelectInteractor.MultiElementSelectInteractor.process_mouse_msg)

#### `start_input(coord_input: CoordinateInput)`

Start the multiple element selection

**Remarks:** Start the multiple element selection

**Parameters:**
- `coord_input` (CoordinateInput) — API object for the coordinate input, element selection, ... in the Allplan view

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/MultiElementSelectInteractor/MultiElementSelectInteractor/#ScriptObjectInteractors.MultiElementSelectInteractor.MultiElementSelectInteractor.start_input)

## MultiElementSelectInteractorResult (class)

Data class containing results of multiple element selection

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/MultiElementSelectInteractor/MultiElementSelectInteractorResult/)

### Constructors
- `MultiElementSelectInteractorResult()` — initialize

### Properties
- `sel_elements` (BaseElementAdapterList, get) — List of selected elements

