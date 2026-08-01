# Breaking changes

## 1.0.0 — `read-model` returns the file's own Z-up frame (was web-ifc's Y-up)

**What moved:** every vertex in `read-model`'s `objects[].positions`.

`0.1.0` returned web-ifc's tessellation frame. web-ifc bakes a fixed rotation —
`(x, y, z) → (x, z, −y)` — into every flat mesh transform so its output lands in the **Y-up** frame a
renderer wants. That was passed straight through while the manifest documented *"the file's own world
frame"*. `1.0.0` undoes the rotation, so the coordinates are now IFC's own **Z-up** world frame: X and
Y in plan, Z up — the same frame `probe`'s `bbox` reports.

Measured on `example-steel-framing.ifc`, a 12 m × 6 m grid on 4500 mm columns:

| | X | Y | Z |
|---|---|---|---|
| `probe` bbox span (unchanged) | 12000 | 6000 | 4500 |
| `read-model` mesh span — `0.1.0` | 12150 | **4625** | 6150 |
| `read-model` mesh span — `1.0.0` | 12150 | **6150** | 4625 |

### If you are upgrading

- **Delete any Y-up → Z-up rotation you added.** Applying one now rotates a correct model out of
  true. That workaround is the reason this is a breaking change rather than a patch: code written
  against `0.1.0` is *correct against `0.1.0`* and wrong against this.
- **Composing with `viewer-3d`, declare `meta.up: "z"`** (it was `"y"`). The scene schema keeps
  coordinates in producer space and converts via `meta.up`, so this is a declaration, not a rotation.
- **Composing with `ifc.write`, you can now stop compensating.** That writer emits positions verbatim
  as absolute IFC coordinates and does not read `meta.up`, so `0.1.0`'s Y-up mesh round-tripped into a
  sideways IFC. `1.0.0` matches IFC's own frame, so the round trip is upright.
- **`probe` is unchanged.** It always reported the file's frame; that is the whole point — the two
  commands now agree.

### How to tell which frame you actually got

Read the `frame` field in the output (`"z-up"` for both `probe` and `read-model`, from `1.0.0`).

Do **not** infer it from a version number. The geometry is produced by the `aware-connection-reader`
bridge binary, which is installed separately (`aware sidecar install connection-reader`) and, when
stale, only prints a warning and runs anyway — so this manifest can read `1.0.0` while an old bridge
returns the old frame. Measured 2026-08-01: an app's `requires:` pin is enforced neither at compile
nor at run time either. The output field is the only trustworthy answer.

### Not affected

`connection-reader.extract` — a different agent with a different contract. Its `parts` are still in
web-ifc's Y-up frame and its `frame` field says `"y-up"`. Aligning the two is tracked in
aware-aeco/aware#347.

Issue: aware-aeco/aware#343.
