# `model-change-set.check-snapshots` — what the change touched, checked

`compare-snapshots` says which objects were added, removed and changed. `check-snapshots` takes the next
step: it runs a pinned set of **checks** over the objects that change touched (and any the caller names) and
says which findings the change **introduced**, which were **already there**, and which it **resolved**,
as `aware.model-change-findings/v1`.

It is **pure and format-blind**, like `compare-snapshots`. It opens no source file, calls no provider and
needs no host or authorization. Two profiles carry every format-specific decision: the **comparison
profile** (the same file `compare-snapshots` takes: identity, volatile keys, key rules) and a **check
profile** (which rules to run, on which types, at which severity). AWARE applies them.

The pairing is not re-implemented: this command imports the pairing and the per-object diff from
`compare-snapshots`, so an object is "added" or "changed" here exactly when `compare-snapshots` says so. Its
scope uses the full pairing, never the capped change rows.

## Lifecycle

`single`. Read-only. Deterministic: the same inputs give **byte-identical canonical JSON**.

## Inputs

| Input | Required | Meaning |
|---|---|---|
| `base-dir` | yes | Directory with the earlier state's `entities.json`, `properties.json`, `relationships.json` |
| `revised-dir` | yes | The same three files for the later state |
| `profile-path` | yes | The comparison profile, `aware.model-compare-profile/v1` (see `compare-snapshots`) |
| `check-profile-path` | yes | The check profile, `aware.model-check-profile/v1` or `/v2` (below) |
| `coverage-base-path` / `coverage-revised-path` | no | A read-coverage document per side, `aware.model-read-coverage/v1` (see Single-object rules). Read only by a v2 profile |
| `validate-only` | no | `true`: validate `check-profile-path` (and `profile-path`, when given) and print the evaluator echo; no snapshot is read, and `base-dir`, `revised-dir` and `profile-path` are not needed |
| `focus-path` | no | `{ "identities": [ "<namespaced identity>", ... ], "sets": [ [a, b], ... ] }` (`sets` only with clash rules): objects to re-check even if unchanged. Identities are written exactly as `compare-snapshots` emits them in `changes[].identity`. Duplicates are rejected |

Documents are validated as in `compare-snapshots`; a bad request exits non-zero with a message on stderr
and nothing on stdout.

## The check profile (`aware.model-check-profile/v1`)

Every key below is required; an unknown key anywhere is an error.

```json
{
  "schema": "aware.model-check-profile/v1",
  "id": "floless.change-checks",
  "version": "1.0.0",
  "budgets": { "findings": 2000, "focusResults": 20000 },
  "rules": [
    {
      "id": "integrity.part-name-blank",
      "version": "1.0.0",
      "severity": "warning",
      "predicate": "blank",
      "target": "entity.name",
      "appliesTo": { "types": ["Beam", "Column", "Contour plate", "Polybeam", "Item"] },
      "worsening": null
    }
  ]
}
```

- `severity`: `warning`, `error` or `info`.
- `predicate`: `blank`, or a clash predicate (`clash-overlap`, `clash-contact`, `clash-bolted-plies`, see Clash below). An unknown predicate is rejected, not skipped.
- `target`: `entity.name` (for `blank`) or `solid` (for the clash predicates). An unknown target is rejected.
- `appliesTo.types`: a non-empty list of entity `type` strings, matched exactly. An object of another type is
  counted `notApplicable` for that rule.
- `worsening`: `null`, or `{ "path": "<evidence key>", "direction": "higher-is-worse" | "lower-is-worse" }`
  for a rule whose evidence carries a number. The `blank` predicate has no number, so it must be `null`.
- Rule ids are unique.

### `blank` on `entity.name`

Fires when the revised entity's `name` is missing, `null`, not a string, or empty after NFKC normalisation and
trim. Evidence is `{ "name": <the raw value, or null when missing> }`; locus is `{}`; there is one
participant.

## Scope and presence

- **In scope:** every revised object that is added or changed against the base, plus every focus identity
  present in the revised state. Everything else is **not checked**, and `unchecked` says so.
- **Excluded:** objects of the revised state whose identity is blank, used twice in the revised state, or used twice in the base state (so they cannot be matched) are never evaluated; the total is counted in
  `scope.excluded`.
- **Presence**, for each finding:
  - `introduced`: no finding on the base object, or the object is new;
  - `pre-existing`: the base object had the same finding, with the same `evidenceFingerprint`;
  - `pre-existing-changed`: the base object had the finding but the evidence differs;
  - `pre-existing-worsened` / `pre-existing-improved`: only when the rule declares `worsening` and the number
    moved that way (equal is `pre-existing-changed`).
- **`resolvedInChange`:** a base finding with no finding for the same `correspondenceKey` now, listed only when
  the rule still applies to the revised object and the object passes it. A rule that stops applying (the
  object's type changed) resolves nothing: the object was not shown to be fixed. Rows carry `locus` (the base
  finding's), so the key can be recomputed from the row.

## Keys

```
correspondenceKey   = sha256( canonicalJson({ ruleId, participants: [sorted identities], locus }) )
evidenceFingerprint = sha256( canonicalJson({ correspondenceKey, ruleVersion, evidence }) )
```

`canonicalJson` is the same sorted-key JSON `compare-snapshots` uses; the hash is over its UTF-8 bytes, as
lowercase hex. The `correspondenceKey` says *which* finding it is; the `evidenceFingerprint` says *what it
says*, so a finding that is still there but now different is told apart from one that is unchanged.

## Focus results

One row per focus identity and rule, sorted by `(identity, ruleId)`:

| State | Meaning |
|---|---|
| `finding` | present in the revised state, the rule applies, a finding was produced |
| `passed` | present, the rule applies, evaluated, no finding |
| `not-applicable` | present, but its type is not in the rule's `appliesTo.types` |
| `excluded` | its identity is used twice in the revised state or in the base state (so it cannot be matched), **or** it is not present while the revised state has any blank or duplicate identity (absence cannot be proven) |
| `absent` | not present, and the revised state has no blank or duplicate identity: a confirmed absence |
| `not-evaluated` | reserved |

## Output (`aware.model-change-findings/v1`)

```
schema            "aware.model-change-findings/v1"
inputs            { base: {entities, properties, relationships}, revised: {...}, profileDigest, checkProfileDigest, focusDigest }
checkProfile      { id, version }
scope             { added, changed, focus, evaluated, notApplicable, excluded, byRule: { <ruleId>: { evaluated, findings, notApplicable } } }
findings          [ { ruleId, ruleVersion, severity, correspondenceKey, participants: [ { identity, entityIdBase, entityIdRevised, type, name } ],
                      locus, evidence, evidenceFingerprint, presence, baseEvidenceFingerprint } ]      sorted by (ruleId, correspondenceKey)
resolvedInChange  [ { ruleId, ruleVersion, correspondenceKey, participants, locus, evidence, evidenceFingerprint } ]
focusResults      [ { identity, ruleId, state } ]
summary           { findings, introduced, preExisting, resolvedInChange, byRule }
truncated         null, or { omitted: { findings, resolvedInChange, focusResults } }
complete          true when nothing was omitted
unchecked         plain statements of what was not checked
```

- `inputs` digests are the sha256 of each document's file bytes, exactly as `compare-snapshots` computes
  them; `profileDigest` is the same as `compare-snapshots`; `checkProfileDigest` and `focusDigest` are the
  sha256 of the canonical JSON of the check profile and the focus file (`focusDigest` is `null` without one).
- `entityIdBase` is `null` for an added object. `name` is the revised entity's name as it is (possibly
  `null`).
- `scope.evaluated` and `scope.notApplicable` count (object, rule) pairs; `scope.focus` counts the focus
  identities present in the revised state (they may also be added or changed).
- `summary.preExisting` counts every `pre-existing*` presence.
- Counts are always complete. `findings` and `resolvedInChange` are capped at `budgets.findings` and
  `focusResults` at `budgets.focusResults`, in the sorted order; `truncated` and `complete` state the cap.

## Clash (0.3.0): `clash-overlap`, `clash-contact`, `clash-bolted-plies`

Three pair rules over the **solids** of the parts a change touched and their dependency closure. They
need two extra inputs and a `geometry` block in the check profile; a profile without a clash rule runs
exactly as before (same bytes) and ignores the geometry inputs.

### Extra inputs

| Input | Required | Meaning |
|---|---|---|
| `base-geometry-dir` / `revised-geometry-dir` | when the profile has a clash rule | A directory with `geometry.json` `{ "tiles": [ { "ordinal", "file", "sha256" } ], "qualification"?: { "file", "sha256" } }` and the GLB files it names (plain file names). Every digest is verified; a mismatch is refused (`geometry-digest-mismatch`). The optional `qualification` file is `{ "properties": [ rows ] }`, rows shaped like `properties.json` rows but possibly without a `sourceKey`; when present, coverage rows for qualification come only from it (matched by `propertyField` and `propertyKeyPattern`), otherwise from `properties.json`. These rows are never compared and never change a part's changed status. A bad shape is `geometry-invalid`. A clash profile without both directories is refused (`input-invalid`), never read as "no clash" |

`focus-path` gains optional `sets`: `{ "identities": [...], "sets": [ [a, b], ... ] }`, pairs of identities
(for `clash-bolted-plies` the bolt group and one ply) to re-check. Duplicate pairs are rejected.

### Rules (predicate, target `solid`)

| Predicate | Fires when | Evidence | Worsening path |
|---|---|---|---|
| `clash-overlap` | two solids share a region thicker than `contactTolerance` | `kind` (`overlap`, `contained`, `coincident`), `overlapThicknessMm`, `overlapExtentMm`, `locationMm`, `regions`, `overlapVolumeMm3`, `thicknessMethod: "across-part-faces"`, `approximate` | `overlapThicknessMm` or `overlapVolumeMm3` |
| `clash-contact` | two solids touch (overlap and gap within tolerance) and no bolt or weld joins them | `gapMm`, `locationMm`, one boolean per profile group (for example `sameAssembly`), `approximate` | none (`null`) |
| `clash-bolted-plies` | a ply a bolt group names touches no ply of the rest of its stack | `gapMm`, `locationMm`, `method: "whole-ply-nearest"`; beyond the search `{ gapMm: null, gapAtLeastMm, locationMm: null }` | `gapMm` |

One rule per predicate. All three are pair rules: a finding has exactly two participants (for
`clash-bolted-plies`, the bolt group then one detached ply) and an empty `locus`, so a clash that moves but
persists stays the same finding. Numbers are rounded half-even to 0.01 mm (volume to 1 mm3) before keys are
computed. A capped gap (`gapMm: null`) has no worsening metric: capped to capped with the same evidence is
`pre-existing`; exact to capped is `pre-existing-changed`; worsened or improved only exact to exact.

`overlapThicknessMm` is the thinnest dimension of the shared region measured across the face directions of
both parts (exact for prismatic overlaps, an upper bound for skewed ones); it is not the distance to move a
part clear. Each connected region is measured on its own, and the finding reports the thickest.

### The `geometry` block

```json
"geometry": {
  "node": { "entityIdPattern": "^element:([0-9]+)$", "nodeName": "object-$1" },
  "units": "mm", "contactTolerance": 0.1, "weldTolerance": 0.001,
  "qualification": { "propertyKeyPattern": "^floless:tekla:coverage:", "propertyField": "id", "json": "checks",
                     "checks": ["geometry-reconstruction"],
                     "whenPresent": ["operations-evaluated", "recipes-evaluated", "placement-confirmed"], "passStates": ["pass", "not-applicable"] },
  "joins": {
    "bolt": { "types": ["Bolt group"], "plies": { "property": "native:bolted-parts", "format": "identity-key-list" } },
    "weld": { "types": ["Physical"], "parts": { "property": "native:welded-parts", "format": "identity-key-list" },
              "unknownWhen": { "property": "native:welded-parts", "absent": true } }
  },
  "groups": [ { "kind": "hosts", "name": "same assembly" }, { "kind": "contains", "name": "same component" } ],
  "budgets": { "trianglesPerObject": 200000, "trianglePairsPerPair": 4000000, "gapSearchMm": 50, "pairs": 200000, "focusSets": 20000 }
}
```

- `node`: which GLB node holds an entity's mesh (regex over the entity id, template with `$1`..`$9`). The GLB
  is read as triangle lists of float32 positions in millimetres; node matrix or translation, rotation and
  scale are applied.
- `qualification`: the coverage property (a JSON string) is the row whose `propertyField` (required: `id` or
  `sourceKey`, the property-row field the `propertyKeyPattern` is matched against) matches. A row without a
  `sourceKey` is kept aside for this purpose only; it is never compared. The JSON's pinned checks decide whether a solid is
  trustworthy. `checks` are required: each must be present and in `passStates`, else the solid is
  `approximate` (`coverage:<check>:missing` when absent). `whenPresent` (required key, may be empty, names
  unique and not in `checks`) lists checks that apply only to some kinds of object (a part carries
  `operations-evaluated`, a bolt `placement-confirmed`, a weld `recipes-evaluated`): absent is fine, present
  must be in `passStates`. A missing coverage property also makes the solid `approximate`. Other checks are ignored.
- `joins`: properties naming, as the identity values the comparison profile reads (matched exactly, then
  case-insensitively), the parts a bolt group joins or a weld joins. A token that matches no part makes the
  join uncertain. A weld with the `unknownWhen` property absent joins unknown parts.
- `groups`: `hosts` / `contains` relationships, shown in contact evidence (never proof of a joint). A group may
  add `providerRelationKinds` (0.7.0, see "Provider relation kinds" below): then a `provider-explicit` row with a
  listed `providerRelationKind` counts too, for example
  `{ "kind": "hosts", "name": "same assembly", "providerRelationKinds": ["tekla-assembly-member"] }`. The `hosts`
  group is also the assembly a weld with unknown parts is looked for in (`weld-parts-unknown`), listed rows included.
- Strictly validated: unknown keys, an unknown predicate or target, a bad regex, a relationship kind outside
  `hosts` / `contains`, a bad `providerRelationKinds` list, or a missing `geometry` block with a clash rule are refused.

### Solid states and pair verdicts

Each solid is `evaluated`, `approximate` (coverage open, or the mesh is not a closed outward-facing solid:
`mesh-open`, `mesh-orientation`, `mesh-degenerate`), `no-geometry` (no node; the coverage code is the
reason) or `over-budget` (more triangles than `budgets.trianglesPerObject`; reason `triangle-budget`). A clean result on an approximate solid is `not-evaluated`, never `passed`; a pair
with a mesh that is not closed cannot be intersected and is `not-evaluated` (`mesh-not-closed`). A bolt or
weld against a part it names is `not-evaluated` for interpenetration (`own-joint`). Contact inside an
assembly holding a weld with unknown parts is `not-evaluated` (`weld-parts-unknown`).

### Report threshold, type pairs and budgets (0.6.0)

- `geometry.minOverlap` (optional; > 0 and >= `contactTolerance`; default `contactTolerance`) is a **report threshold only**.
  Whether two parts interpenetrate, touch or stand apart is still decided at `contactTolerance`. An overlap no thicker than
  `minOverlap` (compared unrounded) is measured, not listed, and never turned into a contact finding. It passes the rule as
  the profile sets it: an earlier finding that shrinks below the threshold is in `resolvedInChange`, and a pair re-check is
  `passed` with a `reason` saying how much the parts still overlap. When `minOverlap` is
  given, the overlap rule's `scope.byRule.<rule>.belowThreshold` is `{ count, maxThicknessMm }` (`maxThicknessMm` null at 0)
  and `unchecked` says how many were left out and the thickest of them.
- `appliesTo.pairs` on a `clash-overlap` or `clash-contact` rule: a non-empty list of unordered type pairs
  (`[["Beam", "Contour plate"], ...]`, each type in `appliesTo.types`) that rule checks. Pairs are normalised (each sorted,
  duplicates dropped) and echoed in `evaluator.geometry.pairs`. A nearby pair of another type pair is not judged by that rule:
  counted as `notEvaluated["pair-excluded"]` (v2) or `pairExcluded` (v1), with an `unchecked` line naming the left-out type
  pairs. It is applied before any budget (a pair no rule checks never uses the pair budget) and identically in pair re-checks
  (`not-applicable`). `clash-bolted-plies` takes no `pairs`.
- The candidate pairs are bounded: the first `budgets.pairs` by identity are measured, with memory bounded by the budget,
  whatever order the sweep meets them in. Distinct reasons: `pair-budget` (a candidate beyond the pair budget),
  `triangle-budget` (a solid or a pair over its triangle limit) and list truncation (`truncated`, `complete: false`).
- A v2 document with a clash rule echoes `evaluator.geometry`:
  `{ contactTolerance, minOverlap, weldTolerance, budgets, pairs: { <overlap/contact ruleId>: [[a, b], ...] | null } }`.

The shared region is A intersected with B (manifold-3d, Apache-2.0, WASM, single-threaded); contact and gap
use exact triangle-to-triangle distance with closest points, searched out to `gapSearchMm`.

### Scope, closure and extra output

Scope adds, for every added, removed or changed bolt group or weld, the union of its earlier and later
plies/parts, and for every in-scope part the bolt groups and welds naming it with all their plies/parts.
Candidate pairs come from the later state and the earlier state (a pair that overlapped only before is
evaluated and, if now clean, listed in `resolvedInChange`). Pairs of two unchanged parts are not checked.

When a clash rule ran the output also carries:

- `inputs.geometry`: `{ base: { qualification: { file, sha256 } | null, tiles: [ { file, ordinal, sha256 } ] }, revised: {...} }`;
- `scope.notEvaluated`: `{ pairs, stacks, byReason }` (evaluation limits and unevaluable pairs; these are not
  omitted list rows and never touch `truncated` or `complete`) and `scope.solids`
  `{ total, evaluated, approximate, noGeometry, overBudget, byReason }` for the solids in scope;
  `scope.byRule[<clash rule>]` also has `notEvaluated`;
- `focusSets`: `[ { identities: [a, b] (sorted), ruleId, state, reason? } ]`, one row per requested pair and
  clash rule, `state` one of `finding`, `passed`, `not-evaluated` (with `reason`), `absent`, `excluded`, or
  `not-applicable` when the rule does not cover that pair. A bolted pair is `passed` only when the bolt
  group's whole contact graph is connected;
- `truncated.omitted.focusSets`, capped by `geometry.budgets.focusSets`;
- `findings[].presence` may also be **`unknown`** (clash findings only): the earlier state had the pair (or
  stack) but could not judge it - no solid, an open mesh, over budget, approximate and clean, or the rule not
  applying to both parts there - so "introduced" or "pre-existing" would be unproven. `baseEvidenceFingerprint`
  is `null`. `summary.unknownPresence` counts them, and `introduced + preExisting + unknownPresence` equals
  `findings`. The key exists only when a clash rule ran;
- with no `clash-overlap` rule, a pair that overlaps is counted as not evaluated for contact (reason
  `interpenetrates`), never dropped;
- `unchecked` statements for what was not checked, including that bearing at each bolt was not checked
  (the snapshot holds no per-bolt position, so a stack is checked between whole plies).

Refusals add: `input-invalid` (clash profile without geometry directories), `geometry-digest-mismatch`,
`geometry-invalid` (a tile that is not a supported GLB), `profile-invalid` for any bad `geometry` key.

## Single-object rules (0.4.0): `aware.model-check-profile/v2`

A v2 profile is a v1 profile with `"schema": "aware.model-check-profile/v2"`, an optional top-level
`"scope": "changed" | "all"` (default `changed`), and rules that may carry `parameters`. v1 stays accepted, is
**not** extended (a v1 profile that names a v2 predicate, target, `parameters` or `scope` is rejected), and
produces the v1 findings document exactly as before. Everything in a rule is strictly validated: an unknown or
missing key in `parameters` is rejected.

| Predicate | Target | `parameters` | Finding when |
|---|---|---|---|
| `property-missing` | `property` | `{ key }` | no row with that canonical key, or its value is blank (NFKC + trim) |
| `property-not-in` | `property` | `{ key, values: [string], match: "exact" \| "nfkc-casefold" }` (1 to 5000 values, each 1 to 64 characters) | the value is present and is not one of `values` |
| `property-out-of-range` | `property` | `{ key, unit, min?, max? }` (at least one bound, `min <= max`) | the value is a number in `unit` and is below `min` or above `max`; the metric `distanceOutside` is the rule's worsening metric (higher is worse) |
| `relationship-missing` | `relationship` | `{ kind, end: "to" \| "from", providerRelationKinds? }` | the object is not the `to` (or `from`) end of any relationship row of that kind (or, with `providerRelationKinds`, of a listed `provider-explicit` row) |

Only rows the comparison profile keeps are visible to a rule: a row whose id starts with one of the profile's `ignoreRowIdPrefixes`, and a row with no `sourceKey`, are not properties for this purpose, so a coverage document must not claim a key that only such rows carry.

`match: "exact"` compares the value as it is. `match: "nfkc-casefold"` compares both sides after NFKC, lower-casing, ß and ẞ to `ss`, and the final sigma to the plain one (a dotless ı stays distinct from i); a value that is not text is compared by its JSON text.

`key` is the **canonical** property key: the key after the comparison profile's `keyRules`, formed by the same
function `compare-snapshots` uses. It is one exact key (no `*`), and it may not be the comparison profile's
identity property (rejected as `profile-invalid`). `worsening` must be `null` (for `property-out-of-range`
`null` or `{ "path": "distanceOutside", "direction": "higher-is-worse" }`).

Evidence: `property-missing` `{ key, present, value }`; `property-not-in` `{ key, value, match }`;
`property-out-of-range` `{ key, value, unit, min, max, distanceOutside }`; `relationship-missing`
`{ kind, end }`, plus `providerRelationKinds` (as given) only when the rule has it. Locus is `{}`: one finding per
object and rule.

#### Provider relation kinds (0.7.0, aware #882)

A snapshot may write a relationship as `kind: "provider-explicit"` with a `providerRelationKind` (the canonical v2
shape) instead of `hosts` or `contains`. The check profile, not AWARE, says what such a kind means: a
`relationship-missing` rule and a clash `geometry.groups[]` entry take an optional `providerRelationKinds`, and a
`provider-explicit` row whose `providerRelationKind` is listed there counts as a row of the rule's or group's `kind`.
For example `{ "kind": "hosts", "end": "to", "providerRelationKinds": ["tekla-assembly-member"] }` is "each must be
in an assembly" on a snapshot that writes assembly membership either way.

- 1 to 32 unique non-empty strings of at most 256 characters, matched exactly; `[]` is refused (leave the key out).
  A `relationship-missing` rule whose `kind` is `provider-explicit` cannot list them (`provider-explicit` with no
  list still means every `provider-explicit` row, as before).
- Only a row whose `kind` is exactly `provider-explicit` and whose `providerRelationKind` is a string can match.
- A `relationship-missing` rule with the list reads two families of rows. A usable row of either family at the
  object's `end` passes it, whatever the coverage says (a row that is there proves the relationship). To call it
  missing, the side's coverage must list both its `kind` and `provider-explicit` in `relationships.complete`
  (`provider-explicit` there means **every** `provider-explicit` row was read, not only the listed kinds), or the
  object is `source-did-not-read` (`coverage-not-supplied` when the side has no coverage document); then a
  discarded row of either family that ended at the object makes it `relationship-endpoint-unusable`. A rule without the list keeps the 0.6.x order (coverage first).
- A clash profile that lists `providerRelationKinds` on any group may have only one `hosts` group, because that
  group is where a weld with unknown parts is looked for. A profile without the key keeps the first-`hosts`-group rule.
- `compare-snapshots` still compares relationships by `kind`, `from` and `to`: a `provider-explicit` row whose
  `providerRelationKind` alone changed is not a change, so in pair mode it does not put its objects in scope.
  `relationship-missing` judges every applicable object anyway (see Scope); clash evidence for an unchanged pair is
  not re-checked for that edit.
- Without the key, validation, verdicts and every output byte are as in 0.6.x apart from `evaluator.version`. An
  evaluator older than 0.7.0 refuses the key as unknown (`profile-invalid`), never ignores it, so a caller gates on
  `evaluator.version`. That is the version the connection-reader bridge reports (`validate-only` echoes it), not the
  installed agent's: a stale bridge left in `<AWARE_HOME>/bridges` after a CLI upgrade still answers as the older
  evaluator, so gate on the echo, and refresh it with `aware sidecar install connection-reader`.

### Scope

`scope: "all"` judges **every** applicable object of the later state (and its counterpart in the earlier state)
for the single-object rules, not only added, changed and focus objects, so identical states with an old
violation report it as `pre-existing`. `scope: "changed"` (and every v1 profile) keeps today's scope; under it a v2 profile's `relationship-missing` rules still judge **every** applicable object of the later state, because whether an object has a relationship is a fact about the whole object and not about a diff (a deleted host changes no beam, but it can leave one unhosted, even when the host has no usable identity). Each single-object rule says which it did in `scope.byRule.<rule>.evaluatedScope`: `all` or `changed`. An old violation comes out as `pre-existing`, never `introduced`. Clash rules
keep their own scope either way. `scope.mode` and `scope.objects` in the output say which was used.

### Read coverage (`aware.model-read-coverage/v1`), one document per side

```json
{ "schema": "aware.model-read-coverage/v1",
  "entities": { "element:58109": { "properties": ["native:profile", "uda:*"] } },
  "relationships": { "complete": ["hosts", "contains"] } }
```

Strict: unknown keys are rejected (`coverage-invalid`). `entities` is keyed by the entity id as it appears in the
side's `entities.json`; `properties` lists the property keys read completely, each an exact canonical key or a
family `<prefix>:*` ending at a colon boundary (`uda:*` covers `uda:text:name:fire_rating`; a bare `*` is
rejected). Coverage is matched against the canonical key. An entity absent from `entities` has nothing read. A
relationship kind not in `complete` is not complete for that side. `coverageSha256` is the sha256 of the exact
file bytes.

A rule is judged for an object only when the side's coverage **affirmatively** covers what it reads (the key
on that entity; the relationship kind for the side). One exception: a `relationship-missing` rule with
`providerRelationKinds` passes an object on a usable matching row whatever the coverage says (see "Provider
relation kinds"); only calling it missing needs the coverage. Otherwise the object is not evaluated: `coverage-not-supplied`
when the side has no document, `source-did-not-read` when it has one that does not cover it.

### Not evaluated is not passed

Per rule the output counts objects it could not judge, by reason, in `scope.byRule.<rule>.notEvaluated`. None is a
finding and none is a pass:

| Reason | Meaning |
|---|---|
| `coverage-not-supplied` / `source-did-not-read` | see above |
| `value-missing` | `property-not-in` / `property-out-of-range`: no row, or a blank value (`property-missing` is that case's rule) |
| `value-not-numeric` | `property-out-of-range`: the value is not a finite number |
| `unit-mismatch` | `property-out-of-range`: the row's unit differs from the rule's `unit` (no conversion) |
| `key-ambiguous` | two rows of the object share the canonical key; neither is guessed |
| `identity-unusable` | the object was excluded for a blank or duplicate identity (counted for every rule that applies to its type) |
| `key-is-identity` | a property rule whose key is the canonical key of a real identity row of either state (identity rows are never kept as properties): this rule names the property used as the object's identity, so it cannot be checked reliably. It reports no finding, is never complete, and every other rule runs unchanged |
| `relationship-endpoint-unusable` | `relationship-missing`: a row of that kind (or a listed `provider-explicit` row) ended at the object but was discarded because its other end has no usable identity |

`relationship-missing` is judged on the side's **source** relationship rows, before identity normalisation.

### Side-specific results and `unknown` presence

The later state alone decides finding, passed or not evaluated. The earlier state is judged only for objects
that have a counterpart (and whose earlier type the rule covers), and only to decide presence: when it cannot
be judged, a finding's presence is `unknown` (never `introduced`), and a finding of the earlier state with no
judged later state is never `resolvedInChange`. Per rule `coverage` is `{ revisedComplete, baseComplete }`:
`revisedComplete` is true only when every object the rule applied to was judged on the later side;
`baseComplete` is true only when every counterpart that had to be judged on the earlier side was.

### Output (`aware.model-change-findings/v2`)

The v1 document plus: `evaluator` `{ agent, version, profileSha256 }` (`profileSha256` is the sha256 of the
canonical check profile that was given; it equals `inputs.checkProfileDigest`); `coverageSha256` `{ base, revised }`
(the digest of each coverage file, or `null`); `scope.mode`, `scope.objects`; per rule `notEvaluated`,
`coverage` and `evaluatedScope` in `scope.byRule` (a clash rule counts pairs: `notEvaluated` is `{ "pair-not-evaluated": n }` and
`baseComplete` is `null`); `unchecked` lines for each reason; and `summary.unknown` (also written as
`summary.unknownPresence`) with `introduced + preExisting + unknown === findings`.

### Single mode (0.5.0, clash 0.6.0): one model version, no earlier state

`mode: "single"` checks ONE saved state on its own, for a standing check of an approved model against the firm's
standards. There is nothing to compare, so it never says what changed or whether a finding is new.

```
{ "mode": "single", "revised-dir": "...", "profile-path": "...", "check-profile-path": "...",
  "coverage-revised-path": "...",       (coverage is optional)
  "revised-geometry-dir": "...",        (required with a clash rule, refused without one)
  "focus-path": "..." }                  (optional, clash only: { identities })
```

Accepted profile: `aware.model-check-profile/v2` with `scope: "all"`: one-object rules (`blank`, `property-missing`,
`property-not-in`, `property-out-of-range`, `relationship-missing`) and, from 0.6.0, clash rules. Refused, with a message on
stderr, a non-zero exit and nothing on stdout: `base-dir`, `coverage-base-path`, `base-geometry-dir`, a clash rule without
`revised-geometry-dir`, `revised-geometry-dir` or `focus-path` without a clash rule, a focus with `sets`, a v1 profile and a
v2 profile whose scope is not `all`. `validate-only: true` with `mode: "single"` applies the same profile rules.

Clash on one state: the earlier side is empty, so every clash finding is `present` and the verdicts equal pair mode's for the
same state against an empty earlier state. A focus narrows clash to pairs that involve the named objects, their dependency
closure (bolt and weld anchors) or anything touching them; `scope.focus` is then `{ requested, effective, unknown }`
(`effective` counts the closure; `unknown` lists requested identities that are not in the snapshot or have no solid, which
are also counted in `scope.solids.noGeometry`, reason `not-in-snapshot` for an identity the snapshot does not have) and
`unchecked` says only the focused objects were checked. One-object rules always run over the whole version.

The output is `aware.model-change-findings/v2` plus `mode: "single"`. `inputs.base` and `coverageSha256.base` are `null`;
`scope.added`, `scope.changed` and `scope.focus` are `0`; `scope.mode` is `all`; every finding has
`presence: "present"`, `baseEvidenceFingerprint: null` and `entityIdBase: null` on its participants; `resolvedInChange`
and `focusResults` are empty; `summary` is `{ findings, present, byRule }` (no `introduced`, `preExisting` or `unknown`) and
`present === findings`; per-rule `coverage.baseComplete` is `null`. Budgets and `truncated` work as in pair mode: counts are
complete, and `summary.findings` equals the listed findings plus `truncated.omitted.findings`. `unchecked` says that only this
version was checked and that nothing is said about what changed (with no "unchanged parts" sentence), and, without a clash
rule, that clash was not run. Without a focus `scope.focus` is `0`. An object whose identity is blank
or used twice in the version is excluded and counted, exactly as in pair mode.

A one-object rule gives the same verdict and the same `notEvaluated` reason as pair mode gives for the later state of that
object: both modes call one judging function.

### `validate-only`

`validate-only: true` with `check-profile-path` validates the profile and prints
`{ schema: "aware.model-check-validation/v1", valid, evaluator, checkProfile, scope, rules, comparisonProfileChecked, supports }`
without reading any snapshot, coverage or geometry. `supports` lists the profile and findings schemas,
the coverage schema, the predicates and the not-evaluated reasons this build knows. An invalid profile exits
non-zero with the message on stderr and nothing on stdout.

## Errors

Unknown or missing keys in either profile, an unknown predicate or target, a `worsening` block on `blank`, a
bad focus file, a malformed read-coverage document (`coverage-invalid`), or a non-facade document: the command exits non-zero with the reason on stderr, before
anything is written to stdout.
