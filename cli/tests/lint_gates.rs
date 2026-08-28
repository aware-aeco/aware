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
//!   * nobody has re-opened any of those gates from `src/` **or `build.rs`** —
//!     with an `#[allow]`, an `#[expect]`, a `clippy::restriction` group allow,
//!     a wrapped attribute, or a level nested in a `cfg_attr` predicate — nor
//!     from `[lints.clippy]` in the manifest, where a group entry that outranks
//!     a specific `deny` switches it off.
//!
//! The last two scan artefacts that are correct today — real `src/`, the real
//! `Cargo.toml` — so they report clean both when they work and when they have
//! stopped matching anything at all. Each therefore has a negative control
//! driving its classifier over synthetic input:
//! `gate_reopener_classifier_matches_its_contract` and
//! `manifest_lint_reader_matches_its_contract`.
//!
//! The clippy-backed ones shell out to `cargo clippy` on a two-file scratch
//! crate with no dependencies (hence `--offline`); the rest are pure file and
//! string checks. If clippy is missing those four skip — except under `CI`,
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
