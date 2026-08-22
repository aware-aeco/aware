//! Negative control for the Python-test gate in `.github/workflows/ci.yml`.
//!
//! The repo's agents are not all Rust and .NET.
//! `20-agents/aeco/visualization/blender/tests/` carries seven `test_*.py`
//! regression tests, and NOTHING ran them: `cargo test` never leaves `cli/`,
//! `release.yml` only builds, and no workflow mentioned Python except three
//! single-purpose scripts of its own. Their green-ness rested entirely on
//! whoever last remembered to run them by hand.
//!
//! That is the fourth instance of one hole, each of the previous three found
//! only after it had shipped a real defect:
//!
//!   * `cli-connection-reader` (#343) — `read-model` returned a different
//!     coordinate frame from `probe`, and the bridge's own tests encoded the
//!     wrong frame because nothing re-read them;
//!   * `steel-detailer-lookup` — six `unwrap()` calls against CLAUDE.md §Code
//!     style and edition 2021 against the 2024 pin, with every check green;
//!   * six of seven .NET suites (`dotnet_suites_gate.rs`) — 529 tests run by
//!     nothing, one of whose files the v0.125.0 scene-contract change edited and
//!     shipped without executing.
//!
//! CLAUDE.md §Engineering rules — "Verify before answering", "No corner-cutting"
//! — is what a suite nobody runs defeats: a green PR check is the claim that the
//! tests passed, and for these seven files that claim was never evaluated.
//!
//! ## The division of labour, and why this file is the half that must be here
//!
//! `scripts/run-agent-python-tests.py` discovers the tests and runs them, and it
//! carries its own negative control (`--self-test`): a synthetic tree with a
//! known-passing, known-skipping, known-failing and known-crashing test in it,
//! asserting each is classified correctly and that one failure fails the run.
//! Because discovery walks the tree rather than naming known suites, a Python
//! test added under a *different* agent tomorrow is picked up with no edit —
//! which is what the enumerate-the-suites approach could not do, and is how
//! `cli-sidecar/Ingest/Generator/Tests` came to sit unrun.
//!
//! What that script cannot prove is that anything still *invokes* it. Delete the
//! CI step and every assertion inside the script stays green while nothing runs.
//! So this file asserts the one fact living outside it: `ci.yml` still carries a
//! blocking, unconditional step running BOTH halves — the self-test and the real
//! run. Same split, and the same reason, as `lockfile_gate.rs`.
//!
//! ## Why the accepted shape is narrow
//!
//! `ci.yml` is roughly half prose — every gate in it carries a paragraph
//! explaining why it exists — and a `run: |` block is one opaque string to the
//! YAML parser, so a *shell* comment inside it survives parsing intact. A step
//! reading `# python3 scripts/run-agent-python-tests.py` would satisfy a
//! substring search while running nothing at all, which is precisely the trap
//! `dotnet_suites_gate.rs` documents falling into twice. [`runs_python_suite`]
//! therefore drops whole-line shell comments and requires an exact line match,
//! in a step and a job that carry no `if:` and no `continue-on-error`. It does
//! not emulate a shell: chaining, quoting and variables all make a step
//! non-evidence, so an unmodelled construct can only make this gate fail closed.
//!
//! Because that reader scans an artefact that is correct today, it would report
//! success both when it works and when it has stopped matching anything at all.
//! [`reader_rejects_every_weakened_form`] is its negative control: it drives the
//! same function over synthetic workflows in which the step has been commented
//! out, made non-blocking, gated behind an `if:`, or stripped of one of its two
//! halves, and requires each to be rejected.

use std::path::PathBuf;

/// Repository root — `cli/`'s parent. The workflow under test lives there.
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap_or_else(|| panic!("{} has no parent", env!("CARGO_MANIFEST_DIR")))
        .to_path_buf()
}

/// The runner, repo-relative. Named once so the two assertions cannot drift.
const RUNNER: &str = "scripts/run-agent-python-tests.py";

/// `true` when `value` is a mapping key set to a truthy `continue-on-error`.
fn continues_on_error(value: &serde_yaml::Value) -> bool {
    value
        .get("continue-on-error")
        .and_then(serde_yaml::Value::as_bool)
        .unwrap_or(false)
}

/// The command lines of a `run:` scalar, with whole-line shell comments dropped.
///
/// Only whole-line comments, deliberately. Stripping a trailing `#` would mean
/// deciding whether it sits inside quotes, which is emulating a shell — and the
/// consequence of getting that wrong is a gate that accepts a step it should
/// reject. A trailing comment simply makes the line stop matching, which fails
/// closed.
fn command_lines(run: &str) -> Vec<String> {
    run.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_owned)
        .collect()
}

/// `true` when `workflow` runs the Python suite in a blocking, unconditional
/// step — both the self-test and the real run.
fn runs_python_suite(workflow: &str) -> bool {
    let Ok(doc) = serde_yaml::from_str::<serde_yaml::Value>(workflow) else {
        return false;
    };
    let Some(jobs) = doc.get("jobs").and_then(serde_yaml::Value::as_mapping) else {
        return false;
    };

    let self_test = format!("python3 {RUNNER} --self-test");
    let real_run = format!("python3 {RUNNER}");

    for (_, job) in jobs {
        // A job that can be switched off, or whose failure is tolerated, proves
        // nothing about the suite on a given commit.
        if job.get("if").is_some() || continues_on_error(job) {
            continue;
        }
        let Some(steps) = job.get("steps").and_then(serde_yaml::Value::as_sequence) else {
            continue;
        };
        for step in steps {
            if step.get("if").is_some() || continues_on_error(step) {
                continue;
            }
            let Some(run) = step.get("run").and_then(serde_yaml::Value::as_str) else {
                continue;
            };
            let lines = command_lines(run);
            // Both halves in ONE step: a self-test in a step that no longer
            // runs the suite (or the reverse) is half a gate, and the halves
            // are worth nothing apart.
            if lines.contains(&self_test) && lines.contains(&real_run) {
                return true;
            }
        }
    }
    false
}

fn ci_workflow() -> String {
    let path = repo_root().join(".github/workflows/ci.yml");
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn the_runner_script_exists() {
    let path = repo_root().join(RUNNER);
    assert!(
        path.is_file(),
        "{} is missing — the CI step below invokes it, so its absence turns that \
         step into a red gate rather than a silent one, but nothing else in the \
         suite would say why",
        path.display()
    );
}

#[test]
fn ci_runs_the_python_suite_in_a_blocking_step() {
    assert!(
        runs_python_suite(&ci_workflow()),
        "`.github/workflows/ci.yml` no longer carries a blocking, unconditional \
         step running BOTH `python3 {RUNNER} --self-test` and `python3 {RUNNER}`. \
         The repo's seven `test_*.py` regression tests are run by nothing else — \
         not `cargo test`, which never leaves `cli/`, and not `release.yml`, \
         which only builds. Without that step they are back to running only when \
         somebody remembers, which is the state in which the four suites named \
         at the top of this file each shipped a defect."
    );
}

/// The negative control for [`runs_python_suite`].
///
/// Each case is a way the step could be present in the file while proving
/// nothing, and every one of them was reachable: the comment case is the trap
/// `dotnet_suites_gate.rs` fell into twice, and the half-a-step cases are what a
/// well-meant "the self-test is enough" edit produces.
#[test]
fn reader_rejects_every_weakened_form() {
    let good = r#"
jobs:
  agent-python-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
      - name: run every Python test in the repo
        run: |
          python3 scripts/run-agent-python-tests.py --self-test
          python3 scripts/run-agent-python-tests.py
"#;
    assert!(
        runs_python_suite(good),
        "the reader rejects the canonical form, so every rejection below proves \
         nothing — it would reject anything at all"
    );

    let cases: [(&str, &str); 7] = [
        (
            "commented out inside the run block",
            r#"
jobs:
  agent-python-tests:
    runs-on: ubuntu-latest
    steps:
      - run: |
          # python3 scripts/run-agent-python-tests.py --self-test
          # python3 scripts/run-agent-python-tests.py
          echo "we'll wire this up later"
"#,
        ),
        (
            "merely echoed, never run",
            r#"
jobs:
  agent-python-tests:
    runs-on: ubuntu-latest
    steps:
      - run: echo "run python3 scripts/run-agent-python-tests.py by hand"
"#,
        ),
        (
            "step tolerates its own failure",
            r#"
jobs:
  agent-python-tests:
    runs-on: ubuntu-latest
    steps:
      - continue-on-error: true
        run: |
          python3 scripts/run-agent-python-tests.py --self-test
          python3 scripts/run-agent-python-tests.py
"#,
        ),
        (
            "job tolerates its own failure",
            r#"
jobs:
  agent-python-tests:
    runs-on: ubuntu-latest
    continue-on-error: true
    steps:
      - run: |
          python3 scripts/run-agent-python-tests.py --self-test
          python3 scripts/run-agent-python-tests.py
"#,
        ),
        (
            "step gated behind a condition",
            r#"
jobs:
  agent-python-tests:
    runs-on: ubuntu-latest
    steps:
      - if: github.event_name == 'schedule'
        run: |
          python3 scripts/run-agent-python-tests.py --self-test
          python3 scripts/run-agent-python-tests.py
"#,
        ),
        (
            "self-test only — the runner is never pointed at the repo",
            r#"
jobs:
  agent-python-tests:
    runs-on: ubuntu-latest
    steps:
      - run: python3 scripts/run-agent-python-tests.py --self-test
"#,
        ),
        (
            "real run only — a runner that stopped detecting failures goes unnoticed",
            r#"
jobs:
  agent-python-tests:
    runs-on: ubuntu-latest
    steps:
      - run: python3 scripts/run-agent-python-tests.py
"#,
        ),
    ];

    for (label, workflow) in cases {
        assert!(
            !runs_python_suite(workflow),
            "the reader accepted a workflow that runs nothing: {label}"
        );
    }
}

/// `command_lines` must keep real commands and drop only whole-line comments.
///
/// Without this the case above could pass for the wrong reason — a reader that
/// dropped every line would reject all seven weakened forms and the canonical
/// one too, which the canonical assertion catches, but a reader that dropped
/// slightly too much would still look right.
#[test]
fn command_lines_drops_comments_and_keeps_commands() {
    let lines = command_lines(
        "  # a leading comment\n\npython3 a.py\n   # indented comment\n  python3 b.py  \n",
    );
    assert_eq!(lines, vec!["python3 a.py", "python3 b.py"]);
}
