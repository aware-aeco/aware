mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn valid_app_exits_0() {
    let tmp = tempfile::tempdir().unwrap();
    let flo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("30-apps/_examples/welded-to-tc.app");
    std::fs::copy(&flo, tmp.path().join("welded-to-tc.app")).unwrap();

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

const ATOM_PREDICATE_APP: &str = r#"app: inline-atom
version: 0.1.0
description: body-less atom predicate repro
requires: []
nodes:
  - id: recent
    inline:
      kind: predicate
      description: Issues newer than last Friday
      atom: 'atom://generic/is-newer-than'
      inputs:
        threshold: '2026-09-12T00:00:00Z'
"#;

#[test]
fn body_less_atom_predicate_rejected_by_validate() {
    // The shape app-spec § Atom references publishes. No `atom://` resolver
    // exists, so the predicate has no executable body and the runtime used to
    // treat it as a literal `true` — a gate that passed everything while
    // reporting an honest-looking {"pass": true} (#554).
    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(tmp.path().join("inline-atom.flo"), ATOM_PREDICATE_APP).unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .args(["app", "validate"])
        .arg(tmp.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("E_APP_INLINE_NO_BODY"))
        .stdout(predicate::str::contains("atom://generic/is-newer-than"));
}

#[test]
fn body_less_atom_predicate_rejected_by_compile() {
    // compile must not mint a lock the runtime would refuse to execute — the
    // lock is the approved artifact, so a body-less gate must never reach one.
    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(tmp.path().join("inline-atom.flo"), ATOM_PREDICATE_APP).unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .args(["app", "compile"])
        .arg(tmp.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("E_APP_INLINE_NO_BODY"));

    assert!(
        !tmp.path().join("inline-atom.lock").exists(),
        "compile refused the app but still wrote a lock"
    );
}

#[test]
fn validate_announces_the_lock_sidecar_it_writes() {
    // `validate` emits `<app>.lock` by design (app-spec.md § Lockfile sidecar → CLI;
    // cli-roadmap.md § v0.24), but it used to write the file and say nothing — so a
    // verb users reach for to ASK A QUESTION dropped an unmentioned build artifact
    // into whatever directory the source lived in (#571). The artifact stays; the
    // silence does not.
    let tmp = tempfile::tempdir().unwrap();
    let flo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("30-apps/_examples/welded-to-tc.app");
    std::fs::copy(&flo, tmp.path().join("welded-to-tc.app")).unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .args(["app", "validate"])
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("is valid"))
        .stdout(predicate::str::contains("welded-to-tc.lock"));

    assert!(
        tmp.path().join("welded-to-tc.lock").exists(),
        "validate stopped emitting the documented sidecar"
    );
}

#[test]
fn validate_still_reports_valid_when_the_lock_sidecar_cannot_be_written() {
    // An app's validity is a fact about the FILE. Writing the sidecar is a side
    // effect, so a location that refuses the write must not restate a valid app as
    // `error: internal` (#571) — the question a user asks about a file has to stay
    // answerable in a directory they cannot write to.
    //
    // The unwritable location is a DIRECTORY already holding the `.lock` name: that
    // fails with EISDIR for every user, where a read-only parent directory would be
    // silently bypassed when the suite runs as root (as it does in CI containers).
    let tmp = tempfile::tempdir().unwrap();
    let flo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("30-apps/_examples/welded-to-tc.app");
    std::fs::copy(&flo, tmp.path().join("welded-to-tc.app")).unwrap();
    std::fs::create_dir(tmp.path().join("welded-to-tc.lock")).unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .args(["app", "validate"])
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("is valid"))
        .stderr(predicate::str::contains("could not be written"));
}

#[test]
fn validate_still_fails_when_the_app_itself_is_invalid() {
    // The warning above is scoped to the ARTIFACT. A bad app must still fail, so
    // downgrading the write failure cannot become a blanket "validate always
    // succeeds" — with the sidecar name blocked AND the app cyclic, the verdict
    // still wins (#571).
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
    std::fs::create_dir(tmp.path().join("cyc.lock")).unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .args(["app", "validate"])
        .arg(tmp.path())
        .assert()
        .failure()
        .code(3)
        .stdout(predicate::str::contains("E_APP_CYCLE"));
}
