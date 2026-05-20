# `ifc-inspector.psets.find-missing` — BEP-compliance pre-check

Stateless, read-only. For every entity of a class, reports which **required** property sets or properties are missing or empty. The lightweight "do all my spaces have the properties the BEP demands?" check — a yes/no gap report without standing up Solibri's rule engine.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `path` | string | `.ifc` / `.ifczip` path. |
| `class` | string | Exact IFC class to check, e.g. `"IfcSpace"`. |
| `required` | array | The contract: each item is `{ pset: string, properties: [string] }` — the Psets and the property names that must be present **and non-empty** on every entity. |

```yaml
required:
  - pset: Pset_SpaceCommon
    properties: [Reference, IsExternal, GrossPlannedArea]
  - pset: COBie_Space
    properties: [RoomTag, UsableHeight]
```

## Outputs

```yaml
missing:
  type: array
  items:
    guid:  string          # entity GlobalId
    class: string
    gaps:
      type: array
      items:
        pset:    string    # the required Pset that was absent OR had missing props
        missing: array     # property names absent or empty ("" = missing whole Pset)
```

Only entities with at least one gap appear in `missing`. An empty `missing` array means **every** entity of `class` satisfies the contract — the pass signal.

## Under the hood — what counts as "missing"

For each entity the agent resolves its effective Psets (instance Psets + inherited type Psets, instance wins — same resolution as [`entities.get-by-guid`](./entities.get-by-guid.md)) and checks the `required` contract:

- **Whole Pset absent** → a gap with `missing: [""]` (empty string marks the Pset itself, not a property).
- **Property absent** → its name in `missing`.
- **Property present but empty/null** → counts as missing. A door with `FireRating = ""` is non-compliant, not compliant. This is the distinction that separates this verb from [`psets.export`](./psets.export.md) (which would just show a blank cell).

This is intentionally **not** a rule engine: it checks presence and non-emptiness, not values. "Does every space have a `GrossPlannedArea`?" → yes. "Is every space's area within code minimums?" → that is a Solibri rule, use the [`solibri`](../../../architecture/solibri-26.4.1/) agent. See [ifc-psets-and-cobie](../skills/ifc-psets-and-cobie.md).

## Composition example — gate COBie issuance

```yaml
- id: gaps
  agent: ifc-inspector
  command: psets.find-missing
  config:
    path:  "{{ inputs.ifc }}"
    class: "IfcSpace"
    required:
      - { pset: Pset_SpaceCommon, properties: [Reference, IsExternal] }
      - { pset: COBie_Space,      properties: [RoomTag] }

- id: gate
  inline:
    kind: assert
    description: Block COBie export until every space is complete
    code: g => g.missing.length === 0
            || `${g.missing.length} spaces fail BEP Pset requirements — fix before issuing COBie`
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `ifc.file-not-found` | `path` doesn't exist | Check the path |
| `ifc.unknown-class` | `class` invalid for the schema | Check spelling/schema |
| `ifc.empty-required` | `required` is empty | Nothing to check — provide at least one `{pset, properties}` |
| `ifc.no-entities` | No entities of `class` exist | Returns `missing: []`; that's "vacuously compliant" — confirm the class is present with [`entities.count-by-class`](./entities.count-by-class.md) |

## See also

- [`psets.export`](./psets.export.md) — the raw data; this verb is the pass/fail layer on top
- [ifc-psets-and-cobie](../skills/ifc-psets-and-cobie.md) — COBie required-property contracts
- [what-this-agent-does](../skills/what-this-agent-does.md) — when this is enough vs when you need `solibri`
