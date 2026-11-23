# What this agent does

`model-change-set` answers one question about a model that was saved twice: **what changed between
the two saves?** It pairs the objects of the two saved states by a native identity (for example a GUID)
and reports what was added, removed and changed.

It knows nothing about any file format. You pass each state's three canonical query documents and a
comparison profile that says which property is the identity, which keys are volatile, how renumbered
definition ids are mapped to stable names, which keys are shape fields, and how far a bounding box may
move. See `commands/compare-snapshots.md` for the profile and the output.

Pairing is by identity only. Objects with no usable identity are excluded with receipts, never guessed.
When almost nothing pairs, the result says `discontinuity` instead of presenting a regeneration as a
change list. Placement is compared on bounding boxes only, and holes, cuts, fittings and solid
geometry are not compared; the result says so in `unchecked`.

## Then: what the change touched, checked

`check-snapshots` runs a pinned set of checks (a check profile) over the objects that were added or changed,
plus any identities you name, and says which findings the change **introduced**, which were **already
there** and which it **resolved**. A finding has a `correspondenceKey` (which rule, which objects) and an
`evidenceFingerprint` (what it says), so "still there, unchanged" and "still there, different" are told apart.
Unchanged objects you did not name are not checked, and the result says so in `unchecked`. See
`commands/check-snapshots.md` for the check profile and the output.

## Standards: one object at a time (0.4.0)

A v2 check profile can also state **standards** about one object: a property that must be present, must be
one of a list, or must lie in a range, and a relationship that must exist. AWARE does not know what the
property means; the profile names its canonical key. A standard is judged only where the source says it
read that property completely (the coverage input); anywhere else the result says "not checked" with the
reason, never "passed". See `commands/check-snapshots.md`.

## One version on its own (0.5.0)

`check-snapshots` with `mode: "single"` checks one saved model version against a v2 standards profile and lists what is
wrong in it now ("present in this version"). It compares nothing, so it says nothing about what changed or whether a
problem is new. From 0.6.0 it also runs clash on that version, given its geometry tiles (`revised-geometry-dir`), and can
be focused on a few objects (a bolt, a member): it then checks those, their connection parts and what touches them, and
says so. See `commands/check-snapshots.md`.

## Clash

A check profile can also ask for **clash**: parts that pass into each other (with the thickness of what
they share), parts that touch with nothing bolting or welding them, and bolted plies that do not meet. These
read the parts' solids from the geometry tiles you pass in (`base-geometry-dir`, `revised-geometry-dir`).
A part whose solid could not be built, or is only approximate, is reported as not checked - never as "no
clash". A profile can set how thin an overlap may be before it is reported (`minOverlap`; thinner ones are counted,
not listed) and which type pairs each clash rule checks (`appliesTo.pairs`). Bearing at each bolt is not checked. See `commands/check-snapshots.md`.
