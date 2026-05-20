---
name: what-this-agent-does
description: This skill should be used when deciding whether ifc-inspector is the right tool for an IFC task — its data-only scope vs the web-ifc renderer and the solibri rule engine, when to reach for each, how the WASM is bundled, what "validation" means here, and the performance envelope on large files. Encodes the agent's boundaries.
---

# What this agent does (and doesn't)

The `ifc-inspector` agent is the **data-side** companion to the
existing `web-ifc` visualization agent.

| Task | This agent | `web-ifc` | `solibri` |
|---|---|---|---|
| Count entities by class | ✓ | partial | ✓ |
| List GUIDs | ✓ | ✗ | ✓ |
| Extract Psets to CSV | ✓ | ✗ | partial |
| Check georeferencing | ✓ | ✗ | ✓ |
| Schema validation | ✓ | ✗ | ✓ |
| Rule-based BEP checking | ✗ — use `solibri` | ✗ | ✓ |
| 3D rendering | ✗ — use `web-ifc` | ✓ | ✓ (UI only) |
| Clash detection | ✗ — use `navisworks-2026` | ✗ | ✓ |

## When to reach for `ifc-inspector` instead of `solibri`

- **You don't have a Solibri licence.** This agent works against any
  IFC file without paid software.
- **You only need schema-level or Pset-level checks.** Solibri's value
  is its rule-engine ruleset library; if your check is "do all spaces
  have a Pset_SpaceCommon.Name?" you don't need the rule-engine.
- **You're in CI.** This agent is a single Rust binary loading WASM —
  spins up in 50ms. Solibri needs a desktop install.

## When to reach for `solibri` instead of this agent

- **Your BEP encodes business rules** (cross-discipline coordination
  checks, classification compliance, code-derived rules). Solibri's
  rule engine + ruleset library are the right tool.
- **The deliverable is a BCF for your client's pipeline.** Solibri's
  BCF export is richer (viewpoints, severity mapping, BCF 3.0).

## How the WASM is bundled

The `aware-ifc-inspector` transport binary embeds the `web-ifc` WASM
inline — no separate runtime install. The binary is ~8 MB
self-contained. First invocation initialises the WASM (~200ms);
subsequent calls reuse the loaded module within a single CLI process.

## What "validation" means here

`validate.schema` runs the IFC through the `web-ifc` parser — a parser,
not a full EXPRESS schema engine — so it catches the structural problems
a parser can see:

- well-formedness (the STEP syntax parses)
- schema membership (entity types exist in the declared schema)
- missing required direct attributes (e.g. an IfcWall with no GlobalId)
- reference integrity (orphan #refs, circular containment)
- basic type conformance

It does **not** catch:

- full EXPRESS WHERE-rule / cardinality constraints (needs IfcOpenShell
  `validate` or the buildingSMART validation service — not web-ifc)
- BEP compliance (Solibri's job)
- "the wall thickness is wrong" (no rule)
- "this IfcSpace has no name" (Pset-level — use `psets.find-missing`)

## Performance

- `file.info` on a 500 MB IFC: ~1s (header-only read)
- `entities.count-by-class` on 500 MB: ~30s
- `validate.schema` on 500 MB: ~5 minutes (full graph walk)
- `psets.export` on 500 MB with 5000 spaces: ~2 minutes

For routine work on large federations, prefer running this overnight
via v0.19's `schedule:` primitive.

## See also

- [ifc-guid-and-class-model](./ifc-guid-and-class-model.md) — GlobalId encoding + the class/subtype model the query verbs rely on
- [ifc-psets-and-cobie](./ifc-psets-and-cobie.md) — property sets, instance vs type, COBie naming
- [ifc-georeferencing](./ifc-georeferencing.md) — the LoGeoRef levels behind `georef.check`
- [ifc-schemas-and-validation](./ifc-schemas-and-validation.md) — what schema validation does and doesn't prove
