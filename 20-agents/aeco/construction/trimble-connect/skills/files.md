---
name: trimble-connect-files
description: This skill should be used when authoring AWARE compositions or generated code that uploads files to Trimble Connect, downloads files, browses or modifies folders, lists file versions, or links files to BCF topics. Covers the 3-step package upload pattern (initiate → PUT to pre-signed S3 URL → complete), the 2-step download pattern, folder navigation by path, file versioning, and the critical conventions (`apiBaseUrl` already includes `/2.0`, IDs are globally unique across projects, DELETE uses `versionId` not `id`, `rootId` not `rootFolderId`).
---

# Trimble Connect — Files & Folders API reference

Base URL (regional, see the [`trimble-connect-auth-flow`](./auth-flow.md) skill): `https://app.connect.trimble.com/tc/api/2.0`

> **CRITICAL CONVENTIONS:**
> - `apiBaseUrl` already includes `/2.0` — NEVER prepend `/2.0/` in code examples.
> - Folder and file endpoints do NOT include `/projects/{projectId}/` — IDs are globally unique.
> - All paths shown are relative to `apiBaseUrl` (e.g., `folders/{id}/items` → `$"{apiBaseUrl}/folders/{id}/items"`).
> - **For .NET implementations:** apply `.ConfigureAwait(false)` on every `await` (avoids UI-thread capture in WPF / WinForms hosts). For non-.NET runtimes (Python, TypeScript, Go), use the language's idiomatic async patterns; the rule does not apply.

The HTTP client referenced as `client` below is the one the trimble-connect agent's runtime provides. It carries the `Authorization: Bearer …` header automatically — see the auth-flow skill. Do not construct a separate authenticated client.

---

## Folders

### Get root folder

Every project has a root folder. The `rootId` property from the project listing IS the root folder ID.

> **IMPORTANT:** The property name is `rootId`, NOT `rootFolderId`.

```csharp
// rootId is returned directly in the project listing response
// No extra API call needed — just use rootId from GET /projects
```

### List folder contents

```
GET folders/{folderId}/items
```

```csharp
var url = $"{apiBaseUrl}/folders/{folderId}/items";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

foreach (var item in doc.RootElement.EnumerateArray())
{
    var id = item.GetProperty("id").GetString();
    var name = item.GetProperty("name").GetString();
    var type = item.GetProperty("type").GetString(); // "FILE" or "FOLDER"

    if (type == "FILE")
    {
        var size = item.TryGetProperty("size", out var s) ? s.GetInt64() : 0;
        var versionId = item.TryGetProperty("versionId", out var v) ? v.GetString() : null;
    }
}
```

### Get folder info

```
GET folders/{folderId}
```

```csharp
var url = $"{apiBaseUrl}/folders/{folderId}";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

var folderName = doc.RootElement.GetProperty("name").GetString();
var parentId = doc.RootElement.TryGetProperty("parentId", out var p) ? p.GetString() : null;
```

### Navigate by path

```
GET folders/by_path?path={searchPath}&projectId={projectId}
```

```csharp
var encodedPath = Uri.EscapeDataString("RootFolder/Reports/Phase5");
var url = $"{apiBaseUrl}/folders/by_path?path={encodedPath}&projectId={projectId}";
var response = await client.GetAsync(url).ConfigureAwait(false);
```

### Create folder

```
POST folders
Content-Type: application/json
Body: { "name": "...", "parentId": "..." }
```

```csharp
var url = $"{apiBaseUrl}/folders";
var payload = new { name = folderName, parentId = parentFolderId };
var content = new StringContent(JsonSerializer.Serialize(payload), System.Text.Encoding.UTF8, "application/json");
var response = await client.PostAsync(url, content).ConfigureAwait(false);

if (response.IsSuccessStatusCode)
{
    var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
    using var doc = JsonDocument.Parse(json);
    var newFolderId = doc.RootElement.GetProperty("id").GetString();
}
```

### Rename or move folder

```
PATCH folders/{folderId}
Header: If-Match: "{folderVersionId}"
Body: { "name": "...", "parentId": "..." }   (both optional)
```

```csharp
var url = $"{apiBaseUrl}/folders/{folderId}";
var payload = new { name = "New Name" }; // or { parentId = newParentId } to move
var content = new StringContent(JsonSerializer.Serialize(payload), System.Text.Encoding.UTF8, "application/json");
var request = new HttpRequestMessage(new HttpMethod("PATCH"), url) { Content = content };
request.Headers.Add("If-Match", $"\"{folderVersionId}\"");
var response = await client.SendAsync(request).ConfigureAwait(false);
```

### Delete folder

```
DELETE folders/{folderVersionId}
```

> **IMPORTANT:** Uses the folder's VERSION id, not the folder id.

```csharp
var url = $"{apiBaseUrl}/folders/{folderVersionId}";
var response = await client.DeleteAsync(url).ConfigureAwait(false);
```

### Folder object schema

| Property | Type | Description |
|----------|------|-------------|
| `id` | string | Unique folder identifier |
| `versionId` | string | Folder version identifier |
| `name` | string | Folder display name |
| `parentId` | string | Parent folder identifier (null for root) |
| `parentType` | string | Parent type (e.g., `"FOLDER"`) |
| `type` | string | Always `"FOLDER"` |
| `status` | string | Status (e.g., `"AVAILABLE"`) |
| `hasChildren` | bool | Whether folder has child items |
| `createdOn` | string (ISO 8601) | Creation timestamp |
| `modifiedOn` | string (ISO 8601) | Last modification timestamp |
| `createdBy` | object | Creator `{id, email, firstName, lastName}` |
| `modifiedBy` | object | Last modifier `{id, email, firstName, lastName}` |
| `permission` | string | Access level for current user |

---

## Files

### Get file info

```
GET files/{fileId}
Query: versionId={versionId}  (optional, for a specific version)
```

```csharp
var url = $"{apiBaseUrl}/files/{fileId}";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

var fileName = doc.RootElement.GetProperty("name").GetString();
var size = doc.RootElement.TryGetProperty("size", out var s) ? s.GetInt64() : 0;
var versionId = doc.RootElement.TryGetProperty("versionId", out var v) ? v.GetString() : null;
```

### Upload file (3-step package upload)

Trimble Connect uses a **3-step package upload**:

1. **Initiate** — `POST files/fs/upload?parentId={folderId}&parentType=FOLDER` → returns a pre-signed URL
2. **Upload** — `PUT` file bytes to the pre-signed URL (different domain, no auth header)
3. **Complete** — `GET files/fs/upload?uploadId={id}&wait=true` → returns file info

> **CRITICAL:**
> - `parentId` and `parentType` are **QUERY parameters**, NOT in the JSON body.
> - The JSON body contains only `name` and `contents`.
> - The pre-signed upload URL is on a different domain (S3) — do NOT send `Authorization` header to it.
> - Do NOT use `POST files` with `multipart/form-data` — returns 415.

```csharp
// Step 1: Initiate upload (parentId and parentType are QUERY PARAMS)
var fileBytes = System.IO.File.ReadAllBytes(localFilePath);
var fileName = System.IO.Path.GetFileName(localFilePath);
var initiateUrl = $"{apiBaseUrl}/files/fs/upload?parentId={parentFolderId}&parentType=FOLDER";

var initiatePayload = new { name = fileName, contents = new[] { new { } } };
var initiateContent = new StringContent(
    JsonSerializer.Serialize(initiatePayload),
    System.Text.Encoding.UTF8, "application/json");

var initiateResponse = await client.PostAsync(initiateUrl, initiateContent).ConfigureAwait(false);
if (initiateResponse.IsSuccessStatusCode is false)
{
    var err = await initiateResponse.Content.ReadAsStringAsync().ConfigureAwait(false);
    throw new Exception($"Upload initiate failed: {err}");
}

var initiateJson = await initiateResponse.Content.ReadAsStringAsync().ConfigureAwait(false);
using var initiateDoc = JsonDocument.Parse(initiateJson);
var uploadId = initiateDoc.RootElement.GetProperty("uploadId").GetString();
var status = initiateDoc.RootElement.GetProperty("status").GetString();

if (status == "UPLOADABLE")
{
    // Step 2: PUT file bytes to pre-signed URL (NO auth header — it's S3)
    var presignedUrl = initiateDoc.RootElement.GetProperty("contents")[0].GetProperty("url").GetString();

    using var uploadClient = new HttpClient(); // separate client without auth headers
    var putContent = new ByteArrayContent(fileBytes);
    putContent.Headers.ContentType = new System.Net.Http.Headers.MediaTypeHeaderValue("application/octet-stream");
    var putResponse = await uploadClient.PutAsync(presignedUrl, putContent).ConfigureAwait(false);

    if (putResponse.IsSuccessStatusCode is false)
    {
        throw new Exception("Failed to upload file content to cloud storage.");
    }

    // Step 3: Confirm completion
    var completeUrl = $"{apiBaseUrl}/files/fs/upload?uploadId={uploadId}&wait=true";
    var completeResponse = await client.GetAsync(completeUrl).ConfigureAwait(false);
    var completeJson = await completeResponse.Content.ReadAsStringAsync().ConfigureAwait(false);
    using var completeDoc = JsonDocument.Parse(completeJson);

    var fileId = completeDoc.RootElement.GetProperty("fileId").GetString();
    var versionId = completeDoc.RootElement.GetProperty("versionId").GetString();
}
else if (status == "DUPLICATE")
{
    // File with same content already exists — fileId available in response
    var fileId = initiateDoc.RootElement.GetProperty("fileId").GetString();
}
```

### Upload response schema

| Property | Type | Description |
|----------|------|-------------|
| `uploadId` | string | Upload session identifier |
| `status` | string | `"UPLOADABLE"`, `"AVAILABLE"`, `"DUPLICATE"` |
| `fileId` | string | File identifier (after completion) |
| `versionId` | string | Version identifier (after completion) |
| `parentId` | string | Parent folder identifier |
| `contents` | array | Upload targets: `[{url, format}]` |

### Download file (2-step)

The SDK uses a 2-step download with pre-signed URLs:

```
GET files/fs/{fileId}/downloadurl
Query: versionId={versionId}  (optional), format={format}  (optional)
```

```csharp
// Step 1: Get pre-signed download URL
var downloadUrlEndpoint = $"{apiBaseUrl}/files/fs/{fileId}/downloadurl";
var urlResponse = await client.GetAsync(downloadUrlEndpoint).ConfigureAwait(false);
var urlJson = await urlResponse.Content.ReadAsStringAsync().ConfigureAwait(false);
using var urlDoc = JsonDocument.Parse(urlJson);
var presignedUrl = urlDoc.RootElement.GetProperty("url").GetString();

// Step 2: Download from pre-signed URL (no auth needed)
using var downloadClient = new HttpClient();
var fileBytes = await downloadClient.GetByteArrayAsync(presignedUrl).ConfigureAwait(false);
System.IO.File.WriteAllBytes(outputPath, fileBytes);
```

### Rename or move file

```
PATCH files/{fileId}
Header: If-Match: "{fileVersionId}"
Body: { "name": "...", "parentId": "..." }   (both optional)
```

```csharp
var url = $"{apiBaseUrl}/files/{fileId}";
var payload = new { name = "new-name.pdf" };
var content = new StringContent(JsonSerializer.Serialize(payload), System.Text.Encoding.UTF8, "application/json");
var request = new HttpRequestMessage(new HttpMethod("PATCH"), url) { Content = content };
request.Headers.Add("If-Match", $"\"{fileVersionId}\"");
var response = await client.SendAsync(request).ConfigureAwait(false);
```

### Delete file

```
DELETE files/{fileVersionId}
```

> **IMPORTANT:** Uses the file's VERSION id, not the file id.

```csharp
var url = $"{apiBaseUrl}/files/{fileVersionId}";
var response = await client.DeleteAsync(url).ConfigureAwait(false);
```

### File object schema

| Property | Type | Description |
|----------|------|-------------|
| `id` | string | Unique file identifier |
| `versionId` | string | Current version identifier |
| `name` | string | File name with extension |
| `parentId` | string | Parent folder identifier |
| `parentType` | string | Parent type (e.g., `"FOLDER"`) |
| `type` | string | Always `"FILE"` |
| `size` | long | File size in bytes |
| `hash` | string | MD5 content hash |
| `status` | string | `"AVAILABLE"`, `"UPLOADABLE"`, `"UPLOADED"`, `"FINALIZING"` |
| `revision` | long | Revision number |
| `thumbnailUrl` | string | Thumbnail image URL |
| `hasChildren` | bool | Whether file has child items |
| `createdOn` | string (ISO 8601) | Upload timestamp |
| `modifiedOn` | string (ISO 8601) | Last modification timestamp |
| `createdBy` | object | Uploader `{id, email, firstName, lastName}` |
| `modifiedBy` | object | Last modifier `{id, email, firstName, lastName}` |
| `checkedOutBy` | object | Person who checked out (null if not checked out) |
| `checkedOutOn` | string (ISO 8601) | Checkout timestamp (null if not checked out) |
| `projectId` | string | Project identifier |
| `permission` | string | Access level for current user |

---

## File versioning

### List file versions

```
GET files/{fileId}/versions
```

```csharp
var url = $"{apiBaseUrl}/files/{fileId}/versions";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

foreach (var version in doc.RootElement.EnumerateArray())
{
    var versionId = version.GetProperty("versionId").GetString();
    var size = version.TryGetProperty("size", out var s) ? s.GetInt64() : 0;
    var createdOn = version.TryGetProperty("createdOn", out var co) ? co.GetString() : null;
}
```

### Download specific version

```csharp
var url = $"{apiBaseUrl}/files/fs/{fileId}/downloadurl?versionId={versionId}";
var urlResponse = await client.GetAsync(url).ConfigureAwait(false);
// Then download from pre-signed URL as shown above
```

---

## Common patterns

### Browse folder tree recursively

```csharp
async Task<List<Dictionary<string, object>>> BrowseFolderTree(
    HttpClient client, string apiBaseUrl, string folderId, string path = "/")
{
    var results = new List<Dictionary<string, object>>();
    var url = $"{apiBaseUrl}/folders/{folderId}/items";
    var response = await client.GetAsync(url).ConfigureAwait(false);

    if (response.IsSuccessStatusCode is false)
        return results;

    var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
    using var doc = JsonDocument.Parse(json);

    foreach (var item in doc.RootElement.EnumerateArray())
    {
        var id = item.GetProperty("id").GetString();
        var name = item.GetProperty("name").GetString();
        var type = item.GetProperty("type").GetString();
        var fullPath = $"{path}{name}";

        results.Add(new Dictionary<string, object>
        {
            ["id"] = id, ["name"] = name, ["type"] = type, ["path"] = fullPath
        });

        if (type == "FOLDER")
        {
            var children = await BrowseFolderTree(
                client, apiBaseUrl, id, $"{fullPath}/").ConfigureAwait(false);
            results.AddRange(children);
        }
    }

    return results;
}
```

### Find file by name

```csharp
var url = $"{apiBaseUrl}/folders/{folderId}/items";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

string matchedFileId = null;
foreach (var item in doc.RootElement.EnumerateArray())
{
    var name = item.GetProperty("name").GetString();
    var type = item.GetProperty("type").GetString();

    if (type == "FILE" && string.Equals(name, targetFileName, StringComparison.OrdinalIgnoreCase))
    {
        matchedFileId = item.GetProperty("id").GetString();
        break;
    }
}
```

### Upload and link to BCF topic

```csharp
// 1. Upload file using the 3-step package upload (see above)
// ... get fileId from upload completion ...

// 2. Create document reference on the topic (BCF API uses a different base URL)
var refUrl = $"{bcfBaseUrl}/projects/{projectId}/topics/{topicId}/document_references";
var refPayload = new { document_guid = fileId, description = "Clash detection report" };
var refContent = new StringContent(
    JsonSerializer.Serialize(refPayload),
    System.Text.Encoding.UTF8, "application/json");
var refResponse = await client.PostAsync(refUrl, refContent).ConfigureAwait(false);
```

---

## File status values

| Status | Description |
|--------|-------------|
| `AVAILABLE` | File is ready to download |
| `UPLOADABLE` | File needs content upload |
| `UPLOADED` | Content uploaded, processing |
| `FINALIZING` | Processing in progress |
| `UPLOAD_FAILED` | Upload error |
| `DELETING` | Marked for deletion |

## Common file types in AEC projects

| Extension | Content type | Description |
|-----------|-------------|-------------|
| `.ifc` | `application/ifc` | IFC Building Model |
| `.ifczip` | `application/ifczip` | Compressed IFC |
| `.pdf` | `application/pdf` | Documents, drawings |
| `.dwg` | `application/dwg` | AutoCAD drawing |
| `.rvt` | `application/rvt` | Revit model |
| `.nwd` / `.nwc` | `application/navisworks` | Navisworks model |
| `.xlsx` | `application/vnd.openxmlformats-officedocument.spreadsheetml.sheet` | Excel spreadsheet |
| `.bcf` | `application/bcf` | BCF issue file |
