---
name: solibri-solibri-rest-model
description: This skill encodes the solibri 26.4.1 surface of the Solibri.Rest.Model namespace — 15 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ModelInfo, ModelInfoList, IfcGuidList, Error, PingResponse, EncodedBinary, AboutResponse, Status, and 7 more types.
---

# Solibri.Rest.Model

Auto-generated from vendor docs for solibri 26.4.1. 15 types in this namespace.

## AboutResponse (class)

(No description provided in vendor docs for Solibri.Rest.Model.AboutResponse.)

[Vendor docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?schema=AboutResponse)

### Properties
- `Product` (string, get/set) — Name of the running product
- `Version` (string, get/set) — Version of the running product

## CameraState (class)

(No description provided in vendor docs for Solibri.Rest.Model.CameraState.)

[Vendor docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?schema=CameraState)

### Properties
- `Projection` (string, get/set) — Camera State Projection. Valid values are PERSPECTIVE and ORTHOGONAL
- `Location` (Vector3D, get/set) — Location of the camera in 3D space. Unit of location is in Meters.
- `UpDirection` (Vector3D, get/set) — Up direction of the camera as unit vector in 3D space
- `Direction` (Vector3D, get/set) — Direction of the camera as unit vector in 3D space
- `FieldOfView` (double, get/set) — Field of View. Unit is in Radians. This is only valid for Perspective Projection.
- `ViewToWorldScale` (double, get/set) — View to World Scale. This is only valid for Orthogonal Projection.

## CheckingResult (class)

(No description provided in vendor docs for Solibri.Rest.Model.CheckingResult.)

[Vendor docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?schema=CheckingResult)

### Properties
- `Name` (string, get/set) — Name of the Checking result
- `Description` (string, get/set) — Description of checking result.
- `UniqueKey` (string, get/set) — Unique key for checking result.
- `Location` (string, get/set) — Location.
- `Severity` (string, get/set) — Severity of the result.

## CheckingResultList (class)

List of checking results.

**Remarks:** List wrapper (root type: array) — see Items property for element type.

[Vendor docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?schema=CheckingResultList)

### Properties
- `Items` (List<CheckingResult>, get/set) — List of checking results.

## EncodedBinary (class)

(No description provided in vendor docs for Solibri.Rest.Model.EncodedBinary.)

[Vendor docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?schema=EncodedBinary)

### Properties
- `Content` (byte[], get/set) — Base64-encoded contents of a file

## Error (class)

(No description provided in vendor docs for Solibri.Rest.Model.Error.)

[Vendor docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?schema=Error)

### Properties
- `Code` (int, get/set) — (No description provided in vendor docs for Error.code.)
- `Message` (string, get/set) — (No description provided in vendor docs for Error.message.)

## IfcGuidList (class)

A list of IFC GUIDS

**Remarks:** List wrapper (root type: array) — see Items property for element type.

[Vendor docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?schema=IfcGuidList)

### Properties
- `Items` (List<string>, get/set) — A list of IFC GUIDS

## ModelInfo (class)

(No description provided in vendor docs for Solibri.Rest.Model.ModelInfo.)

[Vendor docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?schema=ModelInfo)

### Properties
- `Name` (string, get/set) — Name of the model
- `Uuid` (string, get/set) — Solibri internal unique ID (e.g. 66bbc0ca-4496-11e9-b210-d663bd873d93)
- `Metadata` (Dictionary<string, string>, get/set) — Metata of the model as a map

## ModelInfoList (class)

A list of model infos

**Remarks:** List wrapper (root type: array) — see Items property for element type.

[Vendor docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?schema=ModelInfoList)

### Properties
- `Items` (List<ModelInfo>, get/set) — A list of model infos

## PingResponse (class)

(No description provided in vendor docs for Solibri.Rest.Model.PingResponse.)

[Vendor docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?schema=PingResponse)

### Properties
- `Answer` (string, get/set) — (No description provided in vendor docs for PingResponse.answer.)

## PresentationRequest (class)

(No description provided in vendor docs for Solibri.Rest.Model.PresentationRequest.)

[Vendor docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?schema=PresentationRequest)

### Properties
- `Name` (string, get/set) — Name of the presentation.
- `Prefix` (string, get/set) — Tracking ID prefix of the presentation.

## PresentationResponse (class)

(No description provided in vendor docs for Solibri.Rest.Model.PresentationResponse.)

[Vendor docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?schema=PresentationResponse)

### Properties
- `Id` (string, get/set) — Unique id of the presentation

## SlideRequest (class)

(No description provided in vendor docs for Solibri.Rest.Model.SlideRequest.)

[Vendor docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?schema=SlideRequest)

### Properties
- `Title` (string, get/set) — Title of the slide.
- `PresentationId` (string, get/set) — Unique Id of the presentation to add the current slide.
- `Type` (string, get/set) — Type of the issue.
- `Status` (string, get/set) — Status of the issue.
- `Stage` (string, get/set) — Stage of the issue.
- `Priority` (string, get/set) — Priority of the issue.
- `Description` (string, get/set) — Description of the issue.
- `Responsibilities` (List<string>, get/set) — Assignees responsible for the issue resolution.
- `Labels` (List<string>, get/set) — Labels associated to the issue.
- `Duedate` (string, get/set) — Due date(YYYY-MM-DD) for the issue resolution.
- `Components` (List<string>, get/set) — IFC GUIDs of the components associated to the issue.
- `ModifiedTimeInExternalSystem` (string, get/set) — Time when the slide was modified in some external system.

## Status (class)

(No description provided in vendor docs for Solibri.Rest.Model.Status.)

[Vendor docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?schema=Status)

### Properties
- `Busy` (bool, get/set) — True when server is busy
- `Filename` (string, get/set) — Name of saved file
- `Status` (string, get/set) — Status of saved file. One of: UNSAVED, SAVED, MODIFIED
- `Timestamp` (DateTime, get/set) — Time when model was saved. 0 if model is not saved

## Vector3D (class)

(No description provided in vendor docs for Solibri.Rest.Model.Vector3D.)

[Vendor docs](https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml?schema=Vector3D)

### Properties
- `X` (double, get/set) — X Coordinate
- `Y` (double, get/set) — Y Coordinate
- `Z` (double, get/set) — Z Coordinate

