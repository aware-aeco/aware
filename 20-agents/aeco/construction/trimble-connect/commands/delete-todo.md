# `trimble-connect.delete-todo` — delete a ToDo

Write command.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `todo-id` | string | ToDo id. |
| `region` | string (optional) | The project's `location` (`europe`, `unitedKingdom`, …) — routes the call to that region's host. Omitted: the North America master. See [Regions](../skills/projects.md#regions). |

The agent authenticates with the single `trimble-connect` credential from
`aware connect trimble-connect` (the token is refreshed automatically, #198).

## Outputs

Handled in-process by the runtime (`cli/src/runtime/trimble_bcf.rs`, floless.app#2184):
it builds TC's camelCase body from the kebab-case inputs, which the generic REST renderer
cannot express. ToDos are Core API 2.0, on the agent's own base. A non-2xx **fails the
node** with the step, the status and TC's own message.

```yaml
todo-id: string
deleted: bool
region:    string            # the location the call was routed to, when known
```

## REST translation

```http
DELETE {core}/todos/{todo-id}   (Bearer)  → 204
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `missing required input …` / `… must be …` | Bad input — refused before any request | Fix the node config |
| `delete todo: HTTP 404 …` | No such ToDo (already deleted?) | List them with `list-todos` |
| `… HTTP 401 …` | Token expired and could not be refreshed | `aware connect trimble-connect --refresh` |

## See also

- [projects.md](../skills/projects.md) — the API reference
- [trimble-connect.create-todo](./create-todo.md)
