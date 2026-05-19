---
name: dynamo-protocore-utils
description: This skill encodes the dynamo 4.1.0 surface of the ProtoCore.Utils namespace — 14 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ArrayUtils, ClassUtils, VariableLine, ParseParam, CompilerUtils, CoreUtils, FileUtils, MathUtils, and 6 more types.
---

# ProtoCore.Utils

Auto-generated from vendor docs for dynamo 4.1.0. 14 types in this namespace.

## ArrayUtils (static-class)

Type ArrayUtils

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ArrayUtils.cs)

### Methods
#### `bool ContainsDoubleElement(ProtoCore.DSASM.StackValue sv, ProtoCore.RuntimeCore runtimeCore)`

Whether sv is double or arrays contains double value.

**Parameters:**
- `sv` (ProtoCore.DSASM.StackValue) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ArrayUtils.cs)

#### `bool ContainsNonArrayElement(ProtoCore.DSASM.StackValue sv, ProtoCore.RuntimeCore runtimeCore)`

If the passed in value is not an array or an empty array or an array which contains only empty arrays, return false. Otherwise, return true;

**Parameters:**
- `sv` (ProtoCore.DSASM.StackValue) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ArrayUtils.cs)

#### `System.Collections.Generic.IEnumerable<T> GetCommonItems<T>(System.Collections.Generic.IEnumerable<T>[] lists)`

Returns the list of common items from a given collection of generic lists

**Parameters:**
- `lists` (System.Collections.Generic.IEnumerable<T>[]) — 

**Returns:** `System.Collections.Generic.IEnumerable<T>` — list of common items from multiple lists

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ArrayUtils.cs)

#### `bool GetFirstNonArrayStackValue(ProtoCore.DSASM.StackValue svArray, ref ProtoCore.DSASM.StackValue sv, ProtoCore.RuntimeCore runtimeCore)`

Retrieve the first non-array element in an array

**Parameters:**
- `svArray` (ProtoCore.DSASM.StackValue) — 
- `sv` (ref ProtoCore.DSASM.StackValue) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 

**Returns:** `bool` — true if the element was found

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ArrayUtils.cs)

#### `ProtoCore.DSASM.ClassNode GetGreatestCommonSubclassForArray(ProtoCore.DSASM.StackValue array, ProtoCore.RuntimeCore runtimeCore)`

If an empty array is passed, the result will be null if there are instances, but they share no common supertype the result will be var

**Parameters:**
- `array` (ProtoCore.DSASM.StackValue)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.ClassNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ArrayUtils.cs)

#### `int GetMaxRankForArray(ProtoCore.DSASM.StackValue array, ProtoCore.RuntimeCore runtimeCore)`

ArrayUtils.GetMaxRankForArray

**Parameters:**
- `array` (ProtoCore.DSASM.StackValue)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ArrayUtils.cs)

#### `System.Collections.Generic.Dictionary<int, ProtoCore.DSASM.StackValue> GetTypeExamplesForLayer(ProtoCore.DSASM.StackValue paramStackValue, ProtoCore.RuntimeCore runtimeCore)`

This method returns the distinct(by metadata type) reduced params for all the elements inside the paramStackValue, if it is an array. If it is not an array, it just returns the paramStackValue.

**Parameters:**
- `paramStackValue` (ProtoCore.DSASM.StackValue) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 

**Returns:** `System.Collections.Generic.Dictionary<int, ProtoCore.DSASM.StackValue>` — A dictionary where the value is the current ReducedParam and the key is its metaData type

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ArrayUtils.cs)

#### `System.Collections.Generic.Dictionary<ProtoCore.DSASM.ClassNode, int> GetTypeStatisticsForArray(ProtoCore.DSASM.StackValue array, ProtoCore.RuntimeCore runtimeCore)`

Generate type statistics for the whole array

**Parameters:**
- `array` (ProtoCore.DSASM.StackValue) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 

**Returns:** `System.Collections.Generic.Dictionary<ProtoCore.DSASM.ClassNode, int>` — usage frequency by type

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ArrayUtils.cs)

#### `System.Collections.Generic.Dictionary<ProtoCore.DSASM.ClassNode, int> GetTypeStatisticsForLayer(ProtoCore.DSASM.StackValue array, ProtoCore.RuntimeCore runtimeCore)`

Generate type statistics for given layer of an array

**Parameters:**
- `array` (ProtoCore.DSASM.StackValue) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 

**Returns:** `System.Collections.Generic.Dictionary<ProtoCore.DSASM.ClassNode, int>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ArrayUtils.cs)

#### `ProtoCore.DSASM.StackValue[][] GetZippedIndices(System.Collections.Generic.List<ProtoCore.DSASM.StackValue> indices, ProtoCore.RuntimeCore runtimeCore)`

For an array we supporting zipped replicaiton for array indexing as well. I.e., for the following expression: a[1..3][2..4] = x; It will be expanded to a[1][2] = x; a[2][3] = x; a[3][4] = x; So here we need to calculate zipped indices. The length of returned indices is decided by the shortest length of index that used in array indexing. E.g., For array indexing [{1, 2, 3}][{"x", "y"}][{6, 7, 8}], i.e., 1 -> "x" -> 6 2 -> "y" -> 7 3 -> -> 8 The shortest length of index is 2 ({"x", "y"}), so function will returns: {{1, "x", 6}, {2, "y", 7}}

**Parameters:**
- `indices` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 

**Returns:** `ProtoCore.DSASM.StackValue[][]` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ArrayUtils.cs)

#### `bool IsEmpty(ProtoCore.DSASM.StackValue arrayPointer, ProtoCore.RuntimeCore runtimeCore)`

Returns true if an array is an empty list or all its elements are empty lists.

**Parameters:**
- `arrayPointer` (ProtoCore.DSASM.StackValue) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ArrayUtils.cs)

## ClassUtils (static-class)

Type ClassUtils

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ClassUtils.cs)

### Methods
#### `System.Collections.Generic.List<int> GetClassUpcastChain(ProtoCore.DSASM.ClassNode cn, ProtoCore.RuntimeCore runtimeCore)`

Returns the list of classes that this can be upcast to It includes the class itself

**Parameters:**
- `cn` (ProtoCore.DSASM.ClassNode) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 

**Returns:** `System.Collections.Generic.List<int>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ClassUtils.cs)

#### `int GetUpcastCountTo(ProtoCore.DSASM.ClassNode from, ProtoCore.DSASM.ClassNode to, ProtoCore.RuntimeCore runtimeCore)`

Returns the number of upcasts that need to be performed to turn a class into another class in its upcast chain

**Parameters:**
- `from` (ProtoCore.DSASM.ClassNode) — 
- `to` (ProtoCore.DSASM.ClassNode) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ClassUtils.cs)

## CompilerUtils (static-class)

Type CompilerUtils

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CompilerUtils.cs)

### Methods
#### `ProtoCore.BuildStatus PreCompile(string code, ProtoCore.Core core, ProtoCore.AST.AssociativeAST.CodeBlockNode codeBlock, ref int blockId)`

Does the first pass of compilation and returns a list of wanrnings in compilation

**Parameters:**
- `code` (string) — 
- `core` (ProtoCore.Core) — 
- `codeBlock` (ProtoCore.AST.AssociativeAST.CodeBlockNode) — 
- `blockId` (ref int) — 

**Returns:** `ProtoCore.BuildStatus` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CompilerUtils.cs)

#### `bool TryLoadAssemblyIntoCore(ProtoCore.Core core, string assemblyPath)`

CompilerUtils.TryLoadAssemblyIntoCore

**Parameters:**
- `core` (ProtoCore.Core)
- `assemblyPath` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CompilerUtils.cs)

## CoreUtils (static-class)

Type CoreUtils

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

### Methods
#### `ProtoCore.DSASM.StackValue AddStackValueString(ProtoCore.DSASM.StackValue sv1, ProtoCore.DSASM.StackValue sv2, ProtoCore.RuntimeCore runtimeCore)`

Performs addition on 2 StackValues This is used by the VM when adding strings

**Parameters:**
- `sv1` (ProtoCore.DSASM.StackValue) — 
- `sv2` (ProtoCore.DSASM.StackValue) — 
- `runtimeCore` (ProtoCore.RuntimeCore) — 

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> BuildASTList(ProtoCore.Core core, System.Collections.Generic.List<string> codeList)`

Parses designscript code and outputs ProtoAST

**Parameters:**
- `core` (ProtoCore.Core) — 
- `codeList` (System.Collections.Generic.List<string>) — 

**Returns:** `System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> BuildASTList(ProtoCore.Core core, string code)`

Parses designscript code and outputs ProtoAST

**Parameters:**
- `core` (ProtoCore.Core) — 
- `code` (string) — 

**Returns:** `System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `ProtoCore.AST.AssociativeAST.ExprListNode BuildArrayExprList(ProtoCore.AST.AssociativeAST.AssociativeNode arrayNode)`

CoreUtils.BuildArrayExprList

**Parameters:**
- `arrayNode` (ProtoCore.AST.AssociativeAST.AssociativeNode)

**Returns:** `ProtoCore.AST.AssociativeAST.ExprListNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `string BuildSSATemp(ProtoCore.Core core)`

CoreUtils.BuildSSATemp

**Parameters:**
- `core` (ProtoCore.Core)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `ProtoCore.DSASM.StackValue BuildStackValueForPrimitive(ProtoCore.AST.AssociativeAST.AssociativeNode node, ProtoCore.RuntimeCore rt)`

CoreUtils.BuildStackValueForPrimitive

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.AssociativeNode)
- `rt` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `void CopyDebugData(ProtoCore.AST.Node nodeTo, ProtoCore.AST.Node nodeFrom)`

CoreUtils.CopyDebugData

**Parameters:**
- `nodeTo` (ProtoCore.AST.Node)
- `nodeFrom` (ProtoCore.AST.Node)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode CreateNodeByCombiningIdentifiers(System.Collections.Generic.IList<ProtoCore.AST.AssociativeAST.AssociativeNode> nodeList)`

CoreUtils.CreateNodeByCombiningIdentifiers

**Parameters:**
- `nodeList` (System.Collections.Generic.IList<ProtoCore.AST.AssociativeAST.AssociativeNode>)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode CreateNodeFromString(string name)`

Given a name or string of names, this creates an IdentifierNode or IdentifierListNode e.g. Creates an IdentifierNode from A and IdentifierListNode from A.B

**Parameters:**
- `name` (string) — 

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `ProtoCore.AST.AssociativeAST.FunctionDotCallNode GenerateCallDotNode(ProtoCore.AST.AssociativeAST.AssociativeNode lhs, ProtoCore.AST.AssociativeAST.FunctionCallNode rhsCall, ProtoCore.Core core)`

CoreUtils.GenerateCallDotNode

**Parameters:**
- `lhs` (ProtoCore.AST.AssociativeAST.AssociativeNode)
- `rhsCall` (ProtoCore.AST.AssociativeAST.FunctionCallNode)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.AST.AssociativeAST.FunctionDotCallNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `string GenerateIdentListNameString(ProtoCore.AST.AssociativeAST.AssociativeNode node)`

CoreUtils.GenerateIdentListNameString

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.AssociativeNode)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `string GetAssemblyFromClassName(ProtoCore.DSASM.ClassTable classTable, string className)`

Given a partial class name, get assembly to which the class belongs

**Parameters:**
- `classTable` (ProtoCore.DSASM.ClassTable) — class table in Core
- `className` (string) — class name

**Returns:** `string` — assembly to which the class belongs

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `ProtoCore.DSASM.CodeBlock GetCodeBlock(System.Collections.Generic.List<ProtoCore.DSASM.CodeBlock> blockList, int blockId)`

Returns the Codeblock given the blockId

**Parameters:**
- `blockList` (System.Collections.Generic.List<ProtoCore.DSASM.CodeBlock>) — 
- `blockId` (int) — 

**Returns:** `ProtoCore.DSASM.CodeBlock` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `object GetDataOfValue(ProtoCore.Mirror.MirrorData value)`

Returns the CLR object for a given mirror data

**Parameters:**
- `value` (ProtoCore.Mirror.MirrorData)

**Returns:** `object` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `ProtoCore.DSASM.ProcedureNode GetFunctionByName(string name, ProtoCore.DSASM.CodeBlock codeBlock)`

CoreUtils.GetFunctionByName

**Parameters:**
- `name` (string)
- `codeBlock` (ProtoCore.DSASM.CodeBlock)

**Returns:** `ProtoCore.DSASM.ProcedureNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `ProtoCore.DSASM.ProcedureNode GetFunctionBySignature(string name, System.Collections.Generic.List<ProtoCore.Type> argTypeList, ProtoCore.DSASM.CodeBlock codeblock)`

CoreUtils.GetFunctionBySignature

**Parameters:**
- `name` (string)
- `argTypeList` (System.Collections.Generic.List<ProtoCore.Type>)
- `codeblock` (ProtoCore.DSASM.CodeBlock)

**Returns:** `ProtoCore.DSASM.ProcedureNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `int GetFunctionHash(ProtoCore.AST.AssociativeAST.FunctionDefinitionNode functionDef)`

Returns the has id of a function signature given the name and argument types

**Parameters:**
- `functionDef` (ProtoCore.AST.AssociativeAST.FunctionDefinitionNode) — 

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `string GetIdentifierExceptMethodName(ProtoCore.AST.AssociativeAST.IdentifierListNode identList)`

Retrieves the string format of the identifier list from left to right, leaving out any symbols after the last identifier. Given: A.B() Return: "A" Given: A.B.C()[0] Return: "A.B" Given: A.B().C Return: "A" Given: A.B[0].C Return: "A.B[0].C" Given: A().B (global function) Return: empty string Given: A.B[0].C() Return: "A.B[0]"

**Parameters:**
- `identList` (ProtoCore.AST.AssociativeAST.IdentifierListNode) — 

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `string GetIdentifierExceptMethodName(ProtoCore.AST.ImperativeAST.IdentifierListNode identList)`

Retrieves the string format of the identifier list from left to right, leaving out any symbols after the last identifier. Given: A.B() Return: "A" Given: A.B.C()[0] Return: "A.B" Given: A.B().C Return: "A" Given: A.B[0].C Return: "A.B[0].C" Given: A().B (global function) Return: empty string Given: A.B[0].C() Return: "A.B[0]"

**Parameters:**
- `identList` (ProtoCore.AST.ImperativeAST.IdentifierListNode) — 

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `string GetIdentifierStringUntilFirstParenthesis(ProtoCore.AST.Node node)`

Retrieves the string format of the identifier list from left to right, leaving out any symbols after the last identifier. Given: A.B() Return: "A.B" Given: A.B.C()[0] Return: "A.B.C" Given: A.B().C Return: "A.B" Given: A.B[0].C Return: "A.B[0].C"

**Parameters:**
- `node` (ProtoCore.AST.Node) — 

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `string GetLanguageString(ProtoCore.Language language)`

CoreUtils.GetLanguageString

**Parameters:**
- `language` (ProtoCore.Language)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `string GetOperatorString(ProtoCore.DSASM.Operator optr)`

CoreUtils.GetOperatorString

**Parameters:**
- `optr` (ProtoCore.DSASM.Operator)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `string[] GetResolvedClassName(ProtoCore.DSASM.ClassTable classTable, ProtoCore.AST.AssociativeAST.AssociativeNode identifierList)`

Inspects the input identifier list to match all class names with the class used in it

**Parameters:**
- `classTable` (ProtoCore.DSASM.ClassTable) — 
- `identifierList` (ProtoCore.AST.AssociativeAST.AssociativeNode) — single identifier or identifier list

**Returns:** `string[]` — list of fully resolved class names

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `void InsertPredefinedAndBuiltinMethods(ProtoCore.Core core, ProtoCore.AST.AssociativeAST.CodeBlockNode root)`

CoreUtils.InsertPredefinedAndBuiltinMethods

**Parameters:**
- `core` (ProtoCore.Core)
- `root` (ProtoCore.AST.AssociativeAST.CodeBlockNode)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `bool IsAutoGeneratedVar(string name)`

CoreUtils.IsAutoGeneratedVar

**Parameters:**
- `name` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `bool IsDefaultArgTemp(string varname)`

CoreUtils.IsDefaultArgTemp

**Parameters:**
- `varname` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `bool IsDisposeMethod(string methodName)`

CoreUtils.IsDisposeMethod

**Parameters:**
- `methodName` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `bool IsDotMethod(string methodName)`

CoreUtils.IsDotMethod

**Parameters:**
- `methodName` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `bool IsGetTypeMethod(string methodName)`

CoreUtils.IsGetTypeMethod

**Parameters:**
- `methodName` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `bool IsGetter(string propertyName)`

CoreUtils.IsGetter

**Parameters:**
- `propertyName` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `bool IsGetterSetter(string propertyName)`

CoreUtils.IsGetterSetter

**Parameters:**
- `propertyName` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `bool IsInternalFunction(string methodName)`

CoreUtils.IsInternalFunction

**Parameters:**
- `methodName` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `bool IsInternalMethod(string methodName)`

CoreUtils.IsInternalMethod

**Parameters:**
- `methodName` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `bool IsNonStaticPropertyLookupOnClass(ProtoCore.DSASM.ProcedureNode procCallNode, string className)`

CoreUtils.IsNonStaticPropertyLookupOnClass

**Parameters:**
- `procCallNode` (ProtoCore.DSASM.ProcedureNode)
- `className` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `bool IsPrimitiveASTNode(ProtoCore.AST.AssociativeAST.AssociativeNode node)`

Checks if an AST node is a primitive

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.AssociativeNode) — 

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `bool IsPropertyTemp(string varname)`

CoreUtils.IsPropertyTemp

**Parameters:**
- `varname` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `bool IsSSATemp(string ssaVar)`

CoreUtils.IsSSATemp

**Parameters:**
- `ssaVar` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `bool IsSetter(string propertyName)`

CoreUtils.IsSetter

**Parameters:**
- `propertyName` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `void LogSemanticError(ProtoCore.Core core, string msg, string fileName, int line, int col)`

CoreUtils.LogSemanticError

**Parameters:**
- `core` (ProtoCore.Core)
- `msg` (string)
- `fileName` (string)
- `line` (int)
- `col` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `void LogSemanticError(ProtoCore.DSASM.Interpreter dsi, string msg, string fileName, int line, int col)`

CoreUtils.LogSemanticError

**Parameters:**
- `dsi` (ProtoCore.DSASM.Interpreter)
- `msg` (string)
- `fileName` (string)
- `line` (int)
- `col` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `void LogWarning(ProtoCore.Core core, ProtoCore.BuildData.WarningID id, string msg, string fileName, int line, int col)`

CoreUtils.LogWarning

**Parameters:**
- `core` (ProtoCore.Core)
- `id` (ProtoCore.BuildData.WarningID)
- `msg` (string)
- `fileName` (string)
- `line` (int)
- `col` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `void LogWarning(ProtoCore.DSASM.Interpreter dsi, ProtoCore.Runtime.WarningID id, string msg, string fileName, int line, int col)`

CoreUtils.LogWarning

**Parameters:**
- `dsi` (ProtoCore.DSASM.Interpreter)
- `id` (ProtoCore.Runtime.WarningID)
- `msg` (string)
- `fileName` (string)
- `line` (int)
- `col` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `bool StartsWithDoubleUnderscores(string name)`

CoreUtils.StartsWithDoubleUnderscores

**Parameters:**
- `name` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `bool StartsWithSingleUnderscore(string name)`

CoreUtils.StartsWithSingleUnderscore

**Parameters:**
- `name` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `bool TryGetOperator(string methodName, ref ProtoCore.DSASM.Operator op)`

CoreUtils.TryGetOperator

**Parameters:**
- `methodName` (string)
- `op` (ref ProtoCore.DSASM.Operator)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

#### `bool TryGetPropertyName(string methodName, ref string propertyName)`

CoreUtils.TryGetPropertyName

**Parameters:**
- `methodName` (string)
- `propertyName` (ref string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CoreUtils.cs)

## FileUtils (static-class)

Type FileUtils

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/FileUtils.cs)

### Methods
#### `string GetDSFullPathName(string fileName, ProtoCore.Options options)`

Locates the given file from the search path options and gets the full file path.

**Parameters:**
- `fileName` (string) — Name of the file to locate
- `options` (ProtoCore.Options) — Options structure for search path, if options is null it will search only in the executing assembly path or the current directory.

**Returns:** `string` — Full path for the file if located successfully else the file name when failed to locate the given file

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/FileUtils.cs)

#### `string GetFullPathName(string fileName, string currentDirectory)`

FileUtils.GetFullPathName

**Parameters:**
- `fileName` (string)
- `currentDirectory` (string)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/FileUtils.cs)

#### `string GetInstallLocation()`

FileUtils.GetInstallLocation

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/FileUtils.cs)

## MathUtils (static-class)

Type MathUtils

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/MathUtils.cs)

### Methods
#### `bool Equals(double lhs, double rhs)`

MathUtils.Equals

**Parameters:**
- `lhs` (double)
- `rhs` (double)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/MathUtils.cs)

#### `bool Equals(double lhs, double rhs, double tolerance)`

MathUtils.Equals

**Parameters:**
- `lhs` (double)
- `rhs` (double)
- `tolerance` (double)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/MathUtils.cs)

#### `bool IsNumber(object value)`

MathUtils.IsNumber

**Parameters:**
- `value` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/MathUtils.cs)

## NodeUtils (static-class)

Type NodeUtils

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/NodeUtils.cs)

### Methods
#### `ProtoCore.AST.AssociativeAST.AssociativeNode Clone(ProtoCore.AST.AssociativeAST.AssociativeNode rhsNode)`

NodeUtils.Clone

**Parameters:**
- `rhsNode` (ProtoCore.AST.AssociativeAST.AssociativeNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/NodeUtils.cs)

#### `ProtoCore.AST.ImperativeAST.ImperativeNode Clone(ProtoCore.AST.ImperativeAST.ImperativeNode rhsNode)`

NodeUtils.Clone

**Parameters:**
- `rhsNode` (ProtoCore.AST.ImperativeAST.ImperativeNode)

**Returns:** `ProtoCore.AST.ImperativeAST.ImperativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/NodeUtils.cs)

#### `ProtoCore.AST.Node Clone(ProtoCore.AST.Node rhsNode)`

NodeUtils.Clone

**Parameters:**
- `rhsNode` (ProtoCore.AST.Node)

**Returns:** `ProtoCore.AST.Node` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/NodeUtils.cs)

#### `void CopyNodeLocation(ProtoCore.AST.Node node, ProtoCore.AST.Node other)`

NodeUtils.CopyNodeLocation

**Parameters:**
- `node` (ProtoCore.AST.Node)
- `other` (ProtoCore.AST.Node)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/NodeUtils.cs)

#### `bool IsReturnExpressionNode(ProtoCore.AST.AssociativeAST.AssociativeNode node)`

NodeUtils.IsReturnExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.AssociativeNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/NodeUtils.cs)

#### `bool IsReturnExpressionNode(ProtoCore.AST.ImperativeAST.ImperativeNode node)`

NodeUtils.IsReturnExpressionNode

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.ImperativeNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/NodeUtils.cs)

#### `void SetNodeEndLocation(ProtoCore.AST.Node node, ProtoCore.AST.Node other)`

NodeUtils.SetNodeEndLocation

**Parameters:**
- `node` (ProtoCore.AST.Node)
- `other` (ProtoCore.AST.Node)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/NodeUtils.cs)

#### `void SetNodeEndLocation(ProtoCore.AST.Node node, ProtoCore.DesignScriptParser.Token token)`

NodeUtils.SetNodeEndLocation

**Parameters:**
- `node` (ProtoCore.AST.Node)
- `token` (ProtoCore.DesignScriptParser.Token)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/NodeUtils.cs)

#### `void SetNodeLocation(ProtoCore.AST.Node node, ProtoCore.AST.Node startNode, ProtoCore.AST.Node endNode)`

NodeUtils.SetNodeLocation

**Parameters:**
- `node` (ProtoCore.AST.Node)
- `startNode` (ProtoCore.AST.Node)
- `endNode` (ProtoCore.AST.Node)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/NodeUtils.cs)

#### `void SetNodeLocation(ProtoCore.AST.Node node, ProtoCore.DesignScriptParser.Token token)`

NodeUtils.SetNodeLocation

**Parameters:**
- `node` (ProtoCore.AST.Node)
- `token` (ProtoCore.DesignScriptParser.Token)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/NodeUtils.cs)

#### `void SetNodeStartLocation(ProtoCore.AST.Node node, ProtoCore.AST.Node other)`

NodeUtils.SetNodeStartLocation

**Parameters:**
- `node` (ProtoCore.AST.Node)
- `other` (ProtoCore.AST.Node)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/NodeUtils.cs)

#### `void SetNodeStartLocation(ProtoCore.AST.Node node, ProtoCore.DesignScriptParser.Token token)`

NodeUtils.SetNodeStartLocation

**Parameters:**
- `node` (ProtoCore.AST.Node)
- `token` (ProtoCore.DesignScriptParser.Token)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/NodeUtils.cs)

#### `void UpdateBinaryExpressionLocation(ProtoCore.AST.AssociativeAST.BinaryExpressionNode node)`

NodeUtils.UpdateBinaryExpressionLocation

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.BinaryExpressionNode)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/NodeUtils.cs)

#### `void UpdateBinaryExpressionLocation(ProtoCore.AST.ImperativeAST.BinaryExpressionNode node)`

NodeUtils.UpdateBinaryExpressionLocation

**Parameters:**
- `node` (ProtoCore.AST.ImperativeAST.BinaryExpressionNode)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/NodeUtils.cs)

## ParseParam (class)

Type ParseParam

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CompilerUtils.cs)

### Constructors
- `void ParseParam(System.Guid postfixGuid, string code, ProtoCore.Namespace.ElementResolver elementResolver)` — ParseParam.ParseParam

### Methods
#### `void AppendComments(System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode> commentNodes)`

ParseParam.AppendComments

**Parameters:**
- `commentNodes` (System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode>)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CompilerUtils.cs)

#### `void AppendErrors(System.Collections.Generic.IEnumerable<ProtoCore.BuildData.ErrorEntry> errors)`

ParseParam.AppendErrors

**Parameters:**
- `errors` (System.Collections.Generic.IEnumerable<ProtoCore.BuildData.ErrorEntry>)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CompilerUtils.cs)

#### `void AppendParsedNodes(System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode> parsedNodes)`

ParseParam.AppendParsedNodes

**Parameters:**
- `parsedNodes` (System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode>)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CompilerUtils.cs)

#### `void AppendUnboundIdentifier(string displayName, string identifier)`

ParseParam.AppendUnboundIdentifier

**Parameters:**
- `displayName` (string)
- `identifier` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CompilerUtils.cs)

#### `void AppendWarnings(System.Collections.Generic.IEnumerable<ProtoCore.BuildData.WarningEntry> warnings)`

ParseParam.AppendWarnings

**Parameters:**
- `warnings` (System.Collections.Generic.IEnumerable<ProtoCore.BuildData.WarningEntry>)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CompilerUtils.cs)

### Properties
- `ElementResolver` (ProtoCore.Namespace.ElementResolver, get) — ParseParam.ElementResolver
- `Errors` (System.Collections.Generic.IEnumerable<ProtoCore.BuildData.ErrorEntry>, get) — ParseParam.Errors
- `OriginalCode` (string, get) — ParseParam.OriginalCode
- `ParsedComments` (System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode>, get) — ParseParam.ParsedComments
- `ParsedNodes` (System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode>, get) — ParseParam.ParsedNodes
- `PostfixGuid` (System.Guid, get) — ParseParam.PostfixGuid
- `UnboundIdentifiers` (System.Collections.Generic.IDictionary<string, string>, get) — ParseParam.UnboundIdentifiers
- `Warnings` (System.Collections.Generic.IEnumerable<ProtoCore.BuildData.WarningEntry>, get) — ParseParam.Warnings

## ParseResult (class)

Parse result.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ParserUtils.cs)

### Constructors
- `void ParseResult()` — ParseResult.ParseResult

### Properties
- `CodeBlockNode` (ProtoCore.AST.AssociativeAST.CodeBlockNode, get/set) — All code related AST nodes will be saved in a CodeBlockNode.
- `CommentBlockNode` (ProtoCore.AST.AssociativeAST.CodeBlockNode, get/set) — All comment related AST nodes will be saved in a CodeBlockNode.

## ParserUtils (static-class)

These are string manipulation utility functions that focus on lexing and parsing heuristics

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ParserUtils.cs)

### Methods
#### `ProtoCore.DesignScriptParser.Parser CreateParser(string code, ProtoCore.Core core)`

Returns a parser for the DS code.

**Parameters:**
- `code` (string)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.DesignScriptParser.Parser` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ParserUtils.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> FindExprListNodes(ProtoCore.AST.AssociativeAST.CodeBlockNode node)`

ParserUtils.FindExprListNodes

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.CodeBlockNode)

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ParserUtils.cs)

#### `System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode> GetAstNodes(ProtoCore.AST.AssociativeAST.CodeBlockNode codeBlockNode)`

ParserUtils.GetAstNodes

**Parameters:**
- `codeBlockNode` (ProtoCore.AST.AssociativeAST.CodeBlockNode)

**Returns:** `System.Collections.Generic.List<ProtoCore.AST.AssociativeAST.AssociativeNode>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ParserUtils.cs)

#### `System.Collections.Generic.List<string> GetLHSatAssignment(string line, int equalIndex)`

ParserUtils.GetLHSatAssignment

**Parameters:**
- `line` (string)
- `equalIndex` (int)

**Returns:** `System.Collections.Generic.List<string>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ParserUtils.cs)

#### `ProtoCore.AST.AssociativeAST.CodeBlockNode Parse(string code)`

Parses designscript code and returns a ProtoAST CodeBlockNode

**Parameters:**
- `code` (string) — Source code to parse

**Returns:** `ProtoCore.AST.AssociativeAST.CodeBlockNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ParserUtils.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode ParseRHSExpression(string expression, ProtoCore.Core core)`

Parse simple RHS expression

**Parameters:**
- `expression` (string) — 
- `core` (ProtoCore.Core) — 

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ParserUtils.cs)

#### `ProtoCore.Utils.ParseResult ParseWithCore(string code, ProtoCore.Core core)`

Parses desginscript code with specified core and returns a ProtoAST CodeBlockNode

**Parameters:**
- `code` (string) — 
- `core` (ProtoCore.Core) — 

**Returns:** `ProtoCore.Utils.ParseResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ParserUtils.cs)

#### `ProtoCore.AST.AssociativeAST.CodeBlockNode ParseWithDeprecatedListSyntax(string code)`

ParserUtils.ParseWithDeprecatedListSyntax

**Parameters:**
- `code` (string)

**Returns:** `ProtoCore.AST.AssociativeAST.CodeBlockNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ParserUtils.cs)

#### `string TryMigrateDeprecatedListSyntax(string codeText)`

ParserUtils.TryMigrateDeprecatedListSyntax

**Parameters:**
- `codeText` (string)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ParserUtils.cs)

## ProtoDouble (struct)

Type ProtoDouble

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

### Constructors
- `void ProtoDouble(ProtoCore.Utils.ProtoDouble d)` — ProtoDouble.ProtoDouble
- `void ProtoDouble(double value)` — ProtoDouble.ProtoDouble

### Methods
#### `bool Equals(object obj)`

ProtoDouble.Equals

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `int GetHashCode()`

ProtoDouble.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `ProtoCore.Utils.ProtoDouble Parse(string s)`

ProtoDouble.Parse

**Parameters:**
- `s` (string)

**Returns:** `ProtoCore.Utils.ProtoDouble` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `ProtoCore.Utils.ProtoDouble Parse(string s, System.Globalization.NumberStyles style)`

ProtoDouble.Parse

**Parameters:**
- `s` (string)
- `style` (System.Globalization.NumberStyles)

**Returns:** `ProtoCore.Utils.ProtoDouble` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `ProtoCore.Utils.ProtoDouble Parse(string s, System.Globalization.NumberStyles style, System.IFormatProvider provider)`

ProtoDouble.Parse

**Parameters:**
- `s` (string)
- `style` (System.Globalization.NumberStyles)
- `provider` (System.IFormatProvider)

**Returns:** `ProtoCore.Utils.ProtoDouble` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `ProtoCore.Utils.ProtoDouble Parse(string s, System.IFormatProvider provider)`

ProtoDouble.Parse

**Parameters:**
- `s` (string)
- `provider` (System.IFormatProvider)

**Returns:** `ProtoCore.Utils.ProtoDouble` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `string ToString()`

ProtoDouble.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `string ToString(System.IFormatProvider provider)`

ProtoDouble.ToString

**Parameters:**
- `provider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `string ToString(string format)`

ProtoDouble.ToString

**Parameters:**
- `format` (string)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `string ToString(string format, System.IFormatProvider provider)`

ProtoDouble.ToString

**Parameters:**
- `format` (string)
- `provider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `bool TryParse(string s, System.Globalization.NumberStyles style, System.IFormatProvider provider, ref ProtoCore.Utils.ProtoDouble result)`

ProtoDouble.TryParse

**Parameters:**
- `s` (string)
- `style` (System.Globalization.NumberStyles)
- `provider` (System.IFormatProvider)
- `result` (ref ProtoCore.Utils.ProtoDouble)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `bool TryParse(string s, ref ProtoCore.Utils.ProtoDouble result)`

ProtoDouble.TryParse

**Parameters:**
- `s` (string)
- `result` (ref ProtoCore.Utils.ProtoDouble)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `ProtoCore.Utils.ProtoDouble op_Addition(ProtoCore.Utils.ProtoDouble left, ProtoCore.Utils.ProtoDouble right)`

ProtoDouble.op_Addition

**Parameters:**
- `left` (ProtoCore.Utils.ProtoDouble)
- `right` (ProtoCore.Utils.ProtoDouble)

**Returns:** `ProtoCore.Utils.ProtoDouble` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `ProtoCore.Utils.ProtoDouble op_Division(ProtoCore.Utils.ProtoDouble left, ProtoCore.Utils.ProtoDouble right)`

ProtoDouble.op_Division

**Parameters:**
- `left` (ProtoCore.Utils.ProtoDouble)
- `right` (ProtoCore.Utils.ProtoDouble)

**Returns:** `ProtoCore.Utils.ProtoDouble` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `bool op_Equality(ProtoCore.Utils.ProtoDouble left, ProtoCore.Utils.ProtoDouble right)`

ProtoDouble.op_Equality

**Parameters:**
- `left` (ProtoCore.Utils.ProtoDouble)
- `right` (ProtoCore.Utils.ProtoDouble)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `long op_Explicit(ProtoCore.Utils.ProtoDouble d)`

ProtoDouble.op_Explicit

**Parameters:**
- `d` (ProtoCore.Utils.ProtoDouble)

**Returns:** `long` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `float op_Explicit(ProtoCore.Utils.ProtoDouble d)`

ProtoDouble.op_Explicit

**Parameters:**
- `d` (ProtoCore.Utils.ProtoDouble)

**Returns:** `float` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `int op_Explicit(ProtoCore.Utils.ProtoDouble d)`

ProtoDouble.op_Explicit

**Parameters:**
- `d` (ProtoCore.Utils.ProtoDouble)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `bool op_GreaterThan(ProtoCore.Utils.ProtoDouble left, ProtoCore.Utils.ProtoDouble right)`

ProtoDouble.op_GreaterThan

**Parameters:**
- `left` (ProtoCore.Utils.ProtoDouble)
- `right` (ProtoCore.Utils.ProtoDouble)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `bool op_GreaterThanOrEqual(ProtoCore.Utils.ProtoDouble left, ProtoCore.Utils.ProtoDouble right)`

ProtoDouble.op_GreaterThanOrEqual

**Parameters:**
- `left` (ProtoCore.Utils.ProtoDouble)
- `right` (ProtoCore.Utils.ProtoDouble)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `double op_Implicit(ProtoCore.Utils.ProtoDouble d)`

ProtoDouble.op_Implicit

**Parameters:**
- `d` (ProtoCore.Utils.ProtoDouble)

**Returns:** `double` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `ProtoCore.Utils.ProtoDouble op_Implicit(double value)`

ProtoDouble.op_Implicit

**Parameters:**
- `value` (double)

**Returns:** `ProtoCore.Utils.ProtoDouble` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `bool op_Inequality(ProtoCore.Utils.ProtoDouble left, ProtoCore.Utils.ProtoDouble right)`

ProtoDouble.op_Inequality

**Parameters:**
- `left` (ProtoCore.Utils.ProtoDouble)
- `right` (ProtoCore.Utils.ProtoDouble)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `bool op_LessThan(ProtoCore.Utils.ProtoDouble left, ProtoCore.Utils.ProtoDouble right)`

ProtoDouble.op_LessThan

**Parameters:**
- `left` (ProtoCore.Utils.ProtoDouble)
- `right` (ProtoCore.Utils.ProtoDouble)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `bool op_LessThanOrEqual(ProtoCore.Utils.ProtoDouble left, ProtoCore.Utils.ProtoDouble right)`

ProtoDouble.op_LessThanOrEqual

**Parameters:**
- `left` (ProtoCore.Utils.ProtoDouble)
- `right` (ProtoCore.Utils.ProtoDouble)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `ProtoCore.Utils.ProtoDouble op_Multiply(ProtoCore.Utils.ProtoDouble left, ProtoCore.Utils.ProtoDouble right)`

ProtoDouble.op_Multiply

**Parameters:**
- `left` (ProtoCore.Utils.ProtoDouble)
- `right` (ProtoCore.Utils.ProtoDouble)

**Returns:** `ProtoCore.Utils.ProtoDouble` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

#### `ProtoCore.Utils.ProtoDouble op_Subtraction(ProtoCore.Utils.ProtoDouble left, ProtoCore.Utils.ProtoDouble right)`

ProtoDouble.op_Subtraction

**Parameters:**
- `left` (ProtoCore.Utils.ProtoDouble)
- `right` (ProtoCore.Utils.ProtoDouble)

**Returns:** `ProtoCore.Utils.ProtoDouble` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/ProtoDouble.cs)

## StringUtils (static-class)

Type StringUtils

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/StringUtils.cs)

### Methods
#### `string[] BreakTextIntoLines(string text)`

Following suggestions from stackoverflow, A reliable method for breaking text into lines using Regex is used. https://stackoverflow.com/questions/1508203/best-way-to-split-string-into-lines

**Parameters:**
- `text` (string) — text to break into lines

**Returns:** `string[]` — text lines

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/StringUtils.cs)

#### `int CompareString(ProtoCore.DSASM.StackValue s1, ProtoCore.DSASM.StackValue s2, ProtoCore.RuntimeCore runtimeCore)`

StringUtils.CompareString

**Parameters:**
- `s1` (ProtoCore.DSASM.StackValue)
- `s2` (ProtoCore.DSASM.StackValue)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/StringUtils.cs)

#### `ProtoCore.DSASM.StackValue ConcatString(ProtoCore.DSASM.StackValue op1, ProtoCore.DSASM.StackValue op2, ProtoCore.RuntimeCore runtimeCore)`

StringUtils.ConcatString

**Parameters:**
- `op1` (ProtoCore.DSASM.StackValue)
- `op2` (ProtoCore.DSASM.StackValue)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/StringUtils.cs)

#### `ProtoCore.DSASM.StackValue ConvertToString(ProtoCore.DSASM.StackValue sv, ProtoCore.RuntimeCore runtimeCore, ProtoCore.Runtime.RuntimeMemory rmem)`

StringUtils.ConvertToString

**Parameters:**
- `sv` (ProtoCore.DSASM.StackValue)
- `runtimeCore` (ProtoCore.RuntimeCore)
- `rmem` (ProtoCore.Runtime.RuntimeMemory)

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/StringUtils.cs)

#### `string GetStringValue(ProtoCore.DSASM.StackValue sv, ProtoCore.RuntimeCore runtimeCore)`

StringUtils.GetStringValue

**Parameters:**
- `sv` (ProtoCore.DSASM.StackValue)
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/StringUtils.cs)

#### `bool IsStringSpacesWithTabs(string text)`

StringUtils.IsStringSpacesWithTabs

**Parameters:**
- `text` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/StringUtils.cs)

#### `string ReplaceLineOfText(string text, int lineNumber, string newLine)`

StringUtils.ReplaceLineOfText

**Parameters:**
- `text` (string)
- `lineNumber` (int)
- `newLine` (string)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/StringUtils.cs)

#### `string SpaceToTabConversion(string text, int tabSpacingSize)`

Replace all spaces with tabs given the text and tab spacing size

**Parameters:**
- `text` (string) — 
- `tabSpacingSize` (int) — 

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/StringUtils.cs)

#### `string TabToSpaceConversion(string text, int tabSpacingSize)`

Replace all tabs with spaces given the text and tab spacing size

**Parameters:**
- `text` (string) — 
- `tabSpacingSize` (int) — 

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/StringUtils.cs)

## Validity (class)

Type Validity

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/Validity.cs)

### Constructors
- `void Validity()` — Validity.Validity

### Methods
#### `void Assert(bool cond)`

Validity.Assert

**Parameters:**
- `cond` (bool)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/Validity.cs)

#### `void Assert(bool cond, string message)`

Validity.Assert

**Parameters:**
- `cond` (bool)
- `message` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/Validity.cs)

## VariableLine (struct)

Type VariableLine

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CompilerUtils.cs)

### Constructors
- `void VariableLine(int line)` — VariableLine.VariableLine
- `void VariableLine(string variable, int line)` — VariableLine.VariableLine

### Methods
#### `bool Equals(object obj)`

VariableLine.Equals

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CompilerUtils.cs)

#### `int GetHashCode()`

VariableLine.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CompilerUtils.cs)

#### `bool op_Equality(ProtoCore.Utils.VariableLine variableLine1, ProtoCore.Utils.VariableLine variableLine2)`

VariableLine.op_Equality

**Parameters:**
- `variableLine1` (ProtoCore.Utils.VariableLine)
- `variableLine2` (ProtoCore.Utils.VariableLine)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CompilerUtils.cs)

#### `bool op_Inequality(ProtoCore.Utils.VariableLine variableLine1, ProtoCore.Utils.VariableLine variableLine2)`

VariableLine.op_Inequality

**Parameters:**
- `variableLine1` (ProtoCore.Utils.VariableLine)
- `variableLine2` (ProtoCore.Utils.VariableLine)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Engine/ProtoCore/Utils/CompilerUtils.cs)

