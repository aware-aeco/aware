---
name: dynamo-dynamoutilities
description: This skill encodes the dynamo 4.1.1 surface of the DynamoUtilities namespace — 6 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: CertificateVerification, PathHelper, SmartObservableCollection<T>, StringUtilities, TypeSwitch, XmlHelper.
---

# DynamoUtilities

Auto-generated from vendor docs for dynamo 4.1.1. 6 types in this namespace.

## CertificateVerification (class)

Type CertificateVerification

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/CertificateVerification.cs)

### Constructors
- `void CertificateVerification()` — CertificateVerification.CertificateVerification

### Methods
#### `bool CheckAssemblyForValidCertificate(string assemblyPath)`

Check if a .NET assembly can be loaded and has a valid certificate

**Parameters:**
- `assemblyPath` (string) — Path of the assembly file

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/CertificateVerification.cs)

## PathHelper (class)

Type PathHelper

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/PathHelper.cs)

### Constructors
- `void PathHelper()` — PathHelper.PathHelper

### Methods
#### `System.Exception CreateFolderIfNotExist(string folderPath)`

PathHelper.CreateFolderIfNotExist

**Parameters:**
- `folderPath` (string)

**Returns:** `System.Exception` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/PathHelper.cs)

#### `void ExtractAndSaveEmbeddedFont(string resourcePath, string outputPath, string outputFileName, System.Reflection.Assembly assembly)`

This function will extract the embedded font file and save it to a specified directory

**Parameters:**
- `resourcePath` (string) — The location of the font resource
- `outputPath` (string) — The temporary path to save the resource to
- `outputFileName` (string) — The name of the temporary resource file
- `assembly` (System.Reflection.Assembly) — The assembly containing the resource

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/PathHelper.cs)

#### `void FileInfoAtPath(string path, ref bool fileExists, ref string size)`

Checks if the file exists at the specified path and computes size.

**Parameters:**
- `path` (string) — File path
- `fileExists` (ref bool) — 
- `size` (ref string) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/PathHelper.cs)

#### `string GetDateModified(string filePath)`

Utility method to get the last time a file has been modified

**Parameters:**
- `filePath` (string) — 

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/PathHelper.cs)

#### `string GetFileSize(string path)`

Computes the file size from the path.

**Parameters:**
- `path` (string) — File path

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/PathHelper.cs)

#### `string GetScreenCaptureNameFromPath(string filePath, bool isTimeStampIncluded)`

This is a utility method for generating a default name to the snapshot image.

**Parameters:**
- `filePath` (string) — File path
- `isTimeStampIncluded` (bool) — Is timestamp included in file path

**Returns:** `string` — Returns a default name(along with the timestamp) for the workspace image

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/PathHelper.cs)

#### `string GetServiceBackendAddress(object o, string serviceKey)`

Returns the path configured for the requested service to retrieve URL resources as defined inside the config file

**Parameters:**
- `o` (object) — The "this" object from where the function is being called from.
- `serviceKey` (string) — Service or feature for which the address is being requested. It should match the key specified in the config file.

**Returns:** `string` — Path that will be used to fetch resources

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/PathHelper.cs)

#### `bool HasWritePermissionOnDir(string folderPath)`

Returns whether current user has write access to the folder path.

**Parameters:**
- `folderPath` (string) — Folder path

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/PathHelper.cs)

#### `bool IsFileNameInValid(string fileName)`

PathHelper.IsFileNameInValid

**Parameters:**
- `fileName` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/PathHelper.cs)

#### `bool IsReadOnlyPath(string filePath)`

Check if user has readonly privilege to the folder path.

**Parameters:**
- `filePath` (string) — File path

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/PathHelper.cs)

#### `bool IsValidPath(string filePath)`

Checks if the file path string is valid and file exist.

**Parameters:**
- `filePath` (string) — File path

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/PathHelper.cs)

#### `string LoadEmbeddedResourceAsString(string resourcePath, System.Reflection.Assembly assembly)`

Loads embedded resources such as HTML and JS files and returns the content as a string.

**Parameters:**
- `resourcePath` (string) — The resource path to return.
- `assembly` (System.Reflection.Assembly) — The assembly containing the resource.

**Returns:** `string` — The embedded resource as a string.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/PathHelper.cs)

#### `string getServiceConfigValues(object o, string serviceKey)`

Returns the path configured for the requested service to retrieve resources value as defined inside the config file

**Parameters:**
- `o` (object) — The "this" object from where the function is being called from.
- `serviceKey` (string) — Service or feature for which the resource is being requested. It should match the key specified in the config file.

**Returns:** `string` — Value related to the key in the config file

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/PathHelper.cs)

#### `bool isFileContentsValidJson(string fileContents, ref System.Exception ex)`

This is a utility method for checking if a given string represents a valid Json document.

**Parameters:**
- `fileContents` (string) — string contents of target json file
- `ex` (ref System.Exception) — 

**Returns:** `bool` — Return true if fileContents is Json, false if file is not Json, exception as out param

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/PathHelper.cs)

#### `bool isValidJson(string path, ref string fileContents, ref System.Exception ex)`

This is a utility method for checking if given path contains valid Json document.

**Parameters:**
- `path` (string) — path to the target json file
- `fileContents` (ref string) — string contents of target json file
- `ex` (ref System.Exception) — 

**Returns:** `bool` — Return true if file is Json, false if file is not Json, exception as out param

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/PathHelper.cs)

#### `bool isValidXML(string path, ref System.Xml.XmlDocument xmlDoc, ref System.Exception ex)`

This is a utility method for checking if given path contains valid XML document.

**Parameters:**
- `path` (string) — path to the target xml file
- `xmlDoc` (ref System.Xml.XmlDocument) — System.Xml.XmlDocument representation of target xml file
- `ex` (ref System.Exception) — 

**Returns:** `bool` — Return true if file is Json, false if file is not Json, exception as out param

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/PathHelper.cs)

## SmartObservableCollection<T> (class)

Wrapper over System.Collections.ObjectModel.ObservableCollection that fires minimal notifications. This class supports batch operations that should defer CollectionChaned notifications.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/SmartObservableCollection.cs)

### Constructors
- `void SmartObservableCollection()` — SmartObservableCollection.SmartObservableCollection
- `void SmartObservableCollection(System.Collections.Generic.IEnumerable<T> collection)` — SmartObservableCollection.SmartObservableCollection
- `void SmartObservableCollection(System.Collections.Generic.List<T> list)` — SmartObservableCollection.SmartObservableCollection

### Methods
#### `void ClearItems()`

SmartObservableCollection.ClearItems

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/SmartObservableCollection.cs)

#### `void OnCollectionChanged(System.Collections.Specialized.NotifyCollectionChangedEventArgs e)`

SmartObservableCollection.OnCollectionChanged

**Parameters:**
- `e` (System.Collections.Specialized.NotifyCollectionChangedEventArgs)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/SmartObservableCollection.cs)

#### `void OnPropertyChanged(System.ComponentModel.PropertyChangedEventArgs e)`

SmartObservableCollection.OnPropertyChanged

**Parameters:**
- `e` (System.ComponentModel.PropertyChangedEventArgs)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/SmartObservableCollection.cs)

## StringUtilities (class)

String utilities

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/StringUtilities.cs)

### Constructors
- `void StringUtilities()` — StringUtilities.StringUtilities

### Methods
#### `string CapitalizeFirstLetter(string word)`

Return the string with the first letter capitalized

**Parameters:**
- `word` (string) — 

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/StringUtilities.cs)

## TypeSwitch (static-class)

Type TypeSwitch

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/TypeSwitch.cs)

### Methods
#### `DynamoUtilities.TypeSwitch.CaseInfo Case<T>(System.Action action)`

TypeSwitch.Case

**Parameters:**
- `action` (System.Action)

**Returns:** `DynamoUtilities.TypeSwitch.CaseInfo` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/TypeSwitch.cs)

#### `DynamoUtilities.TypeSwitch.CaseInfo Case<T>(System.Action<T> action)`

TypeSwitch.Case

**Parameters:**
- `action` (System.Action<T>)

**Returns:** `DynamoUtilities.TypeSwitch.CaseInfo` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/TypeSwitch.cs)

#### `DynamoUtilities.TypeSwitch.CaseInfo Default(System.Action action)`

TypeSwitch.Default

**Parameters:**
- `action` (System.Action)

**Returns:** `DynamoUtilities.TypeSwitch.CaseInfo` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/TypeSwitch.cs)

#### `void Do(object source, DynamoUtilities.TypeSwitch.CaseInfo[] cases)`

TypeSwitch.Do

**Parameters:**
- `source` (object)
- `cases` (DynamoUtilities.TypeSwitch.CaseInfo[])

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/TypeSwitch.cs)

## XmlHelper (class)

Type XmlHelper

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlHelper.cs)

### Constructors
- `void XmlHelper()` — XmlHelper.XmlHelper

### Methods
#### `void AddAttribute(System.Xml.XmlNode parent, string name, string value)`

XmlHelper.AddAttribute

**Parameters:**
- `parent` (System.Xml.XmlNode)
- `name` (string)
- `value` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlHelper.cs)

#### `System.Xml.XmlNode AddNode(System.Xml.XmlNode parent, string name, string value)`

XmlHelper.AddNode

**Parameters:**
- `parent` (System.Xml.XmlNode)
- `name` (string)
- `value` (string)

**Returns:** `System.Xml.XmlNode` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlHelper.cs)

#### `System.Xml.XmlDocument CreateDocument(string rootName)`

XmlHelper.CreateDocument

**Parameters:**
- `rootName` (string)

**Returns:** `System.Xml.XmlDocument` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlHelper.cs)

