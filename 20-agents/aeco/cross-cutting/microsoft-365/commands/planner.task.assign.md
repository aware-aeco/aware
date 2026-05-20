# `microsoft-365.planner.task.assign`

Add (or replace) assignees on a Planner task.

## When to use

The "ball-in-court" primitive — set who owns the next move. Reassign an RFI follow-up when responsibility shifts, or batch-assign newly created tasks once the owner is known. Pairs with `planner.task.create` (which can leave a task unassigned) and `planner.task.list` (find the task first).

**WRITE-mode** (external state — Planner). Declare `safety:` on the node; for a reassignment, `transaction-group: notify, snapshot: false` is honest.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `task-id` | string | yes | — | The task to modify. |
| `assigned-to` | array<string> | yes | — | User principal names or AAD object ids. |
| `replace` | boolean | no | `false` | If `false`, appends; if `true`, replaces all current assignees. |

## Output

```yaml
task-id:     "abc123..."
assigned-to: ["consultant@acme-mep.com", "pm@acme-arch.com"]
```

## Worked example

```yaml
- id: reassign
  agent: microsoft-365
  command: planner.task.assign
  inputs:
    task-id: "{{ open-actions.tasks[0].task-id }}"
    assigned-to: ["pm@acme-arch.com"]
    replace: true
  safety:
    transaction-group: notify
    snapshot: false
```

## Implementation note

Calls `PATCH /planner/tasks/{task-id}`, sending the `assignments` map (each assignee as `{ "@odata.type": "#microsoft.graph.plannerAssignment", "orderHint": " !" }`; a `null` entry removes one). The transport first GETs the task to read its `@odata.etag` and passes it as the required `If-Match` header. Requires `Group.ReadWrite.All` (or `Tasks.ReadWrite`). On HTTP 429 the transport honors the `Retry-After` header.

## See also

- `planner.task.create` — create then assign
- `planner.task.list` — find the task id
- `planner.task.complete` — mark done
