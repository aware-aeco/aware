---
name: dynamo-dynamo-models-nodemodelassemblyloader
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.Models.NodeModelAssemblyLoader namespace — 2 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AssemblyLoadedHandler, AssemblyLoadedEventArgs.
---

# Dynamo.Models.NodeModelAssemblyLoader

Auto-generated from vendor docs for dynamo 4.1.1. 2 types in this namespace.

## AssemblyLoadedEventArgs (class)

Type AssemblyLoadedEventArgs

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/NodeModelAssemblyLoader.cs)

### Constructors
- `void AssemblyLoadedEventArgs(System.Reflection.Assembly assembly)` — Creates AssemblyLoadedEventArgs

### Properties
- `Assembly` (System.Reflection.Assembly, get) — Loaded assembly.

## AssemblyLoadedHandler (delegate)

Type AssemblyLoadedHandler

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/NodeModelAssemblyLoader.cs)

### Constructors
- `void AssemblyLoadedHandler(object object, IntPtr method)` — AssemblyLoadedHandler.AssemblyLoadedHandler

### Methods
#### `System.IAsyncResult BeginInvoke(Dynamo.Models.NodeModelAssemblyLoader.AssemblyLoadedEventArgs args, System.AsyncCallback callback, object object)`

AssemblyLoadedHandler.BeginInvoke

**Parameters:**
- `args` (Dynamo.Models.NodeModelAssemblyLoader.AssemblyLoadedEventArgs)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/NodeModelAssemblyLoader.cs)

#### `void EndInvoke(System.IAsyncResult result)`

AssemblyLoadedHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/NodeModelAssemblyLoader.cs)

#### `void Invoke(Dynamo.Models.NodeModelAssemblyLoader.AssemblyLoadedEventArgs args)`

AssemblyLoadedHandler.Invoke

**Parameters:**
- `args` (Dynamo.Models.NodeModelAssemblyLoader.AssemblyLoadedEventArgs)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/NodeModelAssemblyLoader.cs)

