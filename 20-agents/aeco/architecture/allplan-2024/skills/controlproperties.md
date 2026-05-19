---
name: allplan-controlproperties
description: This skill encodes the allplan 2024.0 surface of the ControlProperties namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ControlProperties.
---

# ControlProperties

Auto-generated from vendor docs for allplan 2024.0. 1 types in this namespace.

## ControlProperties (enum)

Implementation of the control properties class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ControlProperties/ControlProperties/)

### Constructors
- `ControlProperties(text, value_name, enable_condition, visible_condition, page, expander_name, row_name, value_str, value_list, value_list_2, event_id, control_type=CONTROL, value_index_name='', value_list_start_row=0, value_dialog='', value_list_textids='', as_slider=False, height=22, width=DEFAULT_WIDTH, font_style=2, font_face_code=0, background_color='', row_state_key='', expander_state_key='')` — Set the properties of the control

### Methods
#### `__repr__()`

create the string from the values

**Remarks:** create the string from the values

**Returns:** `str` — string from the values

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ControlProperties/ControlProperties/#ControlProperties.ControlProperties.__repr__)

#### `deep_copy()`

deep copy of the data

**Remarks:** deep copy of the data

**Returns:** `ControlProperties` — copied control properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ControlProperties/ControlProperties/#ControlProperties.ControlProperties.deep_copy)

#### `modify_visible_condition(value_name, condition)`

Modify the visible condition

**Remarks:** Modify the visible condition

**Parameters:**
- `value_name` (str) — Name of the value
- `condition` (str) — Visible condition

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ControlProperties/ControlProperties/#ControlProperties.ControlProperties.modify_visible_condition)

#### `reset_refresh_control()`

reset the refresh state of the control

**Remarks:** reset the refresh state of the control

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ControlProperties/ControlProperties/#ControlProperties.ControlProperties.reset_refresh_control)

#### `set_member_text(member_name, member_text)`

Set the member text of the property member

**Remarks:** Set the member text of the property member

**Parameters:**
- `member_name` (str) — member name
- `member_text` (str) — member text

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ControlProperties/ControlProperties/#ControlProperties.ControlProperties.set_member_text)

#### `set_visible_condition(value_name, condition)`

Set the visible condition of the property

**Remarks:** Set the visible condition of the property

**Parameters:**
- `value_name` (str) — Name of the value
- `condition` (str) — Visible condition

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ControlProperties/ControlProperties/#ControlProperties.ControlProperties.set_visible_condition)

### Properties
- `as_slider` (bool, get/set) — Get the control as slider state
- `background_color` (str, get/set) — Get the background color of the control
- `constraint` (List[str], get/set) — Get the constraint name of the control
- `control_type` (ControlType, get/set) — Get the control type of the control
- `enable` (bool, get/set) — Get the enable state of the control
- `enable_condition` (str, get/set) — Get the enable condition of the control
- `enable_function` (Optional[Callable[..., bool]], get/set) — Get the enable function of the control
- `event_id` (str, get/set) — Get the event id of the control
- `expander_name` (str, get/set) — Get the name of the current expander control
- `expander_state_key` (str, get/set) — Get the expander state key of the control
- `font_face_code` (int, get/set) — Get the font face code of the control
- `font_style` (int, get/set) — Get the font style of the control
- `group_text` (str, get/set) — Get the group text of the property
- `height` (int, get/set) — Get the height of the control
- `interval_value` (str, get/set) — Get the interval value of the control
- `list_group_name` (str, get/set) — Get the name of the list group
- `max_value` (Union[int, float], get/set) — Get the max value of the control
- `max_value_condition` (str, get/set) — Get the max value condition of the control
- `member_text` (Dict[str, str], get/set) — Get the member text of the property members
- `min_value` (Union[int, float], get/set) — Get the min value of the control
- `min_value_condition` (str, get/set) — Get the min value condition of the control
- `page` (int, get/set) — Get the page of the control
- `refresh_control` (bool, get) — Get the refresh control state
- `row_name` (str, get/set) — Get the row name of the control
- `row_state_key` (str, get/set) — Get the row state key of the control
- `text` (str, get/set) — Get the control text
- `value_dialog` (str, get/set) — Get the dialog name for the value input dialog
- `value_index_name` (str, get/set) — Get the value index name of the control
- `value_list` (str, get/set) — Get the text with the values for the combo box list like 1|2|5|7
- `value_list_2` (str, get/set) — Get the second value list
- `value_list_start_row` (int, get/set) — Get the start row of the value list row numbering inside the palette
- `value_list_textids` (str, get) — Get the second value list
- `value_name` (str, get/set) — Get the name of the assigned value from the building element
- `value_str` (str, get) — Get the control value string
- `visible` (Union[bool, List[bool]], get/set) — Get the visible state of the control
- `visible_condition` (str, get/set) — Get the visible condition of the control
- `visible_function` (Optional[Callable[..., bool]], get/set) — Get the visible function of the control
- `width` (int, get/set) — Get the width of the control

