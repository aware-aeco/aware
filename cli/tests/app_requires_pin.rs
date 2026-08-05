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

#[test]
fn an_unsatisfied_pin_never_prescribes_an_agent_command() {
    // Five findings landed on the command this message used to print, ending in a
    // DESTRUCTIVE one: `uninstall` then reinstall-by-version silently assumes the
    // registry, so for a locally-installed agent it removed the only copy and then
    // failed. The validator knows neither the agent's provenance nor what versions
    // the registry holds, so every sequence it printed was wrong for some real
    // case. It now states the goal and names no command — a message that
    // prescribes nothing cannot prescribe something harmful.
    for pin in ["9.9.3", "9.9.x", "9.9.3+build.1", "9.9.3-rc.1"] {
        let (tmp, src) = fixture("1.3.0", pin);
        let out = aware(&tmp.path().join("home"))
            .args(["app", "compile"])
            .arg(src.join("pin-test.flo"))
            .output()
            .unwrap();
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            stderr.contains("E_APP_AGENT_PIN_UNSATISFIED"),
            "pin {pin} should still be refused: {stderr}"
        );
        // The goal survives — the operator still learns what is needed.
        assert!(
            stderr.contains(&format!("install a version matching {pin}")),
            "pin {pin} lost the goal: {stderr}"
        );
        for forbidden in [
            "aware agent uninstall",
            "aware agent install",
            "aware agent update",
        ] {
            assert!(
                !stderr.contains(forbidden),
                "pin {pin} prescribed `{forbidden}`, which cannot be known to work here: {stderr}"
            );
        }
    }
}

#[test]
fn a_nested_unreadable_pin_is_refused_under_simulate_too() {
    // The one place the file-level rule still didn't hold. `--simulate` short-
    // circuits in `Orchestrator` with a synthesized output *before* the app
    // transport, so `resolve_exposed` — which reads a nested app's pins on a real
    // run — is never reached, and the backing app was never loaded to look at.
    // The top-level check moving out of the simulation exemption fixed only the
    // app the operator named; one level down, an unreadable constraint was
    // reported by nothing at all.
    //
    // Reproduced before fixing: `aware app run outer --simulate` exited 0 with an
    // empty stderr against an inner app pinning `probe-agent@not-a-version`.
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
        .args(["app", "run", "outer", "--simulate"])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("E_APP_REQUIRES_MALFORMED"))
        // Names the hop, or the pin appears in neither the app the operator named
        // nor anything else they can see from here.
        .stderr(predicate::str::contains("inner"));
}

#[test]
fn simulate_still_ignores_a_nested_version_mismatch() {
    // The negative control for the test above, and the reason it is scoped to
    // *readability*. Moving the nested file check out of the simulation exemption
    // must not drag the catalogue checks out with it: which version happens to be
    // installed is a fact about the environment, and `--simulate` contacts no
    // binary at either level. Without this, "refuse every nested app under
    // simulate" would pass the test above just as well.
    //
    // `0.1.x` against an installed `1.3.0` is exactly what `app run outer` (no
    // `--simulate`) refuses two tests up, so the pair pins the boundary.
    let tmp = nested_fixture("1.3.0", "0.1.x");
    aware(&tmp.path().join("home"))
        .args(["app", "run", "outer", "--simulate"])
        .assert()
        .success();
}

#[test]
fn one_unreadable_manifest_elsewhere_does_not_switch_the_nested_check_off() {
    // The first cut of the nested pre-flight read the catalogue through
    // `discover_agents`, which aborts the whole walk on the first manifest that
    // won't parse (`loader.rs:48-58`) — so it returned `Err`, the pre-flight
    // returned "no issues", and ONE unrelated broken agent anywhere under
    // `~/.aware/agents/` silently disabled the check for every app on the machine.
    // Fail-open, and far wider than the intended "skip the unloadable thing".
    //
    // The manifests are now loaded one per dispatchable agent, so the silence is
    // scoped to the agent that earned it.
    let tmp = nested_fixture("1.3.0", "1.3.x");
    let home = tmp.path().join("home");
    let inner_flo = home.join("apps/inner/inner.flo");
    let body = std::fs::read_to_string(&inner_flo).unwrap();
    std::fs::write(
        &inner_flo,
        body.replace("probe-agent@1.3.x", "probe-agent@not-a-version"),
    )
    .unwrap();

    // An installed agent this app never mentions, with a manifest nothing can
    // parse — the collateral damage that used to take the check down with it.
    let wreck = home.join("agents").join("unrelated-wreck");
    std::fs::create_dir_all(&wreck).unwrap();
    std::fs::write(
        wreck.join("manifest.yaml"),
        "agent: [this is not\n  a manifest\n",
    )
    .unwrap();

    aware(&home)
        .args(["app", "run", "outer", "--simulate"])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("E_APP_REQUIRES_MALFORMED"))
        .stderr(predicate::str::contains("inner"));
}

#[test]
fn a_mixed_transport_agent_is_not_read_as_app_backed_by_the_nested_check() {
    // `effective_transport` resolves cli > rest > app > builtin and its doc comment
    // is explicit that every guard asking "which transport is this?" must resolve
    // through it, "otherwise the guard and dispatch disagree (#215 review)". The
    // first cut of this pre-flight probed `transport.app` raw instead, so a manifest
    // carrying BOTH `cli:` and `app:` — which dispatches on `cli` and never loads
    // the backing app — had that app's `requires:` read anyway, and an unreadable
    // pin in a file no node would touch refused the run. A false REFUSAL, the
    // opposite direction from the fail-opens this gate was built to close.
    let tmp = nested_fixture("1.3.0", "1.3.x");
    let home = tmp.path().join("home");

    // Break the backing app's pin: if the check fires at all, it fires on this.
    let inner_flo = home.join("apps/inner/inner.flo");
    let body = std::fs::read_to_string(&inner_flo).unwrap();
    std::fs::write(
        &inner_flo,
        body.replace("probe-agent@1.3.x", "probe-agent@not-a-version"),
    )
    .unwrap();

    // …and give the synthesized agent a higher-priority transport, so dispatch
    // resolves it to `cli` and the app behind it is never reached.
    let synth = home.join("agents/inner/manifest.yaml");
    let manifest = std::fs::read_to_string(&synth).unwrap();
    assert!(
        manifest.contains("backed-by: inner"),
        "fixture must start app-backed, else this asserts nothing: {manifest}"
    );
    std::fs::write(
        &synth,
        manifest.replace(
            "transport:\n",
            "transport:\n  cli:\n    binary: aware-inner-cli\n",
        ),
    )
    .unwrap();

    aware(&home)
        .args(["app", "run", "outer", "--simulate"])
        .assert()
        .success();
}

#[test]
fn an_unreadable_manifest_for_a_dispatched_agent_is_a_fault_not_silence() {
    // The other half of the silence question. Skipping an agent whose manifest is
    // ABSENT is right — `--simulate` is how you check a composition before its
    // agents exist. Skipping one that is installed but cannot be PARSED is not: a
    // check that cannot read the manifest it must follow cannot clear the pins
    // behind it either, and `--simulate` is the only gate where this showed,
    // because a real run's `discover_agents` already fails on the same manifest.
    //
    // Before the fix, `let Ok(..) else { continue }` swallowed both cases alike,
    // so corrupting the manifest of the very agent under test switched its own
    // nested check off and the malformed pin one level down simulated clean.
    let tmp = nested_fixture("1.3.0", "1.3.x");
    let home = tmp.path().join("home");
    let inner_flo = home.join("apps/inner/inner.flo");
    let body = std::fs::read_to_string(&inner_flo).unwrap();
    std::fs::write(
        &inner_flo,
        body.replace("probe-agent@1.3.x", "probe-agent@not-a-version"),
    )
    .unwrap();

    // `inner` is the app-backed agent `outer` dispatches to. Corrupt ITS manifest.
    std::fs::write(
        home.join("agents/inner/manifest.yaml"),
        "agent: [this is not\n  a manifest\n",
    )
    .unwrap();

    aware(&home)
        .args(["app", "run", "outer", "--simulate"])
        .assert()
        .failure()
        .code(3)
        // The loader's own message, which names the manifest it could not read.
        .stderr(predicate::str::contains("inner"));
}

#[test]
fn a_manifest_that_is_not_a_file_is_a_fault_too_not_an_absence() {
    // `is_file()` cannot tell "no manifest here" from "something is here that I
    // cannot read as one" — it answers `false` to both, so gating on it left a
    // narrower copy of the fail-open above: a directory (or an unreadable entry)
    // where `manifest.yaml` belongs skipped the agent in silence. Only a
    // NotFound is an absence; everything else propagates.
    let tmp = nested_fixture("1.3.0", "1.3.x");
    let home = tmp.path().join("home");
    let inner_flo = home.join("apps/inner/inner.flo");
    let body = std::fs::read_to_string(&inner_flo).unwrap();
    std::fs::write(
        &inner_flo,
        body.replace("probe-agent@1.3.x", "probe-agent@not-a-version"),
    )
    .unwrap();

    let manifest = home.join("agents/inner/manifest.yaml");
    std::fs::remove_file(&manifest).unwrap();
    std::fs::create_dir(&manifest).unwrap();

    aware(&home)
        .args(["app", "run", "outer", "--simulate"])
        .assert()
        .failure()
        // Named, not just "failed": every agent manifest path contains
        // "manifest.yaml", so the agent has to be named too or the test would
        // pass on a read of any OTHER agent's manifest.
        .stderr(predicate::str::contains("manifest.yaml"))
        .stderr(predicate::str::contains("inner"));
}

#[test]
fn an_unreadable_backing_app_is_a_fault_too() {
    // The same rule one file further in, and the least excusable place for it:
    // the backing app's `.flo` is the file whose `requires:` this pre-flight
    // exists to read. Skipping it on a parse failure was the check going silent
    // about its own subject — so a nested app edited into invalid YAML simulated
    // clean, pins and all.
    //
    // "No .flo/.app in the directory" is still an absence and still skips; this
    // is present-and-unreadable.
    let tmp = nested_fixture("1.3.0", "1.3.x");
    let home = tmp.path().join("home");
    std::fs::write(
        home.join("apps/inner/inner.flo"),
        "app: inner\n  version: [not\n   valid: yaml\n",
    )
    .unwrap();

    aware(&home)
        .args(["app", "run", "outer", "--simulate"])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("inner.flo"))
        // Nailed to the LOADER, not to "something named that file": the pin-syntax
        // check names the backing app too, so without this the assertion would
        // also be satisfied by a malformed pin — the opposite of what is broken.
        .stderr(predicate::str::contains("E_APP_REQUIRES_MALFORMED").not())
        // …and it names the hop, so the operator learns which agent led here.
        .stderr(predicate::str::contains("app-backed agent"))
        // Exactly ONE class prefix. Naming the hop by stringifying the error
        // printed "validation failed:" twice, because `AwareError` Displays its
        // own prefix — so the wrap matches on the variant instead.
        .stderr(predicate::str::contains("validation failed").count(1));
}

#[test]
fn an_unreadable_backing_app_keeps_the_io_exit_code() {
    // The other arm of that wrap, and the one that was silently wrong: naming the
    // hop by re-formatting the error coerced BOTH of `load_app`'s classes into
    // `Validation`, moving an unreadable file from exit 1 to exit 3. `cli-spec.md`
    // keeps those distinct — 1 is a general failure, 3 is "validation failed" —
    // and a file that cannot be READ is not an invalid one. A real run agrees:
    // `resolve_exposed` yields `Io` for the same file.
    //
    // A directory where `inner.flo` belongs is the reachable shape:
    // `find_app_manifest` matches it on extension, then the read fails.
    let tmp = nested_fixture("1.3.0", "1.3.x");
    let home = tmp.path().join("home");
    let flo = home.join("apps/inner/inner.flo");
    std::fs::remove_file(&flo).unwrap();
    std::fs::create_dir(&flo).unwrap();

    aware(&home)
        .args(["app", "run", "outer", "--simulate"])
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("app-backed agent"))
        .stderr(predicate::str::contains("inner.flo"))
        // The io class, not validation — one prefix, and the right one.
        .stderr(predicate::str::contains("validation failed").not());
}

#[test]
fn a_path_shaped_agent_id_never_reads_a_manifest_outside_agents() {
    // The nested pre-flight joins a node's `agent:` id onto `agents/`, and that id
    // comes from the app FILE — nothing validates it as a path segment. Without a
    // guard, `agent: ../elsewhere` walks out of `agents/` and reads an arbitrary
    // `manifest.yaml`, the same traversal `loader::resolve_app_dir` already fences
    // for an app id.
    //
    // The fixture makes the traversal the ONLY way to reach the manifest: it sits
    // in `<home>/elsewhere/`, is app-backed by `inner`, and `inner` carries a
    // malformed pin. So a naive join reports E_APP_REQUIRES_MALFORMED and a fenced
    // one reports nothing — the assertion cannot pass by accident.
    let tmp = nested_fixture("1.3.0", "1.3.x");
    let home = tmp.path().join("home");
    let inner_flo = home.join("apps/inner/inner.flo");
    let body = std::fs::read_to_string(&inner_flo).unwrap();
    std::fs::write(
        &inner_flo,
        body.replace("probe-agent@1.3.x", "probe-agent@not-a-version"),
    )
    .unwrap();

    let outside = home.join("elsewhere");
    std::fs::create_dir_all(&outside).unwrap();
    std::fs::write(
        outside.join("manifest.yaml"),
        "agent: elsewhere\nversion: 1.0.0\ndescription: x\nstateful: false\nlicense: MIT\n\
         transport:\n  app:\n    backed-by: inner\ncommands:\n  run:\n    lifecycle: single\n    \
         description: x\n    mode: read\n",
    )
    .unwrap();

    std::fs::write(
        home.join("apps/outer/outer.flo"),
        "app: outer\nversion: 0.1.0\ndescription: names an agent by traversal\nnodes:\n  \
         - id: call\n    agent: ../elsewhere\n    command: run\nconnections: []\nrequires: []\n",
    )
    .unwrap();

    aware(&home)
        .args(["app", "run", "outer", "--simulate"])
        .assert()
        // Succeeds, not merely "fails for some other reason": a run that died on
        // an unrelated error would satisfy the absence of the code all by itself.
        .success()
        .stderr(predicate::str::contains("E_APP_REQUIRES_MALFORMED").not());
}

#[test]
fn a_path_shaped_agent_id_is_refused_at_real_dispatch_too() {
    // The pre-flight fence only covers the pre-flight. A REAL run reaches
    // `DispatchInvoker::transport_kind`, which joins the same file-controlled id
    // onto `agents/` before it knows the transport — so a traversal manifest
    // declaring `cli:`/`rest:` never touched the app-transport path where the
    // fence first went (Codex, round 3). The fence sits in that funnel now.
    //
    // Getting there is the hard part, and the first cut of this test did NOT:
    // `app run` pre-flights `missing_agents` before dispatching, and a node whose
    // `agent:` matches nothing installed dies right there with
    // E_APP_AGENT_NOT_INSTALLED — whose message also says "not installed", so the
    // test passed with the fence deleted (Codex, round 4). It needs a catalogue
    // entry that MATCHES the path-shaped id, which is what the decoy below is: a
    // manifest sitting at a perfectly ordinary `agents/decoy/`, whose own `agent:`
    // field declares the traversal. `discover_agents` keys on that field, so the
    // pre-flight is satisfied and dispatch really happens.
    let tmp = nested_fixture("1.3.0", "1.3.x");
    let home = tmp.path().join("home");

    // The manifest OUTSIDE `agents/` that a traversal would reach. Its binary name
    // is the marker: it can only appear in the output if the traversal was read.
    let outside = home.join("elsewhere");
    std::fs::create_dir_all(&outside).unwrap();
    std::fs::write(
        outside.join("manifest.yaml"),
        "agent: elsewhere\nversion: 1.0.0\ndescription: x\nstateful: false\nlicense: MIT\n\
         transport:\n  cli:\n    binary: traversal-marker-binary\ncommands:\n  probe:\n    \
         lifecycle: single\n    description: x\n    mode: read\n",
    )
    .unwrap();

    // The decoy: an ordinary catalogue entry whose declared id is the traversal,
    // so `missing_agents` finds the node's agent and lets the run proceed.
    let decoy = home.join("agents").join("decoy");
    std::fs::create_dir_all(&decoy).unwrap();
    std::fs::write(
        decoy.join("manifest.yaml"),
        "agent: ../elsewhere\nversion: 1.0.0\ndescription: x\nstateful: false\nlicense: MIT\n\
         transport:\n  cli:\n    binary: traversal-marker-binary\ncommands:\n  probe:\n    \
         lifecycle: single\n    description: x\n    mode: read\n",
    )
    .unwrap();

    std::fs::write(
        home.join("apps/outer/outer.flo"),
        "app: outer\nversion: 0.1.0\ndescription: names an agent by traversal\nnodes:\n  \
         - id: call\n    agent: ../elsewhere\n    command: probe\nconnections: []\nrequires: []\n",
    )
    .unwrap();

    aware(&home)
        .args(["app", "run", "outer"])
        .assert()
        .failure()
        // Refused as not-installed, which is what a path-shaped id is…
        .stderr(predicate::str::contains(
            "agent ../elsewhere is not installed",
        ))
        // …and NOT because the pre-flight got there first, which is how the
        // previous version of this test fooled itself.
        .stderr(predicate::str::contains("E_APP_AGENT_NOT_INSTALLED").not())
        // The DISPATCH load never happened: without the fence `transport_kind`
        // reads the planted manifest and gets as far as the binary it names.
        // (Read-only lookups earlier in the run — the long-running probe, the
        // orchestrator's mode lookups — still reach that file either way; those
        // joins are filed as #365.)
        .stderr(predicate::str::contains("traversal-marker-binary").not());
}

#[test]
fn a_path_shaped_backed_by_never_reads_an_app_outside_apps() {
    // The second manifest-controlled id in the same pre-flight: an app-backed
    // agent's `backed-by:`, joined onto `apps/` to find the app whose pins we
    // read. A synthesized manifest always names a plain id, but a hand-edited or
    // hand-installed one need not — and this is the only place that reads it
    // before dispatch, so it gets the same fence as the node's `agent:`.
    //
    // Same construction as the test above: the malformed pin lives in an app that
    // ONLY the traversal can reach, so an unfenced join reports it and a fenced
    // one is silent.
    let tmp = nested_fixture("1.3.0", "1.3.x");
    let home = tmp.path().join("home");

    let outside = home.join("elsewhere-app");
    std::fs::create_dir_all(&outside).unwrap();
    std::fs::write(
        outside.join("elsewhere-app.flo"),
        "app: elsewhere-app\nversion: 0.1.0\ndescription: outside apps/\n\
         exposes-as-agent: true\nexposed-commands:\n  run:\n    lifecycle: single\n    \
         outputs:\n      type: single\n      schema:\n        ok: bool\nnodes:\n  \
         - id: probe\n    agent: probe-agent\n    command: probe\n\
         requires:\n  - probe-agent@not-a-version\n",
    )
    .unwrap();

    // Point the installed agent at it by traversal. Everything else is untouched,
    // so `inner`'s own (satisfied) pin is the only other thing on this path.
    let inner_manifest = home.join("agents/inner/manifest.yaml");
    let body = std::fs::read_to_string(&inner_manifest).unwrap();
    assert!(
        body.contains("backed-by: inner"),
        "fixture changed: expected a synthesized `backed-by: inner`, got:\n{body}"
    );
    std::fs::write(
        &inner_manifest,
        body.replace("backed-by: inner", "backed-by: ../elsewhere-app"),
    )
    .unwrap();

    aware(&home)
        .args(["app", "run", "outer", "--simulate"])
        .assert()
        .success()
        .stderr(predicate::str::contains("E_APP_REQUIRES_MALFORMED").not());
}
