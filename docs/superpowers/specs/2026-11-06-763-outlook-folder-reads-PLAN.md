# #763 — Outlook mail-folder reads: folders, a folder's changes, one message's body

Status: plan, rev 4 (2026-11-06). Issue: aware-aeco/aware#763. Builds on #709 (slices
A/B1/C/B2) and #764 (principal carries its tenant). Consumer: floless.app R6.8 slice 1b (#2470,
#2491, epic #2465 row 7) — an Outlook mail folder as a connected Knowledge source, synced into
immutable versions with Sync now / Pause / Remove / Erase.

Base: `origin/main` `2b6215d0c` (v0.174.0). One PR. Release is **not** part of this work (owner's
go-ahead needed); the manifest floor names the next release.

## R4. Rev 4 — Codex round 3 findings, patched by operator ruling (supersede Rev 3 wherever they differ)

Codex R3: REVISE 3, all self-feeding (review counter: **2 self-feeding rounds**, the stop signal). Route
chosen by the operator: `operator: Pawel 2026-11-06 — patch` — write the three findings in as Rev 4, no
fourth plan round. All three are interpretation rules on AWARE's returned evidence; none changes the
output shape or the code. The command pages state each one.

**R4-1 Incompleteness belongs to the whole round, not the page (R3 finding 1).** A page's
`unidentifiable` counts only that page. A consumer must carry it across every page and cursor commit
of a round: one unidentifiable entry anywhere makes the **whole round** incomplete, even when its
terminal page is clean and carries a delta cursor. An incomplete round never authorizes sweeping items
it did not see, and never clears an earlier warning. Only a complete fresh round (no cursor, every page
read, zero unidentifiable across all of them), reconciled against what the consumer holds, clears
earlier uncertainty. Until then, retained content may be stale, including a deletion that arrived as
an unidentifiable entry. This is a consumer obligation (FloLess slice 1b owns the mechanism). AWARE
states it on `outlook.mail.folder.list-messages` ("Unidentifiable entries stay in place").

**R4-2 Output overflow at the maximum budget: retry with a smaller page size (R3 finding 2).** Delta
`output-too-large` means first retrying the identical request with a larger `maxOutputBytes`, up to
1 MiB. At 1 MiB, retry the **same** cursor (or the same first request) with a smaller `page-size`. The
failed attempt advances nothing: no cursor is returned with a failure. At `page-size: 1` and 1 MiB, the
one record does not fit the caller's budget. That is terminal for the caller's automatic retries, as
`raw-too-large` at size 1 is, and is reported with its own reason and `outputBytes`, never as a bad
record. Test: `a_delta_page_over_the_largest_budget_fails_without_a_cursor` (raw under 8 MiB,
projection over 1 MiB, failing at the maximum budget with no cursor in the failure).

**R4-3 `get-message`'s raw cap is intrinsic to the message (R3 finding 3).** The two reasons are
shared, but recovery is per operation. On delta, `raw-too-large` means retrying with a smaller
`page-size`. On `get-message`, it is terminal for that message: the read has only an id, so neither
page size nor budget can repair it, and retrying will not help. `output-too-large` on `get-message`
keeps the larger-budget retry. Both are stated on the `get-message` page ("Too large"). Test:
`a_message_over_the_raw_cap_is_raw_too_large` (exists).

**R4-4 `parent-folder-id` is not proof of a move (correction from the consumer's plan review, FloLess
slice 1b Codex R1 #1).** Rev 3's scope correction said a valid `parent-folder-id` unequal to the folder
proves absence. Nothing guarantees that a message's `parent-folder-id` is spelled like
`outlook.mail.folder.list`'s folder `id`, and no observation of a message can calibrate it, because the
message may already have moved. The command pages now say: a 404 proves deletion; membership is
settled by a fresh round; do not compare the two ids byte for byte. Wording only; no code change. The
live gate records what Graph actually returns.

## R3. Rev 3 — Codex round 2 fixes (supersede Rev 2 wherever they differ)

Codex R2: REVISE 6, all self-feeding (review counter: **1 self-feeding round**). R2 confirmed R1-1, 4,
8-13 resolved and the R-7 fields present on `CallRequest`.

**Scope correction (R2-3, R2-5, R2-6).** The *consumer's* apply/hydrate protocol (Rev 2 R-6's steps,
R-5's "consumer rule") is not AWARE's contract and moves out of this plan into the FloLess slice-1b
plan, where it is hardened on its own (FloLess already serializes one sync pass per source under its
lifecycle locks, so hydration jobs never race — R2-6 — and per-id reconciliation is decided there,
R2-3/R2-5). AWARE's command pages keep only facts a consumer needs: Graph does not order changes, ids
repeat, a removal notice is evidence and not proof, `get-message` 404 or a valid `parent-folder-id`
unequal to the folder proves absence, and a missing/withheld `parent-folder-id` proves nothing. AWARE
always selects `parentFolderId` in get-message (pinned in the fixed `$select`).

**R3-1 Progress past an unidentifiable change, labelled (R2-1, replaces R-3's no-cursor rule).** A
delta entry whose id fails the Outlook grammar, or that is neither an upsert object nor an `@removed`
entry, is emitted in place as `{change: "unidentifiable", position: <index in Graph's page>}` (no id,
no fields, nothing raw), the page's `unidentifiable: <n>` counts them, and the cursor **is** returned.
The consumer must carry the count into its record ("N changes in this folder could not be identified")
and may never claim a complete sync while any are outstanding; only a later completed fresh round
with `unidentifiable: 0` clears it. This makes the stream progress with acknowledged incompleteness
instead of a reset loop. A page Graph's answer to which exceeds the raw read cap at `page-size: 1` is
the one non-progressing case: `E_CALL_OUTPUT_TOO_LARGE reason: "raw-too-large"`, no cursor; the
consumer stops automatic retries for that source and shows it as blocked with the retry choices
(the reads cannot skip a record Graph will not hand over in 8 MiB). Tests: a persistent invalid upsert
and invalid removal across two fresh rounds with valid records and a later deletion behind them (the
later deletion is still delivered); a raw-cap record at page-size 1.

**R3-2 Three distinct size outcomes (R2-2).** `E_CALL_OUTPUT_TOO_LARGE` carries `reason`:
`raw-too-large` (Graph's answer exceeded the operation's read cap — retry the same cursor with a
smaller `page-size`; terminal only at 1), or `output-too-large` with `outputBytes: <n>` (the projected
answer, cursor included, exceeded the caller's `maxOutputBytes` — retry the identical request with a
larger budget, up to 1 MiB; this never implies a bad record). Command pages recommend
`maxOutputBytes: 1048576` for delta and get-message. Both commands get the same split. Existing rows
keep today's undifferentiated `E_CALL_OUTPUT_TOO_LARGE` (no `reason`), pinned. Tests: 1 KiB budget →
`output-too-large` then success at 1 MiB, for both commands; an empty delta page whose cursor exceeds
1 KiB.

**R3-3 The cursor's account is verified on every entry point (R2-4).** New `Operation.binds_account:
bool` (true for `folder.list-messages` only, part of its v2 canonical form, absent elsewhere so no old
digest moves). A cursor execution context `{op_sha, folder, binding}` is built after identity is
verified and before the data request, and is what both checks incoming cursors and mints the outgoing
one: on `agent call` the binding is the request's `bindingId` (lowercased), compared to the cursor
before any network activity and then re-verified live as today; on the workflow and `agent invoke`
paths (`run_reviewed`), an operation with `binds_account` runs `verify_principal` (one `/me`, the same
code the call path uses) and derives the binding from it — so a reconnected slot that now belongs to
another account refuses the old cursor (`cursor-context`) rather than mixing streams. Old rows'
workflow/invoke behaviour (no identity step) is unchanged and pinned. Tests: changed binding → zero
data requests on all three paths; same binding succeeds; reconnect-to-another-account via workflow and
invoke.

**R3-4 Plumbing named (R2 compatibility notes).** `Projection::apply` keeps `Option<Json>` for old
rows; the three new projections return `Result<Json, CallFailure>` through a new
`Projection::apply_checked` so their specific reasons survive (old rows still map `None` to
`response-shape`). `get_bounded` gains an `ErrorBody` policy argument: `Ignore` (every old row and the
identity/organization reads — status only, unchanged) or `ReadCode` (delta only: up to 16 KiB, parse
`error.code`, allowlist); a failed, malformed or oversized error read keeps the received status and
yields no code. The `page-size` of a continuation request goes into the `Prefer` header only, never
the query, and is exempt from the "no other input with a cursor" rule (which then covers only inputs
that would change the query: there are none left after R-1, so the rule reduces to "the cursor's
folder equals `folder-id`").

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
- **L6** (rev 5) `folder.list-message-ids` with `received-after` = 90 days ago → every message of the
  test folder received since then, newest first, ids equal to the delta's and `get-message`'s. With
  `page-size: 2`, page by `received-before` = the oldest `received-at` until `more-available: false`;
  the union equals the one-page listing. After L3: the moved-out and deleted messages are absent and
  the moved-in one is present. Record whether Graph accepted the combined filter and order.
- **L7** a wrong binding → `E_BINDING_CHANGED`, zero data requests (existing behaviour, re-run).
- Record the observed token alphabet (L2) and whether Graph ever sent both links.

## 8. Questions (only if the code cannot settle them)

None for AWARE. FloLess-side questions (one folder or with subfolders; attachments; how much body is
kept) belong to the slice-1b plan; this read supports all of them: `folder.list` walks children,
attachments are named, the body cap is 65,536 chars with `unique-text` alongside.

## 9. Rev 5 — the folder snapshot read

**Operator: Pawel 2026-11-06 — snapshot, 90 days.** The consumer (FloLess R6.8 slice 1b) tried to
keep a copy of a folder from the change stream alone. Four plan rounds kept finding defects, because
the stream has no date window or order, can repeat ids, and is complete only after walking the
folder's whole history. So the consumer now keeps its copy from the folder's **current content**
within a received-date window, and the change stream stays in AWARE unchanged for later use.

`outlook.mail.folder.list-message-ids`:
- `GET /me/mailFolders/{folder-id}/messages`, a plain listing, not delta. Graph's 5,000-message cap
  and filter-in-token rules for filtered delta (R-1) do not apply, and mail moved in is listed by its
  received date.
- **Inputs:**
  - `folder-id` is an `outlook-id/1`;
  - `received-after` (required) and `received-before` (optional) are `graph-utc-timestamp/1`,
    rendered by the code as one `$filter` (`receivedDateTime ge A and receivedDateTime le B`);
  - `page-size` is `$top` 1..1000, default 500.
- **Fixed parts:**
  - `$select=id,receivedDateTime,lastModifiedDateTime,isDraft`;
  - `$orderby=receivedDateTime desc` (Graph requires the ordered property to lead the filter);
  - `Prefer: IdType="ImmutableId"`, so ids equal the delta's and `get-message`'s;
  - an 8 MiB raw cap.
- **Refusals:** `received-before` earlier than `received-after`, since an empty window would read as
  an empty folder.
- **Paging by key, not `$skip`.** The caller sends `received-before` = the oldest `received-at` of
  the last page, so a deletion during paging cannot shift a message out of every page. The bound is
  inclusive, so boundary messages repeat and the caller keeps ids once.
- **Ties (rev 5b; operator: Pawel 2026-11-06 — "(a) for both").** A full page whose entries all share
  one `received-at` T cannot advance by date. The new `skip` input (`$skip` 0..100,000, always sent,
  default 0) pages within the window of that one instant (`received-after` = `received-before` = T).
  The caller then continues with `received-before` = T − 100 ns, the finest `graph-utc-timestamp/1`
  unit, so T is no longer included. `$skip` is positional only inside one instant, and the doc says a
  move during that walk is caught by a later listing.
- **Tie offsets from Graph's own count (rev 5c; operator: Pawel 2026-11-06 — "Yes, route A").** The
  consumer's plan round 6 showed that offsets computed by the caller (0, N, 2N, …) skip messages
  when Graph sends a short page or the page size changes mid-walk. So the listing returns
  `next-skip`, only while `more-available`: Graph's own `$skip` from its next link when it moves
  past the one sent (Codex review: a mailbox scan can count items it does not return, so Graph's
  offset can exceed the page's count), otherwise the `skip` sent plus every entry Graph returned
  (omitted ones too), which can repeat but never skips. `limit-reached: true` replaces it past
  100,000, and an empty page with no later offset has neither. `outlook.mail.folder.list` shares
  the same `with_next_skip`. A tie walk is entered whenever a
  page's oldest `received-at` equals the `received-before` sent, full page or not, and a tie at
  `received-after` ends the window. A walk that ends at `limit-reached` is not complete.
- **Output:** `{messages: [{id, received-at, last-modified-at, draft}], more-available, next-skip?,
  limit-reached?, omitted}`.
  There is no local cut. Dates are exact UTC instants (`utc_timestamp`) or withheld, and absent or
  `null` stays as sent.
- **Digest:** `InputMap.filter_op` enters the canonical entry as `operator` only when set, so every
  earlier row keeps its digest (pinned by `every_row_shipped_before_763_keeps_its_digest`). A
  `since` still renders `gt`.

Tests: wire (one filter, order, bounds, refusals), digest (`operator` only on this row, `since`
unchanged), projection (kept, withheld, null, absent, omitted, no cut; `next-skip` after a short page with an
omitted entry, none on the last page, `limit-reached` past 100,000), a call with `Mail.ReadBasic`
and the immutable-id header, raw cap, and no data request without a mail scope. Live: §7 L6.
