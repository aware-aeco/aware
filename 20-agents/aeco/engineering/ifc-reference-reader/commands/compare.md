# `ifc-reference-reader.compare` — two versions of one file, and what changed between them

A project sends you IFC after IFC as it develops. You already have the reference; the question the next
file raises is not *what is in it* but *what moved*. `compare` answers that: it matches the objects of
two versions of one file and reports what was **added**, **removed** and **changed** — or refuses,
loudly, when the two files share no identity to match on.

It is **metadata only**. No positions, no indices. See [Removed geometry](#removed-geometry-is-a-read-model-call-not-an-output-of-this-one).

## Lifecycle

`single`. Read-only. Deterministic for a given pair of files and a given comparison set, so the result
is content-hash cacheable exactly as `read-model`'s is.

## Inputs

| Input | Meaning |
|---|---|
| `base-ifc-path` | The version you already have |
| `revised-ifc-path` | The version that just arrived |
| `storeys`, `ifc-types`, `ids` | Applied to **both** sides — a filtered reference compares like-for-like |
| `base-*` / `revised-*` variants | Per-side overrides. See [Per-side filters](#per-side-filters-exist-so-a-not-like-for-like-comparison-can-run-at-all) |
| `position-tolerance-mm` | Default 1. Applies to `location` **and** `geometry` |
| `include-unchanged` | Default false |
| `max-vertices`, `max-bytes` | As `read-model`, applied per side |

## Why GlobalId only

The first design of this command had two passes: ids, then a shape-and-position fingerprint for whatever
the ids could not pair. The second pass was cut after checking what the established tools actually do,
and the evidence pointed one way.

| Tool | Matches on | Fallback when ids fail |
|---|---|---|
| IfcOpenShell `ifcdiff` | GlobalId only | none |
| Tekla reference model change detection | GUID | **manual linking** ("Link Objects in Model…") |
| Solibri Model Comparison | GUID first, then geometry/location | it *is* the fallback |

**Solibri's fallback is the thing they shipped a switch to turn off.** Their default is exactly the
rejected two-pass design; in 9.7.15 they added *"Identify components only with GUID"*, which disables it.
A mature product adding an escape hatch in the direction of *give me the strict answer* is a scar, not
an endorsement.

**And the premise was partly wrong.** Research on IFC version control finds exporters usually keep GUIDs
stable for `IfcElement` nodes across runs, with the instability concentrated in `IfcSpatialElement`,
`IfcObjectType`, `IfcProperty` and `IfcRelationship`. This reader returns only placed elements. The
failure the second pass defended against — wholesale regeneration on re-export — is uncommon in exactly
the class of object being compared, and it was being conflated with *absence* and *duplication*, which
are different problems that geometry does not solve either. Those two need honesty, not a fingerprint:
see [`uncomparable`](#uncomparable-is-not-a-footnote).

Measured against five real coordination models totalling 137,752 objects: **not one blank GlobalId**, and
duplication confined to 728 ids in a single 77k-object file (0.53% of the corpus). On a real Tekla
revision pair, **3,232 of 3,232 objects paired.**

If real files ever prove GlobalIds insufficient, the next step is **manual linking** — what Tekla does —
not a shape fingerprint.

## `globalId`, and why `id` is not it

`read-model` objects carry both, and a comparison must use `globalId`.

```
id:       the file's GlobalId, OR the expressID when the file records none
globalId: the file's GlobalId, or null
```

The fallback on `id` is right for what `id` is for — a consumer needs *something* to address an object
by, and an object with no identity still has to be selectable. It is catastrophic for comparing two
files. An expressID is a **file-local sequence number**: two exports of one model share a great many, and
they identify nothing in common. A comparison keyed on `id` would pair unrelated objects and present the
result as a matched diff — the exact confident lie the refusal exists to prevent, entering through the
reader instead of the algorithm. Nothing downstream can detect it either, because a substituted id looks
perfectly non-empty and unique.

`globalId` is present from 1.6.0. **Absent means a pre-1.6.0 bridge, not a file without ids** — the same
caution `frame`, `propertySets` and `colors` already require, for the same reason: the bridge binary is
installed separately and a stale one still runs.

## Usable ids

An object's GlobalId is **usable** when it is non-empty **and** unique within its own file. Uniqueness is
judged per file **after the filter is applied** — the population being compared is the population the ids
have to be unique within.

Both halves come from an existing rule, and the reasoning is not ours: in a file where every id is `''`,
"the id still matches" is trivially true and waves through whatever happened to land in that slot; and
two objects sharing an id make it true after a reorder that swapped them. **An id that cannot distinguish
anything does not get to vote.**

**A duplicated id disqualifies every copy, not the second one onward.** Keeping the first would be
choosing arbitrarily between two objects the file gives us no way to tell apart, which is the one thing
this command exists not to do.

## The three outcomes

| | |
|---|---|
| Usable id in both files | **paired** → classified by the comparison set |
| Usable id in base only | **removed** |
| Usable id in revised only | **added** |
| Id absent or duplicated, either side | **uncomparable** — counted per side, never assigned a status |

Note what falls out of this: **every `removed` object has a usable unique id by construction.** That is
what lets removed geometry be a `read-model --ids` call.

## `uncomparable` is not a footnote

It is **never folded into `unchanged`**. Per side the output carries the count, the reason split (`blank`
vs `duplicated`) and a breakdown by IFC type and storey, so a consumer can say:

> *312 objects in the new file carry no usable IFC id (287 blank, 25 duplicated) and could not be
> compared — mostly IFCBUILDINGELEMENTPROXY on Level 2.*

That sentence is what stops a user reading a clean-looking list as a complete one. A count alone does not
achieve it, which is why the breakdown is in the response rather than left to the caller to compute from
data it does not have.

## The refusal

If **`paired === 0`** while both files contain objects, the comparison is **refused**:
`identity.refused = { reason, baseObjects, revisedObjects, skipped, uncomparable }` and **no `changes`
array at all.**

Two files can honestly share nothing. But a caller reaching this command has just put *this* file
forward as the next version of *that* one, so the likely truths are "wrong file" and "their exporter
regenerated every id" — and both deserve naming over a screenful of fabricated deletions. With no
geometry fallback, everything-added-everything-removed is the precise symptom of the second.

**The `changes` array is omitted rather than returned beside a flag.** A caller renders what it is given;
a list plus a warning is a list. There is deliberately **no "compare anyway"** either — with no fallback
there is nothing left to compare *with*, so the option could only produce the same fabricated list behind
a banner, and banners get ignored. The two real remedies are outside this command: pick a different file,
or keep both versions and look at them side by side.

**One pair is enough to proceed.** The ids either carry signal or they do not, and a single shared id says
they do. A project genuinely can replace all but one object between revisions.

**An empty side is not a refusal — unless it is empty for the wrong reason.** A first version against an
empty file pairs nothing, and the counts already say why. But `read-model` **drops products carrying no
drawable triangle** and reports them as `skipped`, so a side can arrive empty while the file is full of
objects. Reporting *that* as a wholesale deletion is the same lie by another road, and it gets its own
sentence — because a file that cannot be drawn is a repairable condition and an empty file is not.

## Classifying a paired object — the comparison set

Borrowed from Tekla's comparison set, which exists because a diff that reports every difference reports
mostly noise (an export timestamp, a 0.1 mm float wobble).

| Criterion | Compared as | Tolerance |
|---|---|---|
| `location` | centroid of the object's own geometry | `position-tolerance-mm` |
| `geometry` | triangle count + sorted axis-aligned extents | as above |
| `ifcType` | entity name | exact |
| `name` | verbatim string | exact |
| `profile` | verbatim string, case-sensitive | exact |
| `material` | verbatim string | exact |
| `properties` | every property set, property by property, **both ways** | exact |

A `changed` row always carries `changedBy: [...]` naming which fired. A diff that reports a change
without naming it sends the user hunting. For `location` it also carries `delta` and `distance`, so a row
can say *"moved 500 mm"*; for `properties` it carries `fields: [{ name, from, to }]`.

**The property comparison is two-way**, so a **deleted** property is a change. That is the easiest one to
miss and among the more consequential to miss.

**`unchanged` is counted, not listed**, unless `include-unchanged` is passed. On a 12,000-object model
the list is otherwise 99% rows saying nothing happened.

### The set is fixed in this version, and echoed

`criteria` comes back in every response. Tekla makes its set user-configurable and will eventually be
right to; until then the echo is what stops a change list **stored today** from meaning something
different once the set grows. A stored diff whose questions are unknown is not evidence of anything.

### Sorted extents are not rotation-proof, and the limit is stated

Extents are sorted before comparison, so a member turned 90° about a coordinate axis reads as *moved*
rather than *reshaped* — it is the same member. An **arbitrary** rotation does change an axis-aligned
box, so a beam rotated 30° fires `geometry` as well as `location` instead of reporting a pure move.
Conservative: it over-reports and never hides. The general case needs a rotation-invariant shape
descriptor, which is a research problem rather than a slice.

### The tolerance is an input because the default is not measured

1 mm is defensible, not validated. The reader returns canonical millimetres and a re-export of an
unmoved object should reproduce its coordinates, so 1 mm absorbs float noise while sitting far below any
nudge a person means. But if a real re-export drifts unmoved objects further than that, **every object
reads as changed and the list is noise** — so the number is exposed rather than baked.

One real data point, from a Tekla revision pair: changing a column's section from W14X43 to W14X53 moved
its centroid by **0.0588 mm**. Well under tolerance, so `location` correctly stays quiet while `geometry`
and `profile` fire. That is exactly the discrimination the tolerance is for.

It applies to **both** `location` and `geometry` deliberately. The same float noise that moves a centroid
moves an extent, and forgiving one but not the other would fail a shape check for a wobble the location
check had just waved through.

## Per-side filters exist so a not-like-for-like comparison can run at all

The bare `storeys` / `ifc-types` / `ids` apply to both sides, which is the ordinary case. The
`base-*` and `revised-*` variants override one side.

They are not a convenience. When a storey is renamed between revisions — "Level 1" becomes "L01" — a
single shared filter makes the comparison **unrepresentable**, not merely awkward: the filter that
selects the base selects nothing in the revision. The per-side inputs let it run.

The comparison is then explicitly not like-for-like, and the response says so: `selected` comes back
**per side**, so a consumer can banner the scope change — *"v2 read one storey, v3 reads the whole file;
added and removed reflect the change of scope, not only the model."* A diff across a scope change
presented as a plain diff is the same class of lie as the refusal prevents.

`selected.unmatched` carries filter values that matched nothing, exactly as `read-model`'s does, and it
is the receipt to check **on the revised side before anything is swapped**.

## Removed geometry is a `read-model` call, not an output of this one

Every `removed` object has a usable unique id by construction, so its shape is a `read-model` against the
retained previous file with `ids: [...]` — which is exactly what that input is documented for.

Two things follow, both good. This command stays a pure metadata command with a bounded payload; and
removed geometry loads **lazily**, so on a revision with 3,000 deletions only the ones actually looked at
are ever tessellated. An `include-removed-geometry` output would have forced all 3,000 up front, on a
command whose whole value is answering before you pay for geometry.

## Cost: roughly twice a `read-model`

Both files are tessellated. Measured 51.8s against `read-model`'s 27.1s on a 25,533-object, 35 MB model —
a ratio of 1.91.

Objects stream through the same `onObject` sink `read-model` uses and are reduced to a fixed handful of
numbers each (identity, attributes, centroid, sorted extents, triangle count, flattened properties) with
the triangles dropped immediately, so **two complete tessellations are never resident at once.** That is
deliberate: holding both is the memory bill that ruled out doing this comparison anywhere else. But the
parse itself is paid twice, and no filter avoids that.

## Failure modes

| | |
|---|---|
| Either path missing | Refused by name (`base-ifc-path` / `revised-ifc-path`) before anything is opened |
| Files share no usable id | `identity.refused`, no `changes` — see [The refusal](#the-refusal) |
| One side empty because everything was undrawable | `identity.refused` with its own reason, distinct from an honestly empty file |
| A filter value matches nothing | Not an error — it comes back in that side's `selected.unmatched` |
| Bridge predates 1.6.0 | `unknown command 'compare'`. Detect by what came back, never by a version number |
