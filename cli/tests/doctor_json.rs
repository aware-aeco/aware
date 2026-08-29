//! `aware doctor --json` — the machine-readable health report.
//!
//! `commands::doctor::run_json` is a second, independent renderer: it re-walks
//! `apps/*/instances/*` for pidfiles, re-runs `validate_agent_on_disk`, and re-asks
//! `credential_status_json` and `find_bridge_by_id`, none of it shared with the text
//! path. Nothing in the suite ran the command with `--json` at all, so every field
//! below — and every branch that fills one — could be renamed, inverted or dropped
//! with the suite green. These tests parse the whole document, so a stray `println!`
//! on the JSON path fails them too.

use assert_cmd::Command;

/// Run `aware doctor --json` against `home` and parse stdout as one JSON document.
///
/// `AWARE_DISABLE_KEYRING` forces the credentials-file store: without it the answer
/// depends on whether the machine running the suite has a reachable secret service
/// (this container reports `keyring_unavailable`, a developer's desktop does not),
/// and a status that varies by host pins nothing. `PATH` is emptied for the same
/// reason on the bridge side — `find_bridge_by_id` falls back to a `which`-style
/// lookup, so a real `aware-tekla` on the tester's PATH would otherwise decide the
/// answer. Both mirror what `tests/sidecar_cli.rs` already does.
fn doctor_json(home: &std::path::Path) -> serde_json::Value {
    let out = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .env("AWARE_DISABLE_KEYRING", "1")
        .env("PATH", "")
        .args(["doctor", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(out).expect("stdout is UTF-8");
    serde_json::from_str(&text)
        .unwrap_or_else(|e| panic!("stdout is not one JSON document: {e}\n{text}"))
}

/// Write a credentials-file token the way `auth::keychain`'s file fallback reads it.
fn write_credential(home: &std::path::Path, account: &str, expires_at: i64, source: &str) {
    let dir = home.join("credentials");
    std::fs::create_dir_all(&dir).unwrap();
    let body = serde_json::json!({
        "access_token": "tk",
        "refresh_token": "rt",
        "expires_at": expires_at,
        "scope": "s",
        "token_type": "Bearer",
        "integration": account,
        "obtained_at": 0,
        "source": source,
    });
    std::fs::write(dir.join(format!("{account}.json")), body.to_string()).unwrap();
}

/// An agent whose manifest lists a skill file that is not on disk — one real
/// `E_SKILL_FILE_MISSING` for the integrity block to carry.
fn write_agent_with_a_missing_skill(home: &std::path::Path) {
    let agent_dir = home.join("agents/broken");
    std::fs::create_dir_all(agent_dir.join("skills")).unwrap();
    std::fs::write(
        agent_dir.join("manifest.yaml"),
        "agent: broken\nversion: 1.2.3\ndescription: an intentionally-broken agent\n\
         stateful: false\nlicense: MIT\ntransport: { cli: { binary: x } }\n\
         commands: { do: { lifecycle: single, description: z } }\nskills:\n  - real.md\n  - missing.md\n",
    )
    .unwrap();
    std::fs::write(agent_dir.join("skills/real.md"), "# real skill\n").unwrap();
}

#[test]
fn home_and_config_existence_are_reported_as_measured_not_assumed() {
    // Both flags answer "is this actually on disk", and both are the reason to run
    // `doctor` on a machine that is misbehaving — a hardcoded `true` would be
    // indistinguishable from a healthy answer in exactly the case the operator is
    // debugging. The paths are reported whether or not they exist.
    let tmp = tempfile::tempdir().unwrap();
    let absent = tmp.path().join("no-home-here");

    let report = doctor_json(&absent);
    assert_eq!(report["aware_home_exists"], false);
    assert_eq!(report["config_exists"], false);
    assert_eq!(report["aware_home"], absent.display().to_string());
    assert_eq!(
        report["config_path"],
        absent.join("config.yaml").display().to_string()
    );

    // The same home, now real, and with a config in it.
    std::fs::create_dir_all(&absent).unwrap();
    std::fs::write(absent.join("config.yaml"), "log-level: info\n").unwrap();
    let report = doctor_json(&absent);
    assert_eq!(report["aware_home_exists"], true);
    assert_eq!(report["config_exists"], true);
}

#[test]
fn installed_agents_are_listed_with_the_version_each_manifest_declares() {
    // Not just "an agent is present": the version is what an operator compares
    // against a release, and reporting the id twice (or a constant) would read as a
    // plausible report while answering the wrong question.
    let tmp = tempfile::tempdir().unwrap();
    write_agent_with_a_missing_skill(tmp.path());

    let report = doctor_json(tmp.path());
    assert_eq!(
        report["agents"],
        serde_json::json!([{ "id": "broken", "version": "1.2.3" }])
    );
    // An empty apps directory reports an empty list, not a missing key — a consumer
    // indexing `apps` must not have to distinguish the two.
    assert_eq!(report["apps"], serde_json::json!([]));
}

#[test]
fn integrity_carries_every_issue_and_all_pass_follows_the_issue_list() {
    // `all_pass` is the field a CI consumer gates on, and the issue list is what
    // tells a human which agent to fix. A report that finds the fault and still says
    // `all_pass: true` is worse than one that finds nothing.
    let tmp = tempfile::tempdir().unwrap();
    write_agent_with_a_missing_skill(tmp.path());

    let report = doctor_json(tmp.path());
    assert_eq!(report["integrity"]["all_pass"], false);
    let issues = report["integrity"]["issues"].as_array().unwrap();
    let missing_skill = issues
        .iter()
        .find(|i| i["code"] == "E_SKILL_FILE_MISSING")
        .unwrap_or_else(|| panic!("no E_SKILL_FILE_MISSING in {issues:#?}"));
    assert_eq!(missing_skill["agent"], "broken");
    assert_eq!(missing_skill["severity"], "error");
    assert!(
        missing_skill["message"]
            .as_str()
            .unwrap()
            .contains("missing.md"),
        "the message must name the file the operator has to restore: {missing_skill:#?}"
    );

    // With that agent gone, the same home passes — so `all_pass` tracks the list
    // rather than being a constant that happened to match above.
    std::fs::remove_dir_all(tmp.path().join("agents/broken")).unwrap();
    let report = doctor_json(tmp.path());
    assert_eq!(report["integrity"]["all_pass"], true);
    assert_eq!(report["integrity"]["issues"], serde_json::json!([]));
}

#[test]
fn a_running_instance_is_reported_with_every_field_needed_to_find_it() {
    // The pidfile walk is the one part of `doctor` an operator uses to answer "what
    // is running, and where do I look for its trace" — `pid` to signal it, `run_id`
    // to find its log. Dropping either leaves a report that still lists the instance
    // and cannot be acted on.
    let tmp = tempfile::tempdir().unwrap();
    let instance = tmp.path().join("apps/welded-to-tc/instances/default");
    std::fs::create_dir_all(&instance).unwrap();
    std::fs::write(
        instance.join("pidfile.yaml"),
        "app: welded-to-tc\ninstance: default\npid: 99999\n\
         started-at: 2026-05-16T14:23:00Z\nrun-id: r_abc123\n",
    )
    .unwrap();
    // A sibling directory with no pidfile must not become a phantom entry.
    std::fs::create_dir_all(tmp.path().join("apps/welded-to-tc/instances/never-started")).unwrap();
    // …nor may a stray file where an instance directory belongs.
    std::fs::write(
        tmp.path().join("apps/welded-to-tc/instances/notes.txt"),
        "x",
    )
    .unwrap();

    let report = doctor_json(tmp.path());
    assert_eq!(
        report["running"],
        serde_json::json!([{
            "app": "welded-to-tc",
            "instance": "default",
            "pid": 99999,
            "run_id": "r_abc123",
            "started_at": "2026-05-16T14:23:00Z",
        }])
    );
}

#[test]
fn credentials_distinguish_valid_expired_and_missing_for_every_known_integration() {
    // The three statuses are three different actions for the operator: nothing,
    // `--refresh`, and `aware connect`. Collapsing them — reporting an expired token
    // as valid, say — is the failure this block exists to prevent, and it is
    // invisible to any assertion that only checks the integrations are listed.
    let tmp = tempfile::tempdir().unwrap();
    write_credential(tmp.path(), "trimble-connect", 4_102_444_800, "oauth"); // year 2100
    write_credential(tmp.path(), "microsoft-365", 100, "oauth"); // 1970, long expired
    // google-workspace: no file at all.

    let report = doctor_json(tmp.path());
    let by_id = |name: &str| -> serde_json::Value {
        report["credentials"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["integration"] == name)
            .unwrap_or_else(|| panic!("{name} missing from {:#?}", report["credentials"]))
            .clone()
    };

    let tc = by_id("trimble-connect");
    assert_eq!(tc["status"], "valid");
    assert_eq!(tc["source"], "oauth");
    assert!(
        tc["expires_in_secs"].as_i64().unwrap() > 0,
        "a valid token reports the time it has left: {tc:#?}"
    );

    let ms = by_id("microsoft-365");
    assert_eq!(ms["status"], "expired");
    assert_eq!(ms["expires_in_secs"], 0);
    // The flow a UI must offer differs per integration, and Microsoft's is the one
    // that is not `oauth` — so this pins the mapping rather than a shared default.
    assert_eq!(ms["recommended_flow"], "device-code");

    let gw = by_id("google-workspace");
    assert_eq!(gw["status"], "missing");
    assert_eq!(gw["source"], serde_json::Value::Null);
    assert_eq!(gw["expires_in_secs"], serde_json::Value::Null);
}

#[test]
fn a_host_bridge_is_reported_installed_only_when_its_binary_is_on_disk() {
    // Both layouts `find_bridge_in_dir` accepts, because `installed: true` with a
    // path the operator cannot find is the same as no report at all: `rhino` ships
    // as a bare exe, `tekla` extracts into a sub-directory of its own name.
    let tmp = tempfile::tempdir().unwrap();
    let bridges = tmp.path().join("bridges");
    std::fs::create_dir_all(bridges.join("aware-tekla")).unwrap();
    std::fs::write(bridges.join("aware-rhino.exe"), "").unwrap();
    std::fs::write(bridges.join("aware-tekla/aware-tekla.exe"), "").unwrap();

    let report = doctor_json(tmp.path());
    let by_id = |name: &str| -> serde_json::Value {
        report["host_bridges"]
            .as_array()
            .unwrap()
            .iter()
            .find(|b| b["id"] == name)
            .unwrap_or_else(|| panic!("{name} missing from {:#?}", report["host_bridges"]))
            .clone()
    };

    let rhino = by_id("rhino");
    assert_eq!(rhino["installed"], true);
    assert_eq!(
        rhino["path"],
        bridges.join("aware-rhino.exe").display().to_string()
    );

    let tekla = by_id("tekla");
    assert_eq!(tekla["installed"], true);
    assert_eq!(
        tekla["path"],
        bridges
            .join("aware-tekla")
            .join("aware-tekla.exe")
            .display()
            .to_string()
    );

    // The two nothing was planted for stay absent, and report a null path rather
    // than an empty string a consumer would have to special-case.
    for id in ["sketchup", "revit"] {
        let b = by_id(id);
        assert_eq!(b["installed"], false, "{id}");
        assert_eq!(b["path"], serde_json::Value::Null, "{id}");
    }
}

#[test]
fn the_report_names_the_cli_version_that_produced_it() {
    // A health report pasted into an issue is only actionable if it says which build
    // it came from, and this is the field a maintainer reads first.
    let tmp = tempfile::tempdir().unwrap();
    let report = doctor_json(tmp.path());
    assert_eq!(report["version"], env!("CARGO_PKG_VERSION"));
}
