---
name: allplan-buildingelement
description: This skill encodes the allplan 2025.0 surface of the BuildingElement namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BuildingElement.
---

# BuildingElement

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## BuildingElement (class)

Definition of class BuildingElement

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/)

### Constructors
- `BuildingElement()` — Initialization of class BuildingElement

### Methods
#### `__repr__() -> str`

get the string from the object member

**Remarks:** get the string from the object member

**Returns:** `str` — data string

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.__repr__)

#### `add_constant(name: str, value: Any)`

Add a new constant to class

**Remarks:** Add a new constant to class

**Parameters:**
- `name` (str) — the name of the constant
- `value` (Any) — the value of the constant

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.add_constant)

#### `add_material_string_table( global_material_str_table: BuildingElementMaterialStringTable, )`

Sets the global material string table

**Remarks:** Sets the global material string table

**Parameters:**
- `global_material_str_table` (BuildingElementMaterialStringTable) — global material string table

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.add_material_string_table)

#### `add_page(name: str, text: str, visible_condition: str, enable_condition: str)`

Add pages for property dialog

**Remarks:** Add pages for property dialog

**Parameters:**
- `name` (str) — the name of the page.
- `text` (str) — text of the page
- `visible_condition` (str) — visible condition of the page
- `enable_condition` (str) — enable condition of the page

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.add_page)

#### `add_property(name: str, value: ParameterProperty)`

Add a new property to class

**Remarks:** Add a new property to class

**Parameters:**
- `name` (str) — the name of the property.
- `value` (ParameterProperty) — the value of the property.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.add_property)

#### `add_string_tables( local_str_table: BuildingElementStringTable, global_str_table: BuildingElementStringTable, )`

Sets the local and global string table

**Remarks:** Sets the local and global string table

**Parameters:**
- `local_str_table` (BuildingElementStringTable) — local string table
- `global_str_table` (BuildingElementStringTable) — global string table

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.add_string_tables)

#### `change_property(handle_prop: HandleProperties, input_pnt: Point3D) -> bool`

Change property value

**Remarks:** Change property value

**Parameters:**
- `handle_prop` (HandleProperties) — handle property
- `input_pnt` (Point3D) — input point

**Returns:** `bool` — update palette state

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.change_property)

#### `deep_copy() -> BuildingElement`

deep copy of the object member

**Remarks:** deep copy of the object member

**Returns:** `BuildingElement` — copied building element

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.deep_copy)

#### `get_constant(name: str) -> Any < None`

get the constant by name

**Remarks:** get the constant by name

**Parameters:**
- `name` (str) — name of the constant

**Returns:** `Any < None` — constant value, None if not exist

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.get_constant)

#### `get_constant_dict() -> dict[str, Any]`

Get constants dictionary

**Remarks:** Get constants dictionary

**Returns:** `dict[str, Any]` — dictionary of constants

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.get_constant_dict)

#### `get_existing_property(name: str) -> ParameterProperty`

Get a mandatory property parameter by name. If the property doesn't exist, a message box is shown and an exception is thrown

**Remarks:** Get a mandatory property parameter by name. If the property doesn't exist, a message box is shown and an exception is thrown

**Parameters:**
- `name` (str) — the name of the property.

**Returns:** `ParameterProperty` — property

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.get_existing_property)

#### `get_float_version() -> float`

Get the version as float number from the string

**Remarks:** Get the version as float number from the string

**Returns:** `float` — version number as float value

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.get_float_version)

#### `get_hash() -> str`

Calculate a hash value for script name and parameter list

**Remarks:** Calculate a hash value for script name and parameter list

**Returns:** `str` — Calculated hash string.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.get_hash)

#### `get_insert_matrix() -> Matrix3D`

Get matrix for macro/ macro group update

**Remarks:** Get matrix for macro/ macro group update

**Returns:** `Matrix3D` — tuple matrix

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.get_insert_matrix)

#### `get_material_string_table() -> BuildingElementMaterialStringTable`

Get the global material string table

**Remarks:** Get the global material string table

**Returns:** `BuildingElementMaterialStringTable` — global material string table

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.get_material_string_table)

#### `get_model_parameter_dict( exclude_identical: bool = False, ) -> dict[str, ParameterProperty]`

Get parameters dictionary for the model parameter

**Remarks:** Get parameters dictionary for the model parameter

**Parameters:**
- `exclude_identical` (bool) — exclude the parameter which should not be used for the identical check

**Returns:** `dict[str, ParameterProperty]` — dictionary of parameters.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.get_model_parameter_dict)

#### `get_modified_properties() -> Iterator[ParameterProperty]`

Get an iterator for the modified properties

**Remarks:** Get an iterator for the modified properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.get_modified_properties)

#### `get_pages() -> list[PageData]`

Get pages for property dialog

**Remarks:** Get pages for property dialog

**Returns:** `list[PageData]` — list of the pages.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.get_pages)

#### `get_parameter_dict() -> dict[str, Any]`

Get parameters dictionary

**Remarks:** Get parameters dictionary

**Returns:** `dict[str, Any]` — dictionary of parameters.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.get_parameter_dict)

#### `get_params_list() -> list[str]`

Get the list with the parameter values

**Remarks:** Get the list with the parameter values

**Returns:** `list[str]` — list with the parameter values

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.get_params_list)

#### `get_properties() -> Iterator[ParameterProperty]`

Get an iterator for the properties

**Remarks:** Get an iterator for the properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.get_properties)

#### `get_property(name: str) -> ParameterProperty | None`

Get property parameter by name

**Remarks:** Get property parameter by name

**Parameters:**
- `name` (str) — the name of the property.

**Returns:** `ParameterProperty | None` — property parameter, None if not exist

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.get_property)

#### `get_reinforcement_definition_list() -> list[ReinforcementDefinition]`

Get reinforcement list

**Remarks:** Get reinforcement list

**Returns:** `list[ReinforcementDefinition]` — list of reinforcement.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.get_reinforcement_definition_list)

#### `get_string_tables() -> ( tuple[BuildingElementStringTable, BuildingElementStringTable] )`

Get the local and global string tables

**Remarks:** Get the local and global string tables

**Returns:** `tuple[BuildingElementStringTable, BuildingElementStringTable]` — tuple of local and global string table

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.get_string_tables)

#### `is_parameter_property(name: str) -> bool`

Check for a parameter property

**Remarks:** Check for a parameter property

**Parameters:**
- `name` (str) — name of the parameter property

**Returns:** `bool` — name has parameter property

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.is_parameter_property)

#### `modify_value_type(value_type: str, value: Any)`

Modify the value of a value type

**Remarks:** Modify the value of a value type

**Parameters:**
- `value_type` (str) — Value type
- `value` (Any) — Value

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.modify_value_type)

#### `print_values()`

Print the building element values

**Remarks:** Print the building element values

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.print_values)

#### `reset_page(name: str, text: str, visible_condition: str, enable_condition: str)`

Reset pages for property dialog

**Remarks:** Reset pages for property dialog

**Parameters:**
- `name` (str) — the name of the page.
- `text` (str) — text of the page
- `visible_condition` (str) — visible condition
- `enable_condition` (str) — enable condition

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.reset_page)

#### `reset_property_modified()`

Reset the modified state of the properties

**Remarks:** Reset the modified state of the properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.reset_property_modified)

#### `set_insert_matrix(matrix: Matrix3D)`

Sets matrix for insertion

**Remarks:** Sets matrix for insertion

**Parameters:**
- `matrix` (Matrix3D) — insertion matrix

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.set_insert_matrix)

#### `set_property(name: str, value: Any)`

Set property value by name

**Remarks:** Set property value by name

**Parameters:**
- `name` (str) — the name of the property.
- `value` (Any) — new value of the property.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.set_property)

#### `set_reinforcement_definition_list(reinf_list: list[ReinforcementDefinition])`

Set reinforcement list

**Remarks:** Set reinforcement list

**Parameters:**
- `reinf_list` (list[ReinforcementDefinition]) — list of reinforcement.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElement/#BuildingElement.BuildingElement.set_reinforcement_definition_list)

### Properties
- `data_column_width` (int, get/set) — Get data column width
- `element_id` (str, get/set) — Get the element id
- `geometry_expand` (bool, get/set) — Get geometry expand state
- `is_interactor` (bool, get/set) — Get the interactor state
- `node_index` (int, get/set) — Get the node_index for VS
- `pyp_file_name` (str, get/set) — Get corresponding pyp file name
- `pyp_file_path` (str, get/set) — Get corresponding pyp file path
- `read_last_input` (bool, get/set) — Get read last input state
- `script_name` (str, get/set) — Get corresponding script name
- `script_uuid` (str, get/set) — Get the UUID of the pyp file
- `show_favorite_buttons` (bool, get/set) — Get the show_favorite_button state
- `skip_eval` (bool, get/set) — Get corresponding skip_eval
- `title` (str, get/set) — Get the property palette title
- `version` (str, get/set) — Get the version number as string
- `vs_multi_placement` (bool, get/set) — Get the VS multi placement state
- `vs_placement_point_input` (bool, get/set) — Get the VS placement point input state

