---
name: dynamo-protocore-dsasm-mirror
description: This skill encodes the dynamo 4.1.1 surface of the ProtoCore.DSASM.Mirror namespace — 5 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: SymbolNotFoundException, ExecutionMirror, NameNotFoundException, UninitializedVariableException, DsasmArray.
---

# ProtoCore.DSASM.Mirror

Auto-generated from vendor docs for dynamo 4.1.1. 5 types in this namespace.

## DsasmArray (class)

Type DsasmArray

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DSASM/Mirror/ExecutionMirror.cs)

### Constructors
- `void DsasmArray()` — DsasmArray.DsasmArray

## ExecutionMirror (class)

Provides reflective capabilities over the execution of a DSASM Executable

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DSASM/Mirror/ExecutionMirror.cs)

### Constructors
- `void ExecutionMirror(ProtoCore.DSASM.Executive exec, ProtoCore.RuntimeCore coreObj)` — Create a mirror for a given executive

### Methods
#### `System.Collections.Generic.List<ProtoCore.Lang.Obj> GetArrayElements(ProtoCore.Lang.Obj obj)`

ExecutionMirror.GetArrayElements

**Parameters:**
- `obj` (ProtoCore.Lang.Obj)

**Returns:** `System.Collections.Generic.List<ProtoCore.Lang.Obj>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DSASM/Mirror/ExecutionMirror.cs)

#### `string GetClassTrace(ProtoCore.DSASM.StackValue val, ProtoCore.DSASM.Heap heap, int langblock, bool forPrint)`

ExecutionMirror.GetClassTrace

**Parameters:**
- `val` (ProtoCore.DSASM.StackValue)
- `heap` (ProtoCore.DSASM.Heap)
- `langblock` (int)
- `forPrint` (bool)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DSASM/Mirror/ExecutionMirror.cs)

#### `ProtoCore.Lang.Obj GetDebugValue(string name)`

ExecutionMirror.GetDebugValue

**Parameters:**
- `name` (string)

**Returns:** `ProtoCore.Lang.Obj` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DSASM/Mirror/ExecutionMirror.cs)

#### `ProtoCore.Lang.Obj GetFirstValue(string name, int startBlock, int classcope)`

ExecutionMirror.GetFirstValue

**Parameters:**
- `name` (string)
- `startBlock` (int)
- `classcope` (int)

**Returns:** `ProtoCore.Lang.Obj` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DSASM/Mirror/ExecutionMirror.cs)

#### `System.Collections.Generic.Dictionary<string, ProtoCore.Lang.Obj> GetProperties(ProtoCore.Lang.Obj obj, bool excludeStatic)`

ExecutionMirror.GetProperties

**Parameters:**
- `obj` (ProtoCore.Lang.Obj)
- `excludeStatic` (bool)

**Returns:** `System.Collections.Generic.Dictionary<string, ProtoCore.Lang.Obj>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DSASM/Mirror/ExecutionMirror.cs)

#### `ProtoCore.DSASM.StackValue GetRawFirstValue(string name, int startBlock, int classcope)`

ExecutionMirror.GetRawFirstValue

**Parameters:**
- `name` (string)
- `startBlock` (int)
- `classcope` (int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DSASM/Mirror/ExecutionMirror.cs)

#### `string GetStringValue(ProtoCore.DSASM.StackValue val, ProtoCore.DSASM.Heap heap, int langblock, bool forPrint)`



**Parameters:**
- `val` (ProtoCore.DSASM.StackValue) — 
- `heap` (ProtoCore.DSASM.Heap) — 
- `langblock` (int) — 
- `forPrint` (bool) — 

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DSASM/Mirror/ExecutionMirror.cs)

#### `string GetStringValue(ProtoCore.DSASM.StackValue val, ProtoCore.DSASM.Heap heap, int langblock, int maxArraySize, int maxOutputDepth, bool forPrint)`



**Parameters:**
- `val` (ProtoCore.DSASM.StackValue) — 
- `heap` (ProtoCore.DSASM.Heap) — 
- `langblock` (int) — 
- `maxArraySize` (int) — 
- `maxOutputDepth` (int) — 
- `forPrint` (bool) — 

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DSASM/Mirror/ExecutionMirror.cs)

#### `string GetType(ProtoCore.Lang.Obj obj)`

ExecutionMirror.GetType

**Parameters:**
- `obj` (ProtoCore.Lang.Obj)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DSASM/Mirror/ExecutionMirror.cs)

#### `string GetType(string name)`

ExecutionMirror.GetType

**Parameters:**
- `name` (string)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DSASM/Mirror/ExecutionMirror.cs)

#### `ProtoCore.DSASM.StackValue GetValue(string name, int startBlock)`

Searching variable name starting from specified block. Exception: SymbolNotFoundException if variable not found.

**Parameters:**
- `name` (string) — 
- `startBlock` (int) — 

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DSASM/Mirror/ExecutionMirror.cs)

#### `ProtoCore.Lang.Obj GetWatchValue()`

ExecutionMirror.GetWatchValue

**Returns:** `ProtoCore.Lang.Obj` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DSASM/Mirror/ExecutionMirror.cs)

#### `void SetValueAndExecute(string varName, System.Nullable<int> value)`

Reset an existing value and re-execute the vm

**Parameters:**
- `varName` (string) — 
- `value` (System.Nullable<int>) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DSASM/Mirror/ExecutionMirror.cs)

#### `ProtoCore.Lang.Obj Unpack(ProtoCore.DSASM.StackValue val)`

Do the recursive unpacking of the data structure into mirror objects

**Parameters:**
- `val` (ProtoCore.DSASM.StackValue) — 

**Returns:** `ProtoCore.Lang.Obj` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DSASM/Mirror/ExecutionMirror.cs)

#### `ProtoCore.Lang.Obj Unpack(ProtoCore.DSASM.StackValue val, ProtoCore.DSASM.Heap heap, ProtoCore.RuntimeCore runtimeCore, int type)`

Do the recursive unpacking of the data structure into mirror objects

**Parameters:**
- `val` (ProtoCore.DSASM.StackValue) — 
- `heap` (ProtoCore.DSASM.Heap) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 
- `type` (int) — 

**Returns:** `ProtoCore.Lang.Obj` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DSASM/Mirror/ExecutionMirror.cs)

### Properties
- `MirrorTarget` (ProtoCore.DSASM.Executive, get) — ExecutionMirror.MirrorTarget

## NameNotFoundException (class)

Type NameNotFoundException

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DSASM/Mirror/ExecutionMirror.cs)

### Constructors
- `void NameNotFoundException()` — NameNotFoundException.NameNotFoundException

### Properties
- `Name` (string, get/set) — NameNotFoundException.Name

## SymbolNotFoundException (class)

Type SymbolNotFoundException

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DSASM/Mirror/ExecutionMirror.cs)

### Constructors
- `void SymbolNotFoundException(string symbolName)` — SymbolNotFoundException.SymbolNotFoundException

### Properties
- `SymbolName` (string, get) — SymbolNotFoundException.SymbolName

## UninitializedVariableException (class)

Type UninitializedVariableException

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/DSASM/Mirror/ExecutionMirror.cs)

### Constructors
- `void UninitializedVariableException()` — UninitializedVariableException.UninitializedVariableException

### Properties
- `Name` (string, get/set) — UninitializedVariableException.Name

