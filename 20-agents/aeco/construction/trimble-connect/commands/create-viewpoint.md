# `trimble-connect.create-viewpoint` — add a viewpoint (camera and/or snapshot) to a topic

Write command. Cameras, lines, clipping planes and components are given in **BCF's own
wire shape** (snake_case keys, `{x, y, z}` points) and are checked before anything is
sent: at most one camera; every point has finite numeric `x`/`y`/`z`; direction and up
vectors are non-zero; `field_of_view` is in (0, 180); `view_to_world_scale` > 0;
`components.selection` / `coloring` are lists of objects and `visibility` an object.
A camera or a snapshot is required. The snapshot is a base64 PNG or JPEG (max 20 MiB)
that must decode completely; its type is inferred, and a `snapshot-type` that disagrees
is refused. Object/list inputs may also be given as JSON text.

```yaml
- id: view
  agent: trimble-connect
  command: create-viewpoint
  config:
    project-id: "{{ inputs.project }}"
    topic-id:   "{{ issue.topic-id }}"
    perspective-camera:
      camera_view_point: { x: 12.0, y: -4.5, z: 3.2 }
      camera_direction:  { x: -0.6, y: 0.7, z: -0.3 }
      camera_up_vector:  { x: 0, y: 0, z: 1 }
      field_of_view: 60
    snapshot: "{{ render.bytes }}"
```

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `project-id` | string | Project id. |
| `topic-id` | string | Topic guid. |
| `perspective-camera` | object (optional) | BCF perspective_camera: camera_view_point, camera_direction, camera_up_vector ({x,y,z}) and field_of_view. |
| `orthogonal-camera` | object (optional) | BCF orthogonal_camera: the same points plus view_to_world_scale. |
| `lines` | list (optional) | BCF lines: [{start_point, end_point}]. |
| `clipping-planes` | list (optional) | BCF clipping_planes: [{location, direction}]. |
| `components` | object (optional) | BCF components: {selection, coloring, visibility}. |
| `snapshot` | bytes (optional) | PNG or JPEG image, base64 (max 20 MiB). |
| `snapshot-type` | `png` \| `jpg` (optional) | Declared image type; inferred from the bytes when omitted, refused when it disagrees. |

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
viewpoint-id: string
viewpoint: object
```

## REST translation

```http
0. GET {core}/projects/{project-id}   (Bearer)  → location
   GET {core}/regions                 (no auth) → topic-api for that location
   {bcf} = {topic-api}bcf/2.1/projects/{project-id}
1. POST {bcf}/topics/{topic-id}/viewpoints   (Bearer)
   Content-Type: application/json
   { "perspective_camera" | "orthogonal_camera", "lines", "clipping_planes", "components",
     "snapshot": { "snapshot_type": "png" | "jpg", "snapshot_data": "<base64>" } }
   → 201 viewpoint
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `missing required input …` / `… must be …` | Bad input — refused before any request | Fix the node config |
| `BCF region: project: HTTP 404 …` | No such project, or no access | Check `project-id` with `list-projects` |
| `BCF region: … not a known Trimble Connect Topics host` | TC named a Topics URL outside the published five | Report it; the token was not sent |
| `… redirects are not followed` | TC answered 3xx | Report it |
| `… is the zero vector` / `… out of range` / `not a complete PNG or JPEG` | Malformed viewpoint — refused before any request | Fix the camera / image |
| `… HTTP 401 …` | Token expired and could not be refreshed | `aware connect trimble-connect --refresh` |

## See also

- [viewpoints.md](../skills/viewpoints.md) — the API reference
- [trimble-connect.get-viewpoint-snapshot](./get-viewpoint-snapshot.md)
- [trimble-connect.list-viewpoints](./list-viewpoints.md)
- [trimble-connect.create-comment](./create-comment.md)
