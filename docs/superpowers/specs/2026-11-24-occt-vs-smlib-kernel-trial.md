# Open Cascade (OCCT) against NVIDIA SMLib: one exact-geometry kernel for aware-aeco/aware#872

Date: 2026-11-24. Measured on this PC for the floless.app master session. Owner approved the comparison on 2026-11-24.
Scripts and results: `docs/superpowers/specs/smlib-trial/` (the earlier SMLib trial, plus the OCCT additions listed at the end).
Companion report: `2026-11-23-smlib-kernel-trial.md`. Same cases, same seeds, same placements.

## Verdict

**Pick OCCT as the one kernel for #872, with two conditions: all work is done in a member-local frame, and every boolean result is checked before it is trusted.**
SMLib stays out for now. Reasons, in order of weight:

1. **OCCT reads and writes STEP; SMLib does not.** A coped member went out to STEP and back with the volume unchanged to 2.3e-11 and the same 22 faces. Over 40 placed cases the worst volume change was 4.7e-9. SMLib has no STEP, IFC or IGES at all.
2. **OCCT matched SMLib on every ordinary case I ran.** Cope with end cut: 126 of 126 correct. The #4005 shape: 1,200 of 1,200 clean, 7 faces each, volume exact to 3e-13. Parametric I and L sweeps: 600 of 600. Hole plates and flange unions: 200 of 200.
3. **OCCT has one hard failure that SMLib did not show: it goes silently wrong far from the origin.** Beyond about 100 m from the origin, a cope cut quietly did nothing or came out wrong: 12 of 120 at up to 100 m, 48 of 120 at up to 1,000 m. No error, no exception. Re-centring both parts on the origin before the boolean fixed all of it: 0 wrong in 720 runs up to 5 km. SMLib was exact on 120 of 120 at 1,000 m. Our plan already works in the member-local frame, so this is manageable, but it must be a rule with a guard, not a habit.
4. **Footprint is much worse: about 485 MB installed against 108 MB.** The OCP wheel needs the VTK wheel and will not import without it.
5. **Maturity favours OCCT** (decades, large community), but that part is from general knowledge, not measured here (see "not verified").

If the owner puts a small footprint and a clean Apache licence above STEP and age, SMLib is the other defensible choice. I would still not ship either without the result guard in section 7.

## 1. Setup

| Item | Result |
|---|---|
| Binding | `cadquery-ocp` 8.0.1.1.0, `cp312-cp312-win_amd64`, **47,923,262 bytes (45.7 MiB)**, sha256 `0826714b3799d2b43fc0d2c6853a6921dfbc3b2a090ab8c98e9c8f08530c315d` |
| OCCT inside | The version string in `TKernel*.dll` reads 8.0.1. Treat as OCCT 8.0.x (read from the binary, not from release notes) |
| Required dependency | `vtk==9.6.2`, wheel **81,305,902 bytes**, sha256 `83b4af00b31395a13acb20e26a42ee097b85e41a0a087be2340d3749cb58250f`, licence BSD. pip also pulled `cadquery-ocp-proxy` 8.0.1.1.0 (a tiny version shim) and matplotlib and its chain through vtk |
| Python | Python 3.12.10 venv under `%TEMP%` (outside the repo), every script run with `python -I`. `Requires-Python: <3.15,>=3.11` in the metadata. I ran cp312 only |
| Other packages | `numpy`, `manifold3d` 3.5.5 (the same mesh baseline the SMLib trial used). Nothing else |
| Fallback binding | Not needed. OCP has a cp312 Windows wheel, so `pythonocc-core` was not touched |
| Not rebuilt | I did not rerun SMLib. Its numbers are the recorded results of the 2026-11-23 trial. The manifold3d columns of case (b) reproduced that trial exactly (81, 92, 33, 8, 17 ... coincident-face runs), which shows the placements and seeds are identical |

The adapter is `occt_ops.py`. It exposes the same calls as `usd_brep` (`create_box`, `linear_sweep`, `boolean_*`, `Brep.volume`, `tessellate`, `is_manifold_solid`, `brep_distance` and so on). `run_occt.py` installs it under the name `usd_brep`, so `cases.py`, `common.py`, `kernel_ops.py` and `run_kernel.py`, `run_b.py`, `run_c.py`, `run_d.py`, `run_f.py` ran **unchanged**. Only `run_e.py` needed a twin (`run_e_occt.py`), because it calls USD export.

Adapter choices that matter for reading the numbers:
- Booleans are `BRepAlgoAPI_Cut/Fuse/Common`, single thread. A boolean that is not done, or returns a null shape, raises.
- Volume is `BRepGProp.VolumeProperties` (fixed-order Gauss on the exact surfaces). An adaptive mode gave the same pipe values to 8 digits, so the default stands.
- "Valid" is `BRepCheck_Analyzer` plus: exactly one solid, closed shell, volume above zero.
- OCCT meshes each face separately. The adapter welds the nodes on a 1e-6 mm grid so the mesh is one indexed closed mesh (without that, manifold3d rejected the mesh and the clash overlap read 0).
- OCCT returns float64 meshes (SMLib returns float32).

## 2. Side-by-side, per case

SMLib numbers are from the 2026-11-23 trial. "Fail" means: throw, invalid solid, or a volume off by more than the case's limit.

| Case | SMLib | OCCT |
|---|---|---|
| **(a) cope + end cut, 126 cases**, volume against numeric reference | 126 of 126; max error 6.4e-8 | **126 of 126; max error 7.2e-8** (0 throws, 0 invalid, 0 silent wrong, 0 mesh defects) |
| (a) boolean time per case (1 to 2 booleans) | median 12.9 ms, p95 24.6 | median 16.9 ms, p95 33.2 (per boolean: median 11.9, p95 27) |
| (a) tessellate at 0.01 mm | median 31.5 ms, 662 tris, volume 2.7e-5 off | median 11.0 ms, 584 tris, volume 3.1e-5 off |
| **(a) placed up to 1,000 m**, 120 cases | **120 of 120 exact** (2 flagged only for float32 mesh) | **72 of 120. 45 silent wrong volumes (up to 2.1% off), 3 invalid solids, 0 throws** |
| (a) the same, re-centred first | not needed | 0 wrong of 120 at 50, 100, 200, 400, 1,000 and 5,000 m |
| **(b) #4005 shape, 100 placements at 1 m**, three variants that share faces | 0 of 300 defective | **0 of 300 defective** (manifold3d on the same placements: 81, 33, 92 of 100 have coincident sheets; control 0) |
| (b) full 1,200-run check (4 variants x 3 scales x 100) | 1,200 of 1,200, 7 faces, volume within 1e-12 | **1,200 of 1,200, 7 faces, volume within 3e-13** |
| (b) boolean time, median | 0.8 to 1.9 ms | 2.6 to 4.2 ms |
| **(c) clash, planar** (overlap 2, 0.01, 0.001 mm) | 4201.742636, 23.0400000005, 2.3040000005 | **4201.742636, 23.040000000002, 2.304000000002** (closed form 4201.74, 23.04, 2.304) |
| (c) touching versus gap | distance 0 "touching"; 0.05 gap = 0.05 | same: 0 "touching"; 0.04999999999999982 |
| **(c) pipe on pipe, 0.05 mm overlap** | 0.152065 | 0.152060 |
| (c) pipe overlaps against my own quadrature, relative error | 0.005: -2.1e-5; 0.02: -7.9e-6; 0.05: -6.3e-6; 0.5: -2.2e-7 | 0.005: -7.5e-5; 0.02: -4.9e-5; 0.05: -3.9e-5; 0.5: -2.6e-5 |
| (c) mesh at 0.01 chord, 0.05 mm pipes | 0.124 (19% low) | 0.113 (25% low) |
| (c) mesh at 0.1 chord, 0.05 mm pipes | 0.0208 (86% low) | 0.0178 (88% low) |
| (c) time: exact distance / exact overlap | 3 to 27 ms / 5 to 10 ms | 5 to 17 ms / 3 to 8 ms |
| **(d) 300 I sections** with root fillets | 300 of 300; volume 7.4e-14 | **300 of 300; 3.9e-15** |
| **(d) 300 L angles** | 300 of 300; 2.5e-13 | **300 of 300; 1.8e-15** |
| (d) build / tessellate (I), median | 8.6 ms / 17.6 ms (492 tris) | 1.7 ms / 7.6 ms (380 tris) |
| (d) mesh volume error (I), median / max | 3.0e-5 / 4.8e-4 | 4.1e-5 / 3.4e-4 |
| **(e) tessellation 0.5 / 0.1 / 0.01 / 0.001 mm**, time | 7 / 10 / 31 / 129 ms | 7 / 7 / 11 / 33 ms |
| (e) triangles | 138 / 218 / 688 / 2,550 | 188 / 234 / 610 / 1,782 |
| (e) sampled max deviation (mm) | 0.249 / 0.064 / 0.0040 / 0.00073 | 0.082 / 0.046 / 0.0047 / 0.00050 |
| (e) all meshes: open, non-manifold, zero-area | 0 / 0 / 0 | 0 / 0 / 0 |
| **(e) STEP write then read, coped member** | not available | **22 faces back, valid, volume 2.3e-11 off; write 11 ms, read 32 ms, 80 KB** |
| (e) STEP over 40 placed coped members | not available | 40 of 40 valid and same face count; max volume change 4.7e-9 |
| **(f) hole plates, 100 runs** | 100 of 100; median 37.7 ms (max 163) | **100 of 100; median 19.0 ms (max 119); error 2.2e-13** |
| **(f) plate on flange, 100 runs** | 100 of 100; median 2.5 ms | **100 of 100; median 5.0 ms; error 2.9e-13** |

Notes on reading the table:
- Times are indicative. The SMLib trial ran while CI was busy on this PC. My OCCT runs were sequential on a quiet machine, except one earlier case (b) pass that overlapped a long job (it gave 2.7 to 5.2 ms; the quiet reruns gave 2.6 to 4.2). Neither side is a benchmark.
- OCCT is slower on booleans by roughly 1.3x to 2.5x and faster on building and tessellating by roughly 2x to 5x. Per detail, they are the same order of magnitude.
- On curved intersections both kernels are below 1e-4 relative error. For a clash check that is far inside what matters. SMLib is about 4 to 120 times closer to my quadrature on the pipes.
- The pipe test as built by `run_c.py` makes a quarter lens (the second pipe starts at x = 0 and is centred on the first pipe's end plane). Both kernels agree on that geometry. My quadrature (`pipes_ref.py`) accounts for it.
- OCCT's mesh at 0.1 mm chord misses most of a 0.05 mm overlap, like SMLib's. Use the exact intersection volume for a verdict, as the earlier report said.

## 3. Fuzzy booleans, and what fixed what

`SetFuzzyValue` at 1e-6 mm and 1e-4 mm:

| Case | Default | Fuzzy 1e-6 | Fuzzy 1e-4 |
|---|---|---|---|
| (b) shared-face sheets, 1,200 runs | 0 defective | 0 defective | 0 defective |
| (f) coincident faces, 200 runs | 200 of 200 | 200 of 200 | 200 of 200 |
| (a) near, 126 | 126 of 126 | 126 of 126 | 126 of 126 |
| (a) 1,000 m, 120 | 72 of 120 | 72 of 120 | 72 of 120 |
| (c) planar overlaps | exact | exact | exact |
| (c) pipe overlap error, 0.05 mm | -3.9e-5 | -4.0e-5 | +5e-9 |

Conclusion: fuzzy did **not** fix the far-coordinate failure and was not needed for the shared-face case. On the pipes it moved some values closer and others not (no pattern). I would leave it off. What fixed the far failure was re-centring. I also checked that it is not caused by baking the placement into the geometry: with the placement stored as a location only, the result was the same 48 of 120.

## 4. Failures, kernel by kernel

| Kind | SMLib (this trial and its tracker) | OCCT (this trial) |
|---|---|---|
| Throws | 0 in all runs | 0 in all runs |
| Invalid or non-manifold solid | 0 | 3 of 120 at 1,000 m; 0 elsewhere (about 2,200 other runs) |
| Silent wrong volume | 0 in the runs; upstream tracker lists wrong-result booleans in families I did not test (capsule union #66, analytic against NURBS #72) | 45 of 120 at 1,000 m (cut did nothing or removed the wrong part); 12 of 120 already at up to 100 m; 0 up to 50 m |
| Float32 trap | tessellation returns float32 (0.06 mm at 1 km) | none, float64 |
| In-place mutation of operand | yes | not tested; the adapter never relied on it |

The OCCT failure is the dangerous kind: the result passes a quick look. The shape is a valid solid, only the cut is missing. A guard has to compare the result volume with the operands (for a cut: result must be smaller than the part by about the overlap, never larger or equal when the operand overlaps).

## 5. Licence and redistribution for AWARE

- **The wheel's metadata and its only licence file say Apache-2.0.** That covers the OCP binding code. The OCCT libraries inside (`TK*.dll`, renamed with hashes by the wheel builder) are not Apache.
- **OCCT itself is LGPL-2.1 with the Open CASCADE exception.** I did not read that text in this session; this is from general knowledge, so have it confirmed.
- What it means for shipping, in plain terms: AWARE may redistribute the unchanged wheel and call it from its own code. It must ship OCCT's licence text (the wheel does not include it), say OCCT is in the bundle, keep the OCCT libraries as separate replaceable files, and offer the corresponding source of OCCT on request. The exception is there so that using OCCT does not force the user's own code under the LGPL. If AWARE ever patches OCCT, those patches must be offered under the same terms.
- The delvewheel hashed DLL names still count as separate files, but "replaceable by the user" is easier to defend if AWARE ships the wheel as is rather than re-packing the DLLs.
- VTK is BSD, no obstacle. SMLib is simpler: Apache-2.0 plus permissive licences, no LGPL step.
- A lawyer should confirm the LGPL and exception handling before AWARE bundles it.

## 6. Footprint

| | SMLib 1.0.2 | OCCT (cadquery-ocp 8.0.1.1.0) |
|---|---|---|
| Wheels to download | 16.8 MB (Windows) | 47.9 MB + vtk 81.3 MB = **129 MB**, plus matplotlib and its chain |
| Installed, kernel only | 107.7 MB | OCP 105 MB + OCCT libs 65 MB + vtkmodules 50 MB + vtk.libs 265 MB = **about 485 MB** |

I tried to drop VTK: without the `vtk.libs` folder the import fails (`os.add_dll_directory`), and with an empty folder the OCP module fails to load its DLLs. So VTK is a real dependency of this wheel, not an optional extra. Slimming it would mean building OCP ourselves or finding another packaging. I did not try either.

## 7. Maturity and risk

Measured here: the wheel's files are dated 2026-10-02; the binding version 8.0.1.1.0 sits on a 8.0.x OCCT; there were no throws in any run; the one failure class is the far-coordinate one above.
From general knowledge (not checked this session): OCCT is more than 25 years old, open source since 2000, maintained by Open Cascade SAS with a public tracker and a large user community (FreeCAD, CadQuery, Salome, IfcOpenShell). Its boolean operations are known to have a long list of corner cases; I did not count the open ones. SMLib's public life is six weeks, one vendor team, no outside code accepted (earlier report).

Risks I would carry into #872:
- **Silent wrong result at large coordinates.** Measured. Rule: always re-centre in the member-local frame. Guard: compare result volume with the operands, run `BRepCheck`, and on any doubt fall back to the mesh path with a label ("nothing refuses, it asks").
- **Footprint and the VTK dependency.** Measured.
- **LGPL obligations.** Needs a lawyer.
- **Boolean corner cases outside my corpus.** Unmeasured (fillets, shells, offsets, helixes).
- **Packaging.** `cadquery-ocp` is a community-maintained wheel, not an Open Cascade product.

## 8. IFC relevance

IfcOpenShell is built on OCCT (general knowledge; I did not run it). If AWARE adopts OCCT, then IFC geometry going in and out uses the same B-rep objects as the kernel: an IFC extruded or B-rep solid can be turned into an OCCT shape, cut, measured and written back. With SMLib the route would be IFC to OCCT to SMLib through an OCCT tool anyway, and the earlier report notes known defects in the OCCT-to-USD converters. For an AEC product that must read and write IFC and STEP, one OCCT kernel avoids a two-kernel bridge. I verified STEP only; IFC is not tested.

## 9. Fit with AWARE's managed Python 3.12 runtime

- The wheel is cp312 and worked in a Python 3.12.10 venv, which matches the managed runtime plan.
- Its metadata allows 3.11 to 3.14. The frozen parser runs Python 3.13.2, where the SMLib wheel (cp312 only) cannot be imported. Whether a cp313 OCP wheel exists I did not check. If it does, OCCT could run in the parser process; if not, same out-of-process shape as SMLib.
- Shape stays the same: JSON in, JSON out, member-local frame, exact numbers plus an indexed mesh.
- Pin both wheels by sha256 (kernel and VTK), because a floating VTK would change the kernel's runtime.
- Determinism: volumes were identical between my repeated runs (a, a-far, c, f gave the same numbers run to run). Cross-machine bit equality: not tested.

## 10. Recommendation for #872

**One kernel: OCCT.** Both kernels solved the shapes that matter, so the deciding facts are STEP and IFC, maturity, and the 485 MB against 108 MB. If the footprint is a blocker for the first release, the only alternative that fits the trial evidence is SMLib, with STEP/IFC left out of scope.

What changes in #872's first slice, compared with the SMLib draft:
1. Pin `cadquery-ocp` 8.0.1.1.0 and `vtk` 9.6.2 (sha256 above) instead of `usd-brep`. Ship OCCT's LGPL text and notice with the bundle.
2. Same operations as drafted (`boolean`, `distance`, `relationship`, `interference_volume`, `tessellate`), plus `step_write` and `step_read`.
3. **Make re-centring mandatory.** Move both operands to the member-local origin before any boolean or measure, and translate back after. Add a regression test with parts 1 km from the origin (the 120-case probe in `probe_far.py`).
4. **Add a result guard** on every boolean: valid and closed per `BRepCheck`; volume consistent with the operands; on failure return a labelled fallback to the mesh path, never a silent answer.
5. Weld the per-face mesh nodes (the adapter does) and choose the chord height per use (0.01 mm is 11 ms for a coped member).
6. Done criteria use the corpus already in `smlib-trial/`, now run with `run_occt.py`, including a contact assertion per case.
7. Leave fuzzy off. Use the exact intersection volume for clash verdicts, not the mesh.
8. Budget the first slice slightly above the SMLib estimate for the licence step and the VTK packaging question.

## What I did NOT verify

- Linux and macOS wheels; only the Windows cp312 wheel ran. No cp313 wheel check.
- IFC at all, and IfcOpenShell. STEP only on the coped members and 40 placed variants; not on assemblies, curved members, or other CAD systems' STEP files.
- The OCCT licence text itself, the LGPL implications (stated from general knowledge), and AWARE's actual bundle layout.
- OCCT maturity facts: age, cadence, open boolean bugs, community size. None were looked up (no network research was allowed).
- Why the far-coordinate failure happens. I measured it, ruled out baked-versus-location placement and fuzzy, and showed re-centring fixes it. I did not trace it into OCCT.
- Whether OCCT booleans mutate their operands (the adapter never depended on it), and multi-threaded runs (single thread only), memory growth, large assemblies.
- SMLib was not rerun here; its numbers come from the 2026-11-23 trial. Timings between the two trials are not like for like.
- Real parser data for #4005; the shape is the same synthetic one as in the SMLib trial.
- Operations outside the corpus: fillet, shell, offset, helical sweeps, NURBS surfaces.
- Whether a smaller OCCT packaging (no VTK) exists. I tried only removing it from this wheel.

## Files added in `smlib-trial/`

`occt_ops.py` (the adapter), `run_occt.py` (runs any trial script on OCCT; `--fuzzy MM`, `--adaptive`), `run_e_occt.py` (tessellation and STEP), `probe_far.py` and `probe_far_loc.py` (far-coordinate probes), `pipes_ref.py` (independent pipe overlap volumes), and `out/occt_*.json` (summaries and failing rows only), `out/pipes_reference.json`.

Rerun (Python 3.12, venv with `cadquery-ocp==8.0.1.1.0 vtk==9.6.2 numpy manifold3d`), from the trial folder:
`python -I run_occt.py run_kernel.py a` (and `a-far`); `run_occt.py run_b.py 100`; `run_occt.py run_c.py`; `run_occt.py run_d.py 300`; `run_occt.py run_f.py 100`; `run_occt.py run_e_occt.py`; `run_occt.py probe_far.py`; `pipes_ref.py`. Add `--fuzzy 1e-4` before the script name for the fuzzy runs.
