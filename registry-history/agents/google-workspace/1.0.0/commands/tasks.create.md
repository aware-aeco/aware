# `google-workspace.tasks.create`

Create a Google Task. Optional notes + due-date.

## When to use

Turn an audit finding, an open RFI, or a clash into a tracked action item. Pairs with `gmail.search` / `forms.responses.list` to fan inbound items into a task list.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `tasklist-id` | string | no | `@default` | Target task list. |
| `title` | string | yes | — | Task title. |
| `notes` | string | no | — | Free-text detail. |
| `due-date` | string | no | — | ISO 8601 (date-only; Tasks ignores time). |

## Output

```yaml
task-id: string
```

## Worked example

```yaml
- id: open-action
  agent: google-workspace
  command: tasks.create
  inputs:
    title: "Resolve clash at Grid C/4"
    notes: "Beam vs duct — see Monday digest."
    due-date: "2026-05-23"
  safety:
    transaction-group: track
    snapshot: false
```

## Implementation note

Calls `POST tasks.insert` (Tasks API, `https://www.googleapis.com/tasks/v1/`) against the given task list. `due` accepts RFC 3339 but only the date component is honoured. Write scope: `tasks`. Back off on HTTP 429 / 403 `rateLimitExceeded`.

## See also

- [`tasks.list`](./tasks.list.md)
- [`tasks.complete`](./tasks.complete.md)
- [`forms.responses.list`](./forms.responses.list.md)
