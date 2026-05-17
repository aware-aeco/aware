# BCF anatomy

A `.bcfzip` is a ZIP archive with a flat / per-topic-folder layout:

```
project.bcfp                  # project info (BCF 2.1+)
bcf.version                   # version marker
<topic-guid-1>/
  markup.bcf                  # topic + comments XML
  viewpoint.bcfv              # primary viewpoint XML
  snapshot.png                # primary viewpoint screenshot
  <vp-guid>.bcfv              # additional viewpoints
  <vp-guid>.png
<topic-guid-2>/
  ...
```

Each topic is a self-contained folder named by its GUID. Comments and
viewpoints inside the topic are also GUID-keyed.

## Topic XML (markup.bcf)

```xml
<?xml version="1.0" encoding="UTF-8"?>
<Markup>
  <Topic Guid="abc-123" TopicType="Clash" TopicStatus="Open">
    <Title>Pipe clashes with structural beam</Title>
    <Priority>High</Priority>
    <CreationDate>2026-05-17T09:00:00Z</CreationDate>
    <CreationAuthor>P.Lisowski</CreationAuthor>
    <AssignedTo>mep-consultant@acme.com</AssignedTo>
    <Description>Detected by Navisworks clash test 'MEP-Struct-Hard'.</Description>
  </Topic>
  <Comment Guid="def-456">
    <Date>2026-05-17T10:15:00Z</Date>
    <Author>mep-consultant@acme.com</Author>
    <Comment>Acknowledged. Pipe re-routed in v07.</Comment>
  </Comment>
</Markup>
```

## Viewpoint XML (viewpoint.bcfv)

Carries camera + clipping + selected components + colouring. Cameras
come in two flavours:

| Type | Schema |
|---|---|
| Perspective | `{ eye, target, up, fov }` |
| Orthogonal | `{ eye, target, up, view-to-world-scale }` |

Selected components are referenced by **IFC GUID** — which means the
viewpoint survives across tools as long as the underlying IFC GUIDs
are stable.

## Round-trip preservation

`read.topics` → `filter.by-status` → `write.from-csv` (or merge with
another BCF) preserves all topic-level + comment-level metadata. The
agent does **not** strip extension fields it doesn't recognise — they
pass through verbatim, so a BIMcollab-emitted BCF survives a round
through this agent and back unchanged.

## Common pitfalls

- **Topic GUID collisions** when merging two BCFs that originated from
  the same source: `merge` dedupes by GUID + keeps the *later* source
  (timestamp-based). If both sources are equally "current," use
  `filter.by-status` first to scope each.
- **Stale snapshots:** the embedded PNG is captured at viewpoint
  creation time; if the model has since changed, the PNG won't match.
  This is BCF's design (provenance > freshness) — accept it.
- **Encoding:** BCF XML is UTF-8 by spec but some tools emit
  Windows-1252. The agent transparently transcodes on read.
