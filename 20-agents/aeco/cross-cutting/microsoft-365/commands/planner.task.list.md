# `microsoft-365.planner.task.list`

List tasks in a Planner plan, optionally filtered.

## When to use

When you need the current state of work — open ball-in-court items, what's assigned to a person, what's overdue. Query before `planner.task.create` to avoid duplicates, or drive a Teams digest of open actions.

Read-only.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `plan-id` | string | yes | — | The plan to read. |
| `bucket-id` | string | no | — | Optional bucket (column) filter. |
| `assigned-to` | string | no | — | Optional user filter (UPN or AAD object id). |
| `completed` | enum | no | `any` | `any` / `completed-only` / `open-only`. |

## Output

```yaml
tasks:
  - task-id:          "abc123..."
    title:            "Follow up: RFI-042"
    assigned-to:      ["consultant@acme-mep.com"]
    due-date:         "2026-05-22T00:00:00Z"
    percent-complete: 0
```

## Worked example

```yaml
- id: open-actions
  agent: microsoft-365
  command: planner.task.list
  inputs:
    plan-id: "{{ secrets.coordination-plan-id }}"
    completed: open-only
- id: digest
  agent: microsoft-365
  command: teams.channel.post-with-card
  inputs:
    team-id: "Project Acme"
    channel-id: "coordination"
    title: "Open actions — {{ now.date }}"
    rows: "{{ open-actions.tasks }}"
  safety:
    transaction-group: notify
    snapshot: false
```

## Implementation note

Calls `GET /planner/plans/{plan-id}/tasks`; bucket / assignee / completion filters are applied client-side over the page set (`@odata.nextLink` followed automatically). Reading Planner data requires `Group.ReadWrite.All` (or `Tasks.Read` / `Tasks.ReadWrite`); provision via `aware connect microsoft-365`. On HTTP 429 the transport honors the `Retry-After` header.

## See also

- `planner.task.create` — add a task
- `planner.task.assign` — change ball-in-court
- `planner.task.complete` — mark done
