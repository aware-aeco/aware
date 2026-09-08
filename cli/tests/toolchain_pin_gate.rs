//! Guard the compiler pin used by GitHub Actions.
//!
//! The workflow step that reads `cli/rust-toolchain.toml` is deliberately an
//! exact contract. We do not try to interpret shell syntax here. Any change to
//! that small block must update this test visibly and receive review.

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

fn run_text(step: &serde_yaml::Value) -> Option<&str> {
    step.get("run").and_then(serde_yaml::Value::as_str)
}

fn invokes_cargo(run: &str) -> bool {
    run.split(|character: char| {
        !(character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '.' | '/' | '\\'))
    })
    .any(|token| {
        let executable = token
            .rsplit(['/', '\\'])
            .next()
            .unwrap_or(token)
            .to_ascii_lowercase();
        matches!(executable.as_str(), "cargo" | "cargo.exe")
    })
}

fn is_rust_toolchain_action(step: &serde_yaml::Value) -> bool {
    step.get("uses")
        .and_then(serde_yaml::Value::as_str)
        .is_some_and(|uses| uses.starts_with("dtolnay/rust-toolchain@"))
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

fn has_toolchain_override(run: &str) -> bool {
    let lower = run.to_ascii_lowercase();
    let normalized = lower.split_whitespace().collect::<Vec<_>>().join(" ");
    lower.contains("rustup") || normalized.contains("cargo +")
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

fn cargo_job_problems(
    workflow_name: &str,
    job_name: &str,
    workflow: &serde_yaml::Value,
    job: &serde_yaml::Value,
) -> Vec<String> {
    let label = format!("{workflow_name}: job `{job_name}`");
    let Some(steps) = job.get("steps").and_then(serde_yaml::Value::as_sequence) else {
        return vec![format!("{label} has no steps")];
    };

    let cargo_indices: Vec<_> = steps
        .iter()
        .enumerate()
        .filter_map(|(index, step)| {
            run_text(step)
                .filter(|run| invokes_cargo(run))
                .map(|_| index)
        })
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
            "{label} must contain the exact approved `Read pinned toolchain` step once"
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
            "{label} must contain one Rust installer that consumes the pin reader's output"
        ));
    }

    if let (Some(&pin), Some(&(install, _)), Some(&first_cargo)) = (
        pin_indices.first(),
        rust_actions.first(),
        cargo_indices.first(),
    ) && !(pin < install && install < first_cargo)
    {
        problems.push(format!(
            "{label} must read and install the pin before its first Cargo command"
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

fn workflow_problems(name: &str, source: &str) -> (BTreeSet<String>, Vec<String>) {
    let workflow: serde_yaml::Value = serde_yaml::from_str(source)
        .unwrap_or_else(|error| panic!("parse {name} as workflow YAML: {error}"));
    let jobs = workflow
        .get("jobs")
        .and_then(serde_yaml::Value::as_mapping)
        .unwrap_or_else(|| panic!("{name} has no jobs mapping"));

    let mut cargo_jobs = BTreeSet::new();
    let mut problems = Vec::new();
    for (job_name, job) in jobs {
        let Some(job_name) = job_name.as_str() else {
            continue;
        };
        let job_problems = cargo_job_problems(name, job_name, &workflow, job);
        let has_cargo = job
            .get("steps")
            .and_then(serde_yaml::Value::as_sequence)
            .is_some_and(|steps| steps.iter().filter_map(run_text).any(invokes_cargo));
        if has_cargo {
            cargo_jobs.insert(format!("{name}:{job_name}"));
        }
        problems.extend(job_problems);
    }
    (cargo_jobs, problems)
}

#[test]
fn every_workflow_job_that_runs_cargo_installs_the_repository_pin() {
    let workflow_dir = repo_root().join(".github/workflows");
    let mut found = BTreeSet::new();
    let mut problems = Vec::new();

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
        let (jobs, workflow_problems) = workflow_problems(name, &source);
        found.extend(jobs);
        problems.extend(workflow_problems);
    }

    let expected = BTreeSet::from([
        "ci.yml:bridge-windows-packaged".to_owned(),
        "ci.yml:gates".to_owned(),
        "ci.yml:gates-macos".to_owned(),
        "release.yml:build".to_owned(),
    ]);
    assert_eq!(
        found, expected,
        "the known Cargo-job set changed; review the new set and update this explicit coverage list"
    );
    assert!(problems.is_empty(), "{}", problems.join("\n"));
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
    assert!(!has_toolchain_override("cargo build --locked"));
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
