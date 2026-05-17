# Federation and shared coordinates

The Navisworks federation is a *reference* to constituent models — the
file extension matters:

| Extension | What it is | When to use |
|---|---|---|
| `.nwf` | Federation document — references the source models | Working file, models stay external + live-updating |
| `.nwd` | Consolidated — all model geometry baked in | Deliverable, immutable snapshot |
| `.nwc` | Per-model cache — Navisworks's internal compiled form | Output of `Save as NWC` from Revit/CAD/etc. Don't reference directly; append in `.nwf` |

## Shared coordinates

Always confirm the source models carry consistent shared coordinates
before federating. The fastest sanity-check:

1. `federation.open` the existing `.nwf`
2. `viewpoint.list-saved` and pick a known grid intersection
3. `viewpoint.capture-current` — eyeball the new federation against the
   captured viewpoint

If the new model is offset, **don't** "fix it in Navisworks" — that
hides the source bug. Send it back to the discipline lead with a
coordinate-system mismatch RFI.

## Threading

Every Navisworks API method must run on the UI thread. The
`aware-navisworks-2026` transport binary takes care of this — the
Roamer plugin marshals calls onto the UI dispatcher. If you ever see
a `COMException 0x800F0000`, that's the threading constraint biting;
file a bug rather than working around it.

## Performance

- `clash.run-all` on a 4 GB federation takes 4-12 minutes depending on
  test count. Schedule via the v0.19 `schedule:` primitive overnight.
- `selection.by-property` walks every ModelItem; expect ~30s for a
  100k-item federation.
- `viewpoint.capture-current` is fast (~1s) but `viewpoint.from-clash`
  has to render a focused view + dim non-participants — ~3-5s per call.

## Common pitfalls

- **Append vs merge:** `mode: append` keeps source models separate (the
  default); `mode: merge` collapses items into the parent. Almost
  always use `append`.
- **Save format mismatch:** if `path` ends `.nwf` but `format: nwd`,
  the format wins and the extension is rewritten — be explicit.
- **Locked federations:** if the federation is currently open in
  another Navisworks instance, the transport returns `E_LOCKED`. Close
  the other instance or use a copy of the `.nwf`.
