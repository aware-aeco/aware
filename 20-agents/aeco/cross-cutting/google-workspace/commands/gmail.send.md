# `google-workspace.gmail.send`

Irreversibly submit one email through Gmail as the authenticated mailbox.

## Availability and safety

This is the only runnable Google Workspace command in agent version `2.0.0`.
It requires AWARE CLI `0.136.0` or newer. Every other command in the agent is
`planned`.

The command is write-mode and declares `rollback: unsupported`. Gmail accepting
the request does not prove delivery, inbox placement, or that a recipient read
the message. A sent email cannot be recalled by AWARE.

The sender is resolved from the authenticated Google account. Callers cannot
provide or override a `from` address. Attachments are deliberately unavailable
in this contract; arbitrary local paths are not accepted by the mail primitive.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `to` | array<string> | yes | At least one recipient address. |
| `cc` | array<string> | no | Carbon-copy recipients. |
| `bcc` | array<string> | no | Blind-copy recipients. Gmail receives this in the submitted MIME message and removes it from recipient-delivered copies. |
| `subject` | string | yes | Message subject. |
| `body` | string | yes | Plain text or HTML body. |
| `content-type` | enum | no | `text` (default) or `html`. |
| `attempt-id` | string | yes | Caller-stable identifier for this logical send. |

Derive `attempt-id` from stable business identity and action, not a timestamp or
random value generated immediately before each retry. For example, FloLess RFI
001's email action uses `floless-rfi-001-email-v1`.

## Accepted output

```json
{
  "status": "accepted",
  "message-id": "18f7c4a5b6d78901",
  "gmail-message-id": "18f7c4a5b6d78901",
  "thread-id": "18f7c49ab0123456",
  "rfc-message-id": "<aware.floless-rfi-001-email-v1@example.invalid>",
  "attempt-id": "floless-rfi-001-email-v1"
}
```

`message-id` is retained for FloLess #1090 compatibility and is exactly the
same Gmail Message resource ID as `gmail-message-id`. It is not the RFC 5322
`Message-ID` header; that distinct value is `rfc-message-id`. `thread-id` is the
Gmail thread resource ID. `status: accepted` means Gmail returned a usable
resource ID, not that the message was delivered.

## Attempt and retry contract

Before dispatch, AWARE durably records the `attempt-id`, canonical request hash,
and generated `rfc-message-id` in the authenticated account's outbox journal.

- Reusing an accepted attempt with identical inputs returns the cached accepted
  output without sending again.
- Reusing an attempt with different inputs is a conflict.
- An attempt left `dispatching` or `outcome-unknown` is refused. Reconcile it in
  Gmail using the returned attempt and RFC message identifiers.
- AWARE never automatically retries a Gmail send.

Validation, authentication, and sender-identity errors are pre-dispatch and do
not send mail. A definitive provider 4xx rejection is `gmail.send.rejected`.
HTTP 408/5xx, redirects, transport/read/parse failures, cancellation or timeout
after handoff, and a success response without a usable Gmail ID are
`gmail.send.outcome-unknown` with `retryable: false`. The latter may already have
sent the email; do not create a new attempt until reconciliation proves it safe.

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
    attempt-id: "{{ project.id }}-tender-email-v1"
  safety:
    transaction-group: notify
    snapshot: false
```

The runtime constructs a standards-compliant RFC 5322 MIME message and calls the
compile-pinned HTTPS endpoint `POST /gmail/v1/users/me/messages/send`. Production
endpoint overrides and redirects are rejected. Authentication requires exactly
`openid`, `userinfo.email`, and `gmail.send`; see [`../skills/auth.md`](../skills/auth.md).
