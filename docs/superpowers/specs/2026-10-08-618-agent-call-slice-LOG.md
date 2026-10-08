# Plan Review Log: #618 minimal slice — agent call-capabilities + single-shot call (Drive list-files)

Started 2026-10-08. MAX_ROUNDS=5. Reviewers: Codex (`gpt-6-sol`, read-only, adversarial) and a
Fable advisor (read-only), run in parallel each round.

## Round 1 — Codex (gpt-6-sol, thread 01a11b01-a3ca-7d30-9035-2c339a58761c)

The plan needs revision before implementation. The strongest problems are in the credential boundary and the claims made about a completed Drive result.

1. **Refresh can send a long lived credential to an unapproved origin (§5, step 7).** The plan checks the UserInfo and Drive origins, then calls `ensure_fresh`. In [refresh.rs](/C:/Users/bimst/source/repos/aware-aeco-wt-618/cli/src/auth/refresh.rs:17), that function loads a BYO OAuth profile and posts the refresh token to its token URL. The Gmail path already validates that URL before refreshing. **Fix:** resolve one OAuth configuration snapshot, validate its token endpoint against the Google allowlist, and pass it to `ensure_fresh_with_config` in both verbs.

2. **A replaced slot can still dispatch its old token (§2, §5).** Reusing one token snapshot proves that UserInfo and Drive see the same Google account; it does not prove the slot still contains the requested credential when Drive is dispatched. A disconnect or reconnect between those requests leaves the old bearer token usable for this call. **Fix:** define and enforce the replacement boundary, with a final exact slot and generation check immediately before dispatch and a race test that replaces the slot after UserInfo.

3. **The fixed Drive response contract is underspecified (§2, §6).** `more-available` requires `nextPageToken` in the fixed `fields` selection; without it, a successful first page will be reported as complete. Google also allows a page smaller than `pageSize` while another page exists. **Fix:** pin the literal field mask including `nextPageToken` and test a short page with a continuation token. [Google’s files.list reference](https://developers.google.com/workspace/drive/api/reference/rest/v3/files/list) confirms this behavior.

4. **The stated deadline cannot stop an in-flight blocking request (§5).** `tokio::time::timeout` stops waiting for `spawn_blocking`; it does not cancel `ureq`. A call can report `E_CALL_TIMEOUT` while its Drive request remains active. **Fix:** specify timeout as a response deadline, bound the HTTP operation itself, and test that no new phase dispatches after the deadline.

5. **The manifest update omits Drive from `requires.network` (§7).** The current [manifest.yaml](/C:/Users/bimst/source/repos/aware-aeco-wt-618/20-agents/aeco/cross-cutting/google-workspace/manifest.yaml:28) declares Gmail, UserInfo, and OAuth origins, while the new runnable command calls `www.googleapis.com`. **Fix:** add the Drive origin to the manifest’s network requirement and validate the shipped manifest.

6. **Real availability is deferred to a follow-up (§7, §11).** A registry install remains at 2.1.0 with `list-files` planned until a separate publication, and the plan leaves the first-party Drive API and restricted-scope consent unverified. Unit fixtures cannot establish that FloLess can use the released slice. **Fix:** make registry publication and one real first-party account call acceptance gates, or state explicitly that this implementation remains unavailable to registry installs. [Google’s scope guidance](https://developers.google.com/workspace/drive/api/guides/api-specific-auth) classifies `drive.metadata.readonly` as restricted.

VERDICT: REVISE

## Round 1 — Fable advisor (verdict BLOCKING)

B1 unpinned `ensure_fresh` lets a profile file redirect the refresh token (gmail.send pins it);
B2 concurrent refresh compare-and-store conflict turns a parallel capability call into `unverified`;
B3 a `None` generation must never be `verified`; B4 the first-party GCP project needs Drive API +
`drive.metadata.readonly` (owner action). Design: D1 live identity on every listing is costly — prefer
id_token capture, at least drop `reviewed`'s registry fetch from capabilities; D2 widening default
consent is a least-privilege regression — use incremental scopes and required ⊆ granted ⊆ allowlist
for gmail.send; D3 workflow path must honour a baked alias and require the manifest declaration;
D4 document that approvalReceiptId is not enforced; D5 return available:false instead of hard errors
for not-installed/unsupported. Nits C1-C6, test gaps (B1-B3, baked alias, `--as --json`, unknown key
before digest, native doc without size, nextPageToken, mutants).

### Claude's response (rev 2)

Accepted: Codex 1 / Fable B1 (one pinned config snapshot + `ensure_fresh_with_config` in both verbs and
the workflow path; probe gap filed as a follow-up); Codex 2 (re-load the slot and compare generation
immediately before the Drive request, race test); Codex 3 (literal field mask incl. `nextPageToken`,
short-page test); Codex 4 (deadline is a response deadline, per-phase budgets, no phase after the
budget, process exit ends blocking threads); Codex 5 (Drive origin in `requires.network`); Codex 6
(registry publication done in this task as the #620-style follow-up PR; live call explicitly
unverified, owner action named); B2 (re-load once after a refresh failure and accept a fresh token with
the same generation); B3; D2 (default consent unchanged; Drive via the existing `connect --scopes`;
gmail.send = required ⊆ granted ⊆ allowlist); D3; D4; C2 (keys validated before the digest); C3
(no `allow_hyphen_values` — `is_valid_alias` already forbids a leading `-`); C4; C5; C6.
Partly accepted D1: `reviewed` and its registry fetch removed from capabilities (code-owned operation,
manifest digest pinned by the request). Rejected id_token capture for this slice: it changes the
connect/refresh paths and the stored credential format for every integration; the live identity read
is one bounded GET and keeps capabilities truthful (an offline machine reports `unverified`, which is
correct since Drive is unreachable too).
Rejected D5: a capability object needs real `manifestSha256`/`operationSha256`; fabricating them would
make FloLess's own shape check report a contract mismatch, while a failed capability call is already
rendered by FloLess as the slot's `unavailableReason`.

## Round 2 — Fable advisor (verdict NON-BLOCKING)

Verified rev 2's new claims against code. Two new defects: (1) the deadline claim holds only for error
envelopes; an ok:true unverified capability returns normally and the runtime waits for a blocking
refresh (REFRESH_DEADLINE 30 s is more than a 15 s budget); fix flush_exit(0) after printing.
(2) available accepted drive.readonly/drive while the gmail.send allowlist admits only
drive.metadata.readonly; fix one shared allowlist. Notes: file the FloLess --scopes issue; the
home-diff test must allow a legacy generation write.

## Round 2 — Codex

Rev 2 resolves my first-round findings. Two material gaps remain:

1. **The manifest can select a different credential in the workflow path (§8).** That path derives the slot from `auth.secret`, while the call path derives it from the code-owned integration. The existing [resolver](/C:/Users/bimst/source/repos/aware-aeco-wt-618/cli/src/runtime/invoker.rs:1575) accepts other registered integrations and raw handles; the plan does not reject those declarations before sending a bearer token to Google. **Fix:** require OAuth auth for `google-workspace` and a valid Google alias in both capability and execution checks; reject every other manifest credential handle.

2. **Workflow execution has no stated bounds (§8).** It reuses the executor, but supplies neither a deadline nor `maxOutputBytes`; §6’s request limits apply only to the FloLess call. This leaves the workflow route’s HTTP and response limits undefined. **Fix:** specify fixed workflow deadline and body cap values, apply the same per-phase checks, and test a stalled and oversized workflow response.

VERDICT: REVISE

### Claude's response (rev 3)

All four accepted: flush_exit after printing in both verbs (success too); one shared scope allowlist
(drive.metadata.readonly only); the manifest auth must be oauth2 with exactly google-workspace
(capability and call) or google-workspace[.alias] (workflow), any other handle refused before a
credential is read; workflow bounds fixed at 30 s / 1 MiB with stalled and oversized tests.
Also added section 2a at the owner's request (relayed by the coordinator): Google's current scope
classification with sources. drive.metadata.readonly is restricted; gmail.send is sensitive (the
round-1 advisor aside calling gmail.send restricted was wrong). Consequences, alternatives (drive.file
plus Picker, drive.readonly, none) and the recommendation (opt-in restricted scope; Testing plus a BYO
internal client now; Picker as a later non-restricted FloLess flow). It does not change the design.

## Round 3 — Fable advisor (verdict NON-BLOCKING)

Section 2a classification confirmed. Wording nit: REFRESH_DEADLINE is a fixed 30 s, not shorter than a
15 s budget; flush_exit and the pre-phase budget check are what bound the reply. Note: the workflow
path has no flush_exit, so a stalled refresh thread lingers until `aware app run` exits.

## Round 3 — Codex

Rev 3 closes the two material gaps from round 2. I found two details to settle during implementation:

- **Auth mismatch has no capability reason (§5).** It makes `available:false`, but the listed `unavailableCode` values have no auth case. **Fix:** add a fixed `auth-invalid` code and test it, or make the mismatch an `E_CALL_INVALID` hard error.
- **Reconnect guidance needs the selected alias (§5).** The stated command reconnects the default slot when `--as` was used, leaving the selected slot without Drive scope. **Fix:** include the validated alias in the displayed reconnect command.

These do not undermine the credential or FloLess call boundaries.

VERDICT: APPROVED

### Claude's response (final)

Applied in the plan without another round (narrow, non-blocking): `auth-invalid` is its own
unavailableCode (tested); the scope-missing sentence carries `--as=<alias>` for an aliased slot
(tested); the REFRESH_DEADLINE sentence corrected; the workflow lingering-thread note added.
Converged after 3 rounds: Codex APPROVED, advisor NON-BLOCKING.

## Owner update (relayed by the coordinator, after round 3)

Production client 52384839180 will NOT get the restricted Drive scope (it would trigger verification;
tracked as aware-aeco/aware#662, owner Pawel). Live verification uses a separate Testing-mode project
through the existing AWARE_OAUTH_GOOGLE_CLIENT_ID / _SECRET overrides. Section 12 rewritten with the
test-project setup and the live steps. No design change: nothing in the slice is tied to the
production client id; the token-endpoint pin still holds because the env override changes only the
client id and secret.

## Owner rulings after rev 3 (relayed by the coordinator)

drive.readonly (allowlist drive.readonly + drive.metadata.readonly; metadata-only code path pinned
by a test); AI chat disclosed and allowed (CASA may follow); the consent screen and client live in
Google project `aware-aeco` (52384839180), not `floless` — the test-project idea was dropped and the
live check ran on the production client in Testing mode; minimum-cli-version deferred to the
registry-publication follow-up #664. Recorded as plan section 0.

## Live verification (2026-10-08, Pawel's account, aware-aeco client in Testing)

Pawel ran `aware connect google-workspace --as uat618 --oauth --scopes .../drive.readonly` (released
0.156.1) under a temp AWARE_HOME. The token landed in keychain slot google-workspace.uat618; his
default slot was unchanged (same generation, mail-only scopes, before and after). Branch binary:
call-capabilities → verified, available; call (page-size 5) → completed, HTTP 200, 5 files, keys
{id, name, mime-type, size, modified-time}; wrong generation → E_CREDENTIAL_CHANGED; wrong binding →
E_BINDING_CHANGED; `--as nobody` → missing (no fallback); no Google token prefix in any output; five
code-only log lines. Slot disconnected and temp home removed afterwards.

## Code review round 1

pr-review-toolkit: no critical issue; (1) plan not updated for the drive.readonly ruling — fixed
(plan section 0); (2) `no_drive_request_starts_after_identity_spent_the_budget` could not fail for
its stated reason — replaced by `a_spent_budget_starts_no_phase` (an expired start makes zero
requests) and an honestly named `a_slow_identity_read_never_leads_to_a_drive_request`; (3) the
capability deadline test accepted either outcome — now two deterministic tests (stalled identity →
`unverified` inside the budget; stalled token endpoint → `E_CALL_TIMEOUT` inside the budget).
Codex (gpt-6-sol) P1: raise minimum-cli-version — deferred to #664 with the reason in plan
section 0 (this branch's own 0.156.1 build would refuse the agent; the registry is untouched).
Full suite also surfaced the migration-contract guard flagging `agent.version` in agent_call.rs:
a precise exemption (`CALL_VERB_ONLY`, one read) plus a test that the workflow route through the
same file reads no contract-ignored field, with a planted negative control.
