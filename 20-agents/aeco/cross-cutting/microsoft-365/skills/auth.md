---
name: microsoft-365-auth
description: This skill should be used when authoring AWARE compositions or generated code that calls the Microsoft Graph API — OneDrive / SharePoint / Outlook / Teams / Calendar. Encodes that authentication is handled by the AWARE runtime (the agent reads `~/.aware/credentials/microsoft-365.json` provisioned by `aware connect microsoft-365`), the bearer-header injection pattern, sharing-URL encoding for the `/shares` endpoint, the standard error-code table (401/403/404/429/503), and the JSON-collection-via-`value`-array convention. Apply when wiring Graph API calls, troubleshooting auth errors, or debugging sharing-link resolution.
---

# Microsoft 365 (Graph) — auth and HTTP patterns

**Authentication is handled by the AWARE runtime, not by composition code. The agent reads `~/.aware/credentials/microsoft-365.json`. Users provision it once with `aware connect microsoft-365`.**

## What `aware connect microsoft-365` does

1. Opens the Microsoft identity OAuth flow in the user's default browser (auth-code + PKCE).
2. The user signs in with their Azure AD identity.
3. Microsoft redirects back to `localhost:<port>/callback`; the aware CLI listens briefly.
4. The CLI exchanges the code for an access + refresh token, encrypts both, writes to `~/.aware/credentials/microsoft-365.json`.

Tokens refresh transparently before expiry (see [`refresh`](#refresh) below). The composition layer never sees the raw token; the agent injects `Authorization: Bearer …` automatically.

## Critical conventions

- The Graph API base URL is `https://graph.microsoft.com/v1.0/`.
- Graph returns JSON with a `value` array for collection endpoints.
- Use `client` as provided by the agent runtime — it carries the bearer header. Do not construct a separate authenticated client.
- For .NET implementations: apply `.ConfigureAwait(false)` on every `await` (avoids UI-thread capture in WPF / WinForms hosts) and prefer `System.Text.Json` over `Newtonsoft.Json`. For non-.NET runtimes (Python, TypeScript, Go), use the language's idiomatic patterns.

## Refresh

Access tokens expire (typically 1 hour). The agent runtime handles refresh **transparently**:

- Before each request, check if the access token has < 5 minutes left.
- If yes, use the refresh token to get a new pair.
- Update the encrypted credential file.
- Proceed with the original request.

Compositions only see auth failures when the *refresh* token itself has expired (~90 days idle for Azure AD by default). The agent then emits `error.auth-expired` and the user runs `aware connect microsoft-365 --refresh`.

## What never goes in composition code

- **No tokens in app files.** The composition format is plain text; secrets do not belong there.
- **No hardcoded `Bearer ******`** in command headers. The agent injects this at invocation time.
- **No token logging.** Agent logs redact the `Authorization` header automatically.

## JSON parsing pattern

Graph's standard collection shape: `{ "value": [ … ] }`.

```csharp
// For .NET callers (System.Text.Json):
var json = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
using var doc = JsonDocument.Parse(json);

// Collection (the `value` array)
var items = doc.RootElement.GetProperty("value");
foreach (var item in items.EnumerateArray())
{
    var id   = item.GetProperty("id").GetString();
    var name = item.GetProperty("name").GetString();
}

// Single resource
var displayName = doc.RootElement.GetProperty("displayName").GetString();

// Safe property access (may not exist)
if (item.TryGetProperty("description", out var descEl))
{
    var desc = descEl.GetString();
}
```

Python equivalent:

```python
data = response.json()
for item in data["value"]:
    item_id = item["id"]
    name    = item["name"]
    desc    = item.get("description")
```

TypeScript:

```ts
const data = await response.json();
for (const item of data.value) {
  const { id, name, description } = item;
}
```

## Sharing-URL encoding (the `/shares` endpoint)

Users often paste SharePoint or OneDrive sharing links rather than item IDs. To resolve a link to a `driveItem`, encode it for the `/shares` endpoint:

```csharp
static string EncodeSharingUrl(string sharingUrl)
{
    var base64 = Convert.ToBase64String(System.Text.Encoding.UTF8.GetBytes(sharingUrl));
    return "u!" + base64.TrimEnd('=').Replace('/', '_').Replace('+', '-');
}
```

Python:

```python
import base64

def encode_sharing_url(sharing_url: str) -> str:
    b64 = base64.b64encode(sharing_url.encode("utf-8")).decode("ascii")
    return "u!" + b64.rstrip("=").replace("/", "_").replace("+", "-")
```

Then: `GET https://graph.microsoft.com/v1.0/shares/u!{encoded}/driveItem`.

## Error handling

Always check `response.IsSuccessStatusCode` (or the equivalent in your runtime) before reading the body. On failure, surface a structured error from the agent — the composition layer reads `error.message` / `error.code`, not raw HTTP bodies.

```csharp
var response = await client.GetAsync(url).ConfigureAwait(false);

if (response.IsSuccessStatusCode is false)
{
    var errorBody = await response.Content.ReadAsStringAsync().ConfigureAwait(false);
    return Error("m365.api-error", $"Graph returned {(int)response.StatusCode}: {errorBody}");
}
```

## Common error codes

| HTTP | Meaning | Action |
|------|---------|--------|
| 401 | Token expired or invalid | Refresh handled automatically by the agent runtime. If you see this in user-visible output, the refresh token also expired — user runs `aware connect microsoft-365 --refresh`. |
| 403 | Insufficient permissions | The Azure AD app registration backing AWARE doesn't have the required Graph permission. Documented per-command. |
| 404 | Resource not found | Verify drive / item / site IDs. Sharing URLs need encoding (see above). |
| 429 | Throttled | Agent retries with `Retry-After` header value. Surfaces after 3 attempts. |
| 503 | Service unavailable | Agent retries with exponential backoff. Surfaces after 3 attempts. |

## Security notes (substrate-level)

- The agent restricts outbound calls to `https://graph.microsoft.com` and `https://login.microsoftonline.com`. Calls to other domains are blocked.
- Tokens are encrypted at rest via OS keychain (Mac) / DPAPI (Windows) / libsecret (Linux).
- Logs redact `Authorization` and `Set-Cookie` automatically.
- The credential file is never copied between machines — `aware connect` provisions per-host.

## Source

Graph API documented at `learn.microsoft.com/graph`. OAuth flow per Microsoft identity platform docs (`learn.microsoft.com/azure/active-directory/develop`). Refresh-token lifetime verified by tenant policy (default 90 days idle, configurable).
