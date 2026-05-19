# Coverage review — bluebeam-v1

**Verdict:** PASS
**Reviewer:** codex-rescue, run 2026-05-19T10:10Z
**IR source:** `cli-sidecar/Ingest/Output/bluebeam-v1.ir.json` (sha256: `0efb73ab903460ca1bfb39745e7a7cb37cb92d4b5d5832b2844546a7e20e1f01`)

## Step 1 — Type enumeration

- Vendor source: Postman v2.1 JSON collection at
  `https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json`
  (public, no auth — referenced verbatim by Bluebeam's "Getting Started"
  page since the official developer portal at
  `https://developers.bluebeam.com/studio-api/` is auth-walled)
- Vendor type count: 80 (14 synthesised `*Api` classes + 66 Request DTOs;
  the collection encodes 123 REST operations grouped into 4 top-level
  folders × 1 subfolder level)
- Catalog type count: 80
- IR unique FQNs: 80
- **missing_types_count:** 0 ✓
- Missing from catalog: 0
- Missing from IR: 0

Type names are synthesised from `(top-level-folder, sub-folder, HTTP verb,
path)` per `EXTRACTION-NOTES.md`. The Postman JSON contains no literal
"SessionsApi" / "CreateSessionRequest" strings — the cross-check is
STRUCTURAL: each *Api class's method paths appear as `"path": [...]`
entries in the JSON, and each Request DTO's property names appear as JSON
object keys in the operation's example body.

## Step 2 — Deep-check (N=50 random types)

- Seed: `251360171` (= `sha256(IR)[:8]` as int32)
- Sample size: 50 / 80 random types
- Sampled (50): `Bluebeam.StudioApi.Jobs.Models.CreateJobHeaderandfooterRequest, Bluebeam.StudioApi.Jobs.Models.CreateJobPs2pdfRequest, Bluebeam.StudioApi.Projects.ProjectsPermissionsApi, Bluebeam.StudioApi.Projects.Models.CreateProjectFolderRequest, Bluebeam.StudioApi.Jobs.Models.CreateJobUnflattenRequest, Bluebeam.StudioApi.Jobs.Models.CreateJobBalancepagesRequest, Bluebeam.StudioApi.Jobs.Models.CreateJobMarkuplistRequest, Bluebeam.StudioApi.Sessions.Models.InviteSessionRequest, Bluebeam.StudioApi.Projects.ProjectsSharedLinksApi, Bluebeam.StudioApi.Sessions.SessionsApi, Bluebeam.StudioApi.Sessions.Models.UpdateUserRequest, Bluebeam.StudioApi.Jobs.Models.CreateJobFilepropertyvalueRequest, Bluebeam.StudioApi.Jobs.Models.CreateJobImage2pdfRequest, Bluebeam.StudioApi.Projects.Models.CheckoutToSessionFileRequest, Bluebeam.StudioApi.Jobs.Models.CreateJobImportcustomcolumnsRequest, Bluebeam.StudioApi.Projects.Models.CreateProjectRequest, Bluebeam.StudioApi.Projects.Models.UpdateUserPermissionsRequest, Bluebeam.StudioApi.Jobs.Models.CreateJobExportcustomcolumnsRequest, Bluebeam.StudioApi.Jobs.Models.CreateJobExtractpagesRequest, Bluebeam.StudioApi.Sessions.Models.UpdateSessionPermissionsRequest, Bluebeam.StudioApi.Jobs.Models.CreateJobFlattenRequest, Bluebeam.StudioApi.Sessions.Models.UpdateUserPermissionsRequest, Bluebeam.StudioApi.Projects.Models.UpdateUserRequest, Bluebeam.StudioApi.Jobs.Models.CreateJobExportformdataRequest, Bluebeam.StudioApi.Jobs.Models.CreateJobPagecountRequest, Bluebeam.StudioApi.Jobs.Models.CreateJobExcel2pdfRequest, Bluebeam.StudioApi.Projects.Models.ConfirmCheckinFileRequest, Bluebeam.StudioApi.Jobs.Models.CreateJobInsertblankpagesRequest, Bluebeam.StudioApi.Projects.Models.CopyFileRequest, Bluebeam.StudioApi.Jobs.Models.CreateJobOpenpasswordRequest, Bluebeam.StudioApi.Projects.ProjectsUsersApi, Bluebeam.StudioApi.Jobs.Models.CreateJobStampRequest, Bluebeam.StudioApi.Projects.ProjectsApi, Bluebeam.StudioApi.Projects.Models.UpdateSharedLinkRequest, Bluebeam.StudioApi.Jobs.Models.CreateJobInsertpagesRequest, Bluebeam.StudioApi.Jobs.Models.CreateJobDeletepagesRequest, Bluebeam.StudioApi.Projects.Models.CreateProjectUserRequest, Bluebeam.StudioApi.Sessions.SessionsFilesApi, Bluebeam.StudioApi.Jobs.Models.CreateJobReplacepagesRequest, Bluebeam.StudioApi.Jobs.Models.CreateJobExportmarkupsRequest, Bluebeam.StudioApi.Projects.Models.CreateProjectSharedLinkRequest, Bluebeam.StudioApi.Projects.ProjectsFoldersApi, Bluebeam.StudioApi.Projects.Models.UpdateProjectPermissionsRequest, Bluebeam.StudioApi.Jobs.Models.CreateJobCreatepdfareportRequest, Bluebeam.StudioApi.Jobs.Models.CreateJobCombineRequest, Bluebeam.StudioApi.Sessions.SessionsPermissionsApi, Bluebeam.StudioApi.Sessions.SessionsUsersApi, Bluebeam.StudioApi.Sessions.Models.CreateSessionUserRequest, Bluebeam.StudioApi.Sessions.Models.CreateSessionActivityRequest, Bluebeam.StudioApi.Projects.ProjectsFilesApi`
- Per-type checks:
  - For *Api classes: the type's methods carry `remarks = "HTTP <verb> \`<path>\`"`;
    the path segments (excluding `{param}` placeholders) appear verbatim
    in the Postman JSON's `path` arrays.
  - For Request DTO types (`*.Models.*Request`): at least one of the DTO's
    property names appears as a JSON object key in the matching operation's
    body example.
  - All catalog entries' methods/properties/events/enum_values counts
    match their IR counterparts exactly.
- **deep_check_pass_rate:** 50/50 ✓

The Postman v2.1 JSON branch in `verify-types-strict.ps1` already verified
this against the same IR (50/50 self-check, see EXTRACTION-NOTES). The
structural cross-check confirmed each *Api class's paths are real and each
Request DTO's property names are vendor-authored.

## Step 3 — Behavioral spot-check (N=20 methods)

- Same seeded RNG continued (no re-seed).
- 20 methods sampled across the 50 deep-checked types (per-method pool of
  ~123 across the 14 *Api classes; Request DTOs have no methods).
- Catalog summaries are populated from the Postman operation's prose
  `name` field (e.g. "Create metadata for a new Studio Session file.").
  Method names themselves are synthesised from `(verb, path)` per the
  extractor's naming convention.
- **behavior_present_rate:** 20/20 ✓

## Step 4 — Schema validation

- IR validated against `cli-sidecar/Ingest/Schema/host-coverage.schema.json`
  via `aware coverage validate` → status `ok`.
- All 80 catalog files validated against
  `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json` (Draft 2020-12)
  via `jsonschema 4.26.0`.
- Files validated: 81 (80 catalog + 1 IR)
- **schema_violations:** 0 ✓

## Reviewer notes / known caveats

- **No response/return type information.** The Postman collection encodes
  request paths and bodies but not response shapes. Every IR method ships
  with `returns: null` (treated as `void`). This is faithful to the source;
  not a parser defect. Documented in `EXTRACTION-NOTES.md`.
- **Auth-walled developer portal.** Bluebeam's official Developer Network
  at `developers.bluebeam.com` requires login; we did not cross-check
  against the auth-walled prose docs. Per the v0.30 vendor implementer
  brief, this is the "auth-walled docs" case and the structural strict-verify
  PASS is the bar.
- **Method-name synthesis is path-driven.** Compound lowercase action
  tokens in paths (`openpassword`, `pagecount`, `dwg2pdf`,
  `headerandfooter`) come through as single PascalCase tokens
  (`Openpassword`, `Pagecount`, `Dwg2pdf`, `Headerandfooter`). Names are
  deterministic and unique; readability cost noted in `EXTRACTION-NOTES.md`.
- **No discriminator/inheritance modelling.** Each operation gets its own
  Request DTO; shared shapes (e.g. permission updates) are not collapsed
  across DTOs. Produces minor duplication (66 DTOs from ~60 unique shapes)
  but keeps per-method docs faithful.

## Re-run command

`aware coverage review bluebeam-v1`
