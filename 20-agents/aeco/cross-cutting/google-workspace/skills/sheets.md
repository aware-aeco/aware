---
name: google-workspace-sheets
description: Google Sheets API — read cells, write cells, update range, get worksheets, append rows, batch read, batch update, spreadsheet, gsheet, table data, report, A1 notation, cell values, formulas. This skill should be used when working with Google Spreadsheets, reading or writing spreadsheet data, or creating spreadsheet-based reports.
---

# Google Sheets API — Spreadsheet Operations

## Critical Patterns

- All Sheets operations require a valid access token via `inputs["googleAccessToken"]`
- Spreadsheets are identified by spreadsheet ID (from the URL)
- Use A1 notation for ranges: "Sheet1!A1:C10", "A1", "Sheet1!A:A"
- Values are sent/received as 2D arrays (array of rows, each row is array of cells)
- Use `valueInputOption=USER_ENTERED` for writes (interprets formulas and numbers)
- Use `valueInputOption=RAW` for writes that should be stored as-is
- Empty cells return empty string ""
- Use `values.append` to add rows after existing data

## API Endpoints

### Get Spreadsheet Info
```
GET https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}
    ?fields=spreadsheetId,properties.title,sheets.properties
```

### Read Values
```
GET https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values/{range}
```

### Write Values
```
PUT https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values/{range}
    ?valueInputOption=USER_ENTERED
    Body: { "values": [[...], [...]] }
```

### Append Values
```
POST https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values/{range}:append
    ?valueInputOption=USER_ENTERED&insertDataOption=INSERT_ROWS
    Body: { "values": [[...], [...]] }
```

### Batch Get Values
```
GET https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values:batchGet
    ?ranges=Sheet1!A1:B5&ranges=Sheet2!A1:C10
```

### Batch Update Values
```
POST https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values:batchUpdate
    Body: { "valueInputOption": "USER_ENTERED", "data": [{ "range": "...", "values": [...] }, ...] }
```

## Code Examples

### List Worksheets
```csharp
var url = $"https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}" +
          "?fields=sheets.properties";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

var sheets = new List<Dictionary<string, object>>();
foreach (var sheet in doc.RootElement.GetProperty("sheets").EnumerateArray())
{
    var props = sheet.GetProperty("properties");
    sheets.Add(new Dictionary<string, object>
    {
        ["sheetId"] = props.GetProperty("sheetId").GetInt32(),
        ["title"] = props.GetProperty("title").GetString(),
        ["index"] = props.GetProperty("index").GetInt32()
    });
}
```

### Read Cell Value
```csharp
var range = Uri.EscapeDataString($"{sheetName}!{cellAddress}");
var url = $"https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values/{range}";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

// values is a 2D array — single cell = [["value"]]
string cellValue = "";
if (doc.RootElement.TryGetProperty("values", out var values)
    && values.GetArrayLength() > 0
    && values[0].GetArrayLength() > 0)
{
    cellValue = values[0][0].ToString();
}
```

### Read Range
```csharp
var range = Uri.EscapeDataString($"{sheetName}!{rangeAddress}");
var url = $"https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values/{range}";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

var rows = new List<List<string>>();
if (doc.RootElement.TryGetProperty("values", out var values))
{
    foreach (var row in values.EnumerateArray())
    {
        var cells = new List<string>();
        foreach (var cell in row.EnumerateArray())
        {
            cells.Add(cell.ToString());
        }
        rows.Add(cells);
    }
}
```

### Write Cell Value
```csharp
var range = Uri.EscapeDataString($"{sheetName}!{cellAddress}");
var url = $"https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values/{range}" +
          "?valueInputOption=USER_ENTERED";
var body = JsonSerializer.Serialize(new
{
    values = new object[][] { new object[] { cellValue } }
});
var content = new StringContent(body, System.Text.Encoding.UTF8, "application/json");
var response = await client.PutAsync(url, content).ConfigureAwait(false);
```

### Write Range
```csharp
var range = Uri.EscapeDataString($"{sheetName}!{rangeAddress}");
var url = $"https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values/{range}" +
          "?valueInputOption=USER_ENTERED";
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
var response = await client.PutAsync(url, content).ConfigureAwait(false);
```

### Append Rows
```csharp
var range = Uri.EscapeDataString($"{sheetName}!A1");
var url = $"https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values/{range}:append" +
          "?valueInputOption=USER_ENTERED&insertDataOption=INSERT_ROWS";
var body = JsonSerializer.Serialize(new
{
    values = new object[][]
    {
        new object[] { "New Row 1", 42, "2024-01-15" },
        new object[] { "New Row 2", 99, "2024-01-16" }
    }
});
var content = new StringContent(body, System.Text.Encoding.UTF8, "application/json");
var response = await client.PostAsync(url, content).ConfigureAwait(false);
```

### Batch Get Multiple Ranges
```csharp
var ranges = "ranges=Sheet1!A1:B5&ranges=Sheet2!A1:C10";
var url = $"https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values:batchGet?{ranges}";
var response = await client.GetAsync(url).ConfigureAwait(false);
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

foreach (var valueRange in doc.RootElement.GetProperty("valueRanges").EnumerateArray())
{
    var rangeAddr = valueRange.GetProperty("range").GetString();
    if (valueRange.TryGetProperty("values", out var vals))
    {
        // Process vals as 2D array
    }
}
```

## Range Notation

| Example | Description |
|---------|-------------|
| `Sheet1!A1` | Single cell |
| `Sheet1!A1:C10` | Rectangular range |
| `Sheet1!A:A` | Entire column A |
| `Sheet1!1:1` | Entire row 1 |
| `A1:C10` | Range on first visible sheet |
| `Sheet1` | All data on Sheet1 |

## ValueInputOption

| Option | Behavior |
|--------|----------|
| `USER_ENTERED` | Parses input as if typed by user (formulas, numbers, dates interpreted) |
| `RAW` | Stores input as-is (no parsing — everything becomes a string) |

## Response Properties (values.get)

| Property | Type | Description |
|----------|------|-------------|
| range | string | The range that was read (e.g., "Sheet1!A1:C3") |
| majorDimension | string | "ROWS" or "COLUMNS" |
| values | array[][] | 2D array of cell values |
