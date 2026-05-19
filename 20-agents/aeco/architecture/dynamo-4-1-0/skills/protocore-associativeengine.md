---
name: dynamo-protocore-associativeengine
description: This skill encodes the dynamo 4.1.0 surface of the ProtoCore.AssociativeEngine namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Utils.
---

# ProtoCore.AssociativeEngine

Auto-generated from vendor docs for dynamo 4.1.0. 1 types in this namespace.

## Utils (class)

Type Utils

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

### Constructors
- `void Utils()` — Utils.Utils

### Methods
#### `bool AreLHSEqualIdentList(ProtoCore.AssociativeGraph.GraphNode node, ProtoCore.AssociativeGraph.GraphNode otherNode)`

Checks if both nodes are LHS identlists and that their identlists are equal

**Parameters:**
- `node` (ProtoCore.AssociativeGraph.GraphNode) — 
- `otherNode` (ProtoCore.AssociativeGraph.GraphNode) — 

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `void BuildGraphNodeDependencies(System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode> graphNodesInScope)`

Builds the dependencies within the list of graphNodes

**Parameters:**
- `graphNodesInScope` (System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode>) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `ProtoCore.AssociativeGraph.GraphNode GetFirstDirtyGraphNodeFromPC(int pc, System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode> graphNodesInScope)`

Returns the first dirty graphnode starting from the given pc

**Parameters:**
- `pc` (int) — 
- `graphNodesInScope` (System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode>) — 

**Returns:** `ProtoCore.AssociativeGraph.GraphNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `ProtoCore.AssociativeGraph.GraphNode GetFirstSSAGraphnode(int indexOfDirtyNode, System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode> nodesInScope)`

Find the first dirty node of the graphnode residing at indexOfDirtyNode

**Parameters:**
- `indexOfDirtyNode` (int) — 
- `nodesInScope` (System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode>) — 

**Returns:** `ProtoCore.AssociativeGraph.GraphNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `ProtoCore.AssociativeGraph.GraphNode GetGraphNodeAtPC(int pc, System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode> graphNodesInScope)`

Returns the dirty graphnode of the given pc

**Parameters:**
- `pc` (int) — 
- `graphNodesInScope` (System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode>) — 

**Returns:** `ProtoCore.AssociativeGraph.GraphNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode> GetGraphNodesFromAST(ProtoCore.DSASM.Executable exe, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> astList)`

Returns the VM Graphnodes associated with the input ASTs

**Parameters:**
- `exe` (ProtoCore.DSASM.Executable) — 
- `astList` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>) — 

**Returns:** `System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode> GetRedefinedGraphNodes(ProtoCore.RuntimeCore runtimeCore, ProtoCore.AssociativeGraph.GraphNode executingGraphNode, System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode> nodesInScope, int classScope, int functionScope)`

GetRedefinedGraphNodes will return a list of graphnodes that have been redefined by executingGraphNode Given: [1] a = b + c [2] a = d Statement [1] has been redefined by statment [2] Returns true if this has occured

**Parameters:**
- `runtimeCore` (ProtoCore.RuntimeCore) — 
- `executingGraphNode` (ProtoCore.AssociativeGraph.GraphNode) — 
- `nodesInScope` (System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode>) — 
- `classScope` (int) — 
- `functionScope` (int) — 

**Returns:** `System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `bool IsGlobalScopeDirty(ProtoCore.DSASM.Executable exe)`

Determines if at least one graphnode in the glboal scope is dirty

**Parameters:**
- `exe` (ProtoCore.DSASM.Executable) — 

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `bool IsGraphNodeRedefined(ProtoCore.AssociativeGraph.GraphNode gnode, ProtoCore.AssociativeGraph.GraphNode executingNode)`

Determines if a graphnode was redefined by executingNode Given: a = b; a = 1; Where: 'a = b' has been redefined by 'a = 1'

**Parameters:**
- `gnode` (ProtoCore.AssociativeGraph.GraphNode) — 
- `executingNode` (ProtoCore.AssociativeGraph.GraphNode) — 

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `bool IsTempVarLHS(ProtoCore.AssociativeGraph.GraphNode graphNode)`

Utils.IsTempVarLHS

**Parameters:**
- `graphNode` (ProtoCore.AssociativeGraph.GraphNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `ProtoCore.AssociativeGraph.GraphNode MarkGraphNodesDirtyAtGlobalScope(ProtoCore.RuntimeCore core, System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode> nodeList)`

Finds all graphnodes associated with each AST and marks them dirty. Returns the first dirty node

**Parameters:**
- `core` (ProtoCore.RuntimeCore) — 
- `nodeList` (System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode>) — 

**Returns:** `ProtoCore.AssociativeGraph.GraphNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `void MarkGraphNodesDirtyFromFunctionRedef(ProtoCore.RuntimeCore runtimeCore, System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> fnodeList)`

Utils.MarkGraphNodesDirtyFromFunctionRedef

**Parameters:**
- `runtimeCore` (ProtoCore.RuntimeCore)
- `fnodeList` (System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode> UpdateDependencyGraph(ProtoCore.AssociativeGraph.GraphNode executingGraphNode, ProtoCore.DSASM.Executive executive, int exprUID, bool isSSAAssign, bool executeSSA, int languageBlockID, bool recursiveSearch)`

Find and return all graphnodes that can be reached by executingGraphNode

**Parameters:**
- `executingGraphNode` (ProtoCore.AssociativeGraph.GraphNode) — 
- `executive` (ProtoCore.DSASM.Executive) — 
- `exprUID` (int) — 
- `isSSAAssign` (bool) — 
- `executeSSA` (bool) — 
- `languageBlockID` (int) — 
- `recursiveSearch` (bool) — 

**Returns:** `System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

