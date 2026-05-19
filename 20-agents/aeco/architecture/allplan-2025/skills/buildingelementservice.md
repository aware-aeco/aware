---
name: allplan-buildingelementservice
description: This skill encodes the allplan 2025.0 surface of the BuildingElementService namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BuildingElementService.
---

# BuildingElementService

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## BuildingElementService (class)

Definition of class BuildingElementService

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementService/)

### Methods
#### `read_build_ele_from_pyp( file_name: str, ) -> tuple[bool, ModuleType | None, BuildingElement | None]`

Create BuildingElement and python module based on .pyp file This method gets only the BuildingElement and the python module of the script, the .pyp file refers to. The resulting BuildingElement contains parameters with their default values. Use this method in the script of a main element (e.g. PythonPartGroup) to get the BuildingElement of the subordinate elements (e.g. PythonParts in the group).

**Remarks:** Create BuildingElement and python module based on .pyp file This method gets only the BuildingElement and the python module of the script, the .pyp file refers to. The resulting BuildingElement contains parameters with their default values. Use this method in the script of a main element (e.g. PythonPartGroup) to get the BuildingElement of the subordinate elements (e.g. PythonParts in the group).

**Parameters:**
- `file_name` (str) — full path to the .pyp file

**Returns:** `bool` — True, if import was successful, False otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementService/#BuildingElementService.BuildingElementService.read_build_ele_from_pyp)

#### `read_data_from_pyp( file_name: str, str_table: BuildingElementStringTable, is_library_preview: bool, material_str_table: BuildingElementMaterialStringTable, sub_file_name: str = "", init_with_last_input: bool = True, add_sub_file_name: str = "", ) -> Tuple[ Optional[bool], Optional[ModuleType], List[BuildingElement], List[BuildingElementControlProperties], BuildingElementComposite, str, str, ]`

Read the data from the pyp file and check the version

**Remarks:** Read the data from the pyp file and check the version

**Parameters:**
- `file_name` (str) — Name of the pyp file
- `str_table` (BuildingElementStringTable) — String table
- `is_library_preview` (bool) — Called for library preview
- `material_str_table` (BuildingElementMaterialStringTable) — Material string table
- `sub_file_name` (str) — File with the sub elements
- `init_with_last_input` (bool) — Initialize with the data from the last input (only if ReadLastInput is set in the script)
- `add_sub_file_name` (str) — additional sub file

**Returns:** `Optional[bool]` — True, if import was successful, False otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementService/#BuildingElementService.BuildingElementService.read_data_from_pyp)

#### `write_data_to_default_favorite_file(build_ele_list: List[BuildingElement])`

Write the data to the default favorite file

**Remarks:** Write the data to the default favorite file

**Parameters:**
- `build_ele_list` (List[BuildingElement]) — Building element list

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementService/#BuildingElementService.BuildingElementService.write_data_to_default_favorite_file)

