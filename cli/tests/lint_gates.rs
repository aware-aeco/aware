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
//!   * nobody has re-opened any of those gates from `src/` — with an `#[allow]`,
//!     an `#[expect]`, a `clippy::restriction` group allow, or a wrapped
//!     attribute — nor from `[lints]` in the manifest.
//!
//! The last of those carries its own negative control
//! (`gate_reopener_classifier_matches_its_contract`), because it scans real
//! `src/`, which contains no such attribute — so it reports clean both when it
//! works and when it has stopped matching anything at all.
//!
//! They shell out to `cargo clippy` on a two-file scratch crate with no
//! dependencies (hence `--offline`). If clippy is missing the tests skip —
//! except under `CI`, where the workflow installs the component explicitly and
//! a silent skip would be a hole rather than a courtesy.

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
const GATED_GROUPS: [&str; 1] = ["clippy::restriction"];

/// A lint-level attribute that would re-open one of the gates.
#[derive(Debug, PartialEq, Eq)]
struct Reopener {
    /// 1-indexed line the attribute starts on.
    line: usize,
    /// The attribute, whitespace-collapsed onto one line.
    text: String,
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
/// first `#[cfg(test)]` line, and that carve-out was the bug — the first
/// `#[cfg(test)]` in a file is frequently not the unit-test module but a
/// cfg-gated `use`, which blinded the check to the whole rest of the file
/// (`src/runtime/invoker.rs` line 9, exempting 4,193 of its 4,202 lines; 5,332
/// non-test lines across the crate). Dropping the carve-out removes the class of
/// bug rather than patching one instance of it, and costs nothing: the crate's
/// gate is `cfg_attr(not(test), deny(…))`, so inside `#[cfg(test)]` the lint is
/// not denied and such an `allow` is redundant. If one is ever reported from a
/// test module, the fix is to delete it.
///
/// Only line-initial attributes count, and lines opening with `//` are skipped,
/// so prose naming an attribute (several module docs do) is not a finding.
/// Attributes are joined across lines until their brackets balance, because
/// rustfmt splits a long one and a single-line `contains` misses every wrapped
/// form.
fn gate_reopeners(source: &str) -> Vec<Reopener> {
    let lines: Vec<&str> = source.lines().collect();
    let mut found = Vec::new();
    let mut index = 0;
    while index < lines.len() {
        let trimmed = lines[index].trim_start();
        if trimmed.starts_with("//") || !(trimmed.starts_with("#[") || trimmed.starts_with("#![")) {
            index += 1;
            continue;
        }

        // Join until the attribute's brackets balance, so a wrapped attribute is
        // classified as the single token it is.
        let start = index;
        let mut attribute = String::new();
        let mut depth: i32 = 0;
        loop {
            let line = lines.get(index).copied().unwrap_or_default();
            attribute.push_str(line.trim());
            attribute.push(' ');
            depth += line.matches('[').count() as i32 - line.matches(']').count() as i32;
            index += 1;
            if depth <= 0 || index >= lines.len() {
                break;
            }
        }

        let collapsed = attribute.split_whitespace().collect::<Vec<_>>().join(" ");
        // `expect` as well as `allow`: `#[expect(clippy::unwrap_used)]` is stable
        // and sets the same lint level, so grepping only for `allow` left an
        // opening that clippy honours in full.
        let is_lint_level = collapsed.starts_with("#[allow(")
            || collapsed.starts_with("#![allow(")
            || collapsed.starts_with("#[expect(")
            || collapsed.starts_with("#![expect(");
        if !is_lint_level {
            continue;
        }
        let names_gated = GATED_LINTS.iter().any(|lint| collapsed.contains(lint))
            || GATED_GROUPS.iter().any(|group| collapsed.contains(group));
        if names_gated {
            found.push(Reopener {
                line: start + 1,
                text: collapsed,
            });
        }
    }
    found
}

/// `deny` can be re-opened for a single item with one `#[allow]` or `#[expect]`,
/// and clippy stays green — so the attribute alone is not the whole gate. CI has
/// no other signal for this, hence a test.
#[test]
fn no_targeted_allow_reopens_the_gate_in_src() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut offenders = Vec::new();
    let mut files = Vec::new();
    collect_rs_files(&src, &mut files);
    assert!(
        !files.is_empty(),
        "found no .rs files under {}",
        src.display()
    );

    for file in &files {
        let Ok(source) = std::fs::read_to_string(file) else {
            continue;
        };
        let relative = file
            .strip_prefix(&src)
            .unwrap_or(file)
            .display()
            .to_string();
        for hit in gate_reopeners(&source) {
            offenders.push(format!("{relative}:{}: {}", hit.line, hit.text));
        }
    }

    assert!(
        offenders.is_empty(),
        "these attributes re-open a lint gate under `src/` — fix the call site \
         instead (CLAUDE.md §Engineering rules forbids silencing a gate to \
         satisfy it). Inside `#[cfg(test)]` the lint is not denied at all, so an \
         allow there is redundant and should simply be deleted:\n  {}",
        offenders.join("\n  ")
    );
}

/// The negative control for [`gate_reopeners`]. The scan above runs over real
/// `src/`, which currently contains none of these attributes — so it reports
/// clean whether the classifier works or has stopped matching entirely. This
/// drives it with sources that must be caught and sources that must not.
///
/// Every "must be caught" case below was measured against `cargo clippy` (see
/// [`gated_groups_actually_reopen_the_gate_and_others_do_not`]) to genuinely
/// suppress the crate's `deny`; the previous classifier caught only the first.
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
            // The bug this rewrite fixes: a cfg-gated `use` near the top of a
            // file used to exempt everything below it.
            "below an early cfg(test) item",
            "#[cfg(test)]\nuse std::collections::HashMap;\n\n#[allow(clippy::unwrap_used)]\nfn f() {}\n",
        ),
        (
            "inside a unit-test module, where it is redundant",
            "#[cfg(test)]\nmod tests {\n    #[allow(clippy::unwrap_used)]\n    fn t() {}\n}\n",
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
        let (accepted, diagnostics) = run_gate(&violation(group));
        assert!(
            accepted,
            "`#[allow({group})]` no longer suppresses the gate, so listing it in \
             GATED_GROUPS now rejects code that is still gated:\n{diagnostics}"
        );
    }

    for group in ["clippy::all", "clippy::pedantic", "clippy::correctness"] {
        let (accepted, _) = run_gate(&violation(group));
        assert!(
            !accepted,
            "`#[allow({group})]` now suppresses the gate but is absent from \
             GATED_GROUPS — the classifier would let it through"
        );
    }
}

/// `undocumented_unsafe_blocks` is denied in `Cargo.toml`, not in a crate-root
/// attribute, so `crate_roots_actually_carry_the_gate` never saw it and nothing
/// else did either: deleting that one line leaves every check in this file
/// green. This anchors it, and rejects re-opening it through the same manifest.
#[test]
fn the_manifest_still_carries_the_unsafe_gate() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let source =
        std::fs::read_to_string(&manifest).unwrap_or_else(|e| panic!("read Cargo.toml: {e}"));

    assert!(
        source.contains(r#"undocumented_unsafe_blocks = "deny""#),
        "Cargo.toml no longer denies `undocumented_unsafe_blocks` — CLAUDE.md \
         §Code style requires every `unsafe` block to carry a justification, and \
         `cargo clippy -D warnings` does not enable that lint on its own"
    );

    // A `[lints]` entry can re-open a gate as effectively as an `#[allow]`, and
    // from a file the source scan above never reads.
    let reopened: Vec<&str> = source
        .lines()
        .map(str::trim)
        .filter(|line| {
            GATED_LINTS.iter().any(|lint| line.starts_with(lint))
                && (line.contains(r#""allow""#) || line.contains(r#""warn""#))
        })
        .collect();
    assert!(
        reopened.is_empty(),
        "these `[lints]` entries downgrade a gated lint below `deny`:\n  {}",
        reopened.join("\n  ")
    );
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

fn collect_rs_files(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
}
