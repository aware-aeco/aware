# Native connection and structural-grid scene export

**Status:** approved upstream design; AWARE implementation contract
**Date:** 2026-07-15
**Base:** `v0.89.1` / `ca08140d`
**Consumer:** FloLess connection and structural-grid export

## Goal

Extend the neutral 3D scene consumed by `tekla.bake-scene`, `ifc.write`, and
`viewer-3d.render` beyond axis members. The same scene must describe editable
plates and fastener solids, native connection operations, and structural grids.
Every sink must account for every stable input identity; it may not silently
skip a supported record or replace exact geometry with `100*100`.

## Scene contract

All coordinates and dimensions are millimetres. Missing `meta.units` retains
legacy millimetre behaviour. Tekla and IFC reject an explicit unit other than
`mm` until a reviewed conversion is added.

```text
scene.meta = {
  name?: string,
  units?: "mm",
  sourceId: string,
  sceneHash: string
}

scene.elements[] =
  | legacy member {
      id, kind?: "member", from:[x,y,z], to:[x,y,z], section?, role?,
      material?, teklaClass?, meta?
    }
  | plate {
      id, kind:"plate",
      frame:{ origin:[3], uDir:[3], vDir:[3], normal:[3] },
      outline:[[u,v]...], thicknessMm, material?,
      holes:[{ id, center:[u,v], diameterMm }]
    }
  | cylinder {
      id, kind:"rod"|"bolt-shank",
      axis:{ from:[3], to:[3] }, diameterMm, profile?, material?, fastener?
    }
  | annulus {
      id, kind:"washer", center:[3], axis:[3], outerDiameterMm,
      innerDiameterMm, thicknessMm, material?, fastener?
    }
  | hex prism {
      id, kind:"nut"|"bolt-head", center:[3], axis:[3], acrossFlatsMm,
      thicknessMm, phaseRad?, material?, fastener?
    }
  | existing line|box|node|mesh records
```

For a regular hexagon, circumradius is `acrossFlatsMm / sqrt(3)`. The first
vertex lies on local `+u`; subsequent vertices advance by 60 degrees in the
right-handed axis frame. Every sink uses this phase.

`plate.frame.origin` is the authored plate mid-plane origin. The solid spans
`-normal * thicknessMm/2` through `+normal * thicknessMm/2`; it is not a
lower-face placement.

```text
scene.operations[] =
  | {
      id, kind:"bolt-array", partToBoltTo, partToBeBolted,
      frame:{origin:[3],uDir:[3],vDir:[3],normal:[3]},
      uOffsetsMm[], vOffsetsMm[], diameterMm, standard, grade?, toleranceMm,
      boltType:"shop"|"site", threadInMaterial?, components,
      instances:[{
        id, point:[3], shankId, headId, nutIds[], washerIds[],
        holeEffects:[{id,targetId,center:[3],axis:[3],diameterMm}]
      }]
    }
  | {
      id, kind:"weld", mainId, secondaryId, path:[[3]...],
      weldType:"fillet", sizeMm, around:boolean, shop:boolean
    }
  | {
      id, kind:"boolean-cut", targetId,
      tool:{kind:"cylinder",axis:{from:[3],to:[3]},diameterMm}
    }
```

Physical bolt children remain in `elements[]` for IFC and viewer output.
Tekla realizes them through the `BoltArray` and reports the children with
`realizedBy`; it does not insert duplicate child parts. Every bolt position has
a distinct opening effect for every participant it perforates.

```text
scene.referenceSystems[] = {
  id, kind:"structural-grid", name?, origin:[3],
  axes:[{
    id, direction:"x"|"y", offsetMm, label, startMm?, endMm?
  }],
  levels:[{id,elevationMm,label}], // at least one
  bounds:{minX,maxX,minY,maxY,minZ?,maxZ?},
  isMagnetic?:boolean
}
```

`offsetMm` is relative to the reference-system origin. `elevationMm` is an
absolute model-space Z datum. All nested records have stable IDs.

## Identity and preflight

Before output or mutation, every sink validates every record ID, including
plate holes, bolt instances/effects, grid axes, and levels:

- nonempty string after trimming;
- 1–256 UTF-8 bytes;
- no C0 control character or DEL;
- globally unique across all scene collections;
- every relationship target exists and has the required physical kind.

Recognised supported malformed geometry is a failure, never a warning or
silent skip. Unknown kinds are intentionally unsupported.

Tekla stores the stable record identity as
`AWARE_RID_V1:<sha256(full UTF-8 record ID)>` in `USER_FIELD_2`. This fits the
host's short string UDA while preserving the full 256-byte scene-ID contract;
the digest is read back exactly before commit.

## Exhaustive receipt

Each input ID appears exactly once as emitted, failed, or unsupported. Ordered
rows follow scene order.

```json
{
  "ok": true,
  "emitted": [
    {"id":"...","kind":"...","status":"emitted","entityType":"...","nativeGuid":"...","realizedBy":"..."}
  ],
  "failed": [
    {"id":"...","kind":"...","status":"failed","code":"...","message":"...","rolledBack":true}
  ],
  "unsupported": [
    {"id":"...","kind":"...","status":"unsupported","code":"...","message":"..."}
  ],
  "warnings": [
    {"id":"...","code":"...","message":"..."}
  ]
}
```

Legacy count/path/profile fields remain additive compatibility fields. A batch
failure returns `ok:false`, no durable output, and `emitted:[]`; supported IDs
are classified as the causal failure, `batch-rolled-back`, or `batch-aborted`.

## Tekla materialization

- Legacy members remain `Beam`, but the requested profile must insert exactly;
  there is no `100*100` fallback.
- Plates are `ContourPlate` with exact authored outline, frame, thickness, and
  independent openings.
- Base hardware without a physical foundation target remains profile-backed.
- Shear fasteners become one native `BoltArray` per operation.
- Weld operations become `Weld` or `PolygonWeld` according to path shape.
- Cope operations become `BooleanPart` with a finite operative solid whose
  class is `BooleanPart.BooleanOperativeClassName`; the transient original
  operative part is deleted after insertion.
- Structural grids become owned `Grid`/`GridPlane` records and are nonmagnetic
  by default.

`bake-scene` owns the mutation boundary. Generic `exec` retains its existing
post-script auto-commit, but the bake script does not. Bake stages new objects,
sets ownership UDAs after each successful `Insert()` and before commit, and
requires the namespaced `AWARE_BAKE_V1:<sha256>` signature plus source, record,
and scene fields before an existing object is eligible for retirement. It
verifies every GUID and resolved profile/relationship, deletes the previous
complete source-owned set, then calls `CommitChanges()` once. Failure deletes
staged objects in reverse dependency order before the recovery commit and
preserves the old complete set. Present scene collections must be arrays; a
malformed envelope fails preflight and can never retire prior objects.
All model-space insertion and readback runs under a temporary global
transformation plane, with the user's prior work plane restored in `finally`.

`materializationHash` covers the scene hash plus resolved profiles, operations,
reference systems, materializer version, host version, and environment. Retry
recovery requires authoritative GUID readback, not tags alone.

## IFC materialization

- plate → `IfcPlate`;
- anchor rod → `IfcMechanicalFastener(.ANCHORBOLT.)`;
- standalone shear hardware → individual `IfcMechanicalFastener` products;
- canonical bolt array → one `IfcMechanicalFastener(.BOLT.)` containing every
  authored shank, head, washer, and nut solid across all array positions, so the
  array is one selectable IFC product like Tekla's native export;
- grouped bolt components must resolve to one normalized authored material;
  mixed-material arrays are rejected before output because the selectable
  `IfcMechanicalFastener` carries one material association;
- each hole effect → its own `IfcOpeningElement` and `IfcRelVoidsElement` for
  exactly one target product;
- grid → `IfcGrid` plus `IfcGridAxis`;
- elevation datum → `IfcAnnotation`, `ObjectType='Elevation datum'`, placed at
  exact Z with an `IfcGeometricCurveSet` crosshair spanning grid bounds.

One opening cannot void two IFC products, so plate-side and member-side effects
must be distinct opening entities. Weld and Boolean relationships are reported
unsupported until an exact reviewed IFC mapping exists. Bolt child record IDs
remain exhaustive receipt rows and resolve to their containing bolt-array IFC
GlobalId rather than becoming separately selectable products.

GlobalIds for stable input records derive deterministically from record IDs,
not allocation order. The release fixture is validated with
`ifcopenshell==0.8.5` and `ifcopenshell.validate`.
Receipt order is parent-first and follows the scene: grid, axes, then levels,
even though the SPF must allocate axis entities before `IfcGrid`.

## Viewer materialization

The canonical viewer renders exact plates with authored holes, cylinders,
annuli, deterministic hex prisms, weld paths, structural axes, and elevation
references. It preserves legacy scene kinds and legacy label-only `grids[]`.
Operations never duplicate their physical children. Exact arbitrary Boolean CSG
may be reported unsupported until a deterministic implementation is present.
For Z-up scenes the Y/Z conversion is reflective; oriented profiles transform a
deterministic source basis and compensate `phaseRad` so hex hardware is not
mirrored relative to IFC or Tekla.

## Live capability evidence

Live Tekla Structures 2025.0, USA environment, single-user model
`FloLess Demo`:

- exact member insertion/readback: `W14X22`;
- exact plate insertion/readback: `PL10` and `PL0.375`;
- native `BoltArray`: two participants, four positions, exact standard/size
  round-trip;
- native `Weld` and `PolygonWeld`: participant and path readback;
- native `BooleanPart`: father/type readback; operative class constant is
  `BooleanOperativeClassName`;
- native `Grid` and manual `GridPlane`: coordinates, labels, father, and GUID
  readback;
- all committed probe objects were removed by compensating cleanup.

The spike also proved two implementation corrections:

1. Ownership `SetUserProperty` returns `false` before `Insert()` but succeeds
   immediately after insertion and persists through commit. Bake therefore tags
   after insertion but before its one commit.
2. `Tekla.Structures.Catalogs.dll` now resolves, but
   `CatalogHandler.GetLibraryProfileItems()` and
   `LibraryProfileItem.Select()` terminate the sidecar without a receipt on the
   live 2025 host. Production must not call these APIs. Exact insert + commit +
   GUID/profile readback is the supported resolver until issue #283 is fixed.

The bundled 2025/2026 API surfaces required here are identical. Tekla Structures
2026 supports only x64 Open API extensions, so both `aware-tekla` and its test
host now target x64 explicitly.

Live Tekla Structures 2026 SP3.1 build 61483, process 26620, model `FloLess 1`:

- the x64 release bridge established a real Open API connection and compiled the
  full embedded materializer;
- two consecutive identical bakes returned the same
  `materializationHash=0a758d5d21b2ca562f942c201fd354d572974420eb8bd9b7e00d83773ba6b843`;
- each successful bake classified all 41 input records as emitted, with zero
  failed, unsupported, or warning rows;
- independent Open API readback found exactly seven source-owned native objects:
  `Beam`, `ContourPlate`, `BoltArray`, `PolygonWeld`, `Grid`, and two
  `BooleanPart` cuts;
- exact readback proved `W14X22`, `PL10`, four bolt positions at
  `(50900|51100, -100|100, 0)`, `A325N`, diameter 19.05 mm, tolerance 2 mm,
  the two-point 6 mm fillet weld, both Boolean fathers, and grid coordinates
  `0 3000` with labels `A B`, `1 2`, and `+0 +3000`;
- a third replacement bake followed by independent enumeration still found
  exactly those seven owned native objects, proving duplicate-free retirement.
- the same bake executed while a translated/rotated work plane was active;
  readback remained at authored global coordinates and the plane was restored
  exactly before the test returned Tekla to its original global plane.

The live retry also exposed and fixed two Tekla-specific placement/lifecycle
details: `BoltArray` Y spacings are centred about its reference line, and prior
weld/bolt/cut relationships must retire before their participant parts because
Tekla cascades relationship deletion from a part.
