---
name: dynamo-dynamo-packagemanager
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.PackageManager namespace — 22 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: FailFunc, IPathRemapper, IFileSystem, MutatingFileCompressor, MutatingFileSystem, PackageAssembly, PackageLoadState, Package, and 14 more types.
---

# Dynamo.PackageManager

Auto-generated from vendor docs for dynamo 4.1.1. 22 types in this namespace.

## AssemblyLoadingState (enum)

Type AssemblyLoadingState

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageLoader.cs)

### Values
- `AlreadyLoaded` = `2`
- `NotManagedAssembly` = `1`
- `Success` = `0`

## DynamoModelExtensions (static-class)

Type DynamoModelExtensions

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageManagerExtension.cs)

### Methods
#### `Dynamo.PackageManager.PackageManagerExtension GetPackageManagerExtension(Dynamo.Models.DynamoModel model)`

DynamoModelExtensions.GetPackageManagerExtension

**Parameters:**
- `model` (Dynamo.Models.DynamoModel)

**Returns:** `Dynamo.PackageManager.PackageManagerExtension` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageManagerExtension.cs)

## FailFunc (static-class)

Type FailFunc

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/FailFunc.cs)

### Methods
#### `T TryExecute<T>(System.Func<T> func, T failureResult)`

Attempt to run a computation. If the function throws, return the second argument unmodified

**Parameters:**
- `func` (System.Func<T>) — The function
- `failureResult` (T) — The result

**Returns:** `T` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/FailFunc.cs)

## IFileSystem (interface)

An abstract FileSystem for mocking purposes

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/IFileSystem.cs)

### Methods
#### `void CopyFile(string filePath, string destinationPath)`

IFileSystem.CopyFile

**Parameters:**
- `filePath` (string)
- `destinationPath` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/IFileSystem.cs)

#### `void DeleteDirectory(string directoryPath)`

IFileSystem.DeleteDirectory

**Parameters:**
- `directoryPath` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/IFileSystem.cs)

#### `void DeleteFile(string filePath)`

IFileSystem.DeleteFile

**Parameters:**
- `filePath` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/IFileSystem.cs)

#### `bool DirectoryExists(string directoryPath)`

IFileSystem.DirectoryExists

**Parameters:**
- `directoryPath` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/IFileSystem.cs)

#### `bool FileExists(string filePath)`

IFileSystem.FileExists

**Parameters:**
- `filePath` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/IFileSystem.cs)

#### `System.Collections.Generic.IEnumerable<string> GetDirectories(string dir)`

IFileSystem.GetDirectories

**Parameters:**
- `dir` (string)

**Returns:** `System.Collections.Generic.IEnumerable<string>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/IFileSystem.cs)

#### `System.Collections.Generic.IEnumerable<string> GetFiles(string dir)`

IFileSystem.GetFiles

**Parameters:**
- `dir` (string)

**Returns:** `System.Collections.Generic.IEnumerable<string>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/IFileSystem.cs)

#### `Dynamo.PackageManager.Interfaces.IDirectoryInfo TryCreateDirectory(string directoryPath)`

IFileSystem.TryCreateDirectory

**Parameters:**
- `directoryPath` (string)

**Returns:** `Dynamo.PackageManager.Interfaces.IDirectoryInfo` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/IFileSystem.cs)

#### `void WriteAllText(string filePath, string content)`

IFileSystem.WriteAllText

**Parameters:**
- `filePath` (string)
- `content` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/IFileSystem.cs)

## IPackageDirectoryBuilder (interface)

Type IPackageDirectoryBuilder

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageDirectoryBuilder.cs)

### Methods
#### `Dynamo.PackageManager.Interfaces.IDirectoryInfo BuildDirectory(Dynamo.PackageManager.Package packages, string packagesDirectory, System.Collections.Generic.IEnumerable<string> files, System.Collections.Generic.IEnumerable<string> markdownfiles)`

IPackageDirectoryBuilder.BuildDirectory

**Parameters:**
- `packages` (Dynamo.PackageManager.Package)
- `packagesDirectory` (string)
- `files` (System.Collections.Generic.IEnumerable<string>)
- `markdownfiles` (System.Collections.Generic.IEnumerable<string>)

**Returns:** `Dynamo.PackageManager.Interfaces.IDirectoryInfo` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageDirectoryBuilder.cs)

#### `Dynamo.PackageManager.Interfaces.IDirectoryInfo BuildRetainDirectory(Dynamo.PackageManager.Package package, string packagesDirectory, System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<string>> contentFiles, System.Collections.Generic.IEnumerable<string> markdownFiles)`

IPackageDirectoryBuilder.BuildRetainDirectory

**Parameters:**
- `package` (Dynamo.PackageManager.Package)
- `packagesDirectory` (string)
- `contentFiles` (System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<string>>)
- `markdownFiles` (System.Collections.Generic.IEnumerable<string>)

**Returns:** `Dynamo.PackageManager.Interfaces.IDirectoryInfo` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageDirectoryBuilder.cs)

#### `Dynamo.PackageManager.Interfaces.IDirectoryInfo BuildRetainDirectory(Dynamo.PackageManager.Package package, string packagesDirectory, System.Collections.Generic.IEnumerable<string> roots, System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<string>> contentFiles, System.Collections.Generic.IEnumerable<string> markdownFiles)`

IPackageDirectoryBuilder.BuildRetainDirectory

**Parameters:**
- `package` (Dynamo.PackageManager.Package)
- `packagesDirectory` (string)
- `roots` (System.Collections.Generic.IEnumerable<string>)
- `contentFiles` (System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<string>>)
- `markdownFiles` (System.Collections.Generic.IEnumerable<string>)

**Returns:** `Dynamo.PackageManager.Interfaces.IDirectoryInfo` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageDirectoryBuilder.cs)

## IPackageUploadBuilder (interface)

Type IPackageUploadBuilder

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageUploadBuilder.cs)

### Methods
#### `Greg.Requests.PackageUpload NewPackageRetainUpload(Dynamo.PackageManager.Package package, string packagesDirectory, System.Collections.Generic.IEnumerable<string> roots, System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<string>> files, System.Collections.Generic.IEnumerable<string> markdownFiles, Dynamo.PackageManager.PackageUploadHandle handle)`

IPackageUploadBuilder.NewPackageRetainUpload

**Parameters:**
- `package` (Dynamo.PackageManager.Package)
- `packagesDirectory` (string)
- `roots` (System.Collections.Generic.IEnumerable<string>)
- `files` (System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<string>>)
- `markdownFiles` (System.Collections.Generic.IEnumerable<string>)
- `handle` (Dynamo.PackageManager.PackageUploadHandle)

**Returns:** `Greg.Requests.PackageUpload` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageUploadBuilder.cs)

#### `Greg.Requests.PackageUpload NewPackageUpload(Dynamo.PackageManager.Package package, string packagesDirectory, System.Collections.Generic.IEnumerable<string> files, System.Collections.Generic.IEnumerable<string> markdownFiles, Dynamo.PackageManager.PackageUploadHandle handle)`

IPackageUploadBuilder.NewPackageUpload

**Parameters:**
- `package` (Dynamo.PackageManager.Package)
- `packagesDirectory` (string)
- `files` (System.Collections.Generic.IEnumerable<string>)
- `markdownFiles` (System.Collections.Generic.IEnumerable<string>)
- `handle` (Dynamo.PackageManager.PackageUploadHandle)

**Returns:** `Greg.Requests.PackageUpload` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageUploadBuilder.cs)

#### `Greg.Requests.PackageVersionUpload NewPackageVersionRetainUpload(Dynamo.PackageManager.Package package, string packagesDirectory, System.Collections.Generic.IEnumerable<string> roots, System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<string>> files, System.Collections.Generic.IEnumerable<string> markdownFiles, Dynamo.PackageManager.PackageUploadHandle handle)`

IPackageUploadBuilder.NewPackageVersionRetainUpload

**Parameters:**
- `package` (Dynamo.PackageManager.Package)
- `packagesDirectory` (string)
- `roots` (System.Collections.Generic.IEnumerable<string>)
- `files` (System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<string>>)
- `markdownFiles` (System.Collections.Generic.IEnumerable<string>)
- `handle` (Dynamo.PackageManager.PackageUploadHandle)

**Returns:** `Greg.Requests.PackageVersionUpload` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageUploadBuilder.cs)

#### `Greg.Requests.PackageVersionUpload NewPackageVersionUpload(Dynamo.PackageManager.Package package, string packagesDirectory, System.Collections.Generic.IEnumerable<string> files, System.Collections.Generic.IEnumerable<string> markdownFiles, Dynamo.PackageManager.PackageUploadHandle handle)`

IPackageUploadBuilder.NewPackageVersionUpload

**Parameters:**
- `package` (Dynamo.PackageManager.Package)
- `packagesDirectory` (string)
- `files` (System.Collections.Generic.IEnumerable<string>)
- `markdownFiles` (System.Collections.Generic.IEnumerable<string>)
- `handle` (Dynamo.PackageManager.PackageUploadHandle)

**Returns:** `Greg.Requests.PackageVersionUpload` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageUploadBuilder.cs)

## IPathRemapper (interface)

A simplified interface for remapping the file path of a CustomNodeWorkspace. Useful for when a custom node will be moved by a package creation operation.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/CustomNodePathRemapper.cs)

### Methods
#### `bool SetPath(string originalPath, string newDirectoryPath)`

Remap the custom node path

**Parameters:**
- `originalPath` (string) — The original path
- `newDirectoryPath` (string) — The final directory path

**Returns:** `bool` — If successful, returns true

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/CustomNodePathRemapper.cs)

## LoadPackageParams (struct)

Type LoadPackageParams

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageLoader.cs)

### Properties
- `Preferences` (Dynamo.Interfaces.IPreferences, get/set) — LoadPackageParams.Preferences

## MutatingFileCompressor (class)

An IFileCompressor that actually attempts to compress a directory

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/MutatingFileCompressor.cs)

### Constructors
- `void MutatingFileCompressor()` — MutatingFileCompressor.MutatingFileCompressor

### Methods
#### `Dynamo.PackageManager.Interfaces.IFileInfo Zip(Dynamo.PackageManager.Interfaces.IDirectoryInfo directory)`

MutatingFileCompressor.Zip

**Parameters:**
- `directory` (Dynamo.PackageManager.Interfaces.IDirectoryInfo)

**Returns:** `Dynamo.PackageManager.Interfaces.IFileInfo` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/MutatingFileCompressor.cs)

## MutatingFileSystem (class)

An IFileSystem that actually mutates the underlying file system

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/MutatingFileSystem.cs)

### Constructors
- `void MutatingFileSystem()` — MutatingFileSystem.MutatingFileSystem

### Methods
#### `void CopyFile(string filePath, string destinationPath)`

MutatingFileSystem.CopyFile

**Parameters:**
- `filePath` (string)
- `destinationPath` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/MutatingFileSystem.cs)

#### `void DeleteDirectory(string directoryPath)`

MutatingFileSystem.DeleteDirectory

**Parameters:**
- `directoryPath` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/MutatingFileSystem.cs)

#### `void DeleteFile(string filePath)`

MutatingFileSystem.DeleteFile

**Parameters:**
- `filePath` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/MutatingFileSystem.cs)

#### `bool DirectoryExists(string directoryPath)`

MutatingFileSystem.DirectoryExists

**Parameters:**
- `directoryPath` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/MutatingFileSystem.cs)

#### `bool FileExists(string filePath)`

MutatingFileSystem.FileExists

**Parameters:**
- `filePath` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/MutatingFileSystem.cs)

#### `System.Collections.Generic.IEnumerable<string> GetDirectories(string dir)`

MutatingFileSystem.GetDirectories

**Parameters:**
- `dir` (string)

**Returns:** `System.Collections.Generic.IEnumerable<string>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/MutatingFileSystem.cs)

#### `System.Collections.Generic.IEnumerable<string> GetFiles(string dir)`

MutatingFileSystem.GetFiles

**Parameters:**
- `dir` (string)

**Returns:** `System.Collections.Generic.IEnumerable<string>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/MutatingFileSystem.cs)

#### `Dynamo.PackageManager.Interfaces.IDirectoryInfo TryCreateDirectory(string path)`

MutatingFileSystem.TryCreateDirectory

**Parameters:**
- `path` (string)

**Returns:** `Dynamo.PackageManager.Interfaces.IDirectoryInfo` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/MutatingFileSystem.cs)

#### `void WriteAllText(string filePath, string content)`

MutatingFileSystem.WriteAllText

**Parameters:**
- `filePath` (string)
- `content` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Interfaces/MutatingFileSystem.cs)

## Package (class)

Type Package

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Package.cs)

### Constructors
- `void Package(string directory, string name, string versionName, string license)` — Package.Package

### Methods
#### `void EnumerateAdditionalFiles()`

Package.EnumerateAdditionalFiles

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Package.cs)

#### `System.Collections.Generic.IEnumerable<string> EnumerateAssemblyFilesInPackage()`

Package.EnumerateAssemblyFilesInPackage

**Returns:** `System.Collections.Generic.IEnumerable<string>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Package.cs)

#### `Dynamo.PackageManager.Package FromDirectory(string rootPath, Dynamo.Logging.ILogger logger)`

Package.FromDirectory

**Parameters:**
- `rootPath` (string)
- `logger` (Dynamo.Logging.ILogger)

**Returns:** `Dynamo.PackageManager.Package` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Package.cs)

#### `Dynamo.PackageManager.Package FromJson(string headerPath, Dynamo.Logging.ILogger logger)`

Package.FromJson

**Parameters:**
- `headerPath` (string)
- `logger` (Dynamo.Logging.ILogger)

**Returns:** `Dynamo.PackageManager.Package` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Package.cs)

#### `void Log(Dynamo.Logging.ILogMessage obj)`

Package.Log

**Parameters:**
- `obj` (Dynamo.Logging.ILogMessage)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Package.cs)

#### `void Log(string s)`

Package.Log

**Parameters:**
- `s` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Package.cs)

### Properties
- `AdditionalFiles` (System.Collections.ObjectModel.ObservableCollection<Dynamo.PackageManager.PackageFileInfo>, get) — Package.AdditionalFiles
- `BinaryDirectory` (string, get) — Package.BinaryDirectory
- `CompatibilityMatrix` (System.Collections.ObjectModel.ObservableCollection<Greg.Requests.PackageCompatibility>, get) — Package.CompatibilityMatrix
- `Contents` (string, get/set) — Package.Contents
- `CopyrightHolder` (string, get/set) — Package.CopyrightHolder
- `CopyrightYear` (string, get/set) — Package.CopyrightYear
- `CustomNodeDirectory` (string, get) — Package.CustomNodeDirectory
- `Dependencies` (System.Collections.ObjectModel.ObservableCollection<Greg.Requests.PackageDependency>, get) — Package.Dependencies
- `Description` (string, get/set) — Package.Description
- `EngineVersion` (string, get/set) — Package.EngineVersion
- `ExtraDirectory` (string, get) — Package.ExtraDirectory
- `Group` (string, get/set) — Package.Group
- `Header` (Greg.Requests.PackageUploadRequestBody, get) — A header used to create the package, this data does not reflect runtime changes to the package, but instead reflects how the package was formed.
- `HostDependencies` (System.Collections.Generic.IEnumerable<string>, get/set) — Package Host Dependencies, e.g. specifying "Revit" in the list means this package can be guaranteed to work in this host environment only
- `ID` (string, get/set) — Package.ID
- `Keywords` (System.Collections.Generic.IEnumerable<string>, get/set) — Package.Keywords
- `License` (string, get/set) — Package.License
- `LoadedAssemblies` (System.Collections.ObjectModel.ObservableCollection<Dynamo.PackageManager.PackageAssembly>, get) — Package.LoadedAssemblies
- `LoadedCustomNodes` (System.Collections.ObjectModel.ObservableCollection<Dynamo.CustomNodeInfo>, get) — Package.LoadedCustomNodes
- `LoadedTypes` (System.Collections.ObjectModel.ObservableCollection<System.Type>, get) — Package.LoadedTypes
- `Name` (string, get/set) — Package.Name
- `NodeDocumentaionDirectory` (string, get) — Directory path to where node documentation markdown files should be placed.
- `RepositoryUrl` (string, get/set) — Package.RepositoryUrl
- `RootDirectory` (string, get/set) — Package.RootDirectory
- `SiteUrl` (string, get/set) — Package.SiteUrl
- `TypesVisibleInManager` (bool, get/set) — Package.TypesVisibleInManager
- `VersionName` (string, get/set) — Package.VersionName

### Events
#### `MessageLogged` (`System.Action<Dynamo.Logging.ILogMessage>`)

**Signature:** `public event System.Action<Dynamo.Logging.ILogMessage> MessageLogged`

Package.MessageLogged

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Package.cs)

## PackageAssembly (class)

Type PackageAssembly

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Package.cs)

### Constructors
- `void PackageAssembly()` — PackageAssembly.PackageAssembly

### Properties
- `Assembly` (System.Reflection.Assembly, get/set) — PackageAssembly.Assembly
- `IsNodeLibrary` (bool, get/set) — PackageAssembly.IsNodeLibrary
- `LocalFilePath` (string, get/set) — PackageAssembly.LocalFilePath
- `Name` (string, get) — PackageAssembly.Name

## PackageDownloadHandle (class)

View model for the installation of a package

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageDownloadHandle.cs)

### Constructors
- `void PackageDownloadHandle()` — Creates an empty view model for a package installation

### Methods
#### `void Done(string filePath)`

Transition the installation to downloaded with a path to the file

**Parameters:**
- `filePath` (string) — Path to the file

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageDownloadHandle.cs)

#### `void Error(string errorString)`

Transitions the installation to error with an error message

**Parameters:**
- `errorString` (string) — Error message

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageDownloadHandle.cs)

#### `bool Extract(Dynamo.Models.DynamoModel dynamoModel, string installDirectory, ref Dynamo.PackageManager.Package pkg)`

Extracts and parses the metadata of a downloaded package

**Parameters:**
- `dynamoModel` (Dynamo.Models.DynamoModel) — Dynamo model
- `installDirectory` (string) — If specified, overrides Dynamo's default base folder for packages
- `pkg` (ref Dynamo.PackageManager.Package) — Metatda parsed from the package

**Returns:** `bool` — Whether the operation succeeded or not

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageDownloadHandle.cs)

### Properties
- `DownloadPath` (string, get/set) — Path where the package is downloaded to
- `DownloadState` (Dynamo.PackageManager.PackageDownloadHandle.State, get/set) — State of the installation of the package
- `ErrorString` (string, get/set) — Error message that resulted from an unsuccessful installation
- `Id` (string, get/set) — Identifier of the package
- `Name` (string, get/set) — Name of the package
- `VersionName` (string, get/set) — Version of the package

## PackageFileInfo (class)

Type PackageFileInfo

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageFileInfo.cs)

### Constructors
- `void PackageFileInfo(string packageRoot, string filename)` — PackageFileInfo.PackageFileInfo

### Properties
- `Model` (System.IO.FileInfo, get) — PackageFileInfo.Model
- `RelativePath` (string, get) — Filename relative to the package root directory

## PackageLoadState (class)

Describes a package's load state

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/Package.cs)

### Constructors
- `void PackageLoadState()` — PackageLoadState.PackageLoadState

### Properties
- `ErrorMessage` (string, get) — PackageLoadState.ErrorMessage
- `ScheduledState` (Dynamo.PackageManager.PackageLoadState.ScheduledTypes, get) — PackageLoadState.ScheduledState
- `State` (Dynamo.PackageManager.PackageLoadState.StateTypes, get) — PackageLoadState.State

## PackageLoader (class)

Type PackageLoader

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageLoader.cs)

### Methods
#### `Dynamo.PackageManager.Package GetOwnerPackage(Dynamo.CustomNodeInfo def)`

PackageLoader.GetOwnerPackage

**Parameters:**
- `def` (Dynamo.CustomNodeInfo)

**Returns:** `Dynamo.PackageManager.Package` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageLoader.cs)

#### `Dynamo.PackageManager.Package GetOwnerPackage(System.Type t)`

PackageLoader.GetOwnerPackage

**Parameters:**
- `t` (System.Type)

**Returns:** `Dynamo.PackageManager.Package` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageLoader.cs)

#### `Dynamo.PackageManager.Package GetOwnerPackage(string path)`

PackageLoader.GetOwnerPackage

**Parameters:**
- `path` (string)

**Returns:** `Dynamo.PackageManager.Package` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageLoader.cs)

#### `Dynamo.PackageManager.Package GetPackageFromRoot(string path)`

PackageLoader.GetPackageFromRoot

**Parameters:**
- `path` (string)

**Returns:** `Dynamo.PackageManager.Package` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageLoader.cs)

#### `bool IsUnderPackageControl(Dynamo.CustomNodeInfo def)`

PackageLoader.IsUnderPackageControl

**Parameters:**
- `def` (Dynamo.CustomNodeInfo)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageLoader.cs)

#### `bool IsUnderPackageControl(System.Reflection.Assembly t)`

PackageLoader.IsUnderPackageControl

**Parameters:**
- `t` (System.Reflection.Assembly)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageLoader.cs)

#### `bool IsUnderPackageControl(System.Type t)`

PackageLoader.IsUnderPackageControl

**Parameters:**
- `t` (System.Type)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageLoader.cs)

#### `bool IsUnderPackageControl(string path)`

PackageLoader.IsUnderPackageControl

**Parameters:**
- `path` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageLoader.cs)

#### `void LoadAll(Dynamo.PackageManager.LoadPackageParams loadPackageParams)`

Scan the PackagesDirectory for packages and attempt to load all of them. Beware! Fails silently for duplicates.

**Parameters:**
- `loadPackageParams` (Dynamo.PackageManager.LoadPackageParams)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageLoader.cs)

#### `void LoadCustomNodesAndPackages(Dynamo.PackageManager.LoadPackageParams loadPackageParams, Dynamo.Core.CustomNodeManager customNodeManager)`

LoadCustomNodesAndPackages can be used to load custom nodes and packages from temporary paths that do not need to be added to preference settings. To load from temporary custom paths, initialize a local PreferenceSettings object and add the paths to its CustomPackageFolders property, then initialize a new LoadPackageParams with this preferences object and use as input to this method. To load from custom paths that need to be persisted to the preferences, initialize a LoadPackageParams from an existing preferences object.

**Parameters:**
- `loadPackageParams` (Dynamo.PackageManager.LoadPackageParams) — LoadPackageParams initialized with local PreferenceSettings object containing custom package path.
- `customNodeManager` (Dynamo.Core.CustomNodeManager) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageLoader.cs)

#### `void LoadPackages(System.Collections.Generic.IEnumerable<Dynamo.PackageManager.Package> packages)`

Loads and imports all packages.

**Parameters:**
- `packages` (System.Collections.Generic.IEnumerable<Dynamo.PackageManager.Package>) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageLoader.cs)

#### `Dynamo.PackageManager.Package ScanPackageDirectory(string directory)`

PackageLoader.ScanPackageDirectory

**Parameters:**
- `directory` (string)

**Returns:** `Dynamo.PackageManager.Package` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageLoader.cs)

#### `Dynamo.PackageManager.Package ScanPackageDirectory(string directory, bool checkCertificates)`

PackageLoader.ScanPackageDirectory

**Parameters:**
- `directory` (string)
- `checkCertificates` (bool)

**Returns:** `Dynamo.PackageManager.Package` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageLoader.cs)

### Properties
- `LocalPackages` (System.Collections.Generic.IEnumerable<Dynamo.PackageManager.Package>, get) — PackageLoader.LocalPackages
- `RequestedExtensions` (System.Collections.Generic.IEnumerable<Dynamo.Extensions.IExtension>, get) — Collection of ViewExtensions the ViewExtensionSource requested be loaded.

### Events
#### `ConflictingCustomNodePackageLoaded` (`System.Action<Dynamo.PackageManager.Package, Dynamo.PackageManager.Package>`)

**Signature:** `public event System.Action<Dynamo.PackageManager.Package, Dynamo.PackageManager.Package> ConflictingCustomNodePackageLoaded`

Event raised when a custom node package containing conflicting node definition with an existing package is tried to load.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageLoader.cs)

#### `PackageAdded` (`System.Action<Dynamo.PackageManager.Package>`)

**Signature:** `public event System.Action<Dynamo.PackageManager.Package> PackageAdded`

This event is raised when a package is first added to the list of packages this package loader is loading. This event occurs before the package is fully loaded.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageLoader.cs)

#### `PackageRemoved` (`System.Action<Dynamo.PackageManager.Package>`)

**Signature:** `public event System.Action<Dynamo.PackageManager.Package> PackageRemoved`

This event is raised when the package is removed from the list of packages loaded by this packageLoader.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageLoader.cs)

#### `PackgeLoaded` (`System.Action<Dynamo.PackageManager.Package>`)

**Signature:** `public event System.Action<Dynamo.PackageManager.Package> PackgeLoaded`

This event is raised when a package is fully loaded. It will be true that when this event is raised Packge.Loaded will be true for the package.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageLoader.cs)

## PackageManagerClient (class)

Type PackageManagerClient

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageManagerClient.cs)

### Methods
#### `bool SetTermsOfUseAcceptanceStatus()`

PackageManagerClient.SetTermsOfUseAcceptanceStatus

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageManagerClient.cs)

### Properties
- `BaseUrl` (string, get) — The URL of the package manager website

## PackageManagerExtension (class)

Type PackageManagerExtension

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageManagerExtension.cs)

### Constructors
- `void PackageManagerExtension()` — PackageManagerExtension.PackageManagerExtension

### Methods
#### `void Dispose()`

PackageManagerExtension.Dispose

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageManagerExtension.cs)

#### `void Ready(Dynamo.Extensions.ReadyParams sp)`

PackageManagerExtension.Ready

**Parameters:**
- `sp` (Dynamo.Extensions.ReadyParams)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageManagerExtension.cs)

#### `void Shutdown()`

PackageManagerExtension.Shutdown

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageManagerExtension.cs)

#### `void Startup(Dynamo.Extensions.StartupParams startupParams)`

Validate the package manager url and initialize the PackageManagerClient object

**Parameters:**
- `startupParams` (Dynamo.Extensions.StartupParams)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageManagerExtension.cs)

### Properties
- `Name` (string, get) — PackageManagerExtension.Name
- `PackageLoader` (Dynamo.PackageManager.PackageLoader, get) — Manages loading of packages (property meant solely for tests)
- `PackageManagerClient` (Dynamo.PackageManager.PackageManagerClient, get) — Dynamo Package Manager Instance.
- `RequestedExtensions` (System.Collections.Generic.IEnumerable<Dynamo.Extensions.IExtension>, get) — PackageManagerExtension.RequestedExtensions
- `UniqueId` (string, get) — PackageManagerExtension.UniqueId

### Events
#### `MessageLogged` (`System.Action<Dynamo.Logging.ILogMessage>`)

**Signature:** `public event System.Action<Dynamo.Logging.ILogMessage> MessageLogged`

PackageManagerExtension.MessageLogged

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageManagerExtension.cs)

#### `RequestAddExtension` (`System.Action<Dynamo.Extensions.IExtension>`)

**Signature:** `public event System.Action<Dynamo.Extensions.IExtension> RequestAddExtension`

PackageManagerExtension.RequestAddExtension

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageManagerExtension.cs)

#### `RequestLoadExtension` (`System.Func<string, Dynamo.Extensions.IExtension>`)

**Signature:** `public event System.Func<string, Dynamo.Extensions.IExtension> RequestLoadExtension`

PackageManagerExtension.RequestLoadExtension

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageManagerExtension.cs)

## PackageManagerResult (class)

Type PackageManagerResult

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageManagerResult.cs)

### Constructors
- `void PackageManagerResult(string error, bool success)` — PackageManagerResult.PackageManagerResult

### Methods
#### `Dynamo.PackageManager.PackageManagerResult Failed(string error)`

PackageManagerResult.Failed

**Parameters:**
- `error` (string)

**Returns:** `Dynamo.PackageManager.PackageManagerResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageManagerResult.cs)

#### `Dynamo.PackageManager.PackageManagerResult Succeeded()`

PackageManagerResult.Succeeded

**Returns:** `Dynamo.PackageManager.PackageManagerResult` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageManagerResult.cs)

### Properties
- `Error` (string, get/set) — PackageManagerResult.Error
- `Success` (bool, get/set) — PackageManagerResult.Success

## PackageManagerSearchElement (class)

A search element representing an element from the package manager

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageManagerSearchElement.cs)

### Constructors
- `void PackageManagerSearchElement(Greg.Responses.PackageHeader header)` — The class constructor
- `void PackageManagerSearchElement(Greg.Responses.PackageVersion infectedVersion)` — The class constructor

### Methods
#### `void Upvote()`

PackageManagerSearchElement.Upvote

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageManagerSearchElement.cs)

### Properties
- `Description` (string, get) — Description property
- `Downloads` (int, get) — PackageManagerSearchElement.Downloads
- `EngineVersion` (string, get) — PackageManagerSearchElement.EngineVersion
- `Guid` (System.Guid, get) — Guid property
- `HasUpvote` (bool, get) — Shows if the current user has upvoted this package
- `Header` (Greg.Responses.PackageHeader, get) — Header property
- `Hosts` (System.Collections.Generic.List<string>, get) — Hosts dependencies specified for latest version of particular package
- `HostsString` (string, get) — Hosts dependencies string specified for latest version of particular package Used for package search element UI
- `Id` (string, get) — Id property
- `InfectedPackageCreationDate` (string, get/set) — PackageManagerSearchElement.InfectedPackageCreationDate
- `InfectedPackageName` (string, get/set) — PackageManagerSearchElement.InfectedPackageName
- `InfectedPackageVersion` (string, get/set) — PackageManagerSearchElement.InfectedPackageVersion
- `IsDeprecated` (bool, get) — PackageManagerSearchElement.IsDeprecated
- `Keywords` (string, get/set) — PackageManagerSearchElement.Keywords
- `LatestCompatibleVersion` (Dynamo.PackageManager.VersionInformation, get/set) — The latest compatible package version, or latest version if no compatible version is found
- `LatestVersion` (string, get) — PackageManagerSearchElement.LatestVersion
- `LatestVersionCreated` (string, get) — PackageManagerSearchElement.LatestVersionCreated
- `Maintainers` (string, get) — PackageManagerSearchElement.Maintainers
- `Name` (string, get) — Name property
- `PackageVersions` (System.Collections.Generic.IEnumerable<string>, get) — A list of all package versions
- `RepositoryUrl` (string, get) — PackageManagerSearchElement.RepositoryUrl
- `Searchable` (bool, get) — PackageManagerSearchElement.Searchable
- `SelectedVersion` (Dynamo.PackageManager.VersionInformation, get) — The currently selected version - need to pass onto the PackageDetailView
- `SiteUrl` (string, get) — PackageManagerSearchElement.SiteUrl
- `Type` (string, get) — Type property
- `UsedBy` (int, get) — PackageManagerSearchElement.UsedBy
- `VersionDetails` (System.Collections.Generic.List<Dynamo.PackageManager.VersionInformation>, get/set) — A property for package Versions details (+ compatibility info)
- `Votes` (int, get/set) — PackageManagerSearchElement.Votes
- `Weight` (double, get/set) — Weight property

### Events
#### `UpvoteRequested` (`System.Func<string, bool>`)

**Signature:** `public event System.Func<string, bool> UpvoteRequested`

An event that's invoked when the user has attempted to upvote this package.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageManagerSearchElement.cs)

## PackageUploadHandle (class)

Type PackageUploadHandle

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageUploadHandle.cs)

### Constructors
- `void PackageUploadHandle(Greg.Requests.PackageUploadRequestBody header)` — PackageUploadHandle.PackageUploadHandle
- `void PackageUploadHandle(Greg.Requests.PackageVersionUploadRequestBody header)` — PackageUploadHandle.PackageUploadHandle

### Methods
#### `void Done(Greg.Responses.PackageHeader ph)`

PackageUploadHandle.Done

**Parameters:**
- `ph` (Greg.Responses.PackageHeader)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageUploadHandle.cs)

#### `void Error(string errorString)`

PackageUploadHandle.Error

**Parameters:**
- `errorString` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageUploadHandle.cs)

### Properties
- `CompletedHeader` (Greg.Responses.PackageHeader, get/set) — PackageUploadHandle.CompletedHeader
- `ErrorString` (string, get/set) — PackageUploadHandle.ErrorString
- `Header` (Greg.Requests.PackageUploadRequestBody, get) — PackageUploadHandle.Header
- `Name` (string, get) — PackageUploadHandle.Name
- `UploadState` (Dynamo.PackageManager.PackageUploadHandle.State, get/set) — PackageUploadHandle.UploadState
- `VersionHeader` (Greg.Requests.PackageVersionUploadRequestBody, get) — PackageUploadHandle.VersionHeader
- `VersionName` (string, get) — PackageUploadHandle.VersionName

## VersionInformation (class)

Type VersionInformation

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageManagerSearchElement.cs)

### Constructors
- `void VersionInformation()` — VersionInformation.VersionInformation

### Methods
#### `System.Nullable<bool> GetVersionCompatibility(System.Collections.Generic.List<Dynamo.PackageManager.VersionInformation> versionDetails, string packageVersion)`

A helper method to determine the compatibility of a specific package version from the provided version details.

**Parameters:**
- `versionDetails` (System.Collections.Generic.List<Dynamo.PackageManager.VersionInformation>) — A collection of VersionInformation
- `packageVersion` (string) — The version to look for

**Returns:** `System.Nullable<bool>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoPackages/PackageManagerSearchElement.cs)

### Properties
- `IsCompatible` (System.Nullable<bool>, get/set) — Indicates whether the version is compatible (true, false, or unknown)
- `IsInstalled` (bool, get/set) — Indicates whether this version is currently installed locally.
- `Version` (string, get/set) — The version number as string

