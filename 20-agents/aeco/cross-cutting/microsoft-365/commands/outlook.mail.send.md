# `microsoft-365.outlook.mail.send`

Send an email via Outlook / Exchange Online.

> **Status: planned / unavailable.** No runnable Microsoft 365 transport is
> shipped for this command. Apps that reference it are rejected during
> validation / compile until a real transport and truthful receipt contract
> exist.

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

This is an authoring example for the planned interface; it is not runnable yet.

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

A future transport may call `POST /me/sendMail` (or `/users/{from}/sendMail` if
`from` is supplied and scoped). Graph reports success as `202 Accepted` with no
response body. That response proves only that Graph accepted the request for
processing: it does not prove delivery and supplies no message id.

The transport must preserve the ambiguity boundary around this side effect. If
the request may have reached Graph but the response is lost, times out, or cannot
be durably recorded, the result is indeterminate. It must not automatically
retry, because doing so can send the email twice. The command remains `planned`
until a real transport implements and documents a truthful acceptance receipt.
Attachments use `outlook.mail.send-with-attachment` instead.

## See also

- `outlook.mail.send-with-attachment` — for PDFs / Excel / drawings
- `outlook.calendar.create-event` — turn it into a meeting
- `teams.channel.post-message` — internal equivalent
