---
name: dynamo-dynamo-migration-migrationmanager
description: This skill encodes the dynamo 4.1.0 surface of the Dynamo.Migration.MigrationManager namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Decision, FutureFileCallback, ObsoleteFileCallback.
---

# Dynamo.Migration.MigrationManager

Auto-generated from vendor docs for dynamo 4.1.0. 3 types in this namespace.

## Decision (enum)

Type Decision

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Migration/Migration.cs)

### Values
- `Abort` = `0` — The migration should not proceed and the file open operation should be aborted. This can be used to indicate that a version of file that is no longer supported and no migration path is provided.
- `Migrate` = `1` — File migration should proceed to migrate the older file version to a newer one.
- `Retain` = `2` — The file version is up-to-date and the file can be used as-is without migration.

## FutureFileCallback (delegate)

Type FutureFileCallback

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Migration/Migration.cs)

### Constructors
- `void FutureFileCallback(object object, IntPtr method)` — FutureFileCallback.FutureFileCallback

### Methods
#### `System.IAsyncResult BeginInvoke(string fileName, System.Version fileVersion, System.Version currentVersion, System.AsyncCallback callback, object object)`

FutureFileCallback.BeginInvoke

**Parameters:**
- `fileName` (string)
- `fileVersion` (System.Version)
- `currentVersion` (System.Version)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Migration/Migration.cs)

#### `bool EndInvoke(System.IAsyncResult result)`

FutureFileCallback.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Migration/Migration.cs)

#### `bool Invoke(string fileName, System.Version fileVersion, System.Version currentVersion)`

FutureFileCallback.Invoke

**Parameters:**
- `fileName` (string)
- `fileVersion` (System.Version)
- `currentVersion` (System.Version)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Migration/Migration.cs)

## ObsoleteFileCallback (delegate)

Type ObsoleteFileCallback

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Migration/Migration.cs)

### Constructors
- `void ObsoleteFileCallback(object object, IntPtr method)` — ObsoleteFileCallback.ObsoleteFileCallback

### Methods
#### `System.IAsyncResult BeginInvoke(string fileName, System.Version fileVersion, System.Version currentVersion, System.AsyncCallback callback, object object)`

ObsoleteFileCallback.BeginInvoke

**Parameters:**
- `fileName` (string)
- `fileVersion` (System.Version)
- `currentVersion` (System.Version)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Migration/Migration.cs)

#### `void EndInvoke(System.IAsyncResult result)`

ObsoleteFileCallback.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Migration/Migration.cs)

#### `void Invoke(string fileName, System.Version fileVersion, System.Version currentVersion)`

ObsoleteFileCallback.Invoke

**Parameters:**
- `fileName` (string)
- `fileVersion` (System.Version)
- `currentVersion` (System.Version)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Migration/Migration.cs)

