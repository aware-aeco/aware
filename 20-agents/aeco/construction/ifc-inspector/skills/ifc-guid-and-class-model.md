---
name: ifc-guid-and-class-model
description: This skill should be used when querying IFC entities by class or GUID with ifc-inspector — understanding the IFC GlobalId (the 22-char compressed UUID, not the #n line id), the class/subtype hierarchy that makes count-by-class report subtypes separately, and which authoring tools keep GUIDs stable across re-exports. Encodes the identity model behind entities.* verbs.
---

# IFC GUID & class model

## Two identifiers — never confuse them

Every IFC entity that matters for coordination descends from `IfcRoot` and has a **`GlobalId`**: a 22-character string that is a base64-compressed 128-bit UUID, e.g. `2O2Fr$t4X7Zf8NOew3FNr2`. (The compression uses a 64-char alphabet `0-9A-Za-z_$`, packing the 16-byte GUID into 22 chars.)

Separately, the STEP file gives each instance a **line id** `#n` (e.g. `#4711`). This is an internal index, meaningless outside the one file, and it changes on every re-export.

> **Always reconcile across tools on `GlobalId`, never on `#n`.** [`entities.list-guids`](../commands/entities.list-guids.md) and [`entities.get-by-guid`](../commands/entities.get-by-guid.md) take and return the `GlobalId`. `web-ifc` internally indexes by `#n` (its "expressID"); the agent builds a `GlobalId → expressID` map on open so callers only ever deal in `GlobalId`.

`GlobalId` is meant to be **persistent** — the same physical element keeps its GUID across re-exports, which is what lets a BCF viewpoint, a clash result, and a COBie row all point at the same wall. Whether that promise holds depends on the authoring tool (see stability below).

## The class hierarchy and the subtype gotcha

IFC classes form a deep inheritance tree: `IfcWall` is a subtype of `IfcBuildingElement` → `IfcElement` → `IfcProduct` → `IfcObject` → `IfcRoot`. Crucially, some "kinds" of thing split into sibling concrete classes:

- `IfcWall` **and** `IfcWallStandardCase` (the latter is a constrained subtype for prismatic walls — present in IFC2x3/IFC4, deprecated in IFC4X3 where everything is just `IfcWall`).
- `IfcSlab`, `IfcSlabStandardCase`, `IfcSlabElementedCase`.

[`entities.count-by-class`](../commands/entities.count-by-class.md) reports **direct class membership** — `IfcWall` and `IfcWallStandardCase` are separate rows, not rolled up. `web-ifc`'s `GetLineIDsWithType` returns exactly-this-type instances, not subtypes. Consequences:

- To count "all walls," sum the rows whose class matches `Wall` — exactly what the `filter` substring is for.
- [`entities.list-guids`](../commands/entities.list-guids.md) takes one **exact** class per call (no substring). List `IfcWall` and `IfcWallStandardCase` separately and union the results.

## Cross-tool GUID stability (the practical reality)

`GlobalId` is *designed* to persist, but authoring tools vary:

| Tool | Re-export GUID stability |
|---|---|
| Revit (via the IFC exporter) | Usually stable — but **group edits, copy/paste, and some element regenerations can change a GUID**. The exporter stores the IFC GUID in an `IfcGUID` parameter; if that's blank it's derived from the Revit UniqueId. |
| ArchiCAD | Stable — IFC ID is a first-class, persistent element property. |
| Tekla | Stable per part across exports. |
| Civil/infra & converters | Often **regenerate** GUIDs on each export — treat as unstable. |

The failure is silent: a churned GUID doesn't error, it just orphans every BCF viewpoint and clash result that pointed at the old one. [`file.info`](../commands/file.info.md)'s `source-app` is your first signal of which regime you're in. Where stability is doubtful, fall back to Pset-based identity (a stable `Pset` reference value) instead of the GUID — see [ifc-psets-and-cobie](./ifc-psets-and-cobie.md).

## See also

- [`entities.list-guids`](../commands/entities.list-guids.md) · [`entities.get-by-guid`](../commands/entities.get-by-guid.md) · [`entities.count-by-class`](../commands/entities.count-by-class.md) — the verbs this governs
- [ifc-psets-and-cobie](./ifc-psets-and-cobie.md) — Pset-based identity as a GUID fallback
- The [bcf-file](../../bcf-file/skills/bcf-round-trip-interop.md) agent's round-trip skill — the BCF side of GUID anchoring
