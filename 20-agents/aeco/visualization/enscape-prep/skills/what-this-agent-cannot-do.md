# What this agent cannot do (and why)

The honest part of the agent name. Enscape (Chaos) provides:

✅ A Windows desktop automation surface (limited)
✅ Project-file presets (`.epx`)
✅ Asset Library management UI (manual)
✅ Material library (manual)

❌ A REST / RPC API
❌ Headless rendering
❌ Render queue
❌ Material override scripting
❌ Animation timeline scripting
❌ Cloud render dispatch via API

We **cannot**, from a `.flo` app:

- Change Enscape's material library programmatically
- Set up a multi-frame animation render
- Dispatch to Enscape Cloud servers
- Run Enscape headless on a build agent

If your workflow needs any of those, **AWARE is the wrong tool for the rendering step.** Use AWARE to *prep* the source model + *route* the resulting render files; render in Enscape's UI.

## What we CAN do

- Drive Rhino / Revit / SketchUp / ArchiCAD to apply consistent named-view + layer-state + sun position before the designer opens Enscape (`scene.prep`)
- Write an `.epx` preset that Enscape consumes on file-open
- Capture single stills + panoramas through Enscape's Windows automation hooks (`scene.capture-frame`, `scene.capture-panorama`)
- List view names in the host file (`view.list`)

That's the realistic ceiling. The render itself remains a Designer-in-the-loop operation.

## What the audit asked for

> "Render the latest model from the SE corner perspective at 4K with the morning sun, save as PNG, post to Teams."

With this agent:

```yaml
- id: prep
  agent: enscape-prep
  command: scene.prep
  inputs:
    host: rhino
    host-file: '\\office\projects\acme\acme.3dm'
    named-view: 'SE Perspective'
    sun-date: '2026-06-21'
    sun-time: '09:00'
    output-epx: '{{ run.tmp-dir }}/acme-morning-se.epx'
  safety:
    transaction-group: render-prep
    snapshot: false   # source file untouched; .epx is new
```

After this, the designer opens Enscape (if not already running) and clicks Render. The agent captures the frame via `scene.capture-frame` if Enscape is open and the project is loaded.

A future v0.17.x improvement could auto-launch Enscape via Windows shell + wait for ready state — that requires running Enscape headless, which Chaos doesn't support today.

## Cloud render farms

Chaos Cloud is a thing. Chaos Cloud doesn't expose an API that lets us submit + poll jobs as of 2026-05. If/when they do, this agent gets a `scene.queue-cloud` verb.
