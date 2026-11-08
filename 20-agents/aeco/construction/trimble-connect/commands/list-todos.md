# `trimble-connect.list-todos` — a project's ToDos

Stateless read. All of a project's ToDos, or those on one file (its latest version, or
`version-id`). TC pages the list: a 206 (`partial: true`) with `content-range`
(`items 0-99/240`) means more exist — don't read a missing ToDo in a partial page as
absent; pass `range` for the next page.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `project-id` | string | Project id. |
| `file-id` | string (optional) | Only the ToDos on this file (its latest version). |
| `version-id` | string (optional) | Only the ToDos on this file version (needs `file-id`). |
| `range` | string (optional) | A page, e.g. `items=100-199` (zero-based, inclusive). |
| `region` | string (optional) | The project's `location` (`europe`, `unitedKingdom`, …) — routes the call to that region's host. Omitted: the project's own `location` is looked up. See [Regions](../skills/projects.md#regions). |

The agent authenticates with the single `trimble-connect` credential from
`aware connect trimble-connect` (the token is refreshed automatically, #198).

## Outputs

Handled in-process by the runtime (`cli/src/runtime/trimble_bcf.rs`, floless.app#2184):
it builds TC's camelCase body from the kebab-case inputs, which the generic REST renderer
cannot express. ToDos are Core API 2.0, on the agent's own base. A non-2xx **fails the
node** with the step, the status and TC's own message.

```yaml
partial: bool
content-range: string
items:
  type: array
  items:
    id: string
    label: string
    title: string
    description: string
    status: string
    priority: string
    dueDate: string
    assignees: array
region:    string            # the location the call was routed to, when known
```

## REST translation

```http
GET {core}/todos?projectId={project-id}[&fileId={file-id}[&versionId={version-id}]]   (Bearer)
Range: items=0-99            (when `range` is given)
→ 200/206 [todo…] + Content-Range
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `missing required input …` / `… must be …` | Bad input — refused before any request | Fix the node config |
| `… HTTP 401 …` | Token expired and could not be refreshed | `aware connect trimble-connect --refresh` |

## Through `aware agent call` (#788)

A bound, read-only call. Inputs: `region` (required), `project-id`, `range` (optional,
`items=<start>-<end>`, at most 1000 items). File filters are not part of the bound call. Body:
`{region, status, partial, range: {unit, start, end, total} | null, items: [{id, label, title,
description, status, priority, dueDate, percentComplete, assignees: [{id, type, name}], createdOn,
modifiedOn, truncated?}], omitted}`. `status` is TC's HTTP status: 200, 204, 206 and **416** are all
data (a 416 with `items */<total>` is past the end). `content-range-unparsed: true` marks a
`Content-Range` AWARE could not read. Revision fields appear only when TC sends them.

The token goes only to the region's Core host (`app*.connect.trimble.com`) and Topics host (`open*.connect.trimble.com`), resolved through the public `/regions` lookup; redirects are refused. The principal is tenantless: `tenant_id: null`, `namespace: "trimble-connect-user"`, `stable_id` the `/users/me` `id`. Failures carry `details.phase` (`region`, `project` or `item`). A field Trimble sent as `null` stays `null`, one it did not send is absent, and `omitted` counts entries dropped for a missing or malformed id. Full contract: [cli-spec, Trimble Connect reads](../../../../../10-core/cli-spec.md).

## See also

- [projects.md](../skills/projects.md) — the API reference
- [trimble-connect.get-todo](./get-todo.md)
- [trimble-connect.create-todo](./create-todo.md)
