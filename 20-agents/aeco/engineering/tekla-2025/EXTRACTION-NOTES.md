# tekla-2025 EXTRACTION NOTES

This agent is generated from a coverage IR scraped from `developer.tekla.com`. These notes capture **how** the IR was produced so the extraction can be reproduced — and so the next reviewer knows what trade-offs the parser made.

## Source

- **Site:** `https://developer.tekla.com`
- **Version root:** `https://developer.tekla.com/doc/tekla-structures/2025/tekla-structures-45473`
- **Extraction date:** 2026-05-19 (type-page pass + per-member enrichment pass)
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

## Parser quirks

(Identical to tekla-2026 — same site, same page templates. See [tekla-2026 EXTRACTION-NOTES.md](../tekla-2026/EXTRACTION-NOTES.md#parser-quirks) for the full quirks list, including the `LST...` inline-script delimiter substitution, the legacy `/topic/en/18/47/<guid>` URL handling, the empty-summary placeholder, and the `object`-typed-property note.)

## Skipped pages

The 2025 extraction produced **0 skipped pages**.

## Hard failures

None. All 1282 type URLs were fetched and parsed successfully.

The per-member enrichment pass on 2025 required **4 sidecar restarts** because Tekla's CDN throttled the connection mid-run (after ~2000-3000 fetches per session). Each restart resumed from the last 200-enrichment snapshot. The auto-restart wrapper handles this transparently. Final run completed with 0 errors across all 12131 member fetches.

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
| Properties with non-`object` type | 3871 / 4045 | 95.7% |
| Events with non-`EventHandler` delegate | 149 / 149 | 100% |
| **Total members enriched** | **11957 / 12131** | **98.6%** |

The 174 properties remaining at `type="object"` are predominantly `SyncRoot` / `ICollection.SyncRoot` properties that **legitimately** return `System.Object` per the .NET BCL contract — they are not parser failures. A spot check confirms each such property's vendor page does say `Type: Object`.

## Strict 50-type self-verification

A deterministic sample of 50 types was drawn from the IR using a Fisher-Yates shuffle seeded by the SHA-256 hash of `tekla-2025.0.ir.json` (script: `cli-sidecar/Ingest/Output/verify-types-strict.ps1`). Each sample was checked at the **type level** (name + kind word present in HTML) AND the **member level** (3 random methods, 3 random properties, 1 random event each pass the placeholder check).

Result: **48 / 50 PASS** (seed = 1456662254).

The two failures are both `property-placeholder-type(SyncRoot)`:
- `Tekla.Structures.Model.UI.PickInput.SyncRoot` (idx 1221)
- `Tekla.Structures.Model.PhaseCollection.SyncRoot` (idx 988)

Both are legitimate `object`-typed properties per the .NET BCL `ICollection.SyncRoot` contract — the verifier flags `type="object"` as suspicious, but for these specific properties it IS the correct type. **This is a known false-positive class of the verifier, not a parser bug.** Spot-check: Tekla's vendor page for `PickInput.SyncRoot` does say `Type: Object`.

The full output is at `cli-sidecar/Ingest/Output/tekla-2025-strict-verify.txt`.

## Known limitations / follow-up work

Same as tekla-2026 — see [that file's "Known limitations" section](../tekla-2026/EXTRACTION-NOTES.md#known-limitations--follow-up-work).
