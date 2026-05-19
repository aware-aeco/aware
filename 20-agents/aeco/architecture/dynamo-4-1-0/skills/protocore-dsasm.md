---
name: dynamo-protocore-dsasm
description: This skill encodes the dynamo 4.1.0 surface of the ProtoCore.DSASM namespace — 50 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AttributeEntry, ClassNode, ClassTable, InterpreterMode, Operator, UnaryOperator, RangeStepOperator, kw, and 42 more types.
---

# ProtoCore.DSASM

Auto-generated from vendor docs for dynamo 4.1.0. 50 types in this namespace.

## AddressType (enum)

Type AddressType

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

### Values
- `ArrayDim` = `19`
- `ArrayKey` = `27`
- `ArrayPointer` = `15`
- `BlockIndex` = `13`
- `Boolean` = `9`
- `CallingConvention` = `23`
- `Char` = `10`
- `ClassIndex` = `6`
- `DefaultArg` = `18`
- `Double` = `8`
- `Dynamic` = `21`
- `ExplicitCall` = `26`
- `FrameType` = `24`
- `FunctionIndex` = `3`
- `FunctionPointer` = `16`
- `Int` = `7`
- `Invalid` = `0`
- `LabelIndex` = `12`
- `MemVarIndex` = `4`
- `Null` = `17`
- `Pointer` = `14`
- `Register` = `1`
- `ReplicationGuide` = `20`
- `StaticMemVarIndex` = `5`
- `StaticType` = `22`
- `String` = `11`
- `ThisPtr` = `25`
- `VarIndex` = `2`

## ArgumentInfo (struct)

Type ArgumentInfo

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ProcedureTable.cs)

### Properties
- `IsDefault` (bool, get) — ArgumentInfo.IsDefault
- `Name` (string, get/set) — ArgumentInfo.Name

## AttributeEntry (class)

Type AttributeEntry

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AttributeEntry.cs)

### Constructors
- `void AttributeEntry()` — AttributeEntry.AttributeEntry

### Methods
#### `bool IsInternalClassAttribute()`

AttributeEntry.IsInternalClassAttribute

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/AttributeEntry.cs)

### Properties
- `Arguments` (System.Collections.Generic.List<ProtoCore.AST.Node>, get/set) — AttributeEntry.Arguments
- `ClassNode` (ProtoCore.DSASM.ClassNode, get/set) — AttributeEntry.ClassNode

## BiDictionaryOneToOne<TFirst, TSecond> (class)

This is a dictionary guaranteed to have only one of each value and key. It may be searched either by TFirst or by TSecond, giving a unique answer because it is 1 to 1.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/FunctionPointerTable.cs)

### Constructors
- `void BiDictionaryOneToOne()` — BiDictionaryOneToOne.BiDictionaryOneToOne

### Methods
#### `bool TryAdd(TFirst first, TSecond second)`

Tries to add the pair to the dictionary. Returns false if either element is already in the dictionary

**Parameters:**
- `first` (TFirst) — 
- `second` (TSecond) — 

**Returns:** `bool` — true if successfully added, false if either element are already in the dictionary

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/FunctionPointerTable.cs)

#### `bool TryGetByFirst(TFirst first, ref TSecond second)`

Find the TSecond corresponding to the TFirst first. Returns false if first is not in the dictionary.

**Parameters:**
- `first` (TFirst) — the key to search for
- `second` (ref TSecond) — the corresponding value

**Returns:** `bool` — true if first is in the dictionary, false otherwise

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/FunctionPointerTable.cs)

#### `bool TryGetBySecond(TSecond second, ref TFirst first)`

Find the TFirst corresponding to the TSecond second. Returns false if second is not in the dictionary.

**Parameters:**
- `second` (TSecond) — the key to search for
- `first` (ref TFirst) — the corresponding value

**Returns:** `bool` — true if second is in the dictionary, false otherwise

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/FunctionPointerTable.cs)

### Properties
- `Count` (int, get) — The number of pairs stored in the dictionary

## ClassNode (class)

Type ClassNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ClassTable.cs)

### Constructors
- `void ClassNode()` — ClassNode.ClassNode
- `void ClassNode(ProtoCore.DSASM.ClassNode rhs)` — ClassNode.ClassNode

### Methods
#### `bool ConvertibleTo(int type)`

ClassNode.ConvertibleTo

**Parameters:**
- `type` (int)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ClassTable.cs)

#### `int GetCoercionScore(int type)`

ClassNode.GetCoercionScore

**Parameters:**
- `type` (int)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ClassTable.cs)

#### `ProtoCore.DSASM.ProcedureNode GetDisposeMethod()`

ClassNode.GetDisposeMethod

**Returns:** `ProtoCore.DSASM.ProcedureNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ClassTable.cs)

#### `ProtoCore.DSASM.ProcedureNode GetFirstConstructorBy(string procName, int argCount)`

ClassNode.GetFirstConstructorBy

**Parameters:**
- `procName` (string)
- `argCount` (int)

**Returns:** `ProtoCore.DSASM.ProcedureNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ClassTable.cs)

#### `ProtoCore.DSASM.ProcedureNode GetFirstMemberFunctionBy(string procName)`

ClassNode.GetFirstMemberFunctionBy

**Parameters:**
- `procName` (string)

**Returns:** `ProtoCore.DSASM.ProcedureNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ClassTable.cs)

#### `ProtoCore.DSASM.ProcedureNode GetFirstMemberFunctionBy(string procName, int argCount)`

ClassNode.GetFirstMemberFunctionBy

**Parameters:**
- `procName` (string)
- `argCount` (int)

**Returns:** `ProtoCore.DSASM.ProcedureNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ClassTable.cs)

#### `ProtoCore.DSASM.ProcedureNode GetFirstStaticFunctionBy(string procName)`

ClassNode.GetFirstStaticFunctionBy

**Parameters:**
- `procName` (string)

**Returns:** `ProtoCore.DSASM.ProcedureNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ClassTable.cs)

#### `ProtoCore.DSASM.ProcedureNode GetFirstStaticFunctionBy(string procName, int argCount)`

ClassNode.GetFirstStaticFunctionBy

**Parameters:**
- `procName` (string)
- `argCount` (int)

**Returns:** `ProtoCore.DSASM.ProcedureNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ClassTable.cs)

#### `int GetFirstVisibleSymbolNoAccessCheck(string name)`

ClassNode.GetFirstVisibleSymbolNoAccessCheck

**Parameters:**
- `name` (string)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ClassTable.cs)

#### `ProtoCore.DSASM.ProcedureNode GetMemberFunction(string procName, System.Collections.Generic.List<ProtoCore.Type> argTypeList, int classScope, ref bool isAccessible, ref int functionHostClassIndex, bool isStaticOrConstructor)`

ClassNode.GetMemberFunction

**Parameters:**
- `procName` (string)
- `argTypeList` (System.Collections.Generic.List<ProtoCore.Type>)
- `classScope` (int)
- `isAccessible` (ref bool)
- `functionHostClassIndex` (ref int)
- `isStaticOrConstructor` (bool)

**Returns:** `ProtoCore.DSASM.ProcedureNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ClassTable.cs)

#### `bool IsMemberVariable(ProtoCore.DSASM.SymbolNode symbol)`

ClassNode.IsMemberVariable

**Parameters:**
- `symbol` (ProtoCore.DSASM.SymbolNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ClassTable.cs)

#### `bool IsMyBase(int type)`

ClassNode.IsMyBase

**Parameters:**
- `type` (int)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ClassTable.cs)

### Properties
- `Attributes` (System.Collections.Generic.List<ProtoCore.DSASM.AttributeEntry>, get/set) — ClassNode.Attributes
- `Base` (int, get/set) — ClassNode.Base
- `ClassAttributes` (ProtoCore.AST.AssociativeAST.ClassAttributes, get/set) — ClassNode.ClassAttributes
- `CoerceTypes` (System.Collections.Generic.Dictionary<int, int>, get/set) — ClassNode.CoerceTypes
- `ExternLib` (string, get/set) — String description of where the classnode was loaded from The implementation of the class is stored from here
- `ID` (int, get/set) — ClassNode.ID
- `Interfaces` (System.Collections.Generic.List<int>, get/set) — ClassNode.Interfaces
- `IsEmpty` (bool, get) — ClassNode.IsEmpty
- `IsImportedClass` (bool, get/set) — ClassNode.IsImportedClass
- `IsInterface` (bool, get/set) — ClassNode.IsInterface
- `IsStatic` (bool, get/set) — ClassNode.IsStatic
- `Name` (string, get/set) — ClassNode.Name
- `ProcTable` (ProtoCore.DSASM.ProcedureTable, get/set) — ClassNode.ProcTable
- `Rank` (int, get/set) — ClassNode.Rank
- `Size` (int, get/set) — ClassNode.Size
- `Symbols` (ProtoCore.DSASM.SymbolTable, get/set) — ClassNode.Symbols
- `TypeSystem` (ProtoCore.TypeSystem, get/set) — ClassNode.TypeSystem

## ClassTable (class)

Type ClassTable

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ClassTable.cs)

### Constructors
- `void ClassTable()` — ClassTable.ClassTable
- `void ClassTable(ProtoCore.DSASM.ClassTable rhs)` — ClassTable.ClassTable

### Methods
#### `int Append(ProtoCore.DSASM.ClassNode node)`

ClassTable.Append

**Parameters:**
- `node` (ProtoCore.DSASM.ClassNode)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ClassTable.cs)

#### `void AuditMultipleDefinition(ProtoCore.BuildStatus status, ProtoCore.AssociativeGraph.GraphNode graphNode)`

Audits the class table for multiple symbol definition.

**Parameters:**
- `status` (ProtoCore.BuildStatus) — BuildStatus to log the warnings if multiple symbol found.
- `graphNode` (ProtoCore.AssociativeGraph.GraphNode) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ClassTable.cs)

#### `string[] GetAllMatchingClasses(string name)`

Returns all matching classes for the given name from this ClassTable. If the classes have a common base class, this simply returns the given name of the class.

**Parameters:**
- `name` (string) — Partial name of the class for lookup

**Returns:** `string[]` — Array of fully qualified name of all matching symbols

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ClassTable.cs)

#### `int GetClassId(string fullname)`

Returns Class Id for the given fully qualified class name.

**Parameters:**
- `fullname` (string) — Fully qualified class name

**Returns:** `int` — Class Id if found, else ProtoCore.DSASM.Constants.kInvalidIndex

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ClassTable.cs)

#### `string GetTypeName(int UID)`

ClassTable.GetTypeName

**Parameters:**
- `UID` (int)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ClassTable.cs)

#### `int IndexOf(string partialName)`

Find a matching class for given partial class name.

**Parameters:**
- `partialName` (string) — Partial class name for lookup.

**Returns:** `int` — Class Id if found, else ProtoCore.DSASM.Constants.kInvalidIndex

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ClassTable.cs)

#### `void Reserve(int size)`

ClassTable.Reserve

**Parameters:**
- `size` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ClassTable.cs)

#### `void SetClassNodeAt(ProtoCore.DSASM.ClassNode node, int index)`

ClassTable.SetClassNodeAt

**Parameters:**
- `node` (ProtoCore.DSASM.ClassNode)
- `index` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ClassTable.cs)

#### `bool TryGetFullyQualifiedName(string name, ref string fullName)`

Tries to get the fully qualified name for the given name from this ClassTable.

**Parameters:**
- `name` (string) — Partial name of the class for lookup
- `fullName` (ref string) — Fully qualified class name

**Returns:** `bool` — True if the given name results a unique matching symbol in this ClassTable.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ClassTable.cs)

### Properties
- `ClassNodes` (System.Collections.ObjectModel.ReadOnlyCollection<ProtoCore.DSASM.ClassNode>, get) — ClassTable.ClassNodes

## CodeBlock (class)

Type CodeBlock

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeBlock.cs)

### Constructors
- `void CodeBlock(System.Guid guid, ProtoCore.DSASM.CodeBlockType type, ProtoCore.Language langId, int cbID, ProtoCore.DSASM.SymbolTable symbols, ProtoCore.DSASM.ProcedureTable procTable, bool isBreakableBlock, ProtoCore.Core core)` — A CodeBlock represents a body of DS code

### Methods
#### `bool IsMyAncestorBlock(int blockId)`

CodeBlock.IsMyAncestorBlock

**Parameters:**
- `blockId` (int)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/CodeBlock.cs)

### Properties
- `Attributes` (System.Collections.Generic.List<ProtoCore.DSASM.AttributeEntry>, get/set) — CodeBlock.Attributes
- `blockType` (ProtoCore.DSASM.CodeBlockType, get/set) — CodeBlock.blockType
- `children` (System.Collections.Generic.List<ProtoCore.DSASM.CodeBlock>, get/set) — CodeBlock.children
- `codeBlockId` (int, get) — CodeBlock.codeBlockId
- `guid` (System.Guid, get/set) — CodeBlock.guid
- `instrStream` (ProtoCore.DSASM.InstructionStream, get/set) — CodeBlock.instrStream
- `isBreakable` (bool, get/set) — CodeBlock.isBreakable
- `language` (ProtoCore.Language, get) — CodeBlock.language
- `parent` (ProtoCore.DSASM.CodeBlock, get/set) — CodeBlock.parent
- `procedureTable` (ProtoCore.DSASM.ProcedureTable, get) — CodeBlock.procedureTable
- `symbolTable` (ProtoCore.DSASM.SymbolTable, get/set) — CodeBlock.symbolTable

## CodeBlockType (enum)

Type CodeBlockType

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executable.cs)

### Values
- `Construct` = `1`
- `Function` = `2`
- `Language` = `0`

## Constants (struct)

Type Constants

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Defs.cs)

## DSArray (class)

Type DSArray

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSArray.cs)

### Constructors
- `void DSArray(ProtoCore.DSASM.StackValue[] values, ProtoCore.DSASM.Heap heap)` — Create an array and populate with input values
- `void DSArray(int size, ProtoCore.DSASM.Heap heap)` — Create an array with pre-allocated size.

### Methods
#### `bool CompareFromDifferentCore(ProtoCore.DSASM.DSArray array1, ProtoCore.DSASM.DSArray array2, ProtoCore.RuntimeCore rtCore1, ProtoCore.RuntimeCore rtCore2, ProtoCore.Runtime.Context context)`

DSArray.CompareFromDifferentCore

**Parameters:**
- `array1` (ProtoCore.DSASM.DSArray)
- `array2` (ProtoCore.DSASM.DSArray)
- `rtCore1` (ProtoCore.RuntimeCore)
- `rtCore2` (ProtoCore.RuntimeCore)
- `context` (ProtoCore.Runtime.Context)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSArray.cs)

#### `bool ContainsKey(ProtoCore.DSASM.StackValue key)`

Returns true if array contain key or not.

**Parameters:**
- `key` (ProtoCore.DSASM.StackValue)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSArray.cs)

#### `ProtoCore.DSASM.StackValue CopyArray(ProtoCore.RuntimeCore runtimeCore)`

Copy an array and coerce its elements/values to target type

**Parameters:**
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSArray.cs)

#### `ProtoCore.DSASM.StackValue CopyArray(ProtoCore.Type type, ProtoCore.RuntimeCore runtimeCore)`

Copy an array and coerce its elements/values to target type

**Parameters:**
- `type` (ProtoCore.Type)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSArray.cs)

#### `ProtoCore.DSASM.StackValue GetValueFromIndex(ProtoCore.DSASM.StackValue index, ProtoCore.RuntimeCore runtimeCore)`

= array[index]. Note this function doesn't support the replication of array indexing.

**Parameters:**
- `index` (ProtoCore.DSASM.StackValue)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSArray.cs)

#### `ProtoCore.DSASM.StackValue GetValueFromIndex(int index, ProtoCore.RuntimeCore runtimeCore)`

= array[index]

**Parameters:**
- `index` (int)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSArray.cs)

#### `ProtoCore.DSASM.StackValue GetValueFromIndices(ProtoCore.DSASM.StackValue[] indices, ProtoCore.RuntimeCore runtimeCore)`

= array[index1][index2][...][indexN], and indices = {index1, index2, ..., indexN} Note this function doesn't support the replication of array indexing.

**Parameters:**
- `indices` (ProtoCore.DSASM.StackValue[])
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSArray.cs)

#### `ProtoCore.DSASM.StackValue GetValueFromIndices(System.Collections.Generic.List<ProtoCore.DSASM.StackValue> indices, ProtoCore.RuntimeCore runtimeCore)`

= array[index1][index2][...][indexN], and indices = {index1, index2, ..., indexN}

**Parameters:**
- `indices` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSArray.cs)

#### `bool RemoveKey(ProtoCore.DSASM.StackValue key)`

Remove a key from array. Return true if key exsits.

**Parameters:**
- `key` (ProtoCore.DSASM.StackValue)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSArray.cs)

#### `ProtoCore.DSASM.StackValue SetValueForIndex(ProtoCore.DSASM.StackValue index, ProtoCore.DSASM.StackValue value, ProtoCore.RuntimeCore runtimeCore)`

array[index] = value. Here index can be any type. Note this function doesn't support the replication of array indexing.

**Parameters:**
- `index` (ProtoCore.DSASM.StackValue)
- `value` (ProtoCore.DSASM.StackValue)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSArray.cs)

#### `ProtoCore.DSASM.StackValue SetValueForIndex(int index, ProtoCore.DSASM.StackValue value, ProtoCore.RuntimeCore runtimeCore)`

array[index] = value. The array will be expanded if necessary.

**Parameters:**
- `index` (int)
- `value` (ProtoCore.DSASM.StackValue)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSArray.cs)

#### `ProtoCore.DSASM.StackValue SetValueForIndices(ProtoCore.DSASM.StackValue[] indices, ProtoCore.DSASM.StackValue value, ProtoCore.RuntimeCore runtimeCore)`

array[index1][index2][...][indexN] = value, and indices = {index1, index2, ..., indexN} Note this function doesn't support the replication of array indexing.

**Parameters:**
- `indices` (ProtoCore.DSASM.StackValue[])
- `value` (ProtoCore.DSASM.StackValue)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSArray.cs)

#### `ProtoCore.DSASM.StackValue SetValueForIndices(System.Collections.Generic.List<ProtoCore.DSASM.StackValue> indices, ProtoCore.DSASM.StackValue value, ProtoCore.RuntimeCore runtimeCore)`

array[index1][index2][...][indexN] = value, and indices = {index1, index2, ..., indexN}

**Parameters:**
- `indices` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)
- `value` (ProtoCore.DSASM.StackValue)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSArray.cs)

#### `System.Collections.Generic.IDictionary<ProtoCore.DSASM.StackValue, ProtoCore.DSASM.StackValue> ToDictionary()`

DSArray.ToDictionary

**Returns:** `System.Collections.Generic.IDictionary<ProtoCore.DSASM.StackValue, ProtoCore.DSASM.StackValue>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSArray.cs)

### Properties
- `Keys` (System.Collections.Generic.IEnumerable<ProtoCore.DSASM.StackValue>, get) — DSArray.Keys

## DSObject (class)

Type DSObject

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSObject.cs)

### Constructors
- `void DSObject(ProtoCore.DSASM.StackValue[] values, ProtoCore.DSASM.Heap heap)` — DSObject.DSObject
- `void DSObject(int size, ProtoCore.DSASM.Heap heap)` — DSObject.DSObject

### Methods
#### `void ExpandBySize(int size)`

Expand the memory by specified size so that the object can contain extra information. Exception ProtoCore.Exceptions.RunOutOfMemoryException

**Parameters:**
- `size` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSObject.cs)

#### `ProtoCore.DSASM.StackValue GetValueFromIndex(int index, ProtoCore.RuntimeCore runtimeCore)`

DSObject.GetValueFromIndex

**Parameters:**
- `index` (int)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSObject.cs)

#### `ProtoCore.DSASM.StackValue SetValueAtIndex(int index, ProtoCore.DSASM.StackValue value, ProtoCore.RuntimeCore runtimeCore)`

DSObject.SetValueAtIndex

**Parameters:**
- `index` (int)
- `value` (ProtoCore.DSASM.StackValue)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSObject.cs)

## DSString (class)

Type DSString

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSString.cs)

### Constructors
- `void DSString(ProtoCore.DSASM.StackValue[] values, ProtoCore.DSASM.Heap heap)` — DSString.DSString
- `void DSString(int size, ProtoCore.DSASM.Heap heap)` — DSString.DSString

### Methods
#### `bool Equals(object obj)`

DSString.Equals

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSString.cs)

#### `ProtoCore.DSASM.StackValue GetValueAtIndex(ProtoCore.DSASM.StackValue index, ProtoCore.RuntimeCore runtimeCore)`

DSString.GetValueAtIndex

**Parameters:**
- `index` (ProtoCore.DSASM.StackValue)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSString.cs)

#### `ProtoCore.DSASM.StackValue GetValueAtIndex(int index, ProtoCore.RuntimeCore runtimeCore)`

DSString.GetValueAtIndex

**Parameters:**
- `index` (int)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSString.cs)

#### `void SetPointer(ProtoCore.DSASM.StackValue pointer)`

DSString.SetPointer

**Parameters:**
- `pointer` (ProtoCore.DSASM.StackValue)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/DSString.cs)

### Properties
- `MemorySize` (int, get) — DSString.MemorySize
- `Value` (string, get) — DSString.Value

## DebugInfo (class)

Type DebugInfo

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

### Constructors
- `void DebugInfo(int line, int col, int eline, int ecol, string file)` — DebugInfo.DebugInfo

### Properties
- `Location` (ProtoCore.CodeModel.CodeRange, get/set) — DebugInfo.Location

## DyanmicVariableNode (struct)

Type DyanmicVariableNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/StringTable.cs)

### Constructors
- `void DyanmicVariableNode(string varName, int procIndex, int classIndex)` — DyanmicVariableNode.DyanmicVariableNode

## DynamicFunction (class)

It represents an unresolved function in the code. For any unresolved function, a DynamicFunction instance will be created and be added to DynamicFunctionTable. At runtime, callr will fetch the corresponding DynamicFunction instance from DynamicFunctionTable, and based on its name/argument number/class scope to resolves function dynamically.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DynamicFunctionTable.cs)

### Constructors
- `void DynamicFunction(string name, int argumentNumber, int classIndex)` — DynamicFunction.DynamicFunction

### Properties
- `ArgumentNumber` (int, get/set) — DynamicFunction.ArgumentNumber
- `ClassIndex` (int, get/set) — DynamicFunction.ClassIndex
- `Index` (int, get/set) — DynamicFunction.Index
- `Name` (string, get/set) — DynamicFunction.Name

## DynamicFunctionTable (class)

Type DynamicFunctionTable

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DynamicFunctionTable.cs)

### Constructors
- `void DynamicFunctionTable()` — DynamicFunctionTable.DynamicFunctionTable

### Methods
#### `ProtoCore.DSASM.DynamicFunction AddNewFunction(string name, int argumentNumber, int classIndex)`

DynamicFunctionTable.AddNewFunction

**Parameters:**
- `name` (string)
- `argumentNumber` (int)
- `classIndex` (int)

**Returns:** `ProtoCore.DSASM.DynamicFunction` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DynamicFunctionTable.cs)

#### `ProtoCore.DSASM.DynamicFunction GetFunctionAtIndex(int index)`

DynamicFunctionTable.GetFunctionAtIndex

**Parameters:**
- `index` (int)

**Returns:** `ProtoCore.DSASM.DynamicFunction` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DynamicFunctionTable.cs)

#### `bool TryGetFunction(string name, int argumentNumber, int classIndex, ref ProtoCore.DSASM.DynamicFunction func)`

DynamicFunctionTable.TryGetFunction

**Parameters:**
- `name` (string)
- `argumentNumber` (int)
- `classIndex` (int)
- `func` (ref ProtoCore.DSASM.DynamicFunction)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DynamicFunctionTable.cs)

## DynamicVariableTable (class)

Type DynamicVariableTable

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/StringTable.cs)

### Constructors
- `void DynamicVariableTable()` — DynamicVariableTable.DynamicVariableTable

### Properties
- `variableTable` (System.Collections.Generic.List<ProtoCore.DSASM.DyanmicVariableNode>, get/set) — DynamicVariableTable.variableTable

## Executable (class)

Executable holds the body of code that will be executed along with associated meta-information

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executable.cs)

### Constructors
- `void Executable()` — Executable.Executable

### Methods
#### `void Reset()`

Executable.Reset

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executable.cs)

### Properties
- `CodeBlocks` (System.Collections.Generic.List<ProtoCore.DSASM.CodeBlock>, get/set) — Executable.CodeBlocks
- `CodeToLocation` (System.Collections.Generic.Dictionary<ulong, ulong>, get/set) — Executable.CodeToLocation
- `Configurations` (System.Collections.Generic.IDictionary<string, object>, get/set) — Executable.Configurations
- `CurrentDSFileName` (string, get/set) — Executable.CurrentDSFileName
- `DynamicFuncTable` (ProtoCore.DSASM.DynamicFunctionTable, get/set) — Executable.DynamicFuncTable
- `DynamicVarTable` (ProtoCore.DSASM.DynamicVariableTable, get/set) — This is a mapping of the current guid and number of callsites that appear within that guid. Language only execution contains only 1 guid for the entire program. Execution within a visual programming host means 1 guid per node, where 1 node contains a set of DS code. Each of the callsite instances are mapped to a guid and an instance count.
- `ExecutingGraphnode` (ProtoCore.AssociativeGraph.GraphNode, get/set) — Executable.ExecutingGraphnode
- `FuncPointerTable` (ProtoCore.DSASM.FunctionPointerTable, get/set) — Executable.FuncPointerTable
- `FunctionTable` (ProtoCore.Lang.FunctionTable, get/set) — Executable.FunctionTable
- `TypeSystem` (ProtoCore.TypeSystem, get/set) — Executable.TypeSystem
- `UpdatedSymbols` (System.Collections.Generic.HashSet<ProtoCore.DSASM.SymbolNode>, get) — These are the list of symbols updated by the VM after an execution cycle
- `classTable` (ProtoCore.DSASM.ClassTable, get/set) — RuntimeData is set in the executable to isolate data passed to the runtime VM The RuntimeData will eventually be integrated completely into executable, this means moving RuntimeData properties to Executable and deprecating the RuntimeData object
- `iStreamCanvas` (ProtoCore.DSASM.InstructionStream, get/set) — Executable.iStreamCanvas
- `instrStreamList` (ProtoCore.DSASM.InstructionStream[], get/set) — Executable.instrStreamList
- `procedureTable` (ProtoCore.DSASM.ProcedureTable[], get/set) — Executable.procedureTable
- `runtimeSymbols` (ProtoCore.DSASM.SymbolTable[], get/set) — Executable.runtimeSymbols

## Executive (class)

Type Executive

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

### Constructors
- `void Executive(ProtoCore.RuntimeCore runtimeCore, bool isFep)` — Executive.Executive

### Methods
#### `ProtoCore.DSASM.StackValue Bounce(int exeblock, int entry, ProtoCore.DSASM.StackFrame stackFrame, int locals, bool fepRun, ProtoCore.DSASM.Executive exec, System.Collections.Generic.List<ProtoCore.DSASM.Instruction> breakpoints)`

Bounce instantiates a new Executive Execution jumps to the new executive This iverload handles debugger properties when bouncing

**Parameters:**
- `exeblock` (int) — 
- `entry` (int) — 
- `stackFrame` (ProtoCore.DSASM.StackFrame) — 
- `locals` (int) — 
- `fepRun` (bool) — 
- `exec` (ProtoCore.DSASM.Executive) — 
- `breakpoints` (System.Collections.Generic.List<ProtoCore.DSASM.Instruction>) — 

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `ProtoCore.DSASM.StackValue BounceUsingExecutive(ProtoCore.DSASM.Executive executive, int exeblock, int entry, ProtoCore.DSASM.StackFrame stackFrame, int locals, bool fepRun, ProtoCore.DSASM.Executive exec, System.Collections.Generic.List<ProtoCore.DSASM.Instruction> breakpoints)`

Bounce to an existing executive

**Parameters:**
- `executive` (ProtoCore.DSASM.Executive) — 
- `exeblock` (int) — 
- `entry` (int) — 
- `stackFrame` (ProtoCore.DSASM.StackFrame) — 
- `locals` (int) — 
- `fepRun` (bool) — 
- `exec` (ProtoCore.DSASM.Executive) — 
- `breakpoints` (System.Collections.Generic.List<ProtoCore.DSASM.Instruction>) — 

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `void CALLR_Handler(ProtoCore.DSASM.Instruction instruction)`

Executive.CALLR_Handler

**Parameters:**
- `instruction` (ProtoCore.DSASM.Instruction)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `ProtoCore.DSASM.StackValue Callr(int blockDeclId, int functionIndex, int classIndex, ref bool explicitCall, bool isDynamicCall, bool hasDebugInfo)`

Executive.Callr

**Parameters:**
- `blockDeclId` (int)
- `functionIndex` (int)
- `classIndex` (int)
- `explicitCall` (ref bool)
- `isDynamicCall` (bool)
- `hasDebugInfo` (bool)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `System.Collections.Generic.List<ProtoCore.DSASM.StackValue> CollectGCRoots()`

Executive.CollectGCRoots

**Returns:** `System.Collections.Generic.List<ProtoCore.DSASM.StackValue>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `void Execute(int exeblock, int entry, System.Collections.Generic.List<ProtoCore.DSASM.Instruction> breakpoints, ProtoCore.Language language)`

This is the VM execution entry function

**Parameters:**
- `exeblock` (int) — 
- `entry` (int) — 
- `breakpoints` (System.Collections.Generic.List<ProtoCore.DSASM.Instruction>) — 
- `language` (ProtoCore.Language) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `void GCCodeBlock(int blockId, int functionIndex, int classIndex)`

Executive.GCCodeBlock

**Parameters:**
- `blockId` (int)
- `functionIndex` (int)
- `classIndex` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `System.Collections.Generic.List<ProtoCore.AtLevel> GetCachedAtLevels(int argumentCount)`

Executive.GetCachedAtLevels

**Parameters:**
- `argumentCount` (int)

**Returns:** `System.Collections.Generic.List<ProtoCore.AtLevel>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `System.Collections.Generic.List<System.Collections.Generic.List<ProtoCore.ReplicationGuide>> GetCachedReplicationGuides(int argumentCount)`

Executive.GetCachedReplicationGuides

**Parameters:**
- `argumentCount` (int)

**Returns:** `System.Collections.Generic.List<System.Collections.Generic.List<ProtoCore.ReplicationGuide>>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `void GetCallerInformation(ref int classIndex, ref int functionIndex)`

Executive.GetCallerInformation

**Parameters:**
- `classIndex` (ref int)
- `functionIndex` (ref int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `ProtoCore.AssociativeGraph.GraphNode GetFirstGraphNode(string varName, ref int blockId)`

Executive.GetFirstGraphNode

**Parameters:**
- `varName` (string)
- `blockId` (ref int)

**Returns:** `ProtoCore.AssociativeGraph.GraphNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `ProtoCore.DSASM.StackValue GetIndexedArray(ProtoCore.DSASM.StackValue svArray, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> indices)`

Executive.GetIndexedArray

**Parameters:**
- `svArray` (ProtoCore.DSASM.StackValue)
- `indices` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `ProtoCore.DSASM.StackValue GetIndexedArray(System.Collections.Generic.List<ProtoCore.DSASM.StackValue> dims, int blockId, ProtoCore.DSASM.StackValue op1, ProtoCore.DSASM.StackValue op2)`

Executive.GetIndexedArray

**Parameters:**
- `dims` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)
- `blockId` (int)
- `op1` (ProtoCore.DSASM.StackValue)
- `op2` (ProtoCore.DSASM.StackValue)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `ProtoCore.DSASM.StackValue GetIndexedArrayW(int dimensions, int blockId, ProtoCore.DSASM.StackValue op1, ProtoCore.DSASM.StackValue op2)`

Executive.GetIndexedArrayW

**Parameters:**
- `dimensions` (int)
- `blockId` (int)
- `op1` (ProtoCore.DSASM.StackValue)
- `op2` (ProtoCore.DSASM.StackValue)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `ProtoCore.DSASM.ProcedureNode GetProcedureNode(int blockId, int classIndex, int functionIndex)`

Executive.GetProcedureNode

**Parameters:**
- `blockId` (int)
- `classIndex` (int)
- `functionIndex` (int)

**Returns:** `ProtoCore.DSASM.ProcedureNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `System.Collections.Generic.List<ProtoCore.DSASM.StackValue> GetRegisters()`

Executive.GetRegisters

**Returns:** `System.Collections.Generic.List<ProtoCore.DSASM.StackValue>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `ProtoCore.DSASM.SymbolNode GetSymbolNode(int blockId, int classIndex, int symbolIndex)`

Executive.GetSymbolNode

**Parameters:**
- `blockId` (int)
- `classIndex` (int)
- `symbolIndex` (int)

**Returns:** `ProtoCore.DSASM.SymbolNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `void Modify_istream_entrypoint_FromSetValue(int blockId, int pc)`

Executive.Modify_istream_entrypoint_FromSetValue

**Parameters:**
- `blockId` (int)
- `pc` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `void Modify_istream_instrList_FromSetValue(int blockId, int pc, ProtoCore.DSASM.StackValue op)`

Executive.Modify_istream_instrList_FromSetValue

**Parameters:**
- `blockId` (int)
- `pc` (int)
- `op` (ProtoCore.DSASM.StackValue)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `void POPM_Handler(ProtoCore.DSASM.Instruction instruction)`

Executive.POPM_Handler

**Parameters:**
- `instruction` (ProtoCore.DSASM.Instruction)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `ProtoCore.DSASM.StackValue POPM_Helper(ProtoCore.DSASM.Instruction instruction, ref int blockId, ref int classIndex)`

Executive.POPM_Helper

**Parameters:**
- `instruction` (ProtoCore.DSASM.Instruction)
- `blockId` (ref int)
- `classIndex` (ref int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `void POP_Handler(ProtoCore.DSASM.Instruction instruction)`

Executive.POP_Handler

**Parameters:**
- `instruction` (ProtoCore.DSASM.Instruction)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `ProtoCore.DSASM.StackValue POP_helper(ProtoCore.DSASM.Instruction instruction, ref int blockId, ref int dimensions)`

Executive.POP_helper

**Parameters:**
- `instruction` (ProtoCore.DSASM.Instruction)
- `blockId` (ref int)
- `dimensions` (ref int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `ProtoCore.InterpreterProperties PopInterpreterProps()`

Executive.PopInterpreterProps

**Returns:** `ProtoCore.InterpreterProperties` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `ProtoCore.DSASM.StackValue PopTo(int blockId, ProtoCore.DSASM.StackValue op1, ProtoCore.DSASM.StackValue op2, ProtoCore.DSASM.StackValue opVal)`

Executive.PopTo

**Parameters:**
- `blockId` (int)
- `op1` (ProtoCore.DSASM.StackValue)
- `op2` (ProtoCore.DSASM.StackValue)
- `opVal` (ProtoCore.DSASM.StackValue)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `ProtoCore.DSASM.StackValue PopToIndexedArray(int blockId, int symbol, int classIndex, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> dimlist, ProtoCore.DSASM.StackValue data)`

Executive.PopToIndexedArray

**Parameters:**
- `blockId` (int)
- `symbol` (int)
- `classIndex` (int)
- `dimlist` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)
- `data` (ProtoCore.DSASM.StackValue)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `void PushInterpreterProps(ProtoCore.InterpreterProperties properties)`

Executive.PushInterpreterProps

**Parameters:**
- `properties` (ProtoCore.InterpreterProperties)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `void ReturnSiteGC(int blockId, int classIndex, int functionIndex)`

Executive.ReturnSiteGC

**Parameters:**
- `blockId` (int)
- `classIndex` (int)
- `functionIndex` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `void SetupNextExecutableGraph(int function, int classscope)`

Executive.SetupNextExecutableGraph

**Parameters:**
- `function` (int)
- `classscope` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

#### `void runtimeVerify(bool condition, string msg)`

Executive.runtimeVerify

**Parameters:**
- `condition` (bool)
- `msg` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executive.cs)

### Properties
- `IsExplicitCall` (bool, get/set) — Executive.IsExplicitCall
- `PC` (int, get) — Executive.PC
- `Properties` (ProtoCore.InterpreterProperties, get/set) — Executive.Properties
- `RX` (ProtoCore.DSASM.StackValue, get/set) — Executive.RX
- `RuntimeCore` (ProtoCore.RuntimeCore, get) — Executive.RuntimeCore
- `TX` (ProtoCore.DSASM.StackValue, get/set) — Executive.TX
- `deferedGraphNodes` (System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode>, get) — Executive.deferedGraphNodes
- `exe` (ProtoCore.DSASM.Executable, get/set) — Executive.exe
- `rmem` (ProtoCore.Runtime.RuntimeMemory, get/set) — Executive.rmem

## ExecutiveProvider (class)

Type ExecutiveProvider

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/IExecutive.cs)

### Constructors
- `void ExecutiveProvider()` — ExecutiveProvider.ExecutiveProvider

### Methods
#### `ProtoCore.DSASM.Executive CreateExecutive(ProtoCore.RuntimeCore runtimeCore, bool isFep)`

ExecutiveProvider.CreateExecutive

**Parameters:**
- `runtimeCore` (ProtoCore.RuntimeCore)
- `isFep` (bool)

**Returns:** `ProtoCore.DSASM.Executive` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/IExecutive.cs)

## FunctionPointerNode (struct)

Type FunctionPointerNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/FunctionPointerTable.cs)

### Constructors
- `void FunctionPointerNode(ProtoCore.DSASM.ProcedureNode procNode)` — FunctionPointerNode.FunctionPointerNode

## FunctionPointerTable (class)

Type FunctionPointerTable

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/FunctionPointerTable.cs)

### Constructors
- `void FunctionPointerTable()` — FunctionPointerTable.FunctionPointerTable

### Methods
#### `bool TryGetFunction(ProtoCore.DSASM.StackValue functionPointer, ProtoCore.RuntimeCore runtimeCore, ref ProtoCore.DSASM.ProcedureNode procNode)`

Try to get the original procedure node that the function pointer points to.

**Parameters:**
- `functionPointer` (ProtoCore.DSASM.StackValue) — Function pointer
- `runtimeCore` (ProtoCore.RuntimeCore) — Core
- `procNode` (ref ProtoCore.DSASM.ProcedureNode) — Procedure node

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/FunctionPointerTable.cs)

### Properties
- `functionPointerDictionary` (ProtoCore.DSASM.BiDictionaryOneToOne<int, ProtoCore.DSASM.FunctionPointerNode>, get/set) — FunctionPointerTable.functionPointerDictionary

## Heap (class)

Type Heap

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Heap.cs)

### Constructors
- `void Heap()` — Heap.Heap

### Methods
#### `ProtoCore.DSASM.StackValue AllocateArray(ProtoCore.DSASM.StackValue[] values)`

Allocate an array. Exceptions: ProtoCore.Exceptions.RunOutOfMemoryException

**Parameters:**
- `values` (ProtoCore.DSASM.StackValue[]) — Array elements whose indices are integer

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Heap.cs)

#### `ProtoCore.DSASM.StackValue AllocateFixedString(string str)`

Allocate string constant. String constant won't be garbage collected.

**Parameters:**
- `str` (string) — 

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Heap.cs)

#### `ProtoCore.DSASM.StackValue AllocatePointer(ProtoCore.DSASM.StackValue[] values)`

Allocate an object pointer. Exceptions: ProtoCore.Exceptions.RunOutOfMemoryException

**Parameters:**
- `values` (ProtoCore.DSASM.StackValue[]) — Values of object properties

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Heap.cs)

#### `ProtoCore.DSASM.StackValue AllocatePointer(int size, ProtoCore.DSASM.MetaData metadata)`

Allocate an object pointer. Exceptions: ProtoCore.Exceptions.RunOutOfMemoryException

**Parameters:**
- `size` (int) — The size of object properties.
- `metadata` (ProtoCore.DSASM.MetaData) — Object type

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Heap.cs)

#### `ProtoCore.DSASM.StackValue AllocateString(string str)`

Allocate string.

**Parameters:**
- `str` (string) — 

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Heap.cs)

#### `void Free()`

Heap.Free

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Heap.cs)

#### `void FullGC(System.Collections.Generic.IEnumerable<ProtoCore.DSASM.StackValue> gcroots, ProtoCore.DSASM.Executive exe)`

Do a full GC cycle

**Parameters:**
- `gcroots` (System.Collections.Generic.IEnumerable<ProtoCore.DSASM.StackValue>) — 
- `exe` (ProtoCore.DSASM.Executive) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Heap.cs)

#### `void GC()`

GC

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Heap.cs)

#### `string GetString(ProtoCore.DSASM.DSString dsString)`

Returns string that pointer represents.

**Parameters:**
- `dsString` (ProtoCore.DSASM.DSString) — 

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Heap.cs)

#### `void ReportAllocation(int newSize)`

Heap.ReportAllocation

**Parameters:**
- `newSize` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Heap.cs)

#### `bool SetRoots(System.Collections.Generic.IEnumerable<ProtoCore.DSASM.StackValue> gcroots, ProtoCore.DSASM.Executive exe)`

Notify the heap that gc roots are ready so that gc could move forward. The executive is passed for dispoing objects.

**Parameters:**
- `gcroots` (System.Collections.Generic.IEnumerable<ProtoCore.DSASM.StackValue>) — 
- `exe` (ProtoCore.DSASM.Executive) — 

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Heap.cs)

#### `T ToHeapObject<T>(ProtoCore.DSASM.StackValue heapObject)`

Heap.ToHeapObject

**Parameters:**
- `heapObject` (ProtoCore.DSASM.StackValue)

**Returns:** `T` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Heap.cs)

#### `void WriteBarrierForward(ProtoCore.DSASM.HeapElement hp, ProtoCore.DSASM.StackValue value)`

If the heap object is modified, mark the new value that it references to.

**Parameters:**
- `hp` (ProtoCore.DSASM.HeapElement) — Heap object that is to be modified
- `value` (ProtoCore.DSASM.StackValue) — The value that will be put in the heap object

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Heap.cs)

### Properties
- `IsGCRunning` (bool, get) — Heap.IsGCRunning
- `IsWaitingForRoots` (bool, get) — Returns true if the heap is waiting for GC root objects.

## HeapElement (class)

Type HeapElement

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Heap.cs)

### Constructors
- `void HeapElement(ProtoCore.DSASM.StackValue[] values, ProtoCore.DSASM.Heap heap)` — Create HeapElement based on the existing values
- `void HeapElement(int size, ProtoCore.DSASM.Heap heap)` — Create HeapElement

### Methods
#### `void CollectElementsForGC(System.Collections.Generic.Queue<ProtoCore.DSASM.StackValue> gcQueue)`

Enqueue all reference-typed elements for garbage collection. Note: This is used by the heap manager to trace reachable objects during GC.

**Parameters:**
- `gcQueue` (System.Collections.Generic.Queue<ProtoCore.DSASM.StackValue>) — Queue to enqueue reference-typed stack values

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Heap.cs)

#### `int ExpandByAcessingAt(int index)`

HeapElement.ExpandByAcessingAt

**Parameters:**
- `index` (int)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Heap.cs)

#### `ProtoCore.DSASM.StackValue GetValueAt(int index)`

HeapElement.GetValueAt

**Parameters:**
- `index` (int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Heap.cs)

#### `void SetValueAt(int index, ProtoCore.DSASM.StackValue value)`

HeapElement.SetValueAt

**Parameters:**
- `index` (int)
- `value` (ProtoCore.DSASM.StackValue)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Heap.cs)

### Properties
- `Count` (int, get) — HeapElement.Count
- `Mark` (ProtoCore.DSASM.Heap.GCMark, get/set) — HeapElement.Mark
- `MemorySize` (int, get) — HeapElement.MemorySize
- `MetaData` (ProtoCore.DSASM.MetaData, get/set) — HeapElement.MetaData
- `Values` (System.Collections.Generic.IEnumerable<ProtoCore.DSASM.StackValue>, get) — HeapElement.Values

## IExecutiveProvider (interface)

Type IExecutiveProvider

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/IExecutive.cs)

### Methods
#### `ProtoCore.DSASM.Executive CreateExecutive(ProtoCore.RuntimeCore runtimeCore, bool isFep)`

IExecutiveProvider.CreateExecutive

**Parameters:**
- `runtimeCore` (ProtoCore.RuntimeCore)
- `isFep` (bool)

**Returns:** `ProtoCore.DSASM.Executive` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/IExecutive.cs)

## Instruction (class)

Type Instruction

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

### Constructors
- `void Instruction()` — Instruction.Instruction

## InstructionStream (class)

InstructionStream holds the executable dsasm code and relevant information

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Executable.cs)

### Constructors
- `void InstructionStream(ProtoCore.Language langId, ProtoCore.Core core)` — InstructionStream.InstructionStream

### Properties
- `dependencyGraph` (ProtoCore.AssociativeGraph.DependencyGraph, get/set) — InstructionStream.dependencyGraph
- `entrypoint` (int, get/set) — InstructionStream.entrypoint
- `instrList` (System.Collections.Generic.List<ProtoCore.DSASM.Instruction>, get/set) — InstructionStream.instrList
- `language` (ProtoCore.Language, get/set) — InstructionStream.language
- `xUpdateList` (System.Collections.Generic.List<ProtoCore.AssociativeGraph.UpdateNodeRef>, get/set) — InstructionStream.xUpdateList

## Interpreter (class)

Type Interpreter

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Interpreter.cs)

### Constructors
- `void Interpreter(ProtoCore.RuntimeCore runtimeCore, bool isFEP)` — Interpreter.Interpreter

### Methods
#### `void Push(ProtoCore.DSASM.StackValue val)`

Interpreter.Push

**Parameters:**
- `val` (ProtoCore.DSASM.StackValue)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Interpreter.cs)

#### `ProtoCore.DSASM.StackValue Run(int codeblock, int entry, ProtoCore.Language lang, System.Collections.Generic.List<ProtoCore.DSASM.Instruction> breakpoints)`

Interpreter.Run

**Parameters:**
- `codeblock` (int)
- `entry` (int)
- `lang` (ProtoCore.Language)
- `breakpoints` (System.Collections.Generic.List<ProtoCore.DSASM.Instruction>)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Interpreter.cs)

## InterpreterMode (enum)

Type InterpreterMode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Defs.cs)

### Values
- `Expression` = `0`
- `Normal` = `1`

## Literal (struct)

Type Literal

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Defs.cs)

## MemoryRegion (enum)

Type MemoryRegion

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Defs.cs)

### Values
- `InvalidRegion` = `-1`
- `MemStack` = `1`
- `MemStatic` = `0`

## MetaData (struct)

Type MetaData

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

## Op (class)

Translate an operator to other representations.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Defs.cs)

### Constructors
- `void Op()` — Op.Op

### Methods
#### `ProtoCore.DSASM.OpCode GetOpCode(ProtoCore.DSASM.Operator op)`

Returns the corresponding opcode of an operator.

**Parameters:**
- `op` (ProtoCore.DSASM.Operator) — 

**Returns:** `ProtoCore.DSASM.OpCode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Defs.cs)

#### `string GetOpFunction(ProtoCore.DSASM.Operator op)`

Returns the internal function name for operator.

**Parameters:**
- `op` (ProtoCore.DSASM.Operator) — 

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Defs.cs)

#### `string GetOpName(ProtoCore.DSASM.Operator op)`

Returns the string representation of an operator. E.g., return "add" for Operator.add.

**Parameters:**
- `op` (ProtoCore.DSASM.Operator) — 

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Defs.cs)

#### `string GetOpSymbol(ProtoCore.DSASM.Operator op)`

Returns the symbol representation of an operator. E.g., return "+" for Operator.add.

**Parameters:**
- `op` (ProtoCore.DSASM.Operator) — Operator

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Defs.cs)

#### `ProtoCore.DSASM.OpCode GetUnaryOpCode(ProtoCore.DSASM.UnaryOperator op)`

Returns the corresponding opcode of an unary operator.

**Parameters:**
- `op` (ProtoCore.DSASM.UnaryOperator) — 

**Returns:** `ProtoCore.DSASM.OpCode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Defs.cs)

#### `string GetUnaryOpFunction(ProtoCore.DSASM.UnaryOperator op)`

Returns the internal function name for unary operator

**Parameters:**
- `op` (ProtoCore.DSASM.UnaryOperator) — 

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Defs.cs)

#### `string GetUnaryOpName(ProtoCore.DSASM.UnaryOperator op)`

Returns the string representation of an unary operator. E.g., return "not" for UnaryOperator.not.

**Parameters:**
- `op` (ProtoCore.DSASM.UnaryOperator) — 

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Defs.cs)

#### `string GetUnaryOpSymbol(ProtoCore.DSASM.UnaryOperator op)`

Returns the symbol representation of an unary operator. E.g., return "-" for UnaryOperator.neg

**Parameters:**
- `op` (ProtoCore.DSASM.UnaryOperator) — 

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Defs.cs)

## OpCode (enum)

Type OpCode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

### Values
- `ADD` = `1`
- `AND` = `7`
- `BOUNCE` = `38`
- `CALL` = `33`
- `CALLR` = `34`
- `CAST` = `32`
- `CJMP` = `40`
- `DEP` = `42`
- `DIV` = `4`
- `EQ` = `10`
- `GE` = `14`
- `GT` = `12`
- `JDEP` = `41`
- `JMP` = `39`
- `LE` = `15`
- `LOADELEMENT` = `29`
- `LT` = `13`
- `MOD` = `5`
- `MUL` = `3`
- `NEG` = `6`
- `NEWARR` = `16`
- `NEWOBJ` = `17`
- `NONE` = `0`
- `NOT` = `9`
- `NQ` = `11`
- `OR` = `8`
- `POP` = `25`
- `POPM` = `27`
- `POPREPGUIDES` = `28`
- `POPW` = `26`
- `PUSH` = `18`
- `PUSHBLOCK` = `19`
- `PUSHDEP` = `22`
- `PUSHLEVEL` = `24`
- `PUSHM` = `20`
- `PUSHREPGUIDE` = `23`
- `PUSHW` = `21`
- `PUSH_ARRAYKEY` = `43`
- `RETB` = `36`
- `RETCN` = `37`
- `RETURN` = `35`
- `SETELEMENT` = `30`
- `SETEXPUID` = `44`
- `SETMEMElEMENT` = `31`
- `SUB` = `2`

## Operator (enum)

Type Operator

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Defs.cs)

### Values
- `add` = `8`
- `and` = `13`
- `assign` = `1`
- `bitwiseand` = `16`
- `bitwisenegate` = `19`
- `bitwiseor` = `17`
- `bitwisexor` = `18`
- `div` = `11`
- `dot` = `15`
- `eq` = `6`
- `ge` = `5`
- `gt` = `3`
- `le` = `4`
- `lt` = `2`
- `mod` = `12`
- `mul` = `10`
- `none` = `0`
- `nq` = `7`
- `or` = `14`
- `sub` = `9`

## ProcedureDistance (enum)

Type ProcedureDistance

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ProcedureTable.cs)

### Values
- `CoerceBaseClass` = `40`
- `CoerceDoubleToIntScore` = `5`
- `CoerceIntToDoubleScore` = `30`
- `CoerceScore` = `10`
- `ExactMatchDistance` = `0`
- `ExactMatchScore` = `50`
- `InvalidDistance` = `-1`
- `MaxDistance` = `2147483647`
- `NotMatchScore` = `0`

## ProcedureNode (class)

Type ProcedureNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ProcedureTable.cs)

### Constructors
- `void ProcedureNode()` — ProcedureNode.ProcedureNode
- `void ProcedureNode(ProtoCore.DSASM.ProcedureNode rhs)` — ProcedureNode.ProcedureNode

### Methods
#### `bool Equals(object obj)`

ProcedureNode.Equals

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ProcedureTable.cs)

### Properties
- `AccessModifier` (ProtoCore.CompilerDefinitions.AccessModifier, get/set) — ProcedureNode.AccessModifier
- `ArgumentInfos` (System.Collections.Generic.List<ProtoCore.DSASM.ArgumentInfo>, get/set) — List of arguments' information (default value)
- `ArgumentTypes` (System.Collections.Generic.List<ProtoCore.Type>, get/set) — List of arguments required by the procedure
- `ChildCodeBlocks` (System.Collections.Generic.List<int>, get/set) — ProcedureNode.ChildCodeBlocks
- `ClassID` (int, get/set) — Index of the class that the procedure belongs to
- `GraphNodeList` (System.Collections.Generic.List<ProtoCore.AssociativeGraph.GraphNode>, get) — The list of graphnodes that this function owns
- `HashID` (int, get/set) — The hash of the function given the name and argument type string
- `ID` (int, get/set) — Index of the procedure in its procedure table
- `IsActive` (bool, get/set) — ProcedureNode.IsActive
- `IsAssocOperator` (bool, get/set) — ProcedureNode.IsAssocOperator
- `IsAutoGenerated` (bool, get/set) — ProcedureNode.IsAutoGenerated
- `IsAutoGeneratedThisProc` (bool, get/set) — ProcedureNode.IsAutoGeneratedThisProc
- `IsConstructor` (bool, get/set) — Flag whether procedure is a constructor or not
- `IsExternal` (bool, get/set) — ProcedureNode.IsExternal
- `IsStatic` (bool, get/set) — Flag whether procedure is a static function or not
- `IsVarArg` (bool, get/set) — ProcedureNode.IsVarArg
- `LocalCount` (int, get/set) — Number of local variables
- `MethodAttribute` (ProtoCore.AST.AssociativeAST.MethodAttributes, get/set) — ProcedureNode.MethodAttribute
- `Name` (string, get/set) — Name of the procedure
- `PC` (int, get/set) — First instruction of the procedure
- `ReturnType` (ProtoCore.Type, get/set) — Procedure return data type
- `RuntimeIndex` (int, get/set) — Index of the procedure at the runtime executable tables

## ProcedureTable (class)

Type ProcedureTable

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ProcedureTable.cs)

### Constructors
- `void ProcedureTable(ProtoCore.DSASM.ProcedureTable rhs)` — Create an empty procedure table.
- `void ProcedureTable(int runtimeindex)` — Create an empty procedure table.

### Methods
#### `int Append(ProtoCore.DSASM.ProcedureNode node)`

ProcedureTable.Append

**Parameters:**
- `node` (ProtoCore.DSASM.ProcedureNode)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ProcedureTable.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.DSASM.ProcedureNode> GetFunctionsByName(string name)`

Returns function with specified name.

**Parameters:**
- `name` (string) — 

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.DSASM.ProcedureNode>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ProcedureTable.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.DSASM.ProcedureNode> GetFunctionsByNameAndArgumentNumber(string name, int argumentNumber)`

Returns all functions with specified name and the number of argument. It also returns function that has default arguments but the total parameter number is larger that the specified argument number.

**Parameters:**
- `name` (string) — 
- `argumentNumber` (int) — 

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.DSASM.ProcedureNode>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ProcedureTable.cs)

#### `ProtoCore.DSASM.ProcedureNode GetPropertyGetterByName(string name)`

Returns getter function for given property name. Ex: for X property of Point, this method returns get_X(Point p).

**Parameters:**
- `name` (string) — 

**Returns:** `ProtoCore.DSASM.ProcedureNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/ProcedureTable.cs)

### Properties
- `Procedures` (System.Collections.Generic.List<ProtoCore.DSASM.ProcedureNode>, get) — All procedure nodes defined in this procedure table.
- `RuntimeIndex` (int, get) — The index of this procedure table in the executable's procedure table list.

## RangeStepOperator (enum)

Type RangeStepOperator

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Defs.cs)

### Values
- `ApproximateSize` = `2`
- `Number` = `1`
- `StepSize` = `0`

## Registers (enum)

Type Registers

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

### Values
- `LX` = `1`
- `RX` = `0`

## StackAlignToFramePointerRestorer (class)

Type StackAlignToFramePointerRestorer

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Stack.cs)

### Constructors
- `void StackAlignToFramePointerRestorer()` — StackAlignToFramePointerRestorer.StackAlignToFramePointerRestorer

### Methods
#### `void Align(ProtoCore.Runtime.RuntimeMemory rMem)`

StackAlignToFramePointerRestorer.Align

**Parameters:**
- `rMem` (ProtoCore.Runtime.RuntimeMemory)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Stack.cs)

#### `bool Restore()`

StackAlignToFramePointerRestorer.Restore

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Stack.cs)

## StackFrame (class)

Stack frame.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Stack.cs)

### Constructors
- `void StackFrame(ProtoCore.DSASM.StackValue thisPtr, int classIndex, int functionIndex, int returnAddress, int functionBlockIndex, int callerBlockIndex, ProtoCore.DSASM.StackFrameType callerStackFrameType, ProtoCore.DSASM.StackFrameType stackFrameType, int depth, int framePointer, int blockIndex, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> registers, int executionStateSize)` — StackFrame.StackFrame
- `void StackFrame(ProtoCore.DSASM.StackValue[] frame)` — StackFrame.StackFrame
- `void StackFrame(int globalOffset)` — StackFrame.StackFrame

### Methods
#### `System.Collections.Generic.List<ProtoCore.DSASM.StackValue> GetRegisters()`

StackFrame.GetRegisters

**Returns:** `System.Collections.Generic.List<ProtoCore.DSASM.StackValue>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Stack.cs)

### Properties
- `BlockIndex` (ProtoCore.DSASM.StackValue, get/set) — StackFrame.BlockIndex
- `CallerStackFrameType` (ProtoCore.DSASM.StackFrameType, get/set) — StackFrame.CallerStackFrameType
- `ClassScope` (int, get/set) — StackFrame.ClassScope
- `Depth` (int, get/set) — StackFrame.Depth
- `ExecutionStateSize` (int, get/set) — StackFrame.ExecutionStateSize
- `ExecutionStates` (ProtoCore.DSASM.StackValue[], get/set) — StackFrame.ExecutionStates
- `Frame` (ProtoCore.DSASM.StackValue[], get/set) — StackFrame.Frame
- `FramePointer` (int, get/set) — StackFrame.FramePointer
- `FunctionBlock` (int, get/set) — StackFrame.FunctionBlock
- `FunctionCallerBlock` (int, get/set) — StackFrame.FunctionCallerBlock
- `FunctionScope` (int, get/set) — StackFrame.FunctionScope
- `RX` (ProtoCore.DSASM.StackValue, get/set) — StackFrame.RX
- `ReturnPC` (int, get/set) — StackFrame.ReturnPC
- `StackFrameType` (ProtoCore.DSASM.StackFrameType, get/set) — StackFrame.StackFrameType
- `TX` (ProtoCore.DSASM.StackValue, get/set) — StackFrame.TX
- `ThisPtr` (ProtoCore.DSASM.StackValue, get/set) — StackFrame.ThisPtr

## StackFrameType (enum)

Stack frame type.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Stack.cs)

### Values
- `Function` = `0`
- `LanguageBlock` = `1`

## StackUtils (static-class)

Type StackUtils

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Stack.cs)

### Methods
#### `bool CompareStackValues(ProtoCore.DSASM.StackValue sv1, ProtoCore.DSASM.StackValue sv2, ProtoCore.RuntimeCore runtimeCore)`

Deep comparison for two StackValue.

**Parameters:**
- `sv1` (ProtoCore.DSASM.StackValue) — 
- `sv2` (ProtoCore.DSASM.StackValue) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Stack.cs)

#### `bool CompareStackValues(ProtoCore.DSASM.StackValue sv1, ProtoCore.DSASM.StackValue sv2, ProtoCore.RuntimeCore rtCore1, ProtoCore.RuntimeCore rtCore2, ProtoCore.Runtime.Context context)`

Deep comparison for two StackValue.

**Parameters:**
- `sv1` (ProtoCore.DSASM.StackValue) — 
- `sv2` (ProtoCore.DSASM.StackValue) — 
- `rtCore1` (ProtoCore.RuntimeCore)
- `rtCore2` (ProtoCore.RuntimeCore)
- `context` (ProtoCore.Runtime.Context)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Stack.cs)

## StackValue (struct)

Type StackValue

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

### Methods
#### `ProtoCore.DSASM.StackValue BuildArrayDimension(int dimension)`

StackValue.BuildArrayDimension

**Parameters:**
- `dimension` (int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildArrayKey(ProtoCore.DSASM.StackValue array, int index)`

StackValue.BuildArrayKey

**Parameters:**
- `array` (ProtoCore.DSASM.StackValue)
- `index` (int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildArrayKey(int arrayPtr, int index)`

StackValue.BuildArrayKey

**Parameters:**
- `arrayPtr` (int)
- `index` (int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildArrayPointer(long data)`

StackValue.BuildArrayPointer

**Parameters:**
- `data` (long)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildBlockIndex(int blockIndex)`

StackValue.BuildBlockIndex

**Parameters:**
- `blockIndex` (int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildBoolean(bool b)`

StackValue.BuildBoolean

**Parameters:**
- `b` (bool)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildCallingConversion(int data)`

StackValue.BuildCallingConversion

**Parameters:**
- `data` (int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildChar(char ch)`

StackValue.BuildChar

**Parameters:**
- `ch` (char)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildClassIndex(int classIndex)`

StackValue.BuildClassIndex

**Parameters:**
- `classIndex` (int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildDefaultArgument()`

StackValue.BuildDefaultArgument

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildDouble(double data)`

StackValue.BuildDouble

**Parameters:**
- `data` (double)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildDynamic(int dynamic)`

StackValue.BuildDynamic

**Parameters:**
- `dynamic` (int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildDynamicBlock(int block)`

StackValue.BuildDynamicBlock

**Parameters:**
- `block` (int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildExplicitCall(int pc)`

StackValue.BuildExplicitCall

**Parameters:**
- `pc` (int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildFrameType(int type)`

StackValue.BuildFrameType

**Parameters:**
- `type` (int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildFunctionIndex(int functionIndex)`

StackValue.BuildFunctionIndex

**Parameters:**
- `functionIndex` (int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildFunctionPointer(int fptr)`

StackValue.BuildFunctionPointer

**Parameters:**
- `fptr` (int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildInt(long data)`

StackValue.BuildInt

**Parameters:**
- `data` (long)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildInvalid()`

StackValue.BuildInvalid

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `System.Collections.Generic.List<ProtoCore.DSASM.StackValue> BuildInvalidRegisters()`

StackValue.BuildInvalidRegisters

**Returns:** `System.Collections.Generic.List<ProtoCore.DSASM.StackValue>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildLabelIndex(int labelIndex)`

StackValue.BuildLabelIndex

**Parameters:**
- `labelIndex` (int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildMemVarIndex(int varIndex)`

StackValue.BuildMemVarIndex

**Parameters:**
- `varIndex` (int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildNull()`

StackValue.BuildNull

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildPointer(long data)`

StackValue.BuildPointer

**Parameters:**
- `data` (long)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildPointer(long data, ProtoCore.DSASM.MetaData mdata)`

StackValue.BuildPointer

**Parameters:**
- `data` (long)
- `mdata` (ProtoCore.DSASM.MetaData)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildRegister(ProtoCore.DSASM.Registers reg)`

StackValue.BuildRegister

**Parameters:**
- `reg` (ProtoCore.DSASM.Registers)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildReplicationGuide(int guide)`

StackValue.BuildReplicationGuide

**Parameters:**
- `guide` (int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildStaticMemVarIndex(int varIndex)`

StackValue.BuildStaticMemVarIndex

**Parameters:**
- `varIndex` (int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildStaticType(int UID, int rank)`

StackValue.BuildStaticType

**Parameters:**
- `UID` (int)
- `rank` (int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildString(long data)`

StackValue.BuildString

**Parameters:**
- `data` (long)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildString(string str, ProtoCore.DSASM.Heap heap)`

StackValue.BuildString

**Parameters:**
- `str` (string)
- `heap` (ProtoCore.DSASM.Heap)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildThisPtr(int thisptr)`

StackValue.BuildThisPtr

**Parameters:**
- `thisptr` (int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BuildVarIndex(int varIndex)`

StackValue.BuildVarIndex

**Parameters:**
- `varIndex` (int)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue BulildInvalid()`

StackValue.BulildInvalid

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `bool Equals(object other)`

StackValue.Equals

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `int GetHashCode()`

StackValue.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue GetNextKey(ProtoCore.RuntimeCore runtimeCore)`

Returns an array's next key

**Parameters:**
- `runtimeCore` (ProtoCore.RuntimeCore) — 

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue ShallowClone()`

Although StackValue is a struct and assignment creates a copy of StackValue on stack, ShallowClone() has an explicit meaning to do copy.

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue ToBoolean(ProtoCore.RuntimeCore runtimeCore)`

Convert StackValue to boolean typed StackValue. Returns StackValue.Null if not able to do conversion.

**Parameters:**
- `runtimeCore` (ProtoCore.RuntimeCore) — 

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue ToDouble()`

Convert numeric typed StackValue to double typed StackValue. For other types, returns StackValue.Null.

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `ProtoCore.DSASM.StackValue ToInteger()`

Convert numeric typed StackValue to integer typed StackValue. For other types, returns StackValue.Null.

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

#### `bool TryGetArrayKey(ref ProtoCore.DSASM.StackValue array, ref int key)`

Try to get the host array and key value from StackValue. The address type of StackValue should be AddressType.ArrayKey.

**Parameters:**
- `array` (ref ProtoCore.DSASM.StackValue) — 
- `key` (ref int) — 

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/InstructionSet.cs)

### Properties
- `ArrayDimension` (int, get) — StackValue.ArrayDimension
- `ArrayKeyIndex` (int, get) — StackValue.ArrayKeyIndex
- `ArrayPointer` (int, get) — StackValue.ArrayPointer
- `BlockIndex` (int, get) — StackValue.BlockIndex
- `BooleanValue` (bool, get) — StackValue.BooleanValue
- `BounceType` (ProtoCore.DSASM.CallingConvention.BounceType, get) — StackValue.BounceType
- `CallType` (ProtoCore.DSASM.CallingConvention.CallType, get) — StackValue.CallType
- `CharValue` (long, get) — StackValue.CharValue
- `ClassIndex` (int, get) — StackValue.ClassIndex
- `DoubleValue` (double, get) — StackValue.DoubleValue
- `DynamicIndex` (int, get) — StackValue.DynamicIndex
- `ExplicitCallEntry` (int, get) — StackValue.ExplicitCallEntry
- `FrameType` (ProtoCore.DSASM.StackFrameType, get) — StackValue.FrameType
- `FunctionIndex` (int, get) — StackValue.FunctionIndex
- `FunctionPointer` (int, get) — StackValue.FunctionPointer
- `IntegerValue` (long, get) — StackValue.IntegerValue
- `IsArray` (bool, get) — StackValue.IsArray
- `IsArrayDimension` (bool, get) — StackValue.IsArrayDimension
- `IsArrayKey` (bool, get) — StackValue.IsArrayKey
- `IsBlockIndex` (bool, get) — StackValue.IsBlockIndex
- `IsBoolean` (bool, get) — StackValue.IsBoolean
- `IsCallingConvention` (bool, get) — StackValue.IsCallingConvention
- `IsChar` (bool, get) — StackValue.IsChar
- `IsClassIndex` (bool, get) — StackValue.IsClassIndex
- `IsDefaultArgument` (bool, get) — StackValue.IsDefaultArgument
- `IsDouble` (bool, get) — StackValue.IsDouble
- `IsDynamic` (bool, get) — StackValue.IsDynamic
- `IsExplicitCall` (bool, get) — StackValue.IsExplicitCall
- `IsFrameType` (bool, get) — StackValue.IsFrameType
- `IsFunctionIndex` (bool, get) — StackValue.IsFunctionIndex
- `IsFunctionPointer` (bool, get) — StackValue.IsFunctionPointer
- `IsInteger` (bool, get) — StackValue.IsInteger
- `IsInvalid` (bool, get) — StackValue.IsInvalid
- `IsLabelIndex` (bool, get) — StackValue.IsLabelIndex
- `IsMemberVariableIndex` (bool, get) — StackValue.IsMemberVariableIndex
- `IsNull` (bool, get) — StackValue.IsNull
- `IsNumeric` (bool, get) — StackValue.IsNumeric
- `IsPointer` (bool, get) — StackValue.IsPointer
- `IsReferenceType` (bool, get) — StackValue.IsReferenceType
- `IsRegister` (bool, get) — StackValue.IsRegister
- `IsReplicationGuide` (bool, get) — StackValue.IsReplicationGuide
- `IsStaticType` (bool, get) — StackValue.IsStaticType
- `IsStaticVariableIndex` (bool, get) — StackValue.IsStaticVariableIndex
- `IsString` (bool, get) — StackValue.IsString
- `IsThisPtr` (bool, get) — StackValue.IsThisPtr
- `IsVariableIndex` (bool, get) — StackValue.IsVariableIndex
- `LabelIndex` (int, get) — StackValue.LabelIndex
- `MemberVariableIndex` (int, get) — StackValue.MemberVariableIndex
- `Pointer` (int, get) — StackValue.Pointer
- `Rank` (int, get) — StackValue.Rank
- `RawData` (long, get) — Returns raw data without checking its type or do type conversion. Use with caution.
- `Register` (ProtoCore.DSASM.Registers, get) — StackValue.Register
- `ReplicationGuide` (int, get) — StackValue.ReplicationGuide
- `StaticVariableIndex` (int, get) — StackValue.StaticVariableIndex
- `StringPointer` (int, get) — StackValue.StringPointer
- `SymbolIndex` (int, get) — StackValue.SymbolIndex
- `ThisPtr` (int, get) — StackValue.ThisPtr
- `VariableIndex` (int, get) — StackValue.VariableIndex

## StackValueComparer (class)

Type StackValueComparer

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Heap.cs)

### Constructors
- `void StackValueComparer(ProtoCore.RuntimeCore runtimeCore)` — StackValueComparer.StackValueComparer

### Methods
#### `bool Equals(ProtoCore.DSASM.StackValue x, ProtoCore.DSASM.StackValue y)`

StackValueComparer.Equals

**Parameters:**
- `x` (ProtoCore.DSASM.StackValue)
- `y` (ProtoCore.DSASM.StackValue)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Heap.cs)

#### `int GetHashCode(ProtoCore.DSASM.StackValue value)`

StackValueComparer.GetHashCode

**Parameters:**
- `value` (ProtoCore.DSASM.StackValue)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Heap.cs)

## SymbolNode (class)

Type SymbolNode

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SymbolTable.cs)

### Constructors
- `void SymbolNode()` — SymbolNode.SymbolNode
- `void SymbolNode(string name, int index, int functionIndex, ProtoCore.Type datatype, bool isArgument, int runtimeIndex, ProtoCore.DSASM.MemoryRegion memregion, int scope, ProtoCore.CompilerDefinitions.AccessModifier access, bool isStatic, int codeBlockId)` — SymbolNode.SymbolNode

### Methods
#### `bool Equals(object obj)`

SymbolNode.Equals

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/SymbolTable.cs)

## SymbolTable (class)

Type SymbolTable

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Namespace/SymbolTable.cs)

### Constructors
- `void SymbolTable(string scopeName, int runtimeIndex)` — SymbolTable.SymbolTable

### Methods
#### `int Append(ProtoCore.DSASM.SymbolNode node)`

SymbolTable.Append

**Parameters:**
- `node` (ProtoCore.DSASM.SymbolNode)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Namespace/SymbolTable.cs)

#### `int GetGlobalSize()`

SymbolTable.GetGlobalSize

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Namespace/SymbolTable.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.DSASM.SymbolNode> GetNodeForName(string name)`

SymbolTable.GetNodeForName

**Parameters:**
- `name` (string)

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.DSASM.SymbolNode>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Namespace/SymbolTable.cs)

#### `int IndexOf(ProtoCore.DSASM.SymbolNode symbol)`

SymbolTable.IndexOf

**Parameters:**
- `symbol` (ProtoCore.DSASM.SymbolNode)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Namespace/SymbolTable.cs)

#### `int IndexOf(string name)`

SymbolTable.IndexOf

**Parameters:**
- `name` (string)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Namespace/SymbolTable.cs)

#### `int IndexOf(string name, int classScope)`

SymbolTable.IndexOf

**Parameters:**
- `name` (string)
- `classScope` (int)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Namespace/SymbolTable.cs)

#### `int IndexOf(string name, int classScope, int functionindex)`

SymbolTable.IndexOf

**Parameters:**
- `name` (string)
- `classScope` (int)
- `functionindex` (int)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Namespace/SymbolTable.cs)

#### `int IndexOfClass(string name, int classScope, int functionindex)`

SymbolTable.IndexOfClass

**Parameters:**
- `name` (string)
- `classScope` (int)
- `functionindex` (int)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Namespace/SymbolTable.cs)

#### `bool Remove(ProtoCore.DSASM.SymbolNode node)`

SymbolTable.Remove

**Parameters:**
- `node` (ProtoCore.DSASM.SymbolNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Namespace/SymbolTable.cs)

#### `void UndefineSymbol(ProtoCore.DSASM.SymbolNode symbol)`

Method to undefine a symbol from the symboltable entry and cache

**Parameters:**
- `symbol` (ProtoCore.DSASM.SymbolNode) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Namespace/SymbolTable.cs)

### Properties
- `RuntimeIndex` (int, get/set) — SymbolTable.RuntimeIndex
- `ScopeName` (string, get/set) — SymbolTable.ScopeName
- `symbolList` (System.Collections.Generic.IDictionary<int, ProtoCore.DSASM.SymbolNode>, get) — SymbolTable.symbolList

## UnaryOperator (enum)

Type UnaryOperator

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Defs.cs)

### Values
- `Decrement` = `4`
- `Increment` = `3`
- `Neg` = `5`
- `Negate` = `2`
- `None` = `0`
- `Not` = `1`

## kw (struct)

Type kw

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/DSASM/Defs.cs)

