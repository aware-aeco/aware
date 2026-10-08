# #618 minimal slice — `aware agent call-capabilities` + single-shot `aware agent call` (Google Drive list-files, read-only)

Status: plan, **rev 3 (final)** (2026-10-08) after rounds 1-3, approved by Codex and non-blocking for the advisor (Codex + Fable advisor; see the LOG). Issue:
aware-aeco/aware#618. Owner ruling (Pawel, 2026-10-08, "Minimal slice"): build `aware agent
call-capabilities` and a single-shot `aware agent call`, READ-ONLY, for Google Workspace **Drive
list-files only**; bind to the exact account slot with no fallback; no token or raw credential ever
leaves AWARE; reuse the #617 probe rules. `call-status`, `call-cancel` and Microsoft 365 are OUT.

Consumer: floless.app `server/aware-adapter.ts` (`agentCallCapabilities`, `agentCall`),
`server/chat-integration-policy.ts`, `-relay.ts`, `-tools.ts`, `server/index.ts` (origin/master).

## 0. Owner rulings after rev 3 — these supersede the sections below where they differ

Pawel, 2026-10-08, relayed by the coordinator (recorded in the LOG):

1. **Scope: `drive.readonly`.** It is the scope on the `aware-aeco` consent screen and in the
   verification submission. AWARE asks for it (`connect --scopes …/drive.readonly`, the
   `scope-missing` sentence); the single code-owned allowlist (`OPTIONAL_GOOGLE_SCOPES`, shared with
   `gmail.send`) accepts `drive.readonly` and `drive.metadata.readonly`, nothing else. The code still
   calls only `files.list` with the fixed metadata field mask, pinned by
   `a_drive_readonly_slot_still_only_lists_metadata`. Every "`drive.metadata.readonly` only" below
   reads as "`drive.readonly` or `drive.metadata.readonly`"; a slot with full `drive` is
   `scope-missing`. FloLess passes `--scopes https://www.googleapis.com/auth/drive.readonly`.
2. **AI chat: disclose and allow.** Drive metadata may reach the user's AI provider through FloLess
   chat with per-read approval; the verification package says so; a security assessment may follow.
3. **Project: `aware-aeco`** (number 52384839180) holds AWARE's client and consent screen; the
   package (`docs/google-oauth-verification/aware-aeco-drive-readonly.md`, #662) targets it. The
   §12 test-project route was dropped: the live check ran on the production client in Testing.
4. **`minimum-cli-version`** is raised in the registry-publication follow-up (#664), which lands
   after the release that ships this code; this PR does not touch the registry, so no older CLI can
   install 2.2.0 from it.

Implementation notes from the code review: a stalled **refresh** in `call-capabilities` ends as a
hard `E_CALL_TIMEOUT` inside the budget (the refresh keeps its own 30 s deadline); a stalled
**identity** read ends as `unverified` inside the budget (its timeout is the remaining budget less
250 ms). Follow-ups filed: #664 (registry + minimum CLI), #666 (call-status / call-cancel / journal),
#667 (Microsoft 365 list-folder), #668 (probe refresh endpoint), #665 (`connect --list` aliased
slots); production verification #662.

## 1. What exists and what is missing (origin/main `eab4ee1b2`, v0.156.1)

- No `call*` verb. `aware agent probe` (`cli/src/runtime/probe.rs`) has the slot rule, the
  generation-before-refresh check, a bounded no-redirect `send_bounded`, fixed-sentence failures with
  code-only details, the code-only log line.
- `google-workspace` 2.1.0: `list-files` is `status: planned`; default consent is exactly
  `openid userinfo.email gmail.send`, and `gmail.send` refuses any other grant (#495). No Drive scope is
  ever granted today.
- `aware connect <integration> --oauth --scopes <a,b>` already requests **additional** scopes on top of
  the default set (`commands/connect.rs`), so an opt-in Drive grant needs no new connect code.
- `gmail.send` pins the refresh destination: it resolves one `IntegrationConfig` snapshot, refuses a
  `token_url` other than Google's, and refreshes via `ensure_fresh_with_config`. The probe's plain
  `ensure_fresh` does not (a profile file could redirect the refresh token) — a latent #617 gap, filed
  separately; this slice uses the pinned path.

## 2. Design

The Drive read is a **code-owned operation**: method, origin, path, the literal field mask, the
input→wire mapping, bounds, required scopes and output projection live in a Rust table
(`runtime/agent_call.rs`, `OPERATIONS`); only `google-workspace`/`list-files` is in it. The installed
manifest must still declare the command runnable (`mode: read`, not planned) and its digest/version are
pinned by the request, but nothing in a manifest can change where the token goes or what is sent.

The account binding is **stateless and verified live**: both verbs read the exact slot's credential and
ask Google's OpenID userinfo endpoint (code-owned URL) which account it is. `binding_id` is derived from
(integration, slot, Google `sub`), so a reconnect to a different Google account yields a different
`binding_id` and the call is refused before Drive is touched. No new persistent state, no binding store
to go stale. (Considered and rejected for this slice: capturing identity from the `id_token` at
grant/refresh into `StoredToken` — it changes the connect/refresh paths and the credential format for
every integration; and a persisted binding record — new state, new locking. Both stay possible later.)

**Drive is opt-in, not default consent** (least privilege): the default consent is unchanged. A slot
gets Drive only when connected with
`aware connect google-workspace --oauth --scopes https://www.googleapis.com/auth/drive.metadata.readonly`
(the narrowest scope that authorizes `files.list`; Google classifies it *restricted*).
`gmail.send`'s exact-set check becomes **required ⊆ granted ⊆ code-owned allowlist**, where required =
the three mail scopes and the allowlist adds only `drive.metadata.readonly` — so a Drive-enabled slot
still sends mail, and any other extra scope is still refused (its guidance text updated accordingly).
**One** code-owned allowlist serves both checks: Drive list-files is available only with exactly
`drive.metadata.readonly` granted (not `drive.readonly`/`drive`, which `gmail.send` would refuse —
a slot must never silently trade mail for Drive).

### 2a. Google scope classification (verified 2026-10-08 against Google's docs)

| Scope | Class | Can list a user's Drive? | Source |
|---|---|---|---|
| `drive.file` | Non-sensitive | No — only files the app created or the user opened with it (Picker) | [Drive API scopes][drive-scopes] |
| `drive.metadata.readonly` | **Restricted** | Yes (metadata only) | [Drive API scopes][drive-scopes] |
| `drive.readonly`, `drive.metadata`, `drive` | Restricted | Yes (and more) | [Drive API scopes][drive-scopes] |
| `drive.apps.readonly` | Sensitive | No | [Drive API scopes][drive-scopes] |
| `gmail.send` (today's consent) | **Sensitive**, not restricted | — | [Gmail API scopes][gmail-scopes] |

Consequences of a restricted scope for the **first-party** AWARE Google client: while unverified, the
"unverified app" screen precedes consent and the app is capped at 100 new users; in *Testing* status
only listed test users (at most 100) can grant it. Publishing needs Google's restricted-scope
verification, and an app that can reach restricted data "from or through a third-party server" needs a
security assessment (CASA), renewed every 12 months ([restricted scope verification][restricted],
[unverified apps][unverified]). AWARE is a local CLI that keeps tokens and Drive data on the user's
machine; whether Google's review treats it as exempt from the assessment is Google's call and is not
assumed here. Verification is not needed for personal use (under 100 users), Testing status, or an
**internal** app in a Workspace/Cloud Identity org ([when verification is not needed][not-needed]) —
which AWARE already supports as a bring-your-own OAuth profile (`~/.aware/oauth/google-workspace.yaml`,
#146): an organisation's own internal client with `drive.metadata.readonly` sees no warning and no cap.

Alternatives considered: **`drive.file` + Google Picker** is the non-sensitive route Google recommends,
but it cannot implement *list files* — it sees only files the person picked, and the Picker is a browser
widget, so it is a different feature (pick-then-read) belonging in FloLess's UI, not a CLI read; noted
as a later direction. **`drive.readonly`** is equally restricted and broader (content download), so
strictly worse for a metadata listing. **No Drive at all** leaves the owner's ruled feature unbuilt.
**Recommendation:** ship list-files on `drive.metadata.readonly` as an **opt-in** scope (default consent
stays sensitive-only, so mail-only users never meet a restricted scope); run the first-party client in
Testing with the owner as a test user now; before inviting users beyond the cap, complete
restricted-scope verification or point organisations at a BYO internal client; consider a
`drive.file` + Picker flow in FloLess as the long-term non-restricted path. The classification does not
change the design — it is why the scope is opt-in.

[drive-scopes]: https://developers.google.com/workspace/drive/api/guides/api-specific-auth
[gmail-scopes]: https://developers.google.com/workspace/gmail/api/auth/scopes
[restricted]: https://developers.google.com/identity/protocols/oauth2/production-readiness/restricted-scope-verification
[unverified]: https://support.google.com/cloud/answer/7454865
[not-needed]: https://support.google.com/cloud/answer/13464323

## 3. CLI surface

```
aware agent call-capabilities <agent> <command> [--as <alias>] [--timeout-ms <1000..60000>] --json
aware agent call <@request.json> --json
```

- `--as` is validated by `credential::is_valid_alias` (lowercase alnum, `-`/`_`, alnum at both ends),
  the probe's rule, so no valid alias can be read as an option; no `allow_hyphen_values`.
- `call` takes exactly one positional that must start with `@`; the file must be a regular file
  ≤ 64 KiB of UTF-8 JSON. No inline JSON, stdin, or other flags.
- Both print the standard envelope and then end the process with `flush_exit` (exit 0 on success, the
  code's exit status on failure), so a blocking refresh or HTTP thread still in flight can never hold
  the reply past the deadline — including an `ok:true` capability that reports `unverified` because a
  phase ran out of budget. Persistent writes: one code-only line per invocation in
  `logs/agent-call.log` (`<iso> agent-call <verb> <agent> <command> <ok|CODE>`), plus the resolver's
  existing credential maintenance (an OAuth refresh stored to the same slot; a generation written for a
  legacy credential). Nothing else.

## 4. Shared credential step (both verbs)

1. Slot = exactly `integration[.alias]` (`alias` from `--as` / the request), never another; a missing
   slot is `missing`, never the default account.
2. Resolve **one** `IntegrationConfig` snapshot (`for_integration(..).with_profile(home, alias)`) and
   require `token_url() == https://oauth2.googleapis.com/token` → else refuse (`E_CREDENTIAL_EXPIRED
   {reason:"token-endpoint"}`) with zero requests.
3. `load_token`; generation `None` (legacy metadata that could not be persisted) → `unverified` /
   `E_CREDENTIAL_CHANGED {reason:"generation-unavailable"}` — never `verified`.
4. (`call` only) stored generation ≠ request `credentialGeneration` → `E_CREDENTIAL_CHANGED` before any
   refresh.
5. `ensure_fresh_with_config(snapshot)`. On failure, re-`load_token` once: if another process refreshed
   the slot concurrently (compare-and-store conflict) and the stored token is fresh with the **same
   generation**, use it; otherwise `E_CREDENTIAL_EXPIRED`. Generation re-checked after refresh.
6. Token unusable (blank / unsendable in a header) → `missing` / `E_CREDENTIAL_MISSING`.
7. Identity GET `https://openidconnect.googleapis.com/v1/userinfo` with that access token (bounded,
   redirects 0, 64 KiB, status ≥ 300 = failure, status only). `sub` required (1..255, no control
   chars); `email` used as `presentation_label` only when `email_verified === true`.

## 5. `call-capabilities` → `data` (schema `aware.agent-call-capability/v1`)

```json
{
  "schema": "aware.agent-call-capability/v1",
  "agent": "google-workspace", "command": "list-files",
  "installedVersion": "2.2.0",
  "manifestSha256": "<64 hex of installed manifest bytes>",
  "operationSha256": "<64 hex of the canonical code-owned operation>",
  "integration": "google-workspace", "alias": null,
  "transport": "rest", "effect": "read", "cancellation": "none",
  "inputs": [
    {"source":"query","location":"query","name":"q","scalar":"string","required":false,"default":null,
     "minimum":null,"maximum":null,"maxBytes":2048},
    {"source":"page-size","location":"query","name":"pageSize","scalar":"integer","required":false,
     "default":100,"minimum":1,"maximum":100,"maxBytes":null}
  ],
  "credential": {
    "status": "verified", "integration": "google-workspace", "alias": null,
    "binding_id": "<uuid v8>", "revision": 1, "credential_generation": "<uuid v4>",
    "principal": {"integration": "google-workspace", "stable_id": "<Google sub>"},
    "presentation_label": "ada@example.com", "verified_at": 1791460000000
  },
  "available": true, "unavailableReason": null, "unavailableCode": null
}
```

- `credential` is exactly one of FloLess's `AwareBindingStatus` variants: `missing`
  `{status, integration, alias}`; `unverified` `{status, integration, alias, revision: 0}` (identity
  unreadable: offline, 401, timeout, no `sub`, generation unavailable, refresh failed, token endpoint
  not pinned); `verified` as above. `stale` is never emitted in this slice.
- `binding_id` = first 16 bytes of `sha256("aware.agent-call-binding/v1\0" + integration + "\0" +
  slotAccount + "\0" + sub)`, UUID-formatted with version nibble 8 and the RFC 4122 variant (passes
  FloLess's regex). `revision` is always 1 here; a different principal is a different `binding_id`.
- `available` true only when: the command is declared runnable, the operation is in `OPERATIONS`, the
  credential is `verified`, the manifest's `auth` is `scheme: oauth2` with `secret: google-workspace`
  (exactly the integration — no other handle, no baked alias), and the granted scope contains the
  allowlisted `drive.metadata.readonly`. Else `available:false` with a fixed `unavailableReason` and `unavailableCode` ∈
  `auth-invalid | command-planned | scope-missing | credential-missing | identity-unverified |
  credential-expired | generation-unavailable`. `scope-missing`'s sentence names the exact reconnect
  command, including `--as=<alias>` for an aliased slot.
- No `reviewed` field and no registry fetch: FloLess calls this per slot on every Integrations render
  and before every chat read, and the operation is code-owned, so manifest trust is not what protects
  the credential (the request pins `manifestSha256` regardless). Reported deviation from "reuse
  `reviewed`"; it remains on `agent probe` / `agent describe`.
- Hard errors (`ok:false`) only when there is nothing to describe: `E_AGENT_NOT_INSTALLED` (7),
  `E_CALL_UNSUPPORTED` (3, not in `OPERATIONS` — e.g. microsoft-365/list-folder), `E_CALL_INVALID`
  (3, manifest unreadable / folder-id mismatch / runtime requirement), `E_CALL_ALIAS_INVALID` (3),
  `E_CALL_TIMEOUT` (4). FloLess already turns a failed capability call into the slot's
  `unavailableReason`; a capability object with fabricated digests would instead fail FloLess's own
  shape check as a contract mismatch, which is why these stay errors.
- Bounded by `--timeout-ms` (default 15 000).

## 6. `call` request (schema `aware.agent-call/v1`, FloLess's `AwareAgentCallRequest`)

Top-level unknown fields refused. Rules: `invocationId` UUID; `agent`/`command` ids;
`expectedAgentVersion` 1..128; `expectedManifestSha256`/`expectedOperationSha256`/`inputsSha256` 64
lowercase hex; `executionTransport` = `"rest"`; `operationSchemaVersion` = 1; `integration` id; `alias`
null or valid alias; `bindingId` UUID; `bindingRevision` integer ≥ 1; `credentialGeneration` 1..128
`[A-Za-z0-9-]`; `inputs` object of string / safe integer / boolean; `owner` object (opaque, ≤ 4 KiB);
`connectionRevision`, `approvalReceiptId` strings ≤ 256; `timeoutMs` 1000..60000; `maxOutputBytes`
1024..1048576. `owner`, `approvalReceiptId`, `connectionRevision` are **correlation, not permission**:
AWARE does not enforce FloLess's single-use approval; a repeated request (same `invocationId`)
re-reads Drive — read-only, so harmless; dedup arrives with `call-status`.

### Order of checks — nothing credential-bearing before step 7

1. Read + parse + shape-validate → `E_CALL_REQUEST_INVALID {field}`.
2. `(agent, command)` in `OPERATIONS`, `integration` equals the operation's → `E_CALL_UNSUPPORTED`.
3. Installed manifest bytes: SHA-256 == `expectedManifestSha256` before parsing → `E_CALL_CHANGED
   {reason:"manifest"}`; parse; folder id == manifest id; `version` == `expectedAgentVersion` →
   `{reason:"version"}`; runtime requirement; manifest `auth` is `oauth2` / `google-workspace` exactly →
   else `E_CALL_INVALID {reason:"auth"}`; command declared, `mode: read`, not planned →
   `E_CALL_UNAVAILABLE {reason:"command-planned"}`.
4. `operationSha256` == `expectedOperationSha256` → `E_CALL_CHANGED {reason:"operation"}`.
5. Inputs: every key is a mapped source (`query`, `page-size`) with the right type and bounds
   (`query` string ≤ 2048 bytes, no NUL; `page-size` integer 1..100) → `E_CALL_INPUT_INVALID {input}`
   (checked before the digest, so key order can never matter); then canonical JSON (sorted keys,
   `JSON.stringify`-compatible) SHA-256 == `inputsSha256` → `E_CALL_REQUEST_INVALID
   {field:"inputsSha256"}`, pinned against vectors generated by FloLess's own algorithm in node.
6. Both code-owned URLs' origins ∈ `IntegrationConfig::call_origins()` (code-owned: google-workspace →
   `https://www.googleapis.com`, `https://openidconnect.googleapis.com`) → `E_CALL_ORIGIN_NOT_ALLOWED`.
7. §4 credential step (codes `E_CREDENTIAL_MISSING|EXPIRED|CHANGED`, exit 6); no Drive scope →
   `E_CALL_SCOPE_MISSING` (6); identity failure → `E_CALL_IDENTITY_FAILED {status?}` (4); derived
   `binding_id` ≠ `bindingId` or `bindingRevision` ≠ 1 → `E_BINDING_CHANGED` (6). Zero Drive requests
   so far.
8. **Replacement boundary**: immediately before the Drive request, re-`load_token` the slot; it must
   still exist with the same generation → else `E_CREDENTIAL_CHANGED {reason:"replaced"}`. (A
   concurrent same-generation refresh is fine: same grant, same account.) The window after this check
   is the inherent one of any bearer call and is documented.
9. Drive GET `https://www.googleapis.com/drive/v3/files` with exactly `pageSize=<n>`,
   `fields=nextPageToken,incompleteSearch,files(id,name,mimeType,size,modifiedTime)` and `q=<query>`
   when given; headers `Authorization: Bearer` + `Accept: application/json` only; redirects 0;
   body cap = `maxOutputBytes`. Status ≥ 300 → `E_CALL_FAILED {status}` (4); transport →
   `E_CALL_FAILED {reason:"transport"}`; over cap → `E_CALL_OUTPUT_TOO_LARGE`; deadline →
   `E_CALL_TIMEOUT`; not an object with a `files` array → `E_CALL_FAILED {reason:"response-shape"}`.
10. Project (§7) and return.

**Deadline.** `timeoutMs` is the response deadline for the whole verb. Before each network phase
(refresh, identity, Drive) the remaining budget is computed; a phase never starts once it is spent, and
each HTTP request carries its own ureq overall timeout = the remaining budget (refresh keeps its own
fixed 30 s `REFRESH_DEADLINE`, which may exceed the budget; the budget check before the identity phase
and `flush_exit` are what bound the reply). The outer `tokio::time::timeout` returns `E_CALL_TIMEOUT`; the verb then
exits the process (`flush_exit` / `process::exit`), which ends any blocking thread still in flight. The
call spawns no child process, so the probe's tree-kill has no tree here; the bound is tested with a
never-answering fixture and a "no Drive request after the identity phase spent the budget" case.

Every failure is `ok:false` with a code, AWARE's own fixed sentence, and `details` of codes / status /
counts / field names only — never a body, header, token, `q` value or Google message.

## 7. `call` → `data` (schema `aware.agent-call-record/v1`, FloLess's `AwareAgentCallStatus`)

```json
{
  "schema": "aware.agent-call-record/v1",
  "invocationId": "…", "requestDigest": "<sha256 canonical request>",
  "correlationSha256": "<sha256 canonical {invocationId, owner, connectionRevision, approvalReceiptId}>",
  "bindingId": "…", "requestedBindingRevision": 1, "credentialGeneration": "…",
  "resolvedBindingRevision": 1, "state": "completed", "admittedAt": 0, "updatedAt": 0,
  "cancellationRequested": false, "cancelledAfterDispatch": false,
  "httpStatus": 200, "outcome": "ok", "payloadPresent": true, "resultExpired": false,
  "result": {
    "schema": "aware.agent-call-result/v1", "invocationId": "…", "requestDigest": "…",
    "correlationSha256": "…", "bindingId": "…", "bindingRevision": 1, "resolvedBindingRevision": 1,
    "credentialGeneration": "…", "httpStatus": 200, "outcome": "ok",
    "body": {"files": [{"id": "…", "name": "…", "mime-type": "…", "size": 1234, "modified-time": "…"}],
             "more-available": false, "incomplete-search": false},
    "cancelledAfterDispatch": false
  }
}
```

The body is a **projection**: per file only `id` (required, ≤ 256), `name` (≤ 1024), `mimeType` →
`mime-type` (≤ 256), `size` (optional — Google-native docs have none; int64 string → number when
≤ 2^53, else omitted), `modifiedTime` → `modified-time` (optional, ≤ 64); control chars removed; a file
without a usable `id` is dropped; at most `page-size` entries. `more-available` = a non-empty
`nextPageToken` was present (a short page can still have more); `incomplete-search` mirrors Google's
flag. Only `state:"completed"` is ever returned with `ok:true`.

## 8. Manifest, workflow path, registry

- `google-workspace` 2.1.0 → **2.2.0**: `list-files` loses `status: planned`, gains `mode: read`,
  `method: GET`, `path: https://www.googleapis.com/drive/v3/files`, the projected output schema;
  `requires.network` adds `https://www.googleapis.com`; command doc + BREAKING/notes updated.
  `minimum-cli-version` is bumped to the releasing CLI version **in the registry-publication PR**
  (this branch's own build is 0.156.1 and would refuse an unreleased pin).
- Default consent unchanged (§2); `gmail.send` scope check generalized (§2).
- Workflow dispatch: `RestInvoker` routes `google-workspace`/`list-files` to the same executor with the
  §4 credential step (pinned token endpoint). The manifest's `auth` must be `scheme: oauth2` and its
  `secret` exactly `google-workspace` or `google-workspace.<valid alias>` (that baked alias is the slot);
  any other scheme or handle — another registered integration, a raw handle — is refused before any
  credential is read. The command must be declared runnable/`mode: read`; same input bounds, scope
  allowlist and projection; fixed bounds **30 s** for the whole read (per-phase budgets as §6) and a
  **1 MiB** body cap; no identity step (no binding in a workflow). The node's result is bounded at 30 s;
  a refresh thread stalled past that lingers until `aware app run` exits (harmless, noted).
- Registry: after merge, a follow-up PR publishes `google-workspace@2.2.0` pinned to the squash commit
  (#619 → #620 pattern) with the minimum-cli-version bump. Until an AWARE release + that publication,
  registry installs keep 2.1.0 and capabilities reports `command-planned` — stated in the PR.

## 9. FloLess contract alignment

Matches today with no FloLess change: argv; every capability field `assertCapabilityShape` /
`verifiedCredential` / the UUID checks read; the request it writes; the record fields
`chat-integration-tools.ts` reads (`state`, `result.httpStatus/outcome/body`). FloLess-side changes to
report (none blocking the merge):
1. **Drive needs an opt-in grant**: FloLess's Google connect must offer/pass
   `--scopes https://www.googleapis.com/auth/drive.metadata.readonly` when the person wants Drive in
   chat; otherwise capabilities says `scope-missing` (rendered as its existing reconnect ask).
2. `call-status` / `call-cancel` do not exist yet: FloLess must treat a failure of those verbs as
   "unsupported", not as a call outcome.
3. `unverified` carries `revision: 0`.

## 10. Tests (all in `cargo test`; local fixtures, keyring disabled, temp homes)

Unit (`runtime/agent_call_tests.rs`, fixtures injected through a test-only endpoint/allowlist seam;
production builds use only the code-owned constants): one test per refusal code; request grammar
(unknown field, each bad field); digest vectors from node (`{}`, `{"page-size":100}`, a control-char +
non-ASCII query) and an unknown key refused before the digest; slot default vs aliased; **no fallback**
(alias slot empty, default holds a token → refused, zero requests); origin pin (production allowlist +
fixture URL → refused, zero requests); redirect (302 → second fixture sees nothing); token
non-disclosure (fixture echoes the Authorization header in an error body and in an extra 200 field →
envelope, details, log line carry neither token nor echo); body cap; 401/403/500 status-only;
never-answering server within deadline; budget spent by identity → zero Drive requests; generation
changed before any request; generation `None` → never verified; principal changed → `E_BINDING_CHANGED`
with zero Drive requests; slot replaced after identity (fixture handler rewrites the slot) →
`E_CREDENTIAL_CHANGED {replaced}` with zero Drive requests; token_url override → refused, zero
requests; concurrent refresh conflict with same generation → proceeds; two concurrent capability
calls on one expired slot both `verified`; scope missing; projection (native doc without size/
modifiedTime, control chars, bounds, size conversion, `nextPageToken` on a short page →
`more-available:true`); binding-id format passes FloLess's regex; capability variants
missing/unverified/verified; `~/.aware` diff around a call = only `logs/agent-call.log` (plus a legacy
generation the resolver may persist); an `unverified`-by-budget capability arrives within budget + 1 s
with a never-answering token endpoint; a `drive.readonly`-only slot is `scope-missing`; workflow path
honours a baked alias, refuses another integration's or a raw handle, refuses a planned command, and is
bounded (stalled and oversized fixtures). `gmail.send` accepts the legacy set and legacy +
`drive.metadata.readonly`, refuses any other superset.
Integration (`tests/agent_call.rs`): binary envelope + exit codes; `--as --json` / `--as -x` refused
as invalid alias or usage error, never a different slot; `call` without `@`; shipped manifest validates,
`list-files` is `mode: read` and declares the Drive origin.
Mutation checks (recorded in the LOG): `ensure_fresh` instead of `_with_config`; drop the generation
check; drop the replacement re-check; drop the origin check; allow redirects; default-slot fallback;
skip the identity comparison; verified with a `None` generation; return the raw body. Each must turn at
least one test red.

## 11. Out of scope / follow-ups (filed on aware-aeco/aware, linking #618)

`call-status` + `call-cancel` + durable invocation journal; Microsoft 365 `list-folder`; page-token
continuation; the probe's unpinned refresh endpoint (#617 gap, §1).

## 12. Live verification (owner-run) and what stays unverified

**Production stays sensitive-only.** The published first-party Google project (client
`52384839180-…`) does **not** get `drive.metadata.readonly`: adding a restricted scope there triggers
Google's restricted-scope verification. That verification (demo video, security assessment) is
tracked as **aware-aeco/aware#662** (owner: Pawel) and must land before Drive list-files is offered to
users of the first-party client. Nothing in this slice depends on it: the code takes the consent and
the scope set from the resolved `IntegrationConfig` and the slot's granted scopes, never from the
production client id.

**Test project (Testing status, owner as test user).** A separate Google Cloud project with: the
Google Drive API enabled; an OAuth consent screen in *Testing* with Pawel's account as a test user and
the scopes `openid`, `.../auth/userinfo.email`, `.../auth/gmail.send`,
`.../auth/drive.metadata.readonly`; a *Desktop app* OAuth client. AWARE uses it through the existing
environment overrides (`auth/config.rs`: `AWARE_OAUTH_GOOGLE_CLIENT_ID`,
`AWARE_OAUTH_GOOGLE_CLIENT_SECRET`, resolution profile > env > bundled); the token endpoint stays
Google's, so the call's token-endpoint pin holds. The secret is read only from the environment — never
written to a commit, log, fixture, PR or message.

**Steps** (a throwaway `AWARE_HOME` and a build of this branch; the env vars must stay set for every
step, because a refresh uses the same client):

1. `export AWARE_HOME=$(mktemp -d)`; set the two `AWARE_OAUTH_GOOGLE_*` variables in the shell from the
   owner's store (not on the command line of a logged session).
2. `aware agent install <worktree>/20-agents/aeco/cross-cutting/google-workspace` (2.2.0, list-files runnable).
3. `aware connect google-workspace --oauth --scopes https://www.googleapis.com/auth/drive.metadata.readonly`
   — sign in as the test user; accept the unverified-app screen.
4. `aware --json agent call-capabilities google-workspace list-files` → expect `credential.status:
   "verified"`, the account email as `presentation_label`, `available: true`.
5. Build a request file from those values (`expectedAgentVersion`, `expectedManifestSha256`,
   `expectedOperationSha256`, `bindingId`, `bindingRevision: 1`, `credentialGeneration`, inputs e.g.
   `{"page-size":5}` with `inputsSha256` = SHA-256 of `{"page-size":5}`) and run
   `aware --json agent call @req.json` → expect `state: "completed"` and the projected file list.
6. Negative live checks: the same request with `--as`-style alias `alias:"other"` → refused with no
   fallback; connect a second Google account into the default slot and re-run step 5 →
   `E_BINDING_CHANGED`; a slot connected *without* `--scopes` → `scope-missing` / `E_CALL_SCOPE_MISSING`.
7. `rm -rf "$AWARE_HOME"`; unset the variables.

Until the owner runs these, the live Google path (consent with the restricted scope, the real
`files.list` and userinfo answers, refresh against the test client) is **unverified**; every AWARE-side
rule is covered by the local-fixture tests in §10.
