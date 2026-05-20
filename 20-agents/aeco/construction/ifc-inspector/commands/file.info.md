# `ifc-inspector.file.info` — header-only pre-flight

Stateless, read-only. Reads the IFC `HEADER` section without loading geometry — fast even on a 500 MB file (~1s). Returns schema version, originating application, timestamp, total entity count, and file size. The "is this IFC valid enough to bother federating?" gate you run before anything expensive.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `path` | string | Absolute or workspace-relative path to a `.ifc` or `.ifczip` file. |

## Outputs

```yaml
path:         string
schema:       string    # "IFC2X3" | "IFC4" | "IFC4X3"
source-app:   string    # FILE_NAME's OriginatingSystem, e.g. "Autodesk Revit 2025 (ENU)"
timestamp:    string    # FILE_NAME's TimeStamp (ISO-8601)
entity-count: number    # total instances in the DATA section
size-bytes:   number
```

## Under the hood

The IFC `HEADER` is a small, fixed block at the top of the STEP file — the agent reads it without walking the `DATA` section:

```
ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('ViewDefinition [CoordinationView]'),'2;1');
FILE_NAME('proj.ifc','2026-05-17T09:00:00',('P.Lisowski'),
          ('ACME'),'IfcOpenShell','Autodesk Revit 2025 (ENU)','');
FILE_SCHEMA(('IFC4'));
ENDSEC;
DATA;
  #1=IFCPROJECT(...);
  ...
```

`schema` comes from `FILE_SCHEMA`; `source-app` and `timestamp` from `FILE_NAME` (positions 6 and 2 respectively). `entity-count` is the count of `#n=` lines in `DATA` — obtained via `web-ifc` `OpenModel` + a line-count pass, which is cheap because no geometry is meshed. `.ifczip` is unzipped to a temp stream first.

`source-app` is the single most useful field for triage: it tells you which authoring tool produced the file, which predicts the GUID-stability and Pset conventions you'll hit downstream (see [ifc-guid-and-class-model](../skills/ifc-guid-and-class-model.md)).

## Composition example — gate a federation pipeline

```yaml
- id: info
  agent: ifc-inspector
  command: file.info
  config: { path: "{{ inputs.ifc }}" }

- id: gate
  inline:
    kind: assert
    description: Refuse pre-IFC4 files this pipeline doesn't support
    code: i => i.schema !== "IFC2X3" || "This pipeline requires IFC4+; got IFC2X3 from " + i["source-app"]

- id: count-spaces
  agent: ifc-inspector
  command: entities.count-by-class
  config: { path: "{{ inputs.ifc }}", filter: "IfcSpace" }
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `ifc.file-not-found` | `path` doesn't exist | Check the path |
| `ifc.not-an-ifc` | No `ISO-10303-21;` magic / no `FILE_SCHEMA` | Confirm it's a STEP IFC, not an `.ifcXML` or a renamed RVT |
| `ifc.unsupported-schema` | `FILE_SCHEMA` is something `web-ifc` can't open (e.g. IFC4X1) | Re-export from the authoring tool to IFC4 or IFC2X3 |

## See also

- [what-this-agent-does](../skills/what-this-agent-does.md) — scope, performance, and when to use this vs `solibri`
- [ifc-schemas-and-validation](../skills/ifc-schemas-and-validation.md) — what the schema strings mean
- [`entities.count-by-class`](./entities.count-by-class.md) — the natural next step after a clean pre-flight
