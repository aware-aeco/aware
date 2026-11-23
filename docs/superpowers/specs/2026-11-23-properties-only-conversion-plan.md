# Plan: a complete conversion that declares every object but draws none publishes as a properties-only model (#869)
_Round 2 — approved by Codex; two contract details tightened_

## Goal

Refine the #604/#607 zero-tile gate in `model-reference-reader` protocol v3. Today a **complete**
conversion with zero geometry tiles is refused at `canonical-artifact` with
`reference-geometry-invalid` "A complete model conversion requires at least one geometry tile.",
even when the provider read every object and can say, for each part it could not draw, why. The
person then gets nothing. After this change:

- A conversion with zero tiles whose provider **declares the geometry state of every entity** (each
  undrawn entity with a reason, every other entity as having no geometry of its own) publishes as a
  **properties-only model**. The signed root and the reader result carry a label
  `nothing-drawable-yet` with the count of undrawn entities and the provider's reason codes with
  their counts.
- A complete conversion with zero tiles that **declares nothing** stays refused exactly as today
  (same code, same message), as #604 intended. An empty GLB tile stays inadmissible (#607).
- A degraded zero-tile conversion without declarations is unchanged (publishes, no label).

This is a publication/labelling rule only. No model reasons at run time; every check is
deterministic validation of provider-declared, receipted bytes.

## Approach

1. **Provider output: a new closed file kind `geometry-state`** (`model-provider-output.mjs`,
   `model-provider-output-v1.schema.json`). Path `metadata/geometry-state-NNNNNN.jsonl`,
   media type `application/x-ndjson`, at most 256 contiguous shards, receipted and verified exactly
   like the other JSONL kinds (canonical JSON records, LF framing, byte/record limits). No bounds.
   Records are a closed union keyed by `state`:
   - `{"entityId": E, "reason": R, "state": "not-drawn"}` — the entity has geometry the provider
     could not draw; `R` is an opaque provider reason code `^[A-Za-z0-9._-]{1,128}$`.
   - `{"entityId": E, "state": "no-geometry"}` — the entity has no geometry of its own (an
     assembly, a group, a component definition).
   Any other field or state is refused.

2. **Canonicalization** (`model-canonical-v2.mjs`). After the three metadata families (so the
   entity identity set is complete), read the `geometry-state` shards in ordinal order and:
   - refuse them when the conversion has any geometry tile (`reference-metadata-semantic-invalid`,
     "Geometry-state declarations are admitted only when a conversion has no geometry tiles.");
   - normalize each record (closed shape, bounded text, opaque reason);
   - require `entityId` to name an admitted entity that has not been declared yet. No second
     identity set is built: the existing entity identity set is no longer needed once the three
     families are sorted, so each declaration **deletes** its entity from it; a failed delete is a
     dangling or repeated declaration (`reference-metadata-semantic-invalid`, "A geometry-state
     record names an entity that is missing or already declared."). Memory stays at today's peak;
   - require every entity to be declared, i.e. the set is empty at the end
     (`reference-metadata-semantic-invalid`, "Every entity must declare its geometry state.");
   - require at least one `not-drawn` (`reference-metadata-semantic-invalid`, "Geometry-state
     declarations must name at least one undrawn entity.");
   - bound distinct reasons to 64 (`reference-artifact-v2-limit`);
   - reduce to `geometryState = { state: "nothing-drawable-yet", notDrawn, noGeometry,
     reasons: [{ reason, count }] }` with `reasons` sorted by byte order of `reason`.
   The zero-tile gate becomes: zero tiles is admitted when the effective source is degraded **or**
   the provider supplied declarations. Zero tiles, complete, no declaration shards → unchanged
   refusal (`reference-geometry-invalid`, same message). The order of the checks is: geometry tiles
   (unchanged) → zero-tile gate based on whether declaration shards are receipted → metadata families
   → declarations. So a complete conversion with no declarations still fails before any metadata
   work, as today.

3. **Signed root** (`model-artifact-v2.mjs`, `model-reference-manifest-v2.schema.json`).
   `buildArtifactV2Root` accepts an optional `geometryState` input and an exported
   `validateGeometryState(value, entityItemCount)` checks: closed shape; `state` is exactly
   `nothing-drawable-yet`; `notDrawn` ≥ 1 and `noGeometry` ≥ 0 safe integers whose sum equals the
   entities index `itemCount`; `reasons` dense, 1..64 entries, each closed `{reason, count}`, opaque
   reason, `count` a safe integer ≥ 1, strictly ascending by byte order, the running total a safe
   integer and the counts summing to `notDrawn`. The root
   refuses `geometryState` with a non-empty geometry index, and refuses an empty geometry index
   unless the source is degraded or `geometryState` is present ("Only a degraded artifact, or one
   that declares why nothing is drawn, may omit geometry."). The manifest gains `geometryState` only
   when supplied, so every existing root is byte-identical and keeps its digest and cache key.

4. **Cache** (`model-v3-cache.mjs`). A warm hit must not become a route around the gate.
   - Every root with an empty geometry index must have a canonical empty geometry index
     (`objects: []` **and** `itemCount: 0`); a non-degraded one must carry `geometryState`
     (`reference-cache-invalid`).
   - A root that carries `geometryState` (new; no such cache entry exists today) is **rebuilt** on
     every warm read with `buildArtifactV2Root` from its cached parts — the effective-source object
     bytes, the four cached index files and the manifest's identity fields and `geometryState` — and
     its bytes must equal the cached root (`reference-cache-invalid`). This puts the warm path behind
     exactly the cold-path root invariants: index canonicality (rebuilt from receipts), receipt
     binding to `manifest.indexes`, completeness derived from the effective source, empty geometry,
     and `geometryState` arithmetic against the rebuilt entities `itemCount`.
   - Roots without `geometryState` keep today's warm checks unchanged, so no existing cache entry
     can start failing (a full rebuild there would risk refusing entries written by an older builder,
     and the reader treats `reference-cache-invalid` as a hard error, not a miss).
   - Entity shard contents are not re-read on a warm hit, as for drawn roots today: the cache record
     is signed by AWARE's own model-reference key, and shard bytes are digest-checked.

5. **Reader result** (`model-reader.mjs`). The v3 result (`probe`, `read-model`, `read-snapshot`)
   gains `geometryState` copied from the signed root, only when present. `completeness` and
   `counts` are unchanged (`counts.geometry` is `0`). Results for every existing conversion are
   byte-identical.

6. **Contract docs and version negotiation.** `10-core/cli-spec.md` protocol-v3 paragraph: the
   properties-only exception, the declaration record, the label, and that "declares nothing stays
   refused". Negotiation uses mechanisms that already exist, and the spec says so: a provider that
   emits `geometry-state` declares the first AWARE release carrying it as its package
   `minimumAwareVersion`, which `verifyRuntimeCompatibility` already enforces before any launch
   (`reference-provider-package-incompatible`); a host gates on the AWARE CLI version it installs.
   No agent or bridge version is bumped (as in #607, the direct predecessor): the v3 result carries
   no packager identity, so a bump would negotiate nothing on v3 and would only change the legacy
   snapshot preimage's packager identity. Public text never names a vendor, a model, a client or a
   project.

7. **Tests** (`node --test` in `cli-connection-reader`), each mutation-checked by reverting the
   gate it pins and watching it go red:
   - canonical: complete + zero tiles + full declaration → publishes, root `completeness:
     "complete"`, `geometryState` exact, geometry index empty, `counts.geometry` 0;
   - canonical: complete + zero tiles + no declaration → unchanged refusal code and message;
   - canonical: declaration beside a tile, a dangling entity, a duplicate, an undeclared entity,
     all `no-geometry`, an unknown state/extra field, an invalid reason, 65 distinct reasons → each
     refused with its code;
   - canonical: degraded + zero tiles + declaration → publishes with the label; degraded + zero
     tiles without declaration → byte-identical to today's root;
   - artifact-v2: root with `geometryState` builds; inconsistent counts/order/sums refused; a complete
     empty root without it refused; `geometryState` beside geometry refused;
   - provider-output: the new kind admitted with its pattern and media type; bounds on it refused;
   - v3 cache: a signed complete root with a valid `geometryState` round-trips; one without it and
     one with an inconsistent `geometryState` are refused;
   - canonical determinism: the same declarations shuffled and split across two shards produce
     byte-identical root bytes;
   - artifact-v2 direct-builder inputs: a free-standing `geometryState` whose totals do not match
     the entities index, an unsorted/duplicate/zero-count reason, `notDrawn: 0`, an unknown state or
     field — each refused;
   - v3 cache with **coherently signed** malformed entries (signed with the test key, so the
     signature passes and the new checks are what refuse): a complete empty root without
     `geometryState`; a `geometryState` root whose geometry index claims `itemCount > 0`; one whose
     `geometryState` totals disagree with the entities index; one whose root bytes differ from the
     rebuild — each refused; a real canonical properties-only root round-trips;
   - reader: the v3 result carries `geometryState` only when the root does.

8. **Real CLI E2E** (scratch harness, not committed): build the CLI and the reader sidecar from the
   branch, a temp `AWARE_HOME`, a synthetic signed provider package (SEA launcher implementing
   describe/discover/convert) trusted, enrolled, selected and policy-admitted through
   `aware provider …`, the reader agent installed from the branch, then `aware app run` of a
   one-node `read-snapshot` app: (a) zero tiles + full declaration → run succeeds, published root
   and result carry the label; (b) zero tiles + no declaration → run fails with
   `reference-geometry-invalid` and the unchanged message; (c) warm rerun of (a) is a cache hit with
   the same root digest.

## Key decisions & tradeoffs

- **A new `geometry-state` kind rather than** (a) giving the existing, unspecified `diagnostics`
  kind semantics (any canonical JSONL is admitted and dropped today; giving it a schema could break
  a provider that uses it freely), (b) a new field on the canonical entity record (changes the
  published entities shard shape every consumer validates), or (c) provider-specific property
  records (AWARE is format-neutral and cannot read them).
- **Every admitted entity must be declared, not only the undrawn ones.** AWARE cannot know which
  entities have a geometry role, so "declares every object" is only checkable if each admitted
  entity is classified. The label's counts are therefore **provider-declared coverage of the
  entities the provider emitted**: AWARE guarantees every emitted entity is classified exactly once
  and that the counts are the declarations' exact tally; it does not and cannot guarantee that the
  provider emitted every source object or classified each one truthfully — the same trust it
  already places in every entity, property and relationship record of an enrolled, signed provider.
- **At least one `not-drawn` is required.** A conversion whose every entity declares no geometry
  is a positive claim that the model is empty; #604 keeps that refused for a complete conversion.
  This is the existing hard stop retained, not a new one.
- **Declarations are admitted only with zero tiles.** Partial-drawability labelling (some tiles plus
  undrawn entities) is a different feature; silently ignoring declarations beside tiles would drop
  provider data, so they are refused instead.
- **The root carries a summary, not the per-entity list.** The per-entity mapping can be millions of
  rows; the summary is bounded (≤ 64 reasons) and signed. Per-entity detail remains the provider's
  own business in its property records. Every canonical entity already shows `geometry: []`, so no
  entity is presented as drawn. This follows the existing division of labour: semantic checks over
  record contents (dangling properties, entity ownership, declarations) live in canonicalization,
  which reads the records; `buildArtifactV2Root` enforces structural invariants over receipts and the
  small effective-source bytes. The root-level expression of "declares nothing stays refused" is
  that a complete root with an empty geometry index must carry a `geometryState` that tallies with
  its entity count; the root is AWARE's signed attestation of what canonicalization verified.
- **Reason is an opaque code, not provider text.** Consistent with #608: AWARE never forwards
  provider-controlled prose; the host maps the code to copy.
- **`completeness` is unchanged** (it describes source coverage, not drawability), and the label is
  a separate field so a properties-only root can never be read as a drawn one.
- **No new error code for the "declares nothing" refusal**, so hosts that match today's code and
  message keep working (#609 is a separate concern).

## Risks / open questions

- Downstream verifiers that check the root and result with exact keys refuse `geometryState` until
  they are updated. That is intended: such a host refuses zero-tile complete reads today anyway, and
  the field is absent for every other conversion.
- A provider that emits the new kind against an older AWARE gets
  `reference-provider-output-invalid`; providers avoid that through `minimumAwareVersion`.
- Compatibility guarantee, stated narrowly: for every conversion whose provider emits no
  `geometry-state` shard, the v3 root bytes, root digest, cache key, cache warm checks and reader
  result are byte-identical to today and every validly produced cache entry still reads; legacy
  protocol-1/2 output is untouched (no version bumps). The one intentional hardening is that a
  cached empty geometry index must also say `itemCount: 0`; AWARE never wrote any other.

## Out of scope

- Partial-drawability labelling (tiles plus undrawn entities).
- Publishing per-entity reasons in AWARE's artifact.
- A distinct code for the zero-tile refusal (#609).
- The downstream host's provider and verifier changes (tracked downstream, pawellisowski/floless.app#4004).
- Any release (version tag).
