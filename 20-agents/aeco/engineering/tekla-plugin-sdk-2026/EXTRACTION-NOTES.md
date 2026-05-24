# tekla-plugin-sdk-2026 EXTRACTION NOTES

This agent is the **reflected reference** for the Tekla plugin-authoring SDK NuGets that are **absent from developer.tekla.com's scraped Open API reference tree** (so the `tekla-2026` scrape agent does not cover them):

- **`Tekla.Structures.Plugins.DirectManipulation` `2026.0.3`** — the in-model handle / picking / graphics API. Ships an XML doc → rich prose.
- **`TeklaFusion` `4.1.36`** — Trimble's WPF UI framework. Ships **no** XML doc → signatures-only.

## Source

- **Source kind:** `nuget` (reflection, not scrape).
- **Reflection:** `System.Reflection.Metadata` via `cli-sidecar` `MetadataReflector` — reads PE metadata directly, no assembly load, so the net48 DLLs' unresolved dependencies don't matter. Public top-level types only.
- **Extraction date:** 2026-05-24.

## Reproduction

```bash
# 1. Build the sidecar (restore from nuget.org only — the machine's user NuGet.Config
#    lists private ScaffPlan feeds that 401).
dotnet build cli-sidecar -c Release --source "https://api.nuget.org/v3/index.json" --ignore-failed-sources -p:NuGetAudit=false

SIDE=cli-sidecar/bin/Release/net10.0/aware-sidecar.exe

# 2. Extract → IR (fetches both NuGets from nuget.org, reflects, merges).
echo '{"op":"coverage-extract","args":{"vendor":"tekla-plugin-sdk","version":"2026.0","out_path":"cli-sidecar/Ingest/Output/tekla-plugin-sdk-2026.0.ir.json"}}' | "$SIDE"

# 3. Validate IR (Draft 2020-12).
echo '{"op":"coverage-validate","args":{"ir_path":"cli-sidecar/Ingest/Output/tekla-plugin-sdk-2026.0.ir.json"}}' | "$SIDE"

# 4. Generate the agent.
echo '{"op":"coverage-generate","args":{"ir_path":"cli-sidecar/Ingest/Output/tekla-plugin-sdk-2026.0.ir.json","out_dir":"20-agents/aeco/engineering/tekla-plugin-sdk-2026","agent_id":"tekla-plugin-sdk-2026","vendor":"trimble","vertical":"engineering"}}' | "$SIDE"
```

Extractor: `cli-sidecar/Ingest/Extractors/TeklaSdk/` (Extractor + PageParser + MemberPageParser), dispatched from the `coverage-extract` switch on vendor `tekla-plugin-sdk`.

## Scoping (deliberate)

- **DirectManipulation:** the public plugin surface only — `Tekla.Structures.Plugins.DirectManipulation.*` minus `*.Internal*`. The engine/adapter assemblies (`Tekla.BIM.DirectManipulation` ~672 types, `Tekla.Structures.DirectManipulation` ~280 types) are implementation detail and are **not** fetched.
- **Fusion:** a **dialog-tier** subset — namespaces `Fusion` (root), `Fusion.App`, `Fusion.UI*`, `Fusion.CommandHandler`, `Fusion.Formatting`, `Fusion.Data*`, `Fusion.Features.Panels*`, `Fusion.Models`, `Fusion.Extensions`. The GPU / Testing / RemoteObject / Win32 / Native machinery (~550 of the 904 public Fusion types) is excluded as irrelevant to plugin-dialog authoring. Tune via `FusionDialogPrefixes` in the extractor.
  - **net48 pinned:** TeklaFusion ships `lib/net48` + `lib/net6.0-windows`; the extractor reflects the **net48** assembly explicitly (the plugin/runtime TFM) so the surface is deterministic and plugin-accurate.
  - **Root-infra exclusion:** the root `Fusion` namespace also holds non-dialog infrastructure (auth / remoting / web). Types whose name contains `Authentication`, `AccessControl`, `RemoteObject`, `WebView`, or `Credential` are excluded (`FusionRootInfraMarkers`).

## Coverage

348 types, 2886 methods, 41 events, 636 properties.

## Prose fidelity caveat

- **DM** entries carry real Trimble summaries/remarks/params/returns (from the 569 XML-doc members shipped in the nupkg).
- **Fusion** entries are **signatures-only** — TeklaFusion ships no XML doc, and ILSpy decompile-and-summarise of the proprietary assembly is disallowed by the repo's `license-aware-extraction` rule (permissive licenses only). Summaries fall back to a generated `"<name> <kind> in <namespace>."` form. NOTE: the SkillWriter's templated frontmatter description says "full vendor-documented …" for every namespace — that boilerplate is inaccurate for the Fusion namespaces, which are signatures-only.

## Companion (narrative) docs

The conceptual / lifecycle docs live in the core `tekla-2026` agent and cross-link here:
`tekla-2026/skills/tekla-structures-direct-manipulation.md` and `…-fusion-ui.md`.
