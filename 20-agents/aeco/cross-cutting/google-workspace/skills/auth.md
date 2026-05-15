---
name: google-workspace-auth
description: This skill should be used when authoring AWARE compositions or generated code that calls Google Workspace APIs — Drive, Sheets, Calendar, Gmail. Encodes that authentication is handled by the AWARE runtime (the agent reads `~/.aware/credentials/google-workspace.json` provisioned by `aware connect google-workspace`), the bearer-header injection pattern, scope-driven errors (403 means missing Cloud Console scope, not a token problem), and the standard error-code table (401/403/404/429/500/503). Apply when wiring Drive/Sheets/Calendar/Gmail calls, troubleshooting auth or scope errors, or composing AWARE apps that touch user Google data.
---

# Google Workspace — auth and HTTP patterns

**Authentication is handled by the AWARE runtime, not by composition code. The agent reads `~/.aware/credentials/google-workspace.json`. Users provision it once with `aware connect google-workspace`.**

## What `aware connect google-workspace` does

1. Opens Google's OAuth flow in the user's default browser (auth-code + PKCE; loopback redirect).
2. The user signs in with their Google account and grants the requested scopes.
3. Google redirects back to `localhost:<port>/callback`; the aware CLI listens briefly.
4. The CLI exchanges the code for an access + refresh token, encrypts both, writes to `~/.aware/credentials/google-workspace.json`.

The granted scopes are the agent's declared scopes (Drive, Sheets, etc.). To add scopes later, run `aware connect google-workspace --scopes drive,sheets,calendar`.

## API base URLs

| Surface | Base |
|---|---|
| Drive | `https://www.googleapis.com/drive/v3/` |
| Sheets | `https://sheets.googleapis.com/v4/spreadsheets/` |
| Calendar | `https://www.googleapis.com/calendar/v3/` |
| Gmail | `https://gmail.googleapis.com/gmail/v1/` |

## Critical conventions

- Use `client` as provided by the agent runtime — it carries `Authorization: Bearer …`. Do not construct a separate authenticated client.
- Calls outside `googleapis.com` are blocked by the agent's network policy. The agent will reject them before they hit the wire.
- For .NET implementations: apply `.ConfigureAwait(false)` on every `await` and prefer `System.Text.Json` over `Newtonsoft.Json`. For non-.NET runtimes, use the language's idiomatic patterns.

## Refresh

Access tokens expire (typically 1 hour). The agent runtime handles refresh **transparently**:

- Before each request, check if the access token has < 5 minutes left.
- If yes, exchange the refresh token for a new access token.
- Update the encrypted credential file.
- Proceed with the original request.

Refresh tokens are long-lived (no fixed expiry for Workspace accounts, but revocable). If the refresh token has been revoked or the granted scopes were removed, the agent emits `error.auth-expired` and the user runs `aware connect google-workspace --refresh`.

## What never goes in composition code

- **No tokens in app files.** The composition format is plain text; secrets do not belong there.
- **No hardcoded `Bearer ******`** in command headers. The agent injects this at invocation time.
- **No token logging.** Agent logs redact the `Authorization` header automatically.

## JSON parsing pattern

Google's collection shapes vary by API (Drive uses `files`, Sheets uses `values`, Gmail uses `messages`). The pattern is the same: a top-level object with a named array.

```csharp
// For .NET callers (System.Text.Json):
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

// Drive — files array
var files = doc.RootElement.GetProperty("files");
foreach (var file in files.EnumerateArray())
{
    var id   = file.GetProperty("id").GetString();
    var name = file.GetProperty("name").GetString();
}

// Single resource
var kind = doc.RootElement.GetProperty("kind").GetString();

// Safe property access
if (item.TryGetProperty("description", out var descEl))
{
    var desc = descEl.GetString();
}
```

Python equivalent:

```python
data = response.json()
for file in data["files"]:
    file_id = file["id"]
    name    = file["name"]
    desc    = file.get("description")
```

TypeScript:

```ts
const data = await response.json();
for (const file of data.files ?? []) {
  const { id, name, description } = file;
}
```

## Error handling

```csharp
var response = await client.GetAsync(url).ConfigureAwait(false);

if (response.IsSuccessStatusCode is false)
{
    var errorBody = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
    return Error("google.api-error", $"Google returned {(int)response.StatusCode}: {errorBody}");
}
```

## Common error codes

| HTTP | Meaning | Action |
|------|---------|--------|
| 401 | Token expired or invalid | Refresh handled automatically by the agent. If user-visible, the refresh token was revoked — user runs `aware connect google-workspace --refresh`. |
| 403 | **Either** insufficient scope **or** the API not enabled in the project | Two different fixes. Scope issue → `aware connect google-workspace --scopes <missing>`. API-disabled → enable it in the Google Cloud Console project backing the OAuth credentials. |
| 404 | Resource not found | Verify file / spreadsheet / message IDs. Drive trashed items return 404 by default unless `trashed=true` query is set. |
| 429 | Rate-limited (quota exhausted) | Agent retries with `Retry-After` header value. Surfaces after 3 attempts. |
| 500 | Internal server error | Agent retries with exponential backoff. Surfaces after 3 attempts. |
| 503 | Service unavailable | Same as 500. |

### Specific to Google: the dual 403

Google's 403 is ambiguous in a way Microsoft's isn't. A 403 can mean:

1. **The user didn't grant a scope** the call needs (most common; fix: re-consent with the right scopes).
2. **The Cloud Console project hasn't enabled the underlying API** (e.g., Sheets API disabled even though Drive works).

The agent surfaces both as `error.permission-denied` but logs the response body — checking the body reveals which case.

## Security notes (substrate-level)

- Outbound calls are restricted to `googleapis.com` and `accounts.google.com`. Other domains are blocked.
- Tokens are encrypted at rest via OS keychain (Mac) / DPAPI (Windows) / libsecret (Linux).
- Logs redact `Authorization` and `Set-Cookie` automatically.
- The credential file is never copied between machines — `aware connect` provisions per-host.

## Source

Google APIs documented at `developers.google.com/<api>`. OAuth flow per `developers.google.com/identity/protocols/oauth2`. Refresh-token semantics verified against `developers.google.com/identity/protocols/oauth2/native-app#offline`.
