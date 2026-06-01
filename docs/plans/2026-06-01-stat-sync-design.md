# Stat-sync — keep the substrate's numbers from going stale

**Date:** 2026-06-01
**Status:** implemented (`scripts/sync_stats.py` + `.github/workflows/stats.yml`)

## Problem

The substrate's headline numbers — agent count, skills, commands, reference
apps, meta-primitives, decalog truths, CLI version — are repeated across
`README.md`, `00-vision/manifesto.md`, `CLAUDE.md`, `10-core/cli-roadmap.md`,
and `40-diagrams/*`. They were hand-maintained and drifted badly (11 vs 39 vs 66
agents; "five structural truths" when there are nine; the playground showing 39
agents). Every PR that adds an agent silently staled them again.

## Decisions

| Decision | Choice | Why |
|---|---|---|
| Engine | A **deterministic Python script**, not an LLM skill | Counting must never hallucinate (decalog #9: deterministic code is the plan). Matches the existing `build-substrate-playground.py` generator pattern; stdlib-only. |
| Enforcement | **CI check that fails the PR** when a number is stale | Predictable; no bot writing to branches, no write-token, no merge-race. The same script fixes it locally. |
| Doc update | **Invisible HTML-comment markers** `<!--stat:KEY-->66<!--/stat-->` | Opt-in: only marked numbers are managed, so narrative numbers ("began as 7 reference agents") are never clobbered — the exact false-positive class that bit the earlier cleanup. |
| Source of truth | Computed from `registry-index.json` + the `20-agents/` tree + `30-apps/` + `decalog.md` + `cli/Cargo.toml` | One canonical computation; no second source to drift. |

## Architecture

`scripts/sync_stats.py`:

- **`compute_stats()`** — walks the tree once, returns the canonical values:
  `agents_total`, `agents_registered`, `agents_curated`, `agents_reflected`,
  `skills`, `commands`, `catalog`, `apps`, `meta_primitives`, `decalog_truths`,
  `cli_version`.
- **Markers** (`MANAGED_FILES`) — `README.md`, `manifesto.md`, `CLAUDE.md`.
  Every managed number lives inside `<!--stat:KEY-->…<!--/stat-->`. `process_markers`
  replaces only the marked span.
- **Anchor rules** (`ANCHOR_RULES`) — for numbers inside fenced ` ``` ` code
  blocks (README demo + repo tree, manifesto demo) and Mermaid (`.mmd`), where
  HTML comments would render literally. Each rule is a tightly-scoped regex that
  replaces a single value using distinctive surrounding text.
- **Playground** — `substrate-playground.html` inlines its own `RAW_AGENTS`
  dataset. We check that `len(RAW_AGENTS) == agents_total` (OS-independent count),
  rather than byte-diffing a regenerated file (fragile across CRLF/LF and
  platforms). `--write` regenerates it via its own generator.

Modes: `--check` (default, exit 1 + a `file → found vs expected` diff if stale),
`--write` (rewrite markers/anchors + regenerate the playground), `--selftest`
(unit tests of the pure logic).

### Supporting change

`build-substrate-playground.py` now sorts agents by `id` before emitting, so
regeneration is deterministic / byte-stable across machines (was filesystem-order
dependent). This is what makes the playground count-check meaningful.

## CI

`.github/workflows/stats.yml` — the repo's first `on: pull_request` workflow.
Runs `--selftest` then `--check`. No network, no secrets, no write-token, <30s.
A failing check prints the one-line fix: `python scripts/sync_stats.py --write`.

## Testing

`--selftest` (stdlib `unittest`) covers: stale detection, clean pass,
idempotence, unknown-key flagging, **unmarked numbers left untouched**, anchor
replacement, and a `compute_stats()` shape/invariant check
(`agents_total == agents_registered`). Verified end-to-end that a corrupted
playground count fails `--check` and `--write` restores it.

## Coverage & follow-ups

- **Covered now:** all marker/anchor numbers in README/manifesto/CLAUDE, the
  Mermaid decalog count, and the playground agent count.
- **Deferred (YAGNI):** a `/aware-sync-stats` slash-command and a `make stats`
  wrapper (the `--write` command already does the job); extending markers to
  `cli-roadmap.md`'s illustrative numbers; managing per-discipline counts.
- **Adding a new managed number:** wrap it in `<!--stat:KEY-->…<!--/stat-->`
  (or add an `ANCHOR_RULES` entry for code blocks) and add `KEY` to
  `compute_stats()`.
