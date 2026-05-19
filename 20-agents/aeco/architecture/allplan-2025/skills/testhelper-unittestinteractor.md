---
name: allplan-testhelper-unittestinteractor
description: This skill encodes the allplan 2025.0 surface of the TestHelper.UnitTestInteractor namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: UnitTestInteractorWrapper, Functions, UnitTestInteractor.
---

# TestHelper.UnitTestInteractor

Auto-generated from vendor docs for allplan 2025.0. 3 types in this namespace.

## Functions (static-class)

Module-level functions of TestHelper.UnitTestInteractor

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/UnitTestInteractor/)

## UnitTestInteractor (class)

Implementation of the unit test interactor.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/UnitTestInteractor/UnitTestInteractor/)

### Constructors
- `UnitTestInteractor(interactor)` — initialize

### Methods
#### `__new__(interactor) -> UnitTestInteractor`

create the unit test interactor

**Remarks:** create the unit test interactor

**Parameters:**
- `interactor` (object) — original interactor

**Returns:** `UnitTestInteractor` — unit test interactor

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/UnitTestInteractor/UnitTestInteractor/#TestHelper.UnitTestInteractor.UnitTestInteractor.__new__)

## UnitTestInteractorWrapper (class)

implementation of the unit test interactor wrapper

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/UnitTestInteractor/UnitTestInteractorWrapper/)

### Methods
#### `modify_element_property(page: str, name: str, value: Any)`

Modify property of element

**Remarks:** Modify property of element

**Parameters:**
- `page` (str) — the page of the property
- `name` (str) — the name of the property.
- `value` (Any) — new value for property.

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/UnitTestInteractor/UnitTestInteractorWrapper/#TestHelper.UnitTestInteractor.UnitTestInteractorWrapper.modify_element_property)

#### `on_cancel_function() -> bool`

Handles the cancel function event (e.g. by ESC, ...)

**Remarks:** Handles the cancel function event (e.g. by ESC, ...)

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/UnitTestInteractor/UnitTestInteractorWrapper/#TestHelper.UnitTestInteractor.UnitTestInteractorWrapper.on_cancel_function)

#### `on_control_event(event_id: int)`

Handles the on control event

**Remarks:** Handles the on control event

**Parameters:**
- `event_id` (int) — event id of button control.

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/UnitTestInteractor/UnitTestInteractorWrapper/#TestHelper.UnitTestInteractor.UnitTestInteractorWrapper.on_control_event)

#### `on_mouse_leave()`

Handles the mouse leave event

**Remarks:** Handles the mouse leave event

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/UnitTestInteractor/UnitTestInteractorWrapper/#TestHelper.UnitTestInteractor.UnitTestInteractorWrapper.on_mouse_leave)

#### `on_preview_draw()`

Handles the preview draw event

**Remarks:** Handles the preview draw event

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/UnitTestInteractor/UnitTestInteractorWrapper/#TestHelper.UnitTestInteractor.UnitTestInteractorWrapper.on_preview_draw)

#### `on_value_input_control_enter() -> bool`

Handles the enter inside the value input control event

**Remarks:** Handles the enter inside the value input control event

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/UnitTestInteractor/UnitTestInteractorWrapper/#TestHelper.UnitTestInteractor.UnitTestInteractorWrapper.on_value_input_control_enter)

#### `process_mouse_msg(mouse_msg: int, pnt: Point2D, msg_info: Any) -> bool`

Handles the process mouse message event

**Remarks:** Handles the process mouse message event

**Parameters:**
- `mouse_msg` (int) — the mouse message.
- `pnt` () — the input point.
- `msg_info` (object) — additional message info.

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/UnitTestInteractor/UnitTestInteractorWrapper/#TestHelper.UnitTestInteractor.UnitTestInteractorWrapper.process_mouse_msg)

