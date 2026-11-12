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
| `check-profile-path` | yes | The check profile, `aware.model-check-profile/v1` (below) |
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
| `base-geometry-dir` / `revised-geometry-dir` | when the profile has a clash rule | A directory with `geometry.json` `{ "tiles": [ { "ordinal", "file", "sha256" } ] }` and the GLB files it names (plain file names). Every digest is verified; a mismatch is refused. A clash profile without both directories is refused (`input-invalid`), never read as "no clash" |

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
  "qualification": { "propertyKeyPattern": "^floless:tekla:coverage:", "json": "checks",
                     "checks": ["geometry-reconstruction", "operations-evaluated"], "passStates": ["pass", "not-applicable"] },
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
- `qualification`: the coverage property (a JSON string) whose pinned `checks` decide whether a solid is
  trustworthy. A pinned check outside `passStates`, a missing one, or a missing property makes the solid
  `approximate`. Other checks are ignored.
- `joins`: properties naming, as the identity values the comparison profile reads (matched exactly, then
  case-insensitively), the parts a bolt group joins or a weld joins. A token that matches no part makes the
  join uncertain. A weld with the `unknownWhen` property absent joins unknown parts.
- `groups`: `hosts` / `contains` relationships, shown in contact evidence (never proof of a joint).
- Strictly validated: unknown keys, an unknown predicate or target, a bad regex, a relationship kind outside
  `hosts` / `contains`, or a missing `geometry` block with a clash rule are refused.

### Solid states and pair verdicts

Each solid is `evaluated`, `approximate` (coverage open, or the mesh is not a closed outward-facing solid:
`mesh-open`, `mesh-orientation`, `mesh-degenerate`), `no-geometry` (no node; the coverage code is the
reason) or `over-budget`. A clean result on an approximate solid is `not-evaluated`, never `passed`; a pair
with a mesh that is not closed cannot be intersected and is `not-evaluated` (`mesh-not-closed`). A bolt or
weld against a part it names is `not-evaluated` for interpenetration (`own-joint`). Contact inside an
assembly holding a weld with unknown parts is `not-evaluated` (`weld-parts-unknown`).

The shared region is A intersected with B (manifold-3d, Apache-2.0, WASM, single-threaded); contact and gap
use exact triangle-to-triangle distance with closest points, searched out to `gapSearchMm`.

### Scope, closure and extra output

Scope adds, for every added, removed or changed bolt group or weld, the union of its earlier and later
plies/parts, and for every in-scope part the bolt groups and welds naming it with all their plies/parts.
Candidate pairs come from the later state and the earlier state (a pair that overlapped only before is
evaluated and, if now clean, listed in `resolvedInChange`). Pairs of two unchanged parts are not checked.

When a clash rule ran the output also carries:

- `inputs.geometry`: `{ base: { tiles: [ { file, ordinal, sha256 } ] }, revised: {...} }`;
- `scope.notEvaluated`: `{ pairs, stacks, byReason }` (evaluation limits and unevaluable pairs; these are not
  omitted list rows and never touch `truncated` or `complete`) and `scope.solids`
  `{ total, evaluated, approximate, noGeometry, overBudget, byReason }` for the solids in scope;
  `scope.byRule[<clash rule>]` also has `notEvaluated`;
- `focusSets`: `[ { identities: [a, b] (sorted), ruleId, state, reason? } ]`, one row per requested pair and
  clash rule, `state` one of `finding`, `passed`, `not-evaluated` (with `reason`), `absent`, `excluded`, or
  `not-applicable` when the rule does not cover that pair. A bolted pair is `passed` only when the bolt
  group's whole contact graph is connected;
- `truncated.omitted.focusSets`, capped by `geometry.budgets.focusSets`;
- `unchecked` statements for what was not checked, including that bearing at each bolt was not checked
  (the snapshot holds no per-bolt position, so a stack is checked between whole plies).

Refusals add: `input-invalid` (clash profile without geometry directories), `geometry-digest-mismatch`,
`geometry-invalid` (a tile that is not a supported GLB), `profile-invalid` for any bad `geometry` key.

## Errors

Unknown or missing keys in either profile, an unknown predicate or target, a `worsening` block on `blank`, a
bad focus file, or a non-facade document: the command exits non-zero with the reason on stderr, before
anything is written to stdout.
