# bake-scene

Lifecycle: single

Materializes a canonical millimetre scene in the open Revit model. This is a
write command and requires stable `scene.meta.sourceId` and `sceneHash` values,
a Revit instance with a project model open, and the AwareRevit add-in installed
(`cli-revit/install-addin.ps1`).

```powershell
$request | ConvertTo-Json -Depth 100 -Compress |
  aware-revit bake-scene --json-stdin
```

## What lands in the model

Element kinds `member`, `line` and `box` become native Revit elements. A member
whose axis is within one degree of plumb is placed as a **Structural Column**;
everything else is placed as **Structural Framing**, on a bound line between the
two scene points, hosted on the nearest `Level` at or below the member's start
elevation. A document with no level fails cleanly (`no-levels`) before anything
is written.

The profile designation is read from `profile`, then `meta.profile`, then
`meta.name`, and matched against the loaded family types on a **normalised**
name — upper-cased with whitespace removed. This matters: a real document ships
`W16X26` next to `W16x50`, and a case-sensitive match quietly loses half the
members. A matched type is activated (`FamilySymbol.Activate` + `Regenerate`)
before the first instance of it is placed.

When no loaded family type matches, the member still lands: it becomes a
`DirectShape` in Structural Framing carrying an extruded prismatic solid, the
emitted row is marked `fallback`, and a `warnings` row names the profile whose
family is not loaded. The solid's cross-section is a nominal placeholder read
off the designation — it keeps the member visible and selectable, it is not a
section model. Load the family and re-run to get the real section.

Everything the canonical scene can carry but this sink cannot build yet — plates,
rods, bolt shanks, washers, nuts, bolt heads, operations, and reference systems —
is reported in `unsupported`, one row per record. Nothing is dropped silently.

## Units, identity and ownership

Scene millimetres become Revit's internal decimal feet at a single conversion
point (`/304.8`). An explicit `units` other than `mm` is refused before any
model mutation; an absent `units` means legacy millimetres.

Every element the bake creates is stamped with an Extensible Storage entity on
the schema `AwareBakeV1`, carrying `SourceId`, `SceneHash`,
`MaterializationHash` and the per-record `RecordId`. That stamp is what makes
`bake-scene` **retire-and-replace** rather than append: the bake reads the set
this `sourceId` already owns *before* it creates anything, then deletes that
prior set once the replacement is in. Elements owned by a different `sourceId`
are never touched, and an element this bake just created is never retired.

The whole bake — symbol activation, placement, ownership stamps, and the
retirement — runs inside **one** Revit `Transaction`, so a user's Undo takes the
entire bake back in one step and any failure rolls the whole batch back (the
failed rows carry `rolledBack: true`).

## No status message

Tekla's `bake-scene` takes a `label` and puts "adding N objects…" on the status
bar. Revit deliberately does not: it has no status-bar equivalent, and
`TaskDialog` is modal — announcing the bake would block the very operation it
announces, inside an open transaction, on the API thread. Revit's own failure
dialogs are suppressed for the same reason (`SetForcedModalHandling(false)`).

## Receipt

The result carries the identity hashes, an `attemptId`, summary counts, and four
exhaustive arrays of `{id, kind, status, code?, message?}` rows — `emitted`,
`failed`, `unsupported`, `warnings` — plus `retired`, one row per element the
reconciliation deleted (`superseded` when the incoming scene still carries that
record, `removed` when it does not). Emitted rows additionally carry
`elementId`, `profile`, `category`, `level`, `fallback`, and the matched
`symbol`.

Validation codes match the Tekla sink where they apply: `host-unavailable`,
`invalid-scene`, `unsupported-units`, `missing-source-id`, `missing-scene-hash`,
`invalid-id`, `duplicate-id`, `invalid-record`, `invalid-collection`,
`invalid-geometry`, `unsupported-kind`, `batch-aborted`.

## A member's own labels (#818)

A `member`, `line` or `box` may carry its own labels in `meta`, under the same rules the
IFC writer applies (`aware render ifc`), so a record's labels are accepted or refused identically by
every sink that builds that record:

- `meta.description`: a string of 1..256 UTF-8 bytes with no C0 or DEL control characters.
- `meta.properties`: `{name, properties}`. The name and every key match
  `^[A-Za-z][A-Za-z0-9_]{0,63}$`; the name may not start with `Pset_` or `Qto_` in any case;
  the set holds 1..16 string values of at most 255 UTF-8 bytes each.

A malformed label fails that record with `invalid-member-label` and a message naming the
member and the rule, before anything is written, and so aborts the batch like any other
preflight failure. `meta.profile` is untouched by either label: it stays exactly the
catalogue key.

The receipt claims only what was written and read back unchanged. An emitted row gains
`description: true` and `propertySet: "<name>"` for what landed, and `labelsNotWritten:
[{field, reason}]` for anything that was sent and did not; each of those also appears in
`warnings[]` as a `member-label-not-written` row. A property set is all or nothing: if one
of its values does not land, the values already written are removed again.

In Revit:

- The description goes in the element's instance **Comments** parameter
  (`ALL_MODEL_INSTANCE_COMMENTS`), inside the bake's transaction, and is read back before the
  commit. An element without a writable Comments parameter reports it not written.
- A property set is **never written** and always reported in `labelsNotWritten`: Revit has
  no per-element place for a caller's property set, because each value needs a shared or
  project parameter bound to the element's category, and bake-scene adds no parameters to
  the project.
