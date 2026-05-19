---
name: dynamo-dynamo-linting
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.Linting namespace — 2 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: LinterExtensionDescriptor, LinterManager.
---

# Dynamo.Linting

Auto-generated from vendor docs for dynamo 4.1.1. 2 types in this namespace.

## LinterExtensionDescriptor (class)

Type LinterExtensionDescriptor

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Linting/LinterExtensionDescriptor.cs)

### Constructors
- `void LinterExtensionDescriptor(string id, string name)` — LinterExtensionDescriptor.LinterExtensionDescriptor

### Methods
#### `bool Equals(object obj)`

Checks whether two LinterExtensionDescriptor are equal They are equal if their Name and Id are equal

**Parameters:**
- `obj` (object) — 

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Linting/LinterExtensionDescriptor.cs)

#### `int GetHashCode()`

Gets the hashcode for this LinterExtensionDescriptor

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Linting/LinterExtensionDescriptor.cs)

### Properties
- `Id` (string, get) — Id of linter extension
- `Name` (string, get) — LinterExtensionDescriptor.Name

## LinterManager (class)

This class handles registration of Linter extensions and linter rule evaluation management.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Linting/LinterManager.cs)

### Constructors
- `void LinterManager(Dynamo.Extensions.IExtensionManager extensionManager)` — LinterManager.LinterManager

### Methods
#### `void Dispose()`

LinterManager.Dispose

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Linting/LinterManager.cs)

#### `void SetActiveLinter(Dynamo.Linting.LinterExtensionDescriptor value, bool fullActivation)`

The LinterDescriptor setter that can be fully or partially activated

**Parameters:**
- `value` (Dynamo.Linting.LinterExtensionDescriptor)
- `fullActivation` (bool)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Linting/LinterManager.cs)

### Properties
- `ActiveLinter` (Dynamo.Linting.LinterExtensionDescriptor, get) — The LinterDescriptor that is currently set as active
- `AvailableLinters` (System.Collections.Generic.HashSet<Dynamo.Linting.LinterExtensionDescriptor>, get) — Available linters

