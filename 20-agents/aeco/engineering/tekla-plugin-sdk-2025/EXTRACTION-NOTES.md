# tekla-plugin-sdk-2025 EXTRACTION NOTES

This agent is the **reflected reference** for the Tekla plugin-authoring SDK NuGets that are **absent from developer.tekla.com's scraped Open API reference tree** (so the `tekla-2025` scrape agent does not cover them):

- **`Tekla.Structures.Plugins.DirectManipulation` `2025.0.0`** — the in-model handle / picking / graphics API. Ships an XML doc → rich prose.
- **`TeklaFusion` `4.1.10`** — Trimble's WPF UI framework. Ships **no** XML doc → signatures-only.

## Source

- **Source kind:** `nuget` (reflection, not scrape).
- **Reflection:** `System.Reflection.Metadata` via `cli-sidecar` `MetadataReflector` — reads PE metadata directly, no assembly load, so the net48 DLLs' unresolved dependencies don't matter. Public top-level types only.
- **Extraction date:** 2026-05-24.

## Reproduction

```bash
dotnet build cli-sidecar -c Release --source "https://api.nuget.org/v3/index.json" --ignore-failed-sources -p:NuGetAudit=false
SIDE=cli-sidecar/bin/Release/net9.0/aware-sidecar.exe

echo '{"op":"coverage-extract","args":{"vendor":"tekla-plugin-sdk","version":"2025.0","out_path":"cli-sidecar/Ingest/Output/tekla-plugin-sdk-2025.0.ir.json"}}' | "$SIDE"
echo '{"op":"coverage-validate","args":{"ir_path":"cli-sidecar/Ingest/Output/tekla-plugin-sdk-2025.0.ir.json"}}' | "$SIDE"
echo '{"op":"coverage-generate","args":{"ir_path":"cli-sidecar/Ingest/Output/tekla-plugin-sdk-2025.0.ir.json","out_dir":"20-agents/aeco/engineering/tekla-plugin-sdk-2025","agent_id":"tekla-plugin-sdk-2025","vendor":"trimble","vertical":"engineering"}}' | "$SIDE"
```

Extractor: `cli-sidecar/Ingest/Extractors/TeklaSdk/`. Version map (`KnownVersions`): `2025.0` → DM `2025.0.0`, Fusion `4.1.10`.

## Scoping (deliberate)

- **DirectManipulation:** public plugin surface only — `Tekla.Structures.Plugins.DirectManipulation.*` minus `*.Internal*`. Engine/adapter assemblies (`Tekla.BIM.DirectManipulation`, `Tekla.Structures.DirectManipulation`) are not fetched.
- **Fusion:** dialog-tier subset (`Fusion` root, `Fusion.App`, `Fusion.UI*`, `Fusion.CommandHandler`, `Fusion.Formatting`, `Fusion.Data*`, `Fusion.Features.Panels*`, `Fusion.Models`, `Fusion.Extensions`). GPU / Testing / RemoteObject / Win32 / Native excluded. Tune via `FusionDialogPrefixes`.
  - **net48 pinned:** reflects the `lib/net48` Fusion assembly explicitly (deterministic, plugin-accurate).
  - **Root-infra exclusion:** root `Fusion` types named `*Authentication*` / `*AccessControl*` / `*RemoteObject*` / `*WebView*` / `*Credential*` are excluded (`FusionRootInfraMarkers`).

## Coverage

333 types, 2669 methods, 41 events, 555 properties.

## Prose fidelity caveat

- **DM** entries carry real Trimble summaries (from the shipped XML doc).
- **Fusion** entries are **signatures-only** — no XML doc, decompile-summarise disallowed by `license-aware-extraction` (proprietary). Summaries fall back to a generated form. The SkillWriter's templated frontmatter ("full vendor-documented …") is inaccurate for the Fusion namespaces.

## Companion (narrative) docs

Conceptual / lifecycle docs live in the core `tekla-2025` agent and cross-link here:
`tekla-2025/skills/tekla-structures-direct-manipulation.md` and `…-fusion-ui.md`.
