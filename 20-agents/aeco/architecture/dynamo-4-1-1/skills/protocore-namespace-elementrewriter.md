---
name: dynamo-protocore-namespace-elementrewriter
description: This skill encodes the dynamo 4.1.1 surface of the ProtoCore.Namespace.ElementRewriter namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: SymbolConflictWarningHandler.
---

# ProtoCore.Namespace.ElementRewriter

Auto-generated from vendor docs for dynamo 4.1.1. 1 types in this namespace.

## SymbolConflictWarningHandler (delegate)

Type SymbolConflictWarningHandler

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/ElementRewriter.cs)

### Constructors
- `void SymbolConflictWarningHandler(object object, IntPtr method)` — SymbolConflictWarningHandler.SymbolConflictWarningHandler

### Methods
#### `System.IAsyncResult BeginInvoke(string symbolName, string[] collidingSymbolNames, System.AsyncCallback callback, object object)`

SymbolConflictWarningHandler.BeginInvoke

**Parameters:**
- `symbolName` (string)
- `collidingSymbolNames` (string[])
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/ElementRewriter.cs)

#### `void EndInvoke(System.IAsyncResult result)`

SymbolConflictWarningHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/ElementRewriter.cs)

#### `void Invoke(string symbolName, string[] collidingSymbolNames)`

SymbolConflictWarningHandler.Invoke

**Parameters:**
- `symbolName` (string)
- `collidingSymbolNames` (string[])

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/ElementRewriter.cs)

