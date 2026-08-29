//! Negative control for the platform-gated clippy gates in
//! `.github/workflows/ci.yml` — the windows cross-check and the macOS job.
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
//! A windows target closes the windows half and no more: it compiles no
//! `#[cfg(target_os = "macos")]` arm either. The other half is `ci.yml`'s
//! `gates-macos` job, which has to be a real `macos-latest` runner — a darwin
//! cross-check from Linux dies in `zstd-sys`'s build script (`cc: error:
//! unrecognized command-line option '-arch'`) before it reaches this crate at
//! all, so unlike `-msvc` there is no proxy triple to stand in. Both gates are
//! asserted below, separately, because neither is coverage for the other.
//!
//! What is asserted here:
//!   * `ci.yml` still runs clippy for a windows target, with `--all-targets`,
//!     `--locked` and `-D warnings` — each of which the step is useless without;
//!   * the triple it checks is the triple the toolchain step installs, so the
//!     two cannot drift into a step that fails for want of a target rather than
//!     passing for having checked one;
//!   * a job on a macOS runner still lints the crate with the same three flags
//!     and no `--target`, so the macOS cfg set is the one being compiled;
//!   * no `target_env` cfg exists in `cli/`, which is the condition that makes
//!     the `-gnu` triple a faithful proxy for the shipped `-msvc` one;
//!   * windows-gated AND macOS-gated code both still exist, so neither gate is
//!     guarding an empty set — and the macOS-gated lines are not counted as
//!     justification for the windows step.
//!
//! Every one of those scans an artefact that is correct today — the real
//! `ci.yml`, the real `src/` — so each would report clean both when it works and
//! when it has stopped matching anything. Each classifier is therefore driven
//! over synthetic input as well:
//! `the_cross_check_reader_matches_its_contract`,
//! `the_job_reader_matches_its_contract`,
//! `the_target_env_scanner_matches_its_contract` and
//! `the_platform_gate_classifier_matches_its_contract`.
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

/// One workflow job: its YAML key, the runner it asks for, and its body with
/// comment lines removed.
///
/// Comments are dropped for the same reason [`Step`] drops them, and it is not
/// hypothetical here: the job under test is introduced by a comment block that
/// names every flag asserted over it, so a `contains` over the raw text stays
/// green after the flag has been deleted from the command. Measured — removing
/// `-D warnings` from the macOS step left this file passing until this line.
struct Job {
    name: String,
    runs_on: String,
    body: String,
}

/// Split a workflow's jobs, in order.
///
/// Scanned line by line rather than by splitting on the two-space indent: a job
/// body is itself indented, so a `split("\n  ")` cuts each job off at its first
/// nested line and every flag check over the body is then vacuous. A job starts
/// at a bare `  <key>:` inside the `jobs:` mapping and runs to the next one.
/// Confining it to that mapping matters — `on:` has `  pull_request:` under it,
/// which is the same shape as a job key and is not a job.
fn jobs(workflow: &str) -> Vec<Job> {
    fn job_key(line: &str) -> Option<&str> {
        let key = line.strip_prefix("  ")?.strip_suffix(':')?;
        (!key.is_empty()
            && key
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'))
        .then_some(key)
    }

    let mut out: Vec<Job> = Vec::new();
    let mut inside = false;
    for line in workflow.lines() {
        if !line.starts_with([' ', '#']) && !line.trim().is_empty() {
            // A top-level key: `jobs:` opens the mapping, anything else closes it.
            inside = line.trim_end() == "jobs:";
            continue;
        }
        if !inside {
            continue;
        }
        match job_key(line) {
            Some(key) => out.push(Job {
                name: key.to_owned(),
                runs_on: String::new(),
                body: String::new(),
            }),
            None => {
                let Some(job) = out.last_mut() else {
                    continue; // a comment above the first job key
                };
                if line.trim_start().starts_with('#') {
                    continue;
                }
                if let Some(runner) = line.trim().strip_prefix("runs-on:")
                    && job.runs_on.is_empty()
                {
                    job.runs_on = runner.trim().to_owned();
                }
                job.body.push_str(line);
                job.body.push('\n');
            }
        }
    }
    out
}

/// The job whose key is `name`, or a failure naming what was searched.
fn job<'a>(jobs: &'a [Job], name: &str) -> &'a Job {
    jobs.iter().find(|job| job.name == name).unwrap_or_else(|| {
        panic!(
            "ci.yml has no job named {name:?} — it was renamed or removed, and this \
                 test can no longer tell whether the platform-gated code is still linted. \
                 Jobs present: {:?}",
            jobs.iter().map(|j| &j.name).collect::<Vec<_>>()
        )
    })
}

/// The shell text of every `run:` in `body`, and nothing else.
///
/// Not the step's `name:`, which is the second way prose passes for a command
/// here and, unlike a comment, survives comment-stripping: the macOS step is
/// *named* `cargo clippy -D warnings (macos)`, so a check over the whole body is
/// satisfied by the label after the flag has been deleted from the command.
/// Measured — both that and the comment block above it had to go before removing
/// `-D warnings` from the real step failed this file.
fn run_commands(body: &str) -> String {
    let indent_of = |line: &str| line.len() - line.trim_start().len();
    let mut out = String::new();
    let mut block: Option<usize> = None; // indent of an open `run: |`
    for line in body.lines() {
        if let Some(opened_at) = block {
            if line.trim().is_empty() || indent_of(line) > opened_at {
                out.push_str(line.trim());
                out.push('\n');
                continue;
            }
            block = None;
        }
        let Some(rest) = line.trim_start().strip_prefix("run:") else {
            continue;
        };
        match rest.trim() {
            "|" | ">" | "|-" | ">-" => block = Some(indent_of(line)),
            command => {
                out.push_str(command);
                out.push('\n');
            }
        }
    }
    out
}

/// What a clippy gate step is missing to be a gate at all, ignoring `--target`.
///
/// Shared by the windows cross-check and the macOS job, which need exactly the
/// same three flags for exactly the same three reasons and differ only in which
/// cfg set they compile.
fn missing_gate_flag(raw_body: &str) -> Option<&'static str> {
    let body = &run_commands(raw_body);
    if !body.contains("cargo clippy") {
        return Some("cargo clippy");
    }
    // Without `--all-targets` the integration tests under `cli/tests/` are not
    // checked — `tests/app_id_is_a_segment.rs` carries a `#[cfg(windows)]` case.
    if !body.contains("--all-targets") {
        return Some("--all-targets");
    }
    // Without `-D warnings` clippy prints and exits 0, so the step reports green
    // on exactly the code it was added to reject.
    if !body.contains("-D warnings") {
        return Some("-D warnings");
    }
    // Same reason every other cargo call in this job carries it: without
    // `--locked` the step resolves around a stale lockfile instead of failing.
    if !body.contains("--locked") {
        return Some("--locked");
    }
    None
}

/// The triple a `run:` body passes to `--target`, if any.
///
/// Read positionally rather than by substring, because `--all-targets` contains
/// `-target` and a `contains("--target")` therefore reports a cross-check on a
/// step that only ever compiles for the host. Over the commands only, for the
/// reason [`run_commands`] gives.
fn explicit_target(raw_body: &str) -> Option<String> {
    run_commands(raw_body)
        .split_whitespace()
        .skip_while(|token| *token != "--target")
        .nth(1)
        .map(str::to_owned)
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
fn cross_check(body: &str) -> CrossCheck {
    if let Some(missing) = missing_gate_flag(body) {
        return CrossCheck::Missing(missing);
    }
    match explicit_target(body).filter(|triple| triple.contains("-pc-windows-")) {
        Some(triple) => CrossCheck::Checks(triple),
        None => CrossCheck::Missing("--target <a *-pc-windows-* triple>"),
    }
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
    let host_clippy = &step(&steps, "cargo clippy -D warnings").body;
    assert_eq!(
        explicit_target(host_clippy),
        None,
        "ci.yml's host clippy step now passes an explicit target, so it no longer checks \
         the crate for the platform CI actually runs on"
    );
    assert!(
        matches!(cross_check(host_clippy), CrossCheck::Missing(_)),
        "ci.yml's host clippy step now passes a windows `--target`, so nothing checks \
         the crate for the platform CI actually runs on"
    );

    // The triple checked must be one rustup was told to install. Drift here does
    // not fail loudly in an obvious way — it fails with "target may not be
    // installed", which reads like an infrastructure hiccup rather than a gate.
    //
    // Read from the `gates` job rather than by step name: `gates-macos` pins the
    // same toolchain and so carries a step of the same name, and a by-name lookup
    // would silently start reading whichever came first in the file.
    let installed = installed_targets(&job(&jobs(&source), "gates").body);
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

/// The other half: the macOS cfg set is linted, and on a machine that has it.
///
/// The windows cross-check cannot cover this. A windows target compiles no
/// `#[cfg(target_os = "macos")]` arm, and a darwin target cannot be checked from
/// the ubuntu runner at all: `cargo clippy --target aarch64-apple-darwin` dies in
/// `zstd-sys`'s build script with `cc: error: unrecognized command-line option
/// '-arch'` — measured — because the runner's `cc` is not an Apple
/// cross-compiler. Unlike `-msvc`, whose stand-in is `-gnu`, darwin has no proxy
/// triple, because what is missing is a toolchain and not a `target_env`. So the
/// gate is a real macOS runner or it is nothing.
#[test]
fn ci_still_lints_the_macos_cfg_set_on_a_macos_runner() {
    let workflow = repo_root().join(".github/workflows/ci.yml");
    let source = std::fs::read_to_string(&workflow)
        .unwrap_or_else(|e| panic!("read {}: {e}", workflow.display()));
    let jobs = jobs(&source);

    // A floor, as above: a splitter that recovered one job would pass an
    // emptiness test having dropped the rest.
    assert!(
        jobs.len() > 3,
        "parsed only {} jobs out of ci.yml — the splitter is broken, so nothing \
         below means anything",
        jobs.len()
    );

    let macos = job(&jobs, "gates-macos");
    assert!(
        macos.runs_on.contains("macos"),
        "ci.yml's `gates-macos` job now runs on {:?}, so the macOS cfg set is not \
         the one being compiled and the job lints the same code the `gates` job \
         already did",
        macos.runs_on
    );

    if let Some(missing) = missing_gate_flag(&macos.body) {
        panic!(
            "ci.yml's macOS job no longer passes `{missing}`, so `app.rs`'s \
             `#[cfg(target_os = \"macos\")]` arm and `model_reader_host.rs`'s \
             `#[cfg(all(unix, not(target_os = \"linux\")))]` arm are back to being \
             checked by nothing — `release.yml` builds them on macOS, but with \
             `cargo build`, which accepts a `clippy::` lint and ignores it"
        );
    }

    // No `--target`: the whole point is that the HOST cfg set is the macOS one.
    // A `--target` here would compile some other platform on an expensive runner
    // and leave macOS exactly as uncovered as before.
    assert_eq!(
        explicit_target(&macos.body),
        None,
        "ci.yml's macOS job now cross-compiles to another target, so nothing \
         checks the crate for the platform that job exists to reach"
    );
}

/// Negative control for [`jobs`]: it must read keys and runners, not prose.
#[test]
fn the_job_reader_matches_its_contract() {
    let workflow = "name: CI\n\njobs:\n  gates:\n    runs-on: ubuntu-latest\n    steps:\n      \
                    - uses: actions/checkout@v6\n      - name: x\n        run: y\n  \
                    gates-macos:\n    runs-on: macos-latest\n    steps:\n      - run: cargo clippy\n";
    let parsed = jobs(workflow);
    assert_eq!(
        parsed.iter().map(|j| j.name.as_str()).collect::<Vec<_>>(),
        ["gates", "gates-macos"],
        "the job splitter no longer recovers the jobs of an ordinary workflow, so \
         its report over the real ci.yml means nothing"
    );
    assert_eq!(
        parsed
            .iter()
            .map(|j| j.runs_on.as_str())
            .collect::<Vec<_>>(),
        ["ubuntu-latest", "macos-latest"],
        "the job splitter reads the wrong `runs-on`, so it cannot tell a macOS \
         runner from any other"
    );
    assert!(
        parsed[0].body.contains("actions/checkout"),
        "a job body must carry its steps, or the flag checks over it are vacuous"
    );

    // The two failures this reader was measured to have. The real macOS job is
    // introduced by a comment block naming every flag asserted over it, AND its
    // step is *named* `cargo clippy -D warnings (macos)` — so a check over the
    // whole body is satisfied twice over by prose after the flag has been
    // deleted from the command. Both had to go before the deletion failed here.
    let prose = "jobs:\n  gates-macos:\n    runs-on: macos-latest\n    steps:\n      \
                 # cargo clippy --all-targets --locked -- -D warnings\n      \
                 - name: cargo clippy -D warnings (macos)\n        \
                 run: cargo clippy --all-targets --locked\n";
    assert_eq!(
        missing_gate_flag(&jobs(prose)[0].body),
        Some("-D warnings"),
        "a job's comments or its step names are being read as its commands, so \
         every flag assertion over a job body passes on the prose describing it"
    );

    // And the command itself must still be read, or the check above is satisfied
    // by finding nothing at all.
    let real = "jobs:\n  gates-macos:\n    runs-on: macos-latest\n    steps:\n      \
                - name: whatever\n        \
                run: cargo clippy --all-targets --locked -- -D warnings\n";
    assert_eq!(
        missing_gate_flag(&jobs(real)[0].body),
        None,
        "the reader rejects a complete gate command, so it can only report failure"
    );

    // A `run: |` block is the shape the windows step uses; its lines are the
    // command too.
    let block = "jobs:\n  gates:\n    runs-on: ubuntu-latest\n    steps:\n      \
                 - name: x\n        run: |\n          sudo apt-get install -y mingw\n          \
                 cargo clippy --target x86_64-pc-windows-gnu --all-targets --locked -- -D warnings\n";
    assert_eq!(
        explicit_target(&jobs(block)[0].body).as_deref(),
        Some("x86_64-pc-windows-gnu"),
        "a multi-line `run:` block is not being read, so the step that uses one is \
         checked against an empty command"
    );
    // A job's own indented content must not be mistaken for another job — the
    // lookup would then find a "job" with no runner and report it as one.
    assert!(
        !parsed.iter().any(|j| j.runs_on.is_empty()),
        "the splitter emitted a chunk that is not a job: {:?}",
        parsed.iter().map(|j| j.name.as_str()).collect::<Vec<_>>()
    );
}

/// Every `.rs` file in the `cli` package — `src/`, `tests/`, and the Rust
/// targets that sit at the crate root — excluding this one.
///
/// This file is excluded because its own prose and its negative-control fixtures
/// contain the very string the scan looks for; including it would make the scan
/// fail on itself and prove nothing about the crate.
///
/// `build.rs` is in scope and is not an afterthought. A build script is compiled
/// for the HOST, never for `--target`, so a `#[cfg(target_env = "msvc")]` in it
/// is invisible to both ubuntu clippy runs — host and windows cross-check alike —
/// while a Windows release build, whose host IS msvc, compiles the unlinted arm.
/// Walking only `src/` and `tests/` therefore left the very invariant this file
/// asserts breakable in the one file most likely to break it (Codex review, #469).
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
    // `benches/` and `examples/` are absent today; walking them costs nothing
    // and means adding one does not silently fall outside the scan.
    walk(&root.join("benches"), &mut out);
    walk(&root.join("examples"), &mut out);
    // Crate-root targets — `build.rs` above all. Not recursive: the crate root
    // also holds `target/`, which is build output and not this package's source.
    if let Ok(entries) = std::fs::read_dir(&root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "rs") {
                out.push(path);
            }
        }
    }
    out.retain(|path| {
        path.file_name()
            .is_some_and(|name| name != "windows_target_gate.rs")
    });
    out.sort();
    out
}

/// `line` with any `//` comment removed, ignoring one inside a string literal.
///
/// Quote-aware so that a `//` in a string — a URL, a path — does not truncate
/// real code. Cuts only at an ASCII `/`, so the slice is always on a boundary.
fn strip_comment(line: &str) -> &str {
    let bytes = line.as_bytes();
    let mut in_string = false;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' if in_string => i += 1,
            b'"' => in_string = !in_string,
            b'/' if !in_string && bytes.get(i + 1) == Some(&b'/') => return &line[..i],
            _ => {}
        }
        i += 1;
    }
    line
}

/// How far `text` opens or closes parentheses, ignoring those in string literals.
fn paren_delta(text: &str) -> isize {
    let bytes = text.as_bytes();
    let mut depth = 0;
    let mut in_string = false;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' if in_string => i += 1,
            b'"' => in_string = !in_string,
            b'(' if !in_string => depth += 1,
            b')' if !in_string => depth -= 1,
            _ => {}
        }
        i += 1;
    }
    depth
}

/// An attribute is abandoned after this many lines. A stray unbalanced paren
/// would otherwise swallow the rest of the file into one blob.
const MAX_CFG_LINES: usize = 32;

/// Complete `cfg` predicates in `source`, as `(first line number, text)`.
///
/// Accumulated across lines rather than matched per line. rustfmt wraps a long
/// predicate — an `all(...)` with three terms goes onto four lines — and then no
/// single line holds both `cfg` and the term being looked for, so a per-line
/// search reports clean on exactly the branch it exists to catch.
///
/// Comments are stripped first, so an explanation of why a cfg is avoided does
/// not read as a use of it. Lines whose first character is `*` are dropped as
/// block-comment bodies.
fn cfg_attributes(source: &str) -> Vec<(usize, String)> {
    let mut found = Vec::new();
    let mut pending: Option<(usize, String, isize, usize)> = None;
    for (index, raw) in source.lines().enumerate() {
        let trimmed = strip_comment(raw).trim();
        if trimmed.starts_with('*') {
            continue;
        }
        let (start, mut text, mut depth, lines) = match pending.take() {
            Some(open) => open,
            // An opening line has to name `cfg` and open a paren; `cfg` alone is
            // `#[cfg_attr]`'s prefix, a module name, half of `cfg!`.
            None if trimmed.contains("cfg") && trimmed.contains('(') => {
                (index + 1, String::new(), 0, 0)
            }
            None => continue,
        };
        if !text.is_empty() {
            text.push(' ');
        }
        text.push_str(trimmed);
        depth += paren_delta(trimmed);
        if depth > 0 && lines + 1 < MAX_CFG_LINES {
            pending = Some((start, text, depth, lines + 1));
        } else {
            found.push((start, text));
        }
    }
    // An attribute left open at end of file is still worth reporting.
    if let Some((start, text, _, _)) = pending {
        found.push((start, text));
    }
    found
}

/// Complete `cfg` predicates that branch on `target_env`, as `(line, text)`.
fn target_env_branches(source: &str) -> Vec<(usize, String)> {
    cfg_attributes(source)
        .into_iter()
        .filter(|(_, text)| text.contains("target_env"))
        .collect()
}

/// Windows-host-specific logic in a build script, as `(line, text)` pairs.
///
/// A build script is compiled for the HOST, not Cargo's `--target`. The Ubuntu
/// host clippy run and the Windows-GNU cross-check therefore both compile
/// `build.rs` as Linux, while a release build on `windows-latest` compiles its
/// Windows-only branches. Keep the inventory deliberately conservative: any
/// Windows cfg or Cargo-provided target/host query requires either a Windows
/// clippy job or an explicit update to this gate.
fn windows_host_branches(source: &str) -> Vec<(usize, String)> {
    let mut found = cfg_attributes(source)
        .into_iter()
        .filter(|(_, text)| {
            (text.starts_with("#[cfg") || text.starts_with("#![cfg") || text.contains("cfg!("))
                && text.contains("windows")
        })
        .collect::<Vec<_>>();

    // Strip comments and whitespace so ordinary multi-line env reads cannot
    // evade the check. The markers are calls/macros, not bare words, to avoid
    // rejecting a diagnostic that merely mentions TARGET or HOST.
    let compact = source
        .lines()
        .map(strip_comment)
        .flat_map(|line| line.chars().filter(|character| !character.is_whitespace()))
        .collect::<String>();
    const CARGO_HOST_TARGET_MARKERS: &[&str] = &[
        "var(\"TARGET\")",
        "var_os(\"TARGET\")",
        "env!(\"TARGET\")",
        "option_env!(\"TARGET\")",
        "var(\"HOST\")",
        "var_os(\"HOST\")",
        "env!(\"HOST\")",
        "option_env!(\"HOST\")",
        "var(\"CARGO_CFG_WINDOWS\")",
        "var_os(\"CARGO_CFG_WINDOWS\")",
        "env!(\"CARGO_CFG_WINDOWS\")",
        "option_env!(\"CARGO_CFG_WINDOWS\")",
        "var(\"CARGO_CFG_TARGET_OS\")",
        "var_os(\"CARGO_CFG_TARGET_OS\")",
        "env!(\"CARGO_CFG_TARGET_OS\")",
        "option_env!(\"CARGO_CFG_TARGET_OS\")",
        "var(\"CARGO_CFG_TARGET_ENV\")",
        "var_os(\"CARGO_CFG_TARGET_ENV\")",
        "env!(\"CARGO_CFG_TARGET_ENV\")",
        "option_env!(\"CARGO_CFG_TARGET_ENV\")",
    ];
    for marker in CARGO_HOST_TARGET_MARKERS {
        if compact.contains(marker) {
            let line = source
                .lines()
                .enumerate()
                .find(|(_, raw)| strip_comment(raw).contains(marker))
                .map(|(index, _)| index + 1)
                .unwrap_or(1);
            found.push((line, (*marker).to_owned()));
        }
    }
    found.sort_by_key(|(line, _)| *line);
    found
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
    let scanned = crate_sources();

    // The scan is over sources that are clean today, so its inventory needs a
    // tripwire of its own: a walk that quietly stopped reaching a file would
    // report exactly the same clean result. `build.rs` is named because it is
    // the file the walk used to miss, and the one where a `target_env` branch
    // hides best — compiled for the host, so no `--target` run ever lints it.
    assert!(
        scanned
            .iter()
            .any(|path| path.file_name().is_some_and(|name| name == "build.rs")),
        "cli/build.rs is outside the target_env scan, so an msvc branch in the \
         build script would ship uncompiled by either clippy run while this test \
         reported the gnu proxy faithful. Scanned {} files",
        scanned.len()
    );

    let mut found = Vec::new();
    for path in scanned {
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

/// Build scripts run for the host, so a Windows-only build-script branch is
/// invisible to both Ubuntu clippy invocations. Keep that gap explicit until
/// CI gains a Windows-host clippy job that compiles `build.rs` as Windows too.
#[test]
fn build_rs_has_no_unchecked_windows_host_logic() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("build.rs");
    let source =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let found = windows_host_branches(&source);
    assert!(
        found.is_empty(),
        "cli/build.rs contains Windows-host-specific logic that neither Ubuntu clippy gate \
         compiles: {found:?}. Add a Windows-host clippy job or update this test with the \
         corresponding coverage before adding the branch."
    );
}

/// Negative control for [`windows_host_branches`]: it must catch cfg, cfg!,
/// and Cargo target/host environment queries, including wrapped calls.
#[test]
fn the_build_rs_windows_scanner_matches_its_contract() {
    for (source, expected) in [
        ("#[cfg(windows)]\nfn main() {}\n", true),
        ("fn main() { if cfg!(target_os = \"windows\") {} }\n", true),
        (
            "fn main() { let target = std::env::var(\n    \"TARGET\"\n); }\n",
            true,
        ),
        ("fn main() { let host = option_env!(\"HOST\"); }\n", true),
        (
            "// cfg!(windows) and env!(\"TARGET\") are documentation\nfn main() {}\n",
            false,
        ),
        ("fn main() { let value = \"TARGET\"; }\n", false),
    ] {
        assert_eq!(
            !windows_host_branches(source).is_empty(),
            expected,
            "the build.rs scanner misclassified: {source}"
        );
    }
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

    // The shape that matters, and the one a per-line scan misses: rustfmt wraps
    // a predicate this long, and then no line holds both `cfg` and `target_env`.
    let wrapped = "#[cfg(all(\n    windows,\n    target_env = \"msvc\",\n    \
                   feature = \"host\"\n))]\nfn f() {}\n";
    assert_eq!(
        target_env_branches(wrapped),
        vec![(
            1,
            "#[cfg(all( windows, target_env = \"msvc\", feature = \"host\" ))]".to_owned()
        )],
        "the scanner misses an msvc branch that rustfmt wrapped, which is the \
         ordinary way one would arrive — so its clean report over the real \
         sources would survive exactly the addition it exists to catch"
    );

    let wrapped_comment = "// #[cfg(all(\n//     windows,\n//     target_env = \"msvc\"\n// ))]\n";
    assert!(
        target_env_branches(wrapped_comment).is_empty(),
        "the scanner counts a commented-out wrapped branch, so documenting why \
         `target_env` is avoided would fail the gate"
    );

    // A `//` inside a string is not a comment; truncating there would drop the
    // rest of a real line and could hide the term.
    let with_url = "#[cfg(target_env = \"msvc\")] // see https://example.invalid/x\n";
    assert_eq!(
        target_env_branches(with_url).len(),
        1,
        "the scanner mis-handles a line carrying both a string and a comment"
    );

    // A stray unbalanced paren must not swallow the file into one blob and
    // report every later line as part of one attribute.
    let unbalanced = format!("#[cfg(all(windows\n{}", "let x = 1;\n".repeat(64));
    assert_eq!(
        target_env_branches(&unbalanced).len(),
        0,
        "an unbalanced cfg predicate ran away and consumed unrelated code"
    );
    assert!(
        cfg_attributes(&unbalanced).len() == 1,
        "an unbalanced cfg predicate must be closed out and reported once, not \
         dropped and not repeated"
    );
}

/// Which gate, if any, a `cfg` attribute needs in order to be compiled at all.
///
/// Separate predicates on purpose. The two gates cost separate CI time and cover
/// disjoint code, so counting a macOS-gated arm as justification for the windows
/// step — as one combined predicate did — hides the case this pair exists to
/// keep visible: a platform whose only gate has been deleted.
///
/// `not(target_os = "linux")` under `unix` is the macOS arm in
/// `model_reader_host.rs`, and names no platform, so it is matched by shape. A
/// predicate written without rustfmt's spacing is counted as macOS-gated rather
/// than skipped; over-counting keeps a gate alive, under-counting retires one.
fn is_windows_gated(attr: &str) -> bool {
    attr.contains("windows") && !attr.contains("not(windows)")
}

fn is_macos_gated(attr: &str) -> bool {
    (attr.contains("\"macos\"") && !attr.contains("not(target_os = \"macos\")"))
        || attr.contains("not(target_os = \"linux\")")
}

/// `cli/src` attributes matching `gated`, as `path: count` lines.
fn gated_sources(gated: fn(&str) -> bool) -> Vec<String> {
    crate_sources()
        .into_iter()
        .filter(|path| path.starts_with(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src")))
        .filter_map(|path| {
            let source = std::fs::read_to_string(&path).ok()?;
            let count = cfg_attributes(&source)
                .iter()
                .filter(|(_, attr)| attr.starts_with("#[cfg") || attr.starts_with("#![cfg"))
                .filter(|(_, attr)| gated(attr))
                .count();
            (count > 0).then(|| format!("{}: {count}", path.display()))
        })
        .collect()
}

/// Each gate is only worth its minutes if there is code for it to reach. If one
/// of these fails, the honest response is to delete that CI step — not to keep a
/// gate over an empty set, and not to let the other platform's code excuse it.
#[test]
fn platform_gated_code_still_exists_for_each_gate_to_reach() {
    assert!(
        !gated_sources(is_windows_gated).is_empty(),
        "no `#[cfg(windows)]` item is left in cli/src, so the windows cross-check in \
         ci.yml now checks the same code as the host step"
    );
    assert!(
        !gated_sources(is_macos_gated).is_empty(),
        "no macOS-gated item is left in cli/src, so the `gates-macos` job in ci.yml \
         now spends a macOS runner checking the same code the host step already did"
    );
}

/// Negative control for the pair above: over sources that are correct today they
/// report the same thing when they work and when they match nothing.
#[test]
fn the_platform_gate_classifier_matches_its_contract() {
    for (attr, windows, macos, why) in [
        ("#[cfg(windows)]", true, false, "the plain windows gate"),
        (
            "#[cfg(target_os = \"macos\")]",
            false,
            true,
            "the plain macOS gate",
        ),
        (
            "#[cfg(all(unix, not(target_os = \"linux\")))]",
            false,
            true,
            "the macOS arm that names no platform",
        ),
        (
            "#[cfg(all(unix, not(target_os = \"macos\")))]",
            false,
            false,
            "the linux arm — the host step already compiles it",
        ),
        (
            "#[cfg(not(windows))]",
            false,
            false,
            "the non-windows arm — likewise",
        ),
        (
            "#[cfg(target_os = \"linux\")]",
            false,
            false,
            "the linux arm by name",
        ),
    ] {
        assert_eq!(
            (is_windows_gated(attr), is_macos_gated(attr)),
            (windows, macos),
            "the platform classifier misreads {why}: {attr}"
        );
    }
}
