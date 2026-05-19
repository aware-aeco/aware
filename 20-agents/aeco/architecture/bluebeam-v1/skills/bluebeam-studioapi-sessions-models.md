---
name: bluebeam-bluebeam-studioapi-sessions-models
description: This skill encodes the bluebeam v1 surface of the Bluebeam.StudioApi.Sessions.Models namespace — 10 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: CreateSessionRequest, UpdateSessionRequest, InviteSessionRequest, CreateSessionFileRequest, CheckinFileRequest, UpdateSessionPermissionsRequest, UpdateUserPermissionsRequest, UpdateUserRequest, and 2 more types.
---

# Bluebeam.StudioApi.Sessions.Models

Auto-generated from vendor docs for bluebeam v1. 10 types in this namespace.

## CheckinFileRequest (class)

Request body DTO for the corresponding `CheckinFile` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `Comment` (string, get/set) — Request-body field `Comment` (type inferred from example).

## CreateSessionActivityRequest (class)

Request body DTO for the corresponding `CreateSessionActivity` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `Message` (string, get/set) — Request-body field `Message` (type inferred from example).

## CreateSessionFileRequest (class)

Request body DTO for the corresponding `CreateSessionFile` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `CRC` (string, get/set) — Request-body field `CRC` (type inferred from example).
- `Name` (string, get/set) — Request-body field `Name` (type inferred from example).
- `Size` (int, get/set) — Request-body field `Size` (type inferred from example).
- `Source` (string, get/set) — Request-body field `Source` (type inferred from example).

## CreateSessionRequest (class)

Request body DTO for the corresponding `CreateSession` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `Name` (string, get/set) — Request-body field `Name` (type inferred from example).
- `Notification` (bool, get/set) — Request-body field `Notification` (type inferred from example).
- `Restricted` (bool, get/set) — Request-body field `Restricted` (type inferred from example).
- `SessionEndDate` (string, get/set) — Request-body field `SessionEndDate` (type inferred from example).

## CreateSessionUserRequest (class)

Request body DTO for the corresponding `CreateSessionUser` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `Email` (string, get/set) — Request-body field `Email` (type inferred from example).
- `Message` (string, get/set) — Request-body field `Message` (type inferred from example).
- `SendEmail` (bool, get/set) — Request-body field `SendEmail` (type inferred from example).

## InviteSessionRequest (class)

Request body DTO for the corresponding `InviteSession` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `Email` (string, get/set) — Request-body field `Email` (type inferred from example).
- `Message` (string, get/set) — Request-body field `Message` (type inferred from example).

## UpdateSessionPermissionsRequest (class)

Request body DTO for the corresponding `UpdateSessionPermissions` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `Allow` (string, get/set) — Request-body field `Allow` (type inferred from example).
- `Type` (string, get/set) — Request-body field `Type` (type inferred from example).

## UpdateSessionRequest (class)

Request body DTO for the corresponding `UpdateSession` operation. Synthesized from the Postman example schema.

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Properties
- `Name` (string, get/set) — Request-body field `Name` (type inferred from example).
- `Notification` (bool, get/set) — Request-body field `Notification` (type inferred from example).
- `OwnerEmailOrId` (string, get/set) — Request-body field `OwnerEmailOrId` (type inferred from example).
- `Restricted` (bool, get/set) — Request-body field `Restricted` (type inferred from example).
- `SessionEndDate` (string, get/set) — Request-body field `SessionEndDate` (type inferred from example).
- `Status` (string, get/set) — Request-body field `Status` (type inferred from example).

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
- `StatusMessage` (string, get/set) — Request-body field `StatusMessage` (type inferred from example).

