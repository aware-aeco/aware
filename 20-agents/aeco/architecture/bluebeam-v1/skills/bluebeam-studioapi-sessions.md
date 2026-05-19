---
name: bluebeam-bluebeam-studioapi-sessions
description: This skill encodes the bluebeam v1 surface of the Bluebeam.StudioApi.Sessions namespace — 5 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: SessionsApi, SessionsFilesApi, SessionsPermissionsApi, SessionsUsersApi, SessionsActivitiesApi.
---

# Bluebeam.StudioApi.Sessions

Auto-generated from vendor docs for bluebeam v1. 5 types in this namespace.

## SessionsActivitiesApi (class)

Bluebeam Studio API — Sessions / Activities endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /publicapi/v1/. Source: https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Methods
#### `Task CreateSessionActivity(Guid sessionId, CreateSessionActivityRequest request)`

Create a Studio Session activity

**Remarks:** HTTP POST `/publicapi/v1/sessions/{sessionId}/activities`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `sessionId` (Guid) — Path parameter `sessionId` from `/publicapi/v1/sessions/{sessionId}/activities`.
- `request` (CreateSessionActivityRequest) — Request body. JSON object with fields: Message.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task GetActivity(Guid sessionId, Guid id)`

Get a Studio Session activity

**Remarks:** HTTP GET `/publicapi/v1/sessions/{sessionId}/activities/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `sessionId` (Guid) — Path parameter `sessionId` from `/publicapi/v1/sessions/{sessionId}/activities/{id}`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/sessions/{sessionId}/activities/{id}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task ListSessionActivities(Guid sessionId)`

Get all Studio Session activities

**Remarks:** HTTP GET `/publicapi/v1/sessions/{sessionId}/activities`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `sessionId` (Guid) — Path parameter `sessionId` from `/publicapi/v1/sessions/{sessionId}/activities`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

## SessionsApi (class)

Bluebeam Studio API — Sessions endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /publicapi/v1/. Source: https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Methods
#### `Task CreateSession(CreateSessionRequest request)`

Create a Studio Session

**Remarks:** HTTP POST `/publicapi/v1/sessions`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `request` (CreateSessionRequest) — Request body. JSON object with fields: Name, Notification, Restricted, SessionEndDate.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task DeleteSession(Guid id)`

Delete a Studio Session

**Remarks:** HTTP DELETE `/publicapi/v1/sessions/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/sessions/{id}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task GetSession(Guid id)`

Get a single Studio Session metadata

**Remarks:** HTTP GET `/publicapi/v1/sessions/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/sessions/{id}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task InviteSession(Guid sessionId, InviteSessionRequest request)`

Invite user to Studio Session by email

**Remarks:** HTTP POST `/publicapi/v1/sessions/{sessionId}/invite`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `sessionId` (Guid) — Path parameter `sessionId` from `/publicapi/v1/sessions/{sessionId}/invite`.
- `request` (InviteSessionRequest) — Request body. JSON object with fields: Email, Message.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task ListSessions()`

Get all Studio Sessions

**Remarks:** HTTP GET `/publicapi/v1/sessions`. Vendor docs page: https://support.bluebeam.com/developer/

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task UpdateSession(Guid id, UpdateSessionRequest request)`

Update a Studio Session

**Remarks:** HTTP PUT `/publicapi/v1/sessions/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/sessions/{id}`.
- `request` (UpdateSessionRequest) — Request body. JSON object with fields: Name, Notification, OwnerEmailOrId, Restricted, SessionEndDate, Status.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

## SessionsFilesApi (class)

Bluebeam Studio API — Sessions / Files endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /publicapi/v1/. Source: https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Methods
#### `Task CheckinFile(Guid sessionId, Guid id, CheckinFileRequest request)`

Update Studio Project copy

**Remarks:** HTTP POST `/publicapi/v1/sessions/{sessionId}/files/{id}/checkin`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `sessionId` (Guid) — Path parameter `sessionId` from `/publicapi/v1/sessions/{sessionId}/files/{id}/checkin`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/sessions/{sessionId}/files/{id}/checkin`.
- `request` (CheckinFileRequest) — Request body. JSON object with fields: Comment.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task ConfirmUploadFile(Guid sessionId, Guid id)`

Confirm Studio Session file upload

**Remarks:** HTTP POST `/publicapi/v1/sessions/{sessionId}/files/{id}/confirm-upload`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `sessionId` (Guid) — Path parameter `sessionId` from `/publicapi/v1/sessions/{sessionId}/files/{id}/confirm-upload`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/sessions/{sessionId}/files/{id}/confirm-upload`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateFileSnapshot(Guid sessionId, Guid id)`

Start Studio Session document snapshot job

**Remarks:** HTTP POST `/publicapi/v1/sessions/{sessionId}/files/{id}/snapshot`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `sessionId` (Guid) — Path parameter `sessionId` from `/publicapi/v1/sessions/{sessionId}/files/{id}/snapshot`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/sessions/{sessionId}/files/{id}/snapshot`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateSessionFile(Guid sessionId, CreateSessionFileRequest request)`

Create metadata for a new Studio Session file

**Remarks:** HTTP POST `/publicapi/v1/sessions/{sessionId}/files`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `sessionId` (Guid) — Path parameter `sessionId` from `/publicapi/v1/sessions/{sessionId}/files`.
- `request` (CreateSessionFileRequest) — Request body. JSON object with fields: CRC, Name, Size, Source.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task DeleteFile(Guid sessionId, Guid id)`

Delete a Studio Session file

**Remarks:** HTTP DELETE `/publicapi/v1/sessions/{sessionId}/files/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `sessionId` (Guid) — Path parameter `sessionId` from `/publicapi/v1/sessions/{sessionId}/files/{id}`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/sessions/{sessionId}/files/{id}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task GetFile(Guid sessionId, Guid id)`

Get Studio Session file metadata

**Remarks:** HTTP GET `/publicapi/v1/sessions/{sessionId}/files/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `sessionId` (Guid) — Path parameter `sessionId` from `/publicapi/v1/sessions/{sessionId}/files/{id}`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/sessions/{sessionId}/files/{id}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task GetFileSnapshot(Guid sessionId, Guid id)`

Get metadata on Studio Session document snapshot job

**Remarks:** HTTP GET `/publicapi/v1/sessions/{sessionId}/files/{id}/snapshot`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `sessionId` (Guid) — Path parameter `sessionId` from `/publicapi/v1/sessions/{sessionId}/files/{id}/snapshot`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/sessions/{sessionId}/files/{id}/snapshot`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task ListSessionFiles(Guid sessionId)`

Get all files in a Studio Session

**Remarks:** HTTP GET `/publicapi/v1/sessions/{sessionId}/files`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `sessionId` (Guid) — Path parameter `sessionId` from `/publicapi/v1/sessions/{sessionId}/files`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

## SessionsPermissionsApi (class)

Bluebeam Studio API — Sessions / Permissions endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /publicapi/v1/. Source: https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Methods
#### `Task ListSessionPermissions(Guid sessionId)`

Get Studio Session permissions

**Remarks:** HTTP GET `/publicapi/v1/sessions/{sessionId}/permissions`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `sessionId` (Guid) — Path parameter `sessionId` from `/publicapi/v1/sessions/{sessionId}/permissions`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task UpdateSessionPermissions(Guid sessionId, UpdateSessionPermissionsRequest request)`

Update Studio Session permissions

**Remarks:** HTTP PUT `/publicapi/v1/sessions/{sessionId}/permissions`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `sessionId` (Guid) — Path parameter `sessionId` from `/publicapi/v1/sessions/{sessionId}/permissions`.
- `request` (UpdateSessionPermissionsRequest) — Request body. JSON object with fields: Allow, Type.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

## SessionsUsersApi (class)

Bluebeam Studio API — Sessions / Users endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /publicapi/v1/. Source: https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Methods
#### `Task CreateSessionUser(Guid sessionId, CreateSessionUserRequest request)`

Add user to a Studio Session

**Remarks:** HTTP POST `/publicapi/v1/sessions/{sessionId}/users`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `sessionId` (Guid) — Path parameter `sessionId` from `/publicapi/v1/sessions/{sessionId}/users`.
- `request` (CreateSessionUserRequest) — Request body. JSON object with fields: Email, Message, SendEmail.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task GetUser(Guid sessionId, Guid id)`

Get a single user in a Studio Session

**Remarks:** HTTP GET `/publicapi/v1/sessions/{sessionId}/users/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `sessionId` (Guid) — Path parameter `sessionId` from `/publicapi/v1/sessions/{sessionId}/users/{id}`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/sessions/{sessionId}/users/{id}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task ListSessionUsers(Guid sessionId)`

Get all users in a Studio Session

**Remarks:** HTTP GET `/publicapi/v1/sessions/{sessionId}/users`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `sessionId` (Guid) — Path parameter `sessionId` from `/publicapi/v1/sessions/{sessionId}/users`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task ListUserPermissions(Guid sessionId, Guid userId)`

Get Studio Session user permissions

**Remarks:** HTTP GET `/publicapi/v1/sessions/{sessionId}/users/{userId}/permissions`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `sessionId` (Guid) — Path parameter `sessionId` from `/publicapi/v1/sessions/{sessionId}/users/{userId}/permissions`.
- `userId` (Guid) — Path parameter `userId` from `/publicapi/v1/sessions/{sessionId}/users/{userId}/permissions`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task UpdateUser(Guid sessionId, Guid id, UpdateUserRequest request)`

Update user in a Studio Session

**Remarks:** HTTP PUT `/publicapi/v1/sessions/{sessionId}/users/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `sessionId` (Guid) — Path parameter `sessionId` from `/publicapi/v1/sessions/{sessionId}/users/{id}`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/sessions/{sessionId}/users/{id}`.
- `request` (UpdateUserRequest) — Request body. JSON object with fields: RestrictedStatus, StatusMessage.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task UpdateUserPermissions(Guid sessionId, Guid userId, UpdateUserPermissionsRequest request)`

Update Studio Session user permissions

**Remarks:** HTTP PUT `/publicapi/v1/sessions/{sessionId}/users/{userId}/permissions`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `sessionId` (Guid) — Path parameter `sessionId` from `/publicapi/v1/sessions/{sessionId}/users/{userId}/permissions`.
- `userId` (Guid) — Path parameter `userId` from `/publicapi/v1/sessions/{sessionId}/users/{userId}/permissions`.
- `request` (UpdateUserPermissionsRequest) — Request body. JSON object with fields: Allow, Type.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

