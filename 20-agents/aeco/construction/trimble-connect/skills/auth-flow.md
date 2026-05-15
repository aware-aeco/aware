---
name: trimble-connect-auth-flow
description: This skill should be used when authoring AWARE compositions or code that calls the Trimble Connect API — uploading files, reading projects, listing folders, fetching topics or viewpoints. Encodes that authentication is handled by the AWARE runtime, not by composition code. Apply when writing any Trimble-Connect-touching agent command, when troubleshooting auth errors, when supporting multi-account setups (personal + enterprise), or when generated code is about to reference a Trimble token directly (it should not).
---

# Trimble Connect — auth flow

**Authentication is handled by the AWARE runtime, not by composition code. The agent reads `~/.aware/credentials/trimble-connect.json`. Users provision it once with `aware connect trimble-connect`.**

## What `aware connect trimble-connect` does

1. Opens the Trimble OAuth flow in the user's default browser (PKCE — no client secret needed on-device).
2. The user signs in with their Trimble ID.
3. Trimble redirects back to `localhost:<port>/callback` (the aware CLI listens briefly).
4. The CLI receives the authorization code, exchanges it for an access + refresh token, encrypts both, and writes to `~/.aware/credentials/trimble-connect.json`.

The agent never sees the user's password. The browser never sees AWARE internals. The token file is encrypted at rest (OS keychain on Mac, DPAPI on Windows, libsecret on Linux).

## Refresh

Access tokens expire (typically 1 hour). The agent runtime handles refresh **transparently**:

- Before each request, check if the access token has less than 5 minutes left.
- If yes, use the refresh token to get a new pair.
- Update the encrypted credential file.
- Proceed with the original request.

Compositions never see a refresh failure unless the *refresh* token itself has expired (~30 days idle). At that point the agent emits `error.auth-expired` and the user runs `aware connect trimble-connect --refresh`.

## What never goes in composition code

- **No tokens in an app file.** The composition file is plain text; secrets do not belong there. Reference the credential id via `{{ secrets.trimble-connect }}` if direct reference is required (rare — the agent handles it).
- **No hardcoded `Bearer ******`** in a command's headers. The agent runtime injects the header at invocation time.
- **No token logging.** The agent's logs redact the `Authorization` header automatically; do not bypass this.

## Multi-account

Some users have multiple Trimble accounts (personal + enterprise). To support both:

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

## URL convention

`apiBaseUrl` = `"https://app.connect.trimble.com/tc/api/2.0"` (already includes `/2.0`). All endpoint paths are **relative** — never prepend `/2.0/` in code.

- Correct: `$"{apiBaseUrl}/projects"` → `https://app.connect.trimble.com/tc/api/2.0/projects`
- WRONG:   `$"{apiBaseUrl}/2.0/projects"` → double `/2.0/2.0/` = 404

Folder and file endpoints do NOT include `/projects/{projectId}/` — IDs are globally unique.

## Source

OAuth + PKCE per RFC 7636. Trimble's OAuth endpoints documented at `developer.trimble.com/identity`. Refresh-token expiry verified by production observation (Trimble doesn't publish the exact value; 30 days idle is the conservative limit).
