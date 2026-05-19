---
name: allplan-buildingelementstringtable
description: This skill encodes the allplan 2025.0 surface of the BuildingElementStringTable namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BuildingElementStringTable.
---

# BuildingElementStringTable

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## BuildingElementStringTable (class)

Definition of class StringTable

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementStringTable/)

### Constructors
- `BuildingElementStringTable(path: str, numeric: bool, lang: str, force_lang_name: bool = False)` — Initialization of class StringTable

### Methods
#### `add_entry(val_number: str, val_str: str) -> bool`

Creates an string table entry

**Remarks:** Creates an string table entry

**Parameters:**
- `val_number` (str) — index number in the list
- `val_str` (str) — string value

**Returns:** `bool` — True for success

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementStringTable/#BuildingElementStringTable.BuildingElementStringTable.add_entry)

#### `get_entry(str_val_number: Union[str, int]) -> Tuple[str, bool]`

Returns a string table entry

**Remarks:** Returns a string table entry

**Parameters:**
- `str_val_number` (Union[str, int]) — value or number of the string

**Returns:** `Tuple[str, bool]` — (string entry, True/False for success)

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementStringTable/#BuildingElementStringTable.BuildingElementStringTable.get_entry)

#### `get_key(entry: str) -> Optional[str]`

Returns the key for a string table entry

**Remarks:** Returns the key for a string table entry

**Parameters:**
- `entry` (str) — entry to search corresponding key

**Returns:** `Optional[str]` — Key, None if not found

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementStringTable/#BuildingElementStringTable.BuildingElementStringTable.get_key)

#### `get_string(string_id: str, default_string: str) -> str`

Get a string from the table

**Remarks:** Get a string from the table

**Parameters:**
- `string_id` (str) — String ID (if empty, take it from {...} part of the string
- `default_string` (str) — Default string in case of no existing ID

**Returns:** `str` — String from the ID if exist, default string otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementStringTable/#BuildingElementStringTable.BuildingElementStringTable.get_string)

