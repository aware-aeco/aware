---
name: bcf-format-anatomy
description: This skill should be used when reading from or writing to BCF files with the bcf-file agent — understanding the .bcfzip archive layout, the markup.bcf / viewpoint.bcfv / snapshot.png triple, GUID-folder naming, and what the agent preserves vs transcodes on a round-trip. Encodes the on-disk structure every bcf-file verb operates on.
---

# BCF anatomy

BCF (BIM Collaboration Format, an OASIS / buildingSMART standard) is the interchange format for coordination *issues* — not geometry. A `.bcfzip` is just a ZIP archive with a fixed internal layout. Every `bcf-file` verb reads or writes this structure, so understanding it is the prerequisite for reasoning about any of them.

## Archive layout

```
project.bcfp                  # optional — project id + name
bcf.version                   # required — "2.1" or "3.0" version marker
extensions.xml                # 3.0 only — project-defined status/type/priority vocabularies
<topic-guid-1>/
  markup.bcf                  # topic metadata + comments (XML)
  viewpoint.bcfv              # primary viewpoint (camera + selection) (XML)
  snapshot.png                # primary viewpoint screenshot
  <vp-guid>.bcfv              # additional viewpoints
  <vp-guid>.png               # their snapshots
<topic-guid-2>/
  ...
```

Each topic is a **self-contained folder named by its GUID**. This is why folder-level operations ([`filter.by-status`](../commands/filter.by-status.md), [`merge`](../commands/merge.md)) can copy a whole topic — markup, viewpoints, snapshots — without rewriting anything inside it. Comments and viewpoints within a topic are themselves GUID-keyed.

`bcf-file` accepts both a packed `.bcfzip` and an already-unpacked folder; it detects which by probing for the `bcf.version` marker at the root. The on-the-wire format every tool exchanges is the zip.

## Topic XML (`markup.bcf`)

```xml
<?xml version="1.0" encoding="UTF-8"?>
<Markup>
  <Topic Guid="abc-123" TopicType="Clash" TopicStatus="Open">
    <Title>Pipe clashes with structural beam</Title>
    <Priority>High</Priority>
    <CreationDate>2026-05-17T09:00:00Z</CreationDate>
    <CreationAuthor>P.Lisowski</CreationAuthor>
    <AssignedTo>mep-consultant@acme.com</AssignedTo>
    <Labels><Label>MEP</Label></Labels>
    <Description>Detected by Navisworks clash test 'MEP-Struct-Hard'.</Description>
  </Topic>
  <Comment Guid="def-456">
    <Date>2026-05-17T10:15:00Z</Date>
    <Author>mep-consultant@acme.com</Author>
    <Comment>Acknowledged. Pipe re-routed in v07.</Comment>
  </Comment>
</Markup>
```

`TopicType`, `TopicStatus`, and `Priority` are **open string fields** — there is no closed enum in the spec. In 3.0, the project's allowed values are declared in `extensions.xml`; in 2.1 they're entirely free. This is the single biggest source of cross-tool friction — see [bcf-status-and-priority-vocabularies](./bcf-status-and-priority-vocabularies.md).

## Viewpoint XML (`viewpoint.bcfv`)

Carries camera + clipping planes + selected/coloured components. Two camera flavours:

| Type | Schema |
|---|---|
| Perspective | `{ eye, target, up, field-of-view }` |
| Orthogonal | `{ eye, target, up, view-to-world-scale }` |

Selected components are referenced by **IFC GUID**, so a viewpoint survives across tools only as long as the underlying IFC GUIDs are stable. Full treatment in [bcf-viewpoints-and-snapshots](./bcf-viewpoints-and-snapshots.md).

## What the agent preserves vs changes

- **Round-trips are lossless at topic level.** [`read.topics`](../commands/read.topics.md) → [`filter.by-status`](../commands/filter.by-status.md) → [`merge`](../commands/merge.md) preserves all topic + comment + viewpoint metadata. The agent does **not** strip extension fields it doesn't recognise — they pass through verbatim, so a BIMcollab- or Revizto-emitted BCF survives a round trip unchanged. (The exception is [`convert`](../commands/convert.md) between versions, which is necessarily lossy 3.0→2.1.)
- **Encoding is normalised on read.** BCF XML is UTF-8 by spec, but some tools emit Windows-1252. The agent transparently transcodes to UTF-8 on read; writes are always UTF-8.
- **Snapshots are provenance, not live views.** The embedded PNG is captured at viewpoint-creation time. If the model changed since, the PNG won't match — this is BCF's design (provenance over freshness). Accept it; don't expect snapshots to reflect the current model.

## Common pitfalls

- **Topic GUID collisions on merge.** Merging two BCFs that share a source produces duplicate GUIDs; [`merge`](../commands/merge.md) dedupes by GUID (later source wins on conflicting fields, comments unioned). If both sources are equally current, scope each with [`filter.by-status`](../commands/filter.by-status.md) first.
- **Mixed versions don't merge.** A 2.1 + 3.0 merge is refused (`bcf.version-mismatch`) rather than silently degraded — [`convert`](../commands/convert.md) to a common version first.
- **`project.bcfp` is optional.** Many tools omit it; `project-name` then comes back `""`. Don't key logic on it being present.

## See also

- [bcf-21-vs-30](./bcf-21-vs-30.md) — what differs between the two versions
- [bcf-viewpoints-and-snapshots](./bcf-viewpoints-and-snapshots.md) — the camera + selection model
- [bcf-status-and-priority-vocabularies](./bcf-status-and-priority-vocabularies.md) — the free-text-field problem
- [bcf-round-trip-interop](./bcf-round-trip-interop.md) — what survives a trip through other tools
