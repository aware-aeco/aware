# bluebeam-v1 EXTRACTION NOTES

This agent is generated from a host-coverage IR built by fetching the **public Postman v2.1 collection** for the Bluebeam Studio API. Bluebeam's official developer portal at `https://developers.bluebeam.com/studio-api/` requires authentication, but Bluebeam also publishes a fully-public Postman collection JSON that the "Getting Started" page at `https://support.bluebeam.com/developer/getting-started-dev-portal.html` references verbatim. That JSON is the only public machine-readable description of the Studio API and is what we ingest here.

## Source

- **Vendor portal:** `https://developers.bluebeam.com/studio-api/` (Salesforce-hosted Developer Network — returns 401 without sign-in)
- **Canonical machine-readable source:** `https://support.bluebeam.com/developer/resources/bluebeamapi-postman.json` (Postman v2.1 collection, ~45 KB, public, no auth)
- **Human-facing reference (subset only):** `https://support.bluebeam.com/developer/studio-session-guide.html` — covers only the Sessions surface in prose form
- **Postman web renderer (full collection):** `https://www.postman.com/bluebeam-developers/bluebeam-api/documentation/ngtxoq6/studio-api` — interactive HTML view of the same JSON
- **Source kind:** `scrape` (single HTTP fetch of a machine-readable JSON)
- **Extraction date:** 2026-05-19

## Versioning

Bluebeam does NOT version the Studio API surface beyond the `/publicapi/v1/` path prefix. There is no v2, no deprecation tracking, no per-product release notes for the API itself. The IR stamps `host_version: v1` to mirror the path prefix. The extractor accepts both `"v1"` and `"1.0"` (alias) as the input version selector. Per-edition tracking would only make sense once Bluebeam ships an incompatible v2 of the surface; for now this is a single-version agent.

## Reproduction

```bash
# From repo root — run the sidecar via stdin envelope.
echo '{"op":"coverage-extract","args":{"vendor":"bluebeam","version":"v1","out_path":"cli-sidecar/Ingest/Output/bluebeam-v1.ir.json"}}' \
  | cli-sidecar/bin/Release/net10.0/win-x64/publish/aware-sidecar.exe

# Regenerate the agent from the IR:
AWARE_SIDECAR=cli-sidecar/bin/Release/net10.0/win-x64/publish/aware-sidecar.exe \
  cli/target/release/aware.exe coverage generate bluebeam v1 \
  --from-ir cli-sidecar/Ingest/Output/bluebeam-v1.ir.json \
  --vendor bluebeam --vertical architecture
```

The extractor lives at `cli-sidecar/Ingest/Extractors/Bluebeam/` (Extractor.cs, PageParser.cs). The parser is JSON-driven; there is no separate `MemberPageParser` because the entire surface is encoded in one JSON file (no per-member secondary fetch).

## Extraction strategy

A single-pass pipeline. The Postman collection encodes 123 REST operations grouped into 4 top-level folders (Sessions, Projects, Jobs, Misc) with one further level of subfolders (e.g. Sessions / Files, Projects / Folders, Jobs / Project File Jobs).

### Phase 1 — fetch (1 HTTP GET)

The Postman JSON is fetched once via `HttpScraper.FetchHtml` (45,613 bytes on the 2026-05-19 snapshot). No auth, no follow-up requests, no concurrency. End-to-end wall time: under 2 seconds.

### Phase 2 — parse (JSON walk → TypeInfo list)

`PageParser.ParseCollection` walks the Postman tree and emits TypeInfo records:

1. **One synthetic *Api class per (top-level, subfolder) pair.** Anonymous spacer folders and "Single \<X\>" UI-rollup subfolders fold INTO the parent's *Api class (their operations act on the same root path as the parent's own operations). The `Jobs / Job` UI grouping similarly folds into `JobsApi`. Class-name compounding: if the subfolder name already contains the top-level name (e.g. "Project File Jobs" under "Jobs"), the class name uses just the subfolder PascalCased + "Api" (→ `ProjectFileJobsApi`). Otherwise it concatenates top + sub (→ `SessionsFilesApi`).
2. **One synthetic Request DTO per operation that has a JSON body.** The DTO is named `<MethodName>Request` and lives in the parent class's namespace + `.Models` (e.g. `Bluebeam.StudioApi.Sessions.Models.CreateSessionRequest`). Property names come verbatim from the JSON body example; property types are inferred from the example value's JSON type (`""` → `string`, `0` → `int`, `0.0` → `double`, `true` → `bool`, `[]` → `List<T>`).
3. **Method names** are synthesised from `(HTTP verb, path)`:
    - GET on a collection (path tail is a non-param resource) → `List<Resource>` or `List<Parent><SubCollection>`
    - GET on an item (path tail is a `{param}` placeholder) → `Get<Resource>`
    - POST → `Create<Resource>` or `Create<Parent><SubCollection>` (singularised)
    - PUT → `Update<Resource>` (or specialised verbs — see below)
    - DELETE → `Delete<Resource>`
    - Trailing-action segments mapped to imperatives — `invite` → `Invite`, `checkin` → `Checkin`, `checkout` → `Checkout`, `copy` → `Copy`, `cancel` → `Cancel`, `confirm-upload` → `ConfirmUpload`, etc.
4. **Path parameters** like `{sessionId}`, `{projectId}`, `{id}`, `{fileId}`, `{folderId}`, `{userId}`, `{jobId}` become per-method `ParamInfo` entries with types pinned by a path-token → type map (`sessionId/projectId/userId/jobId/id` → `Guid`; `fileId/folderId/resultId` → `int`).
5. **Return types** are NOT documented anywhere in the collection. The Bluebeam Studio Session Guide describes a few of them in prose ("Response: SessionId", "Response: 204 status code") but with no consistent schema. The IR records `returns: null` (treated as void) on every method. A hand-written SDK would surface the response shape via a deserialiser-specific `Task<TResponse>` signature; here the value is omitted by design rather than fabricated.

### Phase 3 — assemble + emit

`IRBuilder` assembles the final `CoverageIR` with `source.kind=scrape`, `source.urls=[<Postman JSON URL>]`, and a single page-count entry.

## Verification

### Strict 50-type self-verification

Result: **50 / 50 PASS** on the most recent run (seed 1641495257).

The `cli-sidecar/Ingest/Output/verify-types-strict.ps1` script was extended with a `.json`-source dispatch branch for this vendor:

- For *Api class types: at least one of the type's methods must reference a path (stamped into `remarks` as `HTTP <verb> \`<path>\``) whose path segments appear as JSON tokens in the collection.
- For Request DTO types (`*.Models.*Request`): at least one of the DTO's property names must appear as a JSON object key in the collection.

This is the right structural cross-check for Postman-sourced vendors because synthesised type names (e.g. `CreateJobInsertblankpagesRequest`) do not literally appear in the JSON or in any human-facing doc — only their structural anchors (paths, body keys) are vendor-authored.

### Schema validation

Result: **0 violations** against `cli-sidecar/Ingest/Schema/host-coverage.schema.json` (root) and `host-coverage-type.schema.json` (per catalog file). Validation runs in the C# test suite at `cli-sidecar/Ingest/Generator/Tests/SchemaTests.cs` — see `Bluebeam_IR_Validates_Against_Schema` and `Bluebeam_Catalog_Validates_Against_Type_Schema`.

## Counts

| section | count |
|---------|------:|
| Source URLs (Postman JSON) | 1 |
| HTTP fetches | 1 |
| Operations in collection | 123 |
| *Api class types in IR | 14 |
| Request DTO model types | 66 |
| Total types in IR | 80 |
| Methods (across all *Api classes) | 123 |
| Properties (across all Request DTOs) | 219 |
| Events | 0 |
| Hard fetch failures after retry | 0 |
| Strict 50-type PASS rate | 50/50 |
| Schema violations | 0 |

### Api class distribution

| Api class | methods |
|-----------|--------:|
| `Bluebeam.StudioApi.Sessions.SessionsApi` | 6 |
| `Bluebeam.StudioApi.Sessions.SessionsFilesApi` | 8 |
| `Bluebeam.StudioApi.Sessions.SessionsPermissionsApi` | 2 |
| `Bluebeam.StudioApi.Sessions.SessionsUsersApi` | 6 |
| `Bluebeam.StudioApi.Sessions.SessionsActivitiesApi` | 3 |
| `Bluebeam.StudioApi.Projects.ProjectsApi` | 7 |
| `Bluebeam.StudioApi.Projects.ProjectsFoldersApi` | 6 |
| `Bluebeam.StudioApi.Projects.ProjectsFilesApi` | 16 |
| `Bluebeam.StudioApi.Projects.ProjectsUsersApi` | 7 |
| `Bluebeam.StudioApi.Projects.ProjectsPermissionsApi` | 4 |
| `Bluebeam.StudioApi.Projects.ProjectsSharedLinksApi` | 5 |
| `Bluebeam.StudioApi.Jobs.JobsApi` | 5 |
| `Bluebeam.StudioApi.Jobs.ProjectFileJobsApi` | 44 |
| `Bluebeam.StudioApi.Misc.MiscApi` | 4 |
| **TOTAL** | **123** |

## Parser quirks + known limitations

1. **No response/return type information.** The Postman collection encodes request paths and (optionally) request bodies, but not response shapes. Every IR method ships with `returns: null`. The vendor's prose docs at `support.bluebeam.com/developer/studio-session-guide.html` describe a handful of response shapes informally; capturing those would require hand-curation, which contradicts the "automated extraction" contract of v0.30. Future enhancement: pair the Postman ingest with an LLM-summarised pass over the prose docs to fill in response types where they can be unambiguously identified.

2. **No example data is preserved.** The Postman JSON's example bodies are blank-valued (`{"CRC":"", "Name":"", "Size":0, "Source":""}`) and only useful for property type inference. The IR records the inferred types but discards the example values — they convey no real semantics beyond the JSON-type signal.

3. **Method-name synthesis is path-driven, not name-driven.** Postman's `name` field is human prose ("Create metadata for a new Studio Session file.") which produces ugly identifiers under PascalCase. The parser instead derives method names from the HTTP verb + last meaningful path segments. The Postman prose is preserved as the method's `summary`. This is the same pattern OpenAPI's `operationId` would take, and matches the SDK shape a hand-written client would expose.

4. **Compound lowercase action tokens are not split.** Path segments like `openpassword`, `filepropertyvalue`, `pagecount`, `dwg2pdf`, `saveaspdfa`, `headerandfooter` come through the synthesizer as single PascalCase tokens (`Openpassword`, `Filepropertyvalue`, etc.) because the vendor uses lowercase compound names with no separator. A more aggressive splitter could attempt to break on dictionary words but the heuristics are unreliable; the current names are unique and deterministic.

5. **Postman v2.1 schema dependency.** The parser is pinned to the v2.1 schema structure (`collection.item[].request.method/url.path[]/body.raw`). If Bluebeam migrates to v2.2 or v3 the parser would need updating. The schema URL is recorded in `collection.info.schema` — a defensive check could assert against this in a future revision.

6. **No discriminator / inheritance modelling.** The Bluebeam JSON has no schema graph; each request body is described inline by its example. Cross-DTO relationships (e.g. shared shapes for `{Allow, Type}` permission updates) are NOT collapsed into a single shared type — each operation gets its own Request DTO. This produces some duplication (66 DTOs from 60-ish unique shapes) but keeps the per-method documentation faithful to the source.

7. **Auth-walled developer portal.** The Bluebeam Developer Network at `developers.bluebeam.com` requires login; we couldn't cross-check the Postman JSON against the official API references behind the auth wall. Per the v0.30 vendor implementer brief, this is the "auth-walled docs" case, and the agent is delivered with the structural strict-verify PASS as the bar — verifying against vendor-authored prose would require manual review by a Bluebeam customer with portal access.

## Extraction errors log

If errors occur during extraction (HTTP failures other than 404, parse failures), they are appended to `cli-sidecar/Ingest/Output/bluebeam-v1-errors.log`. Format matches the other extractors: tab-separated UTC timestamp + phase + URL + message. A current run produces 0 hard errors end-to-end.
