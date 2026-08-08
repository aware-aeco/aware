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
it finds in them (or in the PR body) into the result. Six of the eight carry
GitHub's `---------` separator, which is how it joins several commit messages
into one; the two that do not (2a0bfe40, 8410abe2) came from single-commit
branches, where there is nothing to join. Either way the trailer entered on the
branch, not at the merge button.

Both sources are therefore checked:

  * every commit in the PR range (`--range`), and
  * the PR body (`--message-file`), which GitHub also mines for trailers.

## What this does not reach

Stated plainly, because a gate that is described as absolute gets trusted like
one:

  * a **direct push to `main`** — this runs on `pull_request` only, and CLAUDE.md
    permits direct pushes with approval;
  * a **`--admin` merge**, which CLAUDE.md §Git workflow makes the repo's normal
    merge mechanism, and which bypasses required status checks — a red run here
    does not mechanically block it, only the procedural "CI is green" rule does;
  * the **squash message edited in the merge dialog**, which nothing re-reads.

What it does reach is the path every one of the eight offenders actually took.

## What counts as a trailer

Git's own definition, via `git interpret-trailers --parse`, rather than a
hand-rolled regex over every line. A commit body that *discusses* the rule — as
this change's own commit message does — is prose, not a trailer, and git says so
without this script needing to guess. The rule is about attribution metadata,
and attribution metadata is exactly what git parses.

Known limit: a `Co-authored-by:` line that git does not parse as a trailer
(stranded mid-message, indented, or with no blank line before it) is not flagged
here. Whether GitHub's own parser agrees with git's on those forms has not been
verified, so this is a genuine gap rather than a proven non-issue — it is narrow
because it is not the form any tooling emits.

## Which co-authors are flagged

Only Claude. CLAUDE.md forbids `Co-Authored-By: Claude`, not co-authorship as
such: a human pair-programming partner is a legitimate trailer, and flagging one
would be the false-positive pressure that gets a gate ripped out — the remedy
this prints would then be telling somebody to delete a real person's
attribution. So a trailer is ours when its *name* or *email* names Claude, or
when its address is Anthropic automation (`noreply@anthropic.com`).

Deliberately NOT flagged: a human at an Anthropic address (`colah@anthropic.com`
is a person, not a bot), and a human whose name merely contains the string —
`Jean-Claude` is a common French given name and is excluded by requiring
`claude` to start a word.

The one false positive left: a contributor whose given name is literally Claude.
That is not fixable by pattern alone, and the failure text says so rather than
telling them to erase their own attribution.

## Usage

    python3 scripts/no-claude-coauthor-trailers.py --self-test
    python3 scripts/no-claude-coauthor-trailers.py --range origin/main..HEAD
    python3 scripts/no-claude-coauthor-trailers.py --message-file pr-body.txt

Note `origin/main`, not `main`. A stale local `main` puts the eight unfixable
commits above into the range and reports eight failures nobody can act on.

`--self-test` is the negative control. It covers both halves of the gate — the
classifier *and* the commit-extraction path — because a control that only
exercises the classifier stays green while a refactor of the other half leaves
the gate completely blind.
"""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
import tempfile

# The trailer key CLAUDE.md names. Git treats trailer keys case-insensitively
# and so does GitHub, so `Co-Authored-By`, `Co-authored-by` and `co-authored-by`
# are one key — matching only the spelling in CLAUDE.md would leave two ways past.
COAUTHOR_KEY = "co-authored-by"

# `claude` starting a word. The leading `(?<![\w-])` is what keeps `Jean-Claude`
# out: there, `claude` is preceded by a hyphen, so it is part of somebody's name
# rather than the tool's. `claude-code` and `Claude Opus 5` still match, because
# there the word *starts*.
_CLAUDE_WORD = re.compile(r"(?<![\w-])claude", re.IGNORECASE)

# Anthropic automation, as an address. Scoped to no-reply/bot local parts on
# purpose: `@anthropic.com` alone would flag human employees, whose co-authorship
# is as legitimate as anyone else's.
_ANTHROPIC_AUTOMATION = re.compile(
    r"^(?:noreply|no-reply|[^@]*bot)@(?:[^@]*\.)?anthropic\.com$",
    re.IGNORECASE,
)

# `Name <email>`, the form git writes and GitHub reads. A value that does not
# match is treated as all name, so an address-less trailer is still judged.
_IDENTITY = re.compile(r"^(?P<name>.*?)\s*<(?P<email>[^>]*)>\s*$")


def split_identity(value: str) -> tuple[str, str]:
    """`(name, email)` for a trailer value; email is `""` when there is none."""
    match = _IDENTITY.match(value.strip())
    if not match:
        return value.strip(), ""
    return match.group("name").strip(), match.group("email").strip()


def is_claude_coauthor(key: str, value: str) -> bool:
    """Whether a parsed `(key, value)` trailer is a Claude co-author trailer."""
    if key.strip().lower() != COAUTHOR_KEY:
        return False
    name, email = split_identity(value)
    if _CLAUDE_WORD.search(name) or _CLAUDE_WORD.search(email):
        return True
    return _ANTHROPIC_AUTOMATION.match(email) is not None


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
        # `partition` splits on the FIRST colon, so a value containing one (a
        # URL) keeps it: `See-also: https://…` → `('See-also', 'https://…')`.
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


def commits_in_range(rev_range: str, cwd: str | None = None) -> list[tuple[str, str, str]]:
    """`(sha, subject, full message)` for each commit in `rev_range`."""
    listed = subprocess.run(
        ["git", "rev-list", rev_range],
        capture_output=True,
        text=True,
        check=False,
        cwd=cwd,
    )
    if listed.returncode != 0:
        raise RuntimeError(
            f"git rev-list {rev_range} failed: {listed.stderr.strip()}\n"
            "A shallow clone cannot see the merge base — fetch with "
            "`fetch-depth: 0` before running this."
        )

    commits = []
    for sha in listed.stdout.split():
        # `%B` is the RAW BODY — subject *and* message body. `%s` would be the
        # subject alone, and a trailer never lives in the subject, so that swap
        # leaves this reporting every branch clean. `SELF_TEST` covers it.
        message = subprocess.run(
            ["git", "log", "-1", "--format=%B", sha],
            capture_output=True,
            text=True,
            check=True,
            cwd=cwd,
        )
        subject = subprocess.run(
            ["git", "log", "-1", "--format=%s", sha],
            capture_output=True,
            text=True,
            check=True,
            cwd=cwd,
        )
        commits.append((sha, subject.stdout.strip(), message.stdout))
    return commits


# ── Negative control ─────────────────────────────────────────────────────────

# `(message, expected offender count)`. The classifier is half the gate, so
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
    # The tail of 8410abe2 (#344) — a single-commit branch, so GitHub had
    # nothing to join and emitted no `---------`. Both shapes are pinned.
    ("subject\n\nbody\n\nCo-authored-by: Claude <noreply@anthropic.com>\n", 1),
    # The spelling CLAUDE.md itself uses, and the lower-case one git emits.
    ("subject\n\nCo-Authored-By: Claude <noreply@anthropic.com>\n", 1),
    ("subject\n\nco-authored-by: claude <noreply@anthropic.com>\n", 1),
    # A display name that hides the tool still has the automation address.
    ("subject\n\nCo-authored-by: assistant <noreply@anthropic.com>\n", 1),
    # …and an address that hides it still has the name.
    ("subject\n\nCo-authored-by: Claude Opus 5 <bot@users.noreply.github.com>\n", 1),
    # The name is in the address only.
    ("subject\n\nCo-authored-by: <claude-code[bot]@users.noreply.github.com>\n", 1),
    # Several in one message are all reported, not just the first.
    (
        "subject\n"
        "\n"
        "Co-authored-by: Claude <noreply@anthropic.com>\n"
        "Co-authored-by: Claude Code <noreply@anthropic.com>\n",
        2,
    ),
    # A human co-author is legitimate — CLAUDE.md forbids the Claude trailer,
    # not co-authorship. Flagging these is the false-positive pressure that gets
    # a gate ripped out, so they are pinned as explicitly allowed.
    ("subject\n\nCo-authored-by: Pawel Lisowski <pawellisowski@o2.pl>\n", 0),
    # `Jean-Claude` is a given name, not the tool.
    ("subject\n\nCo-authored-by: Jean-Claude Meunier <jc.meunier@gmail.com>\n", 0),
    # A human at an Anthropic address is a person, not automation.
    ("subject\n\nCo-authored-by: Chris Olah <colah@anthropic.com>\n", 0),
    # An unrelated domain that merely contains the string.
    ("subject\n\nCo-authored-by: Ana Diaz <ana@anthropic-partners.example>\n", 0),
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
    # A trailer value containing a colon must not be split at the wrong place.
    ("subject\n\nSee-also: https://example.com/a#1\n", 0),
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
    ("Co-Authored-By", "someone <noreply@anthropic.com>", True),
    ("Co-authored-by", "Pawel Lisowski <pawellisowski@o2.pl>", False),
    ("Co-authored-by", "Jean-Claude Meunier <jc.meunier@gmail.com>", False),
    ("Co-authored-by", "Chris Olah <colah@anthropic.com>", False),
    ("Signed-off-by", "Claude <noreply@anthropic.com>", False),
    ("Reviewed-by", "Claude <noreply@anthropic.com>", False),
]


def _git(args: list[str], cwd: str) -> None:
    subprocess.run(["git", *args], cwd=cwd, check=True, capture_output=True, text=True)


def _end_to_end_control() -> list[str]:
    """Drive `commits_in_range` over a throwaway repo carrying the real trailer.

    This is the half a classifier-only control cannot reach. Swapping `%B` for
    `%s` in `commits_in_range` makes the gate report every branch clean while
    leaving all the message fixtures green — so without this, the negative
    control certifies a dead gate.
    """
    failures = []
    with tempfile.TemporaryDirectory() as repo:
        # `-c` rather than global config: the runner may have neither an
        # identity nor a default branch set, and signing would prompt.
        _git(["init", "--quiet", "-b", "main"], repo)
        for key, value in (
            ("user.name", "gate probe"),
            ("user.email", "probe@example.invalid"),
            ("commit.gpgsign", "false"),
        ):
            _git(["config", key, value], repo)

        _git(["commit", "--quiet", "--allow-empty", "-m", "base: clean"], repo)
        _git(
            [
                "commit",
                "--quiet",
                "--allow-empty",
                "-m",
                "probe: the violation\n\nCo-authored-by: Claude <noreply@anthropic.com>",
            ],
            repo,
        )

        found = [
            trailer
            for _, _, message in commits_in_range("main~1..main", cwd=repo)
            for trailer in offending_trailers(message)
        ]
        if len(found) != 1:
            failures.append(
                "  end-to-end: a commit carrying the trailer was not caught through "
                f"commits_in_range (found {found}) — the gate reads the wrong part "
                "of the message, so a real branch would report clean"
            )

        clean = [
            trailer
            for _, _, message in commits_in_range("main~1..main~1", cwd=repo)
            for trailer in offending_trailers(message)
        ]
        if clean:
            failures.append(f"  end-to-end: an empty range reported offenders: {clean}")

    return failures


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

    failures.extend(_end_to_end_control())

    if failures:
        print("self-test FAILED — the gate no longer matches its contract:")
        print("\n".join(failures))
        return 1
    print(
        f"self-test ok ({len(_PREDICATE_CASES)} predicate cases + "
        f"{len(SELF_TEST_CASES)} message cases + 2 end-to-end cases)"
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

If the range names commits already on `main`, the range is wrong, not the
commits: use `origin/main..HEAD`, not `main..HEAD`. `main` carries eight of these
from before this check existed and they cannot be rewritten.

If this flagged a real person whose given name is Claude, it is wrong and the
fix is here, not in their trailer — do not delete a human's attribution to get
CI green. Narrow the match in scripts/no-claude-coauthor-trailers.py.

Otherwise, do not work around this by disabling the check: CLAUDE.md
§Engineering rules forbids satisfying a gate by weakening it.
""".rstrip()


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Reject Claude co-author trailers (CLAUDE.md §Git workflow)."
    )
    parser.add_argument("--self-test", action="store_true", help="run the negative control")
    parser.add_argument("--range", help="commit range to check, e.g. origin/main..HEAD")
    parser.add_argument(
        "--message-file",
        help="a file holding one message to check (the PR body)",
    )
    args = parser.parse_args()

    # Mutually exclusive on purpose. `--self-test` returns before checking
    # anything, so accepting both would let a single consolidated command report
    # "self-test ok" while never looking at the branch — a gate disarmed by a
    # plausible-looking CI edit.
    if args.self_test and (args.range or args.message_file):
        parser.error("--self-test runs the negative control only; it checks nothing else")

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
