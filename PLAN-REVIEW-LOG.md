# Plan Review Log: make `google-workspace.gmail.send` genuinely runnable (#495)
Started 2026-09-10 Europe/Warsaw. MAX_ROUNDS=5.

## Round 1 — Codex critique

Codex returned `VERDICT: REVISE` with seventeen findings. Verbatim finding headings and requested fixes:

1. **Critical — FloLess #1090’s contract is not actually defined.** Freeze its exact JSON schema and distinguish Gmail resource ID from RFC `Message-ID`.
2. **Critical — “Require a non-empty provider ID” mishandles accepted-but-unreadable responses.** Define pre-dispatch failure, definitive rejection, confirmed acceptance, and non-retryable post-dispatch unknown outcome.
3. **Critical — “No automatic retry” does not close the durability race.** Fsync an attempt record before transmission and acceptance before success; prohibit replay of unresolved attempts.
4. **High — Manual Sent-folder reconciliation has no durable correlation key.** Persist an RFC `Message-ID` or `X-AWARE-Attempt-ID` and surface it for reconciliation.
5. **High — The existing REST error behavior is unsafe for sends.** Bypass generic response shaping and map all cases to the send taxonomy.
6. **High — OAuth remains grossly over-scoped.** Match new grants to available commands and test authorization/stored scopes.
7. **High — The OAuth compliance documentation is factually wrong.** Correct sensitive vs restricted scope documentation.
8. **High — Refresh failure can silently fall back to a stale token.** Require successful refresh before send.
9. **High — Endpoint injection could exfiltrate bearer tokens.** Compile-pin HTTPS origins/paths, reject redirects/overrides, and inject the client only for tests.
10. **High — “Build RFC 5322 MIME” is far too underspecified.** Use a maintained MIME library and independently parse/golden-test its output.
11. **High — Sender identity is unresolved.** Define and test authenticated sender behavior.
12. **High — Attachments create an undeclared local-file exfiltration primitive.** Prefer artifact handles or constrain file access and limits.
13. **Medium — The encoding path multiplies memory use.** Stream or justify JSON raw with a conservative size ceiling.
14. **High — The canary proves an output event, not exactly one email.** Verify unique correlation in Sent and recipient mailboxes and simulate accepted-then-dropped response.
15. **Medium — Trace privacy is narrower than email privacy.** Redact mail fields and retain bounded status/correlation metadata.
16. **High — Registry publication can mutate an existing version.** Publish a new immutable registry version and preserve the old digest.
17. **Medium — One-way-send rollback semantics remain invalid.** Declare/validate irreversible send semantics.

## Round 1 — response and revision

Accepted the outcome taxonomy, durable attempt journal, correlation ID, Gmail-specific response path, least-privilege scopes, strict refresh, pinned endpoints/no redirects, maintained MIME builder plus independent parser, authenticated sender resolution, size/privacy limits, stronger canary, immutable new registry version, and irreversible-command declaration.

Narrowed the command by removing attachments instead of admitting arbitrary filesystem paths; a later artifact-handle design can add them safely. Kept the established FloLess `message-id` key for compatibility but defined it explicitly as the Gmail Message resource ID and added distinct `gmail-message-id` and `rfc-message-id` keys. Scoped rollback work to declaration and validation because the repository roadmap says `aware app rollback` is not yet wired; this issue will not claim otherwise.

## Round 2 — Codex critique

Codex returned `VERDICT: REVISE` with eight remaining findings:

1. **Critical — All non-2xx responses are incorrectly classified as definitive rejection.** Treat 5xx, 408, and ambiguous intermediary responses as outcome unknown.
2. **Critical — The Bcc design contradicts the Gmail API.** Include Bcc in raw MIME because the API has no separate envelope, then verify Gmail removes it from recipient delivery.
3. **High — Optional `attempt-id` does not provide the claimed replay safety.** Require it or bind an internal ID to a stable queryable run/node key.
4. **High — Refusing reuse of an accepted attempt discards a recoverable result.** Return cached accepted output for an identical accepted attempt.
5. **High — Structured correlation fields do not fit the current error schema.** Add backward-compatible fields/details and test all serialization paths.
6. **High — Journal durability and namespace are incomplete.** Define location, safe keying, credential namespace, locking, directory/file barriers, retention, docs, and crash/concurrency tests.
7. **High — Timeouts and cancellation are unspecified.** Define deadlines and post-handoff cancellation semantics.
8. **Medium — Existing over-privileged grants remain silently active.** Fail closed and guide revoke/reconnect migration.

## Round 2 — response and revision

Accepted all eight. Required caller-stable attempt IDs; cached accepted output; request-hash conflicts; 408/5xx as unknown; Bcc in submitted MIME; optional structured error details; a documented, generation-namespaced, hashed, locked append-only outbox; explicit deadlines/cancellation ownership; and fail-closed legacy-grant migration are now part of the plan. Also made the immutable registry plan executable by including commit-archive root validation.

## Round 3 — Codex critique

Codex returned `VERDICT: REVISE` with five remaining findings:

1. **Critical — Publishing the agent without a CLI release creates version skew.** Add/enforce a minimum CLI constraint or release first.
2. **Critical — Credential generation is the wrong idempotency namespace.** Use a stable authenticated Google account identifier such as hashed OIDC `sub`.
3. **High — Cancellation remains potentially unbounded.** Add a total wall-clock deadline and slow-trickle test.
4. **High — Outbox records have no owner-only permission contract.** Require and test owner-only permissions/ACLs, failing closed.
5. **High — FloLess compatibility covers only output while required `attempt-id` breaks input.** Freeze the whole request/response fixture, identify its stable attempt source, and update the reproducer or coordinate the breaking change.

## Round 3 — response and revision

Accepted all five. Added a minimum-CLI manifest contract enforced at every lifecycle boundary; stable hashed OIDC-sub namespacing; a total 30-second deadline and slow-trickle test; fail-closed owner-only filesystem checks on Unix and Windows; and a complete #1090 fixture whose attempt ID derives from stable RFI/action identity. The current downstream has no shipped AWARE call site, so adoption is documented as a prerequisite rather than misrepresented as backward compatibility.

## Round 4 — Codex critique

Codex returned `VERDICT: REVISE` with one remaining blocker:

**Critical — the minimum-CLI field cannot protect existing binaries.** Manifests intentionally ignore unknown fields, so v0.135.0 would ignore it. Encode compatibility with a manifest value the old binary fails to deserialize, or do not publish.

## Round 4 — response and revision

Accepted. The plan now pairs `minimum-cli-version` with a new `status: requires-runtime` enum value. The new CLI understands and enforces both; v0.135.0 rejects the unknown status during deserialization. The compatibility test runs the actual tagged v0.135.0 binary.
