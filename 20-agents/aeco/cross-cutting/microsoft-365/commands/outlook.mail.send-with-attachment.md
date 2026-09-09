# `microsoft-365.outlook.mail.send-with-attachment`

Send an email with one or more file attachments.

> **Status: planned / unavailable.** No runnable Microsoft 365 transport is
> shipped for this command. Apps that reference it are rejected during
> validation / compile until a real transport and truthful receipt contract
> exist.

## When to use

The external-deliverable primitive — issue a PDF set to a client, send an Excel take-off to a consultant, forward a marked-up drawing to a regulator. Use `outlook.mail.send` when there's nothing to attach. A future transport may use a simple request for attachments under 3 MB; attachments from 3–150 MB require an upload session.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `to` | array<string> | yes | — | Primary recipients. |
| `subject` | string | yes | — | |
| `body` | string | yes | — | Plain text or HTML. |
| `content-type` | enum | no | `text` | `text` / `html`. |
| `attachments` | array<object> | yes | — | Each `{ path, filename }`. |

## Worked example

This is an authoring example for the planned interface; it is not runnable yet.

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

A future transport may send payloads under 3 MB via `POST /me/sendMail` with each file
as an inline `fileAttachment` (base64). For larger attachments it may create a
draft (`POST /me/messages`), upload via an attachment upload session, then call
`POST /me/messages/{id}/send`. The final send succeeds with `202 Accepted` and no
response body. That proves only that Graph accepted the request for processing;
it does not prove delivery and supplies no message id.

The transport must preserve the ambiguity boundary around the final send. If the
request may have reached Graph but the response is lost, times out, or cannot be
durably recorded, the result is indeterminate. It must not automatically retry,
because doing so can send the email twice. The command remains `planned` until a
real transport implements and documents a truthful acceptance receipt. It will
require `Mail.Send`; provision via `aware connect microsoft-365`.

## See also

- `outlook.mail.send` — no attachments
- `download-file` — pull a library file to attach
- `teams.channel.post-with-screenshot` — image into a channel instead
