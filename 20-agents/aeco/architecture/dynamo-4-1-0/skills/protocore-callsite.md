---
name: dynamo-protocore-callsite
description: This skill encodes the dynamo 4.1.0 surface of the ProtoCore.CallSite namespace — 2 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: RawTraceData, SingleRunTraceData.
---

# ProtoCore.CallSite

Auto-generated from vendor docs for dynamo 4.1.0. 2 types in this namespace.

## RawTraceData (class)

Type RawTraceData

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/CallSite.cs)

### Constructors
- `void RawTraceData(string callSiteID, string data)` — RawTraceData.RawTraceData

### Properties
- `Data` (string, get) — RawTraceData.Data
- `ID` (string, get) — RawTraceData.ID

## SingleRunTraceData (class)

Type SingleRunTraceData

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/CallSite.cs)

### Methods
#### `bool Contains(string data)`

SingleRunTraceData.Contains

**Parameters:**
- `data` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/CallSite.cs)

#### `string GetLeftMostData()`

This gets the zero-most, left most index null if no data

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/CallSite.cs)

#### `System.Collections.Generic.List<string> RecursiveGetNestedData()`

SingleRunTraceData.RecursiveGetNestedData

**Returns:** `System.Collections.Generic.List<string>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Lang/CallSite.cs)

### Properties
- `HasAnyNestedData` (bool, get) — Is there any data anywhere in this run data, or is it all empty structure
- `HasData` (bool, get) — SingleRunTraceData.HasData
- `HasNestedData` (bool, get) — SingleRunTraceData.HasNestedData
- `IsEmpty` (bool, get) — Does this struct contain any trace data

