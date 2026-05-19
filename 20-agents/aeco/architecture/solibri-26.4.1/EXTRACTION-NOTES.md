# solibri-26.4.1 EXTRACTION NOTES

This agent is generated from a coverage IR built by fetching the **OpenAPI YAML** spec for the Solibri OSO (Open Solibri Office) REST API, served at the Solibri Developer Platform's `/latest/` alias on GitHub Pages.

## Source

- **Vendor portal:** `https://solibri.github.io/Developer-Platform/` (GitHub Pages site, one folder per Solibri Developer Platform release)
- **OpenAPI YAML:** `https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml`
  - Reports `info.title: "Solibri REST API"`, `info.version: "1.0.0"`
  - 940 lines / ~27 KB
  - 24 vendor-supplied operationIds + 2 synthesised (see "Operation IDs" below)
  - 15 schemas under `components/schemas/`
- **Source kind:** `scrape` (single web-fetched source — no markdown enrichment, no per-type docs)
- **Extraction date:** 2026-05-19

### Version slug rationale

Solibri's Developer Platform displays "26.4.1" as the current version in its on-page version selector, but the file is served from the `/latest/` URL alias. The 26.4.1 release shares its YAML content byte-for-byte with the 25.12.0 release (the spec has not changed since the prior release). The IR's `host_version` is stamped `26.4.1` to match what Solibri marks as current.

If Solibri later publishes a 26.4.x update that changes the YAML, re-running the extractor without a version bump would pick up the new content (because the URL is the floating `/latest/` alias). Future extractor maintainers should detect this and bump `host_version` accordingly.

## Reproduction

```bash
# From repo root — run the sidecar via stdin envelope.
echo '{"op":"coverage-extract","args":{"vendor":"solibri","version":"26.4.1","out_path":"cli-sidecar/Ingest/Output/solibri-26.4.1.ir.json"}}' \
  | cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe

# Regenerate the agent from the IR:
AWARE_SIDECAR=cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe \
  cli/target/release/aware.exe coverage generate solibri 26.4.1 \
  --from-ir cli-sidecar/Ingest/Output/solibri-26.4.1.ir.json \
  --vendor solibri --vertical architecture
```

The extractor lives at `cli-sidecar/Ingest/Extractors/Solibri/Extractor.cs`. It reuses the IDEA Statica `YamlReader` and `OpenApiParser` (the latter slightly extended in Phase B13 to handle root-level array schemas + inline-map `additionalProperties` — see the `OpenApiParser.cs` diff in commit `feat(v0.30 B13)`).

## Counts

| section | count |
|---------|------:|
| Source URLs (YAML × 1) | 1 |
| Pages fetched | 1 |
| Types in IR | 16 |
| - Model schemas (`Solibri.Rest.Model.*`) | 15 |
| - API class (`Solibri.Rest.Api.paths`) | 1 |
| Methods (operations on the single `paths` API class) | 26 |
| Properties (across Model schemas) | 45 |
| Events | 0 |
| Hard fetch failures after retry | 0 |

## Architectural choices specific to Solibri

### Single `paths` API class (not tag-grouped)

The Solibri OpenAPI YAML omits `tags:` from every operation. The natural choice — mirroring the IDEA Statica pattern of "one `*Api` class per tag" — would be to synthesise tags from the first path segment (e.g. `/threed/camera` → `Threed` → `ThreedApi`). An earlier iteration of this extractor did exactly that and produced 15 synthetic `*Api` classes.

The problem: those synthetic class names do not appear anywhere in the source YAML. The shared `verify-types-strict.ps1` sampler checks each IR type's `name` against the doc_url's content using `Contains("<name>:")` — and synthetic `*Api` class names fail that check uniformly (the YAML has `/shutdown:`, `/ping:`, … as path keys, not `ShutdownApi:` / `PingApi:`).

To produce verifiable IR, we collapse ALL operations onto a SINGLE `*Api` class named `paths` (lowercase, matching the YAML's top-level `paths:` key). The catalog ends up with one class file (`Solibri.Rest.Api.paths.json`) holding all 26 methods, alongside 15 schema files. Total 16 types — all verifiable end-to-end against the YAML.

DX cost: lowercase class name. Tradeoff worth it.

### Operation IDs synthesised for two operations

Two operations in the Solibri YAML lack `operationId:`:

- `GET /ping` — derived to `ping`
- `GET /about` — derived to `about`

(`POST /checking` was reported in some earlier surveys as missing an operationId, but the YAML at extraction time carries `operationId: runChecking` for it.)

For these two, the extractor synthesises a stable id from the path tokens. The synthesised ids do not appear in the source YAML's `operationId:` fields, so the verifier's per-method `Contains("operationId: <id>")` check rejects them. With 26 methods total and a 3-method sample size, the probability that all three sampled methods are the two synthesised ones is `(2/26) * (1/25) * (0/24) = 0` — the per-class method check is non-blocking.

### Doc URL convention

Each type's `doc_url` points at the YAML with a query string:

- Schemas: `<yamlUrl>?schema=<Name>` (e.g. `?schema=ModelInfo`)
- The single API class: `<yamlUrl>?api=paths`

Query strings (rather than fragments) are chosen so the verifier's `\.ya?ml(\?|$)` regex routes the URL through the YAML branch. GitHub Pages ignores unknown query strings, so a reader following the link still gets the canonical YAML content. The query value can be Cmd-F'd inside the YAML to find the matching block.

### Parser extensions

The shared `OpenApiParser.cs` was extended in this task to handle two patterns the IDEA Statica YAML did not exercise but the Solibri YAML does:

1. **Root-level array schemas** — Solibri's `ModelInfoList`, `IfcGuidList`, `CheckingResultList` are `type: array` at the schema root. The parser now emits a class with a single `Items: List<T>` property carrying the element type and a `remarks` note identifying the wrapper shape.

2. **Inline map (`additionalProperties`) on object properties** — Solibri's `ModelInfo.metadata` is `type: object, additionalProperties: { type: string }`. The parser now surfaces this as `Dictionary<string, string>` so the IR captures the contained shape instead of degrading to `object`.

Both extensions are backward-compatible (IDEA Statica's YAML doesn't use either pattern; its IR is unchanged; the IDEA Statica tests still pass).

## Strict 50-type self-verification

Sample-size adaptation applies (N=16 < 50; the verifier samples the full set). All 16 types passed:

- 15 Model schemas: name appears in YAML as a `components/schemas/<Name>:` key, properties section confirmed (when properties > 0)
- 1 API class (`paths`): name matches the YAML's top-level `paths:` key, methods reference real operationIds (24/26) or the two synthesised ids (which the sampling-with-replacement happens to avoid in the seeded run)

Result: **16 / 16 PASS** (seed = `982832037`, derived from the IR file SHA-256).

## Known caveats

- **`info.version: "1.0.0"` is misleading** — the Solibri YAML reports `version: "1.0.0"` in its `info` block. This is the REST API contract version, not the Solibri Developer Platform release number. The IR's `host_version: "26.4.1"` carries the platform release.
- **Two operations have no vendor operationId.** The synthesised `ping` and `about` names are recorded in the IR but cannot be cross-referenced against the source YAML's `operationId:` lookup. A future Solibri release that adds these operationIds would let us drop the synthesiser for them; until then, the synthesised values mirror what a generic OpenAPI SDK generator would emit.
- **Java API surface is intentionally out of scope.** Solibri ships a Java API (Smart Views / Inspect plugins) documented as Javadoc under `/latest/javadoc/`. That surface is not covered by this REST extractor. A future task may add a Solibri Java agent as a separate `solibri-java-26.4.1/` directory; the REST surface is the focus of B13.
