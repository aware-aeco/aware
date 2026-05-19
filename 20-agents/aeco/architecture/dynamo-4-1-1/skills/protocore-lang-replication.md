---
name: dynamo-protocore-lang-replication
description: This skill encodes the dynamo 4.1.1 surface of the ProtoCore.Lang.Replication namespace — 6 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: DominantListStructure, ArgumentAtLevelStructure, AtLevelHandler, ZipAlgorithm, ReplicationInstruction, Replicator.
---

# ProtoCore.Lang.Replication

Auto-generated from vendor docs for dynamo 4.1.1. 6 types in this namespace.

## ArgumentAtLevelStructure (class)

Type ArgumentAtLevelStructure

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/Replication/ArgumentAtLevel.cs)

### Constructors
- `void ArgumentAtLevelStructure(System.Collections.Generic.List<ProtoCore.DSASM.StackValue> arguments, ProtoCore.Lang.Replication.DominantListStructure dominantStructure)` — ArgumentAtLevelStructure.ArgumentAtLevelStructure

### Properties
- `Arguments` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>, get) — ArgumentAtLevelStructure.Arguments
- `DominantStructure` (ProtoCore.Lang.Replication.DominantListStructure, get) — ArgumentAtLevelStructure.DominantStructure

## AtLevelHandler (class)

Type AtLevelHandler

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/Replication/ArgumentAtLevel.cs)

### Constructors
- `void AtLevelHandler()` — AtLevelHandler.AtLevelHandler

### Methods
#### `ProtoCore.Lang.Replication.ArgumentAtLevelStructure GetArgumentAtLevelStructure(System.Collections.Generic.List<ProtoCore.DSASM.StackValue> arguments, System.Collections.Generic.List<ProtoCore.AtLevel> atLevels, ProtoCore.RuntimeCore runtimeCore)`

Returns arguments at the corresponding levles and dominant list structure.

**Parameters:**
- `arguments` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>) — 
- `atLevels` (System.Collections.Generic.List<ProtoCore.AtLevel>) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 

**Returns:** `ProtoCore.Lang.Replication.ArgumentAtLevelStructure` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/Replication/ArgumentAtLevel.cs)

#### `ProtoCore.DSASM.StackValue RestoreDominantStructure(ProtoCore.DSASM.StackValue ret, ProtoCore.Lang.Replication.DominantListStructure domStructure, System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction> instructions, ProtoCore.RuntimeCore runtimeCore)`

If an input is a dominant list, restructure the result based on the structure of dominant list. Note the dominant structure will be restored only if the dominant list is zipped with other arguments, or the replication is applied to the dominant list firstly.

**Parameters:**
- `ret` (ProtoCore.DSASM.StackValue) — 
- `domStructure` (ProtoCore.Lang.Replication.DominantListStructure) — 
- `instructions` (System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction>) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/Replication/ArgumentAtLevel.cs)

## DominantListStructure (class)

The positions of items at dominant list.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/Replication/ArgumentAtLevel.cs)

### Constructors
- `void DominantListStructure(System.Collections.Generic.List<System.Collections.Generic.List<int>> indices, int argumentIndex)` — DominantListStructure.DominantListStructure

### Properties
- `ArgumentIndex` (int, get) — DominantListStructure.ArgumentIndex
- `Indices` (System.Collections.Generic.List<System.Collections.Generic.List<int>>, get) — DominantListStructure.Indices

## ReplicationInstruction (struct)

Representation of the replication algorithm

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/Replication/ReplicationInstruction.cs)

### Methods
#### `bool Equals(ProtoCore.Lang.Replication.ReplicationInstruction other)`

ReplicationInstruction.Equals

**Parameters:**
- `other` (ProtoCore.Lang.Replication.ReplicationInstruction)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/Replication/ReplicationInstruction.cs)

#### `string ToString()`

ReplicationInstruction.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/Replication/ReplicationInstruction.cs)

## Replicator (class)

Type Replicator

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/Replication/Replicator.cs)

### Constructors
- `void Replicator()` — Replicator.Replicator

### Methods
#### `System.Collections.Generic.List<System.Collections.Generic.List<int>> BuildAllocation(int count, int maxAlloc)`



**Parameters:**
- `count` (int) — Count is the number of elements left for allocation
- `maxAlloc` (int) — Max alloc is the maximum potential reductions that can yet be applied

**Returns:** `System.Collections.Generic.List<System.Collections.Generic.List<int>>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/Replication/Replicator.cs)

#### `System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction> BuildPartialReplicationInstructions(System.Collections.Generic.List<System.Collections.Generic.List<ProtoCore.ReplicationGuide>> partialRepGuides)`

Calculate partial replication instruciton based on replication guide level. For example, for foo(xs<1><2><3>, ys<1><1><2>, zs<1><1><3>), the guides are: level | 0 | 1 | 2 | ------+-----+-----+-----+ xs | 1 | 2 | 3 | ------+-----+-----+-----+ ys | 1 | 1 | 2 | ------+-----+-----+-----+ zs | 1 | 1 | 3 | This function goes through each level and calculate replication instructions. replication instructions on level 0: Zip replication on (0, 1, 2) (i.e., zip on xs, ys, zs) replication instructions on level 1: Zip replication on (1, 2) (i.e., zip on ys, zs) Cartesian replication on 0 (i.e., on xs) replication instructions on level 2: Cartesian replication on 1 (i.e., on ys) Zip replication on (0, 2) (i.e., zip on xs, zs)

**Parameters:**
- `partialRepGuides` (System.Collections.Generic.List<System.Collections.Generic.List<ProtoCore.ReplicationGuide>>) — 

**Returns:** `System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/Replication/Replicator.cs)

#### `System.Collections.Generic.List<System.Collections.Generic.List<int>> BuildReductions(System.Collections.Generic.List<int> reductionDepths)`

Replicator.BuildReductions

**Parameters:**
- `reductionDepths` (System.Collections.Generic.List<int>)

**Returns:** `System.Collections.Generic.List<System.Collections.Generic.List<int>>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/Replication/Replicator.cs)

#### `System.Collections.Generic.List<System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction>> BuildReplicationCombinations(System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction> providedControl, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> formalParams, ProtoCore.RuntimeCore runtimeCore)`

Build all possible replications based on the rank of parameters and the provided replicatoin guide.

**Parameters:**
- `providedControl` (System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction>) — 
- `formalParams` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 

**Returns:** `System.Collections.Generic.List<System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction>>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/Replication/Replicator.cs)

#### `System.Collections.Generic.List<System.Collections.Generic.List<ProtoCore.DSASM.StackValue>> ComputeAllReducedParams(System.Collections.Generic.List<ProtoCore.DSASM.StackValue> formalParams, System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction> replicationInstructions, ProtoCore.RuntimeCore runtimeCore)`

For each parameter, if there is a replication instruction for it, and if it is an array, expand parameter list based on the types of elements in that array. For example, for parameters {p1, p2, ..., pk, ..., pn} where pk is an array {a1:int, a2:string, a3:double, ...} and there is a Cartesian replication on pk, the parameter list will be expanded to {p1, p2, ..., a1, ..., pn} {p1, p2, ..., a2, ..., pn} {p1, p2, ..., a3, ..., pn} ...

**Parameters:**
- `formalParams` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>) — 
- `replicationInstructions` (System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction>) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 

**Returns:** `System.Collections.Generic.List<System.Collections.Generic.List<ProtoCore.DSASM.StackValue>>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/Replication/Replicator.cs)

#### `System.Collections.Generic.List<ProtoCore.DSASM.StackValue> EstimateReducedParams(System.Collections.Generic.List<ProtoCore.DSASM.StackValue> formalParams, System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction> replicationInstructions, ProtoCore.RuntimeCore runtimeCore)`

Compute the effects of the replication guides on the formal parameter lists The results of this loose data, and will not be correct on jagged arrays of hetrogenius types

**Parameters:**
- `formalParams` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>) — 
- `replicationInstructions` (System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction>) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 

**Returns:** `System.Collections.Generic.List<ProtoCore.DSASM.StackValue>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/Replication/Replicator.cs)

#### `int GetMaxReductionDepth(ProtoCore.DSASM.StackValue sv, ProtoCore.RuntimeCore runtimeCore)`

Returns the maximum depth to which an element can be reduced This will include cases where only partial reduction can be performed on jagged arrays

**Parameters:**
- `sv` (ProtoCore.DSASM.StackValue) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/Replication/Replicator.cs)

#### `System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction> ReductionToInstructions(System.Collections.Generic.List<int> reductions, System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction> providedControl)`

Convert reduction to instruction. Using zip-first strategy. For example, 0 2 4 > Zip on 1,2 0 1 3 > Zip on 1,2 0 0 2 > Cartesian on 2 0 0 1 > Cartesian on 2

**Parameters:**
- `reductions` (System.Collections.Generic.List<int>) — 
- `providedControl` (System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction>) — 

**Returns:** `System.Collections.Generic.List<ProtoCore.Lang.Replication.ReplicationInstruction>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/Replication/Replicator.cs)

## ZipAlgorithm (enum)

How should the zip algorithm handle cases where one data stream is longer than the other

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Lang/Replication/ReplicationInstruction.cs)

### Values
- `Longest` = `1`
- `Shortest` = `0`

