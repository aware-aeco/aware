# tekla-2025 EXTRACTION NOTES

This agent is generated from a coverage IR scraped from `developer.tekla.com`. These notes capture **how** the IR was produced so the extraction can be reproduced — and so the next reviewer knows what trade-offs the parser made.

## Source

- **Site:** `https://developer.tekla.com`
- **Version root:** `https://developer.tekla.com/doc/tekla-structures/2025/tekla-structures-45473`
- **Extraction date:** 2026-05-19 (re-extracted from scratch after the codex-coverage FAIL feedback identified ctor-name + generic-bracket + returns-key + Property-Value/Return-Value parser defects)
- **Source kind:** `scrape` (HttpClient + AngleSharp)

## Reproduction

```bash
# From repo root — run the sidecar via stdin envelope.
echo '{"op":"coverage-extract","args":{"vendor":"tekla","version":"2025.0","out_path":"cli-sidecar/Ingest/Output/tekla-2025.0.ir.json"}}' \
  | cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe

# Or use the restart-on-stall wrapper (recommended — Tekla's CDN throttles after ~3k member fetches per session):
pwsh -File cli-sidecar/Ingest/Output/extract-with-restart.ps1 -Version 2025.0

# Regenerate the agent from the IR:
AWARE_SIDECAR=cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe \
  cli/target/release/aware.exe coverage generate tekla 2025.0 \
  --from-ir cli-sidecar/Ingest/Output/tekla-2025.0.ir.json \
  --vendor trimble --vertical engineering
```

## Crawl strategy

Identical to the tekla-2026 extraction — only the root URL changes. See [tekla-2026 EXTRACTION-NOTES.md](../tekla-2026/EXTRACTION-NOTES.md) for the full two-pass strategy (type-page crawl → per-member enrichment) and CSS selector tables.

The 2025 sidebar enumerates 28 namespaces (one fewer than 2026 — `Tekla.Structures.Model.Internal.BimModelDataProduct.Types` is new in 2026 and absent from 2025's tree). The 2025 layout otherwise matches 2026 exactly, so the same `PageParser` + `MemberPageParser` work without per-version branches.

## Extraction errors log

If an extraction run produces errors (HTTP failure, parse failure, retry-exhausted), they are appended one per line to a dedicated log file alongside the IR output:

```
cli-sidecar/Ingest/Output/tekla-2025.0-errors.log
```

Format and phase values are documented in the canonical [tekla-2026 EXTRACTION-NOTES.md "Extraction errors log" section](../tekla-2026/EXTRACTION-NOTES.md#extraction-errors-log). The file is created lazily on first error, never pre-created empty.

The 2025 run completed with **0 errors** after the 4 sidecar restarts described under "Hard failures" below. Each restart's resumed enrichment loop saw a few transient 5xx/timeout errors mid-throttle, but they were all recovered on the second retry inside `FetchWithFreshScraper` — none of those transient errors reached the errors log, by design (only retry-exhausted failures do). Therefore no log file exists on disk today.

## Concurrency tuning

The 2025 extraction uses the **same** `TypePageConcurrency=4`, `MemberPageConcurrency=8`, `SnapshotEvery=200`, and `StallTimeoutSec=240` as 2026 — same code, same CDN. See the [tekla-2026 EXTRACTION-NOTES.md "Concurrency tuning" section](../tekla-2026/EXTRACTION-NOTES.md#concurrency-tuning) for the full table + reasoning.

The only observable difference: the 2025 run needed **4 sidecar restarts** to complete (vs. 0 for 2026). This is consistent with running 2025 from a cold CDN cache (no recently-fetched member pages on the throttle-state side), whereas the 2026 run benefited from the warm cache the 2025 run had just populated. The auto-restart wrapper made this transparent — the IR ended up at the same 100% enrichment level regardless of restart count.

## Parser quirks

(Identical to tekla-2026 — same site, same page templates. See [tekla-2026 EXTRACTION-NOTES.md](../tekla-2026/EXTRACTION-NOTES.md#parser-quirks) for the full quirks list, including the `LST...` inline-script delimiter substitution, the legacy `/topic/en/18/47/<guid>` URL handling, the empty-summary placeholder, and the `object`-typed-property note.)

## Skipped pages

The 2025 extraction produced **0 skipped pages**.

## Hard failures

None. All 1282 type URLs were fetched and parsed successfully.

The per-member enrichment pass on the re-extraction completed in a single uninterrupted sidecar process (no restarts), running in parallel with the 2026 extraction against the same CDN. Total: 12131 member fetches with 0 errors and 0 retries.

## IR / catalog counts

- **Vendor-published namespaces (from sidebar):** 28
- **Unique type URLs discovered:** 1282
- **Types in IR:** 1282
- **Types in generated catalog:** 1282
- **Skipped (non-type) pages:** 0
- **Hard failures (types):** 0

## Member-page enrichment results

| field | filled / total | % |
|-------|---------------:|--:|
| Methods + constructors with real signature | 7937 / 7937 | 100% |
| Properties with non-`object` type | 4033 / 4045 | 99.7% |
| Events with non-`EventHandler` delegate | 149 / 149 | 100% |
| **Total members enriched** | **12119 / 12131** | **99.9%** |

The 12 properties remaining at `type="object"` are `SyncRoot` / `ICollection.SyncRoot` properties that **legitimately** return `System.Object` per the .NET BCL contract — they are not parser failures. A spot check confirms each such property's vendor page does say `Type: Object`. The remaining 162 properties that were `object` in the previous extraction now carry their real types thanks to the Property Value/Return Value subheading-fallback parser fix shipped alongside this re-extraction.

## Strict 50-type self-verification

A deterministic sample of 50 types was drawn from the IR using a Fisher-Yates shuffle seeded by the SHA-256 hash of `tekla-2025.0.ir.json` (script: `cli-sidecar/Ingest/Output/verify-types-strict.ps1`). Each sample was checked at the **type level** (name + kind word present in HTML, with generic-parameter stripping for types like `Enum<E>`) AND the **member level** (3 random methods, 3 random properties, 1 random event each pass the placeholder check).

Result: **50 / 50 PASS** (seed = 1293876521, IR sha256 = `4d1efd29e27f12626611d980929c1ad56006f26b45b82cd4d121fd26e865270c`).

The full output is at `cli-sidecar/Ingest/Output/tekla-2025-strict-verify.txt`.

## Known limitations / follow-up work

Same as tekla-2026 — see [that file's "Known limitations" section](../tekla-2026/EXTRACTION-NOTES.md#known-limitations--follow-up-work).
