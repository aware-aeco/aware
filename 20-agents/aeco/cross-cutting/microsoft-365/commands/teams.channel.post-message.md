# `microsoft-365.teams.channel.post-message`

Post a plain message to a Teams channel.

## When to use

The bread-and-butter "Monday standup digest" command. Use after any AECO query that wants to land in front of humans — sheet status reports, RFI rollups, clash digests, steel BoM deltas.

**WRITE-mode** (external state — Teams). Apps using this command should declare `safety:` on the node if they want rollback (e.g. delete the message on app failure). For a pure notification, declaring `safety: { transaction-group: notify, snapshot: false }` is honest — there's nothing to snapshot.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `team-id` | string | yes | — | Team id OR team display name. The transport resolves names to IDs via `/teams?$filter=displayName eq '...'`. |
| `channel-id` | string | yes | — | Channel id OR channel display name. |
| `text` | string | yes | — | Message body. Plain text or HTML. |
| `content-type` | enum | no | `text` | `text` / `html`. |

## Output

```yaml
message-id: "1715958331234"
web-url:    "https://teams.microsoft.com/l/message/19:abc.../1715958331234?..."
```

## Worked example

```yaml
- id: digest
  agent: microsoft-365
  command: teams.channel.post-message
  inputs:
    team-id: "Project Acme"
    channel-id: "coordination"
    content-type: html
    text: "<h3>Monday clash digest</h3><p>14 unresolved clashes — see ACC.</p>"
  safety:
    transaction-group: notify
    snapshot: false  # nothing to snapshot for a notification
```

## Implementation note

Calls `POST /teams/{team-id}/channels/{channel-id}/messages`. Microsoft Graph rate-limits to ~4 channel posts/sec per app — the transport stays under by default.

## See also

- `teams.channel.post-with-card` — adaptive cards with rich data
- `teams.channel.post-with-screenshot` — attach an image
- `outlook.mail.send` — same idea via email
