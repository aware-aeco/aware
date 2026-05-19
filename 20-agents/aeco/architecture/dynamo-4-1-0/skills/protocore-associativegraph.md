---
name: dynamo-protocore-associativegraph
description: This skill encodes the dynamo 4.1.0 surface of the ProtoCore.AssociativeGraph namespace — 6 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: UpdateBlock, GraphNode, DependencyGraph, UpdateNodeType, UpdateNode, UpdateNodeRef.
---

# ProtoCore.AssociativeGraph

Auto-generated from vendor docs for dynamo 4.1.0. 6 types in this namespace.

## DependencyGraph (class)

Type DependencyGraph

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

### Constructors
- `void DependencyGraph(ProtoCore.Core core)` — DependencyGraph.DependencyGraph

### Methods
#### `System.Collections.Generic.List<bool> GetExecutionStatesAtScope(int classIndex, int procIndex)`

DependencyGraph.GetExecutionStatesAtScope

**Parameters:**
- `classIndex` (int)
- `procIndex` (int)

**Returns:** `System.Collections.Generic.List<bool>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode> GetGraphNodesAtScope(int classIndex, int procIndex)`

DependencyGraph.GetGraphNodesAtScope

**Parameters:**
- `classIndex` (int)
- `procIndex` (int)

**Returns:** `System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `void Push(ProtoCore.AssociativeGraph.GraphNode node)`

DependencyGraph.Push

**Parameters:**
- `node` (ProtoCore.AssociativeGraph.GraphNode)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `void RemoveNodesFromScope(int classIndex, int procIndex)`

DependencyGraph.RemoveNodesFromScope

**Parameters:**
- `classIndex` (int)
- `procIndex` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

### Properties
- `GraphList` (System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode>, get) — DependencyGraph.GraphList

## GraphNode (class)

Type GraphNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

### Constructors
- `void GraphNode()` — GraphNode.GraphNode

### Methods
#### `bool DependsOn(ProtoCore.AssociativeGraph.UpdateNodeRef modifiedRef, ref ProtoCore.AssociativeGraph.GraphNode dependentNode)`

GraphNode.DependsOn

**Parameters:**
- `modifiedRef` (ProtoCore.AssociativeGraph.UpdateNodeRef)
- `dependentNode` (ref ProtoCore.AssociativeGraph.GraphNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `bool DependsOnTempSSA()`

GraphNode.DependsOnTempSSA

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `bool IsSSANode()`

GraphNode.IsSSANode

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `bool IsUpdateableBy(ProtoCore.AssociativeGraph.UpdateNodeRef modifiedRef)`

GraphNode.IsUpdateableBy

**Parameters:**
- `modifiedRef` (ProtoCore.AssociativeGraph.UpdateNodeRef)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `void PushChildNode(ProtoCore.AssociativeGraph.GraphNode child)`

GraphNode.PushChildNode

**Parameters:**
- `child` (ProtoCore.AssociativeGraph.GraphNode)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `void PushDependent(ProtoCore.AssociativeGraph.GraphNode dependent)`

GraphNode.PushDependent

**Parameters:**
- `dependent` (ProtoCore.AssociativeGraph.GraphNode)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `void PushSymbolReference(ProtoCore.DSASM.SymbolNode symbol)`

GraphNode.PushSymbolReference

**Parameters:**
- `symbol` (ProtoCore.DSASM.SymbolNode)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `void PushSymbolReference(ProtoCore.DSASM.SymbolNode symbol, ProtoCore.AssociativeGraph.UpdateNodeType type)`

GraphNode.PushSymbolReference

**Parameters:**
- `symbol` (ProtoCore.DSASM.SymbolNode)
- `type` (ProtoCore.AssociativeGraph.UpdateNodeType)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `void ResolveLHSArrayIndex()`

GraphNode.ResolveLHSArrayIndex

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

### Properties
- `AstID` (int, get/set) — GraphNode.AstID
- `CallsiteIdentifier` (string, get/set) — GraphNode.CallsiteIdentifier
- `ChildrenNodes` (System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode>, get/set) — Children nodes are nodes that will be marked dirty if this graphnode is executed a = 1 <- the child of this graphnode is 'b = a' b = a
- `IsLHSIdentList` (bool, get/set) — GraphNode.IsLHSIdentList
- `IsLastNodeInSSA` (bool, get/set) — GraphNode.IsLastNodeInSSA
- `IsModifier` (bool, get/set) — GraphNode.IsModifier
- `OriginalAstID` (int, get/set) — GraphNode.OriginalAstID
- `ParentNodes` (System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode>, get/set) — Parent nodes are the nodes that this graphnode is dependent on a = 1 b = a <- the parent of this graphnode is 'a = 1'
- `ProcedureOwned` (bool, get/set) — GraphNode.ProcedureOwned
- `UID` (int, get/set) — GraphNode.UID
- `allowDependents` (bool, get/set) — GraphNode.allowDependents
- `classIndex` (int, get/set) — GraphNode.classIndex
- `counter` (int, get/set) — GraphNode.counter
- `cyclePoint` (ProtoCore.AssociativeGraph.GraphNode, get/set) — GraphNode.cyclePoint
- `dependencyGraphListID` (int, get/set) — GraphNode.dependencyGraphListID
- `dependentList` (System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode>, get/set) — GraphNode.dependentList
- `dimensionNodeList` (System.Collections.Generic.List<ProtoCore.AssociativeGraph.UpdateNode>, get/set) — GraphNode.dimensionNodeList
- `exprUID` (int, get/set) — GraphNode.exprUID
- `firstProc` (ProtoCore.DSASM.ProcedureNode, get/set) — GraphNode.firstProc
- `guid` (System.Guid, get/set) — GraphNode.guid
- `isActive` (bool, get/set) — Flag determines if a graph node is active or not. If inactive, the graph node is invalid this is especially used in the LiveRunner to mark modified/deleted nodes inactive so that they are not executed
- `isAutoGenerated` (bool, get/set) — GraphNode.isAutoGenerated
- `isCyclic` (bool, get/set) — GraphNode.isCyclic
- `isDirty` (bool, get/set) — GraphNode.isDirty
- `isIndexingLHS` (bool, get/set) — GraphNode.isIndexingLHS
- `isInlineConditional` (bool, get/set) — GraphNode.isInlineConditional
- `isLHSNode` (bool, get/set) — GraphNode.isLHSNode
- `isLanguageBlock` (bool, get/set) — GraphNode.isLanguageBlock
- `isReturn` (bool, get/set) — GraphNode.isReturn
- `languageBlockId` (int, get/set) — GraphNode.languageBlockId
- `lastGraphNode` (ProtoCore.AssociativeGraph.GraphNode, get/set) — GraphNode.lastGraphNode
- `procIndex` (int, get/set) — GraphNode.procIndex
- `reExecuteExpression` (bool, get/set) — GraphNode.reExecuteExpression
- `ssaExpressionUID` (int, get/set) — GraphNode.ssaExpressionUID
- `ssaSubExpressionID` (int, get/set) — GraphNode.ssaSubExpressionID
- `symbolListWithinExpression` (System.Collections.Generic.List<ProtoCore.DSASM.SymbolNode>, get/set) — This is the list of lhs symbols in the same expression ID It is applicable for expressions transformed to SSA where each ssa temp in the same expression is in this list This list is only populated on the last SSA assignment as such: Given a = b.c.d [0] t0 = b -> List empty [1] t1 = t0.c -> List empty [2] t2 = t1.d -> List empty [3] a = t2 -> This is the last SSA stmt, its graphnode contains a list of graphnodes {t0,t1,t2}
- `updateBlock` (ProtoCore.AssociativeGraph.UpdateBlock, get/set) — GraphNode.updateBlock
- `updateNodeRefList` (System.Collections.Generic.List<ProtoCore.AssociativeGraph.UpdateNodeRef>, get/set) — GraphNode.updateNodeRefList

## UpdateBlock (class)

Type UpdateBlock

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

### Constructors
- `void UpdateBlock()` — UpdateBlock.UpdateBlock

### Properties
- `endpc` (int, get/set) — UpdateBlock.endpc
- `startpc` (int, get/set) — UpdateBlock.startpc
- `updateRegisterStartPC` (int, get/set) — UpdateBlock.updateRegisterStartPC

## UpdateNode (class)

Type UpdateNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

### Constructors
- `void UpdateNode()` — UpdateNode.UpdateNode

### Methods
#### `bool Equals(object obj)`

UpdateNode.Equals

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

### Properties
- `dimensionNodeList` (System.Collections.Generic.List<ProtoCore.AssociativeGraph.UpdateNode>, get/set) — UpdateNode.dimensionNodeList

## UpdateNodeRef (class)

Type UpdateNodeRef

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

### Constructors
- `void UpdateNodeRef()` — UpdateNodeRef.UpdateNodeRef
- `void UpdateNodeRef(ProtoCore.AssociativeGraph.UpdateNodeRef rhs)` — UpdateNodeRef.UpdateNodeRef

### Methods
#### `bool Equals(object obj)`

UpdateNodeRef.Equals

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `ProtoCore.AssociativeGraph.UpdateNodeRef GetUntilFirstProc()`

UpdateNodeRef.GetUntilFirstProc

**Returns:** `ProtoCore.AssociativeGraph.UpdateNodeRef` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `void PushUpdateNode(ProtoCore.AssociativeGraph.UpdateNode node)`

UpdateNodeRef.PushUpdateNode

**Parameters:**
- `node` (ProtoCore.AssociativeGraph.UpdateNode)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

#### `void PushUpdateNodeRef(ProtoCore.AssociativeGraph.UpdateNodeRef nodeRef)`

UpdateNodeRef.PushUpdateNodeRef

**Parameters:**
- `nodeRef` (ProtoCore.AssociativeGraph.UpdateNodeRef)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

### Properties
- `block` (int, get/set) — UpdateNodeRef.block
- `nodeList` (System.Collections.Generic.List<ProtoCore.AssociativeGraph.UpdateNode>, get/set) — UpdateNodeRef.nodeList
- `symbolData` (ProtoCore.DSASM.StackValue, get/set) — UpdateNodeRef.symbolData

## UpdateNodeType (enum)

Type UpdateNodeType

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AssociativeGraph.cs)

### Values
- `Literal` = `0`
- `Method` = `2`
- `Symbol` = `1`

