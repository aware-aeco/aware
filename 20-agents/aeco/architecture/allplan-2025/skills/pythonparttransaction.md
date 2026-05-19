---
name: allplan-pythonparttransaction
description: This skill encodes the allplan 2025.0 surface of the PythonPartTransaction namespace — 6 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ConnectToElements, ConnectToPythonPart, Functions, ConnectToPythonPartState, ReinforcementRearrange, PythonPartTransaction.
---

# PythonPartTransaction

Auto-generated from vendor docs for allplan 2025.0. 6 types in this namespace.

## ConnectToElements (class)

Data class for the PythonPart connection to element(s)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartTransaction/ConnectToElements/)

## ConnectToPythonPart (class)

Data class for the PythonPart connection.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartTransaction/ConnectToPythonPart/)

## ConnectToPythonPartState (enum)

Enumeration of the possible types of relationships in a PythonPart connection.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartTransaction/ConnectToPythonPartState/)

### Values
- `IS_CHILD` = `2` — Connected PythonPart is the child of the other PythonPart.
- `IS_PARENT` = `1` — Connected PythonPart is the parent of the other PythonPart.
- `IS_PARENT_CHILD` = `3` — Creates a connection in both directions.

## Functions (static-class)

Module-level functions of PythonPartTransaction

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartTransaction/)

## PythonPartTransaction (class)

Implementation of the PythonPart transaction. The transaction creates the elements in the drawing file and introduces additional steps, that would normally have to be introduced separately, after the creation. This is to make the creation easier to implement in a PythonPart script. These additional steps are:

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartTransaction/PythonPartTransaction/)

### Constructors
- `PythonPartTransaction( doc: DocumentAdapter, connect_to_pyp: ConnectToPythonPart = ConnectToPythonPart(), connect_to_ele: ConnectToElements = ConnectToElements(), )` — Initialize

### Methods
#### `execute( placement_matrix: Matrix3D, view_world_projection: ViewWorldProjection, model_ele_list: list[Any], modification_ele_list: ModificationElementList, rearrange_reinf_pos_nr: ReinforcementRearrange = ReinforcementRearrange(), append_reinf_pos_nr: bool = True, asso_ref_object: BaseElementAdapter | None = None, uuid_parameter_name: str = "", _elementation_delete_py: bool = True, use_system_angle: bool = True, elements_to_delete: BaseElementAdapterList | None = None, ) -> BaseElementAdapterList`

Execute the transaction. Calling the method, makes the framework perform following operations:

**Remarks:** Execute the transaction. Calling the method, makes the framework perform following operations:

**Parameters:**
- `placement_matrix` (Matrix3D) — placement matrix
- `view_world_projection` (ViewWorldProjection) — view world projection
- `model_ele_list` (list[Any]) — list with the model elements
- `modification_ele_list` (ModificationElementList) — list with the UUID's of the modified PythonPart elements
- `rearrange_reinf_pos_nr` (ReinforcementRearrange) — data for the reinforcement rearrange
- `append_reinf_pos_nr` (bool) — When set to True, the reinforcement position numbers are appended to the existing position numbers
- `asso_ref_object` (BaseElementAdapter | None) — associative view reference object
- `uuid_parameter_name` (str) — if set, the model object UUID of the created PythonPart is assigned to this name
- `_elementation_delete_py` (bool) — delete the PythonPart after the precast elementation
- `use_system_angle` (bool) — use the default system angle state (False = no use of the system angle) In case of True, the state of defined elements in the modification_ele_list is used - empty list = creation mode: use the system angle - element in list = modification mode: don't use the system angle
- `elements_to_delete` (BaseElementAdapterList | None) — elements which should be delete inside the created transaction

**Returns:** `BaseElementAdapterList` — list with the created elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartTransaction/PythonPartTransaction/#PythonPartTransaction.PythonPartTransaction.execute)

## ReinforcementRearrange (class)

implementation of the data class for the reinforcement rearrange

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartTransaction/ReinforcementRearrange/)

