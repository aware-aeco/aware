# `google-workspace.tasks.complete`

Mark a Google Task complete.

## When to use

Close out an action once the underlying clash / RFI is resolved — the write side of the task loop opened by `tasks.create`.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `task-id` | string | yes | — | Task to complete. |
| `tasklist-id` | string | no | `@default` | List the task belongs to. |

## Output

```yaml
task-id: string
```

## Worked example

```yaml
- id: close-action
  agent: google-workspace
  command: tasks.complete
  inputs:
    task-id: "{{ open-action.task-id }}"
  safety:
    transaction-group: track
    snapshot: false
```

## Implementation note

Calls `PATCH tasks.patch` (Tasks API) setting `status: completed` (the API stamps `completed` with the timestamp). Write scope: `tasks`. Back off on HTTP 429 / 403 `rateLimitExceeded`.

## See also

- [`tasks.create`](./tasks.create.md)
- [`tasks.list`](./tasks.list.md)
- [`chat.post-message`](./chat.post-message.md)
