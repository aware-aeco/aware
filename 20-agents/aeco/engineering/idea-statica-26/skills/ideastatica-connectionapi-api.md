---
name: idea-statica-ideastatica-connectionapi-api
description: This skill encodes the idea-statica 26.0 surface of the IdeaStatiCa.ConnectionApi.Api namespace — 16 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: CalculationApi, ClientApi, ConnectionApi, ConnectionLibraryApi, ConversionApi, ExportApi, LoadEffectApi, MaterialApi, and 8 more types.
---

# IdeaStatiCa.ConnectionApi.Api

Auto-generated from vendor docs for idea-statica 26.0. 16 types in this namespace.

## CalculationApi (class)

Connection Rest API 3.0 — Calculation endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/3/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/CalculationApi.md)

### Methods
#### `Task<List<ConResultSummary>> CalculateAsync(Guid projectId, List<int> requestBody)`

Runs CBFEM calculation and returns the summary of the results.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.
- `requestBody` (List<int>) — List of connection IDs to calculate.

**Returns:** `List<ConResultSummary>` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/CalculationApi.md)

#### `Task<List<string>> GetRawJsonResultsAsync(Guid projectId, List<int> requestBody)`

Gets JSON string which represents raw CBFEM results (an instance of CheckResultsData).

**Remarks:** Bolt identifiers: Bolt keys in the raw results(e.g., in `boltCheckResDataTimbers`) are opaque internalCBFEM solver identifiers. They may start at any number, are not necessarilysequential, and may contain gaps. When a user explodes abolt group or modifies bolt positions, the identifiers may shift. Do not assume bolt keys start at 1 or can be used as array indices.To map bolts to sequential positions, sort the keys numericallyand use the resulting order. `forcesAllLoadCases`dictionary keys: In the per-bolt results within `boltCheckResDataTimbers`, the `forcesAllLoadCases` dictionary is keyedby the load case's internal numeric identifier (as a string).The key is not always `"1"` — it corresponds tothe load case ID. For example, if only load case LE2 is active, the sole key will be `"2"`. API consumers should iterate over availablekeys rather than assuming a specific key value.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.
- `requestBody` (List<int>) — List of connection IDs to calculate.

**Returns:** `List<string>` — **List**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/CalculationApi.md)

#### `Task<List<ConnectionCheckRes>> GetResultsAsync(Guid projectId, List<int> requestBody)`

Gets detailed results of the CBFEM analysis.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.
- `requestBody` (List<int>) — List of connection IDs to calculate.

**Returns:** `List<ConnectionCheckRes>` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/CalculationApi.md)

## ClientApi (class)

Connection Rest API 3.0 — Client endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/3/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ClientApi.md)

### Methods
#### `Task<string> ConnectClientAsync()`

Connects a client to the ConnectionRestApi service and returns a unique identifier for the client.

**Returns:** `string` — **string**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ClientApi.md)

#### `Task<string> GetVersionAsync()`

Gets the IdeaStatica API assembly version.

**Returns:** `string` — **string**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ClientApi.md)

## ConnectionApi (class)

Connection Rest API 3.0 — Connection endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/3/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConnectionApi.md)

### Methods
#### `Task<ConConnection> CopyConnectionAsync(Guid projectId, int connectionId, string name)`

Creates a copy of an existing connection in the project.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.
- `connectionId` (int) — The ID of the source connection to copy.
- `name` (string) — Optional name for the new connection. If null or empty, a unique name is derived from the source connection name.

**Returns:** `ConConnection` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConnectionApi.md)

#### `Task<ConConnection> CreateEmptyConnectionAsync(Guid projectId, string name)`

Adds a new empty connection to the project.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.
- `name` (string) — Optional connection name. If null or empty, a default name `CON{newId}` is assigned.

**Returns:** `ConConnection` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConnectionApi.md)

#### `Task<List<ConConnection>> DeleteConnectionAsync(Guid projectId, int connectionId)`

Deletes a specific connection from the project.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.
- `connectionId` (int) — The ID of the connection to delete.

**Returns:** `List<ConConnection>` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConnectionApi.md)

#### `Task<ConConnection> GetConnectionAsync(Guid projectId, int connectionId)`

Gets data about a specific connection in the project.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.
- `connectionId` (int) — The ID of the requested connection.

**Returns:** `ConConnection` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConnectionApi.md)

#### `Task<string> GetConnectionTopologyAsync(Guid projectId, int connectionId)`

Gets the topology of the connection in JSON format.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.
- `connectionId` (int) — The ID of the connection for which to retrieve the topology.

**Returns:** `string` — **string**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConnectionApi.md)

#### `Task<List<ConConnection>> GetConnectionsAsync(Guid projectId)`

Gets data about all connections in the project.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.

**Returns:** `List<ConConnection>` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConnectionApi.md)

#### `Task<ConProductionCost> GetProductionCostAsync(Guid projectId, int connectionId)`

Gets the production cost of the connection.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.
- `connectionId` (int) — The ID of the requested connection.

**Returns:** `ConProductionCost` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConnectionApi.md)

#### `Task<ConConnection> UpdateConnectionAsync(Guid projectId, int connectionId, ConConnection conConnection)`

Updates data of a specific connection in the project.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.
- `connectionId` (int) — The ID of the connection to be updated.
- `conConnection` (ConConnection) — New connection data to be applied.

**Returns:** `ConConnection` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConnectionApi.md)

## ConnectionLibraryApi (class)

Connection Rest API 3.0 — ConnectionLibrary endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/3/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConnectionLibraryApi.md)

### Methods
#### `Task GetDesignItemPictureAsync(Guid designSetId, Guid designItemId)`

Retrieves the picture associated with the specified design item as a PNG image.

**Remarks:** This method is mapped to API version 2 and produces a PNG image.The image is returned as a file stream result with the filename set to the design item's ID.

**Parameters:**
- `designSetId` (Guid) — The unique identifier of the design set.
- `designItemId` (Guid) — The unique identifier of the design item for which the template is requested.

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConnectionLibraryApi.md)

#### `Task<List<ConDesignSet>> GetDesignSetsAsync()`

Retrieves a list of design sets available for the user.

**Remarks:** This method returns a collection of design sets that are mappedand ready for use. It throws an exception if no design setsare available for the user.

**Returns:** `List<ConDesignSet>` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConnectionLibraryApi.md)

#### `Task<string> GetTemplateAsync(Guid designSetId, Guid designItemId)`

Retrieves the template associated with the specified design set and design item.

**Remarks:** This method is mapped to API version 2 and produces a plain textresponse. It is intended to be used in scenarios where thetemplate of a design item needs to be retrieved for further processing or n display.

**Parameters:**
- `designSetId` (Guid) — The unique identifier of the design set.
- `designItemId` (Guid) — The unique identifier of the design item for which the template is requested.

**Returns:** `string` — **string**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConnectionLibraryApi.md)

#### `Task<List<ConDesignItem>> ProposeAsync(Guid projectId, int connectionId, ConConnectionLibrarySearchParameters conConnectionLibrarySearchParameters)`

Proposes a list of design items for a specified connection within a project.

**Remarks:** This method retrieves the connection model from the specifiedproject and classifies its typology. It then filters andproposes design items based on the connection's typology and design code.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the project containing the connection.
- `connectionId` (int) — The identifier of the connection for which design items are proposed.
- `conConnectionLibrarySearchParameters` (ConConnectionLibrarySearchParameters) — Parameters used to filter and refine the search for proposed connection design items, such as set membership and required connection features.

**Returns:** `List<ConDesignItem>` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConnectionLibraryApi.md)

#### `Task<bool> PublishConnectionAsync(Guid projectId, int connectionId, ConTemplatePublishParam conTemplatePublishParam)`

Publish template to Private or Company set

**Parameters:**
- `projectId` (Guid)
- `connectionId` (int)
- `conTemplatePublishParam` (ConTemplatePublishParam)

**Returns:** `bool` — **bool**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConnectionLibraryApi.md)

## ConversionApi (class)

Connection Rest API 3.0 — Conversion endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/3/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConversionApi.md)

### Methods
#### `Task<string> ChangeCodeAsync(Guid projectId, ConConversionSettings conConversionSettings)`

Changes the design code of the project.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service (only ECEN projects are supported).
- `conConversionSettings` (ConConversionSettings) — Conversion table for materials in the project (pairs 'ECEN MATERIAL' to 'TARGET DESIGN CODE MATERIAL').

**Returns:** `string` — **string**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConversionApi.md)

#### `Task<ConConversionSettings> GetConversionMappingAsync(Guid projectId, CountryCode countryCode)`

Gets default conversion mappings for converting the project to a different design code.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service (only ECEN design code is supported).
- `countryCode` (CountryCode) — The requested design code for the converted project.

**Returns:** `ConConversionSettings` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ConversionApi.md)

## ExportApi (class)

Connection Rest API 3.0 — Export endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/3/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ExportApi.md)

### Methods
#### `Task<string> ExportIFCAsync(Guid projectId, int connectionId)`

Exports the connection to IFC format.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project.
- `connectionId` (int) — The ID of the connection to export.

**Returns:** `string` — **string**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ExportApi.md)

#### `Task<string> ExportIomAsync(Guid projectId, int connectionId, string version)`

Exports the connection to XML which includes the OpenModelContainer (https://github.com/idea-statica/ideastatica-public/blob/main/src/IdeaRS.OpenModel/OpenModelContainer.cs).

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project.
- `connectionId` (int) — The ID of the connection to export.
- `version` (string) — Optional version string for downgrading the IOM model.

**Returns:** `string` — **string**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ExportApi.md)

#### `Task<ConnectionData> ExportIomConnectionDataAsync(Guid projectId, int connectionId)`

Gets the ConnectionData for the specified connection (https://github.com/idea-statica/ideastatica-public/blob/main/src/IdeaRS.OpenModel/Connection/ConnectionData.cs).

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project.
- `connectionId` (int) — The ID of the connection to export.

**Returns:** `ConnectionData` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ExportApi.md)

## LoadEffectApi (class)

Connection Rest API 3.0 — LoadEffect endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/3/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/LoadEffectApi.md)

### Methods
#### `Task<ConLoadEffect> AddLoadEffectAsync(Guid projectId, int connectionId, ConLoadEffect conLoadEffect)`

Adds a new load effect to the connection.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project.
- `connectionId` (int) — The ID of the connection.
- `conLoadEffect` (ConLoadEffect) — The load effect data to add.

**Returns:** `ConLoadEffect` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/LoadEffectApi.md)

#### `Task<int> DeleteLoadEffectAsync(Guid projectId, int connectionId, int loadEffectId)`

Delete load effect loadEffectId

**Parameters:**
- `projectId` (Guid)
- `connectionId` (int)
- `loadEffectId` (int)

**Returns:** `int` — **int**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/LoadEffectApi.md)

#### `Task<ConLoadEffect> GetLoadEffectAsync(Guid projectId, int connectionId, int loadEffectId, bool isPercentage)`

Gets load impulses from the specified load effect.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project.
- `connectionId` (int) — The ID of the connection.
- `loadEffectId` (int) — The ID of the load effect to retrieve.
- `isPercentage` (bool) — Optional flag indicating whether to return percentage-based load effects.

**Returns:** `ConLoadEffect` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/LoadEffectApi.md)

#### `Task<List<ConLoadEffect>> GetLoadEffectsAsync(Guid projectId, int connectionId, bool isPercentage)`

Gets all load effects defined in the specified connection.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project.
- `connectionId` (int) — The ID of the connection.
- `isPercentage` (bool) — Optional flag indicating whether to return percentage-based load effects.

**Returns:** `List<ConLoadEffect>` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/LoadEffectApi.md)

#### `Task<ConLoadSettings> GetLoadSettingsAsync(Guid projectId, int connectionId)`

Get Load settings for connection in project

**Parameters:**
- `projectId` (Guid)
- `connectionId` (int)

**Returns:** `ConLoadSettings` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/LoadEffectApi.md)

#### `Task<ConLoadSettings> SetLoadSettingsAsync(Guid projectId, int connectionId, ConLoadSettings conLoadSettings)`

Set Load settings for connection in project

**Parameters:**
- `projectId` (Guid)
- `connectionId` (int)
- `conLoadSettings` (ConLoadSettings)

**Returns:** `ConLoadSettings` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/LoadEffectApi.md)

#### `Task<ConLoadEffect> UpdateLoadEffectAsync(Guid projectId, int connectionId, ConLoadEffect conLoadEffect)`

Update load impulses in conLoading

**Parameters:**
- `projectId` (Guid)
- `connectionId` (int)
- `conLoadEffect` (ConLoadEffect)

**Returns:** `ConLoadEffect` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/LoadEffectApi.md)

## MaterialApi (class)

Connection Rest API 3.0 — Material endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/3/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/MaterialApi.md)

### Methods
#### `Task AddBoltAssemblyAsync(Guid projectId, ConMprlElement conMprlElement)`

Add bolt assembly to the project

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service
- `conMprlElement` (ConMprlElement) — Definition of a new bolt assemby to be added to the project

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/MaterialApi.md)

#### `Task AddCrossSectionAsync(Guid projectId, ConMprlCrossSection conMprlCrossSection)`

Add cross section to the project

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service
- `conMprlCrossSection` (ConMprlCrossSection) — Definition of a new cross-section to be added to the project

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/MaterialApi.md)

#### `Task AddMaterialBoltGradeAsync(Guid projectId, ConMprlElement conMprlElement)`

Adds a material to the project.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.
- `conMprlElement` (ConMprlElement) — Definition of the new material to be added to the project.

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/MaterialApi.md)

#### `Task AddMaterialConcreteAsync(Guid projectId, ConMprlElement conMprlElement)`

Adds a material to the project.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.
- `conMprlElement` (ConMprlElement) — Definition of the new material to be added to the project.

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/MaterialApi.md)

#### `Task AddMaterialSteelAsync(Guid projectId, ConMprlElement conMprlElement)`

Adds a material to the project.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.
- `conMprlElement` (ConMprlElement) — Definition of the new material to be added to the project.

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/MaterialApi.md)

#### `Task AddMaterialWeldAsync(Guid projectId, ConMprlElement conMprlElement)`

Adds a material to the project.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.
- `conMprlElement` (ConMprlElement) — Definition of the new material to be added to the project.

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/MaterialApi.md)

#### `Task<List<object>> GetAllMaterialsAsync(Guid projectId)`

Gets materials used in the specified project.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.

**Returns:** `List<object>` — **List**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/MaterialApi.md)

#### `Task<List<object>> GetBoltAssembliesAsync(Guid projectId)`

Gets bolt assemblies used in the specified project.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.

**Returns:** `List<object>` — **List**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/MaterialApi.md)

#### `Task<List<object>> GetBoltGradeMaterialsAsync(Guid projectId)`

Gets materials used in the specified project.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.

**Returns:** `List<object>` — **List**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/MaterialApi.md)

#### `Task<List<object>> GetConcreteMaterialsAsync(Guid projectId)`

Gets materials used in the specified project.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.

**Returns:** `List<object>` — **List**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/MaterialApi.md)

#### `Task<List<object>> GetCrossSectionsAsync(Guid projectId)`

Gets cross sections used in the specified project.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.

**Returns:** `List<object>` — **List**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/MaterialApi.md)

#### `Task<List<object>> GetSteelMaterialsAsync(Guid projectId)`

Gets materials used in the specified project.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.

**Returns:** `List<object>` — **List**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/MaterialApi.md)

#### `Task<List<object>> GetWeldingMaterialsAsync(Guid projectId)`

Gets materials used in the specified project.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.

**Returns:** `List<object>` — **List**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/MaterialApi.md)

## MemberApi (class)

Connection Rest API 3.0 — Member endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/3/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/MemberApi.md)

### Methods
#### `Task<ConMember> AddMemberAsync(Guid projectId, int connectionId, ConMember conMember)`

Adds a new member to the connection.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.
- `connectionId` (int) — The ID of the connection to add the member to.
- `conMember` (ConMember) — The member data for the new member.

**Returns:** `ConMember` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/MemberApi.md)

#### `Task<ConMember> GetMemberAsync(Guid projectId, int connectionId, int memberId)`

Gets information about the specified member in the connection.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.
- `connectionId` (int) — The ID of the connection containing the member.
- `memberId` (int) — The ID of the requested member in the connection.

**Returns:** `ConMember` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/MemberApi.md)

#### `Task<List<ConMember>> GetMembersAsync(Guid projectId, int connectionId)`

Gets information about all members in the connection.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.
- `connectionId` (int) — The ID of the connection from which to retrieve members.

**Returns:** `List<ConMember>` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/MemberApi.md)

#### `Task<ConMember> SetBearingMemberAsync(Guid projectId, int connectionId, int memberId)`

Set bearing member for memberIt

**Parameters:**
- `projectId` (Guid)
- `connectionId` (int)
- `memberId` (int)

**Returns:** `ConMember` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/MemberApi.md)

#### `Task<ConMember> UpdateMemberAsync(Guid projectId, int connectionId, ConMember conMember)`

Updates the member in the connection with the provided data.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.
- `connectionId` (int) — The ID of the connection containing the member to update.
- `conMember` (ConMember) — The new member data to apply.

**Returns:** `ConMember` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/MemberApi.md)

## OperationApi (class)

Connection Rest API 3.0 — Operation endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/3/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/OperationApi.md)

### Methods
#### `Task DeleteOperationsAsync(Guid projectId, int connectionId)`

Delete all operations for the connection

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service
- `connectionId` (int) — Id of the connection to be modified

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/OperationApi.md)

#### `Task<ConOperationCommonProperties> GetCommonOperationPropertiesAsync(Guid projectId, int connectionId)`

Gets common operation properties.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project.
- `connectionId` (int) — The ID of the connection.

**Returns:** `ConOperationCommonProperties` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/OperationApi.md)

#### `Task<List<ConOperation>> GetOperationsAsync(Guid projectId, int connectionId)`

Gets the list of operations for the connection.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.
- `connectionId` (int) — The ID of the requested connection.

**Returns:** `List<ConOperation>` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/OperationApi.md)

#### `Task<string> PreDesignWeldsAsync(Guid projectId, int connectionId, ConWeldSizingMethodEnum designType)`

Pre-designs welds in the connection.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project.
- `connectionId` (int) — The ID of the connection.
- `designType` (ConWeldSizingMethodEnum) — The weld sizing method to apply (default is FullStrength).

**Returns:** `string` — **string**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/OperationApi.md)

#### `Task UpdateCommonOperationPropertiesAsync(Guid projectId, int connectionId, ConOperationCommonProperties conOperationCommonProperties)`

Updates common properties for all operations.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project.
- `connectionId` (int) — The ID of the connection.
- `conOperationCommonProperties` (ConOperationCommonProperties) — Common properties to apply (specify material IDs, or keep as null).

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/OperationApi.md)

## ParameterApi (class)

Connection Rest API 3.0 — Parameter endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/3/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ParameterApi.md)

### Methods
#### `Task DeleteParametersAsync(Guid projectId, int connectionId)`

Delete all parameters and parameter model links for the connection connectionId in the project projectId

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service
- `connectionId` (int) — Id of the connection where to delete parameters

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ParameterApi.md)

#### `Task<string> EvaluateExpressionAsync(Guid projectId, int connectionId, string string)`

Evaluate the expression and return the result. For more detailssee documentation about parameters: https://developer.ideastatica.com/docs/api/api_parameters_getting_started.html nor https://developer.ideastatica.com/docs/api/api_parameter_reference_guide.html

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.
- `connectionId` (int) — Id of the connection to use for evaluation expression.
- `string` (string) — Expression to evaluate. See the API documentation for supportedsyntax and examples: https://developer.ideastatica.com/docs/api/api_parameters_getting_started.html

**Returns:** `string` — **string**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ParameterApi.md)

#### `Task<List<IdeaParameter>> GetParametersAsync(Guid projectId, int connectionId, bool includeHidden)`

Gets all parameters defined for the specified project and connection.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.
- `connectionId` (int) — The ID of the connection from which to retrieve parameters.
- `includeHidden` (bool) — If true, includes hidden parameters in the result.

**Returns:** `List<IdeaParameter>` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ParameterApi.md)

#### `Task<ParameterUpdateResponse> UpdateAsync(Guid projectId, int connectionId, List<IdeaParameterUpdate> requestBody)`

Updates parameters for the specified connection in the project with the values provided.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.
- `connectionId` (int) — The ID of the connection to update.
- `requestBody` (List<IdeaParameterUpdate>) — New values of parameters to apply.

**Returns:** `ParameterUpdateResponse` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ParameterApi.md)

## PresentationApi (class)

Connection Rest API 3.0 — Presentation endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/3/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/PresentationApi.md)

### Methods
#### `Task<DrawData> GetDataScene3DAsync(Guid projectId, int connectionId)`

Returns data for Scene3D visualization.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.
- `connectionId` (int) — The ID of the connection to be presented in Scene3D.

**Returns:** `DrawData` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/PresentationApi.md)

#### `Task<string> GetDataScene3DTextAsync(Guid projectId, int connectionId)`

Returns serialized data for Scene3D in JSON format.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project.
- `connectionId` (int) — The ID of the connection to be presented.

**Returns:** `string` — **string**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/PresentationApi.md)

## ProjectApi (class)

Connection Rest API 3.0 — Project endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/3/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ProjectApi.md)

### Methods
#### `Task<string> CloseProjectAsync(Guid projectId)`

Closes the project and releases resources in the service.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the project to be closed.

**Returns:** `string` — **string**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ProjectApi.md)

#### `Task<ConProject> CreateEmptyProjectAsync(ConProjectData conProjectData)`

Creates a new empty IdeaCon project with no connections.

**Parameters:**
- `conProjectData` (ConProjectData) — Optional project metadata. The DesignCode field determines the design code (e.g. \"ECEN\", \"American\", \"AUS\"). Defaults to ECEN if not provided.

**Returns:** `ConProject` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ProjectApi.md)

#### `Task DownloadProjectAsync(Guid projectId)`

Downloads the current IdeaCon project from the service, includingall changes made by previous API calls.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service.

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ProjectApi.md)

#### `Task<List<ConProject>> GetActiveProjectsAsync()`

Gets the list of projects in the service that were opened by the client connected via M:IdeaStatiCa.ConnectionRestApi.Controllers.ClientController.ConnectClient.

**Returns:** `List<ConProject>` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ProjectApi.md)

#### `Task<ConProject> GetProjectDataAsync(Guid projectId)`

Get data of the project.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the requested project

**Returns:** `ConProject` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ProjectApi.md)

#### `Task<ConProject> ImportIOMAsync(ImportIOM_request importIOM_request)`

Create the IDEA Connection project from IOM provided in xml format. nThe parameter 'containerXmlFile' passed in HTTP body represents : [IdeaRS.OpenModel.OpenModelContainer](https://github.com/idea-statica/ideastatica-public/blob/main/src/IdeaRS.OpenModel/OpenModelContainer.cs) nwhich is serialized to XML string by [IdeaRS.OpenModel.Tools.OpenModelContainerToXml](https://github.com/idea-statica/ideastatica-public/blob/main/src/IdeaRS.OpenModel/Tools.cs)

**Parameters:**
- `importIOM_request` (ImportIOM_request)

**Returns:** `ConProject` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ProjectApi.md)

#### `Task<ConProject> OpenProjectAsync(OpenProject_request openProject_request)`

Opens an IdeaCon project from the provided file.

**Parameters:**
- `openProject_request` (OpenProject_request)

**Returns:** `ConProject` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ProjectApi.md)

#### `Task<ConProject> UpdateFromIOMAsync(Guid projectId, UpdateFromIOM_request updateFromIOM_request)`

Update the IDEA Connection project by [IdeaRS.OpenModel.OpenModelContainer](https://github.com/idea-statica/ideastatica-public/blob/main/src/IdeaRS.OpenModel/OpenModelContainer.cs) (model and results). IOM is passed in the body of the request as thexml string. [IdeaRS.OpenModel.Tools.OpenModelContainerToXml](https://github.com/idea-statica/ideastatica-public/blob/main/src/IdeaRS.OpenModel/Tools.cs)should be used to generate the valid xml string

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service to be updated
- `updateFromIOM_request` (UpdateFromIOM_request)

**Returns:** `ConProject` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ProjectApi.md)

#### `Task<ConProject> UpdateProjectDataAsync(Guid projectId, ConProjectData conProjectData)`

Updates ConProjectData of project

**Parameters:**
- `projectId` (Guid)
- `conProjectData` (ConProjectData)

**Returns:** `ConProject` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ProjectApi.md)

## ReportApi (class)

Connection Rest API 3.0 — Report endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/3/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ReportApi.md)

### Methods
#### `Task GenerateHtmlZipAsync(Guid projectId, int connectionId)`

Generates a report for the specified connection in PDF, Word, or HTMLzip format.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project.
- `connectionId` (int) — The ID of the connection to report.

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ReportApi.md)

#### `Task GeneratePdfAsync(Guid projectId, int connectionId)`

Generates a report for the specified connection in PDF, Word, or HTMLzip format.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project.
- `connectionId` (int) — The ID of the connection to report.

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ReportApi.md)

#### `Task GeneratePdfForMutlipleAsync(Guid projectId, List<int> requestBody)`

Generates a report for multiple connections in PDF or Word format.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project.
- `requestBody` (List<int>) — List of connection IDs to include in the report.

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ReportApi.md)

#### `Task GenerateWordAsync(Guid projectId, int connectionId)`

Generates a report for the specified connection in PDF, Word, or HTMLzip format.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project.
- `connectionId` (int) — The ID of the connection to report.

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ReportApi.md)

#### `Task GenerateWordForMultipleAsync(Guid projectId, List<int> requestBody)`

Generates a report for multiple connections in PDF or Word format.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project.
- `requestBody` (List<int>) — List of connection IDs to include in the report.

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/ReportApi.md)

## SettingsApi (class)

Connection Rest API 3.0 — Settings endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/3/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/SettingsApi.md)

### Methods
#### `Task<object> GetSettingsAsync(Guid projectId, string search)`

Gets setting values for the project.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the project.
- `search` (string) — Optional parameter to search for keywords in settings.

**Returns:** `object` — **Dictionary**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/SettingsApi.md)

#### `Task<object> UpdateSettingsAsync(Guid projectId, object body)`

Updates one or multiple setting values in the project.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the project.
- `body` (object) — Dictionary of key-value pairs representing settings to update.

**Returns:** `object` — **Dictionary**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/SettingsApi.md)

## TemplateApi (class)

Connection Rest API 3.0 — Template endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/3/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/TemplateApi.md)

### Methods
#### `Task<ConTemplateApplyResult> ApplyTemplateAsync(Guid projectId, int connectionId, ConTemplateApplyParam conTemplateApplyParam)`

Apply the connection template applyTemplateParam on the connection connectionId in the project projectId

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service
- `connectionId` (int) — Id of the connection where to apply the template
- `conTemplateApplyParam` (ConTemplateApplyParam) — Template to apply

**Returns:** `ConTemplateApplyResult` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/TemplateApi.md)

#### `Task<string> CreateConTemplateAsync(Guid projectId, int connectionId)`

Create a template for the connection connectionId in the project projectId

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service
- `connectionId` (int) — Id of the connection to be converted to a template

**Returns:** `string` — **string**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/TemplateApi.md)

#### `Task DeleteAllAsync(Guid projectId, int connectionId)`

Delete all templates in connection

**Parameters:**
- `projectId` (Guid)
- `connectionId` (int)

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/TemplateApi.md)

#### `Task DeleteAsync(Guid projectId, int connectionId, Guid templateId)`

Delete specific template

**Parameters:**
- `projectId` (Guid)
- `connectionId` (int)
- `templateId` (Guid)

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/TemplateApi.md)

#### `Task ExplodeAllAsync(Guid projectId, int connectionId)`

Explode all templates (delete parameters, keep operations)

**Parameters:**
- `projectId` (Guid)
- `connectionId` (int)

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/TemplateApi.md)

#### `Task ExplodeAsync(Guid projectId, int connectionId, Guid templateId)`

Explode specific template (delete parameters, keep operations)

**Parameters:**
- `projectId` (Guid)
- `connectionId` (int)
- `templateId` (Guid)

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/TemplateApi.md)

#### `Task<TemplateConversions> GetDefaultTemplateMappingAsync(Guid projectId, int connectionId, ConTemplateMappingGetParam conTemplateMappingGetParam)`

Get the default mappings for the application of the connection templatepassed in templateToApply on connectionId in the project projectId

**Remarks:** The result IdeaStatiCa.Api.Connection.Model.TemplateConversionsDefaultmapping to apply the passed template. It can be modified by a user andused for the application of a template M:IdeaStatiCa.ConnectionRestApi.Controllers.TemplateController.ApplyConnectionTemplateAsync(System.Guid,System.Int32,IdeaStatiCa.Api.Connection.Model.ConTemplateApplyParam,System.Threading.CancellationToken)method.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service
- `connectionId` (int) — Id of the connection to get default mapping
- `conTemplateMappingGetParam` (ConTemplateMappingGetParam) — Data of the template to get default mapping

**Returns:** `TemplateConversions` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/TemplateApi.md)

#### `Task<ConOperationCommonProperties> GetTemplateCommonOperationPropertiesAsync(Guid projectId, int connectionId, Guid templateId)`

Get Common properties for specific template

**Parameters:**
- `projectId` (Guid)
- `connectionId` (int)
- `templateId` (Guid)

**Returns:** `ConOperationCommonProperties` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/TemplateApi.md)

#### `Task<ConConnectionTemplate> GetTemplateInConnectionAsync(Guid projectId, int connectionId, int templateInstanceId)`

Retrieves a specific template by its ID for a given connection within a project.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the project containing the connection.
- `connectionId` (int) — The identifier of the connection.
- `templateInstanceId` (int) — The instance identifier of the template to retrieve.

**Returns:** `ConConnectionTemplate` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/TemplateApi.md)

#### `Task<List<ConConnectionTemplate>> GetTemplatesInConnectionAsync(Guid projectId, int connectionId)`

Retrieves a list of templates associated with a specific connection within a project.

**Remarks:** This method fetches the templates applied to a connection withina project. Each template includes details such as its IDwithin the project, template id, members, operations, parameters, and associatedcommon properties.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the project containing the connection.
- `connectionId` (int) — The identifier of the connection for which templates are to be retrieved.

**Returns:** `List<ConConnectionTemplate>` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/TemplateApi.md)

#### `Task<ParameterUpdateResponse> LoadDefaultsAsync(Guid projectId, int connectionId, Guid templateId)`

Load parameter defaults for specific template.

**Parameters:**
- `projectId` (Guid)
- `connectionId` (int)
- `templateId` (Guid)

**Returns:** `ParameterUpdateResponse` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/TemplateApi.md)

#### `Task UpdateTemplateCommonOperationPropertiesAsync(Guid projectId, int connectionId, Guid templateId, ConOperationCommonProperties conOperationCommonProperties)`

Set common properties for specific template

**Parameters:**
- `projectId` (Guid)
- `connectionId` (int)
- `templateId` (Guid)
- `conOperationCommonProperties` (ConOperationCommonProperties)

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/docs/TemplateApi.md)

