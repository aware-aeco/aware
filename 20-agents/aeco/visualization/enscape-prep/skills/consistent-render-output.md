# Consistent render output across a project

The designer-audit's hidden-pain: every render of the same project
looks slightly different. Color profile, exposure, denoise level all
drift across designer machines.

This agent doesn't solve material consistency (that's Enscape's
job — Chaos Asset Library or Material Library). It does solve **scene
state consistency**:

| State | Variance source | This agent fixes |
|---|---|---|
| Camera | Designer puts camera differently each time | `named-view` enforces |
| Layer visibility | Layers toggled per session | `layer-state` enforces (Rhino) |
| Sun position | "Morning" varies by month | `sun-date` + `sun-time` explicit |
| Material override | Per-machine overrides drift | `material-override-set` (project standard) |
| Resolution | Whatever Enscape was last set to | Forced via `scene.capture-frame` inputs |

Once these are locked, Enscape's own render is reproducible-enough for
client-deck purposes. For *forensic* reproducibility (engineering /
litigation evidence), Enscape isn't the renderer to choose — use a
ray-tracer with deterministic settings (V-Ray, Corona, Maxwell).

## The project-deck flow

End-of-week client deck for a designer working on 8 active projects:

```yaml
app: weekly-client-decks
schedule:
  cron: "0 17 * * FRI"
  timezone: "America/New_York"

nodes:
  - id: projects
    agent: aware-runtime
    command: read-yaml
    inputs:
      path: '\\office\projects\active-projects.yaml'

  - id: prep-each
    for-each: '{{ projects.items }}'         # v0.19 primitive
    do:
      - agent: enscape-prep
        command: scene.prep
        inputs:
          host: rhino
          host-file: '{{ item.rhino-path }}'
          named-view: 'Client Hero'
          sun-date: '{{ item.target-time-of-year }}'
          sun-time: '09:00'
          output-epx: '{{ run.tmp-dir }}/{{ item.id }}.epx'
        safety:
          transaction-group: weekly-decks
          snapshot: false

  - id: notify
    agent: microsoft-365
    command: teams.channel.post-with-card
    inputs:
      team-id: 'Design Studio'
      channel-id: 'weekly-decks'
      title: 'Weekly decks prepped — {{ projects.items.length }} scenes ready'
      rows: '{{ projects.items }}'
```

Designer arrives Monday, opens each `.epx`, hits Render, posts the result.
That's the realistic value-add — not "AI renders for you" (false), but
"every Monday morning, your 8 scenes are pre-staged correctly" (true).
