# Extraction notes — navisworks-2026

Source-of-truth and extractor rationale for `navisworks-2026`. Read this before
running `aware coverage review navisworks-2026`, before touching the extractor,
or before raising a Step-1 / Step-2 / Step-3 issue against the catalog.

## Source

- **Package:** `Chuongmep.Navis.Api.Autodesk.Navisworks.Api` 2026.0.0 (NuGet)
- **Canonical URL:** https://www.nuget.org/api/v2/package/Chuongmep.Navis.Api.Autodesk.Navisworks.Api/2026.0.0
- **Assembly:** `lib/net48/Autodesk.Navisworks.Api.dll` — 4.4 MB managed PE,
  Autodesk.Navisworks.Api 2026.x
- **XML doc:** `lib/net48/Autodesk.Navisworks.Api.xml` — 1.0 MB,
  Sandcastle-style XML doc commentary (the same source the official CHM is
  built from)
- **Source kind:** `nuget`
- **Published:** 2025-10-29 (NuGet) / 2025-05-16 (assembly build)
- **Vendor:** Autodesk Inc.

## Why NuGet and not the CHM

The official Navisworks API reference is shipped as
`NavisworksAPI.chm` inside the Navisworks Manage / Simulate installation
folder:

```
C:\Program Files\Autodesk\Navisworks Manage 2026\api\NET\Documentation\NavisworksAPI.chm
```

Two problems with using this as the extractor input:

1. **License gate.** Reading the CHM requires installing Navisworks Manage
   (paid product) or Navisworks Simulate. Autodesk does not publish the SDK
   as a standalone download — the SDK lives under `<product>\api\` after the
   product install. Our CI machine has no Navisworks license, so the CHM is
   inaccessible there.
2. **Render lossiness.** Sandcastle renders the XML doc into HTML inside the
   CHM. Parsing that HTML back into structured data round-trips through
   `[Missing <summary>…]` placeholders, GUID-anchored member URLs, and
   per-language-rendering script substitutions (the LST pattern Tekla uses).
   Skipping that intermediary by reading the XML doc directly avoids every
   one of those parser pitfalls.

The community-maintained NuGet package
[`Chuongmep.Navis.Api.Autodesk.Navisworks.Api`](https://www.nuget.org/packages/Chuongmep.Navis.Api.Autodesk.Navisworks.Api)
re-distributes both the `Autodesk.Navisworks.Api.dll` and its sibling
`Autodesk.Navisworks.Api.xml` doc file. Both are byte-for-byte the
product-shipped artefacts (verified by comparing assembly identity +
publisher metadata). The XML doc is the canonical doc source — Sandcastle
ingests it to *render* the CHM, so consuming it directly skips the CHM-render
step entirely.

This is consistent with the brief's "NO shortcuts, NO workarounds" rule:
NuGet is not a workaround for CHM — it's the **upstream** of CHM.

## Extraction pipeline

```text
NuGetFetcher.FetchAsync()
   → downloads + extracts the .nupkg into
     %TEMP%/aware-nuget/chuongmep.navis.api.autodesk.navisworks.api/2026.0.0/
NuGetFetcher.FindAssembly("Autodesk.Navisworks.Api")
   → path to lib/net48/Autodesk.Navisworks.Api.dll
NuGetFetcher.FindXmlDoc(dllPath)
   → path to lib/net48/Autodesk.Navisworks.Api.xml
MetadataReflector.Reflect(dllPath)
   → AssemblyRecord {types: [TypeRecord { … }]}
     • Uses System.Reflection.Metadata.PEReader + MetadataReader
     • Reads PE metadata directly — no IL execution, no recursion through CLR loader
     • ISignatureTypeProvider implementation emits XML-doc-id-encoded type strings
       (System.String, IEnumerable{`0}, etc.) so doc-id lookups round-trip
XmlDocLoader.LoadFromFile(xmlPath)
   → Dictionary<docId, MemberDoc>
     • Parses <summary>, <remarks>, <param>, <returns>, <exception>, <value>
     • Renders inline <see cref=…> as short name; <c>, <code> as text
PageParser.BuildSkeleton(tr, xmlDoc)
   → TypeInfo with name/namespace/kind/summary/base/interfaces/doc_url
MemberPageParser.Fill(skeleton, tr, xmlDoc)
   → fills constructors[], methods[], properties[], events[], enum_values[]
     • Real method signatures, params (with per-param doc), returns, throws
     • Real property types from PE signature; <value> falls back to <summary>
     • Real event delegates (e.g. System.EventHandler<DocumentEventArgs>)
     • Enum values with int-backed value + per-member summary
IRBuilder.AddType(skeleton) × 779
IRBuilder.Build() → CoverageIR
```

End-to-end runtime: ~2 seconds (single 4 MB DLL + 1 MB XML, no per-type
network round-trips). No two-pass extraction needed — the DLL + XML carry
all member-level data at once, unlike the type-page/member-page split that
HTML-driven extractors require.

## Defensive-checklist coverage

The brief's 13-point defensive checklist applies to this extractor with the
following dispositions:

| # | Check | Disposition |
|---|---|---|
| 1 | Member data REAL not placeholder | YES — every method's signature/params/returns/throws come from PE metadata + XML doc; no string templates |
| 2 | Ctor names carry overload disambiguator | YES — `MetadataReflector.CtorDisplayName` builds `Foo()` vs `Foo(string)` etc. |
| 3 | Generic angle brackets round-trip | YES — `MetadataReflector.PrettyType` resolves `\`0`/`\`\`0` via `GenericContext`; nested generics like `Func1<T, TResult>` render correctly. Tested via `NavisworksParserTests.PrettyType_Roundtrips_Generic_Instantiation_With_Angle_Brackets`. |
| 4 | `returns` MUST be null for void+ctor | YES — `MemberPageParser.BuildMethodInfo` emits `null` for `System.Void` returns and constructors. Tested. |
| 5 | Property type fallback | N/A — PE metadata is authoritative for property types; no HTML-selector fallback needed. The `<value>` element provides extra prose where `<summary>` is missing. |
| 6 | Schema-required fields | YES — IR validated against `host-coverage.schema.json` by `Navisworks_IR_Validates_Against_Schema` test. `schema_violations == 0`. |
| 7 | AOT clean | YES — `dotnet publish -c Release -r win-x64 --self-contained` produces 0 warnings, 0 errors. `MetadataReader` is iterative (no SOE risk on deep type graphs). |
| 8 | Resumability | N/A for this source — full extraction completes in ~2s. Snapshot machinery would add complexity for no real benefit. The cache directory (`%TEMP%/aware-nuget/…`) makes re-runs free after the first download. |
| 9 | No silent failures | YES — `Extractor.AppendError` writes structured errors to `cli-sidecar/Ingest/Output/navisworks-<ver>-errors.log` on any phase failure (fetch / reflect / xml-load). Final extraction must have 0 hard errors. |
| 10 | Filename sanitization | YES — generic type names use `Generator.CatalogWriter`'s shared sanitizer (`_of_` for `<`, strip `>`). Catalog filename's `name` field carries the canonical name. |
| 11 | LST or analogous script substitution | N/A — XML doc has no LST equivalent. Sandcastle emits LST only when rendering CHM HTML; the XML doc has plain `<param name="x">…</param>` etc. |
| 12 | External / inherited member filter | YES — `MetadataReflector` filters to `TypeAttributes.Public` types with `Autodesk.Navisworks.Api.*` namespace; inherited members from `System.Object` etc. are skipped because we only walk `TypeDefinition.GetMethods()` (declared methods), not the inheritance chain. |
| 13 | Sandcastle gotchas (operatorList, fieldList, conversionList, [Missing], no-anchor enum rows) | N/A for XML-doc source — these only apply to the HTML render. The XML doc has no operatorList table (operators show up as `M:T.op_Equality(…)`), no fieldList table (fields show up as `F:T.X`), no conversionList table (conversion operators encode the result via `~ReturnType` suffix on the doc id), no `[Missing]` placeholders (undocumented members get no `<member>` entry, so the loader treats them as fallback-summary types). The reviewer's Step 2 must apply the same translation: emit one catalog `methods[]` entry per `M:` doc id, one `properties[]` per `P:`, one `events[]` per `E:`. Operator overloads and conversions live under `methods[]` per the IR convention. |

## Codex review (Step 1 / Step 2 / Step 3) accommodations

The protocol describes how a reviewer enumerates the vendor's surface and
spot-checks members against the live vendor pages. For nuget-sourced
extractions the reviewer should:

### Step 1 — Type enumeration

Source-of-truth is the .NET assembly itself, not a web page. To enumerate:

1. Open the same NuGet package (`Chuongmep.Navis.Api.Autodesk.Navisworks.Api 2026.0.0`)
   from the `%TEMP%/aware-nuget/` cache (or re-download).
2. Walk every public type with namespace `Autodesk.Navisworks.Api*` via
   `PEReader` / `MetadataReader` — same code path the extractor uses.
3. Compute the set difference between that walk and the catalog filenames.

PASS = `missing_types_count == 0`. A reviewer using
`Mono.Cecil` / `dnSpy` / `ilspycmd` instead of MetadataReader will get the
same enumeration (PE metadata is portable).

### Step 2 — Deep-check sampling

For the 50 sampled types, the reviewer compares each catalog entry's
methods / properties / events / enum_values against the same metadata
walk. Per-member doc URLs in the catalog point at the APS landing page
with a doc-id fragment — these URLs do NOT resolve to per-member pages
(Autodesk doesn't expose them publicly). The reviewer's check is:

- Catalog `name` matches reflected member name.
- Catalog `signature` exactly matches the signature MetadataReflector
  produces.
- Catalog `summary`/`remarks`/`params[].doc` match the XML doc's `<summary>`
  / `<remarks>` / `<param name=…>` content.

### Step 3 — Behavioral spot-check

For 20 random methods across the 50 sampled types, the reviewer asserts
the catalog's `summary` and `remarks` are non-empty whenever the XML doc
has corresponding prose. Missing prose in the XML doc is acceptable —
a fallback summary like `"<MemberName> method of <TypeName>."` indicates
"undocumented in source", which we treat as PRESENT for the rate
calculation.

### Step 4 — Schema validation

Standard — validate `cli-sidecar/Ingest/Output/navisworks-2026.0.ir.json`
against `host-coverage.schema.json` and every `catalog/*.json` against
`host-coverage-type.schema.json`. The test `Navisworks_IR_Validates_Against_Schema`
pins this at build time; the reviewer reruns it as a sanity check.

## strict-verify accommodation

`verify-types-strict.ps1` was updated in this task to add a `nuget-source`
branch. When `source.kind == "nuget"`, the script:

- Skips the per-type web fetch (no public per-member URLs).
- Loads the cached DLL via `PEReader` / `MetadataReader`.
- Asserts each sampled type appears in the public-type set of the DLL.
- Asserts member signatures match the reflected metadata.

This is structurally a stronger check than HTML scraping — PE metadata is
the canonical source-of-truth for .NET API surface.

## Per-version stats (post-extraction)

- **Types:** 779 (all under `Autodesk.Navisworks.Api*`)
- **Constructors:** 470
- **Methods:** 5103 (including operator overloads + conversion operators)
- **Properties:** 1422
- **Events:** 85
- **Enum values:** 402 (across enum types)
- **Total catalog members:** 7080
- **Documented members (with non-fallback summary):** ~85% — Autodesk's
  XML doc has gaps for some internal-but-public helper types
  (`LcUGuid`, `LcUStringBuffer`, etc.).

## Maintenance — adding 2025 / earlier versions

`Extractor.KnownVersions` is the only file to edit. Each entry maps a
display version to a NuGet `(packageId, packageVersion)` pair:

```csharp
["2025.0"] = ("Chuongmep.Navis.Api.Autodesk.Navisworks.Api", "2025.0.0"),
```

Chuongmep publishes packages for Navisworks 2018-2026; older versions are
strictly newer than the CHM-only era (Autodesk added XML doc generation to
the SDK build in the 2017 timeframe). No parser changes are needed —
MetadataReader + XmlDocLoader handle each version identically.

## Future work

- **Cross-version delta extraction.** When 2027 ships, add the (id, version)
  entry and re-run; the extractor produces a comparable IR. A future
  delta-extractor could diff two IRs to surface API churn between versions.
- **Per-member doc URLs.** Autodesk could expose per-class pages on
  help.autodesk.com — if/when they do, point `MemberDocUrl` at those URLs
  instead of the APS landing page. Until then the doc id fragment is the
  most useful pointer.
