---
name: dynamo-protocore-codemodel
description: This skill encodes the dynamo 4.1.1 surface of the ProtoCore.CodeModel namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: CodeFile, CodePoint, CodeRange.
---

# ProtoCore.CodeModel

Auto-generated from vendor docs for dynamo 4.1.1. 3 types in this namespace.

## CodeFile (class)

Type CodeFile

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/CodeFile.cs)

### Constructors
- `void CodeFile()` — CodeFile.CodeFile

### Methods
#### `bool op_Equality(ProtoCore.CodeModel.CodeFile lhs, ProtoCore.CodeModel.CodeFile rhs)`

CodeFile.op_Equality

**Parameters:**
- `lhs` (ProtoCore.CodeModel.CodeFile)
- `rhs` (ProtoCore.CodeModel.CodeFile)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/CodeFile.cs)

#### `bool op_Inequality(ProtoCore.CodeModel.CodeFile lhs, ProtoCore.CodeModel.CodeFile rhs)`

CodeFile.op_Inequality

**Parameters:**
- `lhs` (ProtoCore.CodeModel.CodeFile)
- `rhs` (ProtoCore.CodeModel.CodeFile)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/CodeFile.cs)

### Properties
- `FilePath` (string, get/set) — CodeFile.FilePath

## CodePoint (struct)

Type CodePoint

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/CodePoint.cs)

### Methods
#### `int GetHashCode()`

CodePoint.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/CodePoint.cs)

#### `bool op_Equality(ProtoCore.CodeModel.CodePoint lhs, ProtoCore.CodeModel.CodePoint rhs)`

CodePoint.op_Equality

**Parameters:**
- `lhs` (ProtoCore.CodeModel.CodePoint)
- `rhs` (ProtoCore.CodeModel.CodePoint)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/CodePoint.cs)

#### `bool op_Inequality(ProtoCore.CodeModel.CodePoint lhs, ProtoCore.CodeModel.CodePoint rhs)`

CodePoint.op_Inequality

**Parameters:**
- `lhs` (ProtoCore.CodeModel.CodePoint)
- `rhs` (ProtoCore.CodeModel.CodePoint)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/CodePoint.cs)

### Properties
- `CharNo` (int, get/set) — CodePoint.CharNo
- `LineNo` (int, get/set) — CodePoint.LineNo
- `SourceLocation` (ProtoCore.CodeModel.CodeFile, get/set) — CodePoint.SourceLocation

## CodeRange (struct)

Type CodeRange

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/CodePoint.cs)

### Methods
#### `int GetHashCode()`

CodeRange.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/CodePoint.cs)

#### `bool op_Equality(ProtoCore.CodeModel.CodeRange lhs, ProtoCore.CodeModel.CodeRange rhs)`

CodeRange.op_Equality

**Parameters:**
- `lhs` (ProtoCore.CodeModel.CodeRange)
- `rhs` (ProtoCore.CodeModel.CodeRange)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/CodePoint.cs)

#### `bool op_Inequality(ProtoCore.CodeModel.CodeRange lhs, ProtoCore.CodeModel.CodeRange rhs)`

CodeRange.op_Inequality

**Parameters:**
- `lhs` (ProtoCore.CodeModel.CodeRange)
- `rhs` (ProtoCore.CodeModel.CodeRange)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/CodePoint.cs)

### Properties
- `EndExclusive` (ProtoCore.CodeModel.CodePoint, get/set) — CodeRange.EndExclusive
- `StartInclusive` (ProtoCore.CodeModel.CodePoint, get/set) — CodeRange.StartInclusive

