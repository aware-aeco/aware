# archicad-29 EXTRACTION NOTES

This agent is generated from a **hybrid coverage IR** that combines two ArchiCAD
JSON-API sources. The v29 IR is the canonical surface — the Graphisoft official
JSON Interface documentation site publishes the v29 reference at the time of
extraction.

## Sources

| Surface | Source | URL |
|---|---|---|
| Tapir Add-On commands (community) | ENZYME-APD GitHub raw, branch `main` | `https://raw.githubusercontent.com/ENZYME-APD/tapir-archicad-automation/main/docs/archicad-addon/command_definitions.js` |
| Tapir shared schema types | ENZYME-APD GitHub raw, branch `main` | `https://raw.githubusercontent.com/ENZYME-APD/tapir-archicad-automation/main/docs/archicad-addon/common_schema_definitions.js` |
| Tapir docs page (`doc_url` target) | enzyme-apd.github.io | `https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/` |
| Graphisoft official JSON Interface | archicadapi.graphisoft.com | `https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/` (version 29.0.0.3000 at time of extraction) |
| Graphisoft per-command schemas | (per-command JSON files under) | `https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/content/API.<CmdName>.json` |

- **Source kind:** `hybrid` (matches the schema enum value)
- **Extraction date:** _filled in by the extractor; see metadata.extracted_at in the IR JSON_

## Version handling

ArchiCAD's JSON API does not have versioned doc archives like Tekla or Rhino:

- **Tapir** publishes ONE catalog that targets ArchiCAD 25-29 (the per-version
  Tapir Add-On binaries are built against each AC SDK, but the JSON command
  surface they expose is identical across all five versions Tapir supports).
  Each command carries a Tapir Add-On `version` stamp (e.g. `0.1.0`, `1.3.1`),
  which is preserved in the IR's `MethodInfo.since` field; this is the **Tapir
  Add-On** version that introduced the command, NOT the ArchiCAD version.

- **Graphisoft** publishes ONLY the latest JSON Interface reference
  (version 29.0.0.3000 at the time of extraction — confirmed 2026-05-19). The
  v29 IR's Graphisoft commands are taken verbatim from the live site.

For agents running against ArchiCAD 29, both the Tapir and Graphisoft command
surfaces in this IR are accurate as of the extraction date.

## IR shape

See `../archicad-28/EXTRACTION-NOTES.md` (identical shape — same parsers, same
mapping). The v28 and v29 IRs differ only by:
- The `host_version` field (`"28.0"` vs `"29.0"`)
- The `extracted_at` timestamp
- The Graphisoft official-command compatibility caveat (v28 carries the v29
  superset; v29 carries v29 verbatim)

## Reproduction

```powershell
echo '{"op":"coverage-extract","args":{"vendor":"archicad","version":"29.0","out_path":"cli-sidecar/Ingest/Output/archicad-29.0.ir.json"}}' `
  | cli-sidecar/bin/Release/net10.0/win-x64/publish/aware-sidecar.exe

# Then regenerate the agent:
$env:AWARE_SIDECAR = "C:\Users\bimst\source\repos\aware\cli-sidecar\bin\Release\net10.0\win-x64\publish\aware-sidecar.exe"
& "C:\Users\bimst\source\repos\aware\cli\target\release\aware.exe" coverage generate archicad 29.0 `
  --from-ir cli-sidecar/Ingest/Output/archicad-29.0.ir.json `
  --vendor graphisoft --vertical architecture
```

Implementation lives at `cli-sidecar/Ingest/Extractors/ArchiCAD/`.

## Crawl strategy

Identical to `../archicad-28/EXTRACTION-NOTES.md`. Five phases, in-process,
under 60 seconds wall-clock.

## IR / catalog counts (v29)

- **Tapir command categories:** 14
- **Tapir shared schema types:** 203
- **Graphisoft command groups:** 12
- **Total types in IR:** 229 (14 + 203 + 12)
- **Total methods (= commands across both surfaces):** ~200
  (129 Tapir + ~73 Graphisoft, exact count depends on Graphisoft fetch success)

## Concurrency tuning

- Tapir fetches (2 files): sequential.
- Graphisoft per-command fetches: `MaxDegreeOfParallelism = 4`. The site is
  served by nginx with no rate-limit response observed; we keep concurrency
  low out of politeness to the volunteer-run infrastructure.

## Extraction errors log

Path: `cli-sidecar/Ingest/Output/archicad-29.0-errors.log` (lazily created on
first error). Per-command fetch failures are retried once after a 750ms delay
before being recorded as final.

## Known limitations / follow-up work

- **Cross-file `$ref` not resolved.** Graphisoft's command JSONs `$ref` types
  that live in sibling files (e.g. `APITypes.json#/definitions/ExecutionResults`).
  The parser preserves the fragment tail as the type label but does not import
  the referenced types into the IR. To do so would require fetching ~3 more
  files (APITypes.json, APIAttributeTypes.json, APIAddOnTypes.json) and merging
  their `definitions` into the `Archicad.Schema` namespace. Follow-up.
- **Compatibility metadata not preserved.** Each Graphisoft command's docs
  carry "First version" / "Last version" fields rendered by the v29 docs viewer.
  The IR could preserve these as `MethodInfo.since` / `MethodInfo.deprecated`,
  but the current implementation does not parse them. Follow-up.
