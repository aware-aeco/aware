# `ifc-reference-reader.read-model` — the whole file as reference geometry

Stateless, read-only. Tessellates **every** element into mesh scene objects, in the file's own **Z-up**
world frame, in canonical millimetres. Deterministic for a given file, so the result is content-hash
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
skipped: number         # products streamed but carrying no drawable triangle (see below)
```

## The frame is the file's own, Z-up — do not rotate it again

`positions` are in IFC's world frame: **X and Y in plan, Z up**, millimetres, origin as the file has
it. That is the same frame `probe`'s `bbox` reports, so the cheap box and the expensive mesh are
directly comparable — which is the whole point of having both.

This costs a rotation, because web-ifc does not give it to you. web-ifc bakes a fixed
`(x, y, z) → (x, z, −y)` normalisation into every mesh transform, so its tessellation lands in the
**Y-up** frame a renderer wants. Until `0.2.0` this command passed that through while documenting the
file's frame, and the two commands answered in different frames: measured on `example-steel-framing.ifc`
(a 12 m × 6 m grid on 4500 mm columns) `probe` reported `12000 × 6000 × 4500` while the mesh measured
`12150 × 4625 × 6150` — the height in Y. Every reference model rendered on its side, and the first
consumer to hit it added a compensating rotation of its own.

**If you carry such a workaround, delete it.** With `0.2.0` the same file now measures
`12150 × 6150 × 4625` — the same box as `probe`, axis for axis. The agent's `version` moved to `0.2.0`
because the output changed, so `aware agent list` (or the catalogue's `manifest-version`) tells you
which frame the installed copy returns. It cannot *gate* the change: the registry index key is a
separate axis and its tarball tracks `main`, so an install spec of `0.1.0` still fetches this code.
Check the version rather than assume it.

Handing these meshes to `viewer-3d.render`, declare the frame you are actually in:
`meta.up: "z"`. The scene schema keeps coordinates in producer space and uses `meta.up` to convert to
the renderer's own up-axis, so the mesh renders upright without anyone rotating vertices. (`"y"` was
right for this command before `0.2.0`, and is still right for `extract` — see below.)

`connection-reader.extract` is a **different** command with a different contract: its `parts` are still
in web-ifc's Y-up tessellation frame (see `connection-reader/commands/extract.md`). Read each command's
frame from its own contract; do not carry an assumption across.

## Nothing is dropped silently
A product web-ifc streams but that yields fewer than 3 vertices or 1 triangle cannot be drawn, and is
excluded — an object that loads but renders as nothing is worse than an absent one, because it looks
like success. Those exclusions are **counted in `skipped`** rather than quietly missing, so a consumer
comparing this against `probe`'s element count can account for the difference instead of guessing.

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
The budget decrements as meshes stream and the command aborts **mid-walk**. Note the check happens
once a whole product has been tessellated, so a single enormous object can overshoot the budget by its
own size before the breaker fires; the cap bounds the total, not each step. It is deliberately not a
preflight check: an exact vertex count cannot be known before tessellating, so a cap applied after
this command returns would only fire once the payload that causes the freeze had already been built
and serialised. Use `probe` plus a file-size check for true preflight protection.

## Failure modes
| Error | Cause | Recovery |
|---|---|---|
| `ifc-path is required` | no path given | pass `ifc-path` |
| `ENOENT …` | file not found | check the path |
| `too complex to load as a reference model` | exceeded `max-vertices` | raise the budget, or use a lighter export |
