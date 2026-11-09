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
