---
name: dynamo-dynamo-packagemanager-interfaces
description: This skill encodes the dynamo 4.1.0 surface of the Dynamo.PackageManager.Interfaces namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: IDirectoryInfo, IFileCompressor, IFileInfo.
---

# Dynamo.PackageManager.Interfaces

Auto-generated from vendor docs for dynamo 4.1.0. 3 types in this namespace.

## IDirectoryInfo (interface)

Type IDirectoryInfo

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoPackages/Interfaces/IDirectoryInfo.cs)

### Properties
- `FullName` (string, get) — IDirectoryInfo.FullName

## IFileCompressor (interface)

An abstract IFileCompressor for mocking purposes

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoPackages/Interfaces/IFileCompressor.cs)

### Methods
#### `Dynamo.PackageManager.Interfaces.IFileInfo Zip(Dynamo.PackageManager.Interfaces.IDirectoryInfo directory)`

IFileCompressor.Zip

**Parameters:**
- `directory` (Dynamo.PackageManager.Interfaces.IDirectoryInfo)

**Returns:** `Dynamo.PackageManager.Interfaces.IFileInfo` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoPackages/Interfaces/IFileCompressor.cs)

## IFileInfo (interface)

Type IFileInfo

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoPackages/Interfaces/IFileInfo.cs)

### Properties
- `Length` (long, get) — IFileInfo.Length
- `Name` (string, get) — IFileInfo.Name

