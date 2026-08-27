# AWARE RVT reader — adversarial plan review log

**Plan:** `docs/superpowers/specs/2026-08-23-aware-rvt-reader-plan.md`

**Review model:** Codex CLI, read-only sandbox, maximum five rounds.

**Status:** Final bounded round complete. Round 5 returned `REVISE`; all three concrete findings were
accepted into the plan. The user explicitly authorized implementation after resolving them.

## Round 1

### Codex critique

Task 6 is not safe to implement as written. Material defects:

1. **The provider boundary is not enforced.** A self-reported `execution:"local"` cannot prevent the
   executable from accessing the network, arbitrary files, the cache, or the signing key; “may write
   only inside its output directory” is therefore false. Fix — Run the provider under an OS-enforced
   restricted identity/AppContainer with deny-network and directory ACLs, or explicitly place it in
   the trusted computing base and remove the sandbox claims.
2. **The provider inherits the complete AWARE environment.** Node spawning inherits environment
   variables by default, potentially exposing credentials, `AWARE_HOME`, artifact paths, and progress
   paths. Fix — Spawn with a minimal allowlisted environment and private working directory, passing
   only explicitly approved provider variables.
3. **Receipt signing does not protect against a malicious provider.** The same-user provider can read
   or replace `<AWARE_HOME>\keys\model-reference-reader.sec` and forge receipts. Fix — Isolate signing
   into a process/OS key store inaccessible to the provider, and document exactly which attacker the
   signature is intended to resist.
4. **Source hashing has a TOCTOU hole.** Pre/post hashing the pathname does not stop the source from
   being swapped for conversion and restored before the final hash. Fix — Copy bytes once into a
   private immutable staging file, hash that copy, and give only that staged path to the provider.
5. **Provider descendants can survive successful conversion.** Process-tree termination is specified
   only for timeout/cancellation, so a provider can exit after spawning a child that races output
   validation or mutates files later. Fix — Put every provider invocation in a kill-on-close Job
   Object/process group and ensure the entire group is quiescent before opening outputs.
6. **The metadata protocol is not actually defined.** “A closed bounded projection of the observed
   provider metadata” supplies no normative schema, required fields, types, reference tables,
   relationship enum, or unknown-field policy. Fix — Add a versioned JSON Schema plus canonical
   examples, duplicate-key rejection, exact enums, bounds, and cross-provider conformance vectors.
7. **Revit element IDs can lose identity in JavaScript.** Modern Revit element IDs are 64-bit, but the
   plan does not require decimal strings and could collapse distinct IDs above
   `Number.MAX_SAFE_INTEGER`. Fix — Require element IDs as canonical decimal strings and reject numeric
   IDs outside the safe-integer range.
8. **The geometry join rejects ordinary multipart elements.** Requiring exactly one node per entity
   will classify valid elements exported as several meshes/nodes as unusable. Fix — Permit an entity
   to claim an ordered nonempty set of nodes while requiring every claimed node to have exactly one
   entity owner.
9. **The active-scene fallback is wrong.** glTF does not specify scene 0 when `scene` is absent; clients
   may defer rendering until a scene is selected. Fix — Require an explicit valid `scene` index, or
   make the provider protocol explicitly select a scene and include that policy in the cache key.
10. **Critical glTF validation rules are missing.** The plan does not pin `POSITION` to `VEC3/FLOAT`,
    validate index values against vertex count, reconcile attribute counts, bind the BIN chunk to the
    sole URI-less buffer, or reject ignored textures/material extensions. Fix — Add a complete
    accepted-glTF profile and adversarial tests for every reference, accessor, alignment, count,
    material, texture, and extension rule.
11. **Singular and overflowed transforms are unspecified.** A zero determinant, non-unit quaternion,
    float overflow after millimetre scaling, and triangles made degenerate by transforms have no
    stable outcome. Fix — Define validation and coverage behavior for singular matrices, quaternion
    normalization, `-0`, float32 rounding, overflow, and post-transform degeneracy.
12. **Determinism is tested only with a deterministic provider.** “Stable order” lacks a sorting key,
    Unicode rule, numeric canonicalization, and shuffled-input equivalence tests. Fix — Specify
    bytewise ordering and canonical number/string rules, then permutation-test semantically identical
    GLB and metadata inputs across cold runs.
13. **Canonical JSON is underspecified for the FloLess consumer.** Recursive key sorting alone does
    not define Unicode ordering, escaping, negative zero, exponent form, or cross-language
    reproducibility. Fix — Adopt a named canonicalization standard such as RFC 8785/JCS and share
    golden byte/signature vectors with the Task 5 implementation.
14. **The manifest receipt can become self-referential.** The plan says the manifest contains ordered
    artifact receipts while also requiring every artifact, apparently including the manifest, to have
    a receipt. Fix — State that the manifest contains receipts for the four components only, while the
    external response hashes the manifest and the signature covers those exact five receipts.
15. **The signature has no consumer trust anchor.** An embedded public key or fingerprint proves only
    that some key signed the response unless FloLess already knows which key to trust. Fix — Require
    Task 5 to pin an out-of-band public-key fingerprint and define rotation/revocation behavior.
16. **The cache lease is not a real fencing primitive.** A heartbeat rewrite can race stale-lock rename,
    and ownership can be lost between the final token check and publication. Fix — Use an OS-backed
    exclusive lock keyed by PID plus process-start identity, allow takeover only after proving that
    exact process dead, and hold the fence through publication.
17. **“Atomic no-clobber rename” is not portable Node behavior.** `rename` may replace an existing
    destination, while Node exposes no cross-platform `renameat2(RENAME_NOREPLACE)` equivalent. Fix —
    Name the concrete no-replace primitive/helper and add a last-instruction publication race test on
    Windows, the shipped platform.
18. **The cache is an unbounded persistent disk-DoS.** Per-entry limits do not limit the number of
    sources, requests, provider builds, quarantines, or abandoned staging directories. Fix — Add
    total-byte/entry quotas, deterministic eviction, stale-stage cleanup, quarantine retention, and
    disk-full tests.
19. **Cancellation is simultaneously required and knowingly unimplemented.** The DoD requires
    cancellation, but the plan permits shipping after merely filing an issue if AWARE does not
    propagate it; current one-shot invocation simply awaits the child. Fix — Either implement
    kill-on-drop/process-tree cancellation in `CliInvoker` or remove runtime cancellation from Task
    6's claimed scope and DoD.
20. **Structured errors will not remain structured through AWARE.** `CliInvoker` treats stderr as text
    and wraps it in a network error. Fix — Teach the runtime to recognize and preserve the exact JSON
    error envelope, or document that stable codes exist only at the raw bridge boundary.
21. **Successful operational logs are discarded.** The runtime captures successful stderr without
    publishing it, so proposed cache/timing/takeover diagnostics will not reach the trace. Fix — Emit
    bounded cache state, phase timings, provider exit status, quarantine/takeover reason, and limit
    failures through `AWARE_PROGRESS_FILE` and the response telemetry envelope.
22. **Shared-sidecar versioning is unsafe for the new agent.** A stale `aware-connection-reader` is only
    warned and executed; the hard-current gate applies solely to Tekla bake. Fix — Make the gate
    agent-aware and require a current/protocol-compatible bridge for every `model-reference-reader`
    command.
23. **IFC byte-compatibility conflicts with mixed-path refusal.** An IFC request containing an ignored
    extra `model-path` currently succeeds but would newly fail, and eager model-module initialization
    could make valid IFC calls depend on provider configuration. Fix — Define preservation as
    valid-schema calls only, lazily import RVT modules after dispatch, and regression-test every IFC
    command under missing and hostile provider configuration.
24. **The new manifest's capabilities are unspecified.** It reads RVT/provider/key files, writes
    cache/artifacts, and executes external software, but Slice G does not prescribe corresponding
    `requires.filesystem` and `requires.software` declarations. Fix — Add exact read/write/software
    declarations and validation tests, while acknowledging the current CLI subprocess is not an OS
    sandbox.
25. **RVT `read-model` has no contract when `AWARE_ARTIFACT_DIR` is absent.** Existing direct bridge
    calls work without it, whereas five binary artifacts cannot be returned as the normal JSON
    response. Fix — Define a stable RVT-only refusal when the directory is absent, or add an explicit
    safe direct-output contract and test both modes.
26. **Numeric limits are promised but never selected.** Without concrete defaults and maxima, tests
    cannot prove useful boundedness, memory behavior, or compatibility with realistic RVT output. Fix
    — Put every default/hard ceiling and aggregate memory/disk/time budget in the normative contract
    and test each boundary plus one-over cases.
27. **The Windows SEA/provider harness is not CI-gated.** Current bridge CI runs source tests on Ubuntu
    only, and an `.mjs` fixture is not directly executable with `shell:false` on Windows. Fix — Add a
    Windows CI job that builds the SEA, packages the fixture provider as a real executable, changes
    cwd, and runs the clean-stage RVT and IFC smoke tests.
28. **The registry generation commands are incomplete.** `aware agent reindex` regenerates
    `registry-catalog.json`, not `registry-index.json`; the plan omits the supported
    `aware agent publish <agent-dir>` step. Fix — Stage the index entry with `aware agent publish`,
    then run reindex, reindex-check, and stats synchronization.

`VERDICT: REVISE`

### Builder response

Accepted all concrete contract corrections, with these scoped decisions:

- The separately installed local adapter is explicitly part of the trusted computing base. This task
  will not claim an AppContainer/deny-network sandbox that AWARE does not provide. The bridge itself
  has no network path, gives the adapter a minimal environment and private cwd, and stages one immutable
  source copy. The receipt signature protects cache/publication integrity against corruption and
  non-TCB writers; it does not defend against a malicious same-user provider.
- Runtime process-tree containment, structured bridge errors, current-sidecar fencing, and successful
  telemetry are added to Task 6 rather than waived. On Windows, a kill-on-close Job Object spans the
  sidecar and provider descendants; cancellation is not considered verified until an actual app-run
  cancellation proves cleanup.
- Cache takeover is narrowed to an exact dead `(pid, processStartIdentity)` owner. A merely old
  heartbeat from a live or unverifiable owner never permits takeover. Publication uses content-addressed
  files created with exclusive `wx`, then an exclusive mapping/complete record created last; it does
  not depend on replace-prone rename semantics.
- The five-file manifest/signature relation, multipart joins, explicit scene, glTF profile, 64-bit
  decimal string IDs, RFC 8785 bytes, concrete limits, cache quotas, Windows CI SEA gate, declared
  permissions, and correct `agent publish`/reindex workflow are made normative.
- Direct `aware agent invoke` is removed because current AWARE intentionally supports builtin agents
  only. `preflight`/`probe` may run at the raw bridge boundary without an artifact directory;
  `read-model` refuses with a stable code. Real AWARE proof uses a temporary `.flo` app and run-owned
  artifact retrieval.

## Round 2 — Codex critic

`VERDICT: REVISE`

The critic found 20 remaining material gaps:

1. A Job containing bridge and provider cannot kill only a timed-out provider; use a supervisor with a
   separate nested provider Job/process group and wait for zero descendants before validation.
2. Generic sidecar kill-on-close would break intentional survivor lifecycles such as Tekla launch;
   containment must be model-reader-specific with compatibility tests.
3. Hard-kill cancellation cannot promise bridge cleanup; define cooperative cancellation plus grace,
   then forced termination and crash-recovery semantics.
4. Typed errors cannot reach traces while `RunEvent::NodeError` is string-only; extend provenance and
   orchestrator additively while retaining the legacy string.
5. CLI version is not a bridge protocol fence; require a capability/build handshake.
6. The manifest schema does not expose process/environment/artifact/progress permissions; declare only
   supported requirements and document runtime-owned channels.
7. Public command inputs omit expected source/provider/signer pins and lowerable limits.
8. Provider approval has no defined pin/rotation mechanism.
9. Naming a metadata schema is not enough; specify root tables, required fields, value union, relations,
   and acyclic kinds before implementation and validate against an authorized real sample if available.
10. Define signature domain, framing, encoding, key consistency, and cross-language vectors byte-for-byte.
11. Select a concrete SEA-compatible Windows held-lock and process-start primitive.
12. `wx` blob creation exposes partial files across keys; serialize per digest and commit completed temp
    files while holding that digest lock.
13. LRU observations need a separately ordered crash-tolerant journal.
14. The proposed 256 MiB peak is incompatible with the in-memory limits; lower limits or design a real
    streaming/external-sort pipeline.
15. Multi-gigabyte exact hard-limit tests do not fit the current CI lane; inject small ceilings for logic
    and reserve named stress tests for an appropriate lane.
16. Classify every array as semantic-order or set-like and define exact keys/ties/duplicate rules.
17. Reject every glTF extension outside an explicit allowlist and enumerate exact material/color rules.
18. Make lazy IFC loading concrete by moving the existing implementation intact behind a dispatcher.
19. Include Rust dependencies/Windows features and statically embedded schema packaging in scope.
20. Publish five run artifacts through invocation-owned temporary names, validate all, commit final IDs
    only on complete success, and clean only invocation-owned paths after failure.

### Builder response

Accepted all 20. The provider gets a model-reader host helper in the AWARE Rust CLI: it owns a separate
provider Job/process group, offers held kernel locks and exact process identity, and participates in
two-phase cancellation. This is opt-in and leaves intentional Tekla survivor lifecycles unchanged.
The plan will add the missing provenance/orchestrator files, a capability handshake, only schema-valid
manifest requirements, complete pinned command inputs, a normative metadata structure, exact receipt
framing, per-digest locked publication, ordered cache-access journal, realistic in-memory ceilings and
injectable small-limit tests, complete array/glTF rules, an IFC dispatcher, embedded schemas, and
failure-atomic run-artifact publication.

## Round 3 — Codex critic

`VERDICT: REVISE`

Nine narrower contradictions remained: one-shot runs had no cooperative stop channel; the long-lived
host protocol was unspecified; making `index.mjs` both a lazy dispatcher and synchronous IFC export
module was impossible; signer identity conflicted with deterministic manifest bytes; the provider pin
had no exact preimage; kernel locks and stale-PID policy competed as authorities; provider-explicit
relations and duplicate GUID sources were ambiguous; unsigned identity rules incorrectly covered signed
Revit `ElementId` parameter values; and the plan named the nonexistent root Cargo lock/manifest.

### Builder response

Accepted all nine. The plan now adds one-shot pidfile/cancellation lifecycle work; a bounded framed
long-lived helper protocol; `model-dispatcher.mjs` as the lazy executable while `index.mjs` preserves IFC
exports; signer identity only in cache/external authentication; an exact seven-field JCS provider pin;
kernel-lock acquisition as sole ownership authority; explicit provider relation kinds and one GUID
authority; positive entity/table IDs versus signed int64 ElementId parameter values; and correct
`cli/Cargo.toml` / `cli/Cargo.lock` commands and paths.

## Round 4 — Codex critic

`VERDICT: REVISE`

Seven remaining protocol details were identified: multiplex host control/binary frames; per-instance
one-shot pidfile exclusion; bridge construction of the provider fingerprint; required parameter storage
types; duplicate-key detection in GLB JSON; canonical topology ordering; and explicit Rust command/agent
fencing files.

### Builder response

Accepted all seven. The plan now specifies concurrent typed framing and immediate handles, exclusive
run-token pidfiles, bridge-composed fingerprint pins, exact storage-type/value mappings, duplicate-key
GLB parsing, canonical triangle/vertex remapping, and explicit `main.rs`/`commands/mod.rs` changes with
an exact-agent-ID fence rather than a nonexistent manifest field.

## Round 5 — Codex critic

1. Multiplexed stdout/stderr frames contain no request/run identifier, so concurrent out-of-order
   provider results cannot be associated safely. Fix — Prefix every binary frame with request ID, run
   handle, sequence, and final flag, and specify concurrent draining plus stdin byte encoding.
2. Triangle sorting still uses provider-assigned indices before canonical vertex remapping, so vertex
   permutations can change triangle order and final bytes. Fix — First sort/deduplicate transformed
   `(position,color)` byte tuples and assign canonical indices, then rotate and sort triangles using
   those canonical indices.
3. Exclusive pidfiles are applied to the entire one-shot app path, breaking existing concurrent one-shot
   runs despite cancellation being described as model-reader opt-in. Fix — Apply singleton pidfile/
   cancellation behavior only to app graphs containing the exact `model-reference-reader` agent,
   preserving all other one-shot concurrency.

`VERDICT: REVISE`

### Builder response

Accepted all three material findings. The host protocol now gives every control and binary frame an
exact request ID, run handle, sequence, final flag, and bounded byte payload; provider stdin is an
explicit fourth binary stream, with length reconciliation before launch and concurrent output draining.
Geometry canonicalizes transformed position/color bytes and assigns provider-independent vertex indices
before triangle rotation/sorting. The singleton control pidfile is now opt-in only for a one-shot graph
containing the exact model-reader agent, with regression coverage for unrelated concurrent one-shot apps.

The five-round cap is reached without a literal `APPROVED` verdict, so the transcript records bounded
non-convergence rather than claiming approval. There is no remaining builder/critic disagreement: every
round-5 finding is incorporated, and the user's continuation request already supplied the human gate to
implement after resolving the final material findings.

## Post-integration real xeoRVT validation — 2026-08-23

The user supplied `Residential building.rvt` and evaluation API credentials after the bounded review.
The managed xeoRVT 0.2.0 job imported the exact 30,695,424-byte source and produced a 13,754,080-byte
GLB plus 1,679,898-byte native metadata JSON. Google Drive access was changed from Restricted to
Anyone-with-link/Viewer only for the import window and verified owner-only/`shared: false` immediately
after import. The API's signed output URLs were malformed as `https:/...`; local normalization of that
transport typo was required to download the successful job outputs.

Test-first validation exposed and resolved four concrete GLB-profile mismatches: the real 8,879,700-byte
JSON chunk exceeded the 4 MiB default; xeoRVT uses a second strict Base64 data buffer; its materials carry
names plus metallic/roughness, BLEND alpha and double-sided presentation; and one primitive carries a
NORMAL accessor. The revised reader keeps the existing 16 MiB hard JSON ceiling, accepts only canonical
embedded buffer data, preserves render-affecting material state, transforms normals correctly, and keeps
all external resources and unsupported semantics refused. The real GLB now normalizes to 6,001 parts and
23,786,172 canonical bytes from 738,232 input triangles, dropping 189 degenerate triangles.

The native xeoRVT metadata has 2,470 explicit elements, 747 drawable elements with `appearances`, 51
referenced types, and 11 referenced levels. A conservative one-off translator using only those explicit
indexes passed the AWARE normalizer; the sole unclaimed node is `EVALUATION_WATERMARK`. This also proved
that v1 must allow indexed non-drawable entities with empty `appearances`. The cloud metadata does not,
however, provide the reviewed parameter storage types or explicit relationship semantics: 2,934 of its
3,311 parameter rows use negative IDs, 139 ID values repeat (one 776 times), and values expose only JSON
string/number types. Therefore no provider-specific translator was added to AWARE core and the cloud run
is recorded as real-output compatibility evidence, not proof of the local provider execution contract.

## Issue #464 addendum review — Round 1

Codex returned `VERDICT: REVISE`. The proposed JavaScript-only casing fix identified a deterministic
failure, but the plan incorrectly treated a plain copied environment as the already-proven production
path. The critic required: separation of the original app failure from the later direct diagnostic;
an actual packaged red/green path; explicit exit/output/version/non-skip acceptance; deterministic
semantics for differently cased duplicate variables; all-allowlist and forbidden-variable coverage;
unchanged POSIX semantics; and a behavioral proof of the environment received after Rust `env_clear()`.

### Builder response

Accepted every finding. Section 8 now distinguishes the observations and records the current
live-environment stress result rather than claiming a lifecycle cause. Production snapshots its chosen
environment into a plain object, so the existing packaged command path deterministically crosses the
fixed seam. Windows normalization uses canonical uppercase output and fails closed on conflicting
aliases; tests cover every allowed key, forbidden mixed-case keys, aliases, and POSIX behavior. The
fixture provider itself checks `SYSTEMROOT` and the exact received key set, while the Windows harness
requires valid outputs from all packaged model commands and reports both executable versions.

## Issue #464 addendum review — Round 2

Codex returned `VERDICT: REVISE` with four gaps: snapshotting the whole configuration would remove
Windows' case-insensitive lookup for the bridge's own `AWARE_*` controls; the packaged CI job still used
Node 22 and did not force mixed-case OS variables; the fixture could not know which optional values were
supposed to arrive; and Unicode uppercasing could turn a non-ASCII near-alias into an ASCII allowlisted
name.

### Builder response

Accepted all four. The snapshot now exists only inside the provider allowlist boundary, leaving AWARE
control lookup and host launch semantics intact. The packaged environment deletes all aliases and seeds
all five allowed values under controlled mixed-case spellings, plus a forbidden sentinel; the fixture
requires the exact fixed eight-entry result. The harness and CI pin Node 24.14, include an automated
legacy red control, and ASCII-only folding leaves Unicode near-aliases forbidden.

## Issue #464 addendum review — Round 3

Codex returned `VERDICT: REVISE` because the harness was pinned to Node 24.14 while the release workflow
still built the shipped connection-reader SEA with Node 22. A green Node-24 reader/provider pair would
not prove the production Node-22 reader/Node-24 provider combination.

### Builder response

Accepted. Section 8 now pins the release connection-reader build to Node 24.14.0 as well, making the
tested bridge/provider runtime combination identical to the one shipped by this release.

### Codex follow-up

The mixed-runtime gap is resolved: section 8 now requires both CI and the release connection-reader SEA
build to use Node 24.14.0.

`VERDICT: APPROVED`

## Final implementation review — 2026-08-27

Codex found five material gaps in the preserved #453 implementation: an authoritative cache fence
could not recover a missing/malformed owner diagnostic; one-shot reader apps held a control lock but
published no pidfile for `aware app stop`; crash-left provider run directories were never swept;
request-supplied limits were ignored in favor of test-only dependency limits; and an aborted provider
run was flattened into retryable `reference-provider-failed`.

### Builder response

Accepted all five. Fenced ownership now replaces unverifiable diagnostics while unfenced callers still
fail closed. The model-only one-shot guard publishes and removes the standard pidfile. Provider startup
reclaims only non-link `run-*` directories older than one hour, twice the hard conversion timeout.
Command admission validates one complete limit set and propagates it through every downstream boundary.
An aborted host rejection or result now returns non-retryable `reference-cancelled`. Focused regression
tests cover every branch; the full release gate and a clean follow-up Codex review remain required.

## Final implementation review — follow-up

Codex found two additional runtime edge cases: age-only provider-run cleanup could reclaim a legitimate
conversion whose sequential bounded phases exceed one hour, and explicit `"limits": null` was treated as
an omitted field by nullish fallback.

### Builder response

Accepted both. Every live provider run now renews a private filesystem heartbeat; the stale sweep preserves
an old directory while that lease is fresh and reclaims it after a crash without relying on reusable PIDs.
Limit selection distinguishes
an absent request field from an explicit null, so malformed null limits receive the stable
`reference-limits-invalid` refusal and can never inherit dependency defaults. Regression tests cover an
old live run, an old abandoned run, and null-limit precedence at both command and response boundaries.

## Final implementation review — second follow-up

Codex found that fenced cache-debris removal could still propagate a raw filesystem error and that a
PID-only active-run marker could retain sensitive crash debris after PID reuse.

### Builder response

Accepted both. Every cache-owner removal path now converts I/O failures into bounded
`reference-cache-owned` errors. Provider-run ownership is a renewable filesystem heartbeat lease rather
than a process-id assertion: active runs refresh it, crashes stop refreshing it, and the sweep reclaims
the directory after the lease ages out. Tests inject a fenced removal failure and distinguish fresh from
stale heartbeat markers without exposing private paths.

## Final implementation review — third follow-up

Codex found that a null value nested inside the limits object still selected its default and that failure
to remove a just-created directory after marker creation failed could escape as raw filesystem stderr.

### Builder response

Accepted both. The shared limit normalizer now distinguishes absent properties from present null values,
so every malformed nested override is rejected at admission. Run-root inspection, allocation, ownership
creation, failed-creation cleanup, sweeping, and final cleanup all convert filesystem failures to the
bounded `reference-provider-run-cleanup-failed` envelope. Focused tests cover both top-level and nested
null limits; the independent review gate must be clean before integration.

## Final implementation review — fourth follow-up

Codex found three remaining lifecycle and validation gaps: a direct Ctrl+C or SIGTERM could terminate a
one-shot reader without unwinding its control guard; Unix `aware app stop` signalled only the AWARE PID
without ensuring the reader bridge/provider descendants stopped; and a syntactically valid `null`
cache-owner diagnostic reached raw property access.

### Builder response

Accepted all three. Reader one-shots now race their orchestration against Ctrl+C/SIGTERM, so interruption
drops the run future and its guard; the reader bridge is the only invoker child configured kill-on-drop,
and the provider host already owns its provider child the same way. `app stop` verifies the kernel-held
reader control before trusting its pidfile, which prevents crash-left pidfiles and recycled PIDs from
being signalled; Windows retains explicit tree termination. Cache owners now pass a complete closed
schema and value validation before any field access, including release-time token checks. Focused tests
cover held/released kernel control, stale stop state, and JSON `null` owner debris.

## Final implementation review — fifth follow-up

Codex found four final boundary gaps: requests issued after the managed host exited could be registered
after the pending-request rejection sweep and hang forever; cache receipts, signatures, and blobs were
loaded before their sizes were bounded; stale reader control attempted to parse a missing or malformed
pidfile before consulting the crash-released kernel fence; and signed package provenance still claimed
agent version `0.2.0` after the manifest advanced to `0.4.0`.

### Builder response

Accepted all four. Host failure is now a persistent terminal state, so every later request rejects with
the same stable host error and close performs no dead-pipe write. Cache control files have small fixed
ceilings, artifact records are checked against request limits, and fixed-size reads verify the file did
not change without allocating from attacker-controlled post-check growth. Reader stop consults the
kernel fence before reading diagnostic pidfile state, allowing absent and malformed crash debris to be
reclaimed safely. Signed package provenance now reports agent `0.4.0` while retaining the independently
versioned bridge build identifier. Focused regressions cover each branch; full gates and a fresh clean
independent review remain required before integration.

## Final implementation review — sixth follow-up

Codex found six remaining contract and trust-boundary defects: canonical GLB output duplicated a node
name for each source primitive and could not be normalized again; the JavaScript executable digest
bracket did not bind the verified bytes to the image opened by the Rust host; preflight omitted the
managed authority-store validation used by conversion; stable managed-host output-limit errors were
flattened into retryable provider failures; nested glTF node extensions and null nodes escaped the
closed geometry contract; and cache-owner removal errors escaped as raw filesystem failures.

### Builder response

Accepted all six. Canonical meshes now retain every primitive for one named node, preserving primitive
ordinals without duplicate drawable names. Every host request carries the executable digest; the Rust
host hashes an open image handle and retains it through process creation, denying Windows write/delete
sharing until the suspended image is mapped and using a descriptor-backed launch path on Unix. Both
preflight and conversion apply the same protocol-specific authority-store validator. The client admits
only the host's closed stable error-code set and the provider boundary preserves those codes. Active
glTF nodes must be closed objects with only supported transform/topology properties. Cache-owner release
maps removal failure to a redacted stable cache error. Focused regressions cover each branch; full gates
and a fresh clean independent review remain required before integration.

## Final implementation review — seventh follow-up

Codex found five remaining admission and lifecycle defects: nested reader nodes did not acquire the
one-shot control fence; valid input could expand into canonical object graphs beyond the specified
resident gate; canonical output could exceed the normalizer's own JSON/count profile; byte-backed JSON
accepted malformed UTF-8 through replacement decoding; and pathless model-reader calls fell through to
the legacy IFC route.

### Builder response

Accepted all five. Reader control now uses the same recursive live-node traversal as dispatchability.
Canonicalization reserves a conservative worst-case working-set estimate against the committed 1 GiB
gate before object expansion, then checks emitted JSON and structural counts before final allocation.
The shared strict parser performs fatal UTF-8 validation for byte inputs, including GLB chunks. The
private model-reader invocation marker selects RVT admission even for malformed calls, and source shape
is validated before provider or host setup. Focused regressions cover each branch; complete release
gates and a fresh clean independent review remain required before integration.

## Final implementation review — eighth follow-up

Codex found four remaining failure-boundary and lifecycle defects: unexpected RVT failures exposed raw
private paths; stale reader pidfile cleanup released the kernel fence before removal; provider-tree
termination was initiated but not awaited; and cache hits did not refresh deterministic LRU recency.

### Builder response

Accepted all four. Unexpected RVT failures are wrapped in a stable redacted internal-error envelope while
legacy IFC behavior is retained. Stale pidfile reclamation now owns the reader kernel fence through
removal. The managed host terminates and waits for the Windows Job or Unix process group to reach zero
active processes before accepting output. Successful publications and reads append bounded, crash-tolerant
access observations under the maintenance fence, and eviction uses their total order with a deterministic
key tie-break. Focused regressions cover redaction, fenced reclamation, and read-refreshed LRU eviction;
complete release gates and another clean independent review remain required before integration.

## Final implementation review — ninth follow-up

Codex found three remaining bounded-runtime defects: generated cache manifests could exceed the active
component limit and become unreadable immediately after publication; the managed host's exit-124 timeout
reason was flattened into a generic provider failure; and provider executable provenance hashing loaded
the complete image into one JavaScript buffer.

### Builder response

Accepted all three. Cache publication now normalizes the active request limits and rejects an oversized
manifest before exposing an entry. Managed-host timeout completion becomes retryable
`reference-provider-timeout` and is preserved through the provider boundary. Provider executable
provenance uses the same incremental bounded-memory hashing path as source files while retaining the
existing absolute regular non-link admission. Focused regressions cover the publication fence, typed
timeout, and streamed image digest; complete release gates and a fresh clean independent review remain
required before integration.

## Final implementation review — tenth follow-up

Codex found three remaining supported-input defects: probe bounds omitted active unclaimed geometry;
managed-cloud selection remained hardcoded to protocol v1 inside the signed canonical request; and
null mesh objects escaped the typed geometry-validation boundary as raw JavaScript errors.

### Builder response

Accepted all three. Probe bounds now derive from every canonical active-scene POSITION accessor, including
unclaimed nodes. The selected provider protocol is carried into both canonical-request builders, binding
the provider request, cache identity, and signed provenance to the same v1 or v2 choice. Meshes and
primitives must be objects before field access. Focused regressions cover offset unclaimed geometry, the
managed-cloud request preimage, and null mesh/primitive entries; complete release gates and a fresh clean
independent review remain required before integration.

## Final implementation review — eleventh follow-up

Codex found three remaining public-boundary and lifecycle defects: typed provider failures were reduced
to a display string in app run traces; an unlocked reader control file could cause `app stop` to remove
the pidfile of a later live non-reader run; and the new npm executable target had no Unix Node shebang.

### Builder response

Accepted all three. `node-error` records now retain the legacy display text and add the bridge's bounded
machine-readable `{code, phase, retryable, message, diagnosticId}` object. Reader control persists its
exact run identity under the kernel lock, so stale reclamation removes a pidfile only when it belongs to
that crashed reader and otherwise lets normal stop handling signal the current process. The dispatcher
starts with the portable Node shebang. Focused regressions cover structured trace serialization and
stale-owner mismatch; complete release gates and another clean independent review remain required before
integration.

## Final implementation review — twelfth follow-up

Codex found four remaining control and determinism defects: interruption released the reader fence
before the provider tree had confirmed shutdown; a reader reached through the permitted app-backed hop
bypassed the top-level fence; authenticated warm hits still invoked provider `describe`; and canonical
vertices were sorted as JavaScript numbers rather than by their final little-endian float32 bytes.

### Builder response

Accepted all four. The interrupted one-shot future is retained and awaited so the bridge aborts its
request, the managed host joins provider cleanup, and only then does the control guard drop. The
top-level graph check resolves the one allowed app-backed hop and fences its full run when that backing
graph reaches `model-reference-reader`. Warm lookup scans only closed cache-key directories under the
maintenance fence, authenticates the signed receipt and every blob, and matches the pinned source,
request, provider, and signer identities before deciding whether provider readiness is needed. Vertex
deduplication and ordering use the exact concatenated float32 LE attribute bytes consumed by GLB
publication. Focused regressions cover the nested reader graph, zero provider calls on a warm snapshot,
and byte-order cases where numeric and encoded order differ; complete release gates and a fresh clean
independent review remain required before integration.

## Final implementation review — thirteenth follow-up

The complete Rust gate found one simulation regression in the new nested reader-fence discovery: on
Windows, loading an uninstalled agent surfaced `ERROR_PATH_NOT_FOUND` as a raw I/O failure before the
existing `--simulate` escape hatch could synthesize the node output.

### Builder response

Accepted. Reader-fence discovery now performs the same fenced absent-versus-unreadable manifest check
as the established nested-app scanner: a genuinely absent agent is skipped, while a present malformed
or unreadable manifest still fails closed. A focused unit regression and the existing end-to-end
`--simulate` missing-agent regression cover the boundary; complete release gates and a fresh clean
independent review remain required before integration.

## Final implementation review — fourteenth follow-up

Codex found four remaining lifecycle and validation defects: Unix interruption waited for reader
cleanup without explicitly signalling the child bridge; lifecycle-start graphs reached the commercial
reader before acquiring its instance fence; simulation unnecessarily contended on that fence despite
dispatching no agents; and null glTF buffer-view objects escaped as raw JavaScript errors.

### Builder response

Accepted all four. Every app run now owns a reader-cancellation registry shared through its nested
dispatch invokers. Unix interruption forwards SIGTERM to each registered model-reader bridge before
awaiting orchestration, while a cancellation bit closes the pre-spawn race. Reader control is acquired
before the one-shot/long-running split, retained for the full run, and deliberately bypassed only by
simulation. Active scenes and buffer views must be non-null objects before field access. Focused
regressions cover cancellation registration, the simulation exemption, and typed malformed-GLB errors;
complete release gates and a fresh clean independent review remain required before integration.

## Final implementation review — fifteenth follow-up

Codex found three remaining bounded-validation defects: pinned warm-cache discovery authenticated and
loaded every component blob before filtering candidate identity; the strict JSON parser treated all
JavaScript whitespace as JSON whitespace; and provider provenance string limits disagreed with the
published schemas in both maximum length and byte-versus-character semantics.

### Builder response

Accepted all three. Cache discovery now authenticates each signed receipt and its bounded manifest,
applies the source/request/provider/signer pins, and reads component blobs only for the matching entry;
cache reads also preserve cancellation instead of reducing it to a miss. Strict parsing accepts only
space, tab, carriage return, and line feed between tokens. Provider, engine, and engine-version fields
use the schemas' 128-code-point ceiling while adapter build IDs retain their 256-code-point ceiling.
Focused regressions cover a damaged large blob on an earlier nonmatching candidate, cancelled lookup,
Unicode non-JSON whitespace, multibyte 128-character provenance, and 129-character refusal. The full
276-test Node suite, packaged build, and Windows lifecycle harness pass; a fresh clean independent review
remains required before integration.

## Final implementation review — sixteenth follow-up

Codex found five remaining lifecycle and bounded-validation defects: a simulated long-running reader
graph could overwrite and then remove the live reader instance pidfile; closing a client after a
terminal protocol error could leave the managed host child running; provider output-limit failures did
not interrupt a provider that kept its pipe open; provider stdin delivery failures were ignored; and
zero-length glTF accessors passed structural validation.

### Builder response

Accepted all five. Simulation still discovers reader graphs and therefore neither acquires their live
control fence nor manages their ordinary pidfile. Client close now terminates and awaits a still-live
host after terminal protocol failure. The managed host supervises stdin and bounded stdout/stderr tasks,
kills the provider tree on the first I/O failure, and emits a typed host failure only after containment
and task cleanup; missing stdin is also fail-closed. Accessors require a positive element count. Focused
regressions preserve an existing reader pidfile byte-for-byte, await host termination, prove immediate
output-limit supervision and failed stdin delivery, and reject zero-length accessors. Complete release
gates and a fresh clean independent review remain required before integration.

## Final implementation review — seventeenth follow-up

Codex found four remaining admission, normalization, and lifecycle defects: triangle-strip/fan
expansion could exceed the canonical index limit; an abnormal Node bridge exit could release the app
fence before the orphan host finished provider-tree cleanup; request-only validation still followed
provider configuration and managed-host startup; and duplicate semantic relationship edges were
accepted when their provider relation IDs differed.

### Builder response

Accepted all four. Geometry admission now bounds and reserves the expanded topology before allocating
triangle records, independently of the input accessor limit. The app and Rust host share a second
per-instance kernel fence: the host owns it from startup through protocol shutdown and provider-task
joining, while every replacement run must pass it before launching another host. This remains safe in
the bridge-crash/startup race because competing old and new hosts serialize before either can accept a
provider request. Limits, protocol, canonical request settings, source digest, and required/optional
pins are validated before configuration or host work. Relationship normalization rejects duplicate
`(kind, from, to, providerRelationKind)` tuples as well as duplicate IDs. Focused regressions cover the
expanded-index ceiling, cross-process cleanup-fence blocking, environment-independent request errors,
and duplicate edges. The 280-test Node suite, Rust lint/model-reader/app-run gates, packaged build, and
Windows lifecycle harness pass; a fresh clean independent review remains required before integration.

## Final implementation review — eighteenth follow-up

The bounded fresh pass found that an input primitive containing only degenerate triangles produced a
canonical GLB with a zero-count index accessor, so the output could not pass the same positive-accessor
profile on a second normalization. Rechecking the relationship finding against D5 also showed that the
seventeenth pass had incorrectly treated endpoint equality as duplication: `depends-on` and
`provider-explicit` are explicitly directed multigraphs, and distinct relation IDs may therefore bind
the same endpoints.

### Builder response

Accepted the geometry finding and corrected the relationship response. Degenerate-only parts remain in
the immediate metadata join and coverage result, but are omitted from the drawable canonical GLB, which
now contains no zero-count accessors and is byte-identical after a second normalization. Parallel edges
for directed-multigraph kinds are preserved and deterministically ordered by their distinct relation
IDs; duplicate IDs remain refused. Focused regressions cover both contracts before the final release
gates.

## Final implementation review — nineteenth follow-up

The first pull-request CI run exposed three platform-specific gate failures hidden by the Windows
workspace: Linux reported a symbolic link as merely non-regular before reaching the explicit link
refusal, the Unix SIGTERM handler used `expect` in a non-test binary that denies panic helpers, and two
small libc `kill` calls lacked the safety comments required by the repository-wide Clippy policy.

### Builder response

Accepted all three. Provider path validation now reports link/reparse refusal before the regular-file
shape check. Unix signal installation falls back to the existing Ctrl+C future without panicking, and
both PID-only libc calls document their range and memory-safety invariants immediately above the unsafe
blocks. The focused provider suite and the exact local formatting/Clippy commands pass before the CI
rerun.
