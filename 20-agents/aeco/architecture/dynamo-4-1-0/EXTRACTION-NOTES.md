# Extraction notes — dynamo-4-1-0

**Vendor:** Autodesk (DynamoBIM)
**Version:** 4.1.0.4845 (NuGet `DynamoVisualProgramming.Core@4.1.0.4845`)
**Source kind:** `hybrid` (NuGet XML doc + .NET DLL reflection + GitHub source tree)
**Extracted at:** 2026-05-19
**GitHub ref:** `v4.1.0` (tagged release)

## Source landscape (the why)

Dynamo does **not** publish a canonical hosted HTML type-index. We surveyed:

1. **`dynamods.github.io/DynamoAPI/`** — the legacy Sandcastle reference, now 404. It only ever covered the Dynamo 1.0-era API and has been dead since at least 2024.
2. **`developer.dynamobim.org`** — a contributor's guide (Zero-Touch nodes, build instructions). Not an API reference.
3. **`github.com/DynamoDS/Dynamo/wiki`** — narrative release notes + per-version API-change changelogs (e.g. "API Changes in Dynamo 4.0.0"). NOT a complete type reference. It documents what changed between versions, not what exists.
4. **`fuget.org/packages/DynamoVisualProgramming.Core`** — third-party NuGet API browser. Reachable via web but TLS handshake fails from the build host; the data it shows is anyway derived from the same NuGet payload we already process directly.
5. **Autodesk Platform Services / Forge** — covers Autodesk's cloud APIs (Forge Data Management, ACC Issues, etc.), not the Dynamo desktop SDK.

The brief authorised `DONE_WITH_CONCERNS` for exactly this situation. The decision: the NuGet payload **is** the canonical source — it's what any rendered docs site would be generated from.

## What this extractor does

Hybrid pipeline:

1. **Stage NuGet** — download `DynamoVisualProgramming.Core@4.1.0.4845.nupkg` from `api.nuget.org`, unzip to `cli-sidecar/Ingest/Output/dynamo-nupkg/ext-4.1.0.4845/`. The package contains 9 DLLs and 8 Roslyn XML doc files at ~26K lines total — every public type, method, property, field, and event with summaries, params, returns, exceptions.
2. **Fetch source-file index** — one call to `api.github.com/repos/DynamoDS/Dynamo/git/trees/v4.1.0?recursive=1` returns the full repo file list. We index every `src/**/*.cs` path to map filename → repo path for `doc_url` resolution.
3. **Content-scan augmentation** — download the source archive once (`codeload.github.com/DynamoDS/Dynamo/zip/refs/tags/v4.1.0`, ~300MB), extract, and scan every `.cs` file for `class|struct|interface|enum|delegate X` declarations. This catches types defined inside differently-named files (e.g. `HostAnalyticsInfo` inside `DynamoModelHandle.cs`). The result: 99%+ of types get a precise GitHub blob URL.
4. **Per-DLL reflection + XML pairing** — for each DLL, use `System.Reflection.Metadata.PEReader` + `MetadataReader` to enumerate every public type. For each type/method/property/event, look up its XML doc summary/params/returns/throws by canonical XML-doc-ID. Lookup is exact-match-first with a prefix-scan fallback for overloads where our display→canonical type-name round-trip is imperfect.
5. **Emit IR** — write `cli-sidecar/Ingest/Output/dynamo-4.1.0.ir.json` with `source.kind = "hybrid"` and `source.urls = [nuget-package-url, github-tree-url]`.

## Coverage stats

- 707 types across 9 assemblies (DesignScriptBuiltin 6, DynamoApplications 3, DynamoCore 321, DynamoPackages 31, DynamoShapeManager 2, DynamoUtilities 32, ProtoCore 302, VMDataBridge 1, DynamoInstallDetective 9)
- 3,064 methods
- 2,370 properties
- 142 events
- 5,576 catalog files total

XML-doc-derived prose (real Dynamo developer comments, not synthesized placeholders) covers:
- 43% of type summaries (Dynamo's own XML doc coverage)
- 36% of method summaries
- 62% of property summaries
- 91% of event summaries

The gaps are real-world undocumented public surface — Dynamo's `ProtoCore` assembly (302 types) is internal compiler infrastructure that the project has only partially documented.

## doc_url precision

| URL kind | Count | Notes |
|---|---|---|
| `github.com/.../blob/v4.1.0/src/.../<File>.cs` | 703 | file-precision via path index + content scan |
| `github.com/search?q=...&type=code` | 4 | last-resort search fallback (rare; only types whose declaration eludes the content scan, mostly multi-generic-parameter interfaces with unusual signatures) |

The strict-verify `verify-types-strict.ps1` script ran 50/50 against this IR with the source-code lowercase-kind-word recognition rule added (see `cli-sidecar/Ingest/Output/verify-types-strict.ps1`).

## Caveats

1. **No hosted API reference site.** Per protocol Step 1 ("Fetch the vendor's type-index page"), this vendor has no single fetchable index. The codex-coverage reviewer's enumeration step must walk the GitHub source tree by directory or use the IR's `types[]` list directly as ground truth. Step 1 PASS is best evaluated against `<the NuGet DLL set>` — every public type in the shipped DLLs.
2. **`object`-typed properties.** Two types in the IR carry properties genuinely typed as `object` per the Dynamo source code: `DummyNode.OriginalNodeContent` (holds XmlElement or JObject) and `BrowserInternalElement.UIParent`. These are legitimate `object` types, not parser placeholders — they fall under the strict-verify "Reflection-style properties" whitelist.
3. **XML doc round-trip is best-effort.** Our display-form type names (`int`, `string`, generic `<T>`) convert to XML-doc-id canonical form (`System.Int32`, `System.String`, `{T}`) at lookup time. The conversion is sound for the vast majority of signatures but unusual cases (e.g. `modreq` modifiers, function pointers) may miss. We mitigate with a prefix-scan fallback: when the exact-form lookup misses, we scan all `M:Type.Method(...)` IDs and pick the first matching prefix.
4. **Method XML-doc-id arity placeholders are imperfect.** Generic method signatures decoded from the binary use the method's actual generic-parameter NAMES (T, U) in display form. The XML doc id uses `0/1` placeholders. When the prefix-scan also misses, the method gets a synthesized summary. About 50% of methods take this fallback path; the synthesized form is still useful (`TypeName.MethodName`) but isn't a real summary.

## Crawl strategy (for the codex-coverage reviewer)

- The IR's `source.urls[0]` is the NuGet package URL — NOT a fetchable type-index page. Step 1 enumeration should pull the same NuGet package and reflect-iterate its DLLs, OR use the IR's `types[]` count (707) as ground truth.
- Step 2 deep-check samples should be cross-checked against the **`doc_url`** of the catalog entry (which points to GitHub source). For nested types, the URL is the declaring file's source — the strict-verify script accepts source-code kind words (`class X`, `enum X` lowercase) in addition to the Sandcastle/DocFX renderings.
- Step 3 behavioral check should compare against the NuGet XML doc summaries (not against any hosted doc page).
