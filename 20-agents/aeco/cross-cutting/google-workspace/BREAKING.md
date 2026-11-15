# Breaking changes

## 2.4.0 — Gmail labels, a window of message ids and one message (not breaking)

Three read-only commands become runnable through AWARE's code-owned Gmail read, and need AWARE CLI
0.179.0 or newer and the opt-in `gmail.readonly` scope: `gmail.labels.list`,
`gmail.list-message-ids` (a received-time window, paged by Gmail's page token) and
`gmail.get-message` (one message's text and attachment list, never attachment content). Nothing
that ran before changes what it sends or returns. Two additions a caller may see:

- a Gmail rate limit — a 429, or a 403 whose Google reason is `rateLimitExceeded` or
  `userRateLimitExceeded` — now fails with `reason: "rate-limited"` (and `retry-after-secs` when
  Google sent `Retry-After`) on `E_CALL_FAILED`, `gmail.search` included, where it was the bare
  status before. A plain 403 is still the bare status. `gmail.search`'s operation digest is
  unchanged.
- a Google principal (the capability's `credential.principal`, every google-workspace read) now
  carries `namespace: "google-account"`, as Trimble Connect's carries `trimble-connect-user`. It is
  not part of the binding id, so no binding changes.

## 2.3.0 — Gmail search becomes runnable (not breaking)

`gmail.search` is no longer `planned`: it searches the connected Gmail account,
read-only, through AWARE's code-owned Gmail read, and needs AWARE CLI 0.178.0 or
newer. Its inputs change from the planned free `query` and `max-results` to typed
`text`, `from`, `label` and `page-size` — AWARE writes the search, no Gmail query
language is accepted. The output gains `label-ids`, `preview`, `web-url`,
`truncated`, `more-available` and `omitted`, and `from` becomes
`{address, name}`. Nothing that ran before changes. Gmail read is opt-in: connect
with `--scopes https://www.googleapis.com/auth/gmail.readonly`. A slot connected
that way still sends mail and still lists Drive when it also holds a Drive scope;
`gmail.readonly` alone never lists Drive, a Drive scope alone never searches mail,
and `gmail.send` still refuses every other extra Google scope.

## 2.2.0 — Drive list-files becomes runnable (not breaking)

`list-files` is no longer `planned`: it lists Drive file metadata, read-only,
through AWARE's code-owned Drive read (#618). Nothing existing changes. Drive is
opt-in: connect with `--scopes https://www.googleapis.com/auth/drive.readonly`
(`drive.metadata.readonly` is accepted too). A slot connected that way still sends
mail; any other extra Google scope is still refused by `gmail.send`. The output gains `modified-time`, `more-available` and
`incomplete-search`.

## 2.0.0 — Gmail send becomes the only runnable command

The earlier `0.2.0` manifest advertised 24 Google Workspace commands through an
`aware-google` executable that was not shipped. This release replaces that
unrunnable surface with one implemented, in-process `gmail.send` command and
marks the other 23 commands `planned` until transports exist for them.

`gmail.send` also changes incompatibly:

- `attempt-id` is now required. Derive it from the stable business object and
  mail action, then reuse the same value when reconciling an ambiguous outcome.
- `attachments` is removed. This first safe transport accepts only bounded
  plain-text or HTML bodies; arbitrary local paths are not admitted.
- `message-id` is explicitly the Gmail Message resource ID. The response also
  returns the identical `gmail-message-id`, the Gmail `thread-id`, and the RFC
  5322 header value as `rfc-message-id`.
- Only a definitive non-408 4xx is rejected. A timeout, redirect, 5xx, malformed
  success response, or post-dispatch persistence failure is reported as a
  non-retryable unknown outcome that must be reconciled before another send.

Reconnect Google credentials before using the new command. The accepted grant
is now exactly `openid`, `userinfo.email`, and `gmail.send`; legacy broad grants
fail closed with disconnect/reconnect guidance.

The runtime maintains its at-most-once journal under
`~/.aware/outbox/google-workspace/`, which is now an explicit filesystem write
capability of the agent.
