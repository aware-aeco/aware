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
