# #882 — check-snapshots: let a relationship check name provider relation kinds

Status: plan (pre-code), 2026-11-26. Issue: aware-aeco/aware#882. Downstream: pawellisowski/floless.app#2617 (Change Review).

## Problem

`model-change-set check-snapshots` reads relationships by their `kind` only. Three places do:

1. **Clash groups** (`geometry.groups`, `cli-connection-reader/snapshot-clash.mjs`): `{ kind: "hosts"|"contains", name }`.
   A contact finding's evidence carries one boolean per group (`sameAssembly`, `sameComponent`). Validation refuses
   any other kind.
2. **The weld-parts-unknown guard** (same file, `hostsGroup` / `inUnknownAssembly`): a weld whose parts are unknown
   and that sits in the same `hosts` parent as both touching parts makes the contact *not evaluated*
   (`weld-parts-unknown`) instead of a finding. It uses "the group whose kind is `hosts`".
3. **`relationship-missing`** (`snapshot-check-rules.mjs`): `{ kind, end }`; passes when the object is the `to`/`from`
   end of a row of that `kind`; judged only when the side's read coverage lists `kind` in `relationships.complete`.

A canonical v2 snapshot may write a relationship as `kind: "provider-explicit"` with a `providerRelationKind`
(`model-canonical-v2.mjs` admits exactly that closed shape). FloLess's admitted Tekla snapshots (native-v2/v3) write
assembly membership as `provider-explicit` / `tekla-assembly-member` and component children as
`provider-explicit` / `tekla-component-child` — no `hosts`/`contains` rows at all. Effects today:

- (1) `sameAssembly` / `sameComponent` are always `false` — the #882 report (wording hint, no verdict).
- (2) the weld-parts-unknown guard never fires — a **verdict** effect: a contact that should be *not evaluated* is
  reported as a finding when the weld that may join it has unknown parts. (Found while planning; not in the issue.)
- (3) a firm rule "each beam must be in an assembly" (`relationship-missing`, `kind: hosts`) finds **every** beam,
  because FloLess's read coverage truthfully says the hierarchy was read completely (`complete: [contains, hosts]`)
  and there are no `hosts` rows. A false error, and it cannot be fixed by the profile today, because
  `relationship-missing` cannot name a `providerRelationKind`. (Found while planning; floless `quality-profile.ts:80`.)

## Options considered

- **A. Hard-wire `provider-explicit` + known provider kinds as `hosts`/`contains`.** Puts Tekla names into a generic,
  format-blind substrate. Rejected (CLAUDE.md: format meaning stays in the profiles).
- **B. A group may be `{ kind: "provider-explicit", providerRelationKind, name }`** (the issue's first proposal).
  Generic, but one matcher per group: a profile that must cover snapshots written both ways (FloLess native-v1 used
  `contains`, v2/v3 use provider-explicit) cannot, because group names must be unique; and the weld guard (2) loses
  its anchor (no group has kind `hosts`), so it would have to be re-specified.
- **C (chosen). Keep the semantic `kind`; let the profile add the provider relation kinds that mean the same thing.**
  A group is `{ kind: "hosts"|"contains", name, providerRelationKinds?: [string, …] }`; a `relationship-missing`
  rule's parameters are `{ kind, end, providerRelationKinds?: [string, …] }`. A row matches when its `kind` equals
  `kind`, **or** its `kind` is `provider-explicit` and its `providerRelationKind` is one of the listed strings. The
  profile says what the provider's names mean; AWARE learns no provider name. `hosts` stays the anchor for the weld
  guard, which now sees the aliased rows too, so (2) is fixed by the same field.

## Contract (C)

`providerRelationKinds` — optional on a `geometry.groups[i]` and on `relationship-missing` parameters:

- When present: an array of 1..32 strings, each non-empty, at most 256 characters (the canonical v2 limit), unique
  (exact code-unit comparison). Anything else is `profile-invalid`. `[]` is refused (say nothing instead).
- Absent: exactly today's behaviour. Every existing profile validates and evaluates byte-identically (goldens stay).
- Group `kind` vocabulary is unchanged (`hosts`, `contains`); `relationship-missing.kind` stays any bounded string.
  A `relationship-missing` rule with `kind: "provider-explicit"` and `providerRelationKinds` is refused — say
  `kind` is the semantic kind; aliasing provider-explicit onto itself is meaningless. (Group kinds cannot be
  provider-explicit already.)
- Matching reads `providerRelationKind` from the **source row** only when the row's `kind` is exactly
  `provider-explicit` and the field is a string; other rows never match an alias.

`relationship-missing` with aliases, per object:

- Coverage: judged only when the side's coverage lists `kind` **and** `provider-explicit` in
  `relationships.complete` (the rule reads both families; a side that did not read one of them completely
  cannot say "missing"). Otherwise `source-did-not-read` / `coverage-not-supplied` as today. The coverage schema
  is unchanged (`complete` is already a list of free kind strings).
- Passes when the object is the `end` of a `kind` row or of a provider-explicit row whose `providerRelationKind`
  is listed. `relationship-endpoint-unusable` applies to a lost end of either family, after the pass check (as today).
- Evidence on a finding: `{ kind, end }` unchanged, plus `providerRelationKinds` (the listed array) **only when the
  rule has it**, so old outputs stay byte-identical.

Clash groups with aliases: the parent map for a group takes every matching row (kind or alias); `shares` and the
hosts-anchored weld guard use it unchanged. Evidence keys (`sameAssembly`) unchanged.

Echo: `supports` gains `relationshipAliases: true`? — no: callers gate on the evaluator version (FloLess already
does, `ensureCompareAgent()` + dotted compare). Keep `supports` unchanged; the CHANGELOG and the doc name 0.7.0.

Versioning: `model-change-set` 0.6.1 → **0.7.0** (new optional profile field = minor), `EVALUATOR_VERSION` 0.7.0,
connection-reader bridge package 0.7.0 → **0.8.0**, manifest `minimum-cli-version` → the CLI release that carries
bridge 0.8.0 (next release, 0.187.0 — set by the release commit if the master picks another number; the registry
record follows the merge in its own commit, per #874/#881). The profile `schema` strings stay `…/v1`/`…/v2`: an
added optional field is not a new schema, and an older evaluator refuses the unknown key loudly (`exactKeys`), never
silently ignores it — so a caller sending the field to an old evaluator gets `profile-invalid`, not a wrong answer.

## Implementation

- `snapshot-compare.mjs` side index: alongside `relationships` (kind\0from\0to, used by the diff — unchanged), add
  `providerRelationships: Set<prk\0fromIdentity\0toIdentity>` and `providerRelationshipEnds: Map<prk, ends>` built from
  rows with `kind === 'provider-explicit'` and a string `providerRelationKind`. Diff output unchanged.
- `snapshot-clash.mjs`: shared `parseProviderRelationKinds(value, label)` validator (exported for rules); group
  `exactKeys(group, ['kind','name'], label, ['providerRelationKinds'])` (check exactKeys' optional-key form);
  parent map unions `relationships` (kind) and `providerRelationships` (aliases).
- `snapshot-check-rules.mjs`: parameters accept the optional array; refuse it with `kind === 'provider-explicit'`;
  coverage needs both kinds; ends check unions both maps; evidence adds the array only when present.
- Docs: `commands/check-snapshots.md` (groups, relationship-missing table + coverage paragraph, strict-validation
  bullet), CHANGELOG 0.7.0, manifest version + floor, `skills/what-this-agent-does.md` if it lists group kinds.

## Tests (node:test, touched files only)

Clash (`snapshot-clash.test.mjs` / `snapshot-clash-single.test.mjs`), two stacked plates + assembly:
hosts → `sameAssembly:true`; provider-explicit `tekla-assembly-member` with the alias → `true`; provider-explicit
with another providerRelationKind → `false`; no edges → `false`; same four for `sameComponent` via
`tekla-component-child`; a provider-explicit row WITHOUT the alias in the profile → `false` (old behaviour kept);
weld guard: unknown-part weld member of the assembly via the alias → `weld-parts-unknown`.
Validation: `[]`, duplicate, non-string, empty, >256 chars, >32 entries, unknown key → `profile-invalid`.
Rules (`snapshot-check-rules.test.mjs`): relationship-missing with aliases passes on an alias row, finds on another
providerRelationKind, not-evaluated when coverage lacks `provider-explicit`, `relationship-endpoint-unusable` on a
lost alias end, `kind: provider-explicit` + aliases refused, evidence carries the array only when set.
Goldens (`snapshot-clash.golden.test.mjs`, check goldens) unchanged apart from the evaluator version echo.

## Real E2E

Throwaway `AWARE_HOME`, the branch's CLI build (not the machine CLI): install model-change-set from the worktree,
re-run the #882 repro in single mode (hosts / provider-explicit / none, plus provider-explicit with the alias
profile) and assert `sameAssembly` in the `clash.unjoined-contact` evidence; plus one relationship-missing run.

## floless.app follow-up (separate PR, after the AWARE release)

- `change-checks-profile.ts`: when the installed model-change-set ≥ 0.7.0, groups become
  `{ kind:'hosts', name:'same assembly', providerRelationKinds:['tekla-assembly-member'] }` and
  `{ kind:'contains', name:'same component', providerRelationKinds:['tekla-component-child'] }`; below, today's shape.
- `quality-profile.ts` / compile of `relationship-missing`: the Tekla source maps `hosts`→`tekla-assembly-member`,
  `contains`→`tekla-component-child` as aliases (gated the same way), and `tekla-read-coverage.ts` adds
  `provider-explicit` to `complete` when the hierarchy is clean. Below 0.7.0 the false "not in an assembly" stays
  — say so (label), never send the new field to an old evaluator.
