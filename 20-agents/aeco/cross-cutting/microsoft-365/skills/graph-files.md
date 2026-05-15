---
name: microsoft-365-graph-files
description: Microsoft Graph API — SharePoint sites, OneDrive drives, file upload, file download, folder creation, file listing, sharing URL resolution, driveId, itemId, cloud storage, document management. This skill should be used when working with SharePoint files, OneDrive files, cloud folders, sharing links, or document operations.
---

# Microsoft Graph API — Files & SharePoint Reference

## Critical Patterns

- All file operations require a valid access token via `inputs["m365AccessToken"]`
- Sites, drives, and items are identified by IDs — resolve paths to IDs first
- Folder contents are accessed via `/drive/items/{itemId}/children`
- Large file uploads (>4MB) require upload sessions — use simple PUT for small files
- Item paths use colon syntax: `/drive/root:/path/to/file.txt:`
- Always URL-encode file/folder names containing special characters
- SharePoint document libraries are accessed as drives on a site
- SharePoint/OneDrive sharing URLs must be resolved via `/shares` endpoint before use

## API Endpoints

### Sites
```
GET /sites?search={query}                    — Search for SharePoint sites
GET /sites/{siteId}                          — Get site by ID
GET /sites/{hostname}:/{path}                — Get site by hostname and path
```

### Drives
```
GET /sites/{siteId}/drives                   — List drives (document libraries) on a site
GET /drives/{driveId}                        — Get drive by ID
GET /me/drive                                — Current user's OneDrive
```

### Files & Folders
```
GET /drives/{driveId}/root/children          — List root folder contents
GET /drives/{driveId}/items/{itemId}/children — List folder contents
GET /drives/{driveId}/root:/{path}:/children — List by path
GET /drives/{driveId}/items/{itemId}         — Get item metadata
GET /drives/{driveId}/items/{itemId}/content — Download file content
PUT /drives/{driveId}/items/{itemId}/content — Upload/replace file (< 4MB)
POST /drives/{driveId}/items/{parentId}/children — Create folder
DELETE /drives/{driveId}/items/{itemId}      — Delete file or folder
PATCH /drives/{driveId}/items/{itemId}       — Update item (rename, move)
POST /drives/{driveId}/items/{itemId}/copy   — Copy item
```

## Code Examples

### List Sites
```csharp
var url = "https://graph.microsoft.com/v1.0/sites?search=*";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

var sites = new List<Dictionary<string, object>>();
foreach (var site in doc.RootElement.GetProperty("value").EnumerateArray())
{
    sites.Add(new Dictionary<string, object>
    {
        ["id"] = site.GetProperty("id").GetString(),
        ["name"] = site.GetProperty("displayName").GetString(),
        ["webUrl"] = site.GetProperty("webUrl").GetString()
    });
}
```

### List Drives on a Site
```csharp
var url = $"https://graph.microsoft.com/v1.0/sites/{siteId}/drives";
var response = await client.GetAsync(url).ConfigureAwait(false);
// Parse same pattern as sites — value array with id, name, webUrl
```

### List Files in a Folder
```csharp
var url = $"https://graph.microsoft.com/v1.0/drives/{driveId}/items/{folderId}/children";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

var files = new List<Dictionary<string, object>>();
foreach (var item in doc.RootElement.GetProperty("value").EnumerateArray())
{
    var isFolder = item.TryGetProperty("folder", out _);
    files.Add(new Dictionary<string, object>
    {
        ["id"] = item.GetProperty("id").GetString(),
        ["name"] = item.GetProperty("name").GetString(),
        ["size"] = item.TryGetProperty("size", out var sz) ? sz.GetInt64() : 0L,
        ["isFolder"] = isFolder,
        ["lastModified"] = item.GetProperty("lastModifiedDateTime").GetString()
    });
}
```

### Download File
```csharp
var url = $"https://graph.microsoft.com/v1.0/drives/{driveId}/items/{itemId}/content";
var response = await client.GetAsync(url).ConfigureAwait(false);
var bytes = await response.Content.ReadAsByteArrayAsync().ConfigureAwait(false);
// Or for text: var text = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
```

### Upload File (< 4MB)
```csharp
var url = $"https://graph.microsoft.com/v1.0/drives/{driveId}/items/{parentId}:/{fileName}:/content";
var content = new ByteArrayContent(fileBytes);
content.Headers.ContentType = new System.Net.Http.Headers.MediaTypeHeaderValue("application/octet-stream");
var response = await client.PutAsync(url, content).ConfigureAwait(false);
```

### Create Folder
```csharp
var url = $"https://graph.microsoft.com/v1.0/drives/{driveId}/items/{parentId}/children";
var body = JsonSerializer.Serialize(new
{
    name = folderName,
    folder = new { },
    @microsoft.graph.conflictBehavior = "rename"
});
var content = new StringContent(body, System.Text.Encoding.UTF8, "application/json");
var response = await client.PostAsync(url, content).ConfigureAwait(false);
```

### Copy Item
```csharp
var url = $"https://graph.microsoft.com/v1.0/drives/{driveId}/items/{itemId}/copy";
var body = JsonSerializer.Serialize(new
{
    parentReference = new { driveId = targetDriveId, id = targetFolderId },
    name = newName
});
var content = new StringContent(body, System.Text.Encoding.UTF8, "application/json");
var response = await client.PostAsync(url, content).ConfigureAwait(false);
// Copy returns 202 Accepted with Location header for monitoring
```

### Move / Rename Item
```csharp
var url = $"https://graph.microsoft.com/v1.0/drives/{driveId}/items/{itemId}";
var body = JsonSerializer.Serialize(new
{
    parentReference = new { id = newParentId }, // move
    name = newName                               // rename
});
var content = new StringContent(body, System.Text.Encoding.UTF8, "application/json");
var request = new HttpRequestMessage(new HttpMethod("PATCH"), url) { Content = content };
var response = await client.SendAsync(request).ConfigureAwait(false);
```

## Sharing URL Resolution

When a user provides a SharePoint or OneDrive sharing URL (e.g., `https://contoso-my.sharepoint.com/:x:/g/personal/.../EQC...?e=abc` or a `1drv.ms` short link), you must resolve it to a `driveId` + `itemId` before calling any file or Excel API.

**Detection**: Sharing URLs contain `sharepoint.com/:` (with a single-letter type indicator like `:x:`, `:w:`, `:p:`) or use the `1drv.ms` short-link domain.

### EncodeSharingUrl Helper

```csharp
static string EncodeSharingUrl(string sharingUrl)
{
    var base64 = Convert.ToBase64String(System.Text.Encoding.UTF8.GetBytes(sharingUrl));
    return "u!" + base64.TrimEnd('=').Replace('/', '_').Replace('+', '-');
}
```

### Resolve Sharing URL to DriveItem

```csharp
var encoded = EncodeSharingUrl(sharingUrl);
var url = $"https://graph.microsoft.com/v1.0/shares/{encoded}/driveItem";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

var driveId = doc.RootElement.GetProperty("parentReference").GetProperty("driveId").GetString();
var itemId = doc.RootElement.GetProperty("id").GetString();
```

### End-to-End Example: Resolve Sharing URL then Read Excel Range

```csharp
// 1. Encode and resolve the sharing URL
var encoded = EncodeSharingUrl(sharingUrl);
var resolveUrl = $"https://graph.microsoft.com/v1.0/shares/{encoded}/driveItem";
var resolveResponse = await client.GetAsync(resolveUrl).ConfigureAwait(false);
var resolveJson = await resolveResponse.Content.ReadAsStringAsync().ConfigureAwait(false);
using var resolveDoc = JsonDocument.Parse(resolveJson);

var driveId = resolveDoc.RootElement.GetProperty("parentReference").GetProperty("driveId").GetString();
var itemId = resolveDoc.RootElement.GetProperty("id").GetString();

// 2. Use driveId + itemId for Excel operations
var excelUrl = $"https://graph.microsoft.com/v1.0/drives/{driveId}/items/{itemId}" +
               $"/workbook/worksheets/Sheet1/range(address='A1:C10')";
var excelResponse = await client.GetAsync(excelUrl).ConfigureAwait(false);
var excelJson = await excelResponse.Content.ReadAsStringAsync().ConfigureAwait(false);
using var excelDoc = JsonDocument.Parse(excelJson);

var values = excelDoc.RootElement.GetProperty("values");
```

## DriveItem Properties

| Property | Type | Description |
|----------|------|-------------|
| id | string | Unique item identifier |
| name | string | File or folder name |
| size | long | File size in bytes |
| webUrl | string | Browser-accessible URL |
| lastModifiedDateTime | string | ISO 8601 timestamp |
| createdDateTime | string | ISO 8601 timestamp |
| folder | object | Present if item is a folder (contains childCount) |
| file | object | Present if item is a file (contains mimeType) |
| parentReference | object | Contains driveId, id, path of parent |
