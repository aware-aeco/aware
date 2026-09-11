# `google-workspace.gmail.send`

Send an email via Gmail.

## When to use

External recipients in Google-shop firms — clients, consultants, regulators. Same role as `microsoft-365.outlook.mail.send`.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `to` | array<string> | yes | |
| `cc` | array<string> | no | |
| `bcc` | array<string> | no | |
| `subject` | string | yes | |
| `body` | string | yes | |
| `content-type` | enum | no | `text` (default) / `html`. |
| `attachments` | array | no | `{ path, filename }` per attachment. |

## Worked example

```yaml
- id: notify-client
  agent: google-workspace
  command: gmail.send
  inputs:
    to: ["client@acme-corp.com"]
    subject: "Issued for Tender — {{ project.name }}"
    content-type: html
    body: "<p>Please find drawings at the link below.</p>"
  safety:
    transaction-group: notify
    snapshot: false
```

## Implementation note

Calls `users.messages.send` with a RFC 5322-formatted base64-encoded raw message. Requires `gmail.send` OAuth scope.
