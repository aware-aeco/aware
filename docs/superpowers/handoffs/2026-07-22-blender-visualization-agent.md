# Resume: implement the Blender visualization agent (phase 1 — prototype the bpy scripts)

You are continuing work in the aware-aeco/aware repo (https://github.com/aware-aeco/aware.git).
This is a fresh session with no memory of the prior one — everything you need is below.

> Full handoff doc (this prompt, saved on disk): docs/superpowers/handoffs/2026-07-22-blender-visualization-agent.md
> If you need more than this prompt carries, open that file and read it.

## Where things stand

The prior session brainstormed and approved the design for a `blender` agent — the first
visualization agent in the substrate that renders fully unattended (IFC in → PNG stills +
turntable MP4 out, headless via `blender -b -P`). The design doc is written, committed, and
pushed. No implementation exists yet (verified: no `20-agents/aeco/visualization/blender/`).

## Live repo state (verified at handoff)

- Branch: `claude/session-63df38`, pushed to origin (fetch + check it out if starting on a new machine)
- Working tree: clean
- Recent commits:
  - fe4d29587 docs(specs): blender visualization agent design — IFC-in, headless renders out
  - a560a6fa5 fix(roslyn): bump System.Security.Cryptography.Xml to 9.0.18 to clear NU1903
  - b4740963d chore(release): v0.100.0 — a named error for a missing agent, and CI that runs the gates

## Your task

Read `docs/superpowers/specs/2026-07-22-blender-visualization-agent-design.md` end-to-end first —
it is the approved contract for this work. Then execute its delivery plan, starting with phase 1:

1. **Prototype the bpy scripts** (all real risk lives here): standalone scripts for
   (a) IFC import via `ifcopenshell.geom` iterator building Blender meshes + semantics as
   custom properties, (b) camera auto-framing, (c) EEVEE still render, (d) turntable MP4.
   Verify each headless: `blender -b -P script.py -- args` against a real local Blender.
   Generate the test IFC with the CLI's `ifc.write` from a small scene JSON (see
   `cli/src/render/ifc.rs` for what it emits; `cli/tests/` for how it's invoked).
2. **Build the agent** at `20-agents/aeco/visualization/blender/` — manifest + skills +
   `scripts/`, following the manifest pattern of
   `20-agents/aeco/visualization/twinmotion-prep/manifest.yaml`; register in
   `registry-index.json`; bump the stats markers in CLAUDE.md/README.
3. **Example app** `30-apps/_examples/model-to-renders.app` (see existing examples there).

## Key context (files, decisions, gotchas)

- `docs/superpowers/specs/2026-07-22-blender-visualization-agent-design.md` — the approved design; commands table, look-mapping table, error handling, out-of-scope list.
- `cli/src/render/ifc.rs` — `ifc.write`: deterministic IFC4 with parametric IfcProfileDefs, IfcMaterial from ASTM grade, IfcStyledItem colours. This is the agent's reference input producer.
- `D:\Repos\floless.app\server\contract-to-scene.ts` — the upstream bake (steel.takeoff/v1 contract → generic scene). Context only; the agent must NOT depend on FloLess.
- Decision: **IFC-only input in v1; glTF rejected** (Tekla can't export glTF — import-only; a glTF path would need a new CLI exporter carrying less semantics). Don't re-litigate.
- Decision: **raw `ifcopenshell` in Blender's bundled Python, NOT the Bonsai add-on** (Bonsai is GUI-oriented, unproven under `blender -b`). IfcConvert preprocessing is a v2 escape hatch.
- Decision: agent stays generic — consumes any IFC; zero FloLess/.flo leakage into AWARE content.
- Gotcha: Collada was removed in Blender 5.0 — don't add a DAE path.
- Gotcha: Pawel's machine has a Blender MCP add-on (`mcp__Blender__*` tools) but Blender must be RUNNING for it to connect; for this task prefer plain headless `blender -b` anyway (that's what the agent will do).
- Gotcha: ripgrep times out on the full `20-agents/` tree (3,341 skill files) — scope Grep to subdirs or use `ls`.
- Gotcha: this branch lives in a worktree on the original machine (`D:\Repos\aware-aeco-worktrees\session-63df38`); on a new machine just check out `claude/session-63df38` normally.

## Engineering rules to honor

- Committing is pre-approved; push/merge to `main` need explicit approval. No `Co-Authored-By: Claude` trailers. Stage specific files, never `git add -A`.
- Verify before answering: read primary sources end-to-end; no claims from memory/summaries.
- Rust work: `cargo fmt --all` + `cargo clippy --all-targets -- -D warnings` must pass; errors as data via `thiserror`; no `unwrap()` outside tests.
- Agent skill `.md` files route through the `skill-creator` skill (see `20-agents/_core/aware-skill-builder/`).
- PR review before merge: Codex first (`codex exec review --base main`), fall back to `pr-review-toolkit:code-reviewer` only if Codex is genuinely unavailable.

## Suggested skills

- `superpowers:writing-plans` — invoke FIRST: turn the design doc into the implementation plan (the brainstorm phase is done; this is the agreed next step in the flow).
- `aware-agent` — if Pawel says "drive it end to end": it orchestrates implement → verify → Codex-reviewed PR → merge autonomously.
- `skill-creator` — when authoring the agent's skill `.md` files in phase 2.
- `superpowers:systematic-debugging` — if headless Blender/ifcopenshell misbehaves.

## How to verify you're done

Phase 1 done = a headless smoke test passes on a real Blender: generate the fixture IFC via
`ifc.write` from a checked-in scene JSON → `blender -b -P` import + EEVEE low-res still →
assert the PNG exists, has expected dimensions, and is not a single flat colour; `scene.info`
inventory matches the fixture's known classes/materials. Full task done = agent installed and
runnable via the `aware` CLI per the design doc's commands table, fmt/clippy/tests green.
