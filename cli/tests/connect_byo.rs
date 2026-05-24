//! Integration tests for Tier 2 bring-your-own OAuth (#146).

use assert_cmd::Command;
use predicates::prelude::*;

/// `trimble-connect --oauth` has no bundled app (placeholder client_id), so it
/// must fail fast with a BYO hint rather than sending the placeholder to Trimble
/// ("Unregistered client"). (#153)
#[test]
fn trimble_oauth_without_bundled_app_fails_with_byo_hint() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env("AWARE_DISABLE_KEYRING", "1")
        .env_remove("AWARE_OAUTH_TRIMBLE_CLIENT_ID")
        .args(["connect", "trimble-connect", "--oauth"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no bundled OAuth app"));
}

/// `google-workspace --device-code` must report device-code unsupported (point at
/// --oauth) rather than leaking a raw Google 401. (#151)
#[test]
fn google_device_code_reports_unsupported_not_raw_401() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env("AWARE_DISABLE_KEYRING", "1")
        // Exercise the bundled default client — clear any BYO env overrides so the
        // test doesn't turn Google into a (device-capable) BYO client and hit the net.
        .env_remove("AWARE_OAUTH_GOOGLE_CLIENT_ID")
        .env_remove("AWARE_OAUTH_GOOGLE_CLIENT_SECRET")
        .args(["--json", "connect", "google-workspace", "--device-code"])
        .assert()
        .success()
        .stdout(predicate::str::contains("does not support device-code"))
        .stdout(predicate::str::contains("--oauth"));
}

/// A malformed BYO OAuth profile must NOT block the non-OAuth token-import paths
/// (`--from-file`), since those don't use OAuth config at all. Regression for the
/// eager-profile-load issue found in review.
#[test]
fn malformed_profile_does_not_block_from_file_import() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();
    std::fs::create_dir_all(home.join("oauth")).unwrap();
    // Intentionally broken YAML.
    std::fs::write(
        home.join("oauth/trimble-connect.yaml"),
        "client_id: [broken\n",
    )
    .unwrap();

    let token = home.join("tok.txt");
    std::fs::write(&token, "tk_manual_123\n").unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .env("AWARE_DISABLE_KEYRING", "1")
        .args([
            "connect",
            "trimble-connect",
            "--from-file",
            token.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("stored"));
}

/// `--set-app-secret` reads the secret from stdin and reports success.
#[test]
fn set_app_secret_reads_stdin() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env("AWARE_DISABLE_KEYRING", "1")
        .args(["connect", "google-workspace", "--set-app-secret"])
        .write_stdin("my-byo-secret\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("BYO client secret"));
}

/// `--set-app-secret` with empty stdin is a clear validation error.
#[test]
fn set_app_secret_empty_stdin_errors() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env("AWARE_DISABLE_KEYRING", "1")
        .args(["connect", "google-workspace", "--set-app-secret"])
        .write_stdin("")
        .assert()
        .failure();
}

/// `--as <alias> --set-app-secret` with only a default profile must store the
/// secret in the DEFAULT slot (the slot the OAuth flow reads for that alias),
/// not an orphaned alias slot. Verified via the file fallback paths.
#[test]
fn set_app_secret_alias_without_profile_stores_default_slot() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();
    std::fs::create_dir_all(home.join("oauth")).unwrap();
    // Only a default profile exists (no google-workspace.personal.yaml).
    std::fs::write(home.join("oauth/google-workspace.yaml"), "client_id: org\n").unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .env("AWARE_DISABLE_KEYRING", "1")
        .args([
            "connect",
            "google-workspace",
            "--as",
            "personal",
            "--set-app-secret",
        ])
        .write_stdin("the-secret\n")
        .assert()
        .success();

    // File fallback lands in the default slot, not the alias slot.
    assert!(
        home.join("credentials/oauth-app.google-workspace.json")
            .is_file(),
        "secret should be stored in the default slot"
    );
    assert!(
        !home
            .join("credentials/oauth-app.google-workspace.personal.json")
            .is_file(),
        "secret must NOT be stored in an orphaned alias slot"
    );
}

/// A BYO profile with a client_id surfaces as `byo-profile` in `connect --list --json`,
/// while an untouched integration stays `first-party`.
#[test]
fn list_json_reports_byo_profile() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();
    std::fs::create_dir_all(home.join("oauth")).unwrap();
    std::fs::write(
        home.join("oauth/microsoft-365.yaml"),
        "client_id: 11111111-2222-3333-4444-555555555555\n",
    )
    .unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .env("AWARE_DISABLE_KEYRING", "1")
        .args(["--json", "connect", "--list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"app\": \"byo-profile\""))
        .stdout(predicate::str::contains("\"app\": \"first-party\""));
}
