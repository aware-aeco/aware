---
name: allplan-buildingelementservice
description: This skill encodes the allplan 2024.0 surface of the BuildingElementService namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BuildingElementService.
---

# BuildingElementService

Auto-generated from vendor docs for allplan 2024.0. 1 types in this namespace.

## BuildingElementService (class)

Definition of class BuildingElementService

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementService/BuildingElementService/)

### Methods
#### `read_data_from_pyp(file_name, str_table, is_library_preview, material_str_table, sub_file_name='', init_with_last_input=True, add_sub_file_name='')`

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

**Returns:** `Tuple[Optional[bool], Optional[Any], List[BuildingElement], List[BuildingElementControlProperties], BuildingElementComposite, str, str]` — tuple with the data read from the pyp file

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementService/BuildingElementService/#BuildingElementService.BuildingElementService.read_data_from_pyp)

#### `write_data_to_default_favorite_file(build_ele_list)`

Write the data to the default favorite file

**Remarks:** Write the data to the default favorite file

**Parameters:**
- `build_ele_list` (List[BuildingElement]) — Building element list

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementService/BuildingElementService/#BuildingElementService.BuildingElementService.write_data_to_default_favorite_file)

