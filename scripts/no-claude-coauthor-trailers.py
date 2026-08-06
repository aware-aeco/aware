#!/usr/bin/env python3
"""Fail a pull request whose commits or body carry a Claude co-author trailer.

CLAUDE.md §Git workflow: "**No `Co-Authored-By: Claude ...` trailers** in commit
messages." Nothing enforced it, and the rule is being broken on `main` right
now: 8 of the 50 commits reachable in a depth-50 clone carry
`Co-authored-by: Claude <noreply@anthropic.com>`, the most recent landing
2026-08-05 (b0ee11b5, #362). The others are 86a2bb71 (#361), 0f6eb6a2 (#357),
33a51385 (#356), 2a0bfe40 (#355), 4e1820ff (#346), 80abaaa6 (#345) and
8410abe2 (#344).

Those eight cannot be fixed. CLAUDE.md §Git workflow also makes `main`'s history
append-only ("no force-push, no rewrite"), so the only available fix is to stop
the ninth — which is what this script is.

## Where the trailer actually comes from

It is not typed into the squash message by hand. GitHub composes a squash commit
from the branch's commit messages, and propagates any `Co-authored-by:` trailer
it finds in them (or in the PR body) into the result — that is why every
offender above is a squash commit carrying GitHub's `---------` separator. So
the trailer has to be caught on the *branch*, before the merge button: by the
time it is on `main` it is unfixable.

Both sources are therefore checked:

  * every commit in the PR range (`--range BASE..HEAD`), and
  * the PR body (`--message-file`), which GitHub also mines for trailers.

## What counts as a trailer

Git's own definition, via `git interpret-trailers --parse`, rather than a
hand-rolled regex over every line. A commit body that *discusses* the rule — as
this change's own commit message does — is prose, not a trailer, and git says so
without this script needing to guess. The rule is about attribution metadata,
and attribution metadata is exactly what git parses.

Known limit, stated rather than hidden: a `Co-authored-by:` line stranded in the
middle of a message is not a trailer to git, so it is not flagged here either.
That is not a hole — it is not a trailer to GitHub either, so it never reaches
`main` as attribution.

## What is flagged, and what is not

Only Claude/Anthropic co-authors. CLAUDE.md forbids `Co-Authored-By: Claude`,
not co-authorship as such: a human pair-programming partner is a legitimate
trailer and flagging it would be the false-positive pressure that gets a gate
ripped out. A trailer is ours when either its name or its email names Claude or
Anthropic.

## Usage

    python3 scripts/no-claude-coauthor-trailers.py --self-test
    python3 scripts/no-claude-coauthor-trailers.py --range main..HEAD
    python3 scripts/no-claude-coauthor-trailers.py --message-file pr-body.txt

`--self-test` is the negative control: it runs the classifier over fixtures with
known answers — including the verbatim trailer from b0ee11b5 — so a refactor
that quietly stops matching anything fails loudly instead of reporting a clean
branch.
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys

# The trailer key CLAUDE.md names. Git treats trailer keys case-insensitively
# and so does GitHub, so `Co-Authored-By`, `Co-authored-by` and `co-authored-by`
# are one key — matching only the spelling in CLAUDE.md would leave two ways past.
COAUTHOR_KEY = "co-authored-by"

# Identity fragments that make a co-author *ours*. Matched against the whole
# trailer value (name and email together), case-insensitively.
#
# `anthropic` catches the address every offending commit used
# (`noreply@anthropic.com`) even when the display name is something else;
# `claude` catches the display name even when the address is not Anthropic's.
# Either alone would leave a spelling through.
_CLAUDE_IDENTITY = re.compile(r"claude|anthropic", re.IGNORECASE)


def is_claude_coauthor(key: str, value: str) -> bool:
    """Whether a parsed `(key, value)` trailer is a Claude co-author trailer."""
    if key.strip().lower() != COAUTHOR_KEY:
        return False
    return _CLAUDE_IDENTITY.search(value) is not None


def parse_trailers(message: str) -> list[tuple[str, str]]:
    """The `(key, value)` trailers of `message`, per git's own parser.

    Shelling out to `git interpret-trailers` rather than reimplementing the
    trailer-block heuristic keeps this agreeing with what git — and therefore
    GitHub — actually treats as attribution.
    """
    result = subprocess.run(
        ["git", "interpret-trailers", "--parse"],
        input=message,
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError(
            f"git interpret-trailers failed ({result.returncode}): {result.stderr.strip()}"
        )
    trailers = []
    for line in result.stdout.splitlines():
        key, separator, value = line.partition(":")
        if separator:
            trailers.append((key, value.strip()))
    return trailers


def offending_trailers(message: str) -> list[str]:
    """The rendered Claude co-author trailers in `message` (empty when clean)."""
    return [
        f"{key.strip()}: {value}"
        for key, value in parse_trailers(message)
        if is_claude_coauthor(key, value)
    ]


def commits_in_range(rev_range: str) -> list[tuple[str, str, str]]:
    """`(sha, subject, full message)` for each commit in `rev_range`."""
    listed = subprocess.run(
        ["git", "rev-list", rev_range],
        capture_output=True,
        text=True,
        check=False,
    )
    if listed.returncode != 0:
        raise RuntimeError(
            f"git rev-list {rev_range} failed: {listed.stderr.strip()}\n"
            "A shallow clone cannot see the merge base — fetch with "
            "`fetch-depth: 0` before running this."
        )

    commits = []
    for sha in listed.stdout.split():
        shown = subprocess.run(
            ["git", "log", "-1", "--format=%B", sha],
            capture_output=True,
            text=True,
            check=True,
        )
        subject = subprocess.run(
            ["git", "log", "-1", "--format=%s", sha],
            capture_output=True,
            text=True,
            check=True,
        )
        commits.append((sha, subject.stdout.strip(), shown.stdout))
    return commits


# ── Negative control ─────────────────────────────────────────────────────────

# `(message, expected offender count)`. The classifier is the whole gate, so
# every judgement it makes has a fixture pinning it.
SELF_TEST_CASES: list[tuple[str, int]] = [
    # The verbatim tail of b0ee11b5 (#362), the most recent commit to break the
    # rule. If this stops being flagged, the gate has stopped working.
    (
        "feat(cli): enforce an app's `requires:` agent-version pins\n"
        "\n"
        "Refs #349\n"
        "\n"
        "---------\n"
        "\n"
        "Co-authored-by: Claude <noreply@anthropic.com>\n",
        1,
    ),
    # The spelling CLAUDE.md itself uses, and the lower-case one git emits.
    ("subject\n\nCo-Authored-By: Claude <noreply@anthropic.com>\n", 1),
    ("subject\n\nco-authored-by: claude <noreply@anthropic.com>\n", 1),
    # A display name that hides the tool still has the address.
    ("subject\n\nCo-authored-by: assistant <noreply@anthropic.com>\n", 1),
    # …and an address that hides it still has the name.
    ("subject\n\nCo-authored-by: Claude Opus 5 <bot@users.noreply.github.com>\n", 1),
    # Several in one message are all reported, not just the first.
    (
        "subject\n"
        "\n"
        "Co-authored-by: Claude <noreply@anthropic.com>\n"
        "Co-authored-by: Claude Code <noreply@anthropic.com>\n",
        2,
    ),
    # A human co-author is legitimate — CLAUDE.md forbids the Claude trailer,
    # not co-authorship. Flagging this is the false-positive pressure that gets
    # a gate ripped out, so it is pinned as explicitly allowed.
    ("subject\n\nCo-authored-by: Pawel Lisowski <pawellisowski@o2.pl>\n", 0),
    # A mixed block reports only ours.
    (
        "subject\n"
        "\n"
        "Co-authored-by: Pawel Lisowski <pawellisowski@o2.pl>\n"
        "Co-authored-by: Claude <noreply@anthropic.com>\n",
        1,
    ),
    # Other trailers naming Claude are not co-authorship and are not the rule's
    # business — `Reviewed-by` records who reviewed, which is a true fact.
    ("subject\n\nReviewed-by: Claude <noreply@anthropic.com>\n", 0),
    # Prose that discusses the rule is not a trailer. This change's own commit
    # message is exactly this shape, so a gate that flagged it could not be
    # landed by the commit that introduces it.
    (
        "chore(ci): reject Claude co-author trailers\n"
        "\n"
        "CLAUDE.md forbids `Co-Authored-By: Claude ...` trailers and nothing\n"
        "enforced it. This adds the check.\n"
        "\n"
        "Refs #342\n",
        0,
    ),
    # A clean message with no trailer block at all.
    ("fix(cli): a subject line and nothing else\n", 0),
    # An empty message must not crash the parser.
    ("", 0),
]

# `(key, value, expected)` for the pure predicate, so its contract is pinned
# independently of git's parser.
_PREDICATE_CASES: list[tuple[str, str, bool]] = [
    ("Co-authored-by", "Claude <noreply@anthropic.com>", True),
    ("co-authored-by", "CLAUDE <x@y.z>", True),
    ("Co-Authored-By", "someone <bot@anthropic.com>", True),
    ("Co-authored-by", "Pawel Lisowski <pawellisowski@o2.pl>", False),
    ("Signed-off-by", "Claude <noreply@anthropic.com>", False),
    ("Reviewed-by", "Claude <noreply@anthropic.com>", False),
]


def self_test() -> int:
    failures = []

    for key, value, expected in _PREDICATE_CASES:
        actual = is_claude_coauthor(key, value)
        if actual != expected:
            failures.append(
                f"  is_claude_coauthor({key!r}, {value!r}): "
                f"expected {expected}, got {actual}"
            )

    for message, expected in SELF_TEST_CASES:
        found = offending_trailers(message)
        if len(found) != expected:
            first_line = message.splitlines()[0] if message else "<empty>"
            failures.append(
                f"  {first_line!r}: expected {expected} offender(s), "
                f"got {len(found)}: {found}"
            )

    if failures:
        print("self-test FAILED — the classifier no longer matches its contract:")
        print("\n".join(failures))
        return 1
    print(
        f"self-test ok ({len(_PREDICATE_CASES)} predicate cases + "
        f"{len(SELF_TEST_CASES)} message cases)"
    )
    return 0


# ── Check ────────────────────────────────────────────────────────────────────

REMEDY = """
Remove the trailer and re-push:

  * one commit  — `git commit --amend` and delete the line, then force-push the
    branch (a topic branch, never `main`);
  * several     — `git rebase -i <base>`, mark each offender `reword`, delete
    the line in each editor;
  * PR body     — edit the description on GitHub. Trailers there are copied into
    the squash commit too, so a clean branch is not enough on its own.

Do not work around this by disabling the check: CLAUDE.md §Engineering rules
forbids satisfying a gate by weakening it.
""".rstrip()


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Reject Claude co-author trailers (CLAUDE.md §Git workflow)."
    )
    parser.add_argument("--self-test", action="store_true", help="run the negative control")
    parser.add_argument("--range", help="commit range to check, e.g. main..HEAD")
    parser.add_argument(
        "--message-file",
        help="a file holding one message to check (the PR body)",
    )
    args = parser.parse_args()

    if args.self_test:
        return self_test()

    if not args.range and not args.message_file:
        parser.error("nothing to check: pass --range and/or --message-file")

    offenders = []

    if args.range:
        for sha, subject, message in commits_in_range(args.range):
            for trailer in offending_trailers(message):
                offenders.append(f"commit {sha[:8]} ({subject}): {trailer}")

    if args.message_file:
        with open(args.message_file, encoding="utf-8") as handle:
            body = handle.read()
        for trailer in offending_trailers(body):
            offenders.append(f"pull request body: {trailer}")

    if offenders:
        print(
            "error: CLAUDE.md §Git workflow forbids `Co-Authored-By: Claude ...` "
            "trailers, and these would reach `main` in the squash commit:\n",
            file=sys.stderr,
        )
        for offender in offenders:
            print(f"  {offender}", file=sys.stderr)
        print(REMEDY, file=sys.stderr)
        return 1

    checked = []
    if args.range:
        checked.append(f"commits in {args.range}")
    if args.message_file:
        checked.append("the pull request body")
    print(f"ok: no Claude co-author trailers in {' or '.join(checked)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
