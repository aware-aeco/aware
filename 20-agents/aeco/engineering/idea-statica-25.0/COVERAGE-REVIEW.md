# Coverage review — idea-statica-25.0

**Verdict:** PASS
**Reviewer:** codex-rescue, run 2026-05-19T05:30Z
**IR source:** `cli-sidecar/Ingest/Output/idea-statica-25.0.ir.json` (sha256: `fd24af9bf1c7c9dceed20d4e2ec6a04c66a1aeb549ed3f65e4c68f42ace8483a`)

## Step 1 — Type enumeration
- Vendor sources (hybrid, pinned to GitHub tag `25.1.5.1491`):
  - Connection API spec: `https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/connection-api/clients/csharp/api/openapi.yaml`
  - RCS API spec: `https://raw.githubusercontent.com/idea-statica/ideastatica-public/25.1.5.1491/src/api-sdks/rcs-api/clients/csharp/api/openapi.yaml`
- Vendor type count: 175
  - Connection API classes (one per OpenAPI tag, suffixed `Api`): 16
  - Connection API schemas (`components/schemas/*`): 97
  - RCS API classes: 8
  - RCS API schemas: 54
- Catalog type count: 175
- **missing_types_count:** 0 ✓

Enumeration uses the same logic as the extractor (`OpenApiParser.cs`): every
`paths/<path>/<verb>` operation is grouped under a synthetic `<tag>Api` class
(tag from `operationId.tags[0]`), and every `components/schemas/*` key
becomes a `Model` class.

## Step 2 — Deep-check (50 random types)
- Seed: `2099556251` (= `sha256(IR)[:8]` as int32)
- Sample size: 50 / 175 random types
- Sampled: `IdeaStatiCa.RcsApi.Api.ClientApi, IdeaStatiCa.ConnectionApi.Model.BucklingRes, IdeaStatiCa.RcsApi.Model.RcsTendonBarData, IdeaStatiCa.ConnectionApi.Model.SelectedType, IdeaStatiCa.ConnectionApi.Model.Vector3D, IdeaStatiCa.ConnectionApi.Model.BendData, IdeaStatiCa.ConnectionApi.Api.MaterialApi, IdeaStatiCa.RcsApi.Model.RcsProjectData, IdeaStatiCa.RcsApi.Model.RcsReinforcedCrossSection, IdeaStatiCa.RcsApi.Model.ResultOfInternalForces, IdeaStatiCa.ConnectionApi.Model.ConDesignSetType, IdeaStatiCa.ConnectionApi.Model.IdeaParameterValidationResponse, IdeaStatiCa.RcsApi.Model.TendonDuct, IdeaStatiCa.RcsApi.Model.RcsCssComponentData, IdeaStatiCa.ConnectionApi.Model.DistanceComparison, IdeaStatiCa.ConnectionApi.Model.ConLoadEffect, IdeaStatiCa.RcsApi.Model.RcsSectionResultDetailed, IdeaStatiCa.ConnectionApi.Model.ConConversionSettings, IdeaStatiCa.RcsApi.Model.CheckResult, IdeaStatiCa.ConnectionApi.Model.AnchorGrid, IdeaStatiCa.ConnectionApi.Api.ClientApi, IdeaStatiCa.ConnectionApi.Model.ConTemplateApplyResult, IdeaStatiCa.ConnectionApi.Model.CheckResWeld, IdeaStatiCa.ConnectionApi.Model.ConNonConformityIssue, IdeaStatiCa.ConnectionApi.Api.OperationApi, IdeaStatiCa.ConnectionApi.Model.ConMprlCrossSection, IdeaStatiCa.RcsApi.Api.CrossSectionApi, IdeaStatiCa.ConnectionApi.Model.ConAnalysisTypeEnum, IdeaStatiCa.RcsApi.Model.TendonBarType, IdeaStatiCa.RcsApi.Model.OpenProject_request, IdeaStatiCa.ConnectionApi.Model.CutMethod, IdeaStatiCa.ConnectionApi.Model.CutBeamByBeamData, IdeaStatiCa.ConnectionApi.Model.ConWeldSizingMethodEnum, IdeaStatiCa.ConnectionApi.Model.ParameterUpdateResponse, IdeaStatiCa.ConnectionApi.Model.ConnectionCheckRes, IdeaStatiCa.RcsApi.Model.RcsTendonDuctData, IdeaStatiCa.ConnectionApi.Model.ConMemberPlacementDefinitionTypeEnum, IdeaStatiCa.ConnectionApi.Model.ConMember, IdeaStatiCa.RcsApi.Model.Region2D, IdeaStatiCa.ConnectionApi.Model.CheckResConcreteBlock, IdeaStatiCa.ConnectionApi.Model.PinGrid, IdeaStatiCa.RcsApi.Model.RcsCheckMember, IdeaStatiCa.ConnectionApi.Model.BoltGrid, IdeaStatiCa.ConnectionApi.Model.IdeaParameterValidation, IdeaStatiCa.ConnectionApi.Model.ConLoadEffectMemberLoad, IdeaStatiCa.RcsApi.Model.RcsSection, IdeaStatiCa.ConnectionApi.Model.ConLoadEffectPositionEnum, IdeaStatiCa.ConnectionApi.Model.IGroup, IdeaStatiCa.ConnectionApi.Model.Line, IdeaStatiCa.ConnectionApi.Model.Segment2D`
- Per-type checks:
  - For `*.Api` types: vendor's set of `operationId`s for the matching tag
    matches the IR's `methods[*].name` set (with `Async` suffix per the
    extractor's naming convention).
  - For `*.Model` types: schema name exists in the YAML's
    `components/schemas/` map.
- **deep_check_pass_rate:** 50/50 ✓

## Step 3 — Behavioral spot-check (20 random methods)
- Same seeded RNG continued (no re-seed).
- 20 methods sampled across the 50 deep-checked types.
- For each method: catalog `summary`/`remarks` are populated from the OpenAPI
  operation's `description`/`summary` field. When the catalog is empty, the
  vendor YAML is re-checked for that operationId to confirm vendor also empty
  (true no-prose-vendor counts as present per protocol).
- **behavior_present_rate:** 20/20 ✓

## Step 4 — Schema validation
- IR validated against `cli-sidecar/Ingest/Schema/host-coverage.schema.json` via
  `aware coverage validate` → status `ok`.
- All 175 catalog files validated against
  `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json` (Draft 2020-12) via
  jsonschema 4.26.0.
- Files validated: 176 (175 catalog + 1 IR)
- **schema_violations:** 0 ✓

## Reviewer notes / known caveats
- The agent directory name `idea-statica-25.0` intentionally carries the
  `.0` suffix to match the CLI's `agent-id` derivation `{host}-{version}`
  (i.e. host=`idea-statica`, host_version=`25.0`). This matches the task
  brief and existing `idea-statica-25` placeholder dir is unused.
- Five known markdown 404-fallbacks are documented in `EXTRACTION-NOTES.md`
  (`OpenProject_request`, `ImportIOM_request`, `UpdateFromIOM_request`,
  `ImportIOMFile_request` × 2 — synthesized request-body schemas without
  corresponding `.md` pages). Each fallback's `doc_url` redirects to the
  source `openapi.yaml`. These are NOT missing types — they are present in
  the catalog with valid doc_urls.
- `RcsSetting.Value` and similar object-typed properties are legitimate
  `object` per the OpenAPI source (no `type:` declared, `nullable: true`),
  not parser placeholders. Whitelist applied per protocol's
  "known-legitimate exemptions" section.

## Re-run command
`aware coverage review idea-statica-25.0`
