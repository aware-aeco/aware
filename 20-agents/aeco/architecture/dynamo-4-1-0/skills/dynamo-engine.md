---
name: dynamo-dynamo-engine
description: This skill encodes the dynamo 4.1.0 surface of the Dynamo.Engine namespace — 12 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AstBuiltEventHandler, EngineController, CompilationServices, ITraceReconciliationProcessor, LiveRunnerServices, IFunctionDescriptor, FunctionDescriptorParams, FunctionDescriptor, and 4 more types.
---

# Dynamo.Engine

Auto-generated from vendor docs for dynamo 4.1.0. 12 types in this namespace.

## AstBuiltEventHandler (delegate)

This is a delegate used in AstBuilt event.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Engine/EngineController.cs)

### Constructors
- `void AstBuiltEventHandler(object object, IntPtr method)` — AstBuiltEventHandler.AstBuiltEventHandler

### Methods
#### `System.IAsyncResult BeginInvoke(object sender, Dynamo.Engine.CodeGeneration.CompiledEventArgs e, System.AsyncCallback callback, object object)`

AstBuiltEventHandler.BeginInvoke

**Parameters:**
- `sender` (object)
- `e` (Dynamo.Engine.CodeGeneration.CompiledEventArgs)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Engine/EngineController.cs)

#### `void EndInvoke(System.IAsyncResult result)`

AstBuiltEventHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Engine/EngineController.cs)

#### `void Invoke(object sender, Dynamo.Engine.CodeGeneration.CompiledEventArgs e)`

AstBuiltEventHandler.Invoke

**Parameters:**
- `sender` (object)
- `e` (Dynamo.Engine.CodeGeneration.CompiledEventArgs)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Engine/EngineController.cs)

## CompilationServices (class)

This class is used to precompile code block node. Also it's used as helper for resolving code in Input and Output nodes.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Engine/EngineController.cs)

### Constructors
- `void CompilationServices(Dynamo.Engine.LibraryServices libraryServices)` — Creates CompilationServices.

## EngineController (class)

A controller to coordinate the interactions between some DesignScript sub components like library management, live runner and so on.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Engine/EngineController.cs)

### Constructors
- `void EngineController(Dynamo.Engine.LibraryServices libraryServices, string geometryFactoryFileName, bool verboseLogging)` — This function creates EngineController

### Methods
#### `void Dispose()`

Disposes EngineController.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Engine/EngineController.cs)

#### `void EnableProfiling(bool enable, Dynamo.Graph.Workspaces.HomeWorkspaceModel workspace, System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel> nodes)`

Enables or disables profiling depending on the given argument

**Parameters:**
- `enable` (bool) — Indicates enabling or disabling of profiling.
- `workspace` (Dynamo.Graph.Workspaces.HomeWorkspaceModel) — The workspace to enable or disable profiling for.
- `nodes` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel>) — The list of nodes to enable or disable profiling for.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Engine/EngineController.cs)

#### `ProtoCore.Mirror.RuntimeMirror GetMirror(string variableName)`

Returns runtime mirror for variable.

**Parameters:**
- `variableName` (string) — Unique ID of AST node

**Returns:** `ProtoCore.Mirror.RuntimeMirror` — RuntimeMirror object that reflects status of a single designscript variable

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Engine/EngineController.cs)

#### `void NodeDeleted(Dynamo.Graph.Nodes.NodeModel node)`

NodeDeleted event handler.

**Parameters:**
- `node` (Dynamo.Graph.Nodes.NodeModel) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Engine/EngineController.cs)

#### `void OnCompiled(System.Guid nodeGuid, System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode> astNodes)`

This class represents a state after a nodemodel is compiled to AST nodes.

**Parameters:**
- `nodeGuid` (System.Guid) — Node unique ID
- `astNodes` (System.Collections.Generic.IEnumerable<ProtoCore.AST.AssociativeAST.AssociativeNode>) — Resulting AST nodes

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Engine/EngineController.cs)

#### `void OnCompiling(System.Guid nodeGuid)`

This class represents the intermediate state (Compiling state), when a nodemodel is compiled to AST nodes.

**Parameters:**
- `nodeGuid` (System.Guid) — Node unique ID

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Engine/EngineController.cs)

### Properties
- `ExecutionTimeData` (Dynamo.Engine.Profiling.IProfilingExecutionTimeData, get) — Returns information about time spent compiling and executing nodes.
- `HasPendingGraphSyncData` (bool, get) — Returns true if there are graph sync data in the queue waiting to be executed.
- `IsDisposed` (bool, get) — A property defining whether the EngineController has been disposed or not. This is a conservative field, as there should only be one owner of a valid EngineController or not.
- `LibraryServices` (Dynamo.Engine.LibraryServices, get) — Returns library service instance.
- `LiveRunnerCore` (ProtoCore.Core, get) — Returns DesignScript core.
- `LiveRunnerRuntimeCore` (ProtoCore.RuntimeCore, get) — Returns DesignScript runtime core.

### Events
#### `AstBuilt` (`Dynamo.Engine.AstBuiltEventHandler`)

**Signature:** `public event Dynamo.Engine.AstBuiltEventHandler AstBuilt`

This event is fired when a node has been compiled.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Engine/EngineController.cs)

## FunctionDescriptor (class)

Describe a DesignScript function in a imported library

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/FunctionDescriptor.cs)

### Constructors
- `void FunctionDescriptor(Dynamo.Engine.FunctionDescriptorParams funcDescParams)` — Initializes a new instance of the Dynamo.Engine.FunctionDescriptor class.

### Methods
#### `bool Equals(object obj)`

Overrides equality check of two Dynamo.Engine.FunctionDescriptor objects

**Parameters:**
- `obj` (object) — Dynamo.Engine.FunctionDescriptor object to compare with the current one

**Returns:** `bool` — Returns true if two Dynamo.Engine.FunctionDescriptor objects are equals

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/FunctionDescriptor.cs)

#### `int GetHashCode()`

Overrides computing the hash code for the Dynamo.Engine.FunctionDescriptor

**Returns:** `int` — The hash code for this Dynamo.Engine.FunctionDescriptor

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/FunctionDescriptor.cs)

### Properties
- `Assembly` (string, get) — Returns full path to the assembly the defined this function. This is not always an assembly path, it might be a path to a .ds file or builtin token like 'operators'.
- `CanUpdatePeriodically` (bool, get) — This attribute sets whether the function enables periodic update of the workspace.
- `Category` (string, get) — The category of this function.
- `ClassName` (string, get) — Class name of this function. If the function is global, return String.Empty.
- `Description` (string, get) — A comment describing the function along with the signature
- `DisplayName` (string, get) — QualifiedName with leading namespaces removed.
- `FunctionName` (string, get) — Function name.
- `InputParameters` (System.Collections.Generic.IEnumerable<System.Tuple<string, string>>, get) — Inputs for Node
- `IsBuiltIn` (bool, get) — Indicates if it is a built-in function
- `IsLacingDisabled` (bool, get) — Returns if the lacing strategy is disabled
- `IsObsolete` (bool, get) — Indicates if the function is obsolete
- `IsOverloaded` (bool, get/set) — Indicates if the function overloads
- `IsPackageMember` (bool, get) — Indicates if the function is packaged element (either zero-touch DLLs or DYFs)
- `IsVarArg` (bool, get) — Does the function accept a variable number of arguments?
- `IsVisibleInLibrary` (bool, get) — This attribute sets, if this function is shown in library or not.
- `MangledName` (string, get) — A unique name to identify a function. It is necessary when a function is overloaded.
- `Namespace` (string, get) — Returns namespace where the function is specified
- `ObsoleteMessage` (string, get) — Message specified if function is obsolete
- `Parameters` (System.Collections.Generic.IEnumerable<Dynamo.Library.TypedParameter>, get) — Function parameters.
- `PathManager` (Dynamo.Interfaces.IPathManager, get) — Returns instance of IPathManager
- `QualifiedName` (string, get) — The string that is used to search for this function.
- `ReturnKeys` (System.Collections.Generic.IEnumerable<string>, get) — If the function returns a dictionary, ReturnKeys is the key collection used in returned dictionary.
- `ReturnType` (ProtoCore.Type, get) — Function return type.
- `Returns` (System.Collections.Generic.IEnumerable<System.Tuple<string, string>>, get) — If the XML documentation for the function includes a returns field, this parameter contains a collection of tuples of output names to descriptions. Otherwise, this list will be empty.
- `Signature` (string, get) — The full signature of the function.
- `Summary` (string, get) — Returns summary of the function from its documentation xml using the corresponding FunctionDescriptor object.
- `Type` (Dynamo.Engine.FunctionType, get) — Function type.
- `UnqualifedClassName` (string, get) — Returns class name without namespace
- `UserFriendlyName` (string, get) — Return a user friendly name. E.g., for operator '+' it will return 'Add'

## FunctionDescriptorParams (class)

Contains parameters for function description.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/FunctionDescriptor.cs)

### Constructors
- `void FunctionDescriptorParams()` — Initializes a new instance of the Dynamo.Engine.FunctionDescriptorParams class.

### Properties
- `Assembly` (string, get/set) — Returns full path to the assembly the defined this function. This is not always an assembly path, it might be a path to a .ds file or builtin token like 'operators'.
- `CanUpdatePeriodically` (bool, get/set) — This attribute sets whether the function enables periodic update of the workspace.
- `ClassName` (string, get/set) — Returns class name of this function. If the functinon is global, return String.Empty.
- `FunctionName` (string, get/set) — Returns function name.
- `FunctionType` (Dynamo.Engine.FunctionType, get/set) — Describes type of function
- `IsBuiltIn` (bool, get/set) — Indicates if it is built-in function
- `IsLacingDisabled` (bool, get/set) — Indicates if the lacing strategy is disabled on the function
- `IsPackageMember` (bool, get/set) — Indicates if the function is packaged element (either zero-touch DLLs or DYFs)
- `IsVarArg` (bool, get/set) — Does the function accept a variable number of arguments?
- `IsVisibleInLibrary` (bool, get/set) — This attribute sets, if this function is shown in library or not.
- `ObsoleteMsg` (string, get/set) — Message specified if function is obsolete
- `Parameters` (System.Collections.Generic.IEnumerable<Dynamo.Library.TypedParameter>, get/set) — Returns function parameters data
- `PathManager` (Dynamo.Interfaces.IPathManager, get/set) — Returns instance of IPathManager
- `ReturnKeys` (System.Collections.Generic.IEnumerable<string>, get/set) — If the function returns a dictionary, ReturnKeys is the key collection used in returned dictionary.
- `ReturnType` (ProtoCore.Type, get/set) — Describes the type of object to return by the function
- `Summary` (string, get/set) — Returns comment describing the function along with the signature

## FunctionGroup (class)

A group of overloaded functions

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/FunctionGroup.cs)

### Constructors
- `void FunctionGroup(string qualifiedName)` — Initializes a new instance of the Dynamo.Engine.FunctionGroup class.

### Methods
#### `bool Equals(object obj)`

Overrides equality check of two Dynamo.Engine.FunctionGroup objects

**Parameters:**
- `obj` (object) — Dynamo.Engine.FunctionGroup object to compare with the current one

**Returns:** `bool` — Returns true if two Dynamo.Engine.FunctionGroup objects are equals

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/FunctionGroup.cs)

#### `int GetHashCode()`

Overrides computing the hash code for the Dynamo.Engine.FunctionGroup

**Returns:** `int` — The hash code for this Dynamo.Engine.FunctionGroup

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/FunctionGroup.cs)

### Properties
- `Functions` (System.Collections.Generic.IEnumerable<Dynamo.Engine.FunctionDescriptor>, get) — Returns collection of functions with common qualified name
- `QualifiedName` (string, get) — Returns qualified name of the corresponding functions

## FunctionType (enum)

The type of a function.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/FunctionType.cs)

### Values
- `Constructor` = `1`
- `GenericFunction` = `0`
- `InstanceMethod` = `3`
- `InstanceProperty` = `5`
- `StaticMethod` = `2`
- `StaticProperty` = `4`

## IFunctionDescriptor (interface)

Describes a function, whether imported or defined in a custom node.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/FunctionDescriptor.cs)

### Properties
- `DisplayName` (string, get) — Name to be displayed for the function.
- `FunctionName` (string, get) — Function name.
- `MangledName` (string, get) — An unique name to identify a function. It is used to create a corresponding node instance
- `Parameters` (System.Collections.Generic.IEnumerable<Dynamo.Library.TypedParameter>, get) — Function parameters
- `ReturnKeys` (System.Collections.Generic.IEnumerable<string>, get) — Return keys for multi-output functions.
- `ReturnType` (ProtoCore.Type, get) — Return Type

## ITraceReconciliationProcessor (interface)

Type ITraceReconciliationProcessor

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Engine/EngineController.cs)

### Methods
#### `void PostTraceReconciliation(System.Collections.Generic.Dictionary<System.Guid, System.Collections.Generic.List<string>> orphanedSerializables)`

ITraceReconciliationProcessor.PostTraceReconciliation

**Parameters:**
- `orphanedSerializables` (System.Collections.Generic.Dictionary<System.Guid, System.Collections.Generic.List<string>>)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Engine/EngineController.cs)

## LibraryServices (class)

LibraryServices is a singleton class which manages builtin libraries as well as imported libraries. It is across different sessions.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/LibraryServices.cs)

### Constructors
- `void LibraryServices(ProtoCore.Core libraryManagementCore, Dynamo.Interfaces.IPathManager pathManager)` — Initializes a new instance of the Dynamo.Engine.LibraryServices class.
- `void LibraryServices(ProtoCore.Core libraryManagementCore, Dynamo.Interfaces.IPathManager pathManager, Dynamo.Interfaces.IPreferences preferences)` — Initializes a new instance of the Dynamo.Engine.LibraryServices class.

### Methods
#### `void Dispose()`

Performs application-defined tasks associated with freeing, releasing, or resetting unmanaged resources

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/LibraryServices.cs)

#### `System.Collections.Generic.IEnumerable<Dynamo.Engine.FunctionDescriptor> GetAllFunctionDescriptors(string qualifiedName)`

Returns a list of function descriptors associated with the function name.

**Parameters:**
- `qualifiedName` (string) — 

**Returns:** `System.Collections.Generic.IEnumerable<Dynamo.Engine.FunctionDescriptor>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/LibraryServices.cs)

#### `Dynamo.Engine.FunctionDescriptor GetFunctionDescriptor(string managledName)`

Returns function descriptor from the managed function name.

**Parameters:**
- `managledName` (string) — 

**Returns:** `Dynamo.Engine.FunctionDescriptor` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/LibraryServices.cs)

#### `Dynamo.Engine.FunctionDescriptor GetFunctionDescriptor(string library, string mangledName)`

Returns function descriptor from the managled function name. name.

**Parameters:**
- `library` (string) — Library path
- `mangledName` (string) — Mangled function name

**Returns:** `Dynamo.Engine.FunctionDescriptor` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/LibraryServices.cs)

#### `System.Collections.Generic.Dictionary<string, string> GetPriorNames()`

Returns a dictionary of old names vs. new names for node migration

**Returns:** `System.Collections.Generic.Dictionary<string, string>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/LibraryServices.cs)

### Properties
- `BuiltinFunctionGroups` (System.Collections.Generic.IEnumerable<Dynamo.Engine.FunctionGroup>, get) — Returns built-in function groups.
- `ImportedFunctionGroups` (System.Collections.Generic.IEnumerable<Dynamo.Engine.FunctionGroup>, get) — Returns all imported function groups.
- `ImportedLibraries` (System.Collections.Generic.IEnumerable<string>, get) — Returns a list of imported libraries.
- `LibraryManagementCore` (ProtoCore.Core, get) — Returns core which is used for parsing code and loading libraries

### Events
#### `LibraryLoadFailed` (`System.EventHandler<Dynamo.Engine.LibraryServices.LibraryLoadFailedEventArgs>`)

**Signature:** `public event System.EventHandler<Dynamo.Engine.LibraryServices.LibraryLoadFailedEventArgs> LibraryLoadFailed`

Occurs if a library cannot be loaded

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/LibraryServices.cs)

#### `LibraryLoaded` (`System.EventHandler<Dynamo.Engine.LibraryServices.LibraryLoadedEventArgs>`)

**Signature:** `public event System.EventHandler<Dynamo.Engine.LibraryServices.LibraryLoadedEventArgs> LibraryLoaded`

Occurs after a library is successfully loaded

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/LibraryServices.cs)

#### `LibraryLoading` (`System.EventHandler<Dynamo.Engine.LibraryServices.LibraryLoadingEventArgs>`)

**Signature:** `public event System.EventHandler<Dynamo.Engine.LibraryServices.LibraryLoadingEventArgs> LibraryLoading`

Occurs before a library is loaded

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/LibraryServices.cs)

## LiveRunnerServices (class)

This is a helper class that can get mirror data from live runner, update graph etc.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Engine/LiveRunnerServices.cs)

### Constructors
- `void LiveRunnerServices(Dynamo.Engine.EngineController controller, string geometryFactoryFileName)` — Creates LiveRunnerServices.

### Methods
#### `void Dispose()`

Dispose LiveRunner object.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Engine/LiveRunnerServices.cs)

### Properties
- `Core` (ProtoCore.Core, get) — LiveRunnerServices.Core
- `RuntimeCore` (ProtoCore.RuntimeCore, get) — RuntimeCore of liveRunner. RuntimeCore is an object that is instantiated once across the lifecycle of the runtime. This is the entry point of the runtime VM and its input is a DS Executable format.

## XmlDocumentationExtensions (static-class)

Provides extension methods for reading XML documentation from reflected members.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/XmlDocumentationExtensions.cs)

### Events
#### `LogToConsole` (`System.Action<string>`)

**Signature:** `public event System.Action<string> LogToConsole`

Raise event to log messages to the Dynamo Console.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Library/XmlDocumentationExtensions.cs)

