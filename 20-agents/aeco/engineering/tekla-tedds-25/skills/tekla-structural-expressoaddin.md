---
name: tedds-tekla-structural-expressoaddin
description: This skill encodes the tedds 25.0 surface of the Tekla.Structural.ExpressoAddIn namespace — 7 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AliasAttribute, FieldAttribute, FieldOutput, Requirement, RequirementAttribute, UnitsAttribute, ValidTypesAttribute.
---

# Tekla.Structural.ExpressoAddIn

Auto-generated from vendor docs for tedds 25.0. 7 types in this namespace.

## AliasAttribute (class)

(No description provided in vendor docs for Tekla.Structural.ExpressoAddIn.AliasAttribute.)

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-expressoaddin-aliasattribute-41824)

### Constructors
- `public AliasAttribute(string alias)` — 
- `public AliasAttribute(string alias, bool aliasOnly)` — 

## FieldAttribute (class)

(No description provided in vendor docs for Tekla.Structural.ExpressoAddIn.FieldAttribute.)

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-expressoaddin-fieldattribute-41825)

### Constructors
- `public FieldAttribute(FieldOutput output = FieldOutput.PlainText, bool generateFunction = false)` — 

### Properties
- `Function` (bool, get) — 
- `Output` (FieldOutput, get) — 

## FieldOutput (enum)

(No description provided in vendor docs for Tekla.Structural.ExpressoAddIn.FieldOutput.)

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-expressoaddin-fieldoutput-41826)

### Values
- `AsciiTextFile` = `13`
- `EnhancedMetafile` = `30`
- `PlainText` = `11`
- `ResultText` = `16`
- `Rtf` = `15`

## Requirement (enum)

(No description provided in vendor docs for Tekla.Structural.ExpressoAddIn.Requirement.)

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-expressoaddin-requirement-41827)

### Values
- `Negative` = `0`
- `NonZero` = `2`
- `NonZeroNegative` = `1`
- `NonZeroPositive` = `3`
- `Positive` = `4`

## RequirementAttribute (class)

(No description provided in vendor docs for Tekla.Structural.ExpressoAddIn.RequirementAttribute.)

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-expressoaddin-requirementattribute-41828)

### Constructors
- `public RequirementAttribute(string errorMessage, params char[] invalidChars)` — 
- `public RequirementAttribute(string errorMessage, double min, double max)` — 
- `protected RequirementAttribute(string errorMessage, Func<string, bool> validator)` — 
- `public RequirementAttribute(string errorMessage, Requirement requirement)` — 

### Methods
#### `protected virtual bool Validate(IExpresso expresso, object value)`



**Parameters:**
- `expresso` (IExpresso)
- `value` (object)

**Returns:** `bool` — 

[Docs](https://developer.tekla.com/topic/en/20/45/fb9fb084615248460ff035486dedca71c56abffcbd9d773ea3326b2f74a970a6)

### Properties
- `ErrorMessage` (string, get) — 

## UnitsAttribute (class)

(No description provided in vendor docs for Tekla.Structural.ExpressoAddIn.UnitsAttribute.)

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-expressoaddin-unitsattribute-41829)

### Constructors
- `public UnitsAttribute(double factor, string unit)` — 
- `public UnitsAttribute(string unit)` — 

### Methods
#### `protected virtual string GetUnit(IExpresso expresso)`



**Parameters:**
- `expresso` (IExpresso)

**Returns:** `string` — 

[Docs](https://developer.tekla.com/topic/en/20/45/0fe78571d3d1c2037c8c0c1871c80a6d49c5c15e63b6c9551b041292acade688)

## ValidTypesAttribute (class)

(No description provided in vendor docs for Tekla.Structural.ExpressoAddIn.ValidTypesAttribute.)

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-expressoaddin-validtypesattribute-41830)

### Constructors
- `public ValidTypesAttribute(params Type[] types)` — 

