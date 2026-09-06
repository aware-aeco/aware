mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn doctor_reports_installed_agents_and_apps() {
    // The fixture mirrors every `manifest.yaml` under `20-agents/` and every `.flo`/`.app`
    // under `30-apps/_examples/`. We assert doctor reports installed agents/apps rather than a
    // hardcoded count — the count drifts whenever an agent lands, and pinning it is brittle.
    let home = common::aware_home();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("CLI:"))
        .stdout(predicate::str::contains(format!(
            "aware v{}",
            env!("CARGO_PKG_VERSION")
        )))
        .stdout(predicate::str::contains("Filesystem:"))
        .stdout(predicate::str::contains("installed"));
}

#[test]
fn doctor_empty_home_succeeds() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("0 installed"));
}

/// Run `aware doctor` (text mode) against `home` and return stdout.
///
/// `AWARE_DISABLE_KEYRING` and an emptied `PATH` for the same reasons as
/// `tests/doctor_json.rs`: without them the credential status depends on whether the
/// machine has a reachable secret service, and the bridge status on whether it has a
/// real `aware-*` binary on PATH.
///
/// PATH is pointed at an empty directory rather than set to `""`: `split_paths("")`
/// yields one *empty* entry, so the lookup probes the bare name relative to the
/// child's working directory instead of probing nothing at all.
fn doctor_text(home: &std::path::Path) -> String {
    let nowhere = tempfile::tempdir().unwrap();
    let out = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .env("AWARE_DISABLE_KEYRING", "1")
        .env("PATH", nowhere.path())
        .arg("doctor")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    String::from_utf8(out).expect("stdout is UTF-8")
}

#[test]
fn a_missing_host_bridge_is_named_together_with_the_command_that_installs_it() {
    // The Host Bridges block had no test in either output mode. Its whole value is
    // the remedy: an operator whose Tekla agent will not spawn reads this line to
    // learn the bridge is absent AND what to type. A block that lists four ticks
    // regardless — or names the gap without the fix — is a report they cannot act
    // on, and both readings passed the suite before this.
    let tmp = tempfile::tempdir().unwrap();
    let out = doctor_text(tmp.path());

    assert!(out.contains("Host Bridges:"), "{out}");
    for id in ["tekla", "rhino", "sketchup", "revit"] {
        assert!(
            out.contains(&format!(
                "\u{2717} aware-{id:<10}  not found — run: aware sidecar install {id}"
            )),
            "no actionable missing-bridge line for {id}:\n{out}"
        );
    }
    // The all-clear must NOT be claimed while four bridges are missing.
    assert!(
        !out.contains("all host bridges present"),
        "the summary contradicts the four lines above it:\n{out}"
    );
}

#[test]
fn an_installed_bridge_is_reported_at_the_path_it_was_found() {
    // A tick with no path tells an operator the bridge is fine and not which copy
    // answered — the question that matters when a stale one is shadowing a new one.
    let tmp = tempfile::tempdir().unwrap();
    let bridges = tmp.path().join("bridges");
    std::fs::create_dir_all(&bridges).unwrap();
    std::fs::write(bridges.join("aware-rhino.exe"), "").unwrap();

    let out = doctor_text(tmp.path());
    let path = bridges.join("aware-rhino.exe");
    assert!(
        out.contains(&format!(
            "\u{2713} aware-{:<10}  {}",
            "rhino",
            path.display()
        )),
        "expected a ticked rhino line naming {}:\n{out}",
        path.display()
    );
    // Still not the all-clear: three bridges are absent.
    assert!(!out.contains("all host bridges present"), "{out}");
}

#[test]
fn the_all_clear_is_printed_only_when_every_bridge_is_present() {
    // The other side of `any_missing`. Without this, a summary hardcoded to the
    // all-clear-free branch — or one that never prints the all-clear at all — is
    // indistinguishable from correct.
    let tmp = tempfile::tempdir().unwrap();
    let bridges = tmp.path().join("bridges");
    std::fs::create_dir_all(&bridges).unwrap();
    for binary in [
        "aware-tekla",
        "aware-rhino",
        "aware-sketchup",
        "aware-revit",
    ] {
        std::fs::write(bridges.join(format!("{binary}.exe")), "").unwrap();
    }

    let out = doctor_text(tmp.path());
    assert!(out.contains("all host bridges present"), "{out}");
    assert!(
        !out.contains("not found — run: aware sidecar install"),
        "{out}"
    );
}

#[test]
fn each_credential_line_states_the_status_and_the_action_it_implies() {
    // Replaces `doctor_credentials_block_appears`, which asserted only the literal
    // `"Credentials:"` header — a `println!` in `doctor` itself, printed before the
    // helper is called, so gutting the entire credentials block left it green.
    //
    // The four arms are four different instructions to the operator, and the pair
    // that must never be confused is valid/expired: an expired token reported as
    // valid sends them looking for the fault somewhere else entirely.
    let tmp = tempfile::tempdir().unwrap();
    let creds = tmp.path().join("credentials");
    std::fs::create_dir_all(&creds).unwrap();
    let token = |integration: &str, expires_at: i64, source: &str| {
        serde_json::json!({
            "access_token": "tk",
            "refresh_token": "rt",
            "expires_at": expires_at,
            "scope": "s",
            "token_type": "Bearer",
            "integration": integration,
            "obtained_at": 0,
            "source": source,
        })
        .to_string()
    };
    // Year 2100 — valid for as long as this test will plausibly run.
    std::fs::write(
        creds.join("trimble-connect.json"),
        token("trimble-connect", 4_102_444_800, "oauth"),
    )
    .unwrap();
    // 1970 — expired.
    std::fs::write(
        creds.join("microsoft-365.json"),
        token("microsoft-365", 100, "oauth"),
    )
    .unwrap();
    // A pasted handle has no expiry AWARE can act on, so it is never offered
    // `--refresh`, whatever `expires_at` says.
    std::fs::write(
        creds.join("google-workspace.json"),
        token("google-workspace", 100, "paste"),
    )
    .unwrap();

    let out = doctor_text(tmp.path());
    assert!(
        out.contains("\u{2713} trimble-connect        valid    OAuth, expires in"),
        "a live OAuth token must read as valid with its remaining life:\n{out}"
    );
    assert!(
        out.contains("microsoft-365          expired  run: aware connect microsoft-365 --refresh"),
        "an expired token must say so and name the refresh command:\n{out}"
    );
    assert!(
        out.contains("\u{2713} google-workspace       valid    paste token (user-managed)"),
        "a pasted token is valid and user-managed, never offered --refresh:\n{out}"
    );
    assert!(
        !out.contains("aware connect google-workspace --refresh"),
        "nothing can refresh a pasted handle, so nothing may suggest it:\n{out}"
    );

    // …and with the files removed, the same integrations report missing and name the
    // command that provisions them — including the flow Microsoft needs, which is
    // not the default.
    std::fs::remove_dir_all(&creds).unwrap();
    let out = doctor_text(tmp.path());
    assert!(
        out.contains(
            "\u{2717} trimble-connect        missing  run: aware connect trimble-connect --oauth"
        ),
        "{out}"
    );
    assert!(
        out.contains(
            "\u{2717} microsoft-365          missing  run: aware connect microsoft-365 --device-code"
        ),
        "{out}"
    );
}
