# `ifc-reference-reader.read-model` — a file, or part of one, as reference geometry

Stateless, read-only. Tessellates every element into mesh scene objects — or only the storeys, IFC
types or GlobalIds you ask for — in the file's own **Z-up** world frame, in canonical millimetres.
Deterministic for a given file and filter, so the result is content-hash cacheable.

## Lifecycle
`single` — one call, one response.

## Inputs
| Field | Type | Description |
|---|---|---|
| `ifc-path` | string | Path to the `.ifc` / `.ifczip` file. |
| `max-vertices` | number | Optional vertex budget; aborts mid-tessellation if exceeded. |
| `storeys` | [string] | Optional storey names to read (from `probe.storeys`). Case-insensitive. |
| `ifc-types` | [string] | Optional IFC entity names to read, e.g. `[IFCBEAM, IFCCOLUMN]` (from `probe.types`). Subtypes included; case-insensitive. |
| `ids` | [string] | Optional GlobalIds to read — for re-reading a known selection. |
| `max-bytes` | number | Optional response-size budget. Absent means no limit. |

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
    colors:             # the file's own surface colours, as runs over `indices`. ABSENT — not [] —
                        # when the file authors no colour at all (see below)
      type: array
      items:
        rgba:  [number] # [r,g,b,a], 0..1, as the file styles it. a<1 is authored transparency
        start: number   # first index into `indices` this colour paints
        count: number   # how many indices
skipped: number         # products streamed but carrying no drawable triangle (see below)
count:   number         # objects returned; equals objects.length
colorsAvailable: boolean # whether this file authors ANY colour. Present from 1.3.0 always, so
                        # `false` (this file has none) and ABSENT (older bridge) stay different
selected:               # ONLY present when a filter was passed (see below)
  storeys:  [string]    # echoed back, as asked
  ifcTypes: [string]
  ids:      [string]
  candidates: number    # expressIDs the filter resolved to, before tessellation
  unmatched:  array     # [{storey}|{ifcType}|{id}] — values nothing matched
```

## Reading part of a file

`read-model` was all-or-nothing until 1.2.0, which on a real coordination model meant 289 MB of it or
nothing at all. `storeys`, `ifc-types` and `ids` narrow it.

```yaml
config:
  ifc-path: C:/models/tower.ifc
  storeys: [L2]              # one floor of twelve
  ifc-types: [IFCBEAM, IFCCOLUMN]
```

**Filters are applied before tessellation, not after.** They resolve to expressIDs which are handed to
web-ifc's `StreamMeshes`, so an element nobody asked for is never built. Filtering the output instead
would save the bytes and pay the whole cost — and the cost is the part that hurts: measured on a
17,460-object model, one storey reads in **5 s** where the whole file takes **73 s**.

**Several filters INTERSECT.** `storeys: [L2]` with `ifc-types: [IFCBEAM]` means the beams on L2, not
beams plus everything on L2. Every one of these narrows.

**An empty filter selects NOTHING.** `storeys: []` — or `[""]`, or a list that is all whitespace —
returns no objects, not the whole model. Only *omitting* the key means "unfiltered". The distinction
exists because the other reading fails in the one direction that must never happen: handing the entire
building to a caller who asked for a subset because it cannot afford the entire building. You can tell
the two apart in the response — an empty filter reports `selected.candidates: 0`, an absent one reports
no `selected` at all.

**A filter only ever selects a subset of what an unfiltered read returns.** web-ifc's whole-model walk
skips three classes and a filtered read skips the same three, so neither can hand you geometry the
other would not:

| Skipped | Why it would be wrong to return |
|---|---|
| `IfcOpeningElement`, `IfcOpeningStandardCase` | Voids — the prisms cut out of walls for doors and windows. One real 17,460-object model holds **740**; returned, they fill every doorway with a solid box. |
| `IfcSpace` | Room volumes. A `storeys:` filter would otherwise hand back the air in every room as geometry. |

Without this, one file would describe two different models depending on whether you narrowed it.

**A value that matches nothing is reported, not silently zero.** A misspelled storey selects no
objects, which is indistinguishable from a model that genuinely has none — so it comes back in
`selected.unmatched` and you can say "there is no storey called that" instead of "this model is empty".
This covers a *valid* entity the model lacks too: `IFCCHIMNEY` on a file with no chimneys is reported
unmatched, not silently accepted, because "you spelled it wrong" and "there are none" otherwise look
identical. `selected` is absent entirely when no filter was passed, so "unfiltered" and "a filter that
matched everything" stay different answers.

Run `probe` first for the names worth passing — it reports the storey and type breakdown without
tessellating anything.

## Size: the response is streamed, so there is no length ceiling

Until 1.2.0 the whole response was assembled with one `JSON.stringify`, and a string has a maximum
length. Real coordination models crossed it and died on V8's raw `Invalid string length` — with no way
out, because the only control on offer was `max-vertices` and the budget error told you to *raise* it
while raising it far enough to satisfy the file landed on the ceiling instead. Measured 2026-08-01,
three of five real files were unreadable at **every** budget (aware-aeco/aware#352).

The response is now written one object at a time as it is produced, so no single string approaches the
limit however large the model. Note that `objects` is emitted last in the JSON — `skipped`, `count` and
`selected` are only known once the walk is over, and buffering the geometry to put them first would
reintroduce the peak this removes. JSON object keys are unordered, so no conforming consumer notices.

`max-bytes` is an **opt-in** budget, absent by default. Streaming removed the ceiling that would make a
size cap compulsory, so imposing one here would invent a refusal; it exists because a *consumer* may
still have a ceiling of its own and would rather be told the size than meet it as an out-of-memory. It
refuses in words, naming the size, the limit, and that a subset would fit — counting **UTF-8 bytes**,
not string length, so a file full of non-ASCII property values cannot overrun the size it promised.

When honoured it comes back as a receipt, `budget: { maxBytes, bytes }`. That exists for the same
reason `selected` does: a caller passing only `max-bytes` otherwise had no way to tell a bridge that
enforced the budget from a pre-1.2.0 one that ignored the input, because both return a successful
response of the same shape.

A refusal mid-walk leaves a **truncated** document on stdout — bytes are already written by then — and
exits non-zero. That is safe because the runtime checks the exit status before parsing, and reports the
bridge's **stderr**, where the actionable sentence is.

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

## `colors` — the file's own colours, and the one thing white cannot tell you

A borrowed model rendered in one flat grey is legible as a silhouette and almost nothing else. The
file usually knows better: `IFC House.ifc` styles its 35 objects in 5 colours, `Steel IFC.ifc` its
77,118 in 16. So the surface colour web-ifc resolves rides along.

**Colour is per placed geometry, so it is emitted as RUNS over `indices`, not as a field on the
object.** One object routinely carries several: measured 2026-08-03, **6,358 of the 77,118 objects**
in `Steel IFC.ifc` and 1,127 of 14,409 in `Hospital Arch.ifc` are multi-coloured. A single colour per
object would render a bolted assembly entirely in its bolt's colour — a lie on 8% of an ordinary
steel export. Each run owns `[start, start+count)` of the index buffer, the runs tile it exactly with
no gap or overlap, and adjacent runs of one colour are merged (which on `Steel IFC.ifc` turns 206,621
placed geometries into 111,754 runs).

Channels are rounded to 4 decimal places. Colours are authored as `k/255` overwhelmingly often and 4
places round-trip those exactly, at 6 characters instead of the 19 a raw float costs.

### Absent means "this file has no colours" — because white cannot say it

**web-ifc reports opaque white for geometry nobody styled**, which is indistinguishable from a wall
the architect painted white. `example-steel-framing.ifc` carries zero style entities and every one of
its 13 objects reports `{1,1,1,1}`; `Building-Architecture.ifc` reports the same white for 6 objects
that are genuinely styled that way. A consumer handed white for both would paint an entirely unstyled
model glaring white and call it "the file's real colours".

So the question is asked of the **file**, once — does it carry any `IfcSurfaceStyle` at all? — and the
answer gates the whole field. No style anywhere means **no object carries `colors`**, and a consumer
renders its own default instead of a white nobody chose. `[]` would have been the wrong encoding for
the same reason `propertySets: []` is the right one there: empty is a claim about the object, absent
is a statement about what could be answered.

**Why the file and not the element.** The precise question — "was THIS geometry styled?" — was
implemented first and does not survive a real model. Resolving `IfcStyledItem.Item` against
`geometryExpressID` plus material-associated styles agreed with web-ifc on four small files and then
missed **11,257** genuinely-coloured geometries on `Steel IFC.ifc` and 370 on `Hospital Arch.ifc`;
web-ifc reaches colour through routes that reimplementation did not, and a resolver whose misses are
invisible is worse than none.

The cost of stopping there, stated rather than hidden: **an element that is unstyled inside a styled
file still reports white** — 696 of 31,381 placed geometries on `Hospital Arch.ifc`. That is web-ifc's
answer and this reader does not guess past it.

### `colorsAvailable` separates the two silences

A pre-1.3.0 bridge omits `colors` under a 1.3.0 manifest, exactly as an unstyled file does — so
absence alone conflates a permanent property of the file with a stale install that one
`aware sidecar install connection-reader` would fix. **`colorsAvailable` is emitted on every response
from 1.3.0**, so the three states stay distinct:

| | meaning | what a consumer does |
|---|---|---|
| `colorsAvailable: true` | the file authors colour; objects carry `colors` | paint them |
| `colorsAvailable: false` | this file authors none, and never will | paint your own default, and say so |
| field absent | the bridge predates colours | paint your own default; offer to refresh the bridge |

Rendering is the same in the bottom two rows, which is why the field is a *receipt* rather than a
control — but only one of them is repairable, and a UI that says "this file has no colours" about a
stale bridge is stating something false. Same reason `selected` and `budget` exist.

As with `propertySets`, a consumer caching this response **must not key the cache on the IFC bytes
alone**: identical bytes yield a different shape depending on the bridge that read them.

### The `IfcIndexedColourMap` case, and why it is not counted

IFC4 lets a tessellated face set carry per-face colour through `IfcIndexedColourMap` +
`IfcColourRgbList` with no `IfcSurfaceStyle` anywhere, so on paper the gate above has a false negative
there. Measured 2026-08-03 against web-ifc 0.0.77, with a hand-built IFC4 file — one
`IfcTriangulatedFaceSet`, four faces coloured red, green, blue and yellow, zero surface styles —
**web-ifc reports `{1,1,1,1}`**. It does not implement that route.

So counting the colour map in the gate would not recover those colours. It would switch the gate on
and publish web-ifc's *default white* as though the file had authored it, for a file that is in fact
brightly coloured — turning a correct "no colours here" into a confident lie. Suppressing is the
honest answer until the engine can answer, and this is the note to revisit if it ever does.

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

**The 1.2.0 fields carry the same caution, and one of them bites harder.** A pre-1.2.0 bridge under a
1.2.0 manifest omits `count` and `selected`, and — the dangerous one — **silently ignores `storeys`,
`ifc-types` and `ids`, returning the whole model.** A consumer that asked for one floor and cannot
afford the building must therefore check that it got what it asked for rather than assume: `selected`
present means the filter was understood, absent means it was not. That is the same absent-means-cannot-
answer rule, applied to an input rather than an output, and it is why `selected` is emitted at all
instead of the filter being a silent contract.

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
