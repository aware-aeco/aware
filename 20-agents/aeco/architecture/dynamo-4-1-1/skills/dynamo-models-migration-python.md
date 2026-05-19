---
name: dynamo-dynamo-models-migration-python
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.Models.Migration.Python namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: PythonEngineUpgradeService.
---

# Dynamo.Models.Migration.Python

Auto-generated from vendor docs for dynamo 4.1.1. 1 types in this namespace.

## PythonEngineUpgradeService (class)

Headless service that detects Python engine usage in a workspace, performs temporary engine upgrades on nodes, and (optionally) commits permanent upgrades for custom node workspaces.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/PythonEngineUpgradeService.cs)

### Constructors
- `void PythonEngineUpgradeService(Dynamo.Models.DynamoModel dynamoModel, Dynamo.Interfaces.IPathManager pathManager)` — PythonEngineUpgradeService.PythonEngineUpgradeService

### Methods
#### `string BuildBackupFilePath(Dynamo.Graph.Workspaces.WorkspaceModel workspace, string token)`

Build a backup file path for a .dyn or .dyf backup

**Parameters:**
- `workspace` (Dynamo.Graph.Workspaces.WorkspaceModel)
- `token` (string)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/PythonEngineUpgradeService.cs)

#### `void CommitCustomNodeMigrationsOnSave(Dynamo.Graph.Workspaces.WorkspaceModel hostWorkspace)`

Commits migration changes for custom-node workspaces by editing the .dyf JSON in place: switches Python nodes from CPython3 to PythonNet3 and saves a backup file

**Parameters:**
- `hostWorkspace` (Dynamo.Graph.Workspaces.WorkspaceModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/PythonEngineUpgradeService.cs)

#### `Dynamo.Models.Migration.Python.PythonEngineUpgradeService.Usage DetectPythonUsage(Dynamo.Graph.Workspaces.WorkspaceModel workspace, System.Func<Dynamo.Graph.Nodes.NodeModel, bool> isPythonNode)`

Scan the workspace for python usage (direct) and one level of custom nodes.

**Parameters:**
- `workspace` (Dynamo.Graph.Workspaces.WorkspaceModel)
- `isPythonNode` (System.Func<Dynamo.Graph.Nodes.NodeModel, bool>)

**Returns:** `Dynamo.Models.Migration.Python.PythonEngineUpgradeService.Usage` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/PythonEngineUpgradeService.cs)

#### `int UpgradeNodesInMemory(System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel> pyNodes, Dynamo.Graph.Workspaces.WorkspaceModel workspace, System.Action<Dynamo.Graph.Nodes.NodeModel, Dynamo.Graph.Workspaces.WorkspaceModel> setEngine)`

Upgrade the engine for a set of python nodes in memory and return the count changed.

**Parameters:**
- `pyNodes` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel>)
- `workspace` (Dynamo.Graph.Workspaces.WorkspaceModel)
- `setEngine` (System.Action<Dynamo.Graph.Nodes.NodeModel, Dynamo.Graph.Workspaces.WorkspaceModel>)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/PythonEngineUpgradeService.cs)

