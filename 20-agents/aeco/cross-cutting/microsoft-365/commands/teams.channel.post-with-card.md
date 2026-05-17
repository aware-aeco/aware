# `microsoft-365.teams.channel.post-with-card`

Post an Adaptive Card to a Teams channel.

## When to use

When the data is *structured* — a table of sheets, a list of RFIs, a status rollup. Adaptive Cards render the rows as a FactSet, with optional action buttons. The right primitive for the BIM-manager Monday model audit + architect ball-in-court reports + engineer peer-review delta cards.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `team-id` | string | yes | Team id or display name. |
| `channel-id` | string | yes | Channel id or display name. |
| `title` | string | yes | Card title (rendered as a TextBlock at the top). |
| `rows` | array<object> | no | Generic rows — transport renders as TextBlock + FactSet. |
| `card` | object | no | Raw Adaptive Card v1.5 JSON. If provided, `rows` is ignored. |

## Worked example

```yaml
- id: rfi-rollup
  agent: microsoft-365
  command: teams.channel.post-with-card
  inputs:
    team-id: "Project Acme"
    channel-id: "coordination"
    title: "Open RFIs > 5 days — {{ now.date }}"
    rows:
      - rfi: "RFI-042"
        ball: "MEP consultant"
        age-days: 9
      - rfi: "RFI-051"
        ball: "Structural"
        age-days: 7
  safety:
    transaction-group: notify
    snapshot: false
```

## Implementation note

`rows` are rendered as `{ "type": "TextBlock", ... }` + `{ "type": "FactSet", ... }` blocks under the title. For action buttons or more complex layouts, pass raw `card` JSON. The Adaptive Card schema is at https://adaptivecards.io.

## See also

- `teams.channel.post-message` — plain text
- `teams.channel.post-with-screenshot` — attach an image
