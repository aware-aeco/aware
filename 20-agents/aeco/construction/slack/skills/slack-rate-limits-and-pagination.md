---
name: slack-rate-limits-and-pagination
description: This skill should be used when a slack workflow makes more than a couple of calls — the per-method rate-limit tiers, the special chat.postMessage per-channel limit, honoring Retry-After on 429, and cursor pagination for list/history endpoints. Encodes how to not get throttled.
---

# Slack rate limits & pagination

Slack throttles per method, per workspace. A loop that posts 50 messages or pages through every channel will hit `429` unless it respects the tiers and paginates with cursors.

## Rate-limit tiers

Each method sits in a tier (documented on its API page). Rough budgets (per workspace, per method):

| Tier | ~Requests/min | Typical methods |
|---|---|---|
| Tier 1 | ~1 | rare, heavy admin ops |
| Tier 2 | ~20 | `conversations.list`, `users.list` |
| Tier 3 | ~50 | `conversations.history`, `conversations.replies` |
| Tier 4 | ~100 | light reads (`conversations.info`, `auth.test`) |

> **Major caveat — `conversations.history` / `conversations.replies` are throttled for *newly distributed* apps.** Since **2025-05-29**, **newly-created** apps distributed outside the Slack Marketplace ("unlisted"), and **new installs** of such apps, are capped at **1 request/minute and 15 objects per page** on these two methods — *not* the Tier 3 budget above — to curb bulk conversation-data exfiltration. **Not affected (keep normal limits):** existing installations (grandfathered), Marketplace-approved apps, and **internal customer-built apps**. A self-registered AWARE Slack app used inside your own workspace is internal → fine; a freshly-distributed non-Marketplace app will crawl when paging history. Confirm which regime your app is in before building a history-paging workflow.

**Special case — `chat.postMessage`:** limited to roughly **1 message per second per channel** (short bursts tolerated). A "notify 30 channels" fan-out is fine; a "post 30 messages to one channel" loop will throttle — batch into one Block Kit message instead.

## On `429`, honor `Retry-After`

A throttled response is HTTP `429` with a **`Retry-After: <seconds>`** header. The correct behavior is: wait exactly that many seconds, then retry — not a fixed sleep, not an immediate retry. Treat `Retry-After` as authoritative. (If composing retries in glue, back off on `Retry-After`, cap attempts, and surface a clear error rather than hammering.)

## Cursor pagination

List/history endpoints return at most `limit` items plus a cursor. Page like this:

```
1. call with { limit: 200 }
2. read response_metadata.next_cursor
3. if non-empty, call again with { cursor: <next_cursor>, limit: 200 }
4. repeat until next_cursor is empty/absent
```

Applies to `conversations.list`, `conversations.history`, `conversations.replies`, `conversations.members`, `users.list`, `files.list`, and the `*.list` admin methods. **Don't** assume one call returns everything — a workspace with 500 channels needs 3 pages at `limit: 200`.

**Resolve-once pattern:** channel-name→ID and user-email→ID lookups are paginated Tier 2 calls. Resolve them once at app start and cache for the run rather than re-listing per notification (mirrors the Trimble Connect `cache: app-lifetime` pattern).

## This is a REST agent — no real-time

The slack agent is request/response (`stateful: false`). For *reacting* to Slack events (someone replied, a slash command), that's the Events API / Socket Mode — **not** in this agent's surface. Don't poll `conversations.history` in a tight loop to simulate real-time; you'll burn the Tier 3 budget. Poll on a sane `schedule:` instead, or drive the workflow from the AECO side (Tekla/ACC event → post to Slack).

## See also

- [slack-conversations-model](./slack-conversations-model.md) — the paginated `list`/`history` verbs
- [slack-messaging-and-block-kit](./slack-messaging-and-block-kit.md) — batch into one message vs many
