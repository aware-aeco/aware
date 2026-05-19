---
name: dynamo-dynamo-engine-libraryservices
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.Engine.LibraryServices namespace — 4 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Categories, LibraryLoadFailedEventArgs, LibraryLoadedEventArgs, LibraryLoadingEventArgs.
---

# Dynamo.Engine.LibraryServices

Auto-generated from vendor docs for dynamo 4.1.1. 4 types in this namespace.

## Categories (static-class)

Type Categories

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Library/LibraryServices.cs)

## LibraryLoadFailedEventArgs (class)

Type LibraryLoadFailedEventArgs

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Library/LibraryServices.cs)

### Constructors
- `void LibraryLoadFailedEventArgs(string libraryPath, string reason, bool throwOnFailure)` — LibraryLoadFailedEventArgs.LibraryLoadFailedEventArgs

### Properties
- `LibraryPath` (string, get) — The path to the library that failed to load
- `Reason` (string, get) — The reason that the library failed to load
- `ThrowOnFailure` (bool, get) — Indicates if the failure should result in an exception being thrown

## LibraryLoadedEventArgs (class)

Type LibraryLoadedEventArgs

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Library/LibraryServices.cs)

### Constructors
- `void LibraryLoadedEventArgs(System.Collections.Generic.IEnumerable<string> libraryPaths)` — LibraryLoadedEventArgs.LibraryLoadedEventArgs

### Properties
- `LibraryPaths` (System.Collections.Generic.IEnumerable<string>, get) — Paths to libraries that are loaded.

## LibraryLoadingEventArgs (class)

Type LibraryLoadingEventArgs

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Library/LibraryServices.cs)

### Constructors
- `void LibraryLoadingEventArgs(string libraryPath)` — LibraryLoadingEventArgs.LibraryLoadingEventArgs

### Properties
- `LibraryPath` (string, get) — LibraryLoadingEventArgs.LibraryPath

