//! Guard the compiler pin used by GitHub Actions.
//!
//! `cli/rust-toolchain.toml` is the single source of truth for the compiler, and
//! every workflow job that runs Cargo has to reach it. Before PR #490 two did
//! not: `bridge-windows-packaged` restated `1.88.0` and `release.yml` installed
//! floating `stable`, so the binaries shipped in every release archive were
//! built by whatever compiler that day's `stable` happened to be.
//!
//! ## Why this shape, and not a smarter one
//!
//! PR #490's first guard tried to *understand* the workflows — it matched
//! affixes over the `run:` script, then grew a hand-written shell tokeniser. It
//! absorbed twenty-two review findings across fourteen rounds and was still
//! producing them, in both directions at once: `channel=stable` after a valid
//! assignment walked through it, while an idiomatic `printf 'channel=%s\n'`
//! publish was rejected. Both are the same defect — re-implementing POSIX shell
//! inside a test file — so this guard does not try.
//!
//! Instead the pin reader is an **exact contract**: one canonical step, compared
//! verbatim. There is exactly one such step per Cargo job and it is six lines.
//! Changing it means changing `PIN_RUN` here too, deliberately and visibly, in
//! a diff a reviewer can read. Nothing is parsed, so nothing can be mis-parsed
//! and nothing can be smuggled past a pattern.
//!
//! ## What it checks
//!
//! For every job that reaches Cargo, in `.github/workflows/*.{yml,yaml}`:
//!
//! * the exact approved `Read pinned toolchain` step is present, once;
//! * exactly one Rust installer, consuming that step's output, unconditional;
//! * read, then install, then Cargo — in that order;
//! * no `rustup …` command and no `cargo +toolchain` selector anywhere in the
//!   job, which is why no shell parsing is needed to reject them;
//! * no `RUSTUP_TOOLCHAIN` at workflow, job, `container.env` or step scope,
//!   matched case-insensitively because Windows resolves environment variables
//!   that way;
//! * no `defaults.run` at workflow or job scope, which could move the reader off
//!   the repository root.
//!
//! Repository-wide it also checks that `cli/rust-toolchain.toml` is the only
//! *tracked* toolchain file — a nested one silently overrides the installed pin
//! for anything built from its directory, and two jobs build from exactly such a
//! directory. Tracked, via `git ls-files`, rather than present on disk: walking
//! the filesystem failed on git-ignored `.claude/worktrees/` checkouts, which
//! this repository's `.gitignore` explicitly anticipates.
//!
//! ## Cargo is not only `run: cargo …`
//!
//! An action can invoke Cargo without any script of its own —
//! `Swatinem/rust-cache` shells out to `cargo metadata` for its cache key, which
//! `ci.yml` records in its own comment. Placed above the installer it therefore
//! runs Cargo on the runner's default compiler, invisibly to a guard that only
//! reads `run:` text.
//!
//! Naming the actions that do this is only worth something if the list cannot go
//! quietly out of date, so every `uses:` in every workflow must appear in one of
//! the three lists below. An action nobody has classified fails the guard by
//! name and asks for the decision, rather than being assumed inert.
//!
//! ## What it does not check
//!
//! It proves a job *installs* the pin, never that the compiler can build the
//! crate. Its reach stops at `.github/workflows/`, so Cargo invoked by a script
//! a workflow calls is outside it. It knows `dtolnay/rust-toolchain`
//! specifically, and the Cargo-running actions named below — the classification
//! check is what keeps that knowledge honest.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

const PIN_RUN: &str = r#"channel=$(sed -n 's/^[[:space:]]*channel[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' cli/rust-toolchain.toml | head -1)
if [ -z "$channel" ]; then
  echo "::error::could not read [toolchain] channel from cli/rust-toolchain.toml"
  exit 1
fi
echo "Pinned toolchain: $channel"
echo "channel=$channel" >> "$GITHUB_OUTPUT"
"#;

const TOOLCHAIN_OUTPUT: &str = "${{ steps.pin.outputs.channel }}";

/// The action that installs the pin. Its `@ref` is deliberately not fixed here —
/// [`is_canonical_install`] requires `@master` and the exact output expression.
const RUST_INSTALLER_ACTION: &str = "dtolnay/rust-toolchain";

/// Actions that invoke Cargo themselves, so they must not run before the pin is
/// installed. `Swatinem/rust-cache` runs `cargo metadata` to build its cache key.
const CARGO_RUNNING_ACTIONS: &[&str] = &["swatinem/rust-cache"];

/// Actions established not to invoke Cargo. This list exists so that an action
/// which does can never be *assumed* inert: an unlisted one fails the guard and
/// has to be classified.
const CARGO_INERT_ACTIONS: &[&str] = &[
    "actions/checkout",
    "actions/download-artifact",
    "actions/setup-dotnet",
    "actions/setup-node",
    "actions/setup-python",
    "actions/upload-artifact",
    "softprops/action-gh-release",
];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("cli has a repository parent")
        .to_path_buf()
}

fn canonical_pin_step() -> serde_yaml::Value {
    serde_yaml::from_str(&format!(
        "name: Read pinned toolchain\nid: pin\nshell: bash\nrun: |\n{}\n",
        PIN_RUN
            .lines()
            .map(|line| format!("  {line}"))
            .collect::<Vec<_>>()
            .join("\n")
    ))
    .expect("the canonical pin step is valid YAML")
}

fn fixture_job() -> serde_yaml::Value {
    let indented_pin = PIN_RUN
        .lines()
        .map(|line| format!("      {line}"))
        .collect::<Vec<_>>()
        .join("\n");
    serde_yaml::from_str(&format!(
        "runs-on: ubuntu-latest\nsteps:\n  - name: Read pinned toolchain\n    id: pin\n    shell: bash\n    run: |\n{indented_pin}\n  - name: Install Rust ${{{{ steps.pin.outputs.channel }}}}\n    uses: dtolnay/rust-toolchain@master\n    with:\n      toolchain: ${{{{ steps.pin.outputs.channel }}}}\n  - run: cargo build --locked\n"
    ))
    .expect("the fixture job is valid YAML")
}

fn fixture_problems(job: &serde_yaml::Value) -> Vec<String> {
    let workflow: serde_yaml::Value =
        serde_yaml::from_str("jobs: {}").expect("the fixture workflow is valid YAML");
    cargo_job_problems("fixture.yml", "build", &workflow, job)
}

fn insert_step(job: &mut serde_yaml::Value, index: usize, step: &str) {
    let step: serde_yaml::Value = serde_yaml::from_str(step).expect("fixture step is valid YAML");
    job["steps"]
        .as_sequence_mut()
        .expect("fixture steps are a sequence")
        .insert(index, step);
}

fn run_text(step: &serde_yaml::Value) -> Option<&str> {
    step.get("run").and_then(serde_yaml::Value::as_str)
}

/// A token with the shell punctuation trimmed off either end — quotes, parens,
/// a trailing `;`. `+` is kept, because a toolchain selector is one.
fn bare_token(token: &str) -> &str {
    token.trim_matches(|character: char| {
        !(character.is_ascii_alphanumeric()
            || matches!(character, '_' | '-' | '.' | '/' | '\\' | '+'))
    })
}

/// Whether a token names the Cargo executable — path prefix and `.exe` suffix
/// included, `cargo-nextest` and friends excluded.
///
/// [`invokes_cargo`] and [`has_toolchain_override`] both go through this so they
/// cannot disagree about what Cargo is called. That disagreement was itself a
/// bypass (Codex review, PR #506): `cargo.exe +nightly build` counted as a Cargo
/// invocation, so the job was guarded, while the selector check looked for the
/// literal `cargo ` and never saw it.
fn is_cargo_executable(token: &str) -> bool {
    let token = bare_token(token);
    let executable = token
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(token)
        .to_ascii_lowercase();
    matches!(executable.as_str(), "cargo" | "cargo.exe")
}

fn invokes_cargo(run: &str) -> bool {
    run.split(|character: char| {
        !(character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '.' | '/' | '\\'))
    })
    .any(is_cargo_executable)
}

/// The action a step (or a reusable-workflow job) names, without its `@ref` and
/// lowercased. GitHub resolves these case-insensitively, so the lists above are
/// written lowercase and compared against this.
fn action_reference(value: &serde_yaml::Value) -> Option<String> {
    value
        .get("uses")
        .and_then(serde_yaml::Value::as_str)
        .map(|uses| {
            uses.split('@')
                .next()
                .unwrap_or(uses)
                .trim()
                .to_ascii_lowercase()
        })
}

fn is_rust_toolchain_action(step: &serde_yaml::Value) -> bool {
    action_reference(step).is_some_and(|action| action == RUST_INSTALLER_ACTION)
}

fn is_cargo_running_action(step: &serde_yaml::Value) -> bool {
    action_reference(step).is_some_and(|action| CARGO_RUNNING_ACTIONS.contains(&action.as_str()))
}

/// Cargo is reached either by a script that names it or by an action known to
/// run it. Both count, and both have to sit below the installer.
fn step_invokes_cargo(step: &serde_yaml::Value) -> bool {
    run_text(step).is_some_and(invokes_cargo) || is_cargo_running_action(step)
}

/// How a step is named in a failure message. An action step carries its `uses:`
/// as well as its name, because "this step runs Cargo" is not obvious from a
/// name like `Cache cargo registry + build`.
fn step_label(step: &serde_yaml::Value) -> String {
    let name = step.get("name").and_then(serde_yaml::Value::as_str);
    let uses = step.get("uses").and_then(serde_yaml::Value::as_str);
    match (name, uses) {
        (Some(name), Some(uses)) => format!("{name} — {uses}"),
        (Some(name), None) => name.to_owned(),
        (None, Some(uses)) => uses.to_owned(),
        (None, None) => run_text(step)
            .and_then(|run| run.lines().next())
            .unwrap_or("<unnamed step>")
            .to_owned(),
    }
}

fn is_canonical_install(step: &serde_yaml::Value) -> bool {
    step.get("name").and_then(serde_yaml::Value::as_str)
        == Some("Install Rust ${{ steps.pin.outputs.channel }}")
        && step.get("uses").and_then(serde_yaml::Value::as_str)
            == Some("dtolnay/rust-toolchain@master")
        && step
            .get("with")
            .and_then(|with| with.get("toolchain"))
            .and_then(serde_yaml::Value::as_str)
            == Some(TOOLCHAIN_OUTPUT)
        && step.get("if").is_none()
        && step.get("continue-on-error").is_none()
}

/// A `rustup` command or an explicit `cargo +toolchain` selector anywhere in the
/// job, which either overrides or bypasses the installed pin.
///
/// The backslash-newline is dropped first. The shell removes a line continuation
/// before it splits words, so `cargo \` + newline + `+nightly build` runs as
/// `cargo +nightly build`; leaving it in put a `\` between the two tokens and the
/// selector went unseen. That is a lexical rewrite of two characters, not a step
/// toward interpreting the script.
///
/// The selector itself is then read as two adjacent words — a Cargo executable
/// followed by a `+…` — rather than as the literal text `cargo +`, so every
/// spelling [`is_cargo_executable`] already accepts is covered: `cargo.exe`, an
/// absolute path, a quoted invocation. Both findings on this file were the gap
/// between those two notions of "Cargo". No shell model is needed for either:
/// the question is still only whether these words appear next to each other.
fn has_toolchain_override(run: &str) -> bool {
    let lower = run.to_ascii_lowercase().replace("\\\n", "");
    if lower.contains("rustup") {
        return true;
    }
    lower
        .split_whitespace()
        .collect::<Vec<_>>()
        .windows(2)
        .any(|pair| is_cargo_executable(pair[0]) && bare_token(pair[1]).starts_with('+'))
}

fn declares_rustup_toolchain(value: &serde_yaml::Value) -> bool {
    value
        .get("env")
        .and_then(serde_yaml::Value::as_mapping)
        .is_some_and(|env| {
            env.keys()
                .filter_map(serde_yaml::Value::as_str)
                .any(|key| key.eq_ignore_ascii_case("RUSTUP_TOOLCHAIN"))
        })
}

fn toolchain_files(root: &Path) -> Vec<PathBuf> {
    let output = Command::new("git")
        .args(["-C"])
        .arg(root)
        .args(["ls-files", "-z"])
        .output()
        .unwrap_or_else(|error| panic!("list tracked files under {}: {error}", root.display()));
    assert!(
        output.status.success(),
        "git ls-files failed under {}: {}",
        root.display(),
        String::from_utf8_lossy(&output.stderr)
    );

    let mut found = output
        .stdout
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(|path| PathBuf::from(String::from_utf8_lossy(path).as_ref()))
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| {
                    name.eq_ignore_ascii_case("rust-toolchain")
                        || name.eq_ignore_ascii_case("rust-toolchain.toml")
                })
        })
        .collect::<Vec<_>>();
    found.sort();
    found
}

/// Every `uses:` this job names that no list above classifies. A reusable
/// workflow (`jobs.<id>.uses`) counts too: it can run Cargo, and this guard
/// cannot see inside it.
fn unclassified_actions(
    workflow_name: &str,
    job_name: &str,
    job: &serde_yaml::Value,
) -> Vec<String> {
    let steps = job
        .get("steps")
        .and_then(serde_yaml::Value::as_sequence)
        .map(Vec::as_slice)
        .unwrap_or_default();

    std::iter::once(job)
        .chain(steps)
        .filter_map(action_reference)
        .filter(|action| {
            action != RUST_INSTALLER_ACTION
                && !CARGO_RUNNING_ACTIONS.contains(&action.as_str())
                && !CARGO_INERT_ACTIONS.contains(&action.as_str())
        })
        .map(|action| {
            format!(
                "{workflow_name}: job `{job_name}` uses `{action}`, which this guard has not \
                 classified. Decide whether it invokes Cargo, then add it to CARGO_RUNNING_ACTIONS \
                 or CARGO_INERT_ACTIONS in cli/tests/toolchain_pin_gate.rs"
            )
        })
        .collect()
}

fn cargo_job_problems(
    workflow_name: &str,
    job_name: &str,
    workflow: &serde_yaml::Value,
    job: &serde_yaml::Value,
) -> Vec<String> {
    let label = format!("{workflow_name}: job `{job_name}`");
    // A job with no steps of its own — a reusable-workflow call — runs no script
    // this guard can read. `unclassified_actions` is what refuses to assume it is
    // Cargo-free.
    let Some(steps) = job.get("steps").and_then(serde_yaml::Value::as_sequence) else {
        return Vec::new();
    };

    let cargo_indices: Vec<_> = steps
        .iter()
        .enumerate()
        .filter_map(|(index, step)| step_invokes_cargo(step).then_some(index))
        .collect();
    if cargo_indices.is_empty() {
        return Vec::new();
    }

    let mut problems = Vec::new();
    if workflow.get("defaults").is_some() || job.get("defaults").is_some() {
        problems.push(format!(
            "{label} sets run defaults, so the pin reader may no longer run from the repository root"
        ));
    }

    let pin_indices: Vec<_> = steps
        .iter()
        .enumerate()
        .filter_map(|(index, step)| (step == &canonical_pin_step()).then_some(index))
        .collect();
    if pin_indices.len() != 1 {
        problems.push(format!(
            "{label} must contain the exact approved `Read pinned toolchain` step once — compare \
             it against PIN_RUN in cli/tests/toolchain_pin_gate.rs, and if the step is meant to \
             change, change both in the same commit"
        ));
    }

    let rust_actions: Vec<_> = steps
        .iter()
        .enumerate()
        .filter(|(_, step)| is_rust_toolchain_action(step))
        .collect();
    if rust_actions.len() != 1
        || !rust_actions
            .iter()
            .all(|(_, step)| is_canonical_install(step))
    {
        problems.push(format!(
            "{label} must contain exactly one unconditional `dtolnay/rust-toolchain@master` step, \
             named `Install Rust {TOOLCHAIN_OUTPUT}` and taking `toolchain: {TOOLCHAIN_OUTPUT}` — a \
             floating `@stable` and a restated version are what this rejects"
        ));
    }

    if let (Some(&pin), Some(&(install, _)), Some(&first_cargo)) = (
        pin_indices.first(),
        rust_actions.first(),
        cargo_indices.first(),
    ) && !(pin < install && install < first_cargo)
    {
        problems.push(format!(
            "{label} reaches Cargo at step {first_cargo} (`{}`) without having read and installed \
             the pin first — steps run in order, so that Cargo call uses whatever compiler the \
             runner shipped",
            step_label(&steps[first_cargo])
        ));
    }

    if steps
        .iter()
        .filter_map(run_text)
        .any(has_toolchain_override)
    {
        problems.push(format!(
            "{label} contains a rustup command or explicit `cargo +toolchain` selector"
        ));
    }

    if declares_rustup_toolchain(workflow)
        || declares_rustup_toolchain(job)
        || job.get("container").is_some_and(declares_rustup_toolchain)
        || steps.iter().any(declares_rustup_toolchain)
    {
        problems.push(format!(
            "{label} sets RUSTUP_TOOLCHAIN, which overrides the installed repository pin"
        ));
    }

    problems
}

/// Everything one walk of `.github/workflows/` establishes. Two tests read it,
/// so an unclassified action and an unpinned Cargo job stay separate failures
/// with separate messages.
#[derive(Default)]
struct Findings {
    cargo_jobs: BTreeSet<String>,
    /// Every `uses:` the walk actually read. An empty classification list means
    /// nothing needs a decision only if this is non-empty; otherwise the walk
    /// found no workflows and the check passed by reading nothing.
    actions: BTreeSet<String>,
    pin: Vec<String>,
    classification: Vec<String>,
}

fn workflow_findings(name: &str, source: &str) -> Findings {
    let workflow: serde_yaml::Value = serde_yaml::from_str(source)
        .unwrap_or_else(|error| panic!("parse {name} as workflow YAML: {error}"));
    let jobs = workflow
        .get("jobs")
        .and_then(serde_yaml::Value::as_mapping)
        .unwrap_or_else(|| panic!("{name} has no jobs mapping"));

    let mut findings = Findings::default();
    for (job_name, job) in jobs {
        let Some(job_name) = job_name.as_str() else {
            continue;
        };
        findings.actions.extend(
            std::iter::once(job)
                .chain(
                    job.get("steps")
                        .and_then(serde_yaml::Value::as_sequence)
                        .map(Vec::as_slice)
                        .unwrap_or_default(),
                )
                .filter_map(action_reference),
        );
        findings
            .classification
            .extend(unclassified_actions(name, job_name, job));
        findings
            .pin
            .extend(cargo_job_problems(name, job_name, &workflow, job));
        let reaches_cargo = job
            .get("steps")
            .and_then(serde_yaml::Value::as_sequence)
            .is_some_and(|steps| steps.iter().any(step_invokes_cargo));
        if reaches_cargo {
            findings.cargo_jobs.insert(format!("{name}:{job_name}"));
        }
    }
    findings
}

fn scan_workflows() -> Findings {
    let workflow_dir = repo_root().join(".github/workflows");
    let mut all = Findings::default();

    for entry in std::fs::read_dir(&workflow_dir).expect("read .github/workflows") {
        let path = entry.expect("read workflow entry").path();
        if !matches!(
            path.extension().and_then(|ext| ext.to_str()),
            Some("yml" | "yaml")
        ) {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .expect("workflow has a UTF-8 filename");
        let source = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        let findings = workflow_findings(name, &source);
        all.cargo_jobs.extend(findings.cargo_jobs);
        all.actions.extend(findings.actions);
        all.pin.extend(findings.pin);
        all.classification.extend(findings.classification);
    }
    all
}

#[test]
fn every_workflow_job_that_runs_cargo_installs_the_repository_pin() {
    let findings = scan_workflows();

    let expected = BTreeSet::from([
        "ci.yml:bridge-windows-packaged".to_owned(),
        "ci.yml:gates".to_owned(),
        "ci.yml:gates-macos".to_owned(),
        "release.yml:build".to_owned(),
    ]);
    assert_eq!(
        findings.cargo_jobs, expected,
        "the known Cargo-job set changed; review the new set and update this explicit coverage list"
    );
    assert!(findings.pin.is_empty(), "{}", findings.pin.join("\n"));
}

#[test]
fn every_workflow_action_is_classified_as_cargo_running_or_inert() {
    let findings = scan_workflows();
    assert!(
        findings.classification.is_empty(),
        "{}",
        findings.classification.join("\n")
    );
    // Nothing left to classify means something only if the walk read the
    // workflows at all — a scan that found no files would report the same.
    for expected in [
        "actions/checkout",
        "dtolnay/rust-toolchain",
        "swatinem/rust-cache",
    ] {
        assert!(
            findings.actions.contains(expected),
            "the walk did not reach `{expected}`, so it read fewer workflows than the repository has"
        );
    }

    // …and an unknown action really does fail, rather than the list above being
    // satisfied by there being nothing left to classify.
    let mut job = fixture_job();
    insert_step(&mut job, 1, "uses: acme/does-it-run-cargo@v1");
    assert_eq!(
        unclassified_actions("fixture.yml", "build", &job).len(),
        1,
        "an unlisted action must be reported"
    );

    // A reusable workflow runs steps this guard cannot see, so it needs the same
    // decision.
    let reusable: serde_yaml::Value =
        serde_yaml::from_str("uses: ./.github/workflows/build.yml").expect("valid fixture job");
    assert_eq!(
        unclassified_actions("fixture.yml", "call", &reusable).len(),
        1
    );
}

#[test]
fn the_action_classification_lists_are_normalised_and_disjoint() {
    let mut all: Vec<&str> = CARGO_RUNNING_ACTIONS
        .iter()
        .chain(CARGO_INERT_ACTIONS)
        .chain(std::iter::once(&RUST_INSTALLER_ACTION))
        .copied()
        .collect();
    let listed = all.len();
    all.sort_unstable();
    all.dedup();
    assert_eq!(
        all.len(),
        listed,
        "an action is classified twice; one of the lists disagrees with the other"
    );
    assert!(
        all.iter()
            .all(|action| *action == action.to_ascii_lowercase()),
        "the lists are compared against a lowercased `uses:`, so they must be written lowercase"
    );
}

#[test]
fn the_repository_has_one_toolchain_file_and_therefore_one_pin() {
    assert_eq!(
        toolchain_files(&repo_root()),
        vec![PathBuf::from("cli/rust-toolchain.toml")],
        "keep one source of truth; workflow jobs read cli/rust-toolchain.toml explicitly"
    );
}

#[test]
fn the_pin_reader_is_an_exact_contract() {
    let canonical = canonical_pin_step();
    assert_eq!(
        canonical.get("run").and_then(serde_yaml::Value::as_str),
        Some(PIN_RUN)
    );
    assert!(fixture_problems(&fixture_job()).is_empty());

    for changed in [
        PIN_RUN.replace("\"$GITHUB_OUTPUT\"", "'$GITHUB_OUTPUT'"),
        PIN_RUN.replace("channel=$(sed", "channel=\"$(sed"),
        PIN_RUN.replace(" | head -1)", " | head -1)\""),
        PIN_RUN.replace(
            "echo \"channel=$channel\"",
            "printf 'channel=%s\\n' \"$channel\"",
        ),
        format!("{PIN_RUN}channel=stable\n"),
        format!("{PIN_RUN}if true; then channel=stable; fi\n"),
    ] {
        let mut job = fixture_job();
        job["steps"][0]["run"] = serde_yaml::Value::String(changed);
        assert!(!fixture_problems(&job).is_empty());
    }
}

#[test]
fn later_toolchain_overrides_are_rejected_without_parsing_shell() {
    for run in [
        "rustup override set stable",
        "rustup default stable",
        "rustup run stable cargo build",
        "cargo +stable build",
        "/usr/bin/cargo   +nightly test",
        // The shell removes a backslash-newline before it splits words, so this
        // runs as `cargo +nightly build` (Codex review, PR #506).
        "cargo \\\n  +nightly build",
        // Every spelling of the executable the Cargo-invocation check already
        // accepts, since a job spelling it this way is guarded but was having
        // its selector read past (Codex review, PR #506).
        "cargo.exe +nightly build",
        "C:\\Users\\runner\\.cargo\\bin\\cargo.exe +stable build",
        "sh -c \"cargo +nightly build\"",
    ] {
        assert!(has_toolchain_override(run), "accepted override: {run}");
        let mut job = fixture_job();
        let mut override_step = serde_yaml::Mapping::new();
        override_step.insert(
            serde_yaml::Value::String("run".to_owned()),
            serde_yaml::Value::String(run.to_owned()),
        );
        job["steps"]
            .as_sequence_mut()
            .expect("fixture steps are a sequence")
            .insert(2, serde_yaml::Value::Mapping(override_step));
        assert!(!fixture_problems(&job).is_empty());
    }
    // …and dropping the continuation must not invent a selector where there is
    // none: a wrapped command is still just a wrapped command.
    assert!(!has_toolchain_override("cargo build --locked"));
    assert!(!has_toolchain_override("cargo build \\\n  --locked"));
    assert!(!has_toolchain_override(
        "cargo clippy --all-targets \\\n  -- -D warnings"
    ));
    // A cargo-prefixed subcommand binary is not Cargo, and the `+` has to follow
    // Cargo itself rather than merely appear somewhere in the script.
    assert!(!has_toolchain_override("cargo-nextest run +extra"));
    assert!(!has_toolchain_override("echo 1 + 2 && cargo build"));
}

#[test]
fn floating_or_late_installers_are_rejected() {
    let mut floating = fixture_job();
    floating["steps"][1]["uses"] =
        serde_yaml::Value::String("dtolnay/rust-toolchain@stable".to_owned());
    assert!(!fixture_problems(&floating).is_empty());

    let mut late = fixture_job();
    late["steps"]
        .as_sequence_mut()
        .expect("fixture steps are a sequence")
        .swap(1, 2);
    assert!(!fixture_problems(&late).is_empty());
}

/// `Swatinem/rust-cache` runs `cargo metadata`, with no `run:` script to read —
/// the one bypass left open when PR #490's guard was withdrawn. Position, not
/// presence, is what makes it wrong: the same step below the installer is the
/// arrangement `ci.yml` already ships.
#[test]
fn a_cargo_running_action_above_the_installer_is_rejected() {
    let cache = "name: Cache cargo registry + build\nuses: Swatinem/rust-cache@v2";

    let mut early = fixture_job();
    insert_step(&mut early, 1, cache);
    let problems = fixture_problems(&early);
    assert!(
        problems
            .iter()
            .any(|problem| problem.contains("rust-cache")),
        "a cache action above the installer must be named: {problems:?}"
    );

    let mut late = fixture_job();
    insert_step(&mut late, 2, cache);
    assert!(fixture_problems(&late).is_empty());

    // The action alone makes a job a Cargo job: one that caches but never writes
    // `cargo` in a script still has to install the pin first.
    let cache_only: serde_yaml::Value = serde_yaml::from_str(
        "runs-on: ubuntu-latest\nsteps:\n  - name: Cache cargo registry + build\n    uses: Swatinem/rust-cache@v2\n",
    )
    .expect("the cache-only fixture is valid YAML");
    assert!(!fixture_problems(&cache_only).is_empty());
}

#[test]
fn rustup_toolchain_environment_overrides_are_rejected_at_every_scope() {
    let environment: serde_yaml::Value =
        serde_yaml::from_str("RUSTUP_TOOLCHAIN: stable").expect("fixture env is valid YAML");

    let mut job_scope = fixture_job();
    job_scope["env"] = environment.clone();
    assert!(!fixture_problems(&job_scope).is_empty());

    let mut step_scope = fixture_job();
    step_scope["steps"][2]["env"] = environment.clone();
    assert!(!fixture_problems(&step_scope).is_empty());

    let mut container_scope = fixture_job();
    container_scope["container"] =
        serde_yaml::from_str("image: rust:latest\nenv:\n  rustup_toolchain: stable")
            .expect("fixture container is valid YAML");
    assert!(!fixture_problems(&container_scope).is_empty());

    let mut workflow_scope: serde_yaml::Value =
        serde_yaml::from_str("jobs: {}").expect("fixture workflow is valid YAML");
    workflow_scope["env"] = environment;
    assert!(
        !cargo_job_problems("fixture.yml", "build", &workflow_scope, &fixture_job()).is_empty()
    );
}

#[test]
fn a_nested_toolchain_file_is_detected_without_parsing_its_contents() {
    let root = tempfile::tempdir().expect("create fixture repository");
    let cli = root.path().join("cli");
    let nested = root.path().join("nested/crate");
    std::fs::create_dir_all(&cli).expect("create fixture cli directory");
    std::fs::create_dir_all(&nested).expect("create nested fixture crate");
    std::fs::write(
        cli.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"1.95.0\"\n",
    )
    .expect("write canonical fixture pin");
    std::fs::write(nested.join("rust-toolchain"), "stable\n")
        .expect("write nested fixture override");
    let ignored = root.path().join(".claude/worktrees/ignored/cli");
    std::fs::create_dir_all(&ignored).expect("create ignored fixture worktree");
    std::fs::write(root.path().join(".gitignore"), ".claude/worktrees/\n")
        .expect("write fixture ignore rule");
    std::fs::write(ignored.join("rust-toolchain.toml"), "stable\n")
        .expect("write ignored fixture pin");

    let git = |arguments: &[&str]| {
        let output = Command::new("git")
            .arg("-C")
            .arg(root.path())
            .args(arguments)
            .output()
            .expect("run git for fixture repository");
        assert!(
            output.status.success(),
            "git {arguments:?} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    };
    git(&["init", "--quiet"]);
    git(&["add", ".gitignore", "cli", "nested"]);

    assert_eq!(
        toolchain_files(root.path()),
        vec![
            PathBuf::from("cli/rust-toolchain.toml"),
            PathBuf::from("nested/crate/rust-toolchain"),
        ]
    );
}
