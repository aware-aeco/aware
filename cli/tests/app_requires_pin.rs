//! `requires:` agent-version pins are enforced (#349).
//!
//! Before this, an app's `requires:` block was parsed, echoed by `app show` and
//! resolved into the install-time `lockfile.yaml`, but no command ever compared
//! a declared pin against the version actually installed — so an app pinned to
//! `@9.9.x`, or to the *old* contract of an agent that had since taken a
//! breaking major bump, compiled and ran clean against whatever happened to be
//! on the machine.
//!
//! These drive the real binary rather than the validator, because the gap was
//! never in the check (there wasn't one) but in the wiring: each gate has to
//! call it, and each has a deliberately different posture — `compile` and `run`
//! refuse, `install` warns, `validate` stays silent about the environment.

mod common;

use assert_cmd::Command;
use predicates::prelude::*;

/// An `AWARE_HOME` with one agent installed at `version`, plus a source dir
/// holding a one-node app that pins it as `pin`.
fn fixture(version: &str, pin: &str) -> (tempfile::TempDir, std::path::PathBuf) {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    let agent_dir = home.join("agents").join("probe-agent");
    std::fs::create_dir_all(&agent_dir).unwrap();
    std::fs::write(
        agent_dir.join("manifest.yaml"),
        format!(
            "agent: probe-agent\nversion: {version}\ndescription: x\nstateful: false\n\
             license: MIT\ntransport:\n  cli:\n    binary: aware-probe\ncommands:\n  probe:\n    \
             lifecycle: single\n    description: x\n    mode: read\n"
        ),
    )
    .unwrap();

    let src = tmp.path().join("pin-test");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(
        src.join("pin-test.flo"),
        format!(
            "app: pin-test\nversion: 0.1.0\ndescription: pin enforcement fixture\n\
             requires:\n  - probe-agent@{pin}\nlayout: linear\nnodes:\n  - id: probe\n    \
             agent: probe-agent\n    command: probe\n"
        ),
    )
    .unwrap();
    (tmp, src)
}

fn aware(home: &std::path::Path) -> Command {
    let mut c = Command::cargo_bin("aware").unwrap();
    c.env("AWARE_HOME", home);
    c
}

#[test]
fn compile_refuses_an_app_whose_pin_the_installed_agent_does_not_satisfy() {
    // The issue's own reproduction, against the real binary: `@9.9.x` used to
    // compile clean and exit 0.
    let (tmp, src) = fixture("1.3.0", "9.9.x");
    aware(&tmp.path().join("home"))
        .args(["app", "compile"])
        .arg(src.join("pin-test.flo"))
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("E_APP_AGENT_PIN_UNSATISFIED"))
        // The operator has to be able to tell which side is wrong.
        .stderr(predicate::str::contains("probe-agent@9.9.x"))
        .stderr(predicate::str::contains("1.3.0"));

    // And no lock is left behind — the lock is the approved artifact, so a
    // refused compile must not produce one.
    assert!(
        !src.join("pin-test.lock").exists(),
        "a refused compile must not write a lockfile"
    );
}

#[test]
fn compile_accepts_an_app_whose_pin_is_satisfied() {
    // The negative control: without this, a check that refused everything would
    // still pass the test above.
    let (tmp, src) = fixture("1.3.0", "1.3.x");
    aware(&tmp.path().join("home"))
        .args(["app", "compile"])
        .arg(src.join("pin-test.flo"))
        .assert()
        .success();
    assert!(src.join("pin-test.lock").is_file());
}

#[test]
fn run_refuses_an_app_whose_pin_the_installed_agent_does_not_satisfy() {
    // The realistic case behind the issue: an app pinned to the OLD contract
    // (`0.1.x`) running against the agent that took the breaking bump (#343).
    // Checked against the live catalogue, so an agent swapped out *after* the
    // app was installed is caught too.
    let (tmp, src) = fixture("1.3.0", "0.1.x");
    let home = tmp.path().join("home");
    aware(&home)
        .args(["app", "install"])
        .arg(&src)
        .assert()
        .success();

    aware(&home)
        .args(["app", "run", "pin-test", "--dry-run"])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("E_APP_AGENT_PIN_UNSATISFIED"));
}

#[test]
fn install_warns_but_still_installs() {
    // Installing an app before the agent it pins is legitimate (#170), and the
    // matching version may still be on its way — so install names the mismatch
    // and lets compile/run be the refusals.
    let (tmp, src) = fixture("1.3.0", "0.1.x");
    aware(&tmp.path().join("home"))
        .args(["app", "install"])
        .arg(&src)
        .assert()
        .success()
        .stderr(predicate::str::contains("W_APP_AGENT_PIN_UNSATISFIED"));
}

#[test]
fn simulate_refuses_an_unreadable_pin_but_still_ignores_a_version_mismatch() {
    // The `--simulate` exemption is about the ENVIRONMENT: every node is stubbed
    // and no binary is contacted, so which version happens to be installed cannot
    // matter. Whether a constraint can be *read* is not about the environment —
    // it is a fact about the file, true on every machine — so the same exemption
    // must not swallow it. `run` never calls `validate_app`, so an app edited in
    // place under `~/.aware/apps/` simulated clean with a pin nothing could parse.
    //
    // The two assertions are the whole point: moving the check out of the
    // exemption must not drag the version check out with it, or `--simulate`
    // stops being the way to check a composition before its agents are in place.
    // Install with a readable pin, then break it in place. That is the realistic
    // route — `app install` refuses a malformed pin through `validate_app`, so the
    // only way one reaches `~/.aware/apps/` is an older CLI or an edit afterwards.
    let (tmp, src) = fixture("1.3.0", "1.3.x");
    let home = tmp.path().join("home");
    aware(&home)
        .args(["app", "install"])
        .arg(&src)
        .assert()
        .success();
    let installed = home.join("apps/pin-test/pin-test.flo");
    let body = std::fs::read_to_string(&installed).unwrap();
    std::fs::write(
        &installed,
        body.replace("probe-agent@1.3.x", "probe-agent@not-a-version"),
    )
    .unwrap();

    aware(&home)
        .args(["app", "run", "pin-test", "--simulate"])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("E_APP_REQUIRES_MALFORMED"));

    // …while a well-formed but UNSATISFIED pin still simulates clean, because
    // that one genuinely is about the environment.
    let (tmp2, src2) = fixture("1.3.0", "9.9.x");
    let home2 = tmp2.path().join("home");
    aware(&home2)
        .args(["app", "install"])
        .arg(&src2)
        .assert()
        .success();
    aware(&home2)
        .args(["app", "run", "pin-test", "--simulate"])
        .assert()
        .success();
}

#[test]
fn a_padded_id_still_resolves_into_the_install_lockfile() {
    // The id normalisation had to reach *every* consumer, and this is the one it
    // initially missed. `write_app_lockfile` turns the id into a directory name,
    // so `"probe-agent @1.3.x"` looked for `agents/probe-agent /` and missed —
    // silently, because that resolution is best-effort. The result was the worst
    // of both: the pin accepted and enforced everywhere else, while the agent it
    // names went absent from the app's `lockfile.yaml`.
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    let agent_dir = home.join("agents").join("probe-agent");
    std::fs::create_dir_all(&agent_dir).unwrap();
    std::fs::write(
        agent_dir.join("manifest.yaml"),
        "agent: probe-agent\nversion: 1.3.0\ndescription: x\nstateful: false\nlicense: MIT\n\
         transport:\n  cli:\n    binary: aware-probe\ncommands:\n  probe:\n    \
         lifecycle: single\n    description: x\n    mode: read\n",
    )
    .unwrap();

    let src = tmp.path().join("pin-test");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(
        src.join("pin-test.flo"),
        "app: pin-test\nversion: 0.1.0\ndescription: padded but satisfied pin\n\
         requires:\n  - \"probe-agent @1.3.x\"\nlayout: linear\nnodes:\n  - id: probe\n    \
         agent: probe-agent\n    command: probe\n",
    )
    .unwrap();

    aware(&home)
        .args(["app", "install"])
        .arg(&src)
        .assert()
        .success();

    let lockfile = std::fs::read_to_string(home.join("apps/pin-test/lockfile.yaml")).unwrap();
    assert!(
        lockfile.contains("probe-agent: 1.3.0"),
        "the padded id must resolve into resolved-agents; got:\n{lockfile}"
    );
    // And the key is the normalised id, not the padded spelling — a lockfile
    // keyed `\"probe-agent \"` would read as resolved while matching nothing later.
    assert!(
        !lockfile.contains("probe-agent : "),
        "resolved-agents must be keyed by the normalised id; got:\n{lockfile}"
    );
}

#[test]
fn validate_judges_the_file_not_the_machine() {
    // `app validate` deliberately gives the same verdict everywhere: an
    // unreadable pin is a fact about the file and is rejected, while "which
    // version happens to be installed" is ambient state and is not — the same
    // split `app validate` already applies to the #308 missing-agent check.
    let (tmp, src) = fixture("1.3.0", "0.1.x");
    let home = tmp.path().join("home");
    aware(&home)
        .args(["app", "validate"])
        .arg(src.join("pin-test.flo"))
        .assert()
        .success()
        .stdout(predicate::str::contains("is valid"));

    let (tmp2, src2) = fixture("1.3.0", "not-a-version");
    aware(&tmp2.path().join("home"))
        .args(["app", "validate"])
        .arg(src2.join("pin-test.flo"))
        .assert()
        .failure()
        .code(3)
        .stdout(predicate::str::contains("E_APP_REQUIRES_MALFORMED"));
}

/// An exposed inner app pinning `probe-agent@<pin>`, and an outer app that
/// composes it. The outer app pins nothing, so its own pre-flight is clean and
/// only the nested check can catch the inner app's pin.
///
/// The inner app **dispatches** to `probe-agent`. An earlier version of this
/// fixture pinned the agent while containing only inline glue, which made it
/// test a shape that cannot occur: a pin on an agent no node can reach is not
/// enforced at all now (see `a_frozen_only_agent_is_not_version_gated`), so
/// that fixture would have passed whatever the gate did.
fn nested_fixture(installed_version: &str, inner_pin: &str) -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    let agent_dir = home.join("agents").join("probe-agent");
    std::fs::create_dir_all(&agent_dir).unwrap();
    std::fs::write(
        agent_dir.join("manifest.yaml"),
        format!(
            "agent: probe-agent\nversion: {installed_version}\ndescription: x\nstateful: false\n\
             license: MIT\ntransport:\n  cli:\n    binary: aware-probe\ncommands:\n  probe:\n    \
             lifecycle: single\n    description: x\n    mode: read\n"
        ),
    )
    .unwrap();

    for (name, body) in [
        (
            "inner",
            format!(
                "app: inner\nversion: 0.2.0\ndescription: an exposed inner app\n\
                 exposes-as-agent: true\nexposed-commands:\n  run:\n    lifecycle: single\n    \
                 inputs:\n      phase:\n        type: string\n    outputs:\n      type: single\n      \
                 schema:\n        ok: bool\nnodes:\n  - id: probe\n    agent: probe-agent\n    \
                 command: probe\nrequires:\n  - probe-agent@{inner_pin}\n"
            ),
        ),
        (
            "outer",
            "app: outer\nversion: 0.1.0\ndescription: composes inner as an agent\nnodes:\n  \
             - id: call-inner\n    agent: inner\n    command: run\n    config:\n      \
             phase: design\nconnections: []\nrequires: []\n"
                .to_string(),
        ),
    ] {
        let dir = tmp.path().join("src").join(name);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join(format!("{name}.flo")), body).unwrap();
        aware(&home)
            .args(["app", "install"])
            .arg(&dir)
            .assert()
            .success();
    }
    tmp
}

#[test]
fn a_nested_exposed_app_is_refused_when_its_own_pin_is_unsatisfied() {
    // `aware app run` pre-flights the app the operator NAMED. A nested exposed
    // app is reached only through the app transport, so without a check there
    // its `requires:` was never consulted — upgrading one of its agents to an
    // incompatible version after install still dispatched, which is the same
    // live-catalogue gap the pre-flight exists to close, one level down.
    let tmp = nested_fixture("1.3.0", "0.1.x");
    aware(&tmp.path().join("home"))
        .args(["app", "run", "outer"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("E_APP_AGENT_PIN_UNSATISFIED"))
        // Names the app-backed agent, so the operator knows which hop refused.
        .stderr(predicate::str::contains("inner"));
}

#[test]
fn a_nested_exposed_app_with_a_satisfied_pin_is_not_stopped_by_the_pin_gate() {
    // Negative control: without it, a check that refused every nested dispatch
    // would still pass the test above.
    //
    // It asserts "not refused for a pin reason" rather than plain success,
    // because the inner app now really dispatches to `probe-agent` and no
    // `aware-probe` binary exists in the fixture — so the run legitimately fails
    // *later*, at the transport. That distinction is the whole assertion: the
    // gate let it through and something downstream stopped it.
    let tmp = nested_fixture("1.3.0", "1.3.x");
    aware(&tmp.path().join("home"))
        .args(["app", "run", "outer"])
        .assert()
        .stderr(predicate::str::contains("PIN_UNSATISFIED").not())
        .stderr(predicate::str::contains("REQUIRES_MALFORMED").not())
        .stderr(predicate::str::contains("AGENT_NOT_INSTALLED").not())
        // Reached dispatch — the pin gate is behind it.
        .stderr(predicate::str::contains("aware-probe"));
}

#[test]
fn a_nested_exposed_app_whose_agent_is_missing_is_named_not_left_to_the_transport() {
    // The nested path never ran the missing-agent check at all: the command-level
    // pre-flight only ever sees the app the operator named, so a nested node whose
    // agent isn't installed died at the transport with a bare `os error 3` naming
    // neither the node nor the agent.
    let tmp = nested_fixture("1.3.0", "1.3.x");
    let home = tmp.path().join("home");
    std::fs::remove_dir_all(home.join("agents/probe-agent")).unwrap();

    aware(&home)
        .args(["app", "run", "outer"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("E_APP_AGENT_NOT_INSTALLED"))
        .stderr(predicate::str::contains("probe-agent"));
}

#[test]
fn run_refuses_an_installed_app_whose_pin_became_unreadable() {
    // `app run` never calls `validate_app` — it runs the catalogue checks only.
    // So an app that reaches `~/.aware/apps/` with an unreadable pin (installed by
    // an older CLI, or edited in place afterwards) must still be refused by the pin
    // gate itself. When that gate merely skipped what it couldn't parse, this app
    // ran against whatever happened to be installed.
    let (tmp, src) = fixture("1.3.0", "1.3.x");
    let home = tmp.path().join("home");
    aware(&home)
        .args(["app", "install"])
        .arg(&src)
        .assert()
        .success();

    // Edit the INSTALLED copy, behind install/compile's back.
    let installed_flo = home.join("apps/pin-test/pin-test.flo");
    let body = std::fs::read_to_string(&installed_flo).unwrap();
    std::fs::write(
        &installed_flo,
        body.replace("probe-agent@1.3.x", "probe-agent@not-a-version"),
    )
    .unwrap();

    aware(&home)
        .args(["app", "run", "pin-test", "--dry-run"])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("E_APP_REQUIRES_MALFORMED"))
        // Refused for being unreadable, not dressed up as a version mismatch.
        .stderr(predicate::str::contains("unreadable"));
}

#[test]
fn run_refuses_an_installed_app_whose_pin_lost_its_agent_id() {
    // The same in-place edit, one character smaller: `@1.3.x` with the id dropped.
    // This one parsed, so the syntax check was silent, and the empty id matched no
    // dispatchable agent, so the catalogue check took its "declared but
    // unreachable" exemption — the app ran with an entry that looked like a
    // constraint and was enforced by nothing.
    let (tmp, src) = fixture("1.3.0", "1.3.x");
    let home = tmp.path().join("home");
    aware(&home)
        .args(["app", "install"])
        .arg(&src)
        .assert()
        .success();

    let installed_flo = home.join("apps/pin-test/pin-test.flo");
    let body = std::fs::read_to_string(&installed_flo).unwrap();
    std::fs::write(
        &installed_flo,
        body.replace("probe-agent@1.3.x", "\"@1.3.x\""),
    )
    .unwrap();

    aware(&home)
        .args(["app", "run", "pin-test", "--dry-run"])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("E_APP_REQUIRES_MALFORMED"))
        // Named for what is actually wrong — the missing id, not the pin.
        .stderr(predicate::str::contains("names no agent"));
}

#[test]
fn a_nested_exposed_app_with_an_unreadable_pin_is_refused_too() {
    // The nested dispatch path calls the same check with no `validate_app` either,
    // so the same fail-closed rule has to hold one level down.
    let tmp = nested_fixture("1.3.0", "1.3.x");
    let home = tmp.path().join("home");
    let inner_flo = home.join("apps/inner/inner.flo");
    let body = std::fs::read_to_string(&inner_flo).unwrap();
    std::fs::write(
        &inner_flo,
        body.replace("probe-agent@1.3.x", "probe-agent@not-a-version"),
    )
    .unwrap();

    aware(&home)
        .args(["app", "run", "outer"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("E_APP_REQUIRES_MALFORMED"));
}

#[test]
fn an_uninstalled_pinned_agent_is_reported_as_missing_not_as_a_pin_mismatch() {
    // Two different gaps with two different remedies. A pin verdict needs a
    // version to judge; naming the same gap twice would send the operator to
    // `agent install <id>@<pin>` when the agent isn't there at all.
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    std::fs::create_dir_all(home.join("agents")).unwrap();
    let src = tmp.path().join("pin-test");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(
        src.join("pin-test.flo"),
        "app: pin-test\nversion: 0.1.0\ndescription: x\nrequires:\n  - probe-agent@9.9.x\n\
         layout: linear\nnodes:\n  - id: probe\n    agent: probe-agent\n    command: probe\n",
    )
    .unwrap();

    aware(&home)
        .args(["app", "compile"])
        .arg(src.join("pin-test.flo"))
        .assert()
        .success() // missing agents only warn at compile (#308/#170)
        .stderr(predicate::str::contains("W_APP_AGENT_NOT_INSTALLED"))
        .stderr(predicate::str::contains("PIN_UNSATISFIED").not());
}
