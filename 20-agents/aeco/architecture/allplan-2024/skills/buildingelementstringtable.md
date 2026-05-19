---
name: allplan-buildingelementstringtable
description: This skill encodes the allplan 2024.0 surface of the BuildingElementStringTable namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BuildingElementStringTable.
---

# BuildingElementStringTable

Auto-generated from vendor docs for allplan 2024.0. 1 types in this namespace.

## BuildingElementStringTable (class)

Definition of class StringTable

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementStringTable/BuildingElementStringTable/)

### Constructors
- `BuildingElementStringTable(path, numeric, lang)` — Initialization of class StringTable

### Methods
#### `__is_ascii_underline_numbers(input_str)`

check if a string contains only ascii signs underlines and numbers

**Remarks:** check if a string contains only ascii signs underlines and numbers

**Parameters:**
- `input_str` (str) — input string to check

**Returns:** `bool` — underline numbers: True/False

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementStringTable/BuildingElementStringTable/#BuildingElementStringTable.BuildingElementStringTable.__is_ascii_underline_numbers)

#### `add_entry(val_number, val_str)`

Creates an string table entry

**Remarks:** Creates an string table entry

**Parameters:**
- `val_number` (str) — index number in the list
- `val_str` (str) — string value

**Returns:** `bool` — True for success

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementStringTable/BuildingElementStringTable/#BuildingElementStringTable.BuildingElementStringTable.add_entry)

#### `get_entry(str_val_number)`

Returns a string table entry

**Remarks:** Returns a string table entry

**Parameters:**
- `str_val_number` (Union[str, int]) — value or number of the string

**Returns:** `Tuple[str, bool]` — (string entry, True/False for success)

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementStringTable/BuildingElementStringTable/#BuildingElementStringTable.BuildingElementStringTable.get_entry)

#### `get_key(entry)`

Returns the key for a string table entry

**Remarks:** Returns the key for a string table entry

**Parameters:**
- `entry` (str) — entry to search corresponding key

**Returns:** `Optional[str]` — Key, None if not found

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementStringTable/BuildingElementStringTable/#BuildingElementStringTable.BuildingElementStringTable.get_key)

#### `get_string(string_id, default_string)`

Get a string from the table

**Remarks:** Get a string from the table

**Parameters:**
- `string_id` (str) — String ID (if empty, take it from {...} part of the string
- `default_string` (str) — Default string in case of no existing ID

**Returns:** `str` — String from the ID if exist, default string otherwise

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementStringTable/BuildingElementStringTable/#BuildingElementStringTable.BuildingElementStringTable.get_string)

