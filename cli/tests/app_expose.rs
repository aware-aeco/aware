//! End-to-end tests for `exposes-as-agent`: an app installed as a callable
//! agent, invoked from another app's `nodes:` block (issue #178).

use assert_cmd::Command;
use predicates::prelude::*;

/// Write an app source folder under `<src_root>/<name>/<name>.flo`.
fn write_app(src_root: &std::path::Path, name: &str, flo: &str) -> std::path::PathBuf {
    let dir = src_root.join(name);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join(format!("{name}.flo")), flo).unwrap();
    dir
}

fn install_app(aware: &std::path::Path, src_dir: &std::path::Path) -> assert_cmd::assert::Assert {
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", aware)
        .args(["app", "install"])
        .arg(src_dir)
        .assert()
}

/// A minimal exposed inner app: one inline-predicate node, a `single` command.
/// Runs end-to-end with no host sidecar.
const INNER_FLO: &str = r#"app: inner
version: 0.2.0
description: an exposed inner app
exposes-as-agent: true
exposed-commands:
  run:
    lifecycle: single
    inputs:
      phase:
        type: string
    outputs:
      type: single
      schema:
        ok: bool
nodes:
  - id: gate
    inline:
      kind: predicate
      description: always pass
      code: 'true'
requires: []
"#;

const OUTER_FLO: &str = r#"app: outer
version: 0.1.0
description: composes inner as an agent
nodes:
  - id: call-inner
    agent: inner
    command: run
    config:
      phase: design
connections: []
requires: []
"#;

fn find_jsonl(dir: &std::path::Path) -> Option<std::path::PathBuf> {
    for entry in std::fs::read_dir(dir).ok()?.flatten() {
        let p = entry.path();
        if p.is_dir() {
            if let Some(found) = find_jsonl(&p) {
                return Some(found);
            }
        } else if p.extension().and_then(|e| e.to_str()) == Some("jsonl") {
            return Some(p);
        }
    }
    None
}

#[test]
fn install_registers_synthesized_agent() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let src = tmp.path().join("src");
    let inner = write_app(&src, "inner", INNER_FLO);

    install_app(&aware, &inner).success();

    // A callable agent manifest was synthesized and registered.
    assert!(
        aware.join("agents/inner/manifest.yaml").is_file(),
        "synth agent manifest missing"
    );
    // It shows up in `aware agent list`.
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["agent", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("inner"));
}

#[test]
fn outer_app_invokes_inner_app_as_agent() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let src = tmp.path().join("src");

    install_app(&aware, &write_app(&src, "inner", INNER_FLO)).success();
    install_app(&aware, &write_app(&src, "outer", OUTER_FLO)).success();

    // Running outer dispatches into inner's node chain (no host sidecar needed).
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "run", "outer"])
        .assert()
        .success();

    // The nested run wrote its own provenance trace under logs/inner/nested/.
    let nested = aware.join("logs/inner/nested");
    assert!(
        nested.is_dir() && find_jsonl(&nested).is_some(),
        "nested run trace not written under {}",
        nested.display()
    );
}

#[test]
fn wrong_typed_exposed_input_is_rejected() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let src = tmp.path().join("src");

    // inner declares `phase` as a number; outer passes a string.
    let inner_num = INNER_FLO.replace("type: string", "type: number");
    install_app(&aware, &write_app(&src, "inner", &inner_num)).success();
    install_app(&aware, &write_app(&src, "outer", OUTER_FLO)).success();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "run", "outer"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("expected number"));
}

#[test]
fn exposed_app_cannot_compose_another_exposed_app() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let src = tmp.path().join("src");

    install_app(&aware, &write_app(&src, "inner", INNER_FLO)).success();

    // `mid` is itself exposes-as-agent AND composes the exposed `inner` —
    // forbidden by the v0 no-recursion rule, caught at install/validate time.
    let mid_flo = r#"app: mid
version: 0.1.0
description: exposed app that composes another exposed app
exposes-as-agent: true
exposed-commands:
  run:
    lifecycle: single
nodes:
  - id: call-inner
    agent: inner
    command: run
connections: []
requires: []
"#;
    install_app(&aware, &write_app(&src, "mid", mid_flo))
        .failure()
        .stderr(predicate::str::contains("E_APP_EXPOSED_COMPOSES_EXPOSED"));
}

#[test]
fn uninstall_app_removes_synthesized_agent() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let src = tmp.path().join("src");

    install_app(&aware, &write_app(&src, "inner", INNER_FLO)).success();
    assert!(aware.join("agents/inner/manifest.yaml").is_file());

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "uninstall", "inner"])
        .assert()
        .success();

    assert!(!aware.join("apps/inner").exists());
    assert!(
        !aware.join("agents/inner").exists(),
        "synth agent should be removed on uninstall"
    );
}
