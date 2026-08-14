#!/usr/bin/env python3
"""Fail a pull request that would put a Claude co-author trailer on `main`.

CLAUDE.md §Git workflow: "**No `Co-Authored-By: Claude ...` trailers** in commit
messages." Nothing enforced it, and the rule has been broken on `main` many
times: 14 of the 568 commits on `main` carry
`Co-authored-by: Claude <noreply@anthropic.com>`.

Those 14 cannot be fixed. CLAUDE.md §Git workflow also makes `main`'s history
append-only ("no force-push, no rewrite"), so the only available fix is to stop
the next one — which is what this script is.

## The census this header used to carry was wrong, and that mattered

The first version of this file counted "8 of the 50 commits reachable in a
depth-50 clone" and named b0ee11b5 (2026-08-05, #362) the most recent. Both were
artefacts of measuring in a shallow clone. Over the full history the count is
14 — and five of them landed *after* this gate was merged on 2026-08-08:

    e5432842 (#397)  2026-08-10        d6fb23ab (#396)  2026-08-10
    3d5c4ad4 (#398)  2026-08-10        c32dec83 (#399)  2026-08-11
    0789633b (#408)  2026-08-13

every one of them with `trailers.yml` green on the merged commit. The gate was
not broken; it was aimed at two of the three sources. Closing the third is what
the authorship check below does, and it is why the number here is stated against
the whole history rather than whatever depth a CI checkout happens to reach.

## Where the trailer actually comes from

Three sources. The third landed all five above.

GitHub composes a squash commit from the branch's commit messages and propagates
any `Co-authored-by:` trailer it finds in them — or in the PR body — into the
result. That is the first two, and it is the path the pre-gate offenders took:
six of those carry GitHub's `---------` separator, which is how it joins several
commit messages into one, and the two that do not (2a0bfe40, 8410abe2) came from
single-commit branches where there was nothing to join.

The third is **commit authorship**, and there no message anywhere has to contain
the string. When GitHub squashes, it synthesises a `Co-authored-by:` trailer for
every commit *author* on the branch other than the account clicking merge. So a
branch whose every commit message is clean still lands the trailer — which is
exactly what #408 did: its five commits carried no trailer in any message, all
five were authored by `Claude <noreply@anthropic.com>`, and the squash at
0789633b ends with one. #398 (one clean commit, Claude-authored) and #399 (six
commits, five of them Claude-authored) are the same shape.

All three are therefore checked:

  * every commit message in the PR range (`--range`);
  * the **author identity** of every commit in that range; and
  * the PR body (`--message-file`), which GitHub also mines for trailers.

No exemption is modelled for the account performing the merge, which GitHub does
omit from the trailers it generates. It cannot matter here: the identities this
flags are Claude's, and Claude is not the account that merges.

## Why the author only, and not the committer

The first version of this check flagged a Claude *committer* too, on the reasoning
that a commit committed by Claude is agent-produced whichever field carries it.
That was wrong twice over, and Codex said so on #412.

It defends against nothing established. The synthesis documented above works from
commit *authors*; that GitHub also mines the committer is not shown, and a squash
discards the branch's committer field entirely — the squash commit's committer is
GitHub and its author is the merger. So an unmined committer leaves no trace on
`main` to catch.

And it costs a real false positive. A human who cherry-picks or applies a patch
in a checkout whose `user.email` is still Claude's produces a legitimate,
human-authored commit with a Claude committer — and this repo is exactly where
that happens, because the agent environments leave that identity set globally.
Committer metadata cannot tell that person apart from an agent, so the gate would
have blocked a branch that was never going to put a trailer anywhere.

The committer is still *read*, and asserted in `_end_to_end_control`: it is the
column that proves the identity fields have not shifted, which is the one way
this parser has actually been wrong. If GitHub is ever shown to synthesise from
it, `offending_identities` is one line from covering it.

## What this does not reach

Stated plainly, because a gate that is described as absolute gets trusted like
one:

  * a **direct push to `main`** — this runs on `pull_request` only, and CLAUDE.md
    permits direct pushes with approval;
  * a **`--admin` merge**, which CLAUDE.md §Git workflow makes the repo's normal
    merge mechanism, and which bypasses required status checks — a red run here
    does not mechanically block it, only the procedural "CI is green" rule does;
  * the **squash message edited in the merge dialog**, which nothing re-reads.

Against those three, the durable mitigation is not a check at all: supply the
squash body explicitly at merge time (the REST API's `commit_message`, or
`gh pr merge --body`) instead of accepting the one GitHub generates. That keeps
the trailer out no matter who authored the branch, and CLAUDE.md §Git workflow
requires it.

What this *does* now reach is the path all 14 offenders took.

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

One predicate — `identifies_claude` — answers that for both checks, and that is
deliberate rather than tidiness. The authorship check exists to catch identities
*because* the squash turns them into a trailer, so the set of identities it
flags has to be exactly the set whose generated trailer the trailer check would
flag. Two predicates would be two things to drift apart, and the drift would be
silent in the safe-looking direction: an identity the authorship half ignores
still becomes a trailer nobody sees. `_IDENTITY_PARITY_CASES` pins the two
together.

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

Note `origin/main`, not `main`. A stale local `main` puts the 14 unfixable
commits above into the range and reports failures nobody can act on.

`--self-test` is the negative control, and it covers four layers, because a
control that stops short of one leaves everything past it free to rot:

  * the two classifiers, over message and identity fixtures;
  * the log parser, on hand-built streams (`_parser_control`) — including the
    shapes git never emits, which is the only way to reach the field-count guard;
  * the extraction path through real git (`_end_to_end_control`); and
  * the **real `argv` → `main()` → exit-code path**, as a subprocess.

That last layer is not ceremony. Every other case calls the helpers and rebuilds
the verdict itself, which is not the same thing as the gate *reporting* it —
deleting `main()`'s identity loop left every other case green while CI stopped
checking authorship altogether.

Each layer was confirmed to fail against a deliberately broken build: the
identity check neutered, `%B` swapped for `%s`, `%an` dropped, the classifier
stuck at False, the field-count guard removed, the empty-tail trim removed, the
committer check re-added, the delimiter reverted to `\\x1f`, `--encoding=UTF-8`
dropped, `main()`'s identity loop deleted, its trailer loop deleted, its
`--message-file` offenders ignored, and its exit code forced to 0.

A control nothing can trip certifies nothing — which is how the `\\x1f` bypass
survived its first review here, and how the missing CLI layer survived its
second.
"""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
import tempfile
from typing import NamedTuple

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


def identifies_claude(name: str, email: str) -> bool:
    """Whether a `(name, email)` identity is Claude's, or Anthropic automation's.

    The single judgement behind both halves of the gate: a trailer's value is
    ours when this says so, and so is a commit's author or committer. See
    §"Which co-authors are flagged" for why that must stay one function.
    """
    if _CLAUDE_WORD.search(name) or _CLAUDE_WORD.search(email):
        return True
    return _ANTHROPIC_AUTOMATION.match(email) is not None


def is_claude_coauthor(key: str, value: str) -> bool:
    """Whether a parsed `(key, value)` trailer is a Claude co-author trailer."""
    if key.strip().lower() != COAUTHOR_KEY:
        return False
    return identifies_claude(*split_identity(value))


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


class Identity(NamedTuple):
    """A `Name <email>` pair as git records it on a commit."""

    name: str
    email: str

    def __str__(self) -> str:
        return f"{self.name} <{self.email}>"


class Commit(NamedTuple):
    """One commit in the checked range, carrying every field the gate judges."""

    sha: str
    subject: str
    message: str
    author: Identity
    committer: Identity


# NUL separates the fields, and `git log -z` separates the records with the same
# byte — so the whole stream is one flat token list, read `_LOG_FIELDS` at a time.
#
# NUL because it is the only delimiter that cannot appear INSIDE a field: a git
# commit object cannot carry one, and neither can a name or an address. A byte
# that merely looks out-of-band is not enough. This first used `\x1f`, bounding
# the split so a *message* could not shift the fields ahead of it — but a
# *subject* could, and `%s` is emitted before every identity. A commit subjected
# `clean\x1fsubject` still splits into exactly seven fields, so the count guard
# below stays silent while every identity column has slid one place and the gate
# reads an author out of the wrong column. Verified: with author
# `automation <noreply@anthropic.com>` that commit passed while the same identity
# under an ordinary subject failed. Codex caught it on #412;
# `_end_to_end_control` now pins it.
_FIELD_SEP = "\x00"

# `%B` is the RAW BODY — subject *and* message body. `%s` would be the subject
# alone, and a trailer never lives in the subject, so that swap leaves the gate
# reporting every branch clean. `_end_to_end_control` covers it.
_LOG_FORMAT = "%x00".join(["%H", "%s", "%an", "%ae", "%cn", "%ce", "%B"])
_LOG_FIELDS = 7


def commits_in_range(rev_range: str, cwd: str | None = None) -> list[Commit]:
    """Every commit in `rev_range`, with its message and its two identities.

    One `git log -z` rather than a `rev-list` plus two `git log`s per commit: the
    gate needs six fields now, and spawning a process per field per commit is how
    a long branch turns a five-second check into a timeout.
    """
    listed = subprocess.run(
        # `--encoding=UTF-8` is load-bearing, not decoration. `i18n.logOutputEncoding`
        # re-encodes the whole formatted stream — the format's own `%x00` separators
        # included — so in a checkout that sets a multibyte encoding the field
        # delimiters stop being single NUL bytes and the record structure dissolves:
        # measured at 123 tokens for a ONE-commit range under `UTF-16LE`, against the
        # 7 expected. It pins the output to what Python is decoding here. Codex caught
        # this on #412; `_end_to_end_control` configures that encoding and pins it.
        ["git", "log", "-z", "--encoding=UTF-8", f"--format={_LOG_FORMAT}", rev_range],
        capture_output=True,
        text=True,
        check=False,
        cwd=cwd,
    )
    if listed.returncode != 0:
        raise RuntimeError(
            f"git log {rev_range} failed: {listed.stderr.strip()}\n"
            "A shallow clone cannot see the merge base — fetch with "
            "`fetch-depth: 0` before running this."
        )

    return commits_from_log(listed.stdout)


def commits_from_log(stdout: str) -> list[Commit]:
    """Parse `git log -z --format=_LOG_FORMAT` output into `Commit` records.

    Split out from the subprocess so the field-count guard below is reachable
    without one: it fires only when `_LOG_FORMAT` and `_LOG_FIELDS` disagree,
    which no real `git log` can produce and which therefore had nothing pinning
    it until this was a function of its own.
    """
    tokens = stdout.split(_FIELD_SEP)
    # `-z` TERMINATES the last record rather than separating, so the split leaves
    # one empty tail token. Empty output produces `[""]` and lands at `[]`.
    if tokens and tokens[-1] == "":
        tokens.pop()
    if len(tokens) % _LOG_FIELDS:
        # Loud rather than skipped: a stream this cannot read is a set of commits
        # the gate would otherwise wave through unexamined.
        raise RuntimeError(
            f"git log emitted {len(tokens)} fields, which is not a whole number of "
            f"{_LOG_FIELDS}-field commits — `_LOG_FORMAT` and `_LOG_FIELDS` disagree"
        )

    commits = []
    for start in range(0, len(tokens), _LOG_FIELDS):
        sha, subject, author_name, author_email, committer_name, committer_email, message = tokens[
            start : start + _LOG_FIELDS
        ]
        commits.append(
            Commit(
                sha=sha,
                subject=subject.strip(),
                message=message,
                author=Identity(author_name, author_email),
                committer=Identity(committer_name, committer_email),
            )
        )
    return commits


def offending_identities(commit: Commit) -> list[str]:
    """The Claude identity on `commit` a squash would turn into a trailer.

    The AUTHOR, and deliberately not the committer — see §"Why the author only".
    """
    if identifies_claude(*commit.author):
        return [f"author: {commit.author}"]
    return []


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

# `(name, email, expected)` for the identity predicate — the authorship half of
# the gate, judged before any trailer exists.
_IDENTITY_PARITY_CASES: list[tuple[str, str, bool]] = [
    # The identity that authored all five commits of #408, and #398 and #399.
    ("Claude", "noreply@anthropic.com", True),
    # The same agent under other spellings CI has produced.
    ("Claude Code", "noreply@anthropic.com", True),
    ("claude", "claude-code[bot]@users.noreply.github.com", True),
    # Display name hides the tool; the automation address does not.
    ("automation", "noreply@anthropic.com", True),
    # …and the reverse.
    ("Claude Opus 5", "bot@users.noreply.github.com", True),
    # The identities this repo's humans commit under. Flagging one of these
    # would block every legitimate branch, which is the failure that gets a
    # gate deleted rather than fixed.
    ("Pawel", "pawellisowski@o2.pl", False),
    ("pawellisowski", "35166048+pawellisowski@users.noreply.github.com", False),
    ("Jean-Claude Meunier", "jc.meunier@gmail.com", False),
    ("Chris Olah", "colah@anthropic.com", False),
    ("Ana Diaz", "ana@anthropic-partners.example", False),
    # GitHub's own committer on a squash merge is not Claude.
    ("GitHub", "noreply@github.com", False),
]


def _git(args: list[str], cwd: str, env: dict[str, str] | None = None) -> None:
    subprocess.run(
        ["git", *args],
        cwd=cwd,
        check=True,
        capture_output=True,
        text=True,
        env={**os.environ, **env} if env else None,
    )


# The identity that authored #398, #399 and all five commits of #408.
_CLAUDE_AUTHOR = {"GIT_AUTHOR_NAME": "Claude", "GIT_AUTHOR_EMAIL": "noreply@anthropic.com"}
_CLAUDE_COMMITTER = {
    "GIT_COMMITTER_NAME": "Claude",
    "GIT_COMMITTER_EMAIL": "noreply@anthropic.com",
}


def _parser_control() -> list[str]:
    """Pin `commits_from_log` directly, including the shapes git never emits.

    The field-count guard is the reason this exists: it fires only on a
    `_LOG_FORMAT`/`_LOG_FIELDS` mismatch, so no fixture built by running git can
    reach it, and a guard nothing can trip is a guard that can be deleted without
    a test going red.
    """
    failures = []
    record = _FIELD_SEP.join(
        ["c0ffee", "the subject", "Claude", "noreply@anthropic.com", "Pawel", "p@o2.pl", "body\n"]
    )

    parsed = commits_from_log(record + _FIELD_SEP)
    if len(parsed) != 1:
        failures.append(f"  parser: one record parsed as {len(parsed)} commits")
    elif parsed[0] != Commit(
        sha="c0ffee",
        subject="the subject",
        message="body\n",
        author=Identity("Claude", "noreply@anthropic.com"),
        committer=Identity("Pawel", "p@o2.pl"),
    ):
        failures.append(f"  parser: a record read back as {parsed[0]!r}")

    if len(commits_from_log(_FIELD_SEP.join([record, record]) + _FIELD_SEP)) != 2:
        failures.append("  parser: two records did not parse as two commits")

    if commits_from_log("") != []:
        failures.append("  parser: empty output did not parse as no commits")

    # The delimiter this parser used to use, now appearing in the two fields a
    # commit author controls. Both must be inert: this is the bypass in its
    # general form, and NUL is the whole reason it cannot recur.
    smuggled = _FIELD_SEP.join(
        [
            "c0ffee",
            "sub\x1fject",
            "Claude",
            "noreply@anthropic.com",
            "Pawel",
            "p@o2.pl",
            "body\x1fwith\x1fseparators\n",
        ]
    )
    parsed = commits_from_log(smuggled + _FIELD_SEP)
    if len(parsed) != 1:
        failures.append(f"  parser: a smuggled delimiter split one commit into {len(parsed)}")
    elif parsed[0].author != Identity("Claude", "noreply@anthropic.com"):
        failures.append(
            f"  parser: a smuggled delimiter shifted the author to {parsed[0].author!r} — "
            "the identity columns are forgeable again"
        )

    for bad, description in (
        (_LOG_FIELDS - 1, "one field short"),
        (_LOG_FIELDS + 1, "one field long"),
    ):
        try:
            commits_from_log(_FIELD_SEP.join(["x"] * bad) + _FIELD_SEP)
        except RuntimeError:
            pass
        else:
            failures.append(
                f"  parser: a stream {description} was accepted — the field-count "
                "guard cannot fire, so a format/parser mismatch would pass silently"
            )

    return failures


def _end_to_end_control() -> list[str]:
    """Drive `commits_in_range` over a throwaway repo carrying each real shape.

    This is the half a classifier-only control cannot reach. Swapping `%B` for
    `%s`, or dropping a field from `_LOG_FORMAT`, makes the gate report every
    branch clean while leaving all the message fixtures green — so without this,
    the negative control certifies a dead gate.
    """
    failures = []
    with tempfile.TemporaryDirectory() as repo:
        # Repo-local config rather than global: the runner may have neither an
        # identity nor a default branch set, and signing would prompt.
        _git(["init", "--quiet", "-b", "main"], repo)
        for key, value in (
            ("user.name", "gate probe"),
            ("user.email", "probe@example.invalid"),
            ("commit.gpgsign", "false"),
        ):
            _git(["config", key, value], repo)

        def commit(message: str, env: dict[str, str] | None = None) -> str:
            _git(["commit", "--quiet", "--allow-empty", "-m", message], repo, env)
            return subprocess.run(
                ["git", "rev-parse", "HEAD"],
                cwd=repo,
                capture_output=True,
                text=True,
                check=True,
            ).stdout.strip()

        base = commit("base: clean")
        trailer = commit("probe: the violation\n\nCo-authored-by: Claude <noreply@anthropic.com>")
        authored = commit("probe: a clean message, authored by Claude", _CLAUDE_AUTHOR)
        committed = commit("probe: a clean message, committed by Claude", _CLAUDE_COMMITTER)
        # A subject carrying the delimiter this parser once used. Under `\x1f` it
        # split into the expected seven fields with every identity column shifted
        # one place, so a Claude author read back as something harmless and the
        # gate passed the commit. See `_FIELD_SEP`.
        smuggled = commit("probe: field\x1fseparator\x1fin the subject", _CLAUDE_AUTHOR)
        clean = commit("probe: clean in message and identity alike")

        for rev_range, expected, description in (
            (f"{base}..{trailer}", 1, "a trailer in the commit message"),
            (
                f"{trailer}..{authored}",
                1,
                "a clean message AUTHORED by Claude — the #408 shape, which the "
                "message-only gate passed five times",
            ),
            (
                f"{authored}..{committed}",
                0,
                "a human-authored commit merely COMMITTED by Claude — allowed on "
                "purpose: a squash discards the committer, and a person working in "
                "a checkout with a stale identity is not an offender",
            ),
            (
                f"{committed}..{smuggled}",
                1,
                "a Claude author behind a subject carrying the field delimiter — "
                "the parser-level bypass, which must not reopen",
            ),
            (f"{smuggled}..{clean}", 0, "a commit clean in message and identity"),
            (f"{clean}..{clean}", 0, "an empty range"),
        ):
            found = [
                offender
                for ranged in commits_in_range(rev_range, cwd=repo)
                for offender in offending_trailers(ranged.message) + offending_identities(ranged)
            ]
            if len(found) != expected:
                failures.append(
                    f"  end-to-end: {description}: expected {expected} offender(s), "
                    f"got {len(found)}: {found}"
                )

        # Every field must arrive intact, not merely add up to the right verdict:
        # a format that dropped `%an` would still let the identity cases pass if a
        # neighbouring field happened to carry the string.
        parsed = commits_in_range(f"{trailer}..{authored}", cwd=repo)
        if len(parsed) != 1:
            failures.append(f"  end-to-end: a one-commit range parsed as {len(parsed)} commits")
        else:
            probe = parsed[0]
            for field, actual, wanted in (
                ("author", probe.author, Identity("Claude", "noreply@anthropic.com")),
                ("committer", probe.committer, Identity("gate probe", "probe@example.invalid")),
                ("subject", probe.subject, "probe: a clean message, authored by Claude"),
                ("sha", probe.sha, authored),
            ):
                if actual != wanted:
                    failures.append(
                        f"  end-to-end: the {field} field read back as {actual!r}, "
                        f"expected {wanted!r}"
                    )
            if not probe.message.startswith("probe: a clean message, authored by Claude"):
                failures.append(f"  end-to-end: the message field read back as {probe.message!r}")

        # The real argv → `main()` → exit-code path. Every case above calls the
        # helpers directly and rebuilds the verdict here, which is not the same
        # thing as the gate reporting it: deleting `main()`'s identity loop left
        # every case above green while CI stopped checking authorship entirely.
        # Codex caught that on #412.
        script = os.path.abspath(__file__)

        def cli(*arguments: str) -> subprocess.CompletedProcess[str]:
            return subprocess.run(
                [sys.executable, script, *arguments],
                cwd=repo,
                capture_output=True,
                text=True,
                check=False,
            )

        for arguments, expected_code, description in (
            (("--range", f"{trailer}..{authored}"), 1, "a Claude-authored commit"),
            (("--range", f"{smuggled}..{clean}"), 0, "a commit clean in message and identity"),
            (("--range", f"{base}..{trailer}"), 1, "a trailer in the commit message"),
        ):
            completed = cli(*arguments)
            if completed.returncode != expected_code:
                failures.append(
                    f"  end-to-end: {description} exited {completed.returncode} through the "
                    f"CLI, expected {expected_code} — `main()` no longer reports what the "
                    f"helpers find\n    stderr: {completed.stderr.strip()[:300]}"
                )

        # The PR-body half of `main()`, through the same real path.
        body_file = os.path.join(repo, "pr-body.txt")
        for body, expected_code, description in (
            ("A description\n\nCo-authored-by: Claude <noreply@anthropic.com>\n", 1, "carrying"),
            ("A description with nothing in it.\n", 0, "clean of"),
        ):
            with open(body_file, "w", encoding="utf-8") as handle:
                handle.write(body)
            completed = cli("--message-file", body_file)
            if completed.returncode != expected_code:
                failures.append(
                    f"  end-to-end: a PR body {description} the trailer exited "
                    f"{completed.returncode} through the CLI, expected {expected_code}"
                )

        # Last, because it reconfigures the repo: a checkout that re-encodes log
        # output. Without `--encoding=UTF-8` git transcodes the format's own NUL
        # separators and the field stream dissolves, so the gate stops reading
        # identities at all.
        _git(["config", "i18n.logOutputEncoding", "UTF-16LE"], repo)
        transcoded = cli("--range", f"{trailer}..{authored}")
        if transcoded.returncode != 1:
            failures.append(
                "  end-to-end: under a multibyte `i18n.logOutputEncoding` a Claude-authored "
                f"commit exited {transcoded.returncode}, expected 1 — the log output "
                "encoding is not pinned, so the field delimiters are not really NUL"
            )

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

    for name, email, expected in _IDENTITY_PARITY_CASES:
        actual = identifies_claude(name, email)
        if actual != expected:
            failures.append(
                f"  identifies_claude({name!r}, {email!r}): expected {expected}, got {actual}"
            )
        # The parity that makes the authorship half correct: an identity is
        # flagged exactly when the trailer GitHub would synthesise from it is.
        # Drift here is silent and lands the trailer on `main`.
        as_trailer = is_claude_coauthor("Co-authored-by", f"{name} <{email}>")
        if as_trailer != actual:
            failures.append(
                f"  parity: identifies_claude({name!r}, {email!r}) is {actual}, but the "
                f"trailer it would become is judged {as_trailer} — the two halves of "
                "the gate disagree, so one of them lets this identity through"
            )

    for message, expected in SELF_TEST_CASES:
        found = offending_trailers(message)
        if len(found) != expected:
            first_line = message.splitlines()[0] if message else "<empty>"
            failures.append(
                f"  {first_line!r}: expected {expected} offender(s), "
                f"got {len(found)}: {found}"
            )

    failures.extend(_parser_control())
    failures.extend(_end_to_end_control())

    if failures:
        print("self-test FAILED — the gate no longer matches its contract:")
        print("\n".join(failures))
        return 1
    print(
        f"self-test ok ({len(_PREDICATE_CASES)} trailer-predicate cases + "
        f"{len(_IDENTITY_PARITY_CASES)} identity cases, each also checked for parity, + "
        f"{len(SELF_TEST_CASES)} message cases + 6 parser cases + 6 end-to-end ranges "
        f"+ 6 through the real CLI)"
    )
    return 0


# ── Check ────────────────────────────────────────────────────────────────────

REMEDY = """
For a trailer in a message, remove it and re-push:

  * one commit  — `git commit --amend` and delete the line, then force-push the
    branch (a topic branch, never `main`);
  * several     — `git rebase -i <base>`, mark each offender `reword`, delete
    the line in each editor;
  * PR body     — edit the description on GitHub. Trailers there are copied into
    the squash commit too, so a clean branch is not enough on its own.

For an `author:` / `committer:` line, the message is already clean — the commit
was MADE under a Claude identity, and GitHub turns that into the trailer when it
squashes. Point the clone at the account the work lands under, then restamp what
is already committed:

    git config user.name  "<your name>"
    git config user.email "<your github email>"
    git rebase <base> --exec 'git commit --amend --no-edit --reset-author'

`--reset-author` rewrites the author; the rebase itself rewrites the committer.
Set the identity FIRST — an amend re-reads the config, so restamping before
fixing it just rewrites Claude onto Claude. Do it in the clone before the first
commit and none of this is needed.

If the range names commits already on `main`, the range is wrong, not the
commits: use `origin/main..HEAD`, not `main..HEAD`. `main` carries 14 of these
from before this check reached its third source, and they cannot be rewritten.

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
        for commit in commits_in_range(args.range):
            for trailer in offending_trailers(commit.message):
                offenders.append(f"commit {commit.sha[:8]} ({commit.subject}): {trailer}")
            for identity in offending_identities(commit):
                offenders.append(
                    f"commit {commit.sha[:8]} ({commit.subject}): {identity} "
                    "— the message is clean, but GitHub generates the trailer "
                    "from this identity when it squashes"
                )

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
        checked.append(f"the messages and identities of commits in {args.range}")
    if args.message_file:
        checked.append("the pull request body")
    print(f"ok: no Claude co-author trailers in {' or '.join(checked)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
