# `trimble-connect.create-topic` — create a BCF topic

Write command (`mode: write`, so `--dry-run` stubs it). Type, status, priority and stage
are the project's BCF *extension* values (they differ per project), so they are passed
through as given; TC refuses one the project does not define. Pass `guid` to make a
retried create safe: the second attempt cannot make a second topic.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `project-id` | string | Project id. |
| `title` | string | Topic title. |
| `description` | string (optional) | Topic description. |
| `topic-type` | string (optional) | e.g. "Issue", "Clash", "Request" (a project extension value). |
| `topic-status` | string (optional) | e.g. "Open", "In Progress", "Closed" (a project extension value). |
| `priority` | string (optional) | e.g. "Normal", "High" (a project extension value). |
| `labels` | list (optional) | Labels (list of strings). |
| `assigned-to` | string (optional) | Assignee email. |
| `due-date` | string (optional) | ISO 8601 date-time. |
| `stage` | string (optional) | Stage (a project extension value). |
| `guid` | string (optional) | Client-chosen topic guid, so a retried create cannot make a second topic. |

The agent authenticates with the single `trimble-connect` credential from
`aware connect trimble-connect` (the token is refreshed automatically, #198).

## Outputs

Handled in-process by the runtime (`cli/src/runtime/trimble_bcf.rs`, floless.app#2184).
BCF lives on Trimble's regional Topics API, not under the agent's Core base, so every
BCF command first resolves the project's region — `GET {core}/projects/{project-id}`
gives its `location`, and the public `GET {core}/regions` maps that to the region's
`topic-api` — and sends the token only to one of the five published Topics hosts
(`open11` / `open21` / `open22` / `open31` / `open32` `.connect.trimble.com`). Redirects
are never followed. A non-2xx **fails the node** with the step, the status and TC's own
message — it is not returned as data.

```yaml
topic-id: string
topic: object
```

## REST translation

```http
0. GET {core}/projects/{project-id}   (Bearer)  → location
   GET {core}/regions                 (no auth) → topic-api for that location
   {bcf} = {topic-api}bcf/2.1/projects/{project-id}
1. POST {bcf}/topics   (Bearer)
   Content-Type: application/json
   { "title", "description", "topic_type", "topic_status", "priority",
     "labels", "assigned_to", "due_date", "stage", "guid" }   (only those given)
   → 201 topic
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `missing required input …` / `… must be …` | Bad input — refused before any request | Fix the node config |
| `BCF region: project: HTTP 404 …` | No such project, or no access | Check `project-id` with `list-projects` |
| `BCF region: … not a known Trimble Connect Topics host` | TC named a Topics URL outside the published five | Report it; the token was not sent |
| `… redirects are not followed` | TC answered 3xx | Report it |
| `create topic: HTTP 400 …` | A value the project's extensions do not allow | Use a value from the project's topic settings |
| `… HTTP 401 …` | Token expired and could not be refreshed | `aware connect trimble-connect --refresh` |

## See also

- [topics.md](../skills/topics.md) — the API reference
- [trimble-connect.update-topic](./update-topic.md)
- [trimble-connect.create-comment](./create-comment.md)
- [trimble-connect.create-viewpoint](./create-viewpoint.md)
