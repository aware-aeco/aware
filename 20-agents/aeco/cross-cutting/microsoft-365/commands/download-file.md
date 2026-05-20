# `microsoft-365.download-file`

Download the bytes of a OneDrive / SharePoint file.

## When to use

When an app needs the actual file content — a published PDF set to re-attach to an email, an IFC to feed a downstream agent, a reference drawing to bundle into a report. Resolve the `item-id` first with `get-drive-item` if you only have a link.

Read-only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `drive-id` | string | yes | OneDrive / SharePoint drive id. |
| `item-id` | string | yes | The file's item id. |

## Output

```yaml
bytes:    "<binary>"
filename: "IFR-drawing-set.pdf"
size-kb:  2841
```

## Worked example

```yaml
- id: fetch-set
  agent: microsoft-365
  command: download-file
  inputs:
    drive-id: "{{ secrets.acme-drive-id }}"
    item-id:  "{{ resolve.id }}"
- id: forward
  agent: microsoft-365
  command: outlook.mail.send-with-attachment
  inputs:
    to: ["client@acme-corp.com"]
    subject: "Issued set — {{ project.name }}"
    body: "Set attached."
    attachments:
      - path: "{{ fetch-set.bytes }}"
        filename: "{{ fetch-set.filename }}"
```

## Implementation note

Calls `GET /drives/{drive-id}/items/{item-id}/content`, which 302-redirects to a short-lived pre-authenticated download URL; the transport follows it and streams the bytes. Requires `Files.Read.All` (or `Files.ReadWrite.All`). On HTTP 429 the transport honors the `Retry-After` header.

## See also

- `get-drive-item` — resolve an id / sharing URL first
- `upload-file` — the write-side counterpart
- `outlook.mail.send-with-attachment` — re-send what you downloaded
