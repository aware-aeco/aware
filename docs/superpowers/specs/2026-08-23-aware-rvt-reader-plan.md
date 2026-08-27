# AWARE RVT reader — Task 6 implementation plan

> **For implementation:** Follow this plan test-first on
> `codex/xeorvt-aware-rvt-reader`. Do not push, tag, release, merge to AWARE `main`, or enable a
> FloLess UI. Keep the Residential model, converted artifacts, provider executable, provider URLs,
> credentials, and secrets outside both repositories.

**Status:** Revised after the fifth and final bounded adversarial review. Codex returned `REVISE` with
three concrete protocol findings; all three are incorporated below. The user explicitly authorized
implementation after resolving the final material findings. No reader implementation has been written.

**Goal:** Add a provider-neutral, local RVT reference-model reader to AWARE's existing
`aware-connection-reader` SEA. It converts one `.rvt` through a separately installed adapter,
normalises the adapter's GLB and explicit Revit metadata into deterministic bounded artifacts, and
publishes an authenticated multi-artifact receipt that FloLess Task 5 can ingest without learning
Revit or provider semantics.

**Anchors (verified 2026-08-23):**

- AWARE source: `main@934d5935b9c8a4d3c27e6cfdd862770173458a0e` (`v0.126.0`), clean before this plan.
- Installed CLI: reports `0.127.0`, but it is not the source anchor and likewise has no generic
  `aware secret put/revoke` command.
- Missing generic custom-secret provisioning is tracked by open AWARE issue #448. This task must not
  invent, document, or test a capability that does not exist.
- FloLess consumer line: `codex/xeorvt-reference-model@721d3bb2`; Task 5 owns source bytes, project
  generation/approval CAS, immutable artifact storage, and HTTP authorization.
- Existing bridge: `cli-connection-reader`, shared by `connection-reader` and
  `ifc-reference-reader`; its source entry, SEA build, stdout guard, artifact directory, progress
  channel, and sibling web-ifc WASM conventions are the compatibility baseline.

## 1. Scope and non-negotiable boundaries

### In scope

- Curated `model-reference-reader` agent with `preflight`, `probe`, and `read-model` commands.
- A local, bounded JSON-stdin provider protocol using one separately installed executable.
- RVT only in this slice. RFA can use the protocol later but is not advertised or accepted now.
- Deterministic GLB normalization and separate canonical entity, property, and relationship JSON
  artifacts.
- A content-addressed, concurrency-safe conversion/normalization cache under AWARE state.
- Authenticated ordered receipts, complete provenance, exact coverage, safe artifact publication,
  cancellation, stale-owner takeover, and crash-idempotent retry.
- SEA packaging and current-CLI agent validation.
- Regression proof that IFC command output is unchanged.

### Out of scope

- A cloud/evaluation adapter, URLs, uploads, or any implicit network access.
- AWARE generic secret provisioning. Provider installation/licensing remains out-of-band until #448
  lands or the standalone adapter needs no AWARE-managed credential.
- Shipping proprietary provider bytes.
- RFA, comparison, filtering, FloLess workflow/UI changes, product-master merges, releases, or pushes.
- Inferring Revit Category/Family/Type/Level/relationships from node names, geometry, material, order,
  or English labels. Only explicit provider metadata and validated references are authoritative.

## 2. Contract decisions

### D1 — Configuration has one explicit local trusted-computing boundary

The bridge reads the provider executable from `AWARE_MODEL_REFERENCE_PROVIDER`. The value must be an
absolute path to a regular executable file. Relative paths, PATH lookup, directories, reparse
points/symlinks, and shell commands are refused. The bridge spawns that exact path with
`shell:false`; no model path, token, or configuration value appears in argv.

The separately installed provider is explicitly part of the trusted computing base. AWARE 0.126.0
does not provide an AppContainer, network deny policy, or generic secret facility, so this task does
not claim a malicious same-user provider is contained. The bridge itself has no network client, URL
input, or implicit provider discovery. It launches the trusted provider with a documented allowlist
environment containing only the minimum OS variables needed to start plus protocol locale/timezone
settings. It excludes `AWARE_HOME`, artifact/progress paths, proxy variables, PATH, tokens, and common
credential variables, and gives the provider a new private working directory.

Production accepts `execution:"local"` and `destination:null` only. The committed bridge contains no
HTTP client and no URL input. Provider licensing and credentials are the provider executable's own
local concern. If the authorized provider requires a credential AWARE must provision, preflight
reports the dependency as unavailable; it does not claim issue #448 is solved.

The cache root is `<AWARE_HOME>\cache\model-reference-reader`, falling back to the same user-profile
`~/.aware` convention the AWARE CLI uses when `AWARE_HOME` is absent. Tests inject an isolated root.

### D2 — Provider protocol is closed, versioned, bounded, staged, and binary-safe

The bridge invokes the provider twice:

1. `describe --json-stdin` receives a closed request containing protocol version and limits. It must
   return one bounded JSON object:

   ```json
   {
     "protocolVersion": "1",
     "provider": "provider-id",
     "engine": "engine-id",
     "engineVersion": "x.y.z",
     "adapterBuildId": "opaque-build-id",
     "formats": ["rvt"],
     "execution": "local",
     "destination": null
   }
   ```

2. `convert --json-stdin` receives a closed request containing the absolute path of an immutable staged
   source copy, a new private output directory, expected source SHA-256, canonical conversion settings,
   and byte/count limits.
   It returns one bounded JSON receipt with the same provenance plus `documentKind:"revit-project"`,
   the source hash, and exactly two absolute paths: `geometry.glb` and `metadata.json`.

Before conversion the bridge opens the caller's regular `.rvt`, copies it into an exclusively created
private file while hashing, rehashes that staged file, makes it read-only, and then exposes only the
staged path to the provider. The original is hashed again after staging; disagreement with the expected
hash or staged hash is `reference-source-changed`. This removes the pathname reopen race from provider
execution. Reparse points and link-like source/output entries are refused at every boundary.

The GLB is never placed in stdin/stdout, decoded as UTF-8, or included in an error. Provider stdout,
stderr, request bytes, run time, output-file count, each output size, and total output are capped.
Every provider call goes through the new internal `aware __model-reader-host` helper supplied to the
bridge by the runtime as an exact executable path. On Windows the host starts the provider suspended,
assigns it to a dedicated kill-on-close Job Object, resumes it, and does not report completion until the
Job has zero active descendants; timeout kills that provider Job without killing the bridge. On Unix it
uses a dedicated process group with the same zero-descendant rule. This opt-in host is used only by the
model reader; existing sidecars such as `tekla.launch`, whose child intentionally survives the one-shot
response, retain their lifecycle. Timeout or cancellation terminates and awaits the provider tree.

The host is a long-lived child of the bridge using a closed multiplexed `model-reader-host/v1` protocol.
Every frame has a kind byte (`0x01` JCS control, `0x02` stdout bytes, `0x03` stderr bytes, `0x04`
stdin bytes), unsigned 64-bit big-endian request ID, fixed 32-byte run handle (all zeroes before a run
handle exists), unsigned 32-bit big-endian per-stream sequence, one flags byte whose low bit is `final`,
unsigned 32-bit big-endian payload length, then bounded payload. Control payloads are UTF-8 JCS;
stdin/stdout/stderr payloads are uninterpreted bytes. A `provider-run` control frame declares the exact
stdin byte length and returns its run handle before matching sequenced stdin frames are accepted; the
provider starts only after one matching final stdin frame and exact length reconciliation. The host
drains stdout and stderr concurrently while accepting control/cancel frames, validates monotonic stream
sequences, and correlates every out-of-order completion and binary frame by both request ID and run
handle. Operations are `hello`,
`lock-acquire`, `lock-release`, `provider-run`, `provider-cancel`, and `shutdown`; every request has a
monotonic request ID and every lease/run has an unguessable handle scoped to that host process. Acquired
locks remain held until matching release or host death. Acquire/run immediately acknowledge a handle;
the host processes frames concurrently and returns out-of-order completions by request ID so cancel and
shutdown remain live while another request waits. `provider-run` carries the exact executable,
minimal environment, cwd, stdin bytes, timeout and output caps; stdout/stderr return as separately
length-framed bytes only after zero descendants. EOF/parent death cancels runs, releases locks, and exits.
The runtime passes the canonical current `aware` executable path in a private environment variable and
the bridge verifies the host `hello` build/protocol before sending sensitive paths.
The bridge rejects extra files and reads outputs as bytes only from validated regular descendants of
its private output directory after final-path containment checks. These controls constrain mistakes by
the trusted provider; they are not a claim of hostile-code sandboxing.

The provider request and response are validated against committed closed JSON Schemas, including
`model-provider-v1.schema.json`; duplicate JSON keys, unknown properties, unsafe integers, and trailing
data are refused. Golden request/response vectors are also run against the Windows fixture executable.
The executable SHA-256 is measured before `describe`, after `describe`, before `convert`, and after
`convert`; all four must match. The staged source hash and provider receipt must equal the caller's
expected hash. Describe/convert provenance must agree exactly.

### D3 — Complete canonical request and provider fingerprint define identity

The canonical request is an RFC 8785 JSON Canonicalization Scheme (JCS) object with no omitted-default
ambiguity. Duplicate keys, non-finite values, unsafe integers, and non-canonical Unicode/string inputs
are refused before canonicalization. Committed golden vectors cover key ordering, escaping, Unicode,
numeric forms, negative zero, and collection permutations. It contains:

- schema/protocol/reader versions;
- format and document-kind policy;
- active-scene-only traversal policy;
- geometry modes and component types accepted;
- source and canonical frames, transform and winding policy;
- join and stable-identity policy;
- canonical JSON and GLB encoding policy;
- every input/output/JSON/scene/node/depth/mesh/primitive/accessor/buffer/vertex/index/entity/property/
  relationship/artifact/timeout limit;
- selection policy (unfiltered/full model in this task); and
- provider conversion settings that affect output.

`canonicalRequestSha256` hashes those exact JCS bytes. The provider fingerprint is the closed JCS object:

`(protocolVersion, provider, engine, engineVersion, adapterBuildId,
adapterExecutableSha256, readerSchemaVersion)`.

`expected-provider-sha256` is SHA-256 of that exact seven-field JCS object. Golden vectors pin its bytes;
The bridge constructs the tuple from five described fields, the host-measured executable hash, and its
embedded schema version, then compares the hash to the pin before conversion.

The cache key is SHA-256 of JCS bytes containing `sourceSha256`, the complete canonical request,
the complete fingerprint, and the expected signing-key fingerprint. Source, request/configuration,
signer trust anchor, reader schema, provider identity,
engine/version/build, or executable bytes therefore invalidate a hit. Every field appears in the
published manifest and is mutation-tested for preimage coverage, except signer identity: the signer pin
is intentionally cache/authentication metadata outside deterministic manifest/artifact preimages.

### D4 — GLB normalization is minimal, deterministic, and explicit

`revit-glb.mjs` parses GLB v2 directly from bytes and rejects duplicate JSON keys at every nesting level
before profile validation. It accepts exactly one JSON chunk and at most one
BIN chunk, verifies declared/file/chunk lengths with checked arithmetic, and rejects unknown required
extensions, external/file/HTTP buffer URIs, sparse accessors, Draco/meshopt compression,
morph targets, skins, animations, non-finite values, and unbounded structures.

An explicit integer `scene` is required; missing active-scene selection is refused rather than guessed.
Only that declared active scene is walked.
The walk detects cycles, excessive depth, duplicate child references, and a node reachable by more
than one parent; those are malformed/ambiguous rather than silently instanced. World matrices compose
parent then local transforms. A node may use either `matrix` or TRS, never both.

The accepted GLB profile has a URI-less `buffers[0]` bound to the BIN chunk and at most 15 additional
buffers using the exact canonical `data:application/octet-stream;base64,` form emitted by xeoRVT.
Encoded and decoded lengths, canonical Base64, cumulative decoded bytes, and every buffer reference are
validated before accessor reads; all external or other URI forms remain refused. POSITION is
`VEC3/FLOAT`, all counts and references are safe integers, and chunk, buffer-view, accessor, stride,
alignment, and index ranges are checked with overflow-safe arithmetic. Unused external resources are
still refused. Textures, images, samplers, UVs, tangents, morphs, skins, animations, cameras,
lights, sparse accessors, and compression are unsupported in v1. Every `extensions` object and both
`extensionsUsed`/`extensionsRequired` must be absent; no optional extension semantics are discarded.
NORMAL may be `VEC3/FLOAT`, must match POSITION count, and is transformed by inverse-transpose plus the
canonical frame rotation and unit normalization. Materials may contain a provider-specific string name
(discarded), bounded `pbrMetallicRoughness` base-color/metallic/roughness factors, `alphaMode` OPAQUE or
BLEND, Boolean `doubleSided`, and zero/absent emissive factor. Canonical output moves base color into
vertex color while retaining metallic, roughness, alpha mode, and double-sided presentation; masks,
textures, nonzero emissive values, and all other material fields remain refused. A missing material uses
glTF defaults.

Supported primitives are TRIANGLES, TRIANGLE_STRIP, and TRIANGLE_FAN, indexed or non-indexed, with
`UNSIGNED_BYTE`, `UNSIGNED_SHORT`, or `UNSIGNED_INT` indices. Strides and offsets are checked against
buffer-view and buffer bounds. Strip parity and fan expansion are explicit; degenerate triangles are
counted and dropped. The full world transform is applied first, then glTF Y-up metres are converted to
canonical Z-up millimetres by `(x,y,z) -> (1000*x,-1000*z,1000*y)`. Winding is reversed iff the
determinant of the combined world-plus-frame transform is negative. Matrices must be finite and
nonsingular. Quaternions must be finite and within the configured unit-length tolerance, then are
normalized deterministically. Canonical coordinates are rounded once to IEEE-754 float32, `-0` becomes
`+0`, overflow is refused, and triangles that become degenerate after transformation/rounding are
counted and dropped. Mutations of both determinant
branches, strip parity, and frame axes must fail tests.

Primitive color comes only from glTF `COLOR_0` and material `baseColorFactor`, with explicit alpha and
deterministic multiplication. `COLOR_0` accepts only VEC3/VEC4 FLOAT (not normalized) or normalized
UNSIGNED_BYTE/UNSIGNED_SHORT; RGB implies alpha 1. All other forms are refused. Canonical geometry is rebuilt as a GLB
whose JSON chunk uses JCS-derived ordering plus fixed glTF layout/padding, whose collections use named
stable sort keys with explicit tie-breakers, and whose buffer is packed little-
endian buffers, and no timestamps, random IDs, absolute paths, or generator-machine data.

Arrays are classified, not vaguely "sorted": provider parameter-group references, parameters inside a
group, and multipart appearance names preserve semantic provider order and retain duplicates only where
the schema explicitly permits duplicate parameter names. Set-like entities sort by decimal numeric ID;
geometry parts by `(entityId numeric,appearance ordinal,nodeName UTF-8)`; properties by `(entityId,
group ordinal,parameter ordinal,parameter id)`; relations by `(kind UTF-8,from numeric,to numeric,
relation id)`; coverage buckets by `(reason UTF-8,stableId UTF-8)`. Exact duplicate IDs/edges are refused.
JSON object keys use JCS. Permutation invariance is tested only for set-like arrays.
Transformed `(position,color,normal?)` records are first encoded to their final canonical bytes, sorted
lexicographically, deduplicated, and assigned canonical indices independent of provider order. Triangles
are then remapped to those indices, rotated to their lexicographically smallest oriented tuple, sorted,
and deduplicated only according to the explicit duplicate-triangle policy. Provider-assigned indices are
never a sort key, making provider vertex/triangle permutations byte-identical under adversarial cold-run
tests.

### D5 — Metadata is resolved, not inferred

`revit-metadata.mjs` accepts the committed closed `model-metadata-v1.schema.json`, not an open-ended
projection of one observed provider. Its root is
`{schemaVersion,document:{kind,id},types[],levels[],parameterGroups[],parameters[],elements[],relations[]}`.
Every record has a canonical decimal-string `id`; elements require `id`, explicit nullable `revitClass`,
`category`, `family`, type/level table references, ordered parameter-group references, ordered appearance
node names, and optional exact `ifcGuid`. Parameters require `id`, `name`, optional `unit`, readability
state, a closed `storageType` enum `none|boolean|integer|double|string|element-id`, and one tagged value.
The mapping is exact: none→null/unreadable, boolean→boolean, integer→decimal-string, double→finite-number,
string→string, and element-id→signed int64 decimal-string; mismatches are refused.
Relations require canonical `id`, a kind from `contains|hosts|depends-on|provider-explicit`, and element
endpoints; `contains` and `hosts` are acyclic and single-parent, while the others are directed
multigraphs. `provider-explicit` additionally requires a bounded non-empty `providerRelationKind` that
preserves the provider's exact type. `ifcGuid` is derived only from the authoritative exact `IfcGUID`
parameter record; a redundant provider field, if present, must match byte-for-byte. Unknown fields/kinds
and duplicate table IDs are refused. It resolves all array-index
references (`Type`, `Level`, `ParameterGroups`, `Parameters`) before ordering. An index
must be an in-range safe integer and must reference the expected record kind. Inner Revit `Id` values
are data, never table offsets.

Actual entity/table identities are canonical positive int64 decimal strings; JSON numbers outside the
safe integer range are refused and no ID is coerced through JavaScript `Number`. Parameter records with
storage type `ElementId` preserve signed int64 decimal strings, including documented negative special
values, and never use them as entity/table references. Entity stable identity is
the source Revit element `Id`, namespaced as `element:<Id>`. Durable
cross-revision identity is the exact `IfcGUID` parameter when present and unique; missing and duplicated
GUIDs are explicitly receipted as uncomparable. Category/Family/Type/Level and Revit class come only
from explicit referenced metadata/parameters. Values preserve group order, duplicate names, units,
null, empty, unreadable, numeric, Boolean, and string distinctions; no localized string is parsed into
meaning.

Geometry joins are explicit: metadata `appearances[]` supplies an ordered set of exact GLB node names
for an entity; it may be empty only for an explicitly indexed non-drawable entity. Build multimaps on
both sides. Multipart entities are valid, but each active-
scene geometry node must have exactly one entity owner and each claimed name must resolve exactly once.
Duplicate claims, duplicate node names, missing nodes, and unclaimed/watermark nodes are separate
coverage reasons. The reader never
falls back to element order, `<id>_<n>` parsing, node-name similarity, or geometry.

Hierarchy and relationship edges use namespaced entity IDs, validate both endpoints, are sorted
canonically, and reject cycles where the relationship kind is declared acyclic. Conflicting parents
are `relationship-parent-ambiguous`; they are not resolved by first-wins. Unknown relationship kinds
are retained only as explicitly typed provider relations when the schema allows them; otherwise they
are counted unsupported.

### D6 — Four deterministic artifacts plus one deterministic manifest

Normalization produces separate files:

- `geometry-0000.glb` — canonical binary geometry in Z-up millimetres;
- `entities-0000.json` — identity, classification, bounds, and geometry references;
- `properties-0000.json` — grouped source parameter records keyed by entity;
- `relationships-0000.json` — explicit hierarchy/relationship edges; and
- `manifest.json` — schema/version, source receipt, canonical frames/matrix, complete request hash,
  fingerprint, exact coverage, and the ordered receipts for the four component artifacts.

Files are independently bounded and decodable. JSON uses canonical UTF-8 bytes with no BOM or trailing
newline. GLB stays binary. Artifact names are stable logical names; run-owned AWARE artifact IDs may
be opaque copies, but the deterministic manifest and content hashes never include those runtime IDs.

Exact coverage reconciles discovered entities, indexed entities, drawable entities, geometry nodes,
properties, relationships, and every skipped/ambiguous/unsupported reason. It includes ordered counts
and digests of the exact stable-ID sets so equal totals cannot conceal different omissions. Publication
fails unless every normalized record belongs to exactly one coverage bucket and all component receipt
byte counts/hashes revalidate. The command response carries a fifth, external receipt for the manifest;
the manifest never contains its own hash or receipt.

### D7 — Receipts are authenticated without pretending generic secrets exist

Deterministic artifact bytes contain SHA-256 integrity receipts only. The cache and command response
add an authentication envelope over the manifest receipt plus its four ordered component receipts using the
existing AWARE Ed25519 key format at
`<AWARE_HOME>\keys\model-reference-reader.{sec,pub}`. The key is provisioned by the real supported
`aware key generate model-reference-reader` command, not by a fictional generic secret command.

`preflight` distinguishes:

- provider not configured/not found;
- provider available but receipt signing key absent;
- fully available with provider fingerprint and signing public-key fingerprint.

Every converting `probe`/`read-model` requires the signing key and an input `expected-signer-sha256`
obtained out of band; negative preflight performs no conversion. The five receipts are JCS objects in
fixed order `[geometry,entities,properties,relationships,manifest]`. The Ed25519 signature input is
SHA-256 of ASCII `AWARE\0model-reference-reader\0receipt-set\0v1\0` followed by, for each receipt, its
unsigned 64-bit big-endian byte length and exact JCS bytes. The JCS envelope is
`{schemaVersion:"1",algorithm:"Ed25519-SHA256",keyFingerprintSha256,publicKeyBase64,
preimageSha256,signatureBase64}` with canonical base64 and no padding ambiguity. On load, derive the
public key from `.sec`, require byte equality with `.pub`, verify the key fingerprint, then sign. Shared
Node/Rust/FloLess golden vectors pin preimage and signature bytes. Cache hits verify the public key
fingerprint, signature, manifest, every artifact
receipt, every artifact byte, and cache key before reuse. Tests use generated throwaway keys only.
The expected public-key fingerprint is part of request/cache identity. Key rotation is an explicit
configuration change and cache miss. The signature envelope is outside deterministic artifact
preimages, so installations can use different keys without changing normalized artifact bytes or
revision hashes. Authentication detects cache corruption and changes outside the provider TCB; it does
not protect against a malicious same-user provider that can access the signing key under AWARE 0.126.0.

This does not solve #448: the signing key command already exists in `v0.126.0`; arbitrary provider
credentials still cannot be generically provisioned.

### D8 — Cache ownership and publication are crash-idempotent

Each cache key has immutable content-addressed blobs, an immutable complete mapping, and an OS-backed
exclusive lock held for the whole critical section. The Rust `aware __model-reader-host` uses `fs2`
held advisory locks cross-platform and `windows-sys` process/Job/process-start APIs on Windows; the
bridge fails closed if the exact runtime-supplied host is absent. Locks are kernel-released on process
death. A contender records
`{cacheKey, ownerToken, pid, processStartIdentity, startedAt, heartbeatAt}`.
The random owner token is never logged. Only the current token may refresh, cancel, or publish.

Fresh owners make waiters poll with a bounded deadline and cancellable signal. Successful acquisition of
the kernel lock is the sole authoritative ownership fence; heartbeat and `(pid,processStartIdentity)` are
diagnostic/wait-hint fields only and can never override a held kernel lock. After acquisition, the owner
replaces any stale diagnostic record. A stale process checks ownership before every heartbeat/publication; loss
cancels its provider and forbids publication.

The winner writes only to a private token-named staging directory, fsyncs files and directory where
supported, and validates the complete result. It then holds a host lock for each digest, revalidates an
existing final blob or atomically renames a completed same-directory token-temp to an absent final name,
and releases the digest lock only after final-byte validation. A contender therefore never observes a
partial final blob. The complete JCS
mapping is also created with `wx` and published last; only that mapping makes an entry visible. A loser
deletes only its own staging data, validates the winner, and returns the winner.
Crash points after provider output, after normalization, after signing, before rename, after rename,
and before lock cleanup all retry to either reuse one valid entry or rebuild; no half-entry is visible.
Cleanup targets only resolved descendants of this cache root.

Cancellation is two-phase. The orchestrator signals a runtime-owned cancellation pipe/file watched by
the bridge; the bridge stops heartbeat, asks the host to terminate/await its provider Job, removes only
owner-token staging, releases its held lock, and emits `reference-cancelled` within a five-second grace.
If it does not exit, `CliInvoker` force-terminates only the opt-in reader sidecar tree and crash recovery
removes abandoned staging on retry; the trace records `forced-cancel` rather than claiming cooperative
cleanup. Provider and waiter cancellation are tested separately. Cancellation is a delivery gate.
Only a one-shot app graph containing the exact `model-reference-reader` agent opts into the new control
pidfile. That pidfile contains exact process-start identity plus the cancellation endpoint, installs
Ctrl+C handling, and lets `aware app stop` signal that endpoint. Both wait five seconds for trace-
complete cooperative exit before exact-process force termination; every other one-shot graph keeps its
existing concurrent-run behavior, and the existing long-running path retains its behavior. For an
opted-in reader graph, pidfile creation is exclusive per app/instance; a second concurrent run is
atomically refused, and token/run-ID-checked cleanup cannot remove another run's control file.

The cache appends bounded access observations under a global maintenance lock. Each record's total order
is its monotonic locked journal byte offset; a truncated tail is discarded and compaction is atomic.
Lost observations can only make inactive data appear older, never make active/reachable data evictable.
Deterministic LRU by latest journal sequence then cache-key tie-break evicts under that lock: 20 GiB,
1,000 complete mappings, 6,000 blobs, 128 quarantine entries, and 128 staging entries by
default, all lowerable but not raisable past hard ceilings without a reader-version change. Active
owners and blobs reachable from visible mappings are never evicted. Startup and post-publication sweep
orphan staging/quarantine/blob data within the same bounded quotas.

### D9 — AWARE command/output contract

The new agent shares `transport.cli.binary: aware-connection-reader`. At the binary boundary:

- every valid documented existing `probe`/`read-model` IFC request retains byte-compatible dispatch and
  output, and web-ifc remains lazily loaded only for IFC;
- new `preflight` is RVT-only and requires the out-of-band `expected-provider-sha256` pin;
- new converting `probe`/`read-model` require `model-path`, `expected-source-sha256`, the complete
  `expected-provider-sha256` fingerprint pin, `expected-signer-sha256`, canonical settings, and a closed
  `limits` object containing every lowerable default; mixed `ifc-path`/`model-path` inputs are refused;
- direct bridge and SEA calls use one JSON object on stdin and one JSON object on stdout;
- errors use a stable redacted envelope on stderr and non-zero exit; stdout remains empty.

`preflight` never converts or reads model bytes. `probe` performs/reuses the conversion and returns a
bounded summary plus the same source/fingerprint/frame/coverage receipt. `read-model` copies the five
verified deterministic files into `AWARE_ARTIFACT_DIR` using opaque safe IDs and returns an AWARE-owned
descriptor containing one standard `$aware-artifact` manifest descriptor plus a typed array of the
four component artifact descriptors, their receipts, and the authentication envelope. No absolute
path appears in output. `read-model` refuses with a stable error when `AWARE_ARTIFACT_DIR` is absent;
raw direct execution remains available only for non-publishing preflight/test harnesses.

Provider and signer pins are supplied by the caller out of band, hashed into request/cache identity,
compared before conversion/signing, and rotated only by changing those explicit inputs. The bridge's
mandatory `bridge-info` handshake returns `model-reference-reader/v1` plus immutable build ID before any
RVT command; the released IFC-only binary fails that capability fence even if its CLI version matches.

Run publication writes all five files to invocation-token temporary names inside the artifact directory,
rehashes all five, then commits their opaque final safe IDs. Any failure removes only that invocation's
temporary and already-committed final IDs, so a response never references a partial artifact set.

The AWARE runtime changes in this slice preserve the bridge's typed error envelope in `AwareError`, add
an optional structured field to `RunEvent::NodeError`, propagate it through the orchestrator while
retaining the legacy string, forward bounded progress, make the managed-sidecar capability fence agent-
aware instead of Tekla-bake-only, and implement opt-in one-shot process-
tree cancellation. The agent is exercised through a temporary compiled `.flo` app and artifact
retrieval because `aware agent invoke` supports built-ins only in 0.126.0. The manifest declares only
schema-supported filesystem/network/software/secret/skill requirements; runtime-owned artifact,
progress, cancellation, environment, and process channels are documented but not misrepresented as
manifest-enforced permissions.

The manifest will include at least one plain-English skill explaining the provider and artifact
contract, as required for curated commands. The managed sidecar catalogue description broadens from
IFC-only language without adding a second binary. Agent inventory/registry statistics rise from 78 to
79 and are regenerated by `aware agent publish`, repository index/stat tools, and checks, never
hand-forced around a failing guard.

### D10 — Limits are concrete contract fields

Defaults and hard ceilings are committed constants and canonical-request fields: provider request/
stdout 256 KiB/1 MiB, stderr 64 KiB/256 KiB, conversion 10/30 minutes, staged RVT 150 MiB/4 GiB (staged
by streaming copy, never resident), input GLB 128/512 MiB, metadata 16/64 MiB, aggregate provider output
144/576 MiB, GLB JSON 16/16 MiB with nesting 64/128, scenes 8/32, active nodes 100,000/250,000 at depth
128/256, meshes 100,000/250,000, primitives 200,000/500,000, accessors and buffer views
250,000/1,000,000, vertices 5,000,000/10,000,000, indices 15,000,000/30,000,000, entities
250,000/1,000,000, parameters 2,000,000/5,000,000, relationships 1,000,000/2,000,000, component JSON
32/128 MiB, canonical GLB 256/512 MiB, command response 1 MiB, and each progress frame 8 KiB. The v1
implementation is deliberately in-memory and has a measured 1 GiB resident hard gate; admission uses
checked worst-case allocation estimates before parsing. Logical limit tests inject tiny ceilings and
exercise exact/one-over cases cheaply; a named Windows stress lane covers real maximum GLB/metadata/
resident ceilings, while the 4 GiB staged-copy limit uses sparse/streaming tests. Multiplication and
addition are checked before allocation.

## 3. Threat model and refusal matrix

| Threat / failure | Admission or detection | Stable outcome |
| --- | --- | --- |
| Relative/provider shell/path search | Absolute regular executable, no shell/PATH | `reference-provider-unsafe` |
| Source traversal/symlink/change | `.rvt`, regular file, private staged copy and hash agreement | `reference-source-unsafe` / `reference-source-changed` |
| Untrusted/malicious provider | Explicitly outside this task's guarantee; provider is installed TCB | preflight refuses unapproved fingerprint; no sandbox claim |
| Provider declares remote behavior | Local-only describe receipt; no URL input/client in bridge | `reference-provider-nonlocal` |
| Secret/path/model leak | Redacted envelope and structured safe fields only | `reference-provider-failed` with diagnostic ID |
| Chatty/hung provider | bounded pipes + deadline + process-tree termination | `reference-provider-output-too-large` / `reference-provider-timeout` |
| Output escape/reparse/extra file | private dir containment, regular files, exact file set | `reference-output-unsafe` |
| Malformed/truncated/oversized GLB | checked GLB/chunk/accessor arithmetic and caps | `reference-geometry-invalid` / `reference-output-too-large` |
| External/implicit resource read | no GLB URI and no unsupported extension | `reference-external-resource-refused` |
| Scene cycle/multiple parents | active-scene graph validation | `reference-scene-invalid` |
| Unsupported geometry | explicit reason; no silent reinterpretation | `reference-geometry-unsupported` |
| Ambiguous metadata join | exact bidirectional multimap | `reference-metadata-join-ambiguous` or covered skip |
| Indexed metadata drift | in-range typed resolution before canonical IDs | `reference-metadata-invalid` |
| Relationship cycle/conflicting parent | endpoint/type/acyclic checks | `reference-relationship-invalid` |
| Source/provider/request drift | complete cache-key preimage + bracketing hashes | cache miss/refusal, never stale hit |
| Tampered cache | signature + full manifest/artifact revalidation | quarantine/rebuild; no hit |
| Concurrent/crashed writer | held OS fence, exact process identity, content CAS, mapping-last publish | wait/takeover/validated winner |
| Cancellation/orphan descendants | runtime cancel propagation + Job/process group + token cleanup | `reference-cancelled` |
| Artifact flooding | per-file/aggregate/count/read/chunk limits | `reference-output-too-large` |
| Cache disk exhaustion | quotas, mapping-aware deterministic eviction, orphan sweep | bounded eviction or `reference-cache-full` |

Errors include only `{code, phase, retryable, message, diagnosticId}`. Diagnostic IDs are random and
not cache/revision inputs. Logs may include phase, elapsed time, cache result, fingerprint fields,
digest prefixes, counts, and diagnostic ID; they may not include raw provider output, full source or
provider paths, filenames, parameter values, credentials, URLs, owner tokens, or artifact content.

## 4. Test-first implementation slices

### Slice A — Generated fixtures, canonical primitives, and contract tests

**Create:**

- `cli-connection-reader/model-fixtures.mjs`
- `cli-connection-reader/model-fixtures.test.mjs`
- `cli-connection-reader/model-contract.mjs`
- `cli-connection-reader/model-contract.test.mjs`
- `cli-connection-reader/model-provider-v1.schema.json`
- `cli-connection-reader/model-metadata-v1.schema.json`

Generate tiny GLB bytes in memory from declarative scene/primitive inputs and tiny provider-metadata
objects from source text. Do not commit generated `.glb` or model bytes. Tests first pin RFC 8785 JCS,
schema conformance/golden vectors, 64-bit decimal-string IDs, every exact/one-over limit,
all request/fingerprint fields in the cache preimage, stable error envelopes, and
fixture determinism. Mutation loop changes every leaf of request/fingerprint and requires a different
hash.

**Commit:** `test: define the RVT reader's deterministic boundary`.

### Slice B — GLB parser and geometry normalization

**Create:** `revit-glb.mjs`, `revit-glb.test.mjs`.

Write failing tests for GLB headers/chunks, external URIs and strict embedded Base64 buffers,
accessors/stride/offset/index types, indexed
and non-indexed triangles/strips/fans, degenerates, active versus inactive and missing scenes, nested
matrix/TRS world transforms, duplicate parents/cycles/depth, positive/negative determinant winding,
Y-up metres to Z-up millimetres, vertex/material colors, material presentation, inverse-transpose
normals, multiple primitives, unsupported extensions/
geometry, missing/extra BIN bindings, singular transforms, quaternion tolerance, float32 overflow,
negative-zero, post-transform degeneracy, stable sort ties/permutations, malformed ranges/checked
overflow, non-finite values, and every limit. Implement only the minimum parser
that makes those cases pass. Add mutation controls that deliberately remove the frame transform,
winding reversal, active-scene filter, and range bound and prove the tests fail.

**Commit:** `feat: normalize bounded Revit GLB geometry`.

### Slice C — Explicit Revit metadata and join normalization

**Create:** `revit-metadata.mjs`, `revit-metadata.test.mjs`.

Write failing tests for indexed Type/Level/ParameterGroup/Parameter resolution; source `Id` and exact
`IfcGUID`; Category/Family/Type/Level/class; group order, duplicates, units and value states; unique,
missing, duplicate, unclaimed, and watermark appearance joins; stable namespaced IDs; hierarchy cycles;
multipart entities; ambiguous parents; missing endpoints; unsupported relationship kinds; exact coverage; deterministic
entity/property/relationship bytes. Mutation controls break one index, duplicate one join, remove one
GUID, and swap one relationship endpoint and must fail reconciliation.

**Commit:** `feat: preserve explicit Revit metadata and relationships`.

### Slice D — Provider process, provenance, safety, and redaction

**Create:**

- `model-provider.mjs`
- `model-provider.test.mjs`
- `test-fixtures/model-provider-fixture.mjs`

The fixture adapter has both a deterministic script form and a built Windows executable used through
the real child-process protocol; it writes generated GLB/metadata to the requested private directory.
Tests first cover minimal environment/private cwd, schema/golden-vector conformance, immutable staged
source handling, descendant process containment, describe/convert agreement, executable and source
changes at every bracket, timeout, cancellation,
bounded stdin/stdout/stderr, non-zero exit, malformed/duplicate/missing receipt fields, wrong document
kind, wrong source hash, remote execution, unsafe/external/reparse paths, extra files, oversized files,
and error/log redaction. On Windows, add explicit junction/reparse and process-tree cases; on platforms
where a primitive cannot be created, report the unverified branch rather than skip silently.

**Commit:** `feat: fence the local model provider protocol`.

### Slice E — AWARE one-shot lifecycle, structured errors, and telemetry

**Modify:**

- `cli/src/runtime/invoker.rs`
- `cli/src/error.rs`
- `cli/src/runtime/provenance.rs`
- `cli/src/runtime/orchestrator.rs`
- `cli/src/runtime/lifecycle.rs`
- `cli/src/runtime/pidfile.rs`
- `cli/src/commands/app.rs`
- `cli/src/commands/model_reader_host.rs`
- `cli/src/main.rs`
- `cli/src/commands/mod.rs`
- `cli/Cargo.toml`
- `cli/Cargo.lock`
- `cli/src/commands/sidecar.rs`
- focused Rust integration tests and platform helpers

Write failing tests for the internal provider supervisor/held-lock protocol, exact process-start
identity, dedicated child/grandchild Job/process-group exit, cooperative and forced cancellation, and
kernel lock release on crash. Prove multiplexed stdin/stdout/stderr correlation under two concurrent
out-of-order provider runs, and prove the exclusive control pidfile applies only to app graphs containing
the exact model-reader agent while unrelated one-shot apps remain concurrent. Preserve a bridge's bounded
typed error envelope instead of collapsing it to a generic message, forward bounded success progress,
and require the protocol/build capability only for the exact `model-reference-reader` agent ID, avoiding
an invented manifest field in 0.126.0. Existing non-managed CLI
agents and intentional survivor lifecycles including Tekla launch/watch remain compatible.

**Commit:** `fix: fence managed sidecar lifecycles`.

### Slice F — Authenticated content-addressed cache and crash recovery

**Create:** `model-cache.mjs`, `model-cache.test.mjs`.

Tests first cover key derivation, generated AWARE-format Ed25519 keys, receipt sign/verify, complete hit
validation, tampered/missing/extra files, source/request/fingerprint/provider-binary/signer invalidation,
concurrent same-key winner, cancellable waiter, fresh owner, dead owner, stale live owner, reused PID
with a different process start, unverifiable owner, fenced takeover, old-token heartbeat/publish
refusal, each publication crash point, blob/mapping `wx` destination race, winner
validation, private permissions, token-scoped cleanup, and restart idempotency. Run two cold conversions
with the same source/request/provider but empty entries and assert identical hashes and bytes for all
five deterministic artifacts.

Add quota/eviction tests for each entry/blob/byte/staging/quarantine ceiling, deterministic LRU ties,
reachable-blob protection, active-owner protection, orphan sweep, and `reference-cache-full`.

**Commit:** `feat: publish crash-safe authenticated model conversions`.

### Slice G — Multi-artifact reader commands and IFC compatibility

**Modify:**

- `cli-connection-reader/index.mjs`
- `cli-connection-reader/model-dispatcher.mjs`
- `cli-connection-reader/package.json`

**Create:** `cli-connection-reader/model-reader.test.mjs`.

Tests first drive `preflight`, `probe`, and `read-model` through the actual CLI process with JSON stdin.
Cover negative preflight without conversion, key/provider readiness, mixed/missing paths, cache miss/hit,
bounded summary, five artifacts, exact receipt/coverage, binary GLB equality, stdout purity, redacted
errors, expected signer trust/rotation, absent artifact directory, progress phases, cancellation, and
deterministic cold runs. Enumerate every valid documented IFC request shape, snapshot command bytes
before wiring, and require byte-identical stdout afterward plus lazy web-ifc loading.

Leave `index.mjs` as the IFC-compatible module with all synchronous named exports intact. Make
`model-dispatcher.mjs` the package bin and SEA entrypoint; it dynamically imports `index.mjs` only for
IFC and routes RVT without loading web-ifc. Existing IFC imports/exports and behavior remain in place.

**Commit:** `feat: expose deterministic RVT reference artifacts`.

### Slice H — Agent, sidecar catalogue, registry, and documentation

**Create:**

- `20-agents/aeco/engineering/model-reference-reader/manifest.yaml`
- `20-agents/aeco/engineering/model-reference-reader/commands/preflight.md`
- `20-agents/aeco/engineering/model-reference-reader/commands/probe.md`
- `20-agents/aeco/engineering/model-reference-reader/commands/read-model.md`
- `20-agents/aeco/engineering/model-reference-reader/skills/provider-and-artifact-contract.md`

**Modify:**

- `cli/src/commands/sidecar.rs` (description/docs only; same bridge ID and asset)
- `cli/tests/agent_list.rs` (79 with explicit new-agent assertion/history)
- generated `registry-index.json` and synchronized stats/docs reported by the official tools

Manifest docs explicitly state local-only execution, no committed provider, #448's secret-provisioning
limit, expensive first probe, canonical frame, exact joins, separate artifacts, and the distinction
between missing provider/key versus conversion failure. Then run:

```powershell
aware agent publish 20-agents/aeco/engineering/model-reference-reader
python scripts/sync_stats.py --write
aware agent reindex
aware agent reindex --check
```

Use a temporary `AWARE_HOME` for install/compile tests. Do not hand-edit generated counts to bypass a
tool failure.

**Commit:** `feat: publish the local RVT model reader agent`.

### Slice I — SEA, Windows harness, and no-fallback packaging

**Modify:** `cli-connection-reader/build.mjs`, `.github/workflows/ci.yml`, package metadata/lock only if
dependencies change. Schemas are imported as static data and embedded by esbuild; runtime filesystem
schema reads are forbidden. Rust host dependencies/features are built into `aware`, not adjacent assets.

Add a packaged-executable test/harness that copies only `aware-connection-reader.exe` and
`web-ifc-node.wasm` to a fresh clean staging directory, places the fixture provider and source in a
separate authorized input directory, changes cwd away from the repository, hides/renames no user data,
and drives all three RVT commands. Assert source modules and adjacent repo files are absent/unreadable,
GLB remains binary, component hashes match source-mode output, and existing IFC fixture read still works.
The provider itself remains separate by design and is supplied via its absolute configuration path.
Run that harness in the Windows CI lane so SEA/provider/process-tree behavior is not a local-only claim.

**Commit:** `build: keep RVT normalization inside the shared SEA`.

## 5. Verification ledger

### Focused and compatibility tests

```powershell
npm test -- --test-name-pattern "model|Revit|RVT|GLB|cache|provider|receipt"
node --test model-contract.test.mjs model-fixtures.test.mjs revit-glb.test.mjs revit-metadata.test.mjs model-provider.test.mjs model-cache.test.mjs model-reader.test.mjs
node --test extract.test.mjs probe.test.mjs read-model.test.mjs recognize.test.mjs compare.test.mjs
npm test
npm run build
```

Baseline note: `npm test` currently has one deliberate failure in `compare.cli.test.mjs` because no
external sample IFC is installed; 115 pass and 56 skip. The final report must distinguish this
environmental corpus gate from new failures. In-repo IFC fixtures and all new RVT fixtures must pass.

### Repository gates

```powershell
python scripts/sync_stats.py --write
aware agent reindex --check
cargo fmt --manifest-path cli/Cargo.toml --all -- --check
cargo clippy --manifest-path cli/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path cli/Cargo.toml
```

Run the current source-built CLI where installed `aware` behavior could differ. Record exact CLI
version and manifest/command acceptance output.

### Determinism and mutation proof

- Delete only two isolated test cache entries, convert the same generated source twice, and record
  identical SHA-256 for geometry, entities, properties, relationships, and manifest.
- Change source bytes, each provider fingerprint field, canonical request/configuration, and adapter
  executable bytes independently; each must miss or refuse.
- Mutate critical test seams (frame matrix, winding branch, join uniqueness, one artifact byte, receipt
  signature, owner token, heartbeat freshness, final source hash) and record that the targeted test goes
  red before restoring the implementation.

### Authorized live drill

Only if a licensed local provider executable and Residential input are present and explicitly
authorized:

1. Use a temporary AWARE home and generate the supported receipt key.
2. Build/install the shared SEA and install/reindex the agent locally.
3. Publish/reindex the curated agent, compile a temporary `.flo` app that invokes each command, and run
   `preflight`, `probe`, and `read-model` through `aware app run`; do not use unsupported direct custom-
   agent invocation.
4. Retrieve every artifact through `aware app artifact`, not by treating IDs as paths.
5. Record source hash, canonical request hash, full provider fingerprint, public-key fingerprint,
   artifact hashes, exact coverage, finite canonical bounds, trace/diagnostic IDs, and representative
   explicit Category/Family/Type/Level/parameter/relationship records.
6. Run two cold conversions and compare bytes/hashes.
7. Exercise CLI cancellation and require the one-shot runtime to terminate the complete provider tree.

If the standalone provider, license, generic credential provisioning, or authorization is unavailable,
do not use the old cloud credential or committed URL. Mark the live branch blocked precisely and leave
the FloLess create card gated.

## 6. Definition of done

- Every required happy, malformed, unsafe, oversized, concurrency, crash, cancellation, and
  determinism branch has a test that can be shown to fail under mutation.
- The provider/request/fingerprint preimages are complete and closed; every hit revalidates full
  signed receipts and bytes.
- Geometry is active-scene-only, fully transformed, canonical Z-up millimetres, correctly wound and
  colored; GLB bytes never pass through text.
- Entities, properties, relationships, and geometry are separate, bounded, deterministic artifacts
  with exact reconciled coverage and no inferred Revit meaning.
- SEA runs from a clean staging directory without source-tree or adjacent-source fallback.
- Existing IFC output remains byte-compatible and its in-repo tests pass.
- The new manifest and commands are accepted by the source/current CLI; agent registry and count guards
  are honest.
- No Residential bytes, converted commercial artifacts, provider URL/binary, credential, token, secret,
  absolute sensitive path, or scratch file is committed.
- AWARE commits are coherent, conventional, and have no `Co-Authored-By`; no push/release/merge occurs.
- The final handoff records commits, commands, hashes, fingerprints, trace IDs, known baseline skips,
  verified defects/issues, and every unverified live branch.

## 7. Required Xeorvt integration handoff

After the AWARE branch is committed and all attainable AWARE verification is recorded, invoke the
user-requested `xeorvt-integrate` skill against the FloLess repository. Task 6's implementation commit
belongs to `aware-aeco/aware` and cannot be merged across repositories; the integration workflow must
therefore first establish whether Task 6 produced any coherent FloLess source commit to merge. It must
not manufacture one or stage the three preserved untracked handoffs.

The skill then fetches FloLess `origin`, records all worktrees/status/SHAs, safely advances a clean
local `master`, merges the current master into `codex/xeorvt-reference-model` when required, and runs
its full post-merge acceptance gate on the exact final Xeorvt HEAD. It may create local commits and
merge commits only. It does not authorize a push, PR, release, worktree deletion, force operation, or
merge of Xeorvt into product `master`. If current FloLess master or the integration worktree is dirty,
divergent, or otherwise unsafe, stop and report the precise blocker rather than widening scope.

## 8. Issue #464 Windows Node SEA host regression addendum

The original installed-app run ended as `reference-provider-failed`; a later direct
`runModelCommand(..., { environment: { ...process.env } })` diagnostic produced the captured
`ncrypto::CSPRNG(nullptr, 0)` exit 134. Those observations are related evidence, not proof of one root
cause. On the current Node 24.14 provider, the source-built host completed 20 live-environment
`describe` launches and 20 live-environment `describe`/invalid-`convert` sequences without an abort.
Neither disabling `CREATE_SUSPENDED` nor removing Job Object assignment repaired the deterministic
diagnostic. This change therefore makes no process-lifecycle claim.

The deterministic defect is the environment normalization seam. The Rust host intentionally calls
`env_clear()`, and a Node SEA aborts before JavaScript when `SystemRoot` is absent. The bridge's Windows
allowlist currently performs uppercase property lookup. A plain copy of `process.env` preserves this
machine's `SystemRoot`, `windir`, and `ComSpec` spellings, so those values are silently dropped. The
fix remains at the JavaScript trust boundary and does not broaden the allowlist or inject ambient Rust
state:

1. Snapshot the environment only inside `minimalProviderEnvironment()`, at the provider-bound trust
   boundary. Do not replace the live environment used to resolve or launch the AWARE host: Windows'
   case-insensitive `process.env` semantics for `AWARE_HOME`, provider/key/artifact paths, and
   `AWARE_MODEL_READER_HOST` remain unchanged. The provider view is always an ordinary immutable input,
   so source and packaged commands exercise the same casing seam.
2. On Windows only, build an ASCII-case-insensitive index, then emit the existing canonical keys
   `SYSTEMROOT`, `WINDIR`, `COMSPEC`, `TEMP`, and `TMP`. If differently cased aliases for one canonical
   key contain different non-empty values, fail closed with
   `reference-provider-environment-ambiguous`; identical aliases collapse. Empty/non-string values are
   ignored. Folding accepts only ASCII letters; non-ASCII near-aliases remain forbidden. Non-Windows
   lookup stays case-sensitive and unchanged. `LANG=C`, `LC_ALL=C`, and `TZ=UTC` remain deterministic
   additions.
3. Add unit cases covering mixed-case forms of all five Windows keys, identical and conflicting aliases,
   mixed-case forbidden path/proxy/token/AWARE keys, non-ASCII near-aliases, and unchanged POSIX case
   sensitivity. Mutation back to case-sensitive lookup must fail the mixed-case regression.
4. Give the Windows packaged process a controlled environment that first removes every case-insensitive
   alias of the five allowed names, then inserts all five under deliberately mixed-case spellings with
   fixed values (`C:\\Windows`, `C:\\Windows\\System32\\cmd.exe`, and `C:\\Windows\\Temp`) plus a
   mixed-case forbidden sentinel. Make the fixture provider fail before `describe` unless its environment
   equals the fixed eight-entry canonical map exactly: the five known allowed values plus `LANG=C`,
   `LC_ALL=C`, and `TZ=UTC`. This is a behavioral assertion on the child after Rust `env_clear()`, not a
   circular check derived from received variables.
5. Run the existing packaged Windows harness on locally asserted Node 24.14.x and pin both its Windows
   CI job and the release workflow's connection-reader SEA build to Node 24.14.0, so the tested bridge
   runtime is the shipped bridge runtime and matches the diagnosed provider runtime. It must use
   the source-built `aware` host and newly built fixture SEA, complete `preflight` (`describe`), `probe`
   (`describe` then `convert` through one owned host), `read-model`, and `read-snapshot` with exit 0 and
   valid JSON/artifact hashes, and print the AWARE and Node versions in its non-skipped Windows PASS line.
   Add an automated legacy-lookup child control using the same controlled input; it must exit 134 while
   the fixed packaged path exits 0. The gate fails if the red control unexpectedly starts.
6. Run focused provider/reader tests, the complete connection-reader suite and build, the non-skipped
   Windows packaged harness, then `cargo fmt`, all-target clippy with warnings denied, and the full CLI
   test suite. Do not label #464 QA-ready unless those exact gates pass.

Acceptance is deliberately narrower than the issue's initial lifecycle hypothesis: the reproducible
missing-`SystemRoot` crash is eliminated in source and packaged paths; conflicting aliases fail closed;
the child receives only the canonical closed environment; all four packaged model commands return valid
outputs; and no Job Object, suspended-start, provider retry, or ambient Rust-environment behavior changes.
