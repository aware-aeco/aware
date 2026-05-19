---
name: grasshopper-grasshopper-plugin
description: This skill encodes the grasshopper 8.0 surface of the Grasshopper.Plugin namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GH_PluginUtil, GH_ResourceGate, GH_RhinoScriptInterface.
---

# Grasshopper.Plugin

Auto-generated from vendor docs for grasshopper 8.0. 3 types in this namespace.

## GH_PluginUtil (class)

Grasshopper plugin utility class. This class exports some static (Shared in VB) methods to do with plugin loading and unloading.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Plugin_GH_PluginUtil.htm)

### Methods
#### `public static void BringRhinoToTop()`

Bring the Rhino window to the top of the stack.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_PluginUtil_BringRhinoToTop.htm)

#### `public static bool BringWindowToTop(IntPtr hWnd)`



**Parameters:**
- `hWnd` (System.IntPtr)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_PluginUtil_BringWindowToTop.htm)

#### `public static void FocusOnRhino()`

Set the focus to the Rhino command line.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_PluginUtil_FocusOnRhino.htm)

#### `public static void SendKeyCodeToRhino(char key)`

Send a key-code to the Rhino command line.

**Parameters:**
- `key` (System.Char) — Key char.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_PluginUtil_SendKeyCodeToRhino.htm)

#### `public static void SendKeyCodeToRhino(int key)`

Send a key-code to the Rhino command line.

**Parameters:**
- `key` (System.Int32) — Key code.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_PluginUtil_SendKeyCodeToRhino_1.htm)

#### `public static void SendKeyCodeToRhino(string key)`

Send a string of characters to the Rhino command line.

**Parameters:**
- `key` (System.String) — String

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_PluginUtil_SendKeyCodeToRhino_2.htm)

#### `public static IntPtr SendMessage(IntPtr hWnd, int msg, int wParam, IntPtr lParam)`



**Parameters:**
- `hWnd` (System.IntPtr)
- `msg` (System.Int32)
- `wParam` (System.Int32)
- `lParam` (System.IntPtr)

**Returns:** `IntPtr` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_PluginUtil_SendMessage.htm)

#### `public static void SetFocus(IntPtr hWnd)`



**Parameters:**
- `hWnd` (System.IntPtr)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_PluginUtil_SetFocus.htm)

## GH_ResourceGate (class)

This class provides access to specific resources of the Grasshopper assembly.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Plugin_GH_ResourceGate.htm)

### Properties
- `BlackIcon` (Bitmap, get) — Returns a 24x24 hexagonal black icon.
- `Error_12x12` (Bitmap, get) — Returns a 12x12 round red icon
- `Error_16x16` (Bitmap, get) — Returns a 16x16 round red icon
- `Error_24x24` (Bitmap, get) — Returns a 24x24 round red icon
- `Error_40x40` (Bitmap, get) — Returns a 40x40 round red icon
- `Info_12x12` (Bitmap, get) — Returns a 12x12 round blue icon
- `Info_16x16` (Bitmap, get) — Returns a 16x16 round blue icon
- `Info_24x24` (Bitmap, get) — Returns a 24x24 round blue icon
- `Info_40x40` (Bitmap, get) — Returns a 40x40 round blue icon
- `OK_12x12` (Bitmap, get) — Returns a 12x12 round green icon
- `OK_16x16` (Bitmap, get) — Returns a 16x16 round green icon
- `OK_24x24` (Bitmap, get) — Returns a 24x24 round green icon
- `OK_40x40` (Bitmap, get) — Returns a 40x40 round green icon
- `Warning_12x12` (Bitmap, get) — Returns a 12x12 round yellow icon
- `Warning_16x16` (Bitmap, get) — Returns a 16x16 round yellow icon
- `Warning_24x24` (Bitmap, get) — Returns a 24x24 round yellow icon
- `Warning_40x40` (Bitmap, get) — Returns a 40x40 round yellow icon
- `WhiteIcon` (Bitmap, get) — Returns a 24x24 square white icon.

## GH_RhinoScriptInterface (class)

Plugin Interface object that is exposed via the RhinoScript layer.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Plugin_GH_RhinoScriptInterface.htm)

### Constructors
- `public GH_RhinoScriptInterface()` — Initializes a new instance of the GH_RhinoScriptInterface class

### Methods
#### `public bool AssignDataToParameter(string parameterID, Object data)`

Find a parameter and assign some persistent data.

**Parameters:**
- `parameterID` (System.String) — Instance ID of parameter or parameter name.
- `data` (System.Object) — Data to assign.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_AssignDataToParameter.htm)

#### `public Object BakeDataInObject(string objectID)`

Find an object and bake all geometry inside of it.

**Parameters:**
- `objectID` (System.String) — Object InstanceID or name.

**Returns:** `Object` — An array of Rhino object IDs, or null on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_BakeDataInObject.htm)

#### `public bool CloseAllDocuments()`

Close all Grasshopper documents.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_CloseAllDocuments.htm)

#### `public bool CloseDocument()`

Close the currently active Grasshopper document. If there is not active document, nothing will happen.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_CloseDocument.htm)

#### `public void DisableBanner()`

Disables the display of the Grasshopper banner during Component loading. The banner is typically only shown once during a Grasshopper session, namely when the Editor is first loaded.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_DisableBanner.htm)

#### `public void DisableSolver()`

Disables the Grasshopper Solver. If the Solver is disabled, expired components and parameter will not be recomputed, though any existing solution will remain intact.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_DisableSolver.htm)

#### `public void EnableBanner()`

Enables the display of the Grasshopper banner during Component loading. The banner is typically only shown once during a Grasshopper session, namely when the Editor is first loaded.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_EnableBanner.htm)

#### `public void EnableSolver()`

Enables the Grasshopper Solver. If the Solver is enabled, expired components and parameter will be recomputed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_EnableSolver.htm)

#### `public void HideEditor()`

Hide the main Grasshopper Editor. If the editor hasn't been loaded or if the Editor is already hidden, nothing will happen.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_HideEditor.htm)

#### `public bool IsEditorLoaded()`

Returns the loaded state of the Grasshopper Main window.

**Returns:** `Boolean` — True if the Main Grasshopper Window has been loaded.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_IsEditorLoaded.htm)

#### `public bool IsEditorVisible()`

Returns the visible state of the Grasshopper Main window.

**Returns:** `Boolean` — True if the Main Grasshopper Window has been loaded and is visible.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_IsEditorVisible.htm)

#### `public bool IsSolverEnabled()`

Returns the state of the Grasshopper Solver.

**Returns:** `Boolean` — True if the Grasshopper Solver is enabled.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_IsSolverEnabled.htm)

#### `public void LoadEditor()`

Load the main Grasshopper Editor. If the editor has already been loaded nothing will happen.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_LoadEditor.htm)

#### `public Guid[] LoadedPluginGuids()`

Gets an array of the IDs of all loaded plugins.

**Returns:** `Guid[]` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_LoadedPluginGuids.htm)

#### `public string[] LoadedPluginNames()`

Gets an array of the names and versions (if known) of all loaded plugins.

**Returns:** `String[]` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_LoadedPluginNames.htm)

#### `public bool OpenDocument(string filename)`

Open a Grasshopper document. The editor will be loaded if necessary, but it will not be automatically shown.

**Parameters:**
- `filename` (System.String) — Path of file to open (must be a *.gh or *.ghx extension).

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_OpenDocument.htm)

#### `[ObsoleteAttribute("Please use RunInCommandContext")] public Result RunAsCommand(GH_Document ghDoc, Command command, RhinoDoc rhinoDoc, RunMode mode)`

Obsolete.

**Parameters:**
- `ghDoc` (Grasshopper.Kernel.GH_Document)
- `command` (Command)
- `rhinoDoc` (RhinoDoc)
- `mode` (RunMode)

**Returns:** `Result` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_RunAsCommand.htm)

#### `public void RunHeadless()`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_RunHeadless.htm)

#### `public Result RunInCommandContext(GH_Document ghDoc, Command command, RhinoDoc rhinoDoc, RunMode mode)`



**Parameters:**
- `ghDoc` (Grasshopper.Kernel.GH_Document)
- `command` (Command)
- `rhinoDoc` (RhinoDoc)
- `mode` (RunMode)

**Returns:** `Result` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_RunInCommandContext.htm)

#### `public void RunSolver(bool expireAllObjects)`

Runs the solver once, even if the global solver lock is on.

**Parameters:**
- `expireAllObjects` (System.Boolean) — If true, will expire all objects in the current document.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_RunSolver.htm)

#### `public bool SaveDocument()`

Save the currently active Grasshopper document. If the active document has never been saved, a Save... dialog will be shown. If there is no active document, nothing will happen.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_SaveDocument.htm)

#### `public bool SaveDocumentAs(string filename)`

Save the currently active Grasshopper document in a specific location. If there is no active document, nothing will happen.

**Parameters:**
- `filename` (System.String) — Path of file to save to (must be a *.gh or *ghx extension).

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_SaveDocumentAs.htm)

#### `public bool SetSliderRangeAndValue(string sliderID, double value, double minimum, double maximum)`

Find a slider and assign a new value.

**Parameters:**
- `sliderID` (System.String) — Slider ID.
- `value` (System.Double) — New value. If the value exceeds the limits, it will be clipped.
- `minimum` (System.Double) — New slider minimum.
- `maximum` (System.Double) — New slider maximum.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_SetSliderRangeAndValue.htm)

#### `public bool SetSliderValue(string sliderID, double value)`

Find a slider and assign a new value.

**Parameters:**
- `sliderID` (System.String) — Slider ID.
- `value` (System.Double) — New value. If the value exceeds the limits, it will be clipped.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_SetSliderValue.htm)

#### `public void ShowEditor()`

Show the main Grasshopper Editor. The editor will be loaded first if needed. If the Editor is already on screen, nothing will happen.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Plugin_GH_RhinoScriptInterface_ShowEditor.htm)

