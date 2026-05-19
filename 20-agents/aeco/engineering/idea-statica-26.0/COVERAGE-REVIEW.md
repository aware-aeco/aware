# Coverage review — idea-statica-26.0

**Verdict:** PASS
**Reviewer:** codex-rescue, run 2026-05-19T05:30Z
**IR source:** `cli-sidecar/Ingest/Output/idea-statica-26.0.ir.json` (sha256: `d7fb75a79c5931e6fb3384b0c9493d69a9d4c299aad3b392ecd92e72cd67df88`)

## Step 1 — Type enumeration
- Vendor sources (hybrid, pinned to GitHub tag `26.0.1.2450`):
  - Connection API spec: `https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/connection-api/clients/csharp/api/openapi.yaml`
  - RCS API spec: `https://raw.githubusercontent.com/idea-statica/ideastatica-public/26.0.1.2450/src/api-sdks/rcs-api/clients/csharp/api/openapi.yaml`
- Vendor type count: 178
  - Connection API classes: 16 (same set as v25)
  - Connection API schemas: 100 (3 new schemas vs v25 — see notes)
  - RCS API classes: 8 (same set as v25)
  - RCS API schemas: 54 (unchanged from v25)
- Catalog type count: 178
- **missing_types_count:** 0 ✓

Per `EXTRACTION-NOTES.md`: v26 Connection API adds 4 new operations + 3 new
schemas vs v25 (e.g. `CreateEmptyConnection`, `PublishConnection`, `Propose`,
`AddCrossSection`); RCS API is unchanged.

## Step 2 — Deep-check (50 random types)
- Seed: `1476097447` (= `sha256(IR)[:8]` as int32)
- Sample size: 50 / 178 random types
- Sampled: `IdeaStatiCa.RcsApi.Model.CalculationType, IdeaStatiCa.ConnectionApi.Model.CheckResConcreteBlock, IdeaStatiCa.RcsApi.Model.ConcreteCheckResultOverallItem, IdeaStatiCa.ConnectionApi.Model.CheckResSummary, IdeaStatiCa.ConnectionApi.Model.ConLoadEffectSectionLoad, IdeaStatiCa.ConnectionApi.Model.ConAlignedPlate, IdeaStatiCa.RcsApi.Model.RcsSection, IdeaStatiCa.RcsApi.Model.ConcreteCheckResultOverall, IdeaStatiCa.RcsApi.Model.RcsTendonBarData, IdeaStatiCa.ConnectionApi.Model.ConMember, IdeaStatiCa.ConnectionApi.Model.AnchorGrid, IdeaStatiCa.RcsApi.Api.SectionApi, IdeaStatiCa.ConnectionApi.Api.ProjectApi, IdeaStatiCa.ConnectionApi.Api.ConversionApi, IdeaStatiCa.RcsApi.Model.RcsStirrupsData, IdeaStatiCa.ConnectionApi.Api.MaterialApi, IdeaStatiCa.ConnectionApi.Model.ConAnalysisTypeEnum, IdeaStatiCa.ConnectionApi.Api.PresentationApi, IdeaStatiCa.ConnectionApi.Model.PinGrid, IdeaStatiCa.ConnectionApi.Model.WeldDefinition, IdeaStatiCa.ConnectionApi.Model.OpenMessage, IdeaStatiCa.ConnectionApi.Model.CheckResWeld, IdeaStatiCa.RcsApi.Model.ReinforcedCrossSectionData, IdeaStatiCa.RcsApi.Api.CrossSectionApi, IdeaStatiCa.ConnectionApi.Model.ConLoadEffectMemberLoad, IdeaStatiCa.ConnectionApi.Model.ConConnectionLibrarySearchParameters, IdeaStatiCa.ConnectionApi.Api.SettingsApi, IdeaStatiCa.ConnectionApi.Model.ConMemberForcesInEnum, IdeaStatiCa.RcsApi.Model.ConcreteCheckResult, IdeaStatiCa.RcsApi.Api.ProjectApi, IdeaStatiCa.ConnectionApi.Model.Text, IdeaStatiCa.ConnectionApi.Model.Vector3D, IdeaStatiCa.RcsApi.Model.SectionConcreteCheckResult, IdeaStatiCa.RcsApi.Model.RcsResultParameters, IdeaStatiCa.RcsApi.Model.RcsSectionLoading, IdeaStatiCa.ConnectionApi.Model.ConResultSummary, IdeaStatiCa.ConnectionApi.Model.ConnectionCheckRes, IdeaStatiCa.ConnectionApi.Model.UpdateFromIOM_request, IdeaStatiCa.RcsApi.Model.PolyLine2D, IdeaStatiCa.ConnectionApi.Model.OpenElementId, IdeaStatiCa.RcsApi.Model.RcsSectionResultOverview, IdeaStatiCa.ConnectionApi.Model.ConAlignedPlateSideCodeEnum, IdeaStatiCa.ConnectionApi.Model.IGroup, IdeaStatiCa.ConnectionApi.Model.CheckResPlate, IdeaStatiCa.RcsApi.Model.OpenElementId, IdeaStatiCa.ConnectionApi.Model.BucklingRes, IdeaStatiCa.RcsApi.Model.ReinforcedBar, IdeaStatiCa.ConnectionApi.Model.ConMemberModel, IdeaStatiCa.ConnectionApi.Model.Point2D, IdeaStatiCa.ConnectionApi.Model.ConWeldSizingMethodEnum`
- Per-type checks: same as v25 — Api-class operationId-set match,
  Model-class schema-key existence.
- **deep_check_pass_rate:** 50/50 ✓

## Step 3 — Behavioral spot-check (20 random methods)
- Same seeded RNG continued (no re-seed).
- 20 methods sampled across the 50 deep-checked types.
- Catalog prose pass-through from OpenAPI `description`/`summary`.
- **behavior_present_rate:** 20/20 ✓

## Step 4 — Schema validation
- IR validated against `cli-sidecar/Ingest/Schema/host-coverage.schema.json` via
  `aware coverage validate` → status `ok`.
- All 178 catalog files validated against
  `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json` (Draft 2020-12) via
  jsonschema 4.26.0.
- Files validated: 179 (178 catalog + 1 IR)
- **schema_violations:** 0 ✓

## Reviewer notes
- v26 adds 3 schemas + 4 operations to the Connection API (per
  EXTRACTION-NOTES.md). The reviewer enumeration found the same delta:
  Connection schemas v25=97 → v26=100, RCS unchanged.
- Same naming caveat as v25: agent dir is `idea-statica-26.0` with the `.0`
  suffix to match the CLI's `agent-id` derivation.
- Same 404-fallback list applies (synthesized `*_request` schemas without
  markdown docs); these are NOT missing types.

## Re-run command
`aware coverage review idea-statica-26.0`
