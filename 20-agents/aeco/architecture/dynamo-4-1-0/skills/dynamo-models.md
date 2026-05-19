---
name: dynamo-dynamo-models
description: This skill encodes the dynamo 4.1.0 surface of the Dynamo.Models namespace — 15 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: NodeModelAssemblyLoader, DynamoPreferencesData, HostAnalyticsInfo, IEngineControllerManager, DynamoModel, DynamoModelHandler, NodeHandler, WorkspaceHandler, and 7 more types.
---

# Dynamo.Models

Auto-generated from vendor docs for dynamo 4.1.0. 15 types in this namespace.

## ActionHandler (delegate)

This delegate is used to manage Dynamo's request dispatcher invoke.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

### Constructors
- `void ActionHandler(object object, IntPtr method)` — ActionHandler.ActionHandler

### Methods
#### `System.IAsyncResult BeginInvoke(System.Action action, System.AsyncCallback callback, object object)`

ActionHandler.BeginInvoke

**Parameters:**
- `action` (System.Action)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

#### `void EndInvoke(System.IAsyncResult result)`

ActionHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

#### `void Invoke(System.Action action)`

ActionHandler.Invoke

**Parameters:**
- `action` (System.Action)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

## DynamoModel (class)

The core model of Dynamo.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

### Constructors
- `void DynamoModel(Dynamo.Models.DynamoModel.IStartConfiguration config)` — Default constructor for DynamoModel

### Methods
#### `void AddCustomNodeWorkspace(Dynamo.Graph.Workspaces.CustomNodeWorkspaceModel workspace)`

Add a new, visible Custom Node workspace to Dynamo

**Parameters:**
- `workspace` (Dynamo.Graph.Workspaces.CustomNodeWorkspaceModel) — Dynamo.Graph.Workspaces.CustomNodeWorkspaceModel to add

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void AddHomeWorkspace()`

Add a new HomeWorkspace and set as current

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `bool AddPackagePath(string path, string file)`

Adds a new path to the list of custom package folders, but only if the path does not already exist in the list.

**Parameters:**
- `path` (string) — The path to add.
- `file` (string) — The file to add when importing a library.

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void AddToSelection(object parameters)`

Add an ISelectable object to the selection.

**Parameters:**
- `parameters` (object) — The object to add to the selection.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void AddWorkspace(Dynamo.Graph.Workspaces.WorkspaceModel workspace)`

Adds a workspace to the dynamo model.

**Parameters:**
- `workspace` (Dynamo.Graph.Workspaces.WorkspaceModel) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void ClearCurrentWorkspace()`

Clear the workspace. Removes all nodes, notes, and connectors from the current workspace.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void Copy()`

Copy selected ISelectable objects to the clipboard.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void Dispose()`

Performs application-defined tasks associated with freeing, releasing, or resetting unmanaged resources.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void ExecuteCommand(Dynamo.Models.DynamoModel.RecordableCommand command)`

Executes specified command

**Parameters:**
- `command` (Dynamo.Models.DynamoModel.RecordableCommand) — Command to execute

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void ForceRun()`

Forces an evaluation of the current workspace by resetting the DesignScript VM.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void InsertFileFromPath(string filePath, bool forceManualExecutionMode)`

Inserts a Dynamo graph or Custom Node inside the current workspace from a file path

**Parameters:**
- `filePath` (string) — 
- `forceManualExecutionMode` (bool) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void InsertFileImpl(Dynamo.Models.DynamoModel.InsertFileCommand command)`

DynamoModel.InsertFileImpl

**Parameters:**
- `command` (Dynamo.Models.DynamoModel.InsertFileCommand)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OnCleanup()`

Triggers CleaningUp event

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OnDeletionComplete(object sender, System.EventArgs e)`

Triggers DeletionComplete event

**Parameters:**
- `sender` (object) — The object which caused the event
- `e` (System.EventArgs) — The event data

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OnDeletionStarted(System.Collections.Generic.List<Dynamo.Graph.ModelBase> modelsToDelete, System.ComponentModel.CancelEventArgs cancelEventArgs)`

Called when Deletion started.

**Parameters:**
- `modelsToDelete` (System.Collections.Generic.List<Dynamo.Graph.ModelBase>)
- `cancelEventArgs` (System.ComponentModel.CancelEventArgs)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OnEvaluationCompleted(object sender, Dynamo.Models.EvaluationCompletedEventArgs e)`

Triggers EvaluationCompleted event

**Parameters:**
- `sender` (object) — The object which caused the event
- `e` (Dynamo.Models.EvaluationCompletedEventArgs) — The event data

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OnPropertyChanged(string propertyName)`

DynamoModel.OnPropertyChanged

**Parameters:**
- `propertyName` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OnRefreshCompleted(object sender, System.EventArgs e)`

Triggers RefreshCompleted event

**Parameters:**
- `sender` (object) — The object which caused the event
- `e` (System.EventArgs) — The event data

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OnRequestDispatcherBeginInvoke(System.Action action)`

Tries to invoke a given action asynchronously on Dispather

**Parameters:**
- `action` (System.Action) — Action to invoke

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OnRequestDispatcherInvoke(System.Action action)`

Tries to invoke a given action on Dispather

**Parameters:**
- `action` (System.Action) — Action to invoke

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OnRequestLayoutUpdate(object sender, System.EventArgs e)`

Called when Requests to update UI is made.

**Parameters:**
- `sender` (object) — Object which caused the event
- `e` (System.EventArgs) — The event data

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OnRequestsCrashPrompt(Dynamo.Core.CrashErrorReportArgs args)`

Shows the crash error reporting window. This method will always try to show the Autodesk CER UI first (if the CER tool is found on disk). If the CER tool is not found, the Dynamo in-house crash prompt will be shown.

**Parameters:**
- `args` (Dynamo.Core.CrashErrorReportArgs) — CER options

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OnRequestsRedraw(object sender, System.EventArgs e)`

DynamoModel.OnRequestsRedraw

**Parameters:**
- `sender` (object)
- `e` (System.EventArgs)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OnRunCompleted(object sender, bool success)`

Triggers RunCompleted event

**Parameters:**
- `sender` (object) — The object which caused the event
- `success` (bool) — Indicates if run completed successfully

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OnWorkspaceAdded(Dynamo.Graph.Workspaces.WorkspaceModel obj)`

DynamoModel.OnWorkspaceAdded

**Parameters:**
- `obj` (Dynamo.Graph.Workspaces.WorkspaceModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OnWorkspaceCleared(Dynamo.Graph.Workspaces.WorkspaceModel workspace)`

Triggers WorkspaceCleared event

**Parameters:**
- `workspace` (Dynamo.Graph.Workspaces.WorkspaceModel) — Cleared workspace

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OnWorkspaceClearing()`

Triggers WorkspaceClearing event

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OnWorkspaceClearingStarted(Dynamo.Graph.Workspaces.WorkspaceModel workspace)`

Triggers WorkspaceClearing event

**Parameters:**
- `workspace` (Dynamo.Graph.Workspaces.WorkspaceModel) — Workspace about to be cleared

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OnWorkspaceRemoveStarted(Dynamo.Graph.Workspaces.WorkspaceModel obj)`

DynamoModel.OnWorkspaceRemoveStarted

**Parameters:**
- `obj` (Dynamo.Graph.Workspaces.WorkspaceModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OnWorkspaceRemoved(Dynamo.Graph.Workspaces.WorkspaceModel obj)`

DynamoModel.OnWorkspaceRemoved

**Parameters:**
- `obj` (Dynamo.Graph.Workspaces.WorkspaceModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `bool OpenCustomNodeWorkspace(System.Guid guid)`

Opens an existing custom node workspace.

**Parameters:**
- `guid` (System.Guid) — Identifier of the workspace to open

**Returns:** `bool` — True if workspace was found and open

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OpenFileFromJson(string fileContents, bool forceManualExecutionMode)`

Opens a Dynamo workspace from a Json string.

**Parameters:**
- `fileContents` (string) — Json file content
- `forceManualExecutionMode` (bool) — Set this to true to discard execution mode specified in the file and set manual mode

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OpenFileFromJsonImpl(Dynamo.Models.DynamoModel.OpenFileFromJsonCommand command)`

DynamoModel.OpenFileFromJsonImpl

**Parameters:**
- `command` (Dynamo.Models.DynamoModel.OpenFileFromJsonCommand)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OpenFileFromPath(string filePath, bool forceManualExecutionMode)`

Opens a Dynamo workspace from a path to a file on disk.

**Parameters:**
- `filePath` (string) — Path to file
- `forceManualExecutionMode` (bool) — Set this to true to discard execution mode specified in the file and set manual mode

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OpenFileImpl(Dynamo.Models.DynamoModel.OpenFileCommand command)`

DynamoModel.OpenFileImpl

**Parameters:**
- `command` (Dynamo.Models.DynamoModel.OpenFileCommand)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OpenTemplateFromPath(string filePath, bool forceManualExecutionMode)`

Opens a Dynamo workspace from a path to a template on disk.

**Parameters:**
- `filePath` (string) — Path to file
- `forceManualExecutionMode` (bool) — Set this to true to discard execution mode specified in the file and set manual mode

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void OpenTemplateImpl(Dynamo.Models.DynamoModel.OpenFileCommand command)`

DynamoModel.OpenTemplateImpl

**Parameters:**
- `command` (Dynamo.Models.DynamoModel.OpenFileCommand)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void Paste()`

Paste ISelectable objects from the clipboard to the workspace so that the nodes appear in their original location with a slight offset

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void Paste(Dynamo.Utilities.Point2D targetPoint, bool useOffset)`

Paste ISelectable objects from the clipboard to the workspace at specified point.

**Parameters:**
- `targetPoint` (Dynamo.Utilities.Point2D) — Location where data will be pasted
- `useOffset` (bool) — Indicates whether we will use current workspace offset or paste nodes directly in this point.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void PostShutdownCore(bool shutdownHost)`

DynamoModel.PostShutdownCore

**Parameters:**
- `shutdownHost` (bool)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void PostTraceReconciliation(System.Collections.Generic.Dictionary<System.Guid, System.Collections.Generic.List<string>> orphanedSerializables)`

Deals with orphaned serializables.

**Parameters:**
- `orphanedSerializables` (System.Collections.Generic.Dictionary<System.Guid, System.Collections.Generic.List<string>>) — Collection of orphaned serializables.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void PreShutdownCore(bool shutdownHost)`

DynamoModel.PreShutdownCore

**Parameters:**
- `shutdownHost` (bool)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void RemoveWorkspace(Dynamo.Graph.Workspaces.WorkspaceModel workspace)`

Remove a workspace from the dynamo model.

**Parameters:**
- `workspace` (Dynamo.Graph.Workspaces.WorkspaceModel) — Workspace to remove

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void ResetEngine(bool markNodesAsDirty)`

Call this method to reset the virtual machine, avoiding a race condition by using a thread join inside the vm executive.

**Parameters:**
- `markNodesAsDirty` (bool) — Set this parameter to true to force reset of the execution substrate. Note that setting this parameter to true will have a negative performance impact.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void ResetEngineInternal()`

DynamoModel.ResetEngineInternal

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void SaveBackupFiles(object state)`

Backup all the files

**Parameters:**
- `state` (object)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void SetUICulture(string locale)`

Set UI culture based on preferences

**Parameters:**
- `locale` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void ShutDown(bool shutdownHost)`

External components call this method to shutdown DynamoModel. This method marks 'ShutdownRequested' property to 'true'. This method is used rather than a public virtual method to ensure that the value of ShutdownRequested is set to true.

**Parameters:**
- `shutdownHost` (bool) — Set this parameter to true to shutdown the host application.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `void ShutDownCore(bool shutdownHost)`

DynamoModel.ShutDownCore

**Parameters:**
- `shutdownHost` (bool)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `Dynamo.Models.DynamoModel Start()`

Start DynamoModel with all default configuration options

**Returns:** `Dynamo.Models.DynamoModel` — The instance of Dynamo.Models.DynamoModel

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `Dynamo.Models.DynamoModel Start(Dynamo.Models.DynamoModel.IStartConfiguration configuration)`

Start DynamoModel with custom configuration. Defaults will be assigned not provided.

**Parameters:**
- `configuration` (Dynamo.Models.DynamoModel.IStartConfiguration) — Start configuration

**Returns:** `Dynamo.Models.DynamoModel` — The instance of Dynamo.Models.DynamoModel

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

### Properties
- `AuthenticationManager` (Dynamo.Core.AuthenticationManager, get/set) — Returns authentication manager object for oxygen authentication.
- `CERLocation` (string, get) — The Autodesk CrashReport tool location on disk (directory that contains the "cer.dll")
- `CLIMode` (bool, get) — CLIMode indicates if we are running in DynamoCLI or DynamoWPFCLI mode. Note that in CLI mode Scheduler is synchronous.
- `ClipBoard` (System.Collections.ObjectModel.ObservableCollection<Dynamo.Graph.ModelBase>, get/set) — The copy/paste clipboard.
- `ConnectorType` (Dynamo.Graph.Connectors.ConnectorType, get/set) — Specifies how connectors are displayed in Dynamo.
- `CurrentWorkspace` (Dynamo.Graph.Workspaces.WorkspaceModel, get/set) — The active workspace in Dynamo.
- `EnableMigrationLogging` (bool, get/set) — Setting this flag enables creation of an XML in following format that records node mapping information - which old node has been converted to which to new node(s)
- `EngineController` (Dynamo.Engine.EngineController, get/set) — DesignScript VM EngineController, used for this instance of Dynamo.
- `ExtensionManager` (Dynamo.Extensions.IExtensionManager, get) — Manages all extensions for Dynamo
- `HostAnalyticsInfo` (Dynamo.Models.HostAnalyticsInfo, get/set) — Host analytics info
- `HostVersion` (string, get/set) — Current Version of the Host (i.e. DynamoRevit/DynamoStudio)
- `IsCrashing` (bool, get/set) — Specifies whether or not Dynamo is in a crash-state.
- `IsHeadless` (bool, get/set) — Flag to indicate that there is no UI on this process, and things like the node index process, update manager and the analytics collection should be disabled.
- `IsShowingConnectorTooltip` (bool, get/set) — Flag specifying the current state of whether or not to show tooltips in the graph. In addition to this toggle, tooltip is only available when connectors are set to 'bezier' mode.
- `IsShowingConnectors` (bool, get/set) — Specifies whether connectors are displayed in Dynamo.
- `IsTestMode` (bool, get/set) — Testing flag is used to defer calls to run in the idle thread with the assumption that the entire test will be wrapped in an idle thread call.
- `LinterManager` (Dynamo.Linting.LinterManager, get) — Manages the active linter
- `PathManager` (Dynamo.Interfaces.IPathManager, get) — The path manager that configures path information required for Dynamo to function properly. See IPathManager interface for more details.
- `PreferenceSettings` (Dynamo.Configuration.PreferenceSettings, get) — Preference settings for this instance of Dynamo.
- `Scheduler` (Dynamo.Scheduler.DynamoScheduler, get) — The Dynamo Scheduler, handles scheduling of asynchronous tasks on different threads.
- `ShutdownRequested` (bool, get) — Flag specifying whether a shutdown of Dynamo was requested.
- `State` (Dynamo.Models.DynamoModel.DynamoModelState, get) — The modelState tels us if the RevitDynamoModel was started and if has the the Dynamo UI attached to it or not
- `TraceReconciliationProcessor` (Dynamo.Engine.ITraceReconciliationProcessor, get/set) — An object which implements the ITraceReconciliationProcessor interface, and is used for handlling the results of a trace reconciliation.
- `Version` (string, get) — This version of Dynamo.
- `Workspaces` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Workspaces.WorkspaceModel>, get) — Returns collection of visible workspaces in Dynamo

### Events
#### `CleaningUp` (`System.Action`)

**Signature:** `public event System.Action CleaningUp`

An event triggered when the workspace is being cleaned.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `DeletionComplete` (`System.EventHandler`)

**Signature:** `public event System.EventHandler DeletionComplete`

Occurs after items of workspace are removed

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `DeletionStarted` (`System.Action`)

**Signature:** `public event System.Action DeletionStarted`

Occurs before items of workspace are removed

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `DynamoReady` (`System.Action<Dynamo.Extensions.ReadyParams>`)

**Signature:** `public event System.Action<Dynamo.Extensions.ReadyParams> DynamoReady`

This event is raised when Dynamo is ready for user interaction.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `EvaluationCompleted` (`System.EventHandler<Dynamo.Models.EvaluationCompletedEventArgs>`)

**Signature:** `public event System.EventHandler<Dynamo.Models.EvaluationCompletedEventArgs> EvaluationCompleted`

An event triggered when a single graph evaluation completes.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `PropertyChanged` (`System.ComponentModel.PropertyChangedEventHandler`)

**Signature:** `public event System.ComponentModel.PropertyChangedEventHandler PropertyChanged`

Occurs when a property of DynamoModel is changed

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `RefreshCompleted` (`System.Action<Dynamo.Graph.Workspaces.HomeWorkspaceModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Workspaces.HomeWorkspaceModel> RefreshCompleted`

An event triggered when all tasks in scheduler are completed.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `RequestCancelActiveStateForNode` (`Dynamo.Models.NodeHandler`)

**Signature:** `public event Dynamo.Models.NodeHandler RequestCancelActiveStateForNode`

Called when current state of a node is canelled.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `RequestDispatcherBeginInvoke` (`Dynamo.Models.ActionHandler`)

**Signature:** `public event Dynamo.Models.ActionHandler RequestDispatcherBeginInvoke`

Occurs when an action needs to be invoked asynchronously on a Dispather

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `RequestDispatcherInvoke` (`Dynamo.Models.ActionHandler`)

**Signature:** `public event Dynamo.Models.ActionHandler RequestDispatcherInvoke`

Occurs when an action needs to be invoked on a Dispather

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `RequestLayoutUpdate` (`System.EventHandler`)

**Signature:** `public event System.EventHandler RequestLayoutUpdate`

Occurs when changes in data may affect UI and UI needs to be refreshed

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `RequestNodeSelect` (`Dynamo.Models.NodeEventHandler`)

**Signature:** `public event Dynamo.Models.NodeEventHandler RequestNodeSelect`

An event which requests that a node be selected

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `RequestWorkspaceBackUpSave` (`System.Action<string, bool>`)

**Signature:** `public event System.Action<string, bool> RequestWorkspaceBackUpSave`

Occurs when a workspace is scheduled to be saved to a backup file.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `RequestsCrashPrompt` (`Dynamo.Models.DynamoModel.CrashPromptHandler`)

**Signature:** `public event Dynamo.Models.DynamoModel.CrashPromptHandler RequestsCrashPrompt`

DynamoModel.RequestsCrashPrompt

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `RequestsRedraw` (`System.EventHandler`)

**Signature:** `public event System.EventHandler RequestsRedraw`

DynamoModel.RequestsRedraw

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `RunCompleted` (`Dynamo.Models.DynamoModel.RunCompletedHandler`)

**Signature:** `public event Dynamo.Models.DynamoModel.RunCompletedHandler RunCompleted`

Occurs when running is completed

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `ShutdownCompleted` (`Dynamo.Models.DynamoModelHandler`)

**Signature:** `public event Dynamo.Models.DynamoModelHandler ShutdownCompleted`

This event is raised after DynamoModel has been shut down. At this point the DynamoModel is no longer valid and access to it should be avoided.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `ShutdownStarted` (`Dynamo.Models.DynamoModelHandler`)

**Signature:** `public event Dynamo.Models.DynamoModelHandler ShutdownStarted`

This event is raised right before the shutdown of DynamoModel started. When this event is raised, the shutdown is guaranteed to take place (i.e. user has had a chance to save the work and decided to proceed with shutting down Dynamo). Handlers of this event can still safely access the DynamoModel, the WorkspaceModel (along with its contents), and the DynamoScheduler.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `WorkspaceAdded` (`System.Action<Dynamo.Graph.Workspaces.WorkspaceModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Workspaces.WorkspaceModel> WorkspaceAdded`

Called when a workspace is added.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `WorkspaceCleared` (`System.Action<Dynamo.Graph.Workspaces.WorkspaceModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Workspaces.WorkspaceModel> WorkspaceCleared`

Occurs after current workspace is cleared

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `WorkspaceClearing` (`System.Action`)

**Signature:** `public event System.Action WorkspaceClearing`

Occurs before current workspace is cleared

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `WorkspaceClearingStarted` (`System.Action<Dynamo.Graph.Workspaces.WorkspaceModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Workspaces.WorkspaceModel> WorkspaceClearingStarted`

Occurs before current workspace is cleared

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `WorkspaceHidden` (`Dynamo.Models.WorkspaceHandler`)

**Signature:** `public event Dynamo.Models.WorkspaceHandler WorkspaceHidden`

Event called when a workspace is hidden

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `WorkspaceOpened` (`Dynamo.Models.WorkspaceHandler`)

**Signature:** `public event Dynamo.Models.WorkspaceHandler WorkspaceOpened`

Occurs when a workspaces is opened

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `WorkspaceOpening` (`System.Action<object>`)

**Signature:** `public event System.Action<object> WorkspaceOpening`

Event that is fired during the opening of the workspace. Use the XmlDocument object provided to conduct additional workspace opening operations.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `WorkspaceRemoveStarted` (`System.Action<Dynamo.Graph.Workspaces.WorkspaceModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Workspaces.WorkspaceModel> WorkspaceRemoveStarted`

Occurs before a workspace is removed

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `WorkspaceRemoved` (`System.Action<Dynamo.Graph.Workspaces.WorkspaceModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Workspaces.WorkspaceModel> WorkspaceRemoved`

Occurs after a workspace is removed

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `WorkspaceSaved` (`Dynamo.Models.WorkspaceHandler`)

**Signature:** `public event Dynamo.Models.WorkspaceHandler WorkspaceSaved`

Occurs when a workspace is saved to a file.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

#### `WorkspaceSaving` (`Dynamo.Models.WorkspaceSaveHandler`)

**Signature:** `public event Dynamo.Models.WorkspaceSaveHandler WorkspaceSaving`

Occurs when a workspace is about to be saved to a file.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

## DynamoModelHandler (delegate)

This delegate is used to manage Dynamo's shutting down.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

### Constructors
- `void DynamoModelHandler(object object, IntPtr method)` — DynamoModelHandler.DynamoModelHandler

### Methods
#### `System.IAsyncResult BeginInvoke(Dynamo.Models.DynamoModel model, System.AsyncCallback callback, object object)`

DynamoModelHandler.BeginInvoke

**Parameters:**
- `model` (Dynamo.Models.DynamoModel)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

#### `void EndInvoke(System.IAsyncResult result)`

DynamoModelHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

#### `void Invoke(Dynamo.Models.DynamoModel model)`

DynamoModelHandler.Invoke

**Parameters:**
- `model` (Dynamo.Models.DynamoModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

## DynamoPreferencesData (class)

This class contains the extra Dynamo-specific preferences data

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

### Constructors
- `void DynamoPreferencesData(double scaleFactor, bool hasRunWithoutCrash, bool isVisibleInDynamoLibrary, string version, string runType, string runPeriod)` — DynamoPreferencesData.DynamoPreferencesData

### Methods
#### `Dynamo.Models.DynamoPreferencesData Default()`

DynamoPreferencesData.Default

**Returns:** `Dynamo.Models.DynamoPreferencesData` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

### Properties
- `HasRunWithoutCrash` (bool, get) — DynamoPreferencesData.HasRunWithoutCrash
- `IsVisibleInDynamoLibrary` (bool, get) — DynamoPreferencesData.IsVisibleInDynamoLibrary
- `RunPeriod` (string, get) — DynamoPreferencesData.RunPeriod
- `RunType` (string, get) — DynamoPreferencesData.RunType
- `ScaleFactor` (double, get) — DynamoPreferencesData.ScaleFactor
- `Version` (string, get) — DynamoPreferencesData.Version

## EvaluationCompletedEventArgs (class)

This class represents the arguments when a graph evaluation is completed.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelEventArgs.cs)

### Constructors
- `void EvaluationCompletedEventArgs(bool evaluationTookPlace, System.Exception errorMsg)` — Creates EvaluationCompletedEventArgs

### Properties
- `Error` (System.Exception, get) — Exception thrown during graph evaluation.
- `EvaluationSucceeded` (bool, get) — Returns true if Evaluation is succeeded.
- `EvaluationTookPlace` (bool, get) — Returns true if there was any evaluation.

## HostAnalyticsInfo (struct)

Host analytics related info

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

## IEngineControllerManager (interface)

This class creates an interface for Engine controller.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModel.cs)

### Properties
- `EngineController` (Dynamo.Engine.EngineController, get) — A controller to coordinate the interactions between some DesignScript sub components like library management, live runner and so on.

## NodeAutocompleteSuggestion (enum)

Node Autocomplete suggestion values

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RunSettings.cs)

### Values
- `MLRecommendation` = `0`
- `ObjectType` = `1`

## NodeEventHandler (delegate)

This delegate is used in workspace events.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

### Constructors
- `void NodeEventHandler(object object, IntPtr method)` — NodeEventHandler.NodeEventHandler

### Methods
#### `System.IAsyncResult BeginInvoke(object sender, System.EventArgs e, System.AsyncCallback callback, object object)`

NodeEventHandler.BeginInvoke

**Parameters:**
- `sender` (object)
- `e` (System.EventArgs)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

#### `void EndInvoke(System.IAsyncResult result)`

NodeEventHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

#### `void Invoke(object sender, System.EventArgs e)`

NodeEventHandler.Invoke

**Parameters:**
- `sender` (object)
- `e` (System.EventArgs)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

## NodeHandler (delegate)

Delegate used in events, when it's required to send node.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

### Constructors
- `void NodeHandler(object object, IntPtr method)` — NodeHandler.NodeHandler

### Methods
#### `System.IAsyncResult BeginInvoke(Dynamo.Graph.Nodes.NodeModel node, System.AsyncCallback callback, object object)`

NodeHandler.BeginInvoke

**Parameters:**
- `node` (Dynamo.Graph.Nodes.NodeModel)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

#### `void EndInvoke(System.IAsyncResult result)`

NodeHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

#### `void Invoke(Dynamo.Graph.Nodes.NodeModel node)`

NodeHandler.Invoke

**Parameters:**
- `node` (Dynamo.Graph.Nodes.NodeModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

## NodeModelAssemblyLoader (class)

This class is responsible for loading types that derive from NodeModel. For information about package loading see the PackageLoader. For information about loading other libraries, see LibraryServices.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Core/NodeModelAssemblyLoader.cs)

### Constructors
- `void NodeModelAssemblyLoader()` — NodeModelAssemblyLoader.NodeModelAssemblyLoader

### Properties
- `LoadedAssemblies` (System.Collections.Generic.IEnumerable<System.Reflection.Assembly>, get) — All assemblies that have been loaded into Dynamo.

### Events
#### `AssemblyLoaded` (`Dynamo.Models.NodeModelAssemblyLoader.AssemblyLoadedHandler`)

**Signature:** `public event Dynamo.Models.NodeModelAssemblyLoader.AssemblyLoadedHandler AssemblyLoaded`

Event fired when a new assembly is loaded.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Core/NodeModelAssemblyLoader.cs)

## RunSettings (class)

The RunSettings object contains properties which control how execution is carried out.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RunSettings.cs)

### Constructors
- `void RunSettings()` — This method creates default RunSettings with DefaultRunPeriod, Manual run type and run is enabled.
- `void RunSettings(Dynamo.Models.RunType runType, int period)` — This function creates RunSettings with specified run type and run period.

### Properties
- `RunEnabled` (bool, get/set) — A flag which indicates whether running a graph is possible. If set to true, Dynamo will allow graphs to be run (except if the corresponding dyn file is not trusted) If set to false, Dynamo will not allow graphs to be run. The Dynamo UI will reflect the RunEnabled values by enabling/disabling certain UI features (ex. if RunEnabled is false, the Run button will be disabled).
- `RunPeriod` (int, get/set) — The length, in milliseconds, of the period between requests to execute.
- `RunType` (Dynamo.Models.RunType, get/set) — Returns or sets the current Run Type. E.g. Manual, Automatic, Periodic
- `RunTypesEnabled` (bool, get/set) — This property will enable or disable the ComboBox RunTypes

## RunType (enum)

The RunType enumeration provides values for specifying the type of run that will be conducted.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/RunSettings.cs)

### Values
- `Automatic` = `1`
- `Manual` = `0`
- `Periodic` = `2`

## WorkspaceHandler (delegate)

Represents the method that will handle workspace related events.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

### Constructors
- `void WorkspaceHandler(object object, IntPtr method)` — WorkspaceHandler.WorkspaceHandler

### Methods
#### `System.IAsyncResult BeginInvoke(Dynamo.Graph.Workspaces.WorkspaceModel workspace, System.AsyncCallback callback, object object)`

WorkspaceHandler.BeginInvoke

**Parameters:**
- `workspace` (Dynamo.Graph.Workspaces.WorkspaceModel)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

#### `void EndInvoke(System.IAsyncResult result)`

WorkspaceHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

#### `void Invoke(Dynamo.Graph.Workspaces.WorkspaceModel workspace)`

WorkspaceHandler.Invoke

**Parameters:**
- `workspace` (Dynamo.Graph.Workspaces.WorkspaceModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

## WorkspaceSaveHandler (delegate)

Represents the method that will handle workspace save related events.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

### Constructors
- `void WorkspaceSaveHandler(object object, IntPtr method)` — WorkspaceSaveHandler.WorkspaceSaveHandler

### Methods
#### `System.IAsyncResult BeginInvoke(Dynamo.Graph.Workspaces.WorkspaceModel workspace, Dynamo.Graph.SaveContext saveContext, System.AsyncCallback callback, object object)`

WorkspaceSaveHandler.BeginInvoke

**Parameters:**
- `workspace` (Dynamo.Graph.Workspaces.WorkspaceModel)
- `saveContext` (Dynamo.Graph.SaveContext)
- `callback` (System.AsyncCallback)
- `object` (object)

**Returns:** `System.IAsyncResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

#### `void EndInvoke(System.IAsyncResult result)`

WorkspaceSaveHandler.EndInvoke

**Parameters:**
- `result` (System.IAsyncResult)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

#### `void Invoke(Dynamo.Graph.Workspaces.WorkspaceModel workspace, Dynamo.Graph.SaveContext saveContext)`

WorkspaceSaveHandler.Invoke

**Parameters:**
- `workspace` (Dynamo.Graph.Workspaces.WorkspaceModel)
- `saveContext` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Models/DynamoModelDelegates.cs)

