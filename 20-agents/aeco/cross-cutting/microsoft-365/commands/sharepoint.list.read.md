# `microsoft-365.sharepoint.list.read`

Read one page of a SharePoint list's rows.

## When to use

The "pull the project roster" / "read the door-spec list" primitive — get structured rows a team
keeps in SharePoint into a workflow.

Read-only. Work or school accounts only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `site-id` | string | yes | `root` or `host,site-guid,web-guid`. |
| `list-id` | string | yes | The list's GUID, from `sharepoint.list.list`. |
| `page-size` | int | no | Rows to return, 1 to 200 (default 100). |

List titles are not accepted, only the GUID. Filtering rows is not offered yet.

## Output

```yaml
items:
  - id: "2"
    fields:
      Title: "D-101"
      FireRating: "60 min"
      Quantity: 4
      AuthorLookupId: "12"
    skipped-fields: 0
more-available: false   # Graph has another page, or entries were cut at page-size
omitted: 0              # rows dropped because Graph sent a malformed id
```

Columns are keyed by SharePoint's **internal** column names (a display name "Fire rating" may be
`FireRating` or `Fire_x0020_rating`). Person and lookup columns come back as their `…LookupId`
values, as Graph returns them by default — not resolved names.

Each value is a string, number, true/false, empty, or a list of up to 50 of those. A text value
over 2,048 characters is cut and its column named in `truncated-fields`. Anything AWARE does not
pass on (an object value, a longer list, an unusable column name, columns beyond 200) is counted in
`skipped-fields` — never dropped silently. A row Graph sent without fields has
`fields-missing: true`.

## Implementation note

Runs in-process through AWARE's code-owned Graph read (#709):
`GET /sites/{site-id}/lists/{list-id}/items?$expand=fields`. Graph does not document `$top` here, so
the page is bounded by AWARE; an oversized answer fails with `E_CALL_OUTPUT_TOO_LARGE` rather than
being cut. A `@odata.nextLink` is reported, never followed. Needs `User.Read` and one of
`Sites.Read.All`, `Sites.ReadWrite.All`; connect with
`aware connect microsoft-365 --oauth --scopes User.Read,Sites.Read.All`. None needs tenant admin
consent.
