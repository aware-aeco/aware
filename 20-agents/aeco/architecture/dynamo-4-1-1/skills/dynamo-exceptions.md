---
name: dynamo-dynamo-exceptions
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.Exceptions namespace — 2 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: CustomNodePackageLoadException, LibraryLoadFailedException.
---

# Dynamo.Exceptions

Auto-generated from vendor docs for dynamo 4.1.1. 2 types in this namespace.

## CustomNodePackageLoadException (class)

Type CustomNodePackageLoadException

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Exceptions/CustomNodePackageLoadException.cs)

### Constructors
- `void CustomNodePackageLoadException(string path, string installedPath, string reason)` — Constructor.

### Properties
- `InstalledPath` (string, get) — File path of existing custom node package.

## LibraryLoadFailedException (class)

Customized exception thrown when library load failed.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Exceptions/LibraryLoadFailedException.cs)

### Constructors
- `void LibraryLoadFailedException(string path, string reason)` — Constructor.

