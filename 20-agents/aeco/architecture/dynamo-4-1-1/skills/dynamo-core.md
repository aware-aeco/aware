---
name: dynamo-dynamo-core
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.Core namespace — 6 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AuthenticationManager, CrashPromptArgs, CrashErrorReportArgs, CustomNodeManager, IDSDKManager, NotificationObject.
---

# Dynamo.Core

Auto-generated from vendor docs for dynamo 4.1.1. 6 types in this namespace.

## AuthenticationManager (class)

This is a wrapper for Greg.IAuthProvider functionality. It is used for oxygen authentication.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/AuthenticationManager.cs)

### Constructors
- `void AuthenticationManager(Greg.IAuthProvider authProvider)` — Initializes a new instance of the Dynamo.Core.AuthenticationManager class.

### Properties
- `AuthProvider` (Greg.IAuthProvider, get) — Current IAuthProvider
- `HasAuthProvider` (bool, get) — Determines if the this.client has login capabilities
- `LoginState` (Greg.AuthProviders.LoginState, get) — Specifies whether the user is logged in or not.
- `LoginStateInitial` (Greg.AuthProviders.LoginState, get) — This Property will return the initial LoginState assigned in the constructor in order to be used in the Initialization flow. Others features require direct calls to the LoginState due to are on demand.
- `Username` (string, get) — The username of the current user, if logged in. Otherwise null

### Events
#### `LoginStateChanged` (`System.Action<Greg.AuthProviders.LoginState>`)

**Signature:** `public event System.Action<Greg.AuthProviders.LoginState> LoginStateChanged`

Occurs when login state is changed

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/AuthenticationManager.cs)

## CrashErrorReportArgs (class)

Event argument for CER (crash error reporting) tool. It contains options on what to send out with the crash report.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CrashPromptArgs.cs)

### Constructors
- `void CrashErrorReportArgs(System.Exception e)` — Constructs the options class to customize what CER will collect.

### Properties
- `SendRecordedCommands` (bool, get/set) — Allow Dynamo to send the recorded commands file to the CER system.

## CrashPromptArgs (class)

Event argument for CrashPrompt. It contains display options, details, overriding text and file path.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CrashPromptArgs.cs)

### Constructors
- `void CrashPromptArgs(string details, string overridingText, string filePath)` — Returns initializes a new instance of the Dynamo.Core.CrashPromptArgs class.

### Properties
- `Details` (string, get) — Returns details of crash
- `FilePath` (string, get) — Returns preference file path
- `Options` (Dynamo.Core.CrashPromptArgs.DisplayOptions, get) — Returns Dynamo.Core.CrashPromptArgs.DisplayOptions flag which indicates whether args contain default text overriden, has details or has file path
- `OverridingText` (string, get) — Returns crash message

## CustomNodeManager (class)

Manages instantiation of custom nodes. All custom nodes known to Dynamo should be stored with this type. This object implements late initialization of custom nodes by providing a single interface to initialize custom nodes.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CustomNodeManager.cs)

### Constructors
- `void CustomNodeManager(Dynamo.Graph.Nodes.NodeLoaders.NodeFactory nodeFactory, Dynamo.Migration.MigrationManager migrationManager, Dynamo.Engine.LibraryServices libraryServices)` — This function creates CustomNodeManager

### Methods
#### `bool AddUninitializedCustomNode(string file, bool isTestMode, ref Dynamo.CustomNodeInfo info)`

Import a dyf file for eventual initialization.

**Parameters:**
- `file` (string) — Path to a custom node file on disk.
- `isTestMode` (bool) — Flag specifying whether or not this should operate in "test mode".
- `info` (ref Dynamo.CustomNodeInfo) — If the info was successfully processed, this parameter will be set to it. Otherwise, it will be set to null.

**Returns:** `bool` — True on success, false if the file could not be read properly.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CustomNodeManager.cs)

#### `System.Collections.Generic.IEnumerable<Dynamo.CustomNodeInfo> AddUninitializedCustomNodesInPath(string path, bool isTestMode, Dynamo.Graph.Workspaces.PackageInfo packageInfo)`

Scans the given path for custom node files, retaining their information in the manager for later potential initialization. Should be used when packages load or reload customNodes.

**Parameters:**
- `path` (string) — Path on disk to scan for custom nodes.
- `isTestMode` (bool) — Flag specifying whether or not this should operate in "test mode".
- `packageInfo` (Dynamo.Graph.Workspaces.PackageInfo) — Info about the package that requested this customNode to be loaded or to which the customNode belongs. Is PackageMember property will be true if this property is not null.

**Returns:** `System.Collections.Generic.IEnumerable<Dynamo.CustomNodeInfo>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CustomNodeManager.cs)

#### `System.Collections.Generic.IEnumerable<Dynamo.CustomNodeInfo> AddUninitializedCustomNodesInPath(string path, bool isTestMode, bool isPackageMember)`

Scans the given path for custom node files, retaining their information in the manager for later potential initialization.

**Parameters:**
- `path` (string) — Path on disk to scan for custom nodes.
- `isTestMode` (bool) — Flag specifying whether this should operate in "test mode".
- `isPackageMember` (bool) — Indicates whether custom node comes from package or not.

**Returns:** `System.Collections.Generic.IEnumerable<Dynamo.CustomNodeInfo>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CustomNodeManager.cs)

#### `bool Contains(System.Guid guid)`

Returns true if the custom node's unique identifier is inside of the manager (initialized or not)

**Parameters:**
- `guid` (System.Guid) — The FunctionId

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CustomNodeManager.cs)

#### `bool Contains(string name)`

Returns true if the custom node's name is inside the manager (initialized or not)

**Parameters:**
- `name` (string) — The name of the custom node.

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CustomNodeManager.cs)

#### `Dynamo.Graph.Nodes.CustomNodes.Function CreateCustomNodeInstance(System.Guid id, string name, bool isTestMode)`

Creates a new Custom Node Instance.

**Parameters:**
- `id` (System.Guid) — Identifier referring to a custom node definition.
- `name` (string) — Name for the custom node to be instantiated, used for error recovery if the given id could not be found.
- `isTestMode` (bool) — Flag specifying whether or not this should operate in "test mode".

**Returns:** `Dynamo.Graph.Nodes.CustomNodes.Function` — Custom Node Instance

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CustomNodeManager.cs)

#### `Dynamo.Graph.Nodes.CustomNodes.Function CreateCustomNodeInstance(System.Guid id, string name, bool isTestMode, Dynamo.CustomNodeDefinition def, Dynamo.CustomNodeInfo info)`

Creates a new Custom Node Instance.

**Parameters:**
- `id` (System.Guid) — Identifier referring to a custom node definition.
- `name` (string) — Name for the custom node to be instantiated, used for error recovery if the given id could not be found.
- `isTestMode` (bool) — Flag specifying whether or not this should operate in "test mode".
- `def` (Dynamo.CustomNodeDefinition) — Custom node definition data
- `info` (Dynamo.CustomNodeInfo) — Custom node information data

**Returns:** `Dynamo.Graph.Nodes.CustomNodes.Function` — Custom Node Instance

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CustomNodeManager.cs)

#### `void Dispose()`

Call this method to uninitialize all loaded custom node functions.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CustomNodeManager.cs)

#### `Dynamo.Graph.Workspaces.CustomNodeWorkspaceModel GetWorkspaceById(System.Guid customNodeId)`

Returns custom node workspace by a specified custom node ID

**Parameters:**
- `customNodeId` (System.Guid) — Custom node ID of a requested workspace

**Returns:** `Dynamo.Graph.Workspaces.CustomNodeWorkspaceModel` — Custom node workspace by a specified ID

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CustomNodeManager.cs)

#### `System.Guid GuidFromPath(string path)`

Returns a function id from a guid assuming that the file is already loaded.

**Parameters:**
- `path` (string) — 

**Returns:** `System.Guid` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CustomNodeManager.cs)

#### `void OnCustomNodeRemoved(System.Guid functionId)`

CustomNodeManager.OnCustomNodeRemoved

**Parameters:**
- `functionId` (System.Guid)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CustomNodeManager.cs)

#### `void OnDefinitionUpdated(Dynamo.CustomNodeDefinition obj)`

CustomNodeManager.OnDefinitionUpdated

**Parameters:**
- `obj` (Dynamo.CustomNodeDefinition)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CustomNodeManager.cs)

#### `void OnInfoUpdated(Dynamo.CustomNodeInfo obj)`

CustomNodeManager.OnInfoUpdated

**Parameters:**
- `obj` (Dynamo.CustomNodeInfo)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CustomNodeManager.cs)

#### `bool OpenCustomNodeWorkspace(System.Xml.XmlDocument xmlDoc, Dynamo.Graph.Workspaces.WorkspaceInfo workspaceInfo, bool isTestMode, ref Dynamo.Graph.Workspaces.WorkspaceModel workspace)`

Opens a Custom Node workspace from an XmlDocument, given a pre-constructed WorkspaceInfo.

**Parameters:**
- `xmlDoc` (System.Xml.XmlDocument) — Xml content of workspace
- `workspaceInfo` (Dynamo.Graph.Workspaces.WorkspaceInfo) — Workspace header describing the custom node file.
- `isTestMode` (bool) — Flag specifying whether or not this should operate in "test mode".
- `workspace` (ref Dynamo.Graph.Workspaces.WorkspaceModel) — 

**Returns:** `bool` — Boolean indicating if Custom Node Workspace opened.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CustomNodeManager.cs)

#### `void Remove(System.Guid guid)`

Attempts to remove all traces of a particular custom node from Dynamo, assuming the node is not in a loaded workspace.

**Parameters:**
- `guid` (System.Guid) — Custom node identifier.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CustomNodeManager.cs)

#### `bool TryGetCustomNodeData(System.Guid id, string name, bool isTestMode, ref Dynamo.CustomNodeDefinition def, ref Dynamo.CustomNodeInfo info)`

Attempts to get custom node info and definition data.

**Parameters:**
- `id` (System.Guid) — Identifier referring to a custom node definition.
- `name` (string) — Name for the custom node to be instantiated, used for error recovery if the given id could not be found.
- `isTestMode` (bool) — Flag specifying whether this should operate in "test mode".
- `def` (ref Dynamo.CustomNodeDefinition) — Custom node definition data
- `info` (ref Dynamo.CustomNodeInfo) — Custom node information data

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CustomNodeManager.cs)

#### `bool TryGetFunctionDefinition(System.Guid id, bool isTestMode, ref Dynamo.CustomNodeDefinition definition)`

Returns the function definition from a guid.

**Parameters:**
- `id` (System.Guid) — Custom node identifier.
- `isTestMode` (bool) — Flag specifying whether or not this should operate in "test mode".
- `definition` (ref Dynamo.CustomNodeDefinition) — 

**Returns:** `bool` — Boolean indicating if Custom Node Workspace defination is loaded.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CustomNodeManager.cs)

#### `bool TryGetFunctionWorkspace(System.Guid id, bool isTestMode, ref Dynamo.Graph.Workspaces.CustomNodeWorkspaceModel ws)`

Returns the function workspace from a guid

**Parameters:**
- `id` (System.Guid) — The unique id for the node.
- `isTestMode` (bool) — Flag specifying whether or not this should operate in "test mode".
- `ws` (ref Dynamo.Graph.Workspaces.CustomNodeWorkspaceModel) — 

**Returns:** `bool` — The path to the node or null if it wasn't found.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CustomNodeManager.cs)

#### `bool TryGetFunctionWorkspace(System.Guid id, bool isTestMode, ref Dynamo.Graph.Workspaces.ICustomNodeWorkspaceModel ws)`

Returns the function workspace.

**Parameters:**
- `id` (System.Guid) — The identifier.
- `isTestMode` (bool) — if set to true [is test mode].
- `ws` (ref Dynamo.Graph.Workspaces.ICustomNodeWorkspaceModel) — The workspace.

**Returns:** `bool` — Boolean indicating if Custom Node Workspace defination is loaded.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CustomNodeManager.cs)

### Properties
- `LoadedDefinitions` (System.Collections.Generic.IEnumerable<Dynamo.CustomNodeDefinition>, get) — CustomNodeDefinitions for all loaded custom nodes, in load order.
- `LoadedWorkspaces` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Workspaces.CustomNodeWorkspaceModel>, get) — All loaded custom node workspaces.

### Events
#### `CustomNodeRemoved` (`System.Action<System.Guid>`)

**Signature:** `public event System.Action<System.Guid> CustomNodeRemoved`

An event that is fired when a custom node is removed from Dynamo.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CustomNodeManager.cs)

#### `DefinitionUpdated` (`System.Action<Dynamo.CustomNodeDefinition>`)

**Signature:** `public event System.Action<Dynamo.CustomNodeDefinition> DefinitionUpdated`

An event that is fired when a definition is updated

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CustomNodeManager.cs)

#### `InfoUpdated` (`System.Action<Dynamo.CustomNodeInfo>`)

**Signature:** `public event System.Action<Dynamo.CustomNodeInfo> InfoUpdated`

An event that is fired when new or updated info is available for a custom node.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/CustomNodeManager.cs)

## IDSDKManager (class)

The class to provide auth APIs for IDSDK related methods.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/IDSDKManager.cs)

### Constructors
- `void IDSDKManager()` — IDSDKManager.IDSDKManager

### Methods
#### `void Dispose()`

IDSDKManager.Dispose

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/IDSDKManager.cs)

#### `string GetAccessToken()`

IDSDKManager.GetAccessToken

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/IDSDKManager.cs)

#### `bool IsLoggedIn()`

Returns the login status of the current session.

**Returns:** `bool` — Boolean Status Value

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/IDSDKManager.cs)

#### `bool Login()`

Triggers login using Auth API, if the user is not already logged in.

**Returns:** `bool` — True, if login was successfull, else False

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/IDSDKManager.cs)

#### `void Logout()`

Logs out the user from the current session.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/IDSDKManager.cs)

#### `void SignRequest(ref RestSharp.RestRequest m, RestSharp.RestClient client)`

Used by the auth provider to sign request with the authorized token.

**Parameters:**
- `m` (ref RestSharp.RestRequest)
- `client` (RestSharp.RestClient)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/IDSDKManager.cs)

### Properties
- `IsIDSDKInitialized` (bool, get) — Returns the status of IDSDK installation, will return false if IDSDK fails to initialize.
- `LoginState` (Greg.AuthProviders.LoginState, get) — Returns the login status of the current session.
- `UserId` (string, get) — Gets the userid of the logged in user.
- `Username` (string, get) — Gets the username of the logged in user.

### Events
#### `LoginStateChanged` (`System.Action<Greg.AuthProviders.LoginState>`)

**Signature:** `public event System.Action<Greg.AuthProviders.LoginState> LoginStateChanged`

Tracks any change in the login status.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/IDSDKManager.cs)

#### `RequestLogin` (`System.Func<object, bool>`)

**Signature:** `public event System.Func<object, bool> RequestLogin`

Used by the auth provider to request authentication.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/IDSDKManager.cs)

## NotificationObject (class)

This class notifies the View when there is a change.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/NotificationObject.cs)

### Constructors
- `void NotificationObject()` — NotificationObject.NotificationObject

### Methods
#### `void RaisePropertyChanged(string propertyName)`

Raises this object's PropertyChanged event.

**Parameters:**
- `propertyName` (string) — The property that has a new value.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/NotificationObject.cs)

#### `void RaisePropertyChanged(string[] propertyNames)`

Raises this object's PropertyChanged event for each of the properties.

**Parameters:**
- `propertyNames` (string[]) — The properties that have a new value.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/NotificationObject.cs)

### Events
#### `PropertyChanged` (`System.ComponentModel.PropertyChangedEventHandler`)

**Signature:** `public event System.ComponentModel.PropertyChangedEventHandler PropertyChanged`

Raised when a property on this object has a new value.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Core/NotificationObject.cs)

