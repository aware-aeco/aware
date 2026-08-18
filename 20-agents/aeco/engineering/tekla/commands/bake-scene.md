# bake-scene

Materializes a canonical millimetre scene in the open Tekla model. This is a
write command and requires stable `scene.meta.sourceId` and `sceneHash` values.

```powershell
$request | ConvertTo-Json -Depth 100 -Compress |
  aware-tekla bake-scene --json-stdin
```

The scene may contain exact-profile members, plates, rods, washers, nuts and
bolt heads; native bolt arrays, fillet welds and finite cylindrical Boolean
cuts; and structural grids with axes and elevation levels. Every element,
operation, reference, plate hole, bolt instance and hole effect needs a unique,
stable ID. Explicit units other than `mm` are rejected before model mutation.

Tekla stores one shared rectangular envelope per native Grid, while the canonical
scene may give each axis an independent `startMm` and `endMm`. The bridge inserts
the tightest rectangle that contains every authored segment and both families'
coordinate spans, so no grid line is truncated. When this necessarily lengthens
one or more native lines, the committed result includes a
`tekla-grid-axis-extents-expanded` warning. Every axis and level is emitted with
`realizedBy` pointing to the parent grid and shares that single native GUID.
Single-family grids, duplicate family offsets/elevations, blank labels, and
overflowing derived envelopes are exhaustively `unsupported` without aborting
unrelated supported records.

`bake-scene` owns its commit boundary. It stages source-tagged objects, applies
ownership UDAs after each successful `Insert()`, verifies GUID and tag read-back,
replaces only the previous set with the same source ID, and calls
`CommitChanges()` once. A failure before prior-set retirement deletes staging in
reverse order and commits cleanup, leaving the previous set intact. Tekla does
not expose rollback: a deletion/commit failure after retirement starts is
reported as `commit-state-uncertain`; the bridge deliberately issues no further
commit and requires a source-reconciliation retry. The generic `exec`
auto-commit does not wrap this command.

Members require an exact requested Tekla profile. Connection parts use exact
parametric or authored contour geometry and never fall back to `100*100`.
Profile candidates are accepted only after successful insertion and GUID
read-back; this path does not use `CatalogHandler`.

The result keeps the legacy summary fields and adds exhaustive `emitted`,
`failed`, `unsupported`, and `warnings` arrays. A child or hole effect realized
by a native bolt array carries the bolt array GUID and `realizedBy`, and is not
inserted as a duplicate physical object. Grid axes and levels use the same
parent-realization receipt shape and do not inflate the native object count.
