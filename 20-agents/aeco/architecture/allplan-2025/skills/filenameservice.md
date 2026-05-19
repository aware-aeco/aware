---
name: allplan-filenameservice
description: This skill encodes the allplan 2025.0 surface of the FileNameService namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: FileNameService.
---

# FileNameService

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## FileNameService (class)

Definition of service class FileNameService

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/FileNameService/)

### Methods
#### `get_filename_without_root_path(str_file_name: str, str_path: str) -> str`

Cuts the root_path of the filename

**Remarks:** Cuts the root_path of the filename

**Parameters:**
- `str_file_name` (str) — Full file path
- `str_path` (str) — Root directory path

**Returns:** `str` — File path relative to the root directory

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/FileNameService/#FileNameService.FileNameService.get_filename_without_root_path)

#### `get_global_standard_path(local_path: str) -> str`

Get the absolute path from a path relative to etc/std/prj directory

**Remarks:** Get the absolute path from a path relative to etc/std/prj directory

**Parameters:**
- `local_path` (str) — local path

**Returns:** `str` — global path (empty if not exist)

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/FileNameService/#FileNameService.FileNameService.get_global_standard_path)

#### `get_lib_pyp_sricpt_path(file_name: str) -> str`

Searches the file_name in the parts (Cur_proj, Std, Etc )of the library Cuts the library_path of the filename

**Remarks:** Searches the file_name in the parts (Cur_proj, Std, Etc )of the library Cuts the library_path of the filename

**Parameters:**
- `file_name` (str) — name of file.

**Returns:** `str` — str_file_name without lib_path

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/FileNameService/#FileNameService.FileNameService.get_lib_pyp_sricpt_path)

#### `get_pyp_path_from_lib_struct(file_name: str) -> tuple[bool, str]`

Searches the file_name in the parts (Cur_proj, Std, Etc ) of the library.

**Remarks:** Searches the file_name in the parts (Cur_proj, Std, Etc ) of the library.

**Parameters:**
- `file_name` (str) — name of file without lib_path

**Returns:** `bool` — True when file exist state, False otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/FileNameService/#FileNameService.FileNameService.get_pyp_path_from_lib_struct)

#### `search_pyp_file(file_name: str) -> tuple[bool, str]`

Searches the pyp file_name in all library folders in the parts (Cur_proj, Std, Etc )

**Remarks:** Searches the pyp file_name in all library folders in the parts (Cur_proj, Std, Etc )

**Parameters:**
- `file_name` (str) — name of file without lib_path

**Returns:** `bool` — True when file exist state, False otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/FileNameService/#FileNameService.FileNameService.search_pyp_file)

#### `set_node_script_default_path(node_script_default_path: str)`

Set the default path of the node scripts

**Remarks:** Set the default path of the node scripts

**Parameters:**
- `node_script_default_path` (str) — description

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/FileNameService/#FileNameService.FileNameService.set_node_script_default_path)

#### `update_global_standard_path(global_path: str) -> str`

update the global standard path for a new location

**Remarks:** update the global standard path for a new location

**Parameters:**
- `global_path` (str) — global path

**Returns:** `str` — updated global path (empty if not exists)

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/FileNameService/#FileNameService.FileNameService.update_global_standard_path)

