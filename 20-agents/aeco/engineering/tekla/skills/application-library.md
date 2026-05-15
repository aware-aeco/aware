---
name: tekla-application-library
description: "Tekla Open API - Tekla.Application.Library. Application-level services: TeklaStructures static entry point, MacroBuilder, unit conversion, INP file parsing, concrete rebar utilities, object properties, VirtualFolder. This skill should be used when working with Tekla application context, macros, environment, configuration, or INP files."
---

# Tekla.Application.Library API Reference (v2025)

## Overview

Application-level library providing:
- **TeklaStructures** - Static entry point for application context (connection, environment, configuration, drawing, model)
- **MacroBuilder** - Fluent API for building and running Tekla macros
- **UnitConverter** - Convert between mm and current units
- **InpParser** - Parse INP dialog definition files and extract UDA definitions
- **Concrete utilities** - Rebar classification, splitting, splicing, grouping
- **ObjectPropertiesLibrary** - UI controls for object property management
- **VirtualFolder / ModelFolder** - Search path-aware file resolution

---

## TeklaStructures (Static Entry Point)

### TeklaStructures (static class)
> Application interface - main entry point for accessing Tekla application services.

**Static Properties:**
- `ICommonTasks CommonTasks { get; }` - common task operations
- `Configuration Configuration { get; }` - current application configuration
- `IConnection Connection { get; }` - connection state
- `IDrawing Drawing { get; }` - drawing operations
- `IEnvironment Environment { get; }` - environment settings and paths
- `bool IsRunning { get; }` - whether Tekla Structures is running
- `IMainWindow MainWindow { get; }` - main window control
- `IModel Model { get; }` - model connection
- `IRegistry Registry { get; }` - registry access
- `string Version { get; }` - application version

**Static Methods:**
- `bool Connect()` - connect to running Tekla Structures
- `void Disconnect()` - disconnect from Tekla Structures
- `void ExecuteScript(string script)` - execute a macro script

---

## Configuration

### Configuration (enum)
> Application configuration types.

| Value | Int | Description |
|-------|-----|-------------|
| Viewer | 0 | Viewer configuration |
| ProductionPlanner | 1 | Production planner |
| Drafter | 2 | Drafter |
| ProjectManagement | 3 | Project Management |
| Engineering | 4 | Engineering |
| ReinforcedConcreteDetailing | 5 | Reinforced concrete detailing |
| PrecastConcreteDetailing | 6 | Precast concrete detailing |
| SteelDetailing | 7 | Steel detailing |
| Full | 8 | Full detailing |
| Primary | 9 | Steel detailing limited |
| Educational | 10 | Educational |
| Developer | 11 | Developer |
| ConstructionManagement | 12 | Construction Management |
| Partner | 13 | Partner |
| EPMModeler | 14 | EPM Modeler |
| Carbon | 15 | Carbon |
| Graphite | 16 | Graphite |
| Diamond | 17 | Diamond |
| MEP | 18 | MEP |
| Trial | 19 | Trial |
| Platform | 20 | Tekla Platform |

### ConfigurationSet (sealed class)
> Configuration set for checking multiple configurations.

**Constructors:** `ConfigurationSet()`, `ConfigurationSet(Configuration[] configurations)`

**Static Properties:**
- `ConfigurationSet Empty { get; }` - empty set
- `ConfigurationSet Full { get; }` - set with all configurations

**Methods:**
- `void Add(Configuration configuration)` / `void Add(Configuration[] configurations)`
- `bool Contains(Configuration configuration)`
- `bool ContainsAll(Configuration[] configurations)`
- `bool ContainsAny(Configuration[] configurations)`
- `void Remove(Configuration configuration)` / `void Remove(Configuration[] configurations)`

---

## Application Interfaces

### IConnection
> Connection interface.

- `bool IsActive { get; }` - whether connection is active

### IModel
> Model connection interface.

- `ModelFolder Folder { get; }` - model folder structure
- `string Name { get; }` - model name

### IDrawing
> Drawing connection interface.

- `Drawing Current { get; }` - current drawing
- `ICollection<Drawing> Drawings { get; }` - all drawings
- `bool IsEditorOpen { get; }` - whether drawing editor is open
- `bool Close(bool saveBeforeClosing)` / `bool Close()`
- `bool Open(Drawing drawing)`
- `bool Save()`

### IEnvironment
> Application environment interface.

**Properties:**
- `IEnumerable<string> CloningTemplateModelFolders { get; }`
- `IEnumerable<string> CompanyFolders { get; }`
- `CultureInfo CultureInfo { get; }` - application culture info
- `IEnumerable<string> DrawingMacros { get; }`
- `string Item { get; }` - indexer for environment variables
- `string Language { get; }` - application language
- `Localization Localization { get; }`
- `string MacrosFolder { get; }`
- `IEnumerable<string> ModelMacros { get; }`
- `IEnumerable<Dictionary<string, string>> OptionTypeUDAIndexAndValue { get; }`
- `IEnumerable<string> ProjectFolders { get; }`
- `string SearchPath { get; }`
- `IEnumerable<string> SystemFolders { get; }`
- `IEnumerable<string> UserDefinedAttributes { get; }`
- `IEnumerable<string> UserDefinedAttributesOptionType { get; }`
- `bool UseUSImperialUnitsInInput { get; }`

**Methods:**
- `ICollection<UDA> GetUserDefinedAttributes(UDATypes[] types)`
- `void LoadLocalizationFile(string fileName)`

### IMainWindow
> Tekla Structures main window interface.

- `bool IsActive { get; }` / `bool IsMinimized { get; }`
- `void Activate()`
- `void AttachChildForm(Form form)` / `void DetachChildForm(Form form)`

### ICommonTasks
> Common tasks library interface.

- `void CreateGeneralArrangementDrawingFromTemplate(string name)`
- `void OpenAssemblyDrawingProperties(string name)`
- `void OpenAutoDrawingScript(string name)`
- `void OpenCastUnitDrawingProperties(string name)`
- `void OpenDrawingList()`
- `void OpenGeneralArrangementDrawingProperties(string name)`
- `void OpenNumberingSettings()`
- `void OpenSinglePartDrawingProperties(string name)`
- `void PerformNumbering(bool fullNumbering)`

### IPicker
- `object PickObject(string prompt)`

### ISelection
- `IEnumerable<object> AllObjects { get; }`
- `IEnumerable<object> SelectedObjects { get; }`

### ISelectObject
- `object SelectObjectByIdentifier(Identifier identifier)`

### ITransaction
- `void CommitChanges(IEnumerable<object> objects)`
- `void DiscardChanges(IEnumerable<object> objects)`

### IRunMacro
- `void RunMacro(string macroName)`

### IRegistry
> Provides access to application registry keys and values.

- `string CurrentVersionPath { get; }`
- `string GetVersionPath(string version)`
- `Rectangle LoadDialogBounds(string dialogName, string version)` / `Rectangle LoadDialogBounds(string dialogName)`
- `void SaveDialogBounds(string dialogName, string version, Rectangle bounds)` / `void SaveDialogBounds(string dialogName, Rectangle bounds)`

---

## MacroBuilder

### MacroBuilder (sealed class)
> Fluent API for building and running Tekla Structures macros.

**Constructors:** `MacroBuilder()`, `MacroBuilder(string script)`

**Fluent Methods (return MacroBuilder):**
- `Activate(string dialog, string field)` - activate a dialog field
- `Callback(string callback, string parameter, string frame)` / `Callback(string callback, string parameter)` / `Callback(string callback)`
- `CheckValue(string name, int value)`
- `CloseWpfView(string viewName)`
- `CommandEnd()` / `CommandStart(string command, string parameter, string frame)`
- `FileSelection(string[] items)`
- `ListSelect(string dialog, string field, string[] items)`
- `ModalDialog(int value)`
- `MouseDown(string frame, string subframe, int x, int y, int modifier)` / `MouseUp(string frame, string subframe, int x, int y, int modifier)`
- `PushButton(string button, string frame)`
- `PushWpfButton(string viewName, string buttonAid)` / `PushWpfButton(string viewName, IEnumerable<string> automationIds)`
- `PushWpfContextMenuButton(string viewName, IEnumerable<string> contextMenuParentAids, IEnumerable<string> buttonAids)` / `PushWpfContextMenuButton(string viewName, string contextMenuParentAid, string buttonAid)`
- `PushWpfDataGridColumnHeaderButton(string viewName, string buttonAid, bool emulateHoldingShiftKey)` / `PushWpfDataGridColumnHeaderButton(string viewName, IEnumerable<string> automationIds, bool emulateHoldingShiftKey)`
- `SelectWpfDataGridRows(string viewName, IEnumerable<string> automationIds, IEnumerable<int> indexes)`
- `SelectWpfListViewItemsByName(string viewName, IEnumerable<string> automationIds, IEnumerable<string> itemNames)`
- `SetWpfTextBoxText(string viewName, IEnumerable<string> automationIds, string text)`
- `SetWpfToggleButtonChecked(string viewName, IEnumerable<string> automationIds)` / `SetWpfToggleButtonUnChecked(string viewName, IEnumerable<string> automationIds)`
- `TabChange(string dialog, string field, string item)`
- `TableSelect(string dialog, string field, int[] items)`
- `TreeSelect(string dialog, string field, string rowstring)`
- `ValueChange(string dialog, string field, string data)`
- `WpfCommandRepositoryCommand(string command)`

**Execution Methods:**
- `void Run()` / `void Run(int waitCount)` / `void Run(IConnection connection)`

**Static Methods:**
- `IList<Tuple<int, int>> ConvertToRanges(IEnumerable<int> indexes)`
- `void WaitForMacroToRun(int waitCount)`

### AKIT vs WPF Methods — When to Use Which

MacroBuilder has two families of methods. Using the wrong family produces macros that silently fail.

**AKIT methods** — for legacy WinForms dialogs (pre-2020 UI). Generate `akit.*()` script calls:
- `ValueChange`, `PushButton`, `Callback`, `ListSelect`, `TabChange`, `TableSelect`, `Activate`, `TreeSelect`, `FileSelection`, `ModalDialog`, `CheckValue`
- Use when the target is a traditional dialog: `dia_*`, `diaView*`, `main_frame`, or any dialog with a named frame.

**WPF methods** — for modern WPF controls (Tekla 2020+). Generate `wpf.*()` script calls:
- `SetWpfTextBoxText`, `PushWpfButton`, `SelectWpfListViewItemsByName`, `SelectWpfDataGridRows`, `SetWpfToggleButtonChecked`, `SetWpfToggleButtonUnChecked`, `PushWpfContextMenuButton`, `PushWpfDataGridColumnHeaderButton`, `CloseWpfView`, `WpfCommandRepositoryCommand`
- Use when the target is a WPF view identified by automation IDs.

**Known WPF views (use WPF methods for these):**
- `Toolbars.SnapToolbar` — snap plane type, snap switch (`type_plane`, `snap_switch`)
- `Toolbars.SelectionToolbar` — selection filter, selection switch
- `Toolbars.ViewToolbar` — view type controls
- `MainRibbon` — the ribbon bar and its tabs
- Numbering setup dialog (Tekla 2022+)
- Phase manager (Tekla 2022+)

**Rule of thumb:** If the view name contains `Toolbar` or `Ribbon`, use WPF methods. If it's a `dia_*` dialog, use AKIT methods.

**Example — setting Snap Toolbar plane type:**
```csharp
// WRONG — ValueChange does not work on WPF controls:
macroBuilder.ValueChange("Toolbars.SnapToolbar", "type_plane", "Work plane");

// CORRECT — use SetWpfTextBoxText for WPF combo boxes / text fields:
macroBuilder.SetWpfTextBoxText("Toolbars.SnapToolbar", new[] { "type_plane" }, "Work plane");

// CORRECT — use PushWpfButton for WPF buttons:
macroBuilder.PushWpfButton("Toolbars.SnapToolbar", "snap_switch");

// CORRECT — use WpfCommandRepositoryCommand for named commands:
macroBuilder.WpfCommandRepositoryCommand("Sharing.StartSharing");
```

### MacroBuilderCompatibilityHelper
> Backward-compatible helper for WPF macro methods added in TS 2020.

**Constructor:** `MacroBuilderCompatibilityHelper(MacroBuilder macroBuilder)`

**Invoke Methods (execute WPF macro actions):**
- `void InvokeCloseWpfViewMethod(string viewName)`
- `void InvokePushWpfButtonMethod(string viewName, IEnumerable<string> automationIds)`
- `void InvokePushWpfContextMenuButtonMethod(string viewName, IEnumerable<string> contextMenuParentAids, IEnumerable<string> buttonAids)`
- `void InvokePushWpfDataGridColumnHeaderButtonMethod(string viewName, IEnumerable<string> automationIds, bool emulateHoldingShiftKey)`
- `void InvokeSelectWpfDataGridRowsMethod(string viewName, IEnumerable<string> automationIds, IEnumerable<int> indexes)`
- `void InvokeSelectWpfListViewItemsByNameMethod(string viewName, IEnumerable<string> automationIds, IEnumerable<string> itemNames)`
- `void InvokeSetWpfTextBoxTextMethod(string viewName, IEnumerable<string> automationIds, string text)`
- `void InvokeSetWpfToggleButtonCheckedMethod(string viewName, IEnumerable<string> automationIds)`
- `void InvokeSetWpfToggleButtonUnCheckedMethod(string viewName, IEnumerable<string> automationIds)`
- `void InvokeWpfInvokeCommandMethod(string commandName)`

**IsSupported Methods (check runtime support):**
- `bool IsSupportedCloseWpfViewMethod()`
- `bool IsSupportedPushWpfButtonMethod()`
- `bool IsSupportedPushWpfContextMenuButtonMethod()`
- `bool IsSupportedPushWpfDataGridColumnHeaderButtonMethod()`
- `bool IsSupportedSelectWpfDataGridRowsMethod()`
- `bool IsSupportedSelectWpfListViewItemsByNameMethod()`
- `bool IsSupportedSetWpfTextBoxTextMethod()`
- `bool IsSupportedSetWpfToggleButtonCheckedMethod()`
- `bool IsSupportedSetWpfToggleButtonUnCheckedMethod()`
- `bool IsSupportedWpfCommandRepositoryCommand()`

**Static Methods:**
- `bool CreateMacroCodeForDocumentManager()`
- `bool IsOldDrawingListInUse()`
- `bool MacroBuilderRuntimeVersionSupportsWpfControls()`

### MacroCompatibilityHelperExamples (static class)
- `void CloseDocumentManagerOrDrawingList()`
- `void SelectSinglePartDrawingForSelectedPartInDocumentManagerOrDrawingList()`

---

## Unit Conversion

### UnitConverter (ConversionTool namespace)
> Convert between mm and current Tekla units.

**Constructor:** `UnitConverter(bool convertToImperial)`

**Methods:**
- `double ConvertFromCurrentUnitsToMm(string value)`
- `string ConvertFromMmToCurrentUnits(double value)`
- `string FormatNumericTextIfNecessary(string value)`
- `bool TryConvertFromCurrentUnitsToMm(string value, out double convertedResult)`
- `bool TryConvertFromMmToCurrentUnits(double value, out string convertResult)`
- `bool TryFormatNumericTextIfNecessary(string value, out string formatedResult)`

---

## File System

### VirtualFolder
> Representation of a virtual folder with search path resolution.

**Constructor:** `VirtualFolder(string folderPath, string searchPath)`

**Properties:**
- `string FolderName { get; }` / `string FolderPath { get; }`

**Methods:**
- `void AddSearchPath(string searchPath)`
- `string CreateWritableCopy(string filename)`
- `string FindFile(string filename)`
- `IEnumerable<string> FindFiles(string pattern)`
- `IEnumerable<string> FindFilesWithExtension(string extension)`
- `string GetWritablePath(string filename)`
- `bool IsWritable(string filename)`

### ModelFolder : VirtualFolder
> Representation of a model folder structure.

**Constructor:** `ModelFolder(string folderPath, string searchPath)`

**Properties:**
- `VirtualFolder AttributesFolder { get; }` - model attributes folder
- `VirtualFolder DrawingsFolder { get; }` - model drawings folder

**Static Methods:**
- `bool ContainsModelDatabase(string modelFolder)`

### SearchPath (static class)
> Operations for working with search paths.

- `string FindFile(string searchPath, string filename)`
- `IEnumerable<string> FindFiles(string searchPath, string searchPattern)`
- `IEnumerable<string> FindFilesWithExtension(string searchPath, string extension)`

---

## Cache

### Cache<TKey, TItem> (static class)
> Generic item cache.

- `void Add(TKey key, TItem item)`
- `bool ContainsKey(TKey key)`
- `TItem GetItem(TKey key)`
- `TItem GetItemOrDefault(TKey key, TItem defaultItem)` / `TItem GetItemOrDefault(TKey key)`
- `bool TryGetItem(TKey key, out TItem item)`

---

## RunContext & Events

### RunContext
> Defines thread context for actions.

**Constructors:** `RunContext()`, `RunContext(int uiThreadId)`
- `bool RunInSameThread()`

### SaveInfoEventArgs : EventArgs
> Event args for model saved event.

- `string Reason { get; }` - reason for saving

### SavedInfoEventHandler (sealed delegate)
> `void SavedInfoEventHandler(object sender, SaveInfoEventArgs e)`

---

## INP Parser (Tekla.Structures.InpParser)

Parses INP dialog definition files to extract UDA definitions and tab page layouts.

### Parser
> Main INP file parser.

**Constructor:** `Parser()`

**Properties:**
- `Dictionary<TSModelObjectTypes, TSModelObject> DefinedModelObjects { get; }`
- `Dictionary<string, TSTabPageDefinition> DefinedTabPages { get; }`
- `bool ValidationOn { get; set; }`

**Methods:**
- `ICollection<UDA> FindUdas(UDATypes[] udaTypes)`
- `void Parse(string filePath, bool overrideExisting)`

### TSModelObject
> Model object for which user attributes are defined.

**Constructor:** `TSModelObject(TSModelObjectTypes type)`

**Properties:**
- `List<UDA> Attributes { get; set; }`
- `int DummyNumber { get; set; }`
- `bool Modify { get; set; }`
- `string Name { get; set; }`
- `List<TSTabPageDeclaration> TabPages { get; set; }`
- `TSModelObjectTypes Type { get; set; }`

### TSModelObjectTypes (enum)
> INP model object types.

| Value | Int | Description |
|-------|-----|-------------|
| part | 0 | Part |
| beam | 1 | Beam |
| column | 2 | Column |
| beamortho | 3 | Beam ortho |
| twinprofile | 4 | Twin profile |
| contourplate | 5 | Contour plate |
| foldedplate | 6 | Folded plate |
| concrete_beam | 7 | Concrete beam |
| concrete_column | 8 | Concrete column |
| pad_footing | 9 | Pad footing |
| strip_footing | 10 | Strip footing |
| concrete_panel | 11 | Concrete panel |
| concrete_slab | 12 | Concrete slab |
| rebar | 13 | Rebar |
| surfacing | 14 | Surfacing |
| welding | 15 | Welding |
| bolt | 16 | Bolt |
| singledrawing | 17 | Single part drawing |
| assemblydrawing | 18 | Assembly drawing |
| gadrawing | 19 | GA drawing |
| multidrawing | 20 | Multi drawing |
| castunitdrawing | 21 | Cast unit drawing |
| project | 22 | Project |
| phase | 23 | Phase |
| reference | 24 | Reference model |
| reference_part | 25 | Reference part |
| steelassembly | 26 | Steel assembly |
| precastassembly | 27 | Precast assembly |
| insituassembly | 28 | In-situ assembly |
| grid | 29 | Grid |
| grid_plane | 30 | Grid plane |
| task | 31 | Task |
| pour_object | 32 | Pour object |
| pour_break | 33 | Pour break |

### UDA : TSTabPageObject
> User defined attribute definition.

**Constructors:** `UDA(bool isUnique)`, `UDA(string name)`

**Properties:**
- `string AttributeValueMax { get; set; }` / `string AttributeValueMin { get; set; }`
- `CheckSwitchValues CheckSwitch { get; set; }`
- `string FieldFormat { get; set; }` - C-like format definition
- `bool IsUnique { get; set; }`
- `string LabelText { get; set; }` - label text shown in dialog
- `int PositionValue1 { get; set; }` / `int PositionValue2 { get; set; }` / `int PositionValue3 { get; set; }`
- `bool SpecialFlag { get; set; }` - for parts: consider in numbering; for drawings: display in drawing list
- `string ToggleField { get; set; }`
- `List<UDAValue> Values { get; set; }`
- `UDATypes ValueType { get; set; }`

### UDATypes (enum)
> UDA value types - 71 values covering all Tekla property types.

| Value | Int | Description |
|-------|-----|-------------|
| Integer | 0 | Integer |
| Float | 1 | Float |
| String | 2 | String |
| Date | 3 | Date |
| Option | 4 | Option |
| Label | 5 | Label |
| Material | 6 | Material |
| Profile | 7 | Profile |
| File_in | 8 | File input |
| File_out | 9 | File output |
| Bolt_standard | 10 | Bolt standard |
| Bolt_size | 11 | Bolt size |
| Ratio | 12 | Ratio |
| Strain | 13 | Strain |
| Angle | 14 | Angle |
| Deformation | 15 | Deformation |
| Dimension | 16 | Dimension |
| Radiusofinertia | 17 | Radius of inertia |
| Area | 18 | Area |
| Areaperlength | 19 | Area per length |
| Sectionmodulus | 20 | Section modulus |
| Momentofinertia | 21 | Moment of inertia |
| Torsionconstant | 22 | Torsion constant |
| Warpingconstant | 23 | Warping constant |
| Force | 24 | Force |
| Weight | 25 | Weight |
| Distribload | 26 | Distributed load |
| Springconstant | 27 | Spring constant |
| Surfaceload | 28 | Surface load |
| Strength | 29 | Strength |
| Modulus | 30 | Modulus |
| Density | 31 | Density |
| Moment | 32 | Moment |
| Distribmoment | 33 | Distributed moment |
| Rotspringconst | 34 | Rotational spring constant |
| Temperature | 35 | Temperature |
| Thermdilatcoeff | 36 | Thermal dilatation coefficient |
| Distance | 37 | Distance |
| Distance_list | 38 | Distance list |
| integer_no_toggle | 39 | Integer (no toggle) |
| float_no_toggle | 40 | Float (no toggle) |
| string_no_toggle | 41 | String (no toggle) |
| distance_list_no_toggle | 42 | Distance list (no toggle) |
| distance_no_toggle | 43 | Distance (no toggle) |
| label2 | 44 | Label 2 |
| label3 | 45 | Label 3 |
| toggle | 46 | Toggle |
| toggle_no_toggle | 47 | Toggle (no toggle) |
| optionstring | 48 | Option string |
| radiobutton | 49 | Radio button |
| weld_type | 50 | Weld type |
| chamfer_type | 51 | Chamfer type |
| welding_site | 52 | Welding site |
| bolt_type | 53 | Bolt type |
| hole_type | 54 | Hole type |
| hole_direction | 55 | Hole direction |
| stud_standard | 56 | Stud standard |
| stud_size | 57 | Stud size |
| stud_length | 58 | Stud length |
| rebar_size | 59 | Rebar size |
| rebar_main | 60 | Rebar main |
| rebar_stirrup | 61 | Rebar stirrup |
| rebar_mesh | 62 | Rebar mesh |
| rebar_grade | 63 | Rebar grade |
| rebar_radius | 64 | Rebar radius |
| date_time_min | 65 | Date time (minutes) |
| date_time_sec | 66 | Date time (seconds) |
| factor | 67 | Factor |
| ComponentName | 68 | Component name |
| YesNo | 69 | Yes/No |
| ComponentAttributeFile | 70 | Component attribute file |

### UDAValue
> Attribute value definition.

**Constructors:** `UDAValue(string attributeValue)`, `UDAValue(string attributeValue, int defaultSwitch)`

- `int DefaultSwitch { get; set; }` - 0=no default, 1=default (stored), 2=default (not stored)
- `string Value { get; set; }`

### UDANameComparer (sealed class)
> Comparer for sorting UDAs by name.

### Supporting INP Types

| Type | Kind | Description |
|------|------|-------------|
| Lexer | class | Tokenizer - `static Token GetLexeme(FileStream inputStream, ref int lineNumber)` |
| Token | class | Token with `Value` and `TokenType` properties |
| TokenType | enum | Punctuation(0), Identifier(1), Number(2), String(3) |
| CharType | enum | NotDefined(0), Delimiter(1), Punctuation(2), Letter(3), Number(4) |
| KeyWords | enum | value(0), attribute(1), unique_attribute(2), picture(3), tab_page(4), modify(5) |
| CheckSwitchValues | enum | none(0), check_max(1), check_min(2), check_maxmin(3) |
| TSTabPageObject | class | Base for tab page objects - `string Name { get; set; }` |
| TSTabPageDeclaration | class | Tab page usage - `Name`, `Prompt`, `Index`, `Definition` |
| TSTabPageDefinition | class | Tab page layout - `Name`, `List<TSTabPageObject> Objects` |
| Picture : TSTabPageObject | class | Picture on tab page - `X`, `Y`, `Width`, `Height` |
| EOFException : WrongFormatException | class | EOF reached during parsing - `bool IsCorrectEnd { get; set; }` |
| WrongFormatException : ApplicationException | class | Format error - `ExpectedToken`, `ReceivedToken`, `LineNumber` |

---

## Concrete Utilities (Tekla.Structures.Concrete)

### ClassificatorCalculator
> Classify rebars by level (top/bottom/back/front).

**Constructors:** `ClassificatorCalculator(Localization localize)`, `ClassificatorCalculator(string top, string bot, string back, string front, Localization localize)`

**Methods:**
- `void ClassifyRebar(ModelObject singleObject, ref ArrayList classifiedRebars)`
- `void ClassifyRebars(bool allRebars, ref ArrayList rebars, ref ProgressBar progress)`
- `string SetRebarsLevel(Reinforcement rebar)`
- `void SortRebarsLevels(ref ArrayList classifiedRebars, ref ProgressBar progressBar)`

### CreateSingleRebar
> Create single rebar from reinforcement.

**Constructor:** `CreateSingleRebar(Reinforcement newOriginalRebar)`
- `SingleRebar GetSingleRebar(ArrayList rebarPoints)`

### SingleRebarToRebarGroupConverter
> Convert single rebars to rebar groups.

**Constructor:** `SingleRebarToRebarGroupConverter(RebarGroupConversionData data, ArrayList singleRebars)`
- `bool ConvertSingleRebarToRebarGroup(bool splitForSplicing, out ArrayList newRebarGroups, out ArrayList newUngroupedRebars)`
- `static Vector GetHookDirection(Reinforcement rebar)`
- `static void SwapStartHookWithEndHook(ref SingleRebar newRebar)`

### RebarGroupConversionData
> Configuration for rebar group conversion.

**Constructor:** `RebarGroupConversionData()`

- `int DepthLocation { get; set; }`
- `Part FatherSlab { get; set; }`
- `double Spacing { get; set; }`
- `int SpliceSection { get; set; }`
- `RebarGroupStirrupTypeEnum StirrupType { get; set; }`

### SplitRebarsInPart
> Split rebars that exceed maximum length.

**Constructors:** `SplitRebarsInPart(SplitData splitData, ArrayList singleRebars)`, `SplitRebarsInPart()`

**Methods:**
- `bool SplitRebarsIfNeeded(SplitData, ArrayList singleRebars, out ArrayList splitRebars)`
- `bool SplitRebarsIfNeeded(out ArrayList newSplitRebars)`
- `bool SplitRebarsIfNeeded(SplitData, IList<SingleRebar>, out ArrayList splitRebars)`
- `bool SplitRebarsIfNeeded(SplitData, RebarGroup rebarGroupToSplit, out ArrayList splitRebars, int crossSection)`
- `static double GetRebarLength(RebarGeometry geometry, bool roundUp)`

### SplitData
> Configuration for rebar splitting.

**Constructor:** `SplitData()`

- `int DepthLocation { get; set; }` / `double LappingLength { get; set; }` / `double MaxLength { get; set; }` / `double MinSplitDistance { get; set; }`
- `double SpliceOffset { get; set; }` / `int SpliceSection { get; set; }` / `int SpliceSymmetry { get; set; }` / `int SpliceType { get; set; }`

### SpliceRebarsInPart
> Splice rebars.

**Constructors:** `SpliceRebarsInPart()`, `SpliceRebarsInPart(SpliceData newSpliceData)`

- `bool SpliceRebarsIfNeeded(ArrayList rebarGroups, ArrayList ungroupedRebars, out ArrayList splices)`
- `bool SpliceRebarsIfNeeded(SpliceData, ArrayList rebarGroups, ArrayList ungroupedRebars, out ArrayList splices)`

### SpliceData
> Configuration for rebar splicing.

**Constructor:** `SpliceData()`

- `RebarSpliceBarPositionsEnum BarPositions { get; set; }` / `double LappingLength { get; set; }` / `double LappingLengthFactor { get; set; }` / `int SpliceType { get; set; }`

### PatternInfo
> Rebar pattern information for beam cross-sections.

**Constructor:** `PatternInfo()`

**Properties:**
- `string BeamType { get; set; }` / `string Name { get; set; }`
- `double Width { get; set; }` / `double Height { get; set; }`
- `bool IsSymmetrical { get; set; }`
- `int MaxNumberTop { get; }` / `int MaxNumberBottom { get; }`

**Static Properties:**
- `int BottomClass { get; }` / `string BottomName { get; }`
- `int TopClass { get; }` / `string TopName { get; }`

**Methods:**
- `void AddBottomPos(double x, double y)` / `void AddTopPos(double x, double y)`
- `bool GetBottomXy(int index, ref double x, ref double y)` / `bool GetTopXy(int index, ref double x, ref double y)`
- `void ReadPatternBlock(StreamReader sr)`
- `bool SetWidthHeight(double pointX, double pointY)`

**Static Methods:**
- `ArrayList GetAllBeamTypes()` / `PatternInfo GetPatternInfo(string name)`
- `ArrayList GetPatternsByBeamType(string type)`
- `string GetSysFilePathName(Model m, string name)`

---

## Object Properties Library (Tekla.Structures.ObjectPropertiesLibrary)

### TSConnection
> Connect to Tekla Structures from object properties forms.

**Constructor:** `TSConnection()`

- `string CurrentLanguage { get; set; }`
- `Localization LocalizationForm { get; }`
- `Model OpenModel { get; }`
- `IEvents TeklaEvents { get; }`
- `void Connect()` / `void Dispose()`
- `Dictionary<string, ModelObject> GetAllObjects()`
- `void GetChildrenOfSelected()`
- `ArrayList GetSelectedObjects()` / `bool SelectObjectsInModel(ArrayList selectedObjects)`
- `void SetLanguage()`
- `void ZoomToSelectedObjects()`

### PresentedProperties
> Data class for a Tekla property presented in UI.

**Constructors:** `PresentedProperties()`, `PresentedProperties(string displayName, string reportName, string udaName, string type, int decimals, int width, bool hidden)`, `PresentedProperties(PresentedProperties propertyToCopy)`

**Properties:** `Name`, `ReportPropertyName`, `UdaPropertyName`, `PropertyType`, `DisplayType`, `Decimals`, `Width`, `Hidden`, `Visible`

**Methods:**
- `void Copy(PresentedProperties propertyToCopy)`

### PresentedPropertiesXml
> Object properties read from XML configuration.

**Constructor:** `PresentedPropertiesXml(string fileName, Model openModel)`

**Properties:**
- `string LoadSaveDirectory { get; }`
- `SearchableSortableBindingList<PresentedProperties> PropertiesList { get; set; }`
- `SearchableSortableBindingList<PresentedProperties> VisiblePropertiesList { get; set; }`

**Methods:**
- `bool AddPropertyWithReportName(string reportPropertyName, string valueString)`
- `void ForceReadFile()`
- `int GetPropertiesHashCode(BindingList<PresentedProperties> listToHash)`
- `bool GetPropertyByModelPropertyName(string modelPropertyName, ref PresentedProperties property)`
- `bool GetPropertyByName(string propertyName, ref PresentedProperties property)`
- `bool GetPropertyByNameOrModelPropertyName(string propertyName, ref PresentedProperties property)`
- `bool Initialize(string fileName, Model openModel)`
- `void MergeProperties(PresentedPropertiesXml propertiesToMerge)`
- `void MoveProperty(PresentedProperties propertyToMove, PresentedProperties propertyToMoveBy)`
- `void RemoveProperty(PresentedProperties propertyToRemove)`
- `bool XmlWriteProperties(BindingList<PresentedProperties> listToSerialize)` / `bool XmlWriteProperties()`

**Static Methods:**
- `SearchableSortableBindingList<PresentedProperties> ReadPropertiesListFromFile(string fileName)`
- `void SortBindingList(ref BindingList<object> list, ListSortDirection direction, PropertyDescriptor property)`
- `bool XmlWriteProperties(BindingList<PresentedProperties> listToSerialize, string fileName)`

### PresentedPropertiesManage
> Static helpers for managing presented properties.

- `static bool ChangeHiddenValuesByOtherFile(ref PresentedPropertiesXml all, ref PresentedPropertiesXml shown)`
- `static string GetBaseType(string propertyType)`
- `static object GetPartPropertyValue(ModelObject part, PresentedProperties property, PresentedPropertiesXml instance)`
- `static object GetPartPropertyValue(ModelObject part, string visibleName, PresentedPropertiesXml instance)`
- `static bool MakeFileFromOtherFilesHiddenProperties(ref PresentedPropertiesXml shown, ref PresentedPropertiesXml all)`

### MathEvaluate
> Mathematical expression evaluator.

**Constructor:** `MathEvaluate()`

**Properties:**
- `EvaluateFunctionDelegate DefaultFunctionEvaluation { get; set; }`
- `ArrayList Equation { get; }`
- `bool Error { get; }` / `string ErrorDescription { get; }`
- `ArrayList Postfix { get; }`
- `double Result { get; }`
- `ArrayList Variables { get; set; }`

**Methods:**
- `bool Parse(string newEquation)` / `bool Parse(string newEquation, CultureInfo cultureInfo)`
- `void Infix2Postfix()` / `void EvaluatePostfix()`

**Inner Types:**
- `EvaluateFunctionDelegate` - delegate: `Symbol Invoke(string name, object[] args)`
- `Symbol` - evaluation symbol
- `Type` (enum) - Variable(0), Value(1), Operator(2), Function(3), Result(4), Bracket(5), Comma(6), Error(7)

### UI Controls

| Type | Base | Description |
|------|------|-------------|
| AllPropertiesDialog | UserControl | Dialog for managing all properties. Methods: `InitializeAllPropertiesDialog`, `ShowIncludedColumn`, `ShowSelected`, `UpdateAllProperties` |
| ShownPropertiesDialog | UserControl | Dialog for managing shown properties. Methods: `InitializeShownPropertiesDialog`, `RefreshDGW`, `RemovePropertiesFromShown`, `SaveShown`, `UpdateShownProperties` |
| PropertiesForm | ApplicationFormBase | Combined properties form |
| QuickSearchForDataTable | UserControl | Search/filter for DataTable. Methods: `GetHiddenRows`, `GetVisibleRows`, `QuickSearchExecute`, `SetReferenceProperties`, `SilentlyEmptySearchBox` |
| QuickSearchForDGW | UserControl | Search/filter for DataGridView. Methods: `EmptySearchBox`, `GetHiddenRows`, `GetVisibleRows`, `QuickSearchExecute`, `SetReferenceProperties` |
| NullableDateTimePicker | DateTimePicker | DateTimePicker with null support. Properties: `CustomFormat`, `Format`, `NullValue`, `Value` |
| NullableDateTimePickerCell | DataGridViewTextBoxCell | DataGridView cell for nullable DateTimes |
| DataGridViewNullableDateTimePickerColumn | DataGridViewColumn | Column type for NullableDateTimePicker in DataGridView |
| SearchableSortableBindingList<T> | BindingList<T> | Sortable/searchable binding list with `Load(string)`, `Save(string)`, `Sort(PropertyDescriptor, ListSortDirection)` |
| PropertyComparer<T> | class | Comparer for sorting - `int Compare(T wordX, T wordY)` |
| DrawingControl | class | `SuspendDrawing(Control)` / `ResumeDrawing(Control)` for flicker-free UI |
| LocalisationDelegate | delegate | `void Invoke(Control formToLocalise)` |
| SearchStartDelegate | delegate | `void Invoke()` |
| SearchEndedDelegate | delegate | `void Invoke(DataTable resultTable)` |

---

## Other Types

### IOTools.FileChecksum
- `string Calculate(string file)` - calculate file checksum

### WaitingDialog : Form
> Modal waiting dialog.

**Constructor:** `WaitingDialog(Predicate<WaitingDialog> completed)`

### ExpandableStringComboBox : ComboBox
> ComboBox that allows adding new strings via editing or delegates.

**Constructor:** `ExpandableStringComboBox()`

- `string ListTerminator { get; set; }` - item that triggers GetNewStringDelegates
- `void Initialize(string[] data, string currentValue)`
- `void Select(string currentValue)`

**Inner Types:**
- `AddNewStringHandler` (delegate) - `void Invoke(object sender, ExpandableStringComboBoxEvent Event)`
- `GetNewStringHandler` (delegate) - `void Invoke(object sender, ExpandableStringComboBoxEvent Event)`
- `ExpandableStringComboBoxEvent : EventArgs` - `string Input { get; set; }`, `string Result { get; set; }`
