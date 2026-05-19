---
name: dynamo-dynamo
description: This skill encodes the dynamo 4.1.0 surface of the Dynamo namespace — 2 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: CustomNodeDefinition, CustomNodeInfo.
---

# Dynamo

Auto-generated from vendor docs for dynamo 4.1.0. 2 types in this namespace.

## CustomNodeDefinition (class)

Compiler definition of a Custom Node.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Core/CustomNodeDefinition.cs)

### Constructors
- `void CustomNodeDefinition(System.Guid functionId, string displayName, System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel> nodeModels)` — This function creates CustomNodeDefinition.

### Properties
- `ContainsInvalidInput` (bool, get) — Indicates whether any of this definition's input parameters are invalid. An input is invalid when its input expression fails to parse. For example, this would happen if the input name contained spaces or illegal characters.
- `Dependencies` (System.Collections.Generic.IEnumerable<Dynamo.CustomNodeDefinition>, get) — Returns all custom node definitions.
- `DirectDependencies` (System.Collections.Generic.IEnumerable<Dynamo.CustomNodeDefinition>, get) — Returns custom node definitions for direct dependencies.
- `DisplayName` (string, get) — User friendly name on UI.
- `DisplayParameters` (System.Collections.Generic.IEnumerable<string>, get) — User-friendly parameters
- `FunctionBody` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel>, get) — NodeModels making up the body of the custom node.
- `FunctionId` (System.Guid, get) — Function unique ID.
- `FunctionName` (string, get) — Function name.
- `IsProxy` (bool, get) — Is this CustomNodeDefinition properly loaded?
- `MangledName` (string, get) — Name to create custom node
- `OutputNodes` (System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode>, get) — Identifiers associated with the outputs of the custom node.
- `Parameters` (System.Collections.Generic.IEnumerable<Dynamo.Library.TypedParameter>, get) — Function parameters.
- `ReturnKeys` (System.Collections.Generic.IEnumerable<string>, get) — If the function returns a dictionary, this specifies all keys in that dictionary.
- `ReturnType` (ProtoCore.Type, get) — Return type.
- `Returns` (System.Collections.Generic.IEnumerable<System.Tuple<string, string>>, get) — The collection of output name and its description.

## CustomNodeInfo (class)

Basic information about a custom node.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Core/CustomNodeDefinition.cs)

### Constructors
- `void CustomNodeInfo()` — This function creates CustomNodeInfo.
- `void CustomNodeInfo(System.Guid functionId, string name, string category, string description, string path, bool isVisibleInDynamoLibrary)` — This function creates CustomNodeInfo.

### Properties
- `Category` (string, get/set) — Returns custom node category
- `Description` (string, get/set) — Returns custom node description
- `FunctionId` (System.Guid, get/set) — Returns custom node unique ID
- `IsPackageMember` (bool, get/set) — Indicates if custom node is part of the package. If true, then custom node is part of package manager.
- `IsVisibleInDynamoLibrary` (bool, get/set) — Indicates if custom node is part of the library search. If true, then custom node is part of library search.
- `Name` (string, get/set) — Returns custom node name
- `PackageInfo` (Dynamo.Graph.Workspaces.PackageInfo, get) — Only valid if IsPackageMember is true. Can be used to identify which package requested this CustomNode to load.
- `Path` (string, get/set) — Returns path to custom node

