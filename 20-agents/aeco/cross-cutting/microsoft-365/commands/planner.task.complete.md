# `microsoft-365.planner.task.complete`

Mark a Planner task complete (`percentComplete = 100`).

## When to use

When an app closes the loop on its own work — auto-complete the "publish issued set" task once the upload succeeds, or close an RFI follow-up when the response email arrives. Pairs with `planner.task.list` to find the task and `outlook.mail.search` to detect the closing condition.

**WRITE-mode** (external state — Planner). Declare `safety:` on the node; `transaction-group: notify, snapshot: false` is honest.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `task-id` | string | yes | The task to mark complete. |

## Output

```yaml
task-id: "abc123..."
```

## Worked example

```yaml
- id: publish
  agent: microsoft-365
  command: upload-file
  inputs:
    drive-id:  "{{ secrets.acme-drive-id }}"
    folder-id: "{{ secrets.issued-set-folder-id }}"
    filename:  "{{ project.name }}-IFC.pdf"
    bytes:     "{{ export.pdf }}"
  safety: { transaction-group: notify, snapshot: false }
- id: close-task
  agent: microsoft-365
  command: planner.task.complete
  inputs:
    task-id: "{{ secrets.publish-task-id }}"
  safety:
    transaction-group: notify
    snapshot: false
```

## Implementation note

Calls `PATCH /planner/tasks/{task-id}` with body `{ "percentComplete": 100 }`. The transport first GETs the task to read its `@odata.etag` and passes it as the required `If-Match` header (Planner rejects writes without a current etag). Requires `Group.ReadWrite.All` (or `Tasks.ReadWrite`); provision via `aware connect microsoft-365`. On HTTP 429 the transport honors the `Retry-After` header.

## See also

- `planner.task.list` — find the task id
- `planner.task.create` — the opposite end of the lifecycle
- `planner.task.assign` — change ball-in-court
