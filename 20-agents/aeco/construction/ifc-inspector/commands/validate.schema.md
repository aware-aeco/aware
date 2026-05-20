# `ifc-inspector.validate.schema` — validate against the declared IFC schema

Stateless, read-only. Validates the file against the EXPRESS schema it declares in its header (IFC2x3 / IFC4 / IFC4X3) and reports schema-level errors — missing required attributes, type mismatches, orphan references. This is **structural** validity, not BEP compliance: it answers "is this a well-formed IFC?", not "does it meet our project requirements?"

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `path` | string | `.ifc` / `.ifczip` path. |

## Outputs

```yaml
schema: string         # the schema it was validated against, from FILE_SCHEMA
valid:  boolean        # true iff zero Error-severity issues
errors:
  type: array
  items:
    line:     number   # the #n STEP line id of the offending instance
    severity: string   # "error" | "warning"
    message:  string
```

## What it checks (and what it doesn't)

Validation runs against buildingSMART's published EXPRESS schemas — IFC2x3-TC1, IFC4-Add2-TC1, IFC4X3-ADD2. It catches:

- **Missing required attributes** — e.g. an `IfcWall` with no `GlobalId`.
- **Type mismatches** — a `Real` where the schema demands an `Integer`, a string where an enum is required.
- **Orphan references** — e.g. an `IfcRelAggregates` pointing at a deleted `#n`.
- **Circular containment** — rare, but it happens and breaks traversal.

It does **not** check:

- BEP compliance, classification, cross-discipline coordination → the [`solibri`](../../../architecture/solibri-26.4.1/) agent's rule engine.
- "The wall thickness is wrong" → no rule; this verb has no notion of correct values.
- "This `IfcSpace` has no name" → that's a property gap, use [`psets.find-missing`](./psets.find-missing.md).
- Georeferencing correctness → use [`georef.check`](./georef.check.md).

`valid: true` means *well-formed*, not *good*. A file can be perfectly schema-valid and still be useless for coordination. See [ifc-schemas-and-validation](../skills/ifc-schemas-and-validation.md) for where schema validation sits in the overall quality stack.

## Performance

Full validation walks the entire entity graph. On a 500 MB IFC this is ~5 minutes (vs ~1s for [`file.info`](./file.info.md)). For routine large-federation work, schedule it overnight via the `schedule:` primitive rather than running it inline in an interactive app.

## Composition example — quarantine malformed uploads

```yaml
- id: check
  agent: ifc-inspector
  command: validate.schema
  config: { path: "{{ inputs.uploaded-ifc }}" }

- id: route
  inline:
    kind: branch
    description: Valid files proceed; malformed files are quarantined with the error list
    code: c => c.valid ? "accept" : "quarantine"

- id: quarantine
  agent: file
  command: write
  when: "{{ route == 'quarantine' }}"
  config:
    path:  "./quarantine/{{ inputs.upload-name }}.errors.json"
    bytes: "{{ check.errors }}"
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `ifc.file-not-found` | `path` doesn't exist | Check the path |
| `ifc.unsupported-schema` | `FILE_SCHEMA` is one with no published EXPRESS schema bundled (e.g. IFC4X1) | Re-export to IFC4 or IFC2X3 |
| `ifc.parse-abort` | File is so malformed `web-ifc` can't open it at all | This is worse than `valid: false` — the STEP syntax itself is broken; re-export |

## See also

- [ifc-schemas-and-validation](../skills/ifc-schemas-and-validation.md) — the schema family + where validation sits in the quality stack
- [`psets.find-missing`](./psets.find-missing.md) — property-level checks (a different question)
- [what-this-agent-does](../skills/what-this-agent-does.md) — this vs `solibri`'s rule engine
