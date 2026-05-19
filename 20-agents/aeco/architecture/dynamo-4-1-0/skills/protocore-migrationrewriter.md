---
name: dynamo-protocore-migrationrewriter
description: This skill encodes the dynamo 4.1.0 surface of the ProtoCore.MigrationRewriter namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: LogWarningHandler.
---

# ProtoCore.MigrationRewriter

Auto-generated from vendor docs for dynamo 4.1.0. 1 types in this namespace.

## LogWarningHandler (delegate)

Type LogWarningHandler

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/MigrationRewriter.cs)

### Constructors
- `void LogWarningHandler(object object, IntPtr method)` — LogWarningHandler.LogWarningHandler

### Methods
#### `System.IAsyncResult BeginInvoke(string oldNodeName, string newNodeName, System.AsyncCallback callback, object object)`

LogWarningHandler.BeginInvoke

**Parameters:**
- `oldNodeName` (string)
- `newNodeName` (string)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/MigrationRewriter.cs)

#### `void EndInvoke(System.IAsyncResult result)`

LogWarningHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/MigrationRewriter.cs)

#### `void Invoke(string oldNodeName, string newNodeName)`

LogWarningHandler.Invoke

**Parameters:**
- `oldNodeName` (string)
- `newNodeName` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/MigrationRewriter.cs)

