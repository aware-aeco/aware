# `microsoft-365.planner.task.create`

Create a Planner task.

## When to use

When the app's output is "somebody needs to do X." Architect ball-in-court tracking, RFI follow-ups, training assignments, post-meeting actions. Pair with `planner.task.assign` if the assignment isn't known at creation time.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `plan-id` | string | yes | The plan to add to. |
| `bucket-id` | string | no | Optional bucket (column). |
| `title` | string | yes | |
| `assigned-to` | array<string> | no | User principal names or AAD object ids. |
| `due-date` | string | no | ISO 8601. |
| `notes` | string | no | Markdown body of the task. |

## Output

```yaml
task-id: "abc123..."
web-url: "https://tasks.office.com/.../Home/Task/abc123..."
```

## Worked example

```yaml
- id: open-rfis
  agent: microsoft-365
  command: outlook.mail.search
  inputs:
    query: "subject:RFI category:Open"
- id: create-followups
  agent: microsoft-365
  command: planner.task.create
  inputs:
    plan-id: "{{ secrets.coordination-plan-id }}"
    title: "Follow up: {{ open-rfis.messages[0].subject }}"
    assigned-to: ["{{ open-rfis.messages[0].from }}"]
    due-date: "{{ today | plus: '3d' }}"
    notes: |
      Auto-created from RFI email.
      Source: {{ open-rfis.messages[0].web-url }}
  safety:
    transaction-group: notify
    snapshot: false
```

## Implementation note

Calls `POST /planner/tasks`. The `assigned-to` array is translated into the Planner `assignments` map shape with each assignee's `orderHint`. Bucket assignment uses `bucketId` directly.

## See also

- `planner.task.list` — query before creating to avoid duplicates
- `planner.task.assign` — change assignees later
- `planner.task.complete` — mark done
