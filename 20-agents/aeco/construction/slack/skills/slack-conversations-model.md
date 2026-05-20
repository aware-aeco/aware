---
name: slack-conversations-model
description: This skill should be used when listing channels, reading message history, or resolving where to post with the slack agent — the unified conversations.* API, the channel-ID letter prefixes, public vs private membership rules, and how to DM a user. Encodes the channel model the messaging verbs target.
---

# Slack conversations model

Slack collapsed channels, private groups, DMs, and group-DMs into one **`conversations.*`** API. The legacy `channels.*` / `groups.*` / `im.*` / `mpim.*` families are deprecated — use `conversations.*` for everything. (The reflected catalog may still list legacy names; prefer the `conversations-*` commands.)

## Conversation IDs — and why the prefix does NOT tell you privacy

A conversation ID starts with `C`, `G`, or `D`. It is tempting to read the type off that letter — **don't.** Slack states the **prefix is an unreliable indicator of privacy**: a private channel keeps its `C…` ID (e.g. when a shared private channel is unshared, the `C…` ID does *not* revert to `G…`), so `C…` is **not** "public only."

| Prefix | Rough hint (not authoritative) |
|---|---|
| `C…` | a channel — **public or private** |
| `G…` | legacy private channel / multi-person DM (mpim) |
| `D…` | 1:1 direct message (im) |

**The authoritative type comes from booleans, not the prefix.** Call `conversations.info` and read `is_private`, `is_mpim`, `is_im`; treat the ID itself as opaque. When *listing*, pass the `types` filter (`public_channel,private_channel,mpim,im`) to scope what comes back.

You almost always need the **ID**, not the `#name`. Resolve names → IDs with `conversations.list` (and cache the mapping; see [slack-rate-limits-and-pagination](./slack-rate-limits-and-pagination.md) — `conversations.list` is paginated and rate-limited, so resolve once per run).

## The verbs you'll actually use

- `conversations.list` — enumerate channels. **Pass `types`** (`public_channel,private_channel`) or you get only public. Cursor-paginated.
- `conversations.info` — metadata for one conversation.
- `conversations.history` — messages in a channel (needs `*:history` scope).
- `conversations.replies` — messages in a **thread** (pass the parent `ts`).
- `conversations.members` — who's in it.
- `conversations.create` — make a channel (`is_private` for private). This is the **non-admin** way to create a project channel — not `admin.conversations.create` (Enterprise-Grid only; see [slack-auth-and-scopes](./slack-auth-and-scopes.md)).
- `conversations.invite` / `conversations.join` — add users / the bot.

## Posting requires membership — except public channels

- **Public channel:** with the `chat:write.public` scope the bot can post **without joining**. Otherwise `conversations.join` first.
- **Private channel:** the bot must be a **member** — invite it (`conversations.invite`) once; there is no "post to private without joining."
- **DM a user:** you can't `chat.postMessage` to a `U…` user ID directly. First `conversations.open` with `users: ["U0123"]` → it returns a `D…` channel ID → post to that.

## Common gotcha

`channel_not_found` usually means the bot isn't in a *private* channel (or you passed a `#name` instead of a conversation ID), **not** that the channel is missing. Check membership before assuming the ID is wrong.

## See also

- [slack-messaging-and-block-kit](./slack-messaging-and-block-kit.md) — what to send once you have the channel ID
- [slack-auth-and-scopes](./slack-auth-and-scopes.md) — the read/history scopes per conversation type
- [slack-rate-limits-and-pagination](./slack-rate-limits-and-pagination.md) — `conversations.list`/`history` paging
