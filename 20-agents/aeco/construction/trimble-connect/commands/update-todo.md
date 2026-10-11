# `trimble-connect.update-todo` — change some fields of a ToDo

Write command. TC's `PATCH` changes only the fields sent. An input left out or `""` is
not a change; an explicit `null` clears `due-date` and empties `assignees` (`title`,
`description` and `label` cannot be cleared — TC keeps a label once set). A due date ending
in `Z` is sent as `+0000`, which is the form TC accepts. The Core definition declares no
`If-Match` for ToDos.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `todo-id` | string | ToDo id. |
| `description` | string (optional) | New description. |
| `title` | string (optional) | New title. |
| `type` | string (optional) | New type. |
| `label` | string (optional) | New label (TC keeps a label once set; it cannot be cleared). |
| `due-date` | string (optional) | New due date (ISO 8601; TC keeps the date); null clears. |
| `percent-complete` | int (optional) | Progress, 0-100. |
| `status` | `NEW` \| `IN_PROGRESS` \| `RESOLVED` \| `BLOCKED` \| `CLOSED` (optional) | New status. |
| `priority` | `LOW` \| `NORMAL` \| `HIGH` \| `CRITICAL` (optional) | New priority. |
| `assignees` | list (optional) | Replacement assignees; null clears. |

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
```

## REST translation

```http
PATCH {core}/todos/{todo-id}   (Bearer)
Content-Type: application/json
{ the given fields, camelCase }
→ 200 todo
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `missing required input …` / `… must be …` | Bad input — refused before any request | Fix the node config |
| `update-todo needs at least one field …` | No effective change given | Name a field to change |
| `… HTTP 401 …` | Token expired and could not be refreshed | `aware connect trimble-connect --refresh` |

## See also

- [projects.md](../skills/projects.md) — the API reference
- [trimble-connect.create-todo](./create-todo.md)
- [trimble-connect.get-todo](./get-todo.md)
