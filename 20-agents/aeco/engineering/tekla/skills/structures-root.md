---
name: tekla-structures-root
description: "Tekla Open API - Root namespace. Identifiers, enums for connection/detail/database types, application info, settings, advanced options, file utilities, and Forming states. Foundation types used everywhere."
---

# Tekla.Structures Root API Reference (v2025)

## Overview

Foundation types for the Tekla Open API: identifiers, enums for connection/detail types, database object types, application info, settings, advanced options, file utilities, and Forming states.

---

## STRICT FIELD RULE (applies to all Tekla skills)

Use ONLY properties, methods, and enum values that are documented in these Tekla reference files or listed in the runtime AVAILABLE INPUTS section of your prompt. Do NOT invent, guess, or extrapolate API members — if a member is not documented or listed, it does not exist.

## Canonical identity keys

Different Tekla object families use different identity keys. Use the correct one for the object at hand — do not carry `ModelObject.Identifier` over to drawings.

| Object family | Default identity / comparison key | Notes |
|---|---|---|
| `Tekla.Structures.Model.ModelObject` (Beam, Part, Assembly, Rebar, Weld, …) | `.Identifier` (a `Tekla.Structures.Identifier` with `ID`, `ID2`, `GUID`) | Stable across the model's lifetime. |
| `Tekla.Structures.Drawing.Drawing` (base class — used by most drawing triggers / inputs) | **`.Mark`** (fall back to `.Name`) | `Drawing` has **NO** `Identifier` / `Id` / `GUID`. `DatabaseObject` (the drawing-side base) exposes only `QueryReturnValue`. |
| `AssemblyDrawing` / `SinglePartDrawing` / `CastUnitDrawing` | `.Mark` for drawing identity; `.AssemblyIdentifier` / `.PartIdentifier` / `.CastUnitIdentifier` **only** when you need the underlying MODEL object. | The `*Identifier` properties point to the model object the drawing is built from, not to the drawing itself. |

Common invention traps (verified non-existent on Tekla 2025):
- `Tekla.Structures.Drawing.Drawing.Identifier` / `.Id` / `.GUID` — drawings have no DB identifier; match by `Mark` / `Name`.
- `Tekla.Structures.Drawing.DatabaseObject.Identifier` — only `QueryReturnValue` exists on the drawing-side base class.
- `drawing.PartIdentifier` / `drawing.AssemblyIdentifier` / `drawing.CastUnitIdentifier` on a base `Drawing` reference — only present after casting to the matching subclass.
- `drawing.GetReportProperty(...)` — `GetReportProperty` is a `ModelObject` method; not available on drawings.
- `Drawing.DrawingUpToDateStatusEnum` — the enum is `DrawingUpToDateStatus` (no `Enum` suffix).

When unsure, prefer iterating a known collection (e.g., `drawingHandler.GetDrawings()`, `modelObjectSelector.GetAllObjectsWithType(...)`) and matching by documented business properties over attempting direct lookup by invented identifier fields.

---

## Tekla.Structures

### Identifier
> Holds the identifier number (ID) or globally unique identifier (GUID) of a model object.

**Constructors:** `Identifier()`, `Identifier(int id)`, `Identifier(Guid guid)`, `Identifier(string guid)`

**Properties:**
- `Guid GUID { get; set; }` - the object's globally unique identifier. Setting GUID initializes ID to 0.
- `int ID { get; set; }` - the identifier number. Setting ID initializes GUID to Guid.Empty.
- `int ID2 { get; set; }` - the sub identifier number. Setting ID2 initializes GUID to Guid.Empty.

**Methods:**
- `bool IsValid()` - checks if the identifier is valid

---

### Assertion
> Defines an assertion by the method it failed in and the messages it provided.

**Constructors:** `Assertion(string message, string detailedMessage, string methodName)`

**Properties:**
- `string DetailedMessage { get; set; }`
- `string Message { get; set; }`
- `string MethodName { get; set; }`

---

### TeklaStructuresInfo
> Provides information about the running Tekla Structures instance.

**Static Methods:**
- `static string GetBuildNumber()`
- `static string GetCommonAppDataFolder()`
- `static string GetCopyRightText()`
- `static string GetCurrentProgramVersion()`
- `static string GetCurrentUser()`
- `static string GetFullTSRegistryKeyText()`
- `static string GetLocalAppDataFolder()`
- `static string GetPluginsFolder()`
- `static string GetRevisionDate()`

---

### TeklaStructuresSettings
> Provides methods to inquire and set application settings and advanced options.

**Static Methods:**
- `static bool GetAdvancedOption(string VariableName, ref string Value)`
- `static bool GetAdvancedOption(string VariableName, ref double Value)`
- `static bool GetAdvancedOption(string VariableName, ref bool Value)`
- `static bool GetAdvancedOption(string VariableName, ref int Value)`
- `static bool GetAdvancedOptionPaths(string advancedOption, out List<string> paths, InvalidPathCallback errorHandler)`
- `static bool GetOptions(ref ComponentOptions Options)`
- `static bool GetOptions(ref ClashCheckOptions Options)`
- `static bool IsPourEnabled()`
- `static bool IsToolOptionOn(string toolOptionName)`
- `static bool SetOptions(ClashCheckOptions Options)`
- `static bool SetOptions(ComponentOptions Options)`

---

### TeklaStructuresVariables
> Gets active environment variables and advanced option settings (also checks options.ini / options_user.ini).

**Static Methods:**
- `static void Add(string key)`
- `static bool ContainsVariable(string key)`
- `static string Get(string key)`

---

### TeklaStructuresFiles
> Resolves attribute file paths using standard Tekla search directories.

**Constructors:** `TeklaStructuresFiles(string modelpath)`

**Properties:**
- `List<string> PropertyFileDirectories { get; set; }`

**Methods:**
- `FileInfo GetAttributeFile(List<string> searchDirectories, string fileName)`
- `FileInfo GetAttributeFile(string fileName)`
- `List<string> GetMultiDirectoryFileList(string fileExtension, bool fullpath)`

**Static Methods:**
- `static Dictionary<string, string> GetFileDictionaryByExtension(string fileExtension, string modelPath)`

---

### ClashCheckOptions
> Represents clash check options.

**Constructors:** `ClashCheckOptions()`

**Properties:**
- `double BoltHeadDiameter { get; set; }`
- `double NutThickness { get; set; }`

---

### Enums

#### AutoDirectionTypeEnum
> Defines how a connection/detail coordinate system is oriented automatically.

| Value | Int | Description |
|-------|-----|-------------|
| AUTODIR_NA | 0 | Not available |
| AUTODIR_BASIC | 1 | Usual joints |
| AUTODIR_DIAGONAL | 2 | Diagonal joints |
| AUTODIR_SPLICE | 3 | Splices |
| AUTODIR_DETAIL | 4 | Details |
| AUTODIR_GLOBAL_Z | 5 | Joint direction at global Z |
| AUTODIR_SEATING | 6 | Seating joints |
| AUTODIR_PRIMARY_X | 7 | Direction parallel to primary X-axis |
| AUTODIR_FROM_ATTRIBUTE_FILE | 99 | Fetched from loaded attribute file |

#### ComponentDefinitionTypeEnum
> Tells what kind of component is in question.

| Value | Int | Description |
|-------|-----|-------------|
| SYSTEM | 0 | Defined using developer-kit |
| DOTNET_DRAWING | -300000 | .NET drawing plug-in |
| DOTNET_CONNECTION | -200000 | .NET connection plug-in |
| DOTNET | -100000 | .NET plug-in |
| CUSTOM | -1 | Custom component |

#### ConnectionStatusEnum
> Status of a connection or detail (green/yellow/red symbol in model).

| Value | Int | Description |
|-------|-----|-------------|
| STATUS_UNKNOWN | 0 | Unknown |
| STATUS_OK | 1 | OK (green) |
| STATUS_WARNING | 2 | Warning (yellow) |
| STATUS_ERROR | 3 | Error (red) |

#### DetailTypeEnum
> Defines the kind of detail and X-axis orientation.

| Value | Int | Description |
|-------|-----|-------------|
| END | 0 | X-axis towards part center point |
| INTERMEDIATE | 1 | X-axis towards end point |
| INTERMEDIATE_REVERSE | 2 | X-axis towards start point |

#### PositionTypeEnum
> Position type for connections and seams.

| Value | Int | Description |
|-------|-----|-------------|
| MIDDLE_PLANE | 0 | Middle plane |
| BOX_PLANE | 1 | Box plane |
| COLLISION_PLANE | 2 | Collision plane |
| END_END_PLANE | 3 | End plane |
| GUSSET_PLANE | 4 | Gusset plane |

#### PropertyTypeEnum
> Type of property: int, double, or string.

| Value | Int | Description |
|-------|-----|-------------|
| TYPE_INT | 0 | Integer |
| TYPE_DOUBLE | 1 | Double |
| TYPE_STRING | 2 | String |

#### TeklaStructuresDatabaseTypeEnum
> Object types used in filter expressions and database queries. Critical for understanding model object types.

| Value | Int | Description |
|-------|-----|-------------|
| UNKNOWN | 0 | Unknown type |
| POINT | 1 | Point (not a ModelObject) |
| PART | 2 | Part |
| CONNECTION | 3 | Connection (also details and seams) |
| COMPONENT | 4 | Component (macros, plug-ins, custom parts) |
| GRID | 7 | Grid |
| FITTING | 9 | Fitting |
| BOLT | 10 | Bolt |
| PART_CUT | 11 | Part cut |
| PLANE_CUT | 12 | Plane cut |
| WELDING | 13 | Welding |
| ASSEMBLY | 15 | Assembly |
| CONSTRUCTION_LINE | 24 | Construction line (not a ModelObject) |
| PLANE | 30 | Plane (control planes, grid planes) |
| DRAWING | 34 | Drawing |
| DB_CONNECTION | 35 | Connection type |
| PART_ADD | 38 | Part add |
| WELD_CUT | 39 | Weld preparation |
| CONSTRUCTION_CIRCLE | 42 | Construction circle (not a ModelObject) |
| CONSTRUCTION_ARC | 43 | Construction arc |
| DB_COMPONENT | 46 | Component type |
| REBAR | 47 | Rebar (single, groups, meshes, strands) |
| FOREIGN_OBJECT | 48 | Foreign object |
| LOAD_LOAD | 49 | Load |
| LOAD_POINT | 50 | Load point |
| LOAD_LINE | 51 | Load line |
| LOAD_AREA | 52 | Load area |
| LOAD_UNIFORM | 53 | Load uniform |
| LOAD_GROUP | 54 | Load group |
| LOAD_TEMPERATURELOAD | 72 | Temperature load |
| SURFACE_TREATMENT | 73 | Surface treatment |
| REBAR_SPLICE | 74 | Rebar splice |
| ANALYSIS_MODEL | 75 | Analysis model |
| ANALYSIS_PART | 76 | Analysis part |
| ANALYSIS_NODE | 77 | Analysis node |
| EDGE_CHAMFER | 79 | Edge chamfer |
| TASK | 80 | Task |
| TASK_DEPENDENCY | 81 | Task dependency |
| TASK_WORKTYPE | 83 | Task worktype |
| HIERARCHIC_DEFINITION | 84 | Hierarchic definition |
| HIERARCHIC_OBJECT | 85 | Hierarchic object |
| DB_POUR_BREAK | 89 | Pour break |
| DB_POUR_OBJECT | 90 | Pour object |
| REBARSET_ADDITION | 95 | Rebar set addition |
| REBARSET_MODIFIER | 96 | Rebar set modifier |
| SURFACE_OBJECT | 97 | Surface object |
| BENT_PLATE | 98 | Bent plate |
| HELIX | 99 | Helix |
| DB_POUR_UNIT | 101 | Pour unit |
| CONSTRUCTION_POLYCURVE | 103 | Construction polycurve |
| STOREY | 104 | Building Hierarchy Storey |
| BUILDING_SITE | 105 | Building Hierarchy Building |
| BUILDING | 106 | Building Hierarchy Site |
| SPACE | 108 | Building Hierarchy Space |

---

### Tekla.Structures.Forming

#### DeformingType (Enum)
| Value | Int | Description |
|-------|-----|-------------|
| NOT_SPECIFIED | 0 | Default |
| DEFORMED | 1 | Deformed item |
| UNDEFORMED | 2 | Undeformed item |

#### FoldingType (Enum)
| Value | Int | Description |
|-------|-----|-------------|
| NOT_SPECIFIED | 0 | Default |
| FOLDED | 1 | Folded item |
| UNFOLDED | 2 | Unfolded item |

#### WrappingType (Enum)
| Value | Int | Description |
|-------|-----|-------------|
| NOT_SPECIFIED | 0 | Default |
| WRAPPED | 1 | Wrapped item |
| UNWRAPPED | 2 | Unwrapped item |

#### FormingStates
> Contains different forming options (deforming, folding, wrapping).

**Constructors:** `FormingStates()`, `FormingStates(DeformingType)`, `FormingStates(FoldingType)`, `FormingStates(WrappingType)`, `FormingStates(DeformingType, FoldingType, WrappingType)`

**Properties:**
- `DeformingType Deforming { get; set; }`
- `FoldingType Folding { get; set; }`
- `WrappingType Wrapping { get; set; }`

**Methods:**
- `object Clone()`

