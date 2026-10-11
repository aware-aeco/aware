# `trimble-connect.update-topic` — change some fields of a BCF topic

Write command. BCF's `PUT topic` is specified to replace the **whole** topic, so the topic
is read first and every field of the PUT schema it carries (`topic_type`, `topic_status`, `reference_links`,
`title`, `priority`, `index`, `labels`, `assigned_to`, `assigned_to_uuid`, `assignees`,
`stage`, `description`, `due_date`, `marker`, `bim_snippet`) is sent back with your
changes laid over it — a field you do not name survives. An input left out or `""`
is not a change; an explicit `null` clears `description` (sent as `""`) or `labels` (sent
as `[]`). Verified live: Trimble Connect ignores `null` in a topic PUT and offers no way to
clear `due-date`, `assigned-to` or `stage`, so a `null` there (or on `title`) is refused. When the result equals the topic as
read, nothing is written and `changed` is `false`.

**Concurrency:** BCF 2.1 on Trimble Connect offers no `If-Match` / ETag, so an edit
someone else makes between this command's read and its write is overwritten.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `project-id` | string | Project id. |
| `topic-id` | string | Topic guid. |
| `title` | string (optional) | New title (cannot be cleared). |
| `description` | string (optional) | New description; null clears. |
| `topic-type` | string (optional) | New type. |
| `topic-status` | string (optional) | New status. |
| `priority` | string (optional) | New priority. |
| `labels` | list (optional) | Replacement labels; null clears. |
| `assigned-to` | string (optional) | New assignee email (cannot be cleared). |
| `due-date` | string (optional) | New due date (cannot be cleared). |
| `stage` | string (optional) | New stage (cannot be cleared). |
| `region` | string (optional) | The project's `location` (`europe`, `unitedKingdom`, …) — routes the call to that region's host. Omitted: the project's own `location` is looked up. See [Regions](../skills/projects.md#regions). |

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
changed: bool
region:    string            # the location the call was routed to, when known
```

## REST translation

```http
0. GET {core}/projects/{project-id}   (Bearer)  → location
   GET {core}/regions                 (no auth) → topic-api for that location
   {bcf} = {topic-api}bcf/2.1/projects/{project-id}
1. GET {bcf}/topics/{topic-id}   (Bearer)  → current topic
2. PUT {bcf}/topics/{topic-id}   (Bearer)  — skipped when nothing changes
   Content-Type: application/json
   { every PUT-schema field of the current topic, with the changes applied }
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `missing required input …` / `… must be …` | Bad input — refused before any request | Fix the node config |
| `BCF region: project: HTTP 404 …` | No such project, or no access | Check `project-id` with `list-projects` |
| `BCF region: … not a known Trimble Connect Topics host` | TC named a Topics URL outside the published five | Report it; the token was not sent |
| `… redirects are not followed` | TC answered 3xx | Report it |
| `update-topic needs at least one field …` | No effective change given | Name a field to change |
| `update topic: HTTP 400 …` | A value the project's extensions do not allow | Use a value from the project's topic settings |
| `… HTTP 401 …` | Token expired and could not be refreshed | `aware connect trimble-connect --refresh` |

## See also

- [topics.md](../skills/topics.md) — the API reference
- [trimble-connect.get-topic](./get-topic.md)
- [trimble-connect.create-topic](./create-topic.md)
