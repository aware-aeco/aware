---
name: microsoft-365-graph-excel
description: Microsoft Graph API — Cloud Excel workbook operations. Read cells, write cells, update range, get worksheets, usedRange, workbook sessions, cell values, spreadsheet, cloud Excel, SharePoint Excel, OneDrive Excel. This skill should be used when reading or writing Excel files stored in SharePoint or OneDrive via Microsoft Graph. Do NOT use for local .xlsx files — use ClosedXML instead.
---

# Microsoft Graph API — Excel Workbook Operations

## Critical Patterns

- Excel operations work on files stored in SharePoint/OneDrive (identified by driveId + itemId)
- Workbook sessions can be persistent or non-persistent — prefer non-persistent for simple reads
- Cell addresses use A1 notation: "A1", "B2:D10", "Sheet1!A1:C5"
- Range values are returned as 2D arrays (array of rows, each row is array of cells)
- Write operations send values as 2D arrays in the same format
- Empty cells return empty string "", not null
- Use `createSession` for batching multiple operations on the same workbook
- If the user provides a SharePoint sharing URL instead of driveId + itemId, resolve it first using the `/shares` endpoint (see m365-graph-files skill)
- NEVER read entire columns or rows (A:A, 1:1) — Graph API returns null values for unbounded ranges. Use usedRange first, then read a bounded range.

## API Endpoints

### Worksheets
```
GET /drives/{driveId}/items/{itemId}/workbook/worksheets
    — List all worksheets in workbook

GET /drives/{driveId}/items/{itemId}/workbook/worksheets/{sheetIdOrName}
    — Get specific worksheet
```

### Cells & Ranges
```
GET /drives/{driveId}/items/{itemId}/workbook/worksheets/{sheet}/range(address='{range}')
    — Read range (e.g., address='A1:C10')

PATCH /drives/{driveId}/items/{itemId}/workbook/worksheets/{sheet}/range(address='{range}')
    — Write range values

GET /drives/{driveId}/items/{itemId}/workbook/worksheets/{sheet}/cell(row={r},column={c})
    — Read single cell (0-indexed row and column)
```

### Used Range
```
GET /drives/{driveId}/items/{itemId}/workbook/worksheets/{sheet}/usedRange
    — Get the smallest range encompassing all data on the sheet
    Returns: address, rowCount, columnCount, values[][]
```

### Sessions
```
POST /drives/{driveId}/items/{itemId}/workbook/createSession
    — Create persistent session (for batching)
    Body: { "persistChanges": true }
    Returns: { "id": "session-id" }
    Use header: workbook-session-id: {sessionId}

POST /drives/{driveId}/items/{itemId}/workbook/closeSession
    — Close session
```

## Code Examples

### List Worksheets
```csharp
var url = $"https://graph.microsoft.com/v1.0/drives/{driveId}/items/{itemId}/workbook/worksheets";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

var sheets = new List<Dictionary<string, object>>();
foreach (var sheet in doc.RootElement.GetProperty("value").EnumerateArray())
{
    sheets.Add(new Dictionary<string, object>
    {
        ["id"] = sheet.GetProperty("id").GetString(),
        ["name"] = sheet.GetProperty("name").GetString(),
        ["position"] = sheet.GetProperty("position").GetInt32(),
        ["visibility"] = sheet.GetProperty("visibility").GetString()
    });
}
```

### Read Cell Value
```csharp
var url = $"https://graph.microsoft.com/v1.0/drives/{driveId}/items/{itemId}" +
          $"/workbook/worksheets/{sheetName}/range(address='{cellAddress}')";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

// values is a 2D array — single cell = [[value]]
var values = doc.RootElement.GetProperty("values");
var cellValue = values[0][0].ToString();
```

### Read Range
```csharp
var url = $"https://graph.microsoft.com/v1.0/drives/{driveId}/items/{itemId}" +
          $"/workbook/worksheets/{sheetName}/range(address='{rangeAddress}')";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

var values = doc.RootElement.GetProperty("values");
var rows = new List<List<string>>();
foreach (var row in values.EnumerateArray())
{
    var cells = new List<string>();
    foreach (var cell in row.EnumerateArray())
    {
        cells.Add(cell.ToString());
    }
    rows.Add(cells);
}
```

### Write Cell Value
```csharp
var url = $"https://graph.microsoft.com/v1.0/drives/{driveId}/items/{itemId}" +
          $"/workbook/worksheets/{sheetName}/range(address='{cellAddress}')";
var body = JsonSerializer.Serialize(new
{
    values = new object[][] { new object[] { cellValue } }
});
var content = new StringContent(body, System.Text.Encoding.UTF8, "application/json");
var request = new HttpRequestMessage(new HttpMethod("PATCH"), url) { Content = content };
var response = await client.SendAsync(request).ConfigureAwait(false);
```

### Write Range
```csharp
var url = $"https://graph.microsoft.com/v1.0/drives/{driveId}/items/{itemId}" +
          $"/workbook/worksheets/{sheetName}/range(address='{rangeAddress}')";
var body = JsonSerializer.Serialize(new
{
    values = new object[][]
    {
        new object[] { "Header1", "Header2", "Header3" },
        new object[] { 100, 200, 300 },
        new object[] { "A", "B", "C" }
    }
});
var content = new StringContent(body, System.Text.Encoding.UTF8, "application/json");
var request = new HttpRequestMessage(new HttpMethod("PATCH"), url) { Content = content };
var response = await client.SendAsync(request).ConfigureAwait(false);
```

## Finding Data Extent

> **Warning**: Never use unbounded ranges like `A:A` or `1:1` — Graph API returns null for `values` on these ranges, causing runtime crashes.

### Get Used Range
```csharp
var url = $"https://graph.microsoft.com/v1.0/drives/{driveId}/items/{itemId}" +
          $"/workbook/worksheets/{sheetName}/usedRange";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

var address = doc.RootElement.GetProperty("address").GetString();   // e.g. "Sheet1!A1:D25"
var rowCount = doc.RootElement.GetProperty("rowCount").GetInt32();   // e.g. 25
var columnCount = doc.RootElement.GetProperty("columnCount").GetInt32();
```

### Find First Empty Row (Append Pattern)
```csharp
// Option 1: Use usedRange rowCount to calculate next row
var usedRangeUrl = $"https://graph.microsoft.com/v1.0/drives/{driveId}/items/{itemId}" +
                   $"/workbook/worksheets/{sheetName}/usedRange";
var usedResponse = await client.GetAsync(usedRangeUrl).ConfigureAwait(false);
var usedJson = await usedResponse.Content.ReadAsStringAsync().ConfigureAwait(false);
using var usedDoc = JsonDocument.Parse(usedJson);
var rowCount = usedDoc.RootElement.GetProperty("rowCount").GetInt32();

// Next empty row is rowCount + 1 (1-indexed)
var nextRow = rowCount + 1;
var writeAddress = $"A{nextRow}";

// Option 2: Read a bounded range and find the first empty cell
// Use a reasonable upper bound like A1:A500 instead of A:A
var readUrl = $"https://graph.microsoft.com/v1.0/drives/{driveId}/items/{itemId}" +
              $"/workbook/worksheets/{sheetName}/range(address='A1:A500')";
```

## Working with Sharing URLs

When a user provides a SharePoint/OneDrive sharing URL (containing `sharepoint.com/:` or `1drv.ms`) instead of explicit `driveId` and `itemId` values, you must resolve the sharing URL first.

### EncodeSharingUrl Helper

```csharp
static string EncodeSharingUrl(string sharingUrl)
{
    var base64 = Convert.ToBase64String(System.Text.Encoding.UTF8.GetBytes(sharingUrl));
    return "u!" + base64.TrimEnd('=').Replace('/', '_').Replace('+', '-');
}
```

### Resolve Sharing URL then Write to Excel Cell

```csharp
// 1. Resolve the sharing URL to driveId + itemId
var encoded = EncodeSharingUrl(sharingUrl);
var resolveUrl = $"https://graph.microsoft.com/v1.0/shares/{encoded}/driveItem";
var resolveResponse = await client.GetAsync(resolveUrl).ConfigureAwait(false);
var resolveJson = await resolveResponse.Content.ReadAsStringAsync().ConfigureAwait(false);
using var resolveDoc = JsonDocument.Parse(resolveJson);

var driveId = resolveDoc.RootElement.GetProperty("parentReference").GetProperty("driveId").GetString();
var itemId = resolveDoc.RootElement.GetProperty("id").GetString();

// 2. Write to an Excel cell using the resolved IDs
var url = $"https://graph.microsoft.com/v1.0/drives/{driveId}/items/{itemId}" +
          $"/workbook/worksheets/Sheet1/range(address='A1')";
var body = JsonSerializer.Serialize(new
{
    values = new object[][] { new object[] { "Hello from AWARE" } }
});
var content = new StringContent(body, System.Text.Encoding.UTF8, "application/json");
var request = new HttpRequestMessage(new HttpMethod("PATCH"), url) { Content = content };
var response = await client.SendAsync(request).ConfigureAwait(false);
```

## Range Response Properties

| Property | Type | Description |
|----------|------|-------------|
| address | string | Range address (e.g., "Sheet1!A1:C3") |
| addressLocal | string | Locale-specific address |
| cellCount | int | Number of cells in range |
| columnCount | int | Number of columns |
| rowCount | int | Number of rows |
| values | array[][] | 2D array of cell values |
| text | array[][] | 2D array of formatted text |
| formulas | array[][] | 2D array of formulas |
| numberFormat | array[][] | 2D array of number formats |

## Data Types in Values Array

| JSON Type | Meaning |
|-----------|---------|
| number | Numeric cell value |
| string | Text cell value |
| true/false | Boolean cell value |
| "" (empty string) | Empty cell |
