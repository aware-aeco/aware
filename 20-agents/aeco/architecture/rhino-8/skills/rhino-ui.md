---
name: rhino-rhino-ui
description: This skill encodes the rhino 8.0 surface of the Rhino.UI namespace — 63 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Dialogs, DrawingUtilities, Fonts, GetColorEventArgs, LOC, Localization, LocalizeStringPair, MouseCallback, and 55 more types.
---

# Rhino.UI

Auto-generated from vendor docs for rhino 8.0. 63 types in this namespace.

## CursorStyle (enum)

Standard mouse cursors in Rhino

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_CursorStyle.htm)

### Values
- `Default` = `0`
- `Wait` = `1` — Wait
- `CrossHair` = `2`
- `Hand` = `3`
- `Rotate` = `4`
- `Magnify` = `5`
- `ArrowCopy` = `6` — arrow with +
- `CrosshairCopy` = `7`

## Dialogs (class)

[Missing <summary> documentation for "T:Rhino.UI.Dialogs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_Dialogs.htm)

### Methods
#### `public static void KillSplash()`

Destroy the splash screen if it is being displayed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_KillSplash.htm)

#### `public static void SetCustomColorDialog(EventHandler<GetColorEventArgs> handler)`



**Parameters:**
- `handler` (System.EventHandler<GetColorEventArgs>) — [Missing <param name="handler"/> documentation for "M:Rhino.UI.Dialogs.SetCustomColorDialog(System.EventHandler{Rhino.UI.GetColorEventArgs})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_SetCustomColorDialog.htm)

#### `public static void ShowAboutDialog(bool forceSimpleDialog)`



**Parameters:**
- `forceSimpleDialog` (System.Boolean) — [Missing <param name="forceSimpleDialog"/> documentation for "M:Rhino.UI.Dialogs.ShowAboutDialog(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowAboutDialog.htm)

#### `public static bool[] ShowCheckListBox(string title, string message, IList items, IList<bool> checkState)`

Displays Rhino's check list box.

**Parameters:**
- `title` (System.String) — The dialog title.
- `message` (System.String) — The dialog message.
- `items` (System.Collections.IList) — A list of items to show.
- `checkState` (System.Collections.Generic.IList<Boolean>) — A list of true/false boolean values.

**Returns:** `Boolean[]` — An array or boolean values determining if the user checked the corresponding box. On error, null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowCheckListBox.htm)

#### `public static bool ShowColorDialog(ref Color color)`

Display Rhino's color selection dialog.

**Parameters:**
- `color` (System.Drawing.Color) — [in/out] Default color for dialog, and will receive new color if function returns true.

**Returns:** `Boolean` — true if the color changed. false if the color has not changed or the user pressed cancel.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowColorDialog_1.htm)

#### `public static bool ShowColorDialog(ref Color color, bool includeButtonColors, string dialogTitle)`

Display Rhino's color selection dialog.

**Parameters:**
- `color` (System.Drawing.Color) — [in/out] Default color for dialog, and will receive new color if function returns true.
- `includeButtonColors` (System.Boolean) — Display button face and text options at top of named color list.
- `dialogTitle` (System.String) — The title of the dialog.

**Returns:** `Boolean` — true if the color changed. false if the color has not changed or the user pressed cancel.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowColorDialog_2.htm)

#### `public static bool ShowColorDialog(ref Color color, bool includeButtonColors, string dialogTitle, NamedColorList namedColorList)`

Display Rhino's color selection dialog.

**Parameters:**
- `color` (System.Drawing.Color) — [in/out] Default color for dialog, and will receive new color if function returns true.
- `includeButtonColors` (System.Boolean) — Display button face and text options at top of named color list.
- `dialogTitle` (System.String) — The title of the dialog.
- `namedColorList` (Rhino.UI.NamedColorList) — If not null and contains one or more named colors the Rhino color dialog named color list will be replaces with this list.

**Returns:** `Boolean` — true if the color changed. false if the color has not changed or the user pressed cancel.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowColorDialog_3.htm)

#### `public static bool ShowColorDialog(ref Color4f color, bool allowAlpha)`

Displays the standard modal color picker dialog for floating point colors.

**Parameters:**
- `color` (Rhino.Display.Color4f) — The initial color to set the picker to and also accepts the user's choice.
- `allowAlpha` (System.Boolean) — Specifies if the color picker should allow changes to the alpha channel or not.

**Returns:** `Boolean` — true if a color was picked, false if the user canceled the picker dialog.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowColorDialog.htm)

#### `[ObsoleteAttribute("Use ShowColorDialog(object parent, ref Color4f color, bool allowAlpha)")] public static bool ShowColorDialog(IWin32Window parent, ref Color4f color, bool allowAlpha)`

Displays the standard modal color picker dialog for floating point colors.

**Parameters:**
- `parent` (System.Windows.Forms.IWin32Window) — Parent window for this dialog, should always pass this if calling from a form or user control.
- `color` (Rhino.Display.Color4f) — The initial color to set the picker to and also accepts the user's choice.
- `allowAlpha` (System.Boolean) — Specifies if the color picker should allow changes to the alpha channel or not.

**Returns:** `Boolean` — true if a color was picked, false if the user canceled the picker dialog.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowColorDialog_7.htm)

#### `public static bool ShowColorDialog(Object parent, ref Color4f color, bool allowAlpha)`

Displays the standard modal color picker dialog for floating point colors.

**Parameters:**
- `parent` (System.Object) — Parent window for this dialog, should always pass this if calling from a form or user control.
- `color` (Rhino.Display.Color4f) — The initial color to set the picker to and also accepts the user's choice.
- `allowAlpha` (System.Boolean) — Specifies if the color picker should allow changes to the alpha channel or not.

**Returns:** `Boolean` — true if a color was picked, false if the user canceled the picker dialog.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowColorDialog_4.htm)

#### `public static bool ShowColorDialog(Object parent, ref Color4f color, bool allowAlpha, Dialogs.OnColorChangedEvent colorCallback)`

Displays the standard modal color picker dialog for floating point colors.

**Parameters:**
- `parent` (System.Object) — Parent window for this dialog, should always pass this if calling from a form or user control.
- `color` (Rhino.Display.Color4f) — The initial color to set the picker to and also accepts the user's choice.
- `allowAlpha` (System.Boolean) — Specifies if the color picker should allow changes to the alpha channel or not.
- `colorCallback` (Rhino.UI.Dialogs.OnColorChangedEvent) — May be optionally passed to ShowColorDialog and will get called when the color value changes in the color dialog.

**Returns:** `Boolean` — true if a color was picked, false if the user canceled the picker dialog.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowColorDialog_5.htm)

#### `public static bool ShowColorDialog(Object parent, ref Color4f color, bool allowAlpha, NamedColorList namedColorList, Dialogs.OnColorChangedEvent colorCallback)`

Displays the standard modal color picker dialog for floating point colors.

**Parameters:**
- `parent` (System.Object) — Parent window for this dialog, should always pass this if calling from a form or user control.
- `color` (Rhino.Display.Color4f) — The initial color to set the picker to and also accepts the user's choice.
- `allowAlpha` (System.Boolean) — Specifies if the color picker should allow changes to the alpha channel or not.
- `namedColorList` (Rhino.UI.NamedColorList) — If not null and contains at least one named color this list will replace the standard Rhino Color list displayed by the rhino color dialog.
- `colorCallback` (Rhino.UI.Dialogs.OnColorChangedEvent) — May be optionally passed to ShowColorDialog and will get called when the color value changes in the color dialog.

**Returns:** `Boolean` — true if a color was picked, false if the user canceled the picker dialog.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowColorDialog_6.htm)

#### `public static Object ShowComboListBox(string title, string message, IList items)`

Displays Rhino's combo list box.

**Parameters:**
- `title` (System.String) — The dialog title.
- `message` (System.String) — The dialog message.
- `items` (System.Collections.IList) — A list of items to show.

**Returns:** `Object` — selected item.null if the user canceled.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowComboListBox.htm)

#### `public static int ShowContextMenu(IEnumerable<string> items, Point screenPoint, IEnumerable<int> modes)`

Creates an ETO ContextMenu from an array of strings. Use the modes array to enable/disable menu items

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="items"/> documentation for "M:Rhino.UI.Dialogs.ShowContextMenu(System.Collections.Generic.IEnumerable{System.String},System.Drawing.Point,System.Collections.Generic.IEnumerable{System.Int32})"]
- `screenPoint` (System.Drawing.Point) — [Missing <param name="screenPoint"/> documentation for "M:Rhino.UI.Dialogs.ShowContextMenu(System.Collections.Generic.IEnumerable{System.String},System.Drawing.Point,System.Collections.Generic.IEnumerable{System.Int32})"]
- `modes` (System.Collections.Generic.IEnumerable<Int32>) — [Missing <param name="modes"/> documentation for "M:Rhino.UI.Dialogs.ShowContextMenu(System.Collections.Generic.IEnumerable{System.String},System.Drawing.Point,System.Collections.Generic.IEnumerable{System.Int32})"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.UI.Dialogs.ShowContextMenu(System.Collections.Generic.IEnumerable{System.String},System.Drawing.Point,System.Collections.Generic.IEnumerable{System.Int32})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowContextMenu.htm)

#### `public static bool ShowEditBox(string title, string message, string defaultText, bool multiline, out string text)`

Displays Rhino's string edit box.

**Parameters:**
- `title` (System.String) — The dialog title.
- `message` (System.String) — The dialog message.
- `defaultText` (System.String) — The default text.
- `multiline` (System.Boolean) — Set true for multi line editing.
- `text` (System.String) — The modified text.

**Returns:** `Boolean` — true of OK was clicked, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowEditBox.htm)

#### `public static bool ShowLayerMaterialDialog(RhinoDoc doc, IEnumerable<int> layerIndices)`

Show a modal dialog to edit the material in the layer specified in layerIndices.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — Rhino document
- `layerIndices` (System.Collections.Generic.IEnumerable<Int32>) — true if the new layer button will be visible.

**Returns:** `Boolean` — True if the dialog was closed with the OK button. False if the dialog was closed with escape.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowLayerMaterialDialog.htm)

#### `public static Object ShowLineTypes(string title, string message, RhinoDoc doc)`

Displays Rhino's LineType list box.

**Parameters:**
- `title` (System.String) — The dialog title.
- `message` (System.String) — The dialog message.
- `doc` (Rhino.RhinoDoc) — The active document.

**Returns:** `Object` — The id of the selected item if successful, null on cancel.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowLineTypes.htm)

#### `public static Guid ShowLineTypes(string title, string message, RhinoDoc doc, Guid selectedLineTypeId)`



**Parameters:**
- `title` (System.String) — [Missing <param name="title"/> documentation for "M:Rhino.UI.Dialogs.ShowLineTypes(System.String,System.String,Rhino.RhinoDoc,System.Guid)"]
- `message` (System.String) — [Missing <param name="message"/> documentation for "M:Rhino.UI.Dialogs.ShowLineTypes(System.String,System.String,Rhino.RhinoDoc,System.Guid)"]
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.UI.Dialogs.ShowLineTypes(System.String,System.String,Rhino.RhinoDoc,System.Guid)"]
- `selectedLineTypeId` (System.Guid) — [Missing <param name="selectedLineTypeId"/> documentation for "M:Rhino.UI.Dialogs.ShowLineTypes(System.String,System.String,Rhino.RhinoDoc,System.Guid)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.UI.Dialogs.ShowLineTypes(System.String,System.String,Rhino.RhinoDoc,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowLineTypes_1.htm)

#### `public static Object ShowListBox(string title, string message, IList items)`

Displays Rhino's list box.

**Parameters:**
- `title` (System.String) — The dialog title.
- `message` (System.String) — The dialog message.
- `items` (System.Collections.IList) — A list of items to show.

**Returns:** `Object` — The selected item if successful, null on cancel.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowListBox.htm)

#### `public static Object ShowListBox(string title, string message, IList items, Object selectedItem)`

Displays Rhino's list box.

**Parameters:**
- `title` (System.String) — The dialog title.
- `message` (System.String) — The dialog message.
- `items` (System.Collections.IList) — A list of items to show.
- `selectedItem` (System.Object) — The item to preselect.

**Returns:** `Object` — The selected item if successful, null on cancel.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowListBox_1.htm)

#### `public static ShowMessageResult ShowMessage(Object parent, string message, string title, ShowMessageButton buttons, ShowMessageIcon icon, ShowMessageDefaultButton defaultButton, ShowMessageOptions options, ShowMessageMode mode)`

Same as System.Windows.Forms.MessageBox.Show but using a message box tailored to Rhino.

**Parameters:**
- `parent` (System.Object) — Parent window
- `message` (System.String) — Message box text content.
- `title` (System.String) — Message box title text.
- `buttons` (Rhino.UI.ShowMessageButton) — Which buttons to display in the message box.
- `icon` (Rhino.UI.ShowMessageIcon) — Which icon to display in the message box.
- `defaultButton` (Rhino.UI.ShowMessageDefaultButton) — Which button is the default button.
- `options` (Rhino.UI.ShowMessageOptions) — Additional message box options.
- `mode` (Rhino.UI.ShowMessageMode) — The modality of the message box.

**Returns:** `ShowMessageResult` — One of the ShowMessageBoxResult values.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowMessage.htm)

#### `public static ShowMessageResult ShowMessage(string message, string title)`

Same as System.Windows.Forms.MessageBox.Show but using a message box tailored to Rhino.

**Parameters:**
- `message` (System.String) — Message box text content.
- `title` (System.String) — Message box title text.

**Returns:** `ShowMessageResult` — One of the ShowMessageBoxResult values.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowMessage_1.htm)

#### `public static ShowMessageResult ShowMessage(string message, string title, ShowMessageButton buttons, ShowMessageIcon icon)`

Same as System.Windows.Forms.MessageBox.Show but using a message box tailored to Rhino.

**Parameters:**
- `message` (System.String) — Message box text content.
- `title` (System.String) — Message box title text.
- `buttons` (Rhino.UI.ShowMessageButton) — Which buttons to display in the message box.
- `icon` (Rhino.UI.ShowMessageIcon) — Which icon to display in the message box.

**Returns:** `ShowMessageResult` — One of the ShowMessageBoxResult values.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowMessage_2.htm)

#### `public static string[] ShowMultiListBox(string title, string message, IList<string> items, IList<string> defaults = null)`

Displays Rhino's multiple selection list box.

**Parameters:**
- `title` (System.String) — The dialog title.
- `message` (System.String) — The dialog message.
- `items` (System.Collections.Generic.IList<String>) — A list of items to show.
- `defaults` (System.Collections.Generic.IList<String>) — The items to preselect.

**Returns:** `String[]` — The selected items if successful, null on cancel.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowMultiListBox.htm)

#### `public static bool ShowNumberBox(string title, string message, ref double number)`

Displays Rhino's number edit box.

**Parameters:**
- `title` (System.String) — The dialog title.
- `message` (System.String) — The dialog message.
- `number` (System.Double) — The default and return value.

**Returns:** `Boolean` — true of OK was clicked, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowNumberBox.htm)

#### `public static bool ShowNumberBox(string title, string message, ref double number, double minimum, double maximum)`

Displays Rhino's number edit box.

**Parameters:**
- `title` (System.String) — The dialog title.
- `message` (System.String) — The dialog message.
- `number` (System.Double) — The default and return value.
- `minimum` (System.Double) — The minimum allowable value.
- `maximum` (System.Double) — The maximum allowable value.

**Returns:** `Boolean` — true of OK was clicked, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowNumberBox_1.htm)

#### `public static double ShowPrintWidths(string title, string message)`

Shows the print widths dialogs

**Parameters:**
- `title` (System.String) — [Missing <param name="title"/> documentation for "M:Rhino.UI.Dialogs.ShowPrintWidths(System.String,System.String)"]
- `message` (System.String) — [Missing <param name="message"/> documentation for "M:Rhino.UI.Dialogs.ShowPrintWidths(System.String,System.String)"]

**Returns:** `Double` — The selected print width

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowPrintWidths.htm)

#### `public static double ShowPrintWidths(string title, string message, double selectedWidth)`

Shows the print widths dialogs

**Parameters:**
- `title` (System.String) — [Missing <param name="title"/> documentation for "M:Rhino.UI.Dialogs.ShowPrintWidths(System.String,System.String,System.Double)"]
- `message` (System.String) — [Missing <param name="message"/> documentation for "M:Rhino.UI.Dialogs.ShowPrintWidths(System.String,System.String,System.Double)"]
- `selectedWidth` (System.Double) — [Missing <param name="selectedWidth"/> documentation for "M:Rhino.UI.Dialogs.ShowPrintWidths(System.String,System.String,System.Double)"]

**Returns:** `Double` — The selected print width

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowPrintWidths_1.htm)

#### `public static string[] ShowPropertyListBox(string title, string message, IList items, IList<string> values)`

Displays Rhino's property list box.

**Parameters:**
- `title` (System.String) — The dialog title.
- `message` (System.String) — The dialog message.
- `items` (System.Collections.IList) — A list of property names.
- `values` (System.Collections.Generic.IList<String>) — A list of property values.

**Returns:** `String[]` — A list of property values if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowPropertyListBox_1.htm)

#### `public static string[] ShowPropertyListBox(string title, string message, List<KeyValuePair<string, string>> items)`

Displays Rhino's property list box.

**Parameters:**
- `title` (System.String) — The dialog title.
- `message` (System.String) — The dialog message.
- `items` (System.Collections.Generic.List<KeyValuePair<String,String>>) — A list of key value pairs

**Returns:** `String[]` — A list of property values if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowPropertyListBox.htm)

#### `public static bool ShowSelectLayerDialog(ref int layerIndex, string dialogTitle, bool showNewLayerButton, bool showSetCurrentButton, ref bool initialSetCurrentState)`

Displays Rhino's single layer selection dialog.

**Parameters:**
- `layerIndex` (System.Int32) — Initial layer for the dialog, and will receive selected layer if function returns DialogResult.OK.
- `dialogTitle` (System.String) — The dialog title.
- `showNewLayerButton` (System.Boolean) — true if the new layer button will be visible.
- `showSetCurrentButton` (System.Boolean) — true if the set current button will be visible.
- `initialSetCurrentState` (System.Boolean) — true if the current state will be initially set.

**Returns:** `Boolean` — True if the dialog was closed with the OK button. False if the dialog was closed with escape.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowSelectLayerDialog.htm)

#### `public static bool ShowSelectLinetypeDialog(ref int linetypeIndex, bool displayByLayer)`

Displays Rhino's single linetype selection dialog.

**Parameters:**
- `linetypeIndex` (System.Int32) — Initial linetype for the dialog, and will receive selected linetype if function returns true.
- `displayByLayer` (System.Boolean) — Displays the "ByLayer" linetype in the list. Defaults to false.

**Returns:** `Boolean` — True if the dialog was closed with the OK button. False if the dialog was closed with escape.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowSelectLinetypeDialog.htm)

#### `public static bool ShowSelectMultipleLayersDialog(IEnumerable<int> defaultLayerIndices, string dialogTitle, bool showNewLayerButton, out int[] layerIndices)`



**Parameters:**
- `defaultLayerIndices` (System.Collections.Generic.IEnumerable<Int32>) — [Missing <param name="defaultLayerIndices"/> documentation for "M:Rhino.UI.Dialogs.ShowSelectMultipleLayersDialog(System.Collections.Generic.IEnumerable{System.Int32},System.String,System.Boolean,System.Int32[]@)"]
- `dialogTitle` (System.String) — [Missing <param name="dialogTitle"/> documentation for "M:Rhino.UI.Dialogs.ShowSelectMultipleLayersDialog(System.Collections.Generic.IEnumerable{System.Int32},System.String,System.Boolean,System.Int32[]@)"]
- `showNewLayerButton` (System.Boolean) — [Missing <param name="showNewLayerButton"/> documentation for "M:Rhino.UI.Dialogs.ShowSelectMultipleLayersDialog(System.Collections.Generic.IEnumerable{System.Int32},System.String,System.Boolean,System.Int32[]@)"]
- `layerIndices` (System.Int32[]) — [Missing <param name="layerIndices"/> documentation for "M:Rhino.UI.Dialogs.ShowSelectMultipleLayersDialog(System.Collections.Generic.IEnumerable{System.Int32},System.String,System.Boolean,System.Int32[]@)"]

**Returns:** `Boolean` — True if the dialog was closed with the OK button. False if the dialog was closed with escape.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowSelectMultipleLayersDialog.htm)

#### `public static bool ShowSunDialog(Sun sun)`

Show the modal sun dialog.

**Parameters:**
- `sun` (Rhino.Render.Sun) — [Missing <param name="sun"/> documentation for "M:Rhino.UI.Dialogs.ShowSunDialog(Rhino.Render.Sun)"]

**Returns:** `Boolean` — Returns true if the user clicked OK, or false if the user canceled.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowSunDialog.htm)

#### `public static void ShowTextDialog(string message, string title)`

Display a text dialog similar to the dialog used for the "What" command.

**Parameters:**
- `message` (System.String) — Text to display as the message content.
- `title` (System.String) — Test to display as the form title.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Dialogs_ShowTextDialog.htm)

## Dialogs.OnColorChangedEvent (delegate)

May be optionally passed to ShowColorDialog and will get called when the color value changes in the color dialog.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_Dialogs_OnColorChangedEvent.htm)

## DistanceDisplayMode (enum)

[Missing <summary> documentation for "T:Rhino.UI.DistanceDisplayMode"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_DistanceDisplayMode.htm)

### Values
- `Decimal` = `0`
- `Fractional` = `1`
- `FeetInches` = `2`

## DrawingUtilities (class)

RhinoCommon Drawing Utilities

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_DrawingUtilities.htm)

### Methods
#### `public static Bitmap BitmapFromIconResource(string resourceName, Assembly assembly = null)`

Load a Icon from an embedded resource and convert it to a bitmap of the specified size. Will look for a icon image that is greater than or equal to the requested size, if all images are less than the requested size the largest image will be used.

**Parameters:**
- `resourceName` (System.String) — [Missing <param name="resourceName"/> documentation for "M:Rhino.UI.DrawingUtilities.BitmapFromIconResource(System.String,System.Reflection.Assembly)"]
- `assembly` (System.Reflection.Assembly) — The assembly containing the manifest resource, will use the calling assembly if null.

**Returns:** `Bitmap` — [Missing <returns> documentation for "M:Rhino.UI.DrawingUtilities.BitmapFromIconResource(System.String,System.Reflection.Assembly)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_DrawingUtilities_BitmapFromIconResource_1.htm)

#### `public static Bitmap BitmapFromIconResource(string resourceName, Size bitmapSize, Assembly assembly = null)`

Load a Icon from an embedded resource and convert it to a bitmap of the specified size. Will look for a icon image that is greater than or equal to the requested size, if all images are less than the requested size the largest image will be used.

**Parameters:**
- `resourceName` (System.String) — [Missing <param name="resourceName"/> documentation for "M:Rhino.UI.DrawingUtilities.BitmapFromIconResource(System.String,System.Drawing.Size,System.Reflection.Assembly)"]
- `bitmapSize` (System.Drawing.Size) — Desired bitmap size
- `assembly` (System.Reflection.Assembly) — The assembly containing the manifest resource, will use the calling assembly if null.

**Returns:** `Bitmap` — [Missing <returns> documentation for "M:Rhino.UI.DrawingUtilities.BitmapFromIconResource(System.String,System.Drawing.Size,System.Reflection.Assembly)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_DrawingUtilities_BitmapFromIconResource.htm)

#### `public static Bitmap BitmapFromSvg(string svg, int width, int height)`

Create bitmap of a given size from an svg string

**Parameters:**
- `svg` (System.String) — [Missing <param name="svg"/> documentation for "M:Rhino.UI.DrawingUtilities.BitmapFromSvg(System.String,System.Int32,System.Int32)"]
- `width` (System.Int32) — [Missing <param name="width"/> documentation for "M:Rhino.UI.DrawingUtilities.BitmapFromSvg(System.String,System.Int32,System.Int32)"]
- `height` (System.Int32) — [Missing <param name="height"/> documentation for "M:Rhino.UI.DrawingUtilities.BitmapFromSvg(System.String,System.Int32,System.Int32)"]

**Returns:** `Bitmap` — [Missing <returns> documentation for "M:Rhino.UI.DrawingUtilities.BitmapFromSvg(System.String,System.Int32,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_DrawingUtilities_BitmapFromSvg.htm)

#### `public static Bitmap BitmapFromSvg(string svg, int width, int height, bool adjustForDarkMode)`

Create bitmap of a given size from an svg string

**Parameters:**
- `svg` (System.String) — [Missing <param name="svg"/> documentation for "M:Rhino.UI.DrawingUtilities.BitmapFromSvg(System.String,System.Int32,System.Int32,System.Boolean)"]
- `width` (System.Int32) — [Missing <param name="width"/> documentation for "M:Rhino.UI.DrawingUtilities.BitmapFromSvg(System.String,System.Int32,System.Int32,System.Boolean)"]
- `height` (System.Int32) — [Missing <param name="height"/> documentation for "M:Rhino.UI.DrawingUtilities.BitmapFromSvg(System.String,System.Int32,System.Int32,System.Boolean)"]
- `adjustForDarkMode` (System.Boolean) — [Missing <param name="adjustForDarkMode"/> documentation for "M:Rhino.UI.DrawingUtilities.BitmapFromSvg(System.String,System.Int32,System.Int32,System.Boolean)"]

**Returns:** `Bitmap` — [Missing <returns> documentation for "M:Rhino.UI.DrawingUtilities.BitmapFromSvg(System.String,System.Int32,System.Int32,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_DrawingUtilities_BitmapFromSvg_1.htm)

#### `public static List<Point2f[]> CreateCurvePreviewGeometry(Curve curve, Linetype linetype, int width, int height)`

Create a series of closed polyline loops representing a curve and a linetype combination.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — [Missing <param name="curve"/> documentation for "M:Rhino.UI.DrawingUtilities.CreateCurvePreviewGeometry(Rhino.Geometry.Curve,Rhino.DocObjects.Linetype,System.Int32,System.Int32)"]
- `linetype` (Rhino.DocObjects.Linetype) — [Missing <param name="linetype"/> documentation for "M:Rhino.UI.DrawingUtilities.CreateCurvePreviewGeometry(Rhino.Geometry.Curve,Rhino.DocObjects.Linetype,System.Int32,System.Int32)"]
- `width` (System.Int32) — [Missing <param name="width"/> documentation for "M:Rhino.UI.DrawingUtilities.CreateCurvePreviewGeometry(Rhino.Geometry.Curve,Rhino.DocObjects.Linetype,System.Int32,System.Int32)"]
- `height` (System.Int32) — [Missing <param name="height"/> documentation for "M:Rhino.UI.DrawingUtilities.CreateCurvePreviewGeometry(Rhino.Geometry.Curve,Rhino.DocObjects.Linetype,System.Int32,System.Int32)"]

**Returns:** `List<Point2f[]>` — [Missing <returns> documentation for "M:Rhino.UI.DrawingUtilities.CreateCurvePreviewGeometry(Rhino.Geometry.Curve,Rhino.DocObjects.Linetype,System.Int32,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_DrawingUtilities_CreateCurvePreviewGeometry.htm)

#### `public static Bitmap CreateMeshPreviewImage(IEnumerable<Mesh> meshes, IEnumerable<Color> colors, Size size)`

Creates a preview image of one or more meshes.

**Parameters:**
- `meshes` (System.Collections.Generic.IEnumerable<Mesh>) — The meshes.
- `colors` (System.Collections.Generic.IEnumerable<Color>) — The draw colors, one for each mesh.
- `size` (System.Drawing.Size) — The size of the preview image.

**Returns:** `Bitmap` — A bitmap if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_DrawingUtilities_CreateMeshPreviewImage_2.htm)

#### `public static Bitmap CreateMeshPreviewImage(Mesh mesh, Color color, Size size)`

Creates a preview image of a mesh.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — The mesh.
- `color` (System.Drawing.Color) — The draw color.
- `size` (System.Drawing.Size) — The size of the preview image.

**Returns:** `Bitmap` — A bitmap if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_DrawingUtilities_CreateMeshPreviewImage.htm)

#### `public static Bitmap CreateMeshPreviewImage(RhinoDoc doc, IEnumerable<Mesh> meshes, IEnumerable<Color> colors, Size size)`

Creates a preview image of one or more meshes.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — The active document.
- `meshes` (System.Collections.Generic.IEnumerable<Mesh>) — The meshes.
- `colors` (System.Collections.Generic.IEnumerable<Color>) — The draw colors, one for each mesh.
- `size` (System.Drawing.Size) — The size of the preview image.

**Returns:** `Bitmap` — A bitmap if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_DrawingUtilities_CreateMeshPreviewImage_1.htm)

#### `public static void DarkModeConvertPixel(ref byte r, ref byte g, ref byte b)`

Attempt to adjust pixels to an image that works for dark mode

**Parameters:**
- `r` (System.Byte) — [Missing <param name="r"/> documentation for "M:Rhino.UI.DrawingUtilities.DarkModeConvertPixel(System.Byte@,System.Byte@,System.Byte@)"]
- `g` (System.Byte) — [Missing <param name="g"/> documentation for "M:Rhino.UI.DrawingUtilities.DarkModeConvertPixel(System.Byte@,System.Byte@,System.Byte@)"]
- `b` (System.Byte) — [Missing <param name="b"/> documentation for "M:Rhino.UI.DrawingUtilities.DarkModeConvertPixel(System.Byte@,System.Byte@,System.Byte@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_DrawingUtilities_DarkModeConvertPixel.htm)

#### `public static void DarkModeConvertPixels(ref byte[] rgbaBytes)`

Attempt to adjust pixels to an image that works for dark mode

**Parameters:**
- `rgbaBytes` (System.Byte[]) — [Missing <param name="rgbaBytes"/> documentation for "M:Rhino.UI.DrawingUtilities.DarkModeConvertPixels(System.Byte[]@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_DrawingUtilities_DarkModeConvertPixels.htm)

#### `public static Icon IconFromResource(string resourceName, Assembly assembly = null)`

Load a Icon from an embedded resource.

**Parameters:**
- `resourceName` (System.String) — The case-sensitive name of the icon manifest resource being requested.
- `assembly` (System.Reflection.Assembly) — The assembly containing the manifest resource, will use the calling assembly if null.

**Returns:** `Icon` — The Icon resource if found and loaded otherwise null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_DrawingUtilities_IconFromResource_1.htm)

#### `public static Icon IconFromResource(string resourceName, Size size, Assembly assembly = null)`

Load a Icon from an embedded resource.

**Parameters:**
- `resourceName` (System.String) — The case-sensitive name of the icon manifest resource being requested.
- `size` (System.Drawing.Size) — The desired size of the icon.
- `assembly` (System.Reflection.Assembly) — The assembly containing the manifest resource, will use the calling assembly if null.

**Returns:** `Icon` — The Icon resource if found and loaded otherwise null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_DrawingUtilities_IconFromResource.htm)

#### `public static Image ImageFromResource(string resourceName, Assembly assembly = null)`

Load a Image from an embedded resource.

**Parameters:**
- `resourceName` (System.String) — The case-sensitive name of the image manifest resource being requested.
- `assembly` (System.Reflection.Assembly) — The assembly containing the manifest resource, will use the calling assembly if null.

**Returns:** `Image` — The Image resource if found and loaded otherwise null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_DrawingUtilities_ImageFromResource.htm)

#### `public static Bitmap LoadBitmapWithScaleDown(string iconName, int sizeDesired, Assembly assembly = null)`

Loads an icon from an embedded resource and converts it to a bitmap. If the icon is not a standard size, this function scales down a larger image.

**Parameters:**
- `iconName` (System.String) — The case-sensitive name of the icon manifest resource being requested.
- `sizeDesired` (System.Int32) — The desired size, in pixels, of the icon.
- `assembly` (System.Reflection.Assembly) — The assembly containing the manifest resource.

**Returns:** `Bitmap` — The icon converted to a bitmap if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_DrawingUtilities_LoadBitmapWithScaleDown.htm)

#### `public static Icon LoadIconWithScaleDown(string iconName, int sizeDesired, Assembly assembly = null)`

Loads an icon from an embedded resource. If the icon is not a standard size, this function scales down a larger image.

**Parameters:**
- `iconName` (System.String) — The case-sensitive name of the icon manifest resource being requested.
- `sizeDesired` (System.Int32) — The desired size, in pixels, of the icon.
- `assembly` (System.Reflection.Assembly) — The assembly containing the manifest resource.

**Returns:** `Icon` — The icon if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_DrawingUtilities_LoadIconWithScaleDown.htm)

#### `public static int MakeArgb(byte a, byte r, byte g, byte b)`



**Parameters:**
- `a` (System.Byte) — [Missing <param name="a"/> documentation for "M:Rhino.UI.DrawingUtilities.MakeArgb(System.Byte,System.Byte,System.Byte,System.Byte)"]
- `r` (System.Byte) — [Missing <param name="r"/> documentation for "M:Rhino.UI.DrawingUtilities.MakeArgb(System.Byte,System.Byte,System.Byte,System.Byte)"]
- `g` (System.Byte) — [Missing <param name="g"/> documentation for "M:Rhino.UI.DrawingUtilities.MakeArgb(System.Byte,System.Byte,System.Byte,System.Byte)"]
- `b` (System.Byte) — [Missing <param name="b"/> documentation for "M:Rhino.UI.DrawingUtilities.MakeArgb(System.Byte,System.Byte,System.Byte,System.Byte)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.UI.DrawingUtilities.MakeArgb(System.Byte,System.Byte,System.Byte,System.Byte)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_DrawingUtilities_MakeArgb.htm)

#### `public static byte[] PixelsFromSvg(string svg, int width, int height, bool premultiplyAlpha, Color backgroundColor)`

Get array of bytes that represent RGBA values for an image of a given size created from SVG

**Parameters:**
- `svg` (System.String) — [Missing <param name="svg"/> documentation for "M:Rhino.UI.DrawingUtilities.PixelsFromSvg(System.String,System.Int32,System.Int32,System.Boolean,System.Drawing.Color)"]
- `width` (System.Int32) — [Missing <param name="width"/> documentation for "M:Rhino.UI.DrawingUtilities.PixelsFromSvg(System.String,System.Int32,System.Int32,System.Boolean,System.Drawing.Color)"]
- `height` (System.Int32) — [Missing <param name="height"/> documentation for "M:Rhino.UI.DrawingUtilities.PixelsFromSvg(System.String,System.Int32,System.Int32,System.Boolean,System.Drawing.Color)"]
- `premultiplyAlpha` (System.Boolean) — Pre-lends the background color with the pixels based on their alpha value
- `backgroundColor` (System.Drawing.Color) — Background color - pass Color.Empty if you don't know what to pass.

**Returns:** `Byte[]` — [Missing <returns> documentation for "M:Rhino.UI.DrawingUtilities.PixelsFromSvg(System.String,System.Int32,System.Int32,System.Boolean,System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_DrawingUtilities_PixelsFromSvg.htm)

#### `public static void SvgToRhinoDibIntPtr(string svg, int width, int height, bool adjustForDarkMode, IntPtr pRhinoDib)`

Create bitmap of a given size from an svg string

**Parameters:**
- `svg` (System.String) — [Missing <param name="svg"/> documentation for "M:Rhino.UI.DrawingUtilities.SvgToRhinoDibIntPtr(System.String,System.Int32,System.Int32,System.Boolean,System.IntPtr)"]
- `width` (System.Int32) — [Missing <param name="width"/> documentation for "M:Rhino.UI.DrawingUtilities.SvgToRhinoDibIntPtr(System.String,System.Int32,System.Int32,System.Boolean,System.IntPtr)"]
- `height` (System.Int32) — [Missing <param name="height"/> documentation for "M:Rhino.UI.DrawingUtilities.SvgToRhinoDibIntPtr(System.String,System.Int32,System.Int32,System.Boolean,System.IntPtr)"]
- `adjustForDarkMode` (System.Boolean) — [Missing <param name="adjustForDarkMode"/> documentation for "M:Rhino.UI.DrawingUtilities.SvgToRhinoDibIntPtr(System.String,System.Int32,System.Int32,System.Boolean,System.IntPtr)"]
- `pRhinoDib` (System.IntPtr) — [Missing <param name="pRhinoDib"/> documentation for "M:Rhino.UI.DrawingUtilities.SvgToRhinoDibIntPtr(System.String,System.Int32,System.Int32,System.Boolean,System.IntPtr)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_DrawingUtilities_SvgToRhinoDibIntPtr.htm)

## Fonts (class)

Rhino.Rumtime.UI

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_Fonts.htm)

### Constructors
- `public Fonts()` — Initializes a new instance of the Fonts class

### Methods
#### `public static Font GetUiFont(Fonts.Style style, Fonts.Size size)`

GetUiFont provides access to a set of unmanaged fonts used by Rhino.

**Parameters:**
- `style` (Rhino.UI.Fonts.Style) — Normal, Bold, Italic
- `size` (Rhino.UI.Fonts.Size) — One of several preset enumerable sizes

**Returns:** `Font` — [Missing <returns> documentation for "M:Rhino.UI.Fonts.GetUiFont(Rhino.UI.Fonts.Style,Rhino.UI.Fonts.Size)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Fonts_GetUiFont.htm)

### Properties
- `BoldHeadingFont` (Font, get) — Returns a font that is 1.2x NormalFont and Bold
- `HeadingFont` (Font, get) — Returns a font used for dialog headings. 1.2x the size of NormalFont.
- `NormalFont` (Font, get) — Returns the normal font used for dialog boxes and buttons.
- `SmallFont` (Font, get) — Returns a font use for small text in dialog boxes. 0.8x the size of NormalFont.
- `TitleFont` (Font, get) — Returns a font used for dialog titles. 2x the size of NormalFont, and bold.

## Fonts.Size (enum)

Size for UI font.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_Fonts_Size.htm)

### Values
- `Small` = `0` — 0.8 x Normal
- `Normal` = `1` — 1.0 x Normal
- `Large` = `2` — 1.2 x Normal
- `Title` = `3` — 2.0 x Normal

## Fonts.Style (enum)

Style for UI font.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_Fonts_Style.htm)

### Values
- `Regular` = `0` — Normal
- `Bold` = `1` — Bold
- `Italic` = `2` — Italic
- `Underline` = `4` — Underline
- `Strikeout` = `8` — Strikeout

## GetColorEventArgs (class)

[Missing <summary> documentation for "T:Rhino.UI.GetColorEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_GetColorEventArgs.htm)

### Properties
- `IncludeButtonColors` (Boolean, get) — 
- `InputColor` (Color, get) — 
- `SelectedColor` (Color, get/set) — 
- `Title` (String, get) — 

## IDialogService (interface)

[Missing <summary> documentation for "T:Rhino.UI.IDialogService"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_IDialogService.htm)

### Methods
#### `IntPtr ObjectToWindowHandle(Object window, bool useMainRhinoWindowWhenNull)`



**Parameters:**
- `window` (System.Object) — [Missing <param name="window"/> documentation for "M:Rhino.UI.IDialogService.ObjectToWindowHandle(System.Object,System.Boolean)"]
- `useMainRhinoWindowWhenNull` (System.Boolean) — [Missing <param name="useMainRhinoWindowWhenNull"/> documentation for "M:Rhino.UI.IDialogService.ObjectToWindowHandle(System.Object,System.Boolean)"]

**Returns:** `IntPtr` — [Missing <returns> documentation for "M:Rhino.UI.IDialogService.ObjectToWindowHandle(System.Object,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IDialogService_ObjectToWindowHandle.htm)

#### `bool ShowColorDialog(Object parent, ref Color4f color, bool allowAlpha, Dialogs.OnColorChangedEvent colorCallback)`



**Parameters:**
- `parent` (System.Object) — [Missing <param name="parent"/> documentation for "M:Rhino.UI.IDialogService.ShowColorDialog(System.Object,Rhino.Display.Color4f@,System.Boolean,Rhino.UI.Dialogs.OnColorChangedEvent)"]
- `color` (Rhino.Display.Color4f) — [Missing <param name="color"/> documentation for "M:Rhino.UI.IDialogService.ShowColorDialog(System.Object,Rhino.Display.Color4f@,System.Boolean,Rhino.UI.Dialogs.OnColorChangedEvent)"]
- `allowAlpha` (System.Boolean) — [Missing <param name="allowAlpha"/> documentation for "M:Rhino.UI.IDialogService.ShowColorDialog(System.Object,Rhino.Display.Color4f@,System.Boolean,Rhino.UI.Dialogs.OnColorChangedEvent)"]
- `colorCallback` (Rhino.UI.Dialogs.OnColorChangedEvent) — [Missing <param name="colorCallback"/> documentation for "M:Rhino.UI.IDialogService.ShowColorDialog(System.Object,Rhino.Display.Color4f@,System.Boolean,Rhino.UI.Dialogs.OnColorChangedEvent)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.IDialogService.ShowColorDialog(System.Object,Rhino.Display.Color4f@,System.Boolean,Rhino.UI.Dialogs.OnColorChangedEvent)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IDialogService_ShowColorDialog.htm)

#### `string[] ShowMultiListBox(string title, string message, IList<string> items, IList<string> defaults = null)`



**Parameters:**
- `title` (System.String) — [Missing <param name="title"/> documentation for "M:Rhino.UI.IDialogService.ShowMultiListBox(System.String,System.String,System.Collections.Generic.IList{System.String},System.Collections.Generic.IList{System.String})"]
- `message` (System.String) — [Missing <param name="message"/> documentation for "M:Rhino.UI.IDialogService.ShowMultiListBox(System.String,System.String,System.Collections.Generic.IList{System.String},System.Collections.Generic.IList{System.String})"]
- `items` (System.Collections.Generic.IList<String>) — [Missing <param name="items"/> documentation for "M:Rhino.UI.IDialogService.ShowMultiListBox(System.String,System.String,System.Collections.Generic.IList{System.String},System.Collections.Generic.IList{System.String})"]
- `defaults` (System.Collections.Generic.IList<String>) — [Missing <param name="defaults"/> documentation for "M:Rhino.UI.IDialogService.ShowMultiListBox(System.String,System.String,System.Collections.Generic.IList{System.String},System.Collections.Generic.IList{System.String})"]

**Returns:** `String[]` — [Missing <returns> documentation for "M:Rhino.UI.IDialogService.ShowMultiListBox(System.String,System.String,System.Collections.Generic.IList{System.String},System.Collections.Generic.IList{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IDialogService_ShowMultiListBox.htm)

#### `Object WrapAsIWin32Window(IntPtr handle)`



**Parameters:**
- `handle` (System.IntPtr) — [Missing <param name="handle"/> documentation for "M:Rhino.UI.IDialogService.WrapAsIWin32Window(System.IntPtr)"]

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.UI.IDialogService.WrapAsIWin32Window(System.IntPtr)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IDialogService_WrapAsIWin32Window.htm)

## IHelp (interface)

Implement this class to add help to a modeless UI panel.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_IHelp.htm)

### Properties
- `HelpUrl` (String, get) — Help topic URL which gets passed to RhinoHelp.Show

## ILocalizationService (interface)

[Missing <summary> documentation for "T:Rhino.UI.ILocalizationService"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_ILocalizationService.htm)

### Methods
#### `string LocalizeCommandName(Assembly assembly, int languageId, string english)`



**Parameters:**
- `assembly` (System.Reflection.Assembly) — [Missing <param name="assembly"/> documentation for "M:Rhino.UI.ILocalizationService.LocalizeCommandName(System.Reflection.Assembly,System.Int32,System.String)"]
- `languageId` (System.Int32) — [Missing <param name="languageId"/> documentation for "M:Rhino.UI.ILocalizationService.LocalizeCommandName(System.Reflection.Assembly,System.Int32,System.String)"]
- `english` (System.String) — [Missing <param name="english"/> documentation for "M:Rhino.UI.ILocalizationService.LocalizeCommandName(System.Reflection.Assembly,System.Int32,System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.UI.ILocalizationService.LocalizeCommandName(System.Reflection.Assembly,System.Int32,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ILocalizationService_LocalizeCommandName.htm)

#### `string LocalizeDialogItem(Assembly assembly, int languageId, string key, string english)`



**Parameters:**
- `assembly` (System.Reflection.Assembly) — [Missing <param name="assembly"/> documentation for "M:Rhino.UI.ILocalizationService.LocalizeDialogItem(System.Reflection.Assembly,System.Int32,System.String,System.String)"]
- `languageId` (System.Int32) — [Missing <param name="languageId"/> documentation for "M:Rhino.UI.ILocalizationService.LocalizeDialogItem(System.Reflection.Assembly,System.Int32,System.String,System.String)"]
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.UI.ILocalizationService.LocalizeDialogItem(System.Reflection.Assembly,System.Int32,System.String,System.String)"]
- `english` (System.String) — [Missing <param name="english"/> documentation for "M:Rhino.UI.ILocalizationService.LocalizeDialogItem(System.Reflection.Assembly,System.Int32,System.String,System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.UI.ILocalizationService.LocalizeDialogItem(System.Reflection.Assembly,System.Int32,System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ILocalizationService_LocalizeDialogItem.htm)

#### `void LocalizeForm(Assembly assembly, int languageId, Object formOrUserControl)`



**Parameters:**
- `assembly` (System.Reflection.Assembly) — [Missing <param name="assembly"/> documentation for "M:Rhino.UI.ILocalizationService.LocalizeForm(System.Reflection.Assembly,System.Int32,System.Object)"]
- `languageId` (System.Int32) — [Missing <param name="languageId"/> documentation for "M:Rhino.UI.ILocalizationService.LocalizeForm(System.Reflection.Assembly,System.Int32,System.Object)"]
- `formOrUserControl` (System.Object) — [Missing <param name="formOrUserControl"/> documentation for "M:Rhino.UI.ILocalizationService.LocalizeForm(System.Reflection.Assembly,System.Int32,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ILocalizationService_LocalizeForm.htm)

#### `string LocalizeString(Assembly assembly, int languageId, string english, int contextId)`



**Parameters:**
- `assembly` (System.Reflection.Assembly) — [Missing <param name="assembly"/> documentation for "M:Rhino.UI.ILocalizationService.LocalizeString(System.Reflection.Assembly,System.Int32,System.String,System.Int32)"]
- `languageId` (System.Int32) — [Missing <param name="languageId"/> documentation for "M:Rhino.UI.ILocalizationService.LocalizeString(System.Reflection.Assembly,System.Int32,System.String,System.Int32)"]
- `english` (System.String) — [Missing <param name="english"/> documentation for "M:Rhino.UI.ILocalizationService.LocalizeString(System.Reflection.Assembly,System.Int32,System.String,System.Int32)"]
- `contextId` (System.Int32) — [Missing <param name="contextId"/> documentation for "M:Rhino.UI.ILocalizationService.LocalizeString(System.Reflection.Assembly,System.Int32,System.String,System.Int32)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.UI.ILocalizationService.LocalizeString(System.Reflection.Assembly,System.Int32,System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ILocalizationService_LocalizeString.htm)

## IPanel (interface)

Implement this interface when you want to be notified of when a panel is shown, hidden or closed.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_IPanel.htm)

### Methods
#### `void PanelClosing(uint documentSerialNumber, bool onCloseDocument)`



**Parameters:**
- `documentSerialNumber` (System.UInt32) — [Missing <param name="documentSerialNumber"/> documentation for "M:Rhino.UI.IPanel.PanelClosing(System.UInt32,System.Boolean)"]
- `onCloseDocument` (System.Boolean) — [Missing <param name="onCloseDocument"/> documentation for "M:Rhino.UI.IPanel.PanelClosing(System.UInt32,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IPanel_PanelClosing.htm)

#### `void PanelHidden(uint documentSerialNumber, ShowPanelReason reason)`



**Parameters:**
- `documentSerialNumber` (System.UInt32) — [Missing <param name="documentSerialNumber"/> documentation for "M:Rhino.UI.IPanel.PanelHidden(System.UInt32,Rhino.UI.ShowPanelReason)"]
- `reason` (Rhino.UI.ShowPanelReason) — [Missing <param name="reason"/> documentation for "M:Rhino.UI.IPanel.PanelHidden(System.UInt32,Rhino.UI.ShowPanelReason)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IPanel_PanelHidden.htm)

#### `void PanelShown(uint documentSerialNumber, ShowPanelReason reason)`



**Parameters:**
- `documentSerialNumber` (System.UInt32) — [Missing <param name="documentSerialNumber"/> documentation for "M:Rhino.UI.IPanel.PanelShown(System.UInt32,Rhino.UI.ShowPanelReason)"]
- `reason` (Rhino.UI.ShowPanelReason) — [Missing <param name="reason"/> documentation for "M:Rhino.UI.IPanel.PanelShown(System.UInt32,Rhino.UI.ShowPanelReason)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IPanel_PanelShown.htm)

## IPanelsService (interface)

For internal use, the IPanels service is implemented in RhinoWindows or RhinoMac as appropriate and handles the communication with core Rhino

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_IPanelsService.htm)

### Methods
#### `bool CreateDockBar(Object options)`



**Parameters:**
- `options` (System.Object) — [Missing <param name="options"/> documentation for "M:Rhino.UI.IPanelsService.CreateDockBar(System.Object)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.IPanelsService.CreateDockBar(System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IPanelsService_CreateDockBar.htm)

#### `void DestroyNativeWindow(Object host, Object nativeObject, bool disposeOfNativeObject)`



**Parameters:**
- `host` (System.Object) — [Missing <param name="host"/> documentation for "M:Rhino.UI.IPanelsService.DestroyNativeWindow(System.Object,System.Object,System.Boolean)"]
- `nativeObject` (System.Object) — [Missing <param name="nativeObject"/> documentation for "M:Rhino.UI.IPanelsService.DestroyNativeWindow(System.Object,System.Object,System.Boolean)"]
- `disposeOfNativeObject` (System.Boolean) — [Missing <param name="disposeOfNativeObject"/> documentation for "M:Rhino.UI.IPanelsService.DestroyNativeWindow(System.Object,System.Object,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IPanelsService_DestroyNativeWindow.htm)

#### `bool DockBarIdInUse(Guid barId)`



**Parameters:**
- `barId` (System.Guid) — [Missing <param name="barId"/> documentation for "M:Rhino.UI.IPanelsService.DockBarIdInUse(System.Guid)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.IPanelsService.DockBarIdInUse(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IPanelsService_DockBarIdInUse.htm)

#### `bool DockBarIsVisible(Guid barId)`



**Parameters:**
- `barId` (System.Guid) — [Missing <param name="barId"/> documentation for "M:Rhino.UI.IPanelsService.DockBarIsVisible(System.Guid)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.IPanelsService.DockBarIsVisible(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IPanelsService_DockBarIsVisible.htm)

#### `void FactoryResetSettings()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IPanelsService_FactoryResetSettings.htm)

#### `bool Float(Guid barId, Point point)`



**Parameters:**
- `barId` (System.Guid) — [Missing <param name="barId"/> documentation for "M:Rhino.UI.IPanelsService.Float(System.Guid,System.Drawing.Point)"]
- `point` (System.Drawing.Point) — [Missing <param name="point"/> documentation for "M:Rhino.UI.IPanelsService.Float(System.Guid,System.Drawing.Point)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.IPanelsService.Float(System.Guid,System.Drawing.Point)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IPanelsService_Float.htm)

#### `bool ResizeFloating(Guid barId, Size size)`



**Parameters:**
- `barId` (System.Guid) — [Missing <param name="barId"/> documentation for "M:Rhino.UI.IPanelsService.ResizeFloating(System.Guid,System.Drawing.Size)"]
- `size` (System.Drawing.Size) — [Missing <param name="size"/> documentation for "M:Rhino.UI.IPanelsService.ResizeFloating(System.Guid,System.Drawing.Size)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.IPanelsService.ResizeFloating(System.Guid,System.Drawing.Size)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IPanelsService_ResizeFloating.htm)

#### `void SetF1Hook(Object nativeObject, EventHandler hook)`



**Parameters:**
- `nativeObject` (System.Object) — [Missing <param name="nativeObject"/> documentation for "M:Rhino.UI.IPanelsService.SetF1Hook(System.Object,System.EventHandler)"]
- `hook` (System.EventHandler) — [Missing <param name="hook"/> documentation for "M:Rhino.UI.IPanelsService.SetF1Hook(System.Object,System.EventHandler)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IPanelsService_SetF1Hook.htm)

#### `bool ShowDockBar(Guid barId, bool show)`



**Parameters:**
- `barId` (System.Guid) — [Missing <param name="barId"/> documentation for "M:Rhino.UI.IPanelsService.ShowDockBar(System.Guid,System.Boolean)"]
- `show` (System.Boolean) — [Missing <param name="show"/> documentation for "M:Rhino.UI.IPanelsService.ShowDockBar(System.Guid,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.IPanelsService.ShowDockBar(System.Guid,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IPanelsService_ShowDockBar.htm)

#### `bool StartDraggingDockBar(Guid barId, Point mouseDownPoint, Point screenStartPoint)`



**Parameters:**
- `barId` (System.Guid) — [Missing <param name="barId"/> documentation for "M:Rhino.UI.IPanelsService.StartDraggingDockBar(System.Guid,System.Drawing.Point,System.Drawing.Point)"]
- `mouseDownPoint` (System.Drawing.Point) — [Missing <param name="mouseDownPoint"/> documentation for "M:Rhino.UI.IPanelsService.StartDraggingDockBar(System.Guid,System.Drawing.Point,System.Drawing.Point)"]
- `screenStartPoint` (System.Drawing.Point) — [Missing <param name="screenStartPoint"/> documentation for "M:Rhino.UI.IPanelsService.StartDraggingDockBar(System.Guid,System.Drawing.Point,System.Drawing.Point)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.IPanelsService.StartDraggingDockBar(System.Guid,System.Drawing.Point,System.Drawing.Point)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IPanelsService_StartDraggingDockBar.htm)

#### `bool SupportedType(Type type, out string exceptionMessage)`



**Parameters:**
- `type` (System.Type) — [Missing <param name="type"/> documentation for "M:Rhino.UI.IPanelsService.SupportedType(System.Type,System.String@)"]
- `exceptionMessage` (System.String) — [Missing <param name="exceptionMessage"/> documentation for "M:Rhino.UI.IPanelsService.SupportedType(System.Type,System.String@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.IPanelsService.SupportedType(System.Type,System.String@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IPanelsService_SupportedType.htm)

#### `bool ToggleDocking(Guid barId)`



**Parameters:**
- `barId` (System.Guid) — [Missing <param name="barId"/> documentation for "M:Rhino.UI.IPanelsService.ToggleDocking(System.Guid)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.IPanelsService.ToggleDocking(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IPanelsService_ToggleDocking.htm)

#### `bool UnhookDeleteAndDestroyDockBar(Guid id)`



**Parameters:**
- `id` (System.Guid) — [Missing <param name="id"/> documentation for "M:Rhino.UI.IPanelsService.UnhookDeleteAndDestroyDockBar(System.Guid)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.IPanelsService.UnhookDeleteAndDestroyDockBar(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IPanelsService_UnhookDeleteAndDestroyDockBar.htm)

## IRhinoUiDialogService (interface)

Used by Rhino.UI.Dialogs to access generic Eto dialogs from Rhino Common

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_IRhinoUiDialogService.htm)

### Methods
#### `void DetectColorScheme(out bool defaultLightMode, out bool defaultDarkMode)`



**Parameters:**
- `defaultLightMode` (System.Boolean) — [Missing <param name="defaultLightMode"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.DetectColorScheme(System.Boolean@,System.Boolean@)"]
- `defaultDarkMode` (System.Boolean) — [Missing <param name="defaultDarkMode"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.DetectColorScheme(System.Boolean@,System.Boolean@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IRhinoUiDialogService_DetectColorScheme.htm)

#### `Icon IconFromResourceId(Assembly iconAssembly, string iconResourceId)`



**Parameters:**
- `iconAssembly` (System.Reflection.Assembly) — [Missing <param name="iconAssembly"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.IconFromResourceId(System.Reflection.Assembly,System.String)"]
- `iconResourceId` (System.String) — [Missing <param name="iconResourceId"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.IconFromResourceId(System.Reflection.Assembly,System.String)"]

**Returns:** `Icon` — [Missing <returns> documentation for "M:Rhino.UI.IRhinoUiDialogService.IconFromResourceId(System.Reflection.Assembly,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IRhinoUiDialogService_IconFromResourceId.htm)

#### `bool SetToDefaultColorScheme(bool dark)`



**Parameters:**
- `dark` (System.Boolean) — [Missing <param name="dark"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.SetToDefaultColorScheme(System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.IRhinoUiDialogService.SetToDefaultColorScheme(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IRhinoUiDialogService_SetToDefaultColorScheme.htm)

#### `bool[] ShowCheckListBox(string title, string message, IList items, IList<bool> checkState)`



**Parameters:**
- `title` (System.String) — [Missing <param name="title"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowCheckListBox(System.String,System.String,System.Collections.IList,System.Collections.Generic.IList{System.Boolean})"]
- `message` (System.String) — [Missing <param name="message"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowCheckListBox(System.String,System.String,System.Collections.IList,System.Collections.Generic.IList{System.Boolean})"]
- `items` (System.Collections.IList) — [Missing <param name="items"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowCheckListBox(System.String,System.String,System.Collections.IList,System.Collections.Generic.IList{System.Boolean})"]
- `checkState` (System.Collections.Generic.IList<Boolean>) — [Missing <param name="checkState"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowCheckListBox(System.String,System.String,System.Collections.IList,System.Collections.Generic.IList{System.Boolean})"]

**Returns:** `Boolean[]` — [Missing <returns> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowCheckListBox(System.String,System.String,System.Collections.IList,System.Collections.Generic.IList{System.Boolean})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IRhinoUiDialogService_ShowCheckListBox.htm)

#### `Object ShowComboListBox(string title, string message, IList items)`



**Parameters:**
- `title` (System.String) — [Missing <param name="title"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowComboListBox(System.String,System.String,System.Collections.IList)"]
- `message` (System.String) — [Missing <param name="message"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowComboListBox(System.String,System.String,System.Collections.IList)"]
- `items` (System.Collections.IList) — [Missing <param name="items"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowComboListBox(System.String,System.String,System.Collections.IList)"]

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowComboListBox(System.String,System.String,System.Collections.IList)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IRhinoUiDialogService_ShowComboListBox.htm)

#### `bool ShowEditBox(string title, string message, string defaultText, bool multiline, out string text)`



**Parameters:**
- `title` (System.String) — [Missing <param name="title"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowEditBox(System.String,System.String,System.String,System.Boolean,System.String@)"]
- `message` (System.String) — [Missing <param name="message"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowEditBox(System.String,System.String,System.String,System.Boolean,System.String@)"]
- `defaultText` (System.String) — [Missing <param name="defaultText"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowEditBox(System.String,System.String,System.String,System.Boolean,System.String@)"]
- `multiline` (System.Boolean) — [Missing <param name="multiline"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowEditBox(System.String,System.String,System.String,System.Boolean,System.String@)"]
- `text` (System.String) — [Missing <param name="text"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowEditBox(System.String,System.String,System.String,System.Boolean,System.String@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowEditBox(System.String,System.String,System.String,System.Boolean,System.String@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IRhinoUiDialogService_ShowEditBox.htm)

#### `Object ShowLineTypes(string title, string message, RhinoDoc doc)`



**Parameters:**
- `title` (System.String) — [Missing <param name="title"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowLineTypes(System.String,System.String,Rhino.RhinoDoc)"]
- `message` (System.String) — [Missing <param name="message"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowLineTypes(System.String,System.String,Rhino.RhinoDoc)"]
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowLineTypes(System.String,System.String,Rhino.RhinoDoc)"]

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowLineTypes(System.String,System.String,Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IRhinoUiDialogService_ShowLineTypes.htm)

#### `Guid ShowLineTypes(string title, string message, RhinoDoc doc, Guid selectedLinetypeId)`



**Parameters:**
- `title` (System.String) — [Missing <param name="title"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowLineTypes(System.String,System.String,Rhino.RhinoDoc,System.Guid)"]
- `message` (System.String) — [Missing <param name="message"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowLineTypes(System.String,System.String,Rhino.RhinoDoc,System.Guid)"]
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowLineTypes(System.String,System.String,Rhino.RhinoDoc,System.Guid)"]
- `selectedLinetypeId` (System.Guid) — [Missing <param name="selectedLinetypeId"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowLineTypes(System.String,System.String,Rhino.RhinoDoc,System.Guid)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowLineTypes(System.String,System.String,Rhino.RhinoDoc,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IRhinoUiDialogService_ShowLineTypes_1.htm)

#### `Object ShowListBox(string title, string message, IList items, Object selectedItem)`



**Parameters:**
- `title` (System.String) — [Missing <param name="title"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowListBox(System.String,System.String,System.Collections.IList,System.Object)"]
- `message` (System.String) — [Missing <param name="message"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowListBox(System.String,System.String,System.Collections.IList,System.Object)"]
- `items` (System.Collections.IList) — [Missing <param name="items"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowListBox(System.String,System.String,System.Collections.IList,System.Object)"]
- `selectedItem` (System.Object) — [Missing <param name="selectedItem"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowListBox(System.String,System.String,System.Collections.IList,System.Object)"]

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowListBox(System.String,System.String,System.Collections.IList,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IRhinoUiDialogService_ShowListBox.htm)

#### `string[] ShowMultiListBox(IList<string> items, string message, string title, IList<string> defaults)`



**Parameters:**
- `items` (System.Collections.Generic.IList<String>) — [Missing <param name="items"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowMultiListBox(System.Collections.Generic.IList{System.String},System.String,System.String,System.Collections.Generic.IList{System.String})"]
- `message` (System.String) — [Missing <param name="message"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowMultiListBox(System.Collections.Generic.IList{System.String},System.String,System.String,System.Collections.Generic.IList{System.String})"]
- `title` (System.String) — [Missing <param name="title"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowMultiListBox(System.Collections.Generic.IList{System.String},System.String,System.String,System.Collections.Generic.IList{System.String})"]
- `defaults` (System.Collections.Generic.IList<String>) — [Missing <param name="defaults"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowMultiListBox(System.Collections.Generic.IList{System.String},System.String,System.String,System.Collections.Generic.IList{System.String})"]

**Returns:** `String[]` — [Missing <returns> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowMultiListBox(System.Collections.Generic.IList{System.String},System.String,System.String,System.Collections.Generic.IList{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IRhinoUiDialogService_ShowMultiListBox.htm)

#### `bool ShowNumberBox(string title, string message, ref double number, double minimum, double maximum)`



**Parameters:**
- `title` (System.String) — [Missing <param name="title"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowNumberBox(System.String,System.String,System.Double@,System.Double,System.Double)"]
- `message` (System.String) — [Missing <param name="message"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowNumberBox(System.String,System.String,System.Double@,System.Double,System.Double)"]
- `number` (System.Double) — [Missing <param name="number"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowNumberBox(System.String,System.String,System.Double@,System.Double,System.Double)"]
- `minimum` (System.Double) — [Missing <param name="minimum"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowNumberBox(System.String,System.String,System.Double@,System.Double,System.Double)"]
- `maximum` (System.Double) — [Missing <param name="maximum"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowNumberBox(System.String,System.String,System.Double@,System.Double,System.Double)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowNumberBox(System.String,System.String,System.Double@,System.Double,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IRhinoUiDialogService_ShowNumberBox.htm)

#### `int ShowPopupMenu(string[] arrItems, int[] arrModes, int? screenPointX, int? screenPointY)`



**Parameters:**
- `arrItems` (System.String[]) — [Missing <param name="arrItems"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowPopupMenu(System.String[],System.Int32[],System.Nullable{System.Int32},System.Nullable{System.Int32})"]
- `arrModes` (System.Int32[]) — [Missing <param name="arrModes"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowPopupMenu(System.String[],System.Int32[],System.Nullable{System.Int32},System.Nullable{System.Int32})"]
- `screenPointX` (System.Nullable<Int32>) — [Missing <param name="screenPointX"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowPopupMenu(System.String[],System.Int32[],System.Nullable{System.Int32},System.Nullable{System.Int32})"]
- `screenPointY` (System.Nullable<Int32>) — [Missing <param name="screenPointY"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowPopupMenu(System.String[],System.Int32[],System.Nullable{System.Int32},System.Nullable{System.Int32})"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowPopupMenu(System.String[],System.Int32[],System.Nullable{System.Int32},System.Nullable{System.Int32})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IRhinoUiDialogService_ShowPopupMenu.htm)

#### `double ShowPrintWidths(string title, string message)`



**Parameters:**
- `title` (System.String) — [Missing <param name="title"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowPrintWidths(System.String,System.String)"]
- `message` (System.String) — [Missing <param name="message"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowPrintWidths(System.String,System.String)"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowPrintWidths(System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IRhinoUiDialogService_ShowPrintWidths.htm)

#### `double ShowPrintWidths(string title, string message, double selectedWidth)`



**Parameters:**
- `title` (System.String) — [Missing <param name="title"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowPrintWidths(System.String,System.String,System.Double)"]
- `message` (System.String) — [Missing <param name="message"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowPrintWidths(System.String,System.String,System.Double)"]
- `selectedWidth` (System.Double) — [Missing <param name="selectedWidth"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowPrintWidths(System.String,System.String,System.Double)"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowPrintWidths(System.String,System.String,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IRhinoUiDialogService_ShowPrintWidths_1.htm)

#### `string[] ShowPropertyListBox(string title, string message, IList items, IList<string> values)`



**Parameters:**
- `title` (System.String) — [Missing <param name="title"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowPropertyListBox(System.String,System.String,System.Collections.IList,System.Collections.Generic.IList{System.String})"]
- `message` (System.String) — [Missing <param name="message"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowPropertyListBox(System.String,System.String,System.Collections.IList,System.Collections.Generic.IList{System.String})"]
- `items` (System.Collections.IList) — [Missing <param name="items"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowPropertyListBox(System.String,System.String,System.Collections.IList,System.Collections.Generic.IList{System.String})"]
- `values` (System.Collections.Generic.IList<String>) — [Missing <param name="values"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowPropertyListBox(System.String,System.String,System.Collections.IList,System.Collections.Generic.IList{System.String})"]

**Returns:** `String[]` — [Missing <returns> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowPropertyListBox(System.String,System.String,System.Collections.IList,System.Collections.Generic.IList{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IRhinoUiDialogService_ShowPropertyListBox_1.htm)

#### `string[] ShowPropertyListBox(string title, string message, List<KeyValuePair<string, string>> items)`



**Parameters:**
- `title` (System.String) — [Missing <param name="title"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowPropertyListBox(System.String,System.String,System.Collections.Generic.List{System.Collections.Generic.KeyValuePair{System.String,System.String}})"]
- `message` (System.String) — [Missing <param name="message"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowPropertyListBox(System.String,System.String,System.Collections.Generic.List{System.Collections.Generic.KeyValuePair{System.String,System.String}})"]
- `items` (System.Collections.Generic.List<KeyValuePair<String,String>>) — [Missing <param name="items"/> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowPropertyListBox(System.String,System.String,System.Collections.Generic.List{System.Collections.Generic.KeyValuePair{System.String,System.String}})"]

**Returns:** `String[]` — [Missing <returns> documentation for "M:Rhino.UI.IRhinoUiDialogService.ShowPropertyListBox(System.String,System.String,System.Collections.Generic.List{System.Collections.Generic.KeyValuePair{System.String,System.String}})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IRhinoUiDialogService_ShowPropertyListBox.htm)

## IStackedDialogPageService (interface)

For internal use, the IStackedDialogPageService service is implemented in RhinoWindows or RhinoMac as appropriate and handles the communication with core Rhino

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_IStackedDialogPageService.htm)

### Methods
#### `IntPtr GetImageHandle(Icon icon, bool canBeNull)`

Convert image to platform specific unmanaged pointer

**Parameters:**
- `icon` (System.Drawing.Icon) — [Missing <param name="icon"/> documentation for "M:Rhino.UI.IStackedDialogPageService.GetImageHandle(System.Drawing.Icon,System.Boolean)"]
- `canBeNull` (System.Boolean) — [Missing <param name="canBeNull"/> documentation for "M:Rhino.UI.IStackedDialogPageService.GetImageHandle(System.Drawing.Icon,System.Boolean)"]

**Returns:** `IntPtr` — [Missing <returns> documentation for "M:Rhino.UI.IStackedDialogPageService.GetImageHandle(System.Drawing.Icon,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IStackedDialogPageService_GetImageHandle.htm)

#### `IntPtr GetImageHandle(Image image, bool canBeNull)`

Convert image to platform specific unmanaged pointer

**Parameters:**
- `image` (System.Drawing.Image) — [Missing <param name="image"/> documentation for "M:Rhino.UI.IStackedDialogPageService.GetImageHandle(System.Drawing.Image,System.Boolean)"]
- `canBeNull` (System.Boolean) — [Missing <param name="canBeNull"/> documentation for "M:Rhino.UI.IStackedDialogPageService.GetImageHandle(System.Drawing.Image,System.Boolean)"]

**Returns:** `IntPtr` — [Missing <returns> documentation for "M:Rhino.UI.IStackedDialogPageService.GetImageHandle(System.Drawing.Image,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IStackedDialogPageService_GetImageHandle_1.htm)

#### `IntPtr GetNativePageWindow(Object nativeWindowObject, bool isRhinoPanel, bool applyPanelStyles, out Object host)`

Get the unmanaged pointer associated with the pages content control

**Parameters:**
- `nativeWindowObject` (System.Object) — [Missing <param name="nativeWindowObject"/> documentation for "M:Rhino.UI.IStackedDialogPageService.GetNativePageWindow(System.Object,System.Boolean,System.Boolean,System.Object@)"]
- `isRhinoPanel` (System.Boolean) — [Missing <param name="isRhinoPanel"/> documentation for "M:Rhino.UI.IStackedDialogPageService.GetNativePageWindow(System.Object,System.Boolean,System.Boolean,System.Object@)"]
- `applyPanelStyles` (System.Boolean) — [Missing <param name="applyPanelStyles"/> documentation for "M:Rhino.UI.IStackedDialogPageService.GetNativePageWindow(System.Object,System.Boolean,System.Boolean,System.Object@)"]
- `host` (System.Object) — [Missing <param name="host"/> documentation for "M:Rhino.UI.IStackedDialogPageService.GetNativePageWindow(System.Object,System.Boolean,System.Boolean,System.Object@)"]

**Returns:** `IntPtr` — [Missing <returns> documentation for "M:Rhino.UI.IStackedDialogPageService.GetNativePageWindow(System.Object,System.Boolean,System.Boolean,System.Object@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IStackedDialogPageService_GetNativePageWindow.htm)

#### `IntPtr GetNativePageWindow(Object pageObject, bool isRhinoPanel, bool applyPanelStyles, out Object nativeWindowObject, out Object host)`

Get the unmanaged pointer associated with the pages content control

**Parameters:**
- `pageObject` (System.Object) — [Missing <param name="pageObject"/> documentation for "M:Rhino.UI.IStackedDialogPageService.GetNativePageWindow(System.Object,System.Boolean,System.Boolean,System.Object@,System.Object@)"]
- `isRhinoPanel` (System.Boolean) — [Missing <param name="isRhinoPanel"/> documentation for "M:Rhino.UI.IStackedDialogPageService.GetNativePageWindow(System.Object,System.Boolean,System.Boolean,System.Object@,System.Object@)"]
- `applyPanelStyles` (System.Boolean) — [Missing <param name="applyPanelStyles"/> documentation for "M:Rhino.UI.IStackedDialogPageService.GetNativePageWindow(System.Object,System.Boolean,System.Boolean,System.Object@,System.Object@)"]
- `nativeWindowObject` (System.Object) — [Missing <param name="nativeWindowObject"/> documentation for "M:Rhino.UI.IStackedDialogPageService.GetNativePageWindow(System.Object,System.Boolean,System.Boolean,System.Object@,System.Object@)"]
- `host` (System.Object) — [Missing <param name="host"/> documentation for "M:Rhino.UI.IStackedDialogPageService.GetNativePageWindow(System.Object,System.Boolean,System.Boolean,System.Object@,System.Object@)"]

**Returns:** `IntPtr` — [Missing <returns> documentation for "M:Rhino.UI.IStackedDialogPageService.GetNativePageWindow(System.Object,System.Boolean,System.Boolean,System.Object@,System.Object@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IStackedDialogPageService_GetNativePageWindow_1.htm)

#### `IntPtr NativeHandle(Object host)`

Gets the native window handle associated with the host object if there is one.

**Parameters:**
- `host` (System.Object) — [Missing <param name="host"/> documentation for "M:Rhino.UI.IStackedDialogPageService.NativeHandle(System.Object)"]

**Returns:** `IntPtr` — [Missing <returns> documentation for "M:Rhino.UI.IStackedDialogPageService.NativeHandle(System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IStackedDialogPageService_NativeHandle.htm)

#### `void RedrawPageControl(Object pageControl)`

Redraw the specified control.

**Parameters:**
- `pageControl` (System.Object) — Control to redraw

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IStackedDialogPageService_RedrawPageControl.htm)

#### `bool SetNativeParent(IntPtr hwndParent, Object host)`

Sets the parent on a host returned from GetNativePageWindow

**Parameters:**
- `hwndParent` (System.IntPtr) — [Missing <param name="hwndParent"/> documentation for "M:Rhino.UI.IStackedDialogPageService.SetNativeParent(System.IntPtr,System.Object)"]
- `host` (System.Object) — [Missing <param name="host"/> documentation for "M:Rhino.UI.IStackedDialogPageService.SetNativeParent(System.IntPtr,System.Object)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.IStackedDialogPageService.SetNativeParent(System.IntPtr,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IStackedDialogPageService_SetNativeParent.htm)

#### `bool TryGetControlMinimumSize(Object controlObject, out SizeF size)`

Get the minimum size associated with a control object

**Parameters:**
- `controlObject` (System.Object) — The control object to check for minimum size.
- `size` (System.Drawing.SizeF) — The minimum size of the control if provided.

**Returns:** `Boolean` — Returns true if get control minimum size found, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_IStackedDialogPageService_TryGetControlMinimumSize.htm)

## KeyboardKey (enum)

Keyboard key recognized by shortcuts

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_KeyboardKey.htm)

### Values
- `None` = `0` — No key
- `Tab` = `9` — Tab key
- `PageUp` = `33` — PageUp key
- `PageDown` = `34` — PageDown key
- `End` = `35` — End key
- `Home` = `36` — Home key
- `Num0` = `48` — 0 key
- `Num1` = `49` — 1 key
- `Num2` = `50` — 2 key
- `Num3` = `51` — 3 key
- `Num4` = `52` — 4 key
- `Num5` = `53` — 5 key
- `Num6` = `54` — 6 key
- `Num7` = `55` — 7 key
- `Num8` = `56` — 8 key
- `Num9` = `57` — 9 key
- `A` = `65` — A key
- `B` = `66` — B key
- `C` = `67` — C key
- `D` = `68` — D key
- `E` = `69` — E key
- `F` = `70` — F key
- `G` = `71` — G key
- `H` = `72` — H key
- `I` = `73` — I key
- `J` = `74` — J key
- `K` = `75` — K key
- `L` = `76` — L key
- `M` = `77` — M key
- `N` = `78` — N key
- `O` = `79` — O key
- `P` = `80` — P key
- `Q` = `81` — Q key
- `R` = `82` — R key
- `S` = `83` — S key
- `T` = `84` — T key
- `U` = `85` — U key
- `V` = `86` — V key
- `W` = `87` — W key
- `X` = `88` — X key
- `Y` = `89` — Y key
- `Z` = `90` — Z key
- `F1` = `112` — F1 key
- `F2` = `113` — F2 key
- `F3` = `114` — F3 key
- `F4` = `115` — F4 key
- `F5` = `116` — F5 key
- `F6` = `117` — F6 key
- `F7` = `118` — F7 key
- `F8` = `119` — F8 key
- `F9` = `120` — F9 key
- `F10` = `121` — F10 key
- `F11` = `122` — F11 key
- `F12` = `123` — F12 key
- `Semicolon` = `186` — ; key
- `Equal` = `187` — + key
- `Comma` = `188` — , key
- `Minus` = `189` — - key
- `Period` = `190` — . key
- `Slash` = `191` — / key
- `Grave` = `192` — Backtick key
- `LeftBracket` = `219` — [ key
- `BackSlash` = `220` — Back slash key
- `RightBracket` = `221` — ] key
- `Quote` = `222` — Quote key

## LOC (class)

Used a placeholder which is used by LocalizationProcessor application to create contextId mapped localized strings.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_LOC.htm)

### Methods
#### `public static string COMMANDNAME(string english)`

Command names that need to be localized should call this function. The COMMANDNAME function doesn't actually do anything but return the original string. The LocalizationProcessor application walks through the source code of a project and looks for LOC.COMMANDNAME and builds a record for each command name for the translators that can be used by developers in a commands overridden Rhino.Commands.Command.LocalName which should call Rhino.UI.Localization.LocalizeCommandName(EnglishName)

**Parameters:**
- `english` (System.String) — [in] The English string to localize.

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.UI.LOC.COMMANDNAME(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_LOC_COMMANDNAME.htm)

#### `public static LocalizeStringPair CON(string english)`

Command option name strings that need to be localized should call this function. The CON function doesn't actually do anything but return the original string. The LocalizationProcessor application walks through the source code of a project and looks for LOC.CON. The function is then replaced with a call to Localization.LocalizeCommandOptionName using a unique context ID.

**Parameters:**
- `english` (System.String) — [in] The English string to localize.

**Returns:** `LocalizeStringPair` — Returns localized string pair with both the English and local names set to the English value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_LOC_CON.htm)

#### `[ObsoleteAttribute("Don't copy and paste context IDs", true)] public static LocalizeStringPair CON(string english, int contextid)`

DO NOT use this function, it is a trap to determine where context IDs have been copied from other, already extracted, strings.

**Parameters:**
- `english` (System.String) — The English name.
- `contextid` (System.Int32) — Copied context id.

**Returns:** `LocalizeStringPair` — English name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_LOC_CON_1.htm)

#### `public static LocalizeStringPair CON(string english, Object assemblyFromObject)`

Command option name strings that need to be localized should call this function. The CON function doesn't actually do anything but return the original string. The LocalizationProcessor application walks through the source code of a project and looks for LOC.CON. The function is then replaced with a call to Localization.LocalizeCommandOptionName using a unique context ID.

**Parameters:**
- `english` (System.String) — [in] The English string to localize.
- `assemblyFromObject` (System.Object) — [in] The object that identifies the assembly that owns the command option name.

**Returns:** `LocalizeStringPair` — Returns localized string pair with both the English and local names set to the English value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_LOC_CON_2.htm)

#### `public static LocalizeStringPair COV(string english)`

Command option name strings that need to be localized should call this function. The COV function doesn't actually do anything but return the original string. The LocalizationProcessor application walks through the source code of a project and looks for LOC.COV. The function is then replaced with a call to Localization.LocalizeCommandOptionValue using a unique context ID.

**Parameters:**
- `english` (System.String) — [in] The English string to localize.

**Returns:** `LocalizeStringPair` — Returns localized string pair with both the English and local names set to the English value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_LOC_COV.htm)

#### `public static LocalizeStringPair COV(string english, Object assemblyFromObject)`

Command option name strings that need to be localized should call this function. The COV function doesn't actually do anything but return the original string. The LocalizationProcessor application walks through the source code of a project and looks for LOC.COV. The function is then replaced with a call to Localization.LocalizeCommandOptionValue using a unique context ID.

**Parameters:**
- `english` (System.String) — [in] The English string to localize.
- `assemblyFromObject` (System.Object) — [in] The object that identifies the assembly that owns the command option value.

**Returns:** `LocalizeStringPair` — Returns localized string pair with both the English and local names set to the English value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_LOC_COV_1.htm)

#### `public static string STR(string english)`

Strings that need to be localized should call this function. The STR function doesn't actually do anything but return the original string. The LocalizationProcessor application walks through the source code of a project and looks for LOC.STR. The function is then replaced with a call to Localization.LocalizeString using a unique context ID.

**Parameters:**
- `english` (System.String) — [in] The English string to localize.

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.UI.LOC.STR(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_LOC_STR.htm)

#### `[ObsoleteAttribute("Don't copy and paste context IDs", true)] public static string STR(string english, int contextid)`

DO NOT use this function, it is a trap to determine where context IDs have been copied from other, already extracted, strings.

**Parameters:**
- `english` (System.String) — The English name.
- `contextid` (System.Int32) — Copied context id.

**Returns:** `String` — English name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_LOC_STR_1.htm)

#### `public static string STR(string english, Object assemblyOrObject)`

Similar to Format(String, Object) function.

**Parameters:**
- `english` (System.String) — The English name.
- `assemblyOrObject` (System.Object) — Unused.

**Returns:** `String` — English name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_LOC_STR_2.htm)

## Localization (class)

[Missing <summary> documentation for "T:Rhino.UI.Localization"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_Localization.htm)

### Methods
#### `public static string FormatArea(double area, UnitSystem units, DimensionStyle dimStyle, bool alternate)`

Format an Area string from a number

**Parameters:**
- `area` (System.Double) — [Missing <param name="area"/> documentation for "M:Rhino.UI.Localization.FormatArea(System.Double,Rhino.UnitSystem,Rhino.DocObjects.DimensionStyle,System.Boolean)"]
- `units` (Rhino.UnitSystem) — [Missing <param name="units"/> documentation for "M:Rhino.UI.Localization.FormatArea(System.Double,Rhino.UnitSystem,Rhino.DocObjects.DimensionStyle,System.Boolean)"]
- `dimStyle` (Rhino.DocObjects.DimensionStyle) — [Missing <param name="dimStyle"/> documentation for "M:Rhino.UI.Localization.FormatArea(System.Double,Rhino.UnitSystem,Rhino.DocObjects.DimensionStyle,System.Boolean)"]
- `alternate` (System.Boolean) — [Missing <param name="alternate"/> documentation for "M:Rhino.UI.Localization.FormatArea(System.Double,Rhino.UnitSystem,Rhino.DocObjects.DimensionStyle,System.Boolean)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.UI.Localization.FormatArea(System.Double,Rhino.UnitSystem,Rhino.DocObjects.DimensionStyle,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Localization_FormatArea.htm)

#### `public static string FormatDistanceAndTolerance(double distance, UnitSystem units, DimensionStyle dimStyle, bool alternate)`



**Parameters:**
- `distance` (System.Double) — [Missing <param name="distance"/> documentation for "M:Rhino.UI.Localization.FormatDistanceAndTolerance(System.Double,Rhino.UnitSystem,Rhino.DocObjects.DimensionStyle,System.Boolean)"]
- `units` (Rhino.UnitSystem) — [Missing <param name="units"/> documentation for "M:Rhino.UI.Localization.FormatDistanceAndTolerance(System.Double,Rhino.UnitSystem,Rhino.DocObjects.DimensionStyle,System.Boolean)"]
- `dimStyle` (Rhino.DocObjects.DimensionStyle) — [Missing <param name="dimStyle"/> documentation for "M:Rhino.UI.Localization.FormatDistanceAndTolerance(System.Double,Rhino.UnitSystem,Rhino.DocObjects.DimensionStyle,System.Boolean)"]
- `alternate` (System.Boolean) — primary or alternate

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.UI.Localization.FormatDistanceAndTolerance(System.Double,Rhino.UnitSystem,Rhino.DocObjects.DimensionStyle,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Localization_FormatDistanceAndTolerance.htm)

#### `public static string FormatNumber(double x)`

Get a string version of a number.

**Parameters:**
- `x` (System.Double) — The number to format into a string.

**Returns:** `String` — The formatted number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Localization_FormatNumber.htm)

#### `public static string FormatNumber(double x, UnitSystem units, DistanceDisplayMode mode, int precision, bool appendUnitSystemName)`

Get a string version of a number in a given unit system / display mode.

**Parameters:**
- `x` (System.Double) — The number to format into a string.
- `units` (Rhino.UnitSystem) — The unit system for the number.
- `mode` (Rhino.UI.DistanceDisplayMode) — How the number should be formatted.
- `precision` (System.Int32) — The precision of the number.
- `appendUnitSystemName` (System.Boolean) — Adds unit system name to the end of the number.

**Returns:** `String` — The formatted number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Localization_FormatNumber_1.htm)

#### `public static string FormatVolume(double volume, UnitSystem units, DimensionStyle dimStyle, bool alternate)`

Format a Volume string from a number

**Parameters:**
- `volume` (System.Double) — [Missing <param name="volume"/> documentation for "M:Rhino.UI.Localization.FormatVolume(System.Double,Rhino.UnitSystem,Rhino.DocObjects.DimensionStyle,System.Boolean)"]
- `units` (Rhino.UnitSystem) — [Missing <param name="units"/> documentation for "M:Rhino.UI.Localization.FormatVolume(System.Double,Rhino.UnitSystem,Rhino.DocObjects.DimensionStyle,System.Boolean)"]
- `dimStyle` (Rhino.DocObjects.DimensionStyle) — [Missing <param name="dimStyle"/> documentation for "M:Rhino.UI.Localization.FormatVolume(System.Double,Rhino.UnitSystem,Rhino.DocObjects.DimensionStyle,System.Boolean)"]
- `alternate` (System.Boolean) — [Missing <param name="alternate"/> documentation for "M:Rhino.UI.Localization.FormatVolume(System.Double,Rhino.UnitSystem,Rhino.DocObjects.DimensionStyle,System.Boolean)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.UI.Localization.FormatVolume(System.Double,Rhino.UnitSystem,Rhino.DocObjects.DimensionStyle,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Localization_FormatVolume.htm)

#### `public static bool GetLanguages(out SimpleArrayInt ids, out ClassArrayString names)`



**Parameters:**
- `ids` (Rhino.Runtime.InteropWrappers.SimpleArrayInt) — [Missing <param name="ids"/> documentation for "M:Rhino.UI.Localization.GetLanguages(Rhino.Runtime.InteropWrappers.SimpleArrayInt@,Rhino.Runtime.InteropWrappers.ClassArrayString@)"]
- `names` (Rhino.Runtime.InteropWrappers.ClassArrayString) — [Missing <param name="names"/> documentation for "M:Rhino.UI.Localization.GetLanguages(Rhino.Runtime.InteropWrappers.SimpleArrayInt@,Rhino.Runtime.InteropWrappers.ClassArrayString@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.Localization.GetLanguages(Rhino.Runtime.InteropWrappers.SimpleArrayInt@,Rhino.Runtime.InteropWrappers.ClassArrayString@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Localization_GetLanguages.htm)

#### `public static string LocalizeCommandName(string english)`

Commands that need to be localized should call this function.

**Parameters:**
- `english` (System.String) — The localized command name.

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.UI.Localization.LocalizeCommandName(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Localization_LocalizeCommandName.htm)

#### `public static string LocalizeCommandName(string english, Object assemblyOrObject)`



**Parameters:**
- `english` (System.String) — [Missing <param name="english"/> documentation for "M:Rhino.UI.Localization.LocalizeCommandName(System.String,System.Object)"]
- `assemblyOrObject` (System.Object) — [Missing <param name="assemblyOrObject"/> documentation for "M:Rhino.UI.Localization.LocalizeCommandName(System.String,System.Object)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.UI.Localization.LocalizeCommandName(System.String,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Localization_LocalizeCommandName_1.htm)

#### `public static LocalizeStringPair LocalizeCommandOptionName(string english, int contextId)`



**Parameters:**
- `english` (System.String) — [Missing <param name="english"/> documentation for "M:Rhino.UI.Localization.LocalizeCommandOptionName(System.String,System.Int32)"]
- `contextId` (System.Int32) — [Missing <param name="contextId"/> documentation for "M:Rhino.UI.Localization.LocalizeCommandOptionName(System.String,System.Int32)"]

**Returns:** `LocalizeStringPair` — [Missing <returns> documentation for "M:Rhino.UI.Localization.LocalizeCommandOptionName(System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Localization_LocalizeCommandOptionName.htm)

#### `[ObsoleteAttribute("Don't copy and paste context IDs", true)] public static LocalizeStringPair LocalizeCommandOptionName(string english, int wrongcontextId, int contextId)`

DO NOT use this function, it is a trap to determine where context IDs have been copied from other, already extracted, strings.

**Parameters:**
- `english` (System.String) — The text in English.
- `wrongcontextId` (System.Int32) — The copied ID.
- `contextId` (System.Int32) — The context ID.

**Returns:** `LocalizeStringPair` — The english string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Localization_LocalizeCommandOptionName_1.htm)

#### `public static LocalizeStringPair LocalizeCommandOptionName(string english, Object assemblyOrObject, int contextId)`



**Parameters:**
- `english` (System.String) — [Missing <param name="english"/> documentation for "M:Rhino.UI.Localization.LocalizeCommandOptionName(System.String,System.Object,System.Int32)"]
- `assemblyOrObject` (System.Object) — [Missing <param name="assemblyOrObject"/> documentation for "M:Rhino.UI.Localization.LocalizeCommandOptionName(System.String,System.Object,System.Int32)"]
- `contextId` (System.Int32) — [Missing <param name="contextId"/> documentation for "M:Rhino.UI.Localization.LocalizeCommandOptionName(System.String,System.Object,System.Int32)"]

**Returns:** `LocalizeStringPair` — [Missing <returns> documentation for "M:Rhino.UI.Localization.LocalizeCommandOptionName(System.String,System.Object,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Localization_LocalizeCommandOptionName_2.htm)

#### `public static LocalizeStringPair LocalizeCommandOptionValue(string english, int contextId)`



**Parameters:**
- `english` (System.String) — [Missing <param name="english"/> documentation for "M:Rhino.UI.Localization.LocalizeCommandOptionValue(System.String,System.Int32)"]
- `contextId` (System.Int32) — [Missing <param name="contextId"/> documentation for "M:Rhino.UI.Localization.LocalizeCommandOptionValue(System.String,System.Int32)"]

**Returns:** `LocalizeStringPair` — [Missing <returns> documentation for "M:Rhino.UI.Localization.LocalizeCommandOptionValue(System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Localization_LocalizeCommandOptionValue.htm)

#### `public static LocalizeStringPair LocalizeCommandOptionValue(string english, Object assemblyOrObject, int contextId)`



**Parameters:**
- `english` (System.String) — [Missing <param name="english"/> documentation for "M:Rhino.UI.Localization.LocalizeCommandOptionValue(System.String,System.Object,System.Int32)"]
- `assemblyOrObject` (System.Object) — [Missing <param name="assemblyOrObject"/> documentation for "M:Rhino.UI.Localization.LocalizeCommandOptionValue(System.String,System.Object,System.Int32)"]
- `contextId` (System.Int32) — [Missing <param name="contextId"/> documentation for "M:Rhino.UI.Localization.LocalizeCommandOptionValue(System.String,System.Object,System.Int32)"]

**Returns:** `LocalizeStringPair` — [Missing <returns> documentation for "M:Rhino.UI.Localization.LocalizeCommandOptionValue(System.String,System.Object,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Localization_LocalizeCommandOptionValue_1.htm)

#### `public static string LocalizeDialogItem(Object assemblyOrObject, string key, string english)`

Look in the dialog item list for the specified key and return the translated localized string if the key is found otherwise return the English string.

**Parameters:**
- `assemblyOrObject` (System.Object) — An assembly or an object from an assembly.
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.UI.Localization.LocalizeDialogItem(System.Object,System.String,System.String)"]
- `english` (System.String) — The text in English.

**Returns:** `String` — Look in the dialog item list for the specified key and return the translated localized string if the key is found otherwise return the English string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Localization_LocalizeDialogItem.htm)

#### `public static void LocalizeForm(Object formOrUserControl)`

Look in the dialog item list for the specified key and return the translated localized string if the key is found otherwise return the English string.

**Parameters:**
- `formOrUserControl` (System.Object) — [Missing <param name="formOrUserControl"/> documentation for "M:Rhino.UI.Localization.LocalizeForm(System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Localization_LocalizeForm.htm)

#### `public static string LocalizeString(string english, int contextId)`

Returns localized version of a given English string. This function should be auto-generated by the RmaLDotNetLocalizationProcessor application for every function that uses RMASTR.

**Parameters:**
- `english` (System.String) — The text in English.
- `contextId` (System.Int32) — The context ID.

**Returns:** `String` — The localized string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Localization_LocalizeString.htm)

#### `[ObsoleteAttribute("Don't copy and paste context IDs", true)] public static string LocalizeString(string english, int wrongcontextId, int contextId)`

DO NOT use this function, it is a trap to determine where context IDs have been copied from other, already extracted, strings.

**Parameters:**
- `english` (System.String) — The text in English.
- `wrongcontextId` (System.Int32) — The copied ID.
- `contextId` (System.Int32) — The context ID.

**Returns:** `String` — The english string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Localization_LocalizeString_1.htm)

#### `public static string LocalizeString(string english, Object assemblyOrObject, int contextId)`

Returns localized version of a given English string. This function should be auto-generated by the RmaLDotNetLocalizationProcessor application for every function that uses RMASTR.

**Parameters:**
- `english` (System.String) — The text in English.
- `assemblyOrObject` (System.Object) — An assembly or an object from an assembly.
- `contextId` (System.Int32) — The context ID.

**Returns:** `String` — The localized string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Localization_LocalizeString_2.htm)

#### `public static bool SetLanguageId(int id)`

Sets the Id used for Localization in RhinoCommon. Only useful for when using RhinoCommon outside of the Rhino process

**Parameters:**
- `id` (System.Int32) — [Missing <param name="id"/> documentation for "M:Rhino.UI.Localization.SetLanguageId(System.Int32)"]

**Returns:** `Boolean` — true if the language id could be set

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Localization_SetLanguageId.htm)

#### `public static string UnitSystemName(UnitSystem units, bool capitalize, bool singular, bool abbreviate)`

Gets localized unit system name. Uses current application locale id.

**Parameters:**
- `units` (Rhino.UnitSystem) — The unit system.
- `capitalize` (System.Boolean) — true if the name should be capitalized.
- `singular` (System.Boolean) — true if the name is expressed for a singular element.
- `abbreviate` (System.Boolean) — true if name should be the abbreviation.

**Returns:** `String` — The unit system name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Localization_UnitSystemName.htm)

### Properties
- `CurrentLanguageId` (Int32, get) — 
- `RunningAsEnglish` (Boolean, get) — 

## LocalizeStringPair (class)

Pair of strings used for localization.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_LocalizeStringPair.htm)

### Constructors
- `public LocalizeStringPair(string english, string local)` — Initializes a new instance of the LocalizeStringPair class

### Methods
#### `public static implicit operator string (LocalizeStringPair lcp)`



**Parameters:**
- `lcp` (Rhino.UI.LocalizeStringPair) — [Missing <param name="lcp"/> documentation for "M:Rhino.UI.LocalizeStringPair.op_Implicit(Rhino.UI.LocalizeStringPair)~System.String"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.UI.LocalizeStringPair.op_Implicit(Rhino.UI.LocalizeStringPair)~System.String"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_LocalizeStringPair_op_Implicit.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.UI.LocalizeStringPair.ToString"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_LocalizeStringPair_ToString.htm)

### Properties
- `English` (String, get) — 
- `Local` (String, get) — 

## ModifierKey (enum)

Keyboard keys typically used in combination with other keys

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_ModifierKey.htm)

### Values
- `None` = `0` — No key
- `Control` = `1` — Ctrl key on Windows
- `MacCommand` = `1` — Command key on Mac. This is treated the same as Control key on Windows
- `Shift` = `2` — Shift key
- `Alt` = `4` — Alt key
- `MacControl` = `8` — Control key on Mac

## MouseButton (enum)

[Missing <summary> documentation for "T:Rhino.UI.MouseButton"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_MouseButton.htm)

### Values
- `None` = `0`
- `Left` = `1`
- `Right` = `2`
- `Middle` = `4`

## MouseCallback (class)

Used for intercepting mouse events in the Rhino views.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_MouseCallback.htm)

### Constructors
- `protected MouseCallback()` — Initializes a new instance of the MouseCallback class

### Methods
#### `protected virtual void OnEndMouseDown(MouseCallbackEventArgs e)`

Called at the end of handling of a mouse down event in Rhino. All of the default Rhino mouse down functionality has already been executed unless a MouseCallback has set Cancel to true for the event arguments. You can tell if this is the case by inspecting the Cancel property in the event arguments parameter. Base class implementation of this function does nothing

**Parameters:**
- `e` (Rhino.UI.MouseCallbackEventArgs) — [Missing <param name="e"/> documentation for "M:Rhino.UI.MouseCallback.OnEndMouseDown(Rhino.UI.MouseCallbackEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_MouseCallback_OnEndMouseDown.htm)

#### `protected virtual void OnEndMouseMove(MouseCallbackEventArgs e)`

Called at the end of handling of a mouse move event in Rhino. All of the default Rhino mouse move functionality has already been executed unless a MouseCallback has set Cancel to true for the event arguments. You can tell if this is the case by inspecting the Cancel property in the event arguments parameter. Base class implementation of this function does nothing.

**Parameters:**
- `e` (Rhino.UI.MouseCallbackEventArgs) — [Missing <param name="e"/> documentation for "M:Rhino.UI.MouseCallback.OnEndMouseMove(Rhino.UI.MouseCallbackEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_MouseCallback_OnEndMouseMove.htm)

#### `protected virtual void OnEndMouseUp(MouseCallbackEventArgs e)`

Called at the end of handling of a mouse up event in Rhino. All of the default Rhino mouse down functionality has already been executed unless a MouseCallback has set Cancel to true for the event arguments. You can tell if this is the case by inspecting the Cancel property in the event arguments parameter. Base class implementation of this function does nothing

**Parameters:**
- `e` (Rhino.UI.MouseCallbackEventArgs) — [Missing <param name="e"/> documentation for "M:Rhino.UI.MouseCallback.OnEndMouseUp(Rhino.UI.MouseCallbackEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_MouseCallback_OnEndMouseUp.htm)

#### `protected virtual void OnMouseDoubleClick(MouseCallbackEventArgs e)`



**Parameters:**
- `e` (Rhino.UI.MouseCallbackEventArgs) — [Missing <param name="e"/> documentation for "M:Rhino.UI.MouseCallback.OnMouseDoubleClick(Rhino.UI.MouseCallbackEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_MouseCallback_OnMouseDoubleClick.htm)

#### `protected virtual void OnMouseDown(MouseCallbackEventArgs e)`

Called at the beginning of handling of a mouse down event in Rhino. If you don't want the default Rhino functionality to be run, then set Cancel to true on the passed in event arguments Base class implementation of this function does nothing

**Parameters:**
- `e` (Rhino.UI.MouseCallbackEventArgs) — [Missing <param name="e"/> documentation for "M:Rhino.UI.MouseCallback.OnMouseDown(Rhino.UI.MouseCallbackEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_MouseCallback_OnMouseDown.htm)

#### `protected virtual void OnMouseEnter(MouseCallbackEventArgs e)`



**Parameters:**
- `e` (Rhino.UI.MouseCallbackEventArgs) — [Missing <param name="e"/> documentation for "M:Rhino.UI.MouseCallback.OnMouseEnter(Rhino.UI.MouseCallbackEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_MouseCallback_OnMouseEnter.htm)

#### `protected virtual void OnMouseHover(MouseCallbackEventArgs e)`



**Parameters:**
- `e` (Rhino.UI.MouseCallbackEventArgs) — [Missing <param name="e"/> documentation for "M:Rhino.UI.MouseCallback.OnMouseHover(Rhino.UI.MouseCallbackEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_MouseCallback_OnMouseHover.htm)

#### `protected virtual void OnMouseLeave(MouseCallbackEventArgs e)`



**Parameters:**
- `e` (Rhino.UI.MouseCallbackEventArgs) — [Missing <param name="e"/> documentation for "M:Rhino.UI.MouseCallback.OnMouseLeave(Rhino.UI.MouseCallbackEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_MouseCallback_OnMouseLeave.htm)

#### `protected virtual void OnMouseMove(MouseCallbackEventArgs e)`

Called at the beginning of handling of a mouse move event in Rhino. If you don't want the default Rhino functionality to be run, then set Cancel to true on the passed in event arguments. Base class implementation of this function does nothing.

**Parameters:**
- `e` (Rhino.UI.MouseCallbackEventArgs) — [Missing <param name="e"/> documentation for "M:Rhino.UI.MouseCallback.OnMouseMove(Rhino.UI.MouseCallbackEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_MouseCallback_OnMouseMove.htm)

#### `protected virtual void OnMouseUp(MouseCallbackEventArgs e)`

Called at the beginning of handling of a mouse up event in Rhino. If you don't want the default Rhino functionality to be run, then set Cancel to true on the passed in event arguments Base class implementation of this function does nothing

**Parameters:**
- `e` (Rhino.UI.MouseCallbackEventArgs) — [Missing <param name="e"/> documentation for "M:Rhino.UI.MouseCallback.OnMouseUp(Rhino.UI.MouseCallbackEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_MouseCallback_OnMouseUp.htm)

### Properties
- `Enabled` (Boolean, get/set) — 

## MouseCallbackEventArgs (class)

[Missing <summary> documentation for "T:Rhino.UI.MouseCallbackEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_MouseCallbackEventArgs.htm)

### Methods
#### `public GumballMode IsOverGumball()`



**Returns:** `GumballMode` — [Missing <returns> documentation for "M:Rhino.UI.MouseCallbackEventArgs.IsOverGumball"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_MouseCallbackEventArgs_IsOverGumball.htm)

### Properties
- `CtrlKeyDown` (Boolean, get) — 
- `MouseButton` (MouseButton, get) — 
- `ShiftKeyDown` (Boolean, get) — 
- `View` (RhinoView, get) — 
- `ViewportPoint` (Point, get) — 

## MouseCursor (class)

Contains static methods to control the mouse icon.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_MouseCursor.htm)

### Methods
#### `public static void SetToolTip(string tooltip)`

Sets a cursor tooltip string shown next to the mouse cursor. Overrides all cursor tooltip panes.

**Parameters:**
- `tooltip` (System.String) — The text to show.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_MouseCursor_SetToolTip.htm)

### Properties
- `Location` (Point2d, get) — Retrieves the position of the mouse cursor, in screen coordinates

## NamedColor (class)

An entry with name and color for the NamedColorList

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_NamedColor.htm)

### Constructors
- `public NamedColor(string name, Color color)` — Initializes a new instance of the ColorListEntry with the specified name and color.

### Properties
- `Color` (Color, get) — Gets the color for this entry.
- `Name` (String, get) — Gets the name of this color to show to the user.

## NamedColorList (class)

A list of color name/value pairs.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_NamedColorList.htm)

### Constructors
- `public NamedColorList(string name)` — Initializes a new instance of the ColorList with the specified name.
- `public NamedColorList(string name, IEnumerable<NamedColor> entries)` — Initializes a new instance of the ColorList with the specified name and entries.

### Properties
- `Default` (NamedColorList, get) — Get the Rhino applications named color list.
- `Name` (String, get/set) — The name of the color list, which may be displayed to the user.

## ObjectPropertiesPage (class)

Base class used to add object property user interface panels

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_ObjectPropertiesPage.htm)

### Constructors
- `protected ObjectPropertiesPage()` — Initializes a new instance of the ObjectPropertiesPage class

### Methods
#### `public bool AnySelectedObject<T>() where T : RhinoObject`

Return true if any of the selected objects match the given type

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.ObjectPropertiesPage.AnySelectedObject``1"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ObjectPropertiesPage_AnySelectedObject__1.htm)

#### `public bool AnySelectedObject<T>(bool allMustMatch) where T : RhinoObject`

Return true if any of the selected objects match the given type

**Parameters:**
- `allMustMatch` (System.Boolean) — If true then every selected object must match the object type otherwise; only a single object has to be of the specified type

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.ObjectPropertiesPage.AnySelectedObject``1(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ObjectPropertiesPage_AnySelectedObject__1_1.htm)

#### `public RhinoObject[] GetSelectedObjects(ObjectType filter)`

Get selected objects that match a given filter

**Parameters:**
- `filter` (Rhino.DocObjects.ObjectType) — [Missing <param name="filter"/> documentation for "M:Rhino.UI.ObjectPropertiesPage.GetSelectedObjects(Rhino.DocObjects.ObjectType)"]

**Returns:** `RhinoObject[]` — [Missing <returns> documentation for "M:Rhino.UI.ObjectPropertiesPage.GetSelectedObjects(Rhino.DocObjects.ObjectType)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ObjectPropertiesPage_GetSelectedObjects.htm)

#### `public T[] GetSelectedObjects<T>() where T : RhinoObject`

Get selected objects of a given type

**Returns:** `T[]` — [Missing <returns> documentation for "M:Rhino.UI.ObjectPropertiesPage.GetSelectedObjects``1"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ObjectPropertiesPage_GetSelectedObjects__1.htm)

#### `[ObsoleteAttribute("InitializeControls is obsolete, override UpdatePage instead")] public virtual void InitializeControls(RhinoObject rhObj)`

Called on the active page after the selected objects list has changed to notify the page to initialize its content to reflect the new object list.

**Parameters:**
- `rhObj` (Rhino.DocObjects.RhinoObject) — [Missing <param name="rhObj"/> documentation for "M:Rhino.UI.ObjectPropertiesPage.InitializeControls(Rhino.DocObjects.RhinoObject)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ObjectPropertiesPage_InitializeControls.htm)

#### `public void ModifyPage(Action<ObjectPropertiesPageEventArgs> callbackAction)`

Call this method when the page is ready to modify the selected objects list. Rhino will suspend UpdatePageNotfictaion, call the passed action then restore UpdatePageNotfictaion.

**Parameters:**
- `callbackAction` (System.Action<ObjectPropertiesPageEventArgs>) — Called when it is safe to modify objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ObjectPropertiesPage_ModifyPage.htm)

#### `public virtual bool OnActivate(bool active)`

Called when this page is activated/deactivated.

**Parameters:**
- `active` (System.Boolean) — If true then this page is on top otherwise it is about to be hidden.

**Returns:** `Boolean` — If true then the page is hidden and the requested page is not activated otherwise will not allow you to change the current page. Default returns true. The return value is currently ignored.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ObjectPropertiesPage_OnActivate.htm)

#### `public virtual void OnCreateParent(IntPtr hwndParent)`

Called when the parent container is initially created.

**Parameters:**
- `hwndParent` (System.IntPtr) — [Missing <param name="hwndParent"/> documentation for "M:Rhino.UI.ObjectPropertiesPage.OnCreateParent(System.IntPtr)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ObjectPropertiesPage_OnCreateParent.htm)

#### `public virtual void OnHelp()`

Called when the F1 key or help button is pressed, override to display plug-in specific help for this page.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ObjectPropertiesPage_OnHelp.htm)

#### `public virtual void OnSizeParent(int width, int height)`

Called when the parent containers client rectangle size has changed and the PageControl has been resized.

**Parameters:**
- `width` (System.Int32) — [Missing <param name="width"/> documentation for "M:Rhino.UI.ObjectPropertiesPage.OnSizeParent(System.Int32,System.Int32)"]
- `height` (System.Int32) — [Missing <param name="height"/> documentation for "M:Rhino.UI.ObjectPropertiesPage.OnSizeParent(System.Int32,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ObjectPropertiesPage_OnSizeParent.htm)

#### `public virtual Icon PageIcon(Size sizeInPixels)`

Icon to display in the object properties tab control. Will not get called if PageIconEmbeddedResourceString is overridden and provides a string for a successfully loaded icon resource.

**Parameters:**
- `sizeInPixels` (System.Drawing.Size) — The requested icon size in pixels, DPI scaling has been applied. The default value is 24 X DPI scale.

**Returns:** `Icon` — [Missing <returns> documentation for "M:Rhino.UI.ObjectPropertiesPage.PageIcon(System.Drawing.Size)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ObjectPropertiesPage_PageIcon.htm)

#### `public virtual Result RunScript(ObjectPropertiesPageEventArgs e)`

This method is called when scripting the Rhino Properties command and choosing this page.

**Parameters:**
- `e` (Rhino.UI.ObjectPropertiesPageEventArgs) — Provides access to the selected object list and document.

**Returns:** `Result` — [Missing <returns> documentation for "M:Rhino.UI.ObjectPropertiesPage.RunScript(Rhino.UI.ObjectPropertiesPageEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ObjectPropertiesPage_RunScript_1.htm)

#### `[ObsoleteAttribute("RunScript(RhinoDoc doc, RhinoObject[] objectList) is obsolete, override RunScript(ObjectPropertiesPageEventArgs e) instead")] public virtual Result RunScript(RhinoDoc doc, RhinoObject[] objectList)`

This method is called when scripting the Rhino Properties command and choosing this page.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — Active RhinoDoc
- `objectList` (Rhino.DocObjects.RhinoObject[]) — List of objects selected by the Properties command.

**Returns:** `Result` — [Missing <returns> documentation for "M:Rhino.UI.ObjectPropertiesPage.RunScript(Rhino.RhinoDoc,Rhino.DocObjects.RhinoObject[])"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ObjectPropertiesPage_RunScript.htm)

#### `public virtual bool ShouldDisplay(ObjectPropertiesPageEventArgs e)`

Called when the selected objects list changes, return true if the object list contains one or more object the page can modify.

**Parameters:**
- `e` (Rhino.UI.ObjectPropertiesPageEventArgs) — [Missing <param name="e"/> documentation for "M:Rhino.UI.ObjectPropertiesPage.ShouldDisplay(Rhino.UI.ObjectPropertiesPageEventArgs)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.ObjectPropertiesPage.ShouldDisplay(Rhino.UI.ObjectPropertiesPageEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ObjectPropertiesPage_ShouldDisplay_1.htm)

#### `[ObsoleteAttribute("ShouldDisplay(RhinoObject rhObj) is obsolete, override ShouldDisplay(ObjectPropertiesPageEventArgs e) instead")] public virtual bool ShouldDisplay(RhinoObject rhObj)`

Called when the selected objects list changes, return true if the object list contains one or more object the page can modify.

**Parameters:**
- `rhObj` (Rhino.DocObjects.RhinoObject) — [Missing <param name="rhObj"/> documentation for "M:Rhino.UI.ObjectPropertiesPage.ShouldDisplay(Rhino.DocObjects.RhinoObject)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.ObjectPropertiesPage.ShouldDisplay(Rhino.DocObjects.RhinoObject)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ObjectPropertiesPage_ShouldDisplay.htm)

#### `public virtual void UpdatePage(ObjectPropertiesPageEventArgs e)`

Called on the active page after the selected objects list has changed to notify the page to initialize its content to reflect the new object list.

**Parameters:**
- `e` (Rhino.UI.ObjectPropertiesPageEventArgs) — [Missing <param name="e"/> documentation for "M:Rhino.UI.ObjectPropertiesPage.UpdatePage(Rhino.UI.ObjectPropertiesPageEventArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ObjectPropertiesPage_UpdatePage.htm)

### Properties
- `AllObjectsMustBeSupported` (Boolean, get) — Returns false which only requires a single item of SupportedTypes to be selected. Override if you wish to change the above behavior.
- `EnglishPageTitle` (String, get) — English string used to describe this page
- `Icon` (Icon, get) — (OBSOLETE - Override PageIcon instead) Icon to display in the object properties tab control
- `Index` (Int32, get) — The page navigation control adds buttons in the order the pages are processed, override this method and return a sort index to move the button to the beginning of the list. By default this returns -1 which puts the button at the end of the list.
- `LocalPageTitle` (String, get) — Localized page description string, returns the EnglishPageTitle by default.
- `PageControl` (Object, get) — The control that represents this page. Rhino Windows supports classes that implement the IWin32Windows interface, are derived from System.Windows.FrameworkElement or Eto.Forms.Control. Mac Rhino supports controls that are derived from NSview or Eto.Forms.Control.
- `PageIconEmbeddedResourceString` (String, get) — Resource string for a embedded icon resource in the assembly containing the page instance. If this returns a valid resource and Rhino can load the icon the loaded icon will get used directly otherwise; the PageIcon method will get called.
- `PageType` (PropertyPageType, get) — Override this and return the page you want to replace a specific object properties page.
- `SelectedObjects` (RhinoObject[], get) — Return a list of Rhino objects to be processed by this object properties page
- `SupportedTypes` (ObjectType, get) — Override to specify which objects this page supports
- `SupportsSubObjects` (Boolean, get) — If your object properties page supports sub-object selection, you should override this method and return true. This is ignored for view pages. The default implementation returns false.

## ObjectPropertiesPageCollection (class)

Passed to Rhino.PlugIns.PlugIn.ObjectPropertiesPages to allow a plug-in to add custom ObjectPropertiesPage pages to the Rhino properties panel.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_ObjectPropertiesPageCollection.htm)

### Methods
#### `public void Add(ObjectPropertiesPage page)`

Custom page to add

**Parameters:**
- `page` (Rhino.UI.ObjectPropertiesPage) — [Missing <param name="page"/> documentation for "M:Rhino.UI.ObjectPropertiesPageCollection.Add(Rhino.UI.ObjectPropertiesPage)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ObjectPropertiesPageCollection_Add.htm)

### Properties
- `Document` (RhinoDoc, get) — Document associated with the Rhino properties panel.
- `DocumentRuntimeSerialNumber` (UInt32, get) — Document associated with the Rhino properties panel.

## ObjectPropertiesPageEventArgs (class)

[Missing <summary> documentation for "T:Rhino.UI.ObjectPropertiesPageEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_ObjectPropertiesPageEventArgs.htm)

### Constructors
- `public ObjectPropertiesPageEventArgs(ObjectPropertiesPage page)` — Used by ObjectPropertiesPage to notify the page when updating, modifying or determining if the page should be included in the navigation bar

### Methods
#### `public RhinoObject[] GetObjects(ObjectType filter)`

Get selected objects that match a given filter

**Parameters:**
- `filter` (Rhino.DocObjects.ObjectType) — [Missing <param name="filter"/> documentation for "M:Rhino.UI.ObjectPropertiesPageEventArgs.GetObjects(Rhino.DocObjects.ObjectType)"]

**Returns:** `RhinoObject[]` — [Missing <returns> documentation for "M:Rhino.UI.ObjectPropertiesPageEventArgs.GetObjects(Rhino.DocObjects.ObjectType)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ObjectPropertiesPageEventArgs_GetObjects.htm)

#### `public T[] GetObjects<T>() where T : RhinoObject`

Get selected objects of a given type

**Returns:** `T[]` — [Missing <returns> documentation for "M:Rhino.UI.ObjectPropertiesPageEventArgs.GetObjects``1"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ObjectPropertiesPageEventArgs_GetObjects__1.htm)

#### `public bool IncludesObjectsType(ObjectType objectTypes)`



**Parameters:**
- `objectTypes` (Rhino.DocObjects.ObjectType) — [Missing <param name="objectTypes"/> documentation for "M:Rhino.UI.ObjectPropertiesPageEventArgs.IncludesObjectsType(Rhino.DocObjects.ObjectType)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.ObjectPropertiesPageEventArgs.IncludesObjectsType(Rhino.DocObjects.ObjectType)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ObjectPropertiesPageEventArgs_IncludesObjectsType.htm)

#### `public bool IncludesObjectsType(ObjectType objectTypes, bool allMustMatch)`



**Parameters:**
- `objectTypes` (Rhino.DocObjects.ObjectType) — [Missing <param name="objectTypes"/> documentation for "M:Rhino.UI.ObjectPropertiesPageEventArgs.IncludesObjectsType(Rhino.DocObjects.ObjectType,System.Boolean)"]
- `allMustMatch` (System.Boolean) — If true then every selected object must match the object type otherwise; only a single object has to be of the specified type

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.ObjectPropertiesPageEventArgs.IncludesObjectsType(Rhino.DocObjects.ObjectType,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ObjectPropertiesPageEventArgs_IncludesObjectsType_1.htm)

#### `public bool IncludesObjectsType<T>() where T : RhinoObject`

Return true if any of the selected objects match the given type

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.ObjectPropertiesPageEventArgs.IncludesObjectsType``1"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ObjectPropertiesPageEventArgs_IncludesObjectsType__1.htm)

#### `public bool IncludesObjectsType<T>(bool allMustMatch) where T : RhinoObject`

Return true if any of the selected objects match the given type

**Parameters:**
- `allMustMatch` (System.Boolean) — If true then every selected object must match the object type otherwise; only a single object has to be of the specified type

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.ObjectPropertiesPageEventArgs.IncludesObjectsType``1(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ObjectPropertiesPageEventArgs_IncludesObjectsType__1_1.htm)

### Properties
- `DocRuntimeSerialNumber` (UInt32, get) — Document containing the objects and views
- `Document` (RhinoDoc, get) — Document containing the objects and views
- `EventRuntimeSerialNumber` (UInt32, get) — Gets the runtime serial number.
- `ObjectCount` (Int32, get) — 
- `Objects` (RhinoObject[], get) — Return a list of Rhino objects to be processed by this object properties page
- `ObjectTypes` (UInt32, get) — 
- `Page` (ObjectPropertiesPage, get) — The page sending these arguments
- `View` (RhinoView, get) — Active view
- `Viewport` (RhinoViewport, get) — Active viewport

## OpenFileDialog (class)

Similar to the System.Windows.Forms.OpenFileDialog, but with customized Rhino user interface.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_OpenFileDialog.htm)

### Constructors
- `public OpenFileDialog()` — Create a new open file dialog.

### Methods
#### `[ObsoleteAttribute("Use ShowOpenDialog")] public DialogResult ShowDialog()`

Obsolete.

**Returns:** `DialogResult` — [Missing <returns> documentation for "M:Rhino.UI.OpenFileDialog.ShowDialog"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_OpenFileDialog_ShowDialog.htm)

#### `public bool ShowOpenDialog()`

Show the actual dialog to allow the user to select a file.

**Returns:** `Boolean` — true if a file was selected. false if the dialog was canceled

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_OpenFileDialog_ShowOpenDialog.htm)

### Properties
- `DefaultExt` (String, get/set) — The default file name extension. The returned string does not include the period.
- `FileName` (String, get/set) — Gets or sets a string containing the file name selected in the file dialog box.
- `FileNames` (String[], get) — Gets the names of all of the selected files in the dialog box
- `Filter` (String, get/set) — Gets or sets the current file name filter string, which determines the choices that appear in the "Save as file type" or "Files of type" box in the dialog box. See System.Windows.Forms.FileDialog for details.
- `InitialDirectory` (String, get/set) — Gets or sets the initial directory displayed by the file dialog box.
- `MultiSelect` (Boolean, get/set) — Gets or sets a value indicating whether the dialog box allows multiple files to be selected
- `Title` (String, get/set) — Gets or sets the file dialog box title.

## OptionPageButtons (enum)

Standard IRhinoOptionsPageButton

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_OptionPageButtons.htm)

### Values
- `None` = `0` — Don't display any of the standard buttons.
- `DefaultButton` = `1` — The "Restore Defaults" button located at the bottom of the host.
- `ApplyButton` = `2` — The "Apply" button located at the bottom of the host.

## OptionsDialogPage (class)

[Missing <summary> documentation for "T:Rhino.UI.OptionsDialogPage"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_OptionsDialogPage.htm)

### Constructors
- `protected OptionsDialogPage(string englishPageTitle)` — Initializes a new instance of the OptionsDialogPage class

### Methods
#### `public void AddChildPage(StackedDialogPage pageToAdd)`

Currently only supported on Windows. Call this method to add a child page to a page after the parent dialog has been created.

**Parameters:**
- `pageToAdd` (Rhino.UI.StackedDialogPage) — [Missing <param name="pageToAdd"/> documentation for "M:Rhino.UI.StackedDialogPage.AddChildPage(Rhino.UI.StackedDialogPage)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_AddChildPage.htm)

#### `public void MakeActivePage()`

Make this page the active, visible page

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_MakeActivePage.htm)

#### `public virtual bool OnActivate(bool active)`

Called when this page is activated/deactivated.

**Parameters:**
- `active` (System.Boolean) — If true then this page is on top otherwise it is about to be hidden.

**Returns:** `Boolean` — If true then the page is hidden and the requested page is not activated otherwise will not allow you to change the current page. Default returns true

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_OnActivate.htm)

#### `public virtual bool OnApply()`

Called when stacked dialog OK button is pressed.

**Returns:** `Boolean` — If return value is true then the dialog will be closed. A return of false means there was an error and dialog remains open so page can be properly updated.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_OnApply.htm)

#### `public virtual void OnCancel()`

Called when stacked dialog Cancel button is pressed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_OnCancel.htm)

#### `public virtual void OnCreateParent(IntPtr hwndParent)`

Called when the parent window has been created on Windows platforms only.

**Parameters:**
- `hwndParent` (System.IntPtr) — [Missing <param name="hwndParent"/> documentation for "M:Rhino.UI.StackedDialogPage.OnCreateParent(System.IntPtr)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_OnCreateParent.htm)

#### `public virtual void OnDefaults()`

Called when stacked dialog Defaults button is pressed (see ShowDefaultsButton).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_OnDefaults.htm)

#### `public virtual void OnHelp()`

Called when the parent dialog requests help for this page.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_OnHelp.htm)

#### `public virtual void OnSizeParent(int width, int height)`

Called when the parent window has been resized

**Parameters:**
- `width` (System.Int32) — [Missing <param name="width"/> documentation for "M:Rhino.UI.StackedDialogPage.OnSizeParent(System.Int32,System.Int32)"]
- `height` (System.Int32) — [Missing <param name="height"/> documentation for "M:Rhino.UI.StackedDialogPage.OnSizeParent(System.Int32,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_OnSizeParent.htm)

#### `public void RemovePage()`

Remove this page from the dialog box

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_RemovePage.htm)

#### `public virtual Result RunScript(RhinoDoc doc, RunMode mode)`



**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.UI.OptionsDialogPage.RunScript(Rhino.RhinoDoc,Rhino.Commands.RunMode)"]
- `mode` (Rhino.Commands.RunMode) — [Missing <param name="mode"/> documentation for "M:Rhino.UI.OptionsDialogPage.RunScript(Rhino.RhinoDoc,Rhino.Commands.RunMode)"]

**Returns:** `Result` — [Missing <returns> documentation for "M:Rhino.UI.OptionsDialogPage.RunScript(Rhino.RhinoDoc,Rhino.Commands.RunMode)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_OptionsDialogPage_RunScript.htm)

#### `public bool SetActivePageTo(string pageName, bool documentPropertiesPage)`

(Inherited from StackedDialogPage.)

**Parameters:**
- `pageName` (System.String) — [Missing <param name="pageName"/> documentation for "M:Rhino.UI.StackedDialogPage.SetActivePageTo(System.String,System.Boolean)"]
- `documentPropertiesPage` (System.Boolean) — [Missing <param name="documentPropertiesPage"/> documentation for "M:Rhino.UI.StackedDialogPage.SetActivePageTo(System.String,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.StackedDialogPage.SetActivePageTo(System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_SetActivePageTo.htm)

#### `public void SetEnglishPageTitle(string newPageTile)`

Change the title passed to the constructor and, this will cause LocalPageTitle to get called also.

**Parameters:**
- `newPageTile` (System.String) — [Missing <param name="newPageTile"/> documentation for "M:Rhino.UI.StackedDialogPage.SetEnglishPageTitle(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_SetEnglishPageTitle.htm)

### Properties
- `Children` (List<StackedDialogPage>, get) — List of child (sub) pages of this page
- `EnglishPageTitle` (String, get) — English string used when scripting this page and a user prefixes a command option with "_"
- `Handle` (IntPtr, get) — When running on Windows return the window handle for the parent of this page otherwise; return IntPtr.Zero.
- `HasChildren` (Boolean, get) — Will be true if this page contains sub pages.
- `LocalPageTitle` (String, get) — Localized page title that will appear on a tab or other page navigation control. This is also uses as a command line option when scripting this page.
- `Modified` (Boolean, get/set) — Check to see if the page has been marked as modified or not. Marking the page as modified will cause the Apply button to get enabled if this is currently the visible page and the page includes the Apply button.
- `NavigationTextColor` (Color, get/set) — Currently only used by Windows Rhino. If this is set to true then the tree control item text be drawn using this color. Set the color to System.Drawing.Color.Empty to use the default color.
- `NavigationTextIsBold` (Boolean, get/set) — Currently only used by Windows Rhino. If this is set to true then the tree control item text will be bold.
- `OptionsPageType` (OptionsDialogPage.PageType, get) — 17 March 2021 John Morse For internal use in determining the page type. RhinoMac uses this to ensure pages are sized properly when hosting them.
- `PageControl` (Object, get) — Return the control that represents this page. Rhino Windows supports classes that implement the IWin32Windows interface or are derived from some form of System.Windows.FrameworkElement or Eto.Forms.Control. Mac Rhino supports controls that are derived from NSview or Eto.Forms.Control.
- `PageImage` (Image, get) — Optionally override to provide a image to display in the Mac Rhino UI
- `ShowApplyButton` (Boolean, get) — Called when this page is activated
- `ShowDefaultsButton` (Boolean, get) — Called when this page is activated.

## OptionsDialogPage.PageType (enum)

17 March 2021 John Morse For internal use in determining the page type. RhinoMac uses this to ensure pages are sized properly when hosting them.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_OptionsDialogPage_PageType.htm)

### Values
- `Options` = `0`
- `DocumentProperties` = `1`

## PanelEventArgs (class)

Panels.Show event arguments

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_PanelEventArgs.htm)

### Constructors
- `public PanelEventArgs(Guid panelId, uint documentSerialNumber)` — Initializes a new instance of the PanelEventArgs class

### Properties
- `Document` (RhinoDoc, get) — 
- `DocumentSerialNumber` (UInt32, get) — 
- `PanelId` (Guid, get) — Class Id for panel being shown or hidden

## PanelIds (class)

Standard Rhino panel ids.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_PanelIds.htm)

### Properties
- `BoxEdit` (Guid, get) — Rhino BoxEdit panel.
- `ContextHelp` (Guid, get) — Rhino Context-Sensitive Help panel.
- `Display` (Guid, get) — Rhino Display Properties panel.
- `Environment` (Guid, get) — Rhino Environment panel.
- `FileExplorer` (Guid, get) — Rhino File Explorer panel.
- `GroundPlane` (Guid, get) — Rhino Ground Plane panel.
- `Layers` (Guid, get) — Rhino Layer panel.
- `Libraries` (Guid, get) — Rhino Render Libraries panel.
- `LightManager` (Guid, get) — Rhino Lights panel.
- `Materials` (Guid, get) — Rhino Materials panel.
- `Notes` (Guid, get) — Rhino Notes panel.
- `ObjectProperties` (Guid, get) — Rhino Object Properties panel.
- `Rendering` (Guid, get) — Rhino Rendering Properties panel.
- `Sun` (Guid, get) — Rhino Sun panel.
- `Texture` (Guid, get) — Rhino Texture panel.

## PanelType (enum)

Panel type

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_PanelType.htm)

### Values
- `PerDoc` = `0` — Default panel type, creates a panel instance per document
- `System` = `1` — A System panel may appear in one or more container but the panel will be used for all open documents

## Panels (class)

Access to Rhino panels and register custom panels

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_Panels.htm)

### Methods
#### `public static void ChangePanelIcon(Type panelType, Icon icon)`

Update the icon used for a panel tab.

**Parameters:**
- `panelType` (System.Type) — [Missing <param name="panelType"/> documentation for "M:Rhino.UI.Panels.ChangePanelIcon(System.Type,System.Drawing.Icon)"]
- `icon` (System.Drawing.Icon) — New icon to use

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_ChangePanelIcon.htm)

#### `public static void ChangePanelIcon(Type panelType, string fullPathToResource)`

Update the icon used for a panel tab.

**Parameters:**
- `panelType` (System.Type) — [Missing <param name="panelType"/> documentation for "M:Rhino.UI.Panels.ChangePanelIcon(System.Type,System.String)"]
- `fullPathToResource` (System.String) — Full path to the new icon resource

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_ChangePanelIcon_1.htm)

#### `public static void ClosePanel(Guid panelId)`

Will close or hide the specified panel type, in Windows Rhino, if it is the only visible tab the tab dock bar will be closed as well. In Mac Rhino it will always close the modeless dialog hosting the panel.

**Parameters:**
- `panelId` (System.Guid) — Class type Id of the panel to close.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_ClosePanel.htm)

#### `public static void ClosePanel(Guid panelId, RhinoDoc doc)`

Will close or hide the specified panel type, in Windows Rhino, if it is the only visible tab the tab dock bar will be closed as well. In Mac Rhino it will always close the modeless dialog hosting the panel.

**Parameters:**
- `panelId` (System.Guid) — Class type Id of the panel to close.
- `doc` (Rhino.RhinoDoc) — Document associated with panel you wish to close.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_ClosePanel_1.htm)

#### `public static void ClosePanel(Type panelType)`

Will close or hide the specified panel type, in Windows Rhino, if it is the only visible tab the tab dock bar will be closed as well. In Mac Rhino it will always close the modeless dialog hosting the panel.

**Parameters:**
- `panelType` (System.Type) — Class type of the panel to close.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_ClosePanel_2.htm)

#### `public static void ClosePanel(Type panelType, RhinoDoc doc)`

Will close or hide the specified panel type, in Windows Rhino, if it is the only visible tab the tab dock bar will be closed as well. In Mac Rhino it will always close the modeless dialog hosting the panel.

**Parameters:**
- `panelType` (System.Type) — Class type of the panel to close.
- `doc` (Rhino.RhinoDoc) — Document associated with panel you wish to close.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_ClosePanel_3.htm)

#### `public static bool DockBarIdInUse(Guid dockBarId)`

For internal use, call this method to see if a dock bar Id is currently being used by any Rhino dock bar. Rhino

**Parameters:**
- `dockBarId` (System.Guid) — [Missing <param name="dockBarId"/> documentation for "M:Rhino.UI.Panels.DockBarIdInUse(System.Guid)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.Panels.DockBarIdInUse(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_DockBarIdInUse.htm)

#### `public static bool FloatPanel(Guid panelTypeId, Panels.FloatPanelMode mode)`

Mac support: Display the specified panel in a floating window on Mac, the floating window will only contain the specified panel. Windows support: On Windows this will show or hide the floating container containing the specified panel. If the tab is docked with other tabs it will be floated in a new container.

**Parameters:**
- `panelTypeId` (System.Guid) — Guid for the panel type to show/hide.
- `mode` (Rhino.UI.Panels.FloatPanelMode) — Show, hide or toggle the visibility state of the specified panel

**Returns:** `Boolean` — Return true if the panel visibility state was changed, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_FloatPanel.htm)

#### `public static bool FloatPanel(Type panelType, Panels.FloatPanelMode mode)`

Mac support: Display the specified panel in a floating window on Mac, the floating window will only contain the specified panel. Windows support: On Windows this will show or hide the floating container containing the specified panel. If the tab is docked with other tabs it will be floated in a new container.

**Parameters:**
- `panelType` (System.Type) — Panel type to show/hide.
- `mode` (Rhino.UI.Panels.FloatPanelMode) — Show, hide or toggle the visibility state of the specified panel

**Returns:** `Boolean` — Return true if the panel visibility state was changed, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_FloatPanel_1.htm)

#### `public static Guid[] GetOpenPanelIds()`

Get a list of the currently open panel tabs in Windows Rhino, on Mac Rhino it will be a list of the currently visible modeless panel dialogs.

**Returns:** `Guid[]` — Returns an array of panel class Id's for the currently open panels, if there are no open panels it will be an empty array.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_GetOpenPanelIds.htm)

#### `[ObsoleteAttribute("Obsolete method, use GetPanel<MyClass>(RhinoDoc)")] public static Object GetPanel(Guid panelId)`

Will return an instance of a .Net panel if the panel has been displayed at least once. Panel instances are not created until a panel is displayed.

**Parameters:**
- `panelId` (System.Guid) — Class Id of the panel to search for.

**Returns:** `Object` — Returns the one and only instance of a panel if it has been properly registered and displayed at least once. If the panel has never been displayed then null will be returned even if the panel is properly registered.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_GetPanel.htm)

#### `public static Object GetPanel(Guid panelId, RhinoDoc rhinoDoc)`

Will return an instance of a .Net panel if the panel has been displayed at least once. Panel instances are not created until a panel is displayed.

**Parameters:**
- `panelId` (System.Guid) — Class Id of the panel to search for.
- `rhinoDoc` (Rhino.RhinoDoc) — Runtime document Id associated with the requested panel.

**Returns:** `Object` — Returns the one and only instance of a panel if it has been properly registered and displayed at least once. If the panel has never been displayed then null will be returned even if the panel is properly registered.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_GetPanel_1.htm)

#### `public static Object GetPanel(Guid panelId, uint documentSerialNumber)`

Will return an instance of a .Net panel if the panel has been displayed at least once. Panel instances are not created until a panel is displayed.

**Parameters:**
- `panelId` (System.Guid) — Class Id of the panel to search for.
- `documentSerialNumber` (System.UInt32) — Runtime document Id associated with the requested panel.

**Returns:** `Object` — Returns the one and only instance of a panel if it has been properly registered and displayed at least once. If the panel has never been displayed then null will be returned even if the panel is properly registered.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_GetPanel_2.htm)

#### `[ObsoleteAttribute("Obsolete method, use GetPanel<MyClass>(RhinoDoc)")] public static T GetPanel<T>() where T : class`

Return an instance of a .Net panel

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.UI.Panels.GetPanel``1"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_GetPanel__1.htm)

#### `public static T GetPanel<T>(RhinoDoc rhinoDoc) where T : class`

Return an instance of a .Net panel

**Parameters:**
- `rhinoDoc` (Rhino.RhinoDoc) — Runtime document Id associated with the requested panel.

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.UI.Panels.GetPanel``1(Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_GetPanel__1_1.htm)

#### `public static T GetPanel<T>(uint documentSerialNumber) where T : class`

Return an instance of a .Net panel

**Parameters:**
- `documentSerialNumber` (System.UInt32) — Runtime document Id associated with the requested panel.

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.UI.Panels.GetPanel``1(System.UInt32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_GetPanel__1_2.htm)

#### `public static Object[] GetPanels(Guid panelId, RhinoDoc doc)`

Gets the panels.

**Parameters:**
- `panelId` (System.Guid) — Panel identifier.
- `doc` (Rhino.RhinoDoc) — Document.

**Returns:** `Object[]` — The panels.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_GetPanels.htm)

#### `public static Object[] GetPanels(Guid panelId, uint documentRuntimeSerialNumber)`

Gets the panels.

**Parameters:**
- `panelId` (System.Guid) — Panel identifier.
- `documentRuntimeSerialNumber` (System.UInt32) — Document runtime serial number.

**Returns:** `Object[]` — The panels.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_GetPanels_1.htm)

#### `public static T[] GetPanels<T>(RhinoDoc doc) where T : class`



**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.UI.Panels.GetPanels``1(Rhino.RhinoDoc)"]

**Returns:** `T[]` — [Missing <returns> documentation for "M:Rhino.UI.Panels.GetPanels``1(Rhino.RhinoDoc)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_GetPanels__1.htm)

#### `public static T[] GetPanels<T>(uint documentRuntimeSerialNumber) where T : class`

Gets the panels.

**Parameters:**
- `documentRuntimeSerialNumber` (System.UInt32) — Document runtime serial number.

**Returns:** `T[]` — The panels.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_GetPanels__1_1.htm)

#### `public static bool IsHiding(ShowPanelReason reason)`

Check to see if reason is equal to any of the show events

**Parameters:**
- `reason` (Rhino.UI.ShowPanelReason) — [Missing <param name="reason"/> documentation for "M:Rhino.UI.Panels.IsHiding(Rhino.UI.ShowPanelReason)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.Panels.IsHiding(Rhino.UI.ShowPanelReason)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_IsHiding.htm)

#### `public static bool IsPanelVisible(Guid panelId)`

Check to see if a panel is currently visible, on Windows this means you can see the tab, it does not necessarily mean it is the current tab.

**Parameters:**
- `panelId` (System.Guid) — Class Id for the panel to check.

**Returns:** `Boolean` — Returns true if the tab is visible otherwise it returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_IsPanelVisible.htm)

#### `public static bool IsPanelVisible(Guid panelId, bool isSelectedTab)`

Check to see if a panel is currently visible, if isSelectedTab is true then the tab must be the active tab as well.

**Parameters:**
- `panelId` (System.Guid) — Class Id for the panel to check.
- `isSelectedTab` (System.Boolean) — This parameter is ignored on Mac. If Windows and true the panel must be visible in a container and if it is a tabbed container it must be the active tab to be true.

**Returns:** `Boolean` — On Windows: The return value is dependent on the isSelectedTab value. If isSelectedTab is true then the panel must be included in a visible tabbed container and must also be the active tab to be true. If isSelectedTab is false then the panel only has to be included in a visible tabbed container to be true. On Mac: isSelected is ignored and true is returned if the panel appears in any inspector panel.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_IsPanelVisible_1.htm)

#### `public static bool IsPanelVisible(Type panelType)`

Check to see if a panel is currently visible, on Windows this means you can see the tab, it does not necessarily mean it is the current tab.

**Parameters:**
- `panelType` (System.Type) — Type of panel to check for, this type must include a GUID attribute.

**Returns:** `Boolean` — Returns true if panelType is non null and the tab is visible otherwise it returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_IsPanelVisible_2.htm)

#### `public static bool IsPanelVisible(Type panelType, bool isSelectedTab)`

Check to see if a panel is currently visible, if isSelectedTab is true then the tab must be the active tab as well.

**Parameters:**
- `panelType` (System.Type) — Type of panel to check for, this type must include a GUID attribute.
- `isSelectedTab` (System.Boolean) — This parameter is ignored on Mac. If Windows and true the panel must be visible in a container and if it is a tabbed container it must be the active tab to be true.

**Returns:** `Boolean` — On Windows: The return value is dependent on the isSelectedTab value. If isSelectedTab is true then the panel must be included in a visible tabbed container and must also be the active tab to be true. If isSelectedTab is false then the panel only has to be included in a visible tabbed container to be true. On Mac: isSelected is ignored and true is returned if the panel appears in any inspector panel.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_IsPanelVisible_3.htm)

#### `public static bool IsShowing(ShowPanelReason reason)`

Check to see if reason is equal to any of the hide events

**Parameters:**
- `reason` (Rhino.UI.ShowPanelReason) — [Missing <param name="reason"/> documentation for "M:Rhino.UI.Panels.IsShowing(Rhino.UI.ShowPanelReason)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.Panels.IsShowing(Rhino.UI.ShowPanelReason)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_IsShowing.htm)

#### `public static void OnClosePanel(Guid panelId, uint documentSerialNumber)`

Call this method to raise the Closed event

**Parameters:**
- `panelId` (System.Guid) — Panel identifier.
- `documentSerialNumber` (System.UInt32) — The document associated with the closed panel.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_OnClosePanel.htm)

#### `public static void OnShowPanel(Guid panelId, uint documentSerialNumber, bool show)`

Call this method to raise the Show event

**Parameters:**
- `panelId` (System.Guid) — [Missing <param name="panelId"/> documentation for "M:Rhino.UI.Panels.OnShowPanel(System.Guid,System.UInt32,System.Boolean)"]
- `documentSerialNumber` (System.UInt32) — The document associated with the shown/hidden panel.
- `show` (System.Boolean) — [Missing <param name="show"/> documentation for "M:Rhino.UI.Panels.OnShowPanel(System.Guid,System.UInt32,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_OnShowPanel.htm)

#### `public static void OpenPanel(Guid panelId)`

Open the specified panel in its current or default location and if it is in a dock bar containing more than one tab, make it the current tab.

**Parameters:**
- `panelId` (System.Guid) — Class type Id for the panel to open.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_OpenPanel.htm)

#### `public static void OpenPanel(Guid panelId, bool makeSelectedPanel)`

Open the specified panel in its current or default location and if it is in a dock bar containing more than one tab, make it the current tab.

**Parameters:**
- `panelId` (System.Guid) — Class type Id for the panel to open.
- `makeSelectedPanel` (System.Boolean) — If true then the panel is set as the active tab after opening it otherwise; the panel is opened but not set as the active tab.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_OpenPanel_1.htm)

#### `public static Guid OpenPanel(Guid dockBarId, Guid panelId)`

In Mac Rhino this will just call the version of OpenPanel that takes a class type Id. In Windows Rhino this will look for a dock bar with the specified Id and open or move the specified panel to that dock bar.

**Parameters:**
- `dockBarId` (System.Guid) — Id of the dock bar hosting one or more panels.
- `panelId` (System.Guid) — Class type Id for the panel to open.

**Returns:** `Guid` — Returns true if the

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_OpenPanel_2.htm)

#### `public static Guid OpenPanel(Guid dockBarId, Guid panelId, bool makeSelectedPanel)`

In Mac Rhino this will just call the version of OpenPanel that takes a class type Id. In Windows Rhino this will look for a dock bar with the specified Id and open or move the specified panel to that dock bar.

**Parameters:**
- `dockBarId` (System.Guid) — Id of the dock bar hosting one or more panels.
- `panelId` (System.Guid) — Class type Id for the panel to open.
- `makeSelectedPanel` (System.Boolean) — If true then the panel is set as the active tab after opening it otherwise; the panel is opened but not set as the active tab.

**Returns:** `Guid` — Returns true if the

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_OpenPanel_3.htm)

#### `public static Guid OpenPanel(Guid dockBarId, Type panelType)`

In Mac Rhino this will just call the version of OpenPanel that takes a class type Id. In Windows Rhino this will look for a dock bar with the specified Id and open or move the specified panel to that dock bar.

**Parameters:**
- `dockBarId` (System.Guid) — Id of the dock bar hosting one or more panels.
- `panelType` (System.Type) — Class type for the panel to open.

**Returns:** `Guid` — Returns true if the

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_OpenPanel_4.htm)

#### `public static Guid OpenPanel(Guid dockBarId, Type panelType, bool makeSelectedPanel)`

In Mac Rhino this will just call the version of OpenPanel that takes a class type Id. In Windows Rhino this will look for a dock bar with the specified Id and open or move the specified panel to that dock bar.

**Parameters:**
- `dockBarId` (System.Guid) — Id of the dock bar hosting one or more panels.
- `panelType` (System.Type) — Class type for the panel to open.
- `makeSelectedPanel` (System.Boolean) — If true then the panel is set as the active tab after opening it otherwise; the panel is opened but not set as the active tab.

**Returns:** `Guid` — Returns true if the

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_OpenPanel_5.htm)

#### `public static void OpenPanel(Type panelType)`

Open the specified panel in its current or default location and if it is in a dock bar containing more than one tab, make it the current tab.

**Parameters:**
- `panelType` (System.Type) — Class type of the panel to open.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_OpenPanel_6.htm)

#### `public static void OpenPanel(Type panelType, bool makeSelectedPanel)`

Open the specified panel in its current or default location and if it is in a dock bar containing more than one tab, make it the current tab.

**Parameters:**
- `panelType` (System.Type) — Class type of the panel to open.
- `makeSelectedPanel` (System.Boolean) — If true then the panel is set as the active tab after opening it otherwise; the panel is opened but not set as the active tab.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_OpenPanel_7.htm)

#### `public static bool OpenPanelAsSibling(Guid panelId, Guid siblingPanelId)`

In Mac Rhino this will currently just call OpenPanel, in Windows Rhino this will look for a dock bar which contains the sibling panel and add this panel to that dock bar as necessary, if the panel was in another dock bar it will be moved to this dock bar.

**Parameters:**
- `panelId` (System.Guid) — The class Id of the panel type to open.
- `siblingPanelId` (System.Guid) — The class Id of the sibling panel.

**Returns:** `Boolean` — Returns true if the panel was successfully opened.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_OpenPanelAsSibling.htm)

#### `public static bool OpenPanelAsSibling(Guid panelId, Guid siblingPanelId, bool makeSelectedPanel)`

In Mac Rhino this will currently just call OpenPanel, in Windows Rhino this will look for a dock bar which contains the sibling panel and add this panel to that dock bar as necessary, if the panel was in another dock bar it will be moved to this dock bar.

**Parameters:**
- `panelId` (System.Guid) — The class Id of the panel type to open.
- `siblingPanelId` (System.Guid) — The class Id of the sibling panel.
- `makeSelectedPanel` (System.Boolean) — If true then the panel is set as the active tab after opening it otherwise; the panel is opened but not set as the active tab.

**Returns:** `Boolean` — Returns true if the panel was successfully opened.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_OpenPanelAsSibling_1.htm)

#### `public static Guid PanelDockBar(Guid panelId)`

Will always return Guid.Empty in Mac Rhino. In Windows Rhino it will look for the dock bar which contains the specified panel class Id and return the dock bar Id.

**Parameters:**
- `panelId` (System.Guid) — Panel class Id for of the panel to look for.

**Returns:** `Guid` — Always returns Guid.Empty on Mac Rhino. On Windows Rhino it will return the Id for the dock bar which host the specified panel or Guid.Empty if the panel is not currently visible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_PanelDockBar.htm)

#### `public static Guid PanelDockBar(Type panelType)`

Will always return Guid.Empty in Mac Rhino. In Windows Rhino it will look for the dock bar which contains the specified panel class Id and return the dock bar Id.

**Parameters:**
- `panelType` (System.Type) — Panel class for of the panel to look for.

**Returns:** `Guid` — Always returns Guid.Empty on Mac Rhino. On Windows Rhino it will return the Id for the dock bar which host the specified panel or Guid.Empty if the panel is not currently visible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_PanelDockBar_1.htm)

#### `public static Guid[] PanelDockBars(Guid panelId)`

Will always return a empty array in Mac Rhino. In Windows Rhino it will look for any panel dock bars that contain the specified panel class Id and return the dock bar Id's.

**Parameters:**
- `panelId` (System.Guid) — Panel class Id for of the panel to look for.

**Returns:** `Guid[]` — Always returns Guid.Empty on Mac Rhino. On Windows Rhino it will return the Id for the dock bar which host the specified panel or Guid.Empty if the panel is not currently visible.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_PanelDockBars.htm)

#### `public static void RegisterPanel(PlugIn plugIn, Type type, string caption, Assembly iconAssembly, string iconResourceId, PanelType panelType)`

You typically register your panel class in your plug-in's OnLoad function. This will register your custom call with Rhino, Rhino will create an instance of your class the first time your panel is created and embed this instance of your class in a panel container.

**Parameters:**
- `plugIn` (Rhino.PlugIns.PlugIn) — Plug-in this panel is associated with
- `type` (System.Type) — Class type to construct when a panel is shown. If your class is derived from Eto.Forms.Control it will work on both the Mac and Windows version of Rhino. In addition Windows Rhino will support any class types that implement the IWin32Window interface or that are derived from System.Windows.FrameworkElement. Mac Rhino will also support classes that are derived from NsView. In addition to the type requirements the class must have a public constructor with no parameters or a constructor with a single uint that represents the document serial number and have a GuidAttribute applied with a unique Id. n Windows there is only one panel created which gets recycled for each new document. On the Mac a panel will be created for each open document and destroyed when the document closes. In certain situations in Mac Rhino a a panel may get created and destroyed multiple times when opening/closing a panel while editing a document.
- `caption` (System.String) — Displays in the panel tab on Windows or at the top of the modeless window on Mac.
- `iconAssembly` (System.Reflection.Assembly) — Assembly conataining the iconResourceId, if null it is assumed the iconResourceId is a starndard Rhino resource and the Rhino.UI assembly will be used. assembly will be used
- `iconResourceId` (System.String) — The resource Id string used to load the panel icon from the iconAssembly. On Windows the panel may be displayed using the icon, caption or both. On Mac the icon will be used and the caption will be the tool-tip.
- `panelType` (Rhino.UI.PanelType) — See PanelType

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_RegisterPanel_2.htm)

#### `public static void RegisterPanel(PlugIn plugin, Type panelType, string caption, Icon icon)`

You typically register your panel class in your plug-in's OnLoad function. This will register your custom call with Rhino, Rhino will create an instance of your class the first time your panel is created and embed this instance of your class in a panel container.

**Parameters:**
- `plugin` (Rhino.PlugIns.PlugIn) — Plug-in this panel is associated with
- `panelType` (System.Type) — Class type to construct when a panel is shown. If your class is derived from Eto.Forms.Control it will work on both the Mac and Windows version of Rhino. In addition Windows Rhino will support any class types that implement the IWin32Window interface or that are derived from System.Windows.FrameworkElement. Mac Rhino will also support classes that are derived from NsView. In addition to the type requirements the class must have a public constructor with no parameters or a constructor with a single uint that represents the document serial number and have a GuidAttribute applied with a unique Id. n Windows there is only one panel created which gets recycled for each new document. On the Mac a panel will be created for each open document and destroyed when the document closes. In certain situations in Mac Rhino a a panel may get created and destroyed multiple times when opening/closing a panel while editing a document.
- `caption` (System.String) — Displays in the panel tab on Windows or at the top of the modeless window on Mac.
- `icon` (System.Drawing.Icon) — The panel icon. On Windows the panel may be displayed using the icon, caption or both. On Mac the icon will be used and the caption will be the tool-tip.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_RegisterPanel.htm)

#### `public static void RegisterPanel(PlugIn plugIn, Type type, string caption, Icon icon, PanelType panelType)`

Call once to register a panel type which will get dynamically created and embedded in a Rhino docking/floating location.

**Parameters:**
- `plugIn` (Rhino.PlugIns.PlugIn) — Plug-in restringing the panel
- `type` (System.Type) — Type of the control object to be displayed in the panel
- `caption` (System.String) — Panel caption also used as a tool-tip. On Windows the panel may be displayed using the icon, caption or both. On Mac the icon will be used and the caption will be the tool-tip.
- `icon` (System.Drawing.Icon) — The panel icon. On Windows the panel may be displayed using the icon, caption or both. On Mac the icon will be used and the caption will be the tool-tip.
- `panelType` (Rhino.UI.PanelType) — See PanelType

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_Panels_RegisterPanel_1.htm)

### Properties
- `EtoPanelStyleName` (String, get) — Style applied to Eto controls hosted by the Rhino.UI.Panels and Rhino.UI.ObjectProperties systems.
- `IconSize` (Size, get) — Gets the panel icon size in logical units.
- `IconSizeInPixels` (Size, get) — Gets the panel icon size in pixels.
- `ScaledIconSize` (Size, get) — Gets the panel icon size in pixels with DPI scaling applied.

### Events
#### `Closed` (`System.EventHandler<PanelEventArgs>`)

**Signature:** `public static event EventHandler<PanelEventArgs> Closed`

This event is raised when a panel host is closed

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_UI_Panels_Closed.htm)

#### `Show` (`System.EventHandler<ShowPanelEventArgs>`)

**Signature:** `public static event EventHandler<ShowPanelEventArgs> Show`

This event is called when a panel is shown or hidden. This event will get raised multipThis times when the active document changes in Mac Rhino. It will called with show equal to false for the previous active document and with show equal to true for the current document. When the event is raised with show equal to false it only means the document instance of the panel is not visible it does not mean the panel host has been closed. If you need to know when the panel host closes then subscribe to the Closed event.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_UI_Panels_Show.htm)

## Panels.FloatPanelMode (enum)

Used by the FloatPanel method to determine if the floating panel should be shown or hidden.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_Panels_FloatPanelMode.htm)

### Values
- `Show` = `0` — Show the floating panel
- `Hide` = `1` — Hide the floating panel
- `Toggle` = `2` — Toggle the visibility state

## PropertyPageType (enum)

IRhinoProperties page type

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_PropertyPageType.htm)

### Values
- `Material` = `0` — Replace the material page with this page; only works for the current render plug - in.
- `Light` = `1` — Replace the light page with this page; only works for the current render plug-in.
- `Custom` = `2` — User-defined custom object page.
- `ObjectProperties` = `3` — For internal use only.
- `Dimension` = `4` — If page is provided by Rhino and only dimensions are selected, activate this page.
- `Leader` = `5` — If page is provided by Rhino and leaders are selected, activate this page.
- `Text` = `6` — If page is provided by Rhino and only text objects are selected, activate this page.
- `Hatch` = `7` — If page is provided by Rhino and only hatch objects are selected, activate this page.
- `Dot` = `8` — If page is provided by Rhino and only dot objects are selected, activate this page.
- `TextureMapping` = `9` — Replace the texture mapping page with this page; only works for the current render plug-in.
- `Detail` = `10` — If page is provided by Rhino and only detail objects are selected, activate this page.
- `ClippingPlane` = `11` — If page is provided by Rhino and only clipping plane objects are selected, activate this page.
- `NamedView` = `12` — If page is provided by Rhino and only named view widget objects are selected, activate this page.
- `Decal` = `13` — Decals user interface
- `View` = `14` — Page is a view properties page and uses view properties methods. Does not use object properties methods.
- `PageCount` = `15` — Reserved, do not use.

## RhinoGetPlotWidthArgs (enum)

Argument flags passed to methods used to get Rhino plot width lists

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_RhinoGetPlotWidthArgs.htm)

### Values
- `NoArgs` = `0` — Just get the standard or default width list
- `ByLayer` = `1` — Include a By Layer option
- `ByParent` = `2` — Include a By Parent option
- `HairLine` = `4` — Include a hairline width option
- `Default` = `8` — Include a default width option
- `None` = `32` — Include a no print option
- `All` = `268435455` — Include everything

## RhinoHelp (class)

Provides access to the built in Rhino help system

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_RhinoHelp.htm)

### Methods
#### `public static bool Show(string helpLink)`

Call this method to display standard Rhino help

**Parameters:**
- `helpLink` (System.String) — Rhino help links are formatted like this: http://docs.mcneel.com/rhino/6/help/en-us/index.htm#commands/line.htm This parameter would be equal to "#commands/line.htm" in the link above. Rhino will calculate the string up to and including the index.html and append this value to the end.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.RhinoHelp.Show(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_RhinoHelp_Show.htm)

## RhinoPageInterop (class)

For internal use only, provides access to unmanaged core Rhino.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_RhinoPageInterop.htm)

### Methods
#### `public static IntPtr NewPropertiesPanelPagePointer(ObjectPropertiesPage page, uint rhinoDocRuntimeSn)`

For internal use only, provides access to unmanaged core

**Parameters:**
- `page` (Rhino.UI.ObjectPropertiesPage) — [Missing <param name="page"/> documentation for "M:Rhino.UI.RhinoPageInterop.NewPropertiesPanelPagePointer(Rhino.UI.ObjectPropertiesPage,System.UInt32)"]
- `rhinoDocRuntimeSn` (System.UInt32) — [Missing <param name="rhinoDocRuntimeSn"/> documentation for "M:Rhino.UI.RhinoPageInterop.NewPropertiesPanelPagePointer(Rhino.UI.ObjectPropertiesPage,System.UInt32)"]

**Returns:** `IntPtr` — [Missing <returns> documentation for "M:Rhino.UI.RhinoPageInterop.NewPropertiesPanelPagePointer(Rhino.UI.ObjectPropertiesPage,System.UInt32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_RhinoPageInterop_NewPropertiesPanelPagePointer.htm)

#### `public static StackedDialogPage StackedDialogPageFromUnmanagedPointer(IntPtr pointer)`

For internal use only, provides access to unmanaged core

**Parameters:**
- `pointer` (System.IntPtr) — [Missing <param name="pointer"/> documentation for "M:Rhino.UI.RhinoPageInterop.StackedDialogPageFromUnmanagedPointer(System.IntPtr)"]

**Returns:** `StackedDialogPage` — [Missing <returns> documentation for "M:Rhino.UI.RhinoPageInterop.StackedDialogPageFromUnmanagedPointer(System.IntPtr)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_RhinoPageInterop_StackedDialogPageFromUnmanagedPointer.htm)

## RhinoPlotWidthType (enum)

Supported plot width special types

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_RhinoPlotWidthType.htm)

### Values
- `ByLayer` = `0` — Plot width from layer
- `ByParent` = `1` — Plot width from parent object
- `Hairline` = `2` — System hairline plot width
- `Default` = `3` — Use default plot width
- `None` = `4` — Don't print
- `Varies` = `5` — Multiple objects selected with different types/widths
- `Width` = `6` — Standard or custom width

## RhinoPlotWidthValue (enum)

Default width values used by UI objects to represent different states

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_RhinoPlotWidthValue.htm)

### Values
- `Default` = `0` — Use default plot width
- `None` = `-1` — Don't print
- `ByLayer` = `-10` — Plot width from layer
- `ByParent` = `-15` — Plot width from parent object
- `Varies` = `-20` — Multiple objects selected with different types/widths

## RuiUpdateUi (class)

[Missing <summary> documentation for "T:Rhino.UI.RuiUpdateUi"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_RuiUpdateUi.htm)

### Methods
#### `public static bool RegisterMenuItem(Guid file, Guid menu, Guid item, RuiUpdateUi.UpdateMenuItemEventHandler callBack)`

Register menu item update delegate

**Parameters:**
- `file` (System.Guid) — Menu file Id
- `menu` (System.Guid) — Menu Id
- `item` (System.Guid) — Menu item Id
- `callBack` (Rhino.UI.RuiUpdateUi.UpdateMenuItemEventHandler) — [Missing <param name="callBack"/> documentation for "M:Rhino.UI.RuiUpdateUi.RegisterMenuItem(System.Guid,System.Guid,System.Guid,Rhino.UI.RuiUpdateUi.UpdateMenuItemEventHandler)"]

**Returns:** `Boolean` — true if Registered otherwise false

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_RuiUpdateUi_RegisterMenuItem.htm)

#### `public static bool RegisterMenuItem(string fileId, string menuId, string itemId, RuiUpdateUi.UpdateMenuItemEventHandler callBack)`

Register menu item update delegate

**Parameters:**
- `fileId` (System.String) — Menu file Id
- `menuId` (System.String) — Menu Id
- `itemId` (System.String) — Menu item Id
- `callBack` (Rhino.UI.RuiUpdateUi.UpdateMenuItemEventHandler) — [Missing <param name="callBack"/> documentation for "M:Rhino.UI.RuiUpdateUi.RegisterMenuItem(System.String,System.String,System.String,Rhino.UI.RuiUpdateUi.UpdateMenuItemEventHandler)"]

**Returns:** `Boolean` — true if Registered otherwise false

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_RuiUpdateUi_RegisterMenuItem_1.htm)

### Properties
- `Checked` (Boolean, get/set) — Set to true to enable menu item or false to check menu item
- `Enabled` (Boolean, get/set) — Set to true to enable menu item or false to disable menu item
- `FileId` (Guid, get) — Id of the RUI file that owns this menu item
- `MenuHandle` (IntPtr, get) — Windows menu handle of menu that contains this item
- `MenuId` (Guid, get) — Id of the menu that owns this menu item
- `MenuIndex` (Int32, get) — Zero based index of item in the Windows menu
- `MenuItemId` (Guid, get) — Id of the menu item that owns this menu item
- `RadioChecked` (Boolean, get/set) — Set to true to enable menu item or false to check menu item
- `Text` (String, get/set) — Menu item text
- `WindowsMenuItemId` (UInt32, get) — Windows menu item ID

## RuiUpdateUi.UpdateMenuItemEventHandler (delegate)

Menu item update delegate

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_RuiUpdateUi_UpdateMenuItemEventHandler.htm)

## SaveFileDialog (class)

Similar to the System.Windows.Forms.SaveFileDialog, but with customized Rhino user interface.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_SaveFileDialog.htm)

### Constructors
- `public SaveFileDialog()` — Initializes a new instance of the SaveFileDialog class

### Methods
#### `[ObsoleteAttribute("Use ShowSaveDialog")] public DialogResult ShowDialog()`

Obsolete.

**Returns:** `DialogResult` — [Missing <returns> documentation for "M:Rhino.UI.SaveFileDialog.ShowDialog"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_SaveFileDialog_ShowDialog.htm)

#### `public bool ShowSaveDialog()`



**Returns:** `Boolean` — true if a file was selected. false if the dialog was canceled

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_SaveFileDialog_ShowSaveDialog.htm)

### Properties
- `DefaultExt` (String, get/set) — The default file name extension. The returned string does not include the period.
- `FileName` (String, get/set) — Gets or sets a string containing the file name selected in the file dialog box.
- `Filter` (String, get/set) — Gets or sets the current file name filter string, which determines the choices that appear in the "Save as file type" or "Files of type" box in the dialog box. See System.Windows.Forms.FileDialog for details.
- `InitialDirectory` (String, get/set) — Gets or sets the initial directory displayed by the file dialog box.
- `Title` (String, get/set) — Gets or sets the file dialog box title.

## ShowMessageButton (enum)

[Missing <summary> documentation for "T:Rhino.UI.ShowMessageButton"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_ShowMessageButton.htm)

### Values
- `OK` = `0`
- `OKCancel` = `1`
- `AbortRetryIgnore` = `2`
- `YesNoCancel` = `3`
- `YesNo` = `4`
- `RetryCancel` = `5`

## ShowMessageDefaultButton (enum)

[Missing <summary> documentation for "T:Rhino.UI.ShowMessageDefaultButton"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_ShowMessageDefaultButton.htm)

### Values
- `Button1` = `0`
- `Button2` = `256`
- `Button3` = `512`

## ShowMessageIcon (enum)

[Missing <summary> documentation for "T:Rhino.UI.ShowMessageIcon"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_ShowMessageIcon.htm)

### Values
- `None` = `0`
- `Error` = `16`
- `Hand` = `16`
- `Stop` = `16`
- `Question` = `32`
- `Exclamation` = `48`
- `Warning` = `48`
- `Information` = `64`
- `Asterisk` = `64`

## ShowMessageMode (enum)

[Missing <summary> documentation for "T:Rhino.UI.ShowMessageMode"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_ShowMessageMode.htm)

### Values
- `ApplicationModal` = `0`
- `SystemModal` = `4096`
- `TaskModal` = `8192`

## ShowMessageOptions (enum)

[Missing <summary> documentation for "T:Rhino.UI.ShowMessageOptions"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_ShowMessageOptions.htm)

### Values
- `None` = `0`
- `SetForeground` = `65536`
- `DefaultDesktopOnly` = `131072`
- `TopMost` = `262144`
- `RightAlign` = `524288`
- `RtlReading` = `1048576`
- `ServiceNotification` = `2097152`

## ShowMessageResult (enum)

[Missing <summary> documentation for "T:Rhino.UI.ShowMessageResult"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_ShowMessageResult.htm)

### Values
- `None` = `0`
- `OK` = `1`
- `Cancel` = `2`
- `Abort` = `3`
- `Retry` = `4`
- `Ignore` = `5`
- `Yes` = `6`
- `No` = `7`

## ShowPanelEventArgs (class)

Panels.Show event arguments

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_ShowPanelEventArgs.htm)

### Constructors
- `public ShowPanelEventArgs(Guid panelId, uint documentSerialNumber, bool show)` — Initializes a new instance of the ShowPanelEventArgs class

### Properties
- `Document` (RhinoDoc, get) — (Inherited from PanelEventArgs.)
- `DocumentSerialNumber` (UInt32, get) — (Inherited from PanelEventArgs.)
- `PanelId` (Guid, get) — Class Id for panel being shown or hidden
- `Show` (Boolean, get) — Will be true if showing or false if hiding

## ShowPanelReason (enum)

OnShowDockbar event type

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_ShowPanelReason.htm)

### Values
- `Show` = `0` — Dock bar shown or made visible
- `Hide` = `1` — Dock bar hidden, no longer visible
- `HideOnDeactivate` = `2` — Dock bar temporarily hidden because the main Rhino application is no longer active.
- `ShowOnDeactivate` = `3` — Dock bar that was temporarily hidden when the main Rhino application was deactivated is now being shown.

## StackedDialogPage (class)

Base class to inherit from for the addition of stacked dialog pages.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_StackedDialogPage.htm)

### Constructors
- `protected StackedDialogPage(string englishPageTitle)` — Protected constructor

### Methods
#### `public void AddChildPage(StackedDialogPage pageToAdd)`

Currently only supported on Windows. Call this method to add a child page to a page after the parent dialog has been created.

**Parameters:**
- `pageToAdd` (Rhino.UI.StackedDialogPage) — [Missing <param name="pageToAdd"/> documentation for "M:Rhino.UI.StackedDialogPage.AddChildPage(Rhino.UI.StackedDialogPage)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_AddChildPage.htm)

#### `public void MakeActivePage()`

Make this page the active, visible page

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_MakeActivePage.htm)

#### `public virtual bool OnActivate(bool active)`

Called when this page is activated/deactivated.

**Parameters:**
- `active` (System.Boolean) — If true then this page is on top otherwise it is about to be hidden.

**Returns:** `Boolean` — If true then the page is hidden and the requested page is not activated otherwise will not allow you to change the current page. Default returns true

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_OnActivate.htm)

#### `public virtual bool OnApply()`

Called when stacked dialog OK button is pressed.

**Returns:** `Boolean` — If return value is true then the dialog will be closed. A return of false means there was an error and dialog remains open so page can be properly updated.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_OnApply.htm)

#### `public virtual void OnCancel()`

Called when stacked dialog Cancel button is pressed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_OnCancel.htm)

#### `public virtual void OnCreateParent(IntPtr hwndParent)`

Called when the parent window has been created on Windows platforms only.

**Parameters:**
- `hwndParent` (System.IntPtr) — [Missing <param name="hwndParent"/> documentation for "M:Rhino.UI.StackedDialogPage.OnCreateParent(System.IntPtr)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_OnCreateParent.htm)

#### `public virtual void OnDefaults()`

Called when stacked dialog Defaults button is pressed (see ShowDefaultsButton).

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_OnDefaults.htm)

#### `public virtual void OnHelp()`

Called when the parent dialog requests help for this page.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_OnHelp.htm)

#### `public virtual void OnSizeParent(int width, int height)`

Called when the parent window has been resized

**Parameters:**
- `width` (System.Int32) — [Missing <param name="width"/> documentation for "M:Rhino.UI.StackedDialogPage.OnSizeParent(System.Int32,System.Int32)"]
- `height` (System.Int32) — [Missing <param name="height"/> documentation for "M:Rhino.UI.StackedDialogPage.OnSizeParent(System.Int32,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_OnSizeParent.htm)

#### `public void RemovePage()`

Remove this page from the dialog box

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_RemovePage.htm)

#### `public bool SetActivePageTo(string pageName, bool documentPropertiesPage)`



**Parameters:**
- `pageName` (System.String) — [Missing <param name="pageName"/> documentation for "M:Rhino.UI.StackedDialogPage.SetActivePageTo(System.String,System.Boolean)"]
- `documentPropertiesPage` (System.Boolean) — [Missing <param name="documentPropertiesPage"/> documentation for "M:Rhino.UI.StackedDialogPage.SetActivePageTo(System.String,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.StackedDialogPage.SetActivePageTo(System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_SetActivePageTo.htm)

#### `public void SetEnglishPageTitle(string newPageTile)`

Change the title passed to the constructor and, this will cause LocalPageTitle to get called also.

**Parameters:**
- `newPageTile` (System.String) — [Missing <param name="newPageTile"/> documentation for "M:Rhino.UI.StackedDialogPage.SetEnglishPageTitle(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StackedDialogPage_SetEnglishPageTitle.htm)

### Properties
- `Children` (List<StackedDialogPage>, get) — List of child (sub) pages of this page
- `EnglishPageTitle` (String, get) — English string used when scripting this page and a user prefixes a command option with "_"
- `Handle` (IntPtr, get) — When running on Windows return the window handle for the parent of this page otherwise; return IntPtr.Zero.
- `HasChildren` (Boolean, get) — Will be true if this page contains sub pages.
- `LocalPageTitle` (String, get) — Localized page title that will appear on a tab or other page navigation control. This is also uses as a command line option when scripting this page.
- `Modified` (Boolean, get/set) — Check to see if the page has been marked as modified or not. Marking the page as modified will cause the Apply button to get enabled if this is currently the visible page and the page includes the Apply button.
- `NavigationTextColor` (Color, get/set) — Currently only used by Windows Rhino. If this is set to true then the tree control item text be drawn using this color. Set the color to System.Drawing.Color.Empty to use the default color.
- `NavigationTextIsBold` (Boolean, get/set) — Currently only used by Windows Rhino. If this is set to true then the tree control item text will be bold.
- `PageControl` (Object, get) — Return the control that represents this page. Rhino Windows supports classes that implement the IWin32Windows interface or are derived from some form of System.Windows.FrameworkElement or Eto.Forms.Control. Mac Rhino supports controls that are derived from NSview or Eto.Forms.Control.
- `PageImage` (Image, get) — Optionally override to provide a image to display in the Mac Rhino UI
- `ShowApplyButton` (Boolean, get) — Called when this page is activated
- `ShowDefaultsButton` (Boolean, get) — Called when this page is activated.

## StatusBar (class)

Contains static methods to control the application status bar.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_StatusBar.htm)

### Methods
#### `public static void ClearMessagePane()`

Removes the message from the message pane.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StatusBar_ClearMessagePane.htm)

#### `public static void HideProgressMeter()`

Ends, or hides, Rhino's status bar progress meter.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StatusBar_HideProgressMeter.htm)

#### `public static void HideProgressMeter(uint docSerialNumber)`

Ends, or hides, Rhino's status bar progress meter.

**Parameters:**
- `docSerialNumber` (System.UInt32) — The document runtime serial number.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StatusBar_HideProgressMeter_1.htm)

#### `public static void SetDistancePane(double distance)`

Sets the distance pane to a distance value.

**Parameters:**
- `distance` (System.Double) — The distance value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StatusBar_SetDistancePane.htm)

#### `public static void SetMessagePane(string message)`

Sets the message pane to a message.

**Parameters:**
- `message` (System.String) — The message value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StatusBar_SetMessagePane.htm)

#### `public static void SetNumberPane(double number)`

Sets the number pane to a number value

**Parameters:**
- `number` (System.Double) — [Missing <param name="number"/> documentation for "M:Rhino.UI.StatusBar.SetNumberPane(System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StatusBar_SetNumberPane.htm)

#### `public static void SetPointPane(Point3d point)`

Sets the point pane to a point value.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — The point value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StatusBar_SetPointPane.htm)

#### `public static int ShowProgressMeter(int lowerLimit, int upperLimit, string label, bool embedLabel, bool showPercentComplete)`

Starts, or shows, Rhino's status bar progress meter.

**Parameters:**
- `lowerLimit` (System.Int32) — The lower limit of the progress meter's range.
- `upperLimit` (System.Int32) — The upper limit of the progress meter's range.
- `label` (System.String) — The short description of the progress (e.g. "Calculating", "Meshing", etc)
- `embedLabel` (System.Boolean) — If true, then the label will be embedded in the progress meter. If false, then the label will appear to the left of the progress meter.
- `showPercentComplete` (System.Boolean) — If true, then the percent complete will appear in the progress meter.

**Returns:** `Int32` — 1 - The progress meter was created successfully. 0 - The progress meter was not created. -1 - The progress meter was not created because some other process has already created it.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StatusBar_ShowProgressMeter.htm)

#### `public static int ShowProgressMeter(uint docSerialNumber, int lowerLimit, int upperLimit, string label, bool embedLabel, bool showPercentComplete)`

Starts, or shows, Rhino's status bar progress meter.

**Parameters:**
- `docSerialNumber` (System.UInt32) — The document runtime serial number.
- `lowerLimit` (System.Int32) — The lower limit of the progress meter's range.
- `upperLimit` (System.Int32) — The upper limit of the progress meter's range.
- `label` (System.String) — The short description of the progress (e.g. "Calculating", "Meshing", etc)
- `embedLabel` (System.Boolean) — If true, then the label will be embedded in the progress meter. If false, then the label will appear to the left of the progress meter.
- `showPercentComplete` (System.Boolean) — If true, then the percent complete will appear in the progress meter.

**Returns:** `Int32` — 1 - The progress meter was created successfully. 0 - The progress meter was not created. -1 - The progress meter was not created because some other process has already created it.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StatusBar_ShowProgressMeter_1.htm)

#### `public static int UpdateProgressMeter(int position, bool absolute)`

Sets the current position of Rhino's status bar progress meter.

**Parameters:**
- `position` (System.Int32) — The new value. This can be stated in absolute terms, or relative compared to the current position. The interval bounds are specified when you first show the bar.
- `absolute` (System.Boolean) — If true, then the progress meter is moved to position. If false, then the progress meter is moved position from the current position (relative).

**Returns:** `Int32` — The previous position if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StatusBar_UpdateProgressMeter.htm)

#### `public static int UpdateProgressMeter(string label, int position, bool absolute)`

Sets the label and current position of Rhino's status bar progress meter.

**Parameters:**
- `label` (System.String) — The short description of the progress (e.g. "Calculating", "Meshing", etc)
- `position` (System.Int32) — The new value. This can be stated in absolute terms, or relative compared to the current position. The interval bounds are specified when you first show the bar. Note, if value is , only the label is updated.
- `absolute` (System.Boolean) — If true, then the progress meter is moved to position. If false, then the progress meter is moved position from the current position (relative).

**Returns:** `Int32` — The previous position if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StatusBar_UpdateProgressMeter_1.htm)

#### `public static int UpdateProgressMeter(uint docSerialNumber, int position, bool absolute)`

Sets the current position of Rhino's status bar progress meter.

**Parameters:**
- `docSerialNumber` (System.UInt32) — The document runtime serial number.
- `position` (System.Int32) — The new value. This can be stated in absolute terms, or relative compared to the current position. The interval bounds are specified when you first show the bar.
- `absolute` (System.Boolean) — If true, then the progress meter is moved to position. If false, then the progress meter is moved position from the current position (relative).

**Returns:** `Int32` — The previous position if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StatusBar_UpdateProgressMeter_2.htm)

#### `public static int UpdateProgressMeter(uint docSerialNumber, string label, int position, bool absolute)`

Sets the label and current position of Rhino's status bar progress meter.

**Parameters:**
- `docSerialNumber` (System.UInt32) — The document runtime serial number.
- `label` (System.String) — The short description of the progress (e.g. "Calculating", "Meshing", etc)
- `position` (System.Int32) — The new value. This can be stated in absolute terms, or relative compared to the current position. The interval bounds are specified when you first show the bar. Note, if value is , only the label is updated.
- `absolute` (System.Boolean) — If true, then the progress meter is moved to position. If false, then the progress meter is moved position from the current position (relative).

**Returns:** `Int32` — The previous position if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_StatusBar_UpdateProgressMeter_3.htm)

## Toolbar (class)

Represents a toolbar in a Rhino toolbar, or .RUI, file.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_Toolbar.htm)

### Properties
- `BitmapSize` (Size, get/set) — Gets and sets the size of the toolbar image.
- `Id` (Guid, get) — Gets the id of the toolbar.
- `Name` (String, get) — Gets the name of the toolbar.
- `TabSize` (Size, get/set) — Gets and sets the size of the toolbar tab.

## ToolbarFile (class)

Represents a Rhino toolbar, or .RUI, file.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_ToolbarFile.htm)

### Methods
#### `public bool Close(bool prompt)`

Closes the toolbar file.

**Parameters:**
- `prompt` (System.Boolean) — Set true if you want to be prompted to cllose the file.

**Returns:** `Boolean` — True if successful, false otherwie.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ToolbarFile_Close.htm)

#### `public ToolbarGroup GetGroup(int index)`

Gets a toolbar group.

**Parameters:**
- `index` (System.Int32) — The index of the toolbar group.

**Returns:** `ToolbarGroup` — The toolbar group if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ToolbarFile_GetGroup.htm)

#### `public ToolbarGroup GetGroup(string name)`

Gets a toolbar group.

**Parameters:**
- `name` (System.String) — The name of the toolbar group.

**Returns:** `ToolbarGroup` — The toolbar group if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ToolbarFile_GetGroup_1.htm)

#### `public Toolbar GetToolbar(int index)`

Gets a toolbar.

**Parameters:**
- `index` (System.Int32) — The index of the toolbar.

**Returns:** `Toolbar` — The toolbar if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ToolbarFile_GetToolbar.htm)

#### `public bool Save()`

Saves the toolbar file.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.ToolbarFile.Save"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ToolbarFile_Save.htm)

#### `public bool SaveAs(string path)`

Saves the toolbar file to a different path.

**Parameters:**
- `path` (System.String) — [Missing <param name="path"/> documentation for "M:Rhino.UI.ToolbarFile.SaveAs(System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.ToolbarFile.SaveAs(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ToolbarFile_SaveAs.htm)

### Properties
- `GroupCount` (Int32, get) — Get the number of toolbar groups in the toolbar file.
- `Id` (Guid, get) — Gets the id of the toolbar file.
- `Name` (String, get) — Gets the name, or alias, of the toolbar file.
- `Path` (String, get) — Gets the full path to the toolbar file.
- `ToolbarCount` (Int32, get) — Get the number of toolbars in the toolbar file.

## ToolbarFileCollection (class)

Represents a collection of Rhino toolbars, or .RUI, files.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_ToolbarFileCollection.htm)

### Methods
#### `public ToolbarFile FindByName(string name, bool ignoreCase)`

Gets an open toolbar file by name, or alias.

**Parameters:**
- `name` (System.String) — The name, or alias, of the toolbar file.
- `ignoreCase` (System.Boolean) — true to ignore case during the comparison; otherwise, false.

**Returns:** `ToolbarFile` — The toolbar if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ToolbarFileCollection_FindByName.htm)

#### `public ToolbarFile FindByPath(string path)`

Gets an open toolbar by full path.

**Parameters:**
- `path` (System.String) — The full path to the toolbar file.

**Returns:** `ToolbarFile` — [Missing <returns> documentation for "M:Rhino.UI.ToolbarFileCollection.FindByPath(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ToolbarFileCollection_FindByPath.htm)

#### `public IEnumerator<ToolbarFile> GetEnumerator()`

Gets a toolbar file enumerator.

**Returns:** `IEnumerator<ToolbarFile>` — The enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ToolbarFileCollection_GetEnumerator.htm)

#### `public ToolbarFile Open(string path)`

Opens a toolbar file.

**Parameters:**
- `path` (System.String) — The full path to the toolbar file.

**Returns:** `ToolbarFile` — The toolbar if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_ToolbarFileCollection_Open.htm)

### Properties
- `Count` (Int32, get) — Get tne number of open toolbar files.
- `Item` (ToolbarFile, get) — Gets an open toolbar file by index.
- `MruSidebarIsVisible` (Boolean, get/set) — Returns true if the most-recently-used sizebar is visible.
- `SidebarIsVisible` (Boolean, get/set) — Returns true if the sizebar is visible.

## ToolbarGroup (class)

Represents a toolbar group in a Rhino toolbar, or .RUI, file.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_ToolbarGroup.htm)

### Properties
- `Id` (Guid, get) — Gets the id of the toolbar group.
- `IsDocked` (Boolean, get) — Returns true if the toolbar group is docked.
- `Name` (String, get) — Gets the name of the toolbar group.
- `Visible` (Boolean, get/set) — Gets and sets a toolbar group's visibility.

## WaitCursor (class)

[Missing <summary> documentation for "T:Rhino.UI.WaitCursor"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UI_WaitCursor.htm)

### Constructors
- `public WaitCursor()` — Initializes a new instance of the WaitCursor class

### Methods
#### `public void Clear()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_WaitCursor_Clear.htm)

#### `public void Dispose()`

Releases all resources used by the WaitCursor

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_WaitCursor_Dispose.htm)

#### `public void Set()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_UI_WaitCursor_Set.htm)

