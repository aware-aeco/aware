---
name: allplan-typecollections-modificationelementlist
description: This skill encodes the allplan 2025.0 surface of the TypeCollections.ModificationElementList namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ModificationElementList.
---

# TypeCollections.ModificationElementList

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## ModificationElementList (class)

implementation of the list with the data of the modification elements

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/TypeCollections/ModificationElementList/)

### Constructors
- `ModificationElementList(elements: list[str | BaseElementAdapter] | None = None)` — initialize

### Methods
#### `get_base_element_adapter(doc: DocumentAdapter) -> BaseElementAdapter`

get the BaseElementAdapter for the index

**Remarks:** get the BaseElementAdapter for the index

**Parameters:**
- `doc` (DocumentAdapter) — document of the Allplan drawing files

**Returns:** `BaseElementAdapter` — BaseElementAdapter

[Docs](https://pythonparts.allplan.com/2025/api_reference/TypeCollections/ModificationElementList/#TypeCollections.ModificationElementList.ModificationElementList.get_base_element_adapter)

#### `is_modification_element() -> bool`

check for an existing modification element

**Remarks:** check for an existing modification element

**Returns:** `bool` — modification element state

[Docs](https://pythonparts.allplan.com/2025/api_reference/TypeCollections/ModificationElementList/#TypeCollections.ModificationElementList.ModificationElementList.is_modification_element)

