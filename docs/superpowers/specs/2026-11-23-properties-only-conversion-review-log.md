# Plan Review Log: properties-only conversion (#869)
Started 2026-11-23. MAX_ROUNDS=5. Reviewer: Codex (read-only). Plan: `2026-11-23-properties-only-conversion-plan.md`.

## Round 1 — Codex

The plan still has material gaps at the root-builder and cache boundaries.

1. **Full declarations do not prevent under-reporting.** The plan says this “catches a provider that silently drops a family of objects.” A provider can omit that family from both entities and declarations, or classify drawable objects as `no-geometry`. One `not-drawn` record still admits the conversion. Coverage is established only over provider-emitted entities.
   
   **Fix:** Describe the counts as provider-declared coverage of admitted entities; remove the guarantee against omitted source objects and false classifications.

2. **The root-builder exception accepts invented evidence.** [buildArtifactV2Root](../../../cli-connection-reader/model-artifact-v2.mjs:237) would accept an empty geometry index and an arithmetically valid `geometryState` supplied directly by its caller. No declarations are required, inspected, or retained. Thus the builder independently enforces summary consistency, but cannot enforce “declares nothing stays refused.”
   
   **Fix:** Require and validate declaration evidence for the new exception, and bind its receipts into the root instead of accepting a free-standing summary as proof.

3. **Warm validation does not establish that entities claim no geometry.** [readV3Cache](../../../cli-connection-reader/model-v3-cache.mjs:106) parses indexes and verifies blob digests without validating entity records. Under the proposed checks, a validly signed malformed cache can have an empty geometry index, matching summary counts, and entities referencing nonexistent tiles. Cold canonicalization rejects this; warm reuse republishes it. Ordinary unsigned tampering is already blocked by the cache signature; this is the same signed-malformed-input boundary exercised by the existing #604 test.
   
   **Fix:** For properties-only cache entries, validate entity shards, actual entity totals, and empty ownership arrays, then reconcile declarations and summary.

4. **The warm check trusts an unchecked `itemCount`.** The proposed validator compares the summary against `indexes.entities.index.itemCount`, but cache loading does not rebuild indexes, reconcile their counts with shard contents, or bind the selected indexes to `manifest.indexes`. Similarly, checking only geometry `objects.length` permits an empty index claiming a positive geometry count.
   
   **Fix:** Revalidate canonical index structure, receipt bindings, and counts before using them; require an empty geometry index to have `itemCount: 0`.

5. **The promised version negotiation does not exist on protocol v3.** The plan says providers gate on reader version and hosts inspect echoed packager identity. [buildProviderConversionRequest](../../../cli-connection-reader/model-provider-conversion.mjs:179) sends neither reader version nor supported output kinds. The [v3 result](../../../cli-connection-reader/model-reader.mjs:605) has no packager identity. Bumping package metadata alone supplies neither mechanism.
   
   **Fix:** Define an explicit host/provider capability handshake, or document external version selection as a deployment prerequisite and remove the nonexistent echoed-identity promise.

6. **The memory claim mistakes identity bytes for memory usage.** [admitIdentity](../../../cli-connection-reader/model-canonical-v2.mjs:179) budgets UTF-8 key lengths separately per set, excluding string and `Set` overhead. Another declaration set can add substantial heap usage for millions of short IDs while remaining far below the stated byte limit.
   
   **Fix:** Track declaration status alongside existing entity identities and enforce a cardinality or aggregate memory budget instead of duplicating the identity set.

7. **The blanket compatibility promise contradicts the planned version bumps.** Existing v3 roots can remain byte-identical, but legacy snapshot output includes packager version and bridge build identity in its signed preimage ([model-snapshot.mjs](../../../cli-connection-reader/model-snapshot.mjs:221)). Updating those versions changes existing snapshot results and signatures.
   
   **Fix:** Narrow the compatibility guarantee to unchanged v3 roots/cache identities and explicitly acknowledge the expected legacy snapshot identity changes.

The tests should pin these boundaries with direct-builder inputs and coherently signed malformed caches; unsigned cache edits alone will only exercise signature verification. Also add a declaration-order/shard-boundary permutation test to prove the new summary leaves root bytes deterministic.

VERDICT: REVISE

### Claude's response

- 1 (under-reporting): accepted. The label is now described as provider-declared coverage of the
  emitted entities; the plan no longer claims it catches omitted source objects or false
  classifications.
- 2 (root builder accepts a free-standing summary): rejected as a design change, accepted as a
  documentation point. Semantic checks over record contents already live only in canonicalization
  (dangling properties, entity ownership); the root builder is structural. Binding millions of
  per-entity declarations into the root to let the builder re-derive the summary would make the
  bounded root unbounded. The root-level invariant is that a complete empty-geometry root must carry
  a `geometryState` that tallies with its entity count. Rationale written into the plan.
- 3 (warm path does not establish entities claim no geometry): partly accepted. A `geometryState`
  root is now rebuilt with `buildArtifactV2Root` on every warm read and must equal the cached bytes,
  which re-establishes every structural root invariant. Entity shard contents are not re-read on a
  warm hit, matching drawn roots today; the cache is signed with AWARE's own key.
- 4 (unchecked itemCount; empty index claiming a positive count): accepted. Empty geometry must be
  `objects: []` and `itemCount: 0`; for `geometryState` roots the rebuild re-derives index counts from
  receipts and binds them to `manifest.indexes`.
- 5 (no version negotiation on v3): accepted. Negotiation is the existing package
  `minimumAwareVersion` check (`verifyRuntimeCompatibility`) plus the host's CLI version; the false
  echoed-identity claim is removed.
- 6 (identity memory): accepted. Declarations delete from the existing entity set instead of
  building a second one; dangling and repeated declarations share one message.
- 7 (compatibility vs version bumps): accepted by dropping the agent and bridge bumps (as #607 did),
  and the compatibility guarantee is narrowed to conversions without `geometry-state` shards.
- Tests: added direct-builder cases, coherently signed malformed caches, and a shard/order
  permutation determinism test.

## Round 2 — Codex

No remaining material blocker. The revised trust boundary is coherent: canonicalization verifies record semantics; the signed root attests that result; warm reuse verifies authenticated bytes and rebuilds structural invariants. Retaining every declaration is unnecessary under that explicit trust model.

Two small contract details should be tightened:

- **Reason counts lack an explicit integer requirement.** Step 3 specifies safe integers for `notDrawn` and `noGeometry`, but only `count ≥ 1` for reasons; two counts of `1.5` could satisfy a tally of `3`.
  
  **Fix:** Require every reason count and accumulated total to be a safe integer, and test fractional counts.

- **The compatibility wording contradicts the new empty-index check.** Step 4 and the risks section promise unchanged checks for roots without `geometryState`, while also adding `itemCount: 0` enforcement to all empty geometry indexes. Previously accepted, signed malformed degraded entries would now fail.
  
  **Fix:** Promise compatibility for previously produced valid roots, and explicitly identify rejection of malformed empty indexes as intentional hardening.

The destructive entity-set accounting resolves the additional identity-memory concern, and the runtime-version floor is an actual existing compatibility mechanism. The declaration permutation test and signed malformed-cache fixtures address the important verification gaps.

VERDICT: APPROVED

### Claude's response

Both tightenings applied: every reason count and the running total must be safe integers (fractional counts get a test), and the compatibility promise now covers validly produced roots, naming the `itemCount: 0` requirement on an empty cached geometry index as intentional hardening (AWARE never wrote anything else). Converged in 2 rounds.
