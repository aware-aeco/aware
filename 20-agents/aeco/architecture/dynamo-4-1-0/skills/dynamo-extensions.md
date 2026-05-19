---
name: dynamo-dynamo-extensions
description: This skill encodes the dynamo 4.1.0 surface of the Dynamo.Extensions namespace — 16 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: EnumDescriptionAttribute, ExtensionData, ExtensionDefinition, ExtensionLibraryLoader, ExtensionLoader, IExtensionSource, ExtensionManager, ICommandExecutive, and 8 more types.
---

# Dynamo.Extensions

Auto-generated from vendor docs for dynamo 4.1.0. 16 types in this namespace.

## EnumDescriptionAttribute (class)

Provides description for enum member.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/EnumExtension.cs)

### Constructors
- `void EnumDescriptionAttribute(string resourceKey, System.Type resourceType)` — Creates EnumDescriptionAttribute.

### Properties
- `Description` (string, get) — Description of enum item.

## ExtensionData (class)

Type ExtensionData

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionData.cs)

### Constructors
- `void ExtensionData(string extensionGuid, string name, string version, System.Collections.Generic.Dictionary<string, string> data)` — ExtensionData.ExtensionData

### Properties
- `Data` (System.Collections.Generic.Dictionary<string, string>, get/set) — Extension specific data.
- `ExtensionGuid` (string, get) — Extensions unique id.
- `Name` (string, get) — Name of extension.
- `Version` (string, get) — Extension version.

## ExtensionDefinition (class)

Type ExtensionDefinition

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionDefinition.cs)

### Constructors
- `void ExtensionDefinition()` — ExtensionDefinition.ExtensionDefinition

### Properties
- `AssemblyPath` (string, get/set) — ExtensionDefinition.AssemblyPath
- `TypeName` (string, get/set) — ExtensionDefinition.TypeName

## ExtensionLibraryLoader (class)

Provides functionality for loading node's DLLs

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionLibraryLoader.cs)

### Methods
#### `void LoadNodeLibrary(System.Reflection.Assembly library)`

Loads a ZeroTouch or NodeModel based node into the VM and search. To guarantee the node is correctly added to the LibraryUI this method should not be called while LibraryExtension is loading.

**Parameters:**
- `library` (System.Reflection.Assembly) — The library.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionLibraryLoader.cs)

#### `void LoadPackages(System.Collections.Generic.IEnumerable<System.Reflection.Assembly> assemblies)`

Loads packages for import into VM and for node search.

**Parameters:**
- `assemblies` (System.Collections.Generic.IEnumerable<System.Reflection.Assembly>) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionLibraryLoader.cs)

## ExtensionLoader (class)

Provides functionality for loading Dynamo's extensions. This class loads formatted XMLs which contain information about *Extension.dll and type name of IExtension inheritor. Example: ..\ExtensionName.dllDynamo.ExtensionName.ExtensionTypeName

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionLoader.cs)

### Constructors
- `void ExtensionLoader()` — ExtensionLoader.ExtensionLoader

### Methods
#### `Dynamo.Extensions.IExtension Load(string extensionPath)`

Loads Dynamo.Extensions.IExtension from assembly.

**Parameters:**
- `extensionPath` (string) — Assembly full path

**Returns:** `Dynamo.Extensions.IExtension` — Loaded Dynamo.Extensions.IExtension

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionLoader.cs)

#### `System.Collections.Generic.IEnumerable<Dynamo.Extensions.IExtension> LoadDirectory(string extensionsPath)`

Loads a collection of Dynamo.Extensions.IExtension from given folder

**Parameters:**
- `extensionsPath` (string) — Assemblies location folder

**Returns:** `System.Collections.Generic.IEnumerable<Dynamo.Extensions.IExtension>` — Loaded collection of Dynamo.Extensions.IExtension

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionLoader.cs)

### Events
#### `ExtensionLoading` (`System.Action<Dynamo.Extensions.IExtension>`)

**Signature:** `public event System.Action<Dynamo.Extensions.IExtension> ExtensionLoading`

An event that is raised when an extension starts loading.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionLoader.cs)

#### `MessageLogged` (`System.Action<Dynamo.Logging.ILogMessage>`)

**Signature:** `public event System.Action<Dynamo.Logging.ILogMessage> MessageLogged`

This event is used for logging messages.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionLoader.cs)

## ExtensionManager (class)

This class handles registration, lookup, and disposal of extensions.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionManager.cs)

### Constructors
- `void ExtensionManager()` — Creates ExtensionManager.
- `void ExtensionManager(System.Collections.Generic.IEnumerable<string> directoriesToVerify)` — Creates ExtensionManager with directories which require package certificate verification.

### Methods
#### `void Add(Dynamo.Extensions.IExtension extension)`

Adds an extension to the current session.

**Parameters:**
- `extension` (Dynamo.Extensions.IExtension) — Extension

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionManager.cs)

#### `void Dispose()`

Disposes all the loaded extensions.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionManager.cs)

#### `object GetService(System.Type serviceType)`

Gets the service object of the specified type.

**Parameters:**
- `serviceType` (System.Type) — Type of the service

**Returns:** `object` — The service object if registered else null

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionManager.cs)

#### `string RegisterService<T>(T service)`

Allows extension applications to register some specific service by its type. Only one service of a given type can be registered.

**Parameters:**
- `service` (T) — The service object to register

**Returns:** `string` — A key for the registered service if registeration is successful else null.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionManager.cs)

#### `void Remove(Dynamo.Extensions.IExtension extension)`

Removes an extension from the current session.

**Parameters:**
- `extension` (Dynamo.Extensions.IExtension) — Extension

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionManager.cs)

#### `T Service<T>()`

Gets the service object of the specified type.

**Returns:** `T` — The service object if registered else null

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionManager.cs)

#### `bool UnregisterService<T>(string serviceKey)`

Unregisters a service of given type registered with given key.

**Parameters:**
- `serviceKey` (string) — The service key to ensure that only authorized client is unregistering this service type.

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionManager.cs)

### Properties
- `ExtensionLoader` (Dynamo.Extensions.IExtensionLoader, get) — This loader loads extensions in Dynamo.
- `Extensions` (System.Collections.Generic.IEnumerable<Dynamo.Extensions.IExtension>, get) — Returns the collection of registered extensions
- `StorageAccessExtensions` (System.Collections.Generic.IEnumerable<Dynamo.Extensions.IExtensionStorageAccess>, get) — Returns the collection of registered extensions implementing IExtensionStorageAccess

### Events
#### `ExtensionAdded` (`System.Action<Dynamo.Extensions.IExtension>`)

**Signature:** `public event System.Action<Dynamo.Extensions.IExtension> ExtensionAdded`

This event is fired when a new extension is added.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionManager.cs)

#### `ExtensionRemoved` (`System.Action<Dynamo.Extensions.IExtension>`)

**Signature:** `public event System.Action<Dynamo.Extensions.IExtension> ExtensionRemoved`

This event is fired when a new extension is removed,

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionManager.cs)

#### `MessageLogged` (`System.Action<Dynamo.Logging.ILogMessage>`)

**Signature:** `public event System.Action<Dynamo.Logging.ILogMessage> MessageLogged`

This event is fired when a message is logged.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionManager.cs)

## ICommandExecutive (interface)

Interface to Dynamo's recordable command framework

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ICommandExecutive.cs)

### Methods
#### `void ExecuteCommand(Dynamo.Models.DynamoModel.RecordableCommand command, string uniqueId, string extensionName)`

Endpoint method to execute any commands deriving from RecordableCommand

**Parameters:**
- `command` (Dynamo.Models.DynamoModel.RecordableCommand) — 
- `uniqueId` (string) — 
- `extensionName` (string) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ICommandExecutive.cs)

## IExtension (interface)

An extension to the model layer of Dynamo

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/IExtension.cs)

### Methods
#### `void Ready(Dynamo.Extensions.ReadyParams sp)`

Action to be invoked when the Dynamo has started up and is ready for user interaction. This action is guaranteed to be called even if the extension is installed after startup. Exceptions thrown from this method will be caught by Dynamo and logged.

**Parameters:**
- `sp` (Dynamo.Extensions.ReadyParams)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/IExtension.cs)

#### `void Shutdown()`

Action to be invoked when shutdown has begun.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/IExtension.cs)

#### `void Startup(Dynamo.Extensions.StartupParams sp)`

Action to be invoked when Dynamo begins to start up. This action is *not* guaranteed to be invoked unless the extension is already installed at startup. Exceptions thrown from this method will be caught by Dynamo and logged.

**Parameters:**
- `sp` (Dynamo.Extensions.StartupParams)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/IExtension.cs)

### Properties
- `Name` (string, get) — A name for the Extension. This is used for more user-readable logging.
- `UniqueId` (string, get) — A unique id for this extension instance. There may be multiple instances of the same type, but the application will *not* allow two instances to coexist with the same id.

## IExtensionLoader (interface)

Handles loading extensions given an extension definition files path

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/IExtensionLoader.cs)

### Methods
#### `Dynamo.Extensions.IExtension Load(string extensionPath)`

Extension method for loading assembly from the path. Returns Dynamo.Extensions.IExtension.

**Parameters:**
- `extensionPath` (string)

**Returns:** `Dynamo.Extensions.IExtension` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/IExtensionLoader.cs)

#### `System.Collections.Generic.IEnumerable<Dynamo.Extensions.IExtension> LoadDirectory(string extensionsPath)`

Extension method for loading assembly from a directory. Returns Dynamo.Extensions.IExtension.

**Parameters:**
- `extensionsPath` (string)

**Returns:** `System.Collections.Generic.IEnumerable<Dynamo.Extensions.IExtension>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/IExtensionLoader.cs)

### Events
#### `ExtensionLoading` (`System.Action<Dynamo.Extensions.IExtension>`)

**Signature:** `public event System.Action<Dynamo.Extensions.IExtension> ExtensionLoading`

An event that is raised when an extension starts loading.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/IExtensionLoader.cs)

## IExtensionManager (interface)

This class handles registration, lookup, and disposal of extensions. There should only be one of these per application instance.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/IExtensionManager.cs)

### Methods
#### `void Add(Dynamo.Extensions.IExtension extension)`

Add an extension to the current application session.

**Parameters:**
- `extension` (Dynamo.Extensions.IExtension)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/IExtensionManager.cs)

#### `void Remove(Dynamo.Extensions.IExtension extension)`

Remove an extension from the current application session.

**Parameters:**
- `extension` (Dynamo.Extensions.IExtension)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/IExtensionManager.cs)

### Properties
- `ExtensionLoader` (Dynamo.Extensions.IExtensionLoader, get) — Returns Extension loader.
- `Extensions` (System.Collections.Generic.IEnumerable<Dynamo.Extensions.IExtension>, get) — The collection of currently registered extensions

### Events
#### `ExtensionAdded` (`System.Action<Dynamo.Extensions.IExtension>`)

**Signature:** `public event System.Action<Dynamo.Extensions.IExtension> ExtensionAdded`

Event raised when an extension is added

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/IExtensionManager.cs)

#### `ExtensionRemoved` (`System.Action<Dynamo.Extensions.IExtension>`)

**Signature:** `public event System.Action<Dynamo.Extensions.IExtension> ExtensionRemoved`

Event raised when an extension is removed

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/IExtensionManager.cs)

## IExtensionSource (interface)

An object which may request extensions to be loaded and added to the extensionsManager.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionManager.cs)

### Properties
- `RequestedExtensions` (System.Collections.Generic.IEnumerable<Dynamo.Extensions.IExtension>, get) — Collection of Extensions this ExtensionSource has requested be loaded.

### Events
#### `RequestAddExtension` (`System.Action<Dynamo.Extensions.IExtension>`)

**Signature:** `public event System.Action<Dynamo.Extensions.IExtension> RequestAddExtension`

Event that is raised when ExtensionSource requests an Extension to be added to list of currently loaded extensions.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionManager.cs)

#### `RequestLoadExtension` (`System.Func<string, Dynamo.Extensions.IExtension>`)

**Signature:** `public event System.Func<string, Dynamo.Extensions.IExtension> RequestLoadExtension`

Event that is raised when the ExtensionSource requests an Extension be loaded.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ExtensionManager.cs)

## IExtensionStorageAccess (interface)

Type IExtensionStorageAccess

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/IExtensionStorageAccess.cs)

### Methods
#### `void OnWorkspaceOpen(System.Collections.Generic.Dictionary<string, string> extensionData)`

Action to be invoked when the workspace is opened. The passed extensionData dictionary is a copy of the Dynamo.Extensions.ExtensionData data dictionary. Modifying the extensionData from this action will not modify the stored Dynamo.Extensions.ExtensionData. To modify the stored Dynamo.Extensions.ExtensionData data dictionary use the Dynamo.Extensions.IExtensionStorageAccess.OnWorkspaceSaving(System.Collections.Generic.Dictionary{System.String,System.String},Dynamo.Graph.SaveContext).

**Parameters:**
- `extensionData` (System.Collections.Generic.Dictionary<string, string>) — A copy of the ExtensionData dictionary

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/IExtensionStorageAccess.cs)

#### `void OnWorkspaceSaving(System.Collections.Generic.Dictionary<string, string> extensionData, Dynamo.Graph.SaveContext saveContext)`

Action to be invoked when the workspace has begun its saving process. The passed extensionData dictionary is a direct reference to the stored Dynamo.Extensions.ExtensionData data dictionary, modifications to this dictionary will be reflected in the stored Dynamo.Extensions.ExtensionData data dictionary

**Parameters:**
- `extensionData` (System.Collections.Generic.Dictionary<string, string>) — 
- `saveContext` (Dynamo.Graph.SaveContext) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/IExtensionStorageAccess.cs)

### Properties
- `Name` (string, get) — A name for the Extension. The name will be equivalent to the extension that implements this interface name.
- `UniqueId` (string, get) — A unique id for this extension instance. The id will be equivalent to the extension that implements this interface id.

## IServiceManager (interface)

Defines a mechanism for registering and retrieving a service object; that is, an object that provides custom support to other objects.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/IServiceManager.cs)

### Methods
#### `string RegisterService<T>(T service)`

Allows extension applications to register some specific service by its type. Only one service of a given type can be registered.

**Parameters:**
- `service` (T) — The service object to register

**Returns:** `string` — A key for the registered service if registeration is successful else null.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/IServiceManager.cs)

#### `T Service<T>()`

Gets the service object of the specified type.

**Returns:** `T` — The service object if registered else null

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/IServiceManager.cs)

#### `bool UnregisterService<T>(string serviceKey)`

Unregisters a service of given type registered with given key.

**Parameters:**
- `serviceKey` (string) — The service key to ensure that only authorized client is unregistering this service type.

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/IServiceManager.cs)

## LinterExtensionBase (class)

Base class for all LinterExtensions

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/LinterExtensionBase.cs)

### Constructors
- `void LinterExtensionBase()` — LinterExtensionBase.LinterExtensionBase

### Methods
#### `void AddLinterRule(Dynamo.Linting.Rules.LinterRule linterRule)`

Add a LinterRule

**Parameters:**
- `linterRule` (Dynamo.Linting.Rules.LinterRule) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/LinterExtensionBase.cs)

#### `void Dispose()`

LinterExtensionBase.Dispose

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/LinterExtensionBase.cs)

#### `void Ready(Dynamo.Extensions.ReadyParams sp)`

LinterExtensionBase.Ready

**Parameters:**
- `sp` (Dynamo.Extensions.ReadyParams)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/LinterExtensionBase.cs)

#### `void RemoveLinterRule(Dynamo.Linting.Rules.LinterRule linterRule)`

Remove a LinterRule

**Parameters:**
- `linterRule` (Dynamo.Linting.Rules.LinterRule) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/LinterExtensionBase.cs)

#### `void Shutdown()`

LinterExtensionBase.Shutdown

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/LinterExtensionBase.cs)

#### `void Startup(Dynamo.Extensions.StartupParams sp)`

LinterExtensionBase.Startup

**Parameters:**
- `sp` (Dynamo.Extensions.StartupParams)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/LinterExtensionBase.cs)

### Properties
- `IsActive` (bool, get) — Is this linter currently active for the active workspace.
- `LinterRules` (System.Collections.Generic.HashSet<Dynamo.Linting.Rules.LinterRule>, get) — Collection of the rules in this extension
- `Name` (string, get) — LinterExtensionBase.Name
- `ReadyParamsRef` (Dynamo.Extensions.ReadyParams, get) — LinterExtensionBase.ReadyParamsRef
- `UniqueId` (string, get) — LinterExtensionBase.UniqueId

## ReadyParams (class)

Application-level handles provided to an extension when Dynamo has started and is ready for interaction

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ReadyParams.cs)

### Methods
#### `void Dispose()`

This method clears event handlers from the DynamoModel that the extension framework setup when the model was first loaded.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ReadyParams.cs)

### Properties
- `CommandExecutive` (Dynamo.Extensions.ICommandExecutive, get) — Extension specific implementation to execute Recordable commands on DynamoModel
- `CurrentWorkspaceModel` (Dynamo.Graph.Workspaces.IWorkspaceModel, get) — Returns current workspace
- `StartupParams` (Dynamo.Extensions.StartupParams, get) — A reference to the Dynamo.Extensions.ReadyParams.StartupParams class. Useful if this extension will be loaded from a package as its startup method, will not be called.
- `WorkspaceModels` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Workspaces.IWorkspaceModel>, get) — Returns list of workspaces

### Events
#### `CurrentWorkspaceChanged` (`System.Action<Dynamo.Graph.Workspaces.IWorkspaceModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Workspaces.IWorkspaceModel> CurrentWorkspaceChanged`

Occurs when current workspace is changed

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ReadyParams.cs)

#### `CurrentWorkspaceCleared` (`System.Action<Dynamo.Graph.Workspaces.IWorkspaceModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Workspaces.IWorkspaceModel> CurrentWorkspaceCleared`

Occurs when current workspace is cleared

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ReadyParams.cs)

#### `CurrentWorkspaceClearingStarted` (`System.Action<Dynamo.Graph.Workspaces.IWorkspaceModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Workspaces.IWorkspaceModel> CurrentWorkspaceClearingStarted`

Occurs when current workspace is clearing

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ReadyParams.cs)

#### `CurrentWorkspaceOpened` (`System.Action<Dynamo.Graph.Workspaces.IWorkspaceModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Workspaces.IWorkspaceModel> CurrentWorkspaceOpened`

Occurs when current workspace has finished opening

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ReadyParams.cs)

#### `CurrentWorkspaceRemoveStarted` (`System.Action<Dynamo.Graph.Workspaces.IWorkspaceModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Workspaces.IWorkspaceModel> CurrentWorkspaceRemoveStarted`

Occurs when a worspace is about to be removed

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ReadyParams.cs)

#### `NotificationRecieved` (`System.Action<Dynamo.Logging.NotificationMessage>`)

**Signature:** `public event System.Action<Dynamo.Logging.NotificationMessage> NotificationRecieved`

Event that is raised when the Dynamo Logger logs a notification. This event passes the notificationMessage to any subscribers

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/ReadyParams.cs)

## StartupParams (class)

Application-level handles provided to an extension when Dynamo is starting up and is not yet ready for interaction.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Extensions/StartupParams.cs)

### Properties
- `AuthProvider` (Greg.IAuthProvider, get) — Returns Greg.IAuthProvider for DynamoModel
- `CustomNodeManager` (Dynamo.Graph.Nodes.CustomNodes.ICustomNodeManager, get) — Returns Dynamo.Graph.Nodes.CustomNodes.ICustomNodeManager for DynamoModel
- `DynamoVersion` (System.Version, get) — Defines version of Dynamo
- `LibraryLoader` (Dynamo.Library.ILibraryLoader, get) — Returns Dynamo.Library.ILibraryLoader for DynamoModel
- `LinterManager` (Dynamo.Linting.LinterManager, get) — Returns Sessions Linter Manager
- `NoNetworkMode` (bool, get) — True when Dynamo starts up in offline mode.
- `PathManager` (Dynamo.Interfaces.IPathManager, get) — Returns Dynamo.Interfaces.IPathManager for DynamoModel
- `Preferences` (Dynamo.Interfaces.IPreferences, get) — Returns Dynamo.Interfaces.IPreferences for DynamoModel

