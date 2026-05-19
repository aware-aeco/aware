---
name: allplan-controlproperties
description: This skill encodes the allplan 2025.0 surface of the ControlProperties namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ControlProperties.
---

# ControlProperties

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## ControlProperties (enum)

Implementation of the control properties class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/ControlProperties/)

### Constructors
- `ControlProperties( text: str, value_name: str, enable_condition: str, visible_condition: str, page: int, expander_name: str, row_name: str, value_str: str, value_list: str, value_list_2: str, event_id: str, control_type: ControlType = CONTROL, value_index_name: str = "", value_list_start_row: int = 0, value_dialog: ValueDialogType | ControlProperties()` — Set the properties of the control

### Methods
#### `__repr__() -> str`

create the string from the values

**Remarks:** create the string from the values

**Returns:** `str` — string from the values

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/ControlProperties/#ControlProperties.ControlProperties.__repr__)

#### `deep_copy() -> ControlProperties`

deep copy of the data

**Remarks:** deep copy of the data

**Returns:** `ControlProperties` — copied control properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/ControlProperties/#ControlProperties.ControlProperties.deep_copy)

#### `modify_visible_condition(value_name: str, value: str)`

Modify the visible condition

**Remarks:** Modify the visible condition

**Parameters:**
- `value_name` (str) — Name of the value
- `value` (str) — condition value

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/ControlProperties/#ControlProperties.ControlProperties.modify_visible_condition)

#### `reset_refresh_control()`

reset the refresh state of the control

**Remarks:** reset the refresh state of the control

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/ControlProperties/#ControlProperties.ControlProperties.reset_refresh_control)

#### `set_member_text(member_name: str, member_text: str)`

Set the member text of the property member

**Remarks:** Set the member text of the property member

**Parameters:**
- `member_name` (str) — member name
- `member_text` (str) — member text

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/ControlProperties/#ControlProperties.ControlProperties.set_member_text)

#### `set_visible_condition(value_name: str, condition: str)`

Set the visible condition of the property

**Remarks:** Set the visible condition of the property

**Parameters:**
- `value_name` (str) — Name of the value
- `condition` (str) — Visible condition

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/ControlProperties/#ControlProperties.ControlProperties.set_visible_condition)

### Properties
- `as_slider` (bool, get/set) — Get the control as slider state
- `background_color` (str, get/set) — Get the background color of the control
- `constraint` (list[str], get/set) — Get the constraint name of the control
- `control_type` (ControlType, get/set) — Get the control type of the control
- `enable` (bool, get/set) — Get the enable state of the control
- `enable_condition` (str, get/set) — Get the enable condition of the control
- `enable_function` (Callable[..., bool] | None, get/set) — Get the enable function of the control
- `event_id` (str, get/set) — Get the event id of the control
- `expander_name` (str, get/set) — Get the name of the current expander control
- `expander_state_key` (str, get/set) — Get the expander state key of the control
- `font_face_code` (int, get/set) — Get the font face code of the control
- `font_style` (int, get/set) — Get the font style of the control
- `group_text` (str, get/set) — Get the group text of the property
- `height` (int, get/set) — Get the height of the control
- `interval_value` (str, get/set) — Get the interval value of the control
- `list_group_name` (str, get/set) — Get the name of the list group
- `max_value` (int | float, get/set) — Get the max value of the control
- `max_value_condition` (str, get/set) — Get the max value condition of the control
- `member_text` (dict[str, str], get/set) — Get the member text of the property members
- `min_value` (int | float, get/set) — Get the min value of the control
- `min_value_condition` (str, get/set) — Get the min value condition of the control
- `page` (int, get/set) — Get the page of the control
- `palette_layout_dict` (dict[str, Any], get/set) — get the palette layout dictionary
- `refresh_control` (bool, get) — Get the refresh control state
- `row_name` (str, get/set) — Get the row name of the control
- `row_state_key` (str, get/set) — Get the row state key of the control
- `text` (str, get/set) — Get the control text
- `value_dialog` (ValueDialogType | None, get/set) — Get the dialog name for the value input dialog
- `value_index_name` (str, get/set) — Get the value index name of the control
- `value_list` (str, get/set) — Get the values for the value list list like 1|2|...|...
- `value_list_2` (str, get/set) — Get the second value list
- `value_list_start_row` (int, get/set) — Get the start row of the value list row numbering inside the palette
- `value_list_textids` (str, get) — Get the second value list
- `value_name` (str, get/set) — Get the name of the assigned value from the building element
- `value_str` (str, get) — Get the control value string
- `value_text_list` (str, get/set) — Get the text corresponding to the value list like Rectangle|Circle|...|...
- `visible` (bool | list[bool], get/set) — Get the visible state of the control
- `visible_condition` (str, get/set) — Get the visible condition of the control
- `visible_function` (Callable[..., bool] | None, get/set) — Get the visible function of the control
- `width` (int, get/set) — Get the width of the control

