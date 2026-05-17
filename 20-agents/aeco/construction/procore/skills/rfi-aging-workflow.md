# RFI aging workflow

The architect killer-app the audit named: a weekly Teams card with every RFI open > N days, grouped by ball-in-court.

## Example

```yaml
app: rfi-aging
version: 0.1.0
description: |
  Weekly RFI aging digest — every Monday 7am, list every Procore RFI
  open > 5 days, group by ball-in-court, post to coordination channel.

requires:
  - procore@3.x
  - microsoft-365@1.x

schedule:        # v0.19
  cron: "0 7 * * MON"
  timezone: "Europe/London"

nodes:
  - id: rfis
    agent: procore
    command: rfis.list
    inputs:
      project-id: '{{ secrets.procore-project-id }}'
      status: open

  - id: aging
    inline:
      kind: map
      description: |
        Compute days-open per RFI and filter to > 5 days. Group by
        ball-in-court for the Teams card row order.
      code: |
        rfis => rfis
          .filter(r => r["days-open"] > 5)
          .sort((a,b) => b["days-open"] - a["days-open"])

  - id: post
    agent: microsoft-365
    command: teams.channel.post-with-card
    inputs:
      team-id: 'Project Acme'
      channel-id: 'coordination'
      title: 'RFIs aging > 5 days — {{ run.date }} ({{ aging.length }} open)'
      rows: '{{ aging }}'
    safety:
      transaction-group: notify
      snapshot: false
```

## Threshold tuning

The audit's architect used 5 days as the "stale RFI" threshold. Adjust per project — for construction-phase RFIs that block a trade, 2-3 days is closer. For design-phase clarifications, 7-10 days.

## Response cycle

If the goal is to *close* aging RFIs (not just report them), pair with `procore.rfis.respond`:

```yaml
- id: auto-nudge
  agent: procore
  command: rfis.respond
  inputs:
    rfi-id: '{{ rfis.rfis[0].id }}'
    response: |
      Automated nudge: this RFI has been open for {{ rfis.rfis[0].days-open }} days.
      Ball-in-court: {{ rfis.rfis[0].ball-in-court }}.
      Please respond by close of business today.
  safety:
    transaction-group: notify
    snapshot: false
```

Use sparingly — too many automated nudges in Procore become noise and erode the platform's signal value. The audit's architect specifically called out *manual* digesting + posting to Teams as preferable; automated RFI writes only for chronically-overdue items (e.g. > 14 days).

## Where to store the project-id

In `~/.aware/secrets/procore-project-ids.json`:

```json
{
  "acme-tower":     "12345",
  "acme-school":    "67890",
  "acme-hospital":  "13579"
}
```

App `.flo` files reference these by name via templating: `{{ secrets.procore.acme-tower }}`. Don't hardcode IDs in the `.flo` — they leak when the file is shared.
