---
name: allplan-scriptobjectinteractors-polygoninteractor
description: This skill encodes the allplan 2025.0 surface of the ScriptObjectInteractors.PolygonInteractor namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions, PolygonInteractor, PolygonInteractorResult.
---

# ScriptObjectInteractors.PolygonInteractor

Auto-generated from vendor docs for allplan 2025.0. 3 types in this namespace.

## Functions (static-class)

Module-level functions of ScriptObjectInteractors.PolygonInteractor

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/PolygonInteractor/)

## PolygonInteractor (class)

implementation of the interactor for the polygon input

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/PolygonInteractor/PolygonInteractor/)

### Constructors
- `PolygonInteractor( interactor_result: PolygonInteractorResult, common_prop: CommonProperties = GetCurrentCommonProperties(), z_coord_input: bool = True, multi_polygon_input: bool = True, preview_function: Callable[[Polygon3D], list] | None = None, )` — Create the interactor

### Methods
#### `draw_preview(polygon: Polygon3D)`

draw the preview

**Remarks:** draw the preview

**Parameters:**
- `polygon` (Polygon3D) — polygon

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/PolygonInteractor/PolygonInteractor/#ScriptObjectInteractors.PolygonInteractor.PolygonInteractor.draw_preview)

#### `flatten_polygon(polygon: Polygon3D) -> Polygon3D`

Removes the Z coordinate from all vertices of the polygon

**Remarks:** Removes the Z coordinate from all vertices of the polygon

**Parameters:**
- `polygon` (Polygon3D) — 3d polygon

**Returns:** `Polygon3D` — flattened polygon

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/PolygonInteractor/PolygonInteractor/#ScriptObjectInteractors.PolygonInteractor.PolygonInteractor.flatten_polygon)

#### `on_cancel_function() -> OnCancelFunctionResult`

Handles the cancel function event (triggered e.g. by hitting ESC, ...)

**Remarks:** Handles the cancel function event (triggered e.g. by hitting ESC, ...)

**Returns:** `OnCancelFunctionResult` — True/False for success.

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/PolygonInteractor/PolygonInteractor/#ScriptObjectInteractors.PolygonInteractor.PolygonInteractor.on_cancel_function)

#### `on_mouse_leave()`

Handles the mouse leave event

**Remarks:** Handles the mouse leave event

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/PolygonInteractor/PolygonInteractor/#ScriptObjectInteractors.PolygonInteractor.PolygonInteractor.on_mouse_leave)

#### `on_preview_draw()`

Handles the preview draw event

**Remarks:** Handles the preview draw event

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/PolygonInteractor/PolygonInteractor/#ScriptObjectInteractors.PolygonInteractor.PolygonInteractor.on_preview_draw)

#### `process_mouse_msg(mouse_msg: int, pnt: Point2D, msg_info: Any) -> bool`

Process the mouse message event

**Remarks:** Process the mouse message event

**Parameters:**
- `mouse_msg` (int) — mouse message ID
- `pnt` (Point2D) — input point in Allplan view coordinates
- `msg_info` (Any) — additional mouse message info

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/PolygonInteractor/PolygonInteractor/#ScriptObjectInteractors.PolygonInteractor.PolygonInteractor.process_mouse_msg)

#### `start_input(coord_input: CoordinateInput)`

start the input

**Remarks:** start the input

**Parameters:**
- `coord_input` (CoordinateInput) — API object for the coordinate input, element selection, ... in the Allplan view

[Docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/PolygonInteractor/PolygonInteractor/#ScriptObjectInteractors.PolygonInteractor.PolygonInteractor.start_input)

## PolygonInteractorResult (class)

implementation of the polygon input result

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/ScriptObjectInteractors/PolygonInteractor/PolygonInteractorResult/)

### Properties
- `input_polygon` (Polygon3D, get) — Input polygon

