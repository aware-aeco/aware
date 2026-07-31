# `ifc-reference-reader.read-model` — the whole file as reference geometry

Stateless, read-only. Tessellates **every** element into mesh scene objects, in the file's own world
frame, in canonical millimetres. Deterministic for a given file, so the result is content-hash
cacheable.

## Lifecycle
`single` — one call, one response.

## Inputs
| Field | Type | Description |
|---|---|---|
| `ifc-path` | string | Path to the `.ifc` / `.ifczip` file. |
| `max-vertices` | number | Optional vertex budget; aborts mid-tessellation if exceeded. |

## Outputs
```yaml
objects:
  type: array
  items:
    id:        string   # GlobalId
    name:      string   # as authored, may be null (one real file names everything "-")
    ifcType:   string   # "IFCBEAM" | "IFCBUILDINGELEMENTPROXY" | …
    storey:    string   # containing spatial structure name, may be null
    profile:   string   # profile designation VERBATIM, may be null
    material:  string   # material name, may be null
    positions: [number] # world mm, [x,y,z]*
    indices:   [number] # triangle indices
```

## Two fields that are load-bearing, not decoration

**`profile` is the designation verbatim** — `"W10x33"`, lowercase `x` and all. A section must be
looked up by **name** and never measured off the mesh: exports routinely write a simplified box for a
real profile. In one of the test files `W10x33` is written as a 150 × 250 mm box while the true
section is 247 × 202 — measuring would come out ~25% narrow on the flange, and nothing on screen
would look wrong.

Names are **not normalised** here, because normalising at the reader would hide how much designation
spelling varies between exporters. The consequence belongs to the consumer: match the catalogue
case-insensitively, or `"W10x33"` will miss a catalogue storing `"W10X33"`.

**`material` is the signal that says *do not convert this*.** A member can be named `girder`, typed
`IfcBeam`, and be `wood_spruce_beam`. Type alone would happily turn timber into steel.

## Instancing is preserved
Real exports serve many objects from few shapes via mapped items. Geometry is emitted **per
instance**, each with its own transform. De-duplicating by shape would silently drop objects — one
test file serves 19 objects from 14 shapes, so de-duplicating loses five walls.

## `max-vertices` is a circuit breaker, not a gate
The budget decrements as meshes stream and the command aborts **mid-walk**. It is deliberately not a
preflight check: an exact vertex count cannot be known before tessellating, so a cap applied after
this command returns would only fire once the payload that causes the freeze had already been built
and serialised. Use `probe` plus a file-size check for true preflight protection.

## Failure modes
| Error | Cause | Recovery |
|---|---|---|
| `ifc-path is required` | no path given | pass `ifc-path` |
| `ENOENT …` | file not found | check the path |
| `too complex to load as a reference model` | exceeded `max-vertices` | raise the budget, or use a lighter export |
