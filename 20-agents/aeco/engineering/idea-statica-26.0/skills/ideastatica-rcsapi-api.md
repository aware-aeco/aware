---
name: idea-statica-ideastatica-rcsapi-api
description: This skill encodes the idea-statica 26.0 surface of the IdeaStatiCa.RcsApi.Api namespace — 8 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: CalculationApi, ClientApi, CrossSectionApi, DesignMemberApi, InternalForcesApi, MaterialApi, ProjectApi, SectionApi.
---

# IdeaStatiCa.RcsApi.Api

Auto-generated from vendor docs for idea-statica 26.0. 8 types in this namespace.

## CalculationApi (class)

RCS Rest API 1.0 — Calculation endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/1/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/CalculationApi.md)

### Methods
#### `Task<List<RcsSectionResultOverview>> CalculateAsync(Guid projectId, RcsCalculationParameters rcsCalculationParameters)`

Calculate RCS project

**Parameters:**
- `projectId` (Guid) — Project Id
- `rcsCalculationParameters` (RcsCalculationParameters) — Calculation parameters

**Returns:** `List<RcsSectionResultOverview>` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/CalculationApi.md)

#### `Task<string> GetRawResultsAsync(Guid projectId, RcsResultParameters rcsResultParameters)`

Get calculated results in XML format

**Parameters:**
- `projectId` (Guid)
- `rcsResultParameters` (RcsResultParameters)

**Returns:** `string` — **string**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/CalculationApi.md)

#### `Task<List<RcsSectionResultDetailed>> GetResultsAsync(Guid projectId, RcsResultParameters rcsResultParameters)`

Get calculated results

**Parameters:**
- `projectId` (Guid) — Project Id
- `rcsResultParameters` (RcsResultParameters) — Calculation parameters

**Returns:** `List<RcsSectionResultDetailed>` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/CalculationApi.md)

## ClientApi (class)

RCS Rest API 1.0 — Client endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/1/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/ClientApi.md)

### Methods
#### `Task<string> GetVersionAsync()`

Get the IdeaStatica version

**Returns:** `string` — **string**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/ClientApi.md)

## CrossSectionApi (class)

RCS Rest API 1.0 — CrossSection endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/1/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/CrossSectionApi.md)

### Methods
#### `Task<RcsReinforcedCrossSection> AddReinforcedCrossSectionAsync(Guid projectId, ReinforcedCrossSectionData reinforcedCrossSectionData)`

Add a new reinforced cross-section to the project from embedded geometry

**Parameters:**
- `projectId` (Guid) — Project ID
- `reinforcedCrossSectionData` (ReinforcedCrossSectionData) — Reinforced cross-section with embedded geometry. Materials can be referenced by name (must exist in project) or by ID.

**Returns:** `RcsReinforcedCrossSection` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/CrossSectionApi.md)

#### `Task<ReinforcedCrossSection> GetReinforcedCrossSectionDataAsync(Guid projectId, int reinforcedCssSectionId)`

Get reinforced cross-section with full geometry (IOM format)

**Parameters:**
- `projectId` (Guid) — Project ID
- `reinforcedCssSectionId` (int) — Reinforced cross-section ID

**Returns:** `ReinforcedCrossSection` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/CrossSectionApi.md)

#### `Task<RcsReinforcedCrossSection> ImportReinforcedCrossSectionAsync(Guid projectId, RcsReinforcedCrossSectionImportData rcsReinforcedCrossSectionImportData)`

Import reinforced cross-section from template

**Parameters:**
- `projectId` (Guid) — Project ID
- `rcsReinforcedCrossSectionImportData` (RcsReinforcedCrossSectionImportData) — Import data containing settings and template

**Returns:** `RcsReinforcedCrossSection` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/CrossSectionApi.md)

#### `Task<List<RcsReinforcedCrossSection>> ReinforcedCrossSectionsAsync(Guid projectId)`

Get reinforced cross sections

**Parameters:**
- `projectId` (Guid) — Project ID

**Returns:** `List<RcsReinforcedCrossSection>` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/CrossSectionApi.md)

## DesignMemberApi (class)

RCS Rest API 1.0 — DesignMember endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/1/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/DesignMemberApi.md)

### Methods
#### `Task<List<RcsCheckMember>> MembersAsync(Guid projectId)`

Get members

**Parameters:**
- `projectId` (Guid)

**Returns:** `List<RcsCheckMember>` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/DesignMemberApi.md)

## InternalForcesApi (class)

RCS Rest API 1.0 — InternalForces endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/1/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/InternalForcesApi.md)

### Methods
#### `Task<string> GetSectionLoadingAsync(Guid projectId, int sectionId)`

Get section loading

**Parameters:**
- `projectId` (Guid)
- `sectionId` (int)

**Returns:** `string` — **string**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/InternalForcesApi.md)

#### `Task<string> SetSectionLoadingAsync(Guid projectId, int sectionId, RcsSectionLoading rcsSectionLoading)`

Set section loading

**Parameters:**
- `projectId` (Guid)
- `sectionId` (int)
- `rcsSectionLoading` (RcsSectionLoading)

**Returns:** `string` — **string**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/InternalForcesApi.md)

## MaterialApi (class)

RCS Rest API 1.0 — Material endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/1/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/MaterialApi.md)

### Methods
#### `Task AddConcreteMaterialAsync(Guid projectId, RcsMprlElement rcsMprlElement)`

Add a new material to the project. The material type is determined by the endpoint path.

**Parameters:**
- `projectId` (Guid) — Project ID
- `rcsMprlElement` (RcsMprlElement) — Material data to add (MPRL name)

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/MaterialApi.md)

#### `Task AddPrestressMaterialAsync(Guid projectId, RcsMprlElement rcsMprlElement)`

Add a new material to the project. The material type is determined by the endpoint path.

**Parameters:**
- `projectId` (Guid) — Project ID
- `rcsMprlElement` (RcsMprlElement) — Material data to add (MPRL name)

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/MaterialApi.md)

#### `Task AddReinforcementMaterialAsync(Guid projectId, RcsMprlElement rcsMprlElement)`

Add a new material to the project. The material type is determined by the endpoint path.

**Parameters:**
- `projectId` (Guid) — Project ID
- `rcsMprlElement` (RcsMprlElement) — Material data to add (MPRL name)

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/MaterialApi.md)

#### `Task<List<object>> GetAllMaterialsAsync(Guid projectId)`

Get materials from the project. Use specific path to filter by material type.

**Parameters:**
- `projectId` (Guid) — Project ID

**Returns:** `List<object>` — **List**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/MaterialApi.md)

#### `Task<List<object>> GetConcreteMaterialsAsync(Guid projectId)`

Get materials from the project. Use specific path to filter by material type.

**Parameters:**
- `projectId` (Guid) — Project ID

**Returns:** `List<object>` — **List**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/MaterialApi.md)

#### `Task<List<object>> GetPrestressMaterialsAsync(Guid projectId)`

Get materials from the project. Use specific path to filter by material type.

**Parameters:**
- `projectId` (Guid) — Project ID

**Returns:** `List<object>` — **List**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/MaterialApi.md)

#### `Task<List<object>> GetReinforcementMaterialsAsync(Guid projectId)`

Get materials from the project. Use specific path to filter by material type.

**Parameters:**
- `projectId` (Guid) — Project ID

**Returns:** `List<object>` — **List**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/MaterialApi.md)

## ProjectApi (class)

RCS Rest API 1.0 — Project endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/1/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/ProjectApi.md)

### Methods
#### `Task<string> CloseProjectAsync(Guid projectId)`

PUT /api/1/projects/{projectId}/close

**Parameters:**
- `projectId` (Guid)

**Returns:** `string` — **string**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/ProjectApi.md)

#### `Task DownloadProjectAsync(Guid projectId)`

Download the actual rcs project from the service. It includes all changes which were made by previous API calls.

**Parameters:**
- `projectId` (Guid) — The unique identifier of the opened project in the ConnectionRestApi service

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/ProjectApi.md)

#### `Task<RcsProject> GetActiveProjectAsync()`

GET /api/1/projects/active-project

**Returns:** `RcsProject` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/ProjectApi.md)

#### `Task<string> GetCodeSettingsAsync(Guid projectId)`

GET /api/1/projects/{projectId}/code-settings

**Parameters:**
- `projectId` (Guid)

**Returns:** `string` — **string**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/ProjectApi.md)

#### `Task<string> GetCodeSettingsJsonAsync(Guid projectId)`

GET /api/1/projects/{projectId}/code-settings-json

**Parameters:**
- `projectId` (Guid)

**Returns:** `string` — **string**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/ProjectApi.md)

#### `Task<Guid> ImportIOMAsync()`

POST /api/1/projects/import-iom

**Returns:** `Guid` — **Guid**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/ProjectApi.md)

#### `Task<RcsProject> ImportIOMFileAsync(ImportIOMFile_request importIOMFile_request)`

POST /api/1/projects/import-iom-file

**Parameters:**
- `importIOMFile_request` (ImportIOMFile_request)

**Returns:** `RcsProject` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/ProjectApi.md)

#### `Task<Guid> OpenAsync()`

POST /api/1/projects/open

**Returns:** `Guid` — **Guid**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/ProjectApi.md)

#### `Task<RcsProject> OpenProjectAsync(OpenProject_request openProject_request)`

Open Rcs project from rcsFile

**Parameters:**
- `openProject_request` (OpenProject_request)

**Returns:** `RcsProject` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/ProjectApi.md)

#### `Task<bool> UpdateCodeSettingsAsync(Guid projectId, List<RcsSetting> requestBody)`

PUT /api/1/projects/{projectId}/code-settings

**Parameters:**
- `projectId` (Guid)
- `requestBody` (List<RcsSetting>)

**Returns:** `bool` — **bool**

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/ProjectApi.md)

## SectionApi (class)

RCS Rest API 1.0 — Section endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /api/1/. Source: https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/api/openapi.yaml

[Vendor docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/SectionApi.md)

### Methods
#### `Task<List<RcsSection>> SectionsAsync(Guid projectId)`

Get sections

**Parameters:**
- `projectId` (Guid)

**Returns:** `List<RcsSection>` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/SectionApi.md)

#### `Task<RcsSection> UpdateSectionAsync(Guid projectId, RcsSection rcsSection)`

Update a section in the RCS project

**Parameters:**
- `projectId` (Guid) — Id of the project in cache
- `rcsSection` (RcsSection) — A new reinforced section data. The value defines the section in project to update

**Returns:** `RcsSection` — OK

[Docs](https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/docs/SectionApi.md)

