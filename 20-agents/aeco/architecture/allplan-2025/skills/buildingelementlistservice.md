---
name: allplan-buildingelementlistservice
description: This skill encodes the allplan 2025.0 surface of the BuildingElementListService namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BuildingElementListService.
---

# BuildingElementListService

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## BuildingElementListService (class)

Implementation of functions for managing the data in a list of BuildingElements

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementListService/)

### Methods
#### `get_hash(build_ele_list: list[BuildingElement]) -> str`

Calculate a hash value for script name and parameter list

**Remarks:** Calculate a hash value for script name and parameter list

**Parameters:**
- `build_ele_list` (list[BuildingElement]) — list with the building elements

**Returns:** `str` — Calculated hash string.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementListService/#BuildingElementListService.BuildingElementListService.get_hash)

#### `get_params_list( build_ele_list: list[BuildingElement], persistence: Persistent = MODEL, exclude_parameter_names: list[str] | None = None, ) -> list[str]`

Get the parameter list of the building elements

**Remarks:** Get the parameter list of the building elements

**Parameters:**
- `build_ele_list` (list[BuildingElement]) — list with the building elements
- `persistence` (Persistent) — persistence to check
- `exclude_parameter_names` (list[str] | None) — excluded parameter names from the list

**Returns:** `list[str]` — Parameter list

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementListService/#BuildingElementListService.BuildingElementListService.get_params_list)

#### `migrate_fav_data( fav_param_list: list[str], build_ele_list: list[BuildingElement], script: Any, )`

Migrate the favorite data

**Remarks:** Migrate the favorite data

**Parameters:**
- `fav_param_list` (list[str]) — list with the parameter data
- `build_ele_list` (list[BuildingElement]) — list with the building elements
- `script` (Any) — script

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementListService/#BuildingElementListService.BuildingElementListService.migrate_fav_data)

#### `read_fav_data( fav_param_list: list[str], build_ele_list: list[BuildingElement], persistence: Persistent = MODEL, is_modification_mode: bool = True, script: Any = None, )`

Read the data from a favorite parameter list

**Remarks:** Read the data from a favorite parameter list

**Parameters:**
- `fav_param_list` (list[str]) — list with the favorite parameter
- `build_ele_list` (list[BuildingElement]) — list with the building elements
- `persistence` (Persistent) — Parameter persistency, for which the data should be read
- `is_modification_mode` (bool) — Read the data for the PythonPart modification, True/False
- `script` (Any) — script

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementListService/#BuildingElementListService.BuildingElementListService.read_fav_data)

#### `read_from_default_favorite_file(build_ele_list: list[BuildingElement])`

The parameter values are read from the default favorite file and assigned to the parameters in build_ele_list. The file is located in the Allplan ".../usr/local/tmp" directory and the extension of the file is "pyv"

**Remarks:** The parameter values are read from the default favorite file and assigned to the parameters in build_ele_list. The file is located in the Allplan ".../usr/local/tmp" directory and the extension of the file is "pyv"

**Parameters:**
- `build_ele_list` (list[BuildingElement]) — list with the building elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementListService/#BuildingElementListService.BuildingElementListService.read_from_default_favorite_file)

#### `read_from_file(file_name: str, build_ele_list: list[BuildingElement])`

Read the properties from a file

**Remarks:** Read the properties from a file

**Parameters:**
- `file_name` (str) — Name of the file
- `build_ele_list` (list[BuildingElement]) — list with the building elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementListService/#BuildingElementListService.BuildingElementListService.read_from_file)

#### `reset_param_values(build_ele_list: list[BuildingElement])`

Reset to original parameter values from pyp file

**Remarks:** Reset to original parameter values from pyp file

**Parameters:**
- `build_ele_list` (list[BuildingElement]) — list with the building elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementListService/#BuildingElementListService.BuildingElementListService.reset_param_values)

#### `write_to_default_favorite_file(build_ele_list: list[BuildingElement])`

The parameter values are taken from build_ele_list and written to the default favorite file. The file is located in the Allplan ".../usr/local/tmp" directory and the extension of the file is "pyv"

**Remarks:** The parameter values are taken from build_ele_list and written to the default favorite file. The file is located in the Allplan ".../usr/local/tmp" directory and the extension of the file is "pyv"

**Parameters:**
- `build_ele_list` (list[BuildingElement]) — list with the building elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementListService/#BuildingElementListService.BuildingElementListService.write_to_default_favorite_file)

#### `write_to_file(file_name: str, build_ele_list: list[BuildingElement])`

Write the properties to a file

**Remarks:** Write the properties to a file

**Parameters:**
- `file_name` (str) — Name of the file
- `build_ele_list` (list[BuildingElement]) — list with the building elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementListService/#BuildingElementListService.BuildingElementListService.write_to_file)

