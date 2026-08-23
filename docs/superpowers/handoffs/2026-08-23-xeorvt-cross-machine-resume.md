# Resume: Continue xeoRVT after the completed AWARE RVT reader Task 6

You are continuing xeoRVT work across `aware-aeco/aware` and `pawellisowski/floless.app` on another
machine. This is a fresh session with no memory of the prior one—everything needed to establish the
correct server-backed starting point is below.

> Full handoff doc (this prompt, saved on disk):
> `docs/superpowers/handoffs/2026-08-23-xeorvt-cross-machine-resume.md`
> Fetch and check out `aware-aeco/aware:codex/xeorvt-aware-rvt-reader`, then open this file.

## Where things stand

AWARE Task 6 is implemented test-first: the provider-neutral local RVT reader, bounded host protocol,
deterministic GLB and explicit Revit metadata normalization, authenticated crash-safe cache/publication,
CLI lifecycle fencing, curated agent, tests, build wiring, and documentation are on the long-lived AWARE
integration branch. The fifth and final `codex-review` round returned `REVISE`; all three concrete final
findings were incorporated before implementation under the user's explicit human gate.

Real xeoRVT 0.2.0 output from `Residential building.rvt` was used as compatibility evidence and drove
four bounded-profile fixes. The AWARE reader normalized 6,001 parts and 23,786,172 canonical bytes from
738,232 triangles, dropping 189 degenerates. This proves managed-cloud output compatibility, not the
separately installed local provider execution contract. The corresponding FloLess reference-model line
is also integrated and pushed. Neither integration branch was merged into its product default.

## Live repo state (verified at handoff)

- AWARE origin: `https://github.com/aware-aeco/aware.git`
- AWARE branch: `codex/xeorvt-aware-rvt-reader`
- AWARE implementation tip before this handoff-only commit: `bf873b19efde07fb72c4fd9664fb3dfc5f223ce0`
- AWARE remote matched that tip after a fresh `fetch origin --prune` and `git ls-remote`.
- Current fetched AWARE default: `origin/main` at `295b13b62d509a9e56e91037e83a21dab50e78d9`, proven an ancestor of the integration tip.
- AWARE working tree was clean before adding this handoff.
- Recent AWARE commits:
  - `bf873b19e` Merge origin/main into xeoRVT reader integration
  - `db9216b6f` fix: accept real xeoRVT output without weakening trust boundaries
  - `56c8cd41c` fix: make model host cleanup cancellation-safe
  - `3a01b74d5` fix: guarantee model host teardown
  - `7f977fa56` fix: bound model reader cancellation and crash recovery
- FloLess origin: `https://github.com/pawellisowski/floless.app.git`
- FloLess branch: `codex/xeorvt-reference-model`
- FloLess local and remote tip: `15936dcd10b36bd078529e0aed24a886dfcbf523`
- Current fetched FloLess default: `origin/master` at `aabc1d78086cb90f35b7d1c98fcb9c592e430e93`, proven an ancestor of the integration tip.
- Recent FloLess commits:
  - `15936dcd` test: record merged connection pick seam
  - `ea17293f` fix: preserve selection ownership across Xeorvt merge
  - `1008370b` Merge master into Xeorvt reference model
- No PR, release, product-default merge, tag, or force-push was made.
- Open AWARE issue: `#452`, “Python test gate discovers ignored .tmp review copies”. The canonical
  committed agent Python tree passed; ignored copied review tests contaminated repository-wide discovery.
- Three older Task 3/5/6 handoff files remain untracked in the original FloLess worktree. They were
  preserved and were not uploaded because the old Task 6 handoff describes the pre-implementation state.

## Your task

Establish the two pushed integration lines on the new machine, verify their exact remote state, install
the updated cross-repository `xeorvt-integrate` skill, and continue from the next xeoRVT product task—do
not rebuild Task 6 or treat either integration branch as product `main`/`master`.

First:

1. Clone or fetch both repositories, then create/check out local tracking branches from
   `origin/codex/xeorvt-aware-rvt-reader` and `origin/codex/xeorvt-reference-model`. Run `git status
   --short`, `git log --oneline -8`, and the ancestry checks in “How to verify” before editing.
2. In AWARE, read `CLAUDE.md`, the complete Task 6 plan and review log under
   `docs/superpowers/specs/`, and the reader entry points under `cli-connection-reader/`. Do not infer
   state from this handoff when the primary files can answer it.
3. Install the synchronized skill from the Google Drive folder `Claude/xeorvt-integrate` into the new
   machine's Claude and/or Codex skills directory. Its shared `SKILL.md` covers both repositories;
   `agents/openai.yaml` is the Codex UI metadata. The skill requires independent integration per repo
   and never cross-merges commits.
4. Confirm the next xeoRVT slice from the current product plan or the user's new instruction before
   coding. If it changes a high-stakes provider, binary protocol, trust, cache, lifecycle, or
   FloLess↔AWARE seam, harden its plan with `codex-review` before implementation.

## Key context (files, decisions, gotchas)

- `docs/superpowers/specs/2026-08-23-aware-rvt-reader-plan.md` — complete reviewed Task 6 contract and
  verification ledger. Its opening status predates implementation; Git history and code are now newer.
- `docs/superpowers/specs/2026-08-23-aware-rvt-reader-review-log.md` — all five adversarial rounds plus
  the real xeoRVT compatibility evidence and its exact limitations.
- `cli-connection-reader/model-dispatcher.mjs` — provider-neutral IFC/RVT dispatch boundary.
- `cli-connection-reader/model-reader.mjs`, `model-provider.mjs`, `model-host-client.mjs`, and
  `model-cache.mjs` — orchestration, trusted local provider protocol, host framing, and authenticated
  cache/publication.
- `cli-connection-reader/revit-glb.mjs` and `revit-metadata.mjs` — deterministic bounded normalization;
  metadata is explicit and resolved, never inferred from names or geometry.
- `cli/src/commands/model_reader_host.rs` — Windows Job Object / Unix process-group host, held locks,
  framed binary streams, cancellation, and zero-descendant completion.
- `20-agents/aeco/engineering/model-reference-reader/` — curated `preflight`, `probe`, and `read-model`
  agent surface.
- Decision: FloLess and AWARE are coordinated but independent repositories. AWARE target is
  `codex/xeorvt-aware-rvt-reader` from `main`; FloLess target is `codex/xeorvt-reference-model` from
  `master`. Never merge commits across repositories.
- Decision: both target branches are long-lived xeoRVT integration lines, not replacement defaults.
- Gotcha: `H:/My Drive/FloLess/Revit` contains authorized Revit inputs on the original machine/cloud
  drive. Never commit the RVT files or converted/provider artifacts.
- Gotcha: xeoRVT account configuration was read from
  `D:/Repos/floless-web/apps/web/.env.local` on the original machine. That file is intentionally not on
  GitHub. Provision equivalent secrets locally on the new machine; never paste them into a handoff,
  commit, log, or command output.
- Gotcha: the managed API returned malformed signed output URLs using `https:/...`; local transport
  normalization was needed for the evaluation download. Do not generalize that workaround into AWARE's
  local provider contract.
- Gotcha: the cloud metadata lacks the reviewed storage-type and explicit relationship semantics, so no
  provider-specific cloud translator belongs in AWARE core.

## Engineering rules to honor

- Read each repository's local instructions and primary specs before changing code. In AWARE, also read
  the decalog, manifesto, and applicable agent/app/CLI contracts for load-bearing decisions.
- Work test-first. Critical trust, bounds, cache, receipt, lifecycle, and determinism tests must be
  mutation-checked; never weaken a guard, inventory, assertion, threshold, or discovery scope to go green.
- Keep GLB and framed provider data binary. Keep outputs deterministic, canonically ordered, bounded,
  authenticated, and independently decodable. Never expose secrets, absolute sensitive paths, model
  bytes, provider URLs, or credentials.
- Preserve existing IFC behavior and keep provider-specific semantics behind AWARE. FloLess remains a
  thin consumer of authenticated artifacts; AWARE is extension-agnostic and `.flo` belongs to FloLess.
- Stage explicit files; do not use `git add -A`. Do not add `Co-Authored-By` trailers, force-push, delete
  worktrees, merge to `main`/`master`, open a PR, release, or push without fresh user authority.
- Run `xeorvt-integrate` independently in each changed repository. Fetch first, integrate its fetched
  default, prove ancestry, run that repo's acceptance suite on the exact final tree, and push only when
  explicitly requested.
- Reproduce unexpected AWARE behavior and file a focused issue only when it is an AWARE defect, not a
  rejected bad provider/API call.

## Suggested skills

- `xeorvt-integrate` — after a coherent xeoRVT slice is committed; consolidates the correct repository
  into its long-lived integration branch and proves the final tree.
- `codex-review` — before implementing any new high-stakes protocol, trust, cache, lifecycle, auth, or
  cross-repository contract plan.
- `systematic-debugging` — for local provider, host-process, cache, SEA, or real-output discrepancies.
- `skill-creator` — for any future change to `xeorvt-integrate`; update both Claude and Codex variants
  and validate them before copying the shared package to Drive.
- `handoff` — when the next slice is complete and needs another cross-machine continuation brief.

## How to verify you're done

On the new machine, after fetching:

```powershell
git -C <aware-repo> ls-remote --heads origin codex/xeorvt-aware-rvt-reader
git -C <aware-repo> merge-base --is-ancestor origin/main origin/codex/xeorvt-aware-rvt-reader
git -C <floless-repo> ls-remote --heads origin codex/xeorvt-reference-model
git -C <floless-repo> merge-base --is-ancestor origin/master origin/codex/xeorvt-reference-model
```

The remote AWARE branch must contain implementation commit `bf873b19e`; the remote FloLess branch must
contain `15936dcd`. Both ancestry commands must exit `0`, both working trees must be clean before new
work, and the installed `xeorvt-integrate/SKILL.md` must name both repositories and their correct default
and integration branches. Before claiming a new slice complete, run the exact repository-specific test,
build, lint/guard, real-provider, and real-browser gates applicable to what changed.
