---
parent: google-auth
description: Reusable code snippets for Google API. HttpClient setup, Bearer token, read Google Sheets cells, list Drive files, upload file, JSON parsing.
---

## Reusable Code Snippets

### HttpClient Setup with Bearer Token

```csharp
var accessToken = inputs["googleAccessToken"].ToString();
var client = (HttpClient)inputs["httpClient"];
client.DefaultRequestHeaders.Authorization =
    new System.Net.Http.Headers.AuthenticationHeaderValue("Bearer", accessToken);
```

### Read Google Sheets Cell

```csharp
var spreadsheetId = inputs["spreadsheetId"].ToString();
var sheetName = inputs["sheetName"].ToString();
var cellAddress = inputs["cellAddress"].ToString();

var range = Uri.EscapeDataString($"{sheetName}!{cellAddress}");
var url = $"https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values/{range}";

var response = await client.GetAsync(url).ConfigureAwait(false);
response.EnsureSuccessStatusCode();
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

string cellValue = "";
if (doc.RootElement.TryGetProperty("values", out var values)
    && values.GetArrayLength() > 0
    && values[0].GetArrayLength() > 0)
{
    cellValue = values[0][0].ToString();
}
```

### Write Google Sheets Range

```csharp
var range = Uri.EscapeDataString($"{sheetName}!A1:B2");
var url = $"https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values/{range}" +
          "?valueInputOption=USER_ENTERED";

var body = new { values = new object[][] {
    new object[] { "Header1", "Header2" },
    new object[] { "Value1", "Value2" }
}};

var content = new StringContent(
    JsonSerializer.Serialize(body),
    System.Text.Encoding.UTF8,
    "application/json");

var response = await client.PutAsync(url, content).ConfigureAwait(false);
response.EnsureSuccessStatusCode();
```

### List Google Drive Files

```csharp
var folderId = inputs.ContainsKey("folderId") ? inputs["folderId"].ToString() : "root";
var url = $"https://www.googleapis.com/drive/v3/files" +
          $"?q='{folderId}'+in+parents+and+trashed=false" +
          "&fields=files(id,name,mimeType,size,modifiedTime)";

var response = await client.GetAsync(url).ConfigureAwait(false);
response.EnsureSuccessStatusCode();
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

var files = new List<Dictionary<string, object>>();
foreach (var item in doc.RootElement.GetProperty("files").EnumerateArray())
{
    files.Add(new Dictionary<string, object>
    {
        ["id"] = item.GetProperty("id").GetString(),
        ["name"] = item.GetProperty("name").GetString(),
        ["mimeType"] = item.GetProperty("mimeType").GetString()
    });
}
```
