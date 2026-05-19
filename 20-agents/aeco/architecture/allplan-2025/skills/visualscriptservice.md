---
name: allplan-visualscriptservice
description: This skill encodes the allplan 2025.0 surface of the VisualScriptService namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions, DefaultValue, VisualScriptService.
---

# VisualScriptService

Auto-generated from vendor docs for allplan 2025.0. 3 types in this namespace.

## DefaultValue (class)

implementation of the data class for the default value

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/VisualScriptService/DefaultValue/)

## Functions (static-class)

Module-level functions of VisualScriptService

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/VisualScriptService/)

## VisualScriptService (class)

implementation of the VisualScript service

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/VisualScriptService/VisualScriptService/)

### Constructors
- `VisualScriptService( coord_input: CoordinateInput, script_path: str, global_str_table_service: StringTableService, build_ele_list: list[BuildingElement], control_props_list: list[BuildingElementControlProperties], default_values: list[DefaultValue], )` — Initialize the script

### Methods
#### `close_all()`

close the VS script

**Remarks:** close the VS script

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/VisualScriptService/VisualScriptService/#VisualScriptService.VisualScriptService.close_all)

#### `create_pythonpart( placement_matrix: Matrix3D, local_placement_matrix: Matrix3D ) -> list[Any]`

create the PythonPart

**Remarks:** create the PythonPart

**Parameters:**
- `placement_matrix` (Matrix3D) — placement matrix of the PythonPart (model placement)
- `local_placement_matrix` (Matrix3D) — local placement matrix of the PythonPart, used for the local geometry transformation

**Returns:** `list[Any]` — created PythonPart elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/VisualScriptService/VisualScriptService/#VisualScriptService.VisualScriptService.create_pythonpart)

#### `execute_load_favorite(file_name: str)`

load the favorite data

**Remarks:** load the favorite data

**Parameters:**
- `file_name` (str) — file name

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/VisualScriptService/VisualScriptService/#VisualScriptService.VisualScriptService.execute_load_favorite)

#### `execute_save_favorite(file_name: str)`

save the favorite data

**Remarks:** save the favorite data

**Parameters:**
- `file_name` (str) — file name

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/VisualScriptService/VisualScriptService/#VisualScriptService.VisualScriptService.execute_save_favorite)

#### `get_preview_elements() -> list[Any]`

get the preview elements created by the script

**Remarks:** get the preview elements created by the script

**Returns:** `list[Any]` — preview elements of the script

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/VisualScriptService/VisualScriptService/#VisualScriptService.VisualScriptService.get_preview_elements)

#### `modify_element_property(page: int, name: str, value: Any)`

Modify property of element

**Remarks:** Modify property of element

**Parameters:**
- `page` (int) — page index of the modified property
- `name` (str) — the name of the property.
- `value` (Any) — new value for property.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/VisualScriptService/VisualScriptService/#VisualScriptService.VisualScriptService.modify_element_property)

#### `on_cancel_function() -> bool`

Check for input function cancel in case of ESC

**Remarks:** Check for input function cancel in case of ESC

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/VisualScriptService/VisualScriptService/#VisualScriptService.VisualScriptService.on_cancel_function)

#### `on_control_event(event_id: int)`

Handles on control event

**Remarks:** Handles on control event

**Parameters:**
- `event_id` (int) — event id of the clicked button control

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/VisualScriptService/VisualScriptService/#VisualScriptService.VisualScriptService.on_control_event)

#### `reset_param_values()`

reset the parameter values

**Remarks:** reset the parameter values

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/VisualScriptService/VisualScriptService/#VisualScriptService.VisualScriptService.reset_param_values)

