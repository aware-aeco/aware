mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn valid_app_exits_0() {
    let tmp = tempfile::tempdir().unwrap();
    let flo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("30-apps/_examples/welded-to-tc.flo");
    std::fs::copy(&flo, tmp.path().join("welded-to-tc.flo")).unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .args(["app", "validate"])
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("is valid"));
}

#[test]
fn cyclic_app_exits_3() {
    let tmp = tempfile::tempdir().unwrap();
    let cyclic = r#"app: cyc
version: 0.0.1
description: x
nodes:
  - id: a
  - id: b
connections:
  - { from: a, to: b }
  - { from: b, to: a }
requires: []
"#;
    std::fs::write(tmp.path().join("cyc.flo"), cyclic).unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .args(["app", "validate"])
        .arg(tmp.path())
        .assert()
        .failure()
        .code(3)
        .stdout(predicate::str::contains("E_APP_CYCLE"));
}

#[test]
fn app_referencing_planned_agent_rejected_by_validate() {
    // An agent declared `status: planned` (no shipped transport binary) must make
    // apps referencing it fail validate, not fail at run with "program not found" (#161).
    let home = tempfile::tempdir().unwrap();
    let agent_dir = home.path().join("agents").join("html-report");
    std::fs::create_dir_all(&agent_dir).unwrap();
    std::fs::write(
        agent_dir.join("manifest.yaml"),
        "agent: html-report\nversion: 0.1.0\ndescription: x\nstateful: false\nstatus: planned\n\
         license: MIT\ntransport:\n  cli:\n    binary: aware-html-report\ncommands:\n  render:\n    lifecycle: single\n    description: x\n",
    )
    .unwrap();

    let appdir = tempfile::tempdir().unwrap();
    std::fs::write(
        appdir.path().join("report.flo"),
        "app: uses-planned\nversion: 0.0.1\ndescription: x\nrequires: []\nnodes:\n  - id: report\n    agent: html-report\n    command: render\n",
    )
    .unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .args(["app", "validate"])
        .arg(appdir.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("E_APP_AGENT_UNAVAILABLE"));
}

#[test]
fn inline_shape_kind_rejected_by_validate() {
    // kind: shape parses + would compile, but the runtime only runs `predicate`;
    // validate must reject it up front (#160).
    let tmp = tempfile::tempdir().unwrap();
    let app = r#"app: inline-shape
version: 0.1.0
description: inline shape repro
requires: []
nodes:
  - id: passthrough
    inline:
      kind: shape
      description: reshape a value
      code: "() => ({ ok: true })"
"#;
    std::fs::write(tmp.path().join("inline-shape.flo"), app).unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .args(["app", "validate"])
        .arg(tmp.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("E_APP_INLINE_KIND"));
}

#[test]
fn inline_shape_kind_rejected_by_compile() {
    // compile must not produce a lock for an app the runtime can't execute (#160).
    let tmp = tempfile::tempdir().unwrap();
    let app = r#"app: inline-shape
version: 0.1.0
description: inline shape repro
requires: []
nodes:
  - id: passthrough
    inline:
      kind: shape
      description: reshape a value
      code: "() => ({ ok: true })"
"#;
    std::fs::write(tmp.path().join("inline-shape.flo"), app).unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .args(["app", "compile"])
        .arg(tmp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("E_APP_INLINE_KIND"));
}
