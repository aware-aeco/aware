---
name: dynamo-protocore-namespace
description: This skill encodes the dynamo 4.1.1 surface of the ProtoCore.Namespace namespace — 4 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ElementResolver, ElementRewriter, PermuteUtils, Symbol.
---

# ProtoCore.Namespace

Auto-generated from vendor docs for dynamo 4.1.1. 4 types in this namespace.

## ElementResolver (class)

Responsible for resolving a partial class name to its fully resolved name

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/ElementResolver.cs)

### Constructors
- `void ElementResolver()` — ElementResolver.ElementResolver
- `void ElementResolver(System.Collections.Generic.IDictionary<string, System.Collections.Generic.KeyValuePair<string, string>> namespaceLookupMap)` — ElementResolver.ElementResolver

### Methods
#### `void AddToResolutionMap(string partialName, string resolvedName, string assemblyName)`

ElementResolver.AddToResolutionMap

**Parameters:**
- `partialName` (string)
- `resolvedName` (string)
- `assemblyName` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/ElementResolver.cs)

#### `void CopyResolutionMap(ProtoCore.Namespace.ElementResolver otherResolver)`

ElementResolver.CopyResolutionMap

**Parameters:**
- `otherResolver` (ProtoCore.Namespace.ElementResolver)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/ElementResolver.cs)

#### `string LookupAssemblyName(string partialName)`

ElementResolver.LookupAssemblyName

**Parameters:**
- `partialName` (string)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/ElementResolver.cs)

#### `string LookupResolvedName(string partialName)`

Looks up resolved name in resolution map given the partial name

**Parameters:**
- `partialName` (string) — 

**Returns:** `string` — returns null if partial name is not found in resolution map

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/ElementResolver.cs)

#### `string LookupShortName(string resolvedName)`

ElementResolver.LookupShortName

**Parameters:**
- `resolvedName` (string)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/ElementResolver.cs)

### Properties
- `ResolutionMap` (System.Collections.Generic.IDictionary<string, System.Collections.Generic.KeyValuePair<string, string>>, get) — Maintains a lookup table of partial class identifiers vs. fully qualified class identifier names and assembly name

## ElementRewriter (class)

Type ElementRewriter

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/ElementRewriter.cs)

### Constructors
- `void ElementRewriter(ProtoCore.DSASM.ClassTable classTable, ProtoCore.Namespace.ElementRewriter.SymbolConflictWarningHandler warningHandler, ProtoCore.Namespace.ElementResolver resolver)` — ElementRewriter.ElementRewriter

### Methods
#### `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> RewriteElementNames(ProtoCore.DSASM.ClassTable classTable, ProtoCore.Namespace.ElementResolver elementResolver, System.Collections.Generic.IEnumerable<ProtoCore.AST.Node> astNodes, ProtoCore.Namespace.ElementRewriter.SymbolConflictWarningHandler handler)`

Lookup namespace resolution map to substitute partial classnames with their fully qualified names in ASTs. If partial class is not found in map, update ResolutionMap with fully resolved name from compiler.

**Parameters:**
- `classTable` (ProtoCore.DSASM.ClassTable) — 
- `elementResolver` (ProtoCore.Namespace.ElementResolver) — 
- `astNodes` (System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>) — parent AST node
- `handler` (ProtoCore.Namespace.ElementRewriter.SymbolConflictWarningHandler) — 

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.AST.Node>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/ElementRewriter.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitIdentifierListNode(ProtoCore.AST.AssociativeAST.IdentifierListNode node)`

ElementRewriter.VisitIdentifierListNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.IdentifierListNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/ElementRewriter.cs)

#### `ProtoCore.AST.AssociativeAST.AssociativeNode VisitTypedIdentifierNode(ProtoCore.AST.AssociativeAST.TypedIdentifierNode node)`

ElementRewriter.VisitTypedIdentifierNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.TypedIdentifierNode)

**Returns:** `ProtoCore.AST.AssociativeAST.AssociativeNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/ElementRewriter.cs)

## PermuteUtils (static-class)

Type PermuteUtils

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/SymbolTable.cs)

### Methods
#### `System.Collections.Generic.IEnumerable<T> AllButLast<T>(System.Collections.Generic.IEnumerable<T> source)`

Given a generic enumerable collection of items, returns all but the last item in the collection

**Parameters:**
- `source` (System.Collections.Generic.IEnumerable<T>) — input collection

**Returns:** `System.Collections.Generic.IEnumerable<T>` — new collection of all except the last item in the input collection

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/SymbolTable.cs)

#### `System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<T>> Permute<T>(System.Collections.Generic.IEnumerable<T> list, int count)`

Returns an enumeration of enumerators, one for each permutation of the input. E.g. given an input array: {A, B, C, D, E}, and count of 2, this returns the following permutations in the same order: { AB, AC, AD, AE, BC, BD, BE, CD, CE, DE }

**Parameters:**
- `list` (System.Collections.Generic.IEnumerable<T>) — input collection to choose permutations from
- `count` (int) — Number of items to return permutations of

**Returns:** `System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<T>>` — All possible permuatations of the input list picking (count) items at a time

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/SymbolTable.cs)

## Symbol (class)

Symbol class : It represents a symbol with namespace.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/SymbolTable.cs)

### Constructors
- `void Symbol(string fullname)` — Constructs a FullyQualifiedSymbolName with the given fullname.

### Methods
#### `bool Equals(object obj)`

Checks equality based on FullName

**Parameters:**
- `obj` (object) — 

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/SymbolTable.cs)

#### `int GetHashCode()`

Returns hascode based on FullName

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/SymbolTable.cs)

#### `System.Collections.Generic.Dictionary<ProtoCore.Namespace.Symbol, string> GetShortestUniqueNames(System.Collections.Generic.IEnumerable<ProtoCore.Namespace.Symbol> symbolList)`

Given a list of conflicting namespaces, finds the shortest partial name for a namespace that can be resolved uniquely. For example, given {"A.B.C.D.E", "X.Y.A.B.E.C.E", "X.Y.A.C.B.E"}, all with the same class E, their shortest unique names would be: {"D.E", "E.E", "C.B.E"}

**Parameters:**
- `symbolList` (System.Collections.Generic.IEnumerable<ProtoCore.Namespace.Symbol>) — Input list of conflicting namespaces (having same class name)

**Returns:** `System.Collections.Generic.Dictionary<ProtoCore.Namespace.Symbol, string>` — Map of Symbol vs. short name

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/SymbolTable.cs)

#### `bool Matches(string partialname)`

Checks if all of the namespace prefixes specified in the given partially qualified name appear in this namespace in the same order. For Example: A full namespace "Com.Autodesk.Designscript.ProtoGeometry.Point" will match all of the following partial namespaces Com.Autodesk.Designscript.ProtoGeometry.Point Point DesignScript.Point ProtoGeometry.Point Autodesk.DesignScript.Point whereas it won't match Com.DesignScript.Autodesk.Point

**Parameters:**
- `partialname` (string) — Partially qualified symbol name

**Returns:** `bool` — returns true if partial name matches this

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/SymbolTable.cs)

#### `bool op_Equality(ProtoCore.Namespace.Symbol thisSymbol, ProtoCore.Namespace.Symbol otherSymbol)`

Equality operator overload to have same equality behaviour as object.Equals() override

**Parameters:**
- `thisSymbol` (ProtoCore.Namespace.Symbol) — 
- `otherSymbol` (ProtoCore.Namespace.Symbol) — 

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/SymbolTable.cs)

#### `bool op_Inequality(ProtoCore.Namespace.Symbol thisSymbol, ProtoCore.Namespace.Symbol otherSymbol)`

Inequality operator overload is needed for every equality operator overload

**Parameters:**
- `thisSymbol` (ProtoCore.Namespace.Symbol) — 
- `otherSymbol` (ProtoCore.Namespace.Symbol) — 

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Namespace/SymbolTable.cs)

### Properties
- `FullName` (string, get) — Returns fully qualified symbol name.
- `Id` (int, get/set) — Returns symbol id
- `Name` (string, get) — Returns symbol name

