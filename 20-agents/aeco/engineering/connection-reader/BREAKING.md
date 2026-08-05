# Breaking changes

## 1.0.0 — `extract` returns the file's own Z-up frame (was web-ifc's Y-up)

**What moved:** every vertex in `extract`'s `connection.parts[].positions`.

`0.1.0` returned web-ifc's tessellation frame. web-ifc bakes a fixed rotation —
`(x, y, z) → (x, z, −y)` — into every flat mesh transform so its output lands in the **Y-up** frame a
renderer wants, and `extract` passed that straight through. `1.0.0` undoes it, so the coordinates are
IFC's own **Z-up** world frame: X and Y in plan, Z up.

This finishes what `ifc-reference-reader@1.0.0` started. That release fixed `read-model` and left
`extract` alone, documenting the split rather than removing it — so one bridge binary answered in two
frames, and the difference lived only in prose. It does not any more: `extract`, `read-model` and
`probe` all report `"z-up"`.

Measured on `test-fixtures/baseplate-bp1.ifc`, a 400 × 400 base plate with 250 mm anchors:

| `extract` mesh span | X | Y | Z |
|---|---|---|---|
| `0.1.0` | 400 | **250** | 400 |
| `1.0.0` | 400 | 400 | **250** |

The anchor height moves from axis 1 to axis 2. Nothing changes size — it is a rigid rotation.

### If you are upgrading

- **Delete any Y-up → Z-up rotation you added.** Applying one now rotates a correct connection out of
  true. That workaround is the reason this is a breaking change rather than a patch: code written
  against `0.1.0` is *correct against `0.1.0`* and wrong against this.
- **Composing with `viewer-3d.render`, declare `meta.up: "z"`** (it was `"y"`), or leave it at the
  default. The scene schema keeps coordinates in producer space and converts via `meta.up`, so this
  is a declaration, not a rotation.
- **Composing with `ifc.write`, stop compensating.** That writer emits positions verbatim as absolute
  IFC coordinates and does not read `meta.up`, so `0.1.0`'s Y-up parts round-tripped into a sideways
  IFC — `extract.md` used to tell you to rotate `(x, y, z) → (x, −z, y)` first. `1.0.0` matches IFC's
  own frame, so that composition is upright with no step at all.
- **`list` is unchanged.** It carries no geometry.

### `recipe` consumers are NOT affected

When `extract` recognizes a connection it also returns a parametric `recipe`, whose params are
**scalars in millimetres** (plate size, thickness, bolt grid, Ø, edge distance) that the consumer
re-derives on its own member. No frame ever entered them. A consumer that imports recipes and falls
back to mesh only for unrecognized connections is affected **only on that fallback path** — which is
precisely the path this fixes.

### How to tell which frame you actually got

Read the `frame` field on `connection` (`"z-up"` from `1.0.0`).

Do **not** infer it from a version number. The geometry is produced by the `aware-connection-reader`
bridge binary, which is installed separately (`aware sidecar install connection-reader`) and, when
stale, only prints a warning and runs anyway — so this manifest can read `1.0.0` while an old bridge
returns the old frame. The output field is the only trustworthy answer.

(An app's `requires:` pin is now enforced at compile and run — aware-aeco/aware#349 — so
`connection-reader@1.x` in `requires:` will refuse a `0.1.0` **agent**. That still does not police the
bridge binary, which is the thing that produces the coordinates.)

Issue: aware-aeco/aware#347. See also `ifc-reference-reader`'s `BREAKING.md` 1.0.0, the same change
one command over.
