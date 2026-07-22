---
name: blender-ifc-import-ifcopenshell
description: This skill should be used when importing IFC into Blender headlessly — reading `scene.import` or `scene.info` output, debugging a missing element or blank material, choosing between raw ifcopenshell and Bonsai, or installing ifcopenshell into Blender's bundled Python. Covers the geom-iterator pattern, the user-site install trap, unit scale, multi-hop material resolution through layer-set / profile-set / type wrappers, why openings and spaces are excluded not skipped, and the receipt's invariant.
---

# IFC import with raw ifcopenshell

The agent has to answer two questions from one file: *what does this element look like*
and *what is it*. Geometry alone is not enough — nobody is present to click materials, so
the look has to be deduced from IFC semantics (see the `look-presets` skill). Both answers
come from the same `ifcopenshell` file handle in a single pass.

## Three options, one choice

| Option | Verdict |
|---|---|
| **Raw `ifcopenshell` in Blender's bundled Python** | **Chosen.** pip-install into Blender's Python, tessellate with the `ifcopenshell.geom` iterator, build Blender meshes directly, read class / material / storey off the same file handle. Fully headless-proven, no add-on dependency. |
| **IfcConvert preprocessing** (IFC → glTF + a JSON semantics sidecar) | Fastest for very large models, but more moving parts and a second artifact to keep in sync. The documented v2 escape hatch, not v1. |
| **Bonsai add-on** | Rejected. GUI-oriented, documented instability under load, and no confirmed clean story under `blender -b`. It is built for humans in a window. |

The deciding factor is not speed. It is that the chosen option needs nothing installed
beyond one wheel, and that class / material / storey are readable from the same handle
that produced the mesh — so the semantics cannot drift out of step with the geometry.

## The install, and the trap that makes it look installed when it is not

```bash
blender -b --python-expr "import sys,subprocess; subprocess.check_call([sys.executable,'-m','pip','install','ifcopenshell'])"
```

That is the exact one-liner the agent's `ifcopenshell-missing` error carries in its `hint`
field. Run it against the Blender the agent will use.

**`pip install ifcopenshell` reporting success does NOT mean `import ifcopenshell` works.**
This is the trap, and it costs an hour if you trust pip's output:

- Blender's bundled Python runs with `site.ENABLE_USER_SITE = False` and ignores
  `PYTHONPATH`.
- Its own `site-packages`, under `Program Files`, is not writable without elevation.
- So pip falls back to a `--user` install. It succeeds, prints
  `Successfully installed ifcopenshell-0.8.5`, and puts the package somewhere nothing ever
  adds to `sys.path`.
- A bare `import ifcopenshell` then raises `ImportError` against a correctly installed
  package.

The fix is three lines, and it is not optional:

```python
import site

user_site = site.getusersitepackages()
if user_site and user_site not in sys.path:
    sys.path.insert(0, user_site)
```

`_ifc_import._import_ifcopenshell()` does this before every import attempt, then raises
the named `ifcopenshell-missing` error with the install hint if it still fails. Without it,
every command in the agent fails with a misleading "not installed".

## The geom-iterator pattern

```python
settings = _make_settings(ifcopenshell)
iterator = ifcopenshell.geom.iterator(settings, ifc_file)
if not iterator.initialize():
    raise _result.AwareBlenderError(
        _result.ERR_IFC_EMPTY,
        f"IFC {ifc_path} contains no tessellatable geometry",
    )

while True:
    try:
        shape = iterator.get()
        seen_guids.add(shape.guid)
        product = ifc_file.by_guid(shape.guid)
        ...
    except Exception as exc:  # skip-and-count is the contract
        ...
    if not iterator.next():
        break
```

`shape.geometry.verts` is a flat `[x, y, z, x, y, z, …]` list and `shape.geometry.faces` a
flat triangle-index list, so both are regrouped in threes before `mesh.from_pydata()`. The
iterator tessellates for you: parametric `IfcProfileDef` extrusions and an already-meshed
`IfcTriangulatedFaceSet` alike arrive as triangles, so the importer handles one shape.

`shape.guid` is the bridge back to semantics: `ifc_file.by_guid(shape.guid)` returns the
entity, and class / material / storey are read from it. Those land on the Blender object as
custom properties, so no later command has to reopen the IFC:

| Property | Source |
|---|---|
| `ifc_guid` | `shape.guid` |
| `ifc_class` | `product.is_a()` |
| `ifc_name` | `product.Name` |
| `ifc_material` | `_material_of(product)` — see below |
| `ifc_storey` | Containing `IfcBuildingStorey` name via `ContainedInStructure` |

**A single bad element is never fatal.** The `except` above records the GUID and the
reason and moves on; the render still ships. Only `imported == 0` raises (`ifc-empty`),
and that payload carries the whole `skipped` list.

## The 0.7 / 0.8+ settings split

`ifcopenshell` changed its geom-settings API between generations: 0.7 used enum attributes
(`settings.USE_WORLD_COORDS`), 0.8+ uses string keys (`"use-world-coords"`). The agent
handles both:

```python
settings = ifcopenshell.geom.settings()
try:
    settings.set("use-world-coords", True)
except (TypeError, AttributeError, RuntimeError):
    settings.set(settings.USE_WORLD_COORDS, True)
```

**Measured here: ifcopenshell 0.8.5, string-keys** — the first branch is the live path.
`use-world-coords` is what makes the vertices arrive already placed, so the importer never
has to compose `ObjectPlacement` chains itself.

## Units

**Measured: ifcopenshell converts to metres.** The probe's 6000 mm beam arrived as exactly
`0.0 → 6.0`. Blender's scene unit is also metres, so `unit-scale` stays **1.0** for normal
input and the input exists only for producers that hand back raw file units.

To tell whether a producer needs it, measure a known span after import:

```bash
blender -b model.blend --python-expr "
import bpy
xs = [(o.matrix_world @ v.co).x for o in bpy.data.objects if o.type=='MESH' for v in o.data.vertices]
print('X_SPAN', round(max(xs)-min(xs), 3))
"
```

A 6 m frame reading `6.0` is right. Reading `6000.0` means that producer's geometry is
arriving in millimetres and the call needs `unit-scale: 0.001`.

Getting this wrong is not a visible error — the model imports, the receipt looks perfect,
and every camera fit and every render is silently useless because the bounding sphere is
1000× too big.

## Material resolution is not one hop

**A naive one-level probe silently returns empty for Revit and Tekla exports.** This is
the second expensive trap on this page, and the failure is invisible: geometry still
imports, only `ifc_material` goes blank, and the look mapper quietly falls back to the
class family. The render looks plausible and is wrong.

IFC hangs a material off a product through a family of wrappers whose shape depends on the
producing application:

| Wrapper | Emitted by | Path to the leaf |
|---|---|---|
| `IfcMaterial` (direct) | Simple writers, including the `ifc` agent's `ifc.write` | Already the leaf |
| `IfcMaterialLayerSetUsage` | Revit / ArchiCAD walls and slabs | `ForLayerSet` → `MaterialLayers[]` → `Material` |
| `IfcMaterialProfileSetUsage` | Revit and Tekla structural framing — **the shape this agent meets most often** | `ForProfileSet` → `MaterialProfiles[]` → `Material` |
| `IfcMaterialLayerSet` / `IfcMaterialProfileSet` (direct, no usage) | Several exporters | One hop shorter than the above |
| `IfcMaterialConstituentSet` | IFC4 curtain walls and composite plates | `MaterialConstituents[]` → `Material` |
| `IfcMaterialList` | Legacy IFC2X3-era producers | `Materials[]` |

`_leaf_material_name()` walks those links recursively (depth-bounded at 8, with a visited
set because IFC references form a graph, not a tree) and **only an actual `IfcMaterial`
contributes a name.**

That last restriction is the point:

- A layer set's wrapper carries `LayerSetName`, not `Name`.
- `IfcMaterialLayer.Name` is the **layer's** name — "Core", "Insulation" — not the
  material's.
- `IfcMaterialProfile.Name` is the **section's** name — "W16X26".

Lifting `.Name` off a wrapper therefore returns a plausible-looking wrong answer.
Returning `"Core"` as a material is worse than returning nothing: nothing falls back to the
IFC class and renders sensibly, while `"Core"` matches no grade prefix, produces a
confident `grade:` reason in the receipt, and hides the problem.

**Materials can hang off the type, not the occurrence.** The canonical IFC4
structural-framing split puts the `IfcMaterialProfileSet` on the *type* and only the usage
on the occurrence, so a producer that populates just the type side leaves
`HasAssociations` empty on the beam itself. `_material_of()` falls back one hop through
`IsTypedBy` → `RelatingType`. When both levels carry a material the **occurrence wins**,
being the more specific statement.

`IsTypedBy` is the IFC4 inverse. IFC2X3 spells it `IsDefinedBy` and is **not covered** —
an IFC2X3 file with type-only materials will report blank materials.

**Only the FIRST leaf is returned.** A composite wall reports its first layer's material
rather than blending. That is the contract, not a rounding error: downstream wants exactly
one material per object.

`tests/test_material_resolution.py` builds one synthetic IFC4 model carrying all twelve
association shapes, round-trips it through the SPF serializer so the inverse attributes
resolve the way they do in a real file, and asserts each resolves to its leaf. Run it
headless before changing anything in this area:

```bash
blender -b -P 20-agents/aeco/visualization/blender/tests/test_material_resolution.py
```

## The iterator silently drops degenerate geometry

`ifcopenshell.geom.iterator` excludes a product whose geometry is unfeasible — a zero-depth
extrusion, a zero-area profile — **at the C++ level**. It is never yielded and nothing is
raised, so the `except` in the import loop cannot see it.

The consequence is the one outcome a skip-and-count contract must never produce: the
element vanishes. `imported` drops by one while `skipped` stays empty, and the payload is
indistinguishable from a model that legitimately has one fewer element.

The fix is a reconciliation pass after the loop, against the products that *claim* a shape:

```python
for product in _geometry_bearing_products(ifc_file):
    if product.GlobalId in seen_guids:
        continue
    skipped.append({
        "guid": product.GlobalId,
        "reason": "not yielded by the geometry iterator (degenerate or unfeasible geometry)",
    })
```

**The reconciliation set must NOT be all `IfcProduct`.** `IfcSite`, `IfcBuilding`,
`IfcBuildingStorey`, grids and annotations legitimately carry no representation, so
reconciling against the full product list would report them as skips on **every single
import** and bury the real failures in routine noise. `_geometry_bearing_products()`
therefore keeps only products with a non-empty `Representation.Representations`. A product
that claimed a shape and produced none is a genuine skip; one that never claimed a shape
is not.

`seen_guids` is recorded *before* anything else in the loop body can fail, so a product
that yielded a shape but died during mesh construction is accounted for once, not twice.

## Openings and spaces are never imported

```python
_NON_VISUAL_CLASSES = (
    "IfcOpeningElement",
    "IfcSpace",
)
```

- **`IfcOpeningElement` is a subtractive void.** The host wall already has the boolean
  applied by the time the iterator tessellates it, so importing the opening puts a solid
  box inside every door and window — the hole *and* a block filling it, in the same render.
- **`IfcSpace` is a volumetric zone** (room, circulation area). Analytic, not built.
  Importing it wraps each storey in slabs of solid air that hide everything inside.

Both tessellate perfectly well. That is exactly the problem — they must be excluded **by
class, not by any geometric test**. The check uses `is_a()` rather than class-name equality
so IFC4 subtypes are covered: `IfcOpeningStandardCase` is an `IfcOpeningElement` and must
not slip through on a producer that emits the more specific class.

**They are reported in a separate `excluded` channel, deliberately not in `skipped`.**
"Never meant to render" is a different fact from "failed to render", and on a real building
folding them together would mean dozens of routine exclusions per storey burying the one
genuine failure the `skipped` field exists to surface. Exclusions sit outside the
imported/skipped equation entirely.

The list is kept short and justified on purpose: every entry is invisible in the receipt's
`by-class`, so an unjustified addition silently removes fabric from the picture.

## The receipt

`scene.import` returns, and `scene.info`'s `ifc-path` branch republishes:

| Field | Meaning |
|---|---|
| `imported` | Produced geometry and landed in the scene |
| `skipped` | `[{guid, reason}]` — claimed a shape, produced none |
| `skipped-count` | `len(skipped)` |
| `excluded` | `{class: count}` — deliberately never imported |
| `excluded-count` | Sum of the above |
| `by-class` | `{IfcBeam: 4, IfcColumn: 4, …}` — imported objects only |
| `by-material` | `{A992: 4, A500-GR.B: 4, …}` — blank materials are not counted |
| `by-storey` | `{Level 1: 12, …}` — blank storeys are not counted |

**The invariant:**

```text
imported + skipped-count == number of geometry-bearing products
```

Every product that claimed a shape representation lands in exactly one of the two buckets,
so an element can never disappear from the accounting. `tests/test_import_reconciliation.py`
pins it on a clean model, on a model with a zero-depth extrusion, and on a model containing
an opening and a space — including the assertion that the clean case produces **no false
skips**.

`scene.info`'s `blend-path` branch **omits all four skip/exclusion keys** rather than
reporting zeroes. A staged `.blend` carries no import record; reporting `skipped-count: 0`
there would assert something the command cannot know.

`blend-path` and `ifc-path` are mutually exclusive on `scene.info`, and passing both is an
error rather than a silent preference — otherwise which source was actually read is
undetectable from the payload after the fact. `source` always echoes the answer.

## What survives the import

| Carried | How |
|---|---|
| Tessellated geometry, world-placed | `use-world-coords` + the geom iterator |
| IFC class | `ifc_class` custom property |
| Material grade name | `ifc_material`, resolved to the leaf `IfcMaterial` |
| Storey | `ifc_storey` |
| GUID and name | `ifc_guid`, `ifc_name` — the handle back to the source model |

## What's lossy

- **Parametric definition.** Profiles arrive as triangles. Nothing downstream can ask
  "what section is this" beyond the name string.
- **All but the first material of a composite.** By design.
- **Property sets.** Only class / material / storey / GUID / name are carried. Anything
  else needs a second pass over the IFC.
- **`IfcStyledItem` colours.** Not read at import; the look mapper works from class and
  grade instead.
- **Openings and spaces.** Excluded, and correctly so — but a workflow that needs room
  volumes cannot get them from this agent.
- **Type-level materials in IFC2X3 files.** `IsDefinedBy` is not walked.
- **No Blender materials are created.** `scene.import` writes semantics, not shading. A
  freshly imported `.blend` rendered without a `scene.apply-look` pass shows Blender's
  default grey.

## Worked example

Import, then read the receipt as a sanity gate before spending render time:

```yaml
nodes:
  - id: stage
    agent: blender
    command: scene.import
    inputs:
      ifc-path: '{{ inputs.model-ifc }}'
      blend-path: '{{ run.tmp-dir }}/model.blend'

  - id: inventory
    agent: blender
    command: scene.info
    inputs:
      blend-path: '{{ nodes.stage.blend-path }}'
```

A healthy `scene.import` receipt for the reference portal-frame fixture:

```json
{
  "ok": true,
  "imported": 9,
  "skipped": [],
  "skipped-count": 0,
  "excluded": {},
  "excluded-count": 0,
  "by-class":    { "IfcBeam": 4, "IfcColumn": 4, "IfcMember": 1 },
  "by-material": { "A36": 1, "A500-GR.B": 4, "A992": 4 },
  "by-storey":   {},
  "blend-path": "…/model.blend"
}
```

How to read a receipt that is not healthy:

| Symptom | Diagnosis |
|---|---|
| `by-material` empty, `by-class` populated | Material association is not being walked for this producer — check which wrapper it emits, and whether the association hangs off the type |
| `skipped-count > 0` with `not yielded by the geometry iterator` | Degenerate geometry in the source model; the GUIDs name the offenders |
| `excluded` large, `imported` small | Normal for an architectural model full of openings — not a failure |
| `imported` correct but every camera fit looks wrong | Unit scale; measure a known span |
| `ifcopenshell-missing` after a successful pip install | The user-site trap — check that `site.getusersitepackages()` is on `sys.path` |
