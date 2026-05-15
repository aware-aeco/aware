# Trimble Connect Skill · Auth flow

**Authentication is handled by AWARE, not by you. The agent reads `~/.aware/credentials/trimble-connect.json`. The user provisions it once with `aware connect trimble-connect`.**

## What `aware connect trimble-connect` does

1. Opens the Trimble OAuth flow in the user's default browser (PKCE — no client secret needed on-device)
2. User signs in with their Trimble ID
3. Trimble redirects back to `localhost:<port>/callback` (the aware CLI listens briefly)
4. The CLI receives the authorization code, exchanges it for an access + refresh token, encrypts both, and writes to `~/.aware/credentials/trimble-connect.json`

The agent never sees the user's password. The browser never sees AWARE internals. The token file is encrypted at rest (OS keychain on Mac, DPAPI on Windows, libsecret on Linux).

## Refresh

Access tokens expire (typically 1 hour). The agent runtime handles refresh **transparently**:

- Before each request, check if the access token has < 5 minutes left
- If yes, use the refresh token to get a new pair
- Update the encrypted credential file
- Proceed with the original request

Your composition never sees a refresh failure unless the *refresh* token itself has expired (~30 days idle). In that case the agent emits `error.auth-expired` and the user runs `aware connect trimble-connect --refresh`.

## What you should never do

- **Never put a token in an app file.** The composition file is plain text; secrets don't belong there. Use `{{ secrets.trimble-connect }}` if you must reference the credential ID (rare — the agent does this for you).
- **Never hardcode `Bearer ******`** in a command's `Headers:`. The agent runtime injects the header at invocation time.
- **Never log the token.** The agent's logs redact the `Authorization` header automatically; don't bypass this.

## Multi-account

Some users have multiple Trimble accounts (personal + enterprise). To support:

```bash
aware connect trimble-connect --as personal
aware connect trimble-connect --as bimstudio-enterprise
```

This stores two credential files: `trimble-connect.personal.json` and `trimble-connect.bimstudio-enterprise.json`. In an app:

```yaml
- id: upload
  agent: trimble-connect
  command: upload
  config:
    auth-as: bimstudio-enterprise
    project-id: "..."
```

## Source

OAuth + PKCE per RFC 7636. Trimble's OAuth endpoints documented at `developer.trimble.com/identity`. Refresh token expiry verified via experimentation (Trimble doesn't publish the exact value; 30 days idle is conservative).
