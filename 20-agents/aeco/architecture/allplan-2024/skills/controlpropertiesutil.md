---
name: allplan-controlpropertiesutil
description: This skill encodes the allplan 2024.0 surface of the ControlPropertiesUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ControlPropertiesUtil.
---

# ControlPropertiesUtil

Auto-generated from vendor docs for allplan 2024.0. 1 types in this namespace.

## ControlPropertiesUtil (class)

Implementation of the control properties utilities

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ControlPropertiesUtil/ControlPropertiesUtil/)

### Constructors
- `ControlPropertiesUtil(control_props_list, build_ele_list)` — initialize

### Methods
#### `set_background_color(value_name, background_color, element_id='')`

set the background color control property

**Remarks:** set the background color control property

**Parameters:**
- `value_name` (str) — value name of the control
- `background_color` (str) — background color in the format "(red, green, blue)" with "(-1, -1, -1)" as default
- `element_id` (str) — element ID of the building element. Defaults to "".

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ControlPropertiesUtil/ControlPropertiesUtil/#ControlPropertiesUtil.ControlPropertiesUtil.set_background_color)

#### `set_enable_condition(value_name, enable_condition, element_id='')`

set the enable condition control property

**Remarks:** set the enable condition control property

**Parameters:**
- `value_name` (object) — value name of the control
- `enable_condition` (str) — enable condition control property
- `element_id` (object) — element ID of the building element. Defaults to "".

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ControlPropertiesUtil/ControlPropertiesUtil/#ControlPropertiesUtil.ControlPropertiesUtil.set_enable_condition)

#### `set_enable_function(value_name, enable_function, element_id='')`

set the enable function control property

**Remarks:** set the enable function control property

**Parameters:**
- `value_name` (str) — value name of the control
- `enable_function` (Optional[Callable[..., bool]]) — enable function control property
- `element_id` (str) — element ID of the building element. Defaults to "".

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ControlPropertiesUtil/ControlPropertiesUtil/#ControlPropertiesUtil.ControlPropertiesUtil.set_enable_function)

#### `set_global_attributes(global_attributes)`

set the global attributes

**Remarks:** set the global attributes

**Parameters:**
- `global_attributes` (Dict[str, Any]) — global attributes

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ControlPropertiesUtil/ControlPropertiesUtil/#ControlPropertiesUtil.ControlPropertiesUtil.set_global_attributes)

#### `set_max_value(value_name, max_value, element_id='')`

set the max value control property

**Remarks:** set the max value control property

**Parameters:**
- `value_name` (object) — value name of the control
- `max_value` (object) — maximal value
- `element_id` (object) — element ID of the building element. Defaults to "".

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ControlPropertiesUtil/ControlPropertiesUtil/#ControlPropertiesUtil.ControlPropertiesUtil.set_max_value)

#### `set_min_value(value_name, min_value, element_id='')`

set the min value control property

**Remarks:** set the min value control property

**Parameters:**
- `value_name` (object) — value name of the control
- `min_value` (object) — minimal value
- `element_id` (object) — element ID of the building element. Defaults to "".

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ControlPropertiesUtil/ControlPropertiesUtil/#ControlPropertiesUtil.ControlPropertiesUtil.set_min_value)

#### `set_text(value_name, text, element_id='')`

set the text control property

**Remarks:** set the text control property

**Parameters:**
- `value_name` (str) — value name of the control
- `text` (str) — text control property
- `element_id` (str) — element ID of the building element. Defaults to "".

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ControlPropertiesUtil/ControlPropertiesUtil/#ControlPropertiesUtil.ControlPropertiesUtil.set_text)

#### `set_value_list(value_name, value_list, element_id='')`

set the value list control property

**Remarks:** set the value list control property

**Parameters:**
- `value_name` (str) — value name of the control
- `value_list` (str) — value list control property
- `element_id` (str) — element ID of the building element. Defaults to "".

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ControlPropertiesUtil/ControlPropertiesUtil/#ControlPropertiesUtil.ControlPropertiesUtil.set_value_list)

#### `set_value_list_2(value_name, value_list_2, element_id='')`

set the value list 2 control property

**Remarks:** set the value list 2 control property

**Parameters:**
- `value_name` (str) — value name of the control
- `value_list_2` (str) — value list 2 control property
- `element_id` (str) — element ID of the building element. Defaults to "".

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ControlPropertiesUtil/ControlPropertiesUtil/#ControlPropertiesUtil.ControlPropertiesUtil.set_value_list_2)

#### `set_visible_condition(value_name, visible_condition, element_id='')`

set the visible condition control property

**Remarks:** set the visible condition control property

**Parameters:**
- `value_name` (str) — value name of the control
- `visible_condition` (str) — visible condition control property
- `element_id` (str) — element ID of the building element. Defaults to "".

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ControlPropertiesUtil/ControlPropertiesUtil/#ControlPropertiesUtil.ControlPropertiesUtil.set_visible_condition)

#### `set_visible_function(value_name, visible_function, element_id='')`

set the visible function control property

**Remarks:** set the visible function control property

**Parameters:**
- `value_name` (str) — value name of the control
- `visible_function` (Optional[Callable[..., bool]]) — visible function control property
- `element_id` (str) — element ID of the building element. Defaults to "".

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ControlPropertiesUtil/ControlPropertiesUtil/#ControlPropertiesUtil.ControlPropertiesUtil.set_visible_function)

