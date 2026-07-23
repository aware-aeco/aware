# Resume: presentation visuals for the blender agent (HDRI, ground + contact shadows, camera work)

You are continuing work in the aware-aeco/aware repo (https://github.com/aware-aeco/aware).
This is a fresh session with no memory of the prior one — everything you need is below.

> Full handoff doc (this prompt, saved on disk): docs/superpowers/handoffs/2026-07-22-blender-presentation-visuals.md
> If you need more than this prompt carries, open that file and read it.

## Where things stand

The `blender` agent shipped: IFC in, PNG stills + MP4 turntables out, fully unattended
(`blender -b -P`). Five commands, a Rust builtin transport, 3 skills, 4 test suites, an
end-to-end smoke gate and an example app — merged in #312 and released as **v0.102.0**.

What shipped is a **technical** render: correct geometry, correct materials from IFC
semantics, neutral studio background. Pawel explicitly chose to finish that first and then
do presentation visuals as a follow-up. **That follow-up is your task.**

## Live repo state (verified at handoff)

- Branch: `main`, working tree clean
- Recent commits:
  - `fa3ef89c7` chore(release): v0.102.0 — the substrate renders, unattended
  - `0f83ca3b6` feat(blender): headless visualization agent — IFC in, renders out (#312)
  - `60b546ae3` chore(release): v0.101.0 — bake-scene says what it is doing in Tekla's status bar
- Latest tag: `v0.102.0` (pushed; `release.yml` was mid-build at handoff time)

**OWED — do this first, before any new work:** the v0.102.0 release run was still building.
Confirm it finished and that npm actually published (`publish-npm` is `continue-on-error`, so
a green run is NOT proof):

```bash
gh run list --workflow=release.yml --limit 1
npm view @aware-aeco/cli version      # must be 0.102.0
npm view @aware-aeco/cli dist-tags    # latest must be 0.102.0
```

If npm 404'd but the GitHub Release succeeded, the binaries shipped — re-run the
`publish-npm` job and report the gap. Never silently assume it published.

## Your task

Raise the agent's output from *technically correct* to *presentation quality*, without
losing the unattended property. Three pieces, in this order:

**1. HDRI environment lighting** — extend `setup_world()` in
`20-agents/aeco/visualization/blender/scripts/render_still.py:34`. It currently builds a
**procedural neutral grey gradient** (`TexCoord.Generated → SeparateXYZ.Z → MapRange →
ColorRamp → Background.Color`) — read it before changing it, and read its docstring, which
explains *why* a flat world was wrong. There is **no HDRI, no environment-texture, and no
image-based lighting anywhere in the agent today** (verified by grep). Decide and record:
where does an HDRI come from? Bundled with the agent (licensing + repo size), fetched, or a
caller-supplied path input? Keep a procedural fallback so a missing HDRI never fails a render
— the never-fail principle runs through this whole agent.

**2. Ground plane + contact shadows** — a shadow-catcher plane under the model. None exists
today. Size it from `_framing.scene_bounds()`; it must not appear in `scene.info`'s inventory
(that walks `bpy.data.objects` for meshes with IFC custom properties — check
`scripts/scene_info.py:_inventory`, and note the reviewer already flagged that a non-IFC mesh
would be counted in `count`/`elements` with empty fields). Cycles has a real
`object.is_shadow_catcher`; EEVEE does not, so the draft and production paths may need
different treatments — verify against the installed Blender rather than assuming.

**3. Camera work** — `scripts/_framing.py` currently fits the model's **bounding sphere** and
places the camera on one of six fixed axis directions. The sphere is deliberate: it is
rotation-invariant, so `render.turntable` reuses one solve for every frame. Any improvement
(three-quarter framing, slight vertical offset, focal-length choice, depth of field) must not
break that property, or the turntable will clip at some angle. Read `_framing.py`'s module
docstring before touching it.

Start by reading the three shipped skills — they carry every trap this agent already cost:
`20-agents/aeco/visualization/blender/skills/{headless-rendering,ifc-import-ifcopenshell,look-presets}.md`.

## Key context (files, decisions, gotchas)

- `20-agents/aeco/visualization/blender/scripts/` — `_result.py` (sentinel protocol + named
  errors), `_ifc_import.py`, `_looks.py`, `_framing.py`, and the five command scripts.
- `cli/src/render/blender.rs` — the Rust builtin transport: Blender discovery
  (`$AWARE_BLENDER` → PATH → platform defaults), spawn, sentinel parsing, timeout,
  process-kill. New inputs need declaring in `manifest.yaml`; the transport passes inputs
  through as JSON, so no Rust change is needed for a new script input.
- `docs/superpowers/specs/2026-07-22-blender-visualization-agent-design.md` — the approved v1
  design. `docs/superpowers/plans/2026-07-22-blender-visualization-agent.md` — the plan, with
  a verified-facts table and a written record of every trap.

**Decisions to respect (do not re-litigate):**
- IFC-only input in v1; glTF rejected (Tekla cannot export glTF).
- Raw `ifcopenshell`, not the Bonsai add-on.
- Transport is `builtin`, not a bridge binary — Blender is a standalone exe taking CLI args,
  not a vendor SDK needing a native host process. Reasoning is in the plan doc.
- The gradient world is *lighting infrastructure, not art direction* — it is deliberately
  neutral and preset-agnostic. If HDRI changes that, say so explicitly and justify it.

**Gotchas — each of these cost real time:**
- **Numeric checks cannot validate presentation quality.** The `realistic` preset once
  rendered near-black and passed every automated gate — non-flat, model centred, correct
  dimensions. Only *looking at the image* caught it. For this task especially: render it,
  open it, look. Also note that a corner-sampled "background estimate" silently breaks the
  moment the background becomes a gradient (it misclassified ~73% of the frame as model); use
  an alpha-silhouette mask (`film_transparent=True` companion render) if you need to measure
  the model region. And "low variance = murky" turned out to be the *wrong* proxy — mean
  luminance was the real signal.
- **Metals are almost entirely specular.** A `metallic 0.85` material renders as what it
  reflects; in a near-black world it renders near-black. This is why the gradient exists.
- **`matrix_world` is stale in background mode** after setting `.location`/`.rotation_euler`
  until the next depsgraph evaluation. `bpy.ops.render.render()` self-heals; diagnostics do
  not. Call `bpy.context.view_layer.update()` before reading it.
- **Every command script needs its `if __name__ == "__main__":` guard.** `sys.argv` is
  process-global; unguarded, importing a script re-parses the importer's argv and `sys.exit()`s.
- **Blender 5.2 specifics:** EEVEE is `BLENDER_EEVEE` (not `_NEXT`); `bl_rna` engine enum
  under-reports (returns only `['BLENDER_EEVEE']` though CYCLES is available — never build an
  availability check on it); `image_settings.file_format = "FFMPEG"` needs
  `media_type = "VIDEO"` first; `Action.fcurves` is gone since the 4.4 layered-action redesign.
- **`ifcopenshell` user-site trap:** Blender's bundled Python has `ENABLE_USER_SITE = False`
  and ignores `PYTHONPATH`, so pip's `--user` fallback is never on `sys.path`. Handled in
  `_ifc_import._import_ifcopenshell()` — don't "simplify" that away.
- **`scripts/sync_stats.py` crashes on Windows consoles** with `UnicodeEncodeError` (cp1250
  can't encode `→`) — and it crashes *after* writing `Cargo.toml`, leaving a half-applied
  bump. Always run it as `PYTHONIOENCODING=utf-8 python scripts/sync_stats.py …`.
- **`aware agent reindex --check` is a separate gate from `sync_stats`** — both move when the
  agent/command set changes. This one was red and nearly shipped.
- Blender here: `C:\Program Files\Blender Foundation\Blender 5.2\blender.exe` (5.2.0 LTS,
  bundled Python 3.13.13, ifcopenshell 0.8.5). `$AWARE_BLENDER` overrides discovery.
- `ripgrep` times out on the full `20-agents/` tree (3,344 skill files) — scope to subdirs.

## Engineering rules to honor

- Committing is pre-approved. **Pushing, merging to `main`, and releasing each need explicit
  approval every time.** No `Co-Authored-By` trailers. Stage specific files, never `git add -A`.
- Rust: `cargo fmt --all` + `cargo clippy --all-targets -- -D warnings` must pass; errors as
  data via `thiserror`; no `unwrap()` outside tests. CI does **not** run `cargo test` — that
  gate is yours.
- Skill `.md` files route through `skill-creator` (CLAUDE.md, no exceptions).
- Every PR gets Codex review first: `codex exec review --base main`. Fall back to
  `pr-review-toolkit:code-reviewer` only if Codex is genuinely unavailable.
- Verify before claiming. Adding an agent/command shifts counts: `sync_stats.py --write`,
  `aware agent reindex`, and the strict count assertions in `cli/tests/agent_list.rs` and
  `cli/tests/app_list.rs`.

## Suggested skills

- `superpowers:brainstorming` — invoke FIRST. HDRI sourcing (bundle vs fetch vs caller path)
  and the licensing/repo-size tradeoff is a genuine design fork, not an implementation detail.
- `superpowers:writing-plans` — after the design is agreed, before touching code.
- `skill-creator` — required if you update the agent's `look-presets.md` or
  `headless-rendering.md` skills, which you likely will.
- `superpowers:systematic-debugging` — if a render comes out wrong; check the
  `matrix_world` staleness trap before concluding the framing is broken.
- `aware-agent` / `aware-agent-tag` — if Pawel says "drive it end to end" / "and release it".

## How to verify you're done

1. **The existing gate still passes** — this is non-negotiable, B must not regress A:
   ```bash
   python 20-agents/aeco/visualization/blender/tests/run_smoke.py \
     --blender "/c/Program Files/Blender Foundation/Blender 5.2/blender.exe" \
     --aware-bin ./cli/target/debug/aware.exe
   ```
   Expect `SMOKE PASS` (~16s). Plus `cargo fmt`/`clippy -D warnings`/`test` green.
2. **Look at the output.** Render the fixture with the new environment at 960×540 in both
   `draft` and `production`, open the PNGs, and judge them. State plainly whether they read as
   presentation-quality. If you cannot tell, say so rather than guessing — that honesty is the
   whole point of this task.
3. **The turntable still orbits cleanly** — no clipping at any angle (the bounding-sphere fit
   is what guarantees this; confirm camera distance from the pivot stays constant across
   frames).
4. **Still unattended** — no new input is *required*; every addition has a working default, and
   a missing HDRI degrades to the procedural world rather than failing the render.
