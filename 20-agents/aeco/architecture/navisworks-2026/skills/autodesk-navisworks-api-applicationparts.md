---
name: navisworks-autodesk-navisworks-api-applicationparts
description: This skill encodes the navisworks 2026.0 surface of the Autodesk.Navisworks.Api.ApplicationParts namespace — 8 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ApplicationAutomation, ApplicationDragDrop, ApplicationOptions, ApplicationPlugins, ApplicationResources, ApplicationVersion, IApplicationBim360, IApplicationGui.
---

# Autodesk.Navisworks.Api.ApplicationParts

Auto-generated from vendor docs for navisworks 2026.0. 8 types in this namespace.

## ApplicationAutomation (class)

Provides the same interface as Api.Automation.NavisworksApplication but within the current process. Calls made via NavisworksApplication or command line when automating eventually call methods on here.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ApplicationParts.ApplicationAutomation)

### Methods
#### `void AddPluginAssembly(string fileName)`

Adds another assembly that plugins can be loaded from. Assembly is loaded using the LoadFrom context and can be used outside the Application root.

**Parameters:**
- `fileName` (string)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationAutomation.AddPluginAssembly%28System.String%29)

#### `void AppendFile(string fileName)`

Appends the Navisworks supported file to the currently loaded document

**Parameters:**
- `fileName` (string) — The file name including extension

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationAutomation.AppendFile%28System.String%29)

#### `void CreateCache(string fileNameToCache)`

Creates a corresponding nwc file for specified file.

**Parameters:**
- `fileNameToCache` (string) — The file name, excluding extension, for the cache file

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationAutomation.CreateCache%28System.String%29)

#### `void DisableProgress()`

Normal behavior is that Progress is displayed when performing long operations, even during Automated actions.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationAutomation.DisableProgress)

#### `void EnableProgress()`

Normal behavior is that Progress is displayed when performing long operations, even during Automated actions.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationAutomation.EnableProgress)

#### `int ExecuteAddInPlugin(string pluginId, string[] parameters)`

Executes an Addin Plugin whose full name is given by pluginId, passing in the paramaters given.

**Parameters:**
- `pluginId` (string) — The full identifier for the plugin.
This is formed as Name.DeveloperId
- `parameters` (string[]) — Optional parameters to pass to the plugin

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationAutomation.ExecuteAddInPlugin%28System.String%2CSystem.String%5B%5D%29)

#### `void GenerateThumbnail(int width, int height, string fileName)`

Generates a Thumbnail Image for this Document, and save it as a PNG image

**Parameters:**
- `width` (int) — The width of the image
- `height` (int) — The height of the image
- `fileName` (string) — The file name of the saved image

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationAutomation.GenerateThumbnail%28System.Int32%2CSystem.Int32%2CSystem.String%29)

#### `void GenerateThumbnailByRayTrace(int width, int height, string fileName)`

Generates a thumbnail render by ray trace for this Document, and save it as a PNG image

**Parameters:**
- `width` (int) — The width of the image
- `height` (int) — The height of the image
- `fileName` (string) — The file name of the saved image

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationAutomation.GenerateThumbnailByRayTrace%28System.Int32%2CSystem.Int32%2CSystem.String%29)

#### `void OpenFile(string fileName, string[] moreFiles)`

Loads one or more Navisworks supported files

**Parameters:**
- `fileName` (string) — First file to open
- `moreFiles` (string[]) — Other files to open

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationAutomation.OpenFile%28System.String%2CSystem.String%5B%5D%29)

#### `void Print()`

Sends the current View to the Printer

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationAutomation.Print)

#### `void Print(string printer)`

Sends the current View to the Printer

**Parameters:**
- `printer` (string) — The name of the printer to print to

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationAutomation.Print%28System.String%29)

#### `void Print(string printer, string driver)`

Sends the current View to the Printer

**Parameters:**
- `printer` (string) — The name of the printer to print to
- `driver` (string) — The name of the printer driver to use

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationAutomation.Print%28System.String%2CSystem.String%29)

#### `void Print(string printer, string driver, string port)`

Sends the current View to the Printer

**Parameters:**
- `printer` (string) — The name of the printer to print to
- `driver` (string) — The name of the printer driver to use
- `port` (string) — The name of the port the printer is connected to

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationAutomation.Print%28System.String%2CSystem.String%2CSystem.String%29)

#### `void SaveFile(string fileName)`

Save the documents loaded using OpenFile to a single Navisworks Document

**Parameters:**
- `fileName` (string) — The file name including extension

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationAutomation.SaveFile%28System.String%29)

## ApplicationDragDrop (class)

Represents Application level Drag & Drop.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ApplicationParts.ApplicationDragDrop)

### Methods
#### `bool DecodeData(System.Windows.Forms.IDataObject data, out int processId)`

Tries to decode the data passed in drag drop operations. If successful determines the source Process Id.

**Parameters:**
- `data` (System.Windows.Forms.IDataObject)
- `processId` (int)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationDragDrop.DecodeData%28System.Windows.Forms.IDataObject%2CSystem.Int32%40%29)

#### `bool DecodeDataWpf(System.Windows.IDataObject data, out int processId)`

Tries to decode the data passed in drag drop operations. If successful determines the source Process Id.

**Parameters:**
- `data` (System.Windows.IDataObject)
- `processId` (int)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationDragDrop.DecodeDataWpf%28System.Windows.IDataObject%2CSystem.Int32%40%29)

### Properties
- `Format` (string, get/set) — Name of the format used for Navisworks drag drop operations. Currently is just a notification theat Current Selection has been dragged.

## ApplicationOptions (class)

Represents Application level options

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ApplicationParts.ApplicationOptions)

### Properties
- `Grids` (Autodesk.Navisworks.Api.GridsOptions, get) — Provides access to Grids Options.

## ApplicationPlugins (class)

Provides information about the Plugins available in the application runtime

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ApplicationParts.ApplicationPlugins)

### Methods
#### `void AddPluginAssembly(string fileName)`

Adds another assembly that plugins can be loaded from. Assembly is loaded using the LoadFrom context and can be used outside the Application root.

**Parameters:**
- `fileName` (string)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationPlugins.AddPluginAssembly%28System.String%29)

#### `int ExecuteAddInPlugin(string pluginId, string[] parameters)`

Executes an Addin Plugin whose full name is given by pluginId, passing in the paramaters given.

**Parameters:**
- `pluginId` (string) — The full identifier for the plugin.
This is formed as Name.DeveloperId
- `parameters` (string[]) — Optional parameters to pass to the plugin

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationPlugins.ExecuteAddInPlugin%28System.String%2CSystem.String%5B%5D%29)

#### `System.Collections.ObjectModel.ReadOnlyCollection<Autodesk.Navisworks.Api.Plugins.PluginRecord> FindInterfaces(string interfaceId)`

Returns a collection of PluginRecords that implement a particular interface

**Parameters:**
- `interfaceId` (string) — The Id of the InterfaceRecord

**Returns:** `System.Collections.ObjectModel.ReadOnlyCollection<Autodesk.Navisworks.Api.Plugins.PluginRecord>` — Collection if empty if no mathcing PluginRecordsare found

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationPlugins.FindInterfaces%28System.String%29)

#### `Autodesk.Navisworks.Api.Plugins.PluginRecord FindPlugin(string pluginId)`

Searches for a PluginRecord by Id

**Parameters:**
- `pluginId` (string) — The full identifier for the plugin.
This is formed as Name.DeveloperId

**Returns:** `Autodesk.Navisworks.Api.Plugins.PluginRecord` — Returns the PluginRecord or Null if it cannot be found

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationPlugins.FindPlugin%28System.String%29)

### Properties
- `PluginRecords` (System.Collections.ObjectModel.ReadOnlyCollection<Autodesk.Navisworks.Api.Plugins.PluginRecord>, get) — Gets collection of PluginRecords that describe the available plugins

### Events
#### `PluginRecordsChanged` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> PluginRecordsChanged`

Occurs when the collection of PluginRecords has changed.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationPlugins.PluginRecordsChanged)

#### `PluginRecordsChanging` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> PluginRecordsChanging`

Occurs when the collection of PluginRecords is about to change.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationPlugins.PluginRecordsChanging)

## ApplicationResources (class)

Provides access to Application Resources

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ApplicationParts.ApplicationResources)

### Methods
#### `string GetString(string name)`

Looks up a localizable string

**Parameters:**
- `name` (string) — The identifier of the string to lookup

**Returns:** `string` — Returns localized string

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationResources.GetString%28System.String%29)

#### `string GetStringSafe(string name)`

Looks up a localizable string

**Parameters:**
- `name` (string) — The identifier of the string to lookup

**Returns:** `string` — Returns localized string or value of 'name' if it cannot be found

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationResources.GetStringSafe%28System.String%29)

#### `string TryGetString(string name)`

Looks up a localizable string

**Parameters:**
- `name` (string) — The identifier of the string to lookup

**Returns:** `string` — Returns localized string or Null if it cannot be found

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.ApplicationResources.TryGetString%28System.String%29)

## ApplicationVersion (class)

Provides information about the version of the currently running API and Runtime

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ApplicationParts.ApplicationVersion)

### Properties
- `ApiMajor` (int, get) — Major version number for the Navisworks API. Changes when a non-backwards compatible change is made to a stable version of the API. Always less than or equal to RuntimeMajor.
- `ApiMinor` (int, get) — Minor version number for the Navisworks API. Changes when there is a behavioral or minor change to a stable version of the API that may have an impact on backwards compatibility. Always less than or equal to RuntimeMinor.
- `Build` (int, get) — Build number for Api and Runtime. Changes on every build (and hence every release).
- `IsApiStable` (bool, get) — Is this version of the API stable ? If false, breaking changes may be made at any time and client code must be recompiled for every build. Api will be unstable for alpha and early beta releases.
- `IsRuntimeBeta` (bool, get) — Is this an alpha or beta version of the runtime ?
- `Runtime` (string, get) — Identifies the Navisworks runtime that is providing this implementation of the API. Will typically be an installed Navisworks product.
- `RuntimeLanguage` (string, get) — Gets the language code corresponding to the one Navisworks is using.
- `RuntimeMajor` (int, get) — Major version number for the current Navisworks Runtime. This changes with every major (annual) release of Navisworks products.
- `RuntimeMinor` (int, get) — Minor version number for the current Navisworks Runtime. This changes with every minor (service pack or subscription release) update of Navisworks products.
- `RuntimeProductName` (string, get) — The complete product name for the Navisworks runtime that is providing this implementation of the API.

## IApplicationBim360 (interface)

Provides access to the BIM360 state of the application which is hosting the API.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ApplicationParts.IApplicationBim360)

### Methods
#### `object GetRestApi(Autodesk.Navisworks.Api.Bim360.AccountInfo accountInfo)`

Rest Api access.

**Parameters:**
- `accountInfo` (Autodesk.Navisworks.Api.Bim360.AccountInfo)

**Returns:** `object` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.IApplicationBim360.GetRestApi%28Autodesk.Navisworks.Api.Bim360.AccountInfo%29)

#### `object GetRestApiV3(Autodesk.Navisworks.Api.Bim360.AccountInfo accountInfo)`

Rest Api access.

**Parameters:**
- `accountInfo` (Autodesk.Navisworks.Api.Bim360.AccountInfo)

**Returns:** `object` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.IApplicationBim360.GetRestApiV3%28Autodesk.Navisworks.Api.Bim360.AccountInfo%29)

#### `Autodesk.Navisworks.Api.Bim360.ResourceInfo ParseBim360Uri(string input)`

Attempts to parse a string as a BIM 360 URI, and then determines resource information.

**Parameters:**
- `input` (string)

**Returns:** `Autodesk.Navisworks.Api.Bim360.ResourceInfo` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.IApplicationBim360.ParseBim360Uri%28System.String%29)

#### `bool TryParseBim360Uri(string input, out Autodesk.Navisworks.Api.Bim360.ResourceInfo uriInfo)`

Attempts to parse a string as a BIM 360 URI, and then determines resource information.

**Parameters:**
- `input` (string)
- `uriInfo` (Autodesk.Navisworks.Api.Bim360.ResourceInfo)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.IApplicationBim360.TryParseBim360Uri%28System.String%2CAutodesk.Navisworks.Api.Bim360.ResourceInfo%40%29)

### Properties
- `Accounts` (System.Collections.ObjectModel.Collection<Autodesk.Navisworks.Api.Bim360.AccountInfo>, get) — Gives access to all current Accounts. Youu'll only get these once SignedIn.
- `HasAnyBim360ModelOpen` (bool, get) — Returns true if any BIM 360 Models are loaded.
- `IsSignedIn` (bool, get) — BIM 360 SignIn support.

### Events
#### `RefreshBeginning` (`System.EventHandler`)

**Signature:** `event System.EventHandler RefreshBeginning`

Occurs when BIM 360 data is about to refresh.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.ApplicationParts.IApplicationBim360.RefreshBeginning)

#### `RefreshComplete` (`System.EventHandler`)

**Signature:** `event System.EventHandler RefreshComplete`

Occurs when the refresh of BIM 360 data is complete.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.ApplicationParts.IApplicationBim360.RefreshComplete)

## IApplicationGui (interface)

Provides access to the Gui of the application which is hosting the API.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ApplicationParts.IApplicationGui)

### Methods
#### `bool GetDockPanePluginVisibility(string pluginId)`

Called when a DockPanePlugin gets it's Visible property

**Parameters:**
- `pluginId` (string) — The full identifier for the plugin.
This is formed as Name.DeveloperId

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.IApplicationGui.GetDockPanePluginVisibility%28System.String%29)

#### `bool IsSignedIn()`

Reports if there is an authenticated user.

**Returns:** `bool` — Sign In status.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.IApplicationGui.IsSignedIn)

#### `void SetDockPanePluginActive(string pluginId)`

Called when a DockPanePlugin activates the plugin pane.

**Parameters:**
- `pluginId` (string) — The full identifier for the plugin.
This is formed as Name.DeveloperId

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.IApplicationGui.SetDockPanePluginActive%28System.String%29)

#### `void SetDockPanePluginVisibility(string pluginId, bool visible)`

Called when a DockPanePlugin sets it's Visible property

**Parameters:**
- `pluginId` (string) — The full identifier for the plugin.
This is formed as Name.DeveloperId
- `visible` (bool) — The value to set

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.IApplicationGui.SetDockPanePluginVisibility%28System.String%2CSystem.Boolean%29)

#### `void SetRaiseIdle(System.Action<System.EventArgs> raiseIdle)`

Allows Idle to be raised.

**Parameters:**
- `raiseIdle` (System.Action<System.EventArgs>) — Action to raise Idle. Will be called with valid Action during GUI
creation, and with null during GUI destruction.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.IApplicationGui.SetRaiseIdle%28System.Action%7BSystem.EventArgs%7D%29)

#### `bool SignIn()`

Authenticate with Autodesk ID. Popup generic sign-in UI in Navisworks to allow users to enter the credentials.

**Returns:** `bool` — true if authenticated or false if failed to authenticate.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.IApplicationGui.SignIn)

#### `bool SignOut()`

Sign-out the current authenticated user.

**Returns:** `bool` — true if succeeded or false if failed.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ApplicationParts.IApplicationGui.SignOut)

### Properties
- `MainWindow` (System.Windows.Forms.IWin32Window, get) — Get the main window of the application. Use as parent window for dialogs and message boxes.
- `ServerType` (Autodesk.Navisworks.Api.ServerType, get) — Sign In server type.
- `SignInUserId` (string, get) — The internal user ID of the user currently logged in. The user ID will be null if no user is logged in.
- `SignInUserName` (string, get) — The current authenticated user's Autodesk ID.

### Events
#### `Closed` (`System.EventHandler`)

**Signature:** `event System.EventHandler Closed`

Occurs when the Gui (including the main window) is about to close.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.ApplicationParts.IApplicationGui.Closed)

#### `Closing` (`System.ComponentModel.CancelEventHandler`)

**Signature:** `event System.ComponentModel.CancelEventHandler Closing`

Occurs when the Gui is about to close and can be handled to cancel closure.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.ApplicationParts.IApplicationGui.Closing)

#### `ServerTypeChanged` (`System.EventHandler`)

**Signature:** `event System.EventHandler ServerTypeChanged`

Occurs when the server type changes.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.ApplicationParts.IApplicationGui.ServerTypeChanged)

#### `SignedIn` (`System.EventHandler`)

**Signature:** `event System.EventHandler SignedIn`

Occurs when user signs in.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.ApplicationParts.IApplicationGui.SignedIn)

#### `SignedOut` (`System.EventHandler`)

**Signature:** `event System.EventHandler SignedOut`

Occurs when user has signed out.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.ApplicationParts.IApplicationGui.SignedOut)

#### `SigningOut` (`System.EventHandler`)

**Signature:** `event System.EventHandler SigningOut`

Occurs when user is signing out.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.ApplicationParts.IApplicationGui.SigningOut)

