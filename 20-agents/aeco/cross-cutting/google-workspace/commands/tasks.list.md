# `google-workspace.tasks.list`

List tasks in a tasklist. Optional show-completed.

## When to use

Read the open-action backlog for a Monday digest or a status sheet. Resolve which items are still outstanding before notifying the team.

**READ-mode**.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `tasklist-id` | string | no | `@default` | Task list to read. |
| `show-completed` | boolean | no | `false` | Include completed tasks. |

## Output

```yaml
tasks:
  - task-id: string
    title:   string
    status:  string
    due:     string
    notes:   string
```

## Worked example

```yaml
- id: open-actions
  agent: google-workspace
  command: tasks.list
  inputs:
    show-completed: false
```

## Implementation note

Calls `GET tasks.list` (Tasks API). `show-completed` maps to the `showCompleted` query parameter; `status` is `needsAction` or `completed`. Read scope: `tasks.readonly` (or broader `tasks`). Back off on HTTP 429 / 403 `rateLimitExceeded`.

## See also

- [`tasks.create`](./tasks.create.md)
- [`tasks.complete`](./tasks.complete.md)
- [`sheets.row.append`](./sheets.row.append.md)
