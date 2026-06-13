# RFC — `vision.extract`: a curated, schema-bound runtime extraction agent

> **Status:** Proposal / RFC — 2026-06-13. This is **not a bug report**: AWARE rejecting a
> runtime LLM read today is the substrate working *as designed* (decalog #9). This RFC asks
> whether a single, narrowly-scoped, structurally-fenced exception is worth adding.
> **Maintainers may legitimately decline** — the requesting front door (floless.app) already has
> a shipping fallback (see §9), so nothing is blocked on a "yes".
>
> **Origin:** floless.app's "Visual Inputs" design
> (`floless.app/docs/superpowers/specs/2026-06-12-visual-inputs-design.md`), strategy **B2**.
> Follow-up to **#101 §4** ("Runtime LLM extraction is NOT a request — it's decalog #9 working"),
> which blessed *compose-time* extraction + `approve:` as the answer to "I have a drawing to read."
> This RFC revisits the **one** cell that compose-time extraction cannot serve at zero friction:
> *swap a new image per run, when the read genuinely needs vision.*

---

## 1. The axiom this touches — and the precise line it draws

AWARE forbids an LLM in the **run path**. The canonical statements:

- **Decalog #9** (`00-vision/decalog.md:57–74`): *"AI composes the plan; deterministic code is
  the plan… The LLM can compose the plan. It cannot **be** the plan."* And, structurally:
  *"Validators (`assert:` nodes) must be deterministic… **No `think-node` / `smart-node`
  allowed.**"* (`decalog.md:71`)
- **app-spec § "Reading a drawing / PDF / image (compose-time extraction)"**
  (`10-core/app-spec.md:322–346`). The closing sentence (`app-spec.md:346`):
  > *"What this is **not**: a runtime `think-node` that reads the drawing while the app executes.
  > `aware app validate` rejects that for the same structural reason it rejects an LLM `assert:` —
  > the run path is deterministic, so front doors should extract up front (optionally behind
  > `approve:`), not defer reading to execution."*

The crux of this RFC: **the axiom is "no model *judgment* in the run path," not "no *reading*."**
Determinism in AWARE means *the approved `.lock` fully determines behavior* — a human reviews the
plan once, it runs forever. An LLM in the run path breaks three things:

1. **Reproducibility** — same lock + same inputs must give the same outputs. A free LLM call does not.
2. **Reviewability** — you approve a concrete plan, not "…and then a model does something."
3. **Boundedness** — a model that reads-*and-decides* can author new behavior mid-run (branch
   control flow, synthesize an action), escaping the reviewed topology.

**The tell that this is about judgment, not reading:** deterministic parsers *already run at
runtime and are allowed.* A read-only `exec` node (`mode: read`, no `safety:` — `app-spec.md:729–746`,
issue #165) can run PdfPig / OCR / a barcode decoder against bytes that vary per run, exactly like
live Tekla model state varies per run. That is legitimate I/O, not non-determinism. It is permitted
because it is **reproducible and inspectable**. So the real line AWARE draws is *model judgment* —
and that is the only line `vision.extract` proposes to cross.

## 2. Two verified facts this RFC rests on (checked against the repo at 2026-06-13)

1. **The validator rejects a runtime LLM read.** `app-spec.md:346` (quoted above), reinforced by
   decalog #9. In the Rust validator, the structural enforcement is per-node and per-command:
   `cli/src/validate.rs` `check_inline_nodes` rejects any inline-glue `kind` other than `predicate`
   with `E_APP_INLINE_KIND` (`validate.rs:227–235`), and `validate_app_agents` /
   `validate_app_safety` resolve every node's `agent`+`command` against the installed catalogue
   for mode/availability (`validate.rs:254–304`). There is **no agent+command pair today that a
   node could name to perform a model read** — which is the second fact:
2. **There is no vision/LLM/think/multimodal agent anywhere in the catalogue.** Every agent under
   `20-agents/` is a host integration or a deterministic utility: `aeco/{architecture,construction,
   engineering,cross-cutting,visualization}/*` are CAD/BIM hosts and cloud CDEs (Tekla, Revit,
   ArchiCAD, Navisworks, ACC, Trimble Connect, Bluebeam, M365, Google…), and `_core/*` is
   `aware-agent-builder`, `aware-skill-builder`, `html-report`, `http`, `ui`, `ui-inspector` — none
   an LLM. The `ui` agent manifest even states it explicitly: *"No LLM anywhere: the AI composes,
   AWARE validates/renders"* (`20-agents/_core/ui/manifest.yaml`). A scan of `registry-index.json`
   for `vision|llm|think|gpt|claude|anthropic|openai|multimodal` returns nothing.

So the naïve floless answer — "swap the image, hit Run, a node calls a vision model on the fly" —
is rejected by the substrate twice over (no such agent exists; the validator would reject it as a
runtime LLM read). That is correct behavior. This RFC is the proposal to make a *fenced* version legal.

## 3. The proposal in one paragraph

Add a single **curated** AWARE agent, provisionally `agent: vision, command: extract`. It takes
(a) an image/PDF (bytes on the edge, as `pdf-extract` already does — `app-spec.md:657,694`) and
(b) a **fixed output schema**, and returns **structured JSON conforming to that schema** by calling
a multimodal model **at run time**. It is the desktop "Think Node" reborn as a first-class AWARE
agent — the brain lives in the **AWARE runtime**, never in a front door's UI/server. It is permitted
in the run path as a **principled, structurally-fenced exception** to the no-LLM-in-run-path axiom.
The fence (§5) is what makes the exception defensible; without every clause of it, this degenerates
into a general `llm` agent and should be declined.

## 4. What `vision.extract` may and may not do (the "OCR that happens to be smart" rule)

| It MAY | It MUST NOT |
|---|---|
| Read an image/PDF and emit typed JSON against a **fixed, declared** schema | Decide *which* nodes run next (branch control flow) |
| Be a pure leaf: `(bytes, prompt, schema, model) → JSON` | Author actions / synthesize new topology mid-run |
| Feed deterministic downstream nodes (`for-each`, `compare`, agent writes) | Be used as an `assert:` evaluator (still banned — decalog #9, `app-spec.md:314–320`) |
| Be cached by content hash (§6) | Accept caller-supplied free-form prompts that turn it into a general LLM (§5.5) |

It **extracts, never decides.** Everything downstream of it stays deterministic. The model's only
job is image → structured fields; control flow and writes remain the reviewed, deterministic plan.

## 5. The carve-out contract (each clause is load-bearing — drop one and decline the whole thing)

### 5.1 Curated + capability-flagged agent manifest

`vision.extract` ships as a **`category: curated`** command (`agent-spec.md:170–189`) — typed
`inputs:`/`outputs:`, a `commands/extract.md` with a worked example, and a skill. It declares a
new agent-level capability so the validator's exception is keyed to *this exact thing*, not to "any
agent that happens to call a model." Sketch:

```yaml
agent: vision
version: 0.1.0
display-name: Vision Extract
description: |
  Curated runtime extraction: image/PDF + a fixed output schema → structured JSON,
  via a pinned multimodal model. Extracts, never decides. The single, fenced
  exception to decalog #9's no-LLM-in-run-path rule (see RFC + app-spec § Runtime
  model extraction). NOT a general LLM agent.
provenance: { generated-by: hand-written }     # curated, not reflected
requires:
  network:
    - api.anthropic.com:443        # or the configured model endpoint
  secrets:
    - vision-model                 # ~/.aware/credentials/vision-model.json
capabilities:
  runtime-model-extraction: true   # NEW flag — the only key the validator honors (see 5.2)
commands:
  extract:
    category: curated
    lifecycle: single
    mode: read                     # extraction reads; it never writes host state
    description: Extract structured JSON from an image/PDF against a fixed schema.
    model-extraction: true         # per-command marker the validator checks
    inputs:
      file:    { type: bytes,  required: true,  description: image/PDF bytes on the edge }
      schema:  { type: object, required: true,  description: the FIXED output JSON schema }
      prompt:  { type: string, required: true,  description: extraction instruction (lock-pinned) }
      model:   { type: string, required: true,  description: pinned model id, e.g. claude-… }
    outputs:
      type: single
      schema: { '$ref': '{{ inputs.schema }}' }   # output is shaped by the declared schema
```

### 5.2 The validator exception — concrete (how `validate` tells this apart from a forbidden generic LLM node)

The exception is **NOT** "allow nodes that call a model." It is: *allow exactly the curated command
whose manifest carries the `model-extraction: true` marker, and reject every other path to a model.*
Concretely, in `cli/src/validate.rs`:

- Today there is simply **no agent+command a node can name to read with a model** — the gate is
  implicit (no such agent exists). Adding `vision` removes that implicit gate, so the exception must
  be made **explicit and narrow**, in the agent-aware validator (`validate_app_agents` →
  `check_node_agents`, `validate.rs:291–`):
  - When a node resolves to `agent: vision, command: extract`, the validator checks the resolved
    `Command` (already loaded — `validate.rs:264–269`) for the manifest marker
    `model-extraction: true` **AND** the agent-level `capabilities.runtime-model-extraction: true`.
    Only that exact, curated, double-flagged pair is admitted into the run path.
  - A new error code `E_APP_RUNTIME_MODEL_FORBIDDEN` is emitted for **any other** node whose command
    manifest declares `model-extraction: true` but is **not** `category: curated`, or whose agent
    lacks the capability flag — i.e. a reflected/auto-generated or hand-rolled "call a model" command
    can never sneak through. The flag is honored **only** on a curated command on a
    capability-declaring agent; everywhere else it is itself a validation error.
  - `assert:`/`inline.kind` stay exactly as they are: `vision.extract` is **not** admissible as an
    `assert:` evaluator (decalog #9, `app-spec.md:314–320`) — that remains `E_APP_INLINE_KIND` /
    the assert-expression whitelist. The carve-out is for **data-producing leaf nodes only**.

This keys the exception to a *specific curated command*, not a general capability — exactly the
narrowness the slippery-slope guard (§5.5) demands.

### 5.3 Declared & pinned in the `.lock`

The approved artifact must fully contain the extraction contract. `CompiledNode`
(`cli/src/app_lock.rs:68–107`) already carries `agent`, `command`, `mode`, `inputs`, and
`output_schema` — so `prompt`, `schema`, and `model` ride into the lock as node `inputs` /
`output_schema` today with **no new lock fields required** for the contract itself. Two additions
make the exception auditable rather than implicit:

- **`model-pin`** on the compiled node (or in `LockFile.agent_pins`, `app_lock.rs:51`) — the exact
  model id + provider the lock was approved against, so a model swap invalidates the approval the
  same way a source-hash change does (`LockFile.source_hash`, `app_lock.rs:32`).
- **`runtime-model: true`** marker on the compiled node — so `aware app run`, the Glass Box, and a
  front door can render "this node calls a model at run time" honestly to the approver. The lock
  stays the single source of truth for *"what was approved."*

### 5.4 Content-hash cache contract (the thing that restores per-input determinism)

`vision.extract` MUST be a **pure function with a cache**. The cache key is

```
sha256( input-bytes ‖ prompt ‖ canonicalized-schema ‖ model-id )
```

A cache **hit** returns the stored JSON with no model call; a **miss** calls the model once, stores
the result, returns it. Consequences:

- For a given distinct input the node is deterministic: same image → same extraction, replayable,
  inspectable in the run log. Non-determinism is bounded to *first sight of a new input* — exactly
  the boundary live host I/O already has.
- The cache lives under `~/.aware/apps/<app>/cache/vision/` (alongside existing per-app state,
  `app-spec.md:705`). The stored extraction is part of the provenance/receipt record so a third
  party can replay the run from the cache without a model (engineering-envelope spirit,
  `agent-spec.md:248–283`).
- This is the desktop FloLess "Think Node compile/result cache" pattern, lifted into the substrate.

### 5.5 The narrowness guard (why this can stay a carve-out and not become a hole)

The exception is honored **only** for the curated `vision.extract` command (§5.2). It is explicitly
**not** a general `llm`/`think`/`smart` agent. If it degenerated into "an agent anyone pours arbitrary
prompts into to make arbitrary runtime decisions," AWARE's "deterministic run path" would erode into
"mostly deterministic with model calls sprinkled in" — precisely the unreviewable-automation failure
mode the substrate exists to prevent (the structural-engineer veto in decalog #9: *"PE seals and
steel deliveries do not survive hallucinations"*). Therefore:

- The validator exception is keyed to the **specific curated command + capability flag**, never to a
  general capability or to "any command that sets `model-extraction`."
- `category: reflected` may **never** carry `model-extraction: true` (it would be
  `E_APP_RUNTIME_MODEL_FORBIDDEN`). No `aware build --from-*` output can mint a model-reader.
- The output is **schema-bound**: a fixed JSON schema, not free text the next node `eval`s.
- It cannot be an `assert:` evaluator or drive control flow (§4).

### 5.6 `approve:`-gated (the review gate is preserved, just relocated)

Because the extraction is the one non-deterministic-on-first-input act, the first downstream
write-mode node MUST sit behind an **`approve:`** block (`app-spec.md:329`), so the extracted JSON
appears in the Adaptive Card and a human confirms before any host write. The review gate AWARE
already mandates for compose-time extraction is **preserved** — it simply moves from
*approve-the-baked-app* to *approve-the-extraction-this-run*. For floless's swap-per-run case this is
exactly right: the user swaps the image, sees the model's reading, and approves it before anything is
written.

## 6. Net determinism argument

`vision.extract` is **a pure function `(bytes, prompt, schema, model) → JSON` with a content-hash
cache and a human approve-gate.** Per distinct input it is reproducible (cache), inspectable (JSON in
the log + Card), reviewable (approve gate), bounded (extracts, never decides), and pinned (model id in
the lock). The only honest cost is in §8.

## 7. Modules that would change (implementation sketch — NOT built in this RFC)

- **`20-agents/_core/vision/`** — new curated agent: `manifest.yaml` (per §5.1), `commands/extract.md`
  (worked example), `skills/when-to-extract.md`, output schema. A transport binary (`aware-vision`,
  the multimodal client) — Rust `reqwest` to the model API, reading the `vision-model` credential
  from `~/.aware/credentials/` (cf. the M365 cloud-bridge path, runtime-hello-world design).
- **`10-core/agent-spec.md`** — document the `model-extraction` command marker + the
  `capabilities.runtime-model-extraction` agent flag, and state that it is honored **only** on a
  curated command.
- **`10-core/app-spec.md`** — add a "Runtime model extraction (the one carve-out)" subsection right
  after `app-spec.md:322–346`, stating the exception, the fence, and that `assert:` stays LLM-free.
- **`cli/src/manifest/agent.rs`** — add `model_extraction: bool` to `Command` (≈`agent.rs:199–251`)
  and a `capabilities` block to the agent manifest struct.
- **`cli/src/validate.rs`** — the §5.2 exception in `check_node_agents` + the new
  `E_APP_RUNTIME_MODEL_FORBIDDEN` code; tests mirroring the existing `rejects_*` suite
  (`validate.rs:519–`).
- **`cli/src/app_lock.rs`** — `runtime-model` marker + `model-pin` on `CompiledNode` (§5.3).
- **`cli/src/runtime/`** — orchestrator dispatch already handles curated CLI agents
  (`invoke_single`); the cache layer (§5.4) is the one genuinely new runtime piece (key, store,
  hit/miss, provenance entry).
- **`registry-index.json` / catalog** — register `vision` so `aware agent catalog`/`search` surface it.

## 8. Honest cost + slippery slope (stated plainly so maintainers can weigh it)

This weakens the **fully-unattended** promise *for that one node*: a fresh, never-seen input on a run
with no human present must wait at the `approve:` gate (which is correct, but it is not
zero-touch-unattended for first-sight inputs). And it introduces a model dependency + a credential +
network egress into a substrate that otherwise prides itself on "binaries don't decay"
(decalog #4, CLAUDE.md tech-stack rationale). The slippery slope is real: the *only* thing keeping
this from becoming a general LLM hole is the §5 fence, enforced in the validator. If the maintainers
judge the fence insufficient — or the unattended-promise erosion too high a price — **declining is a
legitimate outcome.**

## 9. Fallback if declined (so nothing is blocked)

floless.app's Visual Inputs design ships **three** strategies with **zero** AWARE changes, and one of
them covers this exact cell:

- **B3 — re-bake on swap:** dropping a new image re-triggers *compose-time* extraction (the terminal
  AI re-reads and re-bakes only that node's literals → recompile → human approves → Run). Same
  determinism as today; the LLM act stays at compose time where AWARE already allows it; the
  human-eyeballs-the-extraction safety is intact. It is the honest "swap + needs vision" answer
  **until/unless** `vision.extract` lands.
- **B1 — deterministic runtime parse:** for inputs a parser can read (vector PDF, clean tables,
  barcodes), a read-only `exec` node with PdfPig/OCR — already legal, already deterministic.

So `vision.extract` is a **zero-friction upgrade to B3's cell**, not a prerequisite. A "no" costs
floless one approve-and-recompile round-trip per swap; it blocks nothing.

## 10. Related issues (cross-reference; not a duplicate)

- **#101 §4** — *"Runtime LLM extraction is NOT a request — it's decalog #9 working."* This RFC is the
  deliberate follow-up: accepting #101's compose-time answer as the default, and asking only about the
  one fenced exception #101 did not consider (swap-per-run + needs-vision).
- **#103** — stubbed dry-run + compile-time reference checking. Same validator surface this RFC extends.
- **#165** — read-only `exec` no longer mislabeled write-mode (the precedent that *runtime reading*
  per se is fine — it's *judgment* that's gated).
- **#106 / #180** — `aware build --from-*` reflection. Relevant only as the thing the §5.5 guard must
  fence out (no reflected command may carry `model-extraction`).
- **#215 / #201** — `ui` / `html-report` "no LLM anywhere" utility agents — the contrast that shows
  the catalogue is deliberately LLM-free today.

---

**Decision asked of maintainers:** is the §5 fence sufficient to admit a single curated
`vision.extract` into the run path as a principled exception to decalog #9 — or should the line hold
and floless rely on the compose-time fallback (§9)? Either answer is actionable for us.
