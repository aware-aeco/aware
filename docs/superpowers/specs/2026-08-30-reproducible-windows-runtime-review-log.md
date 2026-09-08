# Plan Review Log: reproducible AWARE Windows runtime

Started 2026-08-30. MAX_ROUNDS=5.

## Round 1 — VERDICT: REVISE

1. **Critical — “two independent builds” are only two worktrees on one mutable host.** They share Git objects, Cargo/npm caches, compiler, linker, Windows SDK, environment, and potential compromise; `/Brepro` cannot close this, especially because `ring` invokes `cc`/MSVC. **Fix:** Build fresh clones on two independently provisioned builders with isolated caches and digest-pinned Rust, Cargo, MSVC/clang, linker, SDK, Node, and npm toolchains.
2. **Critical — the claimed clean-checkout reproducibility cannot be recreated elsewhere because neither source tip is remotely reachable.** AWARE is eight commits ahead of its remote branch and FloLess is forty-two ahead, while the plan forbids pushing. **Fix:** Retain hash-verified Git bundles/source archives for both exact commits, or publish immutable remote refs.
3. **Critical — the plan relies on signatures that the private package does not have.** The approved internal installer permits unsigned output and `vpk pack` receives no signing arguments. A malicious builder can rewrite binaries, receipts, manifests, and the unsigned executable containing the provider hash coherently. **Fix:** Sign an external canonical manifest/attestation with an independently protected key and verify it during packaging and launch, or explicitly treat `.2` as unauthenticated test media.
4. **High — the release-workflow proposal conflicts with its existing tag-version mutation.** It rewrites `Cargo.toml`, leaves `Cargo.lock` unchanged, injects the forbidden Google secret, and builds without `--locked`. **Fix:** Create a separate internal reproducibility entrypoint and require release versions to be committed consistently rather than rewritten during CI.
5. **High — compiler and dependency authority remains ambient and unrecorded.** Build environment overrides and actual MSVC/SDK/esbuild/postject bytes are not closed by the receipt. **Fix:** Use an environment-allowlisting wrapper with absolute digest-checked tools and record every admitted executable/native input.
6. **High — the receipt has no valid route through the FloLess schema or installer.** Runtime schema v1 is exact and installation hard-codes the old layout. **Fix:** Make the receipt build-only or introduce schema v2 and update both verifiers, layout, installer, marker, manifests, and mutations atomically.
7. **High — verification and staging remain vulnerable to TOCTOU and concurrent builders.** Mutable ignored outputs are checked/copied without one exclusive generation lock. **Fix:** Build into a uniquely owned immutable directory, verify/copy through the same handles, and lock through package hashing.
8. **High — the `.1` transition baseline is not anchored.** The retained runtime is ignored/mutable even though an approved manifest hash exists. **Fix:** Verify retained manifest and full package against approved hashes before deriving the transition inventory, then commit the fixed baseline.
9. **High — the post-integration “any packaged byte” rule is unusable while FloLess packaging remains nondeterministic.** FloLess SEA embeds absolute paths and `vpk` is ambient. **Fix:** Base version decisions on a canonical pre-pack payload manifest or make the entire packaging path reproducible.
10. **Medium — dynamic Git-tree materialization omits Windows filename hazards.** **Fix:** Reject case collisions, reserved/normalizing names, LFS pointers, non-UTF-8 names, unexpected attributes, and invalid file types.
11. **Medium — source-wiring tests do not prove the effective build command.** **Fix:** Behaviorally capture the real invocation and test wrong Node/tool digest, poisoned environment, and omitted-lock mutations.
12. **Medium — receipt identity is absent from operational diagnostics.** **Fix:** Carry receipt digest/build ID into `distribution.json`, installed marker, startup logs, UAT evidence, and field-specific errors.

Reviewer’s simpler safe shape: one dedicated internal build wrapper consuming retained source bundles and a fixed builder image, emitting one externally authenticated attestation and immutable output directory; leave public release and schema v1 untouched until the boundary is implemented end-to-end.

### Response

Accepted all twelve findings. The revision now requires two fresh isolated Windows builders rather than worktrees, self-contained verified Git bundles for unreachable commits, a digest-closed environment-allowlisting build wrapper, behavioral command tests, immutable output plus an exclusive end-to-end lock, NTFS-safe Git materialization, and an anchored `.1` baseline. It leaves the public release workflow unchanged. The receipt is deliberately shipped via an atomic schema-v2 change and exposed in operational evidence. FloLess version identity is based on a canonical pre-pack manifest, with private SEA path leakage and ambient Velopack also closed. Because no independent signing authority exists, `.2` is explicitly unauthenticated internal test media and is hard-stopped from production/release use.

## Round 2 — VERDICT: REVISE

1. **Critical — provenance hashes are self-referential.** The pre-pack manifest covers `distribution.json`, while `distribution.json` was to contain that manifest digest and the later final package hash. **Fix:** Use an inner payload manifest excluding itself/envelope and record the final package hash only in external evidence.
2. **High — source-bundle order is contradictory.** Pre-edit bundles cannot contain later implementation commits. **Fix:** Distinguish pre-change recovery bundles from final build-input bundles created after each implementation commit.
3. **High — a receipt cannot contain exact absolute commands/environment values and also be path-free/byte-identical.** **Fix:** Canonicalize logical tool IDs and root tokens in the receipt; keep raw per-builder transcripts separate.
4. **High — path independence is not actually tested when two VMs may use identical layouts.** **Fix:** Require deliberately different root identities/lengths and scan artifacts for raw/encoded path forms.
5. **High — the `.1` package anchor has no literal expected container hashes.** **Fix:** State approved NUPKG, Setup, and portable hashes/sizes or drop the claim.
6. **High — schema-v1 compatibility may let `.2` bypass v2 receipt requirements.** **Fix:** `.2` staging/boot accept only v2; isolate v1 in a read-only transition utility.
7. **High — “normalize semantically” is undefined/open-ended.** **Fix:** Permit only validated UTF-8 CRLF→LF with BOM, lone CR, final newline, and every other difference rejected.
8. **Medium — fixed 6,916 test count becomes stale as tests are added.** **Fix:** Run dynamically discovered complete suite, record post-change count, and guard against skipped/lost test files.

### Response

Accepted all eight findings. The plan now uses non-circular `prepack-payload-manifest.json` plus an external evidence envelope, splits recovery and final build bundles, tokenizes canonical receipts while retaining raw builder transcripts separately, mandates deliberately different builder paths plus encoded-path scans, pins all three retained `.1` container hashes/sizes, makes `.2` v2-only at stage/install/boot, defines the sole text transition as exact UTF-8 CRLF→LF, and replaces the stale test count with dynamic complete-suite/discovery-loss checks.

## Round 3 — VERDICT: REVISE

The eight named Round 2 defects were substantively addressed, but six adjacent execution boundaries remained:

1. **High — FloLess packaging is not hermetic.** The input list omitted `server/package-lock.json`, while live builds use ambient Node and ignored esbuild/postject/rcedit files. **Fix:** Add a FloLess wrapper that extracts a verified bundle into an empty root, installs offline from the lock, and invokes only digest-bound tools.
2. **High — the Velopack pin authenticates only the launcher, not its 449-file .NET tool closure.** **Fix:** Retain/hash the NUPKG, full resolved tool/runtime closure, and invoke an absolute verified launcher with PATH disabled.
3. **High — retained dependencies are incomplete.** AWARE has 424 registry crates and the reader consumes additional npm/WASM inputs. **Fix:** Vendor/hash every Cargo crate and npm tarball/native artifact, disable network in both builds, and reject lifecycle downloads.
4. **High — post-integration `.3` ordering does not require new bundles.** **Fix:** After every merge, create new verified AWARE/FloLess bundles and build only from them.
5. **High — the exclusive lock has no shared identity/acquisition protocol.** **Fix:** Use a machine-wide package/version-keyed mutex or fixed lock, hold its handle through final hashing, and emit owner/build diagnostics.
6. **Medium — dynamic discovery cannot detect deleted tests by itself.** **Fix:** Compare exact sorted test-file inventory with the recovery bundle plus a reviewed delta, and reject TAP skips outside a committed allowlist.

### Response

Accepted all six findings. The plan now adds a separate offline/digest-closed FloLess build wrapper and includes its lock/dependencies; vendors and hashes all Cargo/npm/native/WASM inputs with network disabled; binds the entire Velopack NuGet/.NET/tool closure; requires fresh post-merge bundles; specifies a fail-closed machine-wide version-keyed mutex held by the outer wrapper; and gates the exact test-file inventory plus TAP skip/todo policy against a reviewed recovery-bundle delta.

## Round 4 — VERDICT: REVISE

Round 3's six mechanisms were generally feasible, but five blockers remained:

1. **High — sequential rebuilds can still produce different `.2` media.** A transient mutex does not enforce never-rebuild. **Fix:** Persist a non-deletable completed-version seal containing source, pre-pack, and package hashes; reject every later build of that version.
2. **High — offline dependency closure is generated in the wrong phase.** Pre-change locks cannot authorize final/post-merge dependencies. **Fix:** Generate a new offline closure from each exact final/post-merge bundle and bind it to that builder manifest.
3. **High — retaining .NET does not force Velopack to use it.** A framework-dependent apphost can roll forward/use ambient hooks/stores/runtimes. **Fix:** Use pinned private `dotnet.exe exec` with explicit runtimeconfig/deps, sanitized host environment, disabled roll-forward/multilevel lookup, and loaded-module verification.
4. **High — “without touching production” conflicts with production service endpoints.** **Fix:** Use non-production endpoints or require a dedicated production test tenant with an enumerated request/mutation allowlist and cleanup evidence.
5. **Medium — assertion counts are unavailable from the current runner.** **Fix:** Gate the available file/test/skip/todo/cancel/failure counts or add runtime assertion instrumentation.

### Response

Accepted all five findings. The plan now creates a permanent create-new version reservation before mutation; any crash burns the version, and a durable completed seal prevents sequential rebuilds. It regenerates/binds offline closures per final/post-merge bundle, invokes Velopack through a fully pinned private `dotnet exec` module graph, explicitly gates production-backed UAT on a dedicated tenant plus closed request and cleanup ledger, and limits TAP claims to the counts the runner can actually observe.

## Round 5 — VERDICT: APPROVED

No concrete implementation blocker remains. Round 4 closes the prior defects:

- The create-new permanent seal reserves the version before mutation, burns failed versions, prevents concurrent and sequential reuse, and has required deletion/crash mutation tests.
- Offline closures are regenerated from each final or post-merge bundle's exact three lockfiles and bound to that generation.
- Velopack runs through pinned private `dotnet.exe exec` with explicit runtime/dependency files, sanitized host authority, and verified loaded modules.
- Production-backed UAT requires a dedicated tenant, closed request policy, mutation ledger, cleanup, baseline reconciliation, and stops before networking without them.
- The test gate uses exact file inventory and observable TAP counts while explicitly rejecting unsupported assertion-count claims.

The reviewer concluded that these requirements adequately replace the ambient dependencies, global `vpk`, production endpoint behavior, and unparsed test output present in the live FloLess code.
