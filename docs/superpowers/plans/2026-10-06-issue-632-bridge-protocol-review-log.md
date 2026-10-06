# Issue #632 plan review log (Codex gpt-6-sol, read-only)

Round 1 - REVISE (6): strict-gate claim incomplete (also model-reference-reader), chosen-manifest rule,
nested wrapper>leaf, null-vs-absent declaration, marker must be installer-attested with stale-claim
protection, CLI-boundary test for the owner-rule decision. All taken.
Round 2 - REVISE (2): dispatch matches binary names exactly (no .exe normalisation) and falls back to PATH;
flat vs sub-dir executable preference means install must remove both before stamping. Both taken.
Round 3 - REVISE (1): bridge_is_current must include the protocol stamp so install/repair recover a bad
stamp. Taken. The remaining item was a narrow recovery case, folded into the plan and verified by test;
no fourth round was spent on it.
