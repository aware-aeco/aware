# `trimble-connect.list-comments` — one page of a topic's comments

Stateless read. BCF 2.1 pages with `top` + `skiptoken`: the response's `next` header names the next page,
whose opaque token comes back as `next-skiptoken` (null on the last page). Pass it as
`skiptoken` to read on. A 204 is an empty list.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `project-id` | string | Project id. |
| `topic-id` | string | Topic guid. |
| `top` | int (optional) | Page size. |
| `skiptoken` | string (optional) | The previous page's `next-skiptoken`. |
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
items:
  type: array
  items:
    guid: string
    comment: string
    author: string
    date: string
    viewpoint_guid: string
    reply_to_comment_guid: string
next-skiptoken: string
region:    string            # the location the call was routed to, when known
```

## REST translation

```http
0. GET {core}/projects/{project-id}   (Bearer)  → location
   GET {core}/regions                 (no auth) → topic-api for that location
   {bcf} = {topic-api}bcf/2.1/projects/{project-id}
1. GET {bcf}/topics/{topic-id}/comments?top={top}&skiptoken={skiptoken}   (Bearer)
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `missing required input …` / `… must be …` | Bad input — refused before any request | Fix the node config |
| `BCF region: project: HTTP 404 …` | No such project, or no access | Check `project-id` with `list-projects` |
| `BCF region: … not a known Trimble Connect Topics host` | TC named a Topics URL outside the published five | Report it; the token was not sent |
| `… redirects are not followed` | TC answered 3xx | Report it |
| `… HTTP 401 …` | Token expired and could not be refreshed | `aware connect trimble-connect --refresh` |

## Through `aware agent call` (#788)

A bound, read-only call. Inputs: `region` (required), `project-id`, `topic-id`, `top` 1..500
(default 100), `skiptoken`. Body: `{region, items: [{guid, comment, author, date, modified_date,
modified_author, viewpoint_guid, reply_to_comment_guid, truncated?}], next-skiptoken,
more-available, omitted}`. `modified_date` / `modified_author` appear only when TC sends them —
AWARE never infers an edit.

The token goes only to the region's Core host (`app*.connect.trimble.com`) and Topics host (`open*.connect.trimble.com`), resolved through the public `/regions` lookup; redirects are refused. The principal is tenantless: `tenant_id: null`, `namespace: "trimble-connect-user"`, `stable_id` the `/users/me` `id`. Failures carry `details.phase` (`region`, `project` or `item`). A field Trimble sent as `null` stays `null`, one it did not send is absent, and `omitted` counts entries dropped for a missing or malformed id. Full contract: [cli-spec, Trimble Connect reads](../../../../../10-core/cli-spec.md).

## See also

- [topics.md](../skills/topics.md) — the API reference
- [trimble-connect.create-comment](./create-comment.md)
