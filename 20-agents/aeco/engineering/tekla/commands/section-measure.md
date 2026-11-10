# section-measure

Measures Tekla's own evaluated cross-section for each profile name, the point its member axis
passes through (the reference point), and where the section sits in world space under each
requested position. It is an oracle: a caller proves its own section definitions against it,
vertex for vertex, and checks placement under rotations and depth positions in one batch.

```powershell
'{"profiles":["PL20*200","L100*80*8"],
  "placement_grid":{"rotations":["FRONT","TOP","BACK","BELOW"],"depths":["FRONT","MIDDLE","BEHIND"]}}' |
  aware-tekla section-measure --json-stdin
```

The section is read from Tekla's catalogue (`CrossSection.Select`), which changes nothing. The
reference point and every placement are measured on a **temporary member**: it is inserted with
the Name `AWARE_SECTION_MEASURE`, its solid read, and deleted in a `finally`. Nothing is
committed, saved or modified, so the verb is declared `mode: write` honestly: the model holds
the member for the moment of the measurement. A delete that fails is listed in
`temporary_members.cleanup_failures` with its id and GUID, never hidden.

A `finally` cannot run if the bridge is killed or Tekla fails between an insert and its delete. So
each run **records** its members in a file of its own,
`%LOCALAPPDATA%\aware\tekla-section-measure\<model key>.<run id>.txt`, appending `+<guid>` when Tekla
accepts a member and `-<guid>` once it is deleted; the file is removed when the run deletes everything
it inserted. Nothing is rewritten and no two runs share a file, so concurrent or interrupted runs
cannot erase each other's records.

Every run then **reports**, and never deletes, what earlier runs left outstanding that still exists in
this model with the name `AWARE_SECTION_MEASURE`: `temporary_members.leftovers_from_earlier_runs`
lists each one's `guid`, `id` and the record file (`recorded_in`), or a `note` when Tekla could not say
whether it still exists. The person removes those members. `unreadable_ledgers` lists record files
that could not be read. Recording is best effort: if the folder or the file cannot be written, the run
still measures (each member is still deleted in its `finally`) and `ledger_error` says that this run's
leftovers may not be recorded. `temporary_members.ledger` is this run's file.

Seen live: when the bridge is killed mid-run, Tekla drops that client's uncommitted members itself, so
there is usually nothing to report.

## Input

| Field | Type | Description |
|---|---|---|
| `profiles` | string[] | Names to measure. Required. Duplicates and blanks are ignored. |
| `placements` | object[] | Positions: `{rotation, rotation_offset, plane, plane_offset, depth, depth_offset, axis}`. `rotation` is `FRONT`, `TOP`, `BACK` or `BELOW`; `plane` is `LEFT`, `MIDDLE` or `RIGHT`; `depth` is `FRONT`, `MIDDLE` or `BEHIND`. Offsets are mm (rotation offset in degrees, Tekla's own convention). Defaults: `FRONT`, `MIDDLE`, `MIDDLE`, 0. |
| `placement_grid` | object | `{rotations, rotation_offsets, planes, plane_offsets, depths, depth_offsets}`. Every combination becomes a placement, appended after `placements` in that nesting order. A list left out is one default value. |
| `axis` | object | Default member axis `{start:[x,y,z], end:[x,y,z]}` in mm. Default `[0,0,0]` to `[length,0,0]`. A placement may carry its own. |
| `length` | number | Member length (mm) for the default axis and the catalogue section. Default 1000. |
| `high_accuracy` | boolean | Read Tekla's high-accuracy section and solid. Default true. |
| `material` | string | Material of the temporary members. Default `STEEL_UNDEFINED`. |
| `version`, `pid` | | As for `exec`. |

One request inserts `profiles × (placements + 1)` temporary members, at most 5000; a grid is sized
before it is expanded. Invalid input (a field of the wrong type, an unknown rotation, a non-numeric
offset, an axis shorter than 1 mm, too many members) is refused before the first insert, with the
field named.

## Output (`result`)

```json
{
  "model_path": "C:\\TeklaStructuresModels\\Feasibility",
  "environment": { "name": "default", "profile_database": "C:\\ProgramData\\Trimble\\Tekla Structures\\2026.0\\\\environments\\default\\General\\Shared\\Profil\\" },
  "length": 1000, "high_accuracy": true, "material": "STEEL_UNDEFINED", "placement_count": 12,
  "profiles": [
    {
      "profile": "L100*80*8", "prefix": "L", "parameter_string": "h*b*t", "resolution": "parametric",
      "parameters": [ { "symbol": "h", "value": 100, "unit_type": "INPUT_DIMENSION_UNIT" },
                      { "symbol": "r", "value": 12, "unit_type": "INPUT_DIMENSION_UNIT" } ],
      "section": {
        "frame": "tekla-profile", "high_accuracy": true,
        "outer": {
          "vertices": [ { "x": -40, "y": -50 }, { "x": 40, "y": -50 }, { "x": 40, "y": -42 },
                        { "x": -32, "y": -42, "chamfer": { "type": "CHAMFER_ROUNDING", "x": 12, "y": 0 } },
                        { "x": -32, "y": 50 }, { "x": -40, "y": 50 } ],
          "points": [ [-40,-50], [40,-50], [40,-42], [-20,-42], [-24.59220118838108,-41.08655439013544], "…" ]
        },
        "holes": [], "area": 1409.7871714788544, "area_vertices": 1376,
        "bounds": { "min": [-40,-50], "max": [40,50] }
      },
      "reference_placement": { "rotation": "FRONT", "plane": "MIDDLE", "depth": "MIDDLE", "status": "ok", "…": "as a placement row" },
      "reference_point": [0, 0],
      "placements": [
        {
          "rotation": "FRONT", "rotation_offset": 0, "plane": "MIDDLE", "plane_offset": 0,
          "depth": "FRONT", "depth_offset": 0,
          "axis": { "start": [0,0,0], "end": [1000,0,0] },
          "status": "ok",
          "start_face": {
            "outer": [ [0,-50,0], [0,50,0], [0,50,8], "…" ], "holes": [],
            "outer_2d": [ [-50,0], [50,0], [50,8], "…" ], "holes_2d": [],
            "bounds_2d": { "min": [-50,0], "max": [50,80] }, "area": 1409.7871714788544, "start_offset": 0
          },
          "bounds": { "min": [0,-50,0], "max": [1000,50,80] },
          "member_frame": { "origin": [0,0,40], "axis_x": [1,0,0], "axis_y": [0,1,0] },
          "frame_2d": { "v": [0,1,0], "w": [0,0,1] },
          "tekla_area": 1409.7871714788541,
          "section_maps": [ { "matrix": [[0,1],[1,0]], "translation": [0,40], "max_residual": 0 } ]
        }
      ],
      "status": "ok"
    }
  ],
  "temporary_members": { "inserted": 13, "deleted": 13, "cleanup_failures": [],
                         "leftovers_from_earlier_runs": [], "unreadable_ledgers": [], "ledger": "…", "ledger_error": null }
}
```

- **`section`** is in Tekla's profile frame: `x` across the width, `y` along the height, as Tekla's
  catalogue returns it. `vertices` are the defining vertices; a vertex with `chamfer` is rounded or
  chamfered there (`type`, `x`, `y` are Tekla's `Chamfer` fields; for `CHAMFER_ROUNDING`, `x` is the
  radius). `points` are Tekla's own expanded outline. `area` is the shoelace area of `points` minus the
  holes; `area_vertices` the same over `vertices` (corners unrounded). Numbers are Tekla's doubles,
  unrounded; only `-0` is written as `0`.
- **`parameters`** are the values Tekla applied for this name, defaults included (a rounding radius
  that the name does not carry).
- **`start_face`** is the solid's end face at the start of the member, in world mm (`outer`, `holes`)
  and in the 2D frame `frame_2d` relative to the axis start (`outer_2d`, `holes_2d`). The frame is
  `v = Z × axis`, `w = axis × v` (for a vertical member `v = axis × X`): a member along +X reads as
  (Y, Z). The outer loop is the largest one; the rest are holes. `bounds` is the solid's world box.
- **`section_maps`** lists every one of the eight 90° rotations or reflections, with a translation,
  that carries the catalogue section onto `outer_2d`: `face = matrix · section + translation`. A map
  must put every anchor onto a face point and match both bounding boxes within 0.001 mm. Anchors are
  the unrounded vertices and the two ends of each rounding arc: Tekla tessellates a rounded corner
  differently in the catalogue and in the solid, so the points between the arc ends are not compared.
  Holes are checked too: the same number on both sides, each mapped hole's bounding box equal to a
  face hole's, and every hole anchor on a face hole point. A symmetric section has several maps; a
  rotation offset that is not a multiple of 90° has none, with `section_map_note` saying so.
- **`reference_point`** is the member axis read back into the section frame from the reference
  placement (`MIDDLE` / `FRONT` / `MIDDLE`, no offsets). It is `null`, with `reference_point_note`,
  when no map fits or two maps put the axis at different points.
- **`status`** per profile: `ok`, `partial` (some placement did not measure), or `no-section` (Tekla
  has no profile of that name, or returned no section). Per placement: `ok`, `insert-refused`,
  `no-start-face` or `error`, each with a `message`.

Refusals are the same as `exec`: exactly one reachable Tekla instance and an open model are required.
