---
name: bluebeam-rest-auth
description: This skill should be used when composing snippets that authenticate against the Bluebeam Studio Prime REST API — for any of the curated session.* / project.* / markup.* verbs. Encodes the OAuth2 + Bluebeam-account flow, the access-token vs API-key choice, the rate-limit headers, the long-lived session token (24h) vs short-lived API token (1h), and the gotcha that Bluebeam's API is REGION-specific (US / EU endpoints differ).
---

# Bluebeam Studio REST auth

The Bluebeam Studio Prime REST API (`studioapi.bluebeam.com`) is the only programmatic surface for Bluebeam workflows from outside the Revu desktop client. Get auth wrong and every other verb fails.

## The two auth modes

| Mode | Token | Lifetime | Use when |
|---|---|---|---|
| **OAuth2 access token** | Issued via Authorization Code flow against `accounts.bluebeam.com` | 1 hour | Interactive user flows; cross-account automation |
| **Studio Session token** | Issued when joining a Session via `POST /sessions/{id}/join` | 24 hours | In-Session markup pulls; long-running comms |
| **API key** | Static; admin-issued in Bluebeam account portal | Indefinite (until revoked) | Service-account automation; AWARE's default |

For AWARE workflows, prefer **API key** when available — it doesn't need refresh logic. For tenant-isolated workflows, OAuth2 is required.

## OAuth2 flow (when needed)

```
1. GET https://accounts.bluebeam.com/oauth/authorize?client_id=...&redirect_uri=...&scope=studio&response_type=code
2. User authorizes; redirected with ?code=...
3. POST https://accounts.bluebeam.com/oauth/token  
   { grant_type: "authorization_code", code: "...", client_id: "...", client_secret: "...", redirect_uri: "..." }
   → { access_token, refresh_token, expires_in: 3600 }
4. Use access_token as Bearer on every API request.
5. When expired (401), POST /oauth/token with grant_type=refresh_token
```

AWARE's `aware connect bluebeam-studio` is the canonical wrapper for this dance.

## API key (simpler when allowed)

```
Authorization: Bearer <api-key>
```

That's it. The key is issued by a Bluebeam Account Admin in the portal. AWARE stores it in `~/.aware/credentials/bluebeam-studio.json` (encrypted via OS keychain — see `aware.auth.keychain`).

## Regional endpoints

Bluebeam Studio is region-isolated. A US-account user CANNOT see an EU-account user's sessions, and vice versa. The endpoint differs:

| Region | Endpoint | Account portal |
|---|---|---|
| US | `https://studioapi.bluebeam.com/v1/` | `https://accounts.bluebeam.com/us/` |
| EU | `https://studioapi-eu.bluebeam.com/v1/` | `https://accounts.bluebeam.com/eu/` |

The agent's `transport.rest.base` config controls which endpoint. If your Bluebeam account is EU-region, override `base` in the app config.

## Rate limits

Bluebeam Studio API rate limits are documented per endpoint. Common limits:

| Endpoint | Limit |
|---|---|
| `GET /sessions` | 60 req/min per token |
| `GET /sessions/{id}/markups` | 30 req/min |
| `POST /sessions/{id}/markups` | 10 req/min (markup creation is heavyweight) |
| `POST /sessions` | 5 req/min (session creation) |

The response headers tell you:

```
X-RateLimit-Limit: 60
X-RateLimit-Remaining: 47
X-RateLimit-Reset: 1716198400   # Unix epoch when quota refills
```

When `Remaining` hits 0, requests return 429 with a `Retry-After: <seconds>` header. The AWARE transport implements automatic exponential backoff on 429.

## Session-vs-API-token

Joining a Studio Session issues a separate **session token** (24h) distinct from the OAuth/API token (1h). To call session-scoped endpoints (markups, comments), you need the SESSION token, not the OAuth one:

```
POST /sessions/abc-123/join
Authorization: Bearer <oauth-token>
→ { sessionToken: "xyz-789", expiresIn: 86400 }

GET /sessions/abc-123/markups
Authorization: Bearer <sessionToken>     # not the oauth token!
```

AWARE's transport handles this transparently — calls to session.* verbs auto-join and refresh session tokens.

## Common gotchas

- **API key Bearer auth doesn't refresh.** If the admin rotates the key, every AWARE workflow breaks until you re-run `aware connect bluebeam-studio --refresh`.
- **OAuth refresh_token has a 90-day idle expiry.** A weekly-cadence workflow is safe; a quarterly-cadence one isn't.
- **Session tokens DON'T transfer between sessions.** Each Session needs its own join.
- **EU + US accounts can't cross-coordinate via Bluebeam Studio.** For a project with US-architect + EU-engineer, both teams need accounts in BOTH regions (Bluebeam Pro Plus license includes both).

## See also

- `sessions-vs-projects.md` — what auth gives you access to
- `markup-disposition-workflow.md` — the read-side primary use case
- [`aware connect bluebeam-studio`](../../../../10-core/cli-spec.md) — credential management
