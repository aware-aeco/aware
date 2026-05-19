---
name: dynamo-protoffi
description: This skill encodes the dynamo 4.1.1 surface of the ProtoFFI namespace — 19 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: CLRModuleType, CLRDLLModule, CSModuleHelper, FFIClassAttributes, FFIMethodAttributes, FFIParamAttributes, ReferenceEqualityComparer, PointerValueComparer, and 11 more types.
---

# ProtoFFI

Auto-generated from vendor docs for dynamo 4.1.1. 19 types in this namespace.

## CLRDLLModule (class)

Implements DLLModule for CLR types and FFI. This class supports .NET module import to DesignScript and provides FFIFunctionPointer & FFIObjectMarshler.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

### Constructors
- `void CLRDLLModule(string name, System.Reflection.Assembly assembly)` — CLRDLLModule.CLRDLLModule
- `void CLRDLLModule(string name, System.Reflection.Module module)` — CLRDLLModule.CLRDLLModule

### Methods
#### `System.Type GetExtensionAppType()`

CLRDLLModule.GetExtensionAppType

**Returns:** `System.Type` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

#### `ProtoFFI.FFIFunctionPointer GetFunctionPointer(string className, string name, System.Collections.Generic.List<ProtoCore.Type> argTypes, ProtoCore.Type returnType)`

CLRDLLModule.GetFunctionPointer

**Parameters:**
- `className` (string)
- `name` (string)
- `argTypes` (System.Collections.Generic.List<ProtoCore.Type>)
- `returnType` (ProtoCore.Type)

**Returns:** `ProtoFFI.FFIFunctionPointer` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

#### `System.Collections.Generic.List<ProtoFFI.FFIFunctionPointer> GetFunctionPointers(string className, string name)`

CLRDLLModule.GetFunctionPointers

**Parameters:**
- `className` (string)
- `name` (string)

**Returns:** `System.Collections.Generic.List<ProtoFFI.FFIFunctionPointer>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

#### `System.Type GetImplemetationType(System.Reflection.Assembly assembly, System.Type interfaceType, System.Type assemblyAttribute, bool searchFromAllExportedType)`

CLRDLLModule.GetImplemetationType

**Parameters:**
- `assembly` (System.Reflection.Assembly)
- `interfaceType` (System.Type)
- `assemblyAttribute` (System.Type)
- `searchFromAllExportedType` (bool)

**Returns:** `System.Type` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

#### `ProtoFFI.FFIObjectMarshaler GetMarshaler(ProtoCore.RuntimeCore runtimeCore)`

CLRDLLModule.GetMarshaler

**Parameters:**
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoFFI.FFIObjectMarshaler` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

#### `ProtoCore.AST.AssociativeAST.CodeBlockNode ImportCodeBlock(string typeName, string alias, ProtoCore.AST.AssociativeAST.CodeBlockNode refNode)`

CLRDLLModule.ImportCodeBlock

**Parameters:**
- `typeName` (string)
- `alias` (string)
- `refNode` (ProtoCore.AST.AssociativeAST.CodeBlockNode)

**Returns:** `ProtoCore.AST.AssociativeAST.CodeBlockNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

### Properties
- `Assembly` (System.Reflection.Assembly, get) — CLRDLLModule.Assembly
- `Module` (System.Reflection.Module, get) — CLRDLLModule.Module
- `Name` (string, get) — CLRDLLModule.Name

## CLRModuleType (class)

This class creates ClassDeclNode for a given type and caches all the imported types. This class also keeps the list of FFIFunctionPointer for the given type.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

### Methods
#### `void ClearTypes()`

This method is for testing, to ensure cache is cleared before every test.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

#### `void EnsureDisposeMethod(ProtoFFI.CLRDLLModule module)`

Ensures that dispose method node is created for this empty type.

**Parameters:**
- `module` (ProtoFFI.CLRDLLModule) — Reference module

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

#### `System.Collections.Generic.List<ProtoFFI.CLRModuleType> GetEmptyTypes()`

Returns all the types, which was referenced by other types but were not imported explicitly. These are empty types and corresponding DS Type don't contain any methods, constructors, properties or fields

**Returns:** `System.Collections.Generic.List<ProtoFFI.CLRModuleType>` — List of CLRModuleType

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

#### `System.Collections.Generic.List<ProtoFFI.FFIFunctionPointer> GetFunctionPointers(string name)`

Returns list of function pointers available on this type for a given function name

**Parameters:**
- `name` (string) — Function name

**Returns:** `System.Collections.Generic.List<ProtoFFI.FFIFunctionPointer>` — List of FFIFunctionPointer

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

#### `System.Type GetImportedType(string typename)`

CLRModuleType.GetImportedType

**Parameters:**
- `typename` (string)

**Returns:** `System.Type` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

#### `ProtoFFI.CLRModuleType GetInstance(System.Type type, ProtoFFI.CLRDLLModule module, string alias)`

Returns CLRModuleType for given Type. If CLRModuleType instance for the given type is not found, it creates a new one. If CLRDLLModule is passed as null, it creates empty CLRModuleType.

**Parameters:**
- `type` (System.Type) — System.Type to be imported in DesignScript
- `module` (ProtoFFI.CLRDLLModule) — CLRDLLModule which imports this type
- `alias` (string) — Alias name, if any. For now its not supported.

**Returns:** `ProtoFFI.CLRModuleType` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

#### `ProtoCore.Type GetProtoCoreType(System.Type type, ProtoFFI.CLRDLLModule module)`

CLRModuleType.GetProtoCoreType

**Parameters:**
- `type` (System.Type)
- `module` (ProtoFFI.CLRDLLModule)

**Returns:** `ProtoCore.Type` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

#### `System.Collections.Generic.List<ProtoFFI.CLRModuleType> GetTypes(System.Func<ProtoFFI.CLRModuleType, bool> predicate)`

Returns all the types for the given predicate.

**Parameters:**
- `predicate` (System.Func<ProtoFFI.CLRModuleType, bool>) — A delegate for defining criteria

**Returns:** `System.Collections.Generic.List<ProtoFFI.CLRModuleType>` — List of CLRModuleType

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

#### `void SetTypeAttributes(System.Type type, ProtoFFI.FFIClassAttributes attributes)`

CLRModuleType.SetTypeAttributes

**Parameters:**
- `type` (System.Type)
- `attributes` (ProtoFFI.FFIClassAttributes)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

#### `bool SupressesImport(System.Reflection.MemberInfo member)`

CLRModuleType.SupressesImport

**Parameters:**
- `member` (System.Reflection.MemberInfo)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

#### `bool TryGetImportedDSType(System.Type type, ref ProtoCore.Type dsType)`

CLRModuleType.TryGetImportedDSType

**Parameters:**
- `type` (System.Type)
- `dsType` (ref ProtoCore.Type)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

#### `bool TryGetTypeAttributes(System.Type type, ref ProtoFFI.FFIClassAttributes attributes)`

CLRModuleType.TryGetTypeAttributes

**Parameters:**
- `type` (System.Type)
- `attributes` (ref ProtoFFI.FFIClassAttributes)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

### Properties
- `CLRType` (System.Type, get) — System.Type that was imported
- `ClassNode` (ProtoCore.AST.AssociativeAST.ClassDeclNode, get) — Imported ClassDeclNode
- `DisposeMethod` (System.Reflection.MethodInfo, get) — CLRModuleType.DisposeMethod
- `FullName` (string, get) — DesignScript Class name, together with Namespace name
- `Module` (ProtoFFI.CLRDLLModule, get) — CLRDLLModule from which this type was imported.
- `ProtoCoreType` (ProtoCore.Type, get) — Imported ProtoCore.Type

## CSModuleHelper (class)

Helper class to load CLR modules.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

### Constructors
- `void CSModuleHelper()` — CSModuleHelper.CSModuleHelper

### Methods
#### `ProtoFFI.FFIObjectMarshaler GetMarshaler(ProtoCore.RuntimeCore runtimeCore)`

CSModuleHelper.GetMarshaler

**Parameters:**
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoFFI.FFIObjectMarshaler` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

#### `ProtoFFI.DLLModule getModule(string name)`

Returns a CLRDLLModule after loading the given assembly.

**Parameters:**
- `name` (string) — Name of assembly.

**Returns:** `ProtoFFI.DLLModule` — CLRDLLModule for given assembly/module name.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

## DLLFFIHandler (class)

Type DLLFFIHandler

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

### Methods
#### `ProtoFFI.FFIFunctionPointer GetFunctionPointer(string dllModuleName, string className, string functionName, System.Collections.Generic.List<ProtoCore.Type> parameterTypes, ProtoCore.Type returnType)`

DLLFFIHandler.GetFunctionPointer

**Parameters:**
- `dllModuleName` (string)
- `className` (string)
- `functionName` (string)
- `parameterTypes` (System.Collections.Generic.List<ProtoCore.Type>)
- `returnType` (ProtoCore.Type)

**Returns:** `ProtoFFI.FFIFunctionPointer` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

#### `ProtoFFI.DLLModule GetModule(string dllModuleName)`

DLLFFIHandler.GetModule

**Parameters:**
- `dllModuleName` (string)

**Returns:** `ProtoFFI.DLLModule` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

#### `ProtoFFI.ModuleHelper GetModuleHelper(ProtoFFI.FFILanguage language)`

DLLFFIHandler.GetModuleHelper

**Parameters:**
- `language` (ProtoFFI.FFILanguage)

**Returns:** `ProtoFFI.ModuleHelper` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

#### `void Register()`

DLLFFIHandler.Register

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

#### `void Register(ProtoFFI.FFILanguage type, ProtoFFI.ModuleHelper helper)`

DLLFFIHandler.Register

**Parameters:**
- `type` (ProtoFFI.FFILanguage)
- `helper` (ProtoFFI.ModuleHelper)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

### Properties
- `Env` (IntPtr, get/set) — DLLFFIHandler.Env

## DLLModule (class)

Type DLLModule

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

### Constructors
- `void DLLModule()` — DLLModule.DLLModule

### Methods
#### `System.Type GetExtensionAppType()`

DLLModule.GetExtensionAppType

**Returns:** `System.Type` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

#### `ProtoFFI.FFIFunctionPointer GetFunctionPointer(string className, string name, System.Collections.Generic.List<ProtoCore.Type> argTypes, ProtoCore.Type returnType)`

DLLModule.GetFunctionPointer

**Parameters:**
- `className` (string)
- `name` (string)
- `argTypes` (System.Collections.Generic.List<ProtoCore.Type>)
- `returnType` (ProtoCore.Type)

**Returns:** `ProtoFFI.FFIFunctionPointer` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

#### `System.Collections.Generic.List<ProtoFFI.FFIFunctionPointer> GetFunctionPointers(string className, string name)`

DLLModule.GetFunctionPointers

**Parameters:**
- `className` (string)
- `name` (string)

**Returns:** `System.Collections.Generic.List<ProtoFFI.FFIFunctionPointer>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

#### `ProtoFFI.FFIObjectMarshaler GetMarshaler(ProtoCore.RuntimeCore runtimeCore)`

DLLModule.GetMarshaler

**Parameters:**
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoFFI.FFIObjectMarshaler` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

#### `ProtoCore.AST.AssociativeAST.CodeBlockNode ImportCodeBlock(string typeName, string alias, ProtoCore.AST.AssociativeAST.CodeBlockNode refnode)`

DLLModule.ImportCodeBlock

**Parameters:**
- `typeName` (string)
- `alias` (string)
- `refnode` (ProtoCore.AST.AssociativeAST.CodeBlockNode)

**Returns:** `ProtoCore.AST.AssociativeAST.CodeBlockNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

## FFIClassAttributes (class)

It keeps FFI class's attributes.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

### Constructors
- `void FFIClassAttributes(System.Type type)` — FFIClassAttributes.FFIClassAttributes

### Properties
- `Attributes` (System.Collections.Generic.IEnumerable<System.Attribute>, get) — FFI class attributes.

## FFIFunctionPointer (class)

Type FFIFunctionPointer

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

### Constructors
- `void FFIFunctionPointer()` — FFIFunctionPointer.FFIFunctionPointer

### Methods
#### `object Execute(ProtoCore.Runtime.Context c, ProtoCore.DSASM.Interpreter dsi, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> stack)`

FFIFunctionPointer.Execute

**Parameters:**
- `c` (ProtoCore.Runtime.Context)
- `dsi` (ProtoCore.DSASM.Interpreter)
- `stack` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)

**Returns:** `object` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

#### `T[] GetUnderlyingArray<T>(System.Collections.Generic.List<T> list)`

FFIFunctionPointer.GetUnderlyingArray

**Parameters:**
- `list` (System.Collections.Generic.List<T>)

**Returns:** `T[]` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

### Properties
- `IsDNI` (bool, get/set) — FFIFunctionPointer.IsDNI

## FFIHandler (class)

Type FFIHandler

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

### Constructors
- `void FFIHandler()` — FFIHandler.FFIHandler

### Methods
#### `ProtoFFI.FFIFunctionPointer GetFunctionPointer(string moduleName, string className, string functionName, System.Collections.Generic.List<ProtoCore.Type> parameterTypes, ProtoCore.Type returnType)`

FFIHandler.GetFunctionPointer

**Parameters:**
- `moduleName` (string)
- `className` (string)
- `functionName` (string)
- `parameterTypes` (System.Collections.Generic.List<ProtoCore.Type>)
- `returnType` (ProtoCore.Type)

**Returns:** `ProtoFFI.FFIFunctionPointer` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

## FFILanguage (enum)

Type FFILanguage

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

### Values
- `CPlusPlus` = `0`
- `CSharp` = `1`

## FFIMethodAttributes (class)

It keeps FFI method's attributes.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

### Constructors
- `void FFIMethodAttributes(System.Reflection.FieldInfo f)` — FFIMethodAttributes.FFIMethodAttributes
- `void FFIMethodAttributes(System.Reflection.MethodBase method, System.Collections.Generic.Dictionary<System.Reflection.MethodInfo, System.Attribute[]> getterAttributes)` — FFIMethodAttributes.FFIMethodAttributes

### Properties
- `AllowRankReduction` (bool, get) — FFIMethodAttributes.AllowRankReduction
- `Attributes` (System.Collections.Generic.IEnumerable<System.Attribute>, get) — FFI method attributes.
- `RequireTracing` (bool, get) — FFIMethodAttributes.RequireTracing

## FFIObjectMarshaler (class)

This class is responsible for marshaling of FFI objects to DS world and vice-versa.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

### Constructors
- `void FFIObjectMarshaler()` — FFIObjectMarshaler.FFIObjectMarshaler

### Methods
#### `ProtoCore.Type GetMarshaledType(System.Type type)`

Returns the marshaled Type of DS object for a given FFI object type

**Parameters:**
- `type` (System.Type) — Type of FFI object

**Returns:** `ProtoCore.Type` — Type of DS object

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

#### `string GetStringValue(ProtoCore.DSASM.StackValue dsObject)`

Returns a string representation for given DS object

**Parameters:**
- `dsObject` (ProtoCore.DSASM.StackValue) — DS Object

**Returns:** `string` — string representation of a DS object

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

#### `ProtoCore.DSASM.StackValue Marshal(object obj, ProtoCore.Runtime.Context context, ProtoCore.DSASM.Interpreter dsi, ProtoCore.Type type)`

Marshales a given FFI object to DS Object of given ProtoCore.Type

**Parameters:**
- `obj` (object) — The FFI object
- `context` (ProtoCore.Runtime.Context) — DS execution context
- `dsi` (ProtoCore.DSASM.Interpreter) — The current runtime interpreter
- `type` (ProtoCore.Type) — Type of DS Object expected

**Returns:** `ProtoCore.DSASM.StackValue` — Marshaled object as Operand

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

#### `void OnDispose(ProtoCore.DSASM.StackValue dsObject, ProtoCore.Runtime.Context context, ProtoCore.DSASM.Interpreter dsi)`

This is a callback method called when the given DS object is disposed. Marshaler gets an opportunity to clear the cache related to the given DS object.

**Parameters:**
- `dsObject` (ProtoCore.DSASM.StackValue) — DS object being disposed
- `context` (ProtoCore.Runtime.Context) — DS execution context
- `dsi` (ProtoCore.DSASM.Interpreter) — The current runtime interpreter

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

#### `object UnMarshal(ProtoCore.DSASM.StackValue dsObject, ProtoCore.Runtime.Context context, ProtoCore.DSASM.Interpreter dsi, System.Type type)`

UnMarshales a given DS object to FFI object of given System.Type

**Parameters:**
- `dsObject` (ProtoCore.DSASM.StackValue) — The DS object
- `context` (ProtoCore.Runtime.Context) — DS execution context
- `dsi` (ProtoCore.DSASM.Interpreter) — The current runtime interpreter
- `type` (System.Type) — Type of FFI object expected

**Returns:** `object` — Unmarshaled FFI object

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

## FFIParamAttributes (class)

A parameter's attributes.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRDLLModule.cs)

### Constructors
- `void FFIParamAttributes(System.Reflection.ParameterInfo parameter)` — FFIParamAttributes.FFIParamAttributes

### Properties
- `DefaultArgumentExpression` (string, get) — FFIParamAttributes.DefaultArgumentExpression
- `IsArbitraryDimensionArray` (bool, get) — FFIParamAttributes.IsArbitraryDimensionArray

## ImportModuleHandler (class)

Type ImportModuleHandler

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/ImportModuleHandler.cs)

### Constructors
- `void ImportModuleHandler(ProtoCore.Core coreObj)` — ImportModuleHandler.ImportModuleHandler

### Methods
#### `ProtoCore.AST.AssociativeAST.ImportNode Import(string moduleName, string typeName, string alias, int line, int col)`

ImportModuleHandler.Import

**Parameters:**
- `moduleName` (string)
- `typeName` (string)
- `alias` (string)
- `line` (int)
- `col` (int)

**Returns:** `ProtoCore.AST.AssociativeAST.ImportNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/ImportModuleHandler.cs)

#### `bool TryGetClassNode(ProtoCore.AST.AssociativeAST.CodeBlockNode node, string typeName, ref ProtoCore.AST.AssociativeAST.ClassDeclNode classNode)`

ImportModuleHandler.TryGetClassNode

**Parameters:**
- `node` (ProtoCore.AST.AssociativeAST.CodeBlockNode)
- `typeName` (string)
- `classNode` (ref ProtoCore.AST.AssociativeAST.ClassDeclNode)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/ImportModuleHandler.cs)

### Properties
- `RootImportNode` (ProtoCore.AST.AssociativeAST.ImportNode, get) — ImportModuleHandler.RootImportNode

## ModuleHelper (class)

Type ModuleHelper

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

### Constructors
- `void ModuleHelper()` — ModuleHelper.ModuleHelper

### Methods
#### `ProtoFFI.FFIObjectMarshaler GetMarshaler(ProtoCore.RuntimeCore runtimeCore)`

ModuleHelper.GetMarshaler

**Parameters:**
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoFFI.FFIObjectMarshaler` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

#### `ProtoFFI.DLLModule getModule(string name)`

ModuleHelper.getModule

**Parameters:**
- `name` (string)

**Returns:** `ProtoFFI.DLLModule` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/FFIHandler.cs)

## PInvokeDLLModule (class)

Type PInvokeDLLModule

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/PInvokeFFI.cs)

### Constructors
- `void PInvokeDLLModule(string name)` — PInvokeDLLModule.PInvokeDLLModule

### Methods
#### `ProtoFFI.FFIFunctionPointer GetFunctionPointer(string className, string name, System.Collections.Generic.List<ProtoCore.Type> argTypes, ProtoCore.Type returnType)`

PInvokeDLLModule.GetFunctionPointer

**Parameters:**
- `className` (string)
- `name` (string)
- `argTypes` (System.Collections.Generic.List<ProtoCore.Type>)
- `returnType` (ProtoCore.Type)

**Returns:** `ProtoFFI.FFIFunctionPointer` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/PInvokeFFI.cs)

#### `System.Collections.Generic.List<ProtoFFI.FFIFunctionPointer> GetFunctionPointers(string className, string name)`

PInvokeDLLModule.GetFunctionPointers

**Parameters:**
- `className` (string)
- `name` (string)

**Returns:** `System.Collections.Generic.List<ProtoFFI.FFIFunctionPointer>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/PInvokeFFI.cs)

## PInvokeFunctionPointer (class)

Type PInvokeFunctionPointer

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/PInvokeFFI.cs)

### Constructors
- `void PInvokeFunctionPointer(ProtoFFI.PInvokeDLLModule module, string name)` — PInvokeFunctionPointer.PInvokeFunctionPointer
- `void PInvokeFunctionPointer(ProtoFFI.PInvokeDLLModule module, string name, ProtoCore.Type returnType)` — PInvokeFunctionPointer.PInvokeFunctionPointer
- `void PInvokeFunctionPointer(ProtoFFI.PInvokeDLLModule module, string name, System.Collections.Generic.List<ProtoCore.Type> argTypes, ProtoCore.Type returnType)` — PInvokeFunctionPointer.PInvokeFunctionPointer

### Methods
#### `object Execute(ProtoCore.Runtime.Context context, ProtoCore.DSASM.Interpreter dsi, System.Collections.Generic.List<ProtoCore.DSASM.StackValue> newStack)`

PInvokeFunctionPointer.Execute

**Parameters:**
- `context` (ProtoCore.Runtime.Context)
- `dsi` (ProtoCore.DSASM.Interpreter)
- `newStack` (System.Collections.Generic.List<ProtoCore.DSASM.StackValue>)

**Returns:** `object` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/PInvokeFFI.cs)

### Properties
- `Module` (ProtoFFI.PInvokeDLLModule, get) — PInvokeFunctionPointer.Module
- `Name` (string, get) — PInvokeFunctionPointer.Name

## PInvokeModuleHelper (class)

Type PInvokeModuleHelper

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/PInvokeFFI.cs)

### Constructors
- `void PInvokeModuleHelper()` — PInvokeModuleHelper.PInvokeModuleHelper

### Methods
#### `ProtoFFI.FFIObjectMarshaler GetMarshaler(ProtoCore.RuntimeCore runtimeCore)`

PInvokeModuleHelper.GetMarshaler

**Parameters:**
- `runtimeCore` (ProtoCore.RuntimeCore)

**Returns:** `ProtoFFI.FFIObjectMarshaler` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/PInvokeFFI.cs)

#### `ProtoFFI.DLLModule getModule(string name)`

PInvokeModuleHelper.getModule

**Parameters:**
- `name` (string)

**Returns:** `ProtoFFI.DLLModule` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/PInvokeFFI.cs)

## PointerValueComparer (class)

This class compares two Pointer type StackValue objects.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRObjectMarshaler.cs)

### Constructors
- `void PointerValueComparer()` — PointerValueComparer.PointerValueComparer

### Methods
#### `bool Equals(ProtoCore.DSASM.StackValue x, ProtoCore.DSASM.StackValue y)`

PointerValueComparer.Equals

**Parameters:**
- `x` (ProtoCore.DSASM.StackValue)
- `y` (ProtoCore.DSASM.StackValue)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRObjectMarshaler.cs)

#### `int GetHashCode(ProtoCore.DSASM.StackValue obj)`

PointerValueComparer.GetHashCode

**Parameters:**
- `obj` (ProtoCore.DSASM.StackValue)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRObjectMarshaler.cs)

## ReferenceEqualityComparer (class)

This class compares two CLR objects using reference equality. It is used in CLRObjectMap to map CLR object instances to their marshaled StackValue representations. Uses System.Runtime.CompilerServices.RuntimeHelpers.GetHashCode(System.Object) to get the object's identity hash code, which ensures well-distributed hash codes even when objects have value-based hash codes that collide (e.g., Point objects with identical coordinates). Note: The hash code computation is intentionally aligned with the reference equality behavior used by System.Object.ReferenceEquals(System.Object,System.Object). This ensures consistent semantics between equality comparison and hash code generation, which is a requirement for proper System.Collections.Generic.IEqualityComparer`1 implementation.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRObjectMarshaler.cs)

### Constructors
- `void ReferenceEqualityComparer()` — ReferenceEqualityComparer.ReferenceEqualityComparer

### Methods
#### `int GetHashCode(object obj)`

ReferenceEqualityComparer.GetHashCode

**Parameters:**
- `obj` (object)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/Engine/ProtoCore/FFI/CLRObjectMarshaler.cs)

