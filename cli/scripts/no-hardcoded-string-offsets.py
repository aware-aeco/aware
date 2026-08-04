#!/usr/bin/env python3
"""Fail the build when non-test code slices a `str` at a hard-coded byte offset.

CLAUDE.md §Code style: "Errors as data, not exceptions." `&s[..97]` is neither —
Rust *aborts the process* when byte 97 lands inside a multi-byte UTF-8
character. `aware search` shipped exactly that, and four command descriptions
already committed under `20-agents/` crashed it (the curly quote in
sketchup-2026's `color-to-s` occupies bytes 95..98):

    thread 'main' panicked at src/commands/search.rs:132:41:
    end byte index 97 is not a char boundary; it is inside '"' (bytes 95..98)

Nothing caught it. `clippy::string_slice` lives in the `restriction` group,
which `cargo clippy -D warnings` does not enable.

## What this checks, and why not simply `string_slice = "deny"`

The crate has ~58 string slices and all but the ones flagged here are safe by
construction: their bounds come from `find()` / `char_indices()`, which return
character boundaries. Denying the lint outright would mean rewriting 58 correct
call sites, and the pressure to `#[allow]` past that is precisely how a gate
gets hollowed out.

So this narrows to the form that can *never* be shown safe: a bound that is a
bare integer literal. `&s[..97]`, `&s[1..]`, `s[2..t.len() - 2]` — an offset
chosen by counting bytes in the source, which holds only while the input stays
ASCII. `&rest[start + 2..]` is *not* flagged: `start` came from `find`, and
adding the byte length of an ASCII delimiter to a boundary keeps it a boundary.

A literal *zero* is the one exception, because byte 0 is a boundary in every
string: `&s[0..]` and `&s[..0]` cannot panic, so flagging them would be exactly
the false-positive pressure that gets a gate ripped out. `&s[..=0]` still is
flagged — an inclusive bound ends one byte later, at 1, which can split.

Type information comes from clippy (`string_slice` fires on `str`, never on
`&[u8]`, so `&digest[..16]` is correctly ignored); the literal-vs-derived
judgement comes from the span text.

Known limit, stated rather than hidden: an offset laundered through a variable
(`let n = 97; &s[..n]`) reads as derived and is not flagged. Widening to catch
it means flagging the 58 safe sites. The shape below is the one that bit.

## Usage

    python3 scripts/no-hardcoded-string-offsets.py          # check (run from cli/)
    python3 scripts/no-hardcoded-string-offsets.py --self-test

`--self-test` is the negative control: it runs the classifier over fixtures with
known answers, so a refactor that quietly stops matching anything fails loudly
instead of reporting a clean crate.
"""

from __future__ import annotations

import json
import re
import subprocess
import sys

LINT = "clippy::string_slice"

# A bound that is nothing but an integer literal: `97`, `1`, `2usize` — and the
# non-decimal spellings, which are the same hazard written differently.
# `&s[..0x61]` slices at byte 97 exactly as `&s[..97]` does, and clippy reports
# both, so a decimal-only test would leave an easy way past this gate.
# Anything containing an identifier is derived and therefore not our business.
#
# The digits are captured so the *value* can be recovered: zero is a boundary in
# every string, so `&s[0..]` is a literal bound that cannot split a character.
_BARE_LITERAL = re.compile(
    r"""^\s*(?:
          0[xX](?P<hex>[0-9a-fA-F_]+)
        | 0[oO](?P<oct>[0-7_]+)
        | 0[bB](?P<bin>[01_]+)
        | (?P<dec>[0-9][0-9_]*)
    )(?:usize|isize|u8|u16|u32|u64|u128|i8|i16|i32|i64|i128)?\s*$""",
    re.VERBOSE,
)

_LITERAL_BASES = (("hex", 16), ("oct", 8), ("bin", 2), ("dec", 10))


def literal_value(bound: str) -> int | None:
    """The integer a bare-literal bound denotes, or None when it is derived.

    Parsed per captured base rather than through `int(text, 0)`, which rejects
    the leading zeros Rust accepts (`007` is 7, not a syntax error).
    """
    match = _BARE_LITERAL.match(bound)
    if not match:
        return None
    for group, base in _LITERAL_BASES:
        digits = (match.group(group) or "").replace("_", "")
        if digits:
            return int(digits, base)
    return None


def is_hardcoded_bound(bound: str, *, inclusive_upper: bool = False) -> bool:
    """True when this bound pins a byte offset that could land mid-character.

    Byte 0 is a character boundary in every string, so a literal zero cannot
    split anything: `&s[0..]` is the whole string and `&s[..0]` is empty, and
    neither can panic. Flagging them was false-positive pressure on a gate whose
    whole argument for existing is that it does not fire on correct code.

    An *inclusive* upper bound is the exception. `&s[..=0]` ends at byte **1**,
    which lands inside a two-byte character, so zero is a hazard there and stays
    flagged. Lower bounds are always exclusive of nothing — `0..` starts at the
    boundary itself — so the exemption is unconditional on that side.
    """
    value = literal_value(bound)
    if value is None:
        return False  # open bound, or derived from `find()` / `char_indices()`
    return inclusive_upper or value != 0


def mask_lexical(text: str) -> str:
    """Blank the *contents* of string/char literals and comments, keeping length.

    Every scan below reads `[`, `]`, `(`, `..` as structure. Inside a Rust
    literal or comment those characters are just bytes, and mistaking one for
    structure breaks bracket matching outright:

        &s[s.find(']').unwrap_or(0)..3]

    The `]` in that char literal makes the reverse scan miscount, so the gate
    found no slice bracket at all and passed a hard-coded `..3`. The same flaw
    hides a bound behind a comment (`&s[.. /* cap */ 3]`) and hides a `..`
    inside a string. Masking once, here, retires the whole class rather than
    the two spellings that happened to be reported.

    Length is preserved so offsets stay valid against the original text; the
    caller reports the original and reasons over the mask.

    Handles line and (nestable) block comments, char literals, plain and byte
    strings, and raw strings with any hash count. A `'` that opens a *lifetime*
    (`&'a str`) is deliberately not treated as a literal — it has no closing
    quote, and consuming to the next one would swallow real structure.
    """
    out = list(text)
    i, n = 0, len(text)

    def blank(start: int, stop: int) -> None:
        for k in range(max(start, 0), min(stop, n)):
            if not out[k].isspace():
                out[k] = " "

    while i < n:
        char = text[i]

        if text.startswith("//", i):
            end = text.find("\n", i)
            end = n if end < 0 else end
            blank(i, end)
            i = end
            continue

        if text.startswith("/*", i):
            depth, j = 1, i + 2
            while j < n and depth:
                if text.startswith("/*", j):
                    depth, j = depth + 1, j + 2
                elif text.startswith("*/", j):
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            blank(i, j)
            i = j
            continue

        # Raw string: r"…", br##"…"##. The hash count sets the terminator.
        raw = re.compile(r'(?:b?r)(#*)"').match(text, i)
        if raw:
            terminator = '"' + raw.group(1)
            end = text.find(terminator, raw.end())
            end = n if end < 0 else end + len(terminator)
            blank(i, end)
            i = end
            continue

        if char == '"' or (char == "b" and text.startswith('b"', i)):
            j = i + (2 if char == "b" else 1)
            while j < n:
                if text[j] == "\\":
                    j += 2
                    continue
                if text[j] == '"':
                    j += 1
                    break
                j += 1
            blank(i, j)
            i = j
            continue

        if char == "'":
            # `'\x'` or `'x'` is a literal; anything else is a lifetime.
            if text.startswith("'\\", i):
                end = text.find("'", i + 2)
                end = n if end < 0 else end + 1
            elif i + 2 < n and text[i + 2] == "'":
                end = i + 3
            else:
                out[i] = " "  # lifetime tick — not structure, not a literal
                i += 1
                continue
            blank(i, end)
            i = end
            continue

        i += 1

    return "".join(out)


def strip_grouping(expr: str) -> str:
    """Peel parentheses or braces that wrap the whole expression.

    `(..97)` and `{ ..97 }` are valid Rust and slice exactly as `..97` does, but
    the wrapper hides the `..` from the depth-aware scan in `bounds_of`, which
    would report a single bound and skip the diagnostic entirely.

    Only a wrapper enclosing the *entire* expression is peeled, so `(a)..(b)`
    is left alone rather than mangled into `a)..(b`.
    """
    expr = expr.strip()
    while len(expr) >= 2 and expr[0] in "({" and expr[-1] in ")}":
        depth = 0
        encloses_all = True
        for i, char in enumerate(expr):
            if char in "([{":
                depth += 1
            elif char in ")]}":
                depth -= 1
                if depth == 0 and i != len(expr) - 1:
                    encloses_all = False
                    break
        if not encloses_all:
            break
        expr = expr[1:-1].strip()
    return expr


def bounds_of(index_expr: str) -> tuple[list[str], bool]:
    """Split a slice index into its bounds, ignoring `..` inside nested brackets.

    `..97` -> (['', '97'], False);  `2..t.len() - 2` -> (['2', 't.len() - 2'], False)

    The second element says whether the range is *inclusive* (`..=n`). The `=` is
    stripped from the bound rather than carried along — a bound has to reach
    `strip_grouping` and `_BARE_LITERAL` unencumbered, or `&s[..=(97)]` stops
    parsing as a literal — but the fact is reported, because `..=0` ends one byte
    past `..0` and only one of the two is safe.
    """
    depth = 0
    for i in range(len(index_expr) - 1):
        char = index_expr[i]
        if char in "([{":
            depth += 1
        elif char in ")]}":
            depth -= 1
        elif depth == 0 and index_expr[i : i + 2] == "..":
            rest = index_expr[i + 2 :]
            inclusive = rest.startswith("=")
            return [index_expr[:i], rest[1:] if inclusive else rest], inclusive
    return [index_expr], False


def slice_index_of(span_text: str) -> str | None:
    """The index expression of the bracket that performs the `str` slice.

    Clippy's primary span *is* the slice expression, so the operation being
    linted is always the final index applied to it: the pair closed by the last
    `]`. Everything before that pair is the receiver, whose brackets belong to
    other types and must not be judged as string offsets.

    That distinction is the whole job. In `strings[0..1][0][..end]` the `[0..1]`
    slices a `Vec` (a hard-coded index there is perfectly safe) and `[0]` is an
    element access; only `[..end]` touches a `str`. Scanning every pair would
    fail CI on that line even though `end` came from `char_indices()`. Reading
    only the *first* pair has the opposite fault — it inspects the receiver and
    lets `&parts[i][..97]` through, the exact panic this gate exists to stop.
    """
    end = span_text.rfind("]")
    if end < 0:
        return None
    depth = 0
    for i in range(end, -1, -1):
        if span_text[i] == "]":
            depth += 1
        elif span_text[i] == "[":
            depth -= 1
            if depth == 0:
                return span_text[i + 1 : end]
    return None


def is_hardcoded(span_text: str) -> bool:
    """True when the `str` slice pins a bound to a bare integer literal.

    Reasons over the masked span throughout, so a bracket, `..` or digit that
    only *looks* structural because it sits in a literal or comment cannot
    steer the result either way.
    """
    index_expr = slice_index_of(mask_lexical(span_text))
    if index_expr is None:
        return False
    bounds, inclusive = bounds_of(strip_grouping(index_expr))
    if len(bounds) < 2:
        return False  # `[i]` / `[0]` — indexing, not slicing.
    lower, upper = (strip_grouping(bound) for bound in bounds)
    return is_hardcoded_bound(lower) or is_hardcoded_bound(
        upper, inclusive_upper=inclusive
    )


def highlighted(span: dict) -> str:
    """Just the source rustc underlined, not the whole line it sits on.

    Each `text` chunk carries the line plus 1-based `highlight_start`/`_end`
    columns bounding the part of *that* line the span covers; concatenating
    those slices reconstructs the expression even when it wraps. Classifying the
    raw line instead would drag in unrelated brackets from elsewhere on it.

    Chunks are joined with a **newline**, not butted together. A line comment
    ends at the end of its line, so flattening `&s[\n .. // display cap\n 3\n]`
    into one line made `mask_lexical` swallow the bound and the closing bracket
    along with the comment — the whole rest of the expression became comment
    text, `is_hardcoded` saw no slice, and the gate printed a clean result. The
    two are only correct together: masking has to be told where the lines were.
    """
    return "\n".join(
        chunk["text"][chunk["highlight_start"] - 1 : chunk["highlight_end"] - 1]
        for chunk in (span.get("text") or [])
    ).strip()


# `--force-warn`, not `-W`. A `-W` on the command line is *overridden* by an
# `#[allow(clippy::string_slice)]` in the source, so a module could switch this
# gate off at the call site and the script would report the crate clean — the
# exact move the failure message tells people not to make, and which CLAUDE.md
# §Engineering rules forbids. `--force-warn` cannot be suppressed by a source
# attribute, so the diagnostic is emitted and judged here whatever the module
# says about it. Hoisted out of the call so `--self-test` can assert on it: a
# revert to `-W` silently disarms the gate and no fixture would notice.
CLIPPY_CMD = [
    "cargo",
    "clippy",
    "--quiet",
    "--message-format=json",
    "--",
    "--force-warn",
    LINT,
]


def collect_offenders() -> list[str]:
    """Run clippy over the bin target and return rendered offender locations.

    The bin target, not `--all-targets`: CLAUDE.md permits this in tests, and
    `src/main.rs` scopes its sibling unwrap gate the same way.
    """
    proc = subprocess.run(
        CLIPPY_CMD,
        capture_output=True,
        text=True,
    )
    offenders = []
    compiled_the_bin = False
    for line in proc.stdout.splitlines():
        if not line.startswith("{"):
            continue
        try:
            record = json.loads(line)
        except json.JSONDecodeError:
            continue
        if record.get("reason") == "compiler-artifact":
            target = record.get("target") or {}
            if target.get("name") == "aware" and "bin" in (target.get("kind") or []):
                compiled_the_bin = True
            continue
        if record.get("reason") != "compiler-message":
            continue
        message = record.get("message") or {}
        if (message.get("code") or {}).get("code") != LINT:
            continue
        for span in message.get("spans") or []:
            if not span.get("is_primary"):
                continue
            text = highlighted(span)
            if is_hardcoded(text):
                offenders.append(f"{span['file_name']}:{span['line_start']}: {text}")

    # A clippy run that never compiled the `aware` bin inspected nothing, so an
    # empty offender list would be an artefact of the failure rather than a
    # clean result. Keyed on the artifact record, not on "we saw a diagnostic":
    # a crate that legitimately reaches zero `string_slice` sites must not start
    # failing this check.
    if not compiled_the_bin or proc.returncode != 0:
        print(
            "error: `cargo clippy` did not compile the `aware` bin "
            f"(exit {proc.returncode}), so this check inspected nothing.",
            file=sys.stderr,
        )
        if proc.stderr:
            print(proc.stderr[-2000:], file=sys.stderr)
        sys.exit(2)

    return offenders


# (span text, should this be flagged?) — the classifier's contract.
#
# These are *spans*, not source lines: what `highlighted()` hands the classifier
# is the expression rustc underlined. Every entry below was taken from real
# clippy 1.95.0 output for the corresponding Rust, so the fixtures cannot drift
# from what the gate actually receives.
SELF_TEST_CASES = [
    # The regression this gate exists for, and its delimiter-shaped cousins.
    ("desc_one[..97]", True),
    ("spec[1..]", True),
    ("t[2..t.len() - 2]", True),
    ("s[..=15]", True),
    # Bounds from `find()` / `char_indices()` — boundaries by construction.
    ("rest[start + 2..]", False),
    ("after[..end]", False),
    ("nuspec[start + 13..start + end]", False),
    ("h[a..a + 2]", False),
    ("inner[..path_end]", False),
    ("s[..cut]", False),
    ("no slice here at all", False),
    # The receiver is itself indexed, so the span carries several bracket pairs.
    # Only the last one slices the `str`; reading the first inspected the
    # receiver and let the real bug through (PR #361 review, round 1).
    ("parts[i][..97]", True),
    ("parts[0][..end]", False),
    ("map[key][..12]", True),
    ("rows[3][start..]", False),
    ("s[offsets[0]..97]", True),
    ("s[offsets[0]..end]", False),
    # A hard-coded range in the *receiver* slices a collection, not a string,
    # and must not fail CI. Judging every range-bearing pair rejected this even
    # when the string bound was derived (round 2).
    ("strings[0..1][0][..end]", False),
    ("strings[0..1][0][..97]", True),
    ("rows[2..8][i][start..]", False),
    # Non-decimal spellings of the same byte offset (round 2).
    ("s[..0x61]", True),
    ("s[..0o141]", True),
    ("s[0b1..]", True),
    ("s[..0xFF_usize]", True),
    # Grouped ranges, which hid the `..` from the depth-aware scan (round 2).
    ("s[(..97)]", True),
    ("s[{ ..97 }]", True),
    ("s[{ 12.. }]", True),
    ("s[(..end)]", False),
    ("s[{ offsets[0].. }]", False),
    # Grouping that does not enclose the whole expression must survive intact.
    ("s[(a + 1)..(b - 1)]", False),
    # Brackets and `..` inside literals or comments are text, not structure.
    # Counting them as structure broke bracket matching outright, so the gate
    # found no slice at all and passed a hard-coded bound (round 3).
    ("s[s.find(']').unwrap_or(0)..3]", True),
    ("s[s.find(']').unwrap_or(0)..end]", False),
    ("line[line.find('[').unwrap_or(0)..80]", True),
    ("line[line.find('[').unwrap_or(0)..cap]", False),
    ('s[s.find("]..[").unwrap_or(0)..12]', True),
    ('s[s.find("]..[").unwrap_or(0)..stop]', False),
    # A bound is not derived just because it is documented.
    ("s[.. /* display cap */ 3]", True),
    ("s[/* from */ 2..end]", True),
    ("s[.. /* cap */ limit]", False),
    ("s[..end] // truncate to 97 bytes", False),
    # Raw and byte strings, and a lifetime tick, must not desynchronise the mask.
    ('s[s.find(r"]#").unwrap_or(0)..7]', True),
    ('s[s.find(r#"a"]"#).unwrap_or(0)..7]', True),
    ('s[s.find(b"]").unwrap_or(0)..end]', False),
    ("Foo::<&'a str>::cut(s)[..15]", True),
    ("Foo::<&'a str>::cut(s)[..end]", False),
    # An escaped quote must not end the literal early.
    ("s[s.find('\\'').unwrap_or(0)..9]", True),
    # Byte 0 is a boundary in every string, so a literal zero cannot split a
    # character and must not fail CI — that is the false-positive pressure this
    # gate exists to avoid (round 5).
    ("s[0..]", False),
    ("s[..0]", False),
    ("s[0..0]", False),
    ("s[0x0..]", False),
    ("s[..0usize]", False),
    ("s[..0_0]", False),
    ("s[{ 0.. }]", False),
    # …but an *inclusive* upper zero ends at byte 1, which can, so it stays.
    ("s[..=0]", True),
    ("s[0..=0]", True),
    ("s[..=0x0]", True),
    # A safe zero on one side does not launder a hard-coded bound on the other.
    ("s[0..3]", True),
    ("s[0..end]", False),
    ("s[..=(97)]", True),
    # Rust reads a leading zero as decimal padding, not as a base prefix: `007`
    # is 7, and must not be mistaken for the exempt zero.
    ("s[..007]", True),
]


def self_test() -> int:
    failures = []
    for text, expected in SELF_TEST_CASES:
        actual = is_hardcoded(text)
        if actual != expected:
            failures.append(
                f"  {text!r}: expected flagged={expected}, got flagged={actual}"
            )

    # `highlighted()` is part of the contract too — masking is only correct if
    # it is told where the lines were. A span rustc reports across three lines
    # must come back across three lines, or a `//` comment in it eats the rest
    # of the expression and the gate goes quiet.
    wrapped = highlighted(
        {
            "text": [
                {"text": "    let _ = &s[", "highlight_start": 14, "highlight_end": 17},
                {
                    "text": "        .. // display cap",
                    "highlight_start": 9,
                    "highlight_end": 27,
                },
                {"text": "        3", "highlight_start": 9, "highlight_end": 10},
                {"text": "    ];", "highlight_start": 5, "highlight_end": 6},
            ]
        }
    )
    if "\n" not in wrapped:
        failures.append(
            f"  highlighted() flattened a multi-line span to {wrapped!r} — a line "
            "comment in it would swallow the rest of the expression"
        )
    elif not is_hardcoded(wrapped):
        failures.append(
            f"  a multi-line slice with a line comment was not flagged: {wrapped!r}"
        )

    # A `-W` here is overridden by `#[allow(clippy::string_slice)]` at the call
    # site, which disarms the whole gate while leaving every fixture green.
    if "--force-warn" not in CLIPPY_CMD or "-W" in CLIPPY_CMD:
        failures.append(
            f"  clippy must be invoked with --force-warn, not -W, or a source "
            f"attribute can silence this gate: {CLIPPY_CMD}"
        )

    if failures:
        print("self-test FAILED — the classifier no longer matches its contract:")
        print("\n".join(failures))
        return 1
    print(f"self-test ok ({len(SELF_TEST_CASES)} cases + 3 contract checks)")
    return 0


def main() -> int:
    if "--self-test" in sys.argv:
        return self_test()

    offenders = collect_offenders()
    if offenders:
        print(
            "error: these slice a `str` at a hard-coded byte offset, which panics "
            "when the offset lands inside a multi-byte character:\n",
            file=sys.stderr,
        )
        for offender in offenders:
            print(f"  {offender}", file=sys.stderr)
        print(
            "\nUse `crate::text::ellipsize` / `cut_after_chars` for display "
            "truncation, or `strip_prefix` / `strip_suffix` for delimiters. Do not "
            "silence this with `#[allow]` — CLAUDE.md §Engineering rules forbids "
            "satisfying a gate by disabling it.",
            file=sys.stderr,
        )
        return 1

    print("ok: no hard-coded byte offsets into strings in non-test code")
    return 0


if __name__ == "__main__":
    sys.exit(main())
