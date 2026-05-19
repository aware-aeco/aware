# v0.30 Phase B Complete — 2026-05-19

**v0.30 Phase B (14-vendor host-agent rework) is complete.** All 12 vendor extractors implemented; 21 agent versions PASS the host-coverage-review-protocol on branch `feature/v0.29-runtime-hello-world`.

## What's shipped

### 12 vendor extractors, 21 agent versions, all PASS

| Tier | Vendor | Versions | Source style | Protocol verdict |
|---|---|---|---|---|
| A | Tekla Structures (B1) | 2025, 2026 | Sandcastle (developer.tekla.com) | PASS / PASS |
| A | SketchUp (B4) | 2025, 2026 | Ruby YARD (ruby.sketchup.com, unified site) | PASS / PASS |
| A | Rhino (B5) | 7, 8 | Sandcastle (github mcneel/rhinocommon-api-docs) | PASS / PASS |
| A | Grasshopper (B6) | 7, 8 | Sandcastle (github mcneel/grasshopper-api-docs) | PASS / PASS |
| A | ArchiCAD (B7) | 28, 29 | Hybrid Tapir GitHub + Graphisoft DevPortal | PASS / PASS |
| A | IDEA Statica (B8) | 25, 26 | Hybrid OpenAPI YAML + markdown SDK | PASS / PASS |
| A | Tekla Tedds (B9) | 25, 26 | DocFX (developer.tekla.com/doc/tekla-tedds) | PASS / PASS |
| B | Allplan (B10) | 2024, 2025 | mkdocs-material/mkdocstrings (pythonparts.allplan.com) | PASS / PASS |
| B | Dynamo (B11) | 4.1.0, 4.1.1 | Hybrid NuGet PE metadata + GitHub source | PASS / PASS |
| B | Navisworks (B12) | 2026 | NuGet PE metadata (Chuongmep.Navis.Api) | PASS |
| B | Solibri (B13) | 26.4.1 | OpenAPI YAML (solibri.github.io/Developer-Platform) | PASS |
| B | Bluebeam (B14) | v1 | Postman v2.1 JSON collection | PASS |

**Total:** ~9,200+ types extracted, ~190,000+ enriched members. Catalog: ~9,500+ JSON files. Skills: ~280+ markdown files.

## v0.30 Phase B build + test state at completion

- **Sidecar build:** `dotnet build -c Release` → 0 warnings, 0 errors
- **NativeAOT publish:** `dotnet publish -c Release -r win-x64 --self-contained` → 0 warnings, 0 errors
- **Sidecar tests:** 138 cli-sidecar.Tests + 44 Ingest/Generator/Tests = 182 tests, all PASS
- **Cargo E2E:** `cargo test --test coverage_e2e --release` → 1 PASS

## Reusable infrastructure shipped during Phase B

### Common helpers (`cli-sidecar/Ingest/Extractors/Common/`)
- `HttpScraper.cs` — AOT-clean HTTP wrapper (replaces Playwright; verified Playwright is NOT AOT-compatible)
- `IRBuilder.cs` — IR accumulator (count tracking, source-URL bookkeeping)
- `ChmParser.cs` — `hh.exe -decompile` wrapper for CHM-based vendors (unused in v0.30; Navisworks pivoted to NuGet)
- `NuGetFetcher.cs` — generic NuGet downloader (built for Navisworks, reusable for Revit/AutoCAD/Civil 3D)
- `XmlDocLoader.cs` — XML doc comment parser (PE-metadata extraction pattern)
- `MetadataReflector.cs` — generic PE-metadata reflector with full ISignatureTypeProvider + GenericContext resolution

### Foundation (Phase A, updated during B)
- `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json` — standalone catalog Type schema (added during B1)
- `cli-sidecar/Ingest/Generator/CatalogWriter.cs` — now throws `InvalidOperationException` on filename collision (added during B11 defect-fixer)
- `cli-sidecar/Ingest/Generator/Models.cs` — `[JsonIgnore(Condition = JsonIgnoreCondition.Never)]` on `MethodInfo.returns` for schema compliance
- `cli-sidecar/Ingest/Generator/{CatalogWriter, ManifestWriter, SkillWriter}.cs` — Ruby `::` namespace + `?`/`!`/`=` suffix support (added during B4 SketchUp)

### Tooling (`cli-sidecar/Ingest/Output/`)
- `extract-with-restart.ps1` — auto-restart wrapper for CDN throttle handling
- `verify-types-strict.ps1` — 50-type strict sampler with multi-source dispatch (Sandcastle HTML, DocFX, mkdocs-material, markdown, YAML, JSON, NuGet, source-code)
- `enrichment-stats.ps1` — member fill-rate reporter

### Protocol (`docs/superpowers/specs/host-coverage-review-protocol.md`)
The codex-coverage protocol grew during v0.30 Phase B as real defects surfaced. Final state has vendor-rule sections for:
- Sandcastle (Rhino, Grasshopper, AutoCAD, Navisworks — for HTML pages)
- DocFX (Tekla Tedds)
- Hybrid OpenAPI/markdown (IDEA Statica)
- mkdocs-material/mkdocstrings (Allplan, future Python SDKs)
- Sample-size adaptation (vendors with N < 50)
- SyncRoot whitelist (`.NET BCL` legitimate `object` properties)
- LST regex (Tekla script-substitution)
- Clustered-FAIL investigation guidance

## Defects caught + fixed during Phase B

The codex-coverage protocol caught **real defects** before shipping:

1. **Tekla LST hardcoded `.` substitution** (B1) — constructors lost overload disambiguator; generics collapsed angle brackets to dots → fixed PageParser to read actual `cs=X` token
2. **Tekla `returns` field omitted from JSON when null** (B1) — schema violation on every ctor + void method → added `[JsonIgnore(Condition = Never)]` to MethodInfo.returns
3. **Tekla cpp-only-LST trailing dot** (B1) — 0.015% of methods had `Validation.` instead of `Validation` → fall back to empty string when no `cs=` key
4. **Rhino external-URL inherited members** (B5) — `System.Object` inherited methods (Equals, ToString) tried to fetch from docs.microsoft.com → IsExternalUrl filter
5. **SketchUp YARD generator support** (B4) — needed Ruby `::` namespace and `?`/`!`/`=` suffix support → extended CatalogWriter/ManifestWriter/SkillWriter
6. **ArchiCAD JS-rendered SPA** (B7) — Tapir + Graphisoft docs are JS-rendered, no HTML body → pivoted to JSON-RPC schema sources directly
7. **Allplan dual mkdocs template** (B10) — 2024 legacy vs 2025 current have different DOM shapes → parser supports both
8. **Dynamo nested-type namespace collision** (B11) — 5× `State` enums collapsed to one catalog entry → qualify nested-type namespaces with declaring-type FQN
9. **CatalogWriter silent collision** (B11 follow-up) — defensive fix: throw on filename collision instead of last-write-wins

Each defect was caught by codex-coverage Step 1/2/3/4 metrics + spot-checks; none escaped to a ship-ready agent.

## Naming conventions established

- **Tekla Structural Designer** (`tsd-25`, `tsd-26`) is a different product from **Tekla Tedds** (`tekla-tedds-25`, `tekla-tedds-26`). Both Trimble products but distinct. Existing `tsd-25/26` agents (Structural Designer) are NuGet-generated, untouched by B9.
- **SketchUp C SDK** intentionally skipped per project memory (EULA + login gate). B4 covers the Ruby API only.
- **Allplan BIF.Core** (.NET cloud integration) is a separate surface from **Allplan PythonParts** (in-app scripting). B10 covers PythonParts; the BIF.Core agents from v0.28 era remain at `allplan-2024` directory but should be moved aside if BIF.Core needs to keep shipping.
- **Catalog filename sanitization:** `_of_` for `<`, strip `>` (Windows-safe for generic types).
- **Agent dir naming:** `<vendor>-<version>` where version is the short form (e.g. `tekla-2025`, not `tekla-2025.0`; `dynamo-4-1-0` not `dynamo-4.1.0`). Dots in version → dashes for filesystem safety.

## What's owed for v0.30 ship-ready PR

1. **`aware coverage validate` CLI wiring.** Currently a JSON-parse stub. Should grow a `coverage-validate` sidecar verb that validates IR against root schema + catalog files against Type fragment schema (the codex-coverage reviewer does this externally via `ajv-cli` / Python `jsonschema`).

2. **Remove or replace pre-existing Tekla-curated-manifest tests.** `cli/tests/` has 3 cargo tests asserting hardcoded counts (31 skills / 20 commands) against the OLD v0.28 curated `tekla` manifest. The v0.30 raw agents have different counts. Either: (a) update the asserted counts, (b) delete the tests (they're checking obsolete behavior), or (c) make the tests target a stable fixture.

3. **Promote the `feature/v0.29-runtime-hello-world` branch to a proper `feature/v0.30-host-coverage` branch** for the PR.

4. **PR for v0.30 to main.** Per the project's PR-by-PR shipping cadence (memory: `feedback_ship_cadence.md`), v0.30 ships as one PR after user "go" signal.

## What's owed beyond v0.30 (not blocking ship)

- **Wire codex-rescue as a true protocol runner.** Currently the codex-rescue agent type is sandbox-blocked; we ran codex-coverage protocol via general-purpose subagents. Either fix codex-rescue's sandbox or formalize the general-purpose pattern in the protocol spec.
- **Add Revit + AutoCAD + Civil 3D extractors.** Same NuGet PE-metadata pattern Navisworks established. ~80 lines each per new vendor (new Extractor.cs + KnownVersions map; shared `MetadataReflector` + `XmlDocLoader` + `NuGetFetcher`).
- **mkdocs-material kind-discrimination in verify-types-strict.ps1.** Allplan flagged this; the strict-verify script doesn't have an `$isMkdocsSource` branch yet (the codex-coverage protocol picks up the slack via vendor-aware spot-checks).
- **`aware coverage validate` schema wiring** (above) — required for v0.30 PR.

## Branch tip at handoff

`713676a66 review(v0.30 C11): dynamo-4-1-1 codex coverage review v2 — PASS`

## Commits in v0.30 Phase B (rough count)

~80+ commits from `e3597bf9` (Phase A start) through `713676a66` (last review). The bulk (~60) landed during Phase B; the rest are Phase A foundation + protocol updates.

## Reproduction

To re-run any vendor's extraction:

```powershell
$env:AWARE_SIDECAR = "C:\Users\bimst\source\repos\aware\cli-sidecar\bin\Release\net9.0\win-x64\publish\aware-sidecar.exe"
# Direct sidecar invocation:
'{"op":"coverage-extract","args":{"vendor":"<vendor>","version":"<version>","out_path":"<absolute-path>"}}' | & $env:AWARE_SIDECAR

# Or via the Rust CLI:
& cli/target/release/aware.exe coverage generate <vendor> <version> --from-ir <path> --vendor <real-vendor> --vertical <vertical>
```

Per-vendor wrappers exist for vendors with CDN throttling (`extract-with-restart.ps1`); see `cli-sidecar/Ingest/Output/extract-*.ps1`.

To re-run the protocol on any agent:
```powershell
& cli-sidecar/Ingest/Output/verify-types-strict.ps1 -IrPath <ir-path>
```
For the full 4-step protocol, dispatch a codex-coverage reviewer subagent following `docs/superpowers/specs/host-coverage-review-protocol.md`.
