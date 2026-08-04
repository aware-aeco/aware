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

# A bound made only of digits (with optional `_` separators and an integer
# suffix): `97`, `1`, `2usize`. Anything containing an identifier is derived and
# therefore not our business.
_BARE_LITERAL = re.compile(r"^\s*[0-9][0-9_]*(?:usize|u32|u64|i32|i64)?\s*$")


def bounds_of(index_expr: str) -> list[str]:
    """Split a slice index into its bounds, ignoring `..` inside nested brackets.

    `..97` -> ['', '97'];  `2..t.len() - 2` -> ['2', 't.len() - 2']
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
            # Inclusive ranges (`..=n`) are the same hazard.
            rest = rest[1:] if rest.startswith("=") else rest
            return [index_expr[:i], rest]
    return [index_expr]


def bracket_contents(span_text: str) -> list[str]:
    """Every balanced `[...]` pair in the span, at any nesting depth.

    Not just the first: when the sliced string is itself produced by indexing,
    clippy's primary span covers both pairs. `&parts[i][..97]` opens with `[i]`,
    so reading only the first bracket would inspect the *index* and never see
    the hard-coded slice — the exact pattern this gate exists to stop would
    sail through, while `&parts[0][..end]` would be rejected for the `0`.
    """
    pairs: list[str] = []
    stack: list[int] = []
    for i, char in enumerate(span_text):
        if char == "[":
            stack.append(i)
        elif char == "]" and stack:
            start = stack.pop()
            pairs.append(span_text[start + 1 : i])
    return pairs


def is_hardcoded(span_text: str) -> bool:
    """True when a *slice* bound in this span is a bare literal.

    Only bracket pairs that actually contain a range are considered. That is
    what separates `[..97]` (a slice, and the hazard) from `[0]` (an index into
    a collection, which cannot split a character), so `&parts[0][..end]` is
    correctly left alone while `&parts[i][..97]` is caught.
    """
    for content in bracket_contents(span_text):
        bounds = bounds_of(content)
        if len(bounds) < 2:
            continue  # `[i]` / `[0]` — indexing, not slicing.
        if any(_BARE_LITERAL.match(b) for b in bounds if b.strip()):
            return True
    return False


def collect_offenders() -> list[str]:
    """Run clippy over the bin target and return rendered offender locations.

    The bin target, not `--all-targets`: CLAUDE.md permits this in tests, and
    `src/main.rs` scopes its sibling unwrap gate the same way.
    """
    proc = subprocess.run(
        ["cargo", "clippy", "--quiet", "--message-format=json", "--", "-W", LINT],
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
            text = "".join(chunk["text"] for chunk in (span.get("text") or []))
            if is_hardcoded(text):
                offenders.append(
                    f"{span['file_name']}:{span['line_start']}: {text.strip()}"
                )

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


# (span text, should this be flagged?) — the classifier's contract, including
# the real regression and the safe shapes it must leave alone.
SELF_TEST_CASES = [
    ('format!("{}…", &desc_one[..97])', True),
    ("let after_scope = &spec[1..];", True),
    ("let inner = t[2..t.len() - 2].trim();", True),
    ("let head = &s[..=15];", True),
    ("let after = &rest[start + 2..];", False),
    ("let inner = after[..end].trim();", False),
    ("rest = &after[end + 2..];", False),
    ("Some(nuspec[start + 13..start + end].trim().to_string())", False),
    ("u8::from_str_radix(&h[a..a + 2], 16)", False),
    ("let parts: Vec<&str> = inner[..path_end]", False),
    ("let kept = &s[..cut];", False),
    ("no slice here at all", False),
    # The sliced string is itself produced by indexing, so the span carries two
    # bracket pairs. Reading only the first inspected the index and missed the
    # slice entirely (PR #361 review). Both directions are pinned.
    ("let head = &parts[i][..97];", True),
    ("let head = &parts[0][..end];", False),
    ("let head = &map[key][..12];", True),
    ("let head = &rows[3][start..];", False),
    # A range whose own bound contains an index expression.
    ("let head = &s[offsets[0]..97];", True),
    ("let head = &s[offsets[0]..end];", False),
]


def self_test() -> int:
    failures = []
    for text, expected in SELF_TEST_CASES:
        actual = is_hardcoded(text)
        if actual != expected:
            failures.append(
                f"  {text!r}: expected flagged={expected}, got flagged={actual}"
            )
    if failures:
        print("self-test FAILED — the classifier no longer matches its contract:")
        print("\n".join(failures))
        return 1
    print(f"self-test ok ({len(SELF_TEST_CASES)} cases)")
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
