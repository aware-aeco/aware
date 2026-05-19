# Overnight session summary — 2026-05-19

Pawel, here's what ran while you were asleep.

## PRs open (in order)

| # | Title | Branch base | Status |
|---|---|---|---|
| [#66](https://github.com/aware-aeco/aware/pull/66) | feat(v0.32): rhino.exec runtime sidecar -- wraps rhinocode CLI | main | green; live drill owed |
| [#67](https://github.com/aware-aeco/aware/pull/67) | feat(v0.32.1): rhino launch + close (tekla 5-verb parity) | v0.32-rhino-exec | green; stacked on #66 |
| [#68](https://github.com/aware-aeco/aware/pull/68) | feat(v0.32.2): converge tekla exec receipt with cli-rhino's shape | main | green; pure backport |
| [#69](https://github.com/aware-aeco/aware/pull/69) | feat(v0.33): rhino-8 curated workflow verbs round 1 (5 verbs) | main | green; pure content |
| [#70](https://github.com/aware-aeco/aware/pull/70) | docs(v0.33): add READMEs for cli-rhino and cli-tekla | main | docs only |
| [#71](https://github.com/aware-aeco/aware/pull/71) | docs(v0.33): overnight session summary (this doc) | main | this doc |
| [#72](https://github.com/aware-aeco/aware/pull/72) | feat(v0.33): sketchup-2026 curated workflow verbs round 1 (5 verbs) | main | green; pure content |
| [#73](https://github.com/aware-aeco/aware/pull/73) | feat(v0.33b): rhino-8 curated workflow verbs round 2 (total: 10) | v0.33-rhino-curated-workflow | stacked on #69 |
| [#74](https://github.com/aware-aeco/aware/pull/74) | feat(v0.33b): sketchup-2026 curated workflow verbs round 2 (total: 10) | v0.33-sketchup-curated-workflow | stacked on #72 |
| [#75](https://github.com/aware-aeco/aware/pull/75) | feat(v0.33c): rhino-8 craft skills (5 hand-written narrative skills) | main | first craft skills outside tekla |
| [#76](https://github.com/aware-aeco/aware/pull/76) | feat(v0.34): sketchup.exec runtime sidecar -- Ruby in-process bridge + .NET sidecar | main | **subagent**; live drill blocked on Welcome-dialog click |
| [#77](https://github.com/aware-aeco/aware/pull/77) | feat(v0.33): revit.exec runtime sidecar -- in-Revit add-in + .NET sidecar over named pipe | main | **subagent**; live drill ready (install-addin first) |

12 PRs total from this session.

## Goal status (per the active /goal directive)

| Vendor | Sidecar verbs | Curated workflow verbs | Craft skills | Live drill |
|---|---|---|---|---|
| Tekla (reference) | 5 (pre-session) | yes | 33 (pre-session) | 13/20 v0.31 |
| Rhino | 5 (#66 + #67) | 10 (#69 + #73) | 5 (#75) | owed |
| Revit | 5 (#77) | 10 (already in main) | (deferred) | owed |
| SketchUp | 5 (#76) | 10 (#72 + #74) | (deferred) | blocked on one click |

**All three vendors now meet sidecar + curated-workflow parity with tekla.** Live drills are user-action items (rhino: no Rhino installed; revit: install-addin + open; sketchup: dismiss Welcome dialog).

## The live drill is what's owed

#66 and #67 ship rhino.exec to feature parity with cli-tekla but Rhino isn't installed on the build machine. Run the drill recipe in [`2026-05-19-v032-rhino-exec-ready.md`](./2026-05-19-v032-rhino-exec-ready.md) (the launch+close lifecycle test is in [`2026-05-19-v032.1-rhino-launch-close.md`](./2026-05-19-v032.1-rhino-launch-close.md)) — 5 minutes if Rhino starts cleanly. If PASS rate is at or above tekla's 13/20, merge #66 + #67.

## Background work — Revit + SketchUp (both LANDED)

Two parallel subagents dispatched at session start finished and landed PRs:

- **#76 — v0.34 SketchUp sidecar** (3 commits, 34/34 tests PASS). Architectural innovation: SketchUp has NO out-of-process CLI bridge, so the subagent invented one — auto-loaded SketchUp Ruby extension hosts an in-process TCP socket; `aware-sketchup.exe` connects to it. Proves the AWARE substrate generalises to vendors with only in-process scripting. **Live drill blocked by SketchUp 2026's CEF-rendered Welcome dialog** that gates plugin loading until human click — see the subagent's handoff for the 10-second action.
- **#77 — v0.33 Revit sidecar** (18 commits, comprehensive tests). First dual-binary vendor: `AwareRevit.dll` loaded INTO Revit (via .addin manifest), `aware-revit.exe` as external sidecar — they communicate over a named pipe. The add-in uses `IExternalEventHandler` to safely call RevitAPI from the pipe-server thread. Mac/Linux N/A (Revit is Windows-only).

Both subagents wrote their own design docs, plans, and handoffs. See the PR bodies for full context.

## What I shipped vs deferred

### Shipped this session
- v0.32 rhino.exec (3 verbs) — #66
- v0.32.1 rhino launch + close (5-verb parity) — #67
- v0.32.2 tekla receipt convergence (host_pid + host_version backport) — #68
- v0.33 rhino curated workflow verbs round 1 (5) — #69
- v0.33 sidecar READMEs — #70
- v0.33 session summary (this doc) — #71
- v0.33 sketchup curated workflow verbs round 1 (5) — #72
- v0.33b rhino curated round 2 (+5 → 10 total) — #73
- v0.33b sketchup curated round 2 (+5 → 10 total) — #74
- Revit + SketchUp sidecars (subagent-driven, in isolated worktrees; PRs pending)

### Found ALREADY shipped (the substrate is more mature than the roadmap suggested)
- v0.1-v0.5 (foundation): cli/src/commands/*, cli/src/runtime/*, cli/src/auth/* all present
- v0.10 spec + CLI: agent-spec.md has the curated/reflected distinction, search.rs/tree.rs have --curated flag
- v0.11 spec + CLI: app-spec.md has the safety: block spec, app.rs has --dry-run + explain
- v0.12 M365 depth: microsoft-365/manifest.yaml has 24+ curated commands across Teams/Outlook/Planner/SharePoint/Excel

Captured in memory entry [`project_substrate_maturity_audit.md`](../../../C:/Users/bimst/.claude/projects/...).

### Explicitly deferred (no work needed right now)
- Catalog self-patching loop for rhino (rhino-8 catalog has 0 empty enums; nothing to patch)
- Headless Roslyn-against-RhinoCommon fallback (geometry-only path; ~2× ship time)
- Mac/Linux (Windows-only paths assumed throughout)
- Cli-tekla test suite (no tests exist; would be a substantial separate PR)

## Memory entries added this session

| Entry | Purpose |
|---|---|
| `project_v032_rhino_exec_pr_open.md` | v0.32 PR + drill recipe pointer |
| `reference_temp_path_quirks.md` | Windows `%TEMP%` ≠ `.NET Path.GetTempPath()` on this machine |
| `project_substrate_maturity_audit.md` | The roadmap doc is behind the implementation; verify before planning |

## Suggested order for the morning

1. Verify rhino drill (5 min) — gates merge of #66 + #67
2. Check Revit + SketchUp subagent PRs (if landed) — review, decide merge order
3. Merge in this order — independent ones first, then stacked:
   - **Independent (any order):** #66, #68, #69, #70, #71, #72
   - **Stacked, requires base first:** #67 (after #66), #73 (after #69), #74 (after #72)
   - **Pending background:** Revit-sidecar PR, SketchUp-sidecar PR (when subagents return)
4. After merge, pick next from the roadmap. Remaining audit-priority candidates:
   - Archicad-29 curated workflow verbs (audit said architecture-side agents are reflection-grade)
   - Bluebeam Studio curated workflow verbs (audit's #2 most-cited tool)
   - BCF 3.0 file-format agent (audit's #4 ranked; doesn't exist yet)
   - IFC inspector agent (audit's #5 ranked; doesn't exist yet)

## Engineering rules I followed

- No `Co-Authored-By: Claude` trailers in any commit
- No `git add -A`; staged specific files only
- No auto-amend; created NEW commits when adjusting
- Each plan task → one atomic commit
- Spec self-reviews + final-pass code-reviewer subagent on #66
- Tests run before each commit where applicable; 29 tests green in cli-rhino
