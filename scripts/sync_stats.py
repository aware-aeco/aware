#!/usr/bin/env python3
"""
sync_stats.py — keep the substrate's stat numbers in sync across the docs.

Single source of truth (all computed, never hand-counted):
  - registry-index.json         → registered-agent count
  - 20-agents/ tree             → manifests, curated/reflected split, skills,
                                   commands, catalog entries, meta-primitives
  - 30-apps/**/*.flo            → reference-app count
  - 00-vision/decalog.md        → number of structural truths
  - cli/Cargo.toml              → CLI version

Each managed number in the docs is wrapped in an invisible marker:

    <!--stat:agents_total-->66<!--/stat-->

The marker is an HTML comment, so it does not render in Markdown. Only the text
between the markers is ever rewritten — anything unmarked (e.g. the narrative
"began as 7 reference agents") is never touched. That opt-in contract is what
keeps the sync from clobbering intentional/historical numbers.

For files where HTML comments can't hide (Mermaid `.mmd`), a small ANCHOR_RULES
list does a single targeted regex replacement instead.

Usage (run from the repo root):
    python scripts/sync_stats.py --check     # verify; exit 1 + diff if stale
    python scripts/sync_stats.py --write      # rewrite the docs in place
    python scripts/sync_stats.py --selftest   # unit-test the pure logic
"""

from __future__ import annotations

import os
import re
import sys
import json
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent

# Markdown docs whose managed numbers live inside <!--stat:KEY-->…<!--/stat-->.
MANAGED_FILES = [
    "README.md",
    "00-vision/manifesto.md",
    "CLAUDE.md",
]

MARKER_RE = re.compile(r"<!--stat:([a-z_]+)-->(.*?)<!--/stat-->", re.DOTALL)

# Files where invisible HTML markers aren't possible (Mermaid renders them
# literally). Each rule replaces the single capture group with the stat value.
ANCHOR_RULES = [
    # Mermaid renders HTML comments literally, so markers can't hide there.
    (
        "40-diagrams/aware-master.mmd",
        re.compile(r"(?<=Decalog<br/>)(\d+)(?= structural truths)"),
        "decalog_truths",
    ),
    # Numbers inside fenced ``` code blocks (README demo + repo tree, manifesto
    # demo) — markers would render literally, so anchor on distinctive context.
    ("README.md", re.compile(r"(?<=plugin: aware-aeco · )(\d[\d,]*)(?= agents)"), "agents_total"),
    ("README.md", re.compile(r"(?<= agents · )(\d[\d,]*)(?= skills)"), "skills"),
    ("README.md", re.compile(r"(\d+)(?= agents · \d[\d,]* curated)"), "agents_total"),
    ("README.md", re.compile(r"(?<=· )(\d+)(?= curated \+)"), "agents_curated"),
    ("README.md", re.compile(r"(?<=\+ )(\d+)(?= reflected · all Apache)"), "agents_reflected"),
    ("00-vision/manifesto.md", re.compile(r"(?<=✓ )(\d+)(?= AECO agents available)"), "agents_total"),
    ("00-vision/manifesto.md", re.compile(r"(?<=aware-aeco · )(\d+)(?= agents available)"), "agents_total"),
]


def _thousands(n: int) -> str:
    return f"{n:,}"


def compute_stats() -> dict[str, str]:
    """Compute the canonical stat values from the repo. Pure read-only."""
    reg = json.loads((REPO / "registry-index.json").read_text(encoding="utf-8"))

    agents_root = REPO / "20-agents"
    gen_marker = re.compile(
        r"auto-generated|generated, raw|from-nuget|from-npm|from-yard|from-openapi",
        re.I,
    )
    manifests = curated = reflected = skills = commands = catalog = 0
    for dirpath, _dirnames, filenames in os.walk(agents_root):
        parts = Path(dirpath).parts
        in_skills = "skills" in parts
        in_commands = "commands" in parts
        in_catalog = "catalog" in parts
        for fn in filenames:
            if fn == "manifest.yaml":
                manifests += 1
                txt = (Path(dirpath) / fn).read_text(encoding="utf-8", errors="ignore")
                if gen_marker.search(txt):
                    reflected += 1
                else:
                    curated += 1
            elif in_skills and fn.endswith(".md"):
                skills += 1
            elif in_commands and fn.endswith(".md"):
                commands += 1
            elif in_catalog and fn.endswith(".json"):
                catalog += 1

    apps = sum(1 for _ in (REPO / "30-apps").rglob("*.flo"))
    core = agents_root / "_core"
    meta = sum(1 for p in core.iterdir() if (p / "manifest.yaml").is_file())
    decalog = len(
        re.findall(r"(?m)^##\s+\d+\.", (REPO / "00-vision" / "decalog.md").read_text(encoding="utf-8"))
    )
    cargo = (REPO / "cli" / "Cargo.toml").read_text(encoding="utf-8")
    m = re.search(r'(?m)^version\s*=\s*"([^"]+)"', cargo)
    cli_version = m.group(1) if m else "?"

    return {
        "agents_total": _thousands(manifests),
        "agents_registered": _thousands(len(reg["agents"])),
        "agents_curated": _thousands(curated),
        "agents_reflected": _thousands(reflected),
        "skills": _thousands(skills),
        "commands": _thousands(commands),
        "catalog": _thousands(catalog),
        "apps": _thousands(apps),
        "meta_primitives": _thousands(meta),
        "decalog_truths": _thousands(decalog),
        "cli_version": cli_version,
    }


def process_markers(text: str, stats: dict[str, str], path: str):
    """Return (new_text, mismatches) for one marker-managed file. Pure."""
    mismatches = []

    def repl(match: re.Match) -> str:
        key, current = match.group(1), match.group(2)
        expected = stats.get(key)
        if expected is None:
            mismatches.append((path, key, current, "<unknown stat key>"))
            return match.group(0)
        if current != expected:
            mismatches.append((path, key, current, expected))
        return f"<!--stat:{key}-->{expected}<!--/stat-->"

    return MARKER_RE.sub(repl, text), mismatches


def process_anchor(text: str, pattern: re.Pattern, expected: str, key: str, path: str):
    """Return (new_text, mismatches) for one anchor-rule file. Pure.

    Each anchor must match exactly once. Zero matches means the surrounding
    text drifted and this stat is silently no longer verified; more than one
    means the anchor is ambiguous. Both are reported as mismatches so coverage
    can never be lost without failing the check.
    """
    mismatches = []
    matches = pattern.findall(text)
    if len(matches) != 1:
        mismatches.append(
            (path, key, f"{len(matches)} anchor matches", "exactly 1 (anchor drifted)")
        )
        return text, mismatches
    current = pattern.search(text).group(1)
    if current != expected:
        mismatches.append((path, key, current, expected))
    return pattern.sub(lambda _m: expected, text), mismatches


def _read(path: Path) -> str:
    # newline="" preserves the file's existing line endings so --write never
    # churns CRLF/LF across the whole file — only the marked spans change.
    with open(path, "r", encoding="utf-8", newline="") as fh:
        return fh.read()


def _write(path: Path, text: str) -> None:
    with open(path, "w", encoding="utf-8", newline="") as fh:
        fh.write(text)


PLAYGROUND = "40-diagrams/substrate-playground.html"


def _committed_raw_agents(path: Path):
    """Parse the RAW_AGENTS dataset currently inlined in the playground."""
    m = re.search(r"const RAW_AGENTS = (\[.*?\]);", _read(path), re.DOTALL)
    return json.loads(m.group(1)) if m else None


def _expected_raw_agents():
    """Recompute the playground dataset from the tree by reusing the
    generator's own builders — so a change to any agent's skills, commands,
    vendor, vertical, etc. is caught, not just a change in the agent count."""
    import importlib.util

    spec = importlib.util.spec_from_file_location(
        "_playground_gen", REPO / "scripts" / "build-substrate-playground.py"
    )
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod.strip_payload(mod.build_agents())


def _playground_mismatch():
    """Return a (key, current, expected) tuple if the committed playground data
    is stale, else None."""
    committed = _committed_raw_agents(REPO / PLAYGROUND)
    expected = _expected_raw_agents()
    if committed == expected:
        return None
    if committed is None:
        return ("playground_data", "unparseable", "regenerate")
    if len(committed) != len(expected):
        return ("playground_agents", str(len(committed)), str(len(expected)))
    drifted = [e["id"] for c, e in zip(committed, expected) if c != e]
    return ("playground_data", f"stale ({', '.join(drifted[:3])} …)", "regenerate")


def _regen_playground() -> None:
    import subprocess

    subprocess.run(
        [sys.executable, str(REPO / "scripts" / "build-substrate-playground.py")],
        check=True,
        cwd=str(REPO),
        stdout=subprocess.DEVNULL,
    )


def run(write: bool) -> int:
    stats = compute_stats()
    mismatches: list[tuple] = []
    changed: list[str] = []

    for rel in MANAGED_FILES:
        path = REPO / rel
        text = _read(path)
        new, mm = process_markers(text, stats, rel)
        mismatches += mm
        if write and new != text:
            _write(path, new)
            changed.append(rel)

    for rel, pattern, key in ANCHOR_RULES:
        path = REPO / rel
        text = _read(path)
        new, mm = process_anchor(text, pattern, stats[key], key, rel)
        mismatches += mm
        if write and new != text:
            _write(path, new)
            changed.append(rel)

    # The playground inlines its own agent dataset. Verify the full parsed
    # dataset (not only the count) by recomputing it from the tree, so a change
    # to any agent's skills / commands / vendor / vertical is caught too.
    # Compared as parsed JSON, so it's OS- and line-ending-agnostic. --write
    # regenerates the file via the generator.
    if write:
        _regen_playground()
        synced = ", ".join(dict.fromkeys(changed + [PLAYGROUND])) or "(none)"
        print(f"sync_stats: synced docs + regenerated playground: {synced}")
        return 0

    pg = _playground_mismatch()
    if pg is not None:
        key, current, expected = pg
        mismatches.append((PLAYGROUND, key, current, expected))

    if mismatches:
        print("sync_stats: STALE / mismatched stats:\n")
        for path, key, current, expected in mismatches:
            print(f"  {path}: stat:{key} is '{current}' but should be '{expected}'")
        print("\nFix:  python scripts/sync_stats.py --write")
        return 1

    total = sum(len(MARKER_RE.findall(_read(REPO / rel))) for rel in MANAGED_FILES)
    total += len(ANCHOR_RULES) + 1  # + the playground agent-count check
    print(f"sync_stats: all {total} managed stats current ({len(stats)} keys).")
    return 0


def run_selftest() -> int:
    import unittest

    class T(unittest.TestCase):
        stats = {"agents_total": "66", "skills": "3,290"}

        def test_detects_stale(self):
            text = "We have <!--stat:agents_total-->39<!--/stat--> agents."
            new, mm = process_markers(text, self.stats, "x")
            self.assertEqual(len(mm), 1)
            self.assertIn("66", new)
            self.assertNotIn(">39<", new)

        def test_clean_passes(self):
            text = "We have <!--stat:agents_total-->66<!--/stat--> agents."
            new, mm = process_markers(text, self.stats, "x")
            self.assertEqual(mm, [])
            self.assertEqual(new, text)

        def test_idempotent(self):
            text = "<!--stat:skills-->1<!--/stat-->"
            once, _ = process_markers(text, self.stats, "x")
            twice, mm = process_markers(once, self.stats, "x")
            self.assertEqual(once, twice)
            self.assertEqual(mm, [])

        def test_unknown_key_flagged(self):
            text = "<!--stat:nope-->5<!--/stat-->"
            _, mm = process_markers(text, self.stats, "x")
            self.assertEqual(len(mm), 1)

        def test_unmarked_numbers_untouched(self):
            text = "began as 7 agents, now <!--stat:agents_total-->66<!--/stat-->"
            new, _ = process_markers(text, self.stats, "x")
            self.assertIn("began as 7 agents", new)

        def test_anchor_replace(self):
            pat = re.compile(r"(?<=Decalog<br/>)(\d+)(?= structural truths)")
            text = "Decalog<br/>5 structural truths"
            new, mm = process_anchor(text, pat, "9", "decalog_truths", "x")
            self.assertEqual(new, "Decalog<br/>9 structural truths")
            self.assertEqual(len(mm), 1)

        def test_anchor_zero_matches_flagged(self):
            # surrounding text drifted → anchor no longer matches → must fail,
            # never silently pass with lost coverage.
            pat = re.compile(r"(?<=Decalog<br/>)(\d+)(?= structural truths)")
            text = "the decalog has nine truths"
            new, mm = process_anchor(text, pat, "9", "decalog_truths", "x")
            self.assertEqual(new, text)
            self.assertEqual(len(mm), 1)

        def test_anchor_multiple_matches_flagged(self):
            pat = re.compile(r"(\d+)(?= cats)")
            text = "3 cats and 4 cats"
            new, mm = process_anchor(text, pat, "9", "k", "x")
            self.assertEqual(new, text)
            self.assertEqual(len(mm), 1)

        def test_compute_stats_shape(self):
            s = compute_stats()
            for key in (
                "agents_total", "agents_registered", "agents_curated",
                "agents_reflected", "skills", "commands", "catalog",
                "apps", "meta_primitives", "decalog_truths", "cli_version",
            ):
                self.assertIn(key, s)
                self.assertTrue(s[key])
            # registry and tree must agree (the invariant the last PR established)
            self.assertEqual(s["agents_total"], s["agents_registered"])

    suite = unittest.TestLoader().loadTestsFromTestCase(T)
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    return 0 if result.wasSuccessful() else 1


def main() -> int:
    args = set(sys.argv[1:])
    if "--selftest" in args:
        return run_selftest()
    if "--write" in args:
        return run(write=True)
    # default + explicit --check
    return run(write=False)


if __name__ == "__main__":
    sys.exit(main())
