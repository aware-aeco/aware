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
//!
//! A job's `if:` is judged by WHAT IT ADMITS, not by whether it exists. The
//! first version of this gate only checked for absence, and Codex found the
//! hole on review: flip the push job's condition to
//! `github.event_name == 'pull_request'` and the job never runs on a push, yet
//! the field is still present and the dormant job still contains the push
//! range — so both tests passed while nothing checked `main` at all. The two
//! conditions are therefore an exact contract, and the job carrying the push
//! range must be the one gated to `push`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

const WORKFLOW: &str = ".github/workflows/trailers.yml";
const CHECKER: &str = "no-claude-coauthor-trailers.py";

/// The exact range the push route must use. A contract, not a pattern: the
/// commits a push added are knowable only from the event's own before/after
/// SHAs, and every other range is either empty or unfixably dirty.
const PUSH_RANGE: &str = r#"--range "${BEFORE}..${AFTER}""#;

const REQUIRED_PR_TYPES: [&str; 4] = ["opened", "synchronize", "reopened", "edited"];

/// The only two conditions this workflow may gate a job with. An exact
/// contract: anything else has to be read by a human, because deciding which
/// events an arbitrary expression admits means evaluating GitHub's expression
/// language, and a gate that tries to do that is a gate that can be fooled.
const PR_IF: &str = "github.event_name == 'pull_request'";
const PUSH_IF: &str = "github.event_name == 'push'";

/// `--message-file` is what makes a step the pull-request one: the PR body is
/// half of what that route checks and exists on no other event.
const PR_BODY_FLAG: &str = "--message-file";

/// The concurrency group must vary by the pushed commit. GitHub keeps at most
/// one running and one PENDING run per group and a newly queued run replaces
/// the pending one, so a group shared across pushes silently drops the middle
/// of three — and each push run scans only its own disjoint range, so those
/// commits are then checked by nothing. `cancel-in-progress: false` protects
/// the running run, not the queued one.
const PUSH_UNIQUE_GROUP: &str = "github.event_name == 'push' && github.sha";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("cli has a repository parent")
        .to_path_buf()
}

#[derive(Debug)]
struct Job {
    id: String,
    /// `None` only when the job has no `if:` at all — it would then run on
    /// both events, including the one whose context it cannot read. Any value
    /// that IS present arrives here verbatim, bools included.
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
    /// Verbatim, so a group keyed by the pushed commit stays distinguishable
    /// from one shared across pushes.
    concurrency_group: String,
    jobs: Vec<Job>,
}

/// Which event a job's `if:` admits. `Unrecognised` is not a third event — it
/// is the gate refusing to guess, and it is itself a fault.
#[derive(Debug, PartialEq, Eq)]
enum Admits {
    PullRequest,
    Push,
    Unrecognised,
}

fn admits(if_expr: Option<&str>) -> Option<Admits> {
    match if_expr {
        None => None,
        Some(expr) if expr.trim() == PR_IF => Some(Admits::PullRequest),
        Some(expr) if expr.trim() == PUSH_IF => Some(Admits::Push),
        Some(_) => Some(Admits::Unrecognised),
    }
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

    let concurrency_group = doc
        .get("concurrency")
        .and_then(|concurrency| concurrency.get("group"))
        .and_then(serde_yaml::Value::as_str)
        .unwrap_or("<absent>")
        .to_owned();

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
            // Any `if:` value, stringified rather than read as a string.
            // `if: false` is a YAML bool, and `as_str` would have returned
            // None for it — collapsing "present but disabling" into "absent",
            // which are different facts with different remedies.
            if_expr: job.get("if").map(|value| match value {
                serde_yaml::Value::String(text) => text.clone(),
                other => serde_yaml::to_string(other)
                    .unwrap_or_default()
                    .trim()
                    .to_owned(),
            }),
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
        concurrency_group,
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

    if !wiring.concurrency_group.contains(PUSH_UNIQUE_GROUP) {
        found.push(format!(
            "the concurrency group `{}` is not keyed by the pushed commit, so \
             two pushes can share it — GitHub keeps one running and one pending \
             run per group and replaces the pending one, dropping a push whose \
             range nothing else scans. It must contain `{PUSH_UNIQUE_GROUP}`",
            wiring.concurrency_group
        ));
    }

    // Which job does which route. Both are required, and each must be gated to
    // the event it reads context from: a push-range job admitted only on
    // `pull_request` is dormant, and its mere presence used to satisfy this
    // gate.
    let mut push_route = None;
    let mut pr_route = None;

    for job in &wiring.jobs {
        let gate = admits(job.if_expr.as_deref());
        match &gate {
            None => found.push(format!(
                "job `{}` has no `if:` pinning it to one event, so it also runs \
                 on the event whose context it cannot read",
                job.id
            )),
            Some(Admits::Unrecognised) => found.push(format!(
                "job `{}` is gated by `{}`, which is neither `{PR_IF}` nor \
                 `{PUSH_IF}` — this gate will not guess which events an \
                 arbitrary expression admits, so the condition is an exact \
                 contract and a new one needs a human",
                job.id,
                job.if_expr.as_deref().unwrap_or_default()
            )),
            Some(_) => {}
        }

        let Some((check_index, run)) = &job.check_at else {
            found.push(format!(
                "job `{}` never invokes {CHECKER} with a range",
                job.id
            ));
            continue;
        };

        match job.self_test_at {
            None => found.push(format!(
                "job `{}` checks a range without running `--self-test` first, \
                 so a dead classifier would certify it",
                job.id
            )),
            Some(self_test_index) if self_test_index > *check_index => found.push(format!(
                "job `{}` runs `--self-test` after the check, which is too late \
                 to stop a dead classifier reporting clean",
                job.id
            )),
            Some(_) => {}
        }

        if run.contains(PUSH_RANGE) {
            // Only a job that actually runs on `push` counts as covering the
            // push route. This is the hole Codex found: without the gate check
            // here, a job frozen off by its own condition still satisfied it.
            if gate == Some(Admits::Push) {
                push_route = Some(job);
            } else {
                found.push(format!(
                    "job `{}` carries the push range but is gated by `{}`, so it \
                     never runs on a push — the commits a push adds are checked \
                     by nothing",
                    job.id,
                    job.if_expr.as_deref().unwrap_or("<no if:>")
                ));
            }
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

        if run.contains(PR_BODY_FLAG) {
            if gate == Some(Admits::PullRequest) {
                pr_route = Some(job);
            } else {
                found.push(format!(
                    "job `{}` checks the pull request body but is gated by `{}`, \
                     so it never runs on a pull request — the preventive route, \
                     the only one that can still stop a trailer, is dead",
                    job.id,
                    job.if_expr.as_deref().unwrap_or("<no if:>")
                ));
            }
        }
    }

    if push_route.is_none() {
        found.push(format!(
            "no job both runs on `push` and checks the commits that push added \
             — the contract is `{PUSH_RANGE}`; `origin/main..` is empty on that \
             route and the full history carries 19 unfixable offenders"
        ));
    }
    if pr_route.is_none() {
        found.push(format!(
            "no job both runs on `pull_request` and checks the branch with \
             `{PR_BODY_FLAG}` — that is the only route on which a trailer can \
             still be prevented rather than merely recorded"
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
            "no job both runs on `push` and checks the commits that push added",
        ),
        // Codex's P2 on review of the first version of this gate. The job is
        // still there and still carries the push range; it simply never runs.
        (
            "the push job frozen off by its own condition",
            live.replace(
                "    if: github.event_name == 'push'",
                "    if: github.event_name == 'pull_request'",
            ),
            "carries the push range but is gated by",
        ),
        (
            "the push job disabled outright",
            live.replace("    if: github.event_name == 'push'", "    if: false"),
            "which is neither",
        ),
        (
            "the pull_request job frozen off by its own condition",
            live.replace(
                "    if: github.event_name == 'pull_request'",
                "    if: github.event_name == 'push'",
            ),
            "never runs on a pull request",
        ),
        // Codex's second P2. The group goes back to being shared between
        // pushes, which silently drops the middle of three.
        (
            "the concurrency group shared between pushes again",
            live.replace(
                "  group: trailers-${{ github.ref }}-${{ github.event_name == 'push' && github.sha || 'pull-request' }}",
                "  group: trailers-${{ github.ref }}",
            ),
            "is not keyed by the pushed commit",
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
