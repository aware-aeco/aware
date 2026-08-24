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
//! Because discovery walks the tree, a Python test added under a *different*
//! agent tomorrow is picked up with no edit. It still keys on a filename
//! convention (`test_*.py`), which is the weak part and is why the script backs
//! it with two execution checks: `dotnet_suites_gate.rs` records a gate that
//! matched `*.Tests.csproj` and reported the repo clean while
//! `cli-sidecar/Ingest/Generator/Tests` — file name plain `Tests.csproj` — ran
//! nowhere. A filename convention is not a fact about the file.
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
//! therefore drops whole-line shell comments and requires the two commands to be
//! the step's ONLY content, in a step and a job that carry no `if:` and no
//! `continue-on-error`, in a workflow that still triggers on `pull_request`.
//!
//! Every one of those clauses is there because its absence was measured to let
//! a do-nothing workflow through (Codex review and review panel, PR #444): a
//! leading `exit 0`, an `if false; then … fi` wrapper, a `continue-on-error`
//! written as a string or a `${{ }}` expression, and `on:` reduced to
//! `workflow_dispatch`. An earlier draft of this paragraph claimed an unmodelled
//! construct "can only make this gate fail closed" — it did not, and the claim
//! was the kind of unearned safety assertion CLAUDE.md §Verify before answering
//! exists to stop. It holds now because the accepted shape is exact rather than
//! a substring or a set membership, which is also why this file does not
//! emulate a shell.
//!
//! Because that reader scans an artefact that is correct today, it would report
//! success both when it works and when it has stopped matching anything at all.
//! [`reader_rejects_every_weakened_form`] is its negative control: it drives the
//! same function over fifteen synthetic workflows in which the step has been
//! commented out, merely echoed, made non-blocking (as a bool, a string and an
//! expression, at step and job level), gated behind an `if:` (step and job),
//! stripped of one of its two halves, short-circuited by `exit 0`, softened with
//! `set +e`, wrapped in `if false`, or left perfect in a workflow that no longer
//! triggers on pull requests — and requires each to be rejected. Two positive
//! fixtures pin the other side, and every fixture is parsed before it is judged,
//! because an unparsable one would be rejected for the wrong reason and prove
//! nothing.

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

/// `true` unless `value` carries no `continue-on-error`, or one explicitly false.
///
/// Fail-closed on everything else, and the difference matters: GitHub accepts an
/// expression there, so `continue-on-error: ${{ true }}` reaches `serde_yaml` as
/// a *string*. Asking `as_bool()` for a verdict yields `None` for it, and a
/// reader that read `None` as "blocking" would call the step evidence while
/// GitHub tolerated its failure at runtime — the gate green, the suite optional.
/// Only a literal `false` (in either spelling) is a promise this reader can
/// check now; anything else is decided later, elsewhere, and is not accepted.
/// Same shape, for the same reason, as `dotnet_suites_gate.rs`.
fn continues_on_error(value: &serde_yaml::Value) -> bool {
    match value.get("continue-on-error") {
        None | Some(serde_yaml::Value::Bool(false)) => false,
        Some(serde_yaml::Value::String(value)) if value.eq_ignore_ascii_case("false") => false,
        Some(_) => true,
    }
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
    // A perfect job that never triggers runs nothing. `on:` reduced to
    // `workflow_dispatch:` left every other assertion here green while the
    // suite executed on no commit at all (review, PR #444). `on` is the YAML
    // 1.1 boolean `true` after parsing, which is why it is looked up both ways.
    let triggers = doc
        .get("on")
        .or_else(|| doc.get(serde_yaml::Value::Bool(true)));
    let runs_on_pull_requests = triggers.is_some_and(|value| match value {
        serde_yaml::Value::Mapping(map) => map.contains_key("pull_request"),
        serde_yaml::Value::Sequence(items) => items
            .iter()
            .any(|item| item.as_str() == Some("pull_request")),
        other => other.as_str() == Some("pull_request"),
    });
    if !runs_on_pull_requests {
        return false;
    }

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
            // Both halves in ONE step, and NOTHING ELSE in that step.
            //
            // Set membership alone was order- and reachability-blind, and three
            // shell constructs defeated it while both exact lines sat in the
            // block untouched (review, PR #444): a leading `exit 0`, a `set +e`
            // plus trailing `exit 0`, and an `if false; then … fi` wrapper. The
            // module doc claimed an unmodelled construct "can only make this
            // gate fail closed"; for those it failed open.
            //
            // Requiring the two commands to be the block's only content is what
            // makes that claim true. It is strict — a legitimate future edit
            // adding a third command turns the gate red until someone updates
            // this test — and that is the correct direction for a gate whose
            // failure mode is otherwise "nothing ran and nobody noticed".
            if lines.len() == 2 && lines.contains(&self_test) && lines.contains(&real_run) {
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
on:
  pull_request:
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
    parses_or_panics("the canonical form", good);
    assert!(
        runs_python_suite(good),
        "the reader rejects the canonical form, so every rejection below proves \
         nothing — it would reject anything at all"
    );

    // An explicit `false` is a promise this reader can check, so it must not be
    // swept up by the fail-closed rule below — otherwise every rejection there
    // would hold for a reader that simply refused any `continue-on-error` key.
    let explicitly_blocking = r#"
on:
  pull_request:
jobs:
  agent-python-tests:
    runs-on: ubuntu-latest
    continue-on-error: false
    steps:
      - continue-on-error: "false"
        run: |
          python3 scripts/run-agent-python-tests.py --self-test
          python3 scripts/run-agent-python-tests.py
"#;
    parses_or_panics("explicitly blocking", explicitly_blocking);
    assert!(
        runs_python_suite(explicitly_blocking),
        "the reader rejected a step that spells out `continue-on-error: false`, \
         which is the blocking case it exists to accept"
    );

    let cases: [(&str, &str); 15] = [
        (
            "commented out inside the run block",
            r#"
on:
  pull_request:
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
on:
  pull_request:
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
on:
  pull_request:
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
on:
  pull_request:
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
on:
  pull_request:
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
            "step tolerates its failure through an expression GitHub resolves later",
            r#"
on:
  pull_request:
jobs:
  agent-python-tests:
    runs-on: ubuntu-latest
    steps:
      - continue-on-error: ${{ true }}
        run: |
          python3 scripts/run-agent-python-tests.py --self-test
          python3 scripts/run-agent-python-tests.py
"#,
        ),
        (
            "job tolerates its failure through an expression GitHub resolves later",
            r#"
on:
  pull_request:
jobs:
  agent-python-tests:
    runs-on: ubuntu-latest
    continue-on-error: ${{ matrix.experimental }}
    steps:
      - run: |
          python3 scripts/run-agent-python-tests.py --self-test
          python3 scripts/run-agent-python-tests.py
"#,
        ),
        (
            "self-test only — the runner is never pointed at the repo",
            r#"
on:
  pull_request:
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
on:
  pull_request:
jobs:
  agent-python-tests:
    runs-on: ubuntu-latest
    steps:
      - run: python3 scripts/run-agent-python-tests.py
"#,
        ),
        (
            "job gated behind a condition",
            r#"
on:
  pull_request:
jobs:
  agent-python-tests:
    runs-on: ubuntu-latest
    if: github.event_name == 'schedule'
    steps:
      - run: |
          python3 scripts/run-agent-python-tests.py --self-test
          python3 scripts/run-agent-python-tests.py
"#,
        ),
        (
            "step tolerance written as a plain string, not a bool",
            r#"
on:
  pull_request:
jobs:
  agent-python-tests:
    runs-on: ubuntu-latest
    steps:
      - continue-on-error: "true"
        run: |
          python3 scripts/run-agent-python-tests.py --self-test
          python3 scripts/run-agent-python-tests.py
"#,
        ),
        (
            "short-circuited by a leading exit 0, both commands intact below it",
            r#"
on:
  pull_request:
jobs:
  agent-python-tests:
    runs-on: ubuntu-latest
    steps:
      - run: |
          exit 0
          python3 scripts/run-agent-python-tests.py --self-test
          python3 scripts/run-agent-python-tests.py
"#,
        ),
        (
            "failures swallowed by set +e and a trailing exit 0",
            r#"
on:
  pull_request:
jobs:
  agent-python-tests:
    runs-on: ubuntu-latest
    steps:
      - run: |
          set +e
          python3 scripts/run-agent-python-tests.py --self-test
          python3 scripts/run-agent-python-tests.py
          exit 0
"#,
        ),
        (
            "wrapped in a branch that never runs",
            r#"
on:
  pull_request:
jobs:
  agent-python-tests:
    runs-on: ubuntu-latest
    steps:
      - run: |
          if false; then
          python3 scripts/run-agent-python-tests.py --self-test
          python3 scripts/run-agent-python-tests.py
          fi
"#,
        ),
        (
            "a perfect job in a workflow that no longer triggers on pull requests",
            r#"
on:
  workflow_dispatch:
jobs:
  agent-python-tests:
    runs-on: ubuntu-latest
    steps:
      - run: |
          python3 scripts/run-agent-python-tests.py --self-test
          python3 scripts/run-agent-python-tests.py
"#,
        ),
    ];

    for (label, workflow) in cases {
        // Parse FIRST. `runs_python_suite` returns false on any YAML error, so
        // a fixture with broken indentation, a tab, or an unquoted `${{ }}`
        // would satisfy the rejection below for a reason having nothing to do
        // with the weakening it is meant to isolate (review, PR #444) — a
        // negative control passing vacuously, in the file whose whole job is
        // being a negative control.
        parses_or_panics(label, workflow);
        assert!(
            !runs_python_suite(workflow),
            "the reader accepted a workflow that runs nothing: {label}"
        );
    }
}

/// Panic unless `workflow` is valid YAML, naming the fixture that is not.
fn parses_or_panics(label: &str, workflow: &str) {
    if let Err(error) = serde_yaml::from_str::<serde_yaml::Value>(workflow) {
        panic!(
            "the `{label}` fixture is not valid YAML, so any assertion about it \
             proves nothing — `runs_python_suite` rejects an unparsable workflow \
             outright: {error}"
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
