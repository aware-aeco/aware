---
name: bcf-21-vs-30
description: This skill should be used when choosing a BCF version to emit or when running the bcf-file convert verb — deciding between BCF 2.1 and 3.0, understanding the (mostly structural) schema differences, and knowing what actually changes on conversion. Encodes the version-selection rule (default 2.1 outbound) and the real, schema-verified conversion behaviour.
---

# BCF 2.1 vs 3.0 — what changed

> Verified against the buildingSMART BCF-XML schemas: `release_2_1/Schemas/markup.xsd`, `release_3_0/Schemas/markup.xsd`, and `visinfo.xsd` for both. Where field-level intuition and the schema disagree, the schema wins — several "obvious" 2.1→3.0 differences are myths (see the box below).

## The rule

**Default to BCF 2.1 for anything outbound.** Accept 3.0 inbound (the agent reads both). As of 2026, downstream support for 3.0 is still patchy — ACC Issues, BIMcollab, Revizto, and Trimble Connect do not all default to it. Emit 2.1 unless every consumer in the chain is confirmed to read 3.0. Set `target-version: 3.0` on [`convert`](../commands/convert.md) only when you control the whole chain (e.g. Solibri ↔ Solibri, or Solibri ↔ Newforma).

Both versions are buildingSMART specs; 3.0 (2021) is the successor to 2.1 (2014).

## What actually changed (3.0 vs 2.1)

The 3.0 changes are **mostly structural re-serialisation, not new topic data**:

| Aspect | 2.1 | 3.0 |
|---|---|---|
| Repeated child elements (`ReferenceLink`, `Label`, `DocumentReference`, `RelatedTopic`) | direct repeated elements on `Topic` | each wrapped in a container (`ReferenceLinks`, `Labels`, `DocumentReferences`, `RelatedTopics`) |
| `Comment`s and `Viewpoints` | siblings of `Topic` under `Markup` | nested **inside** `Topic` (`Topic > Comments`, `Topic > Viewpoints`) |
| `Topic.ServerAssignedId` | not present | **new** — human-readable issue id (the only genuinely new topic datum) |
| `Topic.Index` | present | present but **deprecated** |
| `DocumentReference` | `ReferencedDocument` (string path/URL) + `Description` | `DocumentGuid` \| `Url` choice + `Description` |

## Myths — fields that did NOT change (don't write conversion logic around these)

The following are **identical in both 2.1 and 3.0** per the schemas. Earlier drafts of these docs (and a lot of folklore) get them wrong:

- **`Stage` is in both.** `markup.xsd` declares `Topic/Stage` in 2.1 *and* 3.0. It is not a 3.0 addition; a 3.0→2.1 conversion does **not** drop it.
- **`DueDate` is in both.**
- **`ModifiedDate` is optional in both** (`minOccurs="0"`), on `Topic` and `Comment`. It is not "required in 3.0".
- **`ReferenceLinks` are plain strings in both** (`NonEmptyOrBlankString`). The `{ Url, Description }` object shape belongs to the separate **`DocumentReference`** element, not to reference links. There is no string→object upgrade.
- **Comments are flat in both.** Neither 2.1 nor 3.0 `Comment` has a `ReplyToComment` / threading element (`ReplyToComment` existed only in the abandoned 2.0 and was removed). There is no "3.0 reply tree."
- **Component colouring supports an optional alpha byte in both.** `visinfo.xsd` `Color` pattern is `[0-9A-Fa-f]{6}([0-9A-Fa-f]{2})?` in 2.1 *and* 3.0 — 6 hex (`FF00FF`) or 8 hex with a **trailing** alpha byte (`FF00FF99` = RRGGBB**AA**). 2.1 is not "RGB only", and downgrade does not drop alpha.

## Conversion behaviour ([`convert`](../commands/convert.md))

Because the difference is structural, conversion is mostly reshaping, not data loss.

### 2.1 → 3.0 (structural, lossless)

- Wrap repeated elements into their 3.0 containers (`ReferenceLink`→`ReferenceLinks/ReferenceLink`, `Label`→`Labels/Label`, etc.).
- Move `Comment`s and `Viewpoints` from `Markup` level into `Topic`.
- `ServerAssignedId` has no 2.1 source — left absent.
- All topic data (Stage, DueDate, ModifiedDate, priority, labels, …) carries over unchanged.

### 3.0 → 2.1 (structural; one real drop)

- Un-wrap containers back to direct repeated elements; lift `Comment`s/`Viewpoints` from `Topic` to `Markup` level.
- **`ServerAssignedId` is dropped** (no 2.1 equivalent) — the one genuine data loss; `convert` records it in `warnings`.
- Map `DocumentReference` `DocumentGuid`/`Url` to the 2.1 `ReferencedDocument` string.
- Everything else round-trips losslessly.

## Why pick 3.0 when you can

- **`ServerAssignedId`** gives issues a stable human-readable number.
- **Topic-as-container** (comments + viewpoints nested in the topic) is a cleaner unit to pass around than 2.1's flat `Markup` siblings.
- It's the actively-maintained schema; new tooling targets it first.

None of these outweigh 2.1's wider compatibility for outbound delivery today — hence the default-2.1 rule above.

## See also

- [`convert`](../commands/convert.md) — the verb that performs the translation
- [`merge`](../commands/merge.md) — requires uniform versions; convert first
- [bcf-format-anatomy](./bcf-format-anatomy.md) — the archive layout and the markup structure that gets reshaped
