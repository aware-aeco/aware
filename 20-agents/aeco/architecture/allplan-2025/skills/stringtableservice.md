---
name: allplan-stringtableservice
description: This skill encodes the allplan 2025.0 surface of the StringTableService namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: StringTableService.
---

# StringTableService

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## StringTableService (class)

Definition of class StringTableService

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/StringTableService/)

### Constructors
- `StringTableService(path: str)` — Initialize the data

### Methods
#### `get_string(string_id: str, string: str) -> str`

Get a string from the table

**Remarks:** Get a string from the table

**Parameters:**
- `string_id` (str) — String ID (if empty, take it for {...} part of the string
- `string` (str) — Default string in case of no existing ID

**Returns:** `str` — String from the ID if exist, default string otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/StringTableService/#StringTableService.StringTableService.get_string)

#### `get_string_table_entry( string_table: BuildingElementStringTable, string_id: str, string: str ) -> str`

Get a string from the table

**Remarks:** Get a string from the table

**Parameters:**
- `string_table` (BuildingElementStringTable) — string table
- `string_id` (str) — String ID (if empty, take it for {...} part of the string
- `string` (str) — Default string in case of no existing ID

**Returns:** `str` — String from the ID if exist, default string otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/StringTableService/#StringTableService.StringTableService.get_string_table_entry)

### Properties
- `material_str_table` (BuildingElementMaterialStringTable, get) — get the material string table
- `str_table` (BuildingElementStringTable, get) — get the string table

