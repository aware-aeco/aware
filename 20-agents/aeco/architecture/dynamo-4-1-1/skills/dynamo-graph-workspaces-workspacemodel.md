---
name: dynamo-dynamo-graph-workspaces-workspacemodel
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.Graph.Workspaces.WorkspaceModel namespace — 2 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: WorkspaceSavedEvent, PointEventHandler.
---

# Dynamo.Graph.Workspaces.WorkspaceModel

Auto-generated from vendor docs for dynamo 4.1.1. 2 types in this namespace.

## PointEventHandler (delegate)

Type PointEventHandler

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

### Constructors
- `void PointEventHandler(object object, IntPtr method)` — PointEventHandler.PointEventHandler

### Methods
#### `System.IAsyncResult BeginInvoke(object sender, System.EventArgs e, System.AsyncCallback callback, object object)`

PointEventHandler.BeginInvoke

**Parameters:**
- `sender` (object)
- `e` (System.EventArgs)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void EndInvoke(System.IAsyncResult result)`

PointEventHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void Invoke(object sender, System.EventArgs e)`

PointEventHandler.Invoke

**Parameters:**
- `sender` (object)
- `e` (System.EventArgs)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

## WorkspaceSavedEvent (delegate)

Type WorkspaceSavedEvent

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

### Constructors
- `void WorkspaceSavedEvent(object object, IntPtr method)` — WorkspaceSavedEvent.WorkspaceSavedEvent

### Methods
#### `System.IAsyncResult BeginInvoke(Dynamo.Graph.Workspaces.WorkspaceModel model, System.AsyncCallback callback, object object)`

WorkspaceSavedEvent.BeginInvoke

**Parameters:**
- `model` (Dynamo.Graph.Workspaces.WorkspaceModel)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void EndInvoke(System.IAsyncResult result)`

WorkspaceSavedEvent.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

#### `void Invoke(Dynamo.Graph.Workspaces.WorkspaceModel model)`

WorkspaceSavedEvent.Invoke

**Parameters:**
- `model` (Dynamo.Graph.Workspaces.WorkspaceModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Workspaces/WorkspaceModel.cs)

