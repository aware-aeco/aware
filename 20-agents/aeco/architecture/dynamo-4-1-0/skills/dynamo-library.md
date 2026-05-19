---
name: dynamo-dynamo-library
description: This skill encodes the dynamo 4.1.0 surface of the Dynamo.Library namespace — 2 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ILibraryLoader, TypedParameter.
---

# Dynamo.Library

Auto-generated from vendor docs for dynamo 4.1.0. 2 types in this namespace.

## ILibraryLoader (interface)

Exposes (use cref) LoadNodeLibrary method. Specify the usage of LoadNodeLibrary

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/ILibraryLoader.cs)

### Methods
#### `void LoadNodeLibrary(System.Reflection.Assembly library)`

Loads node's library

**Parameters:**
- `library` (System.Reflection.Assembly)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/ILibraryLoader.cs)

## TypedParameter (class)

A tuple of parameter and its type.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/TypedParameter.cs)

### Constructors
- `void TypedParameter(string name, ProtoCore.Type type, ProtoCore.AST.AssociativeAST.AssociativeNode defaultValue, string shortArgumentName, string summary)` — This function creates TypedParameter
- `void TypedParameter(string name, ProtoCore.Type type, ProtoCore.AST.AssociativeAST.AssociativeNode defaultValue, string shortArgumentName, string summary, bool nameIsValid)` — This function creates TypedParameter
- `void TypedParameter(string name, string TypeName, int TypeRank, string defaultValue, string summary)` — This function creates TypedParameter

### Methods
#### `string ToString()`

Turns into string.

**Returns:** `string` — string

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/TypedParameter.cs)

### Properties
- `DefaultValue` (ProtoCore.AST.AssociativeAST.AssociativeNode, get) — Returns default value of the parameter as AST node.
- `DefaultValueString` (string, get) — Returns fully qualified string representation of AST node.
- `Description` (string, get) — Returns description of the parameter.
- `DisplayTypeName` (string, get) — Returns short type name of the parameter.
- `Function` (Dynamo.Engine.FunctionDescriptor, get) — Returns DesignScript function.
- `Name` (string, get) — Returns the name of the parameter.
- `NameIsValid` (bool, get) — Indicates whether a valid name has been set for this parameter. This property is not serialized.
- `Summary` (string, get) — Returns summary of the parameter. This may include comments or documentation.
- `Type` (ProtoCore.Type, get) — Returns type of the parameter.

