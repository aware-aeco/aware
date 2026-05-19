---
name: allplan-parameterproperty
description: This skill encodes the allplan 2024.0 surface of the ParameterProperty namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ParameterProperty.
---

# ParameterProperty

Auto-generated from vendor docs for allplan 2024.0. 1 types in this namespace.

## ParameterProperty (class)

Definition of class ParameterProperty

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ParameterProperty/ParameterProperty/)

### Constructors
- `ParameterProperty()` — initialize

### Methods
#### `__repr__()`

Print class information

**Remarks:** Print class information

**Returns:** `str` — parameter property as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ParameterProperty/ParameterProperty/#ParameterProperty.ParameterProperty.__repr__)

#### `deep_copy()`

deep copy of the parameter values

**Remarks:** deep copy of the parameter values

**Returns:** `ParameterProperty` — copied parameter property

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ParameterProperty/ParameterProperty/#ParameterProperty.ParameterProperty.deep_copy)

#### `reset_modified()`

Reset the is_modified state

**Remarks:** Reset the is_modified state

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ParameterProperty/ParameterProperty/#ParameterProperty.ParameterProperty.reset_modified)

#### `set_named_tuple_def(typename, field_names)`

Set the named tuple definition

**Remarks:** Set the named tuple definition

**Parameters:**
- `typename` (str) — description
- `field_names` (List[str]) — description

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/ParameterProperty/ParameterProperty/#ParameterProperty.ParameterProperty.set_named_tuple_def)

### Properties
- `attribute_id` (int, get/set) — Get the attribute id of the property
- `attribute_id_str` (str, get/set) — Get the attribute id string of the property
- `dimensions` (str, get/set) — Get the dimensions of the property in case of a list
- `exclude_identical` (bool, get/set) — Get the exclude identical state If True, the parameter is not used for checking identical PythonParts
- `group_name` (str, get/set) — Get the group name of the property
- `input_type` (InputType, get/set) — Getter for input type paramter
- `is_modified` (bool, get/set) — Get the modified state
- `list_reverse` (bool, get/set) — Get the list reverse state
- `list_squeeze` (bool, get/set) — Get the list squeeze state
- `list_state` (int, get/set) — Get the list as block state
- `name` (str, get/set) — Get the name of the property
- `named_tuple_def` (Optional[NamedTupleDef], get) — Get the named tuple definition
- `persistent` (Persistent, get/set) — Get the persistent state of the property
- `selected_value` (Any, get/set) — Get the selected value of the property
- `value` (Any, get/set) — Get the value of the property
- `value_type` (ParameterPropertyValueType, get/set) — Get the value type of the property

