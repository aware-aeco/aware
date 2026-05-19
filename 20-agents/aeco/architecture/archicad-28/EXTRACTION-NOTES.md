# archicad-28 EXTRACTION NOTES

This agent is generated from a **hybrid coverage IR** that combines two ArchiCAD
JSON-API sources because no single site publishes a complete ArchiCAD 28 reference.

## Sources

| Surface | Source | URL |
|---|---|---|
| Tapir Add-On commands (community) | ENZYME-APD GitHub raw, branch `main` | `https://raw.githubusercontent.com/ENZYME-APD/tapir-archicad-automation/main/docs/archicad-addon/command_definitions.js` |
| Tapir shared schema types | ENZYME-APD GitHub raw, branch `main` | `https://raw.githubusercontent.com/ENZYME-APD/tapir-archicad-automation/main/docs/archicad-addon/common_schema_definitions.js` |
| Tapir docs page (`doc_url` target) | enzyme-apd.github.io | `https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/` |
| Graphisoft official JSON Interface | archicadapi.graphisoft.com | `https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/` |
| Graphisoft per-command schemas | (per-command JSON files under) | `https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/content/API.<CmdName>.json` |

- **Source kind:** `hybrid` (matches the schema enum value)
- **Extraction date:** _filled in by the extractor; see metadata.extracted_at in the IR JSON_

## Version handling — critical caveat

ArchiCAD's JSON API does not have versioned doc archives like Tekla or Rhino:

- **Tapir** publishes ONE catalog that targets ArchiCAD 25-29 (the per-version
  Tapir Add-On binaries are built against each AC SDK, but the JSON command
  surface they expose is identical across all five versions Tapir supports).
  Each command carries a Tapir Add-On `version` stamp (e.g. `0.1.0`, `1.3.1`),
  which is preserved in the IR's `MethodInfo.since` field; this is the **Tapir
  Add-On** version that introduced the command, NOT the ArchiCAD version.

- **Graphisoft** publishes ONLY the latest JSON Interface reference
  (currently version 29.0.0.3000 — confirmed 2026-05-19). There is no archived
  v28 site at `archicadapi.graphisoft.com`. The v28 surface is therefore best
  reconstructed from the v29 docs minus any commands the live page marks as
  "First version: Archicad 29" in their compatibility section — but that
  filtering is not yet automated. As a result, **this v28 IR includes the
  full v29 official-command surface**. Agents using ArchiCAD 28 should consult
  per-command compatibility info on the live Graphisoft site for any 28-vs-29
  command-availability differences.

Implication for downstream consumers: an agent running against ArchiCAD 28
may receive a "command not found" error when it tries to call an
official Graphisoft command that is v29-only. This is detectable at runtime
via the standard JSON-RPC error response and does not affect Tapir commands
(which are uniformly compatible across the AC25-29 range Tapir supports).

## IR shape

ArchiCAD's surface is JSON-RPC, not a typed class library. The IR maps this
to the host-coverage schema as follows:

- **Tapir command CATEGORY** (e.g. "Application Commands", "Element Commands")
  → `TypeInfo` with `kind="static-class"`, `namespace="Tapir.AdditionalCommands"`
  - Each command in the category becomes a `MethodInfo` in `methods[]`
  - The method's `params[]` come from the command's `inputScheme.properties`
  - The method's `returns` summarises the command's `outputScheme`
- **Tapir SHARED schema types** (e.g. `ElementId`, `WindowType`, `Hotlinks`)
  → `TypeInfo` with `kind="class"` (objects) or `kind="enum"` (string enums),
  `namespace="Tapir.Schema"`
  - Object types' `properties[]` are populated from the schema's `properties`
  - Enum types' `enum_values[]` are populated from the schema's `enum` array
- **Graphisoft official command GROUP** (e.g. "AddOn Commands", "Attribute Commands")
  → `TypeInfo` with `kind="static-class"`, `namespace="Archicad.OfficialCommands"`
  - Each command in the group becomes a `MethodInfo` in `methods[]`
  - Same schema-to-params/returns mapping as Tapir

This shape lets AI agents read a single skill markdown per namespace and find
the verbs they need; the catalog JSON files give them structured access to
each type's surface.

## Reproduction

```powershell
echo '{"op":"coverage-extract","args":{"vendor":"archicad","version":"28.0","out_path":"cli-sidecar/Ingest/Output/archicad-28.0.ir.json"}}' `
  | cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe

# Then regenerate the agent:
$env:AWARE_SIDECAR = "C:\Users\bimst\source\repos\aware\cli-sidecar\bin\Release\net9.0\win-x64\publish\aware-sidecar.exe"
& "C:\Users\bimst\source\repos\aware\cli\target\release\aware.exe" coverage generate archicad 28.0 `
  --from-ir cli-sidecar/Ingest/Output/archicad-28.0.ir.json `
  --vendor graphisoft --vertical architecture
```

The extractor is at `cli-sidecar/Ingest/Extractors/ArchiCAD/` (Extractor.cs +
PageParser.cs + TapirCommandParser.cs + TapirSchemaParser.cs + GraphisoftMenuParser.cs).
The sidecar verb is `coverage-extract` (Program.cs case `"archicad"`).

## Crawl strategy

Five phases, all in-process — no snapshotting needed because the payload is
under 500KB across ~80 HTTP fetches. A single extraction completes in well
under 60 seconds.

1. Fetch `command_definitions.js` from raw.github (one HTTP GET).
2. Fetch `common_schema_definitions.js` from raw.github (one HTTP GET).
3. Parse the JS-wrapped JSON files into typed records (`TapirCommandParser` /
   `TapirSchemaParser`). The wrappers (`var gCommands = [...]`, trailing `;`)
   are stripped before handing the body to JsonDocument.
4. Fetch `content/menutree.json` from the Graphisoft docs site (one HTTP GET).
   Walk the menu tree depth-first; every leaf with `commanddocumentation`
   becomes a downloadable command JSON.
5. Fan-out fetch every Graphisoft command JSON (4-way concurrent), parse each
   into a `CommandDefinition`.
6. Assemble IR (in-process), emit JSON via source-gen IrJsonContext.

No JavaScript execution is required — the JS files are JSON in a wrapper, and
the Graphisoft site serves a static menu tree + static per-command JSON files.

## IR / catalog counts (v28)

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

Path: `cli-sidecar/Ingest/Output/archicad-28.0-errors.log` (lazily created on
first error). Per-command fetch failures are retried once after a 750ms delay
before being recorded as final.

## Known limitations / follow-up work

- **v28 vs v29 official-command divergence not filtered.** Graphisoft's docs
  carry per-command "First version" / "Last version" metadata; the parser
  does not yet read those fields and emit a v28-specific subset. Follow-up
  would parse the compatibility section of each command JSON and drop commands
  with `first_version > 28.0`.
- **Tapir command versions are Add-On versions, not ArchiCAD versions.** The
  `since` field on each MethodInfo carries the Tapir Add-On version that
  introduced the command. Downstream consumers should treat this as a Tapir
  capability filter, not an ArchiCAD compatibility marker.
- **Graphisoft's command JSON `$ref` paths point to sibling files**
  (e.g. `APITypes.json#/definitions/ExecutionResults`). The parser keeps the
  fragment tail as the type label; the referenced sibling-file types are NOT
  imported as first-class IR types. To resolve them would require fetching
  the additional APITypes.json / APIAttributeTypes.json / APIAddOnTypes.json
  files and merging their definitions in. Follow-up.
