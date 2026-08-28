//! Negative control for the windows-target clippy gate in
//! `.github/workflows/ci.yml`.
//!
//! CLAUDE.md §Code style mechanises three of its rules through clippy:
//! `cargo clippy --all-targets -- -D warnings` must pass, `src/main.rs` carries
//! `#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]` for
//! "no `unwrap()` outside of tests + main entry", and `Cargo.toml` denies
//! `undocumented_unsafe_blocks` for "no `unsafe` without a justification".
//! `cli/tests/lint_gates.rs` proves all three still fire.
//!
//! It proves it for the HOST, which is the whole of the hole this file exists
//! for. `ci.yml`'s `gates` job runs on `ubuntu-latest`, and clippy only ever
//! sees the code the active `cfg` set compiles — so 262 lines of shipped Rust
//! behind `#[cfg(windows)]` / `#[cfg(target_os = "macos")]`, across six files
//! and 182 of them in `commands/model_reader_host.rs`, were checked by nothing
//! at all. Not by `release.yml` and not by `bridge-windows-packaged`: both build
//! the crate on Windows, but with `cargo build`, and rustc accepts a `clippy::`
//! tool lint and then ignores it.
//!
//! Measured before the step was written, so this is a finding rather than a
//! worry: an `unwrap()` planted in the `#[cfg(windows)]` `process_is_alive` in
//! `src/runtime/pidfile.rs` draws zero errors from `cargo clippy --all-targets
//! -- -D warnings` on Linux, and `error: used `unwrap()` on a `Result` value`
//! from the same command under `--target x86_64-pc-windows-gnu`.
//!
//! What is asserted here:
//!   * `ci.yml` still runs clippy for a windows target, with `--all-targets`,
//!     `--locked` and `-D warnings` — each of which the step is useless without;
//!   * the triple it checks is the triple the toolchain step installs, so the
//!     two cannot drift into a step that fails for want of a target rather than
//!     passing for having checked one;
//!   * no `target_env` cfg exists in `cli/`, which is the condition that makes
//!     the `-gnu` triple a faithful proxy for the shipped `-msvc` one;
//!   * platform-gated code still exists, so the step is not guarding an empty
//!     set.
//!
//! The first, third and fourth scan artefacts that are correct today — the real
//! `ci.yml`, the real `src/` — so each would report clean both when it works and
//! when it has stopped matching anything. The first two therefore drive their
//! classifier over synthetic input as well:
//! `the_cross_check_reader_matches_its_contract` and
//! `the_target_env_scanner_matches_its_contract`.
//!
//! Pure file and string checks throughout — nothing here shells out to cargo, so
//! it costs nothing and cannot skip.

use std::path::{Path, PathBuf};

/// Repository root — `cli/`'s parent. The workflow under test lives there.
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap_or_else(|| panic!("{} has no parent", env!("CARGO_MANIFEST_DIR")))
        .to_path_buf()
}

/// One workflow step: its `name:` and its body with comment lines removed.
///
/// Comments are dropped for the same reason `tests/lockfile_gate.rs` drops them:
/// the step under test is preceded by a comment block that names every flag
/// asserted below, so a `contains` over the raw text would pass on the prose
/// after the command had been deleted.
struct Step {
    name: String,
    body: String,
}

/// Split a workflow's steps in order.
fn steps(workflow: &str) -> Vec<Step> {
    workflow
        .split("\n      - ")
        .skip(1) // the job header, before the first step
        .map(|chunk| Step {
            name: chunk
                .lines()
                .next()
                .and_then(|first| first.strip_prefix("name: "))
                .unwrap_or_default() // a bare `uses:` step has no name
                .trim()
                .to_string(),
            body: chunk
                .lines()
                .filter(|line| !line.trim_start().starts_with('#'))
                .collect::<Vec<_>>()
                .join("\n"),
        })
        .collect()
}

/// The step whose `name:` is `name`, or a failure naming what was searched.
fn step<'a>(steps: &'a [Step], name: &str) -> &'a Step {
    steps
        .iter()
        .find(|step| step.name == name)
        .unwrap_or_else(|| {
            panic!(
                "ci.yml has no step named {name:?} — it was renamed or removed, and this \
                 test can no longer tell whether the windows cross-check still runs. \
                 Steps present: {:?}",
                steps.iter().map(|s| &s.name).collect::<Vec<_>>()
            )
        })
}

/// What a windows cross-check step is missing, or the triple it checks.
#[derive(Debug, PartialEq, Eq)]
enum CrossCheck {
    /// The step checks this target triple, with every flag that makes it a gate.
    Checks(String),
    /// The step is present but cannot do its job; the field names what is absent.
    Missing(&'static str),
}

/// Read a step's `run:` body and judge whether it cross-checks a windows target.
///
/// The `--target` value is read positionally rather than by substring, because
/// `--all-targets` contains `-target` and a `contains("--target")` therefore
/// reports a windows cross-check on a step that only ever compiles for the host.
fn cross_check(body: &str) -> CrossCheck {
    if !body.contains("cargo clippy") {
        return CrossCheck::Missing("cargo clippy");
    }
    let Some(triple) = body
        .split_whitespace()
        .skip_while(|token| *token != "--target")
        .nth(1)
        .filter(|triple| triple.contains("-pc-windows-"))
    else {
        return CrossCheck::Missing("--target <a *-pc-windows-* triple>");
    };
    // Without `--all-targets` the integration tests under `cli/tests/` are not
    // checked — `tests/app_id_is_a_segment.rs` carries a `#[cfg(windows)]` case.
    if !body.contains("--all-targets") {
        return CrossCheck::Missing("--all-targets");
    }
    // Without `-D warnings` clippy prints and exits 0, so the step reports green
    // on exactly the code it was added to reject.
    if !body.contains("-D warnings") {
        return CrossCheck::Missing("-D warnings");
    }
    // Same reason every other cargo call in this job carries it: without
    // `--locked` the step resolves around a stale lockfile instead of failing.
    if !body.contains("--locked") {
        return CrossCheck::Missing("--locked");
    }
    CrossCheck::Checks(triple.to_owned())
}

/// The target triples a `dtolnay/rust-toolchain` step installs.
fn installed_targets(body: &str) -> Vec<String> {
    body.lines()
        .filter_map(|line| line.trim().strip_prefix("targets:"))
        .flat_map(|list| {
            list.split(',')
                .map(|triple| triple.trim().to_owned())
                .filter(|triple| !triple.is_empty())
                .collect::<Vec<_>>()
        })
        .collect()
}

/// The gate itself: CI still checks the platform-gated code, on a target it has.
#[test]
fn ci_still_cross_checks_a_windows_target() {
    let workflow = repo_root().join(".github/workflows/ci.yml");
    let source = std::fs::read_to_string(&workflow)
        .unwrap_or_else(|e| panic!("read {}: {e}", workflow.display()));
    let steps = steps(&source);

    // A floor, not an is-empty check: a splitter that recovered one step would
    // pass an emptiness test having silently dropped the rest, and the lookups
    // below would then fail for the wrong reason.
    assert!(
        steps.len() > 8,
        "parsed only {} steps out of ci.yml — the splitter is broken, so nothing \
         below means anything",
        steps.len()
    );

    let checked = match cross_check(&step(&steps, "cargo clippy -D warnings (windows target)").body)
    {
        CrossCheck::Checks(triple) => triple,
        CrossCheck::Missing(what) => panic!(
            "ci.yml's windows cross-check step no longer passes `{what}`, so the \
             `#[cfg(windows)]` half of `cli/src` is back to being checked by nothing. \
             CLAUDE.md §Code style's clippy rules are enforced only over the code the \
             active cfg set compiles, and `gates` runs on ubuntu-latest"
        ),
    };

    // The host step has to stay too. This one only ADDS a cfg set; it is not a
    // replacement, and a step that swapped `--target` onto the existing call
    // would trade one blind spot for another.
    assert!(
        matches!(
            cross_check(&step(&steps, "cargo clippy -D warnings").body),
            CrossCheck::Missing("--target <a *-pc-windows-* triple>")
        ),
        "ci.yml's host clippy step now passes a windows `--target`, so nothing checks \
         the crate for the platform CI actually runs on"
    );

    // The triple checked must be one rustup was told to install. Drift here does
    // not fail loudly in an obvious way — it fails with "target may not be
    // installed", which reads like an infrastructure hiccup rather than a gate.
    let installed =
        installed_targets(&step(&steps, "Install Rust ${{ steps.pin.outputs.channel }}").body);
    assert!(
        installed.contains(&checked),
        "ci.yml cross-checks {checked:?} but the toolchain step installs {installed:?} — \
         the step would fail for want of a target rather than pass for having checked one"
    );
}

/// Negative control for [`cross_check`]: it must reject every degraded shape.
///
/// Without this the assertion above reports clean both when the classifier works
/// and when it has stopped matching anything at all.
#[test]
fn the_cross_check_reader_matches_its_contract() {
    let good = "working-directory: cli\nrun: |\n  cargo clippy --target x86_64-pc-windows-gnu \
                --all-targets --locked -- -D warnings";
    assert_eq!(
        cross_check(good),
        CrossCheck::Checks("x86_64-pc-windows-gnu".to_owned()),
        "the classifier rejects the real step, so it can only ever report failure"
    );
    assert_eq!(
        cross_check(
            "run: cargo clippy --target x86_64-pc-windows-msvc --all-targets --locked -- -D warnings"
        ),
        CrossCheck::Checks("x86_64-pc-windows-msvc".to_owned()),
        "the classifier is pinned to one windows triple; moving to msvc, or to \
         aarch64, must still count as a cross-check"
    );

    for (body, missing, why) in [
        (
            "run: cargo build --target x86_64-pc-windows-gnu --all-targets --locked -- -D warnings",
            "cargo clippy",
            "a build is not a lint run",
        ),
        (
            "run: cargo clippy --all-targets --locked -- -D warnings",
            "--target <a *-pc-windows-* triple>",
            "`--all-targets` contains `-target`, and a substring search reads it as one",
        ),
        (
            "run: cargo clippy --target x86_64-unknown-linux-gnu --all-targets --locked -- -D warnings",
            "--target <a *-pc-windows-* triple>",
            "a second host check is not a windows check",
        ),
        (
            "run: cargo clippy --target x86_64-pc-windows-gnu --locked -- -D warnings",
            "--all-targets",
            "without it the windows-only integration test is not compiled",
        ),
        (
            "run: cargo clippy --target x86_64-pc-windows-gnu --all-targets --locked",
            "-D warnings",
            "clippy prints and exits 0, so the step is green on the code it rejects",
        ),
        (
            "run: cargo clippy --target x86_64-pc-windows-gnu --all-targets -- -D warnings",
            "--locked",
            "the step would resolve around a stale lockfile instead of failing",
        ),
    ] {
        assert_eq!(
            cross_check(body),
            CrossCheck::Missing(missing),
            "the classifier accepted a step it must reject ({why}): {body}"
        );
    }
}

/// Every `.rs` file under `cli/src` and `cli/tests`, excluding this one.
///
/// This file is excluded because its own prose and its negative-control fixtures
/// contain the very string the scan looks for; including it would make the scan
/// fail on itself and prove nothing about the crate.
fn crate_sources() -> Vec<PathBuf> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                out.push(path);
            }
        }
    }
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut out = Vec::new();
    walk(&root.join("src"), &mut out);
    walk(&root.join("tests"), &mut out);
    out.retain(|path| {
        path.file_name()
            .is_some_and(|name| name != "windows_target_gate.rs")
    });
    out.sort();
    out
}

/// Lines of `source` that branch on `target_env`, as `(line number, text)`.
///
/// Comment lines are skipped so that an explanation of why `target_env` is
/// avoided does not read as a use of it. A `cfg` and its `target_env` must be on
/// the same line, which is what rustfmt produces for an attribute this short;
/// the assertion below is a tripwire for a deliberate addition, not a defence
/// against someone hiding one.
fn target_env_branches(source: &str) -> Vec<(usize, String)> {
    source
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            let trimmed = line.trim_start();
            !trimmed.starts_with("//") && !trimmed.starts_with('*')
        })
        .filter(|(_, line)| line.contains("cfg") && line.contains("target_env"))
        .map(|(index, line)| (index + 1, line.trim().to_owned()))
        .collect()
}

/// The condition that makes `-gnu` a faithful stand-in for the shipped `-msvc`.
///
/// `x86_64-pc-windows-gnu` and `x86_64-pc-windows-msvc` agree on `target_os`,
/// `target_family` and `windows`, and differ only under `target_env`. CI checks
/// the gnu triple because cross-checking msvc from a Linux runner is not
/// available — `ring` and `zstd-sys` compile C for the target and their build
/// scripts refuse an msvc target under GNU cc. So the proxy is exact precisely
/// as long as nothing in the crate branches on `target_env`; the first such
/// branch would ship a path CI has never compiled, and this is what says so.
#[test]
fn no_target_env_cfg_leaves_the_gnu_proxy_faithful() {
    let mut found = Vec::new();
    for path in crate_sources() {
        let Ok(source) = std::fs::read_to_string(&path) else {
            continue;
        };
        for (line, text) in target_env_branches(&source) {
            found.push(format!("{}:{line}: {text}", path.display()));
        }
    }
    assert!(
        found.is_empty(),
        "the crate now branches on `target_env`, which the windows cross-check in \
         ci.yml cannot follow: it checks `x86_64-pc-windows-gnu`, so an msvc-only \
         branch would ship uncompiled by any gate. Either drop the branch, or change \
         the CI step to a target that reaches it and update this test.\n{}",
        found.join("\n")
    );
}

/// Negative control for [`target_env_branches`]: the scan above runs over
/// sources that are clean today, so it reports the same thing when it works and
/// when it has stopped matching anything.
#[test]
fn the_target_env_scanner_matches_its_contract() {
    let planted = "#[cfg(all(windows, target_env = \"msvc\"))]\nfn f() {}\n";
    assert_eq!(
        target_env_branches(planted).len(),
        1,
        "the scanner no longer finds a planted `target_env` branch, so its clean \
         report over the real sources means nothing"
    );

    let commented = "// #[cfg(target_env = \"msvc\")] — deliberately avoided\n";
    assert!(
        target_env_branches(commented).is_empty(),
        "the scanner counts a commented-out branch, so documenting why `target_env` \
         is avoided would fail the gate"
    );

    let unrelated = "let target_env = std::env::var(\"TARGET\");\n";
    assert!(
        target_env_branches(unrelated).is_empty(),
        "the scanner matches a local binding that merely shares the name"
    );
}

/// The cross-check is only worth its minute if there is platform-gated code for
/// it to reach. If this ever fails, the honest response is to delete the CI step
/// and this file together — not to keep a gate over an empty set.
#[test]
fn platform_gated_code_still_exists_for_the_cross_check_to_reach() {
    let gated: Vec<String> = crate_sources()
        .into_iter()
        .filter(|path| path.starts_with(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src")))
        .filter_map(|path| {
            let source = std::fs::read_to_string(&path).ok()?;
            let count = source
                .lines()
                .filter(|line| {
                    let trimmed = line.trim();
                    trimmed.starts_with("#[cfg")
                        && (trimmed.contains("windows") || trimmed.contains("\"macos\""))
                        && !trimmed.contains("not(windows)")
                })
                .count();
            (count > 0).then(|| format!("{}: {count}", path.display()))
        })
        .collect();

    assert!(
        !gated.is_empty(),
        "no `#[cfg(windows)]` or macOS-gated item is left in cli/src, so the windows \
         clippy step in ci.yml now checks the same code as the host step"
    );
}
