---
name: tekla-tekla-structures
description: This skill encodes the tekla 2026.0 surface of the Tekla.Structures namespace — 20 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Assertion, ClashCheckOptions, ComponentOptions, Identifier, ModuleManager, TeklaStructuresFiles, TeklaStructuresInfo, TeklaStructuresSettings, and 12 more types.
---

# Tekla.Structures

Auto-generated from vendor docs for tekla 2026.0. 20 types in this namespace.

## Assertion (class)

Defines an assertion by the method it failed in and the messages it provided.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/ad52597a-0408-037b-a3d1-689855b86dc5)

### Constructors
- `public Assertion(string message, string detailedMessage, string methodName)` — Initializes a new instance of the Assertion class.

### Methods
#### `public override bool Equals(Object obj)`

The equals.

**Parameters:**
- `obj` (System.Object) — The obj.

**Returns:** `Boolean` — The Boolean.

[Docs](https://developer.tekla.com/topic/en/18/47/0e1a8744-a12a-74b6-4bcd-6f67e2c54fc2)

#### `public override int GetHashCode()`

Gets a hash code for this instance. Calculated as recommended at http://msdn.microsoft.com/en-us/library/system.object.gethashcode.aspx

**Returns:** `Int32` — The Int32.

[Docs](https://developer.tekla.com/topic/en/18/47/0694a655-97f1-d848-0561-2b43332e2ecf)

#### `public override string ToString()`

The tostring.

**Returns:** `String` — The String.

[Docs](https://developer.tekla.com/topic/en/18/47/ca2065a8-4966-563c-8b29-4e84a9e0acab)

### Properties
- `DetailedMessage` (String, get) — Gets the detailed message provided by the assertion.
- `Message` (String, get) — Gets the message provided by the assertion.
- `MethodName` (String, get) — Gets the name of the method the assertion failed in.

## AutoDirectionTypeEnum (enum)

The auto direction type defines how a connection or detail coordinate system will be oriented automatically.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4a99522f-60bb-44d4-c332-f63428e34119)

### Values
- `AUTODIR_NA` = `0` — The auto direction will not be available.
- `AUTODIR_BASIC` = `1` — The auto direction type for usual joints will be used.
- `AUTODIR_DIAGONAL` = `2` — The auto direction type for diagonal joints will be used.
- `AUTODIR_SPLICE` = `3` — The auto direction type for splices will be used.
- `AUTODIR_DETAIL` = `4` — The auto direction type for details will be used.
- `AUTODIR_GLOBAL_Z` = `5` — The auto direction type with joint direction at the global Z will be used.
- `AUTODIR_SEATING` = `6` — The auto direction type for seating joints will be used.
- `AUTODIR_PRIMARY_X` = `7` — The auto direction type with joint direction parallel to the primary X-axis will be used.
- `AUTODIR_FROM_ATTRIBUTE_FILE` = `99` — The auto direction type is fetched from the loaded attribute file.

## ClashCheckOptions (class)

The ClashCheckOptions class represents the clash check options.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/7d696961-d1d1-d6bd-91a4-96109ba5b0d5)

### Constructors
- `public ClashCheckOptions()` — Initializes a new instance of the ClashCheckOptions class

### Properties
- `BoltHeadDiameter` (Double, get/set) — The bolt head diameter.
- `NutThickness` (Double, get/set) — The nut thickness.

## ComponentDefinitionTypeEnum (enum)

The component definition type tells what kind of a component is in question.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/f28368d2-a894-54a5-ba13-eb1eea31433c)

### Values
- `SYSTEM` = `0` — With the type system, component is defined using developer-kit
- `CUSTOM` = `-1` — With the type custom, component is defined as custom component
- `DOTNET` = `-100000` — With the type dotnet, component is defined using .NET plug-in interface
- `DOTNET_CONNECTION` = `-200000` — With the type dotnet_connection, connection is defined using .NET plug-in interface
- `DOTNET_DRAWING` = `-300000` — With the type dotnet_drawing, drawing component is defined using .NET drawing plug-in interface

## ComponentOptions (class)

The ComponentOptions class represents the component options.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/af50ad50-32e9-d812-74ef-dfe45170f2b0)

### Constructors
- `public ComponentOptions()` — Initializes a new instance of the ComponentOptions class

### Properties
- `AssemblyLoosePartPositionPrefix` (String, get/set) — The assembly loose position prefix.
- `AssemblyLoosePartStartNumber` (Int32, get/set) — The assembly loose part number.
- `BoltEdgeDistanceFactor` (Double, get/set) — The factor of bolt edge distance.
- `BoltEdgeDistanceReference` (ComponentOptions.BoltEdgeDistanceReferenceEnum, get/set) — The bolt edge distance reference.
- `BoltSize` (String, get/set) — The bolt size.
- `BoltStandard` (String, get/set) — The bolt standard.
- `FoldedPlateProfileName` (String, get/set) — The folded plate profile name.
- `LoosePartPositionPrefix` (String, get/set) — The loose part position prefix.
- `LoosePartStartNumber` (Int32, get/set) — The loose part start number.
- `PartMaterial` (String, get/set) — The part material.
- `PartWeldedToPrimaryPositionPrefix` (String, get/set) — The part welded to primary position prefix.
- `PartWeldedToPrimaryStartNumber` (Int32, get/set) — The part welded to primary start number.
- `PartWeldedToSecondaryPositionPrefix` (String, get/set) — The part welded to secondary position prefix.
- `PartWeldedToSecondaryStartNumber` (Int32, get/set) — The part welded to secondary start number.
- `PlateProfileName` (String, get/set) — The plate profile name.

## ComponentOptions.BoltEdgeDistanceReferenceEnum (enum)

The bolt edge distance reference type.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c668e8fe-10a6-c044-ca5d-f8216f73c335)

### Values
- `BOLT_DIAMETER` = `0` — Compares the edge distance to the bolt diameter.
- `HOLE_DIAMETER` = `1` — Compares the edge distance to the hole diameter.

## ConnectionStatusEnum (enum)

The status type defines what the status of a connection or a detail is. In the model the color of the symbol (green, yellow, red) indicates the status.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/89082ad9-82c1-795c-0e98-24eb136ccfed)

### Values
- `STATUS_UNKNOWN` = `0` — The status is unknown.
- `STATUS_OK` = `1` — The status is ok and the symbol color is green.
- `STATUS_WARNING` = `2` — The status indicates a warning and the symbol color is yellow.
- `STATUS_ERROR` = `3` — The status indicates an error and the symbol color is red.

## DetailTypeEnum (enum)

The detail type defines what kind of a detail is in question.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/0d7cdc36-44c0-7f88-778a-d419f621809d)

### Values
- `END` = `0` — With the detail type end, the X-axis is oriented towards the part's center point.
- `INTERMEDIATE` = `1` — With the detail type intermediate, the X-axis is oriented towards the end point of the part.
- `INTERMEDIATE_REVERSE` = `2` — With the detail type intermediate reverse, the X-axis is oriented towards the start point of the part.

## Identifier (class)

The Identifier class represents an identifier that holds information about the identifier number of an object.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/957a7507-7619-f1f9-acfb-6ff8de2d9e27)

### Constructors
- `public Identifier()` — Constructs an empty identifier.
- `public Identifier(Guid guid)` — Constructs an identifier with the given GUID.
- `public Identifier(int id)` — Constructs an identifier with the given ID.
- `public Identifier(string guid)` — Constructs an identifier with the given GUID string.

### Methods
#### `public bool Equals(Identifier otherIdentifier)`

Compares the identifier with another identifier instance.

**Parameters:**
- `otherIdentifier` (Tekla.Structures.Identifier) — The identifier to compare with.

**Returns:** `Boolean` — True if the identifiers are equal, false otherwise.

[Docs](https://developer.tekla.com/topic/en/18/47/05e18bd6-3e91-7fc2-4965-fd6fbc2de331)

#### `public override bool Equals(Object otherObject)`

Compares the identifier with another object instance.

**Parameters:**
- `otherObject` (System.Object) — The object to compare with.

**Returns:** `Boolean` — True if the objects are equal, false otherwise.

[Docs](https://developer.tekla.com/topic/en/18/47/4419ffcd-5422-43bf-66d3-eb2f5d7f8c37)

#### `public override int GetHashCode()`

Gets the hash number of the identifier.

**Returns:** `Int32` — The hashed number.

[Docs](https://developer.tekla.com/topic/en/18/47/db9e3441-19e4-840e-44de-55692dc0890c)

#### `public bool IsValid()`

Returns true if the identifier seems to be valid. The validation is done based on the ID or GUID property.

**Returns:** `Boolean` — True if the identifier seems to be valid.

[Docs](https://developer.tekla.com/topic/en/18/47/ad94afee-596d-cd8c-4a85-854c234e178c)

#### `public override string ToString()`

Returns the integer ID as a string.

**Returns:** `String` — The integer ID as a string.

[Docs](https://developer.tekla.com/topic/en/18/47/ea0f0a40-bce9-e6a4-cf65-06f82a19ac59)

### Properties
- `GUID` (Guid, get/set) — The object's globally unique identifier. If the GUID value is set manually, the ID is initialized to 0 and the identification is done based on the GUID.
- `ID` (Int32, get/set) — The identifier number. If the ID value is set manually, the GUID is initialized to Guid.Empty and the identification is done based on the ID.
- `ID2` (Int32, get/set) — The sub identifier number. If the subID value is set manually, the GUID is initialized to Guid.Empty and the identification is done based on the ID.

## ModuleManager (class)

The ModuleManager class handles the product model module configuration information: the information that defines what configuration the customer is currently running.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/03aec97c-c61b-fc99-7d7e-e4c7c935b7b6)

### Constructors
- `public ModuleManager()` — Initializes a new instance of the ModuleManager class

### Properties
- `AnalysisAndDesign` (Boolean, get) — Indicates whether the Analysis and Design management is enabled.
- `CIPModeling` (Boolean, get) — Indicates whether the CIP modeling is enabled.
- `ConcreteDetailing` (Boolean, get) — Indicates whether the concrete detailing is enabled.
- `Configuration` (ModuleManager.ProgramConfigurationEnum, get) — The currently running configuration of Tekla Structures.
- `LoadModeling` (Boolean, get) — Indicates whether the load modeling is enabled.
- `MultimaterialModeling` (Boolean, get) — Indicates whether the multimaterial modeling is enabled.
- `RebarModeling` (Boolean, get) — Indicates whether the rebar modeling is enabled.
- `SteelDetailing` (Boolean, get) — Indicates whether the steel detailing is enabled.
- `TaskManagement` (Boolean, get) — Indicates whether the task management is enabled.

## ModuleManager.ProgramConfigurationEnum (enum)

The configuration information of the program.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a252a416-c746-9a3f-65f0-aefe05f547f7)

### Values
- `CONFIGURATION_VIEWER` = `0` — The viewer configuration.
- `CONFIGURATION_PRODUCTION_PLANNER` = `1` — The production planner configuration.
- `CONFIGURATION_DRAFTER` = `2` — The drafter configuration.
- `CONFIGURATION_PROJECT_MANAGEMENT` = `3` — The project management configuration.
- `CONFIGURATION_ENGINEERING` = `4` — The construction management configuration.
- `CONFIGURATION_REINFORCED_CONCRETE_DETAILING` = `5` — The reinforced concrete detailing configuration.
- `CONFIGURATION_PRECAST_CONCRETE_DETAILING` = `6` — The precast concrete detailing configuration.
- `CONFIGURATION_STEEL_DETAILING` = `7` — The steel detailing configuration.
- `CONFIGURATION_FULL` = `8` — The full detailing configuration.
- `CONFIGURATION_PRIMARY` = `9` — The Primary configuration.
- `CONFIGURATION_EDUCATIONAL` = `10` — The educational configuration.
- `CONFIGURATION_DEVELOPER` = `11` — The developer configuration.
- `CONFIGURATION_CONSTRUCTION_MODELING` = `12` — The construction management configuration with modeling capabilities.
- `CONFIGURATION_STEEL_DETAILING_LIMITED` = `9` — The old steel detailing limited configuration. This definition was left here to enable old macros in 19.0.
- `CONFIGURATION_CONSTRUCTION_VIEWER` = `12` — The construction management configuration restricted to viewer mode.
- `CONFIGURATION_CONSTRUCTION_MANAGEMENT` = `12`
- `CONFIGURATION_PARTNER` = `13` — The partner configuration.
- `CONFIGURATION_EPM_MODELER` = `14` — The EPM Modeler configuration.
- `CONFIGURATION_CARBON` = `15` — The Carbon configuration.
- `CONFIGURATION_GRAPHITE` = `16` — The Graphite configuration.
- `CONFIGURATION_DIAMOND` = `17` — The Diamond configuration.
- `CONFIGURATION_MEP` = `18` — The MEP configuration.
- `CONFIGURATION_TRIAL` = `19` — The Trial Configuration.
- `CONFIGURATION_PLATFORM` = `20` — The Tekla Platform configuration.

## PositionTypeEnum (enum)

The position type for connections and seams.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c01f30a8-da00-a772-9a9e-2aaaceec2259)

### Values
- `MIDDLE_PLANE` = `0` — The middle plane position type.
- `BOX_PLANE` = `1` — The box plane position type.
- `COLLISION_PLANE` = `2` — The collision plane position type.
- `END_END_PLANE` = `3` — The end plane position type.
- `GUSSET_PLANE` = `4` — The gusset plane position type.

## PropertyTypeEnum (enum)

The type of property: int/double/string.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d70b9e6f-c199-2166-82a9-46fb8d3d6c16)

### Values
- `TYPE_INT` = `0` — The integer property type.
- `TYPE_DOUBLE` = `1` — The double property type.
- `TYPE_STRING` = `2` — The string property type.

## TeklaStructuresDatabaseTypeEnum (enum)

The object types to be used in filter expressions.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/fa0bf845-2589-975e-2598-c8d53823bb09)

### Values
- `UNKNOWN` = `0` — The unknown type.
- `POINT` = `1` — The point type, this is not a ModelObject.
- `PART` = `2` — The part type.
- `CONNECTION` = `3` — The connection type, used for details and seams as well.
- `COMPONENT` = `4` — The component type, used for macros, plug-ins and custom parts.
- `GRID` = `7` — The grid type.
- `FITTING` = `9` — The fitting type.
- `BOLT` = `10` — The bolt type.
- `PART_CUT` = `11` — The part cut type.
- `PLANE_CUT` = `12` — The plane cut type.
- `WELDING` = `13` — The welding type.
- `ASSEMBLY` = `15` — The assembly type.
- `CONSTRUCTION_LINE` = `24` — The construction line type, this is not a ModelObject.
- `PLANE` = `30` — The plane type, used for control planes and grid planes.
- `DRAWING` = `34` — The drawing.
- `DB_CONNECTION` = `35` — The connection type.
- `PART_ADD` = `38` — The part add type.
- `WELD_CUT` = `39` — The weld preparation.
- `CONSTRUCTION_CIRCLE` = `42` — The construction circle type, this is not a ModelObject.
- `CONSTRUCTION_ARC` = `43` — The construction arc
- `DB_COMPONENT` = `46` — The component type.
- `REBAR` = `47` — The rebar type, used for single rebars, groups, meshes and strands.
- `FOREIGN_OBJECT` = `48` — The foreign object.
- `LOAD_LOAD` = `49` — The load type.
- `LOAD_POINT` = `50` — The load point type.
- `LOAD_LINE` = `51` — The load line type.
- `LOAD_AREA` = `52` — The load area type.
- `LOAD_UNIFORM` = `53` — The load uniform type.
- `LOAD_GROUP` = `54` — The load group type.
- `LOAD_TEMPERATURELOAD` = `72` — The temperature load type.
- `SURFACE_TREATMENT` = `73` — The surface treatment type.
- `REBAR_SPLICE` = `74` — The rebar splice type.
- `ANALYSIS_MODEL` = `75` — The analysis model.
- `ANALYSIS_PART` = `76` — The analysis part.
- `ANALYSIS_NODE` = `77` — The analysis node.
- `EDGE_CHAMFER` = `79` — The edge chamfer type.
- `TASK` = `80` — The task type.
- `TASK_DEPENDENCY` = `81` — The task dependency type.
- `TASK_WORKTYPE` = `83` — The task worktype type.
- `HIERARCHIC_DEFINITION` = `84` — The hierarchic definition type.
- `HIERARCHIC_OBJECT` = `85` — The hierarchic object type.
- `DB_POUR_BREAK` = `89` — The pour break type.
- `DB_POUR_OBJECT` = `90` — The pour object type.
- `REBARSET_ADDITION` = `95` — The rebar set addition type.
- `REBARSET_MODIFIER` = `96` — The rebar set modifier type.
- `SURFACE_OBJECT` = `97` — The surface object type
- `BENT_PLATE` = `98` — The bent plate type
- `HELIX` = `99` — The helix type
- `DB_POUR_UNIT` = `101` — The pour unit type
- `CONSTRUCTION_POLYCURVE` = `103` — The construction polycurve
- `STOREY` = `104` — Building Hierarchy Storey
- `BUILDING_SITE` = `105` — Building Hierarchy Building
- `BUILDING` = `106` — Building Hierarchy Site
- `SPACE` = `108` — Building Hierarchy Space

## TeklaStructuresFiles (class)

The TeklaStructuresFiles class is for the paths where the attributes file will be searched for.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/085dde53-6ee8-0f4f-0872-55b88314e523)

### Constructors
- `public TeklaStructuresFiles(string modelpath = "")` — Initializes a new instance of the TeklaStructuresFiles class.

### Methods
#### `[ObsoleteAttribute("Use Generic Locate File")] public FileInfo GetAttributeFile(List<string> searchDirectories, string fileName)`

Gets a file info representing the first match in the search directories.

**Parameters:**
- `searchDirectories` (System.Collections.Generic.List<String>) — The list of directories to be used for searching for the file.
- `fileName` (System.String) — The name of the file including the file extension.

**Returns:** `FileInfo` — A file info for the first match in the directory list. Null if no match was found.

[Docs](https://developer.tekla.com/topic/en/18/47/d5bb70ef-8c5c-698e-3c33-5121d54eee52)

#### `public FileInfo GetAttributeFile(string fileName)`

Gets a file info representing the first match in the standard property file directories.

**Parameters:**
- `fileName` (System.String) — The name of the file including the file extension.

**Returns:** `FileInfo` — A file info for the first match in the directory list. Null if no match was found.

[Docs](https://developer.tekla.com/topic/en/18/47/9a341c18-e6db-ebe1-2ab9-afcf2ec9ce15)

#### `public static Dictionary<string, string> GetFileDictionaryByExtension(string fileExtension, string modelPath)`

Retrieves a dictionary of file names and their corresponding paths from multiple directories.

**Parameters:**
- `fileExtension` (System.String) — The file extension to filter the files by.
- `modelPath` (System.String) — The model path.

**Returns:** `Dictionary<String,String>` — A dictionary containing file names without the extension and their paths.

[Docs](https://developer.tekla.com/topic/en/18/47/a9bbbec7-6b73-ffcd-a2e0-1d3a249dfb05)

#### `public List<string> GetMultiDirectoryFileList(string fileExtension, bool fullpath = false)`

Gets a list of files with the given extension from the default search directories.

**Parameters:**
- `fileExtension` (System.String) — The file extension to be used.
- `fullpath` (System.Boolean) — If true, file with full path with extension is returned.

**Returns:** `List<String>` — A list of files with the given extension.

[Docs](https://developer.tekla.com/topic/en/18/47/da0bbcb4-ea14-c355-cb9e-d98cbbbbf5c7)

#### `public static bool LocateFile(List<string> searchDirectories, string fileName, out string absoluteFileName)`

This method locates a file in the specified search directories. The first found file will be returned. If the file is not found in the search directories, it will return false and the out parameter will be empty.

**Parameters:**
- `searchDirectories` (System.Collections.Generic.List<String>) — list of search directories
- `fileName` (System.String) — filename to search
- `absoluteFileName` (System.String.) — output path if located

**Returns:** `Boolean` — true if found, false if not

[Docs](https://developer.tekla.com/topic/en/18/47/b33077ea-aee2-425c-cf42-13145936849e)

### Properties
- `PropertyFileDirectories` (List<String>, get/set) — The directories where to look for property files.
- `SessionLogFile` (FileInfo, get) — Gets the session history log file.

## TeklaStructuresInfo (class)

The TeklaStructuresInfo class provides information about Tekla Structures.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2a9f3fee-0ba1-486f-02bf-75a848487149)

### Constructors
- `public TeklaStructuresInfo()` — Initializes a new instance of the TeklaStructuresInfo class

### Methods
#### `public static string GetBuildNumber()`

Returns the Tekla Structures build number.

**Returns:** `String` — The current buildnumber of Tekla Structures.

[Docs](https://developer.tekla.com/topic/en/18/47/50f50908-3f5c-2773-53fd-224a6eeff144)

#### `public static string GetCommonAppDataFolder()`

Returns (XSDATADIR)environments installation folder of the Tekla Structures as string.

**Returns:** `String` — The (XSDATADIR)environments installation folder of the Tekla Structures.

[Docs](https://developer.tekla.com/topic/en/18/47/05bcf770-3562-717a-cfb7-54411b2042f8)

#### `public static string GetCopyRightText()`

Returns the Tekla Structures copyright text as string.

**Returns:** `String` — The current copyright text of Tekla Structures.

[Docs](https://developer.tekla.com/topic/en/18/47/e6370f74-6753-5fbe-b34d-1f4707bc8c0c)

#### `public static string GetCurrentProgramVersion()`

Returns the current Tekla Structures version.

**Returns:** `String` — The current version of Tekla Structures.

[Docs](https://developer.tekla.com/topic/en/18/47/fb2133c4-9bbd-c937-7c28-360dcc2a1024)

#### `public static string GetCurrentUser()`

Returns the current Tekla Structures user.

**Returns:** `String` — The current user of Tekla Structures.

[Docs](https://developer.tekla.com/topic/en/18/47/bfb41c15-c986-3f19-deed-441826d2d6a2)

#### `public static string GetFullTSRegistryKeyText()`

Returns full registry key of the Tekla Structures as string.

**Returns:** `String` — The full registry key of the Tekla Structures.

[Docs](https://developer.tekla.com/topic/en/18/47/8c418356-738f-fd33-7979-0be189410276)

#### `public static string GetLocalAppDataFolder()`

Returns (XSUSERDATADIR) user's local appdata folder of the Tekla Structures as string.

**Returns:** `String` — The (XSUSERDATADIR)user's local appdata folder of the Tekla Structures.

[Docs](https://developer.tekla.com/topic/en/18/47/6eab7354-6339-58f6-1a10-6b7087297f4d)

#### `public static string GetPluginsFolder()`

Returns main TS plugins folder, not extensions folder

**Returns:** `String` — plugins folder

[Docs](https://developer.tekla.com/topic/en/18/47/c59332bb-8052-cc1d-e593-db4706d2b0ba)

#### `public static string GetRevisionDate()`

Returns the Tekla Structures revision date as string.

**Returns:** `String` — The current revision date of Tekla Structures.

[Docs](https://developer.tekla.com/topic/en/18/47/0908b72b-8b87-70bb-02c3-076e5ad3cfc5)

## TeklaStructuresSettings (class)

The TeklaStructuresSettings class provides methods to inquire application settings.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/7c4799d1-b9cb-cbb1-9b3d-4ce81b05ccc6)

### Constructors
- `public TeklaStructuresSettings()` — Initializes a new instance of the TeklaStructuresSettings class

### Methods
#### `public static bool GetAdvancedOption(string VariableName, ref bool Value)`

Returns the value of an advanced option variable.

**Parameters:**
- `VariableName` (System.String) — The name of the advanced option.
- `Value` (System.Boolean.) — The returned value of the advanced option.

**Returns:** `Boolean` — True if the value was successfully retrieved.

[Docs](https://developer.tekla.com/topic/en/18/47/8aa73d82-ab8b-e43d-63c7-cdde9ff4eb6e)

#### `public static bool GetAdvancedOption(string VariableName, ref double Value)`

Returns the value of an advanced option variable.

**Parameters:**
- `VariableName` (System.String) — The name of the advanced option.
- `Value` (System.Double.) — The returned value of the advanced option.

**Returns:** `Boolean` — True if the value was successfully retrieved.

[Docs](https://developer.tekla.com/topic/en/18/47/a4ddad5c-eccd-0038-3056-e7c41a86eb13)

#### `public static bool GetAdvancedOption(string VariableName, ref int Value)`

Returns the value of an advanced option variable.

**Parameters:**
- `VariableName` (System.String) — The name of the advanced option.
- `Value` (System.Int32.) — The returned value of the advanced option.

**Returns:** `Boolean` — True if the value was successfully retrieved.

[Docs](https://developer.tekla.com/topic/en/18/47/765beeeb-fe53-93b4-24fd-d68dd05aa37c)

#### `public static bool GetAdvancedOption(string VariableName, ref string Value)`

Returns the value of an advanced option variable.

**Parameters:**
- `VariableName` (System.String) — The name of the advanced option.
- `Value` (System.String.) — The returned value of the advanced option.

**Returns:** `Boolean` — True if the value was successfully retrieved.

[Docs](https://developer.tekla.com/topic/en/18/47/251a7058-3458-8683-9135-d39aa3de182b)

#### `public static bool GetAdvancedOptionPaths(string advancedOption, out List<string> paths, TeklaStructuresSettings.InvalidPathCallback errorHandler = null)`

Gets the value of an advanced option as a list of valid paths. Strings with path separator ; are split into separate paths and any blank paths (containing only white space) are ignored. Paths do not need to exist but must use valid characters and format. Note: All valid paths are returned even when invalid paths are encountered.

**Parameters:**
- `advancedOption` (System.String) — The advanced option name.
- `paths` (System.Collections.Generic.List<String>.) — The paths.
- `errorHandler` (Tekla.Structures.TeklaStructuresSettings.InvalidPathCallback) — The optional error handler callback.

**Returns:** `Boolean` — True if the variable is read successfully and contains no invalid paths; otherwise false.

[Docs](https://developer.tekla.com/topic/en/18/47/00e86aee-ab47-95ed-1985-3f79cc91215c)

#### `public static bool GetOptions(ref ClashCheckOptions Options)`

Returns the value of the clash check options.

**Parameters:**
- `Options` (Tekla.Structures.ClashCheckOptions.) — The returned value of the clash check options.

**Returns:** `Boolean` — True if the values were successfully retrieved.

[Docs](https://developer.tekla.com/topic/en/18/47/fa761e3b-f4ef-59b8-da2d-2cba78d907d4)

#### `public static bool GetOptions(ref ComponentOptions Options)`

Returns the value of the component options.

**Parameters:**
- `Options` (Tekla.Structures.ComponentOptions.) — The returned value of the component options.

**Returns:** `Boolean` — True if the values were successfully retrieved.

[Docs](https://developer.tekla.com/topic/en/18/47/870f7c68-41f6-22e5-e591-550bede2d30b)

#### `public static bool IsPourEnabled()`

Determines whether pour management is enabled.

**Returns:** `Boolean` — true if pour is enabled; otherwise, false.

[Docs](https://developer.tekla.com/topic/en/18/47/11cf2635-0c47-669c-770f-228d798e0d4b)

#### `public static bool IsToolOptionOn(string toolOptionName)`

Determines whether select switch is on.

**Parameters:**
- `toolOptionName` (System.String) — Name of the switch.

**Returns:** `Boolean` — true if select switch is on; otherwise, false.

[Docs](https://developer.tekla.com/topic/en/18/47/43438834-9d98-9090-4a02-15e6fe958c33)

#### `public static bool SetOptions(ClashCheckOptions Options)`

Sets the clash check options.

**Parameters:**
- `Options` (Tekla.Structures.ClashCheckOptions) — The clash check options to be set.

**Returns:** `Boolean` — True if the clash check options were successfully set.

[Docs](https://developer.tekla.com/topic/en/18/47/0de6b6e4-7696-1801-862e-fe2b8cf0aad5)

#### `public static bool SetOptions(ComponentOptions Options)`

Sets the component options.

**Parameters:**
- `Options` (Tekla.Structures.ComponentOptions) — The component options to be set.

**Returns:** `Boolean` — True if the component options were successfully set.

[Docs](https://developer.tekla.com/topic/en/18/47/1c1d8098-bb5b-7a6c-c73a-5069345ac004)

## TeklaStructuresSettings.InvalidPathCallback (delegate)

The invalid path callback - provides error logging for GetAdvancedOptionPaths when a string is found to be invalid as a path

[Vendor docs](https://developer.tekla.com/topic/en/18/47/390aea79-c14b-54ea-a412-7624c2061f40)

## TeklaStructuresSettings.ToolOptionNames (class)

Tool option names.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/547f5380-8245-79e2-3758-11f811c4a0d4)

## TeklaStructuresVariables (class)

The EnvironmentVariables class contains a sorted list specializing in getting active environment variables and advanced option settings. It also checks options.ini files in the active model folder as well as options_user.ini files.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a2b71094-2e13-c03d-5ddd-723c90c5e43f)

### Methods
#### `public static void Add(string key)`

Adds a variable to the list.

**Parameters:**
- `key` (System.String) — The variable to be added.

[Docs](https://developer.tekla.com/topic/en/18/47/6be71bf8-01ce-7aef-1c32-22ba3f8ff426)

#### `public static bool ContainsVariable(string key)`

Gets a key.

**Parameters:**
- `key` (System.String) — The key to get.

**Returns:** `Boolean` — The key.

[Docs](https://developer.tekla.com/topic/en/18/47/35e0bbbf-008d-7f5d-f202-52276614aa3d)

#### `public static string Get(string key)`

Gets a key.

**Parameters:**
- `key` (System.String) — The key to get.

**Returns:** `String` — The key.

[Docs](https://developer.tekla.com/topic/en/18/47/f8b3823f-87a5-7e86-68b1-36598d8403f9)

