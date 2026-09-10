---
name: google-workspace-drive
description: Google Drive API — list files, search files, download, upload, create folder, copy, move, delete, share, file metadata, MIME types, file export, Google Docs, Google Slides. This skill should be used when working with Google Drive files, folders, cloud storage, or document management.
---

# Google Drive API — Files & Folders Reference

## Critical Patterns

- All Drive operations require a valid access token via `inputs["googleAccessToken"]`
- Files and folders are identified by IDs — use Files.list to discover IDs
- Folders have mimeType `application/vnd.google-apps.folder`
- Use query syntax for filtering: `'folderId' in parents and trashed=false`
- Download files via `?alt=media` parameter
- Upload uses multipart or simple upload depending on size
- Always URL-encode query parameters
- Google Drive has no concept of "paths" — use parent folder IDs instead

## API Endpoints

### List Files
```
GET https://www.googleapis.com/drive/v3/files
    ?q={query}&fields=files(id,name,mimeType,size,modifiedTime,parents)
    &orderBy=name&pageSize=100
```

### Get File Metadata
```
GET https://www.googleapis.com/drive/v3/files/{fileId}
    ?fields=id,name,mimeType,size,modifiedTime,webViewLink,parents
```

### Download File
```
GET https://www.googleapis.com/drive/v3/files/{fileId}?alt=media
```

### Upload File (Simple — < 5MB)
```
POST https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart
    Content-Type: multipart/related
```

### Create Folder
```
POST https://www.googleapis.com/drive/v3/files
    Body: { "name": "folderName", "mimeType": "application/vnd.google-apps.folder", "parents": ["parentId"] }
```

### Copy File
```
POST https://www.googleapis.com/drive/v3/files/{fileId}/copy
    Body: { "name": "newName", "parents": ["targetFolderId"] }
```

### Move File (Update Parents)
```
PATCH https://www.googleapis.com/drive/v3/files/{fileId}
    ?addParents={newParentId}&removeParents={oldParentId}
```

### Delete File
```
DELETE https://www.googleapis.com/drive/v3/files/{fileId}
```

## Code Examples

### List Files in a Folder
```csharp
var folderId = "your-folder-id"; // or "root" for root folder
var query = Uri.EscapeDataString($"'{folderId}' in parents and trashed=false");
var url = $"https://www.googleapis.com/drive/v3/files?q={query}" +
          "&fields=files(id,name,mimeType,size,modifiedTime)&orderBy=name&pageSize=100";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

var files = new List<Dictionary<string, object>>();
foreach (var file in doc.RootElement.GetProperty("files").EnumerateArray())
{
    files.Add(new Dictionary<string, object>
    {
        ["id"] = file.GetProperty("id").GetString(),
        ["name"] = file.GetProperty("name").GetString(),
        ["mimeType"] = file.GetProperty("mimeType").GetString(),
        ["size"] = file.TryGetProperty("size", out var sz) ? sz.GetString() : "0",
        ["modifiedTime"] = file.TryGetProperty("modifiedTime", out var mt) ? mt.GetString() : ""
    });
}
```

### Download File Content
```csharp
var url = $"https://www.googleapis.com/drive/v3/files/{fileId}?alt=media";
var response = await client.GetAsync(url).ConfigureAwait(false);
var bytes = await response.Content.ReadAsByteArrayAsync().ConfigureAwait(false);
// Or for text: var text = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
```

### Upload File (Simple Multipart)
```csharp
var metadata = JsonSerializer.Serialize(new
{
    name = fileName,
    parents = new[] { parentFolderId }
});

var metadataContent = new StringContent(metadata, System.Text.Encoding.UTF8, "application/json");
var fileContent = new ByteArrayContent(fileBytes);
fileContent.Headers.ContentType = new System.Net.Http.Headers.MediaTypeHeaderValue("application/octet-stream");

var multipart = new MultipartContent("related");
multipart.Add(metadataContent);
multipart.Add(fileContent);

var url = "https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart";
var response = await client.PostAsync(url, multipart).ConfigureAwait(false);
```

### Create Folder
```csharp
var body = JsonSerializer.Serialize(new
{
    name = folderName,
    mimeType = "application/vnd.google-apps.folder",
    parents = new[] { parentFolderId }
});
var content = new StringContent(body, System.Text.Encoding.UTF8, "application/json");
var url = "https://www.googleapis.com/drive/v3/files";
var response = await client.PostAsync(url, content).ConfigureAwait(false);
```

### Copy File
```csharp
var body = JsonSerializer.Serialize(new
{
    name = newName,
    parents = new[] { targetFolderId }
});
var content = new StringContent(body, System.Text.Encoding.UTF8, "application/json");
var url = $"https://www.googleapis.com/drive/v3/files/{fileId}/copy";
var response = await client.PostAsync(url, content).ConfigureAwait(false);
```

### Move File
```csharp
var url = $"https://www.googleapis.com/drive/v3/files/{fileId}" +
          $"?addParents={newParentId}&removeParents={oldParentId}";
var request = new HttpRequestMessage(new HttpMethod("PATCH"), url);
var response = await client.SendAsync(request).ConfigureAwait(false);
```

### Delete File
```csharp
var url = $"https://www.googleapis.com/drive/v3/files/{fileId}";
var response = await client.DeleteAsync(url).ConfigureAwait(false);
```

## Query Syntax

| Example | Description |
|---------|-------------|
| `'folderId' in parents` | Files in a specific folder |
| `trashed=false` | Exclude trashed files |
| `mimeType='application/vnd.google-apps.folder'` | Only folders |
| `mimeType='application/vnd.google-apps.spreadsheet'` | Only spreadsheets |
| `name contains 'report'` | Name contains substring |
| `modifiedTime > '2024-01-01T00:00:00'` | Modified after date |

Combine with `and`: `'root' in parents and trashed=false and mimeType='application/vnd.google-apps.folder'`

## File Properties

| Property | Type | Description |
|----------|------|-------------|
| id | string | Unique file identifier |
| name | string | File or folder name |
| mimeType | string | MIME type (folders: `application/vnd.google-apps.folder`) |
| size | string | File size in bytes (not present for folders) |
| modifiedTime | string | ISO 8601 timestamp |
| createdTime | string | ISO 8601 timestamp |
| parents | string[] | Parent folder IDs |
| webViewLink | string | Browser-accessible URL |
