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

Plus two more PRs (Revit + SketchUp sidecars) pending from the background subagents — see "Background work" below.

## Goal status (per the active /goal directive)

| Vendor | Sidecar verbs | Curated workflow verbs | Live drill |
|---|---|---|---|
| Tekla (reference) | 5 (shipped pre-session) | yes | shipped (13/20 v0.31) |
| Rhino | 5 (#66 + #67) | 10 (#69 + #73) | owed (recipe in handoff) |
| Revit | in flight (subagent) | **10 already in main** (verified during session) | depends on sidecar |
| SketchUp | in flight (subagent) | 10 (#72 + #74) | depends on sidecar |

All three vendors now have curation parity with tekla on the workflow surface. The sidecar runtime is what's still in flight for Revit + SketchUp.

## The live drill is what's owed

#66 and #67 ship rhino.exec to feature parity with cli-tekla but Rhino isn't installed on the build machine. Run the drill recipe in [`2026-05-19-v032-rhino-exec-ready.md`](./2026-05-19-v032-rhino-exec-ready.md) (the launch+close lifecycle test is in [`2026-05-19-v032.1-rhino-launch-close.md`](./2026-05-19-v032.1-rhino-launch-close.md)) — 5 minutes if Rhino starts cleanly. If PASS rate is at or above tekla's 13/20, merge #66 + #67.

## Background work — Revit + SketchUp

Two parallel subagents were dispatched at session start to build:
- **`cli-revit`** — Revit 2026 add-in + sidecar (in-Revit named-pipe Roslyn host since Revit has no out-of-process CLI). Target: all 5 verbs + live drill on the build machine (Revit 2026 IS installed at `C:/Program Files/Autodesk/Revit 2026/`).
- **`cli-sketchup`** — SketchUp Ruby extension + .NET sidecar. Target: all 5 verbs + live drill (SketchUp 2026 IS installed at `C:/Program Files/SketchUp/SketchUp 2026/`).

Both are in **isolated worktrees** so they don't conflict with each other or with my main-checkout work. Both have authority to open their own PRs. Check `gh pr list --author @me` in the morning to see what they produced.

If either is BLOCKED (Revit add-in load failure, SketchUp socket sandbox, etc.) the subagent's PR body or last message describes what they tried. The notification I get when they complete (success or BLOCKED) will land in your morning Claude Code transcript.

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
