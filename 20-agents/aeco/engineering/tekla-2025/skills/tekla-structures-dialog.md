---
name: tekla-tekla-structures-dialog
description: This skill encodes the tekla 2025.0 surface of the Tekla.Structures.Dialog namespace — 27 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ApplicationWindowBase, ApplicationFormBase, AttributeTypeNameEditor, BindPropertyNameEditor, Dialogs, ErrorDialog, FormBase, HelpViewer, and 19 more types.
---

# Tekla.Structures.Dialog

Auto-generated from vendor docs for tekla 2025.0. 27 types in this namespace.

## ApplicationFormBase (class)

The ApplicationFormBase class is the base class for all Tekla Structures dialogs. The class provides localization, unit conversion and data storage (temporary and file-based) among other things.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/74d5e044-c9fa-b822-ed2d-1d2d7cfbeef2)

### Constructors
- `public ApplicationFormBase()` — The default constructor that does not need parameters. Initializes the form and registers property bindings.

### Methods
#### `public void ApplyValues(string FileName)`

Loads the dialog values from a file and performs Apply() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/c4096cdf-60d9-f7b8-1daf-e71de20cf47d)

#### `public bool GetConnectionStatus()`

Returns true if a proper connection to the Tekla Structures process has been established. Currently, there's no way to re-establish the connection.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/2ba6677a-b8a7-cbfa-2724-e58464054abe)

#### `public void InitializeForm()`

Prepares the data storage for the dialog and scans through the fields.

[Docs](https://developer.tekla.com/topic/en/18/43/70881deb-ea45-96c7-ee73-399bc7b76a90)

#### `public void LoadValues(string FileName)`

Loads the dialog values from a file. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/df11f56f-8372-2886-e011-b4fcad4ac3de)

#### `public void ModifyValues(string FileName)`

Loads the dialog values from a file and performs Modify() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/d20a7c4e-e913-92e5-0925-44015cc68ebf)

#### `public void SaveValues(string fileName)`

Serializes the dialog values to an xml file.

**Parameters:**
- `fileName` (System.String) — The path of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/a817a258-86a2-8616-be00-eda92c7e5f8e)

#### `public void SetAttributeValue(Control Ctrl, Object Value)`

Sets a value for the given control. When the dialog is not shown, setting a property directly for a control (such as textBox1.Text = "text") will not work for controls that have a Tekla Structures AttributeTypeName set. This method is going to have to be used to set the value.

**Parameters:**
- `Ctrl` (System.Windows.Forms.Control) — The control whose value to set.
- `Value` (System.Object) — The new value. It can be of any primitive datatype supported by the control's AttributeType (normally int, double or string).

[Docs](https://developer.tekla.com/topic/en/18/43/30e56a80-1c86-4606-781c-2a91b7bb8884)

#### `public void ShowForm()`

Displays the form.

[Docs](https://developer.tekla.com/topic/en/18/43/79258182-6491-e7e6-99d0-de2072cb85a5)

#### `[ObsoleteAttribute("Does currently nothing. Will be removed in near future.")] public void UpdateValues()`

Rereads and updates all the field values on the form.

[Docs](https://developer.tekla.com/topic/en/18/43/5ab0c9d1-4ddf-1e05-29ca-560e387350e5)

### Properties
- `Localization` (Localization, get) — The localization instance for the dialog. Each dialog has its own localization instance that has read the localization files needed for that dialog.

## ApplicationWindowBase (class)

The ApplicationWindowBase class is the base class for all Tekla Structures WPF dialogs. The class provides localization, unit conversion and data storage (temporary and file-based) among other things.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/bcfd9278-a700-227a-8de3-d6c8c7a8dc87)

### Constructors
- `public ApplicationWindowBase()` — The default constructor that does not need parameters. Initializes the window, property binding can be initialized with InitializeDataStorage()-method.

### Methods
#### `public void ApplyValues(string FileName)`

Loads the dialog values from a file and performs Apply() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/76f35628-b668-e741-3bfe-e761a95e178b)

#### `public bool GetConnectionStatus()`

Returns true if a proper connection to the Tekla Structures process has been established. Currently, there's no way to re-establish the connection.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/58358e40-b265-4e6b-f448-2c25de1dd55e)

#### `public void InitializeDataStorage(Object ViewModel)`

Initializes the datastorage and registers property bindings for defined viewmodel.

**Parameters:**
- `ViewModel` (System.Object) — The view model with StructuresDialog attribute definitions.

[Docs](https://developer.tekla.com/topic/en/18/43/f05aa98c-5ad3-cdd3-d48c-8f07bea99880)

#### `public void InitializeWindow()`

Prepares the data storage for the dialog and scans through the fields.

[Docs](https://developer.tekla.com/topic/en/18/43/716e8a1f-2eaa-7a43-faa8-fa816c0733fc)

#### `public void LoadValues(string FileName)`

Loads the dialog values from a file. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/fec236f4-fc77-fe84-6719-0f260217336c)

#### `public void ModifyValues(string FileName)`

Loads the dialog values from a file and performs Modify() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/51224621-7902-e1c1-1a2b-16aa41986f8f)

#### `public void SaveValues(string fileName)`

Serializes the dialog values to an xml file.

**Parameters:**
- `fileName` (System.String) — The path of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/8a54d96d-6630-b17e-f547-8f78776f9a79)

#### `public void ShowWindow()`

Displays the window.

[Docs](https://developer.tekla.com/topic/en/18/43/982b1878-5d0d-cbd6-6f7c-00ae2ea754b1)

#### `public void UpdateDataStorageFromViewModel(Object viewModelClass)`

DO NOT USE! For internal usage only!

**Parameters:**
- `viewModelClass` (System.Object)

[Docs](https://developer.tekla.com/topic/en/18/43/e3436e31-3a09-9c03-dba7-d531beb44a15)

### Properties
- `Localization` (Localization, get) — The localization instance for the dialog. Each dialog has its own localization instance that has read the localization files needed for that dialog.
- `LocExtension` (LocExtension, get) — The localization instance for the dialog. Each dialog has its own localization instance that has read the localization files needed for that dialog.
- `UseDefaultStyle` (Boolean, get/set) — Use default style in all the window controls or not. If the property is not set in the window constructor the default value (true) is used.

## AttributeTypeNameEditor (class)

The AttributeTypeNameEditor class provides a user interface for selecting a type for the bound attribute.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/70d79b05-cbe6-7dff-fd3d-e673eb494245)

### Constructors
- `public AttributeTypeNameEditor()` — Initializes a new instance of the AttributeTypeNameEditor class

### Methods
#### `public override Object EditValue(ITypeDescriptorContext context, IServiceProvider provider, Object value)`

Edits the specified object's value using the editor style indicated by the GetEditStyle method.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — An ITypeDescriptorContext that can be used to gain additional context information.
- `provider` (System.IServiceProvider) — An IServiceProvider that the editor can use to obtain services.
- `value` (System.Object) — The object to edit.

**Returns:** `Object` — The new value of the object. If the value of the object has not changed, this should return the same object it was passed.

[Docs](https://developer.tekla.com/topic/en/18/43/ae0badca-7025-84e6-60fe-bbfe4f8ea159)

#### `public override UITypeEditorEditStyle GetEditStyle(ITypeDescriptorContext context)`

Gets the editor style used by the EditValue method.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — An ITypeDescriptorContext that can be used to gain additional context information.

**Returns:** `UITypeEditorEditStyle` — A UITypeEditorEditStyle value that indicates the style of the editor used by the EditValue method. If the UITypeEditor does not support the method, then GetEditStyle will return None.

[Docs](https://developer.tekla.com/topic/en/18/43/95040739-3a4f-19d6-197f-ccc3baae80d2)

## BindPropertyNameEditor (class)

The BindPropertyNameEditor class provides a user interface for selecting a property for the attribute binding.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/0f3291e8-c5d8-b8a0-393c-b8fbd856e46d)

### Constructors
- `public BindPropertyNameEditor()` — Initializes a new instance of the BindPropertyNameEditor class

### Methods
#### `public override Object EditValue(ITypeDescriptorContext context, IServiceProvider provider, Object value)`

Edits the specified object's value using the editor style indicated by the GetEditStyle method.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — An ITypeDescriptorContext that can be used to gain additional context information.
- `provider` (System.IServiceProvider) — An IServiceProvider that the editor can use to obtain services.
- `value` (System.Object) — The object to edit.

**Returns:** `Object` — The new value of the object. If the value of the object has not changed, this should return the same object it was passed.

[Docs](https://developer.tekla.com/topic/en/18/43/3514de25-6ae6-f277-0fc1-f2d29bb86009)

#### `public override UITypeEditorEditStyle GetEditStyle(ITypeDescriptorContext context)`

Gets the editor style used by the EditValue method.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — An ITypeDescriptorContext that can be used to gain additional context information.

**Returns:** `UITypeEditorEditStyle` — A UITypeEditorEditStyle value that indicates the style of the editor used by the EditValue method. If the UITypeEditor does not support the method, then GetEditStyle will return None.

[Docs](https://developer.tekla.com/topic/en/18/43/a84dddb2-7bd1-afe7-7be8-3d5949a9971d)

## Dialogs (class)

The Dialogs class contains interface methods for Tekla Structures to handle FormBase and WindowBase dialogs.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/52eae4f7-9a52-7a70-e14d-53fea8d7cd11)

### Constructors
- `public Dialogs()` — Initializes a new instance of the Dialogs class

### Methods
#### `public static void ClosePluginDialogs()`

Close all dialogs to end the threads running the corresponding message loops.

[Docs](https://developer.tekla.com/topic/en/18/43/698fc56b-5264-a788-abf8-77f44b020c14)

#### `public static PluginFormBase GetPluginFormBaseByPluginFormName(string pluginFormName)`

Retrieves an instance of the form.

**Parameters:**
- `pluginFormName` (System.String) — The complete name of the plug-in form (including the plug-in name).

**Returns:** `PluginFormBase` — The existing PluginFormBase or null if it did not exist.

[Docs](https://developer.tekla.com/topic/en/18/43/b8a8fc67-20a4-fad5-a03e-d42b21116129)

#### `public static PluginWindowBase GetPluginWindowBaseByPluginFormName(string pluginFormName)`

Retrieves an instance of the form.

**Parameters:**
- `pluginFormName` (System.String) — The complete name of the plug-in form (including the plug-in name).

**Returns:** `PluginWindowBase` — The existing PluginWindowBase or null if it did not exist.

[Docs](https://developer.tekla.com/topic/en/18/43/19217d5d-dce5-0d67-01b4-4ff4d932e553)

#### `public static int LoadAttributeFileNameToDialogAndApply(string param)`

Uses the attributes found from the attribute file as the applied values for the specified dialog.

**Parameters:**
- `param` (System.String) — Not used.

**Returns:** `Int32` — 0 always. The return value is not useful because it is not sent to the Tekla Structures core.

[Docs](https://developer.tekla.com/topic/en/18/43/771f0656-ff9d-342d-832f-129141070c13)

#### `public static bool LoadAttributeFileNameToDialogAndApply(string attributeFileName, string pluginFormName)`

Loads the specified dialog's applied values from the attribute file.

**Parameters:**
- `attributeFileName` (System.String) — The name of the attribute file.
- `pluginFormName` (System.String) — The full name of the dialog.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/a281b25e-ce09-4aaf-2de1-e480a0fe0c65)

#### `public static int LoadAttributeFileNameToDialogAndModify(string param)`

Uses the attributes found from the attribute file as the current values for the specified dialog. Modifies the current selected instance. Applied values for this dialog are not changed.

**Parameters:**
- `param` (System.String) — Not used.

**Returns:** `Int32` — 0 always. The return value is not useful because it is not sent to the Tekla Structures core.

[Docs](https://developer.tekla.com/topic/en/18/43/850a8d13-508b-642f-9416-0e854a297724)

#### `public static bool LoadAttributeFileNameToDialogAndModify(string attributeFileName, string pluginFormName)`

Loads the specified dialog's current values from the attribute file and modifies the current selected instance.

**Parameters:**
- `attributeFileName` (System.String) — The name of the attribute file.
- `pluginFormName` (System.String) — The full name of the dialog.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/642fbb30-b657-7e34-0fbf-e74f87a70a0c)

#### `public static int LoadAttributeFileNameToDialogAndSave(string param)`

Uses the attributes found from the attribute file for the specified dialog and saves to file.

**Parameters:**
- `param` (System.String) — Not used.

**Returns:** `Int32` — 0 always. The return value is not useful because it is not sent to the Tekla Structures core.

[Docs](https://developer.tekla.com/topic/en/18/43/f41b14e4-1742-ac10-e3e4-bd1221adc387)

#### `public static int LoadAttributeFileToStack(string param)`

Opens a dialog and forces it to get the values from the currently selected part.

**Parameters:**
- `param` (System.String) — Not used.

**Returns:** `Int32` — 0 always.

[Docs](https://developer.tekla.com/topic/en/18/43/f1a177b3-2bf6-99af-bdd4-c1f8dd428c25)

#### `public static int LoadDialogs(string param)`

Reads the PluginFormBase and PluginWindowBase dialogs from all found dll files in bin\dialogs and bin\plugins.

**Parameters:**
- `param` (System.String) — Not used.

**Returns:** `Int32` — 0 always. The return value is not useful because it is not sent to the Tekla Structures core.

[Docs](https://developer.tekla.com/topic/en/18/43/110d4f4b-a6cd-30c1-cf6b-cc4e990810fa)

#### `public static int OpenDialog(string param)`

Opens a dialog. The name of the dialog is asked from the Tekla Structures core.

**Parameters:**
- `param` (System.String) — Not used.

**Returns:** `Int32` — 0 always.

[Docs](https://developer.tekla.com/topic/en/18/43/9ce7137b-d59d-1a3c-d21a-802a942a118a)

#### `public static int OpenDialogAndGet(string param)`

Opens a dialog and forces it to get the values from the currently selected part.

**Parameters:**
- `param` (System.String) — Not used.

**Returns:** `Int32` — 0 always.

[Docs](https://developer.tekla.com/topic/en/18/43/bc1d25fc-6855-5d1a-d03a-bb403f3af5f8)

#### `public static int ReloadDialogs(string param)`

Reloads all PluginFormBase dialogs with default/standard values.

**Parameters:**
- `param` (System.String) — Not used.

**Returns:** `Int32` — 0 always.

[Docs](https://developer.tekla.com/topic/en/18/43/d4b1485e-18d0-b714-17f8-90607c4f1af1)

#### `public static int SetSettings(string param)`

Forces the dialog system to get the language and localization settings from Tekla Structures.

**Parameters:**
- `param` (System.String) — Not used.

**Returns:** `Int32` — 0 always.

[Docs](https://developer.tekla.com/topic/en/18/43/f56ab8b4-e80c-e35e-8f4d-165105ab70e9)

## ErrorDialog (class)

The ErrorDialog class represents the common error dialog for the Open API.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/b4c6e3f4-243d-34ee-f527-4bc0517919ed)

### Methods
#### `public static void Show(Exception Ex, ErrorDialog.Severity Severity)`

Shows a new error dialog.

**Parameters:**
- `Ex` (System.Exception) — The exception to display in the dialog.
- `Severity` (Tekla.Structures.Dialog.ErrorDialog.Severity) — The severity of the exception.

[Docs](https://developer.tekla.com/topic/en/18/43/4c2c999d-3234-5abd-04ba-b9eb664972c6)

#### `public static void Show(string Message, List<string> Details, ErrorDialog.Severity Severity)`

Shows a new error dialog.

**Parameters:**
- `Message` (System.String) — The message to display in the dialog.
- `Details` (System.Collections.Generic.List<String>) — The details to display in the dialog.
- `Severity` (Tekla.Structures.Dialog.ErrorDialog.Severity) — The severity of the message.

[Docs](https://developer.tekla.com/topic/en/18/43/db4cb25f-92be-3852-aa92-df83625e0677)

#### `public static void Show(string Message, string Details, ErrorDialog.Severity Severity)`

Shows a new error dialog.

**Parameters:**
- `Message` (System.String) — The message to display in the dialog.
- `Details` (System.String) — The details to display in the dialog.
- `Severity` (Tekla.Structures.Dialog.ErrorDialog.Severity) — The severity of the message.

[Docs](https://developer.tekla.com/topic/en/18/43/5292fa06-3a04-0624-86ac-2c1d1f0e7385)

## ErrorDialog.Severity (enum)

The severity of the message to be displayed.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/ca65bfc1-0d16-171d-c5c0-0241308720ad)

### Values
- `INFO` = `0` — The message is for informing.
- `WARNING` = `1` — The message is for warning.
- `ERROR` = `2` — The message is for notifying that an error has occurred.

## FormBase (class)

The FormBase class is the base class for all Tekla Structures dialogs. The class provides localization, unit conversion and data storage (temporary and file-based) among other things.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/97b42e5f-994c-d593-c6b3-12c9754c64e9)

### Constructors
- `public FormBase()` — The default constructor that does not need parameters. Initializes the form and registers property bindings.

### Methods
#### `public void ApplyValues(string FileName)`

Loads the dialog values from a file and performs Apply() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/c4096cdf-60d9-f7b8-1daf-e71de20cf47d)

#### `public bool GetConnectionStatus()`

Returns true if a proper connection to the Tekla Structures process has been established. Currently, there's no way to re-establish the connection.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/2ba6677a-b8a7-cbfa-2724-e58464054abe)

#### `public static int GetUnitDecimals(dotdiaUnitTypes_e unit)`

Gets the number of decimals Tekla Structures uses with the specified unit type.

**Parameters:**
- `unit` (Tekla.Structures.Dialog.dotdiaUnitTypes_e) — The specified unit type.

**Returns:** `Int32` — The number of decimals used, 0 (no decimals or invalid unit type) or -1 (failed).

[Docs](https://developer.tekla.com/topic/en/18/43/f9d16d75-bcae-8cbd-7942-32d75e9596e6)

#### `public static bool InitializeAngleUnitDecimals()`

Gets the number of decimals Tekla Structures uses for displaying the angle unit and initializes Angle class to use that.

**Returns:** `Boolean` — True if successful. False if failed.

[Docs](https://developer.tekla.com/topic/en/18/43/11346bd3-a37a-e8c2-b713-52b409b2cb0e)

#### `public static bool InitializeDistanceUnitDecimals()`

Gets the number of decimals Tekla Structures uses for displaying the "length" unit and initializes the distance datatype to use that.

**Returns:** `Boolean` — True if successful. False if failed.

[Docs](https://developer.tekla.com/topic/en/18/43/463b2b8f-138b-60aa-bf50-a29ec3518a43)

#### `public void InitializeForm()`

Prepares the data storage for the dialog and scans through the fields.

[Docs](https://developer.tekla.com/topic/en/18/43/70881deb-ea45-96c7-ee73-399bc7b76a90)

#### `public static bool InitializeUnitDecimals()`

Gets the number of decimals Tekla Structures uses for displaying its units and initializes the Tekla.Structures.Datatype classes to use them.

**Returns:** `Boolean` — True if successful. False if failed.

[Docs](https://developer.tekla.com/topic/en/18/43/956a00ac-ad50-6743-5071-6e8af6f88e9f)

#### `public void LoadValues(string FileName)`

Loads the dialog values from a file. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/df11f56f-8372-2886-e011-b4fcad4ac3de)

#### `public void ModifyValues(string FileName)`

Loads the dialog values from a file and performs Modify() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/d20a7c4e-e913-92e5-0925-44015cc68ebf)

#### `public void SaveValues(string fileName)`

Serializes the dialog values to an xml file.

**Parameters:**
- `fileName` (System.String) — The path of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/a817a258-86a2-8616-be00-eda92c7e5f8e)

#### `public void SetAttributeValue(Control Ctrl, Object Value)`

Sets a value for the given control. When the dialog is not shown, setting a property directly for a control (such as textBox1.Text = "text") will not work for controls that have a Tekla Structures AttributeTypeName set. This method is going to have to be used to set the value.

**Parameters:**
- `Ctrl` (System.Windows.Forms.Control) — The control whose value to set.
- `Value` (System.Object) — The new value. It can be of any primitive datatype supported by the control's AttributeType (normally int, double or string).

[Docs](https://developer.tekla.com/topic/en/18/43/30e56a80-1c86-4606-781c-2a91b7bb8884)

#### `public void ShowForm()`

Displays the form.

[Docs](https://developer.tekla.com/topic/en/18/43/79258182-6491-e7e6-99d0-de2072cb85a5)

#### `[ObsoleteAttribute("Does currently nothing. Will be removed in near future.")] public void UpdateValues()`

Rereads and updates all the field values on the form.

[Docs](https://developer.tekla.com/topic/en/18/43/5ab0c9d1-4ddf-1e05-29ca-560e387350e5)

### Properties
- `Localization` (Localization, get) — The localization instance for the dialog. Each dialog has its own localization instance that has read the localization files needed for that dialog.

## FormBorders (class)

The FormBorders class handles the storing and restoring of the form size and location.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/38618d89-2dee-bf3e-83c3-a3b07acd4443)

### Methods
#### `public static void CheckScreenPosition(Form form)`

Moves form to primary screen.

**Parameters:**
- `form` (System.Windows.Forms.Form) — The form in question.

[Docs](https://developer.tekla.com/topic/en/18/43/5bd0931f-14c8-aee1-586a-e08ff7e2ff29)

#### `public static void CheckScreenPosition(Window window)`

Moves window to primary screen.

**Parameters:**
- `window` (System.Windows.Window) — The window in question.

[Docs](https://developer.tekla.com/topic/en/18/43/de2a2b42-d0cf-6d8d-392f-053250d8565e)

#### `public static bool IsOnActiveDisplays(Form form)`

Checks form quadrants to be sure full extents is on a single screen

**Parameters:**
- `form` (System.Windows.Forms.Form) — The form in question.

**Returns:** `Boolean` — True if fully contained in a screen

[Docs](https://developer.tekla.com/topic/en/18/43/f2d7a679-8079-fe12-a885-46ca66614dac)

#### `public static void RestoreFormSizeAndLocation(Form form)`

Restores the form size and location from the registry.

**Parameters:**
- `form` (System.Windows.Forms.Form) — The form in question.

[Docs](https://developer.tekla.com/topic/en/18/43/30de9914-2bcc-b7ea-5ad6-e0c94bed5ee8)

#### `public static void RestoreFormSizeAndLocation(Window window)`

Restores the WPF window size and location from the registry.

**Parameters:**
- `window` (System.Windows.Window) — The form in question.

[Docs](https://developer.tekla.com/topic/en/18/43/a0beb78e-66ee-619d-fb5a-e69f327c566b)

#### `public static void StoreFormSizeAndLocation(Form form)`

Stores the form size and location into the registry.

**Parameters:**
- `form` (System.Windows.Forms.Form) — The form in question.

[Docs](https://developer.tekla.com/topic/en/18/43/2eaff581-4e11-bf5f-ba5d-07c079b015c8)

#### `public static void StoreFormSizeAndLocation(Window window)`

Stores the window size and location into the registry.

**Parameters:**
- `window` (System.Windows.Window) — The form in question.

[Docs](https://developer.tekla.com/topic/en/18/43/63047cff-3742-8792-52b6-ee14100ac1bf)

## HelpViewer (class)

Class for viewing help files with Tekla Structures HelpViewer.exe.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/5d571035-1ebd-4cf2-051e-415a19a65e2a)

### Methods
#### `public static bool DisplayHelpTopic(string helpTopic)`

Method for opening Tekla Structures help viewer with given help topic. Method requires Tekla Structures to be running.

**Parameters:**
- `helpTopic` (System.String) — Help topic (e.g. Help file name without extension).

**Returns:** `Boolean` — True if success, otherwise false.

[Docs](https://developer.tekla.com/topic/en/18/43/4b0c80b5-237d-c017-4355-cc5d42ea9087)

#### `public static bool DisplayHelpTopicIndependent(string helpViewerFilePath, string helpTopic, string language)`

Method for opening Tekla Structures help viewer independently from given file path with given help topic and language.

**Parameters:**
- `helpViewerFilePath` (System.String) — Full file path to HelpViewer.exe containing the name of the executable.
- `helpTopic` (System.String) — Help topic (e.g. Help file name without extension).
- `language` (System.String) — Help language. Defaults to enu.

**Returns:** `Boolean` — True if success, otherwise false.

[Docs](https://developer.tekla.com/topic/en/18/43/5ed14735-0448-5c67-ba91-447382f56925)

## LocExtension (class)

Localization binding for strings referred in xaml

[Vendor docs](https://developer.tekla.com/topic/en/18/43/643b6677-1792-f0ac-bcc8-80c759173637)

### Constructors
- `public LocExtension(string name)` — Localization extension for strings referred in xaml

## Localization (class)

The Localization class is for translating strings in .NET dialogs.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/1082f32b-492c-fd87-a2ba-b28a2359fc76)

### Constructors
- `public Localization()` — Creates a new localization instance.
- `public Localization(string fileName, string language)` — Creates a new localization instance.

### Methods
#### `public static string GetLocalizationFileFullPath(string fileName)`

Returns localization file if found from environment

**Parameters:**
- `fileName` (System.String) — filename to search

**Returns:** `String` — 

[Docs](https://developer.tekla.com/topic/en/18/43/814de45d-06ca-db66-f8a5-3fb668445041)

#### `public string GetText(string name)`

Gets the translation for the current language. The translations are searched with the given identifier string.

**Parameters:**
- `name` (System.String) — The identifier string.

**Returns:** `String` — The translated string.

[Docs](https://developer.tekla.com/topic/en/18/43/f8c92b4f-4ee9-fff6-8f2f-bc32c6558431)

#### `public void LoadAidFile(string fileName)`

Loads the localization strings from an aid file. Several files may be used concurrently, just call LoadAidFile for each file. The translations are searched in the loading order and the first match is returned.

**Parameters:**
- `fileName` (System.String) — The aid file to be loaded.

[Docs](https://developer.tekla.com/topic/en/18/43/e5f64544-6122-615c-fe3f-fe55dfd3bb15)

#### `public void LoadAilFile(string fileName)`

Loads the localization strings from an ail file. Several files may be used concurrently, just call LoadAilFile for each file. The translations are searched in the loading order and the first match is returned.

**Parameters:**
- `fileName` (System.String) — The ail file to be loaded.

[Docs](https://developer.tekla.com/topic/en/18/43/ce8fe818-f1b3-287c-2556-4aab6e4e4fde)

#### `public void LoadFile(string fileName)`

Loads the localization strings from a file. Several files may be used concurrently, just call LoadFile for each file. The translations are searched in the loading order and the first match is returned. The method uses the extension of the filename to identify the file type (".xml", ".aid" or ".ail").

**Parameters:**
- `fileName` (System.String) — The file to be loaded.

[Docs](https://developer.tekla.com/topic/en/18/43/f3061153-fc54-871b-7a3a-a61c7700ead8)

#### `public void LoadXMLFile(string fileName)`

Loads the localization strings from an xml file. Several files may be used concurrently, just call LoadXMLFile for each file. The translations are searched in the loading order and the first match is returned.

**Parameters:**
- `fileName` (System.String) — The xml file to be loaded.

[Docs](https://developer.tekla.com/topic/en/18/43/81d33ad8-0af5-e71e-f1e4-3e108eeef787)

#### `public void Localize(ApplicationSettingsBase applicationSettings)`

Localizes application settings.

**Parameters:**
- `applicationSettings` (System.Configuration.ApplicationSettingsBase) — The application settings that should be localized.

[Docs](https://developer.tekla.com/topic/en/18/43/8f64771f-7a89-67f8-7a18-e56d01b3260c)

#### `public void Localize(Control control)`

Localizes a control recursively.

**Parameters:**
- `control` (System.Windows.Forms.Control) — The control instance.

[Docs](https://developer.tekla.com/topic/en/18/43/83044e33-c832-1e45-2e54-71a15be3b7ab)

#### `public void Localize(MenuItem menuItem)`

Localizes a MenuItem control recursively.

**Parameters:**
- `menuItem` (System.Windows.Forms.MenuItem) — The MenuItem instance.

[Docs](https://developer.tekla.com/topic/en/18/43/985e14bc-0164-6f24-cc16-099d799c8527)

#### `public void LocalizeToolTip(Control control, ToolTip toolTip)`

Localizes a ToolTip control.

**Parameters:**
- `control` (System.Windows.Forms.Control) — The root control for all the controls which should be localized.
- `toolTip` (System.Windows.Forms.ToolTip) — The ToolTip instance.

[Docs](https://developer.tekla.com/topic/en/18/43/88453655-72be-4c55-8bf7-4bb7fad8df80)

#### `public void RegisterLocalizationCallback(Localization.LocalizationCallback cb, params Type[] types)`

Registers a localization callback which is used to translate the registered control types.

**Parameters:**
- `cb` (Tekla.Structures.Dialog.Localization.LocalizationCallback) — The localization delegate.
- `types` (.System.Type.) — An array of types which will trigger the current delegate.

[Docs](https://developer.tekla.com/topic/en/18/43/9c7c6069-4285-dda1-795d-a478948d46d5)

### Properties
- `DefaultLocalizationFile` (String, get) — The path and name of the default localization file.
- `DefaultLocalizationPath` (String, get) — The path and directory name where to find localization files.
- `Language` (String, get/set) — Gets or sets the language that is currently used in Tekla Structures.

## Localization.LocalizationCallback (delegate)

A delegate for localization functions for different control types.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/bea6095a-d1ee-2738-2e50-936947cc217f)

## Localization.Util (class)

The Util class contains general localizers for different kinds of controls.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/aa928c88-9b75-5a1b-8794-73e855e9d5f0)

### Constructors
- `public Util()` — Initializes a new instance of the Localization.Util class

### Methods
#### `public static void LocalizeDataGridView(Localization localization, Object obj)`

Localizes a DataGridView.

**Parameters:**
- `localization` (Tekla.Structures.Dialog.Localization) — The localization instance to be used.
- `obj` (System.Object) — The object to be localized.

[Docs](https://developer.tekla.com/topic/en/18/43/d441aa17-4e68-79e7-f9f5-d7335ede5bf3)

#### `public static void LocalizeImageListComboBox(Localization localization, Object obj)`

Localizes an ImageListComboBox.

**Parameters:**
- `localization` (Tekla.Structures.Dialog.Localization) — The localization instance to be used.
- `obj` (System.Object) — The object to be localized.

[Docs](https://developer.tekla.com/topic/en/18/43/b545de48-174b-a35c-509f-59fa08f10ff9)

#### `public static void LocalizeListControl(Localization localization, Object obj)`

Localizes a List control.

**Parameters:**
- `localization` (Tekla.Structures.Dialog.Localization) — The localization instance to be used.
- `obj` (System.Object) — The object to be localized.

[Docs](https://developer.tekla.com/topic/en/18/43/8acd981d-ed5c-9a67-9e3f-a2e962900e88)

#### `public static void LocalizeToolStrip(Localization localization, Object obj)`

Localizes a ToolStrip.

**Parameters:**
- `localization` (Tekla.Structures.Dialog.Localization) — The localization instance to be used.
- `obj` (System.Object) — The object to be localized.

[Docs](https://developer.tekla.com/topic/en/18/43/3373bd2e-3e18-5f90-30e9-5fabcae9fb9b)

## Localization.Util.PropertyLocalizer (class)

The PropertyLocalizer class localizes properties.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/165a5181-3757-e188-e0eb-ba009b65c83f)

### Constructors
- `public PropertyLocalizer(string propertyName)` — Creates a new instance of the localizer.

### Methods
#### `public void Localize(Localization localization, Object obj)`

Localizes the object with the given localization instance.

**Parameters:**
- `localization` (Tekla.Structures.Dialog.Localization) — The localization instance to be used.
- `obj` (System.Object) — The object to be localized.

[Docs](https://developer.tekla.com/topic/en/18/43/f0d889df-cd25-7dd5-b112-9359562b4e68)

## MainWindow (class)

The MainWindow class is a class for binding the .NET dialogs under the Tekla Structures main window.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/0e7f0921-9e97-2c96-9ea6-6a34e1552ec5)

### Methods
#### `public void AddExternalWindow(string Name, IntPtr Handle)`

Registers the entire dialog to be insensitive at certain points, for example while modal Tekla Structures system dialog boxes are open. When insensitive, the dialog does not respond to any input. You can remove the registration of the dialog with RemoveExternalWindow(String, IntPtr).

**Parameters:**
- `Name` (System.String) — The unique name of the form.
- `Handle` (System.IntPtr) — The handle to the form that is to be added.

[Docs](https://developer.tekla.com/topic/en/18/43/92595806-5cd0-f58c-9ecb-569eac5bcfc0)

#### `public void RemoveExternalWindow(string Name, IntPtr Handle)`

Removes a registration done with AddExternalWindow(String, IntPtr).

**Parameters:**
- `Name` (System.String) — The unique name of the form.
- `Handle` (System.IntPtr) — The handle to the form that is to be removed.

[Docs](https://developer.tekla.com/topic/en/18/43/9e63470d-b3c6-e44c-eba7-619d7e7e1015)

### Properties
- `Handle` (IntPtr, get) — Gets the handle.

## PluginFormBase (class)

The PluginFormBase class is the base class for plug-in dialogs. The class extends the FormBase class by adding communications with Tekla Structures.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/f0c64315-eb6a-0894-b58a-6fbb077c4d83)

### Constructors
- `public PluginFormBase()` — Runs the FormBase constructor and loads the default .NET localization file (DotNetDialogStrings.ail).

### Methods
#### `public void ApplyValues(string FileName)`

Loads the dialog values from a file and performs Apply() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/c4096cdf-60d9-f7b8-1daf-e71de20cf47d)

#### `public void Get()`

Gets the dialog values from the part that is currently selected in Tekla Structures.

[Docs](https://developer.tekla.com/topic/en/18/43/16e61cb7-daae-4b10-346e-dd62088f996c)

#### `public bool GetConnectionStatus()`

Returns true if a proper connection to the Tekla Structures process has been established. Currently, there's no way to re-establish the connection.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/2ba6677a-b8a7-cbfa-2724-e58464054abe)

#### `public void InitializeForm()`

Prepares the data storage for the dialog and scans through the fields.

[Docs](https://developer.tekla.com/topic/en/18/43/70881deb-ea45-96c7-ee73-399bc7b76a90)

#### `public void LoadValues(string FileName)`

Loads the dialog values from a file. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/df11f56f-8372-2886-e011-b4fcad4ac3de)

#### `public void ModifyValues(string FileName)`

Loads the dialog values from a file and performs Modify() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/d20a7c4e-e913-92e5-0925-44015cc68ebf)

#### `public void ReloadForm()`

Reloads the dialog values.

[Docs](https://developer.tekla.com/topic/en/18/43/22d01ecf-f197-4f85-9648-43ade2a66e70)

#### `public void SaveValues(string fileName)`

Serializes the dialog values to an xml file.

**Parameters:**
- `fileName` (System.String) — The path of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/a817a258-86a2-8616-be00-eda92c7e5f8e)

#### `public void SetAttributeValue(Control Ctrl, Object Value)`

Sets a value for the given control. When the dialog is not shown, setting a property directly for a control (such as textBox1.Text = "text") will not work for controls that have a Tekla Structures AttributeTypeName set. This method is going to have to be used to set the value.

**Parameters:**
- `Ctrl` (System.Windows.Forms.Control) — The control whose value to set.
- `Value` (System.Object) — The new value. It can be of any primitive datatype supported by the control's AttributeType (normally int, double or string).

[Docs](https://developer.tekla.com/topic/en/18/43/30e56a80-1c86-4606-781c-2a91b7bb8884)

#### `public void ShowForm()`

Displays the form.

[Docs](https://developer.tekla.com/topic/en/18/43/79258182-6491-e7e6-99d0-de2072cb85a5)

#### `[ObsoleteAttribute("Does currently nothing. Will be removed in near future.")] public void UpdateValues()`

Rereads and updates all the field values on the form.

[Docs](https://developer.tekla.com/topic/en/18/43/5ab0c9d1-4ddf-1e05-29ca-560e387350e5)

### Properties
- `Localization` (Localization, get) — The localization instance for the dialog. Each dialog has its own localization instance that has read the localization files needed for that dialog.
- `ShowInTaskbar` (Boolean, get/set) — Hides (shadows) the ShowInTaskbar property by setting the property to false.

### Events
#### `AttributesLoadedFromModel` (`System.EventHandler`)

**Signature:** `public event EventHandler AttributesLoadedFromModel`

The AttributesLoadedFromModel event is triggered just after the attributes have been loaded from the model into the dialog.

[Docs](https://developer.tekla.com/topic/en/18/43/cb3d5559-e85d-1152-f617-d47b3e2524ec)

## PluginWindowBase (class)

The PluginWindowBase class is the base class for plug-in WPF dialogs. The class extends the WindowBase class by adding communications with Tekla Structures.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/a9f51cb5-f0b6-bef3-d11f-ba0a18cef07e)

### Constructors
- `public PluginWindowBase()` — Runs the WindowBase constructor and loads the default .NET localization file (DotNetDialogStrings.ail). Datastorage is built based on given viewmodel object

### Methods
#### `public void ApplyValues(string FileName)`

Loads the dialog values from a file and performs Apply() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/76f35628-b668-e741-3bfe-e761a95e178b)

#### `public void Dispose()`

Cleans up any resources being used.

[Docs](https://developer.tekla.com/topic/en/18/43/f3c50357-af2a-4ef0-d579-68f74c93597d)

#### `public void Get()`

Gets the dialog values from the part that is currently selected in Tekla Structures.

[Docs](https://developer.tekla.com/topic/en/18/43/9ddaebb1-667a-29df-9e7f-097e0b546689)

#### `public bool GetConnectionStatus()`

Returns true if a proper connection to the Tekla Structures process has been established. Currently, there's no way to re-establish the connection.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/58358e40-b265-4e6b-f448-2c25de1dd55e)

#### `public void InitializeWindow()`

Prepares the data storage for the dialog and scans through the fields.

[Docs](https://developer.tekla.com/topic/en/18/43/716e8a1f-2eaa-7a43-faa8-fa816c0733fc)

#### `public void LoadValues(string FileName)`

Loads the dialog values from a file. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/fec236f4-fc77-fe84-6719-0f260217336c)

#### `public void ModifyValues(string FileName)`

Loads the dialog values from a file and performs Modify() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/51224621-7902-e1c1-1a2b-16aa41986f8f)

#### `public void ReloadWindow()`

Reloads the dialog values.

[Docs](https://developer.tekla.com/topic/en/18/43/89465fc0-e969-1e83-112f-6173a1ec6c41)

#### `public void SaveValues(string fileName)`

Serializes the dialog values to an xml file.

**Parameters:**
- `fileName` (System.String) — The path of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/8a54d96d-6630-b17e-f547-8f78776f9a79)

#### `public void ShowWindow()`

Displays the window.

[Docs](https://developer.tekla.com/topic/en/18/43/982b1878-5d0d-cbd6-6f7c-00ae2ea754b1)

#### `public void UpdateDataStorageFromViewModel(Object viewModelClass)`

DO NOT USE! For internal usage only!

**Parameters:**
- `viewModelClass` (System.Object)

[Docs](https://developer.tekla.com/topic/en/18/43/e3436e31-3a09-9c03-dba7-d531beb44a15)

### Properties
- `Localization` (Localization, get) — The localization instance for the dialog. Each dialog has its own localization instance that has read the localization files needed for that dialog.
- `LocExtension` (LocExtension, get) — The localization instance for the dialog. Each dialog has its own localization instance that has read the localization files needed for that dialog.
- `ShowInTaskbar` (Boolean, get/set) — Hides (shadows) the ShowInTaskbar property by setting the property to false.
- `UseDefaultStyle` (Boolean, get/set) — Use default style in all the window controls or not. If the property is not set in the window constructor the default value (true) is used.

### Events
#### `AttributesLoadedFromModel` (`System.EventHandler`)

**Signature:** `public event EventHandler AttributesLoadedFromModel`

The AttributesLoadedFromModel event is triggered just after the attributes have been loaded from the model into the dialog.

[Docs](https://developer.tekla.com/topic/en/18/43/33fe50af-47a5-a7a4-2cef-da777b898284)

## ProfileConversion (class)

The ProfileConversion class provides functionalities to convert profile strings from/to current units (set in "Units and decimals, Catalog, Profiles, Section Dimension" options).

[Vendor docs](https://developer.tekla.com/topic/en/18/43/5badd92c-86d5-4a1d-9ce9-3d03537ff562)

### Constructors
- `public ProfileConversion()` — Initializes a new instance of the ProfileConversion class

### Methods
#### `public static bool ConvertFromCurrentUnits(string Profile, ref string ConvertedProfile)`

Converts a profile string from current units to internal units (set in Units and decimals options).

**Parameters:**
- `Profile` (System.String) — The profile string to convert.
- `ConvertedProfile` (System.String.) — The converted profile string.

**Returns:** `Boolean` — True if successfully converted the string.

[Docs](https://developer.tekla.com/topic/en/18/43/9d8e9aad-e97f-8cba-98f6-3648ca66d6a9)

#### `public static bool ConvertToCurrentUnits(string Profile, ref string ConvertedProfile)`

Converts a profile string from internal units to current units (set in Units and decimals options).

**Parameters:**
- `Profile` (System.String) — The profile string to convert.
- `ConvertedProfile` (System.String.) — The converted profile string.

**Returns:** `Boolean` — True if successfully converted the string.

[Docs](https://developer.tekla.com/topic/en/18/43/46ce1d12-a1f5-19e1-129f-94893992d71c)

## StructuresDialogArrayAttribute (class)

The StructuresDialogArrayAttribute class specifies the attribute name for which the attributed interface will be used as a list item definition. The attributed interface is used to define the structure of the DataGridView items. In other words it defines all the possible DataGridView columns. The interface is bound to the DataGridView by referencing the interface's attribute name from the DataGridView's Tag property.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/0f84d991-d0bb-53d0-9345-4cc00a58f738)

### Constructors
- `public StructuresDialogArrayAttribute(string attributeName)` — Initializes a new StructuresDialogArrayAttribute instance.

### Methods
#### `public override string ToString()`

Returns a string that represents the current StructuresDialogArrayAttribute object.

**Returns:** `String` — A string that represents the current StructuresDialogArrayAttribute object.

[Docs](https://developer.tekla.com/topic/en/18/43/caeb7abd-abec-fb6a-c19e-36256e304ae0)

### Properties
- `AttributeName` (String, get) — Gets the attribute name of the interface.

## StructuresDialogAttribute (class)

The StructuresDialogAttribute class specifies the attribute name to which the value of the property will be bound in Tekla Structures.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/1795a6fd-a41a-98af-ec52-cad837ec40d7)

### Constructors
- `public StructuresDialogAttribute(string attributeName)` — Initializes a new StructuresDialogAttribute instance.
- `public StructuresDialogAttribute(string attributeName, string attributeType)` — Initializes a new StructuresDialogAttribute instance.
- `public StructuresDialogAttribute(string attributeName, Type attributeType)` — Initializes a new StructuresDialogAttribute instance.

### Methods
#### `public static Type GetCorrectDialogAttributeType(string attributeType)`

Initializes a new StructuresDialogAttribute instance.

**Parameters:**
- `attributeType` (System.String) — The attribute type of the property, default is String.

**Returns:** `Type` — 

[Docs](https://developer.tekla.com/topic/en/18/43/6c375ab8-7c62-bbea-0f14-67d3bf9d0a5b)

#### `public override string ToString()`

Returns a string that represents the current StructuresDialogAttribute object.

**Returns:** `String` — A string that represents the current StructuresDialogAttribute object.

[Docs](https://developer.tekla.com/topic/en/18/43/2d85f6f4-d740-0286-b74a-ec7c92648b35)

### Properties
- `AttributeName` (String, get) — Gets the attribute name of the property.
- `AttributeType` (Type, get) — Gets the attribute type of the property.

## StructuresDialogFilterAttribute (class)

The StructuresDialogFilterAttribute class specifies the attribute name for which the value of the attributed property will be used as a filter in Tekla Structures. The return type of the attributed property must be boolean. The apply and modify operations will only commit/update those attributes which either do not have a filter property defined or the value of the filter property is true.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/57235798-67b4-a448-5a8b-8cb224973d5b)

### Constructors
- `public StructuresDialogFilterAttribute(string attributeName)` — Initializes a new StructuresDialogFilterAttribute instance.

### Methods
#### `public override string ToString()`

Returns a string that represents the current StructuresDialogFilterAttribute object.

**Returns:** `String` — A string that represents the current StructuresDialogFilterAttribute object.

[Docs](https://developer.tekla.com/topic/en/18/43/b9cfb05a-79d5-038a-52e7-3857c613a07a)

### Properties
- `AttributeName` (String, get) — Gets the attribute name of the property.

## StructuresExtender (class)

The StructuresExtender class is for binding controls to datatypes. The bindings are needed to transfer the dialog values to Tekla Structures and onwards to plug-ins.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/b3cdfd5b-1b70-d1fd-9714-2d5a3426be95)

### Constructors
- `public StructuresExtender()` — Initializes a new instance of the StructuresExtender class

### Methods
#### `public bool CanExtend(Object extendee)`

Specifies whether the current object can provide its extender properties to the specified object.

**Parameters:**
- `extendee` (System.Object) — The object to receive the extender properties.

**Returns:** `Boolean` — True if the current object can provide its extender properties to the specified object; otherwise, false.

[Docs](https://developer.tekla.com/topic/en/18/43/e420da2e-767d-c932-be01-ebb557ff30b3)

#### `public string GetAttributeName(Control control)`

Retrieves the AttributeName text associated with the specified control.

**Parameters:**
- `control` (System.Windows.Forms.Control) — The control for which to retrieve the AttributeName text.

**Returns:** `String` — A string containing the AttributeName text for the specified control, or null if no text is associated with the specified control.

[Docs](https://developer.tekla.com/topic/en/18/43/d3425d49-18cb-66e5-9c9e-91de523b1d1f)

#### `public string GetAttributeTypeName(Control control)`

Retrieves the AttributeTypeName text associated with the specified control.

**Parameters:**
- `control` (System.Windows.Forms.Control) — The control for which to retrieve the AttributeTypeName text.

**Returns:** `String` — A string containing the AttributeTypeName text for the specified control, or null if no text is associated with the specified control.

[Docs](https://developer.tekla.com/topic/en/18/43/f6f015d4-b5e4-74d1-f9d3-0902b7bd6d4d)

#### `public string GetBindPropertyName(Control control)`

Retrieves the BindPropertyName text associated with the specified control.

**Parameters:**
- `control` (System.Windows.Forms.Control) — The control for which to retrieve the BindPropertyName text.

**Returns:** `String` — A string containing the BindPropertyName text for the specified control, or null if no text is associated with the specified control.

[Docs](https://developer.tekla.com/topic/en/18/43/4beb42d7-2857-08f0-de96-454e6993d294)

#### `public bool GetIsFilter(Control control)`

Retrieves the IsFilter boolean value associated with the specified control.

**Parameters:**
- `control` (System.Windows.Forms.Control) — The control for which to retrieve the IsFilter value.

**Returns:** `Boolean` — A boolean containing the IsFilter value for the specified control.

[Docs](https://developer.tekla.com/topic/en/18/43/aa096d0b-8bb2-a7b4-5ef2-0a823abf2b1a)

#### `public void SetAttributeName(Control control, string value)`

Associates AttributeName text with the specified control.

**Parameters:**
- `control` (System.Windows.Forms.Control) — The control to associate the AttributeName text with.
- `value` (System.String) — The AttributeName text to associate with the control.

[Docs](https://developer.tekla.com/topic/en/18/43/be684d7c-2d8e-23f8-2322-f613569ab409)

#### `public void SetAttributeTypeName(Control control, string value)`

Associates the AttributeTypeName text with the specified control.

**Parameters:**
- `control` (System.Windows.Forms.Control) — The control to associate the AttributeTypeName text with.
- `value` (System.String) — The AttributeTypeName text to associate with the control.

[Docs](https://developer.tekla.com/topic/en/18/43/caf131b4-139f-c16e-adce-aaf58eeb48a1)

#### `public void SetBindPropertyName(Control control, string value)`

Associates the BindPropertyName text with the specified control.

**Parameters:**
- `control` (System.Windows.Forms.Control) — The control to associate the BindPropertyName text with.
- `value` (System.String) — The BindPropertyName text to associate with the control.

[Docs](https://developer.tekla.com/topic/en/18/43/d9bc1d5e-9a6e-a099-f857-1a2f5af61a47)

#### `public void SetIsFilter(Control control, bool value)`

Associates the IsFilter boolean value with the specified control.

**Parameters:**
- `control` (System.Windows.Forms.Control) — The control to associate the IsFilter value with.
- `value` (System.Boolean) — The IsFilter value to associate with the control.

[Docs](https://developer.tekla.com/topic/en/18/43/938428a6-0561-e6ae-5376-ded7218ff601)

## StructuresInstallation (class)

Utility functions.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/ec1ff0da-9d71-fc0b-77a4-02e68f94a5db)

### Methods
#### `public static string GetLocalizationFile(string messageFile)`

Retrieves localization file from configured environment

**Parameters:**
- `messageFile` (System.String) — File name of message file.

**Returns:** `String` — Localization file full path

[Docs](https://developer.tekla.com/topic/en/18/43/2e157355-4890-ddfb-0749-32605c12f23b)

#### `public static string GetLocalizationFile(string subPath, string messageFile)`

Retrieves localization file from configured environment

**Parameters:**
- `subPath` (System.String) — Possible sub folder of message file. Empty sub folder has no effect.
- `messageFile` (System.String) — File name of message file.

**Returns:** `String` — localization file full path

[Docs](https://developer.tekla.com/topic/en/18/43/64580a6b-09f0-4bc5-b166-52bfa2590ffe)

### Properties
- `BinFolder` (String, get) — Gets Path to Tekla Structures binary (bin) directory.
- `EnvBaseFolder` (String, get) — Gets Path to Tekla Structures XSDATADIR directory. XSDATADIR is defined in the teklastructures.ini file. It points to a location, where the installation installs the environment files and folders. Example: C:\ProgramData\Trimble\Tekla Structures\Version
- `InstallFolder` (String, get) — Gets path to Tekla Structures installation directory.
- `MessageFolder` (String, get) — Gets path to Tekla Structures messages directory.
- `MessagesFolder` (SortedSet<String>, get) — Gets a list Paths to Tekla Structures messages directories.

## TeklaProgressBar (class)

The TeklaProgressBar class represents the common progress bar control for the Open API.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/f0334e42-760a-9d16-c350-68cac1ccfb36)

### Constructors
- `public TeklaProgressBar(string text = "albl_Processing_Please_wait...")` — Initializes the common progress bar control.

### Properties
- `Text` (String, get/set) — Gets or sets the current message text of the progress bar.
- `Value` (Int32, get/set) — Gets or sets the current position of the progress bar.

### Events
#### `CancelClicked` (`System.EventHandler`)

**Signature:** `public event EventHandler CancelClicked`

The CancelClicked event is raised when the cancel button is clicked.

[Docs](https://developer.tekla.com/topic/en/18/43/3c499df9-b2aa-a9b1-1766-446ecfbb022a)

## WindowBase (class)

The WindowBase class is the base class for all Tekla Structures WPF dialogs. The class provides localization, unit conversion and data storage (temporary and file-based) among other things.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/27802d58-3aa3-6f9c-6c3b-21ca856eaab5)

### Constructors
- `public WindowBase()` — The default constructor that does not need parameters. Initializes the window and registers property bindings.

### Methods
#### `public void ApplyValues(string FileName)`

Loads the dialog values from a file and performs Apply() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/76f35628-b668-e741-3bfe-e761a95e178b)

#### `public bool GetConnectionStatus()`

Returns true if a proper connection to the Tekla Structures process has been established. Currently, there's no way to re-establish the connection.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/58358e40-b265-4e6b-f448-2c25de1dd55e)

#### `public static int GetUnitDecimals(dotdiaUnitTypes_e unit)`

Gets the number of decimals Tekla Structures uses with the specified unit type.

**Parameters:**
- `unit` (Tekla.Structures.Dialog.dotdiaUnitTypes_e) — The specified unit type.

**Returns:** `Int32` — The number of decimals used, 0 (no decimals or invalid unit type) or -1 (failed).

[Docs](https://developer.tekla.com/topic/en/18/43/ab39e575-5bd4-7e45-c6b5-b1c51bbc10f4)

#### `public static bool InitializeAngleUnitDecimals()`

Gets the number of decimals Tekla Structures uses for displaying the angle unit and initializes Angle class to use that.

**Returns:** `Boolean` — True if successful. False if failed.

[Docs](https://developer.tekla.com/topic/en/18/43/af5b7239-e320-7709-7db1-369d4fdfbddf)

#### `public static bool InitializeDistanceUnitDecimals()`

Gets the number of decimals Tekla Structures uses for displaying the "length" unit and initializes the distance datatype to use that.

**Returns:** `Boolean` — True if successful. False if failed.

[Docs](https://developer.tekla.com/topic/en/18/43/f6a549be-d401-5021-9d20-c7f6ff32ac82)

#### `public static bool InitializeUnitDecimals()`

Gets the number of decimals Tekla Structures uses for displaying its units and initializes the Tekla.Structures.Datatype classes to use them.

**Returns:** `Boolean` — True if successful. False if failed.

[Docs](https://developer.tekla.com/topic/en/18/43/773280d6-c14e-3706-ece9-d2eee4d0ad00)

#### `public void InitializeWindow()`

Prepares the data storage for the dialog and scans through the fields.

[Docs](https://developer.tekla.com/topic/en/18/43/716e8a1f-2eaa-7a43-faa8-fa816c0733fc)

#### `public void LoadValues(string FileName)`

Loads the dialog values from a file. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/fec236f4-fc77-fe84-6719-0f260217336c)

#### `public void ModifyValues(string FileName)`

Loads the dialog values from a file and performs Modify() on the loaded values. To match the files to a certain dialog, the file suffix is set as the dialog type's name.

**Parameters:**
- `FileName` (System.String) — The name of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/51224621-7902-e1c1-1a2b-16aa41986f8f)

#### `public void SaveValues(string fileName)`

Serializes the dialog values to an xml file.

**Parameters:**
- `fileName` (System.String) — The path of the file.

[Docs](https://developer.tekla.com/topic/en/18/43/8a54d96d-6630-b17e-f547-8f78776f9a79)

#### `public void ShowWindow()`

Displays the window.

[Docs](https://developer.tekla.com/topic/en/18/43/982b1878-5d0d-cbd6-6f7c-00ae2ea754b1)

#### `public void UpdateDataStorageFromViewModel(Object viewModelClass)`

DO NOT USE! For internal usage only!

**Parameters:**
- `viewModelClass` (System.Object)

[Docs](https://developer.tekla.com/topic/en/18/43/e3436e31-3a09-9c03-dba7-d531beb44a15)

### Properties
- `Localization` (Localization, get) — The localization instance for the dialog. Each dialog has its own localization instance that has read the localization files needed for that dialog.
- `LocExtension` (LocExtension, get) — The localization instance for the dialog. Each dialog has its own localization instance that has read the localization files needed for that dialog.
- `UseDefaultStyle` (Boolean, get/set) — Use default style in all the window controls or not. If the property is not set in the window constructor the default value (true) is used.

## dotdiaUnitTypes_e (enum)

Specifies the Tekla Structures unit types.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/69cd47d1-a0ce-64eb-5654-ae91fb1b9b89)

### Values
- `INPUT_BOOLEAN` = `-3` — The input boolean.
- `INPUT_INTEGER` = `-2` — The input integer.
- `INPUT_STRING` = `-1` — The input string.
- `INPUT_NONE` = `0` — The input none.
- `INPUT_RATIO_UNIT` = `1` — The input ratio unit.
- `INPUT_STRAIN_UNIT` = `2` — The input strain unit.
- `INPUT_ANGLE_UNIT` = `5` — The input angle unit.
- `OUTPUT_ANGLE_UNIT` = `6` — The output angle unit.
- `INPUT_SECTION_ANGLE_UNIT` = `7` — The input section angle unit.
- `INPUT_LENGTH_UNIT` = `10` — The input length unit.
- `OUTPUT_LENGTH_UNIT` = `11` — The output length unit.
- `INPUT_DEFORMATION_UNIT` = `12` — The input deformation unit.
- `OUTPUT_DEFORMATION_UNIT` = `13` — The output deformation unit.
- `INPUT_DIMENSION_UNIT` = `14` — The input dimension unit.
- `INPUT_RADIUSOFINERTIA_UNIT` = `16` — The input radius of inertia unit.
- `INPUT_AREA_UNIT` = `20` — The input area unit.
- `OUTPUT_REINFAREA_UNIT` = `21` — The output reinforced area unit.
- `OUTPUT_TRANSVREINF_UNIT` = `22` — The output transverse reinforcement unit.
- `INPUT_AREAPERLENGTH_UNIT` = `23` — The input area per length unit.
- `OUTPUT_VOLUME_UNIT` = `30` — The output volume unit.
- `INPUT_SECTIONMODULUS_UNIT` = `31` — The input section modulus unit.
- `INPUT_VOLUME_UNIT` = `32` — The input volume unit.
- `INPUT_MOMENTOFINERTIA_UNIT` = `40` — The input moment of inertia unit.
- `INPUT_TORSIONCONSTANT_UNIT` = `41` — The input torsion constant unit.
- `INPUT_WARPINGCONSTANT_UNIT` = `60` — The input warping constant unit.
- `INPUT_FORCE_UNIT` = `100` — The input force unit.
- `OUTPUT_FORCE_UNIT` = `101` — The output force unit.
- `INPUT_WEIGHT_UNIT` = `102` — The input weight unit.
- `OUTPUT_WEIGHT_UNIT` = `103` — The output weight unit.
- `INPUT_DISTRIBLOAD_UNIT` = `110` — The input distributed load unit.
- `OUTPUT_DISTRIBLOAD_UNIT` = `111` — The output distributed load unit.
- `INPUT_SPRINGCONSTANT_UNIT` = `112` — The input spring constant unit.
- `OUTPUT_MASSPERLENGTH_UNIT` = `113` — The output mass per length unit.
- `INPUT_SURFACELOAD_UNIT` = `120` — The input surface load unit.
- `OUTPUT_SURFACELOAD_UNIT` = `121` — The output surface load unit.
- `INPUT_STRENGTH_UNIT` = `122` — The input strength unit.
- `OUTPUT_STRESS_UNIT` = `123` — The output stress unit.
- `INPUT_MODULUS_UNIT` = `124` — The input modulus unit.
- `INPUT_DENSITY_UNIT` = `131` — The input density unit.
- `INPUT_MOMENT_UNIT` = `200` — The input moment unit.
- `OUTPUT_MOMENT_UNIT` = `201` — The output moment unit.
- `INPUT_DISTRIBMOMENT_UNIT` = `205` — The input distributed moment unit.
- `INPUT_ROTSPRINGCONST_UNIT` = `210` — The input rotation spring constant unit.
- `INPUT_TEMPERATURE_UNIT` = `300` — The input temperature unit.
- `OUTPUT_TEMPERATURE_UNIT` = `301` — The output temperature unit.
- `INPUT_THERMDILATCOEFF_UNIT` = `310` — The input thermal dilatation coefficient unit.
- `INPUT_FACTOR_UNIT` = `400` — The input factor unit.
- `INPUT_DATE_UNIT` = `401` — The input date unit.
- `INPUT_DATE_TIME_MIN_UNIT` = `402` — The input date time minutes unit.
- `INPUT_DATE_TIME_SEC_UNIT` = `403` — The input date time seconds unit.
- `INPUT_LENGTH_FRACTIONAL_IMPERIAL` = `1005` — The input length, fractional imperial.
- `INPUT_DEFORMATION_FRACTIONAL_IMPERIAL` = `1305` — The input deformation, fractional imperial.
- `INPUT_DIMENSION_FRACTIONAL_IMPERIAL` = `1405` — The input dimension, fractional imperial.
- `INPUT_RADIUSOFINERTIA_FRACTIONAL_IMPERIAL` = `1605` — The input radius of inertia, fractional imperial.
- `OUTPUT_LENGTH_FRACTIONAL_IMPERIAL` = `1105` — The output length, fractional imperial.
- `OUTPUT_DEFORMATION_FRACTIONAL_IMPERIAL` = `1305` — The output deformation, fractional imperial.

