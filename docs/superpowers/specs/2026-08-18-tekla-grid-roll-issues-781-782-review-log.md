# Plan Review Log: Tekla grid and roll issues #781/#782

Started 2026-08-18 Europe/Warsaw. MAX_ROUNDS=5.

## Round 1

Codex verdict: **REVISE**.

Full critique:

1. **Critical — Materialization identity will lie.** `ComputeBakeMaterializationHash` hard-codes `tekla-connection-materializer-v1`, so old behavior and new behavior would share a hash. Bump the internal payload for each semantic change, retain `AWARE_BAKE_V1` retirement compatibility, and test the transitions.
2. **High — Bounding to `bounds` can truncate.** Authored extents may lie outside declared bounds. Derive the tightest Tekla-representable family envelope from authored extents and perpendicular grid coordinates.
3. **High — Changed grid properties are not read back.** Verify Origin, all four planar extensions, and IsMagnetic, reconstructing actual endpoints before retirement.
4. **High — Child identity can lose `realizedBy`.** Resolve grid children only through `realizedReferences`; test order, shared GUID, parent identity, and one native object.
5. **High — A warning can become false on batch failure.** Queue candidates and emit `status:"warning"` only after successful commit/read-back.
6. **High — `rot` lacks a canonical contract.** Document type, units, right-handed sign, directed axis, zero frame, and applicable kinds.
7. **High — `TOP + rot` is unproven.** Make the live probe determine enum/sign mapping for upward, inverted, horizontal, and sloped axes.
8. **Medium — finite values can still be hostile or strings.** Require JSON numeric input, normalize to a bounded interval, and keep roll off non-member insertions.
9. **High — tests are textual and single-version.** Add executable production-helper tests and semantic compilation against both supported Tekla versions, including failure/read-back cases.
10. **Medium — parallel PRs would conflict.** Deliver sequentially: merge #781, then branch #782 from updated main.

Disposition: all ten findings accepted. The revised plan adds non-truncating envelopes, post-commit warnings, authoritative read-back, exhaustive receipt identity, payload v2/v3 transitions, a canonical `rot` contract, probe-derived conversion, shared executable helpers, two-version semantic compilation, and sequential delivery.

## Round 2

Codex verdict: **REVISE**.

Full critique:

1. **High — Single-family behavior is undefined.** Define preflight rejection or a proven encoding that does not invent unreceipted axes.
2. **High — Finite inputs can overflow derived native values.** Validate checked spacings/envelopes/extensions, duplicate coordinates, and whitespace-sensitive label tokens; add boundary tests.
3. **High — The roll contract ignores `meta.up`.** Reject non-Z-up Tekla scenes or define the complete conversion, and add cross-sink golden tests.
4. **High — One mapping may not cover profile families.** Probe asymmetric W/I, channel, angle, and RHS/HSS families and verify B-rep/DSTV geometry rather than assuming `GetCoordinateSystem()` observes roll.
5. **High — Two-version compilation is not enforceable in ordinary CI.** Use pinned references in CI or a mandatory, non-skippable pre-merge gate that records both local results.
6. **High — Live mutation safety is incomplete.** Require a disposable model, unique source IDs, explicit PID/version, model identity checks, a snapshot, and cleanup.
7. **Medium — Exact zero/no-clash wording is physically incorrect.** Define tolerances and permitted intentional intersections; fail only unexpected penetrations.
8. **Medium — Versioning/documentation targets are inaccurate.** Update canonical and Tekla command docs, bump both parent agent manifests, and do not add skill-level version metadata.
9. **Medium — `qa-ready` lacks a reproducible payload.** Retain a commit-SHA-addressed CI artifact with installation and host requirements or defer QA status.

Disposition: all nine findings accepted. The revised plan preflight-rejects single-family/overflow/token-invalid grids, constrains Tekla to Z-up, adds profile-family B-rep/DSTV probes, makes both installed SDK compilations a mandatory recorded gate, hardens live-model safety and tolerances, corrects parent-agent versioning, and adds a merged-SHA CI bridge artifact before `qa-ready`.

## Round 3

Codex verdict: **REVISE**.

Full critique:

1. **High — A canonical `rot` claim would make viewer documentation false.** Implement/validate roll in `viewer-3d`, including Z-up reflection, or do not claim it in the canonical viewer schema.
2. **High — IFC contradicts the proposed validation contract.** Apply the same numeric/finite/normalization/rejection rules and malformed/periodic cross-sink tests.
3. **High — Tekla-only grid limits lack receipt taxonomy.** Classify representability limits as exhaustive unsupported families without aborting unrelated records, or tighten every consumer consistently.
4. **High — Profile-family fallback lacks production data.** Require a proven universal mapping or discriminate with validated `xsection`; cover rect, CHS, and absent descriptors explicitly.
5. **High — Payload retirement is only hash-tested.** Use scenario-unique but sequence-stable source IDs and live-test v1→v2 and v2→v3 replacement/failure behavior.
6. **Medium — PR #781 also needs a Tekla manifest bump.** Bump Tekla in each sequential PR; viewer only in #782.
7. **Medium — Grid/roll tolerances are movable.** Fix coordinate, angular, and vector-dot constants before implementation and reuse them everywhere.

Disposition: all seven findings accepted. Round 3 now includes viewer and IFC roll conformance, exhaustive Tekla-only unsupported receipt families, a required universal mapping across every canonical/legacy profile shape, live materializer-version retirement drills with stable scenario sources, per-PR parent manifest bumps, and fixed 0.1 mm/0.01 degree/vector-dot acceptance tolerances.

## Round 4

Codex verdict: **REVISE**.

Full critique:

1. **Critical — Model identity is not inside the transaction.** Pass expected model identity into the same bake and verify it immediately before work-plane or object mutation.
2. **High — Normalization/zero-frame boundaries are underspecified.** Choose the interval, negative-zero behavior, near-vertical threshold, and boundary vectors exactly.
3. **High — `meta.up` behavior differs across consumers.** Validate unknown values everywhere and explicitly transform or reject IFC Y-up.
4. **High — Geometry oracle ignores symmetry.** Compare geometry modulo each profile's symmetry group, use scalar read-back for the full angle, and asymmetric profiles for sign/handedness.
5. **High — Viewer tests need not execute JavaScript.** Extend/run the existing Playwright harness and inspect rendered geometry.
6. **Medium — IFC manifest/version are missing.** Correct the public description and bump the parent manifest.

Disposition: all six findings accepted. Round 4 puts a model-path guard inside the bake transaction, fixes normalization to `[-180,180)` and the IFC inclusive near-vertical threshold, validates/rejects `meta.up` explicitly by sink, makes geometry checks symmetry-aware, requires the real browser harness with screenshot/console evidence, and updates/bump the IFC agent.

## Round 5

Codex verdict: **REVISE** (final round; MAX_ROUNDS reached).

Full critique:

1. **High — The model guard is both optional and fail-closed.** Define normal omission as backward-compatible, but require an authoritative QA flag/path pair whose missing or conflicting values fail inside the bake before mutation.
2. **High — The zero-frame algorithm is wrong for Y-up viewer scenes.** Derive the seed from declared scene-up (`+Z` or `+Y`), use `1-(n·up)² <= 1e-6`, and compare Y-up to an equivalent Z-up scene.

### Builder response

Both findings are accepted. The final candidate now distinguishes backward-compatible production omission from mandatory fail-closed QA mode, resolves request/environment conflicts before mutation, and defines the zero frame from the declared up vector with exact threshold and transformed-scene tests. Codex did not review a sixth time because the configured five-round hard cap was reached; human sign-off is required before implementation.
