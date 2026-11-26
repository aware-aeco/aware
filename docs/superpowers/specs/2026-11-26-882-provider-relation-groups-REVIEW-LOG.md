# #882 plan — Codex review log

Reviewer: Codex CLI (`codex exec`, read-only instruction), 2026-11-26, on
`2026-11-26-882-provider-relation-groups-PLAN.md`. Verdict: **APPROVE-WITH-CHANGES**. How each finding was handled:

1. **Alias-only changes and pair-mode scope.** `compare-snapshots` keys relationships by `(kind, from, to)`, so a
   `provider-explicit` row whose `providerRelationKind` alone changes is not a change. Accepted as a documented
   limit, not fixed here: changing the diff key changes `compare-snapshots` output for every provider snapshot.
   `relationship-missing` already judges every applicable object in either scope, so the standards rule is not
   affected; clash evidence for an unchanged pair is. Written into `check-snapshots.md` and the CHANGELOG; a
   follow-up issue covers the diff.
2. **Coverage proves absence, not presence.** Accepted. With `providerRelationKinds`, a usable row of either family
   passes the object whatever the coverage says; a finding needs both `kind` and `provider-explicit` in
   `relationships.complete`, and `provider-explicit` there is documented as "every provider-explicit row". Rules
   without the list keep the 0.6.x order (coverage first). Tests cover presence under partial coverage, absence
   under partial / full / no coverage, and usable-beats-lost.
3. **Weld guard source.** Accepted. A profile that lists `providerRelationKinds` on any group may have only one
   `hosts` group (`profile-invalid` otherwise); legacy profiles keep the first-`hosts`-group rule. Tested: alias
   weld in the assembly → `weld-parts-unknown`; weld only a component child → the contact stays a finding.
   native-v1's `contains`-for-assembly layout is FloLess history (no compatibility owed, owner 2026-10-31) and is
   noted for the FloLess side.
4. **Validation domain and structured index.** Accepted. Names follow the canonical v2 domain (no control
   characters, at most 256 code points). The provider index is a `Map<providerRelationKind, [from, to][]>`, never
   NUL-joined strings. Tests: control character refused, renumbered entity ids, a duplicate-identity end next to a
   usable row.
5. **Compatibility and rollout.** Accepted: the only expected difference for an old profile is `evaluator.version`
   (existing goldens run unchanged). FloLess sends the new key only when the installed model-change-set is 0.7.0 or
   newer; below that it keeps today's profile and says the assembly checks are limited.
