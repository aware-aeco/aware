---
name: trimble-connect-projects
description: Trimble Connect Projects API — list projects, get project details, project members, user information, ToDo tasks, SDK pagination, search projects, project settings, AEC project management. This skill should be used when listing, searching, or managing Trimble Connect projects, members, or tasks.
---

# Trimble Connect — Projects API Reference

Base URL (regional, see trimble-connect-auth skill): `https://app.connect.trimble.com/tc/api/2.0`

> **CRITICAL:** `apiBaseUrl` already includes `/2.0` — NEVER add `/2.0/` in code examples.
> All paths below are relative (e.g., `projects` → `$"{apiBaseUrl}/projects"`).

---

## Projects

### List All Projects

```
GETprojects
```

Returns all projects the authenticated user has access to.

```csharp
var url = $"{apiBaseUrl}/projects";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

var projects = new List<Dictionary<string, object>>();

foreach (var project in doc.RootElement.EnumerateArray())
{
    var id = project.GetProperty("id").GetString();
    var name = project.GetProperty("name").GetString();
    var description = project.TryGetProperty("description", out var d) ? d.GetString() : null;
    var createdOn = project.TryGetProperty("createdOn", out var c) ? c.GetString() : null;
    var updatedOn = project.TryGetProperty("updatedOn", out var u) ? u.GetString() : null;
    var rootId = project.TryGetProperty("rootId", out var r) ? r.GetString() : null;
    var location = project.TryGetProperty("location", out var l) ? l.GetString() : null;

    projects.Add(new Dictionary<string, object>
    {
        ["id"] = id,
        ["name"] = name,
        ["description"] = description ?? string.Empty,
        ["rootId"] = rootId ?? string.Empty,
        ["location"] = location ?? string.Empty,
        ["createdOn"] = createdOn ?? string.Empty,
        ["updatedOn"] = updatedOn ?? string.Empty
    });
}
```

### Get Project Details

```
GETprojects/{projectId}
```

Returns detailed information about a specific project.

```csharp
var url = $"{apiBaseUrl}/projects/{projectId}";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

var name = doc.RootElement.GetProperty("name").GetString();
var id = doc.RootElement.GetProperty("id").GetString();
```

### Project Object Schema

| Property | Type | Description |
|----------|------|-------------|
| `id` | string | Unique project GUID |
| `name` | string | Project display name |
| `description` | string | Project description |
| `rootId` | string | Root folder GUID for file storage |
| `location` | string | Region name (e.g., `"northAmerica"`, `"europe"`) |
| `thumbnail` | string | URL to project thumbnail image |
| `createdOn` | string (ISO 8601) | Creation timestamp |
| `modifiedOn` | string (ISO 8601) | Last modification timestamp |
| `updatedOn` | string (ISO 8601) | Last update timestamp |
| `lastVisitedOn` | string (ISO 8601) | Last visited timestamp |
| `createdBy` | object | User object `{id, email, firstName, lastName, status}` |
| `modifiedBy` | object | User object `{id, email, firstName, lastName, status}` |
| `size` | long | Total project size in bytes |
| `filesCount` | int | Number of files in the project |
| `foldersCount` | int | Number of folders in the project |
| `usersCount` | int | Number of project members |
| `groupsCount` | int | Number of groups |
| `versionsCount` | int | Number of file versions |
| `access` | string | User's access level (e.g., `"FULL_ACCESS"`) |
| `startDate` | string (ISO 8601) | Project start date |
| `endDate` | string (ISO 8601) | Project end date |

> **IMPORTANT:** The root folder property is `rootId`, NOT `rootFolderId`.

---

## Users

### Get Current User

```
GETusers/me
```

Returns information about the authenticated user. Useful for connection testing.

```csharp
var url = $"{apiBaseUrl}/users/me";
var response = await client.GetAsync(url).ConfigureAwait(false);

if (response.IsSuccessStatusCode)
{
    var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
    using var doc = JsonDocument.Parse(json);
    var email = doc.RootElement.TryGetProperty("email", out var e) ? e.GetString() : null;
    var displayName = doc.RootElement.TryGetProperty("displayName", out var n) ? n.GetString() : null;
}
```

### User Object Schema

| Property | Type | Description |
|----------|------|-------------|
| `id` | string | Unique user GUID |
| `tiduuid` | string | Trimble Identity UUID |
| `email` | string | User email address |
| `firstName` | string | First name |
| `lastName` | string | Last name |
| `status` | string | Account status (e.g., `"ACTIVE"`) |

---

## Project Members

### List Project Members

```
GETprojects/{projectId}/users
```

Returns all users with access to the project.

```csharp
var url = $"{apiBaseUrl}/projects/{projectId}/users";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

foreach (var member in doc.RootElement.EnumerateArray())
{
    var email = member.TryGetProperty("email", out var e) ? e.GetString() : null;
    var role = member.TryGetProperty("role", out var r) ? r.GetString() : null;
}
```

---

## Folders & Files

> **IMPORTANT:** Folder/file endpoints do NOT include `/projects/{projectId}/`. IDs are globally unique.

### List Folder Contents

```
GETfolders/{folderId}/items
```

### Get File Info

```
GETfiles/{fileId}
```

### File Object Schema

| Property | Type | Description |
|----------|------|-------------|
| `id` | string | Unique file GUID |
| `name` | string | File name with extension |
| `parentId` | string | Parent folder GUID |
| `size` | long | File size in bytes |
| `createdOn` | string (ISO 8601) | Upload timestamp |
| `modifiedOn` | string (ISO 8601) | Last modification timestamp |
| `versionId` | string | Current version GUID |

---

## ToDos (Task Management)

### List ToDos

```
GETprojects/{projectId}/todos
```

### Create ToDo

```
POSTprojects/{projectId}/todos
Content-Type: application/json
```

**Request Body:**
```json
{
    "label": "Review structural drawings",
    "description": "Check beam connections at grid A-5",
    "priority": 1,
    "assignedTo": "engineer@company.com",
    "dueDate": "2025-06-15T00:00:00Z",
    "status": 0
}
```

### ToDo Status Values

| Value | Meaning |
|-------|---------|
| `0` | Active / Open |
| `1` | Completed |
| `2` | Deleted |

### ToDo Priority Values

| Value | Meaning |
|-------|---------|
| `0` | None |
| `1` | Low |
| `2` | Normal |
| `3` | High |
| `4` | Urgent |

### Update ToDo

```
PUTprojects/{projectId}/todos/{todoId}
Content-Type: application/json
```

```csharp
var url = $"{apiBaseUrl}/projects/{projectId}/todos/{todoId}";
var payload = new Dictionary<string, object>
{
    ["label"] = "Updated task label",
    ["status"] = 1, // Mark as completed
    ["priority"] = 3  // Change to High
};

var jsonContent = JsonSerializer.Serialize(payload);
var content = new StringContent(jsonContent, System.Text.Encoding.UTF8, "application/json");

var request = new HttpRequestMessage(HttpMethod.Put, url) { Content = content };
var response = await client.SendAsync(request).ConfigureAwait(false);
```

### Delete ToDo

```
DELETEprojects/{projectId}/todos/{todoId}
```

```csharp
var url = $"{apiBaseUrl}/projects/{projectId}/todos/{todoId}";
var response = await client.DeleteAsync(url).ConfigureAwait(false);
```

### Get Single ToDo

```
GETprojects/{projectId}/todos/{todoId}
```

### ToDo Object Schema

| Property | Type | Description |
|----------|------|-------------|
| `id` | string | Unique ToDo GUID |
| `label` | string | ToDo title/label |
| `description` | string | Detailed description |
| `status` | int | Status: 0=Open, 1=Completed, 2=Deleted |
| `priority` | int | Priority: 0=None, 1=Low, 2=Normal, 3=High, 4=Urgent |
| `assignedTo` | string | Assignee email address |
| `dueDate` | string (ISO 8601) | Due date |
| `createdOn` | string (ISO 8601) | Creation timestamp (read-only) |
| `modifiedOn` | string (ISO 8601) | Last modified timestamp (read-only) |
| `createdBy` | string | Creator user ID (read-only) |

---

## Pagination

### REST API Pagination (top/skip)

Large result sets may be paginated. Check for pagination headers:

```csharp
// Check if response is paginated
if (response.Headers.TryGetValues("X-Total-Count", out var countValues))
{
    var totalCount = int.Parse(countValues.First());
}

// Request with pagination
var url = $"{apiBaseUrl}/projects?top=50&skip=0";
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `top` | int | Number of items per page (max 100) |
| `skip` | int | Number of items to skip |

### SDK Pagination (`IQueryResult<T>`)

The Trimble Connect .NET SDK returns `IQueryResult<T>` for paginated collections. Use `HasMore` and `GetNextPageAsync()` to iterate through pages:

```csharp
// Using SDK client for paginated results
var projectClient = await trimbleConnectClient.GetProjectClientAsync(project).ConfigureAwait(false);

// Get first page of ToDos
var todosPage = await projectClient.Todos.GetAllAsync().ConfigureAwait(false);

var allTodos = new List<Todo>();
allTodos.AddRange(todosPage);

// Iterate through remaining pages
while (todosPage.HasMore)
{
    todosPage = await todosPage.GetNextPageAsync().ConfigureAwait(false);
    allTodos.AddRange(todosPage);
}
```

**Shortcut — `ReceiveAll()` extension:**

```csharp
// Fetches ALL pages automatically (convenience method)
var allTodos = await projectClient.Todos.GetAllAsync().ReceiveAll().ConfigureAwait(false);
```

**`IQueryResult<T>` interface:**

| Member | Type | Description |
|--------|------|-------------|
| `HasMore` | `bool` | Whether more pages are available |
| `GetNextPageAsync()` | `Task<IQueryResult<T>>` | Fetch the next page |
| `ReceiveAll()` | extension | Fetches all remaining pages into a single list |

**Note:** `ReceiveAll()` loads everything into memory. For large datasets, prefer the `HasMore` / `GetNextPageAsync()` loop with per-page processing.

---

## Search

### Cross-Project Search (SDK)

The `TrimbleConnectClient` provides a `SearchAsync` method for searching across projects:

```csharp
// Search via SDK client
var results = await trimbleConnectClient.SearchAsync(
    query: "beam connection",
    projectId: projectId
).ConfigureAwait(false);
```

### REST API Search (Files)

Search for files within a project by name:

```csharp
var url = $"{apiBaseUrl}/files?name={Uri.EscapeDataString(searchTerm)}&projectId={projectId}";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

foreach (var file in doc.RootElement.EnumerateArray())
{
    var id = file.GetProperty("id").GetString();
    var name = file.GetProperty("name").GetString();
}
```

---

## Project-Level BCF Documents

### List BCF Documents

```
GET /bcf/2.1/projects/{projectId}/documents
```

Returns BCF documents associated with the project:

```csharp
var url = $"{bcfBaseUrl}/projects/{projectId}/documents";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

foreach (var bcfDoc in doc.RootElement.EnumerateArray())
{
    var guid = bcfDoc.GetProperty("guid").GetString();
    var filename = bcfDoc.TryGetProperty("filename", out var f) ? f.GetString() : null;
}
```

### Upload BCF Document

```
POST /bcf/2.1/projects/{projectId}/documents
Content-Type: multipart/form-data
```

```csharp
var url = $"{bcfBaseUrl}/projects/{projectId}/documents";
using var formContent = new MultipartFormDataContent();

var fileBytes = System.IO.File.ReadAllBytes(bcfFilePath);
var fileContent = new ByteArrayContent(fileBytes);
fileContent.Headers.ContentType = new System.Net.Http.Headers.MediaTypeHeaderValue("application/octet-stream");
formContent.Add(fileContent, "file", System.IO.Path.GetFileName(bcfFilePath));

var response = await client.PostAsync(url, formContent).ConfigureAwait(false);
```

---

## Common Patterns

### Project ID Fallback

In AWARE workflows, project ID is resolved in this order:
1. Explicit `projectId` parameter on the node
2. Workflow-level settings override
3. Default project from `aware connect <agent>` in your terminal### Building Topic Web Links

```csharp
// Direct link to a topic in the web portal
var topicUrl = $"{webBaseUrl}/projects/{projectId}/topics/{topicId}";
```

### Sorting Projects

Projects API returns items unsorted. Sort client-side:

```csharp
// Sort by name
var sorted = projects.OrderBy(p => p["name"].ToString()).ToList();
```
