# `microsoft-365.outlook.mail.search`

Search the user's inbox with a KQL keyword query.

## When to use

When the trigger lives in email — pull every open RFI thread, find all messages from a client since a date, gather submittal responses. Feeds downstream actions like `planner.task.create` (one follow-up per hit). Examples: `subject:"RFI"`, `from:client@acme.com received:>2026-04-01`.

Read-only.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `query` | string | yes | — | KQL keyword query (Microsoft's query language). |
| `top` | number | no | `25` | Max messages to return. |

## Output

```yaml
messages:
  - id:          "AAMkAGI2..."
    subject:     "RFI-042 — curtain wall head detail"
    from:        "consultant@acme-mep.com"
    received-at: "2026-05-18T09:14:00Z"
    web-url:     "https://outlook.office365.com/owa/?ItemID=..."
```

## Worked example

```yaml
- id: open-rfis
  agent: microsoft-365
  command: outlook.mail.search
  inputs:
    query: 'subject:"RFI" received:>2026-04-01'
    top: 50
- id: followups
  agent: microsoft-365
  command: planner.task.create
  inputs:
    plan-id: "{{ secrets.coordination-plan-id }}"
    title: "Follow up: {{ open-rfis.messages[0].subject }}"
    assigned-to: ["{{ open-rfis.messages[0].from }}"]
  safety:
    transaction-group: notify
    snapshot: false
```

## Implementation note

Calls `GET /me/messages?$search="{query}"` (the `$search` parameter accepts KQL). Results are unsortable while `$search` is active (Graph constraint), so the transport returns them in relevance order and caps at `top`. Requires `Mail.Read`; provision via `aware connect microsoft-365`. On HTTP 429 the transport honors the `Retry-After` header.

## See also

- `outlook.mail.send` — reply / act on a hit
- `planner.task.create` — turn a hit into a follow-up
- `sharepoint.list.read` — the list-side query equivalent
