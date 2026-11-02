# `microsoft-365.sharepoint.list.list`

List a SharePoint site's lists.

## When to use

Find a list's id before reading its rows with `sharepoint.list.read`.

Read-only. Work or school accounts only. Hidden system lists are not included.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `site-id` | string | yes | `root` (the organisation's root site) or `host,site-guid,web-guid`. |
| `page-size` | int | no | Lists to return, 1 to 200 (default 100). |

A site id is either `root` or exactly a host name, then two GUIDs, comma-separated
(`acme.sharepoint.com,2C712604-…,2D2244C3-…`). Anything else is refused before any request.

## Output

```yaml
lists:
  - id: "243bca4b-4e5e-45af-b37d-25f6135a740d"
    name: "Door schedule"
    web-url: "https://acme.sharepoint.com/Lists/Door%20schedule"
    template: "genericList"
    modified-time: "2026-10-28T14:02:00Z"
more-available: false   # Graph has another page, or entries were cut at page-size
omitted: 0              # lists dropped because Graph sent a malformed id
```

## Implementation note

Runs in-process through AWARE's code-owned Graph read (#709): `GET /sites/{site-id}/lists` with a
fixed `$select`. Graph does not document `$top` here, so the page is bounded by AWARE. A
`@odata.nextLink` is reported, never followed. Needs `User.Read` and one of `Sites.Read.All`,
`Sites.ReadWrite.All`; connect with `aware connect microsoft-365 --oauth --scopes User.Read,Sites.Read.All`.
None needs tenant admin consent.
