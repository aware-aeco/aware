---
name: google-workspace-auth
description: Use when connecting or troubleshooting the Gmail-only Google Workspace runtime. Covers the exact OAuth scopes, authenticated sender identity, pinned endpoints, legacy-grant migration, refresh behavior, and non-retryable send outcomes.
---

# Google Workspace Gmail authentication

Authentication is handled by the AWARE runtime, not by composition code. Run
`aware connect google-workspace --oauth` once; do not put access tokens, refresh tokens,
client secrets, or hand-built `Authorization` headers in an app.

Agent version `2.0.0` exposes only `gmail.send`. Drive, Sheets, Calendar, Gmail
search, Chat, Forms, Slides, Meet, and Tasks remain planned and their scopes are
not requested.

## Exact grant

New Google connections request only:

```text
openid
https://www.googleapis.com/auth/userinfo.email
https://www.googleapis.com/auth/gmail.send
```

`gmail.send` is a Google sensitive scope. It is not a restricted scope.
`gmail.readonly` is restricted and is no longer part of the default grant.

Older credentials may still carry Drive, Gmail read, Calendar, Sheets, Slides,
Forms, Tasks, or other broad scopes. The Gmail send runtime fails closed before
dispatch when it detects one of these legacy broad grants. Migrate deliberately:

First remove the `scopes` entry from the active AWARE home's
`oauth/google-workspace.yaml`, or narrow it to exactly the three scopes above.
The active home is the `AWARE_HOME` override when set and `~/.aware` otherwise.
Profiles replace the bundled scope set, so reconnecting without this step would
request the same broad grant again.

```text
aware disconnect google-workspace
aware connect google-workspace --oauth
```

The published agent uses the default `google-workspace` credential. Do not use
an `--as` alias for this agent: app dispatch does not expose alias selection. If
you previously connected a broad grant under an alias, remove that stored
credential explicitly before connecting the default account:

```text
aware disconnect google-workspace --as=<old-alias>
```

Do not reconnect the alias for this published agent.

Do not bypass this check merely because the old token also includes
`gmail.send`; the extra scopes increase the impact of credential theft.

## Identity and sender

The runtime resolves the stable OIDC `sub` and mailbox email for the
authenticated account before constructing the message. A hash of `sub`
namespaces the durable outbox, so disconnecting and reconnecting the same Google
account does not reset replay protection. The resolved mailbox is the RFC 5322
`From` identity. Apps cannot supply an arbitrary sender.

## Pinned network surface

The agent declares only these HTTPS hosts:

- `oauth2.googleapis.com` for token refresh
- `openidconnect.googleapis.com` for authenticated OIDC identity
- `gmail.googleapis.com` for Gmail submission

The runtime pins the identity call and Gmail's exact
`/gmail/v1/users/me/messages/send` path in code, rejects redirects, and does not
accept a production endpoint override. The manifest's REST base describes the
transport; it is not an authorization to send a bearer token to an arbitrary
URL.

## Refresh and pre-dispatch checks

Access-token refresh must succeed before a send. A refresh error, blank token,
missing required scope, legacy broad grant, or unresolved account identity is a
pre-dispatch authentication error: the runtime does not construct or transmit
the message. Run the disconnect/connect sequence above if consent is stale or
over-broad.

## Send errors are not generic REST errors

`gmail.send` does not use generic REST retries or response shaping:

- Validation, auth, and identity failures occur before dispatch.
- A definitive provider 4xx rejection is `gmail.send.rejected`.
- HTTP 408/5xx, redirects, transport/read/parse failures, cancellation or
  timeout after handoff, and a success body without a usable Gmail ID are
  `gmail.send.outcome-unknown` with `retryable: false`.
- An accepted response is cached against the caller's `attempt-id`; replaying
  identical inputs returns that result without another provider call.

An unknown outcome may already have sent mail. Reconcile using the bounded
`attemptId` and `rfcMessageId` error details. Never retry by inventing a fresh
attempt ID.

## Secret hygiene

- Never log bearer tokens, recipients, Bcc addresses, subjects, or bodies.
- AWARE redacts authorization material. Direct `gmail.send` dry-run previews also
  redact `to`, `cc`, `bcc`, `subject`, and `body`.
- App-level run configuration and arbitrary intermediate values can be persisted in
  traces. Do not pass mail content through exposed-app configuration or unrelated
  nodes unless that trace is protected as sensitive data.
- Only bounded status and correlation metadata belong in errors and QA records.
- Credentials remain per-host and are protected by the operating-system
  credential store.

See [`../commands/gmail.send.md`](../commands/gmail.send.md) for the command and
attempt contract.
