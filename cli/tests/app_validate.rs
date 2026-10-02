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
fn validate_writes_no_lock_and_leaves_an_existing_one_untouched() {
    // `validate` answers a question about the file; `compile` writes the
    // `<app>.lock` approval `run` gates on. A validate that wrote one dropped
    // untracked or stale locks beside sources nobody compiled — and silently
    // re-approved an edited source by overwriting its lock (#571).
    let tmp = tempfile::tempdir().unwrap();
    let flo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("30-apps/_examples/welded-to-tc.app");
    std::fs::copy(&flo, tmp.path().join("welded-to-tc.app")).unwrap();
    let lock = tmp.path().join("welded-to-tc.lock");

    let validate = || {
        Command::cargo_bin("aware")
            .unwrap()
            .args(["app", "validate"])
            .arg(tmp.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("is valid"));
    };

    validate();
    assert!(!lock.exists(), "validate wrote a lock");

    std::fs::write(&lock, "stale approval\n").unwrap();
    validate();
    assert_eq!(std::fs::read_to_string(&lock).unwrap(), "stale approval\n");
}

/// Write a one-command agent whose `close` verb declares `mode: write`, the
/// shape of the escape hatch in #611.
fn write_mode_agent(home: &std::path::Path) {
    let agent_dir = home.join("agents").join("tekla");
    std::fs::create_dir_all(&agent_dir).unwrap();
    std::fs::write(
        agent_dir.join("manifest.yaml"),
        r#"agent: tekla
version: 1.0.0
description: x
stateful: false
license: MIT
transport:
  cli:
    binary: aware-tekla
commands:
  close:
    lifecycle: single
    mode: write
    category: curated
    description: Terminate the session.
"#,
    )
    .unwrap();
}

#[test]
fn write_node_inside_a_for_each_body_rejected_by_validate() {
    // #611: the safety gate iterated top-level nodes only, so wrapping a
    // write-mode node in `for-each: '[1]'` — a no-op loop over one element —
    // skipped the `safety:` requirement. The byte-identical node at the top
    // level was refused. Driven through the real CLI, since that asymmetry is
    // what a user actually met: `aware app validate` said "is valid".
    let home = tempfile::tempdir().unwrap();
    write_mode_agent(home.path());

    let appdir = tempfile::tempdir().unwrap();
    std::fs::write(
        appdir.path().join("nested.flo"),
        r#"app: nested-write
version: 0.0.1
description: x
requires: []
connections: []
nodes:
  - id: loop
    for-each: '[1]'
    do:
      - id: shutdown
        agent: tekla
        command: close
"#,
    )
    .unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .args(["app", "validate"])
        .arg(appdir.path())
        .assert()
        .failure()
        .code(3)
        // Reported by the scoped id the lock uses, so the author can find it.
        .stdout(
            predicate::str::contains("E_APP_WRITE_WITHOUT_SAFETY")
                .and(predicate::str::contains("loop.shutdown")),
        );
}

#[test]
fn write_node_inside_a_for_each_body_accepted_when_it_declares_safety() {
    // The other half: the recursion must not refuse every nested write. A body
    // node carrying `safety:` satisfies the contract, exactly as it does at the
    // top level — otherwise the fix for #611 would break every legitimate
    // `for-each` over a write command.
    let home = tempfile::tempdir().unwrap();
    write_mode_agent(home.path());

    let appdir = tempfile::tempdir().unwrap();
    std::fs::write(
        appdir.path().join("nested.flo"),
        r#"app: nested-write-safe
version: 0.0.1
description: x
requires: []
connections: []
nodes:
  - id: loop
    for-each: '[1]'
    do:
      - id: shutdown
        agent: tekla
        command: close
        safety:
          transaction: true
          snapshot: true
"#,
    )
    .unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .args(["app", "validate"])
        .arg(appdir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("is valid"));
}
