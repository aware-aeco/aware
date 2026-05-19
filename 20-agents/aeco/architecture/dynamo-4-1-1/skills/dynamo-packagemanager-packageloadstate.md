---
name: dynamo-dynamo-packagemanager-packageloadstate
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.PackageManager.PackageLoadState namespace — 2 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: StateTypes, ScheduledTypes.
---

# Dynamo.PackageManager.PackageLoadState

Auto-generated from vendor docs for dynamo 4.1.1. 2 types in this namespace.

## ScheduledTypes (enum)

Type ScheduledTypes

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Package.cs)

### Values
- `None` = `0` — Invalid scheduled state. The initial state for every package, before it is interpreted by Dynamo
- `ScheduledForDeletion` = `2` — The package is scheduled to be deleted. After the next Dynamo restart, the package will deleted from the package locations
- `ScheduledForUnload` = `1` — The package is scheduled to be unloaded. After the next Dynamo restart, the package will be in an Unloaded state

## StateTypes (enum)

Type StateTypes

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Package.cs)

### Values
- `Error` = `3` — The package was not loaded in Dynamo because of an error. See the Error property for more information
- `Loaded` = `1` — The package is fully loaded and is ready to be used
- `None` = `0` — Invalid state. The initial state for every package, before it is interpreted by Dynamo
- `Unloaded` = `2` — The package is not loaded in Dynamo and not deleted from package locations

