# Breaking changes

## 1.0.0 — Gmail send becomes the only runnable command

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
