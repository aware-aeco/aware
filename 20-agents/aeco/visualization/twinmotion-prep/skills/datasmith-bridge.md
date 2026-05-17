# The Datasmith bridge

Twinmotion's interop format. The `.udatasmith` file carries:

- Mesh geometry (per-host LOD)
- Materials (Twinmotion-mapped from host's PBR equivalents)
- Lights (omni, spot, IES)
- Named views / cameras
- Hierarchy / layer groups

## What flows cleanly

| Host | Datasmith export quality |
|---|---|
| Revit 2024+ | Excellent — Datasmith is Epic-blessed |
| Rhino 7+ | Good — via Datasmith Exporter plugin |
| SketchUp 2024+ | Good — official plugin |
| ArchiCAD 27+ | Good — Direct Link |

## What's lossy

- **Procedural materials** (Rhino's PBR map graphs collapse to flattened textures)
- **Animation rigs** in the host (skinned-mesh animation doesn't round-trip)
- **Custom Grasshopper objects** that aren't baked

## Direct Link vs file export

Twinmotion supports a *Direct Link* with several hosts: changes in Rhino flow live to Twinmotion without re-export. This agent's `scene.export-datasmith` does the **file-export** path (more reliable, scriptable). For Direct Link sync, designers use the host plugin manually — this agent doesn't control that.

## Worked example

End-of-week walkthrough for client presentation:

```yaml
- id: datasmith
  agent: twinmotion-prep
  command: scene.export-datasmith
  inputs:
    host: rhino
    host-file: '\\office\projects\acme\acme.3dm'
    output-path: '{{ run.tmp-dir }}/acme.udatasmith'
  safety:
    transaction-group: weekly-walkthrough
    snapshot: false

# Designer opens Twinmotion + imports the udatasmith, then triggers:

- id: walkthrough
  agent: twinmotion-prep
  command: scene.capture-walkthrough
  inputs:
    path-name: 'Client Approach'
    output-path: '\\office\projects\acme\decks\{{ run.date }}-walkthrough.mp4'
    duration-seconds: 45
    resolution: 4k
  safety:
    transaction-group: weekly-walkthrough
    snapshot: false
```

The animation path "Client Approach" was pre-authored in the Twinmotion desktop UI; the designer doesn't re-author it each week — same path, fresh geometry.
