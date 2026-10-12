//! `blender/exec` (#692) through the real CLI: the literal-code rule at validate and compile, and
//! a `--dry-run` that never launches Blender.
//!
//! Both are properties of the SHIPPED manifest (`code: { literal: true }`, `mode: write` without
//! `mode-overridable`), so the agent is installed from the repo's own `20-agents/` copy rather
//! than a synthetic one: a manifest edit that dropped either line must turn this file red.

use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};

fn blender_agent_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root")
        .join("20-agents/aeco/visualization/blender")
}

/// A temporary AWARE_HOME with the repo's `blender` agent installed through `aware agent install`.
fn blender_home() -> tempfile::TempDir {
    let home = tempfile::tempdir().expect("temporary AWARE_HOME");
    Command::cargo_bin("aware")
        .expect("aware test binary")
        .env("AWARE_HOME", home.path())
        .args(["agent", "install"])
        .arg(blender_agent_dir())
        .assert()
        .success();
    home
}

fn app_dir(name: &str, body: &str) -> tempfile::TempDir {
    let app = tempfile::tempdir().expect("temporary app directory");
    fs::write(app.path().join(format!("{name}.flo")), body).expect("write the app");
    app
}

fn run_aware(home: &Path, args: &[&str], target: &Path, envs: &[(&str, &Path)]) -> (bool, String) {
    let mut cmd = Command::cargo_bin("aware").expect("aware test binary");
    cmd.env("AWARE_HOME", home).args(args).arg(target);
    for (k, v) in envs {
        cmd.env(k, v);
    }
    let out = cmd.output().expect("run aware");
    let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    (out.status.success(), text)
}

/// An app with one `blender/exec` node whose `code` is `code_yaml` (already YAML-quoted).
fn exec_app(name: &str, code_yaml: &str) -> String {
    format!(
        "app: {name}\nversion: 0.1.0\ndescription: exec probe\nrequires:\n  - blender@0.2.x\n\
         nodes:\n  - id: animate\n    agent: blender\n    command: exec\n    inputs:\n      \
         output-dir: out\n      args:\n        storeys: '{{{{ inputs.storeys }}}}'\n      \
         code: {code_yaml}\n    safety:\n      transaction-group: probe\n      snapshot: false\n\
         connections: []\n"
    )
}

const LITERAL_CODE: &str =
    "|\n        # Returns a constant.\n        def run(ctx):\n            return 1\n";

#[test]
fn validate_and_compile_refuse_templated_code_and_accept_literal_code() {
    let home = blender_home();

    let templated = app_dir(
        "exec-templated",
        &exec_app("exec-templated", "'{{ inputs.code }}'"),
    );
    let (ok, text) = run_aware(home.path(), &["app", "validate"], templated.path(), &[]);
    assert!(!ok, "templated code must fail validate; got:\n{text}");
    assert!(text.contains("E_APP_INPUT_MUST_BE_LITERAL"), "{text}");
    assert!(
        text.contains("\"animate\""),
        "the error must name the node:\n{text}"
    );
    assert!(
        text.contains("`code`"),
        "the error must name the input:\n{text}"
    );

    let (ok, text) = run_aware(home.path(), &["app", "compile"], templated.path(), &[]);
    assert!(!ok, "templated code must fail compile; got:\n{text}");
    assert!(text.contains("E_APP_INPUT_MUST_BE_LITERAL"), "{text}");
    assert!(
        !templated.path().join("exec-templated.lock").exists(),
        "a refused compile must write no lock"
    );

    // Literal code beside TEMPLATED args (data, not code) is the supported shape.
    let literal = app_dir("exec-literal", &exec_app("exec-literal", LITERAL_CODE));
    let (ok, text) = run_aware(home.path(), &["app", "validate"], literal.path(), &[]);
    assert!(
        ok,
        "literal code with templated args must validate; got:\n{text}"
    );
    let (ok, text) = run_aware(home.path(), &["app", "compile"], literal.path(), &[]);
    assert!(
        ok,
        "literal code with templated args must compile; got:\n{text}"
    );
    assert!(literal.path().join("exec-literal.lock").is_file());
}

/// `exec` is write-mode and NOT mode-overridable, so `--dry-run` reports a would-write and never
/// spawns Blender. The stand-in `AWARE_BLENDER` leaves a marker file if anything launches it —
/// the assertion is on that marker, not on the run's own say-so.
#[test]
fn a_dry_run_of_exec_never_launches_blender() {
    let home = blender_home();
    let scratch = tempfile::tempdir().expect("scratch");
    let marker = scratch.path().join("blender-was-launched");
    let fake = if cfg!(windows) {
        let path = scratch.path().join("fake-blender.cmd");
        fs::write(
            &path,
            format!(
                "@echo off\r\necho launched> \"{}\"\r\nexit /b 1\r\n",
                marker.display()
            ),
        )
        .expect("write fake");
        path
    } else {
        let path = scratch.path().join("fake-blender");
        fs::write(
            &path,
            format!(
                "#!/bin/sh\necho launched > '{}'\nexit 1\n",
                marker.display()
            ),
        )
        .expect("write fake");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).expect("chmod");
        }
        path
    };

    let app = app_dir("exec-dry", &exec_app("exec-dry", LITERAL_CODE));
    let flo = app.path().join("exec-dry.flo");
    let (ok, text) = run_aware(home.path(), &["app", "compile"], &flo, &[]);
    assert!(ok, "compile: {text}");
    let (ok, text) = run_aware(home.path(), &["app", "install"], app.path(), &[]);
    assert!(ok, "install: {text}");

    let mut cmd = Command::cargo_bin("aware").expect("aware test binary");
    let out = cmd
        .env("AWARE_HOME", home.path())
        .env("AWARE_BLENDER", &fake)
        .args(["app", "run", "exec-dry", "--dry-run"])
        .output()
        .expect("run aware");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.status.success(), "dry run failed:\n{text}");
    assert!(
        !marker.exists(),
        "--dry-run launched Blender (the stand-in left its marker):\n{text}"
    );
    assert!(
        text.contains("would-write") || text.contains("would write"),
        "the dry run must report the exec node as a would-write:\n{text}"
    );
}
