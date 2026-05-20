---
name: ifc-schemas-and-validation
description: This skill should be used when validating IFC files or reasoning about IFC schema versions with ifc-inspector — the IFC2x3 / IFC4 / IFC4X3 family, what EXPRESS schema validation catches vs what it cannot, Model View Definitions, and where schema validity sits in the overall quality stack (well-formed ≠ valid ≠ good). Encodes the meaning of validate.schema.
---

# IFC schemas & validation

## The schema family

IFC is defined in the EXPRESS data-modelling language. The versions you meet in 2026:

| Schema string | Common name | Notes |
|---|---|---|
| `IFC2X3` | IFC2x3 TC1 (2007) | Still the most widely exchanged; the safe lowest common denominator. No `IfcMapConversion`. |
| `IFC4` | IFC4 Add2 TC1 (2017) | Adds proper georeferencing (LoGeoRef 50), richer Psets, type objects. The current default for new work. |
| `IFC4X3` | IFC4.3 ADD2 (2024) | Adds infrastructure (alignment, road, rail, bridge, marine). Building tools are only starting to emit it. |

The version is declared in the header's `FILE_SCHEMA` and surfaced by [`file.info`](../commands/file.info.md) as `schema`. The **bundled `web-ifc` build** opens all three; intermediate drafts like `IFC4X1`/`IFC4X2` have variable support across `web-ifc` releases, so treat the supported set as a property of the pinned build, not a permanent fact — re-export odd drafts to IFC4 or IFC2X3.

## What `validate.schema` checks — and the engine's limit

`ifc-inspector` wraps the `web-ifc` parser, which is a **parser, not a full EXPRESS schema engine**. So [`validate.schema`](../commands/validate.schema.md) covers the structural problems a parser can see:

- **Well-formedness** — the STEP / `ISO-10303-21` syntax parses at all.
- **Schema membership** — every entity type exists in the declared schema (no IFC4-only class smuggled into an IFC2x3 file).
- **Missing required *direct* attributes** — an `IfcWall` with no `GlobalId`.
- **Reference integrity** — no `#n` reference pointing at a line that doesn't exist (orphans); no circular containment.
- **Basic type conformance** — a string where an enumeration label is expected.

What it does **not** do: full EXPRESS `WHERE`-rule evaluation, derived-attribute checks, inverse-attribute cardinality, or function-based constraints. Those need a dedicated schema engine — **IfcOpenShell's `ifcopenshell.validate`** or the **buildingSMART validation service** — not a geometry/data parser. If your acceptance gate hinges on WHERE-rule conformance, route to one of those; `validate.schema` is the fast, in-process well-formedness + reference-integrity pre-check, not a certification.

## What it cannot check

This is the load-bearing distinction. Schema validation proves the file is **well-formed**, not that it is **good**:

| Question | Tool |
|---|---|
| Is this a structurally valid IFC4 file? | `validate.schema` (this verb) |
| Does every space have the Psets our BEP requires? | [`psets.find-missing`](../commands/psets.find-missing.md) |
| Is this model georeferenced correctly? | [`georef.check`](../commands/georef.check.md) |
| Do the walls clash with the ducts? | [`navisworks-2026`](../../navisworks-2026/) / [`solibri`](../../../architecture/solibri-26.4.1/) |
| Does it satisfy our coordination rule set / classification? | [`solibri`](../../../architecture/solibri-26.4.1/) rule engine |
| Is the wall thickness correct? | nobody — that's professional judgment |

`valid: true` means the STEP/EXPRESS structure is sound. A perfectly valid file can still be missing every property you need, be at the wrong coordinates, and clash everywhere. **Never report "validated" as "fit for purpose."**

## Model View Definitions (MVDs)

An MVD is a *subset* of the full schema agreed for an exchange — e.g. `CoordinationView 2.0` (the common architectural/MEP coordination subset), `ReferenceView`, `DesignTransferView`. The MVD is named in `FILE_DESCRIPTION` (`ViewDefinition [CoordinationView]`). `validate.schema` validates against the **full schema**, not the declared MVD — so it won't tell you "this file claims CoordinationView but includes entities outside it." MVD-conformance checking is out of scope here; treat the `ViewDefinition` string as metadata, and if MVD conformance matters, use a dedicated buildingSMART validation service.

## The quality stack (run cheap-to-expensive)

1. [`file.info`](../commands/file.info.md) — ~1s. Opens? Right schema? Right source app?
2. [`entities.count-by-class`](../commands/entities.count-by-class.md) — is the content plausible (rooms present, not 0 walls)?
3. [`georef.check`](../commands/georef.check.md) / [`psets.find-missing`](../commands/psets.find-missing.md) — targeted correctness.
4. [`validate.schema`](../commands/validate.schema.md) — ~5 min on 500 MB; full structural walk. Schedule overnight for large federations.

Stop at the first cheap check that fails — no point full-validating a file that has zero `IfcSpace`.

## See also

- [`validate.schema`](../commands/validate.schema.md) — the verb
- [`file.info`](../commands/file.info.md) — where the schema string comes from
- [what-this-agent-does](./what-this-agent-does.md) — the data-only scope vs `solibri`'s rules
