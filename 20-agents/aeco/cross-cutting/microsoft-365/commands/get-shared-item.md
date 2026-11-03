# `microsoft-365.get-shared-item`

Open a OneDrive / SharePoint file or folder by the sharing link someone sent you.

## When to use

A colleague pasted a link to the door schedule in an email or a Teams chat, and the workflow needs
the file itself. This turns the link into the same metadata `get-drive-item` returns, including the
`id` and `drive-id` that `excel.range.read` and `excel.worksheet.list` take.

Read-only. Work or school accounts only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `sharing-url` | string | yes | The https sharing link, as copied from OneDrive or SharePoint. |

The link must be `https://` on a `*.sharepoint.com` host (OneDrive for work or school links are
`<tenant>-my.sharepoint.com`), at most 2,048 characters, with no spaces or fragment (`#…`).
Anything else — a personal OneDrive (`1drv.ms`, `onedrive.live.com`) link, another website — is
refused before any request.

## Output

```yaml
id:            "01ABC...XYZ"
drive-id:      "b!AbC..."
name:          "DoorSchedule.xlsx"
size:          48213
is-folder:     false
mime-type:     "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
modified-time: "2026-10-01T10:00:00Z"
web-url:       "https://acme.sharepoint.com/.../DoorSchedule.xlsx"
truncated:     ["name"]   # only when a text field was cut to its limit
```

## What it never does

It resolves only a link you can already open. Microsoft can be asked, with a `Prefer:
redeemSharingLink` header, to add you to the item for good when you use the link; AWARE never sends
that header, nor a body, so opening a link through AWARE never changes who has access to the file.

## Implementation note

Runs in-process through AWARE's code-owned Graph read (#709): `GET /shares/{token}/driveItem`,
where the token is `u!` followed by the link in unpadded base64url, as Microsoft documents, with the
same fixed `$select` as `get-drive-item` (never a download URL). Microsoft documents only
write-class permissions for this call, so it needs `User.Read` and one of `Files.ReadWrite`,
`Files.ReadWrite.All`, `Sites.ReadWrite.All`; connect with
`aware connect microsoft-365 --oauth --scopes User.Read,Files.ReadWrite`. None needs tenant admin
consent. AWARE's guarantee is about the request it sends, which is a read only.

## See also

- `get-drive-item` — the same metadata by id
- `excel.worksheet.list`, `excel.range.read` — read the workbook it points at
