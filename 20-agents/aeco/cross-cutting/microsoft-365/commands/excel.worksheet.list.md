# `microsoft-365.excel.worksheet.list`

List the worksheets in an Excel workbook on OneDrive / SharePoint.

## When to use

When the sheet isn't known ahead of time — discover the tabs before reading, or confirm a
"Takedown" sheet exists. Pair with `excel.range.read`, which takes a sheet's name or its id.

Read-only: AWARE sends only a `GET` and never changes the workbook. Work or school accounts only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `drive-id` | string | yes | OneDrive / SharePoint drive id, or `me` for the signed-in person's own OneDrive. |
| `item-id` | string | yes | The workbook's item id. |
| `page-size` | int | no | Worksheets to return, 1 to 200 (default 100). |

## Output

```yaml
worksheets:
  - id:         "{00000000-0001-0000-0000-000000000000}"
    name:       "DoorSchedule"
    position:   0
    visibility: "Visible"
  - id:         "{00000000-0002-0000-0000-000000000000}"
    name:       "Takedown"
    position:   1
    visibility: "Hidden"
more-available: false   # Graph has another page, or entries were cut at page-size
omitted: 0              # sheets dropped because Graph sent a malformed id
```

The `id` is stable across renames and can be passed to `excel.range.read` as `worksheet` unchanged;
the `name` is what the person sees on the tab. Hidden sheets are listed too, with their
`visibility`.

## Worked example

```yaml
- id: sheets
  agent: microsoft-365
  command: excel.worksheet.list
  inputs:
    drive-id: me
    item-id:  "{{ inputs.workbook-id }}"
- id: header
  agent: microsoft-365
  command: excel.range.read
  inputs:
    drive-id:  me
    item-id:   "{{ inputs.workbook-id }}"
    worksheet: "{{ sheets.worksheets[0].id }}"
    range:     "A1:Z1"
```

## Implementation note

Runs in-process through AWARE's code-owned Graph read (#709):
`GET /drives/{drive-id}/items/{item-id}/workbook/worksheets` (or under `/me/drive` for
`drive-id: me`) with a fixed `$select` of id, name, position and visibility. A `@odata.nextLink` is
reported, never followed. Microsoft documents only write-class permissions for workbook reads, so
this needs `User.Read` and one of `Files.ReadWrite`, `Files.ReadWrite.All`; connect with
`aware connect microsoft-365 --oauth --scopes User.Read,Files.ReadWrite`. None needs tenant admin
consent.

## See also

- `excel.range.read` — read a range once the sheet is known
- `get-shared-item` — a workbook's ids from its sharing link
