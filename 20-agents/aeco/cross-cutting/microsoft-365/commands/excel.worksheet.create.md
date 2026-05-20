# `microsoft-365.excel.worksheet.create`

Add a new worksheet to an Excel workbook.

## When to use

When a run needs a fresh tab to write into — a `Phase 1` / `Phase 2` sheet per export, a dated `QA-2026-05-20` log, a per-run takedown. Create the sheet, then `excel.range.write` (or `excel.table.append-row`) into it.

If the sheet might already exist, list first with `excel.worksheet.list` and skip the create — Graph returns HTTP 409 `ItemAlreadyExists` when the name collides.

Write-mode.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `drive-id` | string | yes | OneDrive / SharePoint drive id. |
| `item-id` | string | yes | Workbook file id. |
| `name` | string | no | New worksheet name. Omit to let Graph auto-name (`Sheet1`, `Sheet2`, …). Max 31 chars; the characters `\ / ? * [ ]` are rejected, as is a leading/trailing apostrophe. |

## Output

```yaml
id:       "{00000000-0003-...}"
name:     "Phase 1"
position: 2
```

`id` is stable across renames; `name` is not. `position` is the zero-based tab index of the new sheet.

## Worked example

Create a per-run phase sheet, then write the takedown into it:

```yaml
- id: sheet
  agent: microsoft-365
  command: excel.worksheet.create
  inputs:
    drive-id: "{{ secrets.acme-drive-id }}"
    item-id:  "{{ secrets.bom-workbook-id }}"
    name:     "Phase {{ inputs.phase }}"
  safety:
    writes: "a new worksheet in the BOM workbook"
- id: write-bom
  agent: microsoft-365
  command: excel.range.write
  inputs:
    drive-id:  "{{ secrets.acme-drive-id }}"
    item-id:   "{{ secrets.bom-workbook-id }}"
    worksheet: "{{ sheet.name }}"
    range:     "A1:D200"
    values:    "{{ takedown.values }}"
  safety:
    writes: "the BOM rows into the new sheet"
```

## Implementation note

Calls `POST /drives/{drive-id}/items/{item-id}/workbook/worksheets/add` with body `{ "name": "<name>" }` (the body is omitted entirely when `name` is unset, so Graph auto-names). Returns the created worksheet's `id`, `name`, and `position`. Requires `Files.ReadWrite` (or `Files.ReadWrite.All` for shared drives); provision via `aware connect microsoft-365`. On HTTP 429 the transport honors the `Retry-After` header; on 409 the sheet name already exists.

## See also

- `excel.worksheet.list` — discover existing sheets (call first to avoid a 409)
- `excel.range.write` — write a 2D array into the new sheet
- `excel.table.append-row` — append to a named table instead of a raw range
