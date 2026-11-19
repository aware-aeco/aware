# item-create

Sends a member to Tekla as an **Item** when Tekla has no profile for its section. The member's solid
is added to the open model's shape catalogue, and a Tekla Item (`Tekla.Structures.Model.Brep`, whose
`Profile.ProfileString` is the shape's name) is placed on it. The Item is a fixed solid of one size
and one length with no profile name; it is labelled that way in its name, its user properties and the
receipt. Use `profile-project` first: a section Tekla can name should be a profile, not an Item.

```powershell
'{"mode":"create","consent":{"source":"ui-click"},
  "target":{"model_path":"C:\\Models\\Hall","model_guid":"c7ce9de7-9f43-4950-b735-04698c3b9312","shared_model":false,"tekla_version":"2026.0"},
  "members":[{"id":"M12","label":"Bracket M12","material":"S355JR",
              "solid":{"kind":"prism","outer":[[-50,-50],[50,-50],[50,50],[-50,50]],
                       "holes":[[[-44,-44],[-44,44],[44,44],[44,-44]]],"length":2400},
              "placement":{"start":[0,0,0],"end":[2400,0,0]}}]}' |
  aware-tekla item-create --json-stdin
```

## Modes

| `mode` | writes | needs `consent` |
|---|---|---|
| `plan` (default) | nothing: reports what would be created and reused | no |
| `create` | shapes in the model's shape catalogue, then Items | yes |
| `delete` | removes the Items and shapes listed (the `undo` of a receipt) | yes |

`consent` is `{"source": "ui-click"}` or `{"source": "chat-proposal-card"}`, given after the person
confirmed. **An unattended run never calls this verb**: it leaves the member out, labelled, and asks.

## The solid, and why every length is a shape

`solid` is a prism: a section swept along +X. `outer` is the section ring as `[y, z]` points in
millimetres, `holes` are rings inside it, `length` is the sweep. Rings may wind either way (they are
normalised); a ring that crosses itself, a hole outside or touching the outer ring, overlapping holes,
a repeated point, an empty ring or a non-finite number is `invalid-solid` and refused **before Tekla is
touched** (exit 2). A `solid_digest`, if you send one, must equal the digest AWARE computes for the
solid, else `digest-mismatch`.

**Tekla does not stretch a shape to an Item's length** (measured live, 2026.0): the Item has the
shape's own length whatever its end point. So each distinct length is its own shape, and `end` must lie
exactly `length` from `start` (else `length-mismatch`, refused before Tekla). The number of shapes a
call adds is `new_shape_count`, and a confirm card should show it. Members with the same section and
length share one shape.

## Shape names

`AWI-` plus the first 24 hex digits of the solid's SHA-256 (over its normalised rings, every
coordinate and the length): the same solid is always the same name and a different length never is.
`name_prefix` changes `AWI`. **The live rule for shape names is not fully known** (experiment E6 of the
spike has not run), so the verb writes only a conservative allow-list: ASCII letters, digits, `-`,
`_` and `.`; starting with a letter; not ending in a dot; at most 64 characters (Tekla documents 1023).
Spaces, `#`, `/`, `:`, `\`, `*`, non-ASCII letters and control characters are never written. Names are
compared case-insensitively against what Tekla holds, because Tekla may treat a case-only difference
as the same shape.

## What happens, in order

1. The request is validated (above), including consent.
2. One reachable Tekla and an open model, as every verb. The model is **re-checked against `target`**
   (path, GUID from the model's `TeklaStructuresModel.xml`, shared flag, Tekla version); any difference
   is `target-changed` and nothing is written.
3. For each shape's derived name: absent (`would-create`); present with the same `aware.digest`, extents
   and vertices (`reused-identical`); present and different, or matched only by case (`conflict`:
   nothing written, nothing overwritten, both sides reported).
4. Tekla's own validator (`Polymesh.Validate`, every check) runs on each brep before it is offered.
   Tekla's `Insert` accepts an invalid brep, so this is the only guard. Holes are built as inner loops
   wound opposite to the cap's outer loop, the winding Tekla accepts. A rejection is `invalid-solid`.
5. `plan` stops here.
6. The catalogue files (`Shapes\`, `ShapeGeometries\`) are copied to
   `%LOCALAPPDATA%\aware\tekla-item-create\<model key>\<run id>\backup` and the copies verified; if that
   fails, `backup-failed` and nothing is written.
7. Each shape is inserted with `InsertOrGetGuidsOfShapesWithMatchingGeometry`, tagged with
   `aware.digest` and `aware.label`, and read back (a shape that reads back differently stops the run,
   `read-back-mismatch`). A shape whose geometry already exists under another name is **reused and
   reported** (`reused-by-geometry`, with its name); nothing is added for it.
8. Each Item is placed with `Position` Plane MIDDLE, Depth MIDDLE, Rotation FRONT (explicit: Tekla's
   defaults shift the Item by half the section), its origin on the start point and X running start to
   end, committed, and read back.

Writes land in **the model's own folder only**. A shared (multi-user) model is reported
(`target.shared_model`, `target.shared_note`): everyone on it sees the new shapes.

`placement.roll_deg` is applied as Tekla's rotation offset about the member axis; the mapping from a
FloLess roll to Tekla's sign was proved only for 0, so any other roll adds a `warnings` entry.

## The receipt

Always the same keys; `ok` is true only for `planned`, `created` and `deleted`.

| key | meaning |
|---|---|
| `status` | `planned`, `created`, `deleted`, `conflict`, `target-changed`, `invalid-solid`, `backup-failed`, `read-back-mismatch`, `shape-in-use`, `delete-count-mismatch`, `partial`, `failed` |
| `target` | `model_path`, `model_guid`, `shared_model`, `single_user_model`, `tekla_version`, `write_level` (`model`), `shared_note` |
| `shapes[]` | `name`, `derived_name`, `digest`, `length_mm`, `member_ids`, `status` (`would-create`, `created`, `reused-identical`, `reused-by-geometry`, `conflict`, `read-back-mismatch`), `guid`, `geometry_guid`, `metadata_set`, `message` |
| `items[]` | `member_id`, `item_guid`, `shape`, `status`, `verification` (`box_max_deviation_mm`, `volume_ratio`, `origin_to_start_mm`, `axis_dot`, `extent_along_axis_mm`, `shape_instance_count`, `uda_set`, `problems`) |
| `new_shape_count` | shapes this call adds (one per distinct length not already present) |
| `ledger[]` | every shape and Item this run wrote, in order: `kind`, `name`, `guid`, `geometry_guid`, `files[]` (`path`, `sha256_after`: hashed at the end, after the digest was written into the shape's record) |
| `backup.folder` | the pre-write copy of the catalogue files |
| `exists_afterwards[]` | a fresh read-back of everything in the ledger (not intention) |
| `undo` | `request`: the exact `delete` call for what this run made; `restore_files_instead`: the file route with Tekla closed |
| `warnings`, `message`, `error`, `conflicts`, `unledgered_files` | as named |

### A failure partway (`partial`)

Any failure after the first write gives `status: partial`. The verb **never retries and never rolls
back on its own**. `ledger` names exactly what was written (shape guids, their files, Items),
`exists_afterwards` reads it back, and `undo.request` is the delete that removes exactly those, Items
first, and carrying the `target` (model path and GUID) of the model the run wrote to, so run against any other
open model it is refused as `target-changed` rather than deleting that model's identical shape names. Shapes
the run only reused are not in it. An Item is in the ledger the moment Tekla accepts it, even if the commit or
the read-back then fails. If Tekla wrote files without confirming (the call threw
after writing), they are in `unledgered_files` and are **not** part of the undo: restore them from the
backup with Tekla closed. A failure before any write is `failed` with no undo.

## `delete`

```powershell
'{"mode":"delete","consent":{"source":"ui-click"},"items":["<guid>","<guid>"],"shapes":["AWI-…"]}' |
  aware-tekla item-create --json-stdin
```

`ShapeItem.Delete()` deletes a shape **even while Items use it** (measured live): the Items stay, empty
of profile. So the verb counts first: the Items still using the listed shapes once the listed `items`
are deleted. If that is above 0 and no acknowledgement came, `status: shape-in-use` lists them and
nothing is deleted. To go on, pass `delete_with_items` in the request, or `--delete-with-items <n>`
on the command line, equal to that count (several shapes: the total). A different number is
`delete-count-mismatch` and nothing is deleted. After a delete with Items still using the shape,
`items_using_shapes` lists the Items affected and a warning says they have no shape. A catalogue
backup is taken first. A shape that is absent is reported `absent`; a deleted Item cannot be restored,
only placed again.

## Not idempotent for Items

Shapes are reused, but `create` places an Item for every member each time: calling it twice places the
members twice.

## Exit codes

0 ok; 2 a refusal before Tekla (invalid request, consent, solid, digest, length), a structured
`ok: false` receipt (any status other than planned, created, deleted) or a script failure; 4 no single
reachable Tekla.
