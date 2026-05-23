# OAuth App Registration

How AWARE-AECO's first-party OAuth applications are registered, and what
`aware connect <integration>` needs from each provider to work with **zero
end-user setup**. Closes [#141](https://github.com/aware-aeco/aware/issues/141).

The CLI ships bundled client IDs so `--device-code` / `--oauth` work out of the
box. This doc is the runbook for (re-)registering those apps and the one-time
admin steps an end user's organization may need.

> Scopes, endpoints, and flows below are the source of truth in
> [`cli/src/auth/config.rs`](../cli/src/auth/config.rs). Keep this doc and that
> file in sync.

## Flow per provider

| Integration | Default flow | Secondary | Client type | Secret? |
|---|---|---|---|---|
| `microsoft-365` | `--device-code` | `--oauth` (PKCE loopback) | Public, multi-tenant | No |
| `google-workspace` | `--oauth` (PKCE loopback) | — | Desktop app | Yes (non-confidential) |
| `trimble-connect` | `--oauth` (PKCE loopback) | — | — | (not yet registered) |

Rationale:
- **M365** is registered as a *public client* (`allowPublicClient=true`), so both
  device-code and PKCE-loopback work without a secret. Device-code is the
  recommended default on Windows / IT-managed tenants where browser-localhost
  loopback or raw-token extraction is blocked.
- **Google** has no true no-secret desktop client type. The "Desktop app" client
  requires `client_id` + `client_secret` in the token exchange **even under PKCE**.
  Google's native-app guidance explicitly treats this secret as *not confidential*,
  so bundling it is the standard pattern (the `gcloud` CLI ships one too). Device
  flow for Google needs a separate "TV & Limited Input" client and is not used.

## Bundled credentials

`config.rs` resolves credentials env-var-first, bundled-default-second:

| Value | Runtime env override | Bundled default |
|---|---|---|
| M365 client ID | `AWARE_OAUTH_M365_CLIENT_ID` | committed in `config.rs` (public — safe) |
| Google client ID | `AWARE_OAUTH_GOOGLE_CLIENT_ID` | committed in `config.rs` (public — safe) |
| Google client secret | `AWARE_OAUTH_GOOGLE_CLIENT_SECRET` | **never committed** — `option_env!("AWARE_GOOGLE_CLIENT_SECRET")` at build time |
| Trimble client ID | `AWARE_OAUTH_TRIMBLE_CLIENT_ID` | placeholder (not yet registered) |

Client **IDs** are not secrets (they ride in every OAuth request), so they are
committed directly. The Google client **secret** is the one value that must stay
out of source control:

- **Local / dev:** export `AWARE_OAUTH_GOOGLE_CLIENT_SECRET=<secret>` before
  `aware connect google-workspace`.
- **Release builds:** set `AWARE_GOOGLE_CLIENT_SECRET` in the build environment;
  `option_env!` bakes it into the binary at compile time. The committed source
  never contains it.

The runtime env vars also let an enterprise that blocks third-party apps point
AWARE at their own registration.

## Microsoft 365 — Azure AD registration

[portal.azure.com](https://portal.azure.com) → **Microsoft Entra ID → App
registrations → New registration**.

1. **Name** `AWARE-AECO`. **Account types:** *Accounts in any organizational
   directory and personal Microsoft accounts* (multi-tenant — lets any company
   sign in against the `common` endpoint).
2. **Authentication → Add platform → Mobile and desktop applications:**
   - Check the built-in `https://login.microsoftonline.com/common/oauth2/nativeclient`.
   - Add custom redirect `http://localhost` (the PKCE-loopback path binds an
     ephemeral port `127.0.0.1:7421–7430`; Azure matches on the `http://localhost`
     prefix regardless of port).
3. **Authentication → Advanced settings → Allow public client flows → Yes**
   (required for device-code; no effect on PKCE).
4. **API permissions → Microsoft Graph → Delegated** — add the 10 scopes the
   curated agents need:

   ```
   offline_access  User.Read  Files.ReadWrite.All  Sites.ReadWrite.All
   Mail.Read  Mail.Send  Calendars.ReadWrite  ChannelMessage.Send
   Chat.ReadWrite  Tasks.ReadWrite
   ```

5. Copy **Application (client) ID** → M365 client ID.

### Admin consent (one-time per tenant)

`Sites.ReadWrite.All`, `ChannelMessage.Send`, and `Chat.ReadWrite` are
**admin-consent-required** delegated scopes. On a managed tenant a normal user's
sign-in will be blocked until a tenant admin grants consent once:

```
https://login.microsoftonline.com/{tenant}/adminconsent?client_id={AWARE_M365_CLIENT_ID}
```

The admin opens that URL, reviews the permission list, and approves. Every user in
that tenant can then `aware connect microsoft-365 --device-code` without prompts.

## Google Workspace — Cloud client registration

[console.cloud.google.com](https://console.cloud.google.com) → new project
`aware-aeco`.

1. **APIs & Services → Enabled APIs → Enable:** Drive, Sheets, Calendar, Gmail,
   Slides, Forms, Tasks. (A `403` at runtime that isn't a scope error usually means
   one of these APIs is disabled — see the dual-403 note in the agent's `auth.md`.)
2. **OAuth consent screen → External.** Add the scopes the curated agents need:

   ```
   openid  .../auth/userinfo.email  .../auth/drive  .../auth/spreadsheets
   .../auth/calendar  .../auth/gmail.send  .../auth/gmail.readonly
   .../auth/presentations  .../auth/forms.body.readonly
   .../auth/forms.responses.readonly  .../auth/tasks
   ```

3. **Credentials → Create credentials → OAuth client ID → Desktop app.** Copy the
   **client ID and client secret**.

### Restricted scopes — the real gate

`.../auth/drive` and `.../auth/gmail.*` are Google **restricted scopes**.
Publishing the consent screen to *external* users requires a CASA Tier-2 security
assessment (third-party, recurring cost, weeks of lead time). Until that's done:

- Keep the app in **Testing** publishing status. Works immediately, but only for
  explicitly added **test users** (cap 100). Refresh tokens issued in Testing mode
  expire after 7 days.
- Alternatively, narrow to non-restricted scopes (`drive.file` instead of `drive`)
  to skip assessment — at the cost of the broad Drive commands.

This is the one item that blocks "any Google user signs in" until verification
lands. Track it separately from shipping the client ID.

## After registration

1. Commit the registered client **IDs** in
   [`cli/src/auth/config.rs`](../cli/src/auth/config.rs) (M365 + Google done).
   Trimble remains a placeholder until registered. Do **not** commit the Google
   secret — see the bundled-credentials note above.
2. `cargo fmt --all && cargo clippy --all-targets -- -D warnings && cargo test`.
3. Smoke-test (export `AWARE_OAUTH_GOOGLE_CLIENT_SECRET` first):
   ```
   aware connect microsoft-365 --device-code
   aware connect google-workspace --oauth
   aware connect --list          # confirm both show valid
   ```
