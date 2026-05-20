---
name: slack-auth-and-scopes
description: This skill should be used when calling any Slack Web API endpoint through the slack agent — choosing between bot and user tokens, requesting the minimum OAuth scopes a workflow needs, and understanding which endpoints require admin/Enterprise-Grid scopes. Encodes the auth model behind every slack command.
---

# Slack auth & scopes

Every slack endpoint runs against a workspace token whose **OAuth scopes** decide what it may do. Get the token + scopes right and the 172 endpoints just work; get them wrong and you get `not_authed`, `invalid_auth`, or `missing_scope` errors that look like bugs but are permission gaps.

## Two token types

| Token | Prefix | Acts as | Use when |
|---|---|---|---|
| **Bot token** | `xoxb-` | the app's bot user | **Default for automation.** Posts as the AWARE app, survives the installing user leaving. |
| **User token** | `xoxp-` | the authorising human | Only when you must act *as a person* (e.g. read a private channel that human is in, post on their behalf). |

Provision both via `aware connect slack` (browser-paste OAuth). Prefer the **bot token** for AECO notification workflows — it doesn't break when an employee offboards.

## Scopes are per-method and granular

Slack scopes are fine-grained; each endpoint lists the scope(s) it needs. The ones AECO workflows actually use:

| Scope | Unlocks |
|---|---|
| `chat:write` | `chat.postMessage`, `chat.update`, `chat.delete` (channels the bot is in) |
| `chat:write.public` | post to **public** channels the bot has *not* joined (no `conversations.join` needed) |
| `channels:read` / `groups:read` | `conversations.list`, `conversations.info` (public / private channels) |
| `channels:history` / `groups:history` | `conversations.history`, `conversations.replies` (public / private channels) |
| `im:read` / `im:history` · `mpim:read` / `mpim:history` | the **DM** (`im`) and **group-DM** (`mpim`) equivalents — required *in addition* to the channel scopes when your `types` filter or workflow touches DMs / mpims |
| `im:write` / `mpim:write` | open a DM / group DM (`conversations.open`); also needed to DM a user by `U…` id |
| `files:write` | upload files (but see [slack-files-and-uploads](./slack-files-and-uploads.md) — the catalog's `files.upload` is deprecated) |
| `users:read` / `users:read.email` | `users.info`, `users.lookupByEmail` (map a PM's email → Slack user) |
| `reactions:write` | `reactions.add` (✅ a build-passed message) |

**Match scopes to the workflow, never over-request.** Workspace admins review scope grants; an app asking for `admin.*` when it only posts messages gets rejected. Least privilege also limits blast radius if the token leaks.

## The `admin.*` endpoints are a different tier

~60 of the 172 endpoints are `admin.*` (conversations, users, teams, emoji, apps). They require:
- **Enterprise Grid** (not a standard workspace), and
- org-level **admin scopes** (`admin.conversations:write`, `admin.users:read`, …) on a token granted by an org admin.

On a normal workspace these return `not_allowed_token_type` / `team_not_on_enterprise_grid`. Don't reach for `admin.conversations.create` to make a project channel — use plain `conversations.create` ([slack-conversations-model](./slack-conversations-model.md)).

## Token hygiene

Tokens are stored encrypted by `aware connect`; never appear in app files or logs. `auth.test` is the cheap "is this token alive and which workspace/user/bot is it?" probe — call it first when debugging auth. `auth.revoke` kills a token.

## See also

- [slack-conversations-model](./slack-conversations-model.md) — channel IDs + the read/write scopes per conversation type
- [slack-messaging-and-block-kit](./slack-messaging-and-block-kit.md) — what `chat:write` lets you post
- [slack-rate-limits-and-pagination](./slack-rate-limits-and-pagination.md) — even authorised calls are rate-limited
