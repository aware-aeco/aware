---
name: allplan-utils-pythonpart-modifypythonpartutil
description: This skill encodes the allplan 2025.0 surface of the Utils.PythonPart.ModifyPythonPartUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ModifyPythonPartUtil.
---

# Utils.PythonPart.ModifyPythonPartUtil

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## ModifyPythonPartUtil (class)

implementation of the utility class for the PythonPart modification

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/Utils/PythonPart/ModifyPythonPartUtil/)

### Constructors
- `ModifyPythonPartUtil(python_part: BaseElementAdapter)` — initialize

### Methods
#### `modify_element_property(page: int, name: str, value: Any)`

Handles the modify element property event. This event is triggered with each modification of the element property done in the property palette or by using a handle.

**Remarks:** Handles the modify element property event. This event is triggered with each modification of the element property done in the property palette or by using a handle.

**Parameters:**
- `page` (int) — Page of the modified property
- `name` (str) — Name of the modified property.
- `value` (Any) — New value of the modified property.

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/PythonPart/ModifyPythonPartUtil/#Utils.PythonPart.ModifyPythonPartUtil.ModifyPythonPartUtil.modify_element_property)

#### `on_cancel_function() -> bool`

Handles the cancel function event This event is triggered when the ESC button is hit during the runtime of the PythonPart.

**Remarks:** Handles the cancel function event This event is triggered when the ESC button is hit during the runtime of the PythonPart.

**Returns:** `bool` — True when the PythonPart framework should terminate the PythonPart, False otherwise.

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/PythonPart/ModifyPythonPartUtil/#Utils.PythonPart.ModifyPythonPartUtil.ModifyPythonPartUtil.on_cancel_function)

#### `on_control_event(event_id: int)`

Handles the on control event. Called when an event is triggered by a palette control (ex. button).

**Remarks:** Handles the on control event. Called when an event is triggered by a palette control (ex. button).

**Parameters:**
- `event_id` (int) — event id of the clicked button control

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/PythonPart/ModifyPythonPartUtil/#Utils.PythonPart.ModifyPythonPartUtil.ModifyPythonPartUtil.on_control_event)

#### `on_mouse_leave()`

Handles the mouse leave event. This event is triggered, when the mouse leaves the viewport.

**Remarks:** Handles the mouse leave event. This event is triggered, when the mouse leaves the viewport.

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/PythonPart/ModifyPythonPartUtil/#Utils.PythonPart.ModifyPythonPartUtil.ModifyPythonPartUtil.on_mouse_leave)

#### `on_preview_draw()`

Handles the preview draw event. This event is triggered, when an input in the dialog line is done (e.g. input of a coordinate).

**Remarks:** Handles the preview draw event. This event is triggered, when an input in the dialog line is done (e.g. input of a coordinate).

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/PythonPart/ModifyPythonPartUtil/#Utils.PythonPart.ModifyPythonPartUtil.ModifyPythonPartUtil.on_preview_draw)

#### `on_value_input_control_enter() -> bool`

Handles the value input control enter event. This event is triggered, when enter key is hit during the input inside the input control located in the dialog line.

**Remarks:** Handles the value input control enter event. This event is triggered, when enter key is hit during the input inside the input control located in the dialog line.

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/PythonPart/ModifyPythonPartUtil/#Utils.PythonPart.ModifyPythonPartUtil.ModifyPythonPartUtil.on_value_input_control_enter)

#### `process_mouse_msg(mouse_msg: int, pnt: Point2D, msg_info: Any) -> bool`

Handles the process mouse message event. This event is triggered with each message sent by the mouse, which can be a mouse move, mouse click, zoom out, etc. The message is sent only during mouse actions inside a viewport.

**Remarks:** Handles the process mouse message event. This event is triggered with each message sent by the mouse, which can be a mouse move, mouse click, zoom out, etc. The message is sent only during mouse actions inside a viewport.

**Parameters:**
- `mouse_msg` (int) — The mouse message.
- `pnt` (Point2D) — The input point in view coordinates. The origin is the mid point of the viewport
- `msg_info` (Any) — additional message info.

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/PythonPart/ModifyPythonPartUtil/#Utils.PythonPart.ModifyPythonPartUtil.ModifyPythonPartUtil.process_mouse_msg)

#### `start_input(coord_input: CoordinateInput)`

start the modification

**Remarks:** start the modification

**Parameters:**
- `coord_input` (CoordinateInput) — API object for the coordinate input, element selection, ... in the Allplan view

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/PythonPart/ModifyPythonPartUtil/#Utils.PythonPart.ModifyPythonPartUtil.ModifyPythonPartUtil.start_input)

