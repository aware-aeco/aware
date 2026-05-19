---
name: rhino-rhino-applicationsettings
description: This skill encodes the rhino 7.0 surface of the Rhino.ApplicationSettings namespace — 44 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AppearanceSettingsState, CommandAliasList, AppearanceSettings, CursorTooltipSettings, CursorTooltipSettingsState, CurvatureAnalysisSettings, CurvatureAnalysisSettingsState, DraftAngleAnalysisSettings, and 36 more types.
---

# Rhino.ApplicationSettings

Auto-generated from vendor docs for rhino 7.0. 44 types in this namespace.

## AppearanceSettings (class)

Provides static methods and properties to deal with the appearance of the application.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_AppearanceSettings.htm)

### Methods
#### `public static Color DefaultPaintColor(PaintColor whichColor)`

Get a default paint color for Rhino. The current paint color may be different than the default

**Parameters:**
- `whichColor` (Rhino.ApplicationSettings.PaintColor) — The color to retrieve

**Returns:** `Color` — [Missing <returns> documentation for "M:Rhino.ApplicationSettings.AppearanceSettings.DefaultPaintColor(Rhino.ApplicationSettings.PaintColor)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_AppearanceSettings_DefaultPaintColor.htm)

#### `public static AppearanceSettingsState GetCurrentState()`

Gets the current settings of the application.

**Returns:** `AppearanceSettingsState` — An instance of a class that represents all the settings as they appear in the Rhino _Options dialog, joined in a single class.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_AppearanceSettings_GetCurrentState.htm)

#### `public static AppearanceSettingsState GetDefaultState()`

Gets the factory settings of the application.

**Returns:** `AppearanceSettingsState` — An instance of a class that represents all the default settings joined together.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_AppearanceSettings_GetDefaultState.htm)

#### `public static Color GetPaintColor(PaintColor whichColor)`

Gets the color that is currently associated with a paint color.

**Parameters:**
- `whichColor` (Rhino.ApplicationSettings.PaintColor) — A color association.

**Returns:** `Color` — A .Net library color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_AppearanceSettings_GetPaintColor.htm)

#### `public static Color GetPaintColor(PaintColor whichColor, bool compute)`

Gat a paint color. This overload provides a compute option for cases where colors are computed when they are "unset" colors.

**Parameters:**
- `whichColor` (Rhino.ApplicationSettings.PaintColor) — [Missing <param name="whichColor"/> documentation for "M:Rhino.ApplicationSettings.AppearanceSettings.GetPaintColor(Rhino.ApplicationSettings.PaintColor,System.Boolean)"]
- `compute` (System.Boolean) — if true, a color is computed in some cases

**Returns:** `Color` — [Missing <returns> documentation for "M:Rhino.ApplicationSettings.AppearanceSettings.GetPaintColor(Rhino.ApplicationSettings.PaintColor,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_AppearanceSettings_GetPaintColor_1.htm)

#### `public static Color GetWidgetColor(WidgetColor whichColor)`

Gets the .Net library color that is currently associated with a widget color.

**Parameters:**
- `whichColor` (Rhino.ApplicationSettings.WidgetColor) — A color association.

**Returns:** `Color` — A .Net library color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_AppearanceSettings_GetWidgetColor.htm)

#### `public static bool InitialMainWindowPosition(out Rectangle bounds)`

Location where the Main Rhino window attempts to show when the application is first started.

**Parameters:**
- `bounds` (System.Drawing.Rectangle) — The rectangle in which the main window attempts to shows is assigned to this out parameter during the call.

**Returns:** `Boolean` — false if the information could not be retrieved.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_AppearanceSettings_InitialMainWindowPosition.htm)

#### `public static void RestoreDefaults()`

Commits the default settings as the current settings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_AppearanceSettings_RestoreDefaults.htm)

#### `public static void SetPaintColor(PaintColor whichColor, Color c)`

Sets the logical paint color association to a spacific .Net library color, without forced UI update.

**Parameters:**
- `whichColor` (Rhino.ApplicationSettings.PaintColor) — A logical color association.
- `c` (System.Drawing.Color) — A .Net library color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_AppearanceSettings_SetPaintColor.htm)

#### `public static void SetPaintColor(PaintColor whichColor, Color c, bool forceUiUpdate)`

Sets the logical paint color association to a spacific .Net library color.

**Parameters:**
- `whichColor` (Rhino.ApplicationSettings.PaintColor) — A logical color association.
- `c` (System.Drawing.Color) — A .Net library color.
- `forceUiUpdate` (System.Boolean) — true if the UI should be forced to update.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_AppearanceSettings_SetPaintColor_1.htm)

#### `public static void SetWidgetColor(WidgetColor whichColor, Color c)`

Sets the logical widget color association to a spacific .Net library color, without forced UI update.

**Parameters:**
- `whichColor` (Rhino.ApplicationSettings.WidgetColor) — A logical color association.
- `c` (System.Drawing.Color) — A .Net library color.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_AppearanceSettings_SetWidgetColor.htm)

#### `public static void SetWidgetColor(WidgetColor whichColor, Color c, bool forceUiUpdate)`

Sets the logical widget color association to a spacific .Net library color.

**Parameters:**
- `whichColor` (Rhino.ApplicationSettings.WidgetColor) — A logical color association.
- `c` (System.Drawing.Color) — A .Net library color.
- `forceUiUpdate` (System.Boolean) — true if the UI should be forced to update.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_AppearanceSettings_SetWidgetColor_1.htm)

#### `public static void UpdateFromState(AppearanceSettingsState state)`

Sets all settings to a particular defined joined state.

**Parameters:**
- `state` (Rhino.ApplicationSettings.AppearanceSettingsState) — A joined settings object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_AppearanceSettings_UpdateFromState.htm)

### Properties
- `CommandPromptBackgroundColor` (Color, get/set) — Gets or sets the color of the command prompt background.
- `CommandPromptFontSize` (Int32, get/set) — Size of font used in command prompt (in points)
- `CommandPromptHypertextColor` (Color, get/set) — Gets or sets the color of the command prompt hypertext.
- `CommandPromptPosition` (CommandPromptPosition, get/set) — Gets or sets the command prompt position.
- `CommandPromptTextColor` (Color, get/set) — Gets or sets the color of the command prompt text.
- `CrosshairColor` (Color, get/set) — Gets or sets the color of the crosshair icon.
- `CurrentLayerBackgroundColor` (Color, get/set) — Gets or sets the color used by the layer manager dialog as the background color for the current layer.
- `DefaultFontFaceName` (String, get) — Gets or sets the default font face name used in Rhino.
- `DefaultLayerColor` (Color, get/set) — Gets or sets the default layer color.
- `DefaultObjectColor` (Color, get/set) — Gets or sets the default color for new objects.
- `EchoCommandsToHistoryWindow` (Boolean, get/set) — Gets or sets a value that determines if command names are written to the history window.
- `EchoPromptsToHistoryWindow` (Boolean, get/set) — Gets or sets a value that determines if prompt messages are written to the history window.
- `EditCandidateColor` (Color, get/set) — Gets or sets the color of objects that are eligible to be edited.
- `FeedbackColor` (Color, get/set) — Gets or sets the feedback color.
- `FrameBackgroundColor` (Color, get/set) — Gets or sets the background color of the frame.
- `GridThickLineColor` (Color, get/set) — Gets or sets the color of the thick line of the grid.
- `GridThinLineColor` (Color, get/set) — Gets or sets the color of the thin line of the grid.
- `GridXAxisLineColor` (Color, get/set) — Gets or sets the color of the X axis of the grid.
- `GridYAxisLineColor` (Color, get/set) — Gets or sets the color of the Y axis of the grid.
- `GridZAxisLineColor` (Color, get/set) — Gets or sets the color of the Z axis of the grid.
- `LanguageIdentifier` (Int32, get/set) — Gets or sets the language identifier.
- `LockedObjectColor` (Color, get/set) — color used to draw locked objects.
- `MenuVisible` (Boolean, get/set) — Gets or sets a value that determines if the File menu is visible.
- `PageviewPaperColor` (Color, get/set) — Gets or sets the paper background. A rectangle is drawn into the background of page views to represent the printed area. The alpha portion of the color is used to draw the paper blended into the background
- `PreviousLanguageIdentifier` (Int32, get/set) — Gets or sets the previous language identifier.
- `SelectedObjectColor` (Color, get/set) — The color used to draw selected objects. The default is yellow, but this can be customized by the user.
- `SelectionWindowCrossingFillColor` (Color, get/set) — Color used to fill selection crossing window
- `SelectionWindowCrossingStrokeColor` (Color, get/set) — Color used to draw stroke for selection crossing window
- `SelectionWindowFillColor` (Color, get/set) — Color used to fill selection window
- `SelectionWindowStrokeColor` (Color, get/set) — Color used to draw stroke for selection window
- `ShowCrosshairs` (Boolean, get/set) — Gets or sets a value that determines if cross hairs are visible.
- `ShowFullPathInTitleBar` (Boolean, get/set) — Gets or sets a value that determines if the full path of the document is shown in the Rhino title bar.
- `ShowOsnapBar` (Boolean, get/set) — Shows or hides the object snap user interface.
- `ShowSideBar` (Boolean, get/set) — Shows or hides the side bar user interface.
- `ShowStatusBar` (Boolean, get/set) — Shows or hides the status bar user interface.
- `TrackingColor` (Color, get/set) — Gets or sets the tracking color.
- `UsePaintColors` (Boolean, get) — Gets or sets a value indicating if logical paint colors should be used.
- `ViewportBackgroundColor` (Color, get/set) — Gets or sets the viewport background color.
- `WorldCoordIconXAxisColor` (Color, get/set) — Gets or sets the color of the world coordinate X axis.
- `WorldCoordIconYAxisColor` (Color, get/set) — Gets or sets the color of the world coordinate Y axis.
- `WorldCoordIconZAxisColor` (Color, get/set) — Gets or sets the color of the world coordinate Z axis.

## AppearanceSettingsState (class)

snapshot of the values in AppearanceSettings

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_AppearanceSettingsState.htm)

### Properties
- `CommandPromptBackgroundColor` (Color, get/set) — Gets or sets the command prompt background color.
- `CommandPromptFontSize` (Int32, get/set) — Size of the font used in the command prompt (in points)
- `CommandPromptHypertextColor` (Color, get/set) — Gets or sets the command prompt hypertext color.
- `CommandPromptTextColor` (Color, get/set) — Gets or sets the command prompt text color.
- `CrosshairColor` (Color, get/set) — Gets or sets the crosshair color.
- `CurrentLayerBackgroundColor` (Color, get/set) — Gets or sets the color used by the layer manager dialog as the background color for the current layer.
- `DefaultFontFaceName` (String, get/set) — Gets or sets the name of the default font face.
- `DefaultLayerColor` (Color, get/set) — Gets or sets the default layer color.
- `DefaultObjectColor` (Color, get/set) — Gets or sets the default object color.
- `EchoCommandsToHistoryWindow` (Boolean, get/set) — Gets or sets a value that determines if command names are written to the history window.
- `EchoPromptsToHistoryWindow` (Boolean, get/set) — Gets or sets a value that determines if prompt messages are written to the history window.
- `EditCandidateColor` (Color, get/set) — Gets or sets the color of objects that are eligible to be edited.
- `FeedbackColor` (Color, get/set) — Gets or sets the feedback color.
- `FrameBackgroundColor` (Color, get/set) — Gets or sets the frame background color.
- `GridThickLineColor` (Color, get/set) — Gets or sets the color of the thick line in the grid.
- `GridThinLineColor` (Color, get/set) — Gets or sets the color of the thin line in the grid.
- `GridXAxisLineColor` (Color, get/set) — Gets or sets the color of X axis line in the grid.
- `GridYAxisLineColor` (Color, get/set) — Gets or sets the color of Y axis line in the grid.
- `GridZAxisLineColor` (Color, get/set) — Gets or sets the color of Z axis line in the grid.
- `LockedObjectColor` (Color, get/set) — Gets or sets the color used to draw locked objects.
- `PageviewPaperColor` (Color, get/set) — CRhinoPageView paper background. A rectangle is drawn into the background of page views to represent the printed area. The alpha portion of the color is used to draw the paper blended into the background
- `SelectedObjectColor` (Color, get/set) — The color used to draw selected objects. The default is yellow, but this can be customized by the user.
- `SelectionWindowCrossingFillColor` (Color, get/set) — Gets or sets the color used to fill a crossing selection window
- `SelectionWindowCrossingStrokeColor` (Color, get/set) — Gets or sets the color used to draw the stroke of a crossing selection window
- `SelectionWindowFillColor` (Color, get/set) — Gets or sets the color used to fill a selection window
- `SelectionWindowStrokeColor` (Color, get/set) — Gets or sets the color used to draw the stroke of a selection window
- `ShowCrosshairs` (Boolean, get/set) — Gets or sets a value that determines if cross hairs are visible.
- `ShowFullPathInTitleBar` (Boolean, get/set) — Gets or sets a value that determines if the full path of the document is shown in the Rhino title bar.
- `TrackingColor` (Color, get/set) — Gets or sets the tracking color.
- `ViewportBackgroundColor` (Color, get/set) — Gets or sets the viewport background color.
- `WorldCoordIconXAxisColor` (Color, get/set) — Gets or sets the color of the world X axis of the world coordinates icon, appearing usually bottom left in viewports.
- `WorldCoordIconYAxisColor` (Color, get/set) — Gets or sets the color of the world Y axis of the world coordinate icon, appearing usually bottom left in viewports.
- `WorldCoordIconZAxisColor` (Color, get/set) — Gets or sets the color of the world Z axis of the world coordinate icon, appearing usually bottom left in viewports.

## ClipboardState (enum)

Defines enumerated constant values for different behavior that is related to clipboard data.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_ClipboardState.htm)

### Values
- `KeepData` = `0` — Always keep clipboard data, regardless of size and never prompt the user.
- `DeleteData` = `1` — Always delete clipboard data, regardless of size and never prompt the user.
- `PromptWhenBig` = `2` — Prompt user when clipboard memory is large.

## CommandAliasList (class)

Contains static methods and properties to access command aliases.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_CommandAliasList.htm)

### Methods
#### `public static bool Add(string alias, string macro)`

Adds a new command alias to Rhino.

**Parameters:**
- `alias` (System.String) — [in] The name of the command alias.
- `macro` (System.String) — [in] The command macro to run when the alias is executed.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_CommandAliasList_Add.htm)

#### `public static void Clear()`

Removes all aliases from the list.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_CommandAliasList_Clear.htm)

#### `public static bool Delete(string alias)`

Deletes an existing command alias from Rhino.

**Parameters:**
- `alias` (System.String) — [in] The name of the command alias.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_CommandAliasList_Delete.htm)

#### `public static Dictionary<string, string> GetDefaults()`

Constructs a dictionary containing as keys the default names and as value the default macro. The returned dictionary contains a copy of the settings.

**Returns:** `Dictionary<String,String>` — A new dictionary with the default name/macro combinations.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_CommandAliasList_GetDefaults.htm)

#### `public static string GetMacro(string alias)`

Returns the macro of a command alias.

**Parameters:**
- `alias` (System.String) — [in] The name of the command alias.

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.ApplicationSettings.CommandAliasList.GetMacro(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_CommandAliasList_GetMacro.htm)

#### `public static string[] GetNames()`

Returns a list of command alias names.

**Returns:** `String[]` — An array of strings. This can be empty.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_CommandAliasList_GetNames.htm)

#### `public static bool IsAlias(string alias)`

Verifies that a command alias exists in Rhino.

**Parameters:**
- `alias` (System.String) — [in] The name of the command alias.

**Returns:** `Boolean` — true if the alias exists.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_CommandAliasList_IsAlias.htm)

#### `public static bool IsDefault()`

Computes a value indicating if the current alias list is the same as the default alias list.

**Returns:** `Boolean` — true if the current alias list is exactly equal to the default alias list; false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_CommandAliasList_IsDefault.htm)

#### `public static bool SetMacro(string alias, string macro)`

Modifies the macro of a command alias.

**Parameters:**
- `alias` (System.String) — [in] The name of the command alias.
- `macro` (System.String) — [in] The new command macro to run when the alias is executed.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_CommandAliasList_SetMacro.htm)

#### `public static Dictionary<string, string> ToDictionary()`

Constructs a new dictionary that contains: as keys all names and as values all macros. Modifications to this dictionary do not affect any Rhino command alias.

**Returns:** `Dictionary<String,String>` — The new dictionary.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_CommandAliasList_ToDictionary.htm)

### Properties
- `Count` (Int32, get) — Returns the number of command alias in Rhino.

## CommandPromptPosition (enum)

Defines enumerated constant values for default positions of the command prompt inside the frame of the full editor window.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_CommandPromptPosition.htm)

### Values
- `Top` = `0` — The command prompt is shown on top.
- `Bottom` = `1` — The command prompt is shown at the bottom.
- `Floating` = `2` — The command prompt is shown floating.
- `Hidden` = `3` — The command prompt is shown hidden.

## CursorMode (enum)

Defines enumerated constant values for particular OSnap cursor colors.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_CursorMode.htm)

### Values
- `None` = `0` — No OSnap cursor.
- `BlackOnWhite` = `1` — Black on white OSnap cursor.
- `WhiteOnBlack` = `2` — White on black OSnap cursor.

## CursorTooltipSettings (class)

Cursor tooltips place information at the cursor location. Note: Turning on cursor tooltips turns off object snap cursors.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_CursorTooltipSettings.htm)

### Methods
#### `public static CursorTooltipSettingsState GetCurrentState()`

Gets the current settings.

**Returns:** `CursorTooltipSettingsState` — A new cursor tooltip state with current settings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_CursorTooltipSettings_GetCurrentState.htm)

#### `public static CursorTooltipSettingsState GetDefaultState()`

Gets the cursor tooltip factory settings.

**Returns:** `CursorTooltipSettingsState` — A new cursor tooltip state with factory settings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_CursorTooltipSettings_GetDefaultState.htm)

### Properties
- `AutoSuppress` (Boolean, get/set) — Attempts to display only the most useful tooltip.
- `BackgroundColor` (Color, get/set) — Tooltip background color.
- `CommandPromptPane` (Boolean, get/set) — Displays the current command prompt.
- `DistancePane` (Boolean, get/set) — Displays the distance from the last picked point.
- `Offset` (Point, get/set) — The x and y distances in pixels from the cursor location to the tooltip.
- `OsnapPane` (Boolean, get/set) — Displays the current object snap selection.
- `PointPane` (Boolean, get/set) — Displays the current construction plane coordinates.
- `RelativePointPane` (Boolean, get/set) — Displays the relative construction plane coordinates and angle from the last picked point.
- `TextColor` (Color, get/set) — Tooltip text color.
- `TooltipsEnabled` (Boolean, get/set) — Turns on/off cursor tooltips.

## CursorTooltipSettingsState (class)

Represents a snapshot of CursorTooltipSettings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_CursorTooltipSettingsState.htm)

### Constructors
- `public CursorTooltipSettingsState()` — Initializes a new instance of the CursorTooltipSettingsState class

### Properties
- `AutoSuppress` (Boolean, get/set) — Attempts to display only the most useful tooltip.
- `BackgroundColor` (Color, get/set) — Tooltip background color.
- `CommandPromptPane` (Boolean, get/set) — Displays the current command prompt.
- `DistancePane` (Boolean, get/set) — Displays the distance from the last picked point.
- `Offset` (Point, get/set) — The x and y distances in pixels from the cursor location to the tooltip.
- `OsnapPane` (Boolean, get/set) — Displays the current object snap selection.
- `PointPane` (Boolean, get/set) — Displays the current construction plane coordinates.
- `RelativePointPane` (Boolean, get/set) — Displays the relative construction plane coordinates and angle from the last picked point.
- `TextColor` (Color, get/set) — Tooltip text color.
- `TooltipsEnabled` (Boolean, get/set) — Turns on/off cursor tooltips.

## CurvatureAnalysisSettings (class)

Contains static methods and properties to modify curvature analysis-related commands.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_CurvatureAnalysisSettings.htm)

### Methods
#### `public static bool CalculateCurvatureAutoRange(IEnumerable<Mesh> meshes, ref CurvatureAnalysisSettingsState settings)`



**Parameters:**
- `meshes` (System.Collections.Generic.IEnumerable<Mesh>) — [Missing <param name="meshes"/> documentation for "M:Rhino.ApplicationSettings.CurvatureAnalysisSettings.CalculateCurvatureAutoRange(System.Collections.Generic.IEnumerable{Rhino.Geometry.Mesh},Rhino.ApplicationSettings.CurvatureAnalysisSettingsState@)"]
- `settings` (Rhino.ApplicationSettings.CurvatureAnalysisSettingsState) — [Missing <param name="settings"/> documentation for "M:Rhino.ApplicationSettings.CurvatureAnalysisSettings.CalculateCurvatureAutoRange(System.Collections.Generic.IEnumerable{Rhino.Geometry.Mesh},Rhino.ApplicationSettings.CurvatureAnalysisSettingsState@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.ApplicationSettings.CurvatureAnalysisSettings.CalculateCurvatureAutoRange(System.Collections.Generic.IEnumerable{Rhino.Geometry.Mesh},Rhino.ApplicationSettings.CurvatureAnalysisSettingsState@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_CurvatureAnalysisSettings_CalculateCurvatureAutoRange.htm)

#### `public static CurvatureAnalysisSettingsState GetCurrentState()`

Gets the current settings of the application.

**Returns:** `CurvatureAnalysisSettingsState` — [Missing <returns> documentation for "M:Rhino.ApplicationSettings.CurvatureAnalysisSettings.GetCurrentState"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_CurvatureAnalysisSettings_GetCurrentState.htm)

#### `public static CurvatureAnalysisSettingsState GetDefaultState()`

Gets the factory settings of the application.

**Returns:** `CurvatureAnalysisSettingsState` — [Missing <returns> documentation for "M:Rhino.ApplicationSettings.CurvatureAnalysisSettings.GetDefaultState"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_CurvatureAnalysisSettings_GetDefaultState.htm)

#### `public static void RestoreDefaults()`

Commits the default settings as the current settings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_CurvatureAnalysisSettings_RestoreDefaults.htm)

#### `public static void UpdateFromState(CurvatureAnalysisSettingsState state)`

Sets all settings to a particular defined joined state.

**Parameters:**
- `state` (Rhino.ApplicationSettings.CurvatureAnalysisSettingsState) — The particular state.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_CurvatureAnalysisSettings_UpdateFromState.htm)

### Properties
- `GaussRange` (Interval, get/set) — Gets or sets the Gaussian curvature range.
- `MaxRadiusRange` (Interval, get/set) — Gets or sets the Maximum Radius curvature range.
- `MeanRange` (Interval, get/set) — Gets or sets the Mean curvature range.
- `MinRadiusRange` (Interval, get/set) — Gets or sets the Minimum Radius curvature range.
- `Style` (CurvatureAnalysisSettings.CurvatureStyle, get/set) — Gets or sets the curvature analysis style.

## CurvatureAnalysisSettings.CurvatureStyle (enum)

Curvature analysis styles

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_CurvatureAnalysisSettings_CurvatureStyle.htm)

### Values
- `Gaussian` = `0` — Gaussian curvature
- `Mean` = `1` — Mean curvature
- `MinRadius` = `2` — Minimum radius curvature
- `MaxRadius` = `3` — Maximum radius curvature

## CurvatureAnalysisSettingsState (class)

Represents a snapshot of CurvatureAnalysisSettings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_CurvatureAnalysisSettingsState.htm)

### Properties
- `GaussRange` (Interval, get/set) — Gets or sets the Gaussian curvature range.
- `MaxRadiusRange` (Interval, get/set) — Gets or sets the Maximum Radius curvature range.
- `MeanRange` (Interval, get/set) — Gets or sets the Mean curvature range.
- `MinRadiusRange` (Interval, get/set) — Gets or sets the Minimum Radius curvature range.
- `Style` (CurvatureAnalysisSettings.CurvatureStyle, get/set) — Gets or sets the curvature analysis style.

## DraftAngleAnalysisSettings (class)

[Missing <summary> documentation for "T:Rhino.ApplicationSettings.DraftAngleAnalysisSettings"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_DraftAngleAnalysisSettings.htm)

### Methods
#### `public static DraftAngleAnalysisSettingsState GetCurrentState()`

Gets the current settings of the application.

**Returns:** `DraftAngleAnalysisSettingsState` — [Missing <returns> documentation for "M:Rhino.ApplicationSettings.DraftAngleAnalysisSettings.GetCurrentState"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_DraftAngleAnalysisSettings_GetCurrentState.htm)

#### `public static DraftAngleAnalysisSettingsState GetDefaultState()`

Gets the factory settings of the application.

**Returns:** `DraftAngleAnalysisSettingsState` — [Missing <returns> documentation for "M:Rhino.ApplicationSettings.DraftAngleAnalysisSettings.GetDefaultState"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_DraftAngleAnalysisSettings_GetDefaultState.htm)

#### `public static void RestoreDefaults()`

Commits the default settings as the current settings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_DraftAngleAnalysisSettings_RestoreDefaults.htm)

#### `public static void UpdateFromState(DraftAngleAnalysisSettingsState state)`

Sets all settings to a particular defined joined state.

**Parameters:**
- `state` (Rhino.ApplicationSettings.DraftAngleAnalysisSettingsState) — The particular state.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_DraftAngleAnalysisSettings_UpdateFromState.htm)

### Properties
- `AngleRange` (Interval, get/set) — The angle range.
- `ShowIsoCurves` (Boolean, get/set) — Show isoparametric curves.
- `UpDirection` (Vector3d, get/set) — The up direction.

## DraftAngleAnalysisSettingsState (class)

Represents a snapshot of DraftAngleAnalysisSettings

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_DraftAngleAnalysisSettingsState.htm)

### Properties
- `AngleRange` (Interval, get/set) — The angle range.
- `ShowIsoCurves` (Boolean, get/set) — Show isoparametric curves.
- `UpDirection` (Vector3d, get/set) — The up direction.

## EdgeAnalysisSettings (class)

Contains static methods and properties to modify the visibility of edges in edge-related commands.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_EdgeAnalysisSettings.htm)

### Methods
#### `public static EdgeAnalysisSettingsState GetCurrentState()`

Gets the current settings of the application.

**Returns:** `EdgeAnalysisSettingsState` — [Missing <returns> documentation for "M:Rhino.ApplicationSettings.EdgeAnalysisSettings.GetCurrentState"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_EdgeAnalysisSettings_GetCurrentState.htm)

#### `public static EdgeAnalysisSettingsState GetDefaultState()`

Gets the factory settings of the application.

**Returns:** `EdgeAnalysisSettingsState` — [Missing <returns> documentation for "M:Rhino.ApplicationSettings.EdgeAnalysisSettings.GetDefaultState"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_EdgeAnalysisSettings_GetDefaultState.htm)

#### `public static void RestoreDefaults()`

Commits the default settings as the current settings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_EdgeAnalysisSettings_RestoreDefaults.htm)

#### `public static void UpdateFromState(EdgeAnalysisSettingsState state)`

Sets all settings to a particular defined joined state.

**Parameters:**
- `state` (Rhino.ApplicationSettings.EdgeAnalysisSettingsState) — The particular state.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_EdgeAnalysisSettings_UpdateFromState.htm)

### Properties
- `ShowEdgeColor` (Color, get/set) — Gets or sets a color used to enhance display edges in commands like _ShowEdges and _ShowNakedEdges.
- `ShowEdges` (Int32, get/set) — Gets or sets a value referring to the group of edges that are targeted. 0 = all.1 = naked.2 = non-manifold.

## EdgeAnalysisSettingsState (class)

Represents a snapshot of EdgeAnalysisSettings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_EdgeAnalysisSettingsState.htm)

### Properties
- `ShowEdgeColor` (Color, get/set) — Gets or sets a color used to enhance display edges in commands like _ShowEdges and _ShowNakedEdges.
- `ShowEdges` (Int32, get/set) — Gets or sets a value referring to the group of edges that are targeted. 0 = all.1 = naked.2 = non-manifold.

## FileSettings (class)

Contains static methods and properties relating Rhino files.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_FileSettings.htm)

### Methods
#### `public static int AddSearchPath(string folder, int index)`

Adds a new imagePath to Rhino's search imagePath list. See "Options Files settings" in the Rhino help file for more details.

**Parameters:**
- `folder` (System.String) — [in] The valid folder, or imagePath, to add.
- `index` (System.Int32) — [in] A zero-based position index in the search imagePath list to insert the string. If -1, the imagePath will be appended to the end of the list.

**Returns:** `Int32` — The index where the item was inserted if success. -1 on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_FileSettings_AddSearchPath.htm)

#### `public static string[] AutoSaveBeforeCommands()`

Input list of commands that force AutoSave prior to running.

**Returns:** `String[]` — [Missing <returns> documentation for "M:Rhino.ApplicationSettings.FileSettings.AutoSaveBeforeCommands"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_FileSettings_AutoSaveBeforeCommands.htm)

#### `public static bool DeleteSearchPath(string folder)`

Removes an existing imagePath from Rhino's search imagePath list. See "Options Files settings" in the Rhino help file for more details.

**Parameters:**
- `folder` (System.String) — [in] The valid folder, or imagePath, to remove.

**Returns:** `Boolean` — true or false indicating success or failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_FileSettings_DeleteSearchPath.htm)

#### `public static string FindFile(string fileName)`

Searches for a file using Rhino's search imagePath. Rhino will look for a file in the following locations: 1. The current document's folder. 2. Folder's specified in Options dialog, File tab. 3. Rhino's System folders.

**Parameters:**
- `fileName` (System.String) — short file name to search for.

**Returns:** `String` — full imagePath on success; null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_FileSettings_FindFile.htm)

#### `public static FileSettingsState GetCurrentState()`

Returns the current state.

**Returns:** `FileSettingsState` — A new instance containing the current state.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_FileSettings_GetCurrentState.htm)

#### `public static string GetDataFolder(bool currentUser)`

Gets the data folder for machine or current user.

**Parameters:**
- `currentUser` (System.Boolean) — true if the query relates to the current user.

**Returns:** `String` — A directory to user or machine data.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_FileSettings_GetDataFolder.htm)

#### `public static FileSettingsState GetDefaultState()`

Returns the default state.

**Returns:** `FileSettingsState` — A new instance containing the default state.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_FileSettings_GetDefaultState.htm)

#### `public static string[] GetSearchPaths()`

Returns all of the imagePath items in Rhino's search imagePath list. See "Options Files settings" in the Rhino help file for more details.

**Returns:** `String[]` — [Missing <returns> documentation for "M:Rhino.ApplicationSettings.FileSettings.GetSearchPaths"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_FileSettings_GetSearchPaths.htm)

#### `public static string[] RecentlyOpenedFiles()`

Returns a list of recently opened files. Note that this function does not check to make sure that these files still exist.

**Returns:** `String[]` — An array of strings with the paths to the recently opened files.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_FileSettings_RecentlyOpenedFiles.htm)

#### `public static void SetAutoSaveBeforeCommands(string[] commands)`

Set list of commands that force AutoSave prior to running.

**Parameters:**
- `commands` (System.String[]) — [Missing <param name="commands"/> documentation for "M:Rhino.ApplicationSettings.FileSettings.SetAutoSaveBeforeCommands(System.String[])"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_FileSettings_SetAutoSaveBeforeCommands.htm)

### Properties
- `AutoSaveEnabled` (Boolean, get/set) — Enables or disables Rhino's automatic file saving mechanism.
- `AutoSaveFile` (String, get/set) — the file name used by Rhino's automatic file saving mechanism.
- `AutoSaveInterval` (TimeSpan, get/set) — how often the document will be saved when Rhino's automatic file saving mechanism is enabled.
- `AutoSaveMeshes` (Boolean, get/set) — save render and display meshes in autosave file.
- `ClipboardCopyToPreviousRhinoVersion` (Boolean, get/set) — Gets or sets a value that decides if copies to the clipboard are performed in both the current and previous Rhino clipboard formats. This means you will double the size of what is saved in the clipboard but will be able to copy from the current to the previous version using the clipboard.
- `ClipboardOnExit` (ClipboardState, get/set) — Gets or sets a value that determines what to do with clipboard data on exit.
- `CreateBackupFiles` (Boolean, get/set) — Gets or sets a value that controls the creation of backup files.
- `DefaultRuiFile` (String, get) — Gets the path to the default RUI file.
- `ExecutableFolder` (String, get) — Returns the directory where the main Rhino executable is located.
- `FileLockingEnabled` (Boolean, get/set) — Ensure that only one person at a time can have a file open for saving.
- `FileLockingOpenWarning` (Boolean, get/set) — Gets or sets whether to display the information dialog which identifies computer files.
- `HelpFilePath` (String, get) — Gets the Rhino help file path.
- `InstallFolder` (DirectoryInfo, get) — Returns Rhino's installation folder.
- `LocalProfileDataFolder` (String, get) — Get full path to a Rhino specific sub-folder under the per-user Local (non-roaming) Profile folder. This is the folder where user-specific data is stored. On Windows 7, 8, usually someplace like: "C:\Users\[USERNAME]\AppData\Local\McNeel\Rhinoceros\[VERSION_NUMBER]\"
- `SaveViewChanges` (Boolean, get/set) — true for users who consider view changes a document change.
- `SearchPathCount` (Int32, get) — Gets the amount of search paths that are currently defined.
- `TemplateFile` (String, get/set) — Returns or sets the location of Rhino's template file.
- `TemplateFolder` (String, get/set) — Returns or sets the location of Rhino's template files.
- `WorkingFolder` (String, get/set) — Returns or sets Rhino's working directory, or folder. The working folder is the default folder for all file operations.

## FileSettingsState (class)

Represents a snapshot of FileSettings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_FileSettingsState.htm)

### Properties
- `AutoSaveEnabled` (Boolean, get/set) — Enables or disables Rhino's automatic file saving mechanism.
- `AutoSaveInterval` (TimeSpan, get/set) — How often the document will be saved when Rhino's automatic file saving mechanism is enabled.
- `AutoSaveMeshes` (Boolean, get/set) — Saves render and display meshes in autosave file.
- `ClipboardCopyToPreviousRhinoVersion` (Boolean, get/set) — Gets or sets a value that decides if copies to the clipboard are performed in both the current and previous Rhino clipboard formats. This means you will double the size of what is saved in the clipboard but will be able to copy from the current to the previous version using the clipboard.
- `ClipboardOnExit` (ClipboardState, get/set) — Gets or sets a value that determines what to do with clipboard data on exit.
- `CreateBackupFiles` (Boolean, get/set) — Gets or sets a value indicating whether to create backup files.
- `FileLockingEnabled` (Boolean, get/set) — Ensures that only one person at a time can have a file open for saving.
- `FileLockingOpenWarning` (Boolean, get/set) — Displays an information dialog which identifies computer file is open on.
- `SaveViewChanges` (Boolean, get/set) — true for users who consider view changes a document change.

## GeneralSettings (class)

Contains static methods and properties to give access to Rhinoceros settings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_GeneralSettings.htm)

### Methods
#### `public static GeneralSettingsState GetCurrentState()`

Gets the current settings.

**Returns:** `GeneralSettingsState` — A new general state with current settings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_GeneralSettings_GetCurrentState.htm)

#### `public static GeneralSettingsState GetDefaultState()`

Gets the factory settings.

**Returns:** `GeneralSettingsState` — A new general state with factory settings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_GeneralSettings_GetDefaultState.htm)

### Properties
- `AutoUpdateCommandHelp` (Boolean, get/set) — Command help dialog auto-update feature.
- `ContextMenuDelay` (TimeSpan, get/set) — Time to wait before permitting context menu display.
- `EnableContextMenu` (Boolean, get/set) — true if right mouse down + delay will pop up context menu on a mouse up if no move happens.
- `MaximumPopupMenuLines` (Int32, get/set) — Gets or sets the maximum number of popup menu lines.
- `MaximumUndoMemoryMb` (Int32, get/set) — Gets or sets the minimum undo memory Mb. Undo records will be purged if there are more than MinimumUndoSteps and they use more than MaximumUndoMemoryMb.
- `MiddleMouseMacro` (String, get/set) — Gets or sets the toolbar to popup when the middle mouse is clicked on a view, this value is only used when MiddleMouseMode is set to PopupToolbar.
- `MiddleMouseMode` (MiddleMouseMode, get/set) — Gets or sets what happens when the user clicks the middle mouse.
- `MiddleMousePopupToolbar` (String, get/set) — Gets or sets the toolbar to popup when the middle mouse is clicked on a view, this value is only used when MiddleMouseMode is set to PopupToolbar.
- `MinimumUndoSteps` (Int32, get/set) — Gets or sets the minimum undo steps. Undo records will be purged if there are more than MinimumUndoSteps and they use more than MaximumUndoMemoryMb.
- `MouseSelectMode` (MouseSelectMode, get/set) — Gets or sets the current selection mode.
- `NewObjectIsoparmCount` (Int32, get/set) — Gets or sets the number of isoparm curves to show on new objects.
- `UseExtrusions` (Boolean, get) — Should extrusion objects be created for things like cylinders

## GeneralSettingsState (class)

Represents a snapshot of GeneralSettings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_GeneralSettingsState.htm)

### Properties
- `AutoUpdateCommandHelp` (Boolean, get/set) — Gets or sets the command help dialog auto-update feature.
- `ContextMenuDelay` (TimeSpan, get/set) — Gets or sets the time to wait before permitting context menu display.
- `EnableContextMenu` (Boolean, get/set) — true if right mouse down + delay will pop up context menu on a mouse up if no move happens.
- `MaximumPopupMenuLines` (Int32, get/set) — Gets or sets the maximum number of popup menu lines.
- `MaximumUndoMemoryMb` (Int32, get/set) — Gets or sets the minimum undo memory Mb. Undo records will be purged if there are more than MinimumUndoSteps and they use more than MaximumUndoMemoryMb.
- `MiddleMouseMacro` (String, get/set) — Gets or sets the toolbar to popup when the middle mouse is clicked on a view, this value is only used when MiddleMouseMode is set to PopupToolbar.
- `MiddleMouseMode` (MiddleMouseMode, get/set) — Gets or sets what happens when the user clicks the middle mouse.
- `MiddleMousePopupToolbar` (String, get/set) — Gets or sets the toolbar to popup when the middle mouse is clicked on a view, this value is only used when MiddleMouseMode is set to PopupToolbar.
- `MinimumUndoSteps` (Int32, get/set) — Gets or sets the minimum undo steps. Undo records will be purged if there are more than MinimumUndoSteps and they use more than MaximumUndoMemoryMb.
- `MouseSelectMode` (MouseSelectMode, get/set) — Gets or sets the current selection mode.
- `NewObjectIsoparmCount` (Int32, get/set) — Gets or sets the number of isoparm curves to show on new objects.

## HistorySettings (class)

Provides static (Shared in Vb.Net) properties to modify Rhino History settings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_HistorySettings.htm)

### Properties
- `BrokenRecordWarningEnabled` (Boolean, get/set) — Displays a warning dialog when an action is taken that breaks the link between the output and input objects.
- `ObjectLockingEnabled` (Boolean, get/set) — When history object locking is enabled, objects with history on them act as if they were locked and the only way to modify these objects is to edit their inputs.
- `RecordingEnabled` (Boolean, get/set) — When history recording is enabled, new objects keep a record of how they were constructed so that they can be updated if an input object changes.
- `UpdateEnabled` (Boolean, get/set) — When history update is enabled, dependent objects are automatically updated when an antecedent is modified.

## Installation (enum)

The type of Rhino executable that is executing

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_Installation.htm)

### Values
- `Undefined` = `0` — Unknown
- `Commercial` = `1`
- `Educational` = `2`
- `EducationalLab` = `3`
- `NotForResale` = `4`
- `NotForResaleLab` = `5`
- `Beta` = `6`
- `BetaLab` = `7`
- `Evaluation` = `8` — 25 Save limit evaluation version of Rhino
- `Corporate` = `9`
- `EvaluationTimed` = `10` — 90 day time limit evaluation version of Rhino

## LicenseNode (enum)

License node types.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_LicenseNode.htm)

### Values
- `Standalone` = `0` — An independent node.
- `Network` = `1` — Network (obtains license from Zoo server)
- `NetworkCheckedOut` = `2` — Network (has license checked out from Zoo server)

## MiddleMouseMode (enum)

Defines enumerated constant values to define what happens when either the middle mouse button on a three-button mouse is clicked or after pressing the wheel on a wheeled mouse.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_MiddleMouseMode.htm)

### Values
- `PopupMenu` = `0` — Pops up two-part menu at the cursor location. You can list your favorite commands in the top section. The bottom section is the list of most recent commands used.
- `PopupToolbar` = `1` — Choose a toolbar to pop up at the cursor location. Create a toolbar containing your favorite commands or object snaps to use as a pop-up toolbar.
- `RunMacro` = `2` — Lists a series of commands that run when you click the middle mouse button.

## ModelAidSettings (class)

Contains static methods and properties to modify model aid settings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_ModelAidSettings.htm)

### Methods
#### `public static ModelAidSettingsState GetCurrentState()`

Gets the current settings.

**Returns:** `ModelAidSettingsState` — A new model aid state with current settings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_ModelAidSettings_GetCurrentState.htm)

#### `public static ModelAidSettingsState GetDefaultState()`

Gets the factory settings.

**Returns:** `ModelAidSettingsState` — A new model aid state with factory settings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_ModelAidSettings_GetDefaultState.htm)

#### `public static void UpdateFromState(ModelAidSettingsState state)`

Updates from a particular setting state.

**Parameters:**
- `state` (Rhino.ApplicationSettings.ModelAidSettingsState) — The new states that will be set.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_ModelAidSettings_UpdateFromState.htm)

### Properties
- `AltPlusArrow` (Boolean, get/set) — true means Alt+arrow is used for nudging.
- `AutoGumballEnabled` (Boolean, get/set) — When AutoGumball is on, a gumball automatically appears when objects are pre-picked.
- `ControlPolygonDisplayDensity` (Int32, get/set) — Gets or sets the control polygon display density.
- `CtrlNudgeKeyStep` (Double, get/set) — Gets or sets the Ctrl-key based nudge step amount.
- `DisplayControlPolygon` (Boolean, get/set) — Gets or sets the enabled state of Rhino's display control polygon.
- `ExtendToApparentIntersection` (Boolean, get/set) — Gets or sets the enabled state of Rhino's extend to apparent intersections.
- `ExtendTrimLines` (Boolean, get/set) — Gets or sets the enabled state of Rhino's extend trim lines.
- `GridSnap` (Boolean, get/set) — Gets or sets the enabled state of Rhino's grid snap modeling aid.
- `HighlightControlPolygon` (Boolean, get/set) — Gets or sets the enabled state of Rhino's highlight dialog modeling aid.
- `MousePickboxRadius` (Int32, get/set) — radius of mouse pick box in pixels.
- `NudgeKeyStep` (Double, get/set) — Gets or sets the nudge step amount.
- `NudgeMode` (Int32, get/set) — 0 = world, 1 = cplane, 2 = view, 3 = UVN, -1 = not set.
- `Ortho` (Boolean, get/set) — Gets or sets the enabled state of Rhino's ortho modeling aid.
- `OrthoAngle` (Double, get/set) — Gets or sets the base orthogonal angle.
- `Osnap` (Boolean, get/set) — Enables or disables Rhino's object snap modeling aid.
- `OsnapCursorMode` (CursorMode, get/set) — Gets or sets the OSnap cursor mode.
- `OsnapModes` (OsnapModes, get/set) — Returns or sets Rhino's current object snap mode. The mode is a bitwise value based on the OsnapModes enumeration.
- `OsnapPickboxRadius` (Int32, get/set) — Enables or disables Rhino's planar modeling aid.
- `Planar` (Boolean, get/set) — Gets or sets the enabled state of Rhino's Planar modeling aid.
- `PointDisplay` (PointDisplayMode, get/set) — Gets or sets the point display mode.
- `ProjectSnapToCPlane` (Boolean, get/set) — Gets or sets the enabled state of Rhino's Project modeling aid.
- `ShiftNudgeKeyStep` (Double, get/set) — Gets or sets the Shift-key based nudge step amount.
- `SnappyGumballEnabled` (Boolean, get/set) — When SnappyGumball is on, a dragging a gumball moves the center point. When snappy gumball is off, dragging a gumball moves the mouse down point.
- `SnapToLocked` (Boolean, get/set) — Gets or sets the locked state of the snap modeling aid.
- `UniversalConstructionPlaneMode` (Boolean, get/set) — Gets or sets the locked state of the snap modeling aid.
- `UseHorizontalDialog` (Boolean, get/set) — Gets or sets the enabled state of Rhino's use horizontal dialog modeling aid.

## ModelAidSettingsState (class)

Represents a snapshot of ModelAidSettings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_ModelAidSettingsState.htm)

### Properties
- `AltPlusArrow` (Boolean, get/set) — true mean Alt+arrow is used for nudging.
- `ControlPolygonDisplayDensity` (Int32, get/set) — Gets or sets the control polygon display density.
- `CtrlNudgeKeyStep` (Double, get/set) — Gets or sets the Ctrl-key based nudge step amount.
- `DisplayControlPolygon` (Boolean, get/set) — Gets or sets the enabled state of Rhino's display control polygon.
- `ExtendToApparentIntersection` (Boolean, get/set) — Gets or sets the enabled state of Rhino's extend to apparent intersections.
- `ExtendTrimLines` (Boolean, get/set) — Gets or sets the enabled state of Rhino's extend trim lines.
- `GridSnap` (Boolean, get/set) — Gets or sets the enabled state of Rhino's grid snap modeling aid.
- `HighlightControlPolygon` (Boolean, get/set) — Gets or sets the enabled state of Rhino's highlight dialog modeling aid.
- `MousePickboxRadius` (Int32, get/set) — Gets or sets the radius of the mouse pick box in pixels.
- `NudgeKeyStep` (Double, get/set) — Gets or sets the nudge step amount.
- `NudgeMode` (Int32, get/set) — 0 = world, 1 = cplane, 2 = view, 3 = UVN, -1 = not set.
- `Ortho` (Boolean, get/set) — Gets or sets the enabled state of Rhino's ortho modeling aid.
- `OrthoAngle` (Double, get/set) — Gets or sets the base orthogonal angle.
- `Osnap` (Boolean, get/set) — Gets or sets the enabled state of Rhino's object snap modeling aid.
- `OsnapCursorMode` (CursorMode, get/set) — Gets or sets the OSnap cursor mode.
- `OsnapModes` (OsnapModes, get/set) — Returns or sets Rhino's current object snap mode. The mode is a bitwise value based on the OsnapModes enumeration.
- `OsnapPickboxRadius` (Int32, get/set) — Enables or disables Rhino's planar modeling aid.
- `Planar` (Boolean, get/set) — Gets or sets the enabled state of Rhino's Planar modeling aid.
- `PointDisplay` (PointDisplayMode, get/set) — Gets or sets the point display mode.
- `ProjectSnapToCPlane` (Boolean, get/set) — Gets or sets the enabled state of Rhino's Project modeling aid.
- `ShiftNudgeKeyStep` (Double, get/set) — Gets or sets the Shift-key based nudge step amount.
- `SnapToLocked` (Boolean, get/set) — Gets or sets the locked state of the snap modeling aid.
- `UniversalConstructionPlaneMode` (Boolean, get/set) — Gets or sets the locked state of the snap modeling aid.
- `UseHorizontalDialog` (Boolean, get/set) — Gets or sets the enabled state of Rhino's use horizontal dialog modeling aid.

## MouseSelectMode (enum)

Defines enumerated constant values to indicate a particular window selection mode.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_MouseSelectMode.htm)

### Values
- `Crossing` = `0` — Anything that crosses this window will be selected.
- `Window` = `1` — Anything that is inside this window will be selected.
- `Combo` = `2` — Drag a rectangle from left to right for window select. Drag a rectangle from right to left for crossing select.

## NeverRepeatList (class)

Contains static methods and properties relating to the list of commands that are never repeated.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_NeverRepeatList.htm)

### Methods
#### `public static string[] CommandNames()`

The list of commands to not repeat.

**Returns:** `String[]` — [Missing <returns> documentation for "M:Rhino.ApplicationSettings.NeverRepeatList.CommandNames"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_NeverRepeatList_CommandNames.htm)

#### `public static int SetList(string[] commandNames)`

Puts the command name tokens in m_dont_repeat_list.

**Parameters:**
- `commandNames` (System.String[]) — [Missing <param name="commandNames"/> documentation for "M:Rhino.ApplicationSettings.NeverRepeatList.SetList(System.String[])"]

**Returns:** `Int32` — Number of items added to m_dont_repeat_list.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_NeverRepeatList_SetList.htm)

### Properties
- `UseNeverRepeatList` (Boolean, get) — Only use the list if somebody modifies it via CRhinoAppSettings::SetDontRepeatCommands(). Return value of true means CRhinoCommand don't repeat flags will be ignored and the m_dont_repeat_list will be used instead. false means the individual CRhinoCommands will determine if they are repeatable.

## OpenGLSettings (class)

Static methods and properties to control OpenGL settings

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_OpenGLSettings.htm)

### Constructors
- `public OpenGLSettings()` — Initializes a new instance of the OpenGLSettings class

### Methods
#### `public static OpenGLSettingsState GetCurrentState()`

Gets the current settings.

**Returns:** `OpenGLSettingsState` — A new OpenGL state with current settings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_OpenGLSettings_GetCurrentState.htm)

#### `public static OpenGLSettingsState GetDefaultState()`

Gets the OpenGL factory settings.

**Returns:** `OpenGLSettingsState` — A new OpenGL state with factory settings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_OpenGLSettings_GetDefaultState.htm)

#### `public static void RestoreDefaults()`

Updates from the default setting state.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_OpenGLSettings_RestoreDefaults.htm)

#### `public static void UpdateFromState(OpenGLSettingsState state)`

Updates from a particular setting state.

**Parameters:**
- `state` (Rhino.ApplicationSettings.OpenGLSettingsState) — The new state that will be set.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_OpenGLSettings_UpdateFromState.htm)

### Properties
- `AntialiasLevel` (AntialiasLevel, get/set) — Gets or sets the anti-alias level used by OpenGL viewports

## OpenGLSettingsState (class)

Represents a snapshot of OpenGLSettings

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_OpenGLSettingsState.htm)

### Properties
- `AntialiasLevel` (AntialiasLevel, get/set) — AA level used in OpenGL viewports

## OsnapModes (enum)

Defines several bit masks for each of the OSnap that are defined. Refer to the Rhino Help file for further information.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_OsnapModes.htm)

### Values
- `None` = `0` — No OSnap.
- `Near` = `2` — Near OSnap.
- `Focus` = `8` — Focus OSnap.
- `Center` = `32` — Center OSnap.
- `Vertex` = `64` — Vertex OSnap.
- `Knot` = `128` — Knot OSnap.
- `Quadrant` = `512` — Quadrant OSnap.
- `Midpoint` = `2048` — Midpoint OSnap.
- `Intersection` = `8192` — Intersection OSnap.
- `End` = `131072` — End OSnap.
- `Perpendicular` = `524288` — Perpendicular OSnap.
- `Tangent` = `2097152` — Tangent OSnap.
- `Point` = `134217728` — Point OSnap.

## PackageManagerSettings (class)

Settings specific to Rhino's package manager

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_PackageManagerSettings.htm)

### Properties
- `Sources` (String, get/set) — semicolon separated list of paths/urls that the package manager uses for sources

## PaintColor (enum)

Contains enumerated constant values to represent logical colors associated with elements of the user interface.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_PaintColor.htm)

### Values
- `NormalStart` = `0` — Gradient start for active toolbar tab and non-client area of Rhino.
- `NormalEnd` = `1` — Gradient end for active toolbar tab and non-client area of Rhino.
- `NormalBorder` = `2` — Edge color used for grippers, toolbar border, resize bars, status bar pane borders.
- `HotStart` = `3` — Gradient start for inactive toolbar tab.
- `HotEnd` = `4` — Gradient end for inactive toolbar tab.
- `HotBorder` = `5` — Inactive toolbar tab border.
- `PressedStart` = `6` — Pressed gradient start.
- `PressedEnd` = `7` — Pressed gradient end.
- `PressedBorder` = `8` — Pressed border.
- `TextEnabled` = `9` — Toolbar tab text and status bar text.
- `TextDisabled` = `10` — Disabled text color.
- `MouseOverControlStart` = `11` — Color for hovering gradient start.
- `MouseOverControlEnd` = `12` — Color for hovering gradient end.
- `MouseOverControlBorder` = `13` — Color for hovering border.
- `ActiveCaption` = `14` — Active floating window non-client area
- `InactiveCaption` = `15` — Inactive floating window non-client area
- `PanelBackground` = `16` — Background color of panels
- `ActiveViewportTitle` = `17` — Active viewport title.
- `InactiveViewportTitle` = `18` — Inactive viewport title.
- `ModifiedValueControlColor` = `19` — Modified property value label text color
- `EditBoxBackground` = `20` — Background fill for input elements (edit box, checkbox)
- `GridLinesOnPanelBackground` = `21` — Grid lines on panel background color
- `GridLinesOnEditBoxBackground` = `22` — Grid lines on edit box background color
- `InactiveTabBackground` = `23` — Fill for inactive tabs

## PointDisplayMode (enum)

Defines enumerated constant values for world coordinates and CPlane point display modes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_PointDisplayMode.htm)

### Values
- `WorldPoint` = `0` — Points are displayed in world coordinates.
- `CplanePoint` = `1` — Points are displayed in CPlane coordinates.

## SelectionFilterSettings (class)

Selection filter settings restrict any selection mode (SelWindow, SelCrossing, SelAll, etc.) to specified object types. Note, selection filter settings are not persistent.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_SelectionFilterSettings.htm)

### Methods
#### `public static SelectionFilterSettingsState GetCurrentState()`

Gets the current settings of the application.

**Returns:** `SelectionFilterSettingsState` — [Missing <returns> documentation for "M:Rhino.ApplicationSettings.SelectionFilterSettings.GetCurrentState"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_SelectionFilterSettings_GetCurrentState.htm)

#### `public static SelectionFilterSettingsState GetDefaultState()`

Gets the factory settings of the application.

**Returns:** `SelectionFilterSettingsState` — [Missing <returns> documentation for "M:Rhino.ApplicationSettings.SelectionFilterSettings.GetDefaultState"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_SelectionFilterSettings_GetDefaultState.htm)

#### `public static void RestoreDefaults()`

Commits the default settings as the current settings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_SelectionFilterSettings_RestoreDefaults.htm)

#### `public static void UpdateFromState(SelectionFilterSettingsState state)`

Sets all settings to a particular defined joined state.

**Parameters:**
- `state` (Rhino.ApplicationSettings.SelectionFilterSettingsState) — The particular state.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_SelectionFilterSettings_UpdateFromState.htm)

### Properties
- `Enabled` (Boolean, get/set) — Enables or disables the global object selection filter.
- `GlobalGeometryFilter` (ObjectType, get/set) — The global geometry type filter controls which types of geometry will be filtered. Note, the filter can be a bitwise combination of multiple object types.
- `OneShotGeometryFilter` (ObjectType, get/set) — The one-shot geometry type filter controls which types of geometry will be filtered for one selection. Note, the filter can be a bitwise combination of multiple object types.
- `SubObjectSelect` (Boolean, get/set) — Enables or disabled sub-object selection.

## SelectionFilterSettingsState (class)

Represents a snapshot of SelectionFilterSettings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_SelectionFilterSettingsState.htm)

### Properties
- `Enabled` (Boolean, get/set) — Enables or disables the global object selection filter.
- `GlobalGeometryFilter` (ObjectType, get/set) — The global geometry type filter controls which types of geometry will be filtered. Note, the filter can be a bitwise combination of multiple object types.
- `OneShotGeometryFilter` (ObjectType, get/set) — The one-shot geometry type filter controls which types of geometry will be filtered for one selection. Note, the filter can be a bitwise combination of multiple object types.
- `SubObjectSelect` (Boolean, get/set) — Enables or disabled sub-object selection.

## ShortcutKey (enum)

Shortcut key combinations

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_ShortcutKey.htm)

### Values
- `F1` = `0` — F1
- `F2` = `1` — F2
- `F3` = `2` — F3
- `F4` = `3` — F4
- `F5` = `4` — F5
- `F6` = `5` — F6
- `F7` = `6` — F7
- `F8` = `7` — F8
- `F9` = `8` — F9
- `F10` = `9` — F10
- `F11` = `10` — F11
- `F12` = `11` — F12
- `CtrlF1` = `12` — Ctrl + F1
- `CtrlF2` = `13` — Ctrl + F2
- `CtrlF3` = `14` — Ctrl + F3
- `CtrlF4` = `15` — Ctrl + F4
- `CtrlF5` = `16` — Ctrl + F5
- `CtrlF6` = `17` — Ctrl + F6
- `CtrlF7` = `18` — Ctrl + F7
- `CtrlF8` = `19` — Ctrl + F8
- `CtrlF9` = `20` — Ctrl + F9
- `CtrlF10` = `21` — Ctrl + F10
- `CtrlF11` = `22` — Ctrl + F11
- `CtrlF12` = `23` — Ctrl + F12
- `ShiftCtrlF1` = `24` — Shift + Ctrl + F1
- `ShiftCtrlF2` = `25` — Shift + Ctrl + F2
- `ShiftCtrlF3` = `26` — Shift + Ctrl + F3
- `ShiftCtrlF4` = `27` — Shift + Ctrl + F4
- `ShiftCtrlF5` = `28` — Shift + Ctrl + F5
- `ShiftCtrlF6` = `29` — Shift + Ctrl + F6
- `ShiftCtrlF7` = `30` — Shift + Ctrl + F7
- `ShiftCtrlF8` = `31` — Shift + Ctrl + F8
- `ShiftCtrlF9` = `32` — Shift + Ctrl + F9
- `ShiftCtrlF10` = `33` — Shift + Ctrl + F10
- `ShiftCtrlF11` = `34` — Shift + Ctrl + F11
- `ShiftCtrlF12` = `35` — Shift + Ctrl + F12
- `AltCtrlF1` = `36` — Alt + Ctrl + F1
- `AltCtrlF2` = `37` — Alt + Ctrl + F2
- `AltCtrlF3` = `38` — Alt + Ctrl + F3
- `AltCtrlF4` = `39` — Alt + Ctrl + F4
- `AltCtrlF5` = `40` — Alt + Ctrl + F5
- `AltCtrlF6` = `41` — Alt + Ctrl + F6
- `AltCtrlF7` = `42` — Alt + Ctrl + F7
- `AltCtrlF8` = `43` — Alt + Ctrl + F8
- `AltCtrlF9` = `44` — Alt + Ctrl + F9
- `AltCtrlF10` = `45` — Alt + Ctrl + F10
- `AltCtrlF11` = `46` — Alt + Ctrl + F11
- `AltCtrlF12` = `47` — Alt + Ctrl + F12
- `CtrlA` = `48` — Ctrl + A
- `CtrlB` = `49` — Ctrl + B
- `CtrlC` = `50` — Ctrl + C
- `CtrlD` = `51` — Ctrl + D
- `CtrlE` = `52` — Ctrl + E
- `CtrlF` = `53` — Ctrl + F
- `CtrlG` = `54` — Ctrl + G
- `CtrlH` = `55` — Ctrl + H
- `CtrlI` = `56` — Ctrl + I
- `CtrlJ` = `57` — Ctrl + J
- `CtrlK` = `58` — Ctrl + K
- `CtrlL` = `59` — Ctrl + L
- `CtrlM` = `60` — Ctrl + M
- `CtrlN` = `61` — Ctrl + N
- `CtrlO` = `62` — Ctrl + O
- `CtrlP` = `63` — Ctrl + P
- `CtrlQ` = `64` — Ctrl + Q
- `CtrlR` = `65` — Ctrl + R
- `CtrlS` = `66` — Ctrl + S
- `CtrlT` = `67` — Ctrl + T
- `CtrlU` = `68` — Ctrl + U
- `CtrlV` = `69` — Ctrl + V
- `CtrlW` = `70` — Ctrl + W
- `CtrlX` = `71` — Ctrl + X
- `CtrlY` = `72` — Ctrl + Y
- `CtrlZ` = `73` — Ctrl + Z
- `ShiftCtrlA` = `74` — Shift + Ctrl + A
- `ShiftCtrlB` = `75` — Shift + Ctrl + B
- `ShiftCtrlC` = `76` — Shift + Ctrl + C
- `ShiftCtrlD` = `77` — Shift + Ctrl + D
- `ShiftCtrlE` = `78` — Shift + Ctrl + E
- `ShiftCtrlF` = `79` — Shift + Ctrl + F
- `ShiftCtrlG` = `80` — Shift + Ctrl + G
- `ShiftCtrlH` = `81` — Shift + Ctrl + H
- `ShiftCtrlI` = `82` — Shift + Ctrl + I
- `ShiftCtrlJ` = `83` — Shift + Ctrl + J
- `ShiftCtrlK` = `84` — Shift + Ctrl + K
- `ShiftCtrlL` = `85` — Shift + Ctrl + L
- `ShiftCtrlM` = `86` — Shift + Ctrl + M
- `ShiftCtrlN` = `87` — Shift + Ctrl + N
- `ShiftCtrlO` = `88` — Shift + Ctrl + O
- `ShiftCtrlP` = `89` — Shift + Ctrl + P
- `ShiftCtrlQ` = `90` — Shift + Ctrl + Q
- `ShiftCtrlR` = `91` — Shift + Ctrl + R
- `ShiftCtrlS` = `92` — Shift + Ctrl + S
- `ShiftCtrlT` = `93` — Shift + Ctrl + T
- `ShiftCtrlU` = `94` — Shift + Ctrl + U
- `ShiftCtrlV` = `95` — Shift + Ctrl + V
- `ShiftCtrlW` = `96` — Shift + Ctrl + W
- `ShiftCtrlX` = `97` — Shift + Ctrl + X
- `ShiftCtrlY` = `98` — Shift + Ctrl + Y
- `ShiftCtrlZ` = `99` — Shift + Ctrl + Z
- `AltCtrlA` = `100` — Alt + Ctrl + A
- `AltCtrlB` = `101` — Alt + Ctrl + B
- `AltCtrlC` = `102` — Alt + Ctrl + C
- `AltCtrlD` = `103` — Alt + Ctrl + D
- `AltCtrlE` = `104` — Alt + Ctrl + E
- `AltCtrlF` = `105` — Alt + Ctrl + F
- `AltCtrlG` = `106` — Alt + Ctrl + G
- `AltCtrlH` = `107` — Alt + Ctrl + H
- `AltCtrlI` = `108` — Alt + Ctrl + I
- `AltCtrlJ` = `109` — Alt + Ctrl + J
- `AltCtrlK` = `110` — Alt + Ctrl + K
- `AltCtrlL` = `111` — Alt + Ctrl + L
- `AltCtrlM` = `112` — Alt + Ctrl + M
- `AltCtrlN` = `113` — Alt + Ctrl + N
- `AltCtrlO` = `114` — Alt + Ctrl + O
- `AltCtrlP` = `115` — Alt + Ctrl + P
- `AltCtrlQ` = `116` — Alt + Ctrl + Q
- `AltCtrlR` = `117` — Alt + Ctrl + R
- `AltCtrlS` = `118` — Alt + Ctrl + S
- `AltCtrlT` = `119` — Alt + Ctrl + T
- `AltCtrlU` = `120` — Alt + Ctrl + U
- `AltCtrlV` = `121` — Alt + Ctrl + V
- `AltCtrlW` = `122` — Alt + Ctrl + W
- `AltCtrlX` = `123` — Alt + Ctrl + X
- `AltCtrlY` = `124` — Alt + Ctrl + Y
- `AltCtrlZ` = `125` — Alt + Ctrl + Z
- `Ctrl0` = `126` — Ctrl + 0
- `Ctrl1` = `127` — Ctrl + 1
- `Ctrl2` = `128` — Ctrl + 2
- `Ctrl3` = `129` — Ctrl + 3
- `Ctrl4` = `130` — Ctrl + 4
- `Ctrl5` = `131` — Ctrl + 5
- `Ctrl6` = `132` — Ctrl + 6
- `Ctrl7` = `133` — Ctrl + 7
- `Ctrl8` = `134` — Ctrl + 8
- `Ctrl9` = `135` — Ctrl + 9
- `ShiftCtrl0` = `136` — Shift + Ctrl + 0
- `ShiftCtrl1` = `137` — Shift + Ctrl + 1
- `ShiftCtrl2` = `138` — Shift + Ctrl + 2
- `ShiftCtrl3` = `139` — Shift + Ctrl + 3
- `ShiftCtrl4` = `140` — Shift + Ctrl + 4
- `ShiftCtrl5` = `141` — Shift + Ctrl + 5
- `ShiftCtrl6` = `142` — Shift + Ctrl + 6
- `ShiftCtrl7` = `143` — Shift + Ctrl + 7
- `ShiftCtrl8` = `144` — Shift + Ctrl + 8
- `ShiftCtrl9` = `145` — Shift + Ctrl + 9
- `AltCtrl0` = `146` — Alt + Ctrl + 0
- `AltCtrl1` = `147` — Alt + Ctrl + 1
- `AltCtrl2` = `148` — Alt + Ctrl + 2
- `AltCtrl3` = `149` — Alt + Ctrl + 3
- `AltCtrl4` = `150` — Alt + Ctrl + 4
- `AltCtrl5` = `151` — Alt + Ctrl + 5
- `AltCtrl6` = `152` — Alt + Ctrl + 6
- `AltCtrl7` = `153` — Alt + Ctrl + 7
- `AltCtrl8` = `154` — Alt + Ctrl + 8
- `AltCtrl9` = `155` — Alt + Ctrl + 9
- `Home` = `156` — Home
- `End` = `157` — End
- `CtrlHome` = `158` — Ctrl + Home
- `CtrlEnd` = `159` — Ctrl + End
- `ShiftHome` = `160` — Shift + Home
- `ShiftEnd` = `161` — Shift + End
- `ShiftCtrlHome` = `162` — Shift + Ctrl + Home
- `ShiftCtrlEnd` = `163` — Shift + Ctrl + End
- `AltCtrlHome` = `164` — Alt + Ctrl + Home
- `AltCtrlEnd` = `165` — Alt + Ctrl + End
- `PageUp` = `166` — Page Up
- `PageDown` = `167` — Page Down
- `ShiftPageUp` = `168` — Shift + Page Up
- `ShiftPageDown` = `169` — Shift + Page Down
- `CtrlPageUp` = `170` — Ctrl + Page Up
- `CtrlPageDown` = `171` — Ctrl + Page Down
- `ShiftCtrlPageUp` = `172` — Shift + Ctrl + Page Up
- `ShiftCtrlPageDown` = `173` — Shift + Ctrl + Page Down
- `AltCtrlPageUp` = `174` — Alt + Ctrl + Page Up
- `AltCtrlPageDown` = `175` — Alt + Ctrl + Page Down

## ShortcutKeySettings (class)

Contains static methods and properties to control keyboard shortcut keys

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_ShortcutKeySettings.htm)

### Methods
#### `public static string GetMacro(ShortcutKey key)`

Get macro associated with a given keyboard shortcut

**Parameters:**
- `key` (Rhino.ApplicationSettings.ShortcutKey) — [Missing <param name="key"/> documentation for "M:Rhino.ApplicationSettings.ShortcutKeySettings.GetMacro(Rhino.ApplicationSettings.ShortcutKey)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.ApplicationSettings.ShortcutKeySettings.GetMacro(Rhino.ApplicationSettings.ShortcutKey)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_ShortcutKeySettings_GetMacro.htm)

#### `public static void SetMacro(ShortcutKey key, string macro)`

Set macro associated with a keyboard shortcut

**Parameters:**
- `key` (Rhino.ApplicationSettings.ShortcutKey) — [Missing <param name="key"/> documentation for "M:Rhino.ApplicationSettings.ShortcutKeySettings.SetMacro(Rhino.ApplicationSettings.ShortcutKey,System.String)"]
- `macro` (System.String) — [Missing <param name="macro"/> documentation for "M:Rhino.ApplicationSettings.ShortcutKeySettings.SetMacro(Rhino.ApplicationSettings.ShortcutKey,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_ShortcutKeySettings_SetMacro.htm)

## SmartTrackSettings (class)

Contains static methods and properties that target the Smart Track feature behavior.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_SmartTrackSettings.htm)

### Methods
#### `public static SmartTrackSettingsState GetCurrentState()`

Gets the current settings.

**Returns:** `SmartTrackSettingsState` — A new Smart Track state with current settings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_SmartTrackSettings_GetCurrentState.htm)

#### `public static SmartTrackSettingsState GetDefaultState()`

Gets the Smart Track factory settings.

**Returns:** `SmartTrackSettingsState` — A new Smart Track state with factory settings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_SmartTrackSettings_GetDefaultState.htm)

#### `public static void UpdateFromState(SmartTrackSettingsState state)`

Updates from a particular setting state.

**Parameters:**
- `state` (Rhino.ApplicationSettings.SmartTrackSettingsState) — The new state that will be set.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_SmartTrackSettings_UpdateFromState.htm)

### Properties
- `ActivationDelayMilliseconds` (Int32, get/set) — Gets or sets the activation delay in milliseconds.
- `ActivePointColor` (Color, get/set) — Gets or sets the active point color.
- `LineColor` (Color, get/set) — Gets or sets the smart track line color.
- `MaxSmartPoints` (Int32, get/set) — Gets or sets the maximum number of smart points.
- `PointColor` (Color, get/set) — Gets or sets the point color.
- `SmartOrtho` (Boolean, get/set) — Gets or sets a value indicating if the 'Smart Ortho' feature is active. Orthogonal lines are then drawn automatically.
- `SmartTangents` (Boolean, get/set) — Gets or sets a value indicating if the 'Smart Tangents' feature is active.
- `TanPerpLineColor` (Color, get/set) — Gets or sets the tangent and perpendicular line color.
- `UseDottedLines` (Boolean, get/set) — Gets or sets a value indicating if lines are drawn dotted.
- `UseSmartTrack` (Boolean, get/set) — Gets or sets if the Smart Track feature is active.

## SmartTrackSettingsState (class)

Represents a snapshot of SmartTrackSettings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_SmartTrackSettingsState.htm)

### Properties
- `ActivationDelayMilliseconds` (Int32, get/set) — Gets or sets the activation delay in milliseconds.
- `ActivePointColor` (Color, get/set) — Gets or sets the active point color.
- `LineColor` (Color, get/set) — Gets or sets the smart track line color.
- `MaxSmartPoints` (Int32, get/set) — Gets or sets the maximum number of smart points.
- `PointColor` (Color, get/set) — Gets or sets the point color.
- `SmartOrtho` (Boolean, get/set) — Gets or sets a value indicating if the 'Smart Ortho' feature is active.
- `SmartTangents` (Boolean, get/set) — Gets or sets a value indicating if the 'Smart Tangents' feature is active.
- `TanPerpLineColor` (Color, get/set) — Gets or sets the tangent and perpendicular line color.
- `UseDottedLines` (Boolean, get/set) — Gets or sets a value indicating if lines are drawn dotted.
- `UseSmartTrack` (Boolean, get/set) — Gets or sets if the 'smart track' feature is active.

## ViewSettings (class)

Contains static methods and properties to control view settings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_ViewSettings.htm)

### Methods
#### `public static ViewSettingsState GetCurrentState()`

Gets the current settings.

**Returns:** `ViewSettingsState` — A new view state with current settings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_ViewSettings_GetCurrentState.htm)

#### `public static ViewSettingsState GetDefaultState()`

Gets the view factory settings.

**Returns:** `ViewSettingsState` — A new view state with factory settings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_ViewSettings_GetDefaultState.htm)

#### `public static void RestoreDefaults()`

Updates from the default setting state.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_ViewSettings_RestoreDefaults.htm)

#### `public static void UpdateFromState(ViewSettingsState state)`

Updates from a particular setting state.

**Parameters:**
- `state` (Rhino.ApplicationSettings.ViewSettingsState) — The new state that will be set.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_ViewSettings_UpdateFromState.htm)

### Properties
- `AlwaysPanParallelViews` (Boolean, get/set) — Gets or sets the 'always pan parallel views' value. If the view is not looking straight at the construction plane, then sets parallel viewports so they will not rotate.
- `DefinedViewSetCPlane` (Boolean, get/set) — Gets or sets the 'named views set CPlane' value. When true, restoring a named view causes the construction plane saved with that view to also restore.
- `DefinedViewSetProjection` (Boolean, get/set) — Gets or sets the 'named views set projection' value. When true, restoring a named view causes the viewport projection saved with the view to also restore.
- `LinkedViewports` (Boolean, get/set) — Gets or sets the 'linked views' activated setting. true enables real-time view synchronization. When a standard view is manipulated, the camera lens length of all parallel projection viewports are set to match the current viewport.
- `PanReverseKeyboardAction` (Boolean, get/set) — Gets or sets if panning with the keyboard is reversed. false, then Rhino pans the camera in the direction of the arrow key you press. true, then Rhino pans the scene instead.
- `PanScreenFraction` (Double, get/set) — Gets or sets the faction used as multiplier to pan the screen.
- `RotateCircleIncrement` (Int32, get/set) — Gets or sets the rotation increment. When the user rotates a view with the keyboard, Rhino rotates the view in steps. The usual step is 1/60th of a circle, which equals six degrees.
- `RotateReverseKeyboard` (Boolean, get/set) — Gets or sets the rotation direction. If true, then Rhino rotates the camera around the scene, otherwise, rotates the scene itself.
- `RotateToView` (Boolean, get/set) — Gets or sets the rotation reference. If true, then the views rotates relative to the view axes; false, than relative to the world x, y, and z axes.
- `SingleClickMaximize` (Boolean, get/set) — Gets or sets the 'single-click maximize' value. When true, maximizing a viewport needs a single click on the viewport title rather than a double-click.
- `ZoomExtentsParallelViewBorder` (Double, get/set) — Border amount to apply to parallel viewport during zoom extents
- `ZoomExtentsPerspectiveViewBorder` (Double, get/set) — Border amount to apply to perspective viewport during zoom extents
- `ZoomScale` (Double, get/set) — Gets or sets the step size for zooming with a wheeled mouse or the Page Up and Page Down keys.

## ViewSettingsState (class)

Represents a snapshot of ViewSettings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_ViewSettingsState.htm)

### Properties
- `AlwaysPanParallelViews` (Boolean, get/set) — Gets or sets the 'always pan parallel views' value. If the view is not looking straight at the construction plane, then sets parallel viewports so they will not rotate.
- `DefinedViewSetCPlane` (Boolean, get/set) — Gets or sets the 'named views set CPlane' value. When true, restoring a named view causes the construction plane saved with that view to also restore.
- `DefinedViewSetProjection` (Boolean, get/set) — Gets or sets the 'named views set projection' value. When true, restoring a named view causes the viewport projection saved with the view to also restore.
- `LinkedViewports` (Boolean, get/set) — Gets or sets the 'linked views' activated setting. true enables real-time view synchronization. When a standard view is manipulated, the camera lens length of all parallel projection viewports are set to match the current viewport.
- `PanReverseKeyboardAction` (Boolean, get/set) — Gets or sets if panning with the keyboard is reversed. false, then Rhino pans the camera in the direction of the arrow key you press. true, then Rhino pans the scene instead.
- `PanScreenFraction` (Double, get/set) — Gets or sets the faction used as multiplier to pan the screen.
- `RotateCircleIncrement` (Int32, get/set) — Gets or sets the rotation increment. When the user rotates a view with the keyboard, Rhino rotates the view in steps. The usual step is 1/60th of a circle, which equals six degrees.
- `RotateReverseKeyboard` (Boolean, get/set) — Gets or sets the rotation direction. If true, then Rhino rotates the camera around the scene, otherwise, rotates the scene itself.
- `RotateToView` (Boolean, get/set) — Gets or sets the rotation reference. If true, then the views rotates relative to the view axes; false, than relative to the world x, y, and z axes.
- `SingleClickMaximize` (Boolean, get/set) — Gets or sets the 'single-click maximize' value. When true, maximizing a viewport needs a single click on the viewport title rather than a double-click.
- `ZoomExtentsParallelViewBorder` (Double, get/set) — Border amount to apply to parallel viewport during zoom extents
- `ZoomExtentsPerspectiveViewBorder` (Double, get/set) — Border amount to apply to perspective viewport during zoom extents
- `ZoomScale` (Double, get/set) — Gets or sets the step size for zooming with a wheeled mouse or the Page Up and Page Down keys.

## WidgetColor (enum)

Contains enumerated constant values to represent logical colors associated with elements of the user interface.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_WidgetColor.htm)

### Values
- `UAxisColor` = `0` — The color of the mapping widget u-axis.
- `VAxisColor` = `1` — The color of the mapping widget v-axis..
- `WAxisColor` = `2` — The color of the mapping widget w-axis.

## ZebraAnalysisSettings (class)

Contains static methods and properties to modify Zebra analysis-related commands.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_ZebraAnalysisSettings.htm)

### Methods
#### `public static ZebraAnalysisSettingsState GetCurrentState()`

Gets the current settings of the application.

**Returns:** `ZebraAnalysisSettingsState` — [Missing <returns> documentation for "M:Rhino.ApplicationSettings.ZebraAnalysisSettings.GetCurrentState"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_ZebraAnalysisSettings_GetCurrentState.htm)

#### `public static ZebraAnalysisSettingsState GetDefaultState()`

Gets the factory settings of the application.

**Returns:** `ZebraAnalysisSettingsState` — [Missing <returns> documentation for "M:Rhino.ApplicationSettings.ZebraAnalysisSettings.GetDefaultState"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_ZebraAnalysisSettings_GetDefaultState.htm)

#### `public static void RestoreDefaults()`

Commits the default settings as the current settings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_ZebraAnalysisSettings_RestoreDefaults.htm)

#### `public static void UpdateFromState(ZebraAnalysisSettingsState state)`

Sets all settings to a particular defined joined state.

**Parameters:**
- `state` (Rhino.ApplicationSettings.ZebraAnalysisSettingsState) — The particular state.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_ApplicationSettings_ZebraAnalysisSettings_UpdateFromState.htm)

### Properties
- `ShowIsoCurves` (Boolean, get/set) — Get or sets the display of surface isocurves.
- `StripeColor` (Color, get/set) — Gets or sets the stripe color.
- `StripeThickness` (Int32, get/set) — Gets or sets the stripe thickness, where 0 = thinnest and 6 = thickest.
- `VerticalStripes` (Boolean, get/set) — Set to true for vertical stripes, or false for horizontal stripes.

## ZebraAnalysisSettingsState (class)

Represents a snapshot of ZebraAnalysisSettings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_ApplicationSettings_ZebraAnalysisSettingsState.htm)

### Properties
- `ShowIsoCurves` (Boolean, get/set) — Get or sets the display of surface isocurves.
- `StripeColor` (Color, get/set) — Gets or sets the stripe color.
- `StripeThickness` (Int32, get/set) — Gets or sets the stripe thickness, where 0 = thinnest and 6 = thickest.
- `VerticalStripes` (Boolean, get/set) — Set to true for vertical stripes, or false for horizontal stripes.

