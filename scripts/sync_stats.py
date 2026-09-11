#!/usr/bin/env python3
"""
sync_stats.py — keep the substrate's stat numbers in sync across the docs.

Single source of truth (all computed, never hand-counted):
  - registry-index.json         → registered-agent count
  - 20-agents/ tree             → manifests, curated/reflected split, skills,
                                   commands, catalog entries, meta-primitives
  - 30-apps/_examples/*.{flo,app} → reference-app count (top-level only)
  - 00-vision/decalog.md        → number of structural truths
  - cli/Cargo.toml              → authoritative CLI version
  - cli-npm/package.json        → checked npm-wrapper mirror of CLI version

Each managed number in the docs is wrapped in an invisible marker:

    <!--stat:agents_total-->66<!--/stat-->

The marker is an HTML comment, so it does not render in Markdown. Only the text
between the markers is ever rewritten — anything unmarked (e.g. the narrative
"began as 7 reference agents") is never touched. That opt-in contract is what
keeps the sync from clobbering intentional/historical numbers.

For files where HTML comments can't hide (Mermaid `.mmd`), a small ANCHOR_RULES
list does a single targeted regex replacement instead.

Usage (run from the repo root):
    python scripts/sync_stats.py --check       # verify; exit 1 + diff if stale
    python scripts/sync_stats.py --write        # rewrite the docs in place
    python scripts/sync_stats.py --bump 0.89.0  # set Cargo/npm versions + sync docs
    python scripts/sync_stats.py --selftest     # unit-test the pure logic

`--bump` is the release seam: it welds the authoritative Cargo version, the npm
wrapper mirror, and the doc mirror into one command. A release therefore cannot
land with either consumer-facing version stale.
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

# --bump guard: refuse anything that isn't a plain semver, so a fat-fingered
# arg can never be written into Cargo.toml.
SEMVER_RE = re.compile(r"^\d+\.\d+\.\d+([-+.][0-9A-Za-z.-]+)?$")

# What .github/workflows/tag.yml will actually accept, which is NARROWER: its
# `case "$VERSION" in "" | *[!0-9.]*)` alphabet guard and the
# `^[0-9]+\.[0-9]+\.[0-9]+$` shape check below it both reject a prerelease or
# build suffix. That is deliberate — the value reaches GITHUB_ENV in a job
# holding a write-capable token, and the narrow alphabet is the wall that stops
# a newline smuggling a second assignment in — so it is mirrored here rather
# than widened from this side. --bump accepts the wider grammar and release.yml
# ships hyphenated tags as prereleases, so the two genuinely differ, and the
# finish-the-release text has to say which route a given version actually has.
# test_tag_workflow_still_refuses_prerelease_versions fails if tag.yml's shape
# check moves, so this constant cannot quietly go stale.
TAG_WORKFLOW_VERSION_RE = re.compile(r"^\d+\.\d+\.\d+$")

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

    examples = REPO / "30-apps" / "_examples"
    apps = sum(1 for p in examples.iterdir() if p.is_file() and p.suffix in (".flo", ".app"))
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
NPM_PACKAGE = "cli-npm/package.json"


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


def collect_mismatches(stats: dict[str, str]) -> list[tuple]:
    """Read-only: every managed stat currently out of sync. The playground is
    verified against its full recomputed dataset, not just the agent count."""
    mismatches: list[tuple] = []
    for rel in MANAGED_FILES:
        _, mm = process_markers(_read(REPO / rel), stats, rel)
        mismatches += mm
    for rel, pattern, key in ANCHOR_RULES:
        _, mm = process_anchor(_read(REPO / rel), pattern, stats[key], key, rel)
        mismatches += mm
    pg = _playground_mismatch()
    if pg is not None:
        mismatches.append((PLAYGROUND, *pg))
    npm_version = npm_package_version(_read(REPO / NPM_PACKAGE))
    if npm_version != stats["cli_version"]:
        mismatches.append(
            (NPM_PACKAGE, "cli_version", npm_version, stats["cli_version"])
        )
    return mismatches


def apply_writes(stats: dict[str, str]) -> list[str]:
    """Rewrite every fixable marker/anchor and regenerate the playground.
    A drifted anchor or unknown marker key can't be auto-fixed here — those are
    surfaced by the verification pass in run()."""
    changed: list[str] = []
    for rel in MANAGED_FILES:
        text = _read(REPO / rel)
        new, _ = process_markers(text, stats, rel)
        if new != text:
            _write(REPO / rel, new)
            changed.append(rel)
    for rel, pattern, key in ANCHOR_RULES:
        text = _read(REPO / rel)
        new, _ = process_anchor(text, pattern, stats[key], key, rel)
        if new != text:
            _write(REPO / rel, new)
            changed.append(rel)
    _regen_playground()
    changed.append(PLAYGROUND)
    npm_path = REPO / NPM_PACKAGE
    npm_text = _read(npm_path)
    npm_new = bump_npm_package_version(npm_text, stats["cli_version"])
    if npm_new != npm_text:
        _write(npm_path, npm_new)
        changed.append(NPM_PACKAGE)
    return changed


def bump_package_version(cargo_text: str, new_version: str) -> str:
    """Return cli/Cargo.toml text with the [package] version set to new_version.

    Only the line-start `version = "..."` is rewritten. Dependency versions are
    inline (`{ version = ... }`) and never match `^version`, so they're safe —
    this is the same anchor compute_stats() reads the version through. Exactly
    one line-start version must exist, else we fail loudly rather than bump the
    wrong line. Pure.
    """
    new, n = re.subn(
        r'(?m)^(version\s*=\s*")[^"]+(")',
        lambda m: f"{m.group(1)}{new_version}{m.group(2)}",
        cargo_text,
    )
    if n != 1:
        raise ValueError(f"expected exactly one [package] version line, found {n}")
    return new


def npm_package_version(package_text: str) -> str:
    """Read the npm wrapper's authoritative package version, failing loudly.

    The npm shim uses this exact field to choose both the GitHub release it
    downloads and the versioned rescue directory it will execute, so a missing
    or non-string value is not a tolerable default.
    """
    package = json.loads(package_text)
    if not isinstance(package, dict) or not isinstance(package.get("version"), str):
        raise ValueError("cli-npm/package.json needs a string top-level version")
    return package["version"]


def bump_npm_package_version(package_text: str, new_version: str) -> str:
    """Return package.json with only its top-level version changed.

    Serializing through JSON mirrors release.yml's Node rewrite and avoids a
    regex accidentally changing a nested field. The first rewrite normalizes
    formatting; later bumps touch only the version line.
    """
    package = json.loads(package_text)
    if not isinstance(package, dict) or not isinstance(package.get("version"), str):
        raise ValueError("cli-npm/package.json needs a string top-level version")
    package["version"] = new_version
    return json.dumps(package, indent=2, ensure_ascii=False) + "\n"


def finish_release_instructions(version: str) -> str:
    """What to do NEXT, printed where the bump happens.

    A bump is half a release: nothing builds or publishes until the tag exists,
    because release.yml fires on the tag push alone. The half that goes missing
    is always the tag, and always for the same reason — a sandboxed session is
    refused 403 on `git push origin vX.Y.Z`, on POST /git/refs and on POST
    /releases, reads that as "no route exists", and stops with main claiming a
    version that was never built. That is v0.131.1 on 2026-08-28, which is why
    .github/workflows/tag.yml was written (#474), and v0.137.0 on 2026-09-10
    (#524), where the same wall was hit again and the fix proposed was to build
    tag.yml a second time — nothing in the repo named it, so it was never found.

    Naming it here puts the route at the one moment every release passes
    through. Pure, so the selftest can read the text without bumping anything.

    The fallback is version-dependent, and saying so is the point: --bump takes
    a wider grammar than tag.yml does, so a prerelease bump has no dispatch
    route and must be told that rather than sent at a workflow that will refuse
    it. Advertising a route that cannot run is worse than naming none.
    """
    if TAG_WORKFLOW_VERSION_RE.match(version):
        fallback = f"""\
it is already solved. Do NOT re-bump and do NOT conclude there is no route.
Dispatch .github/workflows/tag.yml, which creates the ref from inside Actions
with the runner's own token and then dispatches release.yml at it:

     gh workflow run tag.yml -f version={version} -f sha=<sha>

  ...or, where the GitHub MCP server is the only transport that works, its
  run_workflow method on tag.yml with those same inputs.

tag.yml re-checks every release guard before it writes anything (plain semver,
a commit on the default branch, the manifests at that commit already carrying
this version, CI green on that exact sha, no existing tag of that name
elsewhere), so a wrong argument is refused rather than immortalised in a tag."""
    else:
        fallback = f"""\
the usual fallback does NOT cover this version. .github/workflows/tag.yml takes
plain X.Y.Z only — its input guard rejects '{version}' outright — so there is no
dispatch route for a prerelease or build-metadata bump, and it is honest to say
so rather than send you at a workflow that will refuse the argument.

v{version} has to be pushed from a checkout that holds tag-push rights. (The
tag itself is fine downstream: release.yml marks a hyphenated tag as a
prerelease.) Widening tag.yml's guard to match would work, but that guard is
what keeps a crafted input out of GITHUB_ENV in a job holding a write-capable
token, so it is a change to make deliberately and review on its own."""

    return f"""
sync_stats: the bump is half a release — nothing builds or publishes until the
tag exists. Next:

  1. Land this commit on the default branch and wait for CI to go green ON IT.
  2. Tag it:  git tag v{version} <sha> && git push origin v{version}

If that push comes back 403 — every sandboxed session does, and POST /git/refs
and POST /releases are refused there too — that is the sandbox, not GitHub, and
{fallback}

Do not dispatch release.yml from a branch: its version comes from the ref name,
so it stages 0.0.0-dev, and its release and npm jobs never run off a tag."""


def run_bump(version: str) -> int:
    """Set the authoritative Cargo version, then --write so the npm wrapper
    and cli_version doc stat land in the same breath. Cargo.lock is refreshed
    by the caller's `cargo build`."""
    if not SEMVER_RE.match(version):
        print(f"sync_stats: --bump needs a semver version like 0.89.0 (got '{version}')")
        return 2
    path = REPO / "cli" / "Cargo.toml"
    _write(path, bump_package_version(_read(path), version))
    print(f"sync_stats: bumped cli/Cargo.toml → {version}")
    rc = run(write=True)
    # Only on success: a bump whose doc sync failed is not ready to be tagged,
    # and telling it to go tag anyway is how a half-synced tree gets released.
    if rc == 0:
        print(finish_release_instructions(version))
    return rc


def _report(mismatches: list[tuple]) -> None:
    for path, key, current, expected in mismatches:
        print(f"  {path}: stat:{key} is '{current}' but should be '{expected}'")


def run(write: bool) -> int:
    stats = compute_stats()

    if write:
        changed = apply_writes(stats)
        # Verify after writing: anything still mismatched couldn't be auto-fixed
        # (drifted anchor context / unknown key), so --write must NOT report
        # success and leave the next --check failing.
        remaining = collect_mismatches(stats)
        if remaining:
            print("sync_stats: --write could not fully sync — manual edit needed:\n")
            _report(remaining)
            print("\n(A drifted anchor context or an unknown <!--stat:KEY--> can't be auto-fixed.)")
            return 1
        print(f"sync_stats: synced docs + regenerated playground: {', '.join(dict.fromkeys(changed))}")
        return 0

    mismatches = collect_mismatches(stats)
    if mismatches:
        print("sync_stats: STALE / mismatched stats:\n")
        _report(mismatches)
        print("\nFix:  python scripts/sync_stats.py --write")
        return 1

    total = sum(len(MARKER_RE.findall(_read(REPO / rel))) for rel in MANAGED_FILES)
    total += len(ANCHOR_RULES) + 2  # + playground data + npm version mirror
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

        def test_bump_touches_only_package_version(self):
            cargo = (
                "[package]\n"
                'version     = "0.88.0"\n\n'
                "[dependencies]\n"
                'clap = { version = "4.5", features = ["derive"] }\n'
            )
            new = bump_package_version(cargo, "0.89.0")
            self.assertIn('version     = "0.89.0"', new)
            self.assertIn('clap = { version = "4.5"', new)  # dep untouched

        def test_bump_requires_exactly_one_package_version(self):
            with self.assertRaises(ValueError):
                bump_package_version('[dependencies]\nx = 1\n', "0.89.0")

        def test_npm_bump_changes_only_the_top_level_version(self):
            package = '{"name":"@aware-aeco/cli","version":"0.41.0","scripts":{"version":"leave-me"}}\n'
            new = bump_npm_package_version(package, "0.89.0")
            parsed = json.loads(new)
            self.assertEqual(parsed["version"], "0.89.0")
            self.assertEqual(parsed["scripts"]["version"], "leave-me")
            self.assertEqual(npm_package_version(new), "0.89.0")

        def test_npm_version_requires_a_string_top_level_field(self):
            with self.assertRaises(ValueError):
                npm_package_version('{"name":"@aware-aeco/cli"}')
            with self.assertRaises(ValueError):
                bump_npm_package_version('{"version":129}', "0.89.0")

        def test_semver_guard(self):
            self.assertTrue(SEMVER_RE.match("0.89.0"))
            self.assertTrue(SEMVER_RE.match("1.0.0-rc.1"))
            self.assertFalse(SEMVER_RE.match("v0.89"))
            self.assertFalse(SEMVER_RE.match(""))

        def test_bump_names_both_tag_routes_and_the_version(self):
            text = finish_release_instructions("0.89.0")
            # The plain route, and the one every sandboxed release actually
            # needs. Both carry the version, so neither can be pasted blank.
            self.assertIn("git push origin v0.89.0", text)
            self.assertIn("gh workflow run tag.yml -f version=0.89.0", text)

        def test_bump_points_only_at_workflows_that_exist(self):
            # The instruction is worth no more than the workflow it names. Read
            # the names back OUT of the text rather than restating them here, so
            # renaming or deleting tag.yml turns this red instead of leaving the
            # next release pointed at a file that is not there. Both grammars
            # --bump accepts, since each prints a different fallback.
            for version in ("0.89.0", "1.0.0-rc.1"):
                text = finish_release_instructions(version)
                named = set(re.findall(r"[\w.-]+\.yml", text))
                self.assertIn("tag.yml", named)
                for name in named:
                    self.assertTrue(
                        (REPO / ".github" / "workflows" / name).is_file(),
                        f"--bump points at .github/workflows/{name}, which does not exist",
                    )

        def test_bump_does_not_advertise_a_dispatch_tag_yml_would_refuse(self):
            # --bump accepts 1.0.0-rc.1; tag.yml does not. Sending a prerelease
            # release at a dispatch that rejects the argument would stall it in
            # exactly the way this whole change exists to prevent, so the text
            # must name the limit and drop the command.
            text = finish_release_instructions("1.0.0-rc.1")
            self.assertNotIn("gh workflow run tag.yml", text)
            self.assertIn("plain X.Y.Z only", text)
            # The direct route still has to be there, and still carry the version.
            self.assertIn("git push origin v1.0.0-rc.1", text)

        def test_tag_workflow_still_refuses_prerelease_versions(self):
            # TAG_WORKFLOW_VERSION_RE is a copy of a guard that lives in another
            # file, and a copy is a lie waiting to happen. If tag.yml's shape
            # check moves, this goes red and the constant — and the fallback
            # text that branches on it — get revisited rather than going stale.
            workflow = _read(REPO / ".github" / "workflows" / "tag.yml")
            # assertTrue, not assertIn: the haystack is the whole workflow, and
            # assertIn would print all 229 lines of it on failure.
            self.assertTrue(
                r"^[0-9]+\.[0-9]+\.[0-9]+$" in workflow,
                "tag.yml's version shape check moved — recheck TAG_WORKFLOW_VERSION_RE "
                "and the prerelease branch of finish_release_instructions",
            )
            self.assertFalse(TAG_WORKFLOW_VERSION_RE.match("1.0.0-rc.1"))
            self.assertTrue(TAG_WORKFLOW_VERSION_RE.match("0.89.0"))

        def test_tag_workflow_still_takes_the_inputs_the_bump_prints(self):
            # The printed dispatch passes `version` and `sha`. If tag.yml stops
            # declaring either input, that command is one GitHub rejects — so
            # read tag.yml's own inputs block rather than trusting the text.
            workflow = _read(REPO / ".github" / "workflows" / "tag.yml")
            block = re.search(r"^\s+inputs:\n(.*?)(?=^\S)", workflow, re.DOTALL | re.M)
            self.assertIsNotNone(block, "tag.yml declares no workflow_dispatch inputs")
            body = block.group(1)
            # Only the shallowest keys are the input NAMES; anything deeper is
            # their description/type/default. Read the indent off the file so a
            # reformat does not turn this red on its own.
            indents = {m.group(1) for m in re.finditer(r"^( +)\w+:", body, re.M)}
            top = min(indents, key=len)
            declared = set(re.findall(rf"^{top}(\w+):", body, re.M))
            for name in ("version", "sha"):
                self.assertIn(name, declared, f"tag.yml no longer declares a '{name}' input")

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
    argv = sys.argv[1:]
    args = set(argv)
    if "--selftest" in args:
        return run_selftest()
    if "--bump" in args:
        i = argv.index("--bump")
        return run_bump(argv[i + 1] if i + 1 < len(argv) else "")
    if "--write" in args:
        return run(write=True)
    # default + explicit --check
    return run(write=False)


if __name__ == "__main__":
    sys.exit(main())
