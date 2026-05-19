---
name: dynamo-dynamoinstalldetective
description: This skill encodes the dynamo 4.1.0 surface of the DynamoInstallDetective namespace — 8 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: IInstalledProduct, IProductLookUp, IProductCollection, InstalledProductLookUp, InstalledAscLookUp, InstalledProducts, DynamoProducts, Utilities.
---

# DynamoInstallDetective

Auto-generated from vendor docs for dynamo 4.1.0. 8 types in this namespace.

## DynamoProducts (class)

Type DynamoProducts

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

### Methods
#### `DynamoInstallDetective.DynamoProducts FindDynamoInstallations(string debugPath, DynamoInstallDetective.IProductLookUp lookUp)`

DynamoProducts.FindDynamoInstallations

**Parameters:**
- `debugPath` (string)
- `lookUp` (DynamoInstallDetective.IProductLookUp)

**Returns:** `DynamoInstallDetective.DynamoProducts` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

#### `string GetDynamoPath(System.Version version, string debugPath)`

DynamoProducts.GetDynamoPath

**Parameters:**
- `version` (System.Version)
- `debugPath` (string)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

#### `void LookUpAndInitProducts(DynamoInstallDetective.IProductLookUp lookUp)`

DynamoProducts.LookUpAndInitProducts

**Parameters:**
- `lookUp` (DynamoInstallDetective.IProductLookUp)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

## IInstalledProduct (interface)

Type IInstalledProduct

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

### Properties
- `InstallLocation` (string, get/set) — IInstalledProduct.InstallLocation
- `ProductName` (string, get/set) — IInstalledProduct.ProductName
- `VersionInfo` (System.Tuple<int, int, int, int>, get) — IInstalledProduct.VersionInfo
- `VersionString` (string, get/set) — IInstalledProduct.VersionString

## IProductCollection (interface)

Type IProductCollection

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

### Methods
#### `DynamoInstallDetective.IInstalledProduct GetLatestProduct()`

IProductCollection.GetLatestProduct

**Returns:** `DynamoInstallDetective.IInstalledProduct` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

#### `void LookUpAndInitProducts(DynamoInstallDetective.IProductLookUp lookUp)`

IProductCollection.LookUpAndInitProducts

**Parameters:**
- `lookUp` (DynamoInstallDetective.IProductLookUp)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

### Properties
- `Products` (System.Collections.Generic.IEnumerable<DynamoInstallDetective.IInstalledProduct>, get) — IProductCollection.Products

## IProductLookUp (interface)

Type IProductLookUp

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

### Methods
#### `bool ExistsAtPath(string installPath)`

IProductLookUp.ExistsAtPath

**Parameters:**
- `installPath` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

#### `string GetCoreFilePathFromInstallation(string installPath)`

IProductLookUp.GetCoreFilePathFromInstallation

**Parameters:**
- `installPath` (string)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

#### `DynamoInstallDetective.IInstalledProduct GetProductFromInstallPath(string path)`

IProductLookUp.GetProductFromInstallPath

**Parameters:**
- `path` (string)

**Returns:** `DynamoInstallDetective.IInstalledProduct` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

#### `DynamoInstallDetective.IInstalledProduct GetProductFromProductCode(string productCode)`

IProductLookUp.GetProductFromProductCode

**Parameters:**
- `productCode` (string)

**Returns:** `DynamoInstallDetective.IInstalledProduct` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

#### `DynamoInstallDetective.IInstalledProduct GetProductFromProductName(string name)`

IProductLookUp.GetProductFromProductName

**Parameters:**
- `name` (string)

**Returns:** `DynamoInstallDetective.IInstalledProduct` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

#### `System.Collections.Generic.IEnumerable<string> GetProductNameList()`

IProductLookUp.GetProductNameList

**Returns:** `System.Collections.Generic.IEnumerable<string>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

#### `System.Tuple<int, int, int, int> GetVersionInfoFromFile(string filePath)`

IProductLookUp.GetVersionInfoFromFile

**Parameters:**
- `filePath` (string)

**Returns:** `System.Tuple<int, int, int, int>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

## InstalledAscLookUp (class)

Type InstalledAscLookUp

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

### Constructors
- `void InstalledAscLookUp(string fileLookup)` — InstalledAscLookUp.InstalledAscLookUp

### Methods
#### `string GetInstallLocationFromProductName(string name)`

InstalledAscLookUp.GetInstallLocationFromProductName

**Parameters:**
- `name` (string)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

#### `System.Collections.Generic.IEnumerable<string> GetProductNameList()`

InstalledAscLookUp.GetProductNameList

**Returns:** `System.Collections.Generic.IEnumerable<string>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

## InstalledProductLookUp (class)

Type InstalledProductLookUp

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

### Constructors
- `void InstalledProductLookUp(string lookUpName, System.Func<string, string> fileLocator)` — InstalledProductLookUp.InstalledProductLookUp
- `void InstalledProductLookUp(string lookUpName, string fileLookup)` — InstalledProductLookUp.InstalledProductLookUp

### Methods
#### `bool ExistsAtPath(string basePath)`

InstalledProductLookUp.ExistsAtPath

**Parameters:**
- `basePath` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

#### `string GetCoreFilePathFromInstallation(string installPath)`

InstalledProductLookUp.GetCoreFilePathFromInstallation

**Parameters:**
- `installPath` (string)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

#### `string GetInstallLocationFromProductCode(string productCode, ref string productName)`

InstalledProductLookUp.GetInstallLocationFromProductCode

**Parameters:**
- `productCode` (string)
- `productName` (ref string)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

#### `string GetInstallLocationFromProductName(string name)`

InstalledProductLookUp.GetInstallLocationFromProductName

**Parameters:**
- `name` (string)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

#### `DynamoInstallDetective.IInstalledProduct GetProductFromInstallPath(string path)`

InstalledProductLookUp.GetProductFromInstallPath

**Parameters:**
- `path` (string)

**Returns:** `DynamoInstallDetective.IInstalledProduct` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

#### `DynamoInstallDetective.IInstalledProduct GetProductFromProductCode(string productCode)`

InstalledProductLookUp.GetProductFromProductCode

**Parameters:**
- `productCode` (string)

**Returns:** `DynamoInstallDetective.IInstalledProduct` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

#### `DynamoInstallDetective.IInstalledProduct GetProductFromProductName(string name)`

InstalledProductLookUp.GetProductFromProductName

**Parameters:**
- `name` (string)

**Returns:** `DynamoInstallDetective.IInstalledProduct` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

#### `System.Collections.Generic.IEnumerable<string> GetProductNameList()`

InstalledProductLookUp.GetProductNameList

**Returns:** `System.Collections.Generic.IEnumerable<string>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

#### `System.Tuple<int, int, int, int> GetVersionInfoFromFile(string filePath)`

InstalledProductLookUp.GetVersionInfoFromFile

**Parameters:**
- `filePath` (string)

**Returns:** `System.Tuple<int, int, int, int>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

### Properties
- `ProductLookUpName` (string, get) — InstalledProductLookUp.ProductLookUpName

## InstalledProducts (class)

Type InstalledProducts

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

### Constructors
- `void InstalledProducts()` — InstalledProducts.InstalledProducts

### Methods
#### `DynamoInstallDetective.IInstalledProduct GetLatestProduct()`

InstalledProducts.GetLatestProduct

**Returns:** `DynamoInstallDetective.IInstalledProduct` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

#### `void LookUpAndInitProducts(DynamoInstallDetective.IProductLookUp lookUp)`

InstalledProducts.LookUpAndInitProducts

**Parameters:**
- `lookUp` (DynamoInstallDetective.IProductLookUp)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/ProductLookUp.cs)

### Properties
- `Products` (System.Collections.Generic.IEnumerable<DynamoInstallDetective.IInstalledProduct>, get) — InstalledProducts.Products

## Utilities (static-class)

Type Utilities

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/Utilities.cs)

### Methods
#### `System.Collections.IEnumerable FindDynamoInstallations(string additionalDynamoPath)`

Utilities.FindDynamoInstallations

**Parameters:**
- `additionalDynamoPath` (string)

**Returns:** `System.Collections.IEnumerable` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/Utilities.cs)

#### `System.Collections.IEnumerable FindMultipleProductInstallations(System.Collections.Generic.List<string> productSearchPatterns, string fileSearchPattern)`

Utilities.FindMultipleProductInstallations

**Parameters:**
- `productSearchPatterns` (System.Collections.Generic.List<string>)
- `fileSearchPattern` (string)

**Returns:** `System.Collections.IEnumerable` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/Utilities.cs)

#### `System.Collections.IEnumerable FindProductInstallations(string productSearchPattern, string fileSearchPattern)`

Utilities.FindProductInstallations

**Parameters:**
- `productSearchPattern` (string)
- `fileSearchPattern` (string)

**Returns:** `System.Collections.IEnumerable` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/Utilities.cs)

#### `System.Collections.IEnumerable LocateDynamoInstallations(string additionalDynamoPath, System.Func<string, string> fileLocator)`

Utilities.LocateDynamoInstallations

**Parameters:**
- `additionalDynamoPath` (string)
- `fileLocator` (System.Func<string, string>)

**Returns:** `System.Collections.IEnumerable` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/Tools/DynamoInstallDetective/Utilities.cs)

