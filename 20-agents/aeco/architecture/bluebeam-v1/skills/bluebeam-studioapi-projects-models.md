---
name: bluebeam-bluebeam-studioapi-projects-models
description: This skill encodes the bluebeam v1 surface of the Bluebeam.StudioApi.Projects.Models namespace — 17 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: UpdateProjectRequest, InviteProjectRequest, CreateProjectRequest, UpdateFolderRequest, CreateProjectFolderRequest, CreateProjectFileRequest, CheckoutToSessionFileRequest, ConfirmCheckinFileRequest, and 9 more types.
---

# Bluebeam.StudioApi.Projects.Models

Auto-generated from vendor docs for bluebeam v1. 17 types in this namespace.

## CheckoutToSessionFileRequest (class)

Request body DTO for the corresponding `CheckoutToSessionFile` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `SessionId` (string, get/set) — Request-body field `SessionId` (type inferred from example).

## ConfirmCheckinFileRequest (class)

Request body DTO for the corresponding `ConfirmCheckinFile` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `CRC` (string, get/set) — Request-body field `CRC` (type inferred from example).
- `Comment` (string, get/set) — Request-body field `Comment` (type inferred from example).
- `Size` (int, get/set) — Request-body field `Size` (type inferred from example).

## CopyFileRequest (class)

Request body DTO for the corresponding `CopyFile` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `Name` (string, get/set) — Request-body field `Name` (type inferred from example).
- `ParentFolderId` (int, get/set) — Request-body field `ParentFolderId` (type inferred from example).

## CreateProjectFileRequest (class)

Request body DTO for the corresponding `CreateProjectFile` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `CRC` (string, get/set) — Request-body field `CRC` (type inferred from example).
- `Name` (string, get/set) — Request-body field `Name` (type inferred from example).
- `ParentFolderId` (int, get/set) — Request-body field `ParentFolderId` (type inferred from example).
- `Size` (int, get/set) — Request-body field `Size` (type inferred from example).

## CreateProjectFolderRequest (class)

Request body DTO for the corresponding `CreateProjectFolder` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `Comment` (string, get/set) — Request-body field `Comment` (type inferred from example).
- `Name` (string, get/set) — Request-body field `Name` (type inferred from example).
- `ParentFolderId` (int, get/set) — Request-body field `ParentFolderId` (type inferred from example).

## CreateProjectRequest (class)

Request body DTO for the corresponding `CreateProject` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `Name` (string, get/set) — Request-body field `Name` (type inferred from example).
- `Notification` (bool, get/set) — Request-body field `Notification` (type inferred from example).
- `Restricted` (bool, get/set) — Request-body field `Restricted` (type inferred from example).

## CreateProjectSharedLinkRequest (class)

Request body DTO for the corresponding `CreateProjectSharedLink` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `Expires` (string, get/set) — Request-body field `Expires` (type inferred from example).
- `Flatten` (bool, get/set) — Request-body field `Flatten` (type inferred from example).
- `Password` (string, get/set) — Request-body field `Password` (type inferred from example).
- `PasswordProtected` (bool, get/set) — Request-body field `PasswordProtected` (type inferred from example).
- `ProjectFileId` (int, get/set) — Request-body field `ProjectFileId` (type inferred from example).

## CreateProjectUserRequest (class)

Request body DTO for the corresponding `CreateProjectUser` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `Email` (string, get/set) — Request-body field `Email` (type inferred from example).
- `Message` (string, get/set) — Request-body field `Message` (type inferred from example).
- `SendEmail` (bool, get/set) — Request-body field `SendEmail` (type inferred from example).

## InviteProjectRequest (class)

Request body DTO for the corresponding `InviteProject` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `Email` (string, get/set) — Request-body field `Email` (type inferred from example).
- `Message` (string, get/set) — Request-body field `Message` (type inferred from example).

## UpdateFileRequest (class)

Request body DTO for the corresponding `UpdateFile` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `Name` (string, get/set) — Request-body field `Name` (type inferred from example).
- `ParentFolderId` (int, get/set) — Request-body field `ParentFolderId` (type inferred from example).

## UpdateFolderPermissionsRequest (class)

Request body DTO for the corresponding `UpdateFolderPermissions` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `Permission` (string, get/set) — Request-body field `Permission` (type inferred from example).
- `UserId` (int, get/set) — Request-body field `UserId` (type inferred from example).

## UpdateFolderRequest (class)

Request body DTO for the corresponding `UpdateFolder` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `Name` (string, get/set) — Request-body field `Name` (type inferred from example).
- `ParentFolderId` (int, get/set) — Request-body field `ParentFolderId` (type inferred from example).

## UpdateProjectPermissionsRequest (class)

Request body DTO for the corresponding `UpdateProjectPermissions` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `Allow` (string, get/set) — Request-body field `Allow` (type inferred from example).
- `Type` (string, get/set) — Request-body field `Type` (type inferred from example).

## UpdateProjectRequest (class)

Request body DTO for the corresponding `UpdateProject` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `Name` (string, get/set) — Request-body field `Name` (type inferred from example).
- `Notification` (bool, get/set) — Request-body field `Notification` (type inferred from example).
- `OwnerEmailOrId` (string, get/set) — Request-body field `OwnerEmailOrId` (type inferred from example).
- `Restricted` (bool, get/set) — Request-body field `Restricted` (type inferred from example).

## UpdateSharedLinkRequest (class)

Request body DTO for the corresponding `UpdateSharedLink` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `Enabled` (bool, get/set) — Request-body field `Enabled` (type inferred from example).
- `Hidden` (bool, get/set) — Request-body field `Hidden` (type inferred from example).

## UpdateUserPermissionsRequest (class)

Request body DTO for the corresponding `UpdateUserPermissions` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `Allow` (string, get/set) — Request-body field `Allow` (type inferred from example).
- `Type` (string, get/set) — Request-body field `Type` (type inferred from example).

## UpdateUserRequest (class)

Request body DTO for the corresponding `UpdateUser` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `RestrictedStatus` (string, get/set) — Request-body field `RestrictedStatus` (type inferred from example).

