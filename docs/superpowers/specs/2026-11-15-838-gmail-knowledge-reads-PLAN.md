# google-workspace: Gmail reads for a Knowledge source (#838) — PLAN

Requested by floless.app R6.8 slice 3 (#2470/#2491, floless plan
`docs/superpowers/specs/2026-11-15-2470-r6-8-slice-3-gmail-PLAN.md` §2): a Gmail label or search as a
synced, versioned Knowledge source covering the last 90 days. The Gmail counterpart of the Outlook
folder source (#763/#766). Merges to `main` untagged; the operator folds it into the next release.

## 1. What exists

`gmail.search` (#810) is one page of at most 50 previews, no date bounds, no body, no label names.
It is the model: `Operation` rows in `cli/src/runtime/agent_call.rs`, the Gmail module
`agent_call_gmail.rs` (own canonical form, `read`, projection), one verified account per call
(`binds_account`), the slot rechecked before every request, one deadline for every request,
`gmail.readonly` gate (`E_CALL_SCOPE_MISSING` before any request), nothing partial.

## 2. Contract

All three: provider Google, origin `https://gmail.googleapis.com`, `any_of_scopes: [gmail.readonly]`,
`request_scopes: [openid, userinfo.email, gmail.readonly]`, `binds_account: true` (every route
verifies the account first), `canonical: "v2"` with an own canonical form per row, the slot checked
before every request, one deadline for all requests of a call, never a partial success.

### 2.1 `gmail.labels.list`

- `GET /gmail/v1/users/me/labels` (unpaged). No inputs.
- Output `{labels: [{id, name, type}], omitted, complete}`. `type` is `system` or `user` (anything
  else: `type` absent, named in that label's `truncated`). `id` must be `gmail-label/1` (else the
  label is dropped and counted in `omitted`). `name` cut at 256 chars and named in `truncated`.
- Bounds: raw answer 2 MiB (Gmail allows 10,000 labels; 64 KiB would fail a real account). Labels
  are kept in Gmail's order while the projected answer fits the caller's `maxOutputBytes`; the rest
  are counted in `omitted`. `complete` is `omitted == 0`. Never a silent cap.

### 2.2 `gmail.list-message-ids`

- `GET /gmail/v1/users/me/messages` with `q`, `labelIds`, `maxResults`, `pageToken`,
  `includeSpamTrash=false` (fixed).
- Inputs: `label?` (`gmail-label/1`, never SPAM/TRASH), `text?`, `from?` (the #810 grammars), at
  least one of the three; `received-after` (required) and `received-before?` — exact UTC instants
  (`graph-utc-timestamp/1` grammar), at or after 1970-01-01T00:00:01Z; `received-before` earlier
  than `received-after` is refused; `page-size` 1..500 (default 500) → `maxResults`; `page-token?`
  (`gmail-page-token/1`: 1..512 of `[A-Za-z0-9._~-]`) → `pageToken`.
- `q` = `from:` term, words, `after:<s>`, `before:<s>`, single-spaced. **Conservative widening**:
  `after:` = floor(received-after in epoch seconds) − 1, `before:` = ceil(received-before) + 1, so
  the window contains every message in [after, before] whatever Gmail's inclusive/exclusive meaning
  at second precision; the caller post-filters on `received-at` from `gmail.get-message`. The
  listing carries **no dates**. The measured Gmail meaning is recorded by the live check.
- Output `{messages: [{id, thread-id}], next-page-token?, more-available, omitted}`. No local cut:
  every entry Gmail sent is kept or counted (a cut would make the next token skip messages). A
  malformed or repeated id is counted in `omitted`; a malformed thread id keeps the id and names
  `thread-id` in `truncated`. `more-available` is a non-empty `nextPageToken`; a token outside
  `gmail-page-token/1` is not returned and `next-page-token-withheld: true` is set instead.
- Raw answer cap 64 KiB (500 ids ≈ 28 KiB).
- **Failure reasons** on a listing 400/404 (after the rate-limit check): when a `label` was sent,
  one `GET /gmail/v1/users/me/labels/{label}` probe (same deadline, slot rechecked): a probe 404 is
  `reason: "label-not-found"`; a probe 200 with a `page-token` sent is `"page-token-invalid"`;
  otherwise `"request-invalid"`. Without a label: a `page-token` sent is `"page-token-invalid"`,
  else `"request-invalid"`. Status and `phase` kept. A rejected token never reads as a deleted label.

### 2.3 `gmail.get-message`

- `GET /gmail/v1/users/me/messages/{id}?format=full`; `id` is `gmail-id/1`.
- Output: `id`, `thread-id`, `label-ids` (always present) + `label-ids-complete` (false when
  Gmail's value was not an array, held an invalid id, or exceeded 500), `received-at`
  (`internalDate`), `from {address, name}`, `to[]`, `cc[]` (each ≤ 100, rest in
  `recipients-omitted`), `subject`, `text`, `text-source` (`text/plain` | `text/html`),
  `text-state` (`complete` | `truncated` | `none` | `external-not-fetched` | `undecodable`),
  `truncated`, `attachments[{name, type, size}]` (≤ 50), `attachments-omitted`,
  `attachments-state` (`complete` | `partial`).
- **MIME selection, deterministic.** Walk the payload in body order, depth ≤ 16, ≤ 256 parts. A
  part with a non-empty `filename` or `Content-Disposition: attachment`, and every
  `message/rfc822` part (whose subtree is never entered), is an attachment: listed by name, never
  read for text. Text: in the first `multipart/alternative` (body order), its first `text/plain`,
  else its first `text/html`; with no such alternative (or none in it), the first `text/plain` in
  body order, else the first `text/html`. HTML → text with the existing `html_text` (scraper parse
  only: nothing executes, nothing is fetched). Body data is base64url (padding indifferent), the
  charset from the part's `Content-Type` decoded with mail-parser's decoders; an unknown charset
  or bad base64 is `text-state: undecodable`, no text. A text part behind `body.attachmentId` is
  fetched with one `GET .../messages/{id}/attachments/{attachmentId}` (raw cap 4 MiB, same
  deadline, slot rechecked); over the cap or an unusable id is `external-not-fetched`.
- Caps: raw answer 8 MiB (format=full carries no attachment bytes; 64 KiB would fail ordinary
  HTML mail — the deliberate exception to the 64 KiB rule, as `outlook.mail.get-message` uses 8
  MiB); decoded text 65,536 chars (`truncated`, `text-state: truncated`); subject 1024, address
  320, name 256, attachment name 1024, type 256. A walk cut by depth or count makes
  `attachments-state: partial` and names `parts` in `truncated`. Attachment bytes are never
  returned or fetched.
- 404 is `E_CALL_FAILED` `reason: "gone"` (status 404). Raw over cap: `E_CALL_OUTPUT_TOO_LARGE`
  `reason: raw-too-large`; projected answer over `maxOutputBytes`: `output-too-large` +
  `outputBytes`. An answer for another id: `id-mismatch`.

### 2.4 Rate limit (all Gmail calls, `gmail.search` included)

A 429, or a 403 whose error body (read ≤ 16 KiB) has an `error.errors[].reason` of
`rateLimitExceeded` or `userRateLimitExceeded`, is `E_CALL_FAILED` with `reason: "rate-limited"`,
`status`, `phase`, and `retry-after-secs` when Google sent `Retry-After` (delta-seconds, or an
HTTP-date converted, clamped 0..86400). A plain 403 stays the bare status. No Google text is
returned. This changes only failure details, not what is sent, so `gmail.search`'s canonical form
and pinned digest do not move; the new rows name the rule in their canonical forms.

### 2.5 Principal namespace

A Google principal carries `namespace: "google-account"`, as Trimble's carries
`trimble-connect-user`. Additive: floless.app reads only `integration` and `stable_id`
(`server/chat-integration-policy.ts`, `index.ts`), and the binding id does not include it.

## 3. Delivery

- Gmail HTTP: an own bounded GET in `agent_call_gmail.rs` (no redirects, deadline, body cap, as
  `get_bounded`) that also returns the rate-limit classification and `Retry-After`; `gmail.search`
  moves onto it. `get_bounded` and every non-Gmail row are untouched.
- Manifest google-workspace 2.4.0, `minimum-cli-version: 0.179.0`; CLI bumped to 0.179.0 with
  `sync_stats.py --bump` (as #810 did). `registry-index.json` is not published here (pins a merged
  commit). floless.app still pins 2.2.0 for RFI send; unchanged by this.
- Docs: three command docs, BREAKING 2.4.0 (not breaking: additive + new failure reason), cli-spec
  section, oauth-registration mention.
- Tests (fixture-backed, raw Gmail JSON with base64url parts): paging + token charset, date-bound
  rendering, size step, 404 gone, omitted, rate-limit 429 and 403-with-reason, plain 403 not
  rate-limited, label truncation, HTML-only bodies, MIME selection order, rfc822 not read,
  external body fetch, charset, label-not-found vs page-token-invalid, digests of every earlier row
  unchanged, principal namespace.
- No live Gmail call in this PR; the live check runs later against one test label.

## 4. Revision 1 — after Codex's plan review (governs where it differs from §2)

Codex reviewed §1–3 before any code (9 findings, 4 high). Changes:

1. **Attachment bytes.** `format=full` may carry a small attachment's data inline. The guarantee is
   that attachment content is never decoded, returned or fetched (`attachments.get` is called only
   for a *body* part).
2. **Attachment subtrees are pruned whole.** Any part with a non-empty `filename`, a
   `Content-Disposition: attachment`, or type `message/rfc822` — multipart or not — is listed and
   never entered. Attachment ids for a body fetch: 1..2048 of `[A-Za-z0-9_-]`, pushed through the
   segment builder, never a provider URL.
3. **Container-aware body selection** (replaces the single-part rule; every case the single-part
   rule decides is decided the same way): a `text/plain`/`text/html` leaf is one section;
   `multipart/alternative` takes the first child whose sections are all `text/plain`, else the
   first child that has any section; `multipart/related` and `multipart/signed` take their first
   child; `multipart/mixed` and any other multipart concatenate every child's sections in body
   order (joined by a blank line). `text-source` is `text/plain`, `text/html` or `mixed`.
   `text-state` is the first that applies: `mime-cut` (the walk hit the depth/part cap — never
   `complete` or `none` then), `external-not-fetched`, `undecodable`, `truncated`, `none`,
   `complete`. Whatever sections were read are returned.
4. **Listing failure reasons are evidence, not proof.** A probe 404 is `label-not-found`. A
   page token sent (and, with a label, the probe answering 200) is `page-token-invalid` — the
   caller restarts without the token; if that fails too it reads `request-invalid`. Everything
   else is `request-invalid`. A rate-limited probe is `rate-limited` (phase `label-probe`, with
   `retry-after-secs`); a probe timeout is `E_CALL_TIMEOUT`; any other probe answer leaves
   `request-invalid`.
5. **Body-fetch failure policy.** Only the message GET's 404 is `gone`. A body fetch's 404 or
   over-cap answer is `external-not-fetched` for that section; its rate limit, timeout, transport
   failure or other status fails the call (`phase: "body"`). At most 4 body fetches per message;
   past that the section is `external-not-fetched`.
6. **Page tokens are opaque.** `gmail-page-token/1`: 1..1024 bytes of printable ASCII without
   space (0x21..0x7e), query-encoded. A returned token outside it is withheld
   (`next-page-token-withheld: true`, `more-available: true`). A caller stops on a withheld or
   repeated token.
7. **Worst-case caps.** List raw cap 256 KiB (500 entries at the 64-byte id maximum ≈ 80 KiB);
   labels raw cap 8 MiB, labels kept while the *whole serialized envelope* fits `maxOutputBytes`;
   when even the empty envelope cannot fit, `output-too-large`.
8. **Dates.** The widened window is documented as a superset; the live check must cover exact
   seconds, fractional instants, equal bounds and a message whose `Date` header differs from its
   `internalDate`. A message whose `internalDate` is missing or unreadable has no `received-at`
   and names it in `truncated`; the caller cannot post-filter it and must say so.
9. **Transport regression gates.** A 429 is `rate-limited` from its status alone; a 403 whose
   body is unreadable, oversized or malformed stays a bare 403. Tests cover both `Retry-After`
   forms, overflow and invalid values, and every secondary phase. `gmail.search`'s canonical
   builder, constants and projection are untouched; only its HTTP failure mapping gains
   `rate-limited`.