# Persona Audit — Designer (Rhino / Grasshopper / Twinmotion)

**Verdict — three sentences:**

Adoption likelihood today: **near zero.** The thesis is fine, the docs read well, the engineering scaffolding is real — but for a senior designer who lives in Rhino/Grasshopper/Twinmotion, AWARE is shipping a substrate without the surface I actually touch. Top 3 blockers: (1) **zero real-time viz agents** (no Enscape 4.2, no Twinmotion 2025, no V-Ray 7, no Lands Design — the entire output side of my pipeline is missing), (2) the existing `rhino-8` and `grasshopper-8` agents are **6,000-command raw API dumps with no curated skills** for "take a screenshot" or "load named view" — the things a designer actually does, (3) **no concept of camera / viewport / view-state as first-class workflow primitives** anywhere in the substrate. The one app I'd want next week: a "Monday concept review" pipeline that takes the latest Rhino file, captures three named views at consistent settings, posts them to a Teams channel with a one-line note — and right now nothing in here knows how to do step one.

---

## A. First-impression friction

The manifesto reads well. "App = text. AI = runtime. Open source is what the format does automatically." Fine, I get it. The 60-second demo at the top — `npm install -g @aware-aeco/cli` → one English sentence → working Tekla→Trimble Connect integration — is the right shape of pitch. It's the kind of thing my principal would forward me at 10pm with "have you seen this."

But it's a **fabricator** pitch, not a **designer** pitch. The canonical example is Tekla → Trimble Connect drawing upload. Tekla. The first time anyone at our office touched Tekla was when a structural consultant emailed us an IFC we couldn't open in Rhino without losing materials. The decalog truth #5 says "AECO is the wedge, not the limit" — fair — but the wedge they picked is structural fab, not architectural design. Nothing in `00-vision/` or the example apps speaks to me. Show me a Rhino → Enscape → client folder flow at the top of the manifesto and I'd read the rest with very different eyes.

10-minute install test (mentally): `npm install -g @aware-aeco/cli` — fine, I have Node from when our web team installed Storybook for a marketing site. `aware agent install aware-aeco` — fine. Now what? `aware app list` shows two example `.flo` files about Tekla and a fab pipeline. I have no Tekla. The `aware doctor` output in cli-spec.md cheerfully reports "tekla@2025.0.1 — host software not detected." Good error message; **wrong agents installed by default.** A designer's first-run experience should land them in front of `rhino@8.x` working with the Rhino file they have open right now.

## B. The "no-code" claim

This is where the audit hurts most.

Look at `welded-to-tc.flo`. It's readable as English, sure. `tekla-watch` → `filter-welded` → `tc-upload`. I can squint at it. **But the `filter-welded` node has a JavaScript arrow function inline:**

```yaml
code: |
  e => e.type == "Welded" && e.mark != null
```

That's not no-code. That's micro-code. I write GH Python nodes, occasionally, kicking and screaming. The instant a designer sees an arrow function and a `&&` they bail — and the spec says inline glue is "tiny operations (filters, maps, reshapes) that don't deserve a dedicated agent." Tiny by whose standard? In Grasshopper that "filter" is a Dispatch component with a boolean pattern. **Where's my Dispatch?** A no-code substrate for designers ships predefined filter / sort / pair / group / split atoms that don't require typing JavaScript.

Templating syntax: `{{ tekla-watch.mark }}.pdf` is fine if you read Jinja before. But the kebab-case → underscore translation rule documented in cli/README.md ("kebab-case node ids are auto-translated to underscore; use bracket syntax for raw kebab access: `{{ upstream["my-node"].field }}`") is the kind of footgun that will eat 30 minutes of my Monday. **Designers do not debug template languages.**

Copy-paste-modify test: could I write a Rhino→Enscape version of `welded-to-tc.flo` by copying the file and changing names? No — because (a) there's no Enscape agent, (b) the rhino-8 manifest exposes 6,954 commands and there isn't a "render-view" or "save-png" one that a designer would recognize. The CaptureToBitmap method is buried at line 20,399 of the manifest under `view-capture-capture-to-bitmap` with the description "ViewCapture.CaptureToBitmap." That is the description verbatim. Auto-generated from XML docs. **Useless** without curation.

## C. Coverage gaps in my domain

**Visualization / real-time:** There is **no Enscape agent, no Twinmotion agent, no Lumion agent, no V-Ray agent.** I grepped the entire repo for `enscape|twinmotion|lumion|vray|lands|ladybug|honeybee|climatestudio|diva|rhinoinside` — nine hits, all of them are either the .gitignore, the master diagram, planning docs from CLI v0.3, or a comment in `tekla/commands/save-attributes.md`. **Zero agents.** The whole output half of my workflow doesn't exist.

The `20-agents/aeco/visualization/` folder is misleadingly named. It contains `three`, `xeokit`, `web-ifc`, `speckle-viewer`, `thatopen-components`, `itwin-5-8`, `itwin-5-9` — these are **web-based IFC viewers for online dashboards**. They are not the visualization tools an architectural designer reaches for. None of them produce a frame I'd put in a client deck. None of them have a camera or render queue or material library a designer cares about.

**Sustainability / daylight:** No Ladybug Tools 1.8, no Honeybee, no ClimateStudio, no Diva. The `grep` for `sun-study|daylight|solar` hit only the iTwin viewport tooling. So the "1-hour sun study across spring/autumn equinox" scenario is impossible. I can't even fake it through Grasshopper because the grasshopper-8 agent exposes the C# kernel components by name (`GH_FlattenTreeComponent`) but has zero domain skills for "how do you actually run a sun study in this thing."

**Rhino + Grasshopper depth:** the agents exist on paper. 6,954 / 5,181 / 5,341 raw commands for rhino-8 / grasshopper-8 / sketchup-2026. They are **thin where it counts.** Open `skills/rhino-display.md` — it's a literal namespace listing: "ColorCMYK — Represents a CMYK color." That's verbatim from the McNeel XML doc. No skill that says "to capture a viewport at print resolution, use ViewCaptureSettings with these defaults." No worked example. No "you probably want this for that."

**RhinoInside.Revit:** none. That's the bridge half my team would actually use to talk to Revit, and it's invisible.

**Render / output side, full list of misses:**
- Enscape (Asset Library, GI settings, panorama, video export)
- Twinmotion 2025 (Path Tracer, Cloud, materials)
- V-Ray 7 (frame buffer, render queue, distributed)
- Corona / Maxwell / KeyShot
- Lumion 2024 (effects stack)
- D5 Render
- Lands Design (vegetation library)
- VisualARQ (parametric architectural objects)

**Drawing side:**
- Illustrator / Indesign (I do round-trips with .ai daily for diagrams)
- Bluebeam Revu (PDF mark-up from consultants)
- A1/A0 plot setup, sheet layouts

**Site / context:**
- OSM, Mapbox, Cadmapper, Open Street Map Buildings
- GIS (QGIS, ArcGIS)
- Cesium for 3D tiles

**Visual output as a first-class concept:** completely missing. The substrate has `connections:`, `nodes:`, `inputs:` — but no concept of "I want a 1920×1080 PNG saved here named `<project>_<date>_<viewname>.png`." Screenshots/renders are not workflow primitives; they're command #20,399 buried in a YAML.

## D. Workflow primitives I wish existed

The substrate models the world as nodes-and-edges-of-data. That's correct for a fabricator's pipeline (parts flow through). For a designer, **the workflow primitives are different.** They are:

1. **Camera / Named view / View state** — "the SE perspective at human eye height from the corner of plot 14." I save 30 of these per project. AWARE has no concept of a view as a first-class object that lives across runs. The Rhino agent has `view-table-find` buried in 6,954 commands but no skill that explains "your project conventionally uses these 6 named views — here's how to iterate."

2. **Layer / Layer-state / Block** — "capture this with only the As-Built layers on, then again with the Proposed layers on." Should be a 3-line app. Today, impossible.

3. **Compare A vs B** — the heart of design review. Same camera, same lighting, model A vs model B, side-by-side. No primitive.

4. **Watch + rerun GH definition on file change** — this is the bread-and-butter parametric study loop. The substrate has stateful watchers (the Tekla example) but no first-party `grasshopper.solve(file, parameters)` command exposed cleanly.

5. **Sweep parameters → produce a contact sheet of variants** — Grasshopper Colibri does this; nothing here does.

6. **Take a screenshot of layer X with named view Y at resolution Z** — this is the atomic operation of a designer's day. It should be one line. Today it's 20,000 lines of raw API.

The app-spec.md `inline:` glue block is genuinely the wrong abstraction. A designer doesn't want JavaScript predicates. They want **named atoms** like `for-each-named-view`, `for-each-layer-state`, `with-camera`, `at-resolution`, `save-png`, `compose-grid`.

## E. Trust + visual quality

Would I let AWARE drive the model my principal is reviewing in 30 minutes? **No way.** Not because the runtime might crash — that's fixable. Because:

- **No render queue concept.** A 4K Twinmotion Path Tracer frame takes 12-40 minutes. An overnight 8K animation is a 12-hour job. AWARE has no queue, no priority, no resume-on-fail, no "render on this machine if local, otherwise dispatch to the render farm." The spec is silent on long-running heavy compute. The `stateful: true` watcher model isn't the right primitive — I need a **batch executor with progress, retry, and resource hints.**

- **Versioning of the model.** If AWARE writes into Rhino — modifies layers, inserts geometry, runs a GH definition — can I roll back? The agent-spec says agents declare destructive ops via capabilities, fine. But Rhino itself has no real version history (Undo is per-session, lost on save). The substrate would need to checkpoint the .3dm before mutating it. Nothing in the spec addresses this. **I would not trust it to touch a file my principal is reviewing.**

- **Aesthetic determinism.** The spec is fully oriented at "did the data move from A to B." It has zero language for "is the resulting render visually consistent with the last 15 frames I rendered for this project." Color profile, white balance, exposure, tone-map, denoise settings — all the things that make my client deck look like a deck and not like a tech demo. No skills, no schema, no examples.

## F. The killer feature

**The "Show Me" app.** One sentence: *"Show me what changed since last Monday."*

It would:
1. Diff the active Rhino file against last Monday's git snapshot (or last-archived .3dm)
2. Render the 3 standard project views at consistent settings
3. Overlay a delta map on the plan view (added geometry green, removed red)
4. Drop the result in `\\office\projects\<project>\reviews\<date>\` as a 3-page PDF
5. Post a one-line summary to our Teams channel: "32 m² added to wing B, façade panel 8 reshaped, parking moved 4m east"

If that existed I would never again do my Sunday-night "what did I actually change this week" panic ritual. **That is one app, ~80 lines of YAML, and it requires:** a real Rhino agent with `capture-named-view` + `make-2d` + `layer-state-load`, a git-or-snapshot diffing agent, a PDF assembly agent (the html-report agent might do, charitable reading), and a Teams agent (microsoft-365 has it). Today: blocked on every one of those primitives.

## G. The 5-Mondays adoption test

**Monday 1 — Concept review screenshots:** Open latest Rhino, three perspective shots, post to Teams. Gaps: no curated `capture-view` skill, no Teams target on microsoft-365 (need to check), no consistent-camera primitive. Doable manually in 5 min today. AWARE would take longer because I'd be debugging YAML.

**Monday 2 — A vs B comparison render:** Same camera, two models, side-by-side. Gaps: no V-Ray/Enscape/Twinmotion agent, no compose-grid primitive, no consistent-camera promise across two open files. Blocked entirely.

**Monday 3 — Area schedule to Excel:** Extract GFA/NLA/façade from Rhino, drop in template. Gaps: `excel@1.x` agent exists but Rhino has no curated `compute-areas-by-layer` skill — the raw API has `area-mass-properties-compute` at command #N but no recipe. Doable if I write a GH definition first and call **that** from AWARE.

**Monday 4 — Send-to-consultant export:** Clean Rhino → IFC+DWG+3dm → Trimble Connect. Gaps: `trimble-connect` agent exists, IFC export buried in rhino-8 raw commands (no curated skill), no "remove hidden layers + working geometry" recipe. Halfway viable.

**Monday 5 — Twinmotion walkthrough video:** Push latest Rhino into Twinmotion 2025, capture 30s, drop in team channel. Gaps: **no Twinmotion agent. Entire flow impossible.**

After 5 weeks, conservatively: **maybe 10-15%** of my toil is reduced — mostly on Monday 3 (Excel) and Monday 4 (file exports), and only if I'm willing to learn Rust-CLI semantics. The viz half — which is 50% of my actual job — is untouched.

## H. The dealbreaker

**No visualization agents, no curated rhino/GH skills, no first-class concept of a rendered frame.**

That's the dealbreaker. Not three things — one entwined thing. The substrate's whole pitch is "AI is the runtime that executes plain English." But if I write *"render the latest model from the SE corner perspective at 4K with the morning sun, save as PNG, post to Teams"* — the runtime today has to do that via 6,954 unlabeled API calls in a YAML, on a Rhino agent that doesn't know what "morning sun" means semantically, on a render path that goes through Rhino's own Cycles (which I haven't touched since 2019 because Enscape and Twinmotion exist).

The fabricator scenarios work because Tekla and Trimble Connect have **a small, semantically-coherent API surface** (model object, drawing, file upload). The substrate's auto-generation is fine for that. It is **fundamentally wrong** for the design side, where the right grain of primitive is not "every public C# method" but **the dozen things a designer actually does** wrapped in curated skills that encode taste.

**Fix this and I'm in.** Ship `enscape-4` and `twinmotion-2025` and `vray-7` agents (or — more honestly — say "we won't, because they have no scriptable API; we'll route through Rhino's view-capture-to-bitmap and let the user render outside" — that's an acceptable answer **if you say it in the docs**). Curate the rhino-8 manifest down from 6,954 commands to the 50 commands a designer actually invokes, each with a real skill that says "use this for that." Add `camera`, `named-view`, `layer-state`, `at-resolution`, `compose-grid`, `for-each` as first-class composition primitives so I don't have to write `e => e.type == "Welded"` to filter a list.

Until then, I keep Grasshopper open in one monitor and Twinmotion in the other, and I do Mondays the same way I've done them since 2017. The substrate is structurally sound; the surface it offers me is empty.
