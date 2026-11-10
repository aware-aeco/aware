# google-workspace `gmail.search`: a reviewed, account-bound Gmail read (PLAN)

Requested by floless.app R6.8 (#2470, #2491): FloLess wants a Gmail label or search as a Knowledge source, the
Gmail counterpart of microsoft-365's `outlook.mail.search` (#709/#763). This plan adds that read to AWARE. It merges
to `main` and is **not** tagged: the operator folds it into the next AWARE release.

## 1. What exists and what it costs

- `gmail.search` is in the google-workspace manifest at 2.2.0 as `status: planned`, with raw `query` (Gmail `q`) and
  `max-results`. It is not runnable.
- The reviewed-read machinery (`cli/src/runtime/agent_call.rs`) already serves Google `list-files`: the `OPERATIONS`
  allowlist, account pinning (`load_slot`, `TokenPin::Exact(GOOGLE_TOKEN_URL)`, `verify_principal` on OpenID `sub`,
  `check_not_replaced`), bounded GETs (`get_bounded`: no redirects, body cap, deadline), output caps and code-only logs.
- **Three non-code facts this plan must change, deliberately:**
  1. **Scope.** Gmail search needs `https://www.googleapis.com/auth/gmail.readonly`. (`gmail.metadata` refuses `q`.)
     AWARE today never requests it, and tests pin that (`google_mail.rs` least-privilege test,
     `cli/tests/google_gmail_send_contract.rs` doc assertions). It is a **restricted** scope: per #662 (owner ruling
     2026-10-23) restricted-scope verification for FloLess's Gmail use belongs to FloLess's own Google project;
     `aware-aeco`'s client stays in Testing.
  2. **Host.** `gmail.googleapis.com` is not on the code-owned Google origin list (`cli/src/auth/config.rs`
     `call_origins_for`). Only `www.googleapis.com` and `openidconnect.googleapis.com` are.
  3. **Version pin downstream.** floless.app pins google-workspace at exactly 2.2.0 + digest (`server/rfi-delivery.ts`).
     Installing 2.3.0 makes FloLess's RFI Gmail send say "update agent" until the floless side moves the pin. That
     floless change is the follow-up PR, not this one.

## 2. Contract

`aware agent call google-workspace gmail.search` (and the invoke/workflow routes through `run_reviewed`):

**Inputs (typed, never caller-written query language):**

| Input | Type | Rule |
|---|---|---|
| `text` | string | 1-16 words; each word only letters, digits, `.`, `_`, `-`, `@`; no word starting with `-`; no word equal (case-insensitive) to `OR`, `AND`, `AROUND`; no `:`, `*`, quotes, braces or brackets. Same rule as `outlook.mail.search`'s `SearchFormat::Words`, plus Gmail's own operator words |
| `from` | string | one address (same `SearchFormat::FromAddress` rule) |
| `label` | string | one Gmail label id: system (`INBOX`, `SENT`, `IMPORTANT`, `STARRED`, …) or user (`Label_<digits>`); `^[A-Za-z0-9_-]{1,64}$`. Sent as `labelIds`, never inside `q` |
| `page-size` | number | 1-50, default 25 |

At least one of `text`, `from`, `label` is required; given several, a message must match all.

**Request (code-rendered):**
1. `GET https://gmail.googleapis.com/gmail/v1/users/me/messages?q=<rendered>&labelIds=<label>&maxResults=<n>`,
   where `<rendered>` is `from:<addr>` (if given) followed by the words joined by single spaces. Nothing the caller
   writes reaches `q` except validated words and one validated address.
2. For each returned id, in order: `GET …/users/me/messages/<id>?format=metadata&metadataHeaders=From&metadataHeaders=Subject&metadataHeaders=Date`.
   At most `page-size` such reads; the slot is re-proven held (`still_held()`) before each, the same as the Trimble
   multi-request read (`agent_call_trimble.rs`). A 404 for an id (deleted between the two calls) drops that entry
   and counts it in `omitted`; any other failure fails the call.

**Output:**
`messages[{id, thread-id, subject, from{address,name}, received-at, preview, label-ids, web-url, truncated?}]`,
`more-available`, `omitted`.

- `received-at` from `internalDate` (ms since epoch) as RFC 3339 UTC.
- `preview` is Gmail's `snippet`, capped at 512 like Outlook's preview. No body; bodies are a separate read later.
- `web-url` is `https://mail.google.com/mail/?authuser=<verified email>#all/<id>`, so it opens in the signed-in
  account rather than `u/0`.
- Caps and the `truncated` list are the same as `message_entry` (subject 1024, address 320, name 256).
- `more-available` is true when `nextPageToken` is present or the page was cut locally. The token is never followed.
- Order is Gmail's (newest first), not relevance.

## 3. Changes

1. `agent_call.rs`:
   - an `OPERATIONS` row for `google-workspace`/`gmail.search`, `Provider::Google`, `any_of_scopes = [gmail.readonly]`,
     `request_scopes = [openid, userinfo.email, gmail.readonly]`, `canonical_v2: true`;
   - a `SearchFormat`/render path that emits `q` and `labelIds` (today `wire()` hard-codes Graph's `$search="…"`);
   - `Projection::GmailMessages` and the two-phase read in `read_data`, with a request-count bound and a per-read cap;
   - a Google v2 canonical branch, used only when `canonical_v2` is set, so `list-files` keeps its pinned digest.
2. `config.rs`: add `https://gmail.googleapis.com` to `call_origins_for("google-workspace")`. Update
   `the_production_allowlist_is_drive_and_openid_only` (renamed) to pin the new exact list.
3. `OPTIONAL_GOOGLE_SCOPES` gains `gmail.readonly`, so a slot that holds it can still send. Update the least-privilege
   test so `gmail.readonly` is accepted and anything else (e.g. `gmail.modify`, `mail.google.com`) is still refused.
   The default grant (`openid`, `userinfo.email`, `gmail.send`) does **not** change: search needs
   `aware connect google-workspace --scopes …gmail.readonly`.
4. Manifest 2.3.0: `gmail.search` becomes `mode: read`, `method: GET`, absolute path, inputs as §2 (invoke refuses
   undeclared inputs), outputs as §2, and a `permissions:` block naming the full request set. `minimum-cli-version`
   becomes 0.177.0 (the next CLI). `BREAKING.md`: an additive "(not breaking)" 2.3.0 entry.
5. Docs: `commands/gmail.search.md` rewritten from the planned raw-`q` version; `10-core/cli-spec.md` reviewed-read list
   and B1 table; `oauth-registration.md` and `skills/auth.md` (Gmail read now available on request, restricted scope,
   verification owned by the integrating app's Google project per #662); `commands/agent.rs` help text.
6. Version floor: follow #766. `cli/Cargo.toml` goes to 0.177.0 (version only, **no tag, no release**) so the manifest
   floor is satisfiable on `main` and the version-consistency tests stay green. **Open question for review:** if raising
   the crate version on `main` is considered a release step, the alternative is to leave the crate at 0.176.0 and
   the floor at 0.177.0 with the consistency test noting the pending release. The #766 pattern is the default.
7. `python scripts/sync_stats.py --write` (runnable command count +1) and `aware agent reindex`.
   `PLANNED_COMMAND_COUNT` and runnable-count pins move by one.

## 4. Tests (each fails without its change)

- Exact URLs and query pairs for: text only, from only, label only, all three; the rendered `q` byte-for-byte.
- Refusals with **zero requests sent**: no input; each forbidden word form (`OR`, `-x`, `a:b`, `"x"`, `(x)`, `{x}`,
  `*`); 17 words; a bad label id; page-size 0 and 51; an unknown input.
- Two-phase: one list request + exactly N metadata requests; the next token never followed; a 404 on one id gives
  `omitted: 1`; a 500 on one id fails the call with status only; `still_held()` is re-checked between reads (slot
  replaced mid-read → stop, nothing more sent).
- Missing `gmail.readonly` scope: nothing sent, the hint names the scope.
- A token-bearing request never goes to a host outside the new exact allowlist (attacker-sink test as in #709).
- Canonical: `list-files` digest still `aa42474f…`; `gmail.search` uses v2 and its digest moves when the rendered
  request shape changes.
- `gmail.send` still refuses a slot holding any non-optional scope; still accepts `gmail.readonly`.
- Binary tests (`cli/tests/agent_call.rs`) updated for 2.3.0.

## 5. Real E2E

Temp `AWARE_HOME`, install google-workspace 2.3.0 from the tree, `aware connect google-workspace --scopes
…gmail.readonly` (the person signs in themselves in a browser; the `aware-aeco` client is in Testing, so the account
must be one of its test users), then `aware agent call google-workspace gmail.search --input text=… --input
page-size=3` and assert real messages come back with the §2 shape. If no test-user Google account is available to
sign in, that is recorded in the PR as not verified live, and the request shape is still proven against the local
fixture server.

## 6. Out of scope

Message bodies and attachments; date windows (deferred as for Outlook until proven live); following `nextPageToken`;
threads; any write. The floless.app side (pin 2.3.0, request the scope for a Knowledge source, chat policy) is the
follow-up PR in floless.app.

## 7. Revision 2 — Codex plan review (2026-11-10), every finding accepted

Codex's verdict on revision 1 was "revise before implementation". These rules override §2-§5 where they differ.

- **R2-1 Digest binds the whole two-phase read.** A Google v2 canonical form, used only when `canonical_v2` is set,
  carries: both routes; ordered query pairs with duplicates (each `metadataHeaders`); `format`; the Gmail search grammar
  and renderer ids (`gmail-search-words/1`, `gmail-from/1`, `gmail-label/1`); the required-input rule; the
  request-count bound; every read cap; and the projection id (`gmail-messages/1`). The full canonical string and
  digest are pinned, and each security-relevant change has a mutation test that moves the digest. Every pre-existing
  operation digest and wire fixture stays pinned and unchanged, not only `list-files`.
- **R2-2 Provider ids are inputs too.** Every id from `messages.list` must be non-empty, ≤ 64 characters and
  `[A-Za-z0-9]` only. It is appended through the existing segment builder, never by string formatting. A bad or
  duplicate id is dropped and counted in `omitted` before any metadata request. A metadata response whose `id` is not
  the requested id fails the call. Metadata **attempts**, including 404s, count against the bound.
- **R2-3 One budget.** One decreasing deadline covers the list and every metadata request, with a cap on each
  response, bounded accumulation, and the existing final serialized-output check. On a timeout, a replaced account
  or a non-404 failure, the accumulated entries are discarded and the call fails; there is never a partial success.
  Tests: a slow multi-response deadline, and an aggregate-output overflow.
- **R2-4 Real E2E uses the real surfaces.** `aware connect google-workspace --oauth --scopes
  https://www.googleapis.com/auth/gmail.readonly`; then `aware agent capabilities`, a bound request file and
  `aware agent call @request.json`; then `aware agent invoke … --inputs @inputs.json`; then one workflow node.
- **R2-5 A separate Gmail grammar.** `SearchFormat::Words` (`graph-search-words/1`), its quoting, its
  terms-required rule and `wire()`'s Graph rendering stay byte-for-byte unchanged. Gmail gets its own grammar, chosen
  explicitly per operation. Gmail words follow the Graph word rule (Unicode letters and digits plus `.`, `_`, `-`,
  `@`, each word ≤ 64 characters, no leading `-`) and also refuse `OR`, `AND`, `AROUND`, `NOT`. When only a label is
  given, `q` is omitted.
- **R2-6 Missing fields are allowed.** In the output contract, `preview` (`snippet`) and `received-at`
  (`internalDate`) are optional. `internalDate` is parsed as a checked decimal int64 in milliseconds and rendered with
  millisecond precision; a malformed or out-of-range value omits the field and lists it in `truncated`. There is never
  a silent upgrade to `format=full`. The live E2E records whether metadata responses carry both fields.
- **R2-7 Header parsing.** The parser is the `mail-parser`/`mailparse` crate (whichever is already in the tree,
  otherwise `mailparse`). RFC 2047 encoded words and folded headers are decoded before caps apply. Of duplicate
  headers, the first is used. With several `From` mailboxes, the first is the sender and `from-count` gives the total.
  An unparseable header omits that field and keeps the message. Tests cover encoded words, folded headers, an empty
  subject, duplicates and several senders.
- **R2-8 `web-url`.** It is built only from the verified email that `Principal.label` already carries, passed to the
  projection through a typed context and query-encoded, and omitted when there is none. It is never `u/0` and never
  used as the identity. The doc calls it a convenience link that opens in whichever browser session holds that
  account.
- **R2-9 One account, both phases.** One captured token and one verified `sub` serve both phases. There is no reload,
  alias fallback or refresh mid-loop, and `still_held()` runs before every request. Tests: disconnect or reconnect
  before the list, between metadata reads, and on the invoke and workflow routes; nothing is dispatched after and
  nothing partial returned. A request already in flight cannot be recalled, and the doc says so.
- **R2-10 Labels.** Label ids are an opaque syntactic id (`^[A-Za-z0-9_-]{1,64}$`); the doc no longer claims a
  system-or-user grammar. `includeSpamTrash` is always sent as `false` and is digest-bound, so `SPAM` and `TRASH` as
  a label return nothing; the doc says so. List then get is not a snapshot: labels can change between the two calls,
  and the doc says so.
- **R2-11 No promises Gmail does not make.** Order is whatever Gmail returns. A metadata 404 means the message is
  unavailable and is counted in `omitted`; a list 404 fails the call. An absent `messages` is an empty result. Only a
  non-empty `nextPageToken` sets `more-available`.
- **R2-12 Version floor, as #766 did it.** `sync_stats.py --bump 0.177.0`, `Cargo.lock` refreshed, npm package
  version and docs moved, and **no tag or release dispatch**. The alternative in §3 item 6 is withdrawn. Tests: 0.176.0
  refuses the 2.3.0 manifest, and the new binary accepts it.
- **Regression matrix additions.** `gmail.send` on a slot holding send, readonly and both Drive scopes; readonly
  without send refuses to send; `list-files` on that combined grant. Attacker-sink and redirect tests on both data
  phases and on refresh; no token, query or message sentinel in any error or persistent log. The Google origin list
  stays code-owned, never controlled by profile or manifest.

## 8. Implementation notes (2026-11-10)

Where the code differs from §2-§7, or settles something they left open. Each was chosen as the most
conservative option that keeps the security rules.

- **Where it lives.** The Gmail read is a child module, `cli/src/runtime/agent_call_gmail.rs`, like
  Trimble's; its tests are `agent_call_gmail_tests.rs`, a child of the `list-files` tests so they share
  the Google fixture. `read_data` hands Gmail its own two-phase reader; every other row's path is
  untouched.
- **The verified account reaches the projection as a type.** `read_data` takes
  `Option<VerifiedAccount { binding, label }>` instead of a bare binding. The row sets
  `binds_account: true`, so the direct call, `aware agent invoke` (`run_reviewed`) and a workflow node
  all verify the account first; `run_reviewed` now also keeps `Principal.label` (the verified email)
  for `web-url`. Gmail refuses to run without that context (`E_BINDING_CHANGED`), so no route can
  skip it.
- **R2-7 parser.** `mail-parser` was already in the lock file (a dev-dependency, 0.11.9); it moves to
  `[dependencies]`, so no new crate enters the tree. Each header is rebuilt as one header line first:
  a CRLF followed by a space or tab is unfolded; any other CR, LF or control character refuses the
  value (it could start a second header). An **empty** header value counts as absent, not as
  unreadable, so nothing is named for it; an unreadable value omits the field **and** names it in
  `truncated` (R2-7 said only "omit"; naming it is additive and tells the reader why it is missing).
  The same goes for a malformed `threadId` and for label ids that fail `gmail-label/1` (the rest are
  kept, at most 100). `from-count` appears only when there is more than one sender.
- **The Date header** is requested, as §2 specifies, so the digest binds the full metadata query,
  but it is not projected: `received-at` comes only from `internalDate` (R2-6).
- **`preview`** is Gmail's `snippet` with HTML entities decoded through the existing `html_text`
  (Gmail sends the snippet HTML-escaped), capped at 512 characters.
- **R2-10 deviation: `SPAM` and `TRASH` are refused as a `label` input** (case-insensitive,
  `E_CALL_INPUT_INVALID`, no request), instead of being sent and "returning nothing". Whether Gmail
  honours `includeSpamTrash=false` against an explicit `labelIds=SPAM` is not something this work
  could verify live, so the input closes the question; the refused list is part of the digest. Label
  ids in the *output* are not filtered by this rule.
- **The canonical form names the header rules `mailHeaders`**, not `headers`: an existing Teams test
  forbids a `headers` key in every other row's canonical form, because there it means request
  headers.
- **`list-files` keeps its digest by keeping its own scope list.** `list-files` now names
  `DRIVE_SCOPES` (the same two values as before, so its serialized form is identical), and
  `OPTIONAL_GOOGLE_SCOPES` — the extra scopes `gmail.send` tolerates — gains `gmail.readonly`.
  `gmail.readonly` therefore never lists Drive, and a Drive scope never searches mail.
- **`gmail.send`'s refusal text cap** (`structured`, google_mail.rs) rises from 640 to 768
  characters: the least-privilege hint now names three optional scopes, and at 640 it lost the
  resolved profile path its own test requires.
- **Bounds.** `read_cap` is `None`; Gmail's own caps (64 KiB per answer, 50 metadata reads at most
  whatever `page-size` says) are in its canonical form. The caller's `maxOutputBytes` is checked
  while results accumulate, so an overflow stops the reads early, and again on the final output.
- **R2-1 evidence.** The 25 pre-existing digests are pinned from a build of `cb998fd41` (v0.176.0),
  not computed from the table under test; the Gmail canonical string and digest are pinned in full
  and the digest was computed outside Rust from that string.
- **R2-12.** `sync_stats.py --bump 0.177.0` moved `cli/Cargo.toml`, `cli-npm/package.json`,
  `README.md` and `CLAUDE.md`; `Cargo.lock` follows. `registry-index.json` is **not** changed:
  publishing 2.3.0 there pins a merged commit's tarball, so it is a follow-up after merge, as #664 was
  for 2.2.0 (#766 did not touch the registry either).
- **Not verified.** §5 / R2-4, the live Gmail run, needs a person to sign in to a Google test-user
  account, which this work could not do. Everything about the request and response shape is proven
  against the local fixture server; the real binary was run far enough to show the capability and a
  refused call without the scope. The live E2E also still has to record whether metadata answers
  carry `snippet` and `internalDate` (R2-6).
