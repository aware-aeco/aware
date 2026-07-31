# What this agent does

`ifc-reference-reader` reads a **whole IFC file** as a reference model — geometry you overlay,
measure and build to, but never own.

## The problem it solves

Most modelling happens against somebody else's model. The architect sends an IFC; the structural
engineer sends another; you need yours to line up with both. Loading those files as *your* content is
wrong — they are not your scope, they must never reach your bill of materials or your exports, and
they will be re-sent next week with changes. What you actually want is to **see** them, snap to them,
and build against them.

This agent returns exactly that: every element in the file as mesh geometry, plus what the file says
about each one.

## How it differs from `connection-reader`

They read the same format through the same bridge and answer opposite questions.

| | `connection-reader` | `ifc-reference-reader` |
|---|---|---|
| Question | "pick ONE connection out of this file and make it mine" | "show me this WHOLE file, which is not mine" |
| Scope | one `IfcElementAssembly`'s plates/bolts/welds | every element |
| Intent | import — it becomes your geometry | reference — it stays theirs |

That difference in intent is why they are separate agents rather than four commands on one.

## How it works

- **web-ifc does the parsing.** The bundled WASM engine opens the file and tessellates real geometry —
  extrusions, triangulated facesets, faceted BREP, mapped/instanced items. This agent does not
  re-implement an IFC geometry kernel; it groups and maps.
- **`probe` first, `read-model` second.** `probe` answers schema/units/count/extent without walking
  geometry, so a consumer can refuse a file before paying to load it.
- **Instancing is preserved.** Real exports serve many objects from few shapes. Geometry is emitted
  per instance; de-duplicating by shape would silently drop objects.

## Two things worth knowing

**Units are already applied.** web-ifc reads `IfcUnitAssignment` and normalises geometry to metres
before you see a vertex. `probe`'s `units.declared` is therefore *provenance* — what the file claims —
not a factor to multiply by. Measured across four real files: three millimetre files came back divided
by 1000 and a metre file was left alone. Scaling geometry by the declared unit would break every file
that is honest, to rescue the rare one that lies.

**Correct size does not mean visible.** A real export usually sits at real site coordinates. One of
the test files loads at its true size, 74 m up and 40 m out — perfectly correct and entirely
off-screen next to a model at the origin. `probe`'s `bbox` is there so a consumer can notice that and
offer to zoom to it or move it, which is a far more common problem than wrong scale.

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `ifc-path is required` | no path given | pass `ifc-path` |
| `ENOENT …` | file not found | check the path |
| `too complex to load as a reference model` | exceeded `max-vertices` | raise the budget, or load a lighter export |
