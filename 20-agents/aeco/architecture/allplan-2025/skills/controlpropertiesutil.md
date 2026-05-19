---
name: allplan-controlpropertiesutil
description: This skill encodes the allplan 2025.0 surface of the ControlPropertiesUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ControlPropertiesUtil.
---

# ControlPropertiesUtil

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## ControlPropertiesUtil (class)

Implementation of the control properties utilities

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/ControlPropertiesUtil/)

### Constructors
- `ControlPropertiesUtil( control_props_list: list[BuildingElementControlProperties], build_ele_list: list[BuildingElement], )` — initialize

### Methods
#### `set_background_color( value_name: str, background_color: str, element_id: str = "" )`

set the background color control property

**Remarks:** set the background color control property

**Parameters:**
- `value_name` (str) — value name of the control
- `background_color` (str) — background color in the format "(red, green, blue)" with "(-1, -1, -1)" as default
- `element_id` (str) — element ID of the building element. Defaults to "".

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/ControlPropertiesUtil/#ControlPropertiesUtil.ControlPropertiesUtil.set_background_color)

#### `set_enable_condition( value_name: str, enable_condition: str, element_id: str = "" )`

set the enable condition control property

**Remarks:** set the enable condition control property

**Parameters:**
- `value_name` () — value name of the control
- `enable_condition` (str) — enable condition control property
- `element_id` () — element ID of the building element. Defaults to "".

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/ControlPropertiesUtil/#ControlPropertiesUtil.ControlPropertiesUtil.set_enable_condition)

#### `set_enable_function( value_name: str, enable_function: Callable[..., bool] | None, element_id: str = "", )`

set the enable function control property

**Remarks:** set the enable function control property

**Parameters:**
- `value_name` (str) — value name of the control
- `enable_function` (Callable[..., bool] | None) — enable function control property
- `element_id` (str) — element ID of the building element. Defaults to "".

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/ControlPropertiesUtil/#ControlPropertiesUtil.ControlPropertiesUtil.set_enable_function)

#### `set_global_attributes(global_attributes: dict[str, Any])`

set the global attributes

**Remarks:** set the global attributes

**Parameters:**
- `global_attributes` (dict[str, Any]) — global attributes

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/ControlPropertiesUtil/#ControlPropertiesUtil.ControlPropertiesUtil.set_global_attributes)

#### `set_max_value(value_name: str, max_value: str, element_id: str = '')`

set the max value control property

**Remarks:** set the max value control property

**Parameters:**
- `value_name` (object) — value name of the control
- `max_value` () — maximal value
- `element_id` (object) — element ID of the building element. Defaults to "".

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/ControlPropertiesUtil/#ControlPropertiesUtil.ControlPropertiesUtil.set_max_value)

#### `set_min_value(value_name: str, min_value: str, element_id: str = '')`

set the min value control property

**Remarks:** set the min value control property

**Parameters:**
- `value_name` (object) — value name of the control
- `min_value` () — minimal value
- `element_id` (object) — element ID of the building element. Defaults to "".

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/ControlPropertiesUtil/#ControlPropertiesUtil.ControlPropertiesUtil.set_min_value)

#### `set_text(value_name: str, text: str, element_id: str = '')`

set the text control property

**Remarks:** set the text control property

**Parameters:**
- `value_name` (str) — value name of the control
- `text` (str) — text control property
- `element_id` (str) — element ID of the building element. Defaults to "".

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/ControlPropertiesUtil/#ControlPropertiesUtil.ControlPropertiesUtil.set_text)

#### `set_value_list(value_name: str, value_list: str, element_id: str = '')`

set the value list control property

**Remarks:** set the value list control property

**Parameters:**
- `value_name` (str) — value name of the control
- `value_list` (str) — value list control property
- `element_id` (str) — element ID of the building element. Defaults to "".

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/ControlPropertiesUtil/#ControlPropertiesUtil.ControlPropertiesUtil.set_value_list)

#### `set_value_list_2(value_name: str, value_list_2: str, element_id: str = '')`

set the value list 2 control property

**Remarks:** set the value list 2 control property

**Parameters:**
- `value_name` (str) — value name of the control
- `value_list_2` (str) — value list 2 control property
- `element_id` (str) — element ID of the building element. Defaults to "".

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/ControlPropertiesUtil/#ControlPropertiesUtil.ControlPropertiesUtil.set_value_list_2)

#### `set_value_text_list( value_name: str, value_text_list: str, element_id: str = "" )`

set the value list 2 control property

**Remarks:** set the value list 2 control property

**Parameters:**
- `value_name` (str) — value name of the control
- `value_text_list` (str) — value text list
- `element_id` (str) — element ID of the building element. Defaults to "".

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/ControlPropertiesUtil/#ControlPropertiesUtil.ControlPropertiesUtil.set_value_text_list)

#### `set_visible_condition( value_name: str, visible_condition: str, element_id: str = "" )`

set the visible condition control property

**Remarks:** set the visible condition control property

**Parameters:**
- `value_name` (str) — value name of the control
- `visible_condition` (str) — visible condition control property
- `element_id` (str) — element ID of the building element. Defaults to "".

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/ControlPropertiesUtil/#ControlPropertiesUtil.ControlPropertiesUtil.set_visible_condition)

#### `set_visible_function( value_name: str, visible_function: Callable[..., bool] | None, element_id: str = "", )`

set the visible function control property

**Remarks:** set the visible function control property

**Parameters:**
- `value_name` (str) — value name of the control
- `visible_function` (Callable[..., bool] | None) — visible function control property
- `element_id` (str) — element ID of the building element. Defaults to "".

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/ControlPropertiesUtil/#ControlPropertiesUtil.ControlPropertiesUtil.set_visible_function)

