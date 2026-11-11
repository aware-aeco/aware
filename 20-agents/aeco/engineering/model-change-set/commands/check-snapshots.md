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
| `focus-path` | no | `{ "identities": [ "<namespaced identity>", ... ] }`: objects to re-check even if unchanged. Identities are written exactly as `compare-snapshots` emits them in `changes[].identity`. Duplicates are rejected |

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
- `predicate`: currently only `blank`. An unknown predicate is rejected, not skipped.
- `target`: currently only `entity.name`. An unknown target is rejected.
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
- **Excluded:** objects whose identity is blank or duplicated are never evaluated; they are counted in
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
| `excluded` | its identity is blank or duplicated in the revised state, **or** it is not present while the revised state has any blank or duplicate identity (absence cannot be proven) |
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

## Errors

Unknown or missing keys in either profile, an unknown predicate or target, a `worsening` block on `blank`, a
bad focus file, or a non-facade document: the command exits non-zero with the reason on stderr, before
anything is written to stdout.
