# Google Workspace Gmail send live canary

This runbook is the release gate for `google-workspace.gmail.send`. A successful
profile request, process exit code, or AWARE node output alone is not evidence
that Gmail accepted exactly one intact message.

Do not mark issue #495 `qa-ready` until this canary succeeds against real Google
credentials and controlled mailboxes. The repository test suite cannot supply
that external evidence.

## Safety setup

1. Use a Google test account whose Sent mailbox can be inspected.
2. Use controlled primary and Bcc recipient mailboxes. Never address a customer,
   consultant, regulator, distribution list, or production mailbox.
3. Copy
   `20-agents/aeco/cross-cutting/google-workspace/fixtures/issue-495-gmail-send-canary.app`
   to a temporary location.
4. Replace both `example.invalid` addresses with the controlled recipients.
5. Replace every `replace-with-unique-*` marker. The `attempt-id` must be unique
   for this logical canary and must remain unchanged for any replay check.
6. Confirm the connected credential has exactly `openid`, `userinfo.email`, and
   `gmail.send`. A legacy broad grant must fail closed; disconnect and reconnect
   it with `aware connect google-workspace --oauth` before proceeding. First
   remove or narrow any `scopes` entry in the active AWARE home's
   `oauth/google-workspace.yaml` (`AWARE_HOME` when set, `~/.aware` otherwise);
   profiles replace the bundled scope set. Use the default connection, not an `--as` alias, because published app
   dispatch does not expose alias selection. Explicitly disconnect any old
   broad-scope alias with
   `aware disconnect google-workspace --as=<old-alias>`; do not reconnect it for
   this agent. Google consent must issue a refresh token for the offline
   connection.

## Execute

Validate, compile, and run the copied app using AWARE CLI `0.136.0` or newer.
Capture the accepted output without recording recipient addresses, subject, or
body in public logs. Record only:

- attempt ID
- `message-id`
- `gmail-message-id`
- `thread-id`
- `rfc-message-id`
- timestamps and final status

`message-id` and `gmail-message-id` must be identical nonblank Gmail resource
IDs. `rfc-message-id` must be a distinct RFC 5322 header value. Acceptance does
not prove delivery.

## Provider verification

Using the unique RFC Message-ID and attempt marker:

1. Find exactly one matching message in the authenticated sender's Sent mailbox.
2. Find exactly one matching message in the primary recipient mailbox.
3. Find exactly one matching message in the controlled Bcc mailbox.
4. Confirm no second matching message exists in any of those mailboxes.
5. Inspect the decoded headers and body, not only the provider preview. Verify
   sender, To, subject, content type, body text, line endings, and marker.
6. Confirm the recipient-delivered copies do not contain a `Bcc` header. Gmail
   must have stripped it even though Bcc was present in the submitted MIME.
7. Confirm the recorded Gmail resource ID identifies the Sent message and retain
   the verification evidence in the private QA record.

## Replay check

Run the unchanged app once more with the identical attempt ID and inputs. AWARE
must return the cached accepted output and Gmail must still contain exactly one
matching message. Changing any input while retaining the attempt ID must fail as
an attempt conflict without sending.

If the command returns `gmail.send.outcome-unknown`, stop. It may already have
sent the email and reports `retryable: false`. Reconcile by RFC Message-ID and
attempt marker; do not invent a new attempt ID or rerun until the outcome is
known. A definitive pre-dispatch failure or `gmail.send.rejected` may be
corrected and retried only under the command's documented attempt rules.
