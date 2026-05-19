---
name: allplan-buildingelementmaterialstringtable
description: This skill encodes the allplan 2024.0 surface of the BuildingElementMaterialStringTable namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BuildingElementMaterialStringTable.
---

# BuildingElementMaterialStringTable

Auto-generated from vendor docs for allplan 2024.0. 1 types in this namespace.

## BuildingElementMaterialStringTable (class)

Definition of class MaterialStringTable

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementMaterialStringTable/BuildingElementMaterialStringTable/)

### Constructors
- `BuildingElementMaterialStringTable(path, numeric, lang)` — Initialization of class StringTable

### Methods
#### `add_entry(val_number, val_str)`

Creates an string table entry

**Remarks:** Creates an string table entry

**Parameters:**
- `val_number` (str) — index number in the list
- `val_str` (str) — string value

**Returns:** `bool` — True for success

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementMaterialStringTable/BuildingElementMaterialStringTable/#BuildingElementMaterialStringTable.BuildingElementMaterialStringTable.add_entry)

#### `get_entry(str_val_number)`

Returns an string table entry

**Remarks:** Returns an string table entry

**Parameters:**
- `str_val_number` (str) — value or number of the string

**Returns:** `Tuple[str, bool]` — (string result, True/False for success)

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementMaterialStringTable/BuildingElementMaterialStringTable/#BuildingElementMaterialStringTable.BuildingElementMaterialStringTable.get_entry)

