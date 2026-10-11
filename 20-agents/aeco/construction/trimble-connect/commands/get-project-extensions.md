# `trimble-connect.get-project-extensions` — the project's BCF extension values

Stateless read. Topic types, statuses, labels, priorities and stages are per-project BCF
extensions: `create-topic` / `update-topic` accept only these values (anything else is a
`400 … is not defined in the project extension`). Read them here first. The raw
extensions object (with colours, defaults and the allowed actions) is under `extensions`.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `region` | string (optional) | The project's `location` (`europe`, `unitedKingdom`, …) — routes the call to that region's host. Omitted: the project's own `location` is looked up. See [Regions](../skills/projects.md#regions). |
| `project-id` | string | Project id. |

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
region:    string            # the location the call was routed to, when known
topic-types: array
topic-statuses: array
topic-labels: array
priorities: array
stages: array
extensions: object
```

## REST translation

```http
0. GET {core}/projects/{project-id}   (Bearer)  → location
   GET {core}/regions                 (no auth) → topic-api for that location
   {bcf} = {topic-api}bcf/2.1/projects/{project-id}
1. GET {bcf}/extensions?includeUsers=false   (Bearer)
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `missing required input …` / `… must be …` | Bad input — refused before any request | Fix the node config |
| `BCF region: project: HTTP 404 …` | No such project, or no access | Check `project-id` with `list-projects` |
| `BCF region: … not a known Trimble Connect Topics host` | TC named a Topics URL outside the published five | Report it; the token was not sent |
| `… redirects are not followed` | TC answered 3xx | Report it |
| `… HTTP 401 …` | Token expired and could not be refreshed | `aware connect trimble-connect --refresh` |

## See also

- [topics.md](../skills/topics.md) — the API reference
- [trimble-connect.create-topic](./create-topic.md)
- [trimble-connect.update-topic](./update-topic.md)
