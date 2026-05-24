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
AWARE at their own registration — but the first-class, no-env-var path for that is
**Bring your own app (Tier 2)** below.

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

## Bring your own app (Tier 2)

Everything above describes AWARE-AECO's **bundled first-party app (Tier 1)** — the
zero-setup quick-start. Tier 2 lets an organization point `aware connect` at *its
own* registered OAuth app (its tenant, its client_id/secret, its scopes) as
first-class config — no env vars required. Closes
[#146](https://github.com/aware-aeco/aware/issues/146).

**When to use it.** Registering your own app makes AWARE an *internal-use*
application in your own directory. That sidesteps the two gates the first-party app
hits: M365 publisher verification and Google's CASA assessment for restricted
scopes. Your org owns consent, scopes, and data access end-to-end.

### How resolution works

Each value resolves by precedence **profile ▸ env var ▸ bundled default**:

| Value | Profile (Tier 2) | Env var | Bundled default (Tier 1) |
|---|---|---|---|
| client_id | `client_id:` in the profile YAML | `AWARE_OAUTH_<INT>_CLIENT_ID` | committed in `config.rs` |
| client secret | OS keychain (`--set-app-secret`) | `AWARE_OAUTH_<INT>_CLIENT_SECRET` | build-time only (Google) |
| scopes | `scopes:` (replaces the default set) | — | committed in `config.rs` |
| tenant (M365) | `tenant:` (rewrites `common` → tenant) | `--tenant` flag wins | `common` |

A profile-set `client_id` deliberately suppresses the bundled first-party secret —
that mismatched pair would be rejected by the provider anyway — so a BYO Google app
must store its own secret (below).

### 1. Register your own app

- **M365:** repeat the Azure AD steps above, but choose **Accounts in this
  organizational directory only** (single-tenant). No publisher verification needed
  for your own tenant. Copy the Application (client) ID and your tenant id /
  `*.onmicrosoft.com` domain.
- **Google:** create your own Cloud project + Desktop-app OAuth client. As an
  internal-use app within your Workspace, restricted scopes don't require CASA. Copy
  the client ID **and** client secret.
- **Trimble:** register your own Trimble Identity app; copy the client ID.

### 2. Write the profile (non-secret fields)

Create `~/.aware/oauth/<integration>.yaml`. Only set the keys you want to override:

```yaml
# ~/.aware/oauth/microsoft-365.yaml
client_id: 11111111-2222-3333-4444-555555555555
tenant: contoso.onmicrosoft.com
# scopes:            # optional — omit to keep the curated default set
#   - offline_access
#   - User.Read
#   - Files.ReadWrite.All
```

```yaml
# ~/.aware/oauth/google-workspace.yaml
client_id: 9999-abc.apps.googleusercontent.com
# scopes: [ ... ]    # optional
```

Never put a client secret in this file.

For a sovereign / national cloud or a proxy, also set explicit endpoints — these
override the tenant-substituted public-cloud URLs and are honored by PKCE, refresh,
**and** device-code:

```yaml
# ~/.aware/oauth/microsoft-365.yaml (sovereign cloud example)
client_id: ...
tenant: contoso.onmicrosoft.com
auth_url: https://login.microsoftonline.us/contoso/oauth2/v2.0/authorize
token_url: https://login.microsoftonline.us/contoso/oauth2/v2.0/token
device_authorization_url: https://login.microsoftonline.us/contoso/oauth2/v2.0/devicecode
```

### 3. Store the client secret in the OS keychain

For Google (and any app that needs a secret), pipe it in via stdin — it never
touches argv or shell history, and it lands in the OS keychain, not plaintext:

```bash
echo "$GOOGLE_CLIENT_SECRET" | aware connect google-workspace --set-app-secret
```

### 4. Connect and verify

```bash
aware connect microsoft-365 --device-code
aware connect google-workspace --oauth
aware connect --list      # each line shows [app: byo-profile] when your app is active
```

`aware connect --list` (and `aware doctor`) report the active app per integration:
`first-party`, `byo-profile`, or `byo-env`.

### Per-account (alias) profiles

For multi-account setups, an alias-specific profile
`~/.aware/oauth/<integration>.<alias>.yaml` shadows the default
`<integration>.yaml`, and the secret is stored per alias:

```bash
echo "$SECRET" | aware connect google-workspace --as personal --set-app-secret
aware connect google-workspace --as personal --oauth
```
