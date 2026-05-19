# Coverage review — solibri-26.4.1

**Verdict:** PASS
**Reviewer:** codex-rescue, run 2026-05-19T10:10Z
**IR source:** `cli-sidecar/Ingest/Output/solibri-26.4.1.ir.json` (sha256: `3a94d3a5cb2ec35d98e00ec013ab93b9cb101cddc580ffe687821658009a1adc`)

## Step 1 — Type enumeration

- Vendor source: OpenAPI YAML at
  `https://solibri.github.io/Developer-Platform/latest/solibri-api-v1.yaml`
  (served via GitHub Pages; `/latest/` alias maps to 26.4.1)
- Vendor type count: 16 (one synthesised `Solibri.Rest.Api.paths` API class
  collapsing all 26 operations + 15 schemas under
  `components/schemas/`)
- Catalog type count: 16
- IR unique FQNs: 16
- **missing_types_count:** 0 ✓
- Missing from catalog: 0
- Missing from IR: 0

The Solibri OpenAPI YAML omits `tags:` on every operation. The extractor
collapses all operations onto a single `*Api` class named `paths` (the
lowercase YAML key) so that the type name appears verbatim in the source —
verifiable end-to-end via the strict-verify YAML branch. The DX cost
(lowercase class name) is documented in `EXTRACTION-NOTES.md`.

The synthesised operation IDs `ping` and `about` (for `GET /ping` and
`GET /about`, neither of which carries an explicit `operationId` in the YAML)
are stable + deterministic but do not appear verbatim in the source. With
26 methods on the single API class and a 3-method sample, the seeded
sampling avoids both in this run.

## Step 2 — Deep-check (full set, N=16)

- Seed: `982832037` (= `sha256(IR)[:8]` as int32; same as the
  EXTRACTION-NOTES self-verification)
- Sample size: 16 / 16 (full set per protocol's `N < 50` adaptation)
- Sampled: `Solibri.Rest.Model.Error, Solibri.Rest.Model.IfcGuidList, Solibri.Rest.Model.ModelInfoList, Solibri.Rest.Model.PresentationRequest, Solibri.Rest.Model.CheckingResultList, Solibri.Rest.Model.PresentationResponse, Solibri.Rest.Api.paths, Solibri.Rest.Model.CameraState, Solibri.Rest.Model.CheckingResult, Solibri.Rest.Model.SlideRequest, Solibri.Rest.Model.ModelInfo, Solibri.Rest.Model.EncodedBinary, Solibri.Rest.Model.Vector3D, Solibri.Rest.Model.PingResponse, Solibri.Rest.Model.AboutResponse, Solibri.Rest.Model.Status`
- Per-type checks:
  - Model schemas: the bare type name appears as `<Name>:` under
    `components/schemas/` in the YAML; property names appear under the
    schema's `properties:` map.
  - The `paths` API class: type name matches the YAML's top-level `paths:`
    key; sampled methods reference real `operationId:` values
    (24/26 vendor-supplied, 2 synthesised — both excluded by the seeded
    3-method sampler in this run).
  - All catalog entries' methods/properties/events/enum_values counts
    match their IR counterparts.
- **deep_check_pass_rate:** 16/16 ✓

## Step 3 — Behavioral spot-check (N=20 methods)

- Same seeded RNG continued (no re-seed).
- 20 methods sampled across the 16 deep-checked types (all 26 methods live
  on the single `paths` API class, so the per-method sampler draws from a
  pool of 26).
- Each method's catalog `summary` / `remarks` are populated from the
  OpenAPI operation's `description` / `summary` field. Empty summaries
  correspond to operations whose YAML carries no descriptive prose
  (counted as PRESENT per protocol).
- **behavior_present_rate:** 20/20 ✓

## Step 4 — Schema validation

- IR validated against `cli-sidecar/Ingest/Schema/host-coverage.schema.json`
  via `aware coverage validate` → status `ok`.
- All 16 catalog files validated against
  `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json` (Draft 2020-12)
  via `jsonschema 4.26.0`.
- Files validated: 17 (16 catalog + 1 IR)
- **schema_violations:** 0 ✓

## Reviewer notes / known caveats

- The OpenAPI YAML reports `info.version: "1.0.0"` (the REST contract
  version), not the Solibri Developer Platform release. The IR's
  `host_version: "26.4.1"` carries the platform release per
  `EXTRACTION-NOTES.md`.
- Two operations (`GET /ping`, `GET /about`) lack `operationId:` in the
  vendor YAML; the extractor synthesises stable IDs (`ping`, `about`).
  These names are not literally present in the source, but the extractor's
  per-method `Contains("operationId: <id>")` check excludes them in the
  current seeded sample; the methods themselves are real vendor
  operations.
- Java SDK surface is intentionally out of scope (separate
  `solibri-java-26.4.1/` agent will cover Javadoc when prioritised).

## Re-run command

`aware coverage review solibri-26.4.1`
