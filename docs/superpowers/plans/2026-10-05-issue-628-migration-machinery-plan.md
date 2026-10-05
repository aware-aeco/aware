# Plan — aware-aeco/aware#628: workflow migration machinery

_Round 0 — drafted 2026-10-05 on top of #626 (v0.149.0). Binding owner decision: pawellisowski/floless.app#1985._

## 0. Facts from the code that shape the design

1. **Executable code mostly isn't in the agent package.** CLI agents spawn a bridge binary owned by the CLI version (`resolve_cli_binary`, `runtime/invoker.rs`); no package root is passed. REST/builtin agents are executed by AWARE's own code from manifest fields. Package files are executable only where a helper reads them from the resolved root (e.g. `render/blender.rs` scripts). So a pin's executable contract = run-relevant manifest projection + executable package files + executor identity.
2. **There is no fixed state in the runtime today.** `snapshot` / `model-lock` primitives parse but `orchestrator.rs` returns "runtime execution lands in v0.19.x patches". No HTTP record/replay. Tekla runs against the live model.
3. **Generated-tool `mode` is defaulted, not declared** (`manifest/expose.rs` `exposed_mode` writes `mode: read` when the author wrote nothing).
4. **Tekla 0.1.5 declares almost no read modes**; `exec` is `mode: write` + `mode-overridable: true`. So no real Tekla app qualifies for the no-click path today; the Tekla E2E exercises candidate + click.
5. `LockFile` has no `deny_unknown_fields`, so older AWARE ignores new keys. FloLess authority checks (`rfi-delivery.ts::lockMatchesRuntime`, `tekla-parser-descendant-authority.ts::executableLock`) read top-level `agent-pins`/`agent-bundle-pins` and a canonical lock projection.
6. `write_lockfile` is `std::fs::write` (not atomic) — promotion needs atomic CAS replace.
7. #628 needs only #626 (store immutable, promotion deletes nothing). #627 leases are needed before #629 GC, not before #628.

## 1. Candidate compilation

- CLI: `aware app migrate prepare <app> [--to <agent>@sha256:<hex> | <agent>@<version>]... --json`. Default target per agent = current working copy digest when it differs from the approved pin. `<agent>@<version>` must map to exactly one stored digest (`E_MIGRATE_TARGET_AMBIGUOUS`).
- Location: `<source-dir>/.aware-migration/<app>.candidate.lock` + `<app>.evidence.json` (one candidate per app; re-prepare replaces an unpromoted one).
- `agent_resolution::resolve_pins(paths, app, pins: &PinSet) -> Result<Vec<DiscoveredAgent>>`: side-effect free; target agents use the target digest, every other agent uses the base lock's own digest (a Tekla candidate never drags in a newer google-workspace); frozen-only agents keep base pins (base compiled node copied verbatim if its package isn't stored).
- `app_lock::compile_candidate(source, paths, base, base_bytes, targets) -> (LockFile, CandidateHeader)`: reuses `read_source_snapshot`, `validate_app`, `validate_app_agents`, `unsatisfied_pins`, `compile_snapshot`; adds `validate_app_safety` against the candidate catalogue (a node becoming write with no `safety:` → `blocked`); unsatisfied `requires:` → `blocked(needs-source-edit)`. Header: `base-lock-digest`, `base-source-hash`, `targets`, `candidate-digest`.
- `write_lockfile` becomes atomic same-dir temp + fsync + rename (Windows `MoveFileExW(REPLACE_EXISTING|WRITE_THROUGH)`).
- `app run` is unchanged: it only reads `<app>.lock`. Approved vs promoted is decided by promotion atomically replacing `<app>.lock` (§5). The #626 resolver already runs any stored digest.
- Acceptance: `<app>.lock` sha256 + mtime unchanged by `prepare`.

## 2. Executable-contract comparison (`migration/contract.rs`)

`diff(old, new, called_commands, executor) -> ContractDiff`, computed on the **raw manifest YAML value** (unknown keys compared by default = fail safe):
- Agent level: everything except an ignore-list with no run-path effect: `version, display-name, description, keywords, homepage, vendor, license, provenance, skills, probe`. `probe` reported as `probe-changed` but not counted (`app run` never runs the probe).
- Called commands (non-frozen nodes): full command entry canonical JSON minus top-level `description` (schemas, `mode`, `mode-overridable`, `lifecycle`, `status`, `category`, `method`, `path`, `response`, `no-auth`, `model-extraction`).
- Executable files: sha256 of every package file except a doc allowlist (`skills/**`, `commands/**/*.md`, `CHANGELOG.md`, `README*`, `LICENSE*`, `.aware-install.yaml`, `.aware-package.yaml`); `manifest.yaml` covered by the projections; `atoms/` counted.
- Executor identity: CLI → resolved program path + binary sha256; REST/builtin → `aware-cli <version>`; app transport → n/a. Same executor for both pins at evaluation time; recorded as evidence.
- Compiled-node differences between base and candidate locks (`mode`, `output-schema`, `runtime-model`, `model-pin`) → `plan-changes`.
- Output `aware.contract-diff/v1` per moved agent: `{agent, from, to, unchanged, agent-level.changed[], commands[{command, nodes, unchanged, changes}], executable-files{added,removed,changed}, probe-changed, executor, plan-changes, ignored{doc-files, commands-not-called}}`.
- `unchanged` ⇔ agent-level empty ∧ all called commands unchanged ∧ executable files unchanged ∧ plan-changes empty.
- Guard test (source scan + negative control): no ignored field is read under `runtime/` except `runtime/probe.rs`.

## 3. Declared effect

- `Agent::mode_basis(name, cmd, node_mode) -> ModeBasis { Declared, OverridableDefault, NodeOverride, InferredName, FallbackWrite }`. No new manifest key — an explicit non-overridable `mode:` IS the declaration.
- Describe rows gain `mode`, `mode-basis` (`declared|inferred`), `mode-overridable`.
- Generated tools: manifest mode never treated as declared. `migration/effect.rs::wrapper_effect` = max effect over the backing app's dispatchable nodes judged against its own resolved pins; author-written `mode: write` → write. Describe shows `mode-basis: inherited`, `inherited-from`.
- Node is policy-eligible read iff: basis `Declared` read in BOTH old and new pins, or inline/assert/compare, or a wrapper whose inherited effect is read and whose pin doesn't move; AND not `runtime-model`; AND not `sweep/approve/snapshot/model-lock`.
- Workflow is read-only iff EVERY dispatchable node is eligible read (stricter than "every command on that tool").

## 4. Fixed-state comparison (`migration/compare.rs`)

`Comparison { status: Pass | Fail | NotComparable, method, runs, reason, per_agent }`; NotComparable is first-class and always routes to a person.
- Feasible now — `invocation-identical` (executes nothing): when the contract diff is unchanged, both pins hand the same executor byte-identical instructions (same manifest projection, executable files, executor file, inputs), so results are equal on every state. `pass, runs: 0`, evidence says so.
- Contract changed: CLI/host (Tekla live) → `not-comparable: no-fixed-state (host state is live; snapshot primitive not implemented)`; REST → `not-comparable: no-recorded-exchange`; app transport → `not-comparable: nested-generated-tool`. Two live reads never used.
- Deferred (follow-up issue / PR4): REST record-once/replay through loopback; isolated-copy harness for file-state CLI agents; implement the `snapshot` primitive.

## 5. Successor approval records

- Effective runnable plan at the **top level** of `<app>.lock`; the chain in a new `approval:` block. (Rejected: keeping the original top level + `successors:`, or a separate successor lock — FloLess `lockMatchesRuntime` would keep claiming the old pins while AWARE runs new ones. Top-level pins make pinned-value readers fail closed.)
- Original never overwritten: exact bytes archived content-addressed (`.aware-approvals/<hex>.lock`), approval fields copied verbatim into `approval.original`; successors append-only:

```yaml
approval:
  format: 1
  original: { lock-digest, archive, compiled-at, compiler-version, agent-pins, agent-digests, agent-bundle-pins }
  successors:
    - seq: 1
      kind: carried-forward            # | reverted
      from-lock-digest: sha256:…       # CAS: bytes of the lock this replaced
      to-plan-digest: sha256:…         # app_lock::plan_digest(top level)
      from: { tekla: { version: 0.1.5, digest: sha256:A } }
      to:   { tekla: { version: 0.1.6, digest: sha256:B } }
      carried-forward-by: { kind: person, actor, approval-ref }
                       #  or { kind: policy, policy-id, policy-digest, policy-approved-by }
      evidence-digest: sha256:… ; evidence: .aware-approvals/<hex>.json
      front-door: floless@x.y.z ; cli-version ; promoted-at
```

- `LockFile.approval: Option<ApprovalChain>` (skip if none); `plan_digest(&LockFile)` = canonical JSON minus `compiled-at`, `compiler-version`, `approval` (mirrors FloLess `executableLock`; confirm serde_json `preserve_order` off).
- `check_lock_consistency` extended → `E_APP_LOCK_INVALID` on: unknown `format`; non-contiguous `seq`; seq1.from ≠ original pins or from ≠ previous to; last `to` ≠ top-level pins/digests for moved agents; `to-plan-digest` ≠ `plan_digest(lock)`. Missing archive reported by `app check` (`approval-archive: missing`), doesn't block.
- Reporting: `app check` gains `approval-origin: original|successor` + `successors[]`; run record `run_config.approval {origin, seq, by-kind, policy-id, from, to, evidence-digest}`; nested entries carry the backing app's origin. Text: "approved by a person on <date>; carried forward 0.1.5 → 0.1.6 under policy pol-… (read-only, same instructions)".
- Older AWARE: 0.149 ignores `approval`, runs the top-level (promoted) pins unlabelled; ≤0.148 refuses unless current == new pin. FloLess must require CLI ≥ #628's version before calling `migrate`. Test: frozen copy of the 0.148 `LockFile` struct deserializes a promoted lock.
- `aware app compile` by a person writes a fresh original (no `approval` block).

## 6. Policy

- Immutable `AWARE_HOME/migration-policies/<policy-id>.yaml` (digest = sha256 of bytes); revocation appends `<id>.revoked.yaml`.
  `{policy, format: 1, rule: read-only-patch (closed enum; delegated-maintenance NOT accepted in v1), scope{apps, agents, publishers:[official-registry], bump: patch}, approved-by{actor, front-door, statement-sha256, at}}`. The UI that asks is FloLess's.
- Promotion requires exactly one of `--person <actor> --approval-ref <ref>` or `--policy <id>`; neither → `E_MIGRATE_NO_APPROVER`.
- Policy promotion re-evaluates from scratch: policy exists, not revoked, digest matches; app+agents in scope; bump is `patch` (semver; non-semver ineligible); `to` package verified official against a fresh official index (offline → ineligible); workflow read-only (§3); contract unchanged; comparison pass; app not held, not `exposes-as-agent`/backing, no `runtime-model` node. Any miss → `E_MIGRATE_NOT_ELIGIBLE` with reasons.
- Urgency never approval: reserved `advisory` field; test asserts it never changes `state`.
- Sealed/certified/frozen: `aware app migrate hold|unhold <app> --actor …` (`.aware-approvals/HOLD`); both promotion paths refuse `E_MIGRATE_HELD`.
- Active runs never move (lock read once at preflight; promotion is a rename); output lists `running-instances` from pidfiles.
- No automatic rollback: `migrate revert` appends `kind: reverted`; `--automatic --reason run-failed:<run-id>` allowed only back to the original's pins or a person-carried successor's. Never claims effects were undone.

## 7. CLI verbs (`AppCommand::Migrate`, JSON envelopes; expected states are data, `ok:false` only for errors)

| Verb | Writes |
|---|---|
| `migrate plan [--app …]\|--all [--to …] --json` | nothing; per app `state: up-to-date\|auto-under-policy\|needs-person\|blocked\|held`, targets, effect + effect-detail, contract, comparison, reasons, policy, candidate, running-instances, advisory |
| `migrate prepare <app> [--to …] --json` | candidate + evidence |
| `migrate promote <app> --candidate sha256:… (--person … --approval-ref … \| --policy …) [--front-door …] --json` | archives outgoing lock + evidence, CAS-replaces `<app>.lock` |
| `migrate revert <app> (--person … \| --automatic --reason …) --json` | appends `reverted` successor |
| `migrate discard <app>`, `migrate hold\|unhold <app>` | candidate dir / HOLD |
| `migrate policy record\|list\|revoke --json` | `migration-policies/` |

Promotion: exclusive create-new sentinel `.aware-approvals/.promote.lock` (shared with atomic `write_lockfile`) → verify candidate bytes == `--candidate`, `base-lock-digest` == current lock bytes, source hash, target packages verify (`E_MIGRATE_CANDIDATE_STALE` / `_TAMPERED`) → write + fsync archives → rename. Crash ⇒ old lock or full promotion.

FloLess: after a background `aware agent update` → `plan --all` → `prepare` each affected app → `promote --policy` for `auto-under-policy` → "Approve updated workflows" = `promote --person <user> --approval-ref <batch-id>` per selected app (per-app atomic, batch not transactional) → first-approval policy = `policy record`.

## 8. Tests

Unit: mode_basis; wrapper inherited effect; contract diff cases (description-only → unchanged; schema/overridable/method/path/auth/unknown key/new script → changed; doc files ignored; uncalled ignored; probe → probe-changed only); resolve_pins (non-targets keep base digests; no writes; ambiguity refused); compile_candidate (approved bytes identical; header; blocked cases); plan_digest stability; chain consistency breaks; policy eligibility truth table (each conjunct alone flips; advisory never changes verdict; offline ineligible); promotion CAS/tamper/no approver/both approvers/held/revoked/out-of-scope + fault injection at archive and rename → lock byte-identical; revert rules; legacy 0.148 struct reads a promoted lock; guard test with negative control.
Integration (`tests/app_migrate.rs`, echo fixture 1.0.0→1.0.1): prepare → lock identical → run prints 1.0.0 → promote --person → run prints 1.0.1 → check `approval-origin: successor`, `approval.original` == archived bytes; run record has `approval`; `app compile` resets to original; promoted backing app changes its caller's run and the caller's record shows the nested successor.
Real E2E (temp home, real registry + bridge): tekla 0.1.5 → model-free exec app (`mode: read`) → update to 0.1.6 → `plan`: exec contract unchanged, probe-changed, comparison pass/invocation-identical, state needs-person (reason mode-overridable) → `prepare` (lock sha unchanged; run still digest A) → `promote --policy` refused `E_MIGRATE_NOT_ELIGIBLE` → `promote --person e2e` → run on 0.1.6, archive hash == pre-promotion hash → `revert --automatic` → 0.1.5, seq 2. No-click path: fixture/unit only (no official agent has a declared-read patch move today).

## 9. PR decomposition

1. Read-only plumbing: declared effect in describe, `migration::{effect, contract}`, `resolve_pins`, atomic `write_lockfile`, guard test, spec text.
2. Candidate compile + comparison + `migrate plan/prepare/discard/hold` (writes only `.aware-migration/`).
3. Lock schema: `approval` chain, consistency, archive, `promote/revert`, policy store, `app check` + run-record labels, legacy-read test, spec. (Determinism-gate change.)
4. (Follow-up issue) executed differential: REST record/replay, isolated-copy harness, `snapshot` primitive.

## 10. Risks / open questions

- R1 "The AI never approves" can't be enforced inside the CLI — any same-user process can pass `--person`. AWARE records the claim, refuses with no approver, and requires a person-approved policy for no-click. Options: front-door attestation key; refuse these verbs under an AI-session env marker.
- Q1 Does `invocation-identical` (proof for all states, runs 0) satisfy "same results on fixed state"? If not, no-click is disabled until PR4.
- Q2 Effective pins at top level vs the issue's literal "append"; FloLess `executableLock`/`lockMatchesRuntime` must learn `approval.successors` (FloLess follow-up).
- Q3 `probe` excluded from "unchanged".
- Q4 Workflow-level read-only stricter than per-tool.
- Q5 A promoted backing app silently changes callers; v1 never auto-promotes `exposes-as-agent` apps; should promotion append successors to callers?
- Q6 Tekla should declare `mode:` explicitly on reads; `launch` wrongly inferred read (separate agent issue).
- R2 Older AWARE runs a promoted lock unlabelled — FloLess must version-gate.
- R3 Missing archive warns, doesn't refuse.
- R4 Trusted-publisher check needs network at plan/promote; offline → person.

## 11. Rulings after second-model review (Fable, 2026-10-05) — these override the sections above where they conflict

- **Q1 → `identical-instructions` is a distinct comparison status, never `pass`.** `Comparison.status` gains `IdenticalInstructions` (method `static-inspection`, nothing executed). Labels say exactly what was checked: "carried forward 0.1.4 -> 0.1.6 - read-only; the tool's run instructions are byte-identical (checked by inspection, nothing was run)". Never "same results", never "unchanged behaviour", never "passed 0 comparisons". **Policy eligibility in v1 requires `status == Pass` from an executed fixed-state comparison; `IdenticalInstructions` does NOT qualify until the owner ratifies it** (one-line question to the owner in the run report). No executed method exists in v1, so the no-click path is implemented and unit-tested but dormant in practice; zero real cost today (fact 4). Enabling it after ratification = adding `IdenticalInstructions` to one accepted-methods constant, with a test.
- **Q2 → keep effective pins at top level + `approval:` chain.** Correction: `plan_digest` does NOT mirror FloLess `executableLock` (which canonicalises the whole lock minus only `compiled-at`/`compiler-version`, and refuses unknown keys). Consequence, stated in the spec: a promoted lock is refused by FloLess's descendant-authority checks (fail-closed, correct for sealed descendants). FloLess follow-up: hold its owned internal apps (rfi-gmail, revit-model, tekla-parser) by default and teach `executableLock`/`lockMatchesRuntime` about `approval.successors`. Spec wording: "the lock FILE is replaced; the original approval RECORD is preserved (archived bytes + verbatim `approval.original`) and never rewritten".
- **R1 → `--person` requires BOTH `--approval-ref` and `--front-door`** (required, not optional); recorded as a claim: `carried-forward-by: {kind: person, actor, approval-ref, front-door, attested: false}`, rendered "approved by <actor>, recorded by <front-door>". Tripwire (accident guard, not an adversary guard, and said so in the spec): refuse `migrate promote --person` and `migrate policy record` with `E_MIGRATE_NO_FRONT_DOOR` when an AI-session env marker (`CLAUDECODE`, `CODEX_SANDBOX`, `AWARE_AI_SESSION`) is set AND `--front-door` is absent. `aware app compile` records the invoking front-door too when given. Attestation keys are later work.
- **Q5 → list, mark, ask.** Policy path never promotes `exposes-as-agent`/backing apps (kept). `migrate plan` lists each caller of a moving backing app with state `needs-person` reason `backing-app-moved`; person `promote` of a backing app warns and proceeds; callers are never given successor records by proxy — each caller gets its own candidate + successor only when a person approves it (FloLess batch, same approval-ref). The caller's run record carries the nested origin (§5).
- **Revert:** AWARE never triggers `revert --automatic` itself; only a caller (FloLess policy) invokes it; the record never claims effects were undone.
- **R2 sequencing:** the FloLess CLI-version gate + hold-list must ship before the PR that makes promotion possible is released.
- **PR split (revised):** PR1 plumbing (effect, contract, resolve_pins, atomic write_lockfile, guard). PR2 candidate + comparison + `plan/prepare/discard/hold`. **PR3a** lock schema `approval` chain + consistency + archive format + `app check`/run-record labels + legacy-read test (no way to promote yet). **PR3b** `promote`/`revert`/policy verbs. PR4 (follow-up issue) executed differential.

## 12. Round-1 Codex revisions (override earlier sections where they conflict)

- **Approval bound to exact bytes (R1-1, R1-3 accepted in part).** A person promotion takes `--approval <file>` instead of a bare `--approval-ref`: a JSON record written by the front door `{format:1, kind:"person", actor, front-door, approval-ref, candidate-digest, base-lock-digest, plan-digest, statement-sha256, at}`. AWARE refuses (`E_MIGRATE_APPROVAL_MISMATCH`) unless `candidate-digest`, `base-lock-digest` and `plan-digest` equal the candidate being promoted; the record is archived content-addressed and its digest stored in the successor. Same shape for `policy record` (`statement-sha256` of the exact plain-English policy text shown). **Rejected: cryptographic verification of the person** - the CLI cannot prove a human clicked; that boundary is the front door's. The record says so: `attested: false`, label "claimed person approval, recorded by <front-door>". The AI-session tripwire (refuse when an AI-session marker is set and no `--front-door`) remains an accident guard only, documented as such.
- **`app compile` (R1-2) - out of scope, unchanged.** Compile is today's person-approval act, gated by the FloLess Compile/Approve button; #628 does not change who may compile. The successor machinery never weakens it (a compile always writes a fresh original). Recorded as a known pre-existing boundary; front-door recorded on compile when provided.
- **Promotion revalidates the plan (R1-4 accepted).** Under the promotion sentinel AWARE re-runs `compile_candidate` from the current source + the base lock + the recorded targets and requires its `plan_digest` to equal the candidate's; it also recomputes effect, contract and comparison, and for `--policy` the full eligibility (as §6 already says). Any difference → `E_MIGRATE_CANDIDATE_STALE`; the candidate file is evidence, never the authority.
- **Missing/mismatched original archive (R1-5 accepted for promotion, narrowed for run).** Promotion and revert refuse (`E_MIGRATE_ARCHIVE_INVALID`) if any archive the chain cites is missing or its bytes don't hash to the recorded digest. A run of an already-promoted lock does NOT refuse (repo rule "nothing refuses, it asks"; the run's bytes are still exactly the promoted, digest-pinned packages); instead `app check` reports `approval-record: incomplete` with the missing items and the run record carries `approval.record-complete: false` and the label "carried forward - the original approval record is missing, provenance cannot be shown". FloLess shows it as a warning with "Compile again" as the fix.
- **Full chain verification (R1-6 accepted).** Consistency checks every link: `seq` contiguous; link 1 `from` pin maps == `approval.original` maps (full maps, not only moved agents); each link's `from-lock-digest` == digest of the archived lock it replaced (when the archive exists); each `evidence-digest` == archived evidence bytes; each link's `to` full pin maps == the next link's `from`; last link `to` full maps == top-level `agent-pins`/`agent-digests`/`agent-bundle-pins`; `to-plan-digest` == `plan_digest(lock)`. Missing archives degrade to `approval-record: incomplete` (above), never to a silent pass.
- **Crash recovery (R1-7 accepted).** Transaction dir `.aware-approvals/.txn/<txn-id>/` holding `intent.json` {base-lock-digest, new-lock-digest, archives[]}, staged archives and the staged new lock, all fsynced. Sentinel `.aware-approvals/.promote.lock` is an OS lock (`fs2` `try_lock_exclusive`, released on process death) - no stale-file problem. Order: stage+fsync all files in `.txn` -> rename archives into place (idempotent; content-addressed) -> `MoveFileExW(REPLACE_EXISTING|WRITE_THROUGH)` staged lock over `<app>.lock` (same directory: `.txn` lives under the app dir) -> delete `.txn`. Recovery at the start of every `migrate` verb and `app check`: a leftover `.txn` whose `new-lock-digest` == current lock bytes -> finish (archives already in place, delete txn); else -> roll back (delete txn; current lock untouched). Fault-injection tests after each step.
- **Honest labels (R1-8 accepted).** "claimed person approval" / "under policy <id> (approved by <actor>, claimed)"; effect wording is always "declared read-only" (from manifest declarations, never observed); comparison wording per §11 Q1.
- **PR2 states (R1-9 accepted).** In v1 every real candidate reports `needs-person`; `auto-under-policy` is reachable only when an accepted comparison method exists (none in v1; owner ratification of `identical-instructions` or PR4). Plan output carries `no-click-available: false` + reason "no fixed-state comparison method is available yet".
- **Release gate (R1-10 accepted).** PR3b (the only PR that can promote) is not released until the FloLess CLI-version gate + hold-list of its owned internal apps has merged; PR3b's PR body and the release step check it. PR3a is safe alone (nothing can create a successor without PR3b).

## 13. Round-2 Codex revisions

- **Backing-app callers move only with their own approval, atomically (R2-2 accepted).** Promoting an `exposes-as-agent` backing app requires every caller app that resolves it (found by scanning installed apps' locks for app-transport pins on it) to be either on hold or included in the SAME promotion transaction with its own candidate + approval record (`migrate promote <backing> --with-callers <app>... --approval <file>...`). Otherwise the CLI returns `E_MIGRATE_CALLERS_UNAPPROVED` with the caller list and changes nothing (an ask: the front door shows the list and asks the person). The multi-app transaction uses the §12 txn protocol with one intent covering every lock; recovery finishes or rolls back all of them together.
- **Each link verified against its own result (R2-3 accepted).** Every promotion archives the RESULTING lock too (`.aware-approvals/<hex>.lock`, content-addressed). Link k: `to-plan-digest` == `plan_digest(archived resulting lock k)`, its `to` maps == that lock's top-level maps; only the LAST link is compared with the current `<app>.lock` (whose digest must equal link-last's resulting-lock digest). Missing archives → `approval-record: incomplete` (§12).
- **Older CLIs fail closed on a promoted lock (R2-4 accepted).** A promoted lock writes `source-hash: "successor-v1:<sha256-hex>"`. AWARE ≤ 0.149 compares it to the raw source hash, so it refuses with `E_APP_LOCK_STALE` ("compile again") — never runs new pins unlabelled. The new CLI accepts the prefix only when an `approval:` chain with ≥1 successor is present and verifies the hex part as the source hash. FloLess's Run gate / drift check must learn the prefix (part of the FloLess gate PR that is PR3b's release prerequisite); until then FloLess shows a promoted lock as "Needs compile" — fail-closed. Test: a frozen 0.149 load path refuses a promoted lock with `E_APP_LOCK_STALE`.
- **Person attestation (R2-1) — REJECTED again, logged as an open disagreement for the owner.** Any key or attestation the CLI can verify is readable by every same-user process, including an AI agent, so a "verifiable front-door attestation" would be false assurance — a worse lie than an honest `attested: false` claim. The enforceable boundary is the front door (the FloLess click), plus: exact-digest binding (§12), the AI-session tripwire (accident guard), and labels that say "claimed". If the owner wants a hard technical boundary, it needs an OS-level separation (a FloLess service account or an OS credential prompt) — out of scope for #628, would be its own issue.

## 14. Round-3 Codex revisions — scope cut for backing apps

- **v1 never promotes an `exposes-as-agent` backing app, by either path (R3-1, R3-2 resolved by removal).** §13's multi-app transaction and the caller-hold rule are deleted. `migrate prepare/promote` on a backing app, or on any app whose candidate would move a backing app's pins, returns `E_MIGRATE_BACKING_APP` with the caller list and changes nothing; `migrate plan` reports such apps (and every caller) as `needs-person` with reason `backing-app-moved: compile the backing app and its callers again`. That is the existing person path (`aware app compile`, the FloLess Compile/Approve button), so it is an ask, not a dead end. Single-app promotion remains atomic to runners: it is one `MoveFileExW(REPLACE_EXISTING)` of one lock, which `app run` reads once at preflight (old or new, never half). Multi-app atomic promotion for backing apps → follow-up issue.
- `app run` still runs §12 recovery-free: a leftover `.txn` never affects which lock a run reads (the lock file is either the old or the fully-written new one); recovery only tidies archives.
- **Attestation (R2-1/R3) — open owner decision**, carried to the run report verbatim: "Accept the front door (the FloLess click) as the trust boundary for 'the AI never approves', with exact-digest binding, an AI-session accident guard and 'claimed' labels — or require an OS-level separated boundary before person promotion ships." PR3b (the only PR that can promote) does not merge until the owner answers; PR1, PR2, PR3a are unaffected.
