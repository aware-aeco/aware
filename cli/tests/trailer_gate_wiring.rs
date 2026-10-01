//! Guard the *wiring* of the Claude-co-author-trailer gate.
//!
//! `scripts/no-claude-coauthor-trailers.py` is the classifier and carries its
//! own negative control (`--self-test`): message trailers, commit authorship,
//! bot addresses, confusable spellings, and end-to-end ranges over a throwaway
//! repository. None of that says the gate ever *runs*, and that is precisely how
//! the rule kept breaking.
//!
//! `.github/workflows/trailers.yml` triggered on `pull_request` alone. Release
//! commits are pushed straight to `main` — 40 of the commits added since that
//! workflow was written carry no `(#N)` — so they raise no `pull_request` event
//! and the gate never saw them. Four are authored by
//! `Claude <noreply@anthropic.com>`, two of them (v0.129.0 `dc0092bb`,
//! v0.137.0 `0fe24d5c`) landing after #412 taught the classifier to read
//! authorship. Replaying v0.137.0's push range through the checker flags it
//! immediately: the classifier was right and was simply never invoked.
//!
//! `main` is append-only (CLAUDE.md §Git workflow: "no force-push, no rewrite"),
//! so those commits cannot be fixed and the count is all this gate protects.
//! Which makes the wiring load-bearing in a way a behavioural test cannot
//! reach — delete the `push` trigger and every assertion in the Python
//! self-test still passes while `main` goes unwatched again.
//!
//! ## What it checks
//!
//! * both routes are wired: `pull_request` (preventive — the only place a
//!   trailer can still be stopped) and `push` to `main` (detection — the only
//!   place the synthesised trailer is observable);
//! * `pull_request.types` still carries `edited`, which the workflow's own
//!   comment calls load-bearing and which nothing else held;
//! * `cancel-in-progress` is not unconditionally true, because on an
//!   append-only branch a superseded run is a commit nothing ever checks;
//! * every job is pinned to one event with `if:`, since each reads context the
//!   other event leaves empty;
//! * the push job checks `before..after` — the commits that push added — and
//!   not `origin/main..`, which is empty on that route, nor the whole history,
//!   19 commits of which carry the violation unfixably;
//! * in both jobs `--self-test` runs before the check, so a dead classifier
//!   fails loudly instead of certifying the branch.
//!
//! ## Shape
//!
//! [`read_wiring`] extracts the facts and [`faults`] judges them, so the
//! negative control below can drive both over mutated workflow sources rather
//! than only over the live file — which currently passes, and would pass just
//! as well with the predicate broken.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

const WORKFLOW: &str = ".github/workflows/trailers.yml";
const CHECKER: &str = "no-claude-coauthor-trailers.py";

/// The exact range the push route must use. A contract, not a pattern: the
/// commits a push added are knowable only from the event's own before/after
/// SHAs, and every other range is either empty or unfixably dirty.
const PUSH_RANGE: &str = r#"--range "${BEFORE}..${AFTER}""#;

const REQUIRED_PR_TYPES: [&str; 4] = ["opened", "synchronize", "reopened", "edited"];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("cli has a repository parent")
        .to_path_buf()
}

#[derive(Debug)]
struct Job {
    id: String,
    /// `None` when the job has no `if:` at all — it would then run on both
    /// events, including the one whose context it cannot read.
    if_expr: Option<String>,
    /// Index of the first step invoking the checker with `--self-test`.
    self_test_at: Option<usize>,
    /// Index of the first step invoking the checker with a `--range`, and the
    /// full `run:` text of that step.
    check_at: Option<(usize, String)>,
    /// `env:` keys the checking step binds, mapped to their expressions.
    check_env: Vec<(String, String)>,
}

#[derive(Debug)]
struct Wiring {
    triggers: BTreeSet<String>,
    push_branches: Vec<String>,
    pr_types: BTreeSet<String>,
    /// Verbatim, so an expression and a literal `true` stay distinguishable.
    cancel_in_progress: String,
    jobs: Vec<Job>,
}

/// `on:` is the one key whose spelling depends on the YAML version: 1.1 resolves
/// a bare `on` to the boolean true, 1.2 keeps it a string. Accept either rather
/// than depend on which rule the parser in use today applies.
fn triggers_of(doc: &serde_yaml::Value) -> Option<&serde_yaml::Mapping> {
    for key in [
        serde_yaml::Value::String("on".to_owned()),
        serde_yaml::Value::Bool(true),
    ] {
        if let Some(value) = doc.get(&key) {
            return value.as_mapping();
        }
    }
    None
}

fn step_run(step: &serde_yaml::Value) -> &str {
    step.get("run")
        .and_then(serde_yaml::Value::as_str)
        .unwrap_or("")
}

fn read_wiring(source: &str) -> Result<Wiring, String> {
    let doc: serde_yaml::Value = serde_yaml::from_str(source)
        .map_err(|error| format!("{WORKFLOW} is not valid YAML: {error}"))?;

    let on = triggers_of(&doc).ok_or_else(|| format!("{WORKFLOW} declares no triggers"))?;

    let mut triggers = BTreeSet::new();
    for key in on.keys() {
        triggers.insert(
            key.as_str()
                .ok_or_else(|| "a trigger name is not a string".to_owned())?
                .to_owned(),
        );
    }

    let push_branches = on
        .get(serde_yaml::Value::String("push".to_owned()))
        .and_then(|push| push.get("branches"))
        .and_then(serde_yaml::Value::as_sequence)
        .map(|branches| {
            branches
                .iter()
                .filter_map(serde_yaml::Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();

    let pr_types = on
        .get(serde_yaml::Value::String("pull_request".to_owned()))
        .and_then(|pr| pr.get("types"))
        .and_then(serde_yaml::Value::as_sequence)
        .map(|types| {
            types
                .iter()
                .filter_map(serde_yaml::Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();

    // Absent is not the same as false here, so record the absence as such
    // rather than defaulting it to something that happens to pass.
    let cancel_in_progress = doc
        .get("concurrency")
        .and_then(|concurrency| concurrency.get("cancel-in-progress"))
        .map(|value| match value {
            serde_yaml::Value::String(text) => text.clone(),
            other => serde_yaml::to_string(other)
                .unwrap_or_default()
                .trim()
                .to_owned(),
        })
        .unwrap_or_else(|| "<absent>".to_owned());

    let mut jobs = Vec::new();
    let job_map = doc
        .get("jobs")
        .and_then(serde_yaml::Value::as_mapping)
        .ok_or_else(|| format!("{WORKFLOW} declares no jobs"))?;

    for (id, job) in job_map {
        let id = id
            .as_str()
            .ok_or_else(|| "a job id is not a string".to_owned())?
            .to_owned();
        let steps = job
            .get("steps")
            .and_then(serde_yaml::Value::as_sequence)
            .map(Vec::as_slice)
            .unwrap_or(&[]);

        let mut self_test_at = None;
        let mut check_at: Option<(usize, String)> = None;
        let mut check_env = Vec::new();

        for (index, step) in steps.iter().enumerate() {
            let run = step_run(step);
            if !run.contains(CHECKER) {
                continue;
            }
            if run.contains("--self-test") && self_test_at.is_none() {
                self_test_at = Some(index);
            }
            if run.contains("--range") && check_at.is_none() {
                check_at = Some((index, run.to_owned()));
                if let Some(env) = step.get("env").and_then(serde_yaml::Value::as_mapping) {
                    for (key, value) in env {
                        check_env.push((
                            key.as_str().unwrap_or_default().to_owned(),
                            value.as_str().unwrap_or_default().to_owned(),
                        ));
                    }
                }
            }
        }

        jobs.push(Job {
            id,
            if_expr: job
                .get("if")
                .and_then(serde_yaml::Value::as_str)
                .map(str::to_owned),
            self_test_at,
            check_at,
            check_env,
        });
    }

    Ok(Wiring {
        triggers,
        push_branches,
        pr_types,
        cancel_in_progress,
        jobs,
    })
}

fn faults(wiring: &Wiring) -> Vec<String> {
    let mut found = Vec::new();

    for trigger in ["pull_request", "push"] {
        if !wiring.triggers.contains(trigger) {
            found.push(format!(
                "no `{trigger}` trigger — that route reaches `main` unchecked"
            ));
        }
    }

    if wiring.triggers.contains("push") && !wiring.push_branches.iter().any(|b| b == "main") {
        found.push(format!(
            "the `push` trigger does not cover `main` (covers {:?})",
            wiring.push_branches
        ));
    }

    for required in REQUIRED_PR_TYPES {
        if !wiring.pr_types.contains(required) {
            found.push(format!(
                "`pull_request.types` no longer carries `{required}`"
            ));
        }
    }

    // A literal `true` cancels a superseded run on every ref, `main` included.
    if wiring.cancel_in_progress == "true" {
        found.push(
            "`cancel-in-progress` is unconditionally true — a superseded run on \
             an append-only branch is a commit nothing ever checks"
                .to_owned(),
        );
    }

    let mut checks_push_range = false;
    for job in &wiring.jobs {
        if job.if_expr.is_none() {
            found.push(format!(
                "job `{}` has no `if:` pinning it to one event, so it also runs \
                 on the event whose context it cannot read",
                job.id
            ));
        }
        match &job.check_at {
            None => found.push(format!(
                "job `{}` never invokes {CHECKER} with a range",
                job.id
            )),
            Some((check_index, run)) => {
                match job.self_test_at {
                    None => found.push(format!(
                        "job `{}` checks a range without running `--self-test` first, \
                         so a dead classifier would certify it",
                        job.id
                    )),
                    Some(self_test_index) if self_test_index > *check_index => found.push(format!(
                        "job `{}` runs `--self-test` after the check, which is too \
                             late to stop a dead classifier reporting clean",
                        job.id
                    )),
                    Some(_) => {}
                }
                if run.contains(PUSH_RANGE) {
                    checks_push_range = true;
                    for (name, source) in [
                        ("BEFORE", "github.event.before"),
                        ("AFTER", "github.event.after"),
                    ] {
                        if !job
                            .check_env
                            .iter()
                            .any(|(key, value)| key == name && value.contains(source))
                        {
                            found.push(format!(
                                "job `{}` uses ${{{name}}} in its range but does not bind \
                                 it to `{source}`",
                                job.id
                            ));
                        }
                    }
                }
            }
        }
    }

    if wiring.triggers.contains("push") && !checks_push_range {
        found.push(format!(
            "no job checks the commits a push added — the contract is \
             `{PUSH_RANGE}`; `origin/main..` is empty on that route and the full \
             history carries 19 unfixable offenders"
        ));
    }

    found
}

#[test]
fn the_trailer_gate_watches_both_routes_into_main() {
    let path = repo_root().join(WORKFLOW);
    let source = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    let wiring = read_wiring(&source).unwrap_or_else(|error| panic!("{error}"));

    assert_eq!(
        faults(&wiring),
        Vec::<String>::new(),
        "{WORKFLOW} no longer enforces CLAUDE.md §Git workflow on every route \
         into `main`. Do not satisfy this by narrowing the gate — CLAUDE.md \
         §Engineering rules forbids it, and the 19 offenders already on `main` \
         are what narrowing it costs."
    );
}

/// The negative control. The test above runs over the live workflow, which
/// passes — so it would report clean whether [`faults`] works or has stopped
/// judging anything at all. This drives it over sources that must be rejected.
#[test]
fn the_wiring_classifier_matches_its_contract() {
    let path = repo_root().join(WORKFLOW);
    let live = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));

    // Each mutation is a plausible edit, with the substring it rewrites asserted
    // present first — a mutation that silently matched nothing would leave the
    // live source behind and this control would certify it unchanged.
    let mutations: Vec<(&str, String, &str)> = vec![
        (
            "the push trigger deleted",
            live.replace("  push:\n    branches: [main]\n", ""),
            "no `push` trigger",
        ),
        (
            "the push trigger aimed elsewhere",
            live.replace("    branches: [main]", "    branches: [release]"),
            "does not cover `main`",
        ),
        (
            "`edited` dropped from the pull_request types",
            live.replace(
                "types: [opened, synchronize, reopened, edited]",
                "types: [opened, synchronize, reopened]",
            ),
            "no longer carries `edited`",
        ),
        (
            "cancellation made unconditional again",
            live.replace(
                "cancel-in-progress: ${{ github.event_name == 'pull_request' }}",
                "cancel-in-progress: true",
            ),
            "unconditionally true",
        ),
        (
            "the push range widened to the whole branch",
            live.replace(PUSH_RANGE, r#"--range "origin/main..${AFTER}""#),
            "no job checks the commits a push added",
        ),
        (
            "the event SHAs unbound from the step",
            live.replace("          BEFORE: ${{ github.event.before }}", "          BEFORE: deadbeef"),
            "does not bind it to `github.event.before`",
        ),
        (
            "the event gate removed from a job",
            live.replace("    if: github.event_name == 'push'\n", ""),
            "has no `if:` pinning it to one event",
        ),
        (
            "the self-test dropped from the push job",
            live.replace(
                "      - name: Self-test the trailer classifier\n        run: python3 scripts/no-claude-coauthor-trailers.py --self-test\n\n      # `before..after`",
                "      # `before..after`",
            ),
            "without running `--self-test` first",
        ),
    ];

    for (what, mutated, expected) in mutations {
        assert_ne!(
            mutated, live,
            "the `{what}` mutation rewrote nothing, so it proves nothing — fix \
             the fixture, not the gate"
        );
        let wiring = read_wiring(&mutated)
            .unwrap_or_else(|error| panic!("`{what}` produced unreadable YAML: {error}"));
        let found = faults(&wiring);
        assert!(
            found.iter().any(|fault| fault.contains(expected)),
            "`{what}` was not rejected for {expected:?}; faults were {found:?}"
        );
    }

    // And the live source must still be the thing that passes, so a predicate
    // that simply rejects everything cannot satisfy the loop above.
    let wiring = read_wiring(&live).unwrap_or_else(|error| panic!("{error}"));
    assert_eq!(faults(&wiring), Vec::<String>::new());
}
