---
name: allplan-buildingelementmaterialstringtable
description: This skill encodes the allplan 2025.0 surface of the BuildingElementMaterialStringTable namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BuildingElementMaterialStringTable.
---

# BuildingElementMaterialStringTable

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## BuildingElementMaterialStringTable (class)

Definition of class MaterialStringTable

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementMaterialStringTable/)

### Constructors
- `BuildingElementMaterialStringTable(path: str, numeric: bool, lang: str)` — Initialization of class StringTable

### Methods
#### `add_entry(val_number: str, val_str: str) -> bool`

Creates an string table entry

**Remarks:** Creates an string table entry

**Parameters:**
- `val_number` (str) — index number in the list
- `val_str` (str) — string value

**Returns:** `bool` — True for success

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementMaterialStringTable/#BuildingElementMaterialStringTable.BuildingElementMaterialStringTable.add_entry)

#### `get_entry(str_val_number: str) -> Tuple[str, bool]`

Returns an string table entry

**Remarks:** Returns an string table entry

**Parameters:**
- `str_val_number` (str) — value or number of the string

**Returns:** `Tuple[str, bool]` — (string result, True/False for success)

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementMaterialStringTable/#BuildingElementMaterialStringTable.BuildingElementMaterialStringTable.get_entry)

