# Vendor Implementer Brief — v0.30 Phase B (Tier A)

Shared brief for v0.30 Phase B vendor implementers. Each implementer is responsible for **one vendor (1-2 versions)** following the proven Tekla + Rhino+Grasshopper pattern.

## Required reading

These set the pattern and the contract. **You MUST read them before coding.**

### Phase A foundation (use, do not modify)

- `cli-sidecar/Ingest/Schema/host-coverage.schema.json` — IR schema (root)
- `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json` — catalog Type fragment schema
- `cli-sidecar/Ingest/Generator/Models.cs` — `CoverageIR`, `TypeInfo`, `MethodInfo`, etc.
- `cli-sidecar/Ingest/Generator/IrJsonContext.cs` — source-gen serializer context
- `cli-sidecar/Ingest/Extractors/Common/HttpScraper.cs` — AOT-clean HTTP wrapper (use this, NOT Playwright)
- `cli-sidecar/Ingest/Extractors/Common/IRBuilder.cs` — IR accumulator
- `cli-sidecar/Ingest/Extractors/Common/ChmParser.cs` — CHM helper (use only if your vendor ships CHMs)
- `cli-sidecar/Ingest/Output/extract-with-restart.ps1` — auto-restart wrapper template (copy + adapt)
- `cli-sidecar/Ingest/Output/verify-types-strict.ps1` — 50-type strict sampler (use as-is)
- `cli-sidecar/Ingest/Output/enrichment-stats.ps1` — fill-rate reporter

### Proven vendor implementations (use as templates)

- **Tekla** (`cli-sidecar/Ingest/Extractors/Tekla/`) — Trimble Sandcastle-style with LST script-substitution markup
- **Rhino + Grasshopper** (`cli-sidecar/Ingest/Extractors/Rhino/`, `Grasshopper/`) — mcneel Sandcastle, github raw branch source, external-URL filtering

### Spec contract

- `docs/superpowers/specs/host-coverage-review-protocol.md` — 4-step review protocol you must pass

### Recent PASSED COVERAGE-REVIEW.md (the bar you must clear)

- `20-agents/aeco/engineering/tekla-2025/COVERAGE-REVIEW.md`
- `20-agents/aeco/architecture/rhino-8/COVERAGE-REVIEW.md`

## Defensive checklist — 13 learnings from Tekla + Rhino+GH

These were caught during prior B-tasks. Apply defensively to YOUR vendor:

1. **Member data must be REAL, not placeholders.** Two-pass extraction (type-page Pass 1 → member-page Pass 2) is mandatory. Real method `signature`, `params[].type`, `returns.type`, `throws[]`, `remarks`. Real property `type`. Real event `delegate`, `handler_thread`.
2. **Constructor names carry overload disambiguator** — `Foo()` vs `Foo(Args)`, never collapsed to `Foo`.
3. **Generic angle brackets round-trip** — `IEnumerable<T>` stays as-is, never `IEnumerable.T.`.
4. **`returns` field MUST be `null` for void methods + ctors** — not omitted. Use `[JsonIgnore(Condition = JsonIgnoreCondition.Never)]` on the property, or per-property override.
5. **Property type fallback** — try multiple selectors. Some vendors use `<h4>Return Value</h4>` instead of `<h4>Property Value</h4>` for property pages (esp. overrides).
6. **Schema-required fields** — validate IR + every catalog file against the two schemas before commit. `schema_violations` must be 0.
7. **AOT clean** — `dotnet publish -c Release -r win-x64 --self-contained` must produce 0 warnings, 0 errors. No reflection-based JSON. Use `IrJsonContext` source-gen.
8. **Resumability** — snapshot every 200 enrichments. Auto-restart wrapper for CDN throttle. Idempotency check skips already-populated members.
9. **No silent failures** — errors go to `cli-sidecar/Ingest/Output/<vendor>-<version>-errors.log`. Final extraction must have 0 hard errors after retries.
10. **Filename sanitization** — generic type names use `_of_` for `<`, strip `>`. Catalog JSON's `name` field carries the canonical name (with `<>`).
11. **LST or analogous script-substitution markup** — Tekla uses inline `<script>AddLanguageSpecificTextSet(...)</script>` for language-specific delimiters. Your vendor may have something analogous. Read `MemberPageParser.cs` LST handling for the pattern.
12. **External / inherited member filter** — for .NET-doc'd vendors, type pages list inherited `System.Object` members with hrefs to `docs.microsoft.com`. Skip them at type-page parse time.
13. **Sandcastle gotchas** — multiple member tables (operatorList, fieldList, conversionList) fold into single catalog `methods[]` / `properties[]`. Enum rows have no anchor (read name from td[2] text). `[Missing <summary>…]` placeholders must be filtered.

## Standard architecture

Your vendor extractor lives in `cli-sidecar/Ingest/Extractors/<Vendor>/`:

- `Extractor.cs` — orchestrator. Two-pass extraction (type pages → member pages). Snapshot/resume.
- `PageParser.cs` — type-page DOM. Returns `TypeInfo` per page.
- `MemberPageParser.cs` — member-page DOM. Fills `MethodInfo`, `PropertyInfo`, `EventInfo` real signatures/types/delegates.

Wire your vendor into `cli-sidecar/Program.cs` line ~111 — add a `"<vendor>" => YourVendor.Extractor.Extract(version, outPath).GetAwaiter().GetResult()` case.

## Standard verification

```powershell
$env:PATH = "C:\Program Files (x86)\Microsoft Visual Studio\Installer\;" + $env:PATH
cd C:\Users\bimst\source\repos\aware\cli-sidecar
& "C:\Program Files\dotnet\dotnet.exe" build -nologo --verbosity minimal             # 0 warnings 0 errors
& "C:\Program Files\dotnet\dotnet.exe" publish -c Release -r win-x64 --self-contained -nologo --verbosity minimal   # 0 warnings 0 errors
& "C:\Program Files\dotnet\dotnet.exe" test Ingest/Generator/Tests --nologo --verbosity minimal                    # all pass
& "C:\Program Files\dotnet\dotnet.exe" test Tests --nologo --verbosity minimal                                     # all pass (incl. your new fixtures)
cd C:\Users\bimst\source\repos\aware\cli
& "C:\Users\bimst\.cargo\bin\cargo.exe" test --test coverage_e2e --release                                          # 1 passed
```

Then strict verify per version:
```powershell
& "C:\Users\bimst\source\repos\aware\cli-sidecar\Ingest\Output\verify-types-strict.ps1" -IrPath "<your-ir.json>"
# Target: 50/50 PASS. Tolerate up to 2/50 false-positives on legitimate object-typed properties (.SyncRoot etc.).
```

## Engineering rules

- **NO shortcuts, NO workarounds.** If you hit a vendor-specific quirk, FIX the parser. Don't add a fallback that silently masks the issue.
- **Don't touch Phase A foundation files** beyond narrowly-required IrJsonContext registrations.
- **Don't yield mid-extraction.** Use background processes via Bash `run_in_background: true` + Monitor. Stay alive until DONE or BLOCKED.
- **Match commit message style.** Per version:
  ```
  feat(v0.30 B<N>): <vendor>-<version> full-coverage extraction from <source-domain>
  ```
- **No `Co-Authored-By:` trailer.**
- **Stage explicit paths only** — no `git add -A`.

## Out of scope

- Don't run codex-coverage protocol yourself — the orchestrator dispatches a separate reviewer
- Don't modify other vendors' extractors
- Don't modify the Phase A schema/models
- Don't write skills or commands for the regenerated agents (Generator.Generate handles that)
- Don't push to remote / open PRs

## Report format

When done:
- **Status:** DONE | DONE_WITH_CONCERNS | BLOCKED
- Full file list (created / modified)
- Per-version stats: types in IR, members enriched (M/P/E percentages), strict-verify 50/50 result
- AOT publish + tests: green
- Commit SHAs (one per version)
- Vendor-specific quirks worth documenting for future maintainers / protocol updates
- Wall-clock spent

Use DONE_WITH_CONCERNS only if you completed work but have doubts. BLOCKED only if you cannot complete after exhausting options.
