//! `aware connect` token-import paths and credential-status reporting.
//!
//! The unit tests in `commands::connect` own the pure logic — `parse_scopes`,
//! both token loaders, the JSON status shape. What only the binary shows lives
//! here:
//!
//! * The exit code and stderr line a user actually gets. `run_connect`'s
//!   argument guards and `load_token_from_env` *return* errors; `main` is what
//!   turns them into an exit code and a message (see `error.rs`).
//! * `print_credential_status_text` prints with `println!` and returns nothing,
//!   so the process's stdout is the only surface it has.
//! * That the CLI wiring in between — flag routing, and the write to the
//!   credential store — actually happens end to end.
//!
//! Pure `load_token_from_env` behaviour is *not* re-tested here: the crate
//! ships `test_env::EnvVarGuard` for exactly that (it serializes the write
//! against the rest of the test binary), and the unit module uses it.
//!
//! The binary under test is not `cfg(test)`, so the unit-test keychain default
//! in `auth/keychain.rs` does not reach it. `AWARE_DISABLE_KEYRING` makes
//! `<AWARE_HOME>/credentials/` the sole store: the OS keychain is
//! process-global and not scoped by `AWARE_HOME`, and on a machine with no
//! keyring backend every status would otherwise come back
//! `keyring_unavailable`.

use assert_cmd::Command;
use predicates::prelude::*;

/// Where a credential lands with the keyring disabled — the sole store here,
/// not a fallback. Mirrors `cred_file_path` in `auth/keychain.rs`.
fn cred_file(home: &std::path::Path, integration: &str) -> std::path::PathBuf {
    home.join("credentials").join(format!("{integration}.json"))
}

fn unix_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

/// A stored credential expiring `offset` seconds from now.
///
/// This hand-transcribes `StoredToken`'s wire shape (`auth/keychain.rs`)
/// because an integration test cannot import it — the crate has no `lib.rs`.
/// That transcription can drift, and drift is *silent* in a nasty way: on a
/// strict-parse failure `read_cred_file` falls through to a legacy branch that
/// hardcodes `source: Paste`, so a seeded "expired oauth" credential would come
/// back as a never-expiring paste token. [`assert_seed_round_trips`] is the
/// tripwire; call it wherever the distinction matters.
fn seed_credential(home: &std::path::Path, integration: &str, source: &str, offset: i64) {
    let now = unix_now();
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
            "source": source,
        })
        .to_string(),
    )
    .unwrap();
}

fn seed_oauth_credential(home: &std::path::Path, integration: &str, offset: i64) {
    seed_credential(home, integration, "oauth", offset);
}

/// Fail loudly if a seeded credential no longer deserializes as the `source` it
/// was written with — i.e. if `StoredToken` has drifted away from the JSON
/// above and the legacy branch has quietly taken over.
fn assert_seed_round_trips(home: &std::path::Path, integration: &str, expected_source: &str) {
    let out = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .env("AWARE_DISABLE_KEYRING", "1")
        .args(["--json", "connect", "--list"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let v: serde_json::Value = serde_json::from_slice(&out).unwrap();
    let entry = v
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e["integration"] == integration)
        .unwrap_or_else(|| panic!("no status entry for {integration}"));
    assert_eq!(
        entry["source"], expected_source,
        "seeded credential for {integration} did not read back as {expected_source} — \
         the JSON in seed_credential has drifted from StoredToken in auth/keychain.rs"
    );
}

/// The end-to-end import: the derived variable is read, the token is stripped,
/// and it lands in the credential store. The derivation itself is pinned by the
/// unit test; what this adds is that the CLI path around it — flag routing and
/// the store write — actually runs.
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
    // Stored as `paste` — a reporting label, not a refresh policy: it puts the
    // credential on the status branch that reports `valid` with a null
    // countdown, so the `expires_at: 0` an env import writes is never read as
    // "expired".
    assert_eq!(stored["source"], "paste");
    assert_eq!(stored["expires_at"], 0);
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
///
/// It covers four flags, and there is no clap `conflicts_with` backstop, so
/// each pairing is exercised: testing only `--from-file --from-env` leaves the
/// `--oauth` and `--device-code` arms free to be deleted unnoticed.
#[test]
fn two_input_sources_are_rejected_rather_than_silently_ordered() {
    let tmp = tempfile::tempdir().unwrap();
    let token = tmp.path().join("tok.txt");
    std::fs::write(&token, "tk_from_file\n").unwrap();
    let token = token.to_str().unwrap().to_string();

    let pairs: [&[&str]; 5] = [
        &["--from-file", &token, "--from-env"],
        &["--from-file", &token, "--oauth"],
        &["--from-file", &token, "--device-code"],
        &["--from-env", "--oauth"],
        &["--oauth", "--device-code"],
    ];

    for extra in pairs {
        // A credential already in place: the guard must reject before any flow
        // runs, so the existing bytes have to survive untouched. `!exists()`
        // alone would also be satisfied by a crash during argument parsing.
        seed_oauth_credential(tmp.path(), "trimble-connect", 7_200);
        let before = std::fs::read(cred_file(tmp.path(), "trimble-connect")).unwrap();

        let mut args = vec!["connect", "trimble-connect"];
        args.extend_from_slice(extra);
        Command::cargo_bin("aware")
            .unwrap()
            .env("AWARE_HOME", tmp.path())
            .env("AWARE_DISABLE_KEYRING", "1")
            .env("AWARE_TOKEN_TRIMBLE_CONNECT", "tk_from_env")
            .args(&args)
            .assert()
            .failure()
            .stderr(predicate::str::contains("at most one of"));

        let after = std::fs::read(cred_file(tmp.path(), "trimble-connect")).unwrap();
        assert_eq!(
            before, after,
            "{extra:?} was rejected but the stored credential changed"
        );
    }
}

/// `--tenant` is wired only into the M365 endpoint rewrite (auth/config.rs).
/// Anywhere else it can only be silently ignored, leaving the user believing
/// they scoped the connection when nothing read the flag — hence a hard error
/// rather than a warning.
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
        // Anchor on the flag under test. "microsoft-365" alone would also be
        // satisfied by any future error that merely enumerates the supported
        // integrations; "--tenant" appears in this guard and nowhere else on
        // this path.
        .stderr(predicate::str::contains("--tenant"));

    assert!(
        !cred_file(tmp.path(), "google-workspace").exists(),
        "the import must not proceed once the flag is refused"
    );
}

/// The text rendering of `connect --list` — the default, non-`--json` output —
/// has to separate the states an operator acts on differently: a live OAuth
/// credential (do nothing), an expired one (refresh it), and an absent one
/// (connect it). The two actionable states print the exact command to run; the
/// live one deliberately prints none.
#[test]
fn list_text_separates_live_expired_and_absent_credentials() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();
    seed_oauth_credential(home, "google-workspace", 7_200);
    seed_oauth_credential(home, "trimble-connect", -7_200);
    // microsoft-365 is deliberately left unseeded.
    assert_seed_round_trips(home, "google-workspace", "oauth");

    let text = list_text(home);
    let live = line_for(&text, "google-workspace");
    assert!(live.contains("valid"), "{live}");
    // Reported in whole minutes. Seeded two hours out, so the countdown must
    // land just under 120 — this kills "printed the raw seconds" (7200) and
    // "printed the unix deadline" (~1.8e9). It does not pin floor-vs-ceil
    // rounding, which is unpinnable at a whole-minute offset; the exact
    // arithmetic is owned by the `credential_status_json_*` unit tests, which
    // inject `now` instead of reading the clock.
    let mins = minutes_in(&live);
    assert!(
        (118..=120).contains(&mins),
        "expected a ~120m countdown, got {mins}m in: {live}"
    );

    let stale = line_for(&text, "trimble-connect");
    assert!(stale.contains("expired"), "{stale}");
    assert!(stale.contains("--refresh"), "{stale}");

    // The absent one suggests the flow that actually works for that provider:
    // M365's bundled public client has no loopback redirect, so its hint must
    // read `--device-code` where the other two integrations get `--oauth`
    // (auth/config.rs, #158).
    let absent = line_for(&text, "microsoft-365");
    assert!(absent.contains("missing"), "{absent}");
    assert!(absent.contains("--device-code"), "{absent}");
}

/// The other two branches of the same renderer, which the states above never
/// reach: a paste token (no expiry to report at all) and a credential the store
/// cannot read. Without these, both branches can be replaced with anything and
/// no test in the crate notices.
#[test]
fn list_text_renders_paste_and_unreadable_credentials() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();
    // A paste token: user-managed, so it is reported valid with no countdown
    // and no refresh hint — this CLI has no way to renew it.
    seed_credential(home, "google-workspace", "paste", 0);
    assert_seed_round_trips(home, "google-workspace", "paste");
    // An unreadable credential: present but not parseable.
    std::fs::write(cred_file(home, "trimble-connect"), "not json at all").unwrap();

    let text = list_text(home);

    let paste = line_for(&text, "google-workspace");
    assert!(paste.contains("valid"), "{paste}");
    assert!(paste.contains("paste token (user-managed)"), "{paste}");
    assert!(
        !paste.contains("expires in"),
        "a paste token has no expiry to count down: {paste}"
    );

    // Reported as "keyring unavailable" even here, where the keyring is
    // switched off and the real cause is the corrupt file — the same misnomer
    // the unit test records. What matters is that it is not silently rendered
    // as `missing`, which would tell the operator to connect an integration
    // that already has a credential.
    let unreadable = line_for(&text, "trimble-connect");
    assert!(unreadable.contains("keyring unavailable"), "{unreadable}");
    assert!(!unreadable.contains("missing"), "{unreadable}");
}

fn list_text(home: &std::path::Path) -> String {
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
    String::from_utf8(out).unwrap()
}

fn line_for(text: &str, integration: &str) -> String {
    text.lines()
        .find(|l| l.contains(integration))
        .unwrap_or_else(|| panic!("no line for {integration} in:\n{text}"))
        .to_string()
}

/// The `<n>` out of an `expires in <n>m` line.
fn minutes_in(line: &str) -> i64 {
    let after = line
        .split_once("expires in ")
        .unwrap_or_else(|| panic!("no countdown in: {line}"))
        .1;
    let digits: String = after.chars().take_while(char::is_ascii_digit).collect();
    digits
        .parse()
        .unwrap_or_else(|e| panic!("countdown {digits:?} is not a number ({e}) in: {line}"))
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
