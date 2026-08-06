mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn describes_tekla_agent() {
    let home = common::aware_home();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["agent", "describe", "tekla"])
        .assert()
        .success()
        .stdout(predicate::str::contains("agent:"))
        .stdout(predicate::str::contains("tekla"))
        // The curated tekla manifest uses the version-agnostic `2025+` shorthand for
        // sdk-target (a structured form will land in a later hardening pass).
        .stdout(predicate::str::contains("2025+"))
        .stdout(predicate::str::contains("stateful:"))
        .stdout(predicate::str::contains("watch"))
        .stdout(predicate::str::contains("insert"))
        .stdout(predicate::str::contains("save-attributes"))
        .stdout(predicate::str::contains("drawing-identity.md"));
}

#[test]
fn missing_agent_returns_not_found_exit_7() {
    let home = common::aware_home();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["agent", "describe", "does-not-exist"])
        .assert()
        .failure()
        .code(7);
}

#[test]
fn json_describe_returns_envelope() {
    let home = common::aware_home();
    let output = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["--json", "agent", "describe", "tekla"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let v: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(v["ok"], true);
    assert_eq!(v["data"]["agent"], "tekla");
    // tekla currently declares 33 skills and 24 commands (grew from 23 with the
    // `bake-scene` verb, #235). The agent.rs unit tests have the same expectation
    // — see `cli/src/manifest/agent.rs`.
    assert_eq!(v["data"]["skill_count"], 33);
    assert_eq!(v["data"]["command_count"], 24);
}

// ── #363: which versions does the registry actually have? ─────────────────────

/// A catalog carrying THREE versions of one agent, so "lists them all" is
/// distinguishable from "prints the newest" — which is all `describe` did before.
const MULTI_VERSION_CATALOG: &str = r#"{
  "version": "1",
  "updated-at": "2026-06-16T00:00:00Z",
  "agents": {
    "probe-agent": {
      "versions": {
        "1.9.0":  { "description": "old", "status": "available", "stateful": false,
                    "transport": "cli", "command_count": 1, "skills": [] },
        "1.10.0": { "description": "middle", "status": "available", "stateful": false,
                    "transport": "cli", "command_count": 1, "skills": [] },
        "1.10.1": { "description": "newest", "status": "available", "stateful": false,
                    "transport": "cli", "command_count": 1, "skills": [] }
      }
    }
  }
}"#;

fn describe_available(catalog_body: &str, json: bool) -> (tempfile::TempDir, Vec<u8>) {
    let tmp = tempfile::tempdir().unwrap();
    let cat = tmp.path().join("fixture-catalog.json");
    std::fs::write(&cat, catalog_body).unwrap();
    let mut cmd = Command::cargo_bin("aware").unwrap();
    cmd.env("AWARE_HOME", tmp.path())
        .env("AWARE_CATALOG", format!("file://{}", cat.display()))
        .args(["agent", "describe", "probe-agent", "--available"]);
    if json {
        cmd.arg("--json");
    }
    let out = cmd.assert().success().get_output().stdout.clone();
    (tmp, out)
}

#[test]
fn describe_available_lists_every_version_the_registry_has() {
    // The discovery half of #363. `describe` called `agent.latest()` and printed
    // that one version, so when the newest release fell outside an app's
    // `requires:` pin there was no way to find out which older version satisfied
    // it — the information was in the catalogue and simply never surfaced. The
    // unsatisfied-pin remedy now points here, so it has to actually answer.
    let (_tmp, out) = describe_available(MULTI_VERSION_CATALOG, false);
    let text = String::from_utf8_lossy(&out);

    // #371 landed, so this asserts the truth rather than characterising the defect: the
    // fixture is chosen so lexicographic and semver DISAGREE (`"1.10.1" < "1.9.0"` as
    // strings), which is what makes the assertion mean something. It was the
    // characterisation test that announced the fix, exactly as intended.
    assert!(
        text.contains("version:      1.10.1"),
        "the newest by SEMVER is the one described: {text}"
    );
    for v in ["1.9.0", "1.10.0", "1.10.1"] {
        assert!(
            text.lines()
                .any(|l| l.starts_with("versions:") && l.contains(v)),
            "version {v} missing from the versions line: {text}"
        );
    }
    // And it names the verb that acts on them, so the answer is actionable
    // rather than merely informative.
    assert!(
        text.contains("aware agent update probe-agent@<version>"),
        "the versions line should say what to do with them: {text}"
    );
}

#[test]
fn describe_available_json_carries_the_versions_too() {
    // The UI reads JSON, not the table — floless.app's "Available agents" tab is
    // the consumer that made `vendor`/`keywords` a field, and version selection
    // needs the same treatment.
    let (_tmp, out) = describe_available(MULTI_VERSION_CATALOG, true);
    let v: serde_json::Value = serde_json::from_slice(&out).unwrap();
    let versions: Vec<&str> = v["data"]["versions"]
        .as_array()
        .expect("versions array")
        .iter()
        .map(|x| x.as_str().unwrap())
        .collect();
    // `1.9.0` before `1.10.0` is the whole point. The keys live in a
    // `BTreeMap<String, _>`, so the obvious `versions.keys()` sorts them
    // LEXICOGRAPHICALLY and puts `1.10.0` first — and this registry ships
    // calendar-shaped versions (`tekla@2025.0.1`), where that misordering is
    // routine rather than exotic. A fixture of 1.1/1.2/1.3 would have passed
    // either way and proved nothing about the order it asserts.
    assert_eq!(
        versions,
        ["1.9.0", "1.10.0", "1.10.1"],
        "oldest first by SEMVER"
    );
    // …and `version` now AGREES with the last entry of the list beside it. Those two
    // contradicted each other on screen before #371, which is what a consumer reading this
    // envelope would have had to reconcile.
    assert_eq!(
        v["data"]["version"], "1.10.1",
        "`version` must be the newest by semver, matching the list"
    );
}

#[test]
fn a_single_version_agent_prints_no_versions_line() {
    // Negative control, and a deliberate quiet: repeating one version on a second
    // line teaches the reader that the line means something it does not. The
    // line appears when there is a CHOICE to make.
    let (_tmp, out) = describe_available(
        r#"{"version":"1","updated-at":"2026-06-16T00:00:00Z","agents":{"probe-agent":{"versions":{
             "1.3.0": { "description": "only", "status": "available", "stateful": false,
                        "transport": "cli", "command_count": 1, "skills": [] }}}}}"#,
        false,
    );
    let text = String::from_utf8_lossy(&out);
    assert!(text.contains("version:      1.3.0"));
    assert!(
        !text.lines().any(|l| l.starts_with("versions:")),
        "one version is not a choice: {text}"
    );
}
