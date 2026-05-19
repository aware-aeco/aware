---
name: allplan-visualscriptservice
description: This skill encodes the allplan 2024.0 surface of the VisualScriptService namespace — 2 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: DefaultValue, VisualScriptService.
---

# VisualScriptService

Auto-generated from vendor docs for allplan 2024.0. 2 types in this namespace.

## DefaultValue (class)

implementation of the data class for the default value

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/VisualScriptService/DefaultValue/)

## VisualScriptService (class)

implementation of the VisualScript service

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/VisualScriptService/VisualScriptService/)

### Constructors
- `VisualScriptService(coord_input, script_path, global_str_table_service, build_ele_list, control_props_list, default_values)` — Initialize the script

### Methods
#### `close_all()`

close the VS script

**Remarks:** close the VS script

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/VisualScriptService/VisualScriptService/#VisualScriptService.VisualScriptService.close_all)

#### `create_pythonpart(placement_matrix, local_placement_matrix)`

create the PythonPart

**Remarks:** create the PythonPart

**Parameters:**
- `placement_matrix` (Matrix3D) — placement matrix of the PythonPart (model placement)
- `local_placement_matrix` (Matrix3D) — local placement matrix of the PythonPart, used for the local geometry transformation

**Returns:** `List[Any]` — created PythonPart elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/VisualScriptService/VisualScriptService/#VisualScriptService.VisualScriptService.create_pythonpart)

#### `execute_load_favorite(file_name)`

load the favorite data

**Remarks:** load the favorite data

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/VisualScriptService/VisualScriptService/#VisualScriptService.VisualScriptService.execute_load_favorite)

#### `execute_save_favorite(file_name)`

save the favorite data

**Remarks:** save the favorite data

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/VisualScriptService/VisualScriptService/#VisualScriptService.VisualScriptService.execute_save_favorite)

#### `get_preview_elements()`

get the preview elements created by the script

**Remarks:** get the preview elements created by the script

**Returns:** `List[Any]` — preview elements of the script

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/VisualScriptService/VisualScriptService/#VisualScriptService.VisualScriptService.get_preview_elements)

#### `modify_element_property(page, name, value)`

Modify property of element

**Remarks:** Modify property of element

**Parameters:**
- `page` (int) — the page of the property
- `name` (str) — the name of the property.
- `value` (Any) — new value for property.

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/VisualScriptService/VisualScriptService/#VisualScriptService.VisualScriptService.modify_element_property)

#### `on_cancel_function()`

Check for input function cancel in case of ESC

**Remarks:** Check for input function cancel in case of ESC

**Returns:** `bool` — True/False for success.

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/VisualScriptService/VisualScriptService/#VisualScriptService.VisualScriptService.on_cancel_function)

#### `on_control_event(event_id)`

Handles on control event

**Remarks:** Handles on control event

**Parameters:**
- `event_id` (int) — event id of control.

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/VisualScriptService/VisualScriptService/#VisualScriptService.VisualScriptService.on_control_event)

