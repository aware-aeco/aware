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
//!   * every workflow job that runs `cargo` has a `dtolnay/rust-toolchain` step;
//!   * that step's `toolchain:` is a workflow expression, never a literal
//!     version and never left implicit on the action ref (`@stable`);
//!   * the expression reads a step in the same job that names
//!     `cli/rust-toolchain.toml`, so the value cannot come from somewhere else;
//!   * no `rust-toolchain.toml` anywhere in the repo disagrees with the pin,
//!     which is the other way a directory-sensitive lookup can drift;
//!   * the pin names a concrete version rather than a moving channel.
//!
//! Each of those scans an artefact that is correct today — the real workflows,
//! the real pin file — so each would report clean both when it works and when it
//! has stopped matching anything at all. Every classifier is therefore also
//! driven over synthetic input (`the_channel_reader_matches_its_contract`,
//! `the_cargo_classifier_matches_its_contract`,
//! `the_pin_expression_classifier_matches_its_contract`), the walk itself is
//! driven over a planted offender end to end
//! (`the_scan_reports_a_planted_literal_pin_by_job_and_value`), and the coverage
//! assertion names the jobs it found so a gate guarding an empty set fails
//! loudly (`the_scan_still_reaches_the_two_jobs_that_had_drifted`).
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

/// One workflow job: its YAML key and its body.
struct Job {
    key: String,
    body: String,
}

/// Split a workflow's `jobs:` into jobs, in order.
///
/// A job key is the only thing in these files indented by exactly two spaces and
/// ending in a colon, which is what makes this reliable without a YAML parser.
fn jobs(workflow: &str) -> Vec<Job> {
    let mut jobs: Vec<Job> = Vec::new();
    let mut in_jobs = false;
    for line in workflow.lines() {
        if line.trim_end() == "jobs:" {
            in_jobs = true;
            continue;
        }
        if !in_jobs {
            continue;
        }
        let is_key = line.starts_with("  ")
            && !line.starts_with("   ")
            && line.trim_end().ends_with(':')
            && !line.trim_start().starts_with('#');
        if is_key {
            jobs.push(Job {
                key: line.trim().trim_end_matches(':').to_string(),
                body: String::new(),
            });
        } else if let Some(current) = jobs.last_mut() {
            current.body.push_str(line);
            current.body.push('\n');
        }
    }
    jobs
}

/// One step of a job: the `uses:` it names (empty if none) and its body.
struct Step {
    uses: String,
    body: String,
}

/// Split a job body into steps.
///
/// Steps sit at six spaces in these workflows, so `\n      - ` is the separator —
/// the same split `tests/windows_target_gate.rs` uses.
fn steps(job_body: &str) -> Vec<Step> {
    let padded = format!("\n{job_body}");
    padded
        .split("\n      - ")
        .skip(1) // the job header, before the first step
        .map(|chunk| Step {
            uses: chunk
                .lines()
                .map(str::trim_start)
                .find_map(|line| line.strip_prefix("uses:"))
                .unwrap_or_default()
                .trim()
                .to_string(),
            body: chunk.to_string(),
        })
        .collect()
}

/// The text of every `run:` block in a step, joined.
///
/// Only `run:` content, never the whole step — a `name:` is prose, and this
/// repo has a step called "Cache cargo registry + build" that a naive scan of
/// the step text would read as a cargo invocation.
fn run_blocks(step_body: &str) -> String {
    let lines: Vec<&str> = step_body.lines().collect();
    let mut out = String::new();
    let mut idx = 0;
    while idx < lines.len() {
        let line = lines[idx];
        let indent = line.len() - line.trim_start().len();
        let Some(rest) = line.trim_start().strip_prefix("run:") else {
            idx += 1;
            continue;
        };
        let rest = rest.trim();
        if rest == "|" || rest == ">" || rest == "|-" || rest == ">-" {
            idx += 1;
            while idx < lines.len() {
                let body = lines[idx];
                let body_indent = body.len() - body.trim_start().len();
                if !body.trim().is_empty() && body_indent <= indent {
                    break;
                }
                out.push_str(body);
                out.push('\n');
                idx += 1;
            }
        } else {
            out.push_str(rest);
            out.push('\n');
            idx += 1;
        }
    }
    out
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

/// The `toolchain:` input a step passes, if it passes one.
fn toolchain_input(step_body: &str) -> Option<String> {
    step_body
        .lines()
        .map(str::trim_start)
        .filter(|line| !line.starts_with('#'))
        .find_map(|line| line.strip_prefix("toolchain:"))
        .map(|value| value.trim().to_string())
}

/// Does this `toolchain:` value read the pin rather than restate it?
///
/// A workflow expression referring to a step output. A literal (`1.88.0`) or a
/// moving channel (`stable`) is exactly the drift this gate exists to catch, and
/// so is an expression pointing at an input or a variable rather than a step.
fn reads_a_step_output(value: &str) -> bool {
    let Some(inner) = value.strip_prefix("${{") else {
        return false;
    };
    let Some(inner) = inner.strip_suffix("}}") else {
        return false;
    };
    let inner = inner.trim();
    inner.starts_with("steps.") && inner.ends_with(".outputs.channel")
}

/// The step id an expression like `${{ steps.pin.outputs.channel }}` names.
fn referenced_step_id(value: &str) -> Option<String> {
    let inner = value.strip_prefix("${{")?.strip_suffix("}}")?.trim();
    let rest = inner.strip_prefix("steps.")?;
    let end = rest.find('.')?;
    Some(rest[..end].to_string())
}

/// Every `.yml` under `.github/workflows/`, as (name, text).
fn workflows() -> Vec<(String, String)> {
    let dir = repo_root().join(".github/workflows");
    let mut found: Vec<(String, String)> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension()? != "yml" {
                return None;
            }
            let name = path.file_name()?.to_string_lossy().into_owned();
            Some((name, std::fs::read_to_string(&path).ok()?))
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
struct CargoJob {
    workflow: String,
    key: String,
    toolchain_steps: Vec<(String, Option<String>)>,
    step_ids: Vec<String>,
}

/// Every job across `workflows` that invokes cargo.
fn cargo_jobs(workflows: &[(String, String)]) -> Vec<CargoJob> {
    let mut found = Vec::new();
    for (name, text) in workflows {
        for job in jobs(text) {
            let steps = steps(&job.body);
            if !steps
                .iter()
                .any(|step| invokes_cargo(&run_blocks(&step.body)))
            {
                continue;
            }
            let toolchain_steps = steps
                .iter()
                .filter(|step| step.uses.starts_with("dtolnay/rust-toolchain"))
                .map(|step| (step.uses.clone(), toolchain_input(&step.body)))
                .collect();
            let step_ids = steps
                .iter()
                .filter_map(|step| {
                    step.body
                        .lines()
                        .map(str::trim_start)
                        .find_map(|line| line.strip_prefix("id:"))
                        .map(|id| id.trim().to_string())
                })
                .collect();
            found.push(CargoJob {
                workflow: name.clone(),
                key: job.key.clone(),
                toolchain_steps,
                step_ids,
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

        for (uses, input) in &job.toolchain_steps {
            let Some(value) = input else {
                panic!(
                    "{where_} installs `{uses}` with no `toolchain:` input, so the \
                     compiler is whatever the action ref resolves to that day — the \
                     exact condition the pin in cli/rust-toolchain.toml was added to \
                     remove (#298). Pass `toolchain: ${{{{ steps.<id>.outputs.channel }}}}`."
                );
            };
            assert!(
                reads_a_step_output(value),
                "{where_} pins `toolchain: {value}` literally. The pin lives in \
                 cli/rust-toolchain.toml and must be read, never restated — a \
                 restated version drifts silently, which is how this job came to \
                 build on 1.88.0 while the pin said {pin}."
            );
            let id = referenced_step_id(value)
                .unwrap_or_else(|| panic!("{where_}: could not read a step id out of `{value}`"));
            assert!(
                job.step_ids.contains(&id),
                "{where_} reads `{value}`, but no step in that job declares \
                 `id: {id}`. Step ids present: {:?}",
                job.step_ids
            );
        }
    }
}

#[test]
fn the_step_the_pin_is_read_from_names_the_pin_file() {
    for (name, text) in workflows() {
        for job in jobs(&text) {
            let steps = steps(&job.body);
            let ids: Vec<String> = steps
                .iter()
                .filter(|step| step.uses.starts_with("dtolnay/rust-toolchain"))
                .filter_map(|step| toolchain_input(&step.body))
                .filter_map(|value| referenced_step_id(&value))
                .collect();
            for id in ids {
                let source = steps
                    .iter()
                    .find(|step| {
                        step.body.lines().map(str::trim_start).any(|line| {
                            line.strip_prefix("id:").map(str::trim) == Some(id.as_str())
                        })
                    })
                    .unwrap_or_else(|| {
                        panic!("{name}: job `{}` has no step with `id: {id}`", job.key)
                    });
                assert!(
                    run_blocks(&source.body).contains("cli/rust-toolchain.toml"),
                    "{name}: job `{}` takes its toolchain from step `{id}`, but that \
                     step's script never reads cli/rust-toolchain.toml — so the \
                     version it emits is not the pin.",
                    job.key
                );
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
    assert_eq!(referenced_step_id("1.88.0"), None);
}

#[test]
fn the_run_block_reader_takes_scripts_and_not_step_names() {
    let step = "name: Cache cargo registry + build\n        uses: Swatinem/rust-cache@v2\n        with:\n          workspaces: cli\n";
    assert!(
        !invokes_cargo(&run_blocks(step)),
        "a step NAMED `Cache cargo registry + build` was read as a cargo \
         invocation — the reader is scanning prose, so every job with that step \
         would be required to pin a compiler it never uses"
    );

    let block =
        "name: gates\n        run: |\n          cargo fmt --all -- --check\n          cargo test\n";
    assert!(invokes_cargo(&run_blocks(block)));

    let inline = "name: build\n        run: cargo build --release\n";
    assert!(invokes_cargo(&run_blocks(inline)));

    // A block ends where the indent returns to the key's level; a cargo call in
    // the NEXT step must not be attributed to this one.
    let bounded =
        "name: echo\n        run: |\n          echo hi\n        env:\n          X: cargo build\n";
    assert!(!invokes_cargo(&run_blocks(bounded)));
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
    let (uses, input) = &restated.toolchain_steps[0];
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
        floating.toolchain_steps[0].1, None,
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
        .1
        .clone()
        .expect("the corrected form passes a `toolchain:` input");
    assert!(
        reads_a_step_output(&value),
        "the corrected form was not recognised as reading the pin: {value:?}"
    );
    let id = referenced_step_id(&value).expect("expression names a step");
    assert!(
        job.step_ids.contains(&id),
        "the walk did not collect the `id: {id}` the expression refers to; ids \
         seen: {:?}",
        job.step_ids
    );
}
