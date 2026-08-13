//! Negative control for the lint gate in `src/main.rs`.
//!
//! A `deny` attribute is only worth anything if it still fires. These tests
//! compile a throwaway crate carrying the *same* attribute this crate carries
//! and assert that clippy rejects a violation and accepts its absence — so a
//! future edit that removes or narrows the gate fails the suite instead of
//! passing quietly.
//!
//! This mirrors `cli/tests/lint_gates.rs`, which guards the identical attribute
//! on the `aware` binary. Deliberately a copy rather than a shared helper: the
//! two live in separate cargo workspaces (this crate declares its own
//! `[workspace]`), so there is no crate they could both depend on without
//! inventing one, and a gate's negative control that can be broken from another
//! workspace is not much of a control.
//!
//! What is covered, precisely:
//!   * the attribute denies `unwrap()` / `expect()` in non-test code, and it is
//!     the *denied lint* that rejects — not an unrelated probe failure;
//!   * clean code is still accepted, so a rejection above means something;
//!   * the `cfg(test)` carve-out still holds, witnessed by a companion probe so
//!     the assertion cannot pass by never compiling a test target at all;
//!   * `src/main.rs` still carries the gate verbatim;
//!   * nobody has re-opened it with an `#[allow]` / `#[expect]` / a
//!     `clippy::restriction` group allow, nor from `[lints.clippy]` in the
//!     manifest, where a group entry that outranks a specific `deny` switches
//!     it off.
//!
//! The last two scan an artefact that is correct today — the real `src/main.rs`,
//! the real `Cargo.toml` — so they would report clean both when they work and
//! when they have stopped matching anything at all. Hence
//! `gate_reopener_classifier_matches_its_contract`, which drives the classifier
//! over synthetic input with known answers.
//!
//! The clippy-backed probes shell out to `cargo clippy` on a scratch crate with
//! no dependencies (hence `--offline`); the rest are pure file and string
//! checks. If clippy is missing the probes skip — except under `CI`, where the
//! workflow installs the component explicitly and a silent skip would be a hole
//! rather than a courtesy.

use std::path::Path;
use std::process::Command;

/// The gate under test, kept byte-identical to the attribute in `src/main.rs`.
const GATE: &str = "#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]";

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
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    std::fs::create_dir_all(root.join("src")).expect("src dir");
    std::fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"gate_probe\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[workspace]\n",
    )
    .expect("write manifest");
    std::fs::write(root.join("src/main.rs"), format!("{GATE}\n\n{body}")).expect("write main");

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
    // reason alone rather than because the carve-out held.
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
fn crate_root_actually_carries_the_gate() {
    // The probes above prove the attribute denies. This proves the shipped
    // crate is the thing carrying it — otherwise the gate could be deleted from
    // `src/main.rs` and every behavioural assertion above would still pass.
    let source = std::fs::read_to_string(manifest_dir().join("src/main.rs")).expect("read main.rs");
    assert!(
        source.contains(GATE),
        "`src/main.rs` no longer carries the gate verbatim:\n  {GATE}\n\
         If it was deliberately reworded, update `GATE` here in the same change \
         so the probes keep testing what the crate actually carries."
    );
}

#[test]
fn nobody_reopened_the_gate_from_source() {
    let source = std::fs::read_to_string(manifest_dir().join("src/main.rs")).expect("read main.rs");
    let reopened: Vec<(usize, &str)> = source
        .lines()
        .enumerate()
        .filter(|(_, line)| reopens_gate(line))
        .map(|(i, line)| (i + 1, line.trim()))
        .collect();
    assert!(
        reopened.is_empty(),
        "`src/main.rs` re-opens the gate it is supposed to be under. \
         CLAUDE.md forbids silencing a gate rather than fixing the violation \
         under it:\n{reopened:#?}"
    );
}

#[test]
fn nobody_reopened_the_gate_from_the_manifest() {
    // `[lints.clippy]` outranks a crate-root attribute, so a group-level allow
    // there switches the gate off with nothing in `src/` to show for it.
    let manifest = std::fs::read_to_string(manifest_dir().join("Cargo.toml")).expect("read mani");
    let reopened: Vec<&str> = manifest
        .lines()
        .map(str::trim)
        .filter(|line| manifest_reopens_gate(line))
        .collect();
    assert!(
        reopened.is_empty(),
        "`Cargo.toml` switches off a lint the crate root denies:\n{reopened:#?}"
    );
    assert!(
        manifest.contains("undocumented_unsafe_blocks = \"deny\""),
        "`Cargo.toml` no longer denies `undocumented_unsafe_blocks` — CLAUDE.md's \
         \"no `unsafe` without a justification\" rule has lost its only gate here."
    );
}

/// Negative control for the two scanners above. They read artefacts that are
/// clean today, so they report success both when they work and when they have
/// stopped matching anything; this drives them over inputs with known answers.
#[test]
fn gate_reopener_classifier_matches_its_contract() {
    for line in [
        "#[allow(clippy::unwrap_used)]",
        "    #![allow(clippy::expect_used)]",
        "#[expect(clippy::unwrap_used)]",
        "#![allow(clippy::restriction)]",
        "#[cfg_attr(windows, allow(clippy::unwrap_used))]",
        "#[allow(clippy::undocumented_unsafe_blocks)]",
    ] {
        assert!(reopens_gate(line), "should have been flagged: {line}");
    }
    for line in [
        "#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]",
        "#[allow(dead_code)]",
        "// an allow(clippy::unwrap_used) mentioned in prose, not an attribute",
        "let unwrap_used = 1;",
    ] {
        assert!(!reopens_gate(line), "should not have been flagged: {line}");
    }

    for line in [
        "unwrap_used = \"allow\"",
        "expect_used = 'allow'",
        "restriction = \"allow\"",
        "undocumented_unsafe_blocks = \"warn\"",
    ] {
        assert!(
            manifest_reopens_gate(line),
            "manifest line should have been flagged: {line}"
        );
    }
    for line in [
        "undocumented_unsafe_blocks = \"deny\"",
        "serde_json = \"1\"",
        "# unwrap_used = \"allow\" — a commented-out example, not active",
    ] {
        assert!(
            !manifest_reopens_gate(line),
            "manifest line should not have been flagged: {line}"
        );
    }
}

/// This crate's root directory.
fn manifest_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

/// `true` when `line` is an attribute that relaxes one of the denied lints.
///
/// Matches on the attribute opener so prose and identifiers that merely contain
/// a lint name do not trip it, and covers `cfg_attr`-wrapped levels because the
/// level is what takes effect, not the predicate around it.
fn reopens_gate(line: &str) -> bool {
    let trimmed = line.trim();
    if !(trimmed.starts_with("#[") || trimmed.starts_with("#![")) {
        return false;
    }
    let relaxes = ["allow(", "expect(", "warn("];
    let targets = [
        "clippy::unwrap_used",
        "clippy::expect_used",
        "clippy::undocumented_unsafe_blocks",
        "clippy::restriction",
    ];
    relaxes.iter().any(|level| trimmed.contains(level))
        && targets.iter().any(|target| trimmed.contains(target))
}

/// `true` when `line` is a `[lints.clippy]` entry that relaxes a denied lint.
fn manifest_reopens_gate(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.starts_with('#') {
        return false;
    }
    let Some((key, value)) = trimmed.split_once('=') else {
        return false;
    };
    let key = key.trim();
    let value = value.trim().trim_matches(['"', '\'']);
    let gated = [
        "unwrap_used",
        "expect_used",
        "undocumented_unsafe_blocks",
        "restriction",
    ];
    gated.contains(&key) && matches!(value, "allow" | "warn")
}
