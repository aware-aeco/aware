# `ifc-inspector.entities.count-by-class` — histogram of entities by IFC class

Stateless, read-only. Counts every entity grouped by its IFC class. The "is this model what I expect?" sanity check — should the architectural IFC have 5,000 `IfcWall`s or 50? An empty `IfcSpace` count means the rooms didn't export. Optionally narrowed by a class-name substring.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `path` | string | `.ifc` / `.ifczip` path. |
| `filter` | string (optional) | Case-insensitive substring against the class name. `"IfcSpace"` → only spaces; `"Wall"` → `IfcWall` + `IfcWallStandardCase` + `IfcCurtainWall`; omit → every class. |

## Outputs

```yaml
counts:
  type: array
  items:
    class: string    # e.g. "IfcWallStandardCase"
    count: number
```

Sorted by `count` descending — the classes that dominate the model surface first.

## Under the hood

`web-ifc` indexes instances by type id. The agent enumerates the model's present types and counts the line ids for each:

```js
const counts = [];
for (const typeId of api.GetAllTypesOfModel(modelID)) {
  const ids = api.GetLineIDsWithType(modelID, typeId.typeID);
  if (ids.size() > 0) counts.push({ class: typeId.typeName, count: ids.size() });
}
// filter substring applied to typeName, then sort by count desc
```

This counts **direct class membership**, not subtype rollups (`GetLineIDsWithType` defaults to exact-type): `IfcWall` and `IfcWallStandardCase` are reported as separate rows. To total "all walls," sum the matching rows — that's what the `filter` substring is for. **But the substring is a dumb text match**: `"Wall"` also sweeps in `IfcCurtainWall`, which is a *different element category* (a curtain-wall assembly, not a layered wall) — don't blindly fold it into a wall area takeoff. When precision matters, enumerate the concrete classes you want with [`entities.list-guids`](./entities.list-guids.md) (one exact class per call) rather than trusting the substring. See [ifc-guid-and-class-model](../skills/ifc-guid-and-class-model.md) for the subtype gotcha.

## Composition example — sanity-check a discipline IFC

```yaml
- id: counts
  agent: ifc-inspector
  command: entities.count-by-class
  config: { path: "{{ inputs.arch-ifc }}", filter: "IfcSpace" }

- id: gate
  inline:
    kind: assert
    description: Architect's IFC must contain rooms or the COBie deliverable is impossible
    code: c => c.counts.reduce((n, r) => n + r.count, 0) > 0
            || "No IfcSpace in this IFC — rooms weren't exported. Reject."
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `ifc.file-not-found` | `path` doesn't exist | Check the path |
| `ifc.not-an-ifc` | Not a STEP IFC | See [`file.info`](./file.info.md) |
| `ifc.empty-result` | `filter` matched no class | Returns `counts: []` (not an error); widen or drop the filter |

## See also

- [`entities.list-guids`](./entities.list-guids.md) — drill from a count into the actual GUIDs
- [ifc-guid-and-class-model](../skills/ifc-guid-and-class-model.md) — direct class vs subtype rollup
- [`file.info`](./file.info.md) — confirm the file opens before counting
