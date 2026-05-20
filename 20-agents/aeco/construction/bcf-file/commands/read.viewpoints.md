# `bcf-file.read.viewpoints` — extract cameras, selections, and snapshots

Stateless, read-only. Lists every viewpoint in a BCF — the camera pose, the selected/coloured components, and the embedded snapshot PNG. The PNG is extracted to disk and its path returned, so this is the verb behind "build me a per-issue screenshot gallery." Scope to one topic with `topic-guid`, or omit for all.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `path` | string | `.bcfzip` file or unpacked BCF folder. |
| `topic-guid` | string (optional) | Scope to one topic. Omit for all viewpoints in the file. |

## Outputs

```yaml
viewpoints:
  type: array
  items:
    topic-guid:     string
    viewpoint-guid: string
    snapshot-path:  string    # absolute path of the extracted PNG (see note)
    camera-type:    string    # "perspective" | "orthogonal"
    camera:
      eye:    array           # [x, y, z] camera position, world coords
      target: array           # [x, y, z] look-at point
      up:     array           # [x, y, z] up vector
```

> **Snapshot extraction.** Embedded PNGs are written to a per-run temp directory under the workspace (`./.aware/bcf-snapshots/<topic-guid>/<viewpoint-guid>.png`) and the absolute path is returned in `snapshot-path`. If a viewpoint has no embedded snapshot, `snapshot-path` is `""` — the viewpoint (camera + selection) still returns.

## Under the hood

A topic folder holds a primary `viewpoint.bcfv` plus zero or more `<vp-guid>.bcfv` extras, each with a matching `.png`:

```xml
<VisualizationInfo>
  <Components>
    <Selection>
      <Component IfcGuid="2O2Fr$t4X7Zf8NOew3FNr2" />   <!-- referenced by IFC GUID -->
    </Selection>
    <Coloring>
      <Color Color="ffff0000"><Component IfcGuid="…"/></Color>
    </Coloring>
  </Components>
  <PerspectiveCamera>
    <CameraViewPoint><X>12.3</X><Y>4.5</Y><Z>1.7</Z></CameraViewPoint>
    <CameraDirection>…</CameraDirection>
    <CameraUpVector>…</CameraUpVector>
    <FieldOfView>60</FieldOfView>
  </PerspectiveCamera>
</VisualizationInfo>
```

Two camera flavours exist: `<PerspectiveCamera>` (`camera-type: perspective`, carries `FieldOfView`) and `<OrthogonalCamera>` (`camera-type: orthogonal`, carries `ViewToWorldScale`). The agent normalises both to the `{ eye, target, up }` triple — `eye` from `CameraViewPoint`, `target` derived from `CameraViewPoint + CameraDirection`, `up` from `CameraUpVector`.

**Selections reference IFC GUIDs, not BCF GUIDs.** The viewpoint stays meaningful across tools only as long as the underlying IFC GUIDs are stable. If a Revit re-export churns GUIDs, the camera still works but the highlighted components no longer resolve. See [bcf-round-trip-interop](../skills/bcf-round-trip-interop.md).

## Composition example — per-issue gallery in an HTML report

```yaml
- id: topics
  agent: bcf-file
  command: read.topics
  config: { path: "{{ inputs.bcf }}" }

- id: viewpoints
  agent: bcf-file
  command: read.viewpoints
  config: { path: "{{ inputs.bcf }}" }

- id: report
  agent: html-report
  command: render
  config:
    title: "Clash gallery — {{ topics.project-name }}"
    # join viewpoints to topics on topic-guid; embed each snapshot-path as an <img>
    sections: "{{ join(topics.topics, viewpoints.viewpoints, 'guid', 'topic-guid') }}"
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `bcf.file-not-found` | `path` doesn't exist | Check the path |
| `bcf.topic-not-found` | `topic-guid` given but no such topic | Run [`read.topics`](./read.topics.md) first |
| `bcf.snapshot-extract-failed` | Workspace temp dir not writable | Check filesystem `write` permission for the workspace |
| `bcf.unknown-camera` | A `.bcfv` declares neither perspective nor orthogonal camera (malformed) | Camera fields return empty; selection still extracted |

## See also

- [bcf-viewpoints-and-snapshots](../skills/bcf-viewpoints-and-snapshots.md) — camera math + the IFC-GUID anchoring rule
- [bcf-round-trip-interop](../skills/bcf-round-trip-interop.md) — why snapshots go stale and selections lose their anchor
- [`read.topics`](./read.topics.md) — get topic GUIDs to scope by
