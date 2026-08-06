# Vendored review agents

Six agents copied verbatim from the `pr-review-toolkit` plugin (claude-plugins-official marketplace).
They are here so the **cloud maintenance routines** can reach them.

## Why vendored rather than installed

The cloud maintenance routines (defined in `pawellisowski/floless.app` under `.claude/routines/`) run in cloud sessions that clone this repository and nothing else.
Two things follow:

1. A plugin installed on a laptop is not there. A file committed here is — project agents under
   `.claude/agents/` load automatically from the clone, with no marketplace, plugin config or
   environment image involved.
2. Installing the plugin would not have worked anyway. Every routine's `allowed_tools` is
   `Bash, Read, Write, Edit, Glob, Grep, Task` — no `Skill` tool, so `/pr-review-toolkit:review-pr`
   is not callable there. `Task` is, which is what actually dispatches these six.

## What they are for

They are the **fallback reviewer** on every routine, for when the Codex code review is unavailable —
the standing rule is that a Codex rate limit never stalls a review. Six specialised adversarial
reviewers replace the single generic one the prompts used to spawn.

They share this session's model, so what they cannot supply is the cross-model guarantee. The routines
account for that themselves: only the two mechanical sweeps (dead code, test hygiene) may merge on a
clean local review; everything making a judgement call still waits for a human.

## Keeping them current

Refresh by re-copying from the plugin cache; do not edit them in place, or the next refresh silently
reverts the edit:

```bash
cp ~/.claude/plugins/cache/claude-plugins-official/pr-review-toolkit/unknown/agents/*.md .claude/agents/
```

The same six files are vendored in `pawellisowski/floless.app`, which is where every routine definition actually lives (`.claude/routines/`).
