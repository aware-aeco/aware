# `trimble-connect.get-viewpoint-snapshot` — a viewpoint's snapshot image

Stateless read. Returns the image bytes base64-encoded (the same shape as `download`), at
most 20 MiB.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `project-id` | string | Project id. |
| `topic-id` | string | Topic guid. |
| `viewpoint-id` | string | Viewpoint guid. |

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
bytes: string
encoding: string
content-type: string
size-kb: number
```

## REST translation

```http
0. GET {core}/projects/{project-id}   (Bearer)  → location
   GET {core}/regions                 (no auth) → topic-api for that location
   {bcf} = {topic-api}bcf/2.1/projects/{project-id}
1. GET {bcf}/topics/{topic-id}/viewpoints/{viewpoint-id}/snapshot   (Bearer)  → image bytes
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `missing required input …` / `… must be …` | Bad input — refused before any request | Fix the node config |
| `BCF region: project: HTTP 404 …` | No such project, or no access | Check `project-id` with `list-projects` |
| `BCF region: … not a known Trimble Connect Topics host` | TC named a Topics URL outside the published five | Report it; the token was not sent |
| `… redirects are not followed` | TC answered 3xx | Report it |
| `get viewpoint snapshot: HTTP 404 …` | The viewpoint has no snapshot | Check `list-viewpoints` |
| `… HTTP 401 …` | Token expired and could not be refreshed | `aware connect trimble-connect --refresh` |

## See also

- [viewpoints.md](../skills/viewpoints.md) — the API reference
- [trimble-connect.create-viewpoint](./create-viewpoint.md)
