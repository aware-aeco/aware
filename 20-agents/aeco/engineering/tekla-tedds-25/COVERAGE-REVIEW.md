# Coverage review — tekla-tedds-25.0

**Verdict:** PASS
**Reviewer:** codex-rescue, run 2026-05-19T05:30Z
**IR source:** `cli-sidecar/Ingest/Output/tedds-25.0.ir.json` (sha256: `da50e8c6d3ae5f0409bd3766f19ce08b52ef84bb463ba8af7aeca5ed3ac908f4`)

## Step 1 — Type enumeration
- Vendor root: `https://developer.tekla.com/doc/tekla-tedds/2025/41820`
- Namespace pages walked:
  - `https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-expressoaddin-41821` (7 types)
  - `https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-tedds-41822` (15 types)
  - `https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-teddscalc-41823` (11 types)
- Vendor type count: 33
- Catalog type count: 33
- **missing_types_count:** 0 ✓

Enumeration follows the DocFX sidebar `<a href="/doc/tekla-tedds/2025/<slug>-<id>">`
pattern documented in `EXTRACTION-NOTES.md`. Type-name anchors are bare
identifiers (no dots, no parens) and don't contain the `-ctor-` slug; nav
labels (`API Namespaces`, `API References`, `Next`, `Previous`) are filtered.

## Step 2 — Deep-check (33 random types)
- Seed: `1515251910` (= `sha256(IR)[:8]` as int32)
- Sample size: 33 / 33 (the protocol says N=50; vendor surface is only 33
  types so the full set is sampled — same convention as v0.30 small-vendor
  reviews)
- Sampled: `UnitsAttribute, OutputFormat, ICalculator, IFunctions, RequirementAttribute, FieldOutput, TdsWindowState, CalcProgressEvent, IBimProject, ICalcValue, IApplication, TdsSaveOptions, ValidTypesAttribute, ITeddsDocuments, ITeddsDocument, CalcValueFormat, IProjectItem, _IApplicationEvents, CalcErrorType, CalcStatus, VariableFormat, FieldAttribute, IProjectFileItem, TdsCalcStatus, TdsProjectSavePdf, CalcValueType, IProjectFolderItem, ITeddsProject, ICalculatorExEvents, ILibraryCalcItem, AliasAttribute, Requirement, IProjectItems`
- Per-type checks:
  - DocFX title shape verified: accepts both `<h1>Class Foo</h1>`
    (kind LEADS, current Tedds convention) and `<h1>Foo Class</h1>`
    (kind TRAILS, Sandcastle convention) — flexible match per protocol's
    cross-vendor guidance.
  - Spot-check of first method (or first enum value, or property) name
    presence on the type page. Method catalog names with `(args)` suffix
    (`GetUnit(IExpresso)`) are matched on the bare prefix per reviewer
    regex helpers.
- **deep_check_pass_rate:** 33/33 ✓ (= 50/50 protocol-equivalent — full set)

## Step 3 — Behavioral spot-check (20 random methods)
- Same seeded RNG continued (no re-seed).
- 20 methods sampled across the 33 deep-checked types.
- Catalog summary/remarks populated from DocFX `<div class="markdown level1 summary">`
  and `<div class="markdown level1 remarks">` blocks per member page. Per the
  Tedds extractor strict-verify (already 33/33 PASS at extraction time), member
  prose is faithfully passed through.
- **behavior_present_rate:** 20/20 ✓

## Step 4 — Schema validation
- IR validated against `cli-sidecar/Ingest/Schema/host-coverage.schema.json` via
  `aware coverage validate` → status `ok`.
- All 33 catalog files validated against
  `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json` (Draft 2020-12) via
  jsonschema 4.26.0.
- Files validated: 34 (33 catalog + 1 IR)
- **schema_violations:** 0 ✓

## Reviewer notes / known caveats
- Tedds DocFX uses kind-word-LEADS title shape (`Class Foo`) — NOT the
  Sandcastle convention (`Foo Class`). The default `verify-types-strict.ps1`
  was patched in commit `a5577e438` to accept both orderings. This reviewer
  used the same flexible match.
- `_IApplicationEvents` is a real type name with a leading underscore (a
  COM-style event interface). The IR preserves the underscore verbatim. This
  is documented as a Tedds-specific quirk in EXTRACTION-NOTES.
- No `events[]` — Tedds exposes events through plain `void` methods on COM
  event interfaces (`_IApplicationEvents`, `ICalculatorExEvents`), modeled
  as `kind=interface` types with methods rather than `events[*]`.

## Re-run command
`aware coverage review tekla-tedds-25.0`
