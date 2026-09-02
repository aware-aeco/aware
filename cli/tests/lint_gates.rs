//! Negative controls for the lint gates that enforce CLAUDE.md §Code style.
//!
//! A `deny` attribute is only worth anything if it still fires. These tests
//! compile a throwaway crate carrying the *same* attribute as `src/main.rs` and
//! assert that clippy rejects a violation and accepts its absence — so a future
//! edit that removes or narrows the gate fails the suite instead of passing
//! quietly.
//!
//! What they cover, precisely:
//!   * the attribute denies `unwrap()` / `expect()` in non-test code, and it is
//!     the *denied lint* that rejects — not an unrelated probe failure;
//!   * the `cfg(test)` carve-out still holds, witnessed by a companion probe so
//!     the assertion cannot pass by never compiling a test target at all;
//!   * `src/main.rs` and `build.rs` still carry their gate;
//!   * the module-scoped panicking-index gate denies a runtime index, and
//!     `src/render/table.rs` still carries it;
//!   * `Cargo.toml` still denies `undocumented_unsafe_blocks`, the gate for
//!     CLAUDE.md's "no `unsafe` without a justification" rule;
//!   * every `unsafe` construct that gate does *not* reach — `unsafe extern`
//!     blocks and `#[unsafe(…)]` attributes, both edition-2024 forms clippy
//!     still ignores — carries a `// SAFETY:` comment, with the miss itself
//!     re-measured against live clippy so the scan is deleted rather than
//!     duplicated once the lint catches up;
//!   * nobody has re-opened any of those gates from `src/` **or `build.rs`** —
//!     with an `#[allow]`, an `#[expect]`, a `clippy::restriction` group allow,
//!     a wrapped attribute, or a level nested in a `cfg_attr` predicate — nor
//!     from `[lints.clippy]` in the manifest, where a group entry that outranks
//!     a specific `deny` switches it off.
//!
//! Three of those scan artefacts that are correct today — real `src/`, the real
//! `Cargo.toml` — so they report clean both when they work and when they have
//! stopped matching anything at all. Each therefore has a negative control
//! driving its classifier over synthetic input:
//! `gate_reopener_classifier_matches_its_contract`,
//! `manifest_lint_reader_matches_its_contract` and
//! `undocumented_unsafe_classifier_matches_its_contract` — the last paired with
//! `the_unsafe_scan_reports_planted_offenders_by_line_and_text`, which drives the
//! walk itself over a planted file so the scan is controlled end to end and not
//! only at the classifier.
//!
//! The clippy-backed ones shell out to `cargo clippy` on a two-file scratch
//! crate with no dependencies (hence `--offline`); the rest are pure file and
//! string checks. If clippy is missing those five skip — except under `CI`,
//! where the workflow installs the component explicitly and a silent skip would
//! be a hole rather than a courtesy.

use std::path::Path;
use std::process::Command;

/// The gate under test, kept byte-identical to the attribute in `src/main.rs`.
const GATE: &str = "#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]";

/// The `build.rs` variant. `build.rs` is a separate crate root, so the `main.rs`
/// attribute does not reach it and it carries its own.
const BUILD_GATE: &str = "#![deny(clippy::unwrap_used, clippy::expect_used)]";

/// The module-scoped panicking-index gate. `src/main.rs` names indexing as a
/// panic class its own attribute does not see; `clippy::indexing_slicing`
/// cannot be denied crate-wide (it fires on `serde_json::Value` indexing, which
/// does not panic — ~100 sites), so it is denied in `src/render/table.rs`, the
/// formatting module where it already shipped a process abort.
const INDEX_GATE: &str = "#![cfg_attr(not(test), deny(clippy::indexing_slicing))]";

/// `true` when the probe can run. Panics instead of skipping under `CI`.
fn clippy_available() -> bool {
    let available = Command::new("cargo")
        .args(["clippy", "--version"])
        .output()
        .is_ok_and(|out| out.status.success());
    assert!(
        available || std::env::var_os("CI").is_none(),
        "cargo clippy is unavailable under CI, where the workflow installs it — \
         skipping here would silently drop the gate's only negative control"
    );
    available
}

/// Run `cargo clippy` over a scratch crate whose `main.rs` is `body`, prefixed
/// with the gate attribute. Returns `(clippy succeeded, combined diagnostics)`.
///
/// The diagnostics matter: asserting only on the exit status would let these
/// tests pass for the wrong reason — a probe that fails to build at all also
/// "fails", which would make a dead gate look enforced.
fn run_gate(body: &str) -> (bool, String) {
    run_gate_with(GATE, body)
}

/// As [`run_gate`], but with an explicit gate attribute — so a second gate can
/// be probed through the same harness instead of a copy of it.
fn run_gate_with(gate: &str, body: &str) -> (bool, String) {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    std::fs::create_dir_all(root.join("src")).expect("src dir");
    std::fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"gate_probe\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[workspace]\n",
    )
    .expect("write manifest");
    std::fs::write(root.join("src/main.rs"), format!("{gate}\n\n{body}")).expect("write main");

    let out = Command::new("cargo")
        // `--all-targets` matches how CI invokes clippy, and is what makes the
        // `cfg(test)` carve-out testable at all: without it the probe never
        // compiles a `#[cfg(test)]` module, so that assertion would pass
        // vacuously.
        .args(["clippy", "--all-targets", "--offline", "--quiet"])
        .current_dir(root)
        // Keep the probe's build products inside the tempdir so it neither
        // pollutes nor contends on this crate's target directory.
        .env("CARGO_TARGET_DIR", root.join("target"))
        .output()
        .expect("run cargo clippy");
    let diagnostics = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    (out.status.success(), diagnostics)
}

/// Assert clippy rejected `body`, and that it rejected it *because of the gate*
/// rather than because the probe crate failed to build.
fn assert_gate_rejects(body: &str, expected_lint: &str, what: &str) {
    let (accepted, diagnostics) = run_gate(body);
    assert!(
        !accepted,
        "clippy accepted {what} in non-test code — the gate is not enforcing"
    );
    assert!(
        diagnostics.contains(expected_lint),
        "clippy rejected {what}, but not for `{expected_lint}` — the probe may be \
         failing for an unrelated reason. Diagnostics:\n{diagnostics}"
    );
}

#[test]
fn unwrap_gate_rejects_unwrap_and_expect_outside_tests() {
    if !clippy_available() {
        eprintln!("skipping: cargo clippy unavailable");
        return;
    }

    // Positive control: the same crate, same attribute, no violation. If this
    // fails, the probe itself is broken and the rejections below prove nothing.
    let clean = "fn main() {\n    let v: Option<u8> = std::env::args().count().try_into().ok();\n    println!(\"{v:?}\");\n}\n";
    let (accepted, diagnostics) = run_gate(clean);
    assert!(
        accepted,
        "gate probe rejected clean code — the probe is broken, not the gate:\n{diagnostics}"
    );

    // `Option<u8>` from a runtime value, not a `Some(1)` literal: a literal also
    // trips `unnecessary_literal_unwrap`, which would muddy which lint fired.
    assert_gate_rejects(
        "fn main() {\n    let v: Option<u8> = std::env::args().count().try_into().ok();\n    println!(\"{}\", v.unwrap());\n}\n",
        "unwrap_used",
        "`.unwrap()`",
    );

    assert_gate_rejects(
        "fn main() {\n    let v: Option<u8> = std::env::args().count().try_into().ok();\n    println!(\"{}\", v.expect(\"set\"));\n}\n",
        "expect_used",
        "`.expect()`",
    );
}

#[test]
fn unwrap_gate_still_permits_unwrap_in_tests() {
    if !clippy_available() {
        eprintln!("skipping: cargo clippy unavailable");
        return;
    }

    // Witness first. This probe puts a hard type error inside `#[cfg(test)]`;
    // clippy must reject it. If it does not, the probe never compiled a test
    // target — and the carve-out assertion below would then pass for that
    // reason alone rather than because the carve-out held. Without this, simply
    // dropping `--all-targets` from `run_gate` turns the next assertion green
    // while proving nothing.
    let (witness_accepted, witness_diagnostics) = run_gate(
        "fn main() {}\n\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn t() {\n        let _x: u8 = \"not a u8\";\n    }\n}\n",
    );
    assert!(
        !witness_accepted,
        "a type error inside `#[cfg(test)]` was not caught, so the probe is not \
         compiling test targets — the carve-out assertion below would be vacuous.\n\
         Check that `run_gate` still passes `--all-targets`.\n{witness_diagnostics}"
    );

    // CLAUDE.md permits `unwrap()` in tests. `cfg_attr(not(test), …)` is what
    // buys that carve-out; this asserts the carve-out survives, so nobody
    // "fixes" the gate into one that would force `unwrap`-free test code.
    let (accepted, diagnostics) = run_gate(
        "fn main() {}\n\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn t() {\n        let v: Option<u8> = Some(1);\n        assert_eq!(v.unwrap(), 1);\n    }\n}\n",
    );
    assert!(
        accepted,
        "the gate fired inside `#[cfg(test)]`, where CLAUDE.md permits `unwrap()`:\n{diagnostics}"
    );
}

#[test]
fn crate_roots_actually_carry_the_gate() {
    // The tests above prove the attribute denies. This proves the shipped crate
    // is the thing carrying it — otherwise the gate could be deleted from
    // `src/main.rs` and every behavioural assertion above would still pass.
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for (relative, expected) in [("src/main.rs", GATE), ("build.rs", BUILD_GATE)] {
        let source = std::fs::read_to_string(root.join(relative))
            .unwrap_or_else(|e| panic!("read {relative}: {e}"));
        assert!(
            source.contains(expected),
            "{relative} no longer carries the unwrap/expect gate:\n  {expected}"
        );
    }
}

/// `src/render/table.rs` shipped `widths[i]` where `widths` was sized from the
/// header row and `i` came from a *row* — so a row with more cells than headers
/// aborted the process. `cargo clippy -D warnings` never saw it:
/// `indexing_slicing` is a `restriction` lint. The module now denies it.
///
/// Two assertions, and both are needed. The first proves the attribute still
/// rejects a panicking index (a lint that stopped firing would leave a gate that
/// only looks like one); the second proves the shipped module is the thing
/// carrying it, since the first would pass just as well with the attribute
/// deleted from `table.rs` entirely.
#[test]
fn indexing_gate_rejects_panicking_indexes_and_table_still_carries_it() {
    if !clippy_available() {
        eprintln!("skipping: cargo clippy unavailable");
        return;
    }

    // Positive control first: the same attribute over index-free code. If this
    // fails the probe is broken and the rejection below proves nothing.
    let clean = "fn main() {\n    let v: Vec<u8> = std::env::args().map(|a| a.len() as u8).collect();\n    println!(\"{:?}\", v.first());\n}\n";
    let (accepted, diagnostics) = run_gate_with(INDEX_GATE, clean);
    assert!(
        accepted,
        "the index gate rejected index-free code — the probe is broken, not the gate:\n{diagnostics}"
    );

    // A runtime-length `Vec` and a runtime index: a constant index into a
    // fixed-size array is a compile-time bound and does not trip the lint, which
    // would make this pass for the wrong reason.
    let (accepted, diagnostics) = run_gate_with(
        INDEX_GATE,
        "fn main() {\n    let v: Vec<u8> = std::env::args().map(|a| a.len() as u8).collect();\n    println!(\"{}\", v[std::env::args().count()]);\n}\n",
    );
    assert!(
        !accepted,
        "clippy accepted a panicking index in non-test code — the gate is not enforcing"
    );
    assert!(
        diagnostics.contains("indexing_slicing"),
        "clippy rejected the index, but not for `indexing_slicing` — the probe may \
         be failing for an unrelated reason. Diagnostics:\n{diagnostics}"
    );

    let table = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/render/table.rs");
    let source = std::fs::read_to_string(&table).unwrap_or_else(|e| panic!("read table.rs: {e}"));
    assert!(
        source.contains(INDEX_GATE),
        "src/render/table.rs no longer carries the panicking-index gate:\n  {INDEX_GATE}"
    );
}

/// The lints this crate's `deny` attributes rest on. All four live in clippy's
/// `restriction` group, which is why `clippy::restriction` appears in
/// [`GATED_GROUPS`] and the other groups do not.
const GATED_LINTS: [&str; 4] = [
    "unwrap_used",
    "expect_used",
    "indexing_slicing",
    "undocumented_unsafe_blocks",
];

/// Lint *groups* whose allow-level suppresses a gated lint. Probed, not
/// reasoned about: `clippy::all`, `pedantic`, `nursery` and `correctness` were
/// all measured NOT to suppress `unwrap_used` (they do not contain it), so
/// listing them would make this reject code that is in fact gated.
/// `gated_groups_actually_reopen_the_gate_and_others_do_not` re-measures both
/// halves, so a future clippy that moves these lints between groups fails here
/// instead of silently widening or narrowing the check.
///
/// Bare, like [`GATED_LINTS`], because both are matched two ways — against
/// `clippy::`-qualified attribute text in [`gate_reopeners`], and against the
/// unqualified keys a `[lints.clippy]` table uses.
const GATED_GROUPS: [&str; 1] = ["restriction"];

/// `true` when `collapsed` sets an `allow` or `expect` lint level *anywhere*
/// inside it, including nested in a `cfg_attr` payload.
///
/// Nesting is the point. A prefix test (`starts_with("#[allow(")`) misses
/// `#[cfg_attr(not(test), allow(clippy::unwrap_used))]`, which clippy 1.95
/// honours in full — and that form is the exact mirror of the crate's own gate
/// in `src/main.rs`, so it is the *first* thing someone reaching for a target-
/// conditional override would write. `cfg_attr(windows, …)` and
/// `cfg_attr(feature = "x", …)` are the same hazard.
///
/// A lint level is recognised by its position rather than by enumerating
/// wrappers: with whitespace removed, `allow(`/`expect(` counts only when the
/// character before it opens a list — `[`, `(` or `,`. That admits `#[allow(`,
/// `#![allow(` and any `cfg_attr(<pred>, allow(` depth, while the crate's own
/// `deny(…)` gate is left alone. Whitespace is stripped rather than trusted, so
/// `#[allow (clippy::unwrap_used)]` — which clippy honours — still counts.
fn opens_a_lint(collapsed: &str) -> bool {
    let dense: String = collapsed.chars().filter(|c| !c.is_whitespace()).collect();
    ["allow(", "expect("].iter().any(|level| {
        dense.match_indices(level).any(|(at, _)| {
            // A level at index 0 has no opening bracket before it, so it is not
            // an attribute at all.
            matches!(dense[..at].chars().next_back(), Some('[' | '(' | ','))
        })
    })
}

/// A lint-level attribute that would re-open one of the gates.
#[derive(Debug, PartialEq, Eq)]
struct Reopener {
    /// 1-indexed line the attribute starts on.
    line: usize,
    /// The attribute, whitespace-collapsed onto one line.
    text: String,
}

/// Blank out comment and string-literal *contents*, preserving every character
/// position and newline so offsets still map to lines.
///
/// This is what makes bracket matching trustworthy, and it is not a nicety. A
/// scanner that counts `[` and `]` over raw text is thrown by one unbalanced
/// bracket inside a comment — `// TODO(#412): first element is items[0` — and
/// then runs on until the brackets happen to rebalance, swallowing every
/// attribute in between. That is the same whole-file blindness the `#[cfg(test)]`
/// cutoff used to cause, re-entering through a different door, and `src/` still
/// carries a commented `#[allow(unused_imports)]` (on the `pub use index::{…}`
/// facade in `src/registry/mod.rs`) that sits one bracket away from triggering
/// it.
///
/// Blanking string contents earns its keep twice more: it removes the
/// false-positive class where a trailing comment or an `#[expect(…, reason =
/// "…")]` merely *names* a gated lint, and it disarms `#[doc = "…[…"]`.
fn blank_comments_and_strings(chars: &[char]) -> Vec<char> {
    let mut out = chars.to_vec();
    let mut i = 0;
    let at = |k: usize| chars.get(k).copied();
    // Blank `chars[k]` unless it is the newline that keeps line numbers aligned.
    let blank = |out: &mut Vec<char>, k: usize| {
        if chars.get(k).is_some_and(|c| *c != '\n')
            && let Some(slot) = out.get_mut(k)
        {
            *slot = ' ';
        }
    };
    while i < chars.len() {
        match (chars[i], at(i + 1)) {
            ('/', Some('/')) => {
                while i < chars.len() && chars[i] != '\n' {
                    blank(&mut out, i);
                    i += 1;
                }
            }
            ('/', Some('*')) => {
                let mut depth = 0usize;
                while i < chars.len() {
                    if chars[i] == '/' && at(i + 1) == Some('*') {
                        depth += 1;
                        blank(&mut out, i);
                        blank(&mut out, i + 1);
                        i += 2;
                        continue;
                    }
                    if chars[i] == '*' && at(i + 1) == Some('/') {
                        depth -= 1;
                        blank(&mut out, i);
                        blank(&mut out, i + 1);
                        i += 2;
                        if depth == 0 {
                            break;
                        }
                        continue;
                    }
                    blank(&mut out, i);
                    i += 1;
                }
            }
            // Raw string: `r`, then any number of `#`, then `"`. Anything else
            // beginning with `r` is an ordinary identifier.
            ('r', Some('"' | '#')) => {
                let mut hashes = 0;
                let mut j = i + 1;
                while at(j) == Some('#') {
                    hashes += 1;
                    j += 1;
                }
                if at(j) != Some('"') {
                    i += 1;
                    continue;
                }
                j += 1;
                while j < chars.len() {
                    if chars[j] == '"' {
                        let closed = (1..=hashes).all(|n| at(j + n) == Some('#'));
                        if closed {
                            j += hashes + 1;
                            break;
                        }
                    }
                    blank(&mut out, j);
                    j += 1;
                }
                i = j;
            }
            ('"', _) => {
                let mut j = i + 1;
                while j < chars.len() && chars[j] != '"' {
                    // Skip the character an escape protects, so `\"` does not
                    // look like the closing quote.
                    let step = usize::from(chars[j] == '\\') + 1;
                    for k in j..j + step {
                        blank(&mut out, k);
                    }
                    j += step;
                }
                i = j + 1;
            }
            // A char literal, not a lifetime: `'x'`, `'\n'`. A lifetime (`'a`)
            // has no closing quote and must be left alone.
            ('\'', _) => {
                let width = if at(i + 1) == Some('\\') { 3 } else { 2 };
                if at(i + width) == Some('\'') {
                    for k in i + 1..i + width {
                        blank(&mut out, k);
                    }
                    i += width + 1;
                } else {
                    i += 1;
                }
            }
            _ => i += 1,
        }
    }
    out
}

/// Report every `allow`/`expect` attribute in `source` that re-opens a gate.
///
/// Pure, so [`gate_reopener_classifier_matches_its_contract`] can drive it with
/// synthetic sources — the scan over real `src/` below has nothing to say about
/// whether the classifier still matches anything at all.
///
/// The rule is deliberately absolute: **no** allow/expect of a gated lint (or of
/// `clippy::restriction`) anywhere under `src/`, test modules included. The
/// previous version carved out test code by ignoring everything after a file's
/// first `#[cfg(test)]` line, and that carve-out was the bug: that line is not
/// always the unit-test module. In `src/runtime/invoker.rs` it gates a `use` on
/// line 9, so 4,194 of the file's 4,202 lines went unchecked; in
/// `src/commands/agent.rs` and `src/commands/app.rs` it does open a test module,
/// but production code follows it. Roughly 2,800 lines of non-test code were
/// exempt crate-wide. Dropping the carve-out removes the class of bug rather
/// than patching one instance of it, and costs nothing today — `src/` contains
/// no such attribute at all.
///
/// What to do about one that IS reported depends on which gate it re-opens.
/// `unwrap_used`, `expect_used` and `indexing_slicing` are denied by
/// `cfg_attr(not(test), …)` attributes, so inside `#[cfg(test)]` they are not
/// denied and an allow of them is redundant — delete it. `undocumented_unsafe_
/// blocks` is denied from `Cargo.toml`, and `[lints]` applies to *every* target
/// including tests (`src/main.rs` says so), so an allow of that one is
/// load-bearing: the fix there is a `// SAFETY:` comment, never a deletion.
///
/// Attributes are found anywhere, not just at the start of a line — clippy
/// honours `let w = { #[allow(clippy::unwrap_used)] v.unwrap() };` as readily as
/// a line-initial one — by matching brackets over the output of
/// [`blank_comments_and_strings`]. That also covers attributes rustfmt has
/// wrapped across lines. The lint level itself is found by position via
/// [`opens_a_lint`], so a level nested in a `cfg_attr` predicate counts too.
fn gate_reopeners(source: &str) -> Vec<Reopener> {
    let all: Vec<&str> = GATED_LINTS
        .iter()
        .chain(GATED_GROUPS.iter())
        .copied()
        .collect();
    gate_reopeners_for(source, &all)
}

/// The lints denied from `Cargo.toml` rather than from a crate-root attribute.
///
/// They are the only ones that reach *every* Cargo target, integration tests
/// included, which is why [`gated_source_files`] looks for these — and only
/// these — under `tests/`. The rest are denied by `cfg_attr(not(test), …)`
/// attributes in `src/main.rs` and `src/render/table.rs`, which never reach a
/// separate integration-test crate root; an allow of one there suppresses
/// nothing, and CLAUDE.md permits `unwrap()` in tests anyway.
const MANIFEST_SOURCED: [&str; 2] = ["undocumented_unsafe_blocks", "restriction"];

/// As [`gate_reopeners`], but looking only for `names`.
fn gate_reopeners_for(source: &str, names: &[&str]) -> Vec<Reopener> {
    let chars: Vec<char> = source.chars().collect();
    let code = blank_comments_and_strings(&chars);
    let mut found = Vec::new();
    let mut i = 0;
    while i < code.len() {
        // An attribute opens with `#[` or `#![` and nothing else.
        let open = match (code.get(i), code.get(i + 1), code.get(i + 2)) {
            (Some('#'), Some('['), _) => i + 1,
            (Some('#'), Some('!'), Some('[')) => i + 2,
            _ => {
                i += 1;
                continue;
            }
        };

        let mut depth = 0usize;
        let mut end = None;
        for (k, c) in code.iter().enumerate().skip(open) {
            match c {
                '[' => depth += 1,
                ']' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(k);
                        break;
                    }
                }
                _ => {}
            }
        }
        let Some(end) = end else {
            // Unterminated — not an attribute this can reason about. Step past
            // the `#` only, so nothing after it is swallowed.
            i += 1;
            continue;
        };

        let collapsed = collapse(&code[i..=end]);
        // Qualified, so `#[allow(othertool::unwrap_used)]` is another tool's
        // business rather than a finding here.
        let names_gated = names
            .iter()
            .any(|name| collapsed.contains(&format!("clippy::{name}")));
        if opens_a_lint(&collapsed) && names_gated {
            found.push(Reopener {
                line: code[..i].iter().filter(|c| **c == '\n').count() + 1,
                // Reported from the original, so a maintainer sees the text as
                // written rather than with its strings blanked out.
                text: collapse(&chars[i..=end]),
            });
        }
        i = end + 1;
    }
    found
}

/// Whitespace-collapse a character span onto one line.
fn collapse(span: &[char]) -> String {
    span.iter()
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// `deny` can be re-opened for a single item with one `#[allow]` or `#[expect]`,
/// and clippy stays green — so the attribute alone is not the whole gate. CI has
/// no other signal for this, hence a test.
/// Every `.rs` file a gate reaches, paired with the lint names re-openable there.
///
/// Three roots, because a gate's reach depends on where it is declared. `src/`
/// and `build.rs` carry crate-root `deny` attributes *and* are covered by the
/// manifest, so everything applies. `build.rs` is easy to miss because it sits
/// beside `src/` rather than in it — `crate_roots_actually_carry_the_gate`
/// asserts it carries `BUILD_GATE`, but a scan rooted at `src/` never checks
/// whether the next line re-opens it, and it injects the Google client secret
/// into release builds.
///
/// `tests/` is narrower. A crate-root attribute in `src/main.rs` cannot reach a
/// separate integration-test crate, but a `[lints.clippy]` entry reaches every
/// Cargo target — measured: an `#![allow(clippy::undocumented_unsafe_blocks)]`
/// in an integration test lets an unsafe block ship with no safety comment and
/// `clippy -D warnings` stays green. So only [`MANIFEST_SOURCED`] is looked for
/// there; flagging `unwrap_used` in `tests/` would reject what CLAUDE.md
/// explicitly permits.
fn gated_source_files() -> Vec<(std::path::PathBuf, &'static [&'static str])> {
    static ALL: &[&str] = &[
        "unwrap_used",
        "expect_used",
        "indexing_slicing",
        "undocumented_unsafe_blocks",
        "restriction",
    ];
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));

    let mut compiled = vec![root.join("build.rs")];
    collect_rs_files(&root.join("src"), &mut compiled);
    let mut integration = Vec::new();
    collect_rs_files(&root.join("tests"), &mut integration);

    compiled
        .into_iter()
        .map(|path| (path, ALL))
        .chain(
            integration
                .into_iter()
                .map(|path| (path, &MANIFEST_SOURCED[..])),
        )
        .collect()
}

#[test]
fn no_targeted_allow_reopens_the_gate_in_src() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let files = gated_source_files();
    // A floor, not an is-empty check. `src/` alone holds ~90 files, so a walk
    // that recovered one of them would pass an emptiness test while having
    // silently dropped the rest — coverage collapse looks exactly like coverage.
    assert!(
        files.len() > 120,
        "scanned only {} files under {} — the walk is truncated, so a clean \
         result means nothing",
        files.len(),
        root.display()
    );

    let mut offenders = Vec::new();
    for (file, names) in &files {
        // Not `else { continue }`. A file this gate cannot read is a file it
        // cannot clear, and turning that into silence is the exact failure mode
        // CLAUDE.md §Engineering rules forbids — in the file that enforces it.
        let source = std::fs::read_to_string(file)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", file.display()));
        let relative = file
            .strip_prefix(root)
            .unwrap_or(file)
            .display()
            .to_string();
        for hit in gate_reopeners_for(&source, names) {
            offenders.push(format!("{relative}:{}: {}", hit.line, hit.text));
        }
    }

    assert!(
        offenders.is_empty(),
        "these attributes re-open a lint gate — fix the call site instead \
         (CLAUDE.md §Engineering rules forbids silencing a gate to satisfy it). \
         For `unwrap_used` / `expect_used` / `indexing_slicing` inside \
         `#[cfg(test)]` the lint is not denied at all, so the allow is redundant \
         and should simply be deleted; `undocumented_unsafe_blocks` is denied \
         from Cargo.toml for every target, so there the fix is a `// SAFETY:` \
         comment, not a deletion:\n  {}",
        offenders.join("\n  ")
    );
}

/// The negative control for [`gate_reopeners`]. The scan above runs over real
/// `src/`, which currently contains none of these attributes — so it reports
/// clean whether the classifier works or has stopped matching entirely. This
/// drives it with sources that must be caught and sources that must not.
///
/// The *group* rows are measured against live clippy by
/// [`gated_groups_actually_reopen_the_gate_and_others_do_not`]. The attribute
/// *forms* are asserted against the classifier only — each was checked by hand
/// against clippy 1.95 when it was added, but nothing here re-measures them.
#[test]
fn gate_reopener_classifier_matches_its_contract() {
    let must_catch = [
        ("plain allow", "#[allow(clippy::unwrap_used)]\nfn f() {}\n"),
        (
            "expect, not allow",
            "#[expect(clippy::unwrap_used)]\nfn f() {}\n",
        ),
        (
            "the restriction group",
            "#[allow(clippy::restriction)]\nfn f() {}\n",
        ),
        (
            "wrapped across lines",
            "#[allow(\n    clippy::unwrap_used,\n    clippy::expect_used\n)]\nfn f() {}\n",
        ),
        (
            "inner attribute",
            "#![allow(clippy::indexing_slicing)]\nfn f() {}\n",
        ),
        (
            "the unsafe gate",
            "#[allow(clippy::undocumented_unsafe_blocks)]\nfn f() {}\n",
        ),
        (
            // Alone, deliberately. Every other row that names `expect_used`
            // names `unwrap_used` too, so dropping `expect_used` from
            // GATED_LINTS left the whole suite green.
            "expect_used as the only gated lint named",
            "#[allow(clippy::expect_used)]\nfn f() {}\n",
        ),
        (
            // The bug this rewrite fixes: a cfg-gated `use` near the top of a
            // file used to exempt everything below it.
            "below an early cfg(test) item",
            "#[cfg(test)]\nuse std::collections::HashMap;\n\n#[allow(clippy::unwrap_used)]\nfn f() {}\n",
        ),
        (
            "inside a unit-test module, where it is redundant",
            "#[cfg(test)]\nmod tests {\n    #[allow(clippy::unwrap_used)]\n    fn t() {}\n}\n",
        ),
        (
            // The mirror of the crate's own gate, and so the most likely form
            // anyone reaching for a conditional override would reach for.
            "an allow nested in cfg_attr",
            "#[cfg_attr(not(test), allow(clippy::unwrap_used))]\nfn f() {}\n",
        ),
        (
            "an expect nested in a platform cfg_attr",
            "#[cfg_attr(windows, expect(clippy::indexing_slicing))]\nfn f() {}\n",
        ),
        (
            "a group allow nested in a feature cfg_attr",
            "#[cfg_attr(feature = \"x\", allow(clippy::restriction))]\nfn f() {}\n",
        ),
        (
            // Clippy honours an attribute mid-expression exactly as it does a
            // line-initial one.
            "an attribute that is not at the start of its line",
            "fn f() -> usize {\n    let w = { #[allow(clippy::unwrap_used)] g().unwrap() };\n    w\n}\n",
        ),
        (
            "a space between the level and its list",
            "#[allow (clippy::unwrap_used)]\nfn f() {}\n",
        ),
        (
            // The regression that made the bracket scanner lexer-based: an
            // unbalanced `[` in a comment used to swallow everything after it.
            "below a comment holding an unbalanced bracket",
            "#[derive(Debug)] // TODO(#412): first element is items[0\nstruct S;\n\n#[allow(clippy::unwrap_used)]\nfn f() {}\n",
        ),
        (
            "below a string holding an unbalanced bracket",
            "const S: &str = \"items[0\";\n\n#[allow(clippy::unwrap_used)]\nfn f() {}\n",
        ),
    ];
    for (what, source) in must_catch {
        assert!(
            !gate_reopeners(source).is_empty(),
            "the classifier missed {what}, which clippy honours as a re-opened gate:\n{source}"
        );
    }

    let must_ignore = [
        ("an unrelated lint", "#[allow(dead_code)]\nfn f() {}\n"),
        (
            // Measured not to contain the gated lints; flagging it would reject
            // code that is still fully gated.
            "a group that does not contain them",
            "#[allow(clippy::all)]\nfn f() {}\n",
        ),
        (
            "prose naming the attribute",
            "//! Fields carry `#[allow(clippy::unwrap_used)]` in the old design.\nfn f() {}\n",
        ),
        (
            "the deny that IS the gate",
            "#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]\nfn f() {}\n",
        ),
        (
            // String contents are blanked before matching, so a level inside a
            // literal is not a level.
            "a lint level quoted inside a doc attribute",
            "#[doc = \"allow(clippy::unwrap_used)\"]\nfn f() {}\n",
        ),
        (
            "a gated lint named in a trailing comment",
            "#[allow(dead_code)] // deliberately not clippy::unwrap_used, see #344\nfn f() {}\n",
        ),
        (
            "a gated lint named in a block comment",
            "/* clippy::unwrap_used is allowed nowhere */\n#[allow(dead_code)]\nfn f() {}\n",
        ),
        (
            // `reason` is `expect`'s idiomatic companion and exists to talk
            // about lints; naming one there gates nothing.
            "a gated lint named only in an expect's reason string",
            "#[expect(dead_code, reason = \"until the unwrap_used sites are fixed\")]\nfn f() {}\n",
        ),
        (
            "another tool's lint that happens to share a name",
            "#[allow(othertool::unwrap_used)]\nfn f() {}\n",
        ),
        ("no attributes at all", "fn f() {}\n"),
    ];
    for (what, source) in must_ignore {
        assert_eq!(
            gate_reopeners(source),
            Vec::new(),
            "the classifier reported {what}, which does not re-open any gate:\n{source}"
        );
    }

    // Line numbers are what a maintainer navigates by; an off-by-one here would
    // send them to the wrong place.
    let hits = gate_reopeners("fn a() {}\n\n#[allow(clippy::unwrap_used)]\nfn b() {}\n");
    assert_eq!(hits.len(), 1, "expected exactly one hit, got {hits:?}");
    assert_eq!(hits[0].line, 3, "wrong line reported: {hits:?}");
}

/// The classifier is exercised on strings and the `src/` scan finds nothing, so
/// between them the *walk* is untested: pointing it at one subdirectory, dropping
/// its recursion, or never recording a hit all leave the suite green. This drives
/// the walk and the scan together over a planted offender.
#[test]
fn the_scan_finds_a_planted_reopener_in_a_nested_file() {
    let dir = tempfile::tempdir().expect("tempdir");
    let nested = dir.path().join("a/b");
    std::fs::create_dir_all(&nested).expect("dirs");
    std::fs::write(
        nested.join("deep.rs"),
        "fn g() -> Option<u8> { None }\n\n#[allow(clippy::unwrap_used)]\nfn f() -> u8 { g().unwrap() }\n",
    )
    .expect("write nested source");
    // A non-Rust file carrying the same text, to pin the extension filter.
    std::fs::write(
        dir.path().join("notes.txt"),
        "#[allow(clippy::unwrap_used)]\n",
    )
    .expect("write decoy");

    let mut files = Vec::new();
    collect_rs_files(dir.path(), &mut files);
    assert_eq!(
        files.len(),
        1,
        "the walk must recurse into nested directories and take only `.rs`: {files:?}"
    );

    let source = std::fs::read_to_string(&files[0]).expect("read planted file");
    let hits = gate_reopeners(&source);
    assert_eq!(hits.len(), 1, "expected the planted offender, got {hits:?}");
    assert_eq!(hits[0].line, 3, "wrong line reported: {hits:?}");
}

/// [`GATED_GROUPS`] claims `clippy::restriction` suppresses a gated lint and the
/// other groups do not. That is a fact about clippy, not about this crate, so it
/// is measured rather than asserted — a clippy release that moves these lints
/// into another group would otherwise leave the classifier quietly wrong in
/// whichever direction the move went.
#[test]
fn gated_groups_actually_reopen_the_gate_and_others_do_not() {
    if !clippy_available() {
        eprintln!("skipping: cargo clippy unavailable");
        return;
    }

    let violation = |group: &str| {
        format!(
            "#[allow({group})]\nfn main() {{\n    let v: Option<u8> = std::env::args().count().try_into().ok();\n    println!(\"{{}}\", v.unwrap());\n}}\n"
        )
    };

    for group in GATED_GROUPS {
        let qualified = format!("clippy::{group}");
        let (accepted, diagnostics) = run_gate(&violation(&qualified));
        assert!(
            accepted,
            "`#[allow({qualified})]` no longer suppresses the gate, so listing it \
             in GATED_GROUPS now rejects code that is still gated:\n{diagnostics}"
        );
    }

    // The doc on GATED_GROUPS names all four of these as measured, so all four
    // are measured. `assert_gate_rejects` rather than a bare status check: a
    // probe crate that failed to build for an unrelated reason also reports
    // not-accepted, which would let this half pass without the lint ever firing.
    for group in [
        "clippy::all",
        "clippy::pedantic",
        "clippy::nursery",
        "clippy::correctness",
    ] {
        assert_gate_rejects(
            &violation(group),
            "unwrap_used",
            &format!("`.unwrap()` under `#[allow({group})]`"),
        );
    }
}

/// The level and priority a manifest `[lints.clippy]` table assigns to `name`,
/// or `None` when it assigns none. Priority defaults to 0, as cargo does.
///
/// Reads *active* entries only. A `contains` over the whole file cannot tell
/// `undocumented_unsafe_blocks = "deny"` from the same line commented out, so
/// the gate could be switched off by prefixing one `#` and the anchor below
/// would stay green — the precise failure it exists to prevent. Tracking the
/// table header also keeps a same-named `[lints.rust]` entry from answering for
/// the clippy one.
///
/// Priority is read because it decides whether a *group* entry wins. Measured
/// against the real gate: with `undocumented_unsafe_blocks = "deny"` present,
/// `restriction = { level = "allow", priority = 1 }` turns the unsafe gate off,
/// while the same entry at `priority = -1` leaves it enforcing. Flagging the
/// second would reject a legitimate manifest.
///
/// Deliberately not a TOML parser: this crate has no TOML dependency, and the
/// two forms cargo accepts are `name = "level"` and
/// `name = { level = "level", priority = N }`, both of which carry what is
/// needed in quotes on the entry's first line.
fn manifest_clippy_entry(manifest: &str, name: &str) -> Option<(String, i64)> {
    let dotted = format!("clippy.{name}");
    let mut table = "";
    for line in manifest.lines().map(str::trim) {
        if line.starts_with('#') {
            continue;
        }
        if let Some(header) = line.strip_prefix('[') {
            table = header.split(']').next().unwrap_or_default();
            continue;
        }
        // Under `[lints]`, cargo also accepts the dotted key `clippy.<lint>` —
        // a form a bare `starts_with(name)` never sees.
        let rest = match table {
            "lints.clippy" => line.strip_prefix(name),
            "lints" => line.strip_prefix(dotted.as_str()),
            _ => None,
        };
        // `strip_prefix` alone would match `unwrap_used_extra`; require the
        // assignment to begin right after the name.
        let Some(rest) = rest.map(str::trim_start).filter(|r| r.starts_with('=')) else {
            continue;
        };
        // Single quotes are valid TOML, so a quote-specific split would read
        // `= 'allow'` as no level at all and call a downgraded lint clean.
        let level = rest
            .split(['"', '\''])
            .nth(1)
            .unwrap_or_default()
            .to_string();
        let priority = rest
            .split_once("priority")
            .and_then(|(_, tail)| {
                let digits: String = tail
                    .trim_start()
                    .trim_start_matches('=')
                    .trim()
                    .chars()
                    .take_while(|c| c.is_ascii_digit() || *c == '-')
                    .collect();
                digits.parse::<i64>().ok()
            })
            .unwrap_or(0);
        return Some((level, priority));
    }
    None
}

/// `true` when `level` still enforces.
fn level_enforces(level: &str) -> bool {
    level == "deny" || level == "forbid"
}

/// `undocumented_unsafe_blocks` is denied in `Cargo.toml`, not in a crate-root
/// attribute, so `crate_roots_actually_carry_the_gate` never saw it and nothing
/// else did either: deleting that one line leaves every check in this file
/// green. This anchors it, and rejects re-opening any gated lint through the
/// same manifest.
#[test]
fn the_manifest_still_carries_the_unsafe_gate() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let source =
        std::fs::read_to_string(&manifest).unwrap_or_else(|e| panic!("read Cargo.toml: {e}"));

    assert_eq!(
        manifest_clippy_entry(&source, "undocumented_unsafe_blocks")
            .map(|(level, _)| level)
            .as_deref(),
        Some("deny"),
        "Cargo.toml no longer actively denies `undocumented_unsafe_blocks` under \
         `[lints.clippy]` — CLAUDE.md §Code style requires every `unsafe` block \
         to carry a justification, and `cargo clippy -D warnings` does not \
         enable that lint on its own"
    );

    // A `[lints]` entry can re-open a gate as effectively as an `#[allow]`, and
    // from a file the source scan above never reads.
    let mut reopened: Vec<String> = GATED_LINTS
        .iter()
        .filter_map(|lint| {
            let (level, _) = manifest_clippy_entry(&source, lint)?;
            (!level_enforces(&level)).then(|| format!("{lint} = \"{level}\""))
        })
        .collect();

    // Groups need a priority comparison, not a priority threshold.
    // `undocumented_unsafe_blocks` is the one gate that lives only in this
    // manifest, so a group entry that outranks it switches it off with every
    // other check in this file still green. Cargo applies the higher-priority
    // entry last, so the group wins exactly when its priority is strictly
    // greater than the lint's — measured on clippy 1.95:
    //
    //   deny @ 0   vs allow @  1  -> gate OFF     deny @  0 vs allow @  0 -> holds
    //   deny @ -2  vs allow @ -1  -> gate OFF     deny @ -1 vs allow @ -2 -> holds
    //
    // Comparing the group against zero instead would miss the second row, where
    // both priorities are negative, and comparing it against nothing at all
    // would reject the legitimate manifests on the right.
    for group in GATED_GROUPS {
        let Some((group_level, group_priority)) = manifest_clippy_entry(&source, group) else {
            continue;
        };
        if level_enforces(&group_level) {
            continue;
        }
        for lint in GATED_LINTS {
            let Some((lint_level, lint_priority)) = manifest_clippy_entry(&source, lint) else {
                continue;
            };
            if level_enforces(&lint_level) && group_priority > lint_priority {
                reopened.push(format!(
                    "{group} = {{ level = \"{group_level}\", priority = {group_priority} }} \
                     outranks {lint} = {{ level = \"{lint_level}\", priority = {lint_priority} }}"
                ));
            }
        }
    }

    assert!(
        reopened.is_empty(),
        "these `[lints.clippy]` entries put a gated lint below `deny`:\n  {}",
        reopened.join("\n  ")
    );
}

/// The negative control for [`manifest_clippy_entry`]. The test above reads the
/// real `Cargo.toml`, which is correct today — so it passes both when the reader
/// works and when it has stopped reading anything.
#[test]
fn manifest_lint_reader_matches_its_contract() {
    let cases = [
        (
            "the gate as it actually ships",
            "[lints.clippy]\nundocumented_unsafe_blocks = \"deny\"\n",
            Some(("deny", 0)),
        ),
        (
            "commented out — the gate is off, however the line reads",
            "[lints.clippy]\n# undocumented_unsafe_blocks = \"deny\"\n",
            None,
        ),
        (
            "downgraded",
            "[lints.clippy]\nundocumented_unsafe_blocks = \"allow\"\n",
            Some(("allow", 0)),
        ),
        (
            "the table form cargo also accepts",
            "[lints.clippy]\nundocumented_unsafe_blocks = { level = \"warn\", priority = 1 }\n",
            Some(("warn", 1)),
        ),
        (
            "a negative priority, which loses to the specific lint",
            "[lints.clippy]\nundocumented_unsafe_blocks = { level = \"allow\", priority = -1 }\n",
            Some(("allow", -1)),
        ),
        (
            "a different tool's table cannot answer for clippy's",
            "[lints.rust]\nundocumented_unsafe_blocks = \"deny\"\n",
            None,
        ),
        (
            "the entry moved out from under the table header",
            "[lints.clippy]\nother = \"deny\"\n\n[profile.release]\nundocumented_unsafe_blocks = \"deny\"\n",
            None,
        ),
        (
            "a longer lint name must not answer for this one",
            "[lints.clippy]\nundocumented_unsafe_blocks_extra = \"deny\"\n",
            None,
        ),
        ("absent entirely", "[lints.clippy]\n", None),
    ];
    for (what, manifest, expected) in cases {
        let expected = expected.map(|(level, priority)| (level.to_string(), priority));
        assert_eq!(
            manifest_clippy_entry(manifest, "undocumented_unsafe_blocks"),
            expected,
            "misread the manifest with {what}:\n{manifest}"
        );
    }

    // The group form the priority comparison exists for, including the negative
    // pair where a threshold-against-zero test reads both as harmless.
    for (manifest, expected) in [
        (
            "[lints.clippy]\nrestriction = { level = \"allow\", priority = 1 }\n",
            ("allow", 1),
        ),
        (
            "[lints.clippy]\nrestriction = { level = \"allow\", priority = -1 }\n",
            ("allow", -1),
        ),
    ] {
        assert_eq!(
            manifest_clippy_entry(manifest, "restriction"),
            Some((expected.0.to_string(), expected.1)),
            "misread the group entry:\n{manifest}"
        );
    }
}

/// `gate_reopeners_for` narrows the scan by lint name, which is what lets
/// `tests/` be checked for the manifest-sourced gates without flagging the
/// `unwrap()` CLAUDE.md permits there.
#[test]
fn the_scan_narrows_by_lint_name() {
    let unsafe_allow = "#![allow(clippy::undocumented_unsafe_blocks)]\nfn f() {}\n";
    let unwrap_allow = "#[allow(clippy::unwrap_used)]\nfn f() {}\n";

    assert_eq!(
        gate_reopeners_for(unsafe_allow, &MANIFEST_SOURCED).len(),
        1,
        "the manifest-sourced gate reaches integration tests and must be checked there"
    );
    assert_eq!(
        gate_reopeners_for(unwrap_allow, &MANIFEST_SOURCED),
        Vec::new(),
        "`unwrap_used` is denied by a crate-root attribute that never reaches an \
         integration-test crate, so an allow of it there suppresses nothing"
    );
    // The wide form still sees both, so narrowing did not weaken `src/`.
    assert_eq!(gate_reopeners(unwrap_allow).len(), 1);
    assert_eq!(gate_reopeners(unsafe_allow).len(), 1);
}

/// The hard-coded-byte-offset gate (`scripts/no-hardcoded-string-offsets.py`)
/// runs in CI, where it can afford the extra clippy pass. Its *classifier* is
/// pure and cheap, though, and a classifier that silently stops matching would
/// report a clean crate forever — so run its self-test here too, giving `cargo
/// test` the same signal locally.
///
/// Skips when `python3` is absent, and — as with clippy above — refuses to skip
/// under `CI`, where the runner has it.
#[test]
fn hardcoded_offset_gate_classifier_still_matches_its_contract() {
    let script = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("scripts")
        .join("no-hardcoded-string-offsets.py");
    assert!(
        script.is_file(),
        "the gate script is gone: {}",
        script.display()
    );

    let out = Command::new("python3").args(["--version"]).output();
    let have_python = out.is_ok_and(|o| o.status.success());
    assert!(
        have_python || std::env::var_os("CI").is_none(),
        "python3 is unavailable under CI, where the workflow runs this gate — \
         skipping here would drop its only negative control"
    );
    if !have_python {
        eprintln!("skipping: python3 unavailable");
        return;
    }

    let out = Command::new("python3")
        .arg(&script)
        .arg("--self-test")
        .output()
        .expect("run the gate self-test");
    assert!(
        out.status.success(),
        "the gate's classifier no longer matches its own contract:\n{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

/// One `unsafe` construct found by [`unsafe_constructs`].
#[derive(Debug, PartialEq, Eq)]
struct UnsafeConstruct {
    /// 1-indexed line the construct starts on.
    line: usize,
    /// The line, trimmed — enough to name the site in a failure message.
    text: String,
    /// Whether a `// SAFETY:` comment sits immediately above it.
    documented: bool,
}

/// Every `unsafe` construct in `source` that `clippy::undocumented_unsafe_blocks`
/// does not reach, each paired with whether it carries a `// SAFETY:` comment.
///
/// Two constructs, both introduced by edition 2024 and both outside the lint —
/// measured by [`clippy_still_misses_the_constructs_this_gate_covers`], not
/// assumed:
///
///   * `unsafe extern "C" { … }` **blocks**, where the assertion being made is
///     that the declared signature matches the platform's real one. A wrong
///     signature is undefined behaviour at every call site, and the `// SAFETY:`
///     comment on the *call* does not cover it — that one is about the
///     arguments.
///   * `#[unsafe(…)]` attributes (`no_mangle`, `export_name`, `link_section`),
///     where the assertion is about the symbol it forces.
///
/// `unsafe impl` is deliberately absent: clippy *does* reach that one, measured
/// alongside the two above, so listing it here would duplicate a live lint with
/// a text scan. A private `unsafe fn` is a third construct the lint misses
/// (`missing_safety_doc` only fires on exported items) — this crate has none, so
/// the scan does not cover it; the enumeration above is what is covered, not the
/// whole of what clippy misses.
///
/// Matching runs over [`blank_comments_and_strings`] so that prose mentioning
/// `unsafe extern` — this crate has several — cannot be mistaken for code.
///
/// **Limit, and it is a deliberate trade.** An `#[unsafe(…)]` written inside a
/// macro invocation's delimiters is not counted, because the tokens there are
/// arguments the macro may discard — see [`attribute_spans`]. A macro that
/// *expands* to an undocumented unsafe attribute therefore escapes this scan.
/// That is the same trade the `m!(unsafe(foo))` exclusion already made one level
/// down: no text scan resolves macro expansion, and the alternative is a gate
/// satisfiable only by writing a `SAFETY:` comment about an attribute that never
/// exists. This crate has no such macro today.
fn unsafe_constructs(source: &str) -> Vec<UnsafeConstruct> {
    let chars: Vec<char> = source.chars().collect();
    let code = blank_comments_and_strings(&chars);
    let lines: Vec<&str> = source.lines().collect();
    // Line-for-line with `lines`: blanking preserves every newline and every
    // character position, so index `n` is the same line in both.
    let blanked: String = code.iter().collect();
    let code_lines: Vec<&str> = blanked.lines().collect();
    let block_lines = block_comment_lines(source);

    // `unsafe` must be a whole word, so a trailing-`unsafe` identifier such as
    // `my_unsafe extern_thing` cannot open a match.
    let word_start = |at: usize| {
        !code
            .get(at.wrapping_sub(1))
            .is_some_and(|c| c.is_alphanumeric() || *c == '_')
    };
    let skip_ws = |mut k: usize| {
        while code.get(k).is_some_and(|c| c.is_whitespace()) {
            k += 1;
        }
        k
    };
    // Compared char-by-char rather than by collecting `word`: this runs at every
    // char of every file the crate compiles, and an allocation per position
    // turned the scan from milliseconds into seconds.
    let matches_at = |k: usize, word: &str| {
        word.chars()
            .enumerate()
            .all(|(o, c)| code.get(k + o) == Some(&c))
    };
    let attributes = attribute_spans(&code);
    let attribute_lines = attribute_only_lines(&code, &attributes);

    let mut found = Vec::new();
    for i in 0..code.len() {
        if !(matches_at(i, "unsafe") && word_start(i)) {
            continue;
        }
        let bare = i + "unsafe".len();
        let after = skip_ws(bare);
        let hit = if after > bare && matches_at(after, "extern") {
            // A BLOCK, not merely the token pair. `pub unsafe extern "C" fn f()`,
            // `type Cb = unsafe extern "C" fn(i32)`, and an
            // `Option<unsafe extern "system" fn(u32)>` struct field are all
            // `unsafe extern`, and none of them declares a foreign signature: a
            // function-pointer type asserts nothing, so demanding a `// SAFETY:`
            // above one would be a comment about nothing — a gate satisfiable
            // only by writing something meaningless, which is the failure this
            // whole file exists to avoid. So: optional ABI string, then `{`.
            let mut k = skip_ws(after + "extern".len());
            if code.get(k) == Some(&'"') {
                k += 1;
                while k < code.len() && code.get(k) != Some(&'"') {
                    k += 1;
                }
                k = skip_ws(k + 1);
            }
            code.get(k) == Some(&'{')
        } else {
            // `unsafe(` anywhere INSIDE an attribute, rather than only as the
            // first token after `#[`. Same reason `opens_a_lint` finds a lint
            // level by position: an `#[unsafe(no_mangle)]` that applies on one
            // platform is written `#[cfg_attr(unix, unsafe(no_mangle))]`, and a
            // first-token test never sees it. Measured — that form compiles,
            // really does export the symbol, and draws no clippy diagnostic; both
            // real sites here are unix-only, so it is the first spelling an
            // author would reach for. Bounded to `attribute_spans` so macro
            // syntax spelling the same tokens is not mistaken for one.
            code.get(after) == Some(&'(')
                && attributes.iter().any(|span| i > span.open && i < span.end)
        };
        if !hit {
            continue;
        }
        let line = code[..i].iter().filter(|c| **c == '\n').count() + 1;
        // Two constructs can share a line only in generated code; keep the first.
        if found
            .last()
            .is_some_and(|last: &UnsafeConstruct| last.line == line)
        {
            continue;
        }
        found.push(UnsafeConstruct {
            line,
            // `.expect`, not `.unwrap_or_default()`: `line` is derived from this
            // same source, so a miss is impossible — and an empty `text` would
            // render as `src/foo.rs:12: `, a location naming nothing. Silence is
            // the failure mode this file rejects four lines below.
            text: lines
                .get(line - 1)
                .map(|l| l.trim().to_string())
                .expect("line index derived from the same source"),
            documented: has_safety_comment_above(
                &lines,
                &code_lines,
                &block_lines,
                &attribute_lines,
                line,
            ),
        });
    }
    found
}

/// One `#[…]` or `#![…]` attribute, brackets matched.
#[derive(Debug)]
struct AttributeSpan {
    /// Index of the `[` that opens it.
    open: usize,
    /// Index of the `]` that closes it, or the end of input if unterminated.
    end: usize,
    /// Index of the `#`. `open` is not enough to bound the *lines* an attribute
    /// covers: `# [unsafe(no_mangle)]` is legal, so the two can differ.
    hash: usize,
}

/// Every attribute in `code` (a [`blank_comments_and_strings`] copy), brackets
/// matched, so `#[cfg_attr(unix, unsafe(no_mangle))]` is one span with the
/// nested level inside it.
///
/// Membership in one of these, not "the previous character opens a list", is
/// what makes `unsafe(` an attribute. A bare list-opener test also matched
/// `m!(unsafe(foo))` and `macro_rules! m { (unsafe($x:ident)) => … }`, where no
/// attribute exists and no obligation is owed — a false positive satisfiable
/// only by a meaningless `SAFETY:` comment (Codex review, #479).
///
/// Tokens inside a macro invocation's delimiters are excluded for the same
/// reason, one level up: `discard!(#[unsafe(no_mangle)]);` compiles and rustfmt
/// preserves it, but wrapping the tokens in `#[…]` does not make them an applied
/// attribute — they are arguments, and the macro is free to drop them. The limit
/// this trades for is recorded on [`unsafe_constructs`].
fn attribute_spans(code: &[char]) -> Vec<AttributeSpan> {
    let at = |k: usize| code.get(k).copied();
    let skip_ws = |mut k: usize| {
        while at(k).is_some_and(|c| c.is_whitespace()) {
            k += 1;
        }
        k
    };
    let macros = macro_token_spans(code);
    let mut spans = Vec::new();
    let mut k = 0;
    while k < code.len() {
        if at(k) != Some('#') {
            k += 1;
            continue;
        }
        let after_hash = if at(k + 1) == Some('!') { k + 2 } else { k + 1 };
        let open = skip_ws(after_hash);
        if at(open) != Some('[') {
            k += 1;
            continue;
        }
        let mut depth = 0usize;
        let mut end = code.len();
        for j in open..code.len() {
            match at(j) {
                Some('[') => depth += 1,
                Some(']') => {
                    depth -= 1;
                    if depth == 0 {
                        end = j;
                        break;
                    }
                }
                _ => {}
            }
        }
        if !macros.iter().any(|(start, stop)| k > *start && k < *stop) {
            spans.push(AttributeSpan { open, end, hash: k });
        }
        k = end.saturating_add(1).max(open + 1);
    }
    spans
}

/// The character ranges a macro invocation's delimiters enclose — `ident!(…)`,
/// `ident![…]`, `ident!{…}`, and the body of a `macro_rules! name { … }`, whose
/// name sits between the `!` and the body.
///
/// A `!` that is not preceded by an identifier character is the negation
/// operator (`if !flag`, `a != b`), never a macro, so it opens nothing.
fn macro_token_spans(code: &[char]) -> Vec<(usize, usize)> {
    let at = |k: usize| code.get(k).copied();
    let is_ident = |c: char| c.is_alphanumeric() || c == '_';
    let skip_ws = |mut k: usize| {
        while at(k).is_some_and(|c| c.is_whitespace()) {
            k += 1;
        }
        k
    };
    let closing = |c: char| match c {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        _ => None,
    };
    let mut spans = Vec::new();
    let mut i = 0usize;
    while i < code.len() {
        if at(i) != Some('!') || !at(i.wrapping_sub(1)).is_some_and(is_ident) {
            i += 1;
            continue;
        }
        let mut k = skip_ws(i + 1);
        // `macro_rules! m { … }` — skip the name to reach the body.
        if at(k).is_some_and(is_ident) {
            while at(k).is_some_and(is_ident) {
                k += 1;
            }
            k = skip_ws(k);
        }
        let (Some(open), Some(close)) = (at(k), at(k).and_then(closing)) else {
            i += 1;
            continue;
        };
        let mut depth = 0usize;
        let mut end = code.len();
        for j in k..code.len() {
            if at(j) == Some(open) {
                depth += 1;
            } else if at(j) == Some(close) {
                depth -= 1;
                if depth == 0 {
                    end = j;
                    break;
                }
            }
        }
        spans.push((k, end));
        i = end.saturating_add(1).max(k + 1);
    }
    spans
}

/// For each line, whether it belongs to an attribute and to nothing else: it
/// touches one of `spans` and carries no code outside them.
///
/// This is what lets [`has_safety_comment_above`] walk a rustfmt-wrapped
/// attribute. `#[cfg_attr(` / `unix,` / `unsafe(no_mangle)` / `)]` records the
/// construct on its third line, and a walk that recognised only a line *starting*
/// with `#[` met the predicate line first, read it as code, and stopped short of
/// the justification sitting right above the attribute (Codex review, #479).
///
/// "And to nothing else" is the half that keeps this from clearing anything: a
/// line like `let x = 1; #[attr] f();` carries code outside the span, so the walk
/// still stops there rather than crediting a comment written about something
/// further up.
fn attribute_only_lines(code: &[char], spans: &[AttributeSpan]) -> Vec<bool> {
    let mut in_span = vec![false; code.len()];
    for span in spans {
        for slot in in_span
            .get_mut(span.hash..=span.end.min(code.len().saturating_sub(1)))
            .unwrap_or_default()
        {
            *slot = true;
        }
    }
    let mut touches = Vec::new();
    let mut outside = Vec::new();
    let (mut line_touches, mut line_outside) = (false, false);
    for (i, c) in code.iter().enumerate() {
        if *c == '\n' {
            touches.push(line_touches);
            outside.push(line_outside);
            (line_touches, line_outside) = (false, false);
            continue;
        }
        let marked = in_span.get(i).copied().unwrap_or(false);
        line_touches |= marked;
        line_outside |= !marked && !c.is_whitespace();
    }
    touches.push(line_touches);
    outside.push(line_outside);
    touches
        .into_iter()
        .zip(outside)
        .map(|(touches, outside)| touches && !outside)
        .collect()
}

/// For each line of `source`, whether any of it lies inside a `/* … */` block
/// comment (the marker lines included).
///
/// [`blank_comments_and_strings`] cannot answer this, which is why this exists
/// rather than reusing it: it blanks a comment's characters to whitespace, so an
/// empty interior line of a block comment and a blank separator line outside one
/// are *identical* in its output. The difference is load-bearing — the first
/// continues a justification, the second ends it.
///
/// Strings are skipped, so a `/*` inside a literal cannot open a phantom comment
/// that swallows the rest of the file.
fn block_comment_lines(source: &str) -> Vec<bool> {
    let chars: Vec<char> = source.chars().collect();
    let at = |k: usize| chars.get(k).copied();
    let mut inside = vec![false; source.split('\n').count()];
    let mut line = 0usize;
    let mut i = 0usize;
    while i < chars.len() {
        match (chars[i], at(i + 1)) {
            ('/', Some('/')) => {
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
            }
            ('/', Some('*')) => {
                let mut depth = 0usize;
                while i < chars.len() {
                    if let Some(slot) = inside.get_mut(line) {
                        *slot = true;
                    }
                    if chars[i] == '/' && at(i + 1) == Some('*') {
                        depth += 1;
                        i += 2;
                        continue;
                    }
                    if chars[i] == '*' && at(i + 1) == Some('/') {
                        depth -= 1;
                        i += 2;
                        if depth == 0 {
                            break;
                        }
                        continue;
                    }
                    if chars[i] == '\n' {
                        line += 1;
                    }
                    i += 1;
                }
            }
            ('r', Some('"' | '#')) => {
                let mut hashes = 0;
                let mut j = i + 1;
                while at(j) == Some('#') {
                    hashes += 1;
                    j += 1;
                }
                if at(j) != Some('"') {
                    i += 1;
                    continue;
                }
                j += 1;
                while j < chars.len() {
                    if chars[j] == '"' && (1..=hashes).all(|n| at(j + n) == Some('#')) {
                        j += hashes + 1;
                        break;
                    }
                    if chars[j] == '\n' {
                        line += 1;
                    }
                    j += 1;
                }
                i = j;
            }
            ('"', _) => {
                let mut j = i + 1;
                while j < chars.len() && chars[j] != '"' {
                    let escaped = chars[j] == '\\';
                    if chars[j] == '\n' {
                        line += 1;
                    }
                    j += 1;
                    if escaped && j < chars.len() {
                        if chars[j] == '\n' {
                            line += 1;
                        }
                        j += 1;
                    }
                }
                i = j + 1;
            }
            // A char literal, not a lifetime: `'x'`, `'\n'`. A lifetime (`'a`)
            // has no closing quote and must be left alone — the same test
            // [`blank_comments_and_strings`] makes.
            //
            // Without this arm a char literal holding a quote — `let quote =
            // '"';`, and this file has one — read as a string opener, and the
            // scan ran to the next `"` anywhere below it. Every `/* … */` in
            // between went unmarked, so a blank interior line in one of them
            // ended the walk in `has_safety_comment_above` and rejected a
            // justification that was really there (Codex review, #479).
            ('\'', _) => {
                let width = if at(i + 1) == Some('\\') { 3 } else { 2 };
                i += if at(i + width) == Some('\'') {
                    width + 1
                } else {
                    1
                };
            }
            (c, _) => {
                if c == '\n' {
                    line += 1;
                }
                i += 1;
            }
        }
    }
    inside
}

/// `true` when a `SAFETY:` comment sits in the unbroken run of comment and
/// attribute lines directly above `line` (1-indexed). `code_lines` is the
/// [`blank_comments_and_strings`] copy of the same source, line for line.
///
/// **A line is a comment when it carries no code**, per the blanked copy — never
/// because it starts with `//`, `*` or `/*`. A prefix test gets two things wrong,
/// in both directions:
///
///   * it rejects valid documentation. A block comment's continuation lines
///     carry no marker of their own, so `/* SAFETY:` / `the invariant holds.` /
///     `*/` reads as comment, *code*, comment — and the walk stops one line
///     short of the justification it was looking for.
///   * it clears undocumented code. `*slot = ' ';` — a real line in this very
///     file — starts with `*`, so the walk crossed statements and credited a
///     `SAFETY:` comment written about something else entirely. That is a false
///     *clearance*, the one direction a gate must never fail in.
///
/// Attribute lines are walked through so a `#[cfg(unix)]`-gated construct can
/// carry its comment above the `cfg` rather than wedged beneath it. That comes
/// from `attribute_lines` — [`attribute_only_lines`], brackets matched — rather
/// than from a `starts_with("#[")` test, so a rustfmt-wrapped attribute is
/// walked at every line of it and not just the one it opens on. Blank lines
/// **outside a comment** are not: walking them let a file-header
/// `//! … SAFETY: …` document the first construct in the file from any distance,
/// and an unbounded false clearance is worse than asking an author to close up
/// one blank line. A blank line *inside* a block comment is a different thing
/// entirely and is walked through — hence `block_lines`.
fn has_safety_comment_above(
    lines: &[&str],
    code_lines: &[&str],
    block_lines: &[bool],
    attribute_lines: &[bool],
    line: usize,
) -> bool {
    for above in (0..line.saturating_sub(1)).rev() {
        let Some(text) = lines.get(above).map(|l| l.trim()) else {
            return false;
        };
        let in_attribute = attribute_lines.get(above).copied().unwrap_or(false);
        if text.is_empty() {
            // An empty line INSIDE a `/* … */` continues the justification;
            // rustfmt preserves one, so `/* SAFETY: …` / `` / `details` / `*/`
            // is documentation the gate must accept (Codex review, #479). An
            // empty line OUTSIDE a comment ends the run, which is what stops a
            // file-header `//! … SAFETY: …` reaching down the file.
            if block_lines.get(above).copied().unwrap_or(false) || in_attribute {
                continue;
            }
            return false;
        }
        let is_comment = code_lines.get(above).is_some_and(|c| c.trim().is_empty());
        if is_comment {
            // Case-insensitive: clippy accepts `// Safety:` for the blocks it
            // does lint, and two gates for one CLAUDE.md rule disagreeing about
            // casing would reject a comment that visibly says Safety — then tell
            // the author not to reach for `#[allow]`.
            if text.to_ascii_uppercase().contains("SAFETY:") {
                return true;
            }
            continue;
        }
        if in_attribute {
            continue;
        }
        return false;
    }
    false
}

/// CLAUDE.md §Code style: "No `unsafe` unless explicitly justified with a
/// comment block explaining the invariant." `undocumented_unsafe_blocks` in
/// `Cargo.toml` is the gate for that rule — and it has a hole. It reaches
/// `unsafe { … }` blocks and `unsafe impl`, and nothing else; with that deny in
/// force, an `unsafe extern "C"` block carrying no justification at all still
/// exits clean ([`clippy_still_misses_the_constructs_this_gate_covers`] measures
/// every half, the `unsafe impl` exclusion included). Note the deny has to come
/// from the manifest or a crate root — `cargo clippy -D warnings` alone does not
/// enable a `restriction` lint at all, as `Cargo.toml` says. This crate declares
/// `kill` in two such blocks — `runtime/invoker.rs` and `runtime/pidfile.rs` —
/// so the rule was unenforced exactly where the crate makes its only raw FFI
/// declarations.
///
/// Scanned over every `.rs` file the crate compiles, `tests/` included: the
/// manifest `[lints]` table reaches every Cargo target, so this gate's reach
/// should match it rather than stop at `src/`. It does stop at `cli/`, though —
/// `20-agents/aeco/engineering/steel-detailer-lookup` is a separate workspace,
/// gated separately in `ci.yml`, and has no `unsafe` at all today.
///
/// Two limits worth stating plainly, so nobody reads more into a green run than
/// it earns:
///
///   * this proves a `SAFETY:` comment is *present*, never that it is *true* or
///     even that it is about the construct beneath it. A justification written
///     for one item, followed by prose that disclaims it, still reads as
///     documented. That is not a defect peculiar to this scan —
///     `undocumented_unsafe_blocks` has exactly the same property for the
///     blocks it does lint — and no text scan can close it. Reviewing whether
///     the words are true is a human's job; this only guarantees there are
///     words to review, which is the difference from the state before.
///   * the constructs covered are the two enumerated on [`unsafe_constructs`],
///     not everything clippy misses.
#[test]
fn every_unsafe_construct_clippy_misses_is_documented() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = vec![root.join("build.rs")];
    collect_rs_files(&root.join("src"), &mut files);
    collect_rs_files(&root.join("tests"), &mut files);
    assert!(
        files.len() > 120,
        "scanned only {} files under {} — the walk is truncated, so a clean \
         result means nothing",
        files.len(),
        root.display()
    );
    // A count is not coverage. `src/runtime/` is 11 of these ~156 files, so
    // losing it entirely still clears the floor above — and `offenders` would
    // then be empty because nothing was scanned rather than because nothing is
    // wrong. Assert the walk reaches the two files that hold the constructs.
    //
    // Named as *files*, deliberately, rather than as an expected construct
    // count: that is what lets this survive replacing either hand-rolled
    // declaration with `libc::kill`, which deletes the hazard outright instead
    // of documenting it. A control that punishes the better fix is worse than
    // no control.
    for required in ["src/runtime/invoker.rs", "src/runtime/pidfile.rs"] {
        assert!(
            files.iter().any(|f| f.ends_with(required)),
            "the walk never reached {required}, which is where this crate's raw \
             FFI declarations live — so a clean result means nothing"
        );
    }

    let mut offenders = Vec::new();
    for file in &files {
        // As in `no_targeted_allow_reopens_the_gate_in_src`: a file this gate
        // cannot read is a file it cannot clear, and silence there is the
        // failure mode CLAUDE.md §Engineering rules forbids.
        let source = std::fs::read_to_string(file)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", file.display()));
        let relative = file
            .strip_prefix(root)
            .unwrap_or(file)
            .display()
            .to_string();
        for construct in unsafe_constructs(&source) {
            if !construct.documented {
                offenders.push(format!("{relative}:{}: {}", construct.line, construct.text));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "these `unsafe` constructs carry no `// SAFETY:` comment, and clippy \
         does not lint them — write the invariant down above the construct. Do \
         not silence this with `#[allow]`; CLAUDE.md §Engineering rules forbids \
         satisfying a gate by disabling it:\n  {}",
        offenders.join("\n  ")
    );
}

/// The end-to-end control for the scan above, and the analogue of
/// [`the_scan_finds_a_planted_reopener_in_a_nested_file`]. That scan walks real
/// sources which are correct today, so it reports clean whether it works or has
/// stopped matching entirely; this plants **both** covered constructs in a
/// nested file, drives the same walk and classifier over them, and asserts what
/// the failure message would actually say — line and text, which are what a
/// maintainer navigates by.
///
/// This replaces a `seen >= 2` floor taken over the real crate, which counted
/// the wrong thing in two ways. It was satisfied by any two matches of *either*
/// kind, so the `#[unsafe(…)]` half could rot away undetected while the two real
/// `unsafe extern` blocks kept it green — and it was exactly tight, so replacing
/// either hand-rolled declaration with `libc::kill`, which deletes the hazard
/// rather than documenting it, would have failed the suite with "the classifier
/// has stopped matching". A control that punishes the better fix is worse than
/// no control.
#[test]
fn the_unsafe_scan_reports_planted_offenders_by_line_and_text() {
    let dir = tempfile::tempdir().expect("tempdir");
    let nested = dir.path().join("a/b");
    std::fs::create_dir_all(&nested).expect("create nested dirs");
    std::fs::write(
        nested.join("deep.rs"),
        "fn pad() {}\n\nunsafe extern \"C\" {\n    fn kill(pid: i32) -> i32;\n}\n\n\
         #[unsafe(no_mangle)]\npub extern \"C\" fn exported() {}\n",
    )
    .expect("write the planted file");

    let mut files = Vec::new();
    collect_rs_files(dir.path(), &mut files);
    assert_eq!(
        files.len(),
        1,
        "the walk did not reach the nested file: {files:?}"
    );

    let source = std::fs::read_to_string(&files[0]).expect("read the planted file");
    let found = unsafe_constructs(&source);
    let reported: Vec<(usize, &str)> = found.iter().map(|c| (c.line, c.text.as_str())).collect();
    assert_eq!(
        reported,
        vec![(3, "unsafe extern \"C\" {"), (7, "#[unsafe(no_mangle)]")],
        "the scan must find both covered constructs and name each at its own line"
    );
    assert!(
        found.iter().all(|c| !c.documented),
        "neither planted construct carries a SAFETY comment: {found:?}"
    );
}

/// The negative control for [`unsafe_constructs`] and
/// [`has_safety_comment_above`]. The scan above runs over real sources that are
/// correct today, so it reports clean both when it works and when it has stopped
/// matching; this drives the classifier over input it must catch and input it
/// must not.
#[test]
fn undocumented_unsafe_classifier_matches_its_contract() {
    let must_catch = [
        (
            "bare unsafe extern",
            "unsafe extern \"C\" {\n    fn kill(pid: i32) -> i32;\n}\n",
        ),
        (
            "unsafe extern under an unrelated comment",
            "// terminate the child\nunsafe extern \"C\" {\n    fn kill(pid: i32) -> i32;\n}\n",
        ),
        (
            "unsafe extern whose SAFETY comment is separated by real code",
            "// SAFETY: covers something else\nlet x = 1;\nunsafe extern \"C\" {\n    fn kill(p: i32);\n}\n",
        ),
        (
            "rustfmt-wrapped unsafe extern",
            "unsafe\nextern \"C\" {\n    fn kill(pid: i32) -> i32;\n}\n",
        ),
        ("unsafe attribute", "#[unsafe(no_mangle)]\nfn f() {}\n"),
        (
            "inner unsafe attribute",
            "#![unsafe(link_section = \".text\")]\n",
        ),
        // A first-token test misses these three. Measured: the `cfg_attr` form
        // compiles, exports the symbol, and draws no clippy diagnostic.
        (
            "unsafe attribute nested in a cfg_attr",
            "#[cfg_attr(unix, unsafe(no_mangle))]\npub extern \"C\" fn f() {}\n",
        ),
        (
            "unsafe attribute nested in a feature cfg_attr",
            "#[cfg_attr(feature = \"x\", unsafe(export_name = \"y\"))]\npub extern \"C\" fn f() {}\n",
        ),
        (
            "unsafe attribute with a space before the bracket",
            "# [unsafe(no_mangle)]\npub extern \"C\" fn f() {}\n",
        ),
        // False *clearances* — each of these once reported `documented: true`.
        (
            "a SAFETY comment separated from the construct by blank lines",
            "// SAFETY: about something else.\n\n\n\nunsafe extern \"C\" {\n    fn kill(p: i32);\n}\n",
        ),
        (
            "a module-doc SAFETY mention does not document a construct below it",
            "//! SAFETY: this module wraps libc.\n\n#![allow(dead_code)]\n\nunsafe extern \"C\" {\n    fn kill(p: i32);\n}\n",
        ),
        (
            "a SAFETY comment the walk would have to cross a statement to reach",
            "// SAFETY: about the line below.\n*slot = ' ';\nunsafe extern \"C\" {\n    fn kill(p: i32);\n}\n",
        ),
        (
            "a multi-line string mentioning SAFETY does not document what follows",
            "let s = \"SAFETY: not a comment\n\";\nunsafe extern \"C\" {\n    fn kill(p: i32);\n}\n",
        ),
    ];
    for (what, source) in must_catch {
        let found = unsafe_constructs(source);
        assert_eq!(
            found.len(),
            1,
            "{what}: expected one construct, got {found:?}"
        );
        assert!(
            !found[0].documented,
            "{what}: reported as documented, so the gate would pass it: {found:?}"
        );
    }

    let must_not_catch = [
        (
            "documented unsafe extern",
            "// SAFETY: matches POSIX `int kill(pid_t, int)`.\nunsafe extern \"C\" {\n    fn kill(p: i32) -> i32;\n}\n",
        ),
        (
            "documented through an intervening cfg attribute",
            "// SAFETY: matches POSIX.\n#[cfg(unix)]\nunsafe extern \"C\" {\n    fn kill(p: i32) -> i32;\n}\n",
        ),
        (
            "documented at the end of a longer comment block",
            "// Terminate the child.\n//\n// SAFETY: matches POSIX.\nunsafe extern \"C\" {\n    fn kill(p: i32);\n}\n",
        ),
        (
            "documented unsafe attribute",
            "// SAFETY: the symbol is unique to this crate.\n#[unsafe(no_mangle)]\nfn f() {}\n",
        ),
        // Codex review, PR #479: a block comment's continuation lines carry no
        // marker of their own, so a prefix test read the middle line as code and
        // stopped one line short of the justification — rejecting documentation
        // that complies with the rule the gate is enforcing.
        (
            "documented with a multiline block comment",
            "/* SAFETY: matches POSIX `int kill(pid_t, int)`.\n   The invariant holds.\n*/\nunsafe extern \"C\" {\n    fn kill(p: i32);\n}\n",
        ),
        (
            "documented with a starred block comment",
            "/*\n * SAFETY: matches POSIX.\n */\nunsafe extern \"C\" {\n    fn kill(p: i32);\n}\n",
        ),
        (
            "documented in the casing clippy also accepts",
            "// Safety: matches POSIX.\nunsafe extern \"C\" {\n    fn kill(p: i32);\n}\n",
        ),
        (
            "documented unsafe attribute nested in a cfg_attr",
            "// SAFETY: the symbol is unique to this crate.\n#[cfg_attr(unix, unsafe(no_mangle))]\npub extern \"C\" fn f() {}\n",
        ),
        // Codex review, PR #479 round 2: rustfmt preserves an empty line inside
        // a `/* … */`, so this is documentation the gate must accept — while a
        // blank line OUTSIDE a comment still ends the run (see `must_catch`).
        (
            "documented with a block comment containing a blank interior line",
            "/* SAFETY: matches POSIX `int kill(pid_t, int)`.\n\n   The invariant holds.\n*/\nunsafe extern \"C\" {\n    fn kill(p: i32);\n}\n",
        ),
        // Codex review, PR #479 round 3: rustfmt wraps a long `cfg_attr` across
        // lines, and the construct is then recorded on the inner `unsafe(…)`
        // line. A walk that starts there meets the predicate line first — no
        // `#[`, no comment — and stopped short of a justification that is
        // sitting right above the attribute. Both roles of a wrapped attribute
        // are the same defect: the one *carrying* the construct, and one merely
        // standing between the comment and it.
        (
            "documented through a rustfmt-wrapped cfg_attr carrying the construct",
            "// SAFETY: the symbol is unique to this crate.\n#[cfg_attr(\n    unix,\n    unsafe(no_mangle)\n)]\npub extern \"C\" fn f() {}\n",
        ),
        (
            "documented through an intervening rustfmt-wrapped attribute",
            "// SAFETY: matches POSIX.\n#[cfg_attr(\n    unix,\n    allow(dead_code)\n)]\nunsafe extern \"C\" {\n    fn kill(p: i32);\n}\n",
        ),
        // Codex review, PR #479 round 3: a char literal holding a double quote
        // is not a string opener. Reading it as one skips to the next `"` in the
        // file — here the one in `extern "C"` — so the block comment in between
        // is never marked, and its blank interior line then ends the walk.
        (
            "documented below a char literal holding a double quote",
            "let quote = '\"';\n/* SAFETY: matches POSIX.\n\n   The invariant holds.\n*/\nunsafe extern \"C\" {\n    fn kill(p: i32);\n}\n",
        ),
    ];
    for (what, source) in must_not_catch {
        let found = unsafe_constructs(source);
        assert_eq!(
            found.len(),
            1,
            "{what}: expected one construct, got {found:?}"
        );
        assert!(
            found[0].documented,
            "{what}: reported as undocumented, so the gate would reject correct \
             code: {found:?}"
        );
    }

    // Forms that are not these constructs at all. Anything matched here would be
    // a false positive the gate could only be satisfied by deleting.
    let must_not_match = [
        (
            "prose in a line comment",
            "// see the unsafe extern below\n",
        ),
        (
            "prose in a block comment",
            "/* an #[unsafe(no_mangle)] attribute */\n",
        ),
        (
            "the words inside a string literal",
            "let s = \"unsafe extern and #[unsafe(no_mangle)]\";\n",
        ),
        ("an identifier with the prefix", "let unsafe_extern = 1;\n"),
        (
            "a plain unsafe block, which clippy does lint",
            "fn f() {\n    unsafe { g() };\n}\n",
        ),
        (
            "an unsafe impl, which clippy does lint",
            "unsafe impl Send for T {}\n",
        ),
        (
            "a safe extern block",
            "extern \"C\" {\n    fn kill(p: i32);\n}\n",
        ),
        // `unsafe extern` is also the spelling of a callback signature, and none
        // of these declares anything: a function-pointer type asserts no
        // signature match, so a `// SAFETY:` above one would be a comment about
        // nothing. The crate already links `windows-sys`, so the struct-field
        // form is what the next Windows callback will look like.
        (
            "an unsafe extern fn definition",
            "pub unsafe extern \"C\" fn cb(a: i32) -> i32 { a }\n",
        ),
        (
            "an unsafe extern fn-pointer type alias",
            "type Cb = unsafe extern \"C\" fn(i32) -> i32;\n",
        ),
        (
            "an unsafe extern fn pointer in a struct field",
            "struct S {\n    cb: Option<unsafe extern \"system\" fn(u32) -> i32>,\n}\n",
        ),
        // Codex review, PR #479 round 2: `unsafe(` spelled in macro syntax opens
        // no attribute and owes no justification. A "the previous character
        // opens a list" test matched both of these.
        (
            "unsafe( in a macro_rules pattern",
            "macro_rules! m {\n    (unsafe($x:ident)) => { $x };\n}\n",
        ),
        ("unsafe( in a macro invocation", "m!(unsafe(foo));\n"),
        // Codex review, PR #479 round 3: wrapping the tokens in `#[…]` does not
        // make them an applied attribute — inside a macro's delimiters they are
        // arguments, which the macro is free to discard. rustc accepts this and
        // rustfmt preserves it, so flagging it demands a `SAFETY:` comment about
        // an attribute that never exists. See the limit this trades for, in the
        // doc comment on `unsafe_constructs`.
        (
            "attribute-shaped tokens passed to a macro",
            "discard!(#[unsafe(no_mangle)]);\n",
        ),
        (
            "attribute-shaped tokens in a macro_rules body",
            "macro_rules! m {\n    () => { discard!(#[unsafe(no_mangle)]) };\n}\n",
        ),
    ];
    for (what, source) in must_not_match {
        assert!(
            unsafe_constructs(source).is_empty(),
            "{what}: matched, but it is not a construct this gate covers: {:?}",
            unsafe_constructs(source)
        );
    }
}

/// The measurement the whole gate above rests on: clippy really does miss these
/// constructs, and really does catch the one they are contrasted with.
///
/// Without this, the gate is a text scan justified by a claim about clippy — and
/// if a future clippy extends `undocumented_unsafe_blocks` to cover
/// `unsafe extern`, nothing would say so and the scan would quietly become a
/// duplicate. This fails when that happens, which is the signal to delete it.
#[test]
fn clippy_still_misses_the_constructs_this_gate_covers() {
    if !clippy_available() {
        eprintln!("skipping: cargo clippy unavailable");
        return;
    }
    const UNSAFE_GATE: &str = "#![deny(clippy::undocumented_unsafe_blocks)]";

    // The hole: an `unsafe extern` block with no justification anywhere, whose
    // only *call* is documented. Clippy accepts it.
    let (accepted, diagnostics) = run_gate_with(
        UNSAFE_GATE,
        "unsafe extern \"C\" {\n    fn getpid() -> i32;\n}\n\n\
         fn main() {\n    // SAFETY: probe\n    let p = unsafe { getpid() };\n    \
         println!(\"{p}\");\n}\n",
    );
    assert!(
        accepted,
        "clippy now rejects an undocumented `unsafe extern` block — the lint has \
         been extended to cover it, so `every_unsafe_construct_clippy_misses_is_\
         documented` is now a duplicate of a live lint and its `unsafe extern` \
         half should be deleted. Diagnostics:\n{diagnostics}"
    );

    // An `#[unsafe(…)]` attribute, likewise unreached — in both the plain and
    // the `cfg_attr`-nested spelling, since the scan covers both.
    for attribute in [
        "#[unsafe(no_mangle)]",
        "#[cfg_attr(all(), unsafe(no_mangle))]",
    ] {
        let (accepted, diagnostics) = run_gate_with(
            UNSAFE_GATE,
            &format!("{attribute}\npub extern \"C\" fn probe() {{}}\n\nfn main() {{}}\n"),
        );
        assert!(
            accepted,
            "clippy now rejects an undocumented `{attribute}`, so that half of \
             the scan is a duplicate. Diagnostics:\n{diagnostics}"
        );
    }

    // The exclusion, measured rather than trusted. `unsafe impl` is deliberately
    // left OUT of `unsafe_constructs` because clippy reaches it — and that was
    // the one clippy fact this file asserted without checking. If a future
    // clippy narrows the lint off `unsafe impl`, every other test here stays
    // green while an undocumented `unsafe impl Send` ships ungated.
    let (accepted, diagnostics) = run_gate_with(
        UNSAFE_GATE,
        "struct T;\nunsafe impl Send for T {}\n\nfn main() {}\n",
    );
    assert!(
        !accepted,
        "clippy no longer rejects an undocumented `unsafe impl`. \
         `unsafe_constructs` excludes it on the strength of this measurement, so \
         the scan must now grow to cover it. Diagnostics:\n{diagnostics}"
    );
    assert!(
        diagnostics.contains("unsafe impl missing a safety comment"),
        "the `unsafe impl` probe was rejected, but not by the unsafe gate. \
         Diagnostics:\n{diagnostics}"
    );

    // The control. Without it the assertions above would also pass if the
    // probe silently stopped enforcing anything at all.
    let (accepted, diagnostics) = run_gate_with(
        UNSAFE_GATE,
        "unsafe extern \"C\" {\n    fn getpid() -> i32;\n}\n\n\
         fn main() {\n    let p = unsafe { getpid() };\n    println!(\"{p}\");\n}\n",
    );
    assert!(
        !accepted,
        "clippy accepted an undocumented `unsafe {{ … }}` block — the probe is \
         not enforcing `undocumented_unsafe_blocks`, so the two assertions above \
         prove nothing"
    );
    assert!(
        diagnostics.contains("undocumented_unsafe_blocks")
            || diagnostics.contains("missing a safety comment"),
        "the probe was rejected, but not by the unsafe gate — it may be failing \
         for an unrelated reason. Diagnostics:\n{diagnostics}"
    );
}

/// Collect every `.rs` file under `dir`, recursively.
///
/// Panics rather than returning early on an unreadable directory. The previous
/// `let Ok(…) else { return }` turned a failed walk into a short file list, and
/// the caller's only defence was an is-empty check — so losing every nested
/// module but keeping the top-level ones read as a clean scan.
fn collect_rs_files(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    let entries =
        std::fs::read_dir(dir).unwrap_or_else(|e| panic!("cannot walk {}: {e}", dir.display()));
    for entry in entries {
        let path = entry
            .unwrap_or_else(|e| panic!("cannot read an entry in {}: {e}", dir.display()))
            .path();
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
}
