---
name: allplan-buildingelementlistservice
description: This skill encodes the allplan 2024.0 surface of the BuildingElementListService namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BuildingElementListService.
---

# BuildingElementListService

Auto-generated from vendor docs for allplan 2024.0. 1 types in this namespace.

## BuildingElementListService (class)

Implementation of functions for managing the data in a list of BuildingElements

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementListService/BuildingElementListService/)

### Methods
#### `get_hash(build_ele_list)`

Calculate a hash value for script name and parameter list

**Remarks:** Calculate a hash value for script name and parameter list

**Parameters:**
- `build_ele_list` (List[BuildingElement]) — list with the building elements

**Returns:** `str` — Calculated hash string.

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementListService/BuildingElementListService/#BuildingElementListService.BuildingElementListService.get_hash)

#### `get_params_list(build_ele_list, persistence=MODEL, exclude_parameter_names=None)`

Get the parameter list of the building elements

**Remarks:** Get the parameter list of the building elements

**Parameters:**
- `build_ele_list` (List[BuildingElement]) — list with the building elements
- `persistence` (Persistent) — persistence to check
- `exclude_parameter_names` (Optional[List[str]]) — excluded parameter names from the list

**Returns:** `List[str]` — Parameter list

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementListService/BuildingElementListService/#BuildingElementListService.BuildingElementListService.get_params_list)

#### `migrate_fav_data(fav_param_list, build_ele_list, script)`

Migrate the favorite data

**Remarks:** Migrate the favorite data

**Parameters:**
- `fav_param_list` (List[str]) — List with the parameter data
- `build_ele_list` (List[BuildingElement]) — list with the building elements
- `script` (Any) — script

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementListService/BuildingElementListService/#BuildingElementListService.BuildingElementListService.migrate_fav_data)

#### `read_fav_data(fav_param_list, build_ele_list, persistence=MODEL, is_modification_mode=True, script=None)`

Read the data from a favorite parameter list

**Remarks:** Read the data from a favorite parameter list

**Parameters:**
- `fav_param_list` (List[str]) — List with the favorite parameter
- `build_ele_list` (List[BuildingElement]) — list with the building elements
- `persistence` (Persistent) — Parameter persistency, for which the data should be read
- `is_modification_mode` (bool) — Read the data for the PythonPart modification, True/False
- `script` (Any) — script

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementListService/BuildingElementListService/#BuildingElementListService.BuildingElementListService.read_fav_data)

#### `read_from_default_favorite_file(build_ele_list)`

The parameter values are read from the default favorite file and assigned to the parameters in build_ele_list. The file is located in the Allplan ".../usr/local/tmp" directory and the extension of the file is "pyv"

**Remarks:** The parameter values are read from the default favorite file and assigned to the parameters in build_ele_list. The file is located in the Allplan ".../usr/local/tmp" directory and the extension of the file is "pyv"

**Parameters:**
- `build_ele_list` (List[BuildingElement]) — list with the building elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementListService/BuildingElementListService/#BuildingElementListService.BuildingElementListService.read_from_default_favorite_file)

#### `read_from_file(file_name, build_ele_list)`

Read the properties from a file

**Remarks:** Read the properties from a file

**Parameters:**
- `file_name` (str) — Name of the file
- `build_ele_list` (List[BuildingElement]) — list with the building elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementListService/BuildingElementListService/#BuildingElementListService.BuildingElementListService.read_from_file)

#### `write_to_default_favorite_file(build_ele_list)`

The parameter values are taken from build_ele_list and written to the default favorite file. The file is located in the Allplan ".../usr/local/tmp" directory and the extension of the file is "pyv"

**Remarks:** The parameter values are taken from build_ele_list and written to the default favorite file. The file is located in the Allplan ".../usr/local/tmp" directory and the extension of the file is "pyv"

**Parameters:**
- `build_ele_list` (List[BuildingElement]) — list with the building elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementListService/BuildingElementListService/#BuildingElementListService.BuildingElementListService.write_to_default_favorite_file)

#### `write_to_file(file_name, build_ele_list)`

Write the properties to a file

**Remarks:** Write the properties to a file

**Parameters:**
- `file_name` (str) — Name of the file
- `build_ele_list` (List[BuildingElement]) — list with the building elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementListService/BuildingElementListService/#BuildingElementListService.BuildingElementListService.write_to_file)

