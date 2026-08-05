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
//!   * nobody has re-opened either gate with a targeted `#[allow]` in `src/`.
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

/// `deny` can be re-opened for a single function with one `#[allow]`, and
/// clippy stays green — so the attribute alone is not the whole gate. CI has no
/// other signal for this, hence a test.
///
/// Heuristic, and deliberately a blunt one: an `allow` for these lints is
/// reported unless it sits after the file's first `#[cfg(test)]`, which is where
/// this crate puts its unit tests. CLAUDE.md permits `unwrap()` in tests, so an
/// `allow` down there is redundant rather than harmful.
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
        let test_mod_line = source
            .lines()
            .position(|line| line.trim_start().starts_with("#[cfg(test)]"))
            .unwrap_or(usize::MAX);
        for (index, line) in source.lines().enumerate() {
            let opens_gate = line.contains("allow(clippy::unwrap_used")
                || line.contains("allow(clippy::expect_used")
                || line.contains("allow(clippy::indexing_slicing");
            if opens_gate && index < test_mod_line {
                offenders.push(format!(
                    "{}:{}: {}",
                    file.strip_prefix(&src).unwrap_or(file).display(),
                    index + 1,
                    line.trim()
                ));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "these `#[allow]`s re-open the unwrap/expect gate in non-test code — fix \
         the call site instead (CLAUDE.md §Engineering rules forbids silencing a \
         gate to satisfy it):\n  {}",
        offenders.join("\n  ")
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
