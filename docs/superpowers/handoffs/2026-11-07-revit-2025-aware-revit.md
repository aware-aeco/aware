# Resume: verify revit-2025 on the aware-revit bridge against a real Revit 2025, then ship it

You are continuing work in the AWARE repo (https://github.com/aware-aeco/aware). This is a
fresh session on a different machine, the one with Revit 2025 installed. It has no memory of
the prior session; everything you need is below.

> Full handoff doc (this prompt, saved on disk): docs/superpowers/handoffs/2026-11-07-revit-2025-aware-revit.md
> If you need more than this prompt carries, open that file and read it.

## Where things stand
- During the #710 audit, #774 marked `revit-2025` `status: planned` because it named
  `aware-revit-2025`, which ships nowhere. The real reason was that nobody had pointed it at the
  shared `aware-revit` bridge, the way 54b3ccb5a did for `revit-2026` and 9f8b94785 for `revit-2027`.
  Revit 2025 itself is supported.
- The prior session checked the code: the bridge serves 2025. It also prepared the manifest change
  (commit 96b8cc22d). It could not run anything, because that machine has only Revit 2026 and no
  disk space for 2025.
- **Nothing is verified at runtime.** CI tests the add-in only at RevitVersion=2027
  (`.github/workflows/ci.yml:551-559`). The 2025 add-in is only ever *compiled*, in
  `release.yml:258-259`. That is why the real run below is the gate.

## Live repo state (verified at handoff)
- Branch: `claude/revit-2025-aware-revit`, pushed to origin. Based on origin/main 273f136d4.
- Commits on the branch:
  - 96b8cc22d fix(agents): wire revit-2025 to the shared aware-revit bridge (#710)
  - (this handoff doc, committed on top)
- No PR yet. Issue #710 stays open; use `Refs #710`, never `Closes`.
- Registry: `revit-2025` has two records, 2025.0.2.419 (pinned to 91887fa35, manifest 0.1.0) and
  2025.0.2.420 (pinned to a43f4845d, manifest 0.1.1, planned). **Both are already pinned, not
  main-tracking, so no freeze step is needed.**

## Your task
Prove that a real `aware app run` of an `exec` node works against Revit 2025, then ship the branch
and publish the registry record.

1. `git fetch && git switch claude/revit-2025-aware-revit`. Read commit 96b8cc22d. It changes
   `20-agents/aeco/architecture/revit-2025/manifest.yaml`: version 0.1.1 → 0.1.2, binary
   `aware-revit`, `status: planned` removed, and the five runtime verbs `exec`, `list-instances`,
   `send-status`, `launch` and `close` added. Their text is identical to revit-2026/2027 except that
   the exec hint says `-RevitVersion 2025`.
2. Set up the bridge for 2025:
   - `aware sidecar install revit`
   - `pwsh "$HOME\.aware\bridges\install-addin.ps1" -RevitVersion 2025`
   - Start Revit 2025 and open any model, e.g. a sample from its `Samples` folder.
   - Run `aware-revit` with `{"verb":"list-instances"}` on `--json-stdin`. Wait until the 2025
     instance shows `addin_loaded: true`, which takes about 20-25 s.
3. Install the agent from the branch: `aware agent install 20-agents/aeco/architecture/revit-2025`.
   Then write a throwaway app **outside the repo** that `requires: [revit-2025@0.1.x]`, with one
   node: `agent: revit-2025`, `command: exec`, `mode: read`, and inputs `version: "2025"` and a
   literal `code` such as `return uiapp.ActiveUIDocument.Document.Title;`. Copy the shape of
   `30-apps/_examples/model-to-construction-sequence.app`; the spec is in `10-core/app-spec.md`
   (exec/`mode: read` near line 1093). Run `aware app validate`, `aware app compile` and
   `aware app run <app> --json`. Done means `ok: true` and the 2025 document's title in the result.
   **Never use `--simulate` or `--dry-run`: they don't reach Revit.**
4. If the run fails, debug it (see Gotchas). Fix it in this branch only if the fix belongs to
   revit-2025's manifest. Anything in `cli-revit/` gets reported first, because it affects 2026
   and 2027 too.
5. Ship it:
   - Push, then open a PR with `Refs #710`.
   - Get a Codex review of the **final** commit: `codex exec review --base main`, or comment
     `@codex review` on the PR.
   - Wait for green CI, then run `gh pr merge <n> --squash --admin --delete-branch --body '...'`
     with an explicit body.
   - Merging needs Pawel's approval unless he already gave it.
   - Decide whether to keep this handoff doc in the PR (handoffs are committed here by convention)
     or drop it.
6. Publish the record in a follow-up PR, following #773 (8538bd3ca) and #776 (84211ef22):
   - Append `2025.0.2.421` to `registry-index.json`: tarball and subdir pinned to the squash
     commit, `manifest-version` 0.1.2, bundle digest from the pinned Git tree.
   - Move the aware-aeco bundle entry from 2025.0.2.420 to 2025.0.2.421.
   - Regenerate the catalog with `aware agent reindex`. It must report revit-2025 `available`.
   - Same Codex and CI gate, then merge.

## Key context (files, decisions, gotchas)
- `cli/src/commands/sidecar.rs` BRIDGES: `aware-revit` is described as "Revit 2025–2027 (net8 sidecar
  + version-specific add-ins)".
- `cli-revit/AwareRevit/AwareRevit.csproj`: RevitVersion=2025 builds net8.0-windows against
  `Autodesk.Revit.SDK 2025.0.2.419`. It writes `aware-revit-version.txt`, which
  `install-addin.ps1` checks.
- `cli-revit/install-addin.ps1`: accepts 2025/2026/2027 and takes the payload from
  `~/.aware/bridges/AwareRevit/2025`. It refuses a payload built for the wrong year.
- `cli-revit/Sidecar/RevitProcessFinder.cs`: reads the year from the Revit.exe path using the regex
  `/Autodesk/Revit (\d{4})/`, with one pipe per instance, `aware-revit-<pid>`.
  **Gotcha:** if Revit 2025 is installed in a folder that isn't named `...\Autodesk\Revit 2025\`,
  the sidecar silently skips it and `list-instances` shows nothing.
- **Gotcha:** Revit may show the "Cannot verify the publisher" prompt for AwareRevit.dll at
  startup. Click "Always Load", otherwise the pipe never opens. Background is in
  `docs/superpowers/handoffs/2026-05-19-v033-revit-exec-ready.md`.
- **Gotcha:** `cargo test` leaks disk on these machines. Run filtered tests only, e.g.
  `cargo test --locked --bin aware -- registry:: manifest:: validate::`.
- Decision: `bake-scene` was left out on purpose; the brief asked for the five verbs only.
  revit-2026/2027 have it, and `cli-revit/Sidecar/BakeSceneScript.cs` says it is neutral across
  2025 and 2026. Offer it as a follow-up; don't fold it in.
- Registry rule: records are append-only. The only rewrite allowed is re-pinning a record to a
  fixed commit with an identical digest. CI enforces this with
  `scripts/check_registry_append_only.py` and `aware agent reindex --check` (`ci.yml:215-230`).

## Engineering rules to honor
- Rust/CLI changes must pass `cargo fmt --all` and `cargo clippy --all-targets -- -D warnings`.
- Committing is pre-approved. Pushing and merging to `main` need explicit approval each time.
- Commit messages carry no `Co-Authored-By: Claude` trailers. Always pass an explicit `--body` when
  merging. Stage specific paths, never `git add -A`.
- Every PR gets a Codex review of its final commit. Fall back to `pr-review-toolkit:code-reviewer`
  only if Codex is unavailable both by CLI and on GitHub, and say so in the PR.
- Verify before claiming: "works with Revit 2025" means a real run's output, not a green build.

## Suggested skills
- `aware-update`: run it first so the CLI and the substrate match main.
- `superpowers:systematic-debugging`: use it if `list-instances` or `exec` fails against 2025.
- `aware-agent`: use it to drive the PR through Codex review, CI and merge, if that's the house flow.
- `superpowers:verification-before-completion`: use it before reporting done.

## How to verify you're done
1. `aware app run` of the exec node returns `ok: true` with live data from the Revit 2025 document.
   Paste the JSON into the PR.
2. Both PRs are merged with Codex clean on their final commits and CI green.
3. On a fresh `AWARE_HOME`, `aware agent catalog --json` reports `revit-2025` `available` at
   0.1.2, and `aware agent install revit-2025` gives 0.1.2 with a verified digest.
