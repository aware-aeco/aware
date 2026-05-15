# `tekla.insert` — create a connection in the model

Stateless command. Creates a single `ConnectionPart` at a world-space position, attached to the named beams. Writes are wrapped in a Tekla transaction; the model is committed on success and rolled back on failure.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `type` | string | Tekla connection catalog name (e.g. `"End plate 144"`, `"Welded gusset 27"`). |
| `position` | object `{x, y, z}` | **World-space** coordinates. See [coordinate-systems skill](../skills/coordinate-systems.md). |
| `beams` | array of string | GUIDs of beams to attach. The connection's primary + secondaries are resolved by the connection type's rules. |
| `attributes` | object (optional) | Connection-specific parameter overrides. Maps to the connection dialog's named fields. |

## Outputs

```yaml
guid: string        # GUID of the created ConnectionPart
warnings: array     # non-fatal issues (e.g. beams not perpendicular)
```

## Generated code (Smart Node form)

```csharp
using (var tx = new Transaction(model, "AWARE · tekla.insert"))
{
    tx.Start();

    var conn = new ConnectionPart(connectionType)
    {
        Position = worldPt,                 // world-space! see coordinate-systems
        PrimaryBeam = ResolveBeam(beams[0]),
        SecondaryBeams = beams.Skip(1).Select(ResolveBeam).ToArray()
    };

    if (attributes != null)
        foreach (var (key, val) in attributes)
            conn.SetAttribute(key, val);

    if (!conn.Insert())
    {
        tx.Rollback();
        return Error("tekla.insert-failed", lastError);
    }

    model.CommitChanges();
    tx.Commit();
    return Ok(new { guid = conn.Identifier.GUID, warnings = conn.GetWarnings() });
}
```

## Coordinate-space contract

The `position` input **must be world-space coordinates**. If you're piping from a PDF extraction or a drawing pick, you have to convert first. Don't pass drawing-space coordinates directly. See [coordinate-systems.md](../skills/coordinate-systems.md).

A common safe pattern in composition:

```yaml
- id: pdf-extract
  agent: think-node
  command: extract
  outputs: { drawingPt: object, transform: object }   # captures both

- id: to-world
  inline:
    kind: map
    code: |
      ({drawingPt, transform}) => ({
        position: applyTransform(drawingPt, transform.inverse)
      })

- id: tekla-insert
  agent: tekla
  command: insert
  config:
    type: "End plate 144"
    position: "{{ to-world.position }}"
    beams: ["{{ ctx.primaryBeam }}", "{{ ctx.secondaryBeam }}"]
```

The intermediate `to-world` glue makes the coordinate conversion **explicit and visible in the topology** — so anyone auditing the app can see it happened.

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `tekla.insert-failed` | Tekla rejected the insert (geometry impossible, beam GUID invalid, type not in catalog) | Inspect `warnings`; fix and retry. Transaction is rolled back automatically. |
| `tekla.world-space-required` | Position appears to be in drawing space (z=0 or magnitude very small relative to model) | Convert first. The agent rejects ambiguous coordinates rather than silently misplacing the connection. |
| `tekla.transaction-conflict` | Another writer (UI or another agent) is mid-transaction | Retry after 200ms backoff. Agent retries up to 3 times before surfacing. |

## See also

- [coordinate-systems.md](../skills/coordinate-systems.md) — the critical conversion
- [drawing-identity.md](../skills/drawing-identity.md) — when working with marks downstream of insert
- [tekla.watch](./watch.md) — the read-side counterpart
