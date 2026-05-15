---
parent: m365-graph-auth
description: Reusable code snippets for Microsoft Graph API. HttpClient setup, Bearer token, read Excel cells, list files, download file, SharePoint, OneDrive, JSON parsing.
---

## Reusable Code Snippets

### HttpClient Setup with Bearer Token

```csharp
var accessToken = inputs["m365AccessToken"].ToString();
var client = (HttpClient)inputs["httpClient"];
client.DefaultRequestHeaders.Authorization =
    new System.Net.Http.Headers.AuthenticationHeaderValue("Bearer", accessToken);
```

### Read Excel Cell via Graph API

```csharp
var driveId = inputs["driveId"].ToString();
var itemId = inputs["itemId"].ToString();
var sheetName = inputs["sheetName"].ToString();
var cellAddress = inputs["cellAddress"].ToString();

var url = $"https://graph.microsoft.com/v1.0/drives/{driveId}/items/{itemId}" +
          $"/workbook/worksheets/{sheetName}/range(address='{cellAddress}')";

var response = await client.GetAsync(url).ConfigureAwait(false);
response.EnsureSuccessStatusCode();
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

var values = doc.RootElement.GetProperty("values");
var cellValue = values[0][0].ToString();
```

### Write Excel Range via Graph API

```csharp
var rangeAddress = "A1:B2";
var url = $"https://graph.microsoft.com/v1.0/drives/{driveId}/items/{itemId}" +
          $"/workbook/worksheets/{sheetName}/range(address='{rangeAddress}')";

var body = new { values = new object[][] {
    new object[] { "Header1", "Header2" },
    new object[] { "Value1", "Value2" }
}};

var content = new StringContent(
    JsonSerializer.Serialize(body),
    System.Text.Encoding.UTF8,
    "application/json");

var response = await client.SendAsync(
    new HttpRequestMessage(HttpMethod.Patch, url) { Content = content })
    .ConfigureAwait(false);
response.EnsureSuccessStatusCode();
```

### List SharePoint/OneDrive Files

```csharp
var driveId = inputs["driveId"].ToString();
var folderId = inputs.ContainsKey("folderId") ? inputs["folderId"].ToString() : "root";

var url = $"https://graph.microsoft.com/v1.0/drives/{driveId}/items/{folderId}/children";
var response = await client.GetAsync(url).ConfigureAwait(false);
response.EnsureSuccessStatusCode();
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

var files = new List<Dictionary<string, object>>();
foreach (var item in doc.RootElement.GetProperty("value").EnumerateArray())
{
    files.Add(new Dictionary<string, object>
    {
        ["id"] = item.GetProperty("id").GetString(),
        ["name"] = item.GetProperty("name").GetString(),
        ["size"] = item.GetProperty("size").GetInt64(),
        ["isFolder"] = item.TryGetProperty("folder", out _)
    });
}
```
