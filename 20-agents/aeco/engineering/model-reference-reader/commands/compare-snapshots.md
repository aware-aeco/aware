# `model-reference-reader.compare-snapshots` — two saved states of one model, and what changed between them

A model changes between saves. You already hold the earlier state; the question the next save raises is
not *what is in it* but *what moved*. `compare-snapshots` pairs the objects of two saved states by a
**native identity** and reports what was **added**, **removed** and **changed** as
`aware.model-change-set/v1`.

It is **pure and format-blind**. It opens no source file, calls no provider and needs no host or
authorization. The caller supplies each state's three canonical query documents and a **comparison
profile** (data) that carries every format-specific decision. AWARE applies the profile; it does not know
what a GUID, a UDA or a Tekla property is. (This keeps the `model-reference-reader` contract: *format
meaning remains inside an enrolled read-only provider*.)

## Lifecycle

`single`. Read-only. Deterministic: the same inputs give **byte-identical canonical JSON** (keys sorted,
rows in a pinned order).

## Inputs

| Input | Required | Meaning |
|---|---|---|
| `base-dir` | yes | Directory with the earlier state's `entities.json`, `properties.json`, `relationships.json` |
| `revised-dir` | yes | The same three files for the later state |
| `profile-path` | yes | The comparison profile, `aware.model-compare-profile/v1` (below). Unknown or missing keys are rejected |
| `witness-path` | no | `{ "entries": [ { "identity": "...", "action": "created" | "modified" | "deleted" } ] }` |

Each document must be exactly one top-level key: `{ "entities": [...] }`, `{ "properties": [...] }`,
`{ "relationships": [...] }`. Anything else is rejected before a result is written (non-zero exit, message
on stderr, nothing on stdout). The documents are read whole; for a 25,000-object state expect hundreds of
MB of parsed JSON in memory.

Row shapes the command reads (extra keys are ignored):

- entity: `{ id, type, name, geometry: [ { tileOrdinal, bounds: [minX,minY,minZ,maxX,maxY,maxZ] } ] }`
- property: `{ id, entityId, name, value, valueType, unit, sourceKey, ... }` (`sourceKey` is what identity
  and comparison keys are built from; rows without one are counted and skipped)
- relationship: `{ id, kind, from, to }` (`from`/`to` are entity ids)

## The profile (`aware.model-compare-profile/v1`)

Every key below is required; an unknown key anywhere is an error.

```json
{
  "schema": "aware.model-compare-profile/v1",
  "id": "floless.tekla-native",
  "version": "1.0.0",
  "identity": { "sourceKey": "native:guid", "namespace": "tekla-model-A" },
  "ignoreRowIdPrefixes": ["viewer:"],
  "volatile": ["native:id"],
  "keyRules": [
    { "match": "^uda:(text|number):(0|[1-9][0-9]*)$", "key": "uda:$1:name:{name|nfkc-lower}" }
  ],
  "kinds": {
    "native:profile": "shape-fields",
    "native:length": "shape-fields",
    "uda:*": "properties"
  },
  "geometry": { "field": "bounds", "toleranceMm": 1 },
  "discontinuityThreshold": 0.5,
  "budgets": { "changes": 5000, "groupIds": 200, "receiptsPerKind": 2000, "outputBytes": 33554432 },
  "uncheckedStatements": ["Component internals were not compared."]
}
```

| Field | Meaning |
|---|---|
| `id`, `version` | Non-empty strings, echoed in the output |
| `identity.sourceKey` | The `sourceKey` of the property row whose `value` is an object's identity |
| `identity.namespace` | Prefix (model lineage). An object's identity in the output is `<namespace>:<value>` |
| `ignoreRowIdPrefixes` | Property rows whose `id` starts with one of these are ignored (counted in `inputs.rows`) |
| `volatile` | Keys that never count as a change. Matched against a row's `sourceKey` and its comparison key |
| `keyRules` | Ordered. The first rule whose `match` regex matches the **whole** `sourceKey` forms the comparison key from `key`. No rule matches: the key is the `sourceKey` |
| `kinds` | Comparison key (or prefix ending in one `*`) to `shape-fields`, `placement` or `properties`. Exact beats prefix; longest prefix wins; unknown keys are `properties`. Entity-level fields `entity.type` and `entity.name` are classified the same way |
| `geometry` | `field` must be `bounds`; `toleranceMm` >= 0, in the units of the bounds |
| `discontinuityThreshold` | In (0, 1]. `identity.discontinuity` is true when `paired / base < threshold` |
| `budgets` | Non-negative integers (`outputBytes` >= 1024). See [Budgets](#budgets) |
| `uncheckedStatements` | Extra plain statements appended to `unchecked` |

**Key-rule templates** support `$1`..`$9` (captures that the `match` defines) and
`{name}` / `{groupName}` with an optional `|nfkc-lower` transform (Unicode NFKC, then lower-case). Any
other `{...}` is rejected. Example: a definition id that is renumbered between saves is replaced by the
definition **name**, so a re-save is not a change.

Because comparison keys are not row ids, two rows of one object can produce the **same** key (two
definitions that normalise to one name). Neither is compared: the object gets an `ambiguous-field`
receipt and the key is listed in `unchecked`. Distinct keys are never collapsed.

## Identity and pairing

An object's identity is the `value` of its property row with `sourceKey == identity.sourceKey`.

- no such row, or an empty / non-string value: **blank** receipt;
- two such rows on one object, or one value on two objects: **duplicate** receipt for *every* copy;
- an identity that the *other* state holds twice (or an object carrying two identity rows) cannot be paired
  and is **not** reported as added or removed (that would claim an absence); the unique side gets a
  `duplicate` receipt too.

Excluded objects are never paired. `identity.base` counts every base object, so
`base = paired + removed + excluded.blank.base.count + excluded.duplicate.base.count`.

Pairing is by identity **only**. `unpaired` groups removed and added objects by `(type, name)` so a
regenerated component reads as one group; that is a presentation, never a pairing.

## What counts as a change

For a paired object:

- `entity.type` / `entity.name`;
- **placement**: the union of the entity's tile bounds, each of the six numbers compared within
  `toleranceMm` (a move equal to the tolerance is not a change). Bounds are the only placement evidence in
  the documents, so a rotation or move that keeps the box within tolerance is **not detected**;
- every non-volatile, non-identity property, by comparison key, on `value`, `unit` and `valueType`;
- **relationships**, as `(kind, identity(from), identity(to))` triples, never numeric ids. A difference is
  attributed to both ends only when **both** ends are paired; a link to an added or removed object follows
  from that add/remove.

Each changed row's `kinds` is the subset of `shape-fields`, `placement`, `properties`, `relationships`
present among its fields. **Shape is limited to the fields the profile marks `shape-fields`**: holes, cuts,
fittings and solid geometry are not compared, and `unchecked` always says so.

## Output

```json
{
  "schema": "aware.model-change-set/v1",
  "inputs": {
    "base": { "entities": "<sha256>", "properties": "<sha256>", "relationships": "<sha256>" },
    "revised": { "entities": "...", "properties": "...", "relationships": "..." },
    "profileDigest": "<sha256 of the canonical profile>",
    "witnessDigest": "<sha256 of the witness file bytes> | null",
    "rows": { "base": { "entities": 3, "properties": 6, "relationships": 0, "ignoredRows": 0, "orphanProperties": 0, "unkeyedProperties": 0 }, "revised": { "...": 0 } }
  },
  "profile": { "id": "floless.tekla-native", "version": "1.0.0" },
  "identity": {
    "base": 3, "revised": 3, "paired": 2, "added": 1, "removed": 1, "pairedShareOfBase": 0.666667,
    "excluded": { "blank": { "base": { "count": 0, "entityIds": [], "truncated": 0 }, "revised": { "...": 0 } },
                  "duplicate": { "base": { "...": 0 }, "revised": { "...": 0 } } },
    "discontinuity": false, "discontinuityThreshold": 0.5
  },
  "changes": [
    { "status": "removed", "identity": "tekla-model-A:G3", "entityIdBase": "element:3", "entityIdRevised": null, "type": "BEAM", "name": "b" },
    { "status": "added", "identity": "tekla-model-A:G4", "entityIdBase": null, "entityIdRevised": "element:3", "type": "BEAM", "name": "b" },
    { "status": "changed", "identity": "tekla-model-A:G2", "entityIdBase": "element:2", "entityIdRevised": "element:2",
      "type": "BEAM", "name": "b", "kinds": ["shape-fields"], "truncatedFields": 0,
      "fields": [ { "key": "native:profile", "kind": "shape-fields", "op": "modified", "before": "HEA200", "after": "HEB300" } ] }
  ],
  "unpaired": { "groupCount": 1, "groups": [ { "type": "BEAM", "name": "b",
      "removed": { "count": 1, "entityIds": ["element:3"], "truncated": 0 },
      "added": { "count": 1, "entityIds": ["element:3"], "truncated": 0 } } ] },
  "ambiguousFields": { "count": 0, "items": [], "truncated": 0 },
  "summary": { "added": 1, "removed": 1, "changed": 1, "unchanged": 1,
               "byKind": { "shape-fields": 1, "placement": 0, "properties": 0, "relationships": 0 },
               "relationships": { "base": 0, "revised": 0, "added": 0, "removed": 0, "onUnpaired": 0 } },
  "truncated": { "omitted": { "added": 0, "removed": 0, "changed": 0 }, "groups": 0, "receipts": 0 },
  "unchecked": [ "Placement was compared on each object's bounding box only, within 1 mm. ...", "Holes, cuts, fittings and solid geometry were not compared; ..." ],
  "witness": null
}
```

- A field's `op` is `added`, `removed` or `modified`; `before`/`after` are the property values (`null` when
  absent). Placement fields use key `geometry.bounds` with the six-number union as `before`/`after`.
  Relationship fields use key `relationship:<kind>:out:<identity>` / `relationship:<kind>:in:<identity>`.
  `unitBefore`/`unitAfter` and `typeBefore`/`typeAfter` appear only when those differ.
- Fields are sorted by key and capped at 50 per object; `truncatedFields` counts the rest. `summary.byKind`
  counts objects, from *all* fields, before that cap.
- `inputs` echoes the digests of the exact bytes hashed. The caller, which holds the signed receipts,
  decides whether they match.

### Reconciliation

For the emitted statuses, `summary[status] = rows emitted with that status + truncated.omitted[status]`.
`unchanged` is never emitted: `summary.unchanged = identity.paired - summary.changed`, and
`identity.added/removed = summary.added/removed`.

### Budgets

Counts are computed before any cap, so they are always complete. Rows are emitted in the pinned order
**removed, then added, then changed, each sorted by identity**, so which rows are omitted is
deterministic. `budgets.changes` caps rows, `groupIds` caps entity ids per group and side,
`receiptsPerKind` caps excluded / ambiguous / witness receipt lists, and `outputBytes` caps the whole
document (rows are shed from the end of the pinned order, then groups). Anything omitted is stated in
`truncated` and in `unchecked` ("Partial list: ..."). A budget too small to hold even the counts is an
error (`budget-too-small`).

### Discontinuity

`identity.discontinuity` is a first-class outcome, not an error. At 27 of 3,095 base objects paired, the
two states probably do not share a lineage and the added/removed lists are a regeneration, not a change
list. The unpaired groups are the useful view; `unchecked` says so.

### Witness

A source's own change log may be passed as a witness. Per identity, `created` agrees with `added`,
`modified` with `changed`, `deleted` with `removed`. A bare value (without the `identity.namespace:`
prefix) is accepted. The report is `witness: { digest, entries, agrees, witnessOnly, diffOnly }`, each a
`{ count, items, truncated }`: `witnessOnly` is a log claim the diff did not see (for example the log says
modified and the objects compare equal); `diffOnly` is a diff change no log entry agrees with. **The
witness never changes the change set.**

## Failure modes

Errors print one line on stderr, nothing on stdout, and exit non-zero:

- missing or non-string `base-dir` / `revised-dir` / `profile-path`;
- a missing file, invalid JSON, or a document that is not exactly one top-level array key;
- a profile with an unknown or missing key, an invalid kind, key rule, template or budget;
- duplicate entity ids within one document, or geometry bounds that are not six finite numbers;
- a malformed witness.

An old `aware-connection-reader` that predates this command fails it (it is routed to the IFC reader, which rejects the request); a caller
should treat that as "reader too old" and offer to update AWARE.
