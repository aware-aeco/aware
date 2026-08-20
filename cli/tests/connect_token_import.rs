//! `aware connect` token-import paths and credential-status reporting.
//!
//! The unit tests in `commands::connect` cover `load_token_from_file` and the
//! JSON status shape. Three things they cannot reach live here instead:
//!
//! * `--from-env` reads a *process* environment variable, and setting one from a
//!   unit test is `unsafe` under edition 2024 (and racy across the test
//!   binary's threads). A subprocess gets its own environment, so the whole
//!   path — including the variable-name derivation — is exercised honestly.
//! * `print_credential_status_text` writes to stdout and returns nothing, so
//!   the only way to assert on it is to read the process's output.
//! * The argument guards in `run_connect` reject combinations before any flow
//!   starts; they are only reachable through the parsed CLI.
//!
//! `AWARE_DISABLE_KEYRING` pins every case to the credentials-file fallback
//! under `AWARE_HOME` — the OS keychain is global and would leak between tests.

use assert_cmd::Command;
use predicates::prelude::*;

/// Path of the credentials-file fallback for an integration under `home`.
fn cred_file(home: &std::path::Path, integration: &str) -> std::path::PathBuf {
    home.join("credentials").join(format!("{integration}.json"))
}

/// A stored OAuth credential expiring `offset` seconds from now, written
/// straight into the fallback store the CLI reads with the keyring disabled.
fn seed_oauth_credential(home: &std::path::Path, integration: &str, offset: i64) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let creds = home.join("credentials");
    std::fs::create_dir_all(&creds).unwrap();
    std::fs::write(
        creds.join(format!("{integration}.json")),
        serde_json::json!({
            "access_token": "tk",
            "refresh_token": "rt",
            "expires_at": now + offset,
            "scope": "openid",
            "token_type": "Bearer",
            "integration": integration,
            "obtained_at": now,
            "source": "oauth",
        })
        .to_string(),
    )
    .unwrap();
}

/// `--from-env` derives the variable name from the integration id by
/// upper-casing it and turning kebab hyphens into underscores. `microsoft-365`
/// is the case that proves both halves at once: a hyphen is not a legal
/// character in a POSIX environment-variable name, so reading it back is the
/// only way to know the transformation happened.
#[test]
fn from_env_reads_the_variable_named_for_the_integration() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .env("AWARE_DISABLE_KEYRING", "1")
        .env("AWARE_TOKEN_MICROSOFT_365", "  tk_from_env  ")
        .args(["connect", "microsoft-365", "--from-env"])
        .assert()
        .success()
        .stdout(predicate::str::contains("stored"));

    let stored: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(cred_file(home, "microsoft-365")).unwrap())
            .unwrap();
    // Surrounding whitespace is stripped: a token sent with a trailing space is
    // a different bearer value to the provider and 401s.
    assert_eq!(stored["access_token"], "tk_from_env");
    // An imported token is user-managed, not one this CLI can refresh.
    assert_eq!(stored["source"], "paste");
}

/// With the variable unset the command fails and names the variable it looked
/// for, plus the other way in — otherwise the user has to guess the exact
/// spelling of a name they never typed.
#[test]
fn from_env_without_the_variable_names_it_in_the_error() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env("AWARE_DISABLE_KEYRING", "1")
        .env_remove("AWARE_TOKEN_TRIMBLE_CONNECT")
        .args(["connect", "trimble-connect", "--from-env"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("AWARE_TOKEN_TRIMBLE_CONNECT"))
        .stderr(predicate::str::contains("--from-file"));

    assert!(
        !cred_file(tmp.path(), "trimble-connect").exists(),
        "a failed import must not leave a credential behind"
    );
}

/// A variable that is set but blank is rejected rather than stored. Storing it
/// would make `connect --list` report the integration as connected while every
/// call it authorizes fails with an empty bearer.
#[test]
fn from_env_rejects_a_blank_variable_instead_of_storing_it() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env("AWARE_DISABLE_KEYRING", "1")
        .env("AWARE_TOKEN_TRIMBLE_CONNECT", "   \n")
        .args(["connect", "trimble-connect", "--from-env"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("empty"));

    assert!(
        !cred_file(tmp.path(), "trimble-connect").exists(),
        "a blank token must not be stored"
    );
}

/// The input-source flags are mutually exclusive. Without the guard the first
/// branch of the `if` chain silently wins and the user gets a credential from a
/// source they did not ask for — so the guard must reject, not pick.
#[test]
fn two_input_sources_are_rejected_rather_than_silently_ordered() {
    let tmp = tempfile::tempdir().unwrap();
    let token = tmp.path().join("tok.txt");
    std::fs::write(&token, "tk_from_file\n").unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env("AWARE_DISABLE_KEYRING", "1")
        .env("AWARE_TOKEN_TRIMBLE_CONNECT", "tk_from_env")
        .args([
            "connect",
            "trimble-connect",
            "--from-file",
            token.to_str().unwrap(),
            "--from-env",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("at most one of"));

    assert!(
        !cred_file(tmp.path(), "trimble-connect").exists(),
        "the rejected invocation must not have stored either token"
    );
}

/// `--tenant` only means something for Microsoft. Accepting it elsewhere would
/// take the flag, ignore it, and connect to the wrong directory without saying
/// so — hence a hard error rather than a warning.
#[test]
fn tenant_is_refused_for_integrations_that_have_no_tenants() {
    let tmp = tempfile::tempdir().unwrap();
    let token = tmp.path().join("tok.txt");
    std::fs::write(&token, "tk_from_file\n").unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env("AWARE_DISABLE_KEYRING", "1")
        .args([
            "connect",
            "google-workspace",
            "--tenant",
            "contoso.onmicrosoft.com",
            "--from-file",
            token.to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("microsoft-365"));

    assert!(
        !cred_file(tmp.path(), "google-workspace").exists(),
        "the import must not proceed once the flag is refused"
    );
}

/// The text rendering of `connect --list` — the default, non-`--json` output —
/// has to separate the three states an operator acts on differently: a live
/// credential (do nothing), an expired one (refresh it), and an absent one
/// (connect it). Each line carries the command for its state.
#[test]
fn list_text_separates_live_expired_and_absent_credentials() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();
    seed_oauth_credential(home, "google-workspace", 7_200);
    seed_oauth_credential(home, "trimble-connect", -7_200);
    // microsoft-365 is deliberately left unseeded.

    let out = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .env("AWARE_DISABLE_KEYRING", "1")
        .args(["connect", "--list"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(out).unwrap();

    let line = |id: &str| {
        text.lines()
            .find(|l| l.contains(id))
            .unwrap_or_else(|| panic!("no line for {id} in:\n{text}"))
            .to_string()
    };

    let live = line("google-workspace");
    assert!(live.contains("valid"), "{live}");
    // Two hours out, reported in whole minutes — not seconds, and not the raw
    // unix deadline.
    assert!(
        live.contains("expires in 119m") || live.contains("expires in 120m"),
        "{live}"
    );

    let stale = line("trimble-connect");
    assert!(stale.contains("expired"), "{stale}");
    assert!(stale.contains("--refresh"), "{stale}");
    assert!(!stale.contains("valid"), "{stale}");

    // The absent one suggests the flow that actually works for that provider:
    // M365's bundled public client has no loopback redirect, so it is
    // device-code, not the `--oauth` default (#158).
    let absent = line("microsoft-365");
    assert!(absent.contains("missing"), "{absent}");
    assert!(absent.contains("--device-code"), "{absent}");
}

/// `aware disconnect` has to actually remove the credential, not just print
/// that it did: a stale credential left on disk keeps `connect --list` claiming
/// the integration is wired up.
#[test]
fn disconnect_removes_the_credential_it_reports_removing() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();
    seed_oauth_credential(home, "google-workspace", 7_200);
    assert!(cred_file(home, "google-workspace").exists());

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .env("AWARE_DISABLE_KEYRING", "1")
        .args(["disconnect", "google-workspace"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed"));

    assert!(
        !cred_file(home, "google-workspace").exists(),
        "disconnect reported success but the credential file is still there"
    );

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .env("AWARE_DISABLE_KEYRING", "1")
        .args(["--json", "connect", "--list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"status\": \"missing\""));
}

/// `disconnect --as <alias>` has to reach the alias's own slot. Dropping the
/// alias on the way through would sign the user out of the *default* account
/// while leaving the one they named connected — the exact opposite of what they
/// asked for, and silent, because the command reports success either way.
#[test]
fn disconnect_with_an_alias_clears_that_alias_and_not_the_default() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();
    seed_oauth_credential(home, "google-workspace", 7_200);
    seed_oauth_credential(home, "google-workspace.personal", 7_200);

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .env("AWARE_DISABLE_KEYRING", "1")
        .args(["disconnect", "google-workspace", "--as", "personal"])
        .assert()
        .success();

    assert!(
        !cred_file(home, "google-workspace.personal").exists(),
        "the aliased credential named on the command line is still there"
    );
    assert!(
        cred_file(home, "google-workspace").exists(),
        "disconnecting an alias must not clear the default account"
    );
}
