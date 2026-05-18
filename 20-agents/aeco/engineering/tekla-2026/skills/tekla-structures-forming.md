---
name: tekla-tekla-structures-forming
description: This skill encodes the tekla 2026.0 surface of the Tekla.Structures.Forming namespace — 4 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: FormingStates, WrappingType, DeformingType, FoldingType.
---

# Tekla.Structures.Forming

Auto-generated from vendor docs for tekla 2026.0. 4 types in this namespace.

## DeformingType (enum)

Enumeration for deforming type.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/568a687f-8617-ffc0-3fa4-e41a3704a231)

### Values
- `NOT_SPECIFIED` = `0` — Not specified, default value.
- `DEFORMED` = `1` — Indiactes deformed item.
- `UNDEFORMED` = `2` — Indicates undeformed item.

## FoldingType (enum)

Enumeration for folding type.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c886da23-040c-cf1b-842b-6154335c7852)

### Values
- `NOT_SPECIFIED` = `0` — Not specified, default value.
- `FOLDED` = `1` — Indicates folded item.
- `UNFOLDED` = `2` — Indicates unfolded item.

## FormingStates (class)

FormingStates class which contains different forming options.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/798a3ceb-2766-0a5c-7853-023059f04b55)

### Constructors
- `public FormingStates()` — Initializes a new instance of the FormingStates class.
- `public FormingStates(DeformingType deforming)` — Initializes a new instance of the FormingStates class.
- `public FormingStates(FoldingType folding)` — Initializes a new instance of the FormingStates class.
- `public FormingStates(WrappingType wrapping)` — Initializes a new instance of the FormingStates class.
- `public FormingStates(DeformingType deforming, FoldingType folding, WrappingType wrapping)` — Initializes a new instance of the FormingStates class.

### Methods
#### `public Object Clone()`

Method for cloning the current object.

**Returns:** `Object` — Cloned object.

[Docs](https://developer.tekla.com/topic/en/18/47/0798cc99-1eaa-efe0-401e-c789ae54bbf4)

### Properties
- `Deforming` (DeformingType, get/set) — Gets or sets the deforming type.
- `Folding` (FoldingType, get/set) — Gets or sets the folding type.
- `Wrapping` (WrappingType, get/set) — Gets or sets the wrapping type.

## WrappingType (enum)

Enumeration for wrapping type.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f473d02d-8108-78fb-d9ed-6c522b848120)

### Values
- `NOT_SPECIFIED` = `0` — Not specified, default value.
- `WRAPPED` = `1` — Indicates wrapped item.
- `UNWRAPPED` = `2` — Indicates unwrapped item.

