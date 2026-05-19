# Coverage review — tekla-tedds-26.0

**Verdict:** PASS
**Reviewer:** codex-rescue, run 2026-05-19T05:30Z
**IR source:** `cli-sidecar/Ingest/Output/tedds-26.0.ir.json` (sha256: `4894ffea2f45e729abe41e6233325636f9a83c539cb5fa9452fcd14830fb2bb4`)

## Step 1 — Type enumeration
- Vendor root: `https://developer.tekla.com/doc/tekla-tedds/2026/64124`
- Namespace pages walked (discovered from the root sidebar):
  - `https://developer.tekla.com/doc/tekla-tedds/2026/tekla-structural-expressoaddin-64125` (7 types)
  - `https://developer.tekla.com/doc/tekla-tedds/2026/tekla-structural-interopassemblies-tedds-64126` (15 types)
  - `https://developer.tekla.com/doc/tekla-tedds/2026/tekla-structural-interopassemblies-teddscalc-64127` (11 types)
- Vendor type count: 33
- Catalog type count: 33
- **missing_types_count:** 0 ✓

Per `EXTRACTION-NOTES.md`: TSD-26 surface is identical to TSD-25 — same 33
types, same 77 methods, same 71 properties. Only the underlying assembly
version + topic IDs change. Two distinct agents are published for honest
version pinning.

## Step 2 — Deep-check (33 random types)
- Seed: `1217724394` (= `sha256(IR)[:8]` as int32)
- Sample size: 33 / 33 (full vendor surface — same N=50-equivalent
  convention as v25)
- Sampled: `IApplication, FieldAttribute, Requirement, ICalculatorExEvents, CalcErrorType, CalcProgressEvent, TdsSaveOptions, AliasAttribute, _IApplicationEvents, TdsProjectSavePdf, CalcValueFormat, IProjectItem, ICalculator, TdsWindowState, IProjectFileItem, IProjectFolderItem, ILibraryCalcItem, TdsCalcStatus, RequirementAttribute, ITeddsProject, VariableFormat, UnitsAttribute, IProjectItems, IFunctions, IBimProject, ITeddsDocuments, ITeddsDocument, OutputFormat, CalcStatus, FieldOutput, ValidTypesAttribute, CalcValueType, ICalcValue`
- Per-type checks: DocFX title shape (kind-LEADS / kind-TRAILS both accepted),
  member spot-check (method bare name / enum value name / property name).
- **deep_check_pass_rate:** 33/33 ✓

## Step 3 — Behavioral spot-check (20 random methods)
- Same seeded RNG continued (no re-seed).
- 20 methods sampled across the 33 deep-checked types.
- Catalog summary/remarks from DocFX per-member pages, faithfully
  pass-through per the Tedds extractor.
- **behavior_present_rate:** 20/20 ✓

## Step 4 — Schema validation
- IR validated against `cli-sidecar/Ingest/Schema/host-coverage.schema.json` via
  `aware coverage validate` → status `ok`.
- All 33 catalog files validated against
  `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json` (Draft 2020-12) via
  jsonschema 4.26.0.
- Files validated: 34 (33 catalog + 1 IR)
- **schema_violations:** 0 ✓

## Reviewer notes
- Same DocFX-vs-Sandcastle title-shape caveat as v25 — kind word LEADS on
  Tedds (`Class Foo`), not Sandcastle's TRAILS (`Foo Class`). The
  `verify-types-strict.ps1` was patched (commit `a5577e438`) to accept both
  orderings; this reviewer uses the same flexible match.
- v25 and v26 type SHAs are different (`da50e8c6…` vs `4894ffea…`) because
  the doc_url topic IDs change between versions (e.g. `…-41824` vs `…-64128`);
  the structural payload (type names, member shapes, prose) is otherwise
  identical.

## Re-run command
`aware coverage review tekla-tedds-26.0`
