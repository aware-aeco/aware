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
frame: string       # "z-up" — the frame `positions` are in. Check this, not a version (see below)
objects:
  type: array
  items:
    id:        string   # GlobalId
    name:      string   # as authored, may be null (one real file names everything "-")
    ifcType:   string   # "IFCBEAM" | "IFCBUILDINGELEMENTPROXY" | …
    storey:    string   # containing spatial structure name, may be null
    profile:   string   # profile designation VERBATIM, may be null
    material:  string   # material name, may be null
    propertySets:       # every IfcPropertySet, verbatim — always present, [] when none
      type: array
      items:
        name:       string   # the set name as authored ("AllplanAttributes", "Pset_SlabCommon")
        properties:          # [{ name, value }] — value is text as written, or null
          type: array
    positions: [number] # world mm, [x,y,z]*
    indices:   [number] # triangle indices
skipped: number         # products streamed but carrying no drawable triangle (see below)
```

## The frame is the file's own, Z-up — do not rotate it again

`positions` are in IFC's world frame: **X and Y in plan, Z up**, millimetres, origin as the file has
it — and the output says so: `frame` is `"z-up"`. `probe`'s `bbox` reports the same `frame`, so the
cheap box and the expensive mesh share axes and units, which is what makes the box usable to
sanity-check the mesh ("is this 1000× off?"). They do **not** share one coherent world origin: probe
unions the file's raw points across their several local coordinate systems, so a placed mesh can sit
outside the box (see `probe.md`). Compare scale and axes, not containment.

This costs a rotation, because web-ifc does not give it to you. web-ifc bakes a fixed
`(x, y, z) → (x, z, −y)` normalisation into every mesh transform, so its tessellation lands in the
**Y-up** frame a renderer wants. Until `1.0.0` this command passed that through while documenting the
file's frame, and the two commands answered in different frames: measured on `example-steel-framing.ifc`
(a 12 m × 6 m grid on 4500 mm columns) `probe` reported `12000 × 6000 × 4500` while the mesh measured
`12150 × 4625 × 6150` — the height in Y. Every reference model rendered on its side, and the first
consumer to hit it added a compensating rotation of its own.

**If you carry such a workaround, delete it.** With `1.0.0` the same file now measures
`12150 × 6150 × 4625` — probe's box, axis for axis. See `BREAKING.md`.

**Check `frame`, not a version.** No version number can answer "which frame did I just get". The
geometry is produced by the `aware-connection-reader` bridge binary, installed separately from the
agent (`aware sidecar install connection-reader`); a stale bridge only prints a warning and runs
anyway, so this manifest can read `1.0.0` while an old bridge returns Y-up. Measured 2026-08-01: an
app's `requires:` pin is enforced neither at compile nor at run time either. The `frame` field is
produced by the binary that produced the vertices, which is why it is the one to trust.

Handing these meshes to `viewer-3d.render`, declare the frame you are in: `meta.up: "z"`. The scene
schema keeps coordinates in producer space and converts via `meta.up`, so the mesh renders upright
with nobody rotating vertices. (`"y"` was right for this command before `1.0.0`, and is still right
for `extract`.)

`ifc.write` needs no help: it emits `positions` verbatim as absolute IFC coordinates and does not read
`meta.up` (`cli/src/render/ifc.rs`), and IFC is Z-up — so these meshes round-trip upright. Under
`0.1.0` that same composition silently wrote a sideways file.

`connection-reader.extract` is a **different** agent with a different contract: its `parts` are still
in web-ifc's Y-up frame and its own `frame` field says `"y-up"` (see
`connection-reader/commands/extract.md`). Read each command's frame from its own output; do not carry
an assumption across.

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

## `propertySets` is where the meaning is in a real file

The five well-known fields describe an object the exporter bothered to describe. Plenty do not.
`11134_V_Motebello_Heistopp_Rev.ifc` is the case that settles it: every one of its 19 objects is an
`IFCBUILDINGELEMENTPROXY`, so `ifcType` separates nothing, and the names are no help either — while
**31 property sets carry 271 values**, in Norwegian, under vendor set names. A consumer given only the
well-known fields is handed an object tree in which nothing can be told apart.

So the element's `IfcPropertySet`s ride along, **grouped and named exactly as the file wrote them**:

```yaml
propertySets:
  - name: AllplanAttributes
    properties:
      - { name: "2:Etasje", value: "1" }
      - { name: "6:Max VCT", value: "0" }
```

**Nothing is normalised, translated or renamed.** Vendor sets in the author's own language are the
norm rather than the exception, and showing them as authored is the entire value — a reader that
tidied them would be discarding the only thing that distinguishes one proxy from another. No unit
conversion, no localisation, no renaming of sets or properties.

**What a value is, precisely: its canonical textual form, not the file's literal tokens.** Values
arrive through web-ifc already parsed, so an authored `IFCREAL(1.2300)` reaches us as the number
`1.23` and is rendered `"1.23"`. Lexical preservation would need the raw STEP tokens, which this
reader never sees. So: exact for text, codes and identifiers — which is what property sets mostly
carry — and canonical for numerics. Two known edges: an `IfcLogical` of `UNKNOWN` currently reads as
`null`, indistinguishable from "no value"; and `IfcPropertyReferenceValue` / `IfcPropertyTableValue`
degrade to `null` rather than being rendered.

`propertySets` is `[]` when the file carries none, so a consumer renders "no properties" rather than
having to tell absent from empty. **A missing field is a different statement from an empty one**, and
it is possible: the geometry comes from a separately-installed bridge binary, a stale one only warns
before running, and a pre-1.1.0 bridge omits `propertySets` entirely under a 1.1.0 manifest. Treat
absent as *"this bridge cannot answer"* — exactly as you already must for `frame` — and refresh with
`aware sidecar install connection-reader`. For the same reason a consumer caching this response
**must not key the cache on the IFC bytes alone**: identical bytes yield a different shape depending
on the bridge that read them.

**Occurrence and type sets are merged property by property, the occurrence winning.** IFC lets a
property sit on the element type with each occurrence inheriting it, so following only
`IfcRelDefinesByProperties` returns nothing for perfectly ordinary exports. Merging at the *set* level
instead would silently drop a type property the occurrence never mentioned — a type
`Pset_WallCommon{FireRating, LoadBearing}` beside an occurrence `Pset_WallCommon{FireRating}` would
lose `LoadBearing`, which the file plainly states. This is also the rule
[`ifc-inspector.entities.get-by-guid`](../../../construction/ifc-inspector/commands/entities.get-by-guid.md)
applies; two agents reading one file must not disagree about what it says.

**Which attachments are followed.** Direct occurrence sets (`IfcRelDefinesByProperties`, including
IFC4's aggregate form where one relationship carries several sets) and type sets (`IfcRelDefinesByType`
→ the type's `HasPropertySets`). **`IfcRelDefinesByObject` is NOT followed** — an object declaring
another's properties by reflection. It is valid IFC4 but uncommon and outside the standard subsets, and
its precedence against direct occurrence properties is not something this reader can settle without a
real file to measure; a reflected object therefore reports only what it carries directly.

Quantity sets (`IfcElementQuantity`) are **not** included — this is `IfcPropertySet` only, matching
`ifc-inspector`.

**Cost.** Properties are read for every element the file relates, before any mesh is streamed, and
shared type sets are repeated per occurrence in the response — on the Motebello sample that is ~20% of
the serialised bytes. `max-vertices` bounds geometry and does not bound this. A federated model with
heavily reused type sets can therefore carry materially more metadata than its vertex count suggests.
No property budget is applied: silently truncating what a consumer asked for would be worse than the
weight, and an aborting budget would refuse files that load correctly today.

### Why this is here and not only in `ifc-inspector`

It overlaps that agent deliberately, and the **shape** is what differs. `ifc-inspector` answers
per-GUID (`entities.get-by-guid`) or exports a whole class to CSV (`psets.export`), on its own binary.
An interactive object tree needs every object's properties out of the **one read it already pays
for**: a call per click would mean a second sidecar download and an agent round trip on every
selection. Reach for `ifc-inspector` when you want one element, a schedule, or a compliance check;
reach for this when you are drawing a tree of a whole borrowed model.

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
