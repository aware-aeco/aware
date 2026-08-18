# Plan: Materialize Tekla grids with multi-word authored labels
_Round 5 candidate — one native Grid only_

## Goal
Make the exact FloLess #781 project grid materialize as one native Tekla `Grid`, with every elevation populated through that grid's `CoordinateZ`/`LabelZ`, even when an authored label is `2nd Floor`.

## Approach
1. Require every axis/level label at the script trust boundary to be an actual JSON string; do not coerce with `ToString()`. A non-string is a canonical `failed` record. A blank string remains the existing exhaustive Tekla-only unsupported grid family and does not abort unrelated records.
2. After live target resolution, overwrite any caller value and inject immutable `resolvedHostVersion` into script args; gate exclusively on that value. Preserve the existing path byte-for-byte when every label is already one whitespace-free Tekla token. For resolved Tekla 2026 only, accept exactly two nonblank, control-free tokens separated by one ASCII space, with no leading/trailing or other whitespace.
3. Reserve the complete set of authored whitespace-free tokens in each X/Y/Z family first. Replace the ASCII space with `_`, then allocate against the complete reserved/final set using deterministic `~<family-position>-<attempt>` retries until unique. V4 supports this workaround only when both authored and allocated token are at most 40 UTF-16 code units (`string.Length`, matching the FloLess/JavaScript authoring boundary)—a conservative Tekla-adapter capability boundary, not a canonical-schema or vendor maximum. Longer direct-caller labels remain exhaustive Tekla-only unsupported. The plan owns final `CoordinateX/Y/Z` and `LabelX/Y/Z`; it never inserts or modifies separate `GridPlane` records.
4. Queue one transactional `tekla-grid-label-tokenized` warning containing ordered `{ id, family, authoredLabel, nativeLabel }` mappings only for changed labels. Explain that Tekla's parent-grid label grammar cannot carry spaces and the native token changed. Publish only after successful read-back/commit; discard on failure.
5. Before retiring prior source-owned objects, reselect the one parent Grid and compare `CoordinateX/Y/Z`, `LabelX/Y/Z`, origin, extensions, and magnetism directly with immutable plan values. Enumerate automatically generated child planes read-only and match every child one-to-one by native label and global X/Y/elevation coordinate within 0.1 mm. Normalize each plane normal and require `abs(dot(normal, expectedFamilyAxis)) >= 1 - 1e-9`; the coordinate disambiguates X, Y, and Z families while allowing Tekla's observed negative Y-family normal. Never create, modify, tag, or receipt child planes independently. Canonical child receipts keep the parent GUID and `realizedBy` parent ID.
6. Bump `BakeMaterializerIdentity` v3→v4 and test the materialization-hash transition plus safe replacement of objects carrying the existing `AWARE_BAKE_V1` ownership marker.
7. Add unit/static tests for unchanged exact tokens; X/Y/Z `A B`→`A_B`; collisions against later authored and suffix-looking tokens; percent/underscore ordinary tokens; rejection of leading/trailing, repeated, tab/newline, Unicode whitespace, controls, and three-token labels; UTF-16/surrogate-pair boundary behavior; non-string canonical failure versus blank/grammar/length/host exhaustive unsupported; immutable parent read-back; one-to-one child label/normal/coordinate association; warning mapping/order beside the envelope warning; warning suppression on failure; and forged/requested 2026 overwritten by resolved live 2025. Update README/command docs, bump the Tekla manifest, and regenerate registries.
8. Build/test the bridge and strict-compile the canonical script against installed Tekla 2025/2026 SDKs. Live-test v3→v4 replacement, read-back, cleanup-before-retirement, and the full FloLess v24 browser send on Tekla 2026. State explicitly that Tekla 2025 received semantic compilation only and retains the existing whitespace-label unsupported classification.

## Key decisions & tradeoffs
- One canonical structural grid remains exactly one native Tekla `Grid`; elevations are one `CoordinateZ`/`LabelZ` sequence on that object.
- `2nd_Floor` is less pretty than the authored label, but deterministic, length-preserving, valid in Tekla's token grammar, and explicitly mapped in the receipt. This is preferable to dropping the entire grid.
- Exact token-only grids are unchanged and silent. Canonical AWARE/FloLess and IFC labels remain untouched because tokenization is Tekla-specific.

## Risks / open questions
- The native label is intentionally lossy in Tekla 2026. The structured warning is the audit trail and must be visible in FloLess receipt details.
- Tekla 2025 exposes the same parent Grid properties but has no running host in this session, so whitespace labels remain unsupported there.

## Out of scope
- Separately authored or modified native `GridPlane` objects, changing canonical labels, or claiming live Tekla 2025 verification.
