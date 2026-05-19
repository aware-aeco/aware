---
name: bluebeam-bluebeam-studioapi-projects
description: This skill encodes the bluebeam v1 surface of the Bluebeam.StudioApi.Projects namespace — 6 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ProjectsApi, ProjectsFoldersApi, ProjectsFilesApi, ProjectsUsersApi, ProjectsPermissionsApi, ProjectsSharedLinksApi.
---

# Bluebeam.StudioApi.Projects

Auto-generated from vendor docs for bluebeam v1. 6 types in this namespace.

## ProjectsApi (class)

Bluebeam Studio API — Projects endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /publicapi/v1/. Source: https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Methods
#### `Task CreateProject(CreateProjectRequest request)`

Create a project

**Remarks:** HTTP POST `/publicapi/v1/projects`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `request` (CreateProjectRequest) — Request body. JSON object with fields: Name, Notification, Restricted.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task DeleteProject(Guid id)`

Delete a project

**Remarks:** HTTP DELETE `/publicapi/v1/projects/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{id}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task GetHealthcheckDetails()`

GET /publicapi/v1/projects/healthcheck/details

**Remarks:** HTTP GET `/publicapi/v1/projects/healthcheck/details`. Vendor docs page: https://support.bluebeam.com/developer/

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task GetProject(Guid id)`

Get a single project

**Remarks:** HTTP GET `/publicapi/v1/projects/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{id}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task InviteProject(Guid projectId, InviteProjectRequest request)`

Invite user to project by email

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/invite`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/invite`.
- `request` (InviteProjectRequest) — Request body. JSON object with fields: Email, Message.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task ListProjects()`

Get all projects

**Remarks:** HTTP GET `/publicapi/v1/projects`. Vendor docs page: https://support.bluebeam.com/developer/

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task UpdateProject(Guid id, UpdateProjectRequest request)`

Update a project

**Remarks:** HTTP PUT `/publicapi/v1/projects/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{id}`.
- `request` (UpdateProjectRequest) — Request body. JSON object with fields: Name, Notification, OwnerEmailOrId, Restricted.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

## ProjectsFilesApi (class)

Bluebeam Studio API — Projects / Files endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /publicapi/v1/. Source: https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Methods
#### `Task CheckinFile(Guid projectId, Guid id)`

Checkin project file.              PUT actual file to UploadUrl returned in a create response              POST confirm-checkin at the end of file upload to make it available in a project

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{id}/checkin`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{id}/checkin`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{projectId}/files/{id}/checkin`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CheckoutFile(Guid projectId, Guid id)`

Checkout file for modification

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{id}/checkout`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{id}/checkout`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{projectId}/files/{id}/checkout`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CheckoutToSessionFile(Guid projectId, Guid id, CheckoutToSessionFileRequest request)`

Checkout project file to Studio Session.              This operation will link project to a Session

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{id}/checkout-to-session`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{id}/checkout-to-session`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{projectId}/files/{id}/checkout-to-session`.
- `request` (CheckoutToSessionFileRequest) — Request body. JSON object with fields: SessionId.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task ConfirmCheckinFile(Guid projectId, Guid id, ConfirmCheckinFileRequest request)`

Confirm checkin of project file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{id}/confirm-checkin`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{id}/confirm-checkin`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{projectId}/files/{id}/confirm-checkin`.
- `request` (ConfirmCheckinFileRequest) — Request body. JSON object with fields: CRC, Comment, Size.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task ConfirmUploadFile(Guid projectId, Guid id)`

Confirm Project file upload and make it available in a Session. Should be called after uploading project file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{id}/confirm-upload`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{id}/confirm-upload`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{projectId}/files/{id}/confirm-upload`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CopyFile(Guid projectId, Guid id, CopyFileRequest request)`

Copy a file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{id}/copy`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{id}/copy`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{projectId}/files/{id}/copy`.
- `request` (CopyFileRequest) — Request body. JSON object with fields: Name, ParentFolderId.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task CreateProjectFile(Guid projectId, CreateProjectFileRequest request)`

Create metadata for a new project file.              PUT actual file to UploadUrl returned in a create response              POST confirm-upload at the end of file upload to make it available in a project

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files`.
- `request` (CreateProjectFileRequest) — Request body. JSON object with fields: CRC, Name, ParentFolderId, Size.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task DeleteFile(Guid projectId, Guid id)`

Delete a project file

**Remarks:** HTTP DELETE `/publicapi/v1/projects/{projectId}/files/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{id}`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{projectId}/files/{id}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task DeleteFileCheckout(Guid projectId, Guid id)`

Revoke file checkout. This will put the file in a conflicted state in Revu if there are local changes

**Remarks:** HTTP DELETE `/publicapi/v1/projects/{projectId}/files/{id}/checkout`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{id}/checkout`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{projectId}/files/{id}/checkout`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task GetFile(Guid projectId, Guid id)`

Get file metadata.              Includes DownloadUrl which can be used to download file content

**Remarks:** HTTP GET `/publicapi/v1/projects/{projectId}/files/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{id}`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{projectId}/files/{id}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task GetFileByPath(Guid projectId)`

Get file metadata by path.              Includes DownloadUrl which can be used to download file content

**Remarks:** HTTP GET `/publicapi/v1/projects/{projectId}/files/by-path`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/by-path`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task GetRevision(Guid projectId, int fileId, Guid id)`

Get a file revision metadata.              Includes DownloadUrl which can be used to download file content.              For DownloadUrl of most recent revision use Get /v1/projects/{PublicId}/files/{id}

**Remarks:** HTTP GET `/publicapi/v1/projects/{projectId}/files/{fileId}/revisions/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/revisions/{id}`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/revisions/{id}`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{projectId}/files/{fileId}/revisions/{id}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task ListFileRevisions(Guid projectId, int fileId)`

Get all file revisions

**Remarks:** HTTP GET `/publicapi/v1/projects/{projectId}/files/{fileId}/revisions`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/revisions`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/revisions`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task ListProjectFiles(Guid projectId)`

Get all files in a project

**Remarks:** HTTP GET `/publicapi/v1/projects/{projectId}/files`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task RestoreRevision(Guid projectId, int fileId, Guid id)`

Restore file revision

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/files/{fileId}/revisions/{id}/restore`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/revisions/{id}/restore`.
- `fileId` (int) — Path parameter `fileId` from `/publicapi/v1/projects/{projectId}/files/{fileId}/revisions/{id}/restore`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{projectId}/files/{fileId}/revisions/{id}/restore`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task UpdateFile(Guid projectId, Guid id, UpdateFileRequest request)`

Update project file. To move the file, change its parent folder id

**Remarks:** HTTP PUT `/publicapi/v1/projects/{projectId}/files/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/files/{id}`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{projectId}/files/{id}`.
- `request` (UpdateFileRequest) — Request body. JSON object with fields: Name, ParentFolderId.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

## ProjectsFoldersApi (class)

Bluebeam Studio API — Projects / Folders endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /publicapi/v1/. Source: https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Methods
#### `Task CreateProjectFolder(Guid projectId, CreateProjectFolderRequest request)`

Create a new empty project folder

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/folders`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/folders`.
- `request` (CreateProjectFolderRequest) — Request body. JSON object with fields: Comment, Name, ParentFolderId.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task DeleteFolder(Guid projectId, Guid id)`

Delete a project folder

**Remarks:** HTTP DELETE `/publicapi/v1/projects/{projectId}/folders/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/folders/{id}`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{projectId}/folders/{id}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task GetFolder(Guid projectId, Guid id)`

Get project folder metadata. Use 0 to get the project's root folder

**Remarks:** HTTP GET `/publicapi/v1/projects/{projectId}/folders/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/folders/{id}`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{projectId}/folders/{id}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task ListFolderItems(Guid projectId, Guid id)`

Get files and/or folders contained within specified folder. Use 0 to get the project's root folder items

**Remarks:** HTTP GET `/publicapi/v1/projects/{projectId}/folders/{id}/items`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/folders/{id}/items`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{projectId}/folders/{id}/items`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task ListProjectFolders(Guid projectId)`

Get all files in a project

**Remarks:** HTTP GET `/publicapi/v1/projects/{projectId}/folders`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/folders`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task UpdateFolder(Guid projectId, Guid id, UpdateFolderRequest request)`

Update project folder. To move the folder, change its parent folder id

**Remarks:** HTTP PUT `/publicapi/v1/projects/{projectId}/folders/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/folders/{id}`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{projectId}/folders/{id}`.
- `request` (UpdateFolderRequest) — Request body. JSON object with fields: Name, ParentFolderId.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

## ProjectsPermissionsApi (class)

Bluebeam Studio API — Projects / Permissions endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /publicapi/v1/. Source: https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Methods
#### `Task ListFolderPermissions(Guid projectId, int folderId)`

Get project folder permission.              Will return empty string if permission is not set

**Remarks:** HTTP GET `/publicapi/v1/projects/{projectId}/folders/{folderId}/permissions`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/folders/{folderId}/permissions`.
- `folderId` (int) — Path parameter `folderId` from `/publicapi/v1/projects/{projectId}/folders/{folderId}/permissions`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task ListProjectPermissions(Guid projectId)`

Get project permissions

**Remarks:** HTTP GET `/publicapi/v1/projects/{projectId}/permissions`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/permissions`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task UpdateFolderPermissions(Guid projectId, int folderId, UpdateFolderPermissionsRequest request)`

Update project folder permissions.              Permission types: Hidden, Read, ReadWrite, ReadWriteDelete.              Set Permission to null to inherit permission from parent folder

**Remarks:** HTTP PUT `/publicapi/v1/projects/{projectId}/folders/{folderId}/permissions`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/folders/{folderId}/permissions`.
- `folderId` (int) — Path parameter `folderId` from `/publicapi/v1/projects/{projectId}/folders/{folderId}/permissions`.
- `request` (UpdateFolderPermissionsRequest) — Request body. JSON object with fields: Permission, UserId.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task UpdateProjectPermissions(Guid projectId, UpdateProjectPermissionsRequest request)`

Update project permissions.              Set value to allow, deny, or default

**Remarks:** HTTP PUT `/publicapi/v1/projects/{projectId}/permissions`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/permissions`.
- `request` (UpdateProjectPermissionsRequest) — Request body. JSON object with fields: Allow, Type.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

## ProjectsSharedLinksApi (class)

Bluebeam Studio API — Projects / Shared Links endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /publicapi/v1/. Source: https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Methods
#### `Task CreateProjectSharedLink(Guid projectId, CreateProjectSharedLinkRequest request)`

Create a shared link for a project file

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/sharedlinks`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/sharedlinks`.
- `request` (CreateProjectSharedLinkRequest) — Request body. JSON object with fields: Expires, Flatten, Password, PasswordProtected, ProjectFileId.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task DeleteSharedLink(Guid projectId, Guid id)`

Delete shared link

**Remarks:** HTTP DELETE `/publicapi/v1/projects/{projectId}/sharedlinks/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/sharedlinks/{id}`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{projectId}/sharedlinks/{id}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task GetSharedLink(Guid projectId, Guid id)`

Get shared link metadata

**Remarks:** HTTP GET `/publicapi/v1/projects/{projectId}/sharedlinks/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/sharedlinks/{id}`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{projectId}/sharedlinks/{id}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task ListProjectSharedlinks(Guid projectId)`

Get all shared links in a project

**Remarks:** HTTP GET `/publicapi/v1/projects/{projectId}/sharedlinks`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/sharedlinks`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task UpdateSharedLink(Guid projectId, Guid id, UpdateSharedLinkRequest request)`

Update shared link. Can be used to disable or hide shared link

**Remarks:** HTTP PUT `/publicapi/v1/projects/{projectId}/sharedlinks/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/sharedlinks/{id}`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{projectId}/sharedlinks/{id}`.
- `request` (UpdateSharedLinkRequest) — Request body. JSON object with fields: Enabled, Hidden.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

## ProjectsUsersApi (class)

Bluebeam Studio API — Projects / Users endpoints. Auto-generated REST surface; one method per HTTP operation.

**Remarks:** REST root: /publicapi/v1/. Source: https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json

[Vendor docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

### Methods
#### `Task CreateProjectUser(Guid projectId, CreateProjectUserRequest request)`

Add user to a project

**Remarks:** HTTP POST `/publicapi/v1/projects/{projectId}/users`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/users`.
- `request` (CreateProjectUserRequest) — Request body. JSON object with fields: Email, Message, SendEmail.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task DeleteUser(Guid projectId, Guid id)`

Remove user from project

**Remarks:** HTTP DELETE `/publicapi/v1/projects/{projectId}/users/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/users/{id}`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{projectId}/users/{id}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task GetUser(Guid projectId, Guid id)`

Get a single user in a project

**Remarks:** HTTP GET `/publicapi/v1/projects/{projectId}/users/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/users/{id}`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{projectId}/users/{id}`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task ListProjectUsers(Guid projectId)`

Get all users in a project

**Remarks:** HTTP GET `/publicapi/v1/projects/{projectId}/users`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/users`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task ListUserPermissions(Guid projectId, Guid userId)`

Get project user permissions

**Remarks:** HTTP GET `/publicapi/v1/projects/{projectId}/users/{userId}/permissions`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/users/{userId}/permissions`.
- `userId` (Guid) — Path parameter `userId` from `/publicapi/v1/projects/{projectId}/users/{userId}/permissions`.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task UpdateUser(Guid projectId, Guid id, UpdateUserRequest request)`

Update user in a project

**Remarks:** HTTP PUT `/publicapi/v1/projects/{projectId}/users/{id}`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/users/{id}`.
- `id` (Guid) — Path parameter `id` from `/publicapi/v1/projects/{projectId}/users/{id}`.
- `request` (UpdateUserRequest) — Request body. JSON object with fields: RestrictedStatus.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

#### `Task UpdateUserPermissions(Guid projectId, Guid userId, UpdateUserPermissionsRequest request)`

Update project user permissions.              Permission types: CreateSessions, UndoCheckouts, Invite, ManageParticipants, ManagePermissions, FullControl, ShareItems.              Set value to allow, deny, or default

**Remarks:** HTTP PUT `/publicapi/v1/projects/{projectId}/users/{userId}/permissions`. Vendor docs page: https://support.bluebeam.com/developer/

**Parameters:**
- `projectId` (Guid) — Path parameter `projectId` from `/publicapi/v1/projects/{projectId}/users/{userId}/permissions`.
- `userId` (Guid) — Path parameter `userId` from `/publicapi/v1/projects/{projectId}/users/{userId}/permissions`.
- `request` (UpdateUserPermissionsRequest) — Request body. JSON object with fields: Allow, Type.

[Docs](https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json)

