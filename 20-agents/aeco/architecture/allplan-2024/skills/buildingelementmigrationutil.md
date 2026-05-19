---
name: allplan-buildingelementmigrationutil
description: This skill encodes the allplan 2024.0 surface of the BuildingElementMigrationUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BuildingElementMigrationUtil.
---

# BuildingElementMigrationUtil

Auto-generated from vendor docs for allplan 2024.0. 1 types in this namespace.

## BuildingElementMigrationUtil (class)

Implementation of the migration utilities for the building element

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementMigrationUtil/BuildingElementMigrationUtil/)

### Methods
#### `transfer_parameter_value(parameter_list, old_element_id, old_value_name, new_element_id, new_value_name, converter_function=None, append_parameter=False)`

transfer a parameter value for an old ID and name to a new ID and name Args: parameter_list : list with the parameter names and values old_element_id : old element ID of the PythonPart old_value_name : old value name new_element_id : new element ID of the PythonPart new_value_name : new value name converter_function: convert function for the value as converter_function(value: str) -> str append_parameter : append the parameter (False replaces the old parameter)

**Remarks:** transfer a parameter value for an old ID and name to a new ID and name Args: parameter_list : list with the parameter names and values old_element_id : old element ID of the PythonPart old_value_name : old value name new_element_id : new element ID of the PythonPart new_value_name : new value name converter_function: convert function for the value as converter_function(value: str) -> str append_parameter : append the parameter (False replaces the old parameter)

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementMigrationUtil/BuildingElementMigrationUtil/#BuildingElementMigrationUtil.BuildingElementMigrationUtil.transfer_parameter_value)

