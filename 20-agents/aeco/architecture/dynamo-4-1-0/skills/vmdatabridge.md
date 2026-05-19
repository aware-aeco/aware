---
name: dynamo-vmdatabridge
description: This skill encodes the dynamo 4.1.0 surface of the VMDataBridge namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: DataBridge.
---

# VMDataBridge

Auto-generated from vendor docs for dynamo 4.1.0. 1 types in this namespace.

## DataBridge (class)

Provides callback registration by GUID, allows for hooking Actions into the VM.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Libraries/VMDataBridge/VMDataBridge.cs)

### Constructors
- `void DataBridge()` — DataBridge.DataBridge

### Methods
#### `void BridgeData(string id, object data)`

Calls the registered callback for the given guid string with the given data. This is safe to include in standalone DS scripts, since if there are no callbacks registered then the method will do nothing.

**Parameters:**
- `id` (string) — String identifying which registered callback to invoke.
- `data` (object) — Data to be passed to the callback.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Libraries/VMDataBridge/VMDataBridge.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode GenerateBridgeDataAst(string id, ProtoCore.AST.AssociativeAST.AssociativeNode input)`

Produces AST that, when executed by the VM, will perform Data Bridging by calling BridgeData.

**Parameters:**
- `id` (string) — Guid identifying which registered callback to be invoked.
- `input` (ProtoCore.AST.AssociativeAST.AssociativeNode) — AST representing the data to be passed to the callback.

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Libraries/VMDataBridge/VMDataBridge.cs)

#### `void RegisterCallback(string id, System.Action<object> callback)`

Registers a callback for a given GUID, to be invoked by the VM on an arbitrary thread. There are no guarantees as to what thread the callback will be invoked on.

**Parameters:**
- `id` (string) — Guid used to identify the callback.
- `callback` (System.Action<object>) — Action to be invoked with data from the VM.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Libraries/VMDataBridge/VMDataBridge.cs)

#### `bool UnregisterCallback(string id)`

Unregisters a callback for a given GUID.

**Parameters:**
- `id` (string) — Guid identifying the callback to be removed.

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Libraries/VMDataBridge/VMDataBridge.cs)

### Properties
- `Instance` (VMDataBridge.DataBridge, get) — DataBridge Singleton

