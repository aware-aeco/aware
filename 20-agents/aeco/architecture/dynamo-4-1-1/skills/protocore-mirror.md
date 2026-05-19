---
name: dynamo-protocore-mirror
description: This skill encodes the dynamo 4.1.1 surface of the ProtoCore.Mirror namespace — 9 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: MirrorObject, RuntimeMirror, StaticMirror, ClassMirror, MethodMirror, PropertyMirror, LibraryMirror, MirrorData, and 1 more types.
---

# ProtoCore.Mirror

Auto-generated from vendor docs for dynamo 4.1.1. 9 types in this namespace.

## ClassMirror (class)

A ClassMirror object reflects upon the type of a single designscript variable The information here is populated during the code generation phase

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

### Constructors
- `void ClassMirror(string className, ProtoCore.Core core)` — Constructor to construct ClassMirror from runtime data i.e. StackValue

### Methods
#### `ProtoCore.AST.AssociativeAST.ClassAttributes GetClassAttributes()`

ClassMirror.GetClassAttributes

**Returns:** `ProtoCore.AST.AssociativeAST.ClassAttributes` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.ClassMirror> GetClassHierarchy()`

Returns the base class hierarchy for the given class

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.ClassMirror>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.MethodMirror> GetConstructors()`

Returns the list of constructors defined for the given class

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.MethodMirror>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.MethodMirror> GetFunctions()`

Returns the list of functions of the class only

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.MethodMirror>` — function nodes

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.StaticMirror> GetInstanceMembers()`

Returns the instance methods and properties belonging to the type and its base types. Filters out repeating names.

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.StaticMirror>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.StaticMirror> GetMembers()`

Returns a list of constructors and static methods and properties belonging to the type and its base types. Filters out repeating names.

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.StaticMirror>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.MethodMirror> GetOverloads(string methodName)`

Given the method name, return the list of all matching constructors and member functions of this type only

**Parameters:**
- `methodName` (string) — 

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.MethodMirror>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.MethodMirror> GetOverloadsOnInstance(string methodName)`

Given a method name, return the matching list of instance methods on this type and its base types

**Parameters:**
- `methodName` (string) — 

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.MethodMirror>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.MethodMirror> GetOverloadsOnType(string methodName)`

Given a method name, return the matching list of constructors or static methods on this type and its base types Excludes hidden methods from base types.

**Parameters:**
- `methodName` (string) — 

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.MethodMirror>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.PropertyMirror> GetProperties()`

Returns the list of class properties of this class

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.PropertyMirror>` — symbol nodes

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

### Properties
- `Alias` (string, get/set) — Class name
- `ClassName` (string, get) — Fully qualified class name
- `ClassNodeID` (int, get) — The ID of the underlying ClassNode. This is only valid if a known imported type was found in the vm. Returns -1 if there is no valid underlying classNode this mirror reflects.
- `IsEmpty` (bool, get) — True if the class is a dummy, placeholder class
- `IsHiddenInLibrary` (bool, get) — ClassMirror.IsHiddenInLibrary

## LibraryMirror (class)

The LibraryMirror reflects upon an assembly or DS file to return assembly specific information such as imported classes, global methods, etc.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

### Constructors
- `void LibraryMirror(ProtoCore.Core core, string libName, System.Collections.Generic.IList<ProtoCore.DSASM.ClassNode> classNodes)` — LibraryMirror.LibraryMirror
- `void LibraryMirror(string libName, ProtoCore.Core core)` — LibraryMirror.LibraryMirror

### Methods
#### `System.Collections.Generic.List<ProtoCore.Mirror.ClassMirror> GetClasses()`

Returns list of classes imported from a given assembly

**Returns:** `System.Collections.Generic.List<ProtoCore.Mirror.ClassMirror>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

### Properties
- `LibraryName` (string, get/set) — LibraryMirror.LibraryName

## MethodMirror (class)

Reflects upon a Function to retrieve its arguments

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

### Methods
#### `System.Collections.Generic.List<ProtoCore.Type> GetArgumentTypes()`

MethodMirror.GetArgumentTypes

**Returns:** `System.Collections.Generic.List<ProtoCore.Type>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

#### `ProtoCore.AST.AssociativeAST.MethodAttributes GetMethodAttributes()`

MethodMirror.GetMethodAttributes

**Returns:** `ProtoCore.AST.AssociativeAST.MethodAttributes` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

#### `string ToString()`

MethodMirror.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

### Properties
- `ArgumentList` (System.Collections.Generic.IEnumerable<System.Collections.Generic.KeyValuePair<string, string>>, get) — MethodMirror.ArgumentList
- `IsConstructor` (bool, get) — MethodMirror.IsConstructor
- `IsStatic` (bool, get) — MethodMirror.IsStatic
- `MethodName` (string, get) — MethodMirror.MethodName
- `ReturnType` (System.Nullable<ProtoCore.Type>, get) — MethodMirror.ReturnType

## MirrorData (class)

An object that performs marshalling of all relevant data associated with this object

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/MirrorData.cs)

### Constructors
- `void MirrorData(ProtoCore.Core core, ProtoCore.DSASM.StackValue sv)` — Experimental constructor that takes in a core object Takes a core object to read static data
- `void MirrorData(ProtoCore.Core core, ProtoCore.RuntimeCore runtimeCore, ProtoCore.DSASM.StackValue sv)` — Takes a runtime core object to read runtime data

### Methods
#### `bool Equals(object obj)`



**Parameters:**
- `obj` (object) — 

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/MirrorData.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.MirrorData> GetElements()`

Returns the list of MirrorData if this data represents a collection, else null.

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.MirrorData>` — List of MirrorData represented by this data.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/MirrorData.cs)

#### `ProtoCore.DSASM.StackValue GetStackValue()`

Retrieve the stack value for this mirror

**Returns:** `ProtoCore.DSASM.StackValue` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/MirrorData.cs)

### Properties
- `Class` (ProtoCore.Mirror.ClassMirror, get) — Returns ClassMirror if this data is an instance of a DesignScript Class.
- `Data` (object, get) — Returns the CLR object for all the value type or FFI objects, else null
- `IsCollection` (bool, get) — Determines if this data points to a collection.
- `IsDictionary` (bool, get) — MirrorData.IsDictionary
- `IsFunction` (bool, get) — MirrorData.IsFunction
- `IsNull` (bool, get) — Returns if this data points to null.
- `IsPointer` (bool, get) — Determines if this data is a pointer
- `StringData` (string, get) — Returns string representation of data

## MirrorObject (class)

An abstract MirrorObject that represents a generic DesignScript object that can reflected Reflection on this object can be done at compiletime or runtime

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

### Constructors
- `void MirrorObject()` — MirrorObject.MirrorObject
- `void MirrorObject(ProtoCore.RuntimeCore runtimeCore, ProtoCore.Core staticCore)` — MirrorObject.MirrorObject

## PropertyMirror (class)

Type PropertyMirror

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

### Methods
#### `string ToString()`

For an instance property of a class, called "Property" this returns the string "Property : 'return type' (this : 'class type')." For a static property of a class, this returns "Property : 'return type'."

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

### Properties
- `IsStatic` (bool, get) — PropertyMirror.IsStatic
- `PropertyName` (string, get) — PropertyMirror.PropertyName

## Reflection (static-class)

Type Reflection

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Reflection.cs)

### Methods
#### `ProtoCore.Mirror.LibraryMirror Reflect(string assemblyName, System.Collections.Generic.IList<ProtoCore.DSASM.ClassNode> classNodes, ProtoCore.Core core)`

Returns a library mirror that can be reflected upon The LibraryMirror is used for static reflection of classes etc.

**Parameters:**
- `assemblyName` (string)
- `classNodes` (System.Collections.Generic.IList<ProtoCore.DSASM.ClassNode>)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.Mirror.LibraryMirror` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Reflection.cs)

#### `ProtoCore.Mirror.RuntimeMirror Reflect(string varname, int blockDecl, ProtoCore.RuntimeCore runtimeCore, ProtoCore.Core core)`

Returns a runtime mirror that can be reflected upon

**Parameters:**
- `varname` (string)
- `blockDecl` (int)
- `runtimeCore` (ProtoCore.RuntimeCore)
- `core` (ProtoCore.Core)

**Returns:** `ProtoCore.Mirror.RuntimeMirror` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Reflection.cs)

## RuntimeMirror (class)

A RuntimeMirror object is used to reflect on the runtime status of a single designsript variable

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

### Constructors
- `void RuntimeMirror(string varname, int blockDecl, ProtoCore.RuntimeCore runtimeCore, ProtoCore.Core staticCore)` — RuntimeMirror.RuntimeMirror

### Methods
#### `ProtoCore.Mirror.MirrorData GetData()`

Retrieve the data associated with this RuntimeMirror

**Returns:** `ProtoCore.Mirror.MirrorData` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

#### `string GetStringData()`

This method will return the string representation of the mirror data if it is available

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

### Properties
- `TargetExecutive` (ProtoCore.DSASM.Executive, get) — The runtime executive that we are reflecting on

## StaticMirror (class)

StaticMirror is a base class representing all Mirror classes that perform static (build time) reflection on types Static reflection can be done without executing the code

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

### Constructors
- `void StaticMirror()` — StaticMirror.StaticMirror
- `void StaticMirror(ProtoCore.Core core, string name)` — StaticMirror.StaticMirror

### Methods
#### `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.ClassMirror> GetAllTypes(ProtoCore.Core core)`

Get all types except for internal VM types

**Parameters:**
- `core` (ProtoCore.Core) — 

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.ClassMirror>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.MethodMirror> GetBuiltInMethods(ProtoCore.Core core)`

List of built-in methods that are preloaded by default

**Parameters:**
- `core` (ProtoCore.Core)

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.MethodMirror>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.ClassMirror> GetClasses(ProtoCore.Core core)`

Get all reference type classes imported in the VM except for internal and placeholder types

**Parameters:**
- `core` (ProtoCore.Core) — 

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.ClassMirror>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.StaticMirror> GetGlobals(ProtoCore.Core core)`

Get all methods and properties for all classes

**Parameters:**
- `core` (ProtoCore.Core) — 

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.StaticMirror>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

#### `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.MethodMirror> GetOverloadsOnBuiltIns(ProtoCore.Core core, string methodName)`

Returns list of overloads (one or more) for a given built-in method

**Parameters:**
- `core` (ProtoCore.Core) — 
- `methodName` (string) — 

**Returns:** `System.Collections.Generic.IEnumerable<ProtoCore.Mirror.MethodMirror>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

#### `string ToString()`

StaticMirror.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/Reflection/Mirror.cs)

### Properties
- `Name` (string, get) — Name of the Mirror object - In the case of: ClassMirror: class name MethodMirror: method name PropertyMirror: property name

