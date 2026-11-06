# #763 — Outlook mail-folder reads: folders, a folder's changes, one message's body

Status: plan, rev 1 (2026-11-06), before code. Issue: aware-aeco/aware#763. Builds on #709 (slices
A/B1/C/B2) and #764 (principal carries its tenant). Consumer: floless.app R6.8 slice 1b (#2470,
#2491, epic #2465 row 7) — an Outlook mail folder as a connected Knowledge source, synced into
immutable versions with Sync now / Pause / Remove / Erase.

Base: `origin/main` `2b6215d0c` (v0.174.0). One PR. Release is **not** part of this work (owner's
go-ahead needed); the manifest floor names the next release.

## R. Rev 2 — Codex round 1 fixes (supersede §§0-8 wherever they differ)

Codex R1: REVISE 13 (LOG). All accepted; the fixes below are the contract.

**R-1 No time filter on the sync read (R1-1).** `received-after` is removed from
`folder.list-messages`. Graph encodes a first-round filter into every later token, so a filtered
stream never returns an old message moved into the folder, and filtered message delta is documented
as capped at 5,000 messages. The stream is always unfiltered; a time window is the consumer's local
inclusion policy over the full stream. No filtered variant ships in this PR.

**R-2 Page size the caller controls, on every request (R1-2).** Input `page-size` 1..50, default 25,
rendered by code into the one `Prefer` value on **every** delta request, cursor requests included
(`Prefer: odata.maxpagesize=<n>, IdType="ImmutableId"`) — a page too large to read is retried *on the
same cursor* with a smaller `page-size`. Budgets: raw read cap 8 MiB for delta and get-message (new
`Operation.read_cap`), output cap = the caller's `maxOutputBytes` (≤ 1 MiB) on the projected answer.
Projection reduces recipients (50 each, counted), so the projected size of one record is bounded
(~40 KB worst case). Outcomes, all documented in the command page: page over the raw or output cap →
`E_CALL_OUTPUT_TOO_LARGE` with `reason: "page-too-large"` and nothing advanced — retry with a smaller
`page-size`; at `page-size: 1` the same answer is terminal for that stream (`reason:
"record-too-large"`), and the consumer's recovery is a reset (a new first round) plus labelling the
source as having one unreadable change. Live gate L8 proves a same-cursor retry with a smaller page
size is accepted by Graph; if it is not, this PR stops and the plan is revised (no silent fallback).
The "default page of 10" fallback is withdrawn.

**R-3 An unrepresentable change fails the page, never passes it (R1-3).** In a delta page, an
upsert or a removal whose `id` fails the Outlook id grammar (R-4), or an entry that is neither an
upsert object nor a removal, makes the whole call `E_CALL_FAILED reason: "unrepresentable-change"`
with **no cursor** — the caller cannot commit past it. There is no `omitted` on delta. A withheld
*optional* field (an internet-message-id or conversation-id outside its grammar, a name over its cap)
does not fail the page; it is marked (`<field>-withheld: true`, or `truncated`). Recovery for a poison
page is the same as R-2's terminal case: retry at `page-size: 1` to isolate, then a reset. Fixtures put
a bad upsert and a bad removal between two valid records and assert no cursor and no records.

**R-4 Outlook ids are exact and round-trip (R1-4).** New `outlook_id()` validator used for input
*and* output: 1..512 bytes, `[A-Za-z0-9_-]+` then 0..2 `=`, kept byte for byte (no case change, no
cleaning, never through `Entry::text`). Folder aliases (the fixed well-known list) are accepted **only**
by `folder.list`'s `parent-id`; `folder.list-messages` takes a real folder id (from `folder.list`) so
the consumer can compare it with a message's `parent-folder-id`, and `get-message` takes a message id.
Join ids: `conversation-id` must pass `outlook_id`, else absent with `conversation-id-withheld: true`;
`internet-message-id` exact printable ASCII 3..998 bytes starting `<` and ending `>`, else withheld;
`parent-folder-id` and attachment ids as `outlook_id`, else withheld / the attachment counted.
`exact_id()` and every existing row's projection are untouched; boundary tests 256/257/512/513, and a
test that every id an Outlook read emits is accepted by the input it feeds.

**R-5 One ordered change list; delta is a change notice, get-message is the state (R1-5).** Output:

```yaml
changes:                      # Graph's order, never regrouped
  - change: upsert
    id: "AAkALg…"
    fields: { subject: "…", from: {…}, received-at: "…", has-attachments: true, … }   # only what Graph sent
  - change: removed
    id: "AAkALg…"
    reason: "deleted"
cursor: "<opaque, R-7>"
more-available: true          # cursor continues this round (skip); false: round done (delta)
```

An upsert's `fields` holds only properties Graph sent; absent means *unknown*, never `false`/empty, and
`null` from Graph is kept as `null`. No default is invented (`kind` is present only when `@odata.type`
was sent). The same id may appear more than once, and as upsert and removal in either order, within a
page or across pages; AWARE neither merges nor dedups. **Consumer rule (documented in the command
page):** apply changes in order to a pending set keyed by id; the authoritative state of every id left
as "upserted" is `get-message` (R-6); a removal cancels pending work for that id. Tests: id-only
upsert, read-only update, repeated upsert, removal-then-upsert and upsert-then-removal, a cross-page
replay.

**R-6 get-message carries version and membership; the consumer protocol is written down (R1-6).**
get-message adds `parent-folder-id` and `change-key` (exact, withheld if malformed) and keeps
`last-modified-at`. The command pages document the protocol a consumer follows (FloLess slice 1b is
the first): (1) read a page; (2) durably record its changes as pending; (3) only then store the new
cursor; (4) hydrate each pending upsert with `get-message`; a 404 or a `parent-folder-id` that is not
the source folder means the message is not in the folder (treated as removed); (5) transient failures
(timeout, 429/5xx) stay pending and retry independently of the cursor; terminal ones (404, a body over
the raw cap — `E_CALL_OUTPUT_TOO_LARGE reason: "message-too-large"`) resolve the item (removed, or kept
with headers and labelled "body unreadable"); (6) a removal or Erase arriving later cancels pending
work, and a late hydration result for a cancelled id is discarded. AWARE ships no state for this; it is
the consumer's.

**R-7 A versioned cursor envelope (R1-7).** `cursor` is `odc1.` + unpadded base64url of the canonical
JSON `{"v":1,"op":"<operation sha256>","folder":"<folder-id>","binding":"<bindingId>","kind":"skip"|"delta","token":"<token>"}`.
Input grammar: the prefix, then base64url, ≤ 16,384 bytes; decoded JSON must have exactly those keys,
`v: 1`, `op` equal to this operation's digest, `folder` byte-equal to the call's `folder-id`, `binding`
equal to the call's bound account, `kind` one of the two, `token` 1..8,192 bytes with no control
character — else `E_CALL_INPUT_INVALID reason: "cursor-context"` (zero requests; no token or link in
the failure). The envelope is **not authenticated and not a security boundary**: a caller can forge
one, and gains nothing — the route, `$select`-free continuation, scopes, binding check and the
projection allowlist are code-owned, so a foreign token can at most produce a Graph error or fields
the projection drops. It exists to keep streams apart (another folder, account or contract version is
refused, not silently mixed). Token handling: the link's query is decoded exactly once (`url` crate
`query_pairs`), stored decoded, and sent through ureq's `query()` (encoded exactly once); a test feeds
a token holding `%`, `+`, `=` and `/` through a round trip. **Supported link shape** (live gate L2/L3):
Graph origin, any path (unused), and a query holding exactly one pair — `$skiptoken` on a nextLink,
`$deltatoken` on a deltaLink; anything else → `E_CALL_FAILED reason: "cursor-link-unsupported"`
(recoverable only by an AWARE update; the consumer keeps its last cursor). Token-only reconstruction is
an explicit live gate for both kinds, not a documented Microsoft promise.

**R-8 Dead cursors are recognised from Graph's error code, bounded (R1-8).** `get_bounded` gains, for
status ≥ 400 on operations that opt in (delta only), a bounded read (16 KiB) of the error body and a
parser that keeps only `error.code` when it is one of an allowlist: `syncStateNotFound`,
`syncStateInvalid`, `resyncRequired`, `SyncStateNotFound`-casing folded. Mapping on delta: status 410
(any code), or 400/404 with an allowlisted code → `E_CALL_FAILED reason: "cursor-expired"` with the
numeric `status`. A 404 without an allowlisted code stays `status: 404` (folder gone). Never the
provider's message, headers, URL or raw JSON. Tests: 410; 404 + `syncStateNotFound`; 404 plain; an
error body over 16 KiB; malformed JSON; a body holding a sentinel secret, asserted absent from output,
error and log. **Consumer reset rule (documented):** on `cursor-expired` start a new unfiltered round
and remove items unseen by it only after that round **completes** (a delta cursor), never at its start.

**R-9 Attachments: a settled query and honest completeness (R1-9).** The query
`$expand=attachments($select=id,name,contentType,size,isInline)` is the contract; whether Graph v1.0
accepts it is **live gate L5, run before the digest is frozen** (a failure revises the plan; no
automatic fallback). Output `attachments-state`: `complete` (Graph sent no `attachments@odata.nextLink`
and AWARE cut nothing), `partial` (either happened; `attachments-omitted` counts only the ones AWARE
saw and cut), `unavailable` (Graph sent no `attachments` array). Inline attachments are listed with
`inline: true` (`hasAttachments` excludes inline-only ones, so it is reported as Graph's flag and
nothing more). `@odata.type` → `kind: file | item | reference | other`. Any `contentBytes` or item
payload is never projected and marks `content-withheld: true`. L5 seeds file, item, reference and inline
attachments and an attachment > 8 MiB with a small body, and checks the raw response size stays small.

**R-10 Folder paging by a code-owned offset (R1-10).** `folder.list` output adds `next-skip`: present
when Graph sent `@odata.nextLink`, equal to the request's `skip` plus the number of entries Graph
returned in `value` (omitted ones included, so nothing repeats or is skipped). `skip` max 100,000; a
`next-skip` beyond it is not emitted and `limit-reached: true` is. Hidden folders are excluded (Graph's
default; `includeHiddenFolders` is not sent), stated in the command page. L1 forces multiple pages
(`page-size: 2`) at root and beneath a folder. No local cut on folder.list either: page-size goes to
`$top`, and if Graph returns more than asked the page is still returned whole.

**R-11 A live gate that exercises the risky paths (R1-11).** §7 is replaced by: a dedicated folder
Pawel creates, **seeded with ≥ 7 labelled messages** (subjects `763-A`…`763-G`); `page-size: 2`, so the
baseline is guaranteed multipage. L2 baseline: record every id; both cursor kinds reach Graph as
token-only requests and the pages keep the same field set; the final `delta` cursor is valid and
reusable (no claim that a quiet round returns a different token). L3 mutations, matched by recorded id
with up to 3 retries 30 s apart for visibility: a new delivery; an old message moved in from another
folder (its id equals its id read there before the move — same-mailbox immutability); one moved out
and one deleted (removals carry the baseline ids); read/unread toggle (an upsert whose projected fields
are unchanged); a draft whose body is edited (`change-key` changes in get-message); a **successful**
cloud recall of a message sent to Pawel from a second mailbox he controls, if available (the
recipient copy's removal is asserted after the recall reports success; if no second mailbox is
available it is recorded as not verified, not inferred). L4 get-message on each surviving id. L5 as
R-9. L8 same-cursor retry with a smaller page size. L9 a cursor for another folder → `cursor-context`,
zero requests. L7 wrong binding. Implementation acceptance is **held** until L1-L9 pass or the plan is
revised; the PR carries `hold` until then.

**R-12 Tests that can go red (R1-12).** (a) A new `pre_763_rows_keep_their_digests` pins **every**
existing row, B2 included, by explicit command list and literal digest captured from `main` before any
edit; the B2-era test's filter additionally excludes `outlook.mail.folder.*` and
`outlook.mail.get-message` (membership scoped, no pin changed). (b) Digest-binding mutations: for each
new row, changing `read_cap`, the cursor location/grammar id, a fixed header, the fixed query or the
projection id changes `sha256()` (asserted by building a modified copy of the row). (c) The whole-page
guard is tested through the real fixture path: a Graph page of 120 entries with `page-size: 25` must
come back with 120 changes; a deliberate `take(limit)` injected into the projection (verified once by
hand and recorded in the PR) makes it fail. (d) End-to-end fixture tests for R-2/R-3/R-7/R-8 failures;
raw OData or a URL in any input refused with zero requests; secret sentinels in Graph bodies, links,
error bodies and headers never reach output, failure details or `agent-call.log`. (e) Existing
projections' behaviour pinned where helpers are shared (`html_text`, `message_text` unchanged).

**R-13 Erase belongs to the consumer, and the hand-off says so (R1-13).** These reads detect
deletions; they do not make a consumer's stored copies erasable. The command page names the consumer's
obligations (fence late results by source/generation id; commit page and cursor durably and
idempotently; Remove / Pause / Erase are distinct; Erase cancels pending hydration and purges every
retained copy it promises to). FloLess slice 1b owns and verifies them (its plan, its E2E: erase while
a page or body read is in flight, after several versions). This PR claims only the reads.

## 0. What the consumer needs, and why the issue's shape is changed

The issue asks for `folder.list`, `folder.list-messages` with `received-after` (oldest first, tiebreak
on id) and `get-message`. A `received-after` cursor answers "what arrived since", but a Knowledge
source must also see mail that was **deleted, moved out of the folder or recalled**, and mail **moved
into** the folder (its `receivedDateTime` is old, so a received-after cursor never returns it). Graph
cannot order mail by id either, so the issue's tiebreak cannot be rendered. Microsoft's answer for "keep
a local store of a folder in step" is the message **delta** query
(`GET /me/mailFolders/{id}/messages/delta`, permissions `Mail.ReadBasic` least, `Mail.Read`,
`Mail.ReadWrite`): pages carry `@odata.nextLink` (`$skiptoken`) until a final `@odata.deltaLink`
(`$deltatoken`); deletions and moves-out arrive as `{"id": …, "@removed": {"reason": "deleted"}}`;
read/unread changes arrive as updates. The best long-term shape is therefore a **delta read with an
opaque cursor**, and `received-after` survives only as an optional bound on the *first* round (Graph
supports exactly `$filter=receivedDateTime ge|gt {value}` there). The issue is updated to say so.

Bodies are a separate read: one HTML newsletter can be hundreds of KB, so bodies inside a 50-message
delta page would push the page over the response cap and **wedge the cursor** (a page that cannot be
read can never be advanced past). The delta page carries headers and Microsoft's preview; the body is
`get-message`, one message per call, with its own larger read cap.

## 1. Commands (microsoft-365 0.7.0, `minimum-cli-version` 0.175.0)

| Command | Request | Inputs | Accepted scopes (requested) | Admin consent |
|---|---|---|---|---|
| `outlook.mail.folder.list` | `GET /v1.0/me/mailFolders/{parent-id}/childFolders?$select=id,displayName,parentFolderId,childFolderCount,totalItemCount,unreadItemCount&$top=N&$skip=K` | `parent-id` outlook id (`msgfolderroot` = top level); `page-size` `$top` 1..100, default 50; `skip` `$skip` 0..10000, default 0 | `Mail.ReadBasic`, `Mail.Read`, `Mail.ReadWrite` (`Mail.Read`) | no |
| `outlook.mail.folder.list-messages` | first round: `GET /v1.0/me/mailFolders/{folder-id}/messages/delta?$select=<fixed>[&$filter=receivedDateTime gt {received-after}]`; continuing: the same path with **only** `$skiptoken=<t>` or `$deltatoken=<t>` | `folder-id` outlook id (well-known names allowed); `cursor` optional; `received-after` optional, only without `cursor` | `Mail.Read`, `Mail.ReadWrite` (`Mail.Read`) — `Mail.ReadBasic` has no `bodyPreview` | no |
| `outlook.mail.get-message` | `GET /v1.0/me/messages/{id}?$select=<fixed>&$expand=attachments($select=id,name,contentType,size,isInline)` | `id` outlook id | `Mail.Read`, `Mail.ReadWrite` (`Mail.Read`) | no |

All three send `Prefer` on the data request only (never on `/me`), as one header value (ureq's
`set` replaces a same-name header, so two `Prefer` pairs would silently collapse — a test pins the one
value):

- list-messages: `Prefer: odata.maxpagesize=50, IdType="ImmutableId"`
- get-message: `Prefer: IdType="ImmutableId"`
- folder.list: none (folder ids do not change on move of their messages).

**Immutable ids.** Outlook's default message id changes when a message moves between folders; a
source keyed on it would see a move as delete + new mail. `IdType="ImmutableId"` keeps a message's id
for its life in the mailbox, and delta and `GET /me/messages/{id}` both honour it, so the id a page
returns is the id `get-message` accepts. **Live check L4** proves the two agree (one id read through
both); if Graph refuses the combined `Prefer` value on delta, the fallback is to send
`IdType="ImmutableId"` alone and drop `odata.maxpagesize` (Graph's default page is 10), never to drop
immutable ids.

Fixed `$select`s:

- list-messages: `id,internetMessageId,conversationId,subject,from,toRecipients,ccRecipients,receivedDateTime,sentDateTime,lastModifiedDateTime,hasAttachments,bodyPreview,webLink,isDraft`
- get-message: the same plus `body,uniqueBody`.

No `$orderby` on delta: Graph supports only `receivedDateTime desc` there and the order is not part of
the contract; the consumer keys on id.

## 2. New input kinds (code-owned grammars; `E_CALL_INPUT_INVALID`, zero requests, otherwise)

- **`IdKind::OutlookId`** (`pattern: "outlook-id/1"`): 1..512 bytes; either a well-known folder name
  from a fixed list (`inbox`, `sentitems`, `drafts`, `deleteditems`, `archive`, `junkemail`,
  `msgfolderroot`, `outbox`, `clutter`, `conversationhistory`, `scheduled`, `searchfolders`,
  `recoverableitemsdeletions` — compared case-insensitively, sent lower case), or base64url
  `[A-Za-z0-9_-]+` followed by 0..2 `=`. No `.`, `/`, `%`, `+`, `?`, `#`, `:` or whitespace, so a
  segment can never be `.`/`..` or escape the route. A real folder id, a real default message id and a
  real immutable id (all three carry `=` padding and `-`) are fixtures. A malformed `id` coming back
  *from* Graph is dropped and counted (`omitted`), never cleaned. Well-known names are accepted only by
  the two folder inputs, not by `get-message`'s `id`.
- **`cursor`** (`Location::DeltaState`, `format: "graph-delta-cursor/1"`): `skip:` or `delta:` then
  1..4096 bytes of URL-unreserved `[A-Za-z0-9._~-]`, so percent-encoding is the identity and the
  token Graph sent is the token Graph gets back byte for byte. With a cursor the request carries the
  token as `$skiptoken` / `$deltatoken` and **nothing else** — no `$select`, no `$filter` (Graph
  encodes the first round's options into the token; resending them is undocumented). If live shows a
  token outside that alphabet, the grammar is widened to the observed alphabet with a fixture, never
  to "anything".
- **`received-after`** (`format: graph-utc-timestamp/1`, `Location::Filter`, optional): rendered
  `$filter=receivedDateTime gt <literal>` by the existing renderer. Given together with `cursor` →
  `E_CALL_INPUT_INVALID` (`input: received-after`, reason `with-cursor`): the bound belongs to the
  first round and a token already carries it.

## 3. Output

### `outlook.mail.folder.list` — projection `outlook-folders/1`

```yaml
folders:
  - id: "AAMkAG…AAA="
    name: "Project 2411"
    parent-id: "AAMkAG…AQ=="
    child-count: 2
    total-count: 318
    unread-count: 4
more-available: false   # Graph sent @odata.nextLink, or AWARE cut the page
omitted: 0
```

`project_collection` with a folder entry; counts are kept only when they are non-negative integers
(else absent). `@odata.nextLink` is reported, never followed; the caller pages with `skip`.

### `outlook.mail.folder.list-messages` — projection `outlook-delta/1`

```yaml
messages:
  - id: "AAkALg…"                  # immutable id
    internet-message-id: "<CAF…@mail.gmail.com>"
    conversation-id: "AAQkAG…"
    kind: "message"                # also event-message, other — from @odata.type
    subject: "RE: RFI 042"
    from: { address: "engineer@acme.com", name: "A. Engineer" }
    to: [ { address: "…", name: "…" } ]
    cc: []
    recipients-omitted: 0
    received-at: "2026-10-30T09:12:00Z"
    sent-at: "2026-10-30T09:11:58Z"
    last-modified-at: "2026-10-30T09:12:01Z"
    draft: false
    has-attachments: true
    preview: "Please see attached sketch…"
    web-url: "https://outlook.office365.com/owa/?ItemID=…"
removed:
  - { id: "AAkALg…", reason: "deleted" }   # deleted or moved out of the folder
cursor: "skip:…"         # or "delta:…" — pass back as `cursor`
more-available: true     # exactly: cursor is a skip cursor (this round is not finished)
omitted: 0               # entries dropped: malformed id, or neither a message nor a removal
```

Rules, each a test:

1. **Never cut a delta page.** A page is returned whole; AWARE has no local bound on it, because a
   dropped entry would be lost when the caller advances the cursor. The response cap (the call's
   `maxOutputBytes`, ≤ 1 MiB) bounds it; a page over the cap is `E_CALL_OUTPUT_TOO_LARGE` and the
   cursor the caller holds is still valid (nothing advanced). With `maxpagesize=50` and no bodies a
   page is ~50 × 2 KB.
2. **The cursor comes from exactly one link.** Exactly one of `@odata.nextLink` / `@odata.deltaLink`
   must be present; both or neither → `E_CALL_FAILED response-shape`. The link must parse as a URL on
   the Graph origin (`https://graph.microsoft.com`) whose query holds exactly one pair, `$skiptoken`
   for nextLink / `$deltatoken` for deltaLink, whose value passes the cursor alphabet; anything else
   → `response-shape`. **Only the token leaves**; the link's path is not used (AWARE rebuilds the
   request from its own route and the caller's `folder-id`), so a link pointing elsewhere can never
   be followed. AWARE never follows the link itself (one page per call, as #709).
3. **Removals.** An entry with an `@removed` object becomes `removed: {id, reason}` (`reason` capped
   text, Graph documents only `deleted`); its id must pass `exact_id` or it is counted in `omitted`.
4. **Text fields** use the existing `Entry` rules (control characters removed, capped, named in
   `truncated`): subject 1024, preview 512, names 256, addresses 320, ids 512 (outlook id) /
   internet-message-id 998 (RFC 5322 line limit). `to`/`cc` keep at most 50 each; the rest are
   counted in `recipients-omitted`.
5. **`kind`** from `@odata.type`: `#microsoft.graph.message` → `message`,
   `#microsoft.graph.eventMessage` / `eventMessageRequest` / `eventMessageResponse` → `event-message`,
   absent → `message` (Graph omits it on plain messages), anything else → `other`. Entries are kept
   whatever their kind; the caller decides.
6. Read/unread changes arrive as updates with unchanged projected fields (`isRead` is not selected);
   the caller sees an entry identical to what it holds and writes nothing.

### `outlook.mail.get-message` — projection `outlook-message/1`

The list-messages entry fields, plus:

```yaml
text: "Hi Anna,\nThe reaction at grid C is 120 kN…"     # body as plain text
unique-text: "The reaction at grid C is 120 kN…"       # only what this message added to its thread
attachments:
  - { id: "AAMk…", name: "calc.pdf", content-type: "application/pdf", size: 183220, inline: false }
attachments-omitted: 0
truncated: [text]          # when a text was cut
```

- **Body → text in AWARE** with the existing `html_text` + `message_text` (the Teams reducer), so the
  conversion is AWARE's, versioned by the projection id, and a Microsoft-side converter change cannot
  silently re-version a consumer's content. `contentType: text` is taken as is. Cap: 65,536 chars per
  text, cut named in `truncated`.
- **Read cap.** get-message reads up to 8 MiB from Graph (a per-operation `read_cap`, new field), then
  projects; the *output* still obeys the caller's `maxOutputBytes`. Over 8 MiB from Graph →
  `E_CALL_OUTPUT_TOO_LARGE`; the consumer keeps headers + preview and labels the body unread.
- **Attachments are names, never bytes.** `$expand=attachments($select=id,name,contentType,size,isInline)`
  — `contentBytes` is never selected; if Graph sends it anyway it is not projected and the entry gets
  `content-withheld: true`. At most 20, the rest counted. Reference attachments (`@odata.type`
  `referenceAttachment`) keep `kind: reference` and no URL. Live check L5 confirms `$expand` with an
  inner `$select` on attachments works on v1.0 for a delegated read; if not, the expand is dropped and
  `has-attachments` stands alone.

## 4. Errors the consumer must tell apart

- A **dead cursor**: Graph answers 410 Gone (sync state expired or reset). `E_CALL_FAILED` with
  `status: 410` and `reason: "cursor-expired"` (a code-owned mapping of status 410 on this operation
  only). FloLess then starts a new round and reconciles against what it holds.
- A **removed folder or message**: 404 → `E_CALL_FAILED status 404` (existing behaviour).
- Everything else as #709 (scope-missing before the data request, binding, timeout, cap).

## 5. Code changes (all in `cli/src/runtime/agent_call.rs` unless named)

1. `IdKind::OutlookId` + grammar; `Location::DeltaState` + cursor grammar; optional
   `InputMap::received_after()` (a non-required `since`-kind filter input named `received-after`).
2. `wire()`: a `DeltaState` input present → refuse any other non-path input given by the caller
   (`with-cursor`), and build the query as exactly one token pair, skipping `fixed_query`; otherwise
   unchanged. Defaults of absent optional inputs are not sent on the cursor path.
3. `Operation`: new `read_cap: Option<usize>`; `read_data` uses it for the HTTP read when set, the
   caller's cap for the projected output. Digest: included in the v2 canonical form only when set, so
   every earlier digest is unchanged (pinned).
4. Projections `OutlookFolders`, `OutlookDelta`, `OutlookMessage`; a 410-to-`cursor-expired` mapping
   keyed on projection `OutlookDelta`.
5. Manifest `20-agents/aeco/cross-cutting/microsoft-365/manifest.yaml` 0.7.0, floor 0.175.0, three
   commands with inputs/outputs/permissions; command docs `commands/outlook.mail.folder.list.md`,
   `outlook.mail.folder.list-messages.md`, `outlook.mail.get-message.md`; `10-core/cli-spec.md` list of
   reviewed reads; `BREAKING.md` entry (additive, floor change); example apps and skills that pin
   `microsoft-365` follow (as #740 did); `python scripts/sync_stats.py --write`; `aware agent reindex`.
6. `outlook.mail.search` unchanged (its digest pinned unchanged).

## 6. Tests (fixture Graph server, as #709)

- Grammar tables: `OutlookId` (real folder / default / immutable ids with `=`, `-`, `_`; well-known
  names any case; reject `.`, `..`, `a/b`, `%2F`, `+`, whitespace, `=` mid-id, 513 bytes, well-known
  name as get-message `id`); cursor (both prefixes; reject other prefix, empty token, `%`, `&`, `=`,
  space, 4097 bytes).
- Wire: exact decoded query per command — first round with and without `received-after`, both cursor
  kinds (one pair, no `$select`), `Prefer` value exact and absent from the `/me` request;
  `received-after` + `cursor` → zero requests.
- Delta projection: nextLink-only and deltaLink-only pages → `skip:`/`delta:` cursors with the exact
  token; both / neither / foreign host / extra query pair / token outside alphabet / `$skiptoken` on a
  deltaLink → `response-shape`; `@removed` entries; malformed ids counted; event message kind; a page
  of 120 entries returned whole (no cut); recipients cap; 410 → `cursor-expired`.
- get-message: HTML body + uniqueBody reduced; 70,000-char body cut and named; `contentBytes` present
  → withheld, not projected; attachments cap; 9 MiB body → `E_CALL_OUTPUT_TOO_LARGE`; 3 MiB body under
  read cap with output under `maxOutputBytes` → ok.
- Scope-missing per command (`Mail.ReadBasic` alone passes folder.list, fails the other two).
- Digest pins for the three new rows; unchanged pins for every existing row (they must not move).
- Manifest test (0.7.0, 0.175.0, all three runnable, writes still planned); one workflow node.
- **Negative controls:** each new grammar's reject table runs on every `cargo test`; the "never cut a
  delta page" test fails if a local bound is applied (checked by temporarily setting `limit_input`).

## 7. Real E2E (gate)

A locally built CLI, a temp `AWARE_HOME` holding a copy of the `microsoft-365` slot FloLess Desktop
already stores for Pawel's floless.io account (it holds `Mail.Read`; no new sign-in is needed — if a
copy cannot be made without his action, Pawel signs in through FloLess Desktop in the browser, never by
device code). Test folder: a folder Pawel names (or a new one he makes) with ≥ 3 messages.

- **L1** `folder.list` with `msgfolderroot` → the folder appears with its counts; a child read with
  `parent-id` = a folder with children.
- **L2** delta first round with no cursor → pages until a `delta:` cursor; record ids and page count.
  Delta again with that cursor → no messages, no removals, a new `delta:` cursor.
- **L3** Pawel moves one message out and one in, deletes a third, marks a fourth read → delta with the
  cursor returns: the moved-in message (new id), removals for the moved-out and deleted ones, and the
  read one as an unchanged update.
- **L4** `get-message` on an id from L2 → body text; the same id read before and after a move (L3)
  is unchanged (immutable ids hold); the combined `Prefer` header was accepted.
- **L5** a message with an attachment → names only, no bytes.
- **L6** `received-after` = yesterday on a fresh first round → only mail received after it (plus any
  documented out-of-filter events, recorded).
- **L7** a wrong binding → `E_BINDING_CHANGED`, zero data requests (existing behaviour, re-run).
- Record the observed token alphabet (L2) and whether Graph ever sent both links.

## 8. Questions (only if the code cannot settle them)

None for AWARE. FloLess-side questions (one folder or with subfolders; attachments; how much body is
kept) belong to the slice-1b plan; this read supports all of them: `folder.list` walks children,
attachments are named, the body cap is 65,536 chars with `unique-text` alongside.
