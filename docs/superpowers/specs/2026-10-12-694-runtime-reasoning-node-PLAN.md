# Plan: opt-in runtime reasoning node (aware-aeco/aware#694)

_Round 0 — initial draft_

## Goal
Admit ONE more fenced model-in-the-run-path carve-out beside RFC #223 `vision.extract`:
an *acknowledged, non-deterministic* AI reasoning step. AWARE stays model-free: it only
admits, marks and labels the node. Whatever agent transport serves the command (FloLess
runs the reasoning through the person's own provider) is out of AWARE's scope. "Build once,
run forever" holds: source hash pins the step (instructions/config), only its OUTPUT varies.
Everything else keeps `E_APP_RUNTIME_MODEL_FORBIDDEN`; the `vision.extract` carve-out is unchanged.

## Contract (new names)
Agent manifest:
- `capabilities.runtime-model-reasoning: true` (agent level)   -> `Capabilities.runtime_model_reasoning`
- command `model-reasoning: true` (+ `category: curated`)       -> `Command.model_reasoning` + `Command.model_reasoning_instructions`
App source:
- `nondeterministic-output: acknowledged` (top level). The only accepted forms are the string
  `acknowledged`, or a map whose `status` is `acknowledged` (optional `by`, `at`, free text
  supplied by the front door; part of the source hash). Anything else present => error.
Lock:
- node `runtime-reasoning: true` (skipped when false)  -- NEW BOOL, deliberately NOT a change to
  the existing bool `runtime-model` (that is already read by floless `app-reader`, the migration
  plan digest and effect model as a boolean meaning "extraction + model-pin"; making it a
  string would break those readers and turn `reasoning` truthy for extraction logic).
  `runtime-model` stays false on a reasoning node. `model-pin` is NOT stamped (FloLess picks
  the provider/model at run time; no model is pinned by AWARE).
- lock top-level `nondeterministic-output: true` (skipped when false) = at least one compiled
  node is `runtime-reasoning`. Derived from the nodes, not from the source ack.
Run record (JSONL trace):
- `node-output` event gains `nondeterministic: true` (skipped when false) when the node's
  output was produced live by a reasoning command. Set from the actual live dispatch site only (ordinary and stream-propagation paths, including a plain
  dry-run READ which really executes); never for `--simulate` stubs, dry-run write stubs or `frozen:` outputs.
Errors (validate; agent-level in `validate_agent`, app-level in the agent-aware pass `validate_app_agents`):
- ONE shared predicate `reasoning_fence_gap(manifest, command) -> Option<reason>` decides "admitted". A
  reasoning command is admitted only if ALL hold: `category: curated`; agent
  `capabilities.runtime-model-reasoning: true`; agent declares `status: requires-runtime` with
  `minimum-cli-version >= REASONING_MIN_CLI` (0.159.0 = the first release carrying this fence, so an older
  CLI that cannot honour the fence refuses the agent through the EXISTING runtime gate instead of running
  it as an ordinary node); the command names its instruction input explicitly with
  `model-reasoning-instructions: <input-name>` (new Command field), and that input exists, is `literal: true`
  AND `required: true` (the existing node-level `E_APP_INPUT_MUST_BE_LITERAL` check then forces the value to be
  written out in the source, so the source hash pins the step, not a `{{ inputs.x }}` reference; a dummy literal
  input or an omitted/templated instruction cannot satisfy it); `lifecycle: single` only (streaming sources
  have separate emission paths and are out of scope); and it does not also carry `model-extraction`.
  The predicate checks only what the manifest declares (not the running CLI version) so it is testable;
  the existing `runtime_requirement_error` gate separately refuses the agent on a CLI older than its floor.
  `validate_agent` reports the same gaps for a capability-claiming agent at install time
  (`E_AGENT_REASONING_FENCE`), and the node pass reports `E_APP_RUNTIME_MODEL_FORBIDDEN` (reused) for any
  non-admitted `model-reasoning` command. The extraction capability never confers reasoning or vice versa.
- `E_APP_NONDETERMINISTIC_UNACKNOWLEDGED` (new): a live (non-frozen) admitted reasoning node and no ack.
- `E_APP_NONDETERMINISTIC_ACK_MALFORMED` (new): `nondeterministic-output` present but not an accepted form.
- `E_APP_REASONING_AS_ASSERT` (new): a reasoning node that also declares `assert:`.
- `E_APP_REASONING_NOT_EXPOSABLE` (new): an `exposes-as-agent` app with a reasoning node. Synthesized
  agents carry no model flags, so the label and the caller's ack would be lost; refused until propagation
  exists (explicit carve-out, recorded in the spec).
- Frozen subtrees are skipped by validation (existing behaviour) AND never stamped at compile (marker comes
  from live nodes only; candidate carry-over recomputes the lock-level marker).
- NO static "downstream write must sit behind approve" check. Rationale (Codex r1): `approve:` is not yet
  executed by the runtime ("not yet implemented"), so a static check would certify an ineffective gate, and a
  data-edge heuristic mislabels `reason -> approve -> write`. Question (b) stays a spec-level default
  (front doors put the first write behind approval), exactly as for `vision.extract` today, flagged
  "pending owner confirmation". Reported to the owner.
Runtime defence in depth (covers nested/backing apps and legacy locks, Codex r2): (1) the orchestrator, at the
live dispatch of any `model-reasoning` command, fails closed with `E_APP_NONDETERMINISTIC_UNACKNOWLEDGED` if
the app it is executing lacks the ack; (2) run preflight compares each lock node's `runtime-reasoning` marker
with the live manifest classification and refuses a disagreement ("recompile") so a lock approved before the
command became reasoning cannot hide the label.
Consumers made visible (not just serialized): `aware app logs` replay prints a non-deterministic tag; the
Glass Box shows an "AI reasoning - non-deterministic" badge per node and a banner when the lock marker is set.
New fields use `serde(default)` so old traces/locks still load.
Other consumers kept honest:
- migration `NodeEffect.runtime_model` := extraction || reasoning (so a reasoning node is never
  "eligible read"); `plan_changes` also diffs `runtime-reasoning`.
- agent probe rejects `model-reasoning` commands like `model-extraction` ones.

## Open questions -> defaults (recorded "pending owner confirmation")
(a) reasoning output MAY feed control flow (predicate/for-each/assert expr reading it) because it is labelled.
(b) downstream host write sits behind `approve:` by default => spec-level guidance only; not statically enforced (see above).

## Steps
1. Manifest fields (agent.rs, app.rs). 2. validate.rs fence + ack + assert + exposable check (no approve warning).
3. app_lock.rs markers (+ every struct literal). 4. migration effect/plan + probe.
5. orchestrator/provenance `nondeterministic` on node-output. 6. Specs: 10-core/app-spec.md new
section "Runtime model reasoning (the second carve-out)", agent-spec.md capability/command field.
7. Tests (below). 8. Real E2E on temp AWARE_HOME with a throwaway `cli` transport test agent
(AWARE never ships a model-calling agent in this change). 9. fmt/clippy/test; Codex code review; PR; merge.

## Tests (each must fail when its code is reverted)
validate: admits curated+capable+acked; rejects non-curated; rejects capability missing;
rejects extraction-capability-only agent; rejects both flags; rejects missing ack; rejects malformed ack;
admits map-form ack; reasoning+assert rejected; not-exposable; floor/status/literal-input gaps each rejected; frozen leaf and frozen ancestor not stamped; candidate carry-over recomputes marker; existing RFC #223 tests untouched.
app_lock: compile stamps node + lock marker; ordinary and extraction nodes carry neither; extraction still `runtime-model`.
orchestrator: live run emits `nondeterministic:true`; ordinary node and simulate do not.
migration: reasoning node ineligible as read. probe rejects it. Every existing rejection test stays.

## Risks
- Execution of the node is the transport's concern: AWARE ships no reasoning agent here.
- Older AWARE: ignores unknown manifest/lock fields, so it would run a reasoning command as an ordinary node.
  Mitigated by the mandatory `requires-runtime` floor (REASONING_MIN_CLI = 0.159.0, the next minor; the dev
  build is 0.158.0, so E2E uses a LOCAL uncommitted Cargo version bump that is reverted before commit; this
  change does not bump or release). If the release is cut as a different version the constant must match.
- Hash: ack is in source => ack change re-invalidates approval. Intended.

## Out of scope
FloLess side, a shipped reasoning agent, release/tag, replay/caching of reasoning output.

## Extra tests (r2)
dummy-literal, omitted-instruction and non-literal-instruction commands rejected; streaming lifecycle rejected;
orchestrator fail-closed without ack; preflight refuses lock/manifest marker disagreement; Glass Box badge + banner
and replay tag present (assert on rendered strings); `plan_changes` reports a `runtime-reasoning` change;
old trace/lock JSON/YAML without the new fields still deserialises; dry-run read vs simulate vs frozen labelling.

## Round 3 amendments
- ONE shared function `reasoning_issues(app, agents) -> Vec<ValidationIssue>` (validate.rs) holds EVERY node-level
  reasoning rule: fence gap, non-null instruction value present in merged params (omitted/null => error
  `E_APP_REASONING_INSTRUCTIONS_MISSING`), ack, assert-mix, not-exposable. It is called (a) from
  `validate_app_agents`, (b) from the nested backing-app resolution in `runtime/invoker.rs` (non-simulate, against the
  backing app's OWN resolved catalogue; only these reasoning issues, so unrelated pre-existing behaviour is unchanged),
  and (c) the orchestrator's live dispatch gate re-checks the single node (fence + ack + instruction) fail-closed.
- ONE source-aware classifier `app_lock::reasoning_node_ids(app, agents) -> BTreeSet<scoped id>` (live nodes only:
  frozen leaves and nodes under a frozen ancestor excluded). Compile stamps from it; run preflight requires
  lock `runtime-reasoning` == classifier per node AND lock `nondeterministic-output` == any(node marker) (aggregate
  invariant, both directions). Candidate carry-over recomputes the aggregate then re-checks the invariant.
- Version: v0.158.0 is released; REASONING_MIN_CLI=0.159.0. A dev tree still at 0.158.0 therefore refuses reasoning
  agents via the existing runtime gate until the release that carries the feature - the intended behaviour, not a
  defect. Bumping Cargo.toml is a release action and is out of scope (task: no release). Unit tests assert on the
  specific codes (not "no errors") and test the gate with `runtime_requirement_error(.., "0.158.0"/"0.159.0")`;
  E2E runs against a binary built with an uncommitted local 0.159.0 bump (reverted before commit, stated in the PR).

## Owner update 2026-10-12
Both defaults CONFIRMED as owner rulings: (a) output may feed control flow; (b) downstream host write behind `approve:` by default.
(b) enforcement stays guidance-only: decided with Codex (plan review rounds 1-3), owner to review - `approve:` does not execute yet.
