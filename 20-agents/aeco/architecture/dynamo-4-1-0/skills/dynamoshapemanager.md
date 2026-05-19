---
name: dynamo-dynamoshapemanager
description: This skill encodes the dynamo 4.1.0 surface of the DynamoShapeManager namespace — 2 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Preloader, Utilities.
---

# DynamoShapeManager

Auto-generated from vendor docs for dynamo 4.1.0. 2 types in this namespace.

## Preloader (class)

Shape manager preloader class that helps with preloading Autodesk Shape Manager (ASM) binaries through geometry library (LibG). This class being part of Dynamo core module, relies on IGeometryConfiguration supplied by the host application to determine the installed location of ASM binaries.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoShapeManager/Preloader.cs)

### Constructors
- `void Preloader(string rootFolder)` — Constructs a preloader object to help preload shape manager.
- `void Preloader(string rootFolder, System.Collections.Generic.IEnumerable<System.Version> versions)` — Constructs a preloader object to help preload a specific version of shape manager.

### Methods
#### `void Preload()`

Attempts to load the geometry library binaries using the version and location specified when the Preloader was constructed.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoShapeManager/Preloader.cs)

### Properties
- `GeometryFactoryPath` (string, get) — Preloader.GeometryFactoryPath
- `PreloaderLocation` (string, get) — Preloader.PreloaderLocation
- `ShapeManagerPath` (string, get/set) — Preloader.ShapeManagerPath
- `Version2` (System.Version, get) — Preloader.Version2

## Utilities (static-class)

Type Utilities

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoShapeManager/Utilities.cs)

### Methods
#### `string GetGeometryFactoryPath2(string rootFolder, System.Version version)`

This method will return the path to the GeometryFactory assembly location for a requested version of the geometry library. This method is tolerant to the requested version in that it will attempt to locate an exact or lower version of the GeometryFactory assembly.

**Parameters:**
- `rootFolder` (string) — Full path of the directory that contains LibG_xxx_y_z folder, where 'xxx y z' represents the library version of asm. In a typical setup this would be the same directory that contains Dynamo core modules. This must represent a valid directory - it cannot be null.
- `version` (System.Version) — Version number of the targeted geometry library. If the resulting assembly does not exist, this method will look for a lower version match. This parameter cannot be null.

**Returns:** `string` — The full path to GeometryFactoryAssembly assembly.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoShapeManager/Utilities.cs)

#### `System.Version GetInstalledAsmVersion2(System.Collections.Generic.IEnumerable<System.Version> versions, ref string location, string rootFolder, System.Func<string, System.Collections.IEnumerable> getASMInstallsFunc)`

Call this method to determine the version of ASM that is installed on the user machine. The method scans through a list of known Autodesk product folders for ASM binaries with the targeted version.

**Parameters:**
- `versions` (System.Collections.Generic.IEnumerable<System.Version>) — A IEnumerable of version numbers to check for in order of preference. This argument cannot be null or empty.
- `location` (ref string) — The full path of the directory in which targeted ASM binaries are found. This argument cannot be null.
- `rootFolder` (string) — This method makes use of DynamoInstallDetective to determine the installation location of various Autodesk products. This argument is not optional and must represent the full path to the folder which contains DynamoInstallDetective.dll. An exception is thrown if the assembly cannot be located.
- `getASMInstallsFunc` (System.Func<string, System.Collections.IEnumerable>) — A delegate which can be used to replace the default ASM install lookup method. This is primarily used for testing. The delegate should return an IEnumerable of Tuples - these represent versions of ASM which are located on the user's machine.

**Returns:** `System.Version` — Returns System.Version of ASM if any installed ASM is found, or null otherwise.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoShapeManager/Utilities.cs)

#### `string GetLibGPreloaderLocation(System.Version version, string rootFolder)`

Get the corresponding libG preloader location for the target ASM loading version. If there is exact match preloader version to the target ASM version, use it, otherwise use the closest below.

**Parameters:**
- `version` (System.Version) — The target loading version of ASM.
- `rootFolder` (string) — Full path of the directory that contains LibG_major_minor_build folder. In a typical setup this would be the same directory that contains Dynamo core modules. This must represent a valid directory.

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoShapeManager/Utilities.cs)

#### `System.Version GetVersionFromPath(string asmPath, string searchPattern)`

Extracts version of ASM dlls from a path by scanning for ASM dlls in the path. Throws if ASM binaries cannot be found in the path.

**Parameters:**
- `asmPath` (string) — path to directory containing asm dlls
- `searchPattern` (string) — optional - to be used for testing - default is the ASM search pattern.

**Returns:** `System.Version` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoShapeManager/Utilities.cs)

#### `void PreloadAsmFromPath(string preloaderLocation, string asmLocation)`

Call this method to preload ASM binaries from a specific location. This method does not have a return value, any failures in loading ASM binaries will result in an exception being thrown.

**Parameters:**
- `preloaderLocation` (string) — Full path of the folder that contains PreloaderAssembly assembly. This argument must represent a valid path to the loader.
- `asmLocation` (string) — Full path of the folder that contains ASM binaries to load. This argument cannot be null or empty.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoShapeManager/Utilities.cs)

