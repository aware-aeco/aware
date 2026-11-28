# NVIDIA SMLib ("USD BRep") kernel trial: would it help FloLess/AWARE with exact steel geometry?

Date: 2026-11-23. Run on PAWELDELL by a research agent for the floless.app master session. Owner asked for this.
Method: measured on our own problem cases, against what we do today. Nothing here is judged from docs or screenshots.
Scripts, raw results and how to rerun: `docs/superpowers/specs/smlib-trial/` (the downloaded kernel is not committed).

## Verdict

**Adopt in AWARE, narrowly, as an opt-in exact-geometry component, with the mesh path kept as the fallback.**
Not as a replacement of the viewer's cope path, and not yet as a dependency of anything customers' runs cannot do without.

Why, in four numbers:

- It is exact where we are approximate. The exact volume of a coped, end-cut wide-flange beam matched an independent
  numerical reference to 6.4e-8 (the reference's own limit) on 126 of 126 random placements. Our mesh volumes are off by
  3.3e-4 to 4.0e-4 on the same parts, because arcs are polygons.
- It does not leave the #4005 sheets. On the issue's shape (operand sharing faces with the plate) manifold3d produced
  coincident opposite faces on 81 to 92 of 100 random placements at 1 m coordinates (control with an overshooting
  operand: 0 of 100). The exact boolean gave a clean 7-face solid on 1,200 of 1,200 runs, volume exact to 1e-12.
- For curved members it sees overlaps a mesh underestimates: a 0.05 mm pipe-on-pipe overlap is 0.152 mm3 exactly; a
  mesh at 0.1 mm chord says 0.021 (86% low), at 0.01 mm says 0.124 (19% low).
- The cost is real and the risk is early-project: it is six weeks old, one vendor team behind a release bot, accepts no
  outside code, and its own tracker lists wrong-result booleans and crashes in families I did not test (section 3).

Our real cope path (three-bvh-csg, member-local frame) did **not** fail in 225 headless runs; it never threw, and its
meshes are visually adequate. The gain is exactness (volumes, distances, clash volumes, curved faces), not rescuing a
failing cope. That is why the verdict is "narrow", and why the first slice is aimed at #4005, which is a real
defect today, and at exact clash numbers.

Not verified, so do not read the verdict as covering it: a build from source; Linux wheels (only the Windows wheel was
run); real parser data for #4005 (not available to me; the shape is synthesised from the issue text); IFC/STEP (the wheel
has none); multi-threaded behaviour, memory growth and large assemblies; operations outside the families listed below.

## 1. Getting it running

| Item | Result |
|---|---|
| Official repo | `github.com/NVIDIA-Omniverse/usd-brep` ("USD BRep": C++ and Python library for NURBS B-rep solid modeling; the kernel is SMLib). Created 2026-10-08; 2,128 stars, 326 forks. |
| Pinned release | **v1.0.2**, published 2026-11-19. Tag object `bb60af7b9c2c3dd240b8e3c853c9f2ff491a710c`, commit `9701dd0cf4b8f4fe379290b3032e9407deb220df`. |
| What I ran | The PyPI wheel `usd-brep==1.0.2`, `cp312-cp312-win_amd64`, 16.8 MB, sha256 `22e0485f13200d504be3f8f4c4f877bb5e07e60ed8e9f8f51995fdd7f02a7475`. Installed into a throwaway venv in the scratchpad (107.7 MB on disk, 2,413 files), run with `python -I`. Only numpy was pulled in (plus manifold3d and trimesh for the baseline). The repo itself was read through raw files at the tag, not cloned. |
| Not done | **Built from source: no.** CI on this PC was running the v0.174.0 release job, and the wheel made a source build unnecessary for the measurements. `BUILDING.md` says a source build needs Visual Studio with the C++ toolset (host compiler), network for Packman to fetch OpenUSD, Python 3.12, premake and `repo_*` tools, then `prebuild`, `repo build`. Treat the build as untested pain. |
| Platforms | Wheels: Windows x86_64 (16.8 MB), manylinux_2_34 x86_64 (87.4 MB), manylinux_2_34 aarch64 (85 MB). **CPython 3.12 only** (`>=3.12,<3.13`). No macOS wheel. Binary 7z packages also exist for the same three platforms (30 to 45 MB). Android arm64 builds from source. |
| Size | Windows install 107.7 MB: SMLib.dll 12.2 MB, bundled OpenUSD 25.11 DLLs, tbb, the pybind module (2.9 MB). |
| Python API | Flat `usd_brep` namespace: `create_box/cylinder/...`, `linear_sweep`, `boolean_*`, `fillet_*`, `Brep.volume/area/distance_to/relationship_to`, `tessellate`, `usd.export_brep/import_breps/heal_file`, `poly_boolean_*`. |

### Licence and notices

- `LICENSE`: Apache-2.0, "Copyright (c) 2020-2026 NVIDIA CORPORATION & AFFILIATES". Kernel source headers (checked
  `SmBrep.h`) carry `SPDX-License-Identifier: Apache-2.0`. There is no separate `NOTICE` file in the repo root;
  `THIRD_PARTY_NOTICES.md` (33 KB) plays that role.
- Wheel metadata licence expression: `Apache-2.0 AND LicenseRef-TOST-1.0 AND BSD-2-Clause AND BSD-3-Clause AND MIT AND BSL-1.0`.
- What the wheel actually ships and licenses (its `PACKAGE-LICENSES`): OpenUSD (Tomorrow Open Source Technology License
  1.0, a modified Apache 2.0), oneTBB (Apache 2.0), pybind11 (BSD-3), hwloc (BSD-3, Windows). No GPL or LGPL in the wheel.
- Notices that apply to the binary release packages only, not the wheel: MaterialX, OpenSubdiv, CPython, HACL*, and
  libaom/libavif with the **AOMedia Patent License 1.0**. An optional developer GUI group in `uv.lock` pulls LGPLv3 and
  others; it is not shipped.
- Also listed: AOUSD proposal texts under "OpenUSD Technical Proposal Supplemental Terms" (documentation, not code).
- Read: fine for AWARE to redistribute the wheel if its `PACKAGE-LICENSES` travel with it. A lawyer should confirm the
  TOST and AOMedia terms for any bundle that includes the binary package instead of the wheel.

## 2. Measurements

Setup common to all: millimetres, a W360x57-like wide flange (d 358, bf 172, tf 13, tw 8, root fillets r 12) of 3 m,
member along +Z. The exact outline is lines plus true circular root-fillet arcs. The reference volumes come from my own
Gauss-Legendre quadrature over the section (`common.py`), not from the kernel and not from a mesh; its agreement with
the kernel (5e-8) is the reference's limit, so "exact" below means "equal to the reference within 1e-7".
CI was running on this PC during the timings (one core each), so times are indicative, not benchmarks.

Two things to know before reading any number:

- `Brep.volume()` defaults to `relative_accuracy=1e-3`, a coarse numerical integral: on the plain filleted beam it was
  5.4e-6 off. At 1e-9 the error was 1.7e-11 and at 1e-12 it was 1.3e-14, for 9 to 124 ms per call (the first call carries warm-up). All volumes below use 1e-12.
- Booleans **mutate their first operand in place** (`r is b` is true) and `cut()` returns a zero-volume sheet, not a half
  solid. Copy first; use a slab and `boolean_difference` for a plane cut.

### (a) Coped wide-flange beam, with an end plane-cut

Cases: cope only; cope + 25 degree end plane-cut (overlapping the cope); both with the cutter exactly coplanar with the
member's top, sides and end face; cutter at the flange inner face; cutter tangent to the root fillet; and 120 random
variants (cope depth 8 to 60, length 45 to 200, re-entrant radius 0 to 15, skew up to +-40 degrees, random rotation and a
translation up to 50 m, a quarter of them exactly coplanar). Failure means: exception, not a manifold solid, volume
more than 1e-6 from the reference, or a defect in the tessellation (open or non-manifold edge, zero-area triangle,
coincident opposite faces).

| | SMLib | Today's path (three-bvh-csg, same cutter construction as `applyCopes`) |
|---|---|---|
| Runs | 126 | 225 (6 named x 3 cutter variants, plus 120 random x 1 or 2) |
| Threw / failed outright | 0 | 0 |
| Volume vs reference | max 6.4e-8 (exact) | 3.3e-4 to 4.0e-4 off every time: the arcs are 8-segment polygons, so this is tessellation, not a CSG error. CSG itself is consistent to about 1e-5 |
| Clean (no defect as defined) | **126 of 126** | exact-coplanar cutter 11 of 39; 0.6 mm padded as the app does 55 of 93; 10 mm padded 80 of 93 |
| Defect kinds | none | zero-area triangles in 8 / 20 / 5 runs (coplanar / app-padded / 10 mm); non-manifold edges after a 1e-3 mm weld in 27 / 38 / 11 runs (partly T-junction artefacts, see below) |
| Gaps between faces | 0 open edges in every tessellation, by topology | unmatched (T-junction) edges in every run: median 286 to 307 per mesh. They are not visible leaks, but nothing can be welded or measured from such a mesh |
| Time, boolean only | median 12.9 ms, p95 24.6 ms (Python binding, in process) | median 4.4 to 4.6 ms, p95 about 10 ms; coplanar cutter 32.6 ms median, 71 ms p95 (Node) |
| Plus, to get a drawable mesh | tessellate at 0.01 mm chord: median 31.5 ms, 662 triangles, mesh volume 2.7e-5 off | none (the boolean output is the mesh), 332 to 376 triangles |
| Random placement up to 1,000 m | B-rep exact on 120 of 120; **2 of 120 flagged** only because the tessellation comes back as float32 (about 0.06 mm resolution at 1 km), which fabricated a coincident-face pair | not applicable (the app cuts in the member-local frame) |

Reading: SMLib is more robust and exact, but today's mesh path was not failing on this part. The mesh checks use
my welded-vertex metrics; three-bvh-csg's output is unwelded float32, so its "non-manifold" counts overstate. The
zero-area-triangle counts are real. Tessellate in the member-local frame, as the app already does.

### (b) The #4005 shape: operand sharing faces with the part

Issue read: a part cut evaluated as `solid - cutter` in manifold3d keeps zero-thickness sheets (coincident opposite faces,
zero-area triangles) when the operand shares a face with the part. The Tekla model is not available to me, so I
synthesised the described shape: a 400x250x20 plate with a 60x45 corner-clip prism cut at the corner, in four forms (legs
on the plate edges and equal thickness = shares four planes; shared top and bottom only; legs on the edges, thicker;
overshooting control). Each solid is meshed, placed independently with a random rigid transform (rounding differs
between the two, as in real data), and cut with manifold3d fed float32 meshes (its Python binding takes float32) at
three coordinate scales, 100 placements each. Same placements went to SMLib. Metric: coincident opposite faces with
overlap above 0.001 mm2, and zero-area triangles above 1e-6 mm2, on each result's mesh.

| Variant (coordinates within +-1 m / 10 m / 100 m) | manifold3d: runs with coincident opposite faces | manifold3d: runs with zero-area triangles | SMLib: defects | SMLib faces |
|---|---|---|---|---|
| legs on edges + same thickness (the issue) | 81 / 8 / 1 of 100 | 0 / 1 / 9 | 0 of 100 at each scale | 7 (box 6 + chamfer 1) |
| shared top and bottom only | 33 / 6 / 2 | 3 / 1 / 3 | 0 | 7 |
| legs on edges, thicker | 92 / 17 / 4 | 12 / 8 / 3 | 0 | 7 |
| overshooting operand (control) | 0 / 0 / 0 | 0 / 0 / 0 | 0 | 7 |

- The signature reproduces and is selective: the control never produces it. The detector loses sensitivity at larger
  scales (float32 noise of 8 um at 100 m exceeds my 1 um plane bucket), so the falling counts at 100 m are partly
  detector blindness, and the volume error there shows the damage differently: manifold3d's volume was off by up to
  2.2e-4 at 100 m, SMLib's by 6.7e-13.
- SMLib: 1,200 of 1,200 runs a manifold solid with exactly 7 faces, volume within 1e-12 of the closed form, no
  coincident faces and no zero-area triangles in its tessellation. Median 0.8 to 1.9 ms per boolean.
- Caveat that matters: the manifold3d side is my reconstruction (float32 mesh bridge, independent transforms), not the
  parser's code. It shows the failure class and that an exact boolean does not have it; it does not prove the parser's
  exact trigger. The fix for the parser still needs the DB 9.50 model replayed through the exact path.

### (c) Clash between two trimmed member solids

A girder along Y and a W358 beam framing into its web, coped top and bottom, with an end plane-cut. Exact answers:
`distance_to` and `relationship_to` (separate / touching / interpenetrating) and the volume of `boolean_intersection`.
Mesh answer: the same two solids tessellated (0.1 and 0.01 mm chord) and intersected in manifold3d.

| Case | Exact distance / relationship | Exact overlap volume | Mesh overlap (0.01 mm chord) |
|---|---|---|---|
| clear 12.7 mm | 12.7 (12.699999999999722), separate | 0 | 0 |
| clear 0.05 mm | 0.05, separate | 0 | 0 |
| touching, zero gap (face on face) | 0, touching | 0 | 0 (cannot tell touching from 3 mm apart without a distance) |
| overlap 2 mm, end cut 10 deg | interpenetrating | 4201.742636 mm3 (closed form 4201.74) | 4201.742615 |
| overlap 0.01 mm | interpenetrating | 23.0400000005 (closed form 23.04) | 23.0399780 |
| overlap 0.001 mm | interpenetrating | 2.3040000005 | 2.3038330 |
| two round pipes, overlap 0.5 mm | interpenetrating | 15.1838 | 14.819 (chord 0.01), 11.985 (0.1) |
| pipes, overlap 0.05 mm | interpenetrating | 0.15207 | 0.1239 (0.01), 0.0208 (0.1) |
| pipes, overlap 0.02 mm | interpenetrating | 0.02433 | 0.01399 (0.01), 0.00133 (0.1) |
| pipes, overlap 0.005 mm | interpenetrating | 0.001521 | 0.000225 (0.01), 0.0000207 (0.1) |
| pipes, gap 0.05 / 0.5 mm | 0.05 / 0.5 exact | 0 | gap found only by a separate distance routine |

- For planar contact a mesh check is as good as the exact one here: both are exact on planes. Where the exact answer
  pays is **curved surfaces** (round tubes, root fillets) and **touching versus gap**, which a mesh intersection cannot
  distinguish. Detection (volume above zero) still worked on the meshes for every overlap tried; the volume magnitude
  was 5 to 1000 times low at the coarse tolerance.
- Time: exact distance 4 to 40 ms, exact intersection 8 to 15 ms; mesh route 2 to 5 ms plus tessellation.
- Known issue found in the tracker, not reproduced: `brep_relationship` can report a shallow overlap as touching when
  no sample point falls in it (KNOWN_ISSUES, "Measurement"). Use the intersection volume, not the label, for a verdict.
- My analytic expectation for the 2 mm skewed case was first wrong (I assumed the whole web end is trimmed; the slab
  trims only the x<0 half). The kernel was right and agreed with the corrected closed form to 6 digits.

### (d) Parametric I and L section sweeps with root fillets (#3382 shapes)

300 random I-sections (d 100 to 1000 mm, flange 0.25 to 0.6 d, 15% sharp roots, root radius up to 95% of what fits) and
300 random L-angles (legs 40 to 250, thickness 3 to 0.2 x leg, root radius 0 to 1.5 t, toe radius up to 0.9 min(r1, t)).
Each swept 6 m along Z with true arcs. Failure: exception, not manifold, volume more than 1e-9 off the analytic area x length, or a mesh defect at a 0.02 mm chord.

| | I-sections | L-angles |
|---|---|---|
| Built clean | 300 of 300 | 300 of 300 |
| Max volume error vs analytic | 7.4e-14 | 2.5e-13 |
| Build time, median / max | 8.6 / 37.9 ms | 6.9 / 18.7 ms |
| Tessellate at 0.02 mm chord, median / max | 17.6 / 74.6 ms | 6.6 / 31.5 ms |
| Triangles, median | 492 | 148 |
| Mesh defects | 0 | 0 |
| Mesh volume error, median / max (0.02 mm chord) | 3.0e-5 / 4.8e-4 | 1.0e-5 / 1.0e-4 |
| Today's drawn outline (arcs as 8 segments per quarter), area error median / max | 3.6e-4 / 4.8e-3 | 5.2e-5 / 1.1e-3 |

The exact section is exact and cheap; the polygonal outline we draw today is up to 0.5% off in area for a big-root
I-section. Whether that matters depends on whether the catalogue ever needs the exact solid (weight, IFC, export).

### (e) Output routes

Coped, skewed beam (22 faces), exact volume 21,484,028.708 mm3. Tessellation, with deviation **measured** by sampling 400
random points on triangles and asking the exact B-rep for the closest-point distance (a sampled lower bound of the
true maximum):

| Chord height | Time | Triangles | Max sampled deviation | Mesh volume error | Open / non-manifold / zero-area |
|---|---|---|---|---|---|
| 0.5 mm | 7.0 ms | 138 | 0.249 mm | 1.5e-3 | 0 / 0 / 0 |
| 0.1 mm | 9.9 ms | 218 | 0.064 mm | 3.9e-4 | 0 / 0 / 0 |
| 0.01 mm | 31 ms | 688 | 0.0040 mm | 2.4e-5 | 0 / 0 / 0 |
| 0.001 mm | 129 ms | 2,550 | 0.00073 mm | 1.6e-6 | 0 / 0 / 0 |

The sampled deviation stayed inside the requested tolerance at every
level. The mesh is indexed and welded; three.js ingested the arrays directly (checked headless with FloLess's own vendored
three: 218 / 688 triangles, bounding box exact, signed volume matches). Hard edges need per-corner normals, which the API
offers (`to_numpy_arrays(face_varying_normals=True)`); `computeVertexNormals` on the welded mesh would smooth them. **The
returned arrays are float32**, so tessellate in a local frame.

Interchange:

- **USD BRep round trip works**: 22 faces back, still a manifold solid, volume diff 6.6e-15, file 40 KB, export 93 ms,
  import 20 ms. Native ASCII file round trip also exact (175 KB).
- **No STEP, IFC, IGES, Parasolid or ACIS** symbol exists in the wheel (searched `dir()` of both modules). Reports that
  there is no STEP translator are correct for this package.
- The repo ships OpenCASCADE `.brep` converters (`occt_to_usd`, `usd_to_occt`) in the C++ binary package, not the wheel.
  Its own known-issues list says `occt_to_usd` rejects extrusions and revolutions of circles and ellipses, and that
  `usd_to_occt` output can give OCCT wrong areas and volumes on some faces. So a STEP route today would be STEP to OCCT
  to `.brep` to USD BRep, through an OCCT tool of our own. Not tested; I did not run the binary converters.

### (f) Extra: ordinary detailing operations with coincident faces

200 runs, random rigid placement up to 50 m, closed-form volumes: 100 bolt-hole plates (1 to 16 holes, cylinder ends
overshooting or exactly coplanar with the plate) and 100 plates unioned onto a beam's top flange (underside coplanar, half
with an edge also coplanar). **200 of 200** correct: solid, face count as expected on the hole plates, volume within
1.5e-12 relative. Hole plates median 38 ms (max 163); flange union median 2.5 ms. (My first run of the hole plates showed
33% "failures": that was my generator putting holes off the plate, caught by the face-count assertion. The kernel was
right to cut what overlapped.)

## 3. Fit: how it would plug into AWARE, cost, risk

### Where it goes

By the guardrails the geometry engine belongs in AWARE, never in floless.app's web layer, and floless.app calls it only
through `server/aware-adapter.ts`. Fits an exec or runtime component next to AWARE's managed Python runtime:

- The managed Python runtime plan (`2026-11-20-managed-python-runtime-PLAN.md`) already pins CPython 3.12.x with
  hash-locked archives; usd-brep's wheel is cp312 only, so it matches that pin. **It does not match** the frozen
  parser that runs Python 3.13.2 (where the #4005 manifold3d call lives): the wheel cannot be imported there. Either
  the kernel runs out of process in the 3.12 runtime (the natural shape) or the parser moves to 3.12.
- Shape: a small JSON-in, JSON-out program (`aware exec` of a Python script, or a registered geometry agent) that takes a
  boolean/measure request in the member-local frame and returns exact numbers plus a tessellation. Inputs and outputs
  are plain data; AWARE's compile/run determinism rules are untouched (a pinned wheel and fixed tolerances are
  deterministic; I did not test cross-machine bit-equality, and the release notes themselves mention a fused multiply-add
  branch difference fixed only on macOS/aarch64).
- Footprint: +16.8 MB (Windows) / +87 MB (Linux) per runtime, plus the licence texts.

### Rough cost (estimates, not measured)

- First slice (below): about 1 to 2 weeks for one engineer, most of it the pin/verify/lifecycle work and the corpus,
  not the geometry.
- A broader move (parametric section solids, clash numbers in the product, STEP/IFC via OCCT): a quarter, and it is
  where the real risk sits.

### Risks, from the repo's own record (read at v1.0.2, 2026-11-23)

- **Age and bus factor.** Repo created 2026-10-08; four releases in six weeks (1.0.0 on 10-09, 1.0.1 on 10-22, 1.0.2 on
  11-19). Public history is squashed: every commit is by `nvidia-omniverse-bot`. The copyright line dates from 2020, so the
  code predates the public repo, but its public life is six weeks.
- **No outside contributions.** `CONTRIBUTING.md`: "currently not accepting external code contributions"; issues
  welcome. A community patch (PR #17, #18) was closed with "We had already fixed this one locally." So fixes arrive in
  batched releases from one team (the 1.0.2 changelog cites about nine external reports), response is fast on some
  (closed in 2 to 7 days in October) and a month on others (batch closed 2026-11-20).
- **Open wrong-result and crash bugs.** 46 open issues of 73 filed. Ones that bear on steel work: boolean_union of two
  capsules returns less than one capsule (#66); booleans of analytic vs NURBS operands differ (#72); a boolean that
  returned success with an empty B-rep for coincident skins (#35, closed in 1.0.2); a union returning success with an
  invalid B-rep (#33, closed); `fillet_edges` on collinear edges crashes with SIGSEGV (#70); `stitch_brep_with_face`
  crashes on every input (#65, also in KNOWN_ISSUES); shell/offset fail on some solids; healing mis-repairs closed
  periodic surfaces. Two of these (silent success with a wrong answer) are exactly the class that hurts a "checked"
  deliverable, so **every kernel result must be validated** (volume against a closed form or the operand volumes,
  `is_manifold_solid`, `brep_relationship` as a cross-check) rather than trusted.
- **API traps seen in this trial:** in-place mutation of the first operand; coarse default volume accuracy; float32
  tessellation; `cut()` leaves a sheet; a 2e-5 mm bounding-box pad.
- **Platform gaps:** cp312 only; no macOS wheel; on Windows, `usd_brep` and PyPI `usd-core` cannot share a process
  (issue #15; irrelevant unless we also use `pxr`).
- **What it will not do for us:** STEP/IFC translation; anything about drawing production.
- **Mitigations:** pin the wheel by sha256; keep the mesh path as the fallback ("nothing refuses, it asks": a kernel
  failure must warn and fall back, never refuse); gate adoption on a regression corpus built from this trial's cases;
  re-run it on every version bump; file defects at the vendor tracker (issues are accepted).

## 4. Recommendation and first slice

Adopt narrowly. Build one opt-in AWARE component that does **exact part-cut evaluation and exact clash numbers**, used first
where the mesh route is demonstrably wrong (the #4005 sheets; curved or touching clashes), validated on every call, with
the existing mesh path as the labelled fallback. Do not route the viewer's cope rendering through it yet: today's path was
not failing, and adding a 12 to 130 ms round trip per member to a render loop buys nothing. Revisit parametric section
solids and a STEP route after the first slice shows the kernel holds up on real models for a release or two.

Decision needed from the owner: whether the #4005 fix lives in the frozen parser (needs its Python on 3.12 or an
out-of-process call) or is reached by calling the AWARE component from the parser's step. This trial cannot decide that.

### Drafted first-slice issue for `aware-aeco/aware` (not filed; neutral wording, names no host, parser or client)

> **Title:** Exact B-rep boolean and clash component (pinned SMLib wheel) as an opt-in geometry exec, first slice
>
> **Problem.** Geometry that is cut or checked on triangle meshes loses exactness in two ways we have measured. (1) A cut
> whose operand shares faces with the part (a corner clip flush with a plate's edges and faces) evaluated as a mesh
> boolean in float32 leaves zero-thickness sheets: coincident opposite faces in 81 to 92 of 100 random placements at
> 1 m coordinates, while the exact boolean gave a clean 7-face solid in 1,200 of 1,200. (2) Overlap volumes between
> curved members (round tubes) are 5 to 1000 times low on meshes at viewer tolerances, and a mesh cannot tell touching
> from a small gap.
>
> **Proposal.** A managed, pinned `usd-brep` 1.0.2 wheel (cp312, sha256-locked; Apache-2.0 plus the licences in its
> `PACKAGE-LICENSES`) in AWARE's Python runtime, behind a JSON-in, JSON-out exec with these operations, in the member-local frame:
> `boolean(difference|union|intersection)`, `distance`, `relationship`, `interference_volume`, and `tessellate(chord_height)`.
> Outputs: result volume at relative accuracy 1e-12, face count, `is_manifold_solid`, and an indexed mesh (per-corner normals).
>
> **Done means.** (a) The corpus in the trial scripts (coped and end-cut wide-flange members, flush-operand plate clips,
> hole plates, flange unions, pipe pairs, 300 I and 300 L sweeps) passes under a checked-in assertion suite, including a
> contact/clash assertion per case. (b) Every call validates its result (volume against operand volumes or a closed form,
> manifold check) and on failure returns a labelled fallback, never a silent wrong answer. (c) Boolean operands are copied
> before the call (the kernel mutates its first operand). (d) The wheel version and sha256 are pinned and a version bump
> re-runs the corpus. (e) Documented limits: cp312 only, float32 tessellation (use a local frame), no STEP/IFC.
>
> **Out of scope.** Viewer cope rendering, fillet/shell/offset operations (open upstream defects), STEP/IFC export.
>
> **Risks.** Young project with a single vendor team and no outside code accepted; open upstream wrong-result issues in
> families outside this slice (see upstream issues #66, #72, #70, #65). Kill switch: disable the component, fall back to the mesh path.

## Reproduce

Needs Python 3.12 (`py -3.12`), Node 24, a scratch venv with `usd-brep==1.0.2 numpy manifold3d trimesh`. From the trial
folder: `python -I cases.py`; `python -I run_kernel.py a` (and `a-far`); `node --import ./three_register.mjs baseline_three.mjs`
then `python -I analyze_a.py`; `python -I run_b.py 100`; `python -I run_c.py`; `python -I run_d.py 300`; `python -I run_e.py` then
`node --import ./three_register.mjs check_viewer.mjs`; `python -I run_f.py 100`. Raw results are in `out/*.json`
(meshes under `out/mesh/` are git-ignored). `three_register.mjs` loads FloLess's own `web/vendor` three and
three-bvh-csg read-only.
