# `trimble-connect.upload-bcf-document` — upload a project BCF document

Write command. Sends the file as `multipart/form-data` (the form that carries the file
name). On Trimble Connect it lands as a **file in the project's root folder**; the
returned `document-id` is that file's id. BCF has no delete for documents — remove one
with `delete-file`. Max 256 MiB.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `region` | string (optional) | The project's `location` (`europe`, `unitedKingdom`, …) — routes the call to that region's host. Omitted: the project's own `location` is looked up. See [Regions](../skills/projects.md#regions). |
| `project-id` | string | Project id. |
| `filename` | string | File name (no path). |
| `bytes` | bytes | File content, base64 (max 256 MiB). |

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
document-id: string
filename: string
```

## REST translation

```http
0. GET {core}/projects/{project-id}   (Bearer)  → location
   GET {core}/regions                 (no auth) → topic-api for that location
   {bcf} = {topic-api}bcf/2.1/projects/{project-id}
1. POST {bcf}/documents   (Bearer)
   Content-Type: multipart/form-data; boundary=…
   file=<bytes>; filename="{filename}"
   → 201 {"guid", "filename"}
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
- [trimble-connect.create-document-reference](./create-document-reference.md)
- [trimble-connect.list-bcf-documents](./list-bcf-documents.md)
