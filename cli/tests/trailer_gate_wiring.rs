//! Guard the *wiring* of the Claude-co-author-trailer gate.
//!
//! `scripts/no-claude-coauthor-trailers.py` is the classifier and carries its
//! own negative control (`--self-test`): message trailers, commit authorship,
//! bot addresses, confusable spellings, and end-to-end ranges over a throwaway
//! repository. None of that says the gate ever *runs*, and that is precisely how
//! the rule kept breaking.
//!
//! `.github/workflows/trailers.yml` triggered on `pull_request` alone. Release
//! bumps are pushed straight to `main`, so they raise no `pull_request` event
//! and the gate never saw them. Four are authored by
//! `Claude <noreply@anthropic.com>`: v0.120.0 (a2cd3a6d), v0.122.0 (f6ed04bc),
//! v0.129.0 (dc0092bb), v0.137.0 (0fe24d5c) — the last two landing after #412
//! taught the classifier to read authorship. Replaying v0.137.0's push range
//! through the checker flags it immediately: the classifier was right and was
//! simply never invoked.
//!
//! `main` is append-only (CLAUDE.md §Git workflow: "no force-push, no rewrite"),
//! so those commits cannot be fixed and holding the count is all this gate
//! protects. Which makes the wiring load-bearing in a way a behavioural test
//! cannot reach — delete the `push` trigger and every assertion in the Python
//! self-test still passes while `main` goes unwatched again.
//!
//! ## Everything here is a contract, because a pattern can be satisfied by dead text
//!
//! The first version of this gate matched substrings over `run:` text and
//! checked that fields were merely PRESENT. Review found nine one-line edits
//! that disarmed the workflow with both tests still green: freezing a job off
//! with `if: github.event_name == 'pull_request'`, moving the push range into
//! the pull_request job, putting `exit 0` first in the step, commenting the
//! checker call out (the contract string survives as a comment), prefixing it
//! with `echo`, appending `|| true` to the self-test, `continue-on-error: true`,
//! a step-level `if: false`, and inverting `cancel-in-progress` to
//! `${{ github.event_name == 'push' }}` — which cancels on the append-only
//! branch and preserves PR runs, the exact harm the workflow comment forbids.
//!
//! So nothing here is judged by presence or by substring alone:
//!
//! * a job's `if:` is judged by WHAT IT ADMITS, as an exact contract of the two
//!   permitted expressions, and the job carrying the push range must be the one
//!   gated to `push` — the PR-body job likewise to `pull_request`;
//! * `run:` text has `#` comment lines stripped before matching, and the
//!   checker must be invoked by a line that STARTS with `python3 ` and names the
//!   checker by path (which the test also asserts exists on disk);
//! * a step that invokes the checker may not carry `continue-on-error`, a
//!   step-level `if:`, `|| true`, `|| :`, `set +e` or a bare `exit 0`, each of
//!   which makes the step run, fail, and report success;
//! * both ranges are contracts — the push route's `before..after` and the
//!   pull_request route's `origin/$BASE_REF..$HEAD_SHA`. The second was
//!   unpinned, so `--range "HEAD..HEAD"` passed, and an empty range makes the
//!   checker print "ok";
//! * every `env:` binding the ranges depend on is compared for EQUALITY, not
//!   containment: `BEFORE: ${{ ...base.sha }}${{ github.event.before && '' }}`
//!   mentions the right context while binding the wrong value;
//! * `cancel-in-progress` and the concurrency group are exact contracts too,
//!   since the safe set for each is a single string;
//! * `--self-test` must run before the check — compared by (step, byte offset),
//!   so two calls in one step are ordered correctly — and must itself be
//!   unconditional and fatal.
//!
//! Step-level `env:` is required even though GitHub also resolves `env:` at job
//! and workflow scope. That is deliberately stricter than Actions: reproducing
//! env resolution here would be the kind of partial re-implementation that this
//! repository's `toolchain_pin_gate.rs` was rewritten to avoid.
//!
//! ## Shape
//!
//! [`read_wiring`] extracts facts and [`faults`] judges them, so the negative
//! control below can drive both over mutated workflow sources rather than only
//! over the live file — which passes, and would pass just as well with the
//! predicate broken.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

const WORKFLOW: &str = ".github/workflows/trailers.yml";

/// By path, not basename: repointing the call at `scripts/gone/…` used to pass.
const CHECKER: &str = "scripts/no-claude-coauthor-trailers.py";

/// The commits a push added are knowable only from the event's own before/after
/// SHAs. Every other range is either empty on this route or unfixably dirty.
const PUSH_RANGE: &str = r#"--range "${BEFORE}..${AFTER}""#;

/// The live base ref, not `base.sha`, which lags and drags commits already on
/// `main` into the range. Pinned because `--range "HEAD..HEAD"` passed before.
const PR_RANGE: &str = r#"--range "origin/$BASE_REF..$HEAD_SHA""#;

/// The PR body is half of what the preventive route checks.
const PR_BODY_FLAG: &str = "--message-file";

/// The only two conditions this workflow may gate a job with. An exact
/// contract: deciding which events an arbitrary expression admits means
/// evaluating GitHub's expression language, and a gate that tries to do that is
/// a gate that can be fooled.
const PR_IF: &str = "github.event_name == 'pull_request'";
const PUSH_IF: &str = "github.event_name == 'push'";

/// Cancelling is safe only for a pull request. The inverse spelling cancels on
/// the append-only branch, where a superseded run is a commit nothing rechecks.
const CANCEL_CONTRACT: &str = "${{ github.event_name == 'pull_request' }}";

/// The group must vary by the pushed commit. GitHub keeps at most one running
/// and one PENDING run per group and a newly queued run replaces the pending
/// one, so a group shared across pushes drops the middle of three — and each
/// push run scans only its own disjoint range.
const GROUP_CONTRACT: &str =
    "trailers-${{ github.ref }}-${{ github.event_name == 'push' && github.sha || 'pull-request' }}";

/// Fragments that make a step run, fail, and still report success.
const DISARMERS: [&str; 3] = ["|| true", "|| :", "set +e"];

const REQUIRED_PR_TYPES: [&str; 4] = ["opened", "synchronize", "reopened", "edited"];

const PUSH_ENV: [(&str, &str); 2] = [
    ("BEFORE", "${{ github.event.before }}"),
    ("AFTER", "${{ github.event.after }}"),
];

const PR_ENV: [(&str, &str); 3] = [
    ("BASE_REF", "${{ github.event.pull_request.base.ref }}"),
    ("HEAD_SHA", "${{ github.event.pull_request.head.sha }}"),
    ("PR_BODY", "${{ github.event.pull_request.body }}"),
];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("cli has a repository parent")
        .to_path_buf()
}

/// One step, reduced to the facts [`faults`] judges. `run` has `#` comment
/// lines removed, so text a `#` disabled cannot satisfy a contract.
#[derive(Debug)]
struct Step {
    index: usize,
    run: String,
    env: BTreeMap<String, String>,
    conditional: bool,
    continue_on_error: bool,
}

#[derive(Debug)]
struct Job {
    id: String,
    /// `None` only when there is no `if:` at all. Any value present arrives
    /// verbatim, bools included: `if: false` is a YAML bool, and reading it as
    /// a string turned "present but disabling" into "absent".
    if_expr: Option<String>,
    continue_on_error: bool,
    checks_out_full_history: bool,
    steps: Vec<Step>,
}

#[derive(Debug)]
struct Wiring {
    triggers: BTreeSet<String>,
    /// `None` when `push:` declares no `branches` — which is every branch, and
    /// therefore covers `main`. Collapsing that into an empty list produced the
    /// false diagnosis "does not cover `main` (covers [])" on a legal workflow.
    push_branches: Option<Vec<String>>,
    pr_types: BTreeSet<String>,
    /// Keys other than `branches` under `push:` — a path filter here silences
    /// the route without touching anything this gate used to read.
    push_filters: BTreeSet<String>,
    cancel_in_progress: String,
    concurrency_group: String,
    jobs: Vec<Job>,
}

/// Which event a job's `if:` admits. `Unrecognised` is not a third event — it
/// is the gate declining to guess, and it is itself a fault.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Admits {
    PullRequest,
    Push,
    Unrecognised,
}

fn admits(if_expr: Option<&str>) -> Option<Admits> {
    match if_expr.map(str::trim) {
        None => None,
        Some(expr) if expr == PR_IF => Some(Admits::PullRequest),
        Some(expr) if expr == PUSH_IF => Some(Admits::Push),
        Some(_) => Some(Admits::Unrecognised),
    }
}

/// Stringify any YAML scalar verbatim, so a bool or number stays
/// distinguishable from an absent key and from a string.
fn verbatim(value: &serde_yaml::Value) -> String {
    match value {
        serde_yaml::Value::String(text) => text.clone(),
        other => serde_yaml::to_string(other)
            .unwrap_or_default()
            .trim()
            .to_owned(),
    }
}

/// Whether `continue-on-error` makes a failure tolerated. Same shape, and for
/// the same reason, as `agent_python_suites_gate.rs::continues_on_error`: only a
/// literal `false` in either spelling is a promise this reader can check now.
/// Anything else — an expression, a context lookup — is decided later and
/// elsewhere, so it is not accepted.
fn continues_on_error(container: Option<&serde_yaml::Value>) -> bool {
    match container.and_then(|value| value.get("continue-on-error")) {
        None | Some(serde_yaml::Value::Bool(false)) => false,
        Some(serde_yaml::Value::String(text)) if text.eq_ignore_ascii_case("false") => false,
        Some(_) => true,
    }
}

/// Drop whole-line `#` comments. A `#` inside a quoted string is dropped too;
/// the result is only ever matched against contracts, never executed, and
/// erring toward *less* text can only cost a false fault, never a false pass.
fn strip_comments(run: &str) -> String {
    run.lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Byte offset of the LOGICAL command that invokes the checker with `flag`.
///
/// Lines are joined across trailing backslashes first, so the live workflow's
/// continued `python3 … \` / `--range …` / `--message-file …` is one command,
/// while two separate `python3` calls stay separate. An earlier version looked
/// ahead to the rest of the whole block instead, which reported the `--range`
/// call's offset as the `--self-test` call's — and so could not see a self-test
/// moved after the check inside one step.
///
/// The command's FIRST line must start with `python3 `, which is what rejects
/// `echo python3 …`; comment lines are already gone by this point.
fn invocation_offset(run: &str, flag: &str) -> Option<usize> {
    let mut at = 0usize;
    let mut command_start = 0usize;
    let mut command = String::new();
    let mut first_line = String::new();

    for line in run.split_inclusive('\n') {
        if command.is_empty() {
            command_start = at;
            first_line = line.trim_start().to_owned();
        }
        command.push_str(line);
        at += line.len();

        // A trailing backslash continues the command onto the next line.
        if line.trim_end().ends_with('\\') {
            continue;
        }
        if first_line.starts_with("python3 ") && command.contains(CHECKER) && command.contains(flag)
        {
            return Some(command_start);
        }
        command.clear();
    }

    if first_line.starts_with("python3 ") && command.contains(CHECKER) && command.contains(flag) {
        return Some(command_start);
    }
    None
}

/// `on:` is the one key whose spelling depends on the YAML version: 1.1
/// resolves a bare `on` to the boolean true, 1.2 keeps it a string. Accept
/// either, and accept the sequence form (`on: [push, pull_request]`), which is
/// legal and used to be rejected as a malformed workflow.
fn triggers_of(doc: &serde_yaml::Value) -> Option<&serde_yaml::Value> {
    for key in [
        serde_yaml::Value::String("on".to_owned()),
        serde_yaml::Value::Bool(true),
    ] {
        if let Some(value) = doc.get(&key) {
            return Some(value);
        }
    }
    None
}

fn read_wiring(source: &str, label: &str) -> Result<Wiring, String> {
    let doc: serde_yaml::Value = serde_yaml::from_str(source)
        .map_err(|error| format!("{label} is not valid YAML: {error}"))?;

    let on = triggers_of(&doc).ok_or_else(|| format!("{label} declares no triggers"))?;

    let mut triggers = BTreeSet::new();
    if let Some(map) = on.as_mapping() {
        for key in map.keys() {
            if let Some(name) = key.as_str() {
                triggers.insert(name.to_owned());
            }
        }
    } else if let Some(seq) = on.as_sequence() {
        for item in seq {
            if let Some(name) = item.as_str() {
                triggers.insert(name.to_owned());
            }
        }
    } else if let Some(name) = on.as_str() {
        triggers.insert(name.to_owned());
    } else {
        return Err(format!(
            "{label} declares `on:` in a form this gate cannot read"
        ));
    }

    let push = on.get("push");
    let push_branches = match push.and_then(|push| push.get("branches")) {
        None => None,
        Some(serde_yaml::Value::Sequence(items)) => Some(
            items
                .iter()
                .filter_map(serde_yaml::Value::as_str)
                .map(str::to_owned)
                .collect(),
        ),
        // `branches: main` — a bare scalar is legal and covers main.
        Some(other) => Some(vec![verbatim(other)]),
    };

    let push_filters = push
        .and_then(serde_yaml::Value::as_mapping)
        .map(|map| {
            map.keys()
                .filter_map(serde_yaml::Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();

    let pr_types = on
        .get("pull_request")
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

    let concurrency = doc.get("concurrency");
    let cancel_in_progress = concurrency
        .and_then(|c| c.get("cancel-in-progress"))
        .map(verbatim)
        .unwrap_or_else(|| "<absent>".to_owned());
    let concurrency_group = concurrency
        .and_then(|c| c.get("group"))
        .map(verbatim)
        .unwrap_or_else(|| "<absent>".to_owned());

    let mut jobs = Vec::new();
    let job_map = doc
        .get("jobs")
        .and_then(serde_yaml::Value::as_mapping)
        .ok_or_else(|| format!("{label} declares no jobs"))?;

    for (id, job) in job_map {
        let id = id
            .as_str()
            .ok_or_else(|| "a job id is not a string".to_owned())?
            .to_owned();
        let raw_steps = job
            .get("steps")
            .and_then(serde_yaml::Value::as_sequence)
            .map(Vec::as_slice)
            .unwrap_or(&[]);

        let mut steps = Vec::new();
        let mut checks_out_full_history = false;
        for (index, step) in raw_steps.iter().enumerate() {
            if step
                .get("uses")
                .and_then(serde_yaml::Value::as_str)
                .is_some_and(|uses| uses.starts_with("actions/checkout"))
                && step
                    .get("with")
                    .and_then(|with| with.get("fetch-depth"))
                    .map(verbatim)
                    .as_deref()
                    == Some("0")
            {
                checks_out_full_history = true;
            }

            let run = step.get("run").map(verbatim).unwrap_or_default();
            let mut env = BTreeMap::new();
            if let Some(map) = step.get("env").and_then(serde_yaml::Value::as_mapping) {
                for (key, value) in map {
                    if let Some(key) = key.as_str() {
                        env.insert(key.to_owned(), verbatim(value));
                    }
                }
            }
            steps.push(Step {
                index,
                run: strip_comments(&run),
                env,
                conditional: step.get("if").is_some(),
                continue_on_error: continues_on_error(Some(step)),
            });
        }

        jobs.push(Job {
            id,
            if_expr: job.get("if").map(verbatim),
            continue_on_error: continues_on_error(Some(job)),
            checks_out_full_history,
            steps,
        });
    }

    Ok(Wiring {
        triggers,
        push_branches,
        pr_types,
        push_filters,
        cancel_in_progress,
        concurrency_group,
        jobs,
    })
}

/// The step invoking the checker with `flag`, and the offset of that call.
fn call<'a>(job: &'a Job, flag: &str) -> Option<(&'a Step, usize)> {
    job.steps
        .iter()
        .find_map(|step| invocation_offset(&step.run, flag).map(|offset| (step, offset)))
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

    // `None` is every branch, which includes `main`.
    if wiring.triggers.contains("push")
        && let Some(branches) = &wiring.push_branches
        && !branches.iter().any(|b| b == "main")
    {
        found.push(format!(
            "the `push` trigger does not cover `main` (covers {branches:?})"
        ));
    }

    // `paths-ignore: ['**']` leaves the trigger present and covering `main`
    // while the workflow never fires. A release bump touches cli/Cargo.toml,
    // cli-npm/package.json and docs, so even a well-meant narrowing re-opens
    // this route.
    for filter in ["paths", "paths-ignore"] {
        if wiring.push_filters.contains(filter) {
            found.push(format!(
                "the `push` trigger carries `{filter}`, which can leave it \
                 present and aimed at `main` while the workflow never fires on \
                 the commits it is meant to check"
            ));
        }
    }

    for required in REQUIRED_PR_TYPES {
        if !wiring.pr_types.contains(required) {
            found.push(format!(
                "`pull_request.types` no longer carries `{required}`"
            ));
        }
    }

    // An allowlist, because the safe set is one string plus the two spellings
    // of "never cancel". `${{ github.event_name == 'push' }}` is the exact
    // inverse of the policy and used to pass, as did `yes`, `${{ true }}` and
    // the typo `ture`.
    if !matches!(
        wiring.cancel_in_progress.as_str(),
        CANCEL_CONTRACT | "false" | "<absent>"
    ) {
        found.push(format!(
            "`cancel-in-progress` is `{}`, which is neither `{CANCEL_CONTRACT}` \
             nor `false` nor absent — on an append-only branch a cancelled run \
             is a commit nothing ever rechecks, so this is an allowlist and a \
             new value needs a human",
            wiring.cancel_in_progress
        ));
    }

    if wiring.concurrency_group != GROUP_CONTRACT {
        found.push(format!(
            "the concurrency group is `{}`, not `{GROUP_CONTRACT}` — it must be \
             keyed by the pushed commit, because GitHub keeps one running and \
             one pending run per group and replaces the pending one, dropping a \
             push whose range nothing else scans",
            wiring.concurrency_group
        ));
    }

    let mut push_route = false;
    let mut pr_route = false;

    for job in &wiring.jobs {
        let gate = admits(job.if_expr.as_deref());
        let shown = job.if_expr.as_deref().unwrap_or("<no if:>");
        match gate {
            None => found.push(format!(
                "job `{}` has no `if:` pinning it to one event, so it also runs \
                 on the event whose context it cannot read",
                job.id
            )),
            Some(Admits::Unrecognised) => found.push(format!(
                "job `{}` is gated by `{shown}`, which is neither `{PR_IF}` nor \
                 `{PUSH_IF}` — this gate will not guess which events an \
                 arbitrary expression admits, so the condition is an exact \
                 contract and a new one needs a human",
                job.id
            )),
            Some(_) => {}
        }

        if job.continue_on_error {
            found.push(format!(
                "job `{}` carries `continue-on-error`, so it runs, fails, and \
                 reports success",
                job.id
            ));
        }

        let self_test = call(job, "--self-test");
        let check = call(job, "--range");

        let Some((check_step, check_offset)) = check else {
            found.push(format!(
                "job `{}` has no line starting with `python3 ` that runs \
                 `{CHECKER}` with a `--range`",
                job.id
            ));
            continue;
        };

        if !job.checks_out_full_history {
            found.push(format!(
                "job `{}` checks a range without `actions/checkout` at \
                 `fetch-depth: 0` — a shallow clone cannot reach the commits \
                 the range names",
                job.id
            ));
        }

        match self_test {
            None => found.push(format!(
                "job `{}` checks a range without running `--self-test` first, so \
                 a dead classifier would certify it",
                job.id
            )),
            Some((self_test_step, self_test_offset)) => {
                // (step, offset): two calls in one step order correctly too.
                if (self_test_step.index, self_test_offset) > (check_step.index, check_offset) {
                    found.push(format!(
                        "job `{}` runs `--self-test` after the check, which is too \
                         late to stop a dead classifier reporting clean",
                        job.id
                    ));
                }
                if self_test_step.conditional
                    || self_test_step.continue_on_error
                    || DISARMERS
                        .iter()
                        .any(|disarmer| self_test_step.run.contains(disarmer))
                {
                    found.push(format!(
                        "job `{}` runs `--self-test` in a step that is \
                         conditional, non-fatal, or disarmed with one of \
                         {DISARMERS:?} — the negative control can then be \
                         skipped, or can fail while the job reports success",
                        job.id
                    ));
                }
            }
        }

        if check_step.conditional {
            found.push(format!(
                "the checking step in job `{}` carries its own `if:`, so it can \
                 be skipped while the job still reports success",
                job.id
            ));
        }
        if check_step.continue_on_error {
            found.push(format!(
                "the checking step in job `{}` carries `continue-on-error`, so a \
                 flagged commit would not fail the run",
                job.id
            ));
        }
        for disarmer in DISARMERS {
            if check_step.run.contains(disarmer) {
                found.push(format!(
                    "the checking step in job `{}` contains `{disarmer}`, which \
                     turns a flagged commit into a clean verdict",
                    job.id
                ));
            }
        }
        if check_step
            .run
            .lines()
            .any(|line| line.trim() == "exit 0" || line.trim_start().starts_with("exit 0 "))
        {
            found.push(format!(
                "the checking step in job `{}` has a bare `exit 0`, so it can \
                 return success without reaching the checker — every exit from \
                 that step except the checker call must be a refusal",
                job.id
            ));
        }

        let env_faults = |expected: &[(&str, &str)], route: &str| -> Vec<String> {
            expected
                .iter()
                .filter(|(name, value)| {
                    check_step.env.get(*name).map(String::as_str) != Some(*value)
                })
                .map(|(name, value)| {
                    format!(
                        "job `{}` binds `{name}` to {:?}, not `{value}` — the \
                         {route} route's range depends on it, and a value that \
                         merely mentions the right context is not the right value",
                        job.id,
                        check_step.env.get(*name)
                    )
                })
                .collect()
        };

        if check_step.run.contains(PUSH_RANGE) {
            // Only a job that actually runs on `push` covers the push route.
            // Without this, a job frozen off by its own condition still
            // satisfied the gate, and so did moving the range into the PR job.
            if gate == Some(Admits::Push) {
                push_route = true;
            } else {
                found.push(format!(
                    "job `{}` carries the push range but is gated by `{shown}`, \
                     so it never runs on a push — the commits a push adds are \
                     checked by nothing",
                    job.id
                ));
            }
            found.extend(env_faults(&PUSH_ENV, "push"));
        }

        if check_step.run.contains(PR_RANGE) {
            if gate == Some(Admits::PullRequest) {
                pr_route = true;
            } else {
                found.push(format!(
                    "job `{}` carries the pull request range but is gated by \
                     `{shown}`, so it never runs on a pull request — the \
                     preventive route, the only one that can still stop a \
                     trailer, is dead",
                    job.id
                ));
            }
            found.extend(env_faults(&PR_ENV, "pull request"));
            if !check_step.run.contains(PR_BODY_FLAG) {
                found.push(format!(
                    "job `{}` checks the pull request branch without \
                     `{PR_BODY_FLAG}` — GitHub copies trailers out of the \
                     description into the squash commit, so the body is half of \
                     what this route checks",
                    job.id
                ));
            }
        }
    }

    if !push_route {
        found.push(format!(
            "no job both runs on `push` and checks the commits that push added \
             with `{PUSH_RANGE}` — `origin/main..` is empty on that route and \
             the full history carries offenders that can never be fixed"
        ));
    }
    if !pr_route {
        found.push(format!(
            "no job both runs on `pull_request` and checks the branch with \
             `{PR_RANGE}` — that is the only route on which a trailer can still \
             be prevented rather than merely recorded, and an unpinned range \
             lets `HEAD..HEAD` through, over which the checker prints \"ok\""
        ));
    }

    found
}

fn live_source() -> String {
    let path = repo_root().join(WORKFLOW);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

/// A minimal workflow that satisfies every contract, written here rather than
/// derived from the live file.
///
/// This is the whole point of the fixture rewrite. The first version mutated
/// `live_source()` with `str::replace`, which review showed was worse than no
/// control: weakening the workflow made a fixture anchor stop matching, and the
/// resulting panic read "fix the fixture, not the gate" — so following the
/// test's own instruction re-greened a disarmed gate. Measured: weaken
/// `cancel-in-progress`, update the anchor, `2 passed`.
///
/// Synthetic fixtures cannot be invalidated by editing the workflow, so the
/// only way to make a weakened workflow pass is to weaken [`faults`] itself,
/// where a reviewer sees it.
const CANONICAL: &str = r#"
name: Trailers
on:
  pull_request:
    types: [opened, synchronize, reopened, edited]
  push:
    branches: [main]
concurrency:
  group: trailers-${{ github.ref }}-${{ github.event_name == 'push' && github.sha || 'pull-request' }}
  cancel-in-progress: ${{ github.event_name == 'pull_request' }}
jobs:
  trailers:
    if: github.event_name == 'pull_request'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 0
      - run: python3 scripts/no-claude-coauthor-trailers.py --self-test
      - env:
          PR_BODY: ${{ github.event.pull_request.body }}
          BASE_REF: ${{ github.event.pull_request.base.ref }}
          HEAD_SHA: ${{ github.event.pull_request.head.sha }}
        run: |
          python3 scripts/no-claude-coauthor-trailers.py --range "origin/$BASE_REF..$HEAD_SHA" --message-file "$RUNNER_TEMP/pr-body.txt"
  trailers-main:
    if: github.event_name == 'push'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 0
      - run: python3 scripts/no-claude-coauthor-trailers.py --self-test
      - env:
          BEFORE: ${{ github.event.before }}
          AFTER: ${{ github.event.after }}
        run: |
          python3 scripts/no-claude-coauthor-trailers.py --range "${BEFORE}..${AFTER}"
"#;

#[test]
fn the_checker_the_workflow_names_exists() {
    // The contract names the checker by path so it cannot be repointed at a
    // file that is not there; that is only worth something if the path is real.
    let path = repo_root().join(CHECKER);
    assert!(
        path.is_file(),
        "{WORKFLOW} invokes {CHECKER}, which is not a file at {}",
        path.display()
    );
}

#[test]
fn the_trailer_gate_watches_both_routes_into_main() {
    let wiring = read_wiring(&live_source(), WORKFLOW).unwrap_or_else(|error| panic!("{error}"));

    assert_eq!(
        faults(&wiring),
        Vec::<String>::new(),
        "{WORKFLOW} no longer enforces CLAUDE.md §Git workflow on every route \
         into `main`. Do not satisfy this by narrowing the gate — CLAUDE.md \
         §Engineering rules forbids it, and the offenders already on `main` are \
         what narrowing it costs."
    );
}

/// The negative control. The test above runs over the live workflow, which
/// passes — so it would report clean whether [`faults`] works or has stopped
/// judging anything at all.
#[test]
fn the_wiring_classifier_matches_its_contract() {
    // First: the canonical fixture must PASS, or every rejection below is
    // satisfied by a predicate that simply rejects everything.
    let canonical = read_wiring(CANONICAL, "<canonical>")
        .unwrap_or_else(|error| panic!("the canonical fixture is unreadable: {error}"));
    assert_eq!(
        faults(&canonical),
        Vec::<String>::new(),
        "the canonical fixture is rejected, so every rejection below proves nothing"
    );

    // Each mutation is one plausible weakening, applied to the fixture above.
    let mutations: Vec<(&str, String, &str)> = vec![
        (
            "the push trigger deleted",
            CANONICAL.replace("  push:\n    branches: [main]\n", ""),
            "no `push` trigger",
        ),
        (
            "the push trigger aimed elsewhere",
            CANONICAL.replace("branches: [main]", "branches: [release]"),
            "does not cover `main`",
        ),
        (
            "the push trigger silenced with a path filter",
            CANONICAL.replace(
                "  push:\n    branches: [main]",
                "  push:\n    branches: [main]\n    paths-ignore: ['**']",
            ),
            "carries `paths-ignore`",
        ),
        (
            "`edited` dropped from the pull_request types",
            CANONICAL.replace(", reopened, edited]", ", reopened]"),
            "no longer carries `edited`",
        ),
        (
            "cancellation made unconditional",
            CANONICAL.replace(CANCEL_CONTRACT, "true"),
            "`cancel-in-progress` is `true`",
        ),
        (
            "cancellation spelled `${{ true }}`",
            CANONICAL.replace(CANCEL_CONTRACT, "${{ true }}"),
            "which is neither",
        ),
        (
            "cancellation spelled `yes`",
            CANONICAL.replace(CANCEL_CONTRACT, "yes"),
            "which is neither",
        ),
        (
            "cancellation INVERTED — cancels on main, preserves PR runs",
            CANONICAL.replace(CANCEL_CONTRACT, "${{ github.event_name == 'push' }}"),
            "which is neither",
        ),
        (
            "the concurrency group shared between pushes",
            CANONICAL.replace(GROUP_CONTRACT, "trailers-${{ github.ref }}"),
            "keyed by the pushed commit",
        ),
        (
            "the push range widened to the whole branch",
            CANONICAL.replace(PUSH_RANGE, r#"--range "origin/main..${AFTER}""#),
            "no job both runs on `push`",
        ),
        (
            "the push range narrowed to the tip commit",
            CANONICAL.replace(PUSH_RANGE, r#"--range "${AFTER}~1..${AFTER}""#),
            "no job both runs on `push`",
        ),
        (
            "the pull_request range made empty",
            CANONICAL.replace(PR_RANGE, r#"--range "HEAD..HEAD""#),
            "no job both runs on `pull_request`",
        ),
        (
            "the pull_request range narrowed to the tip commit",
            CANONICAL.replace(PR_RANGE, r#"--range "$HEAD_SHA~1..$HEAD_SHA""#),
            "no job both runs on `pull_request`",
        ),
        (
            "the push job frozen off by its own condition",
            CANONICAL.replace(
                "    if: github.event_name == 'push'",
                "    if: github.event_name == 'pull_request'",
            ),
            "job `trailers-main` carries the push range but is gated by",
        ),
        (
            "the push job disabled outright",
            CANONICAL.replace("    if: github.event_name == 'push'", "    if: false"),
            "job `trailers-main` is gated by `false`",
        ),
        (
            "the push job gated by always()",
            CANONICAL.replace("    if: github.event_name == 'push'", "    if: always()"),
            "job `trailers-main` is gated by `always()`",
        ),
        (
            "the event gate removed from the push job",
            CANONICAL.replace("    if: github.event_name == 'push'\n", ""),
            "job `trailers-main` has no `if:` pinning it to one event",
        ),
        (
            "the pull_request job frozen off by its own condition",
            CANONICAL.replace(
                "    if: github.event_name == 'pull_request'\n",
                "    if: github.event_name == 'push'\n",
            ),
            "never runs on a pull request",
        ),
        (
            "the pull_request job deleted, its range moved into the push job",
            CANONICAL
                .replace(PR_RANGE, PUSH_RANGE)
                .replace("    if: github.event_name == 'pull_request'", "    if: false"),
            "no job both runs on `pull_request`",
        ),
        (
            "a step-level `if:` on the checking step",
            CANONICAL.replace(
                "      - env:\n          BEFORE:",
                "      - if: github.event_name == 'release'\n        env:\n          BEFORE:",
            ),
            "the checking step in job `trailers-main` carries its own `if:`",
        ),
        (
            "`continue-on-error` on the checking step",
            CANONICAL.replace(
                "      - env:\n          BEFORE:",
                "      - continue-on-error: true\n        env:\n          BEFORE:",
            ),
            "the checking step in job `trailers-main` carries `continue-on-error`",
        ),
        (
            "`continue-on-error` on the push job",
            CANONICAL.replace(
                "    if: github.event_name == 'push'",
                "    if: github.event_name == 'push'\n    continue-on-error: true",
            ),
            "job `trailers-main` carries `continue-on-error`",
        ),
        (
            "the checker call commented out",
            CANONICAL.replace(
                "          python3 scripts/no-claude-coauthor-trailers.py --range \"${BEFORE}..${AFTER}\"",
                "          # python3 scripts/no-claude-coauthor-trailers.py --range \"${BEFORE}..${AFTER}\"\n          echo ok",
            ),
            "no line starting with `python3 `",
        ),
        (
            "the checker call prefixed with echo",
            CANONICAL.replace(
                "          python3 scripts/no-claude-coauthor-trailers.py --range",
                "          echo python3 scripts/no-claude-coauthor-trailers.py --range",
            ),
            "no line starting with `python3 `",
        ),
        (
            "the checker repointed at a path that does not exist",
            CANONICAL.replace(CHECKER, "scripts/gone/no-claude-coauthor-trailers.py"),
            "no line starting with `python3 `",
        ),
        (
            "the checker call disarmed with `|| true`",
            CANONICAL.replace(
                "--range \"${BEFORE}..${AFTER}\"",
                "--range \"${BEFORE}..${AFTER}\" || true",
            ),
            "contains `|| true`",
        ),
        (
            "a bare `exit 0` ahead of the checker call",
            CANONICAL.replace(
                "          python3 scripts/no-claude-coauthor-trailers.py --range \"${BEFORE}..${AFTER}\"",
                "          exit 0\n          python3 scripts/no-claude-coauthor-trailers.py --range \"${BEFORE}..${AFTER}\"",
            ),
            "bare `exit 0`",
        ),
        (
            "the self-test made non-fatal",
            CANONICAL.replace(
                "      - run: python3 scripts/no-claude-coauthor-trailers.py --self-test\n      - env:\n          BEFORE:",
                "      - run: python3 scripts/no-claude-coauthor-trailers.py --self-test || true\n      - env:\n          BEFORE:",
            ),
            "conditional, non-fatal, or disarmed",
        ),
        (
            "the self-test dropped from the push job",
            CANONICAL.replace(
                "      - run: python3 scripts/no-claude-coauthor-trailers.py --self-test\n      - env:\n          BEFORE:",
                "      - env:\n          BEFORE:",
            ),
            "job `trailers-main` checks a range without running `--self-test` first",
        ),
        (
            "the self-test moved after the check, in the same step",
            CANONICAL.replace(
                "      - run: python3 scripts/no-claude-coauthor-trailers.py --self-test\n      - env:\n          BEFORE:",
                "      - env:\n          BEFORE:",
            )
            .replace(
                "          python3 scripts/no-claude-coauthor-trailers.py --range \"${BEFORE}..${AFTER}\"",
                "          python3 scripts/no-claude-coauthor-trailers.py --range \"${BEFORE}..${AFTER}\"\n          python3 scripts/no-claude-coauthor-trailers.py --self-test",
            ),
            "runs `--self-test` after the check",
        ),
        (
            "the event SHAs unbound from the step",
            CANONICAL.replace(
                "          BEFORE: ${{ github.event.before }}",
                "          BEFORE: ${{ github.event.pull_request.base.sha }}${{ github.event.before && '' }}",
            ),
            "job `trailers-main` binds `BEFORE` to",
        ),
        (
            "the PR body no longer checked",
            CANONICAL.replace(" --message-file \"$RUNNER_TEMP/pr-body.txt\"", ""),
            "without `--message-file`",
        ),
        (
            "fetch-depth dropped from the push job",
            CANONICAL.replace(
                "      - uses: actions/checkout@v6\n        with:\n          fetch-depth: 0\n      - run: python3 scripts/no-claude-coauthor-trailers.py --self-test\n      - env:\n          BEFORE:",
                "      - uses: actions/checkout@v6\n      - run: python3 scripts/no-claude-coauthor-trailers.py --self-test\n      - env:\n          BEFORE:",
            ),
            "without `actions/checkout` at `fetch-depth: 0`",
        ),
    ];

    for (what, mutated, expected) in mutations {
        assert_ne!(
            mutated, CANONICAL,
            "the `{what}` mutation rewrote nothing, so it proves nothing. The \
             fixture is CANONICAL in this file, not the live workflow, so this \
             is a fixture bug — but never make it pass by loosening `faults`."
        );
        let wiring = read_wiring(&mutated, "<mutated fixture>")
            .unwrap_or_else(|error| panic!("`{what}` produced unreadable YAML: {error}"));
        let found = faults(&wiring);
        let flat = |text: &str| text.split_whitespace().collect::<Vec<_>>().join(" ");
        let needle = flat(expected);
        assert!(
            found.iter().any(|fault| flat(fault).contains(&needle)),
            "`{what}` was not rejected for {needle:?}; faults were {found:#?}"
        );
    }
}

/// `on:` has three legal spellings and a bare `push:` means every branch. The
/// first version rejected the sequence form as a malformed workflow and
/// reported a bare `push:` as "does not cover `main` (covers [])", which was
/// false on a legal and strictly broader configuration.
#[test]
fn the_reader_accepts_every_legal_form_of_the_keys_it_reads() {
    let sequence = "on: [push, pull_request]\njobs:\n  a:\n    steps: []\n";
    let wiring = read_wiring(sequence, "<fixture>").expect("the sequence form of `on:` is legal");
    assert!(wiring.triggers.contains("push") && wiring.triggers.contains("pull_request"));

    let bare_push = "on:\n  push:\njobs:\n  a:\n    steps: []\n";
    let wiring = read_wiring(bare_push, "<fixture>").expect("a bare `push:` is legal");
    assert_eq!(wiring.push_branches, None, "no filter is every branch");
    assert!(
        !faults(&wiring)
            .iter()
            .any(|fault| fault.contains("does not cover `main`")),
        "a bare `push:` covers every branch, `main` included"
    );

    let scalar = "on:\n  push:\n    branches: main\njobs:\n  a:\n    steps: []\n";
    let wiring = read_wiring(scalar, "<fixture>").expect("a scalar `branches:` is legal");
    assert_eq!(
        wiring.push_branches.as_deref(),
        Some(&["main".to_owned()][..])
    );

    // Only a literal `false` is a promise; an expression is decided elsewhere.
    for (spelling, tolerated) in [
        ("false", false),
        ("\"false\"", false),
        ("true", true),
        ("${{ github.event_name == 'push' }}", true),
    ] {
        let source = format!(
            "on:\n  push:\njobs:\n  a:\n    continue-on-error: {spelling}\n    steps: []\n"
        );
        let wiring = read_wiring(&source, "<fixture>").expect("legal");
        assert_eq!(
            wiring.jobs[0].continue_on_error, tolerated,
            "continue-on-error: {spelling}"
        );
    }
}

/// The push step's shell, executed.
///
/// [`faults`] judges the workflow's *shape*; nothing above runs a line of the
/// step's 49-line body, which carries five refusal paths. Review found that gap
/// to be the largest one here, and pointed at the precedent: this gate's own
/// classifier proves itself over a throwaway repository
/// (`no-claude-coauthor-trailers.py::_end_to_end_control`), so the wiring can
/// too.
///
/// Hermetic on purpose — a throwaway repo, not this one. Pinning the real
/// escape by its SHA would make the test depend on clone depth, and the
/// regression is not "commit 0fe24d5c"; it is "a push whose added commits carry
/// the violation is refused, by this step, through its own shell". The offender
/// below therefore has a CLEAN message and a `Claude <noreply@anthropic.com>`
/// author, which is the shape that escaped: dc0092bb and 0fe24d5c both landed
/// after #412 taught the classifier authorship, so the failure was purely
/// wiring.
#[test]
fn the_push_step_shell_refuses_every_range_it_cannot_vouch_for() {
    use std::process::Command;

    // The step body, from the live workflow, uncommented and unmodified.
    let doc: serde_yaml::Value =
        serde_yaml::from_str(&live_source()).expect("the live workflow parses");
    let script = doc
        .get("jobs")
        .and_then(serde_yaml::Value::as_mapping)
        .expect("jobs")
        .iter()
        .filter(|(_, job)| job.get("if").map(verbatim).as_deref() == Some(PUSH_IF))
        .flat_map(|(_, job)| {
            job.get("steps")
                .and_then(serde_yaml::Value::as_sequence)
                .map(Vec::as_slice)
                .unwrap_or(&[])
        })
        .filter_map(|step| step.get("run").and_then(serde_yaml::Value::as_str))
        .find(|run| run.contains(PUSH_RANGE))
        .unwrap_or_else(|| {
            panic!("no push-gated step in {WORKFLOW} invokes the checker with {PUSH_RANGE}")
        })
        .to_owned();

    let repo = tempfile::tempdir().expect("tempdir");
    let root = repo.path();
    let git = |args: &[&str], env: &[(&str, &str)]| {
        let mut command = Command::new("git");
        command.args(args).current_dir(root);
        for (key, value) in env {
            command.env(key, value);
        }
        let output = command.output().expect("run git");
        assert!(
            output.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8_lossy(&output.stdout).trim().to_owned()
    };

    git(&["init", "--quiet", "-b", "main"], &[]);
    for (key, value) in [
        ("user.name", "gate probe"),
        ("user.email", "probe@example.invalid"),
        ("commit.gpgsign", "false"),
    ] {
        git(&["config", key, value], &[]);
    }
    std::fs::create_dir_all(root.join("scripts")).expect("scripts dir");
    std::fs::copy(repo_root().join(CHECKER), root.join(CHECKER)).expect("copy the checker");
    std::fs::write(root.join("step.sh"), &script).expect("write the step");

    let commit = |message: &str, env: &[(&str, &str)]| -> String {
        git(&["commit", "--quiet", "--allow-empty", "-m", message], env);
        git(&["rev-parse", "HEAD"], &[])
    };
    let base = commit("probe: base", &[]);
    let clean = commit("probe: an ordinary release bump", &[]);
    // A CLEAN message, authored by Claude — the shape that escaped.
    let offender = commit(
        "chore(release): v9.9.9 — nothing to attribute in this message",
        &[
            ("GIT_AUTHOR_NAME", "Claude"),
            ("GIT_AUTHOR_EMAIL", "noreply@anthropic.com"),
        ],
    );

    let run_step = |before: &str, after: &str| -> (bool, String) {
        let output = Command::new("bash")
            .arg("step.sh")
            .current_dir(root)
            .env("BEFORE", before)
            .env("AFTER", after)
            .output()
            .expect("run the step under bash");
        let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
        text.push_str(&String::from_utf8_lossy(&output.stderr));
        (output.status.success(), text)
    };

    let zero = "0".repeat(40);
    let missing = "dead".repeat(10);

    // The one path that may succeed, and it must say what it examined — a step
    // that exits 0 having checked nothing is the defect this gate exists for.
    let (ok, text) = run_step(&base, &clean);
    assert!(ok, "a clean fast-forward push must pass:\n{text}");
    assert!(
        text.contains("checking 1 commit(s) added by this push") && text.contains("ok:"),
        "the passing path must name what it checked:\n{text}"
    );

    for (what, before, after, expected) in [
        (
            "a clean message authored by Claude — the shape that escaped",
            clean.as_str(),
            offender.as_str(),
            "noreply@anthropic.com",
        ),
        (
            "an empty range, over which the checker would print ok",
            clean.as_str(),
            clean.as_str(),
            "contains no commits",
        ),
        (
            "a backwards push, whose range is empty while history moved",
            offender.as_str(),
            base.as_str(),
            "is not an ancestor of",
        ),
        (
            "the all-zeros sha of a branch creation or deletion",
            zero.as_str(),
            clean.as_str(),
            "all-zeros sha",
        ),
        (
            "a pre-push tip this checkout cannot resolve",
            missing.as_str(),
            clean.as_str(),
            "cannot resolve",
        ),
    ] {
        let (ok, text) = run_step(before, after);
        assert!(!ok, "the step must refuse {what}, but it exited 0:\n{text}");
        assert!(
            text.contains(expected),
            "refusing {what} must say why ({expected:?}):\n{text}"
        );
    }
}
