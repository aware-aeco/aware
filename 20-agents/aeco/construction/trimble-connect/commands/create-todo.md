# `trimble-connect.create-todo` — create a ToDo

Write command. `status` and `priority` are checked against the Core definition's fixed
values; `type` is one of the project's ToDo types and passes through. `assignees` takes
user ids (sent as `{id, type: USER}`) or `{id, type: USER|GROUP}` objects.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `project-id` | string | Project id. |
| `description` | string | What needs doing. |
| `title` | string (optional) | Short title. |
| `type` | string (optional) | A ToDo type of the project (e.g. Request, Issue). |
| `label` | string (optional) | Label, unique within the project. |
| `due-date` | string (optional) | ISO 8601 date or date-time (TC keeps the date). `Z` is sent as `+0000`. |
| `percent-complete` | int (optional) | Progress, 0-100. |
| `status` | `NEW` \| `IN_PROGRESS` \| `RESOLVED` \| `BLOCKED` \| `CLOSED` (optional) | Status. |
| `priority` | `LOW` \| `NORMAL` \| `HIGH` \| `CRITICAL` (optional) | Priority. |
| `assignees` | list (optional) | User ids, or `{id, type: USER|GROUP}` objects. |
| `region` | string (optional) | The project's `location` (`europe`, `unitedKingdom`, …) — routes the call to that region's host. Omitted: the project's own `location` is looked up. See [Regions](../skills/projects.md#regions). |

The agent authenticates with the single `trimble-connect` credential from
`aware connect trimble-connect` (the token is refreshed automatically, #198).

## Outputs

Handled in-process by the runtime (`cli/src/runtime/trimble_bcf.rs`, floless.app#2184):
it builds TC's camelCase body from the kebab-case inputs, which the generic REST renderer
cannot express. ToDos are Core API 2.0, on the agent's own base. A non-2xx **fails the
node** with the step, the status and TC's own message.

```yaml
todo-id: string
todo: object
region:    string            # the location the call was routed to, when known
```

## REST translation

```http
POST {core}/todos   (Bearer)
Content-Type: application/json
{ "projectId", "description", "title", "type", "label", "dueDate",
  "percentComplete", "status", "priority", "assignees": [{"id", "type"}] }   (only those given)
→ 201 todo
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `missing required input …` / `… must be …` | Bad input — refused before any request | Fix the node config |
| `create todo: HTTP 400 …` | e.g. a `label` already used in the project | Use another label |
| `… HTTP 401 …` | Token expired and could not be refreshed | `aware connect trimble-connect --refresh` |

## See also

- [projects.md](../skills/projects.md) — the API reference
- [trimble-connect.update-todo](./update-todo.md)
- [trimble-connect.list-todos](./list-todos.md)
