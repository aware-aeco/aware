# `ifc-inspector.entities.list-guids` — every GUID of a given class

Stateless, read-only. Lists the IFC `GlobalId` (and Name) of every entity of one class. The cross-tool reconciliation verb: "do my MEP IFC's space GUIDs match the architect's room IDs?" — because BCF viewpoints, clash reports, and COBie all key on these GUIDs.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `path` | string | `.ifc` / `.ifczip` path. |
| `class` | string | Exact IFC class name, e.g. `"IfcSpace"`, `"IfcDoor"`. Case-insensitive; **not** a substring — one class per call. |

## Outputs

```yaml
class: string
guids:
  type: array
  items:
    guid: string    # the 22-char IFC GlobalId (compressed base64 form, e.g. "2O2Fr$t4X7Zf8NOew3FNr2")
    name: string    # the entity's Name attribute, or "" if unset
```

## Under the hood

```js
const typeId = api.GetTypeCodeFromName(class);          // "IfcSpace" -> numeric type
const ids = api.GetLineIDsWithType(modelID, typeId);
const out = [];
for (let i = 0; i < ids.size(); i++) {
  const e = api.GetLine(modelID, ids.get(i));
  out.push({ guid: e.GlobalId.value, name: e.Name?.value ?? "" });
}
```

The GUID is the IFC **`GlobalId`** — a 22-character base64-compressed 128-bit UUID, *not* the integer `#n` STEP line id. The `GlobalId` is what travels between tools and is meant to be stable across re-exports; the line id is meaningless outside the one file. Always reconcile on `guid`. Whether the GUID actually stays stable across a re-export depends on the authoring tool — see [ifc-guid-and-class-model](../skills/ifc-guid-and-class-model.md).

`class` is exact (one class per call) — unlike [`entities.count-by-class`](./entities.count-by-class.md)'s substring `filter`. To list all wall-ish entities, call once per concrete class (`IfcWall`, `IfcWallStandardCase`).

## Composition example — find rooms the architect has but MEP doesn't

```yaml
- id: arch-spaces
  agent: ifc-inspector
  command: entities.list-guids
  config: { path: "{{ inputs.arch-ifc }}", class: "IfcSpace" }

- id: mep-spaces
  agent: ifc-inspector
  command: entities.list-guids
  config: { path: "{{ inputs.mep-ifc }}", class: "IfcSpace" }

- id: orphans
  inline:
    kind: set-difference
    description: Rooms present in the architect's model but missing from MEP's
    code: ({arch, mep}) => arch.filter(a => !mep.some(m => m.guid === a.guid))
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `ifc.file-not-found` | `path` doesn't exist | Check the path |
| `ifc.unknown-class` | `class` is not a valid IFC class for the file's schema | Check spelling/schema; `IfcSpace` exists in all, but some classes are IFC4-only |
| `ifc.empty-result` | Class is valid but no instances exist | Returns `guids: []` (not an error) |

## See also

- [`entities.get-by-guid`](./entities.get-by-guid.md) — resolve one of these GUIDs to its full record
- [`entities.count-by-class`](./entities.count-by-class.md) — counts first, GUIDs second
- [ifc-guid-and-class-model](../skills/ifc-guid-and-class-model.md) — GlobalId encoding + cross-tool stability
