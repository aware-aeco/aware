//! Negative control for the compiler pin — `cli/rust-toolchain.toml`.
//!
//! CLAUDE.md §Tech stack pins the language at "Rust (edition 2024)" and
//! mechanises §Code style through `cargo fmt` and `cargo clippy -D warnings`,
//! "required to pass before merge". A lint gate only means something against a
//! *known* compiler, which is what `cli/rust-toolchain.toml` exists to fix
//! (#298): "Pin the compiler so a new stable can't introduce lints against
//! existing code with nothing to catch it."
//!
//! Nothing checked that the pin was actually reached. It is honoured two very
//! different ways, and only one of them is robust:
//!
//!   * a workflow step names it on `dtolnay/rust-toolchain`, which sets the
//!     default toolchain for the whole job; or
//!   * rustup finds a `rust-toolchain.toml` while resolving — and that lookup
//!     walks up from the **current directory**, nothing else.
//!
//! The second is an accident of `working-directory:`, and two jobs fell out of
//! it. Measured on a probe crate rather than assumed: `cargo build
//! --manifest-path <dir>/Cargo.toml` run from a directory with no toolchain file
//! ignores a `rust-toolchain.toml` sitting beside the manifest it was handed,
//! and builds on the default toolchain.
//!
//! What that cost, before this file:
//!
//!   * `ci.yml`'s `bridge-windows-packaged` restated `toolchain: 1.88.0` — seven
//!     minor versions behind the pin — and built the host with
//!     `--manifest-path cli/Cargo.toml` from the repo root, so 1.88.0 is the
//!     compiler that actually ran. The packaged harness exercised a binary no
//!     other job's compiler had ever produced, and any feature stabilised after
//!     1.88 would have failed there alone.
//!   * `release.yml` installed `dtolnay/rust-toolchain@stable` with no
//!     `toolchain:` input at all — the precise state #298 was raised to remove,
//!     still live while the pin file's own header described it in the past
//!     tense. Its CLI build masked this by setting `working-directory: cli`, so
//!     rustup silently overrode `@stable` back to the pin for that one step. The
//!     steel-detailer build runs from `20-agents/aeco/engineering/steel-detailer-lookup`,
//!     which carries no toolchain file, so the three binaries shipped in every
//!     release archive were compiled by floating stable.
//!
//! So the invariant asserted here is the robust one, and it is per job: every
//! job that invokes `cargo` installs the pinned toolchain, taking the version
//! from `cli/rust-toolchain.toml` rather than restating it. A job that does that
//! is correct from any directory, which is what makes the assertion independent
//! of where a future step happens to `cd`.
//!
//! Assertions over the real tree:
//!   * every workflow job that runs `cargo` has a `dtolnay/rust-toolchain` step,
//!     and it runs BEFORE the first cargo invocation — steps run in order, so a
//!     job that installs the pin after a build did not pin that build;
//!   * that step's `toolchain:` is a workflow expression, never a literal
//!     version and never left implicit on the action ref (`@stable`);
//!   * the line in that step which ASSIGNS `channel` reads
//!     `cli/rust-toolchain.toml`, and the step publishes `channel=$channel` to
//!     `$GITHUB_OUTPUT` — the command, never a mention of the path in a comment
//!     or an `::error::` message, both of which survive deleting the command
//!     they describe, and the parsed value, never a literal written to the
//!     right file;
//!   * no `rust-toolchain.toml` anywhere in the repo disagrees with the pin,
//!     which is the other way a directory-sensitive lookup can drift;
//!   * the pin names a concrete version rather than a moving channel.
//!
//! Each of those scans an artefact that is correct today — the real workflows,
//! the real pin file — so each would report clean both when it works and when it
//! has stopped matching anything at all. Every classifier is therefore also
//! driven over synthetic input (`the_channel_reader_matches_its_contract`,
//! `the_cargo_classifier_matches_its_contract`,
//! `the_pin_expression_classifier_matches_its_contract`,
//! `the_publish_check_requires_the_real_output_file`), the walk itself is driven
//! over a planted offender end to end
//! (`the_scan_reports_a_planted_literal_pin_by_job_and_value`), and the coverage
//! assertion names the jobs it found so a gate guarding an empty set fails
//! loudly (`the_scan_still_reaches_the_two_jobs_that_had_drifted`).
//!
//! The workflows are read with `serde_yaml`, not scraped. An earlier version
//! split on this repo's own indentation — job keys at exactly two spaces, steps
//! on `"\n      - "` — and a four-space workflow, which YAML permits and GitHub
//! runs, therefore produced no jobs and vanished from the scan in silence with
//! every test green (Codex review, PR #490). Neither coverage assertion caught
//! it: the file-count check passes because the file *was* read, and the
//! named-job check only asserts the four known jobs are still found. A gate that
//! scrapes text can be blind to a whole file while reporting that it read it.
//! `the_parser_reads_a_workflow_whatever_its_indentation` is the control.
//!
//! Known limits, stated plainly because a gate described as absolute gets
//! trusted like one:
//!   * it proves a job INSTALLS the pin, never that the compiler it names can
//!     build the crate;
//!   * its reach stops at `.github/workflows/` — a cargo invocation reached
//!     through a script the workflow calls is outside it;
//!   * it reads the `dtolnay/rust-toolchain` action specifically, so a different
//!     toolchain action would need teaching here.
//!
//! Pure file and string checks throughout — nothing here shells out to cargo or
//! rustup, so it costs nothing and cannot skip.

use std::path::{Path, PathBuf};

/// Repository root — `cli/`'s parent. The workflows and the pin live there.
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap_or_else(|| panic!("{} has no parent", env!("CARGO_MANIFEST_DIR")))
        .to_path_buf()
}

/// The `[toolchain] channel` of a `rust-toolchain.toml`, by the same rule the
/// workflows' `sed` applies: the first `channel = "…"` assignment.
///
/// Kept deliberately close to that expression — if the two disagree about what
/// the pin is, this gate is measuring something CI does not.
fn pinned_channel(toml: &str) -> Option<String> {
    toml.lines()
        .map(str::trim_start)
        .filter(|line| !line.starts_with('#'))
        .find_map(|line| {
            let rest = line.strip_prefix("channel")?.trim_start();
            let rest = rest.strip_prefix('=')?.trim_start();
            let rest = rest.strip_prefix('"')?;
            let end = rest.find('"')?;
            Some(rest[..end].to_string())
        })
}

/// One workflow job: its YAML key and its steps, in order.
struct Job {
    key: String,
    steps: Vec<Step>,
}

/// One step of a job.
///
/// Every field is taken from the parsed document rather than scraped out of
/// text, so none of them depends on how the file happens to be indented.
struct Step {
    /// The step's `name:`, for failure messages only — never scanned, since a
    /// name is prose and this repo has one reading "Cache cargo registry".
    name: String,
    /// The action ref, empty when the step has no `uses:`.
    uses: String,
    /// The step's `id:`, which is what `steps.<id>.…` resolves against.
    id: Option<String>,
    /// The shell of the step's `run:`, empty when it has none.
    run: String,
    /// `with.toolchain`, absent when the key is missing or resolves to null.
    toolchain: Option<String>,
}

/// Parse a workflow's jobs with a real YAML parser.
///
/// This used to split on fixed indentation — job keys at exactly two spaces,
/// steps on `"\n      - "` — copied from the sibling gates. YAML does not
/// promise either. A workflow indented four spaces under `jobs:` is perfectly
/// valid, produced no jobs at all under that rule, and so was dropped from the
/// scan in silence: measured before this rewrite, a four-space workflow whose
/// cargo job pinned `1.88.0` left all fourteen tests green (Codex review,
/// PR #490).
///
/// Neither coverage assertion caught it, which is the part worth remembering:
/// the file-count check passes because the file *was* read, and the named-job
/// check only asserts the four known jobs are still found. A gate that scrapes
/// text can be blind to a whole file while reporting that it read it.
///
/// `serde_yaml` is already a dependency of this crate, so this costs nothing
/// and removes the entire class.
fn jobs(workflow: &str) -> Vec<Job> {
    let doc: serde_yaml::Value = match serde_yaml::from_str(workflow) {
        Ok(doc) => doc,
        // A workflow this gate cannot parse is a hard failure, never a skip —
        // an unparseable file would otherwise vanish from the scan exactly the
        // way a four-space one used to.
        Err(e) => panic!("workflow is not valid YAML: {e}"),
    };
    let Some(jobs) = doc.get("jobs").and_then(serde_yaml::Value::as_mapping) else {
        return Vec::new();
    };
    jobs.iter()
        .map(|(key, job)| Job {
            key: scalar(key).unwrap_or_default(),
            steps: job
                .get("steps")
                .and_then(serde_yaml::Value::as_sequence)
                .map(|steps| steps.iter().map(step).collect())
                .unwrap_or_default(),
        })
        .collect()
}

/// One step of the parsed `steps:` sequence.
fn step(value: &serde_yaml::Value) -> Step {
    Step {
        name: value.get("name").and_then(scalar).unwrap_or_default(),
        uses: value.get("uses").and_then(scalar).unwrap_or_default(),
        id: value.get("id").and_then(scalar),
        run: value.get("run").and_then(scalar).unwrap_or_default(),
        // An empty value, `~` or `null` reads back as absent rather than as
        // `Some("")`. YAML resolves all three to null, the action then falls
        // back to its ref's default channel, and the compiler is unpinned — the
        // ABSENT case, not a restated one. Reporting it as ``pins `toolchain: `
        // literally`` names the wrong defect and points at the wrong fix.
        toolchain: value
            .get("with")
            .and_then(|with| with.get("toolchain"))
            .and_then(scalar)
            .filter(|value| !value.trim().is_empty()),
    }
}

/// A YAML scalar as the string a workflow author wrote.
///
/// Numbers are rendered rather than dropped: `toolchain: 1.88` parses as a
/// float, and a restated pin must be caught whether or not it happens to have
/// two dots in it.
fn scalar(value: &serde_yaml::Value) -> Option<String> {
    match value {
        serde_yaml::Value::String(s) => Some(s.clone()),
        serde_yaml::Value::Number(n) => Some(n.to_string()),
        serde_yaml::Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

/// The shell of a step's `run:`, taken from the parsed document.
///
/// Only `run:` content, never the whole step — a `name:` is prose, and this
/// repo has a step called "Cache cargo registry + build" that a scan of the
/// step text would read as a cargo invocation. The parser hands back the block
/// scalar already unindented and joined, so the `|` / `>` / `|-` / `>-` forms
/// and every indentation style come out identical here.
fn run_blocks(step: &Step) -> &str {
    &step.run
}

/// Does this shell text invoke `cargo`?
///
/// A `cargo` token in command position: preceded by nothing, whitespace, or a
/// shell separator, and followed by whitespace. That excludes the three
/// look-alikes this repo actually contains — `Cargo.toml`, `tag.yml`'s
/// `cargo_version=` shell variable, and `'cargo generate-lockfile'` quoted
/// inside an `echo` telling a human what to run.
fn invokes_cargo(shell: &str) -> bool {
    for line in shell.lines() {
        let line = line.trim_start();
        if line.starts_with('#') {
            continue;
        }
        for (idx, matched) in line.match_indices("cargo") {
            // `idx` comes from `match_indices`, so both slices are on char
            // boundaries even when the line holds non-ASCII (these workflows are
            // full of em-dashes).
            let before = line[..idx].chars().next_back();
            let after = line[idx + matched.len()..].chars().next();
            let opens = matches!(
                before,
                None | Some(' ' | '\t' | ';' | '&' | '|' | '(' | '{')
            );
            let closes = matches!(after, None | Some(' ' | '\t'));
            if opens && closes {
                return true;
            }
        }
    }
    false
}

/// The `toolchain:` input a step passes, if it passes a usable one.
///
/// Read off the parsed step; the absent/null/empty collapse happens in `step`.
fn toolchain_input(step: &Step) -> Option<String> {
    step.toolchain.clone()
}

/// The line of a shell script that ASSIGNS the `channel` variable.
///
/// Command position only — a line whose first token is `channel=`. That is what
/// separates the assignment producing the value from the two lines that merely
/// mention it: `echo "channel=$channel" >> "$GITHUB_OUTPUT"`, which emits it,
/// and the `::error::` message, which names the pin file in prose.
///
/// The distinction is load-bearing and was measured, not argued. The check this
/// feeds used to be `run_blocks(..).contains("cli/rust-toolchain.toml")`, and
/// `ci.yml`'s pin step names that path in its error message as well as in its
/// `sed` — so the assertion was already vacuous with respect to the command it
/// was meant to guard. Replacing the `sed` with `channel=stable` while leaving
/// the error text alone kept all eleven tests green (Codex review, PR #490;
/// reproduced against the real `ci.yml` before this fix).
fn channel_assignment(shell: &str) -> Option<&str> {
    shell
        .lines()
        .map(str::trim)
        .filter(|line| !line.starts_with('#'))
        .find(|line| line.starts_with("channel="))
}

/// Does this script publish `channel` as a step output?
///
/// The other half of the contract: the expression the action reads is
/// `steps.<id>.outputs.channel`, so a step that computes the pin correctly and
/// never writes it to `$GITHUB_OUTPUT` hands the action an empty toolchain.
///
/// Three things are checked, and each was a hole the previous version had
/// (Codex review, PR #490, rounds two and three):
///
///   * the redirection TARGET is `$GITHUB_OUTPUT` itself, not merely a word
///     containing it — `>> "${GITHUB_OUTPUT}.bak"` publishes nothing;
///   * the key written is `channel`, the one the action's expression reads;
///   * the VALUE is the `$channel` the script computed. Without this last one
///     `echo "channel=stable" >> "$GITHUB_OUTPUT"` passes while the `sed` above
///     it still reads the pin: the assignment check is satisfied, the publish
///     check is satisfied, and the action is handed a moving channel.
fn emits_channel_output(shell: &str) -> bool {
    publish_problem(shell).is_none()
}

/// Why this script does not publish the parsed channel, or `None` if it does.
///
/// The two failures are different mistakes with different fixes, so they get
/// different sentences. Reporting "never writes to $GITHUB_OUTPUT" for a step
/// that writes `channel=stable` to exactly the right file sends the reader
/// looking for the wrong thing.
fn publish_problem(shell: &str) -> Option<&'static str> {
    let mut redirects = false;
    for line in shell
        .lines()
        .map(str::trim)
        .filter(|line| !line.starts_with('#'))
    {
        if !line.split(">>").skip(1).any(redirects_to_github_output) {
            continue;
        }
        redirects = true;
        if emits_parsed_channel(line) {
            return None;
        }
    }
    if redirects {
        Some(
            "it writes to $GITHUB_OUTPUT but not `channel=$channel` — the value \
             published is not the one the script parsed, so the action is handed \
             a channel nobody read out of the pin file",
        )
    } else {
        Some(
            "it never writes `channel=` to $GITHUB_OUTPUT, so the action receives \
             an empty toolchain and falls back to its ref's default",
        )
    }
}

/// Does this line publish `channel=` with the value the script parsed?
///
/// The assignment is matched as a whole word, so the KEY has to be `channel`
/// and not merely end with it. A substring search finds the `channel=` inside
/// `toolchain_channel=$channel` and reads the right value out of it, so the
/// publish looks correct here while GitHub publishes `toolchain_channel` and
/// leaves `steps.<id>.outputs.channel` — the property the action's expression
/// actually reads — empty (Codex review, PR #490).
fn emits_parsed_channel(line: &str) -> bool {
    let emitted = line.split(">>").next().unwrap_or(line);
    emitted
        .split_whitespace()
        .filter_map(|word| unquote(word).strip_prefix("channel="))
        .any(|value| {
            // Exactly the variable, not merely a prefix of one. `$channels` and
            // `$channel_override` both start with `$channel` and are different
            // variables — usually unset, so the step publishes an empty value,
            // the action falls back to its ref's default, and this gate reports
            // success (Codex review, PR #490). Shell quoting is stripped; the
            // name is not.
            let value = unquote(value);
            value == "$channel" || value == "${channel}"
        })
}

/// A shell word with its surrounding quotes removed.
fn unquote(word: &str) -> &str {
    word.trim_matches(|c| c == '"' || c == '\'')
}

/// Is this the target of a `>>` redirection to `$GITHUB_OUTPUT` itself?
///
/// The first whitespace-delimited word after the operator, stripped of quotes,
/// must be exactly the variable — `${GITHUB_OUTPUT}.bak` is a different file.
fn redirects_to_github_output(target: &str) -> bool {
    let Some(word) = target.split_whitespace().next() else {
        return false;
    };
    let word = unquote(word);
    word == "$GITHUB_OUTPUT" || word == "${GITHUB_OUTPUT}"
}

/// Does this `toolchain:` value read the pin rather than restate it?
///
/// A workflow expression referring to a step output. A literal (`1.88.0`) or a
/// moving channel (`stable`) is exactly the drift this gate exists to catch, and
/// so is an expression pointing at an input or a variable rather than a step.
fn reads_a_step_output(value: &str) -> bool {
    referenced_step_id(value).is_some()
}

/// The step id in an expression reading exactly `steps.<id>.outputs.channel`.
///
/// The path is matched segment by segment rather than by prefix and suffix.
/// `${{ steps.pin.typo.outputs.channel }}` starts with `steps.` and ends with
/// `.outputs.channel`, so a prefix/suffix test accepts it and reports `pin` as
/// the producer — but the expression reads a property that step never published,
/// the action receives an empty toolchain, and the compiler is unpinned with
/// every other assertion here green (Codex review, PR #490).
fn referenced_step_id(value: &str) -> Option<String> {
    let inner = value.strip_prefix("${{")?.strip_suffix("}}")?.trim();
    let mut segments = inner.strip_prefix("steps.")?.split('.');
    let id = segments.next()?;
    if id.is_empty() || segments.next()? != "outputs" || segments.next()? != "channel" {
        return None;
    }
    // `steps.pin.outputs.channel.extra` is not the path either.
    if segments.next().is_some() {
        return None;
    }
    Some(id.to_string())
}

/// Every workflow under `.github/workflows/`, as (name, text).
///
/// Both extensions. GitHub reads `.yml` and `.yaml` alike, so scanning only
/// `.yml` would let a cargo-building workflow added as `.yaml` restate or omit
/// the pin with this gate still green (Codex review, PR #490). The tree happens
/// to use `.yml` throughout today, which is exactly why the filter has to be
/// about what GitHub accepts rather than about what is currently there.
///
/// A file that cannot be read is a hard failure rather than a skip: silently
/// dropping a workflow from the scan would shrink the gate's coverage with no
/// signal, which is the failure mode this whole file exists to prevent.
fn workflows() -> Vec<(String, String)> {
    let dir = repo_root().join(".github/workflows");
    let mut found: Vec<(String, String)> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
        .filter_map(|entry| {
            let path = entry
                .unwrap_or_else(|e| panic!("read a dir entry: {e}"))
                .path();
            let ext = path.extension()?.to_string_lossy().into_owned();
            if ext != "yml" && ext != "yaml" {
                return None;
            }
            let name = path.file_name()?.to_string_lossy().into_owned();
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
            Some((name, text))
        })
        .collect();
    found.sort_by(|a, b| a.0.cmp(&b.0));
    assert!(
        !found.is_empty(),
        "no workflows found under {} — this gate is scanning nothing",
        dir.display()
    );
    found
}

/// The pin every job must reach.
fn pin() -> String {
    let path = repo_root().join("cli/rust-toolchain.toml");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    pinned_channel(&text).unwrap_or_else(|| {
        panic!(
            "{} has no `[toolchain] channel` — CI's own `sed` reads the same \
             assignment and would fail with `could not read [toolchain] channel`",
            path.display()
        )
    })
}

/// A job that runs cargo, with the toolchain step it installs (if any).
///
/// Step POSITIONS are carried, not just their contents. Steps run in order and
/// the action sets the job's default toolchain when it runs, so an install that
/// happens after a build did not pin that build — and a check that merely asks
/// "does this job contain both" cannot tell the difference (Codex review,
/// PR #490).
struct CargoJob {
    workflow: String,
    key: String,
    /// (position, action ref, `toolchain:` input) for each toolchain step.
    toolchain_steps: Vec<(usize, String, Option<String>)>,
    /// (position, `id:`) for each step that declares one. Positional for the
    /// same reason as `toolchain_steps`: a step's outputs do not exist until it
    /// has run, so an `id` that appears *after* the action reading it is not
    /// available to that action (Codex review, PR #490).
    step_ids: Vec<(usize, String)>,
    /// Position of the first step that invokes cargo.
    first_cargo: usize,
    /// The name of that step, so a failure can point at it.
    first_cargo_name: String,
}

/// Every job across `workflows` that invokes cargo.
fn cargo_jobs(workflows: &[(String, String)]) -> Vec<CargoJob> {
    let mut found = Vec::new();
    for (name, text) in workflows {
        for job in jobs(text) {
            let Some((first_cargo, cargo_step)) = job
                .steps
                .iter()
                .enumerate()
                .find(|(_, step)| invokes_cargo(run_blocks(step)))
            else {
                continue;
            };
            let toolchain_steps = job
                .steps
                .iter()
                .enumerate()
                .filter(|(_, step)| step.uses.starts_with("dtolnay/rust-toolchain"))
                .map(|(at, step)| (at, step.uses.clone(), toolchain_input(step)))
                .collect();
            let step_ids = job
                .steps
                .iter()
                .enumerate()
                .filter_map(|(at, step)| step.id.clone().map(|id| (at, id)))
                .collect();
            found.push(CargoJob {
                workflow: name.clone(),
                key: job.key.clone(),
                toolchain_steps,
                step_ids,
                first_cargo,
                first_cargo_name: cargo_step.name.clone(),
            });
        }
    }
    found
}

// ---------------------------------------------------------------------------
// Assertions over the real tree
// ---------------------------------------------------------------------------

#[test]
fn every_cargo_job_installs_the_pin_and_none_restates_it() {
    let pin = pin();
    let jobs = cargo_jobs(&workflows());

    for job in &jobs {
        let where_ = format!("{}: job `{}`", job.workflow, job.key);
        assert!(
            !job.toolchain_steps.is_empty(),
            "{where_} runs cargo but installs no Rust toolchain, so it builds on \
             whatever the runner ships. Add a `Read pinned toolchain` step and a \
             `dtolnay/rust-toolchain@master` step taking its output, as \
             `ci.yml`'s `gates` job does."
        );

        // Order, not just presence. The action sets the job's default toolchain
        // when it runs, so an install placed after a build did not pin that
        // build. Moving `Install Rust` below the steel-detailer build in
        // `release.yml` would leave those shipped binaries on the runner's
        // default compiler — the very defect this PR fixes — and a
        // contains-both check cannot see it (Codex review, PR #490).
        let first_install = job
            .toolchain_steps
            .iter()
            .map(|(at, _, _)| *at)
            .min()
            .unwrap_or(usize::MAX);
        assert!(
            first_install < job.first_cargo,
            "{where_} installs its Rust toolchain at step {first_install}, but \
             already invokes cargo at step {} (`{}`). Steps run in order, so that \
             cargo call uses whatever compiler the runner shipped, not the pin. \
             Move the toolchain install above it.",
            job.first_cargo,
            job.first_cargo_name
        );

        for (consumer_at, uses, input) in &job.toolchain_steps {
            let Some(value) = input else {
                panic!(
                    "{where_} installs `{uses}` with no `toolchain:` input, so the \
                     compiler is whatever the action ref resolves to that day — the \
                     exact condition the pin in cli/rust-toolchain.toml was added to \
                     remove (#298). Pass `toolchain: ${{{{ steps.<id>.outputs.channel }}}}`."
                );
            };
            // Two different mistakes, so two different sentences. Restating the
            // version is the drift this gate exists to catch; an expression that
            // names a step but reads the wrong property off it is a typo that
            // silently yields an empty toolchain. Reporting the first for the
            // second sends the reader looking for a hard-coded version that is
            // not there.
            let Some(id) = referenced_step_id(value) else {
                let names_a_step = value
                    .strip_prefix("${{")
                    .and_then(|inner| inner.strip_suffix("}}"))
                    .is_some_and(|inner| inner.trim().starts_with("steps."));
                if names_a_step {
                    panic!(
                        "{where_} reads `toolchain: {value}`, which is not \
                         `${{{{ steps.<id>.outputs.channel }}}}`. The expression \
                         resolves to the empty string, so `{uses}` falls back to \
                         its ref's default channel rather than the pin."
                    );
                }
                panic!(
                    "{where_} pins `toolchain: {value}` literally. The pin lives in \
                     cli/rust-toolchain.toml and must be read, never restated — a \
                     restated version drifts silently, which is how this job came to \
                     build on 1.88.0 while the pin said {pin}."
                );
            };
            let producer_at = job
                .step_ids
                .iter()
                .find(|(_, declared)| *declared == id)
                .map(|(at, _)| *at);
            let Some(producer_at) = producer_at else {
                panic!(
                    "{where_} reads `{value}`, but no step in that job declares \
                     `id: {id}`. Step ids present: {:?}",
                    job.step_ids
                );
            };
            // Presence is not enough: the producer must have RUN. `steps.<id>.
            // outputs.*` is resolved when the consuming step is evaluated, so an
            // id declared further down the job resolves to the empty string, the
            // action falls back to its ref's default channel, and the compiler is
            // unpinned with every other assertion here green. Moving `Read pinned
            // toolchain` below `Install Rust` — both still above the build — is
            // the case that passed before this check existed (Codex review,
            // PR #490).
            assert!(
                producer_at < *consumer_at,
                "{where_} reads `{value}` at step {consumer_at}, but the step \
                 declaring `id: {id}` is step {producer_at} — at or after the one \
                 consuming it. A step's outputs do not exist until it has run, so \
                 `{uses}` would receive an empty toolchain and fall back to its \
                 ref's default channel. Move `id: {id}` above step {consumer_at}."
            );
        }
    }
}

#[test]
fn the_step_the_pin_is_read_from_names_the_pin_file() {
    for (name, text) in workflows() {
        for job in jobs(&text) {
            let ids: Vec<String> = job
                .steps
                .iter()
                .filter(|step| step.uses.starts_with("dtolnay/rust-toolchain"))
                .filter_map(toolchain_input)
                .filter_map(|value| referenced_step_id(&value))
                .collect();
            for id in ids {
                let source = job
                    .steps
                    .iter()
                    .find(|step| step.id.as_deref() == Some(id.as_str()))
                    .unwrap_or_else(|| {
                        panic!("{name}: job `{}` has no step with `id: {id}`", job.key)
                    });
                let run = run_blocks(source);
                let assignment = channel_assignment(run).unwrap_or_else(|| {
                    panic!(
                        "{name}: job `{}` takes its toolchain from step `{id}`, but that \
                         step's script never assigns `channel=` — nothing in it \
                         computes the value the action is handed. Script:\n{run}",
                        job.key
                    )
                });
                assert!(
                    assignment.contains("cli/rust-toolchain.toml"),
                    "{name}: job `{}` takes its toolchain from step `{id}`, but the line \
                     that assigns it — `{assignment}` — does not read \
                     cli/rust-toolchain.toml, so the version it emits is not the pin. \
                     A mention of the path elsewhere in the script (an `::error::` \
                     message, a comment) is prose and does not count.",
                    job.key
                );
                if let Some(problem) = publish_problem(run) {
                    panic!(
                        "{name}: job `{}` reads `steps.{id}.outputs.channel`, but {problem}. \
                         Script:\n{run}",
                        job.key
                    );
                }
            }
        }
    }
}

#[test]
fn the_scan_still_reaches_the_two_jobs_that_had_drifted() {
    let jobs = cargo_jobs(&workflows());
    let found: Vec<String> = jobs
        .iter()
        .map(|job| format!("{}:{}", job.workflow, job.key))
        .collect();

    // Named rather than counted. Both of these built Rust on an unpinned or
    // stale compiler, and both were invisible because neither job's name says
    // "Rust". If either is renamed this fails and a human decides whether the
    // coverage moved with it — which is the point of naming them.
    for expected in [
        "ci.yml:gates",
        "ci.yml:gates-macos",
        "ci.yml:bridge-windows-packaged",
        "release.yml:build",
    ] {
        assert!(
            found.iter().any(|job| job == expected),
            "the scan no longer sees `{expected}` as a cargo-invoking job, so the \
             pin is no longer being checked there. Jobs found: {found:?}"
        );
    }
}

#[test]
fn no_toolchain_file_in_the_repo_disagrees_with_the_pin() {
    let pin = pin();
    let root = repo_root();
    let mut found = Vec::new();
    collect_toolchain_files(&root, &mut found);

    assert!(
        found
            .iter()
            .any(|path| path.ends_with("cli/rust-toolchain.toml")),
        "cli/rust-toolchain.toml was not found by the walk — the scan is broken, \
         not the tree. Found: {found:?}"
    );

    for path in &found {
        let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
        let channel = pinned_channel(&text)
            .unwrap_or_else(|| panic!("{} declares no [toolchain] channel", path.display()));
        assert_eq!(
            channel,
            pin,
            "{} pins {channel}, but cli/rust-toolchain.toml pins {pin}. rustup \
             resolves a toolchain file from the CURRENT DIRECTORY, so two files \
             disagreeing means the compiler depends on which directory a step \
             happens to run from.",
            path.display()
        );
    }
}

#[test]
fn the_pin_names_a_concrete_version_not_a_moving_channel() {
    let pin = pin();
    assert!(
        pin.split('.').count() == 3 && pin.split('.').all(|part| !part.is_empty()),
        "cli/rust-toolchain.toml pins `{pin}`, which is not an x.y.z version. A \
         moving channel is what #298 removed: it makes every stable release an \
         unreviewed change to this repo's compiler."
    );
    assert!(
        pin.split('.')
            .all(|part| part.chars().all(|c| c.is_ascii_digit())),
        "cli/rust-toolchain.toml pins `{pin}`, which is not a numeric version"
    );
}

/// Walk for `rust-toolchain.toml` / `rust-toolchain`, skipping build output and
/// vendored trees.
fn collect_toolchain_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if path.is_dir() {
            if matches!(
                name.as_str(),
                "target" | "node_modules" | ".git" | "bin" | "obj"
            ) {
                continue;
            }
            collect_toolchain_files(&path, out);
        } else if name == "rust-toolchain.toml" || name == "rust-toolchain" {
            out.push(path);
        }
    }
}

// ---------------------------------------------------------------------------
// Negative controls — every classifier above, driven over synthetic input
// ---------------------------------------------------------------------------

#[test]
fn the_channel_reader_matches_its_contract() {
    for (input, expected) in [
        ("[toolchain]\nchannel = \"1.95.0\"\n", Some("1.95.0")),
        ("[toolchain]\nchannel=\"1.95.0\"", Some("1.95.0")),
        ("  channel  =  \"stable\"  ", Some("stable")),
        // A commented-out pin is not a pin. This is the shape a "temporarily
        // bump the compiler" edit leaves behind.
        (
            "# channel = \"1.95.0\"\nchannel = \"1.90.0\"",
            Some("1.90.0"),
        ),
        ("# channel = \"1.95.0\"\n", None),
        ("[toolchain]\ncomponents = [\"clippy\"]\n", None),
        ("channel = 1.95.0\n", None),
        ("", None),
    ] {
        assert_eq!(
            pinned_channel(input).as_deref(),
            expected,
            "channel reader disagreed on {input:?}"
        );
    }
}

#[test]
fn the_cargo_classifier_matches_its_contract() {
    for (input, expected) in [
        ("cargo build --release", true),
        ("  cargo test --locked", true),
        ("if ! (cd \"$crate\" && cargo metadata); then", true),
        ("VERSION=$(cargo pkgid)", true),
        ("cargo", true),
        // The three look-alikes this repo actually contains. Each would make
        // the gate fire on a job that runs no Rust at all.
        (
            "cargo_version=\"$(git show \"$SHA:cli/Cargo.toml\")\"",
            false,
        ),
        ("echo \"run 'cargo generate-lockfile' in $crate\"", false),
        ("grep -m1 '^version' cli/Cargo.toml", false),
        ("# cargo build --release", false),
        ("dotnet publish cli-sidecar", false),
        ("", false),
    ] {
        assert_eq!(
            invokes_cargo(input),
            expected,
            "cargo classifier disagreed on {input:?}"
        );
    }
}

#[test]
fn the_pin_expression_classifier_matches_its_contract() {
    for (input, expected) in [
        ("${{ steps.pin.outputs.channel }}", true),
        ("${{steps.pin.outputs.channel}}", true),
        ("${{ steps.read-the-pin.outputs.channel }}", true),
        // Every way of not reading the pin, including the two that shipped.
        ("1.88.0", false),
        ("stable", false),
        ("nightly", false),
        ("${{ inputs.toolchain }}", false),
        ("${{ env.RUST_VERSION }}", false),
        ("${{ matrix.toolchain }}", false),
        ("${{ steps.pin.outputs.version }}", false),
        // A property path that is not `steps.<id>.outputs.channel`. Each starts
        // with `steps.` and the first ends with `.outputs.channel`, so a
        // prefix/suffix test accepts it while the expression reads something the
        // producer never published — an empty toolchain, silently (Codex review,
        // PR #490).
        ("${{ steps.pin.typo.outputs.channel }}", false),
        ("${{ steps.pin.outputs.channel.extra }}", false),
        ("${{ steps.pin.outputs }}", false),
        ("${{ steps.pin }}", false),
        ("${{ steps..outputs.channel }}", false),
        ("${{ steps. }}", false),
        ("", false),
    ] {
        assert_eq!(
            reads_a_step_output(input),
            expected,
            "pin-expression classifier disagreed on {input:?}"
        );
    }
    assert_eq!(
        referenced_step_id("${{ steps.pin.outputs.channel }}").as_deref(),
        Some("pin")
    );
    assert_eq!(
        referenced_step_id("${{ steps.read-the-pin.outputs.channel }}").as_deref(),
        Some("read-the-pin"),
        "a hyphenated id is a valid step id and must still be read"
    );
    assert_eq!(referenced_step_id("1.88.0"), None);
    // The id reader and the acceptance test must agree, or a rejected
    // expression could still name a producer the ordering checks then trust.
    assert_eq!(
        referenced_step_id("${{ steps.pin.typo.outputs.channel }}"),
        None,
        "a wrong property path must yield no step id, not `pin`"
    );
}

#[test]
fn the_channel_assignment_reader_separates_the_command_from_the_prose() {
    // The real pin step in shape: the path appears three times and only ONE of
    // those lines computes anything.
    let real = r#"# POSIX sed, not `grep -oP` — cli/rust-toolchain.toml
channel=$(sed -n 's/^channel = "\(.*\)"/\1/p' cli/rust-toolchain.toml | head -1)
if [ -z "$channel" ]; then
  echo "::error::could not read [toolchain] channel from cli/rust-toolchain.toml"
  exit 1
fi
echo "channel=$channel" >> "$GITHUB_OUTPUT"
"#;
    let found = channel_assignment(real).expect("the real step assigns channel");
    assert!(
        found.starts_with("channel=$(sed"),
        "picked the wrong line as the assignment: {found:?}"
    );
    assert!(found.contains("cli/rust-toolchain.toml"));
    assert!(emits_channel_output(real));

    // Codex's scenario, PR #490: hard-code the channel, keep every mention of
    // the pin file. A `contains` over the whole block accepts this.
    let gutted = r#"# was: sed ... cli/rust-toolchain.toml
channel=stable
if [ -z "$channel" ]; then
  echo "::error::could not read [toolchain] channel from cli/rust-toolchain.toml"
  exit 1
fi
echo "channel=$channel" >> "$GITHUB_OUTPUT"
"#;
    assert!(
        gutted.contains("cli/rust-toolchain.toml"),
        "the planted script must still MENTION the pin file — otherwise this \
         proves nothing about prose being rejected"
    );
    let found = channel_assignment(gutted).expect("the planted step assigns channel");
    assert_eq!(found, "channel=stable");
    assert!(
        !found.contains("cli/rust-toolchain.toml"),
        "the gutted assignment must not read the pin file — this is the exact \
         comparison that now fails the gate"
    );

    // Computes the pin, never publishes it: the action gets an empty input.
    let unpublished = "channel=$(sed -n 's/x/y/p' cli/rust-toolchain.toml)\necho \"$channel\"\n";
    assert!(channel_assignment(unpublished).is_some());
    assert!(
        !emits_channel_output(unpublished),
        "a step that never writes to $GITHUB_OUTPUT must be reported"
    );

    // No assignment at all — an emit is not an assignment, nor is a comment.
    assert_eq!(channel_assignment("echo \"channel=1.95.0\"\n"), None);
    assert_eq!(channel_assignment("# channel=1.95.0\n"), None);
    assert_eq!(channel_assignment(""), None);
}

/// One step, parsed out of a one-job workflow, for the classifier tests below.
fn only_step(step_yaml: &str) -> Step {
    let workflow = format!("jobs:\n  j:\n    steps:\n      - {step_yaml}");
    let mut jobs = jobs(&workflow);
    assert_eq!(jobs.len(), 1, "probe workflow did not parse to one job");
    let mut job = jobs.remove(0);
    assert_eq!(
        job.steps.len(),
        1,
        "probe workflow did not parse to one step"
    );
    job.steps.remove(0)
}

#[test]
fn an_empty_toolchain_input_reads_as_absent_not_as_a_literal() {
    // YAML resolves all three to null; the action then falls back to its ref's
    // default channel. That is the unpinned case, and reporting it as a restated
    // literal names the wrong defect and sends the reader to the wrong fix.
    for empty in [
        "uses: a@v1\n        with:\n          toolchain:",
        "uses: a@v1\n        with:\n          toolchain: ~",
        "uses: a@v1\n        with:\n          toolchain: null",
        "uses: a@v1\n        with:\n          toolchain: \"\"",
    ] {
        assert_eq!(
            toolchain_input(&only_step(empty)),
            None,
            "an empty toolchain input must read as absent: {empty:?}"
        );
    }
    assert_eq!(
        toolchain_input(&only_step(
            "uses: a@v1\n        with:\n          toolchain: 1.88.0"
        ))
        .as_deref(),
        Some("1.88.0")
    );
    // `1.88` parses as a FLOAT, not a string. A restated pin must be caught
    // whether or not it happens to carry two dots.
    assert_eq!(
        toolchain_input(&only_step(
            "uses: a@v1\n        with:\n          toolchain: 1.88"
        ))
        .as_deref(),
        Some("1.88")
    );
    assert_eq!(
        toolchain_input(&only_step(
            "uses: a@v1\n        with:\n          components: clippy"
        )),
        None
    );
    assert_eq!(toolchain_input(&only_step("uses: a@v1")), None);
}

#[test]
fn the_install_must_come_before_the_cargo_call() {
    // Steps run in order, so a job containing both an install and a build is
    // not thereby pinned. Codex's case: move `Install Rust` below the build in
    // `release.yml` and those shipped binaries are on the runner's default
    // compiler, with a contains-both check none the wiser (PR #490).
    let after = "jobs:\n  build:\n    steps:\n      - name: Build\n        run: cargo build --release\n      - name: Install Rust\n        uses: dtolnay/rust-toolchain@master\n        with:\n          toolchain: ${{ steps.pin.outputs.channel }}\n";
    let found = cargo_jobs(&[("after.yml".to_string(), after.to_string())]);
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].first_cargo, 0, "the cargo step is the first step");
    assert_eq!(
        found[0].toolchain_steps[0].0, 1,
        "the install is the second step, i.e. too late"
    );
    assert!(
        found[0].toolchain_steps[0].0 > found[0].first_cargo,
        "this ordering is exactly what the assertion must reject"
    );
    assert_eq!(found[0].first_cargo_name, "Build");

    // The same two steps the right way round.
    let before = "jobs:\n  build:\n    steps:\n      - name: Install Rust\n        uses: dtolnay/rust-toolchain@master\n        with:\n          toolchain: ${{ steps.pin.outputs.channel }}\n      - name: Build\n        run: cargo build --release\n";
    let found = cargo_jobs(&[("before.yml".to_string(), before.to_string())]);
    assert_eq!(found.len(), 1);
    assert!(
        found[0].toolchain_steps[0].0 < found[0].first_cargo,
        "the correct ordering must be accepted, or the assertion rejects \
         everything and proves nothing"
    );
}

#[test]
fn the_publish_check_requires_the_parsed_channel_not_a_literal() {
    // The gap left after round two: the assignment reads the pin, the redirect
    // target is right, and the step still publishes a hard-coded channel. Every
    // other check passes and the action is handed `stable` (Codex review,
    // PR #490).
    let gutted_publish = "channel=$(sed -n 's/x/y/p' cli/rust-toolchain.toml | head -1)\necho \"channel=stable\" >> \"$GITHUB_OUTPUT\"\n";
    assert!(
        channel_assignment(gutted_publish)
            .expect("assignment present")
            .contains("cli/rust-toolchain.toml"),
        "the assignment half must still pass — otherwise this proves nothing \
         about the publish half"
    );
    assert!(
        !emits_channel_output(gutted_publish),
        "a publish of a hard-coded channel must be rejected even though the \
         assignment above it reads the pin"
    );

    // A variable whose name merely BEGINS with `channel` is a different
    // variable. Both of these are typically unset, so the step publishes an
    // empty value and the action falls back to its ref's default — while a
    // `starts_with("$channel")` test reports success (Codex review, PR #490).
    for near_miss in [
        "channel=$(sed -n 's/x/y/p' cli/rust-toolchain.toml)\necho \"channel=$channels\" >> \"$GITHUB_OUTPUT\"\n",
        "channel=$(sed -n 's/x/y/p' cli/rust-toolchain.toml)\necho \"channel=$channel_override\" >> \"$GITHUB_OUTPUT\"\n",
        "channel=$(sed -n 's/x/y/p' cli/rust-toolchain.toml)\necho \"channel=${channel}x\" >> \"$GITHUB_OUTPUT\"\n",
    ] {
        assert!(
            !emits_channel_output(near_miss),
            "a variable that is not `$channel` must be rejected: {near_miss:?}"
        );
    }

    // An output key that merely ENDS with `channel` is a different key. The
    // value published is the parsed one, and the redirection target is right,
    // but GitHub publishes `toolchain_channel` and the action's
    // `steps.<id>.outputs.channel` resolves to empty — so it falls back to its
    // ref's default while a substring search reports success (Codex review,
    // PR #490).
    for renamed_key in [
        "channel=$(sed -n 's/x/y/p' cli/rust-toolchain.toml)\necho \"toolchain_channel=$channel\" >> \"$GITHUB_OUTPUT\"\n",
        "channel=$(sed -n 's/x/y/p' cli/rust-toolchain.toml)\necho \"rust_channel=${channel}\" >> \"$GITHUB_OUTPUT\"\n",
    ] {
        assert!(
            !emits_channel_output(renamed_key),
            "an output key that is not `channel` must be rejected: {renamed_key:?}"
        );
    }

    // The real form, and the braced variable. Without these the check could
    // reject everything and prove nothing.
    assert!(emits_channel_output(
        "echo \"channel=$channel\" >> \"$GITHUB_OUTPUT\"\n"
    ));
    assert!(emits_channel_output(
        "echo \"channel=${channel}\" >> \"$GITHUB_OUTPUT\"\n"
    ));
    // The unquoted form is the same publish.
    assert!(emits_channel_output(
        "echo channel=$channel >> $GITHUB_OUTPUT\n"
    ));
}

#[test]
fn the_publish_check_requires_the_real_output_file() {
    // Both substrings on one line is not enough: this writes a DIFFERENT file
    // and publishes nothing, so the action falls back to its default — the
    // regression the assertion exists to prevent (Codex review, PR #490).
    assert!(!emits_channel_output(
        "echo \"channel=$channel\" >> \"${GITHUB_OUTPUT}.bak\"\n"
    ));
    assert!(!emits_channel_output(
        "echo \"channel=$channel\" >> /tmp/GITHUB_OUTPUT\n"
    ));
    assert!(!emits_channel_output("echo \"channel=$channel\"\n"));
    // The forms the real workflows use, and the quoting variants around them.
    for good in [
        "echo \"channel=$channel\" >> \"$GITHUB_OUTPUT\"\n",
        "echo \"channel=$channel\" >> $GITHUB_OUTPUT\n",
        "echo \"channel=$channel\" >> \"${GITHUB_OUTPUT}\"\n",
        "echo \"channel=$channel\" >>\"$GITHUB_OUTPUT\"\n",
    ] {
        assert!(
            emits_channel_output(good),
            "a real publish was rejected: {good:?}"
        );
    }
}

#[test]
fn the_workflow_scan_covers_both_extensions_github_accepts() {
    // Not a style point: GitHub runs `.yaml` exactly as it runs `.yml`, so a
    // scan narrowed to one of them is a hole a new file walks straight through
    // (Codex review, PR #490).
    let dir = repo_root().join(".github/workflows");
    let on_disk: Vec<String> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
        .filter_map(|entry| Some(entry.ok()?.file_name().to_string_lossy().into_owned()))
        .filter(|name| name.ends_with(".yml") || name.ends_with(".yaml"))
        .collect();
    let scanned: Vec<String> = workflows().into_iter().map(|(name, _)| name).collect();
    assert_eq!(
        scanned.len(),
        on_disk.len(),
        "the scan reached {} of the {} workflow files on disk. Scanned: {scanned:?}; \
         on disk: {on_disk:?}",
        scanned.len(),
        on_disk.len()
    );
}

#[test]
fn the_run_block_reader_takes_scripts_and_not_step_names() {
    let named = only_step(
        "name: Cache cargo registry + build\n        uses: Swatinem/rust-cache@v2\n        with:\n          workspaces: cli",
    );
    assert!(
        !invokes_cargo(run_blocks(&named)),
        "a step NAMED `Cache cargo registry + build` was read as a cargo \
         invocation — the reader is scanning prose, so every job with that step \
         would be required to pin a compiler it never uses"
    );

    let block = only_step(
        "name: gates\n        run: |\n          cargo fmt --all -- --check\n          cargo test",
    );
    assert!(invokes_cargo(run_blocks(&block)));

    let inline = only_step("name: build\n        run: cargo build --release");
    assert!(invokes_cargo(run_blocks(&inline)));

    // `env:` is not script. A cargo string sitting in an environment value must
    // not be read as a command the job runs.
    let env_only = only_step(
        "name: echo\n        run: |\n          echo hi\n        env:\n          X: cargo build",
    );
    assert!(!invokes_cargo(run_blocks(&env_only)));

    // Every block-scalar form yields the same script, which is the property the
    // hand-rolled reader had to special-case and this one gets for free.
    for form in ["|", ">", "|-", ">-"] {
        let scalar = only_step(&format!("run: {form}\n          cargo build --release"));
        assert!(
            invokes_cargo(run_blocks(&scalar)),
            "block scalar form `{form}` did not yield the script"
        );
    }
}

#[test]
fn the_parser_reads_a_workflow_whatever_its_indentation() {
    // The hole this rewrite closed. Four-space indentation under `jobs:` is
    // valid YAML that GitHub runs; the old fixed-indent split produced NO jobs
    // for it, so such a file was dropped from the scan in silence while the
    // file-count assertion still passed (Codex review, PR #490).
    let four_space = "name: Probe\njobs:\n    probe:\n        runs-on: ubuntu-latest\n        steps:\n            - uses: dtolnay/rust-toolchain@stable\n              with:\n                  toolchain: 1.88.0\n            - name: Build\n              run: cargo build --release\n";
    let parsed = jobs(four_space);
    assert_eq!(
        parsed.len(),
        1,
        "a four-space workflow produced no jobs — the parser is back to \
         assuming this repo's indentation"
    );
    assert_eq!(parsed[0].key, "probe");
    assert_eq!(parsed[0].steps.len(), 2);
    assert_eq!(
        toolchain_input(&parsed[0].steps[0]).as_deref(),
        Some("1.88.0")
    );
    assert!(invokes_cargo(run_blocks(&parsed[0].steps[1])));

    // And the whole walk sees it, which is what the coverage assertions could
    // not tell us before.
    let found = cargo_jobs(&[("four.yaml".to_string(), four_space.to_string())]);
    assert_eq!(found.len(), 1, "the walk did not reach the four-space job");
    assert_eq!(found[0].toolchain_steps[0].2.as_deref(), Some("1.88.0"));

    // Two-space, tab-free, and flow-style mappings all reach the same place.
    let flow = "jobs:\n  a: { runs-on: ubuntu-latest, steps: [ { run: cargo test } ] }\n";
    let parsed = jobs(flow);
    assert_eq!(parsed.len(), 1, "a flow-style job was not parsed");
    assert!(invokes_cargo(run_blocks(&parsed[0].steps[0])));
}

#[test]
fn the_scan_reports_a_planted_literal_pin_by_job_and_value() {
    // The whole walk — jobs, steps, run blocks, classifiers — over a workflow
    // carrying exactly the two defects this gate was written for, so a failure
    // is proven to name the job and the value rather than merely to occur.
    let planted = r#"name: Planted
jobs:
  restates-the-pin:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v6

      - uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: 1.88.0

      - name: Build source AWARE host
        run: cargo build --manifest-path cli/Cargo.toml --locked

  floating-stable:
    runs-on: ubuntu-latest
    steps:
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Build
        working-directory: 20-agents/aeco/engineering/steel-detailer-lookup
        run: cargo build --release

  no-rust-here:
    runs-on: ubuntu-latest
    steps:
      - name: Cache cargo registry + build
        uses: Swatinem/rust-cache@v2

      - name: Test
        run: dotnet test cli-roslyn
"#;

    let found = cargo_jobs(&[("planted.yml".to_string(), planted.to_string())]);
    let keys: Vec<&str> = found.iter().map(|job| job.key.as_str()).collect();
    assert_eq!(
        keys,
        vec!["restates-the-pin", "floating-stable"],
        "the walk did not pick out exactly the two cargo-invoking jobs — a \
         `dotnet` job with a step merely NAMED `Cache cargo registry + build` \
         must not be one of them"
    );

    let restated = &found[0];
    assert_eq!(
        restated.toolchain_steps.len(),
        1,
        "the walk lost the toolchain step it is supposed to inspect"
    );
    let (_, uses, input) = &restated.toolchain_steps[0];
    assert_eq!(uses, "dtolnay/rust-toolchain@stable");
    assert_eq!(
        input.as_deref(),
        Some("1.88.0"),
        "the walk did not read back the literal version it must reject"
    );
    assert!(
        !reads_a_step_output("1.88.0"),
        "a literal version was accepted as reading the pin"
    );

    let floating = &found[1];
    assert_eq!(floating.toolchain_steps.len(), 1);
    assert_eq!(
        floating.toolchain_steps[0].2, None,
        "a `@stable` step with no `toolchain:` input must read back as no input \
         at all — that is the case the gate reports as an unpinned compiler"
    );
}

#[test]
fn the_scan_accepts_the_corrected_form() {
    // The positive control for the test above: the same two jobs, fixed. Without
    // this, a walk that rejected every workflow ever written would look correct.
    let fixed = r#"name: Fixed
jobs:
  reads-the-pin:
    runs-on: windows-latest
    steps:
      - name: Read pinned toolchain
        id: pin
        shell: bash
        run: |
          channel=$(sed -n 's/^channel = "\(.*\)"/\1/p' cli/rust-toolchain.toml | head -1)
          echo "channel=$channel" >> "$GITHUB_OUTPUT"

      - name: Install Rust ${{ steps.pin.outputs.channel }}
        uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ steps.pin.outputs.channel }}

      - name: Build source AWARE host
        run: cargo build --manifest-path cli/Cargo.toml --locked
"#;

    let found = cargo_jobs(&[("fixed.yml".to_string(), fixed.to_string())]);
    assert_eq!(found.len(), 1);
    let job = &found[0];
    assert_eq!(job.toolchain_steps.len(), 1);
    let value = job.toolchain_steps[0]
        .2
        .clone()
        .expect("the corrected form passes a `toolchain:` input");
    assert!(
        reads_a_step_output(&value),
        "the corrected form was not recognised as reading the pin: {value:?}"
    );
    let id = referenced_step_id(&value).expect("expression names a step");
    let producer_at = job
        .step_ids
        .iter()
        .find(|(_, declared)| *declared == id)
        .map(|(at, _)| *at);
    assert_eq!(
        producer_at,
        Some(0),
        "the walk did not collect the `id: {id}` the expression refers to at its \
         real position; ids seen: {:?}",
        job.step_ids
    );
    assert!(
        producer_at.unwrap() < job.toolchain_steps[0].0,
        "the corrected form declares the id before the step consuming it, and the \
         positive control must accept that ordering"
    );
}

#[test]
fn the_step_declaring_the_id_must_come_before_the_step_reading_it() {
    // The gap left after round three. `Install Rust` is above the build, so the
    // install-before-cargo check passes, and `id: pin` exists in the job, so the
    // presence check passes — but it is declared BELOW the action reading its
    // output. `steps.pin.outputs.channel` is resolved when the install runs, at
    // which point that step has not run, so the action is handed an empty
    // toolchain and silently falls back to its ref's default (Codex review,
    // PR #490).
    let producer_too_late = r#"jobs:
  probe:
    steps:
      - name: Install Rust ${{ steps.pin.outputs.channel }}
        uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ steps.pin.outputs.channel }}

      - name: Read pinned toolchain
        id: pin
        shell: bash
        run: |
          channel=$(sed -n 's/^channel = "\(.*\)"/\1/p' cli/rust-toolchain.toml | head -1)
          echo "channel=$channel" >> "$GITHUB_OUTPUT"

      - name: Build
        run: cargo build --locked
"#;
    let found = cargo_jobs(&[("late.yml".to_string(), producer_too_late.to_string())]);
    assert_eq!(found.len(), 1);
    let job = &found[0];
    // Everything the earlier rounds check is satisfied by this job: the install
    // is present, it precedes cargo, and the id it names is declared somewhere.
    assert_eq!(job.toolchain_steps[0].0, 0, "the install is the first step");
    assert!(
        job.toolchain_steps[0].0 < job.first_cargo,
        "the install still precedes cargo, so round three's check passes here"
    );
    assert!(
        job.step_ids.iter().any(|(_, declared)| declared == "pin"),
        "the id is present, so a contains-only check passes here too"
    );
    // Only the position separates this from the correct form.
    assert_eq!(
        job.step_ids
            .iter()
            .find(|(_, declared)| declared == "pin")
            .map(|(at, _)| *at),
        Some(1),
        "the producer is the second step, i.e. after the step consuming it"
    );

    // The same three steps with the producer first: the ordering that must be
    // accepted, or the assertion rejects everything and proves nothing.
    let producer_first = producer_too_late.replace(
        "      - name: Install Rust ${{ steps.pin.outputs.channel }}\n        uses: dtolnay/rust-toolchain@master\n        with:\n          toolchain: ${{ steps.pin.outputs.channel }}\n\n      - name: Read pinned toolchain\n        id: pin\n        shell: bash\n        run: |\n          channel=$(sed -n 's/^channel = \"\\(.*\\)\"/\\1/p' cli/rust-toolchain.toml | head -1)\n          echo \"channel=$channel\" >> \"$GITHUB_OUTPUT\"\n",
        "      - name: Read pinned toolchain\n        id: pin\n        shell: bash\n        run: |\n          channel=$(sed -n 's/^channel = \"\\(.*\\)\"/\\1/p' cli/rust-toolchain.toml | head -1)\n          echo \"channel=$channel\" >> \"$GITHUB_OUTPUT\"\n\n      - name: Install Rust ${{ steps.pin.outputs.channel }}\n        uses: dtolnay/rust-toolchain@master\n        with:\n          toolchain: ${{ steps.pin.outputs.channel }}\n",
    );
    assert_ne!(
        producer_first, producer_too_late,
        "the reordering must actually apply, or the positive control is the \
         negative one again"
    );
    let found = cargo_jobs(&[("early.yml".to_string(), producer_first)]);
    assert_eq!(found.len(), 1);
    let job = &found[0];
    assert_eq!(
        job.step_ids
            .iter()
            .find(|(_, declared)| declared == "pin")
            .map(|(at, _)| *at),
        Some(0),
        "the producer is now the first step"
    );
    assert!(
        job.step_ids
            .iter()
            .find(|(_, declared)| declared == "pin")
            .map(|(at, _)| *at)
            .unwrap()
            < job.toolchain_steps[0].0,
        "the correct ordering must be accepted"
    );
}
