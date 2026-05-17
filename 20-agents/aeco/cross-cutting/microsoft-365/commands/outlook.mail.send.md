# `microsoft-365.outlook.mail.send`

Send an email via Outlook / Exchange Online.

## When to use

Whenever the recipient is *not* a Teams user (external consultants, clients, regulators). For shared-mailbox sends, the user needs `Mail.Send.Shared` granted. Most apps mix Teams (internal) + Outlook (external) — use both side by side.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `to` | array<string> | yes | — | Primary recipients. |
| `cc` | array<string> | no | `[]` | |
| `bcc` | array<string> | no | `[]` | |
| `subject` | string | yes | — | |
| `body` | string | yes | — | Plain text or HTML. |
| `content-type` | enum | no | `text` | `text` / `html`. |
| `from` | string | no | authenticated user | Shared-mailbox address (requires extra scope). |

## Worked example

```yaml
- id: notify-client
  agent: microsoft-365
  command: outlook.mail.send
  inputs:
    to: ["client@acme-corp.com"]
    cc: ["pm@acme-arch.com"]
    subject: "Issued for Tender — {{ project.name }} — {{ run.date }}"
    content-type: html
    body: |
      <p>Dear Acme Corp,</p>
      <p>Please find the issued-for-tender drawing set at the link below.</p>
      <p>Total sheets: {{ tender-pdfs.written.length }}.</p>
  safety:
    transaction-group: notify
    snapshot: false
```

## Implementation note

Calls `POST /me/sendMail` (or `/users/{from}/sendMail` if `from` is supplied + scoped). Body is wrapped in the Graph `message` shape; attachments use `outlook.mail.send-with-attachment` instead.

## See also

- `outlook.mail.send-with-attachment` — for PDFs / Excel / drawings
- `outlook.calendar.create-event` — turn it into a meeting
- `teams.channel.post-message` — internal equivalent
