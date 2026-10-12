# Plan Review Log: 694
MAX_ROUNDS=5
The plan leaves several concrete gate and consumer gaps.

1. **High — App-backed agents bypass the caller’s acknowledgment and markers.** [Synthesized commands](C:/Users/bimst/source/repos/aware-aeco-694/cli/src/manifest/expose.rs:57) carry neither model flags nor capabilities. An acknowledged reasoning app can therefore appear as an ordinary read command in an unacknowledged caller. Worse, [nested resolution](C:/Users/bimst/source/repos/aware-aeco-694/cli/src/agent_resolution.rs:590) and [dispatch](C:/Users/bimst/source/repos/aware-aeco-694/cli/src/runtime/invoker.rs:3174) do not run `validate_app_agents` on the backing app, allowing a legacy-compiled backing app to evade capability/ack checks.
   
   **Fix:** Validate every resolved backing app and propagate reasoning classification plus caller acknowledgment requirements, or prohibit exposing reasoning apps until that propagation exists.

2. **High — The older-CLI mitigation does not protect AWARE users.** An older CLI ignores the new manifest flags as well as lock fields, so it can itself compile and run an unacknowledged reasoning command; compile-time validation and an external FloLess version floor do not close this path.
   
   **Fix:** Require reasoning agents to declare the existing `requires-runtime` minimum for this feature, enforce that declaration, and test older-CLI compile/run rejection.

3. **High — Instructions are not necessarily pinned.** The contract permits `prompt: '{{ inputs.prompt }}'`, or an entire templated `config:` mapping, unless the command independently declares literal inputs. The source hash pins the reference, allowing instructions to change after approval. Existing [literal-input enforcement](C:/Users/bimst/source/repos/aware-aeco-694/cli/src/manifest/agent.rs:677) demonstrates the missing boundary.
   
   **Fix:** Require declared instruction/configuration inputs to be `literal: true` and test both individual-field and whole-mapping template bypasses.

4. **High — Frozen nodes can mint reasoning markers without acknowledgment or capability.** [Validation skips frozen subtrees](C:/Users/bimst/source/repos/aware-aeco-694/cli/src/validate.rs:1402), while [compilation flattens their bodies](C:/Users/bimst/source/repos/aware-aeco-694/cli/src/app_lock.rs:562) and derives model markers directly from command flags. Copying that pattern for reasoning stamps an unchecked frozen node—or a descendant of a frozen container—as runtime reasoning.
   
   **Fix:** Derive runtime reasoning markers from validated, dispatchable nodes only, with tests for frozen leaves and frozen ancestors containing incapable/unacknowledged reasoning commands.

5. **High — The proposed warning becomes a run-blocking error.** [Run preflight](C:/Users/bimst/source/repos/aware-aeco-694/cli/src/commands/app.rs:481) calls `.first()` on `validate_app_agents` and returns an error regardless of severity. Conversely, compilation filters errors and does not persist this new warning, so the lock can omit it.
   
   **Fix:** Audit all callers for severity handling, persist/display the warning, and add an integration test proving compilation surfaces it and running follows the chosen warning policy.

6. **High — `approve:` cannot currently provide the promised review gate.** [Runtime execution](C:/Users/bimst/source/repos/aware-aeco-694/cli/src/runtime/orchestrator.rs:1189) returns “not yet implemented” for `approve:`. A mixed `agent` + `approve` node executes the agent branch first; a frozen approval emits its pinned value. Suppressing the warning merely because an `approve:` field exists therefore accepts ineffective gates.
   
   **Fix:** Implement a real approval execution path before treating it as protection, and reject or exclude mixed/frozen approval shapes from gate recognition.

7. **Medium — The warning’s graph criterion misclassifies normal gated writes and underspecifies loops.** With `reason → approve → write` and `write.config` referencing `reason.field`, derived edges create a direct `reason → write` path without approval despite the approval prerequisite. Meanwhile, [loop bodies execute sequentially](C:/Users/bimst/source/repos/aware-aeco-694/cli/src/runtime/orchestrator.rs:1285), including dependencies conveyed through `item` and loop aggregates.
   
   **Fix:** Define protection using execution prerequisites and scoped data dependencies, and test direct references across approval gates, bypass diamonds, sequential bodies, `item`, and aggregate outputs.

8. **Medium — Dry-run reasoning outputs can be mislabeled deterministic.** Plain dry-run executes read commands live in both [ordinary execution](C:/Users/bimst/source/repos/aware-aeco-694/cli/src/runtime/orchestrator.rs:1078) and [stream propagation](C:/Users/bimst/source/repos/aware-aeco-694/cli/src/runtime/orchestrator.rs:716). A blanket preview-mode exclusion would hide a real model call. The proposed tests cover simulate but omit this distinction.
   
   **Fix:** Set the event marker from actual live dispatch, testing live dry-run reads, simulated reads, dry-run write stubs, frozen outputs, and streaming execution separately.

9. **Medium — Existing review consumers silently discard the new labels.** [Glass Box](C:/Users/bimst/source/repos/aware-aeco-694/cli/src/commands/app.rs:2742) renders mode/safety only, and [log replay](C:/Users/bimst/source/repos/aware-aeco-694/cli/src/commands/app.rs:1649) ignores additional `NodeOutput` fields through `..`. Adding serialized booleans alone does not make reasoning visible to reviewers.
   
   **Fix:** Add explicit Glass Box and replay labels, require `serde(default)` for historical traces/locks, and test rendered labels plus old-record deserialization.

10. **Medium — Candidate compilation can contradict the new lock-level marker.** [Candidate compilation](C:/Users/bimst/source/repos/aware-aeco-694/cli/src/app_lock/candidate.rs:196) compiles first, then restores frozen-agent nodes verbatim. A restored node can carry `runtime-reasoning: true` after the aggregate was computed as false. The listed migration test exercises only direct read eligibility.
    
    **Fix:** Recompute and verify aggregate markers after carry-over, and add candidate/promotion tests covering marker changes, digest sensitivity, frozen carry-over, and policy rejection under either pin set.

VERDICT: REVISE
### Claude response r1
Accepted 1,2,3,4,8,9,10 (see PLAN). 5,6,7 resolved by dropping the static approve warning: approve: is unimplemented at runtime so the check would be false assurance; (b) remains a documented default.
Remaining concrete gaps:

1. **High — The exposed-app ban still misses nested run preflight.** The ban lives in `validate_app_agents`, but [backing-app resolution](C:/Users/bimst/source/repos/aware-aeco-694/cli/src/agent_resolution.rs:590) and [nested dispatch](C:/Users/bimst/source/repos/aware-aeco-694/cli/src/runtime/invoker.rs:3174) never call it. Legacy backing locks can therefore reach reasoning commands without enforcing the new exposure, acknowledgment, or capability checks; the caller sees an unflagged synthesized command.

   **Fix:** Run structural and agent-aware validation on every resolved backing app before dispatch, with legacy-lock tests for exposure, missing acknowledgment, and missing capability.

2. **High — “At least one literal input” does not pin the instructions.** A command can declare `dummy: {literal: true}` while accepting `prompt: '{{ inputs.prompt }}'`. It can even omit `dummy`: [existing literal validation](C:/Users/bimst/source/repos/aware-aeco-694/cli/src/manifest/agent.rs:708) checks only supplied values. Both shapes satisfy the revised manifest fence while instructions remain mutable after approval.

   **Fix:** Identify instruction/configuration inputs explicitly, require those inputs to be literal and present, and test dummy-literal, omitted-instruction, and templated-prompt bypasses.

3. **Medium — The chosen runtime floor makes the planned implementation unrunnable.** The plan requires minimum CLI **0.159.0**, while [Cargo.toml](C:/Users/bimst/source/repos/aware-aeco-694/cli/Cargo.toml:3) builds **0.158.0**. No step updates the package version, so the existing runtime gate rejects every conforming reasoning agent during positive validation and E2E.

   **Fix:** Add the package-version and Cargo.lock updates to implementation scope and test admission using the actual newly built CLI.

4. **Medium — Streaming reasoning sources have a third output path.** The fence permits `lifecycle: start`, but the trace instructions name ordinary execution and downstream stream propagation. Live source events emit separately at [orchestrator.rs:368](C:/Users/bimst/source/repos/aware-aeco-694/cli/src/runtime/orchestrator.rs:368); simulated source events share that emission site.

   **Fix:** Include source-stream emissions in classification and test live versus simulated reasoning sources, or explicitly restrict reasoning commands to `lifecycle: single`.

5. **Medium — Legacy locks can authorize reasoning while displaying false markers.** [Version-only locks use the current same-version agent](C:/Users/bimst/source/repos/aware-aeco-694/cli/src/agent_resolution.rs:757). If that manifest gains reasoning declarations, an already acknowledged source can pass the new runtime fence while its old lock defaults both new markers to false. Glass Box then conceals the actual reasoning step.

   **Fix:** Compare live reasoning classification against the approved node markers and verify the aggregate marker at run preflight, requiring recompilation on disagreement.

6. **Medium — The test matrix does not protect several accepted fixes.** Removing every new Glass Box/replay label or removing `runtime-reasoning` from `plan_changes` would still satisfy the listed tests. Dry-run reads, streaming propagation, historical records, digest sensitivity, and migration policy rejection under either pin set also lack explicit coverage.

   **Fix:** Add targeted consumer, serialization, dispatch-path, migration-diff/digest, and promotion-policy regression tests.

7. **Low — Step 2 still mandates the removed approval warning.** [The steps](C:/Users/bimst/source/repos/aware-aeco-694/docs/superpowers/specs/2026-10-12-694-runtime-reasoning-node-PLAN.md:72) contradict the revised contract; the Risks section also retains the superseded compile-time/FloLess-floor mitigation.

   **Fix:** Remove the warning from Step 2 and rewrite the compatibility risk around the mandatory runtime floor and legacy-lock checks.

VERDICT: REVISE
### Claude response r2
Accepted all: explicit instructions-input, single lifecycle, runtime fail-closed + preflight marker check, plan text fixed, version-floor handling stated.
Four gaps remain:

1. **High — `required: true` does not enforce instruction presence in the app.** The revised fence checks the input declaration, but [existing literal validation](C:/Users/bimst/source/repos/aware-aeco-694/cli/src/manifest/agent.rs:702) only checks supplied values for templates. A valid manifest with a required literal `prompt` still passes that check when the node omits `prompt` or supplies null. The transport can then choose instructions outside the approved source.

   **Fix:** Explicitly require a non-null instruction value in merged node parameters during validation and live dispatch, and test omission using an otherwise valid manifest.

2. **High — Nested runtime defense still enforces only acknowledgment.** [The revised defense](C:/Users/bimst/source/repos/aware-aeco-694/docs/superpowers/specs/2026-10-12-694-runtime-reasoning-node-PLAN.md:64) does not enforce `reasoning_fence_gap`, `E_APP_REASONING_NOT_EXPOSABLE`, or the mixed-assert prohibition. [Nested dispatch](C:/Users/bimst/source/repos/aware-aeco-694/cli/src/runtime/invoker.rs:3174) currently checks only missing agents and pins. An acknowledged backing app with agreeing markers can therefore reach dispatch without proving those conditions; marker agreement is not admission validation.

   **Fix:** Run structural and agent-aware validation against every backing app’s resolved catalogue before dispatch, with nested tests for missing capability, exposure, and reasoning-plus-assert.

3. **Medium — Preflight marker comparison needs source-aware classification and an aggregate invariant.** Frozen reasoning nodes deliberately compile with false markers, but their manifests still declare reasoning; comparing every flat lock node directly against its manifest would reject valid frozen apps. Conversely, checking individual nodes alone accepts `nondeterministic-output: false` alongside a true node marker, suppressing the Glass Box banner.

   **Fix:** Share a source-aware classifier that excludes frozen ancestors, enforce aggregate equality at preflight, and test frozen leaves/bodies plus a contradictory aggregate marker.

4. **Medium — The temporary version bump leaves final-tree validation unresolved.** The plan tests E2E under an uncommitted **0.159.0** bump and then restores **0.158.0**. [Agent-aware validation](C:/Users/bimst/source/repos/aware-aeco-694/cli/src/validate.rs:1440) still checks the compiled package version, so the committed tree rejects every conforming reasoning agent. Separating the manifest predicate from version checks does not make the promised positive app-validation tests pass.

   **Fix:** Commit the matching package-version bump without publishing a release, then run positive validation and E2E against the final tree.

VERDICT: REVISE
### Claude response r3
Accepted 1,2,3 (shared reasoning_issues + classifier + nested + live gate). 4 declined with reason: a version bump is a release action the task forbids; below-floor dev trees refusing the agent is the intended behaviour; tests assert on specific codes and E2E uses a local uncommitted bump.
No remaining blocking findings. Round 3 closes the instruction-presence, nested-validation, and marker-consistency gaps.

The versioning rationale is acceptable within the no-release scope: below-floor builds intentionally refuse reasoning agents, and the E2E version override will be disclosed.

VERDICT: APPROVED