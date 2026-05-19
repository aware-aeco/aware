---
name: allplan-pythonparttransaction
description: This skill encodes the allplan 2024.0 surface of the PythonPartTransaction namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ConnectToPythonPart, PythonPartTransaction, ConnectToPythonPartState.
---

# PythonPartTransaction

Auto-generated from vendor docs for allplan 2024.0. 3 types in this namespace.

## ConnectToPythonPart (class)

data class for the PythonPart connection

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPartTransaction/ConnectToPythonPart/)

## ConnectToPythonPartState (enum)

definition of the states for the PythonPart connection

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPartTransaction/ConnectToPythonPartState/)

### Values
- `IS_CHILD` = `2`
- `IS_PARENT` = `1`
- `IS_PARENT_CHILD` = `3`

## PythonPartTransaction (class)

Implementation of the PythonPart transaction

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPartTransaction/PythonPartTransaction/)

### Constructors
- `PythonPartTransaction(doc, connect_to_pyp=ConnectToPythonPart())` — Initialize

### Methods
#### `create_connection(connect_ele, pyp_uuid)`

create the PythonPart connection

**Remarks:** create the PythonPart connection

**Parameters:**
- `connect_ele` (BaseElementAdapter) — connection element
- `pyp_uuid` (GUID) — UUID of the PythonPart

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPartTransaction/PythonPartTransaction/#PythonPartTransaction.PythonPartTransaction.create_connection)

#### `execute(placement_matrix, view_world_projection, model_ele_list, modification_ele_list, rearrange_reinf_pos_nr=ReinforcementRearrange(), append_reinf_pos_nr=True, asso_ref_object=None, uuid_parameter_name='', _elementation_delete_py=True)`

execute the transaction

**Remarks:** execute the transaction

**Parameters:**
- `placement_matrix` (Matrix3D) — placement matrix
- `view_world_projection` (ViewWorldProjection) — view world projection
- `model_ele_list` (list[Any]) — list with the model elements
- `modification_ele_list` (ModificationElementList) — list with the UUID's of the modified PythonPart elements
- `rearrange_reinf_pos_nr` (ReinforcementRearrange) — data for the reinforcement rearrange
- `append_reinf_pos_nr` (bool) — True: Append the reinforcement position numbers to the existing position numbers
- `asso_ref_object` (BaseElementAdapter | None) — associative view reference object
- `uuid_parameter_name` (str) — if set, the model object UUID of the created PythonPart is assigned to this name
- `_elementation_delete_py` (bool) — delete the PythonPart after the precast elementation

**Returns:** `BaseElementAdapterList` — list with the created elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/PythonPartTransaction/PythonPartTransaction/#PythonPartTransaction.PythonPartTransaction.execute)

