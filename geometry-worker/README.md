# AWARE geometry development contract

The #872 M1 development build targets Windows x64. Build the CLI with `--features geometry-dev` and stage it alongside the pinned Node 24.14.0 and Manifold 3.5.4 helper after `npm --prefix cli-connection-reader ci`. Use `stage-development.ps1` to create the test layout. Top-level help omits the geometry group. Ordinary builds omit it entirely. [M1 acceptance and delivery gates](../docs/superpowers/specs/2026-11-28-872-occt-brep-exec-PLAN.md)

```text
aware geometry install --occt --python <absolute-python.exe> [--wheelhouse <directory>] --json
aware geometry exec @request.json --json
aware geometry exec - --json
```

`fixtures/generate_difference.py` generates paired BRep and indexed meshes for dimensioned convex prisms. The authoritative decimal profile is in member coordinates. The returned BRep and mesh use millimetres near the operation origin. Reconstruct a world point as `origin_mm + axes * (offset_mm + local_point)`, using the columns of `axes`.

Each invocation reads one request with a 32 MiB UTF-8 input limit, 100,000 triangles per mesh and a 32 MiB stdout envelope limit. The supervised child caps are 32 MiB stdout and 64 KiB stderr with a 30-second deadline per launch. Successful responses echo both operand IDs. A `mesh_fallback` result has null BRep and a `chord_height_unverified` warning. An `unavailable` response exits zero with a machine-readable `reason_code`, the operand IDs and `result: null`. Malformed requests use the error envelope and exit nonzero.

The section oracle arranges the authoritative convex XY profiles with exact rational predicates and integrates retained and removed occupancy over open Z slabs. It checks vertical sides and cap transitions against the BRep's oriented face wires and the delivered indexed mesh. A cut can have several separated components, a through-hole, or an enclosed cavity. Cavity shells contribute negative signed volume. Touching shells with disconnected vertex fans are rejected.

Admission adds per-axis budgets for producer error, placement and anchor reconstruction, construction arithmetic, and each pinned kernel. The exact path includes the OCCT subshape tolerance cap, native fuzzy floor and seam weld allowance. The mesh path includes input and output float32 rounding and Manifold's scale-based tolerance. The two paths share the stricter clearance threshold of eight times the resulting budget. The exact tessellation uses the requested chord height and sews shared OCCT edges and vertices within 0.0000001 mm before checking oriented mesh topology. [Pinned Manifold tolerance calculation](https://raw.githubusercontent.com/elalish/manifold/v3.5.4/src/impl.cpp)

The Windows **Geometry M1 gate** checks the named suites, the replayed raw failure and the distinct normal and fault-enabled artifacts against the candidate SHA and lock. The fault-enabled build uses cargo feature geometry-test-faults and is for test staging only. For delivery and wheel-lock changes, use the [separate offline provisioning gate](OFFLINE-GATE.md).

## Native runtime and notices

The two-wheel [manifest](wheel-manifest.json) and [hash lock](win-amd64-cp312.requirements.lock) record the trial set. In a clean CPython 3.12.10 x64 venv, OCP alone could not load the VTK libraries. Adding VTK 9.6.2 allowed both imports and reported OCP 8.0.1.1 with no proxy or matplotlib chain installed. The OCP distribution is 8.0.1.1.0. Its bundled OCCT is 8.0.1, and `SetFuzzyValue(0)` reads back `1e-7 mm`.

Install verifies wheel lengths and SHA-256 before the isolated local pip command. It binds the completion marker to the lock, native probe, base Python executable and Python runtime DLL. Exec verifies the recorded base identity before launching the exact worker.

The embedded notices include the [OCP Apache-2.0 text](licenses/OCP-LICENSE.txt) from `cadquery_ocp/LICENSE`, the [VTK license](licenses/VTK-LICENSE.txt) from `vtk-9.6.2.dist-info/licenses/LICENSE`, and the [OCCT LGPL-2.1 text](licenses/LICENSE_LGPL_21.txt) and [exception](licenses/OCCT_LGPL_EXCEPTION.txt) from the corresponding OCCT revision. Preserve these texts and the corresponding OCCT source reference with an opt-in runtime distribution. [OCCT 8.0.1 source revision](https://github.com/Open-Cascade-SAS/OCCT/tree/b8f597c677811d1f9f4d8a97f5ae2825c0353a42)

## Flush replay

The [stress fixture](fixtures/flush-stress.json) uses the 400 × 250 × 20 mm trial plate and the 60 × 45 mm flush clip. It derives the placement from row zero of a fixed sweep with seed 4005. Under uncentred float32 conversion, Manifold reports `NoError` for both constructors and subtraction, and returns `1973002.2216796875 mm³` against the analytic `1973000 mm³`. The replay treats a discrepancy above `0.01 mm³` as a failed volume invariant. [Trial report](../docs/superpowers/specs/2026-11-24-occt-vs-smlib-kernel-trial.md)
