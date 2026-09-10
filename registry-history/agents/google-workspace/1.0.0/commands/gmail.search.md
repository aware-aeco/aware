# `google-workspace.gmail.search`

Search the user's mailbox using Gmail's `q` search syntax.

## When to use

Find RFI threads, client replies, or transmittal confirmations to feed downstream steps — `from:client@acme.com newer_than:7d`, `subject:RFI`. The triage primitive for mail-driven flows.

**READ-mode**.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `query` | string | yes | — | Gmail `q` search expression. |
| `max-results` | number | no | `25` | Cap on returned messages. |

## Output

```yaml
messages:
  - id:          string
    thread-id:   string
    subject:     string
    from:        string
    received-at: string
```

## Worked example

```yaml
- id: find-rfi-mail
  agent: google-workspace
  command: gmail.search
  inputs:
    query: "subject:RFI newer_than:7d"
    max-results: 50
```

## Implementation note

Calls `GET users.messages.list` with the `q` parameter; subject / from / date are read from each message's headers (a follow-up `messages.get` with `format=metadata`). Read scope: `gmail.readonly`. Back off on HTTP 429 / 403 `rateLimitExceeded`.

## See also

- [`gmail.send`](./gmail.send.md)
- [`tasks.create`](./tasks.create.md)
- [`forms.responses.list`](./forms.responses.list.md)
