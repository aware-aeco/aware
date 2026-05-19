---
name: dynamo-dynamo-applications
description: This skill encodes the dynamo 4.1.0 surface of the Dynamo.Applications namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: StartupUtils.
---

# Dynamo.Applications

Auto-generated from vendor docs for dynamo 4.1.0. 1 types in this namespace.

## StartupUtils (static-class)

Type StartupUtils

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoApplications/StartupUtils.cs)

### Methods
#### `System.Collections.Generic.List<System.Exception> CheckAssemblyForVersionMismatches(System.Reflection.Assembly assembly)`

Checks that an assembly does not have any dependencies that have already been loaded into the appDomain with an incompatible to the one Dynamo requires.

**Parameters:**
- `assembly` (System.Reflection.Assembly) — 

**Returns:** `System.Collections.Generic.List<System.Exception>` — returns a list of fileLoad exceptions - if the list is empty no mismatched assemblies were encountered

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoApplications/StartupUtils.cs)

#### `Dynamo.Models.DynamoModel MakeCLIModel(Dynamo.Applications.StartupUtils.CommandLineArguments cmdLineArgs)`

Use this overload to construct a DynamoModel in CLI context when the location of ASM to use is known, host analytics info is known and you want to set data paths.

**Parameters:**
- `cmdLineArgs` (Dynamo.Applications.StartupUtils.CommandLineArguments) — 

**Returns:** `Dynamo.Models.DynamoModel` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoApplications/StartupUtils.cs)

#### `Dynamo.Models.DynamoModel MakeCLIModel(string asmPath, string userDataFolder, string commonDataFolder, Dynamo.Models.HostAnalyticsInfo info)`

Use this overload to construct a DynamoModel in CLI context when the location of ASM to use is known, host analytics info is known and you want to set data paths.

**Parameters:**
- `asmPath` (string) — Path to directory containing geometry library binaries
- `userDataFolder` (string) — Path to be used by PathResolver for UserDataFolder
- `commonDataFolder` (string) — Path to be used by PathResolver for CommonDataFolder
- `info` (Dynamo.Models.HostAnalyticsInfo) — Host analytics info specifying Dynamo launching host related information.

**Returns:** `Dynamo.Models.DynamoModel` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoApplications/StartupUtils.cs)

#### `Dynamo.Models.DynamoModel MakeModel(bool CLImode, bool noNetworkMode, string asmPath, Dynamo.Models.HostAnalyticsInfo info)`

Use this overload to construct a DynamoModel when the location of ASM to use is known and host analytics info is known.

**Parameters:**
- `CLImode` (bool) — CLI mode starts the model in test mode and uses a separate path resolver.
- `noNetworkMode` (bool) — Option to initialize Dynamo in no-network mode
- `asmPath` (string) — Path to directory containing geometry library binaries
- `info` (Dynamo.Models.HostAnalyticsInfo) — Host analytics info specifying Dynamo launching host related information.

**Returns:** `Dynamo.Models.DynamoModel` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoApplications/StartupUtils.cs)

#### `Dynamo.Models.DynamoModel MakeModel(bool CLImode, string CLIlocale, bool noNetworkMode, string asmPath, Dynamo.Models.HostAnalyticsInfo info)`

Use this overload to construct a DynamoModel when the location of ASM to use is known and host analytics info is known.

**Parameters:**
- `CLImode` (bool) — CLI mode starts the model in test mode and uses a separate path resolver.
- `CLIlocale` (string) — CLI argument to force dynamo locale
- `noNetworkMode` (bool) — Option to initialize Dynamo in no-network mode
- `asmPath` (string) — Path to directory containing geometry library binaries
- `info` (Dynamo.Models.HostAnalyticsInfo) — Host analytics info specifying Dynamo launching host related information.

**Returns:** `Dynamo.Models.DynamoModel` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoApplications/StartupUtils.cs)

#### `Dynamo.Models.DynamoModel MakeModel(bool CLImode, string asmPath, string hostName)`

Use this overload to construct a DynamoModel when the location of ASM to use is known and host name is known.

**Parameters:**
- `CLImode` (bool) — CLI mode starts the model in test mode and uses a separate path resolver.
- `asmPath` (string) — Path to directory containing geometry library binaries
- `hostName` (string) — Dynamo variation identified by host.

**Returns:** `Dynamo.Models.DynamoModel` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoApplications/StartupUtils.cs)

#### `void PreloadShapeManager(ref string geometryFactoryPath, ref string preloaderLocation)`

Attempts to load the geometry library binaries using the location params.

**Parameters:**
- `geometryFactoryPath` (ref string) — libG ProtoInterface path
- `preloaderLocation` (ref string) — libG folder path

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoApplications/StartupUtils.cs)

#### `void PreloadShapeManager(ref string geometryFactoryPath, ref string preloaderLocation, ref System.Version asmVersion, ref string asmPath)`

Attempts to load the geometry library binaries and returns ASM version information.

**Parameters:**
- `geometryFactoryPath` (ref string) — libG ProtoInterface path
- `preloaderLocation` (ref string) — libG folder path
- `asmVersion` (ref System.Version) — Version of the loaded ASM, or null if version cannot be determined
- `asmPath` (ref string) — Path where ASM binaries are located, or empty string if not found

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoApplications/StartupUtils.cs)

#### `string SetLocale(Dynamo.Applications.StartupUtils.CommandLineArguments cmdLineArgs)`

StartupUtils.SetLocale

**Parameters:**
- `cmdLineArgs` (Dynamo.Applications.StartupUtils.CommandLineArguments)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoApplications/StartupUtils.cs)

### Events
#### `ASMPreloadFailure` (`System.Action<string>`)

**Signature:** `public event System.Action<string> ASMPreloadFailure`

Raised when loading of the ASM binaries fails. A failure message is passed as a parameter.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoApplications/StartupUtils.cs)

