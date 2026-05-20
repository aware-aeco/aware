# `microsoft-365.outlook.mail.send-with-attachment`

Send an email with one or more file attachments.

## When to use

The external-deliverable primitive — issue a PDF set to a client, send an Excel take-off to a consultant, forward a marked-up drawing to a regulator. Use `outlook.mail.send` when there's nothing to attach. Each attachment ≤ 4 MB simple, ≤ 150 MB resumable; the transport picks the mode automatically.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `to` | array<string> | yes | — | Primary recipients. |
| `subject` | string | yes | — | |
| `body` | string | yes | — | Plain text or HTML. |
| `content-type` | enum | no | `text` | `text` / `html`. |
| `attachments` | array<object> | yes | — | Each `{ path, filename }`. |

## Output

```yaml
message-id: "AAMkAGI2..."
```

## Worked example

```yaml
- id: issue-set
  agent: microsoft-365
  command: outlook.mail.send-with-attachment
  inputs:
    to: ["client@acme-corp.com"]
    subject: "Issued for Construction — {{ project.name }} — {{ run.date }}"
    content-type: html
    body: |
      <p>Dear Acme Corp,</p>
      <p>Please find the IFC drawing set attached.</p>
    attachments:
      - path: "{{ export.pdf-path }}"
        filename: "{{ project.name }}-IFC.pdf"
  safety:
    transaction-group: notify
    snapshot: false
```

## Implementation note

For small payloads the transport calls `POST /me/sendMail` with each file as an inline `fileAttachment` (base64). When an attachment exceeds 4 MB it first creates a draft (`POST /me/messages`), uploads the file via an attachment upload session, then sends (`POST /me/messages/{id}/send`). Requires `Mail.Send`; provision via `aware connect microsoft-365`. On HTTP 429 the transport honors the `Retry-After` header.

## See also

- `outlook.mail.send` — no attachments
- `download-file` — pull a library file to attach
- `teams.channel.post-with-screenshot` — image into a channel instead
