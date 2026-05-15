---
name: tekla-structures-dialog
description: Tekla Open API - Tekla.Structures.Dialog namespace. Plugin dialog base classes, WinForms dialog, WPF dialog, plugin UI, localization, catalog selectors, profile picker, material picker, save load, attribute binding, user interface controls. This skill should be used when creating Tekla plugin dialogs, UI forms, or attribute editors.
---

# Tekla.Structures.Dialog API Reference (v2025)

## Overview

Dialog framework for Tekla Structures plugins. Provides base classes for WinForms (`FormBase`) and WPF (`WindowBase`) dialogs with built-in localization, unit conversion, data storage, and attribute binding to Tekla Structures.

## Dialog Hierarchy

```
FormBase (WinForms)
  ├── ApplicationFormBase (standalone app dialogs)
  │   ├── CommitAction, CreateDialog, OrganizerDialog, etc.
  │   └── PropertiesDialog, TreeViewDialog
  └── PluginFormBase (plugin dialogs with Tekla communication)

WindowBase (WPF)
  ├── ApplicationWindowBase (standalone WPF app dialogs)
  └── PluginWindowBase (WPF plugin dialogs with Tekla communication)
```

## Core Base Classes

### FormBase : Form (abstract)
> Base class for all Tekla Structures WinForms dialogs. Provides localization, unit conversion and data storage.

**Properties:**
- `Localization Localization { get; }` - dialog's localization instance

**Methods:**
- `void InitializeForm()` - initialize the form
- `void ShowForm()` - show the form
- `void LoadValues(string FileName)` - load attribute values from file
- `void SaveValues(string fileName)` - save attribute values to file
- `void ApplyValues(string FileName)` - apply attribute values
- `void ModifyValues(string FileName)` - modify attribute values
- `void UpdateValues()` - update dialog values
- `void SetAttributeValue(Control Ctrl, object Value)` - set control value
- `bool GetConnectionStatus()` - check Tekla connection

**Static Methods:**
- `static int GetUnitDecimals(dotdiaUnitTypes_e unit)`
- `static bool InitializeUnitDecimals()`
- `static bool InitializeDistanceUnitDecimals()`
- `static bool InitializeAngleUnitDecimals()`

### WindowBase : Window (abstract)
> Base class for all Tekla Structures WPF dialogs. Same capabilities as FormBase for WPF.

**Properties:**
- `Localization Localization { get; }` - dialog's localization instance
- `LocExtension LocExtension { get; }` - XAML localization binding
- `bool UseDefaultStyle { get; set; }` - use default Tekla style (default: true)

**Methods:**
- `void InitializeWindow()` - initialize the window
- `void ShowWindow()` - show the window
- `void LoadValues(string FileName)` / `SaveValues` / `ApplyValues` / `ModifyValues`
- `bool GetConnectionStatus()` - check Tekla connection
- `void UpdateDataStorageFromViewModel(object viewModelClass)` - sync ViewModel to data storage

### PluginFormBase : FormBase
> Base class for plugin WinForms dialogs. Adds Tekla communication.

**Properties:**
- `bool ShowInTaskbar { get; set; }` - always false for plugin dialogs

**Methods:**
- `void Get()` - get current values from Tekla
- `void ReloadForm()` - reload the form

### PluginWindowBase : WindowBase
> Base class for plugin WPF dialogs. Adds Tekla communication.

**Properties:**
- `bool ShowInTaskbar { get; set; }` - always false for plugin dialogs

**Methods:**
- `void Get()` - get current values from Tekla
- `void ReloadWindow()` - reload the window
- `void Dispose()` - dispose resources

### ApplicationFormBase : FormBase
> Base class for standalone application dialogs (non-plugin).

### ApplicationWindowBase : WindowBase
> Base class for standalone WPF application dialogs.

**Methods:**
- `void InitializeDataStorage(object ViewModel)` - init data storage from ViewModel

## Attribute Binding

### StructuresDialogAttribute : Attribute
> Binds a property value to a Tekla Structures attribute.

**Constructors:**
- `StructuresDialogAttribute(string attributeName)`
- `StructuresDialogAttribute(string attributeName, Type attributeType)`
- `StructuresDialogAttribute(string attributeName, string attributeType)`

**Properties:**
- `string AttributeName { get; }` - the bound attribute name
- `Type AttributeType { get; }` - the attribute type

### StructuresDialogFilterAttribute : Attribute
> Marks a boolean property as a filter for apply/modify operations.

- `string AttributeName { get; }` - the filtered attribute name

### StructuresDialogArrayAttribute : Attribute
> Marks an interface as a DataGridView item definition.

- `string AttributeName { get; }` - the attribute name

### StructuresExtender : Component
> Binds WinForms controls to Tekla datatypes for value transfer.

**Methods:**
- `string GetAttributeName(Control control)` / `SetAttributeName(...)`
- `string GetAttributeTypeName(Control control)` / `SetAttributeTypeName(...)`
- `string GetBindPropertyName(Control control)` / `SetBindPropertyName(...)`
- `bool GetIsFilter(Control control)` / `SetIsFilter(...)`
- `bool CanExtend(object extendee)`

## Localization

### Localization : MarshalByRefObject
> Translates strings in .NET dialogs.

**Constructors:**
- `Localization()`
- `Localization(string fileName, string language)`

**Properties:**
- `string Language { get; set; }` - current language

**Static Properties:**
- `string DefaultLocalizationFile { get; }`
- `string DefaultLocalizationPath { get; }`

**Methods:**
- `string GetText(string name)` - get translated string
- `void LoadFile(string fileName)` / `LoadAidFile` / `LoadAilFile` / `LoadXMLFile` - load translation files
- `void Localize(Control control)` - localize a WinForms control
- `void Localize(ApplicationSettingsBase settings)` - localize app settings
- `void Localize(MenuItem menuItem)` - localize menu item
- `void LocalizeToolTip(Control control, ToolTip toolTip)`
- `void RegisterLocalizationCallback(LocalizationCallback cb, Type[] types)`
- `static string GetLocalizationFileFullPath(string fileName)`

### LocExtension : Binding
> XAML localization binding for WPF dialogs.

**Constructor:** `LocExtension(string name)`

## Utility Classes

### Dialogs : MarshalByRefObject (static methods)
> Interface methods for Tekla to handle plugin dialogs.

- `static void ClosePluginDialogs()`
- `static PluginFormBase GetPluginFormBaseByPluginFormName(string name)`
- `static PluginWindowBase GetPluginWindowBaseByPluginFormName(string name)`
- `static bool LoadAttributeFileNameToDialogAndApply(string fileName, string pluginFormName)`
- `static bool LoadAttributeFileNameToDialogAndModify(string fileName, string pluginFormName)`
- `static int OpenDialog(string param)` / `OpenDialogAndGet` / `LoadDialogs` / `ReloadDialogs`

### ErrorDialog : Form (sealed, static methods)
> Common error/info dialog.

- `static void Show(string Message, string Details, Severity Severity)`
- `static void Show(string Message, List<string> Details, Severity Severity)`
- `static void Show(Exception Ex, Severity Severity)`

**Severity enum:** `INFO = 0`, `WARNING = 1`, `ERROR = 2`

### MainWindow
> Binding .NET dialogs under the Tekla main window.

- `IntPtr Handle { get; }`
- `void AddExternalWindow(string Name, IntPtr Handle)`
- `void RemoveExternalWindow(string Name, IntPtr Handle)`

### TeklaProgressBar : Form
> Common progress bar control.

- `TeklaProgressBar(string text)`
- `string Text { get; set; }` - message text
- `int Value { get; set; }` - progress position

### ProfileConversion (static methods)
> Convert profile strings from/to current units.

- `static bool ConvertFromCurrentUnits(string Profile, ref string ConvertedProfile)`
- `static bool ConvertToCurrentUnits(string Profile, ref string ConvertedProfile)`

### StructuresInstallation (static)
> Tekla installation paths.

- `string BinFolder { get; }`, `string EnvBaseFolder { get; }`, `string InstallFolder { get; }`, `string MessageFolder { get; }`
- `SortedSet<string> MessagesFolder { get; }`
- `static string GetLocalizationFile(string messageFile)` / with `subPath`

### FormBorders (static)
> Store/restore dialog size and position.

- `static void StoreFormSizeAndLocation(Form form)` / `(Window window)`
- `static void RestoreFormSizeAndLocation(Form form)` / `(Window window)`
- `static void CheckScreenPosition(Form form)` / `(Window window)`

### HelpViewer (static)
> View help files with Tekla HelpViewer.exe.

- `static bool DisplayHelpTopic(string helpTopic)`
- `static bool DisplayHelpTopicIndependent(string helpViewerFilePath, string helpTopic, string language)`

## UI Controls (Tekla.Structures.Dialog.UIControls)

### Catalog Selectors

| Control | WPF Variant | Purpose | Key Properties |
|---------|-------------|---------|----------------|
| `ProfileCatalog` | `WpfProfileCatalog` | Profile selection | `SelectedProfile`, `ButtonText` |
| `MaterialCatalog` | `WpfMaterialCatalog` | Material selection | `SelectedMaterial`, `ButtonText` |
| `ReinforcementCatalog` | `WpfReinforcementCatalog` | Rebar selection | `SelectedRebarSize`, `SelectedRebarGrade`, `SelectedRebarBendingRadius` |
| `MeshCatalog` | `WpfMeshCatalog` | Mesh selection | `SelectedMeshName`, `SelectedMeshGrade` |
| `ShapeCatalog` | `WpfShapeCatalog` | Shape selection | `SelectedShape` |
| `ComponentCatalog` | `WpfComponentCatalog` | Component selection | `SelectedName`, `SelectedNumber` |
| `BoltCatalogStandard` + `BoltCatalogSize` | `WpfBoltCatalogStandard` + `WpfBoltCatalogSize` | Bolt selection (always paired) | `LinkedBoltCatalogSize` |

### Selection Dialogs

| Dialog | Purpose | Key Property |
|--------|---------|-------------|
| `ProfileSelectionForm` | Profile picker dialog | `SelectedProfile` |
| `MaterialSelectionForm` | Material picker dialog | `SelectedMaterial` |
| `ReinforcementSelectionForm` | Rebar picker dialog | `SelectedSize`, `SelectedGrade`, `SelectedBendingRadius` |
| `MeshSelectionForm` | Mesh picker dialog | `SelectedMeshName`, `SelectedMeshGrade` |
| `ShapeSelectionForm` | Shape picker dialog | `SelectedShape` |

### Button Groups

| Control | WPF Variant | Buttons |
|---------|-------------|---------|
| `OkCancel` | - | OK, Cancel |
| `OkApplyCancel` | - | OK, Apply, Cancel |
| `CreateApplyCancel` | `WpfCreateApplyCancel` | Create, Apply, Cancel |
| `OkApplyModifyGetOnOffCancel` | `WpfOkApplyModifyGetOnOffCancel` | OK, Apply, Modify, Get, On/Off, Cancel |
| - | `WpfOkCreateCancel` | OK, Create, Cancel |

### Other Controls

| Control | Purpose |
|---------|---------|
| `SaveLoad` / `WpfSaveLoad` | Save/Load/SaveAs with help integration |
| `ImageComboBox` | ComboBox with images |
| `ImageListComboBox` | ComboBox with ImageList images |
| `DataGrid` | DataGridView with image support |
| `Tree` | TreeView with image support |
| `BindableRadioButton` | RadioButton bindable to attributes |
| `WpfFilterCheckBox` | CheckBox for enable/disable dialog attributes |
| `WpfSteelFinishComboBox` | Steel finish selector |
| `ModelAccess` | Helper for connecting to model |
| `EnvironmentFiles` | Attribute file path resolution |
| `EnvironmentVariables` | Environment variable access |
| `LoadingForm` | Loading dialog |
| `LocalizeForm` | Form localization helper |

## Unit Types Enum (dotdiaUnitTypes_e)

| Category | Input | Output |
|----------|-------|--------|
| None | `INPUT_NONE = 0` | - |
| Ratio | `INPUT_RATIO_UNIT = 1` | - |
| Strain | `INPUT_STRAIN_UNIT = 2` | - |
| Angle | `INPUT_ANGLE_UNIT = 5` | `OUTPUT_ANGLE_UNIT = 6` |
| Section Angle | `INPUT_SECTION_ANGLE_UNIT = 7` | - |
| Length | `INPUT_LENGTH_UNIT = 10` | `OUTPUT_LENGTH_UNIT = 11` |
| Deformation | `INPUT_DEFORMATION_UNIT = 12` | `OUTPUT_DEFORMATION_UNIT = 13` |
| Dimension | `INPUT_DIMENSION_UNIT = 14` | - |
| Area | `INPUT_AREA_UNIT = 20` | `OUTPUT_REINFAREA_UNIT = 21` |
| Volume | `INPUT_VOLUME_UNIT = 32` | `OUTPUT_VOLUME_UNIT = 30` |
| Force | `INPUT_FORCE_UNIT = 100` | `OUTPUT_FORCE_UNIT = 101` |
| Weight | `INPUT_WEIGHT_UNIT = 102` | `OUTPUT_WEIGHT_UNIT = 103` |
| Moment | `INPUT_MOMENT_UNIT = 200` | `OUTPUT_MOMENT_UNIT = 201` |
| Temperature | `INPUT_TEMPERATURE_UNIT = 300` | `OUTPUT_TEMPERATURE_UNIT = 301` |
| Factor | `INPUT_FACTOR_UNIT = 400` | - |
| Primitives | `INPUT_BOOLEAN = -3`, `INPUT_INTEGER = -2`, `INPUT_STRING = -1` | - |
