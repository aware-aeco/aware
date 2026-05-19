---
name: solibri-solibri-rest-api
description: This skill encodes the solibri 26.4.1 surface of the Solibri.Rest.Api namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: paths.
---

# Solibri.Rest.Api

Auto-generated from vendor docs for solibri 26.4.1. 1 types in this namespace.

## paths (class)

Solibri REST API — paths endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /shutdown. Source: https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml

[Vendor docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

### Methods
#### `Task<AboutResponse> aboutAsync()`

Get information about running product

**Returns:** `AboutResponse` — OK

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task<PresentationResponse> addPresentationAsync(PresentationRequest presentationRequest)`

Adds a new presentation

**Remarks:** Adds a new presentation to the current project.

**Parameters:**
- `presentationRequest` (PresentationRequest) — Presentation details

**Returns:** `PresentationResponse` — Presentation added to the current project

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task addSlideAsync(SlideRequest slideRequest)`

Adds a new slide to the presentation

**Remarks:** Adds a new slide to the presentaion identified by presentationId.

**Parameters:**
- `slideRequest` (SlideRequest) — Slide details

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task closeSMCProjectAsync(bool force)`

Close a Solibri project

**Parameters:**
- `force` (bool) — Forceful close or not

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task deleteComponentsAsync(IfcGuidList ifcGuidList)`

Delete components matching the given IFC guids

**Parameters:**
- `ifcGuidList` (IfcGuidList) — List of IFC GUIDs to be deleted

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task<EncodedBinary> getBCFAsync()`

Access to all, selected or marked issues based on the scope. If no scope is set the scope is all issues. Returns content of a BCF file.

**Returns:** `EncodedBinary` — BFC file content

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task getCameraStateAsync()`

Get Camera State

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task<IfcGuidList> getComponentGuidsAsync()`

Get a list of IFC Global Unique ID (Guid) of the components

**Returns:** `IfcGuidList` — List of IFC GUIDs of the components in the model.

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task<string> getInformationAsync()`

Get the information for given component.

**Returns:** `string` — Information of the component.

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task getMetadataAsync()`

Get metadata value of specified key

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task<ModelInfoList> getModelsAsync()`

Get current models

**Returns:** `ModelInfoList` — Information of all models currently open

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task<string> getParametricInformationAsync(Dictionary<string, string> dictionary<string, string>)`

Get the parametric information of the component

**Parameters:**
- `dictionary<string, string>` (Dictionary<string, string>) — Parameter values as JSON object where parameter values are in String format.

**Returns:** `string` — Parametric information of the component

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task<IfcGuidList> getSelectionAsync()`

Get IFC guids of the selected components

**Returns:** `IfcGuidList` — List of IFC GUIDs of the components that are selected

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task<Status> getStatusAsync()`

Get server status information

**Returns:** `Status` — Information about the server status

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task<ModelInfo> openIFCModelAsync(string name, Stream stream)`

Open IFC Model

**Parameters:**
- `name` (string) — Name of the model
- `stream` (Stream)

**Returns:** `ModelInfo` — Information of the model that was opened

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task<ModelInfo> openSMCProjectAsync(string name, Stream stream)`

Open a Solibri project

**Parameters:**
- `name` (string) — Name of the project
- `stream` (Stream)

**Returns:** `ModelInfo` — Information of the project that was opened

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task<ModelInfo> partialUpdateIFCModelAsync(Stream stream)`

Partial Update IFC Model

**Parameters:**
- `stream` (Stream)

**Returns:** `ModelInfo` — Information of the model that was partially updated

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task<PingResponse> pingAsync()`

GET /ping

**Returns:** `PingResponse` — OK

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task<CheckingResultList> runCheckingAsync(bool checkSelected)`

Runs checking and returns checking results

**Parameters:**
- `checkSelected` (bool) — Run in seected mode or not

**Returns:** `CheckingResultList` — Checking results

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task<ModelInfo> saveSMCProjectAsync(string destination)`

Save a Solibri project

**Parameters:**
- `destination` (string) — The target filepath for the project

**Returns:** `ModelInfo` — Information of the project that was saved.

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task setCameraStateAsync(CameraState cameraState)`

Sets Camera State

**Parameters:**
- `cameraState` (CameraState) — Camera State

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task setMetadataAsync(string value)`

Set metadata value for specified key

**Parameters:**
- `value` (string) — Value of the metadata

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task setSelectionAsync(IfcGuidList ifcGuidList)`

Select components matching the given IFC guids

**Parameters:**
- `ifcGuidList` (IfcGuidList) — List of IFC GUIDs of the components to be selected

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task showInfoAsync()`

Show component's info in the Info view of the user interface.

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task shutdownAsync(bool force)`

Shutdown the application

**Parameters:**
- `force` (bool) — Forceful shutdown or not

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

#### `Task<ModelInfo> updateIFCModelAsync(Stream stream)`

Update IFC Model

**Parameters:**
- `stream` (Stream)

**Returns:** `ModelInfo` —  Information of the model that was updated

[Docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?api=paths)

