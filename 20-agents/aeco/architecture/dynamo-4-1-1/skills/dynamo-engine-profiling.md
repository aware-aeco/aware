---
name: dynamo-dynamo-engine-profiling
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.Engine.Profiling namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: IProfilingExecutionTimeData.
---

# Dynamo.Engine.Profiling

Auto-generated from vendor docs for dynamo 4.1.1. 1 types in this namespace.

## IProfilingExecutionTimeData (interface)

Returns information about time spent compiling and executing nodes.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Engine/Profiling/IProfilingExecutionTimeData.cs)

### Methods
#### `System.Nullable<System.TimeSpan> NodeExecutionTime(Dynamo.Graph.Nodes.NodeModel node)`

Returns the amount of time spent compiling and executing a specific node. Returns null if the node was not executed during the most recent graph run.

**Parameters:**
- `node` (Dynamo.Graph.Nodes.NodeModel)

**Returns:** `System.Nullable<System.TimeSpan>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Engine/Profiling/IProfilingExecutionTimeData.cs)

### Properties
- `TotalExecutionTime` (System.Nullable<System.TimeSpan>, get) — Returns the total amount of time spent compiling and executing nodes during the most recent graph run.

