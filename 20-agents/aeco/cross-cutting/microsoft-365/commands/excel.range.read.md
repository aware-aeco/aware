# `microsoft-365.excel.range.read`

Read one rectangle of cells from an Excel workbook on OneDrive / SharePoint.

## When to use

When the source of truth for some structured data lives in Excel — fire ratings keyed by mark, a
load take-down, parameter values for a bulk write, a schedule of values for a take-off. Find the
workbook with `list-folder` or `get-shared-item`, and the sheet with `excel.worksheet.list`.

Read-only: AWARE sends only a `GET`, with no body, no workbook session and no `Prefer` header, and
issues no request that changes the workbook. Work or school accounts only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `drive-id` | string | yes | OneDrive / SharePoint drive id, or `me` for the signed-in person's own OneDrive. |
| `item-id` | string | yes | The workbook's item id. |
| `worksheet` | string | yes | The sheet's name (`Load Table`) or its `{…}` id from `excel.worksheet.list`. |
| `range` | string | yes | A1 notation, e.g. `A2:D500` or `B7`; at most 5,000 cells. |

`worksheet` is either a braced id exactly as `excel.worksheet.list` returns it, or a name Excel
would accept: 1 to 31 characters, none of `\ / ? * [ ] : '`, no leading or trailing space. `range`
is one cell or one rectangle, with no `$` and no sheet name (the sheet is `worksheet`); either
corner may come first. A range of more than 5,000 cells, a whole column (`A:A`) or a list of ranges
is refused before any request — split a larger table into several reads.

## Output

```yaml
address:      "'Load Table'!A1:C3"
row-count:    3
column-count: 3
values:
  - ["Mark", "Load (kN)", "Checked"]
  - ["B1", 125.5, true]
  - ["B2", 46021, false]       # a date is its serial number here…
text:
  - ["Mark", "Load (kN)", "Checked"]
  - ["B1", "125.50", "TRUE"]
  - ["B2", "2025-12-29", "FALSE"]  # …and its formatted form here
value-types:
  - ["String", "String", "String"]
  - ["String", "Double", "Boolean"]
  - ["String", "Double", "Boolean"]   # whole numbers and dates are Double too
truncated-cells: []            # [row, column] of cut cells, zero-based within the range
truncated-cells-omitted: 0
```

`values`, `text` and `value-types` are always exactly `row-count` rows of `column-count` cells, in
the order of the sheet. Values are as Excel holds them: numbers stay numbers, `true`/`false` stay
booleans, an empty cell is `""`, an error cell is its error text (`#DIV/0!`) with type `Error`. A
date or time is Excel's serial number in `values` — read `text` for what the person sees, in their regional format (`125,50` in a Polish locale). Text is
passed on exactly, line breaks and all; the only change AWARE ever makes is to cut a cell over
2,048 characters, and every cut cell is listed in `truncated-cells` (the first 100; the rest are
counted in `truncated-cells-omitted`).

An answer that is not the rectangle asked for, or whose cells are not values, text and types
Microsoft documents, is refused as a malformed response — never returned as a partial table.

## Worked example

```yaml
- id: sheets
  agent: microsoft-365
  command: excel.worksheet.list
  inputs:
    drive-id: me
    item-id:  "{{ inputs.workbook-id }}"
- id: loads
  agent: microsoft-365
  command: excel.range.read
  inputs:
    drive-id:  me
    item-id:   "{{ inputs.workbook-id }}"
    worksheet: "{{ sheets.worksheets[0].id }}"
    range:     "A1:D200"
```

## Implementation note

Runs in-process through AWARE's code-owned Graph read (#709):
`GET /drives/{drive-id}/items/{item-id}/workbook/worksheets/{worksheet}/range(address='{range}')`
(or under `/me/drive` for `drive-id: me`) with a fixed `$select` of the address, counts, values,
text and value types. The 5,000-cell bound is checked before the request; the answer is also held to
the call's output budget, so a rectangle of very long text fails with `E_CALL_OUTPUT_TOO_LARGE`
rather than being cut.

Microsoft documents only write-class permissions for workbook reads, so this needs `User.Read` and
one of `Files.ReadWrite`, `Files.ReadWrite.All`; connect with
`aware connect microsoft-365 --oauth --scopes User.Read,Files.ReadWrite`. None needs tenant admin
consent. The grant would let a program change your files; AWARE's guarantee is about the requests it
sends, which are reads only. Microsoft says calls without a session keep any change they make; a
`GET` makes none.

## See also

- `excel.worksheet.list` — the sheets of a workbook
- `get-shared-item` — a workbook's ids from its sharing link
- `excel.range.write` — write a range (planned)
