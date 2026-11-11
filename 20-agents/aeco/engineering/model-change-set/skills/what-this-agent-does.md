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
