# Vendor curation matrix — 2026-05-19

A consolidated snapshot of every AECO vendor agent's curation level, intended as a planning aid for "which vendor to curate next." Status as of end of the 2026-05-19 overnight session.

## Curation grades (the v0.10 framework, from `agent-spec.md`)

| Grade | Definition |
|---|---|
| **Reflected** | Auto-generated from vendor docs / SDK / NuGet / OpenAPI. Wide coverage, but un-curated leaves. |
| **Workflow-curated** | Hand-written `category: curated` commands with typed inputs/outputs and worked examples in `commands/<name>.md`. |
| **Craft-skilled** | Hand-written narrative skills covering vendor-specific concepts (transactions, units, layers, etc.). |
| **Runtime-bridged** | Has an `aware-<vendor>` sidecar binary that can dispatch `exec` against the live vendor process. |
| **Drilled** | Live 20-prompt drill has been run with PASS rate ≥ tekla's 13/20 baseline. |

## The full matrix

### Engineering vendors

| Vendor | Reflected | Workflow-curated | Craft-skilled | Runtime-bridged | Drilled |
|---|---|---|---|---|---|
| tekla (curated) | yes | 3 verbs | 33 skills | yes (v0.31) | **13/20** (v0.31) |
| tekla-2025 / tekla-2026 | 280 / 291 types | (reflected only) | (no narrative) | shared with above | n/a |
| tekla-tedds-25 / -26 | yes | — | — | — | — |
| idea-statica-25 / -26 | 119 types | — | — | — | — |
| revit-2025 | 7,432 cmds | — | — | — | — |
| revit-2026 | 7,659 cmds | **10 verbs** (in main) | **6 craft skills** (#79) | **5 verbs** (#77) | **20/20 PASS** ✨ |
| autocad-2025 / -2026 | 4,415 cmds | **5 verbs** (autocad-2026, #82) | **6 craft skills** (#81) | — | — |
| dynamo-4-1-0 / -4-1-1 | 2,433 cmds | **5 verbs** (dynamo-4-1-1, #83) | — | — | — |

### Architecture vendors

| Vendor | Reflected | Workflow-curated | Craft-skilled | Runtime-bridged | Drilled |
|---|---|---|---|---|---|
| rhino-7 / rhino-8 | 1,399 types | **10 verbs** (rhino-8, #69+#73) | **5 craft skills** (#75) | **5 verbs** (#66+#67) | physically blocked |
| grasshopper-7 / -8 | 4,849 / 11,268 cmds | — | — | — | — |
| sketchup-2025 / -2026 | 155 types | **10 verbs** (sketchup-2026, #72+#74) | **6 craft skills** (#78) | **5 verbs** (#76) | physically blocked |
| archicad-28 / -29 | 204 types | — | (3 craft skills pre-session) | — | — |
| allplan-2024 / -2025 | 155 / 192 types | — | — | — | — |

### Construction vendors

| Vendor | Reflected | Workflow-curated | Craft-skilled | Runtime-bridged | Drilled |
|---|---|---|---|---|---|
| navisworks-2026 (curated) | (12 pre-session) | 12 verbs | **8 craft skills** (was 3; +5 in #80) | — | — |
| solibri-26.4.1 | 28 types | — | (2 craft skills pre-session) | — | — |
| bluebeam-studio | 13 cmds | — | — | — | — |
| bluebeam-v1 | 14 cmds | — | — | — | — |

### Specs / Documents

| Vendor | Reflected | Workflow-curated | Craft-skilled | Runtime-bridged | Drilled |
|---|---|---|---|---|---|
| avitru-speclink | yes | 10 verbs (pre-session) | (2 craft skills) | — | — |
| csi-masterformat | yes | 6 verbs (pre-session) | (1 craft skill) | — | — |
| nbs-chorus | yes | 9 verbs (pre-session) | (2 craft skills) | — | — |

### Cross-cutting

| Vendor | Reflected | Workflow-curated | Craft-skilled | Runtime-bridged | Drilled |
|---|---|---|---|---|---|
| microsoft-365 | yes | 24+ verbs (pre-session) | (skills folder) | — | — |
| google-workspace | yes | (similar) | — | — | — |

## What this session shipped

12 vendor-related PRs this session, plus 8 supporting PRs (20 total):

**Sidecar runtime (3 vendors):**
- #66, #67 — Rhino (3-verb + 2-verb = 5-verb parity)
- #76 — SketchUp (5 verbs; in-process Ruby bridge + .NET sidecar)
- #77 — Revit (5 verbs; in-Revit add-in + .NET sidecar, **20/20 live drill PASS**)

**Curated workflow verbs (4 vendors):**
- #69, #73 — Rhino (5 + 5 = 10 verbs)
- #72, #74 — SketchUp (5 + 5 = 10 verbs)
- #82 — AutoCAD (5 verbs)
- #83 — Dynamo (5 graph-lifecycle verbs)

**Craft skills (4 vendors):**
- #75 — Rhino (5 skills)
- #78 — SketchUp (6 skills)
- #79 — Revit (6 skills)
- #80 — Navisworks (5 more, was 3 → now 8)
- #81 — AutoCAD (6 skills)

**Supporting:**
- #68 — Tekla receipt-shape convergence
- #70 — Sidecar READMEs (cli-rhino + cli-tekla)
- #71 — Session summary

## Recommended next phases (by audit priority)

| Priority | Vendor | What's missing |
|---|---|---|
| 1 | Archicad-29 | Both curated workflow verbs AND craft skills (only 3 craft skills exist) |
| 2 | Bluebeam Studio | Curated workflow verbs + craft skills (audit's #2 most-cited tool) |
| 3 | Solibri | Curated workflow verbs (28 reflected; only 2 craft skills) |
| 4 | Allplan-2025 | Curated workflow verbs + craft skills (audit's largest empty-enum case) |
| 5 | IDEA Statica | Same |
| 6 | BCF 3.0 file-format agent | NEW agent (doesn't exist; audit's #4 ranked) |
| 7 | IFC inspector agent | NEW agent (doesn't exist; audit's #5 ranked) |

## What "tekla level" actually means (as observed)

The audit defined tekla-parity as "31 craft skills + 3 workflow commands + 5-verb sidecar + live drill PASS." This session matched the SIDECAR + curated WORKFLOW level for rhino + revit + sketchup, and demonstrated 5-6 craft skills per vendor (still ~5x short of tekla's 33).

For new vendors entering AWARE, the realistic v0.10 first-wave target is:
- **5 sidecar verbs** (`exec`, `list-instances`, `send-status`, `launch`, `close`)
- **10 curated workflow verbs** (the 80% of practitioner mental units)
- **5-6 craft skills** (the most-frequently-needed vendor concepts)
- **Live 20-prompt drill** ≥ 13/20 PASS (matching tekla)

This roadmap should land any vendor at workflow-grade in 1-2 days of focused work.
