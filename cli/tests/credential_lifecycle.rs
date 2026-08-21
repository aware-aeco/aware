//! `aware credential put / delete / status` — the opaque-credential lifecycle (#436).
//!
//! The unit tests in `commands::credential` own the pure logic: handle
//! validation, the env-var derivation, and every branch of reading the secret.
//! What only the binary can show lives here.
//!
//! * **The lifecycle actually composes.** Each verb is unit-tested in isolation;
//!   that a `put` is what a later `status` sees, and that a `delete` is what
//!   makes it stop seeing it, is a property of the three together against one
//!   real store.
//! * **The stored shape is one the runtime reads.** The whole point of the
//!   command is that the credential it writes resolves at run time. That claim
//!   is about the bytes on disk, which a unit test over `read_secret` never
//!   touches.
//! * **Exit codes and messages.** `run_put` and friends *return* errors; `main`
//!   turns them into an exit code and a stderr line (see `error.rs`).
//! * **stdin.** The default input path cannot be driven from a unit test at all.
//!
//! The binary under test is not `cfg(test)`, so the unit-test keychain default
//! in `auth/keychain.rs` does not reach it. `AWARE_DISABLE_KEYRING` makes
//! `<AWARE_HOME>/credentials/` the sole store: the OS keychain is
//! process-global and not scoped by `AWARE_HOME`, so without it these tests
//! would write to (and delete from) the developer's real login keyring.

use assert_cmd::Command;
use predicates::prelude::*;

/// Where a credential lands with the keyring disabled — the sole store here,
/// not a fallback. Mirrors `cred_file_path` in `auth/keychain.rs`.
fn cred_file(home: &std::path::Path, account: &str) -> std::path::PathBuf {
    home.join("credentials").join(format!("{account}.json"))
}

fn aware(home: &std::path::Path) -> Command {
    let mut cmd = Command::cargo_bin("aware").unwrap();
    cmd.env("AWARE_HOME", home)
        .env("AWARE_DISABLE_KEYRING", "1");
    cmd
}

/// Run `aware --json credential …` and parse the object it prints.
fn json_run(home: &std::path::Path, args: &[&str], stdin: Option<&str>) -> serde_json::Value {
    let mut cmd = aware(home);
    cmd.arg("--json").arg("credential").args(args);
    if let Some(body) = stdin {
        cmd.write_stdin(body.to_string());
    }
    let out = cmd.assert().success().get_output().stdout.clone();
    let text = String::from_utf8(out).unwrap();
    serde_json::from_str(&text)
        .unwrap_or_else(|e| panic!("credential {args:?} printed non-JSON ({e}): {text}"))
}

fn status_of(home: &std::path::Path, handle: &str) -> String {
    json_run(home, &["status", handle], None)["status"]
        .as_str()
        .unwrap()
        .to_string()
}

/// The full lifecycle the issue asks for, in the order a caller performs it:
/// provision, rotate, revoke — with `status` as the observer at each step and
/// the store never touched by hand.
#[test]
fn put_rotate_and_delete_compose_into_a_lifecycle() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();

    assert_eq!(status_of(home, "floless-workspace"), "missing");

    let put = json_run(home, &["put", "floless-workspace"], Some("tk_first\n"));
    assert_eq!(put["status"], "stored");
    assert_eq!(put["handle"], "floless-workspace");
    assert_eq!(status_of(home, "floless-workspace"), "present");

    // Rotation: the same verb over a provisioned handle reports `rotated` and the
    // stored value is the new one. Reporting `stored` again would leave a caller
    // unable to tell a rotation from a first provision.
    let rotated = json_run(home, &["put", "floless-workspace"], Some("tk_second\n"));
    assert_eq!(rotated["status"], "rotated");
    assert_eq!(stored_secret(home, "floless-workspace"), "tk_second");

    let deleted = json_run(home, &["delete", "floless-workspace"], None);
    assert_eq!(deleted["status"], "revoked");
    assert_eq!(status_of(home, "floless-workspace"), "missing");
    // Revoked means gone from the store, not merely reported gone — the file the
    // runtime reads must not survive.
    assert!(!cred_file(home, "floless-workspace").is_file());
}

/// Read back the secret exactly as the runtime does.
///
/// `runtime::invoker::resolve_rest_credential` → `load_secret_value` →
/// `runtime::context::load_secret` → `auth::keychain::load_token`, whose value
/// is then probed by `secret_as_str` for `token` / `access_token` / … . So
/// `access_token` in this file is the field that actually reaches an
/// `Authorization: Bearer` header, and asserting it here is asserting the
/// command wrote something the runtime can resolve.
fn stored_secret(home: &std::path::Path, account: &str) -> String {
    let body = std::fs::read_to_string(cred_file(home, account))
        .unwrap_or_else(|e| panic!("no credential file for {account}: {e}"));
    let v: serde_json::Value = serde_json::from_str(&body).unwrap();
    v["access_token"].as_str().unwrap().to_string()
}

/// The stored credential carries the shape the runtime's resolver consumes —
/// including `source: paste`, which is what keeps `connect --list` and `doctor`
/// from offering `--refresh` on a credential nothing can refresh.
#[test]
fn the_stored_credential_has_the_shape_the_runtime_resolves() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();

    aware(home)
        .args(["credential", "put", "my-api"])
        .write_stdin("tk_runtime")
        .assert()
        .success();

    let body = std::fs::read_to_string(cred_file(home, "my-api")).unwrap();
    let v: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(v["access_token"], "tk_runtime");
    assert_eq!(v["source"], "paste");
    assert_eq!(v["token_type"], "Bearer");
    assert_eq!(v["integration"], "my-api");
    // No expiry is claimed: AWARE runs no refresh flow for an opaque handle, so
    // recording a deadline would advertise an enforcement that does not exist.
    assert_eq!(v["expires_at"], 0);
    assert_eq!(v["refresh_token"], serde_json::Value::Null);
}

/// An alias is folded into the handle the runtime looks up, so the credential
/// lands at `<handle>.<alias>` — the string an agent manifest must name.
#[test]
fn an_alias_lands_at_the_account_the_runtime_looks_up() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();

    let put = json_run(
        home,
        &["put", "floless-workspace", "--as", "session"],
        Some("tk_aliased"),
    );
    assert_eq!(put["handle"], "floless-workspace.session");
    assert_eq!(
        stored_secret(home, "floless-workspace.session"),
        "tk_aliased"
    );
    // The unaliased handle is a different credential, not the same one.
    assert!(!cred_file(home, "floless-workspace").is_file());
}

/// Revocation is idempotent: a handle that was never provisioned is success with
/// `absent`, so a caller tearing down on sign-out need not check first.
#[test]
fn deleting_an_absent_credential_succeeds_and_says_so() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();

    let first = json_run(home, &["delete", "never-provisioned"], None);
    assert_eq!(first["status"], "absent");

    aware(home)
        .args(["credential", "put", "twice-deleted"])
        .write_stdin("tk")
        .assert()
        .success();
    assert_eq!(
        json_run(home, &["delete", "twice-deleted"], None)["status"],
        "revoked"
    );
    // The second delete of the same handle is the idempotent case that matters:
    // a retried teardown must not fail.
    assert_eq!(
        json_run(home, &["delete", "twice-deleted"], None)["status"],
        "absent"
    );
}

/// A stored-but-unreadable credential is reported apart from a missing one, and
/// still at exit 0 — `status` answers in its field, not its exit code. Collapsing
/// the two would send a caller down the "provision it" path while a credential is
/// in fact present and broken.
#[test]
fn an_unreadable_credential_is_reported_apart_from_a_missing_one() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();
    std::fs::create_dir_all(home.join("credentials")).unwrap();
    std::fs::write(cred_file(home, "corrupt-api"), "not json at all").unwrap();

    assert_eq!(status_of(home, "corrupt-api"), "unreadable");
    // And it can still be revoked — a credential you cannot read is exactly one
    // you want gone.
    assert_eq!(
        json_run(home, &["delete", "corrupt-api"], None)["status"],
        "revoked"
    );
    assert_eq!(status_of(home, "corrupt-api"), "missing");
}

/// `status` answers with the REST transport's format rules, not the narrower set
/// `auth::keychain::load_token` parses.
///
/// Each shape here is one `runtime::invoker::secret_as_str` accepts and
/// `load_token` does not, so before this was shared each reported `unreadable`
/// while the runtime authenticated with it every call. `{"key": …}` is not a
/// hypothetical: it is the fixture the invoker's own
/// `rest_injects_declared_apikey_header_from_credential` uses.
#[test]
fn status_recognises_every_shape_the_transport_can_authenticate_with() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();
    std::fs::create_dir_all(home.join("credentials")).unwrap();

    for (handle, body) in [
        ("keyed-api", r#"{"key":"sk-live-123"}"#),
        ("apikey-api", r#"{"apikey":"sk-live-123"}"#),
        ("snake-api", r#"{"api_key":"sk-live-123"}"#),
        ("value-api", r#"{"value":"sk-live-123"}"#),
        ("secret-api", r#"{"secret":"sk-live-123"}"#),
        // A bare JSON string is a credential file the transport also accepts.
        ("bare-api", r#""sk-live-123""#),
    ] {
        std::fs::write(cred_file(home, handle), body).unwrap();
        assert_eq!(
            status_of(home, handle),
            "present",
            "{handle} holds {body}, which the REST transport authenticates with, \
             but status did not report it as present"
        );
    }
}

/// The other side of that: sharing the transport's rules must not turn
/// `unreadable` into a state nothing can reach. A file that will not parse fails
/// both readers and is still reported apart from a missing one.
#[test]
fn status_still_reports_a_genuinely_corrupt_credential_as_unreadable() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();
    std::fs::create_dir_all(home.join("credentials")).unwrap();

    std::fs::write(cred_file(home, "corrupt-api"), "not json at all").unwrap();
    assert_eq!(status_of(home, "corrupt-api"), "unreadable");
    // Valid JSON, but no field any reader can pull a secret out of.
    std::fs::write(
        cred_file(home, "empty-obj-api"),
        r#"{"note":"nothing here"}"#,
    )
    .unwrap();
    assert_eq!(status_of(home, "empty-obj-api"), "unreadable");
}

/// The `missing` hint names the ACCOUNT, not the bare handle. With `--as session`
/// the bare handle provisions a *different* credential and leaves this one
/// missing, so the suggested command would report success and fix nothing.
#[test]
fn the_missing_hint_names_the_account_so_running_it_actually_provisions_it() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();

    let out = aware(home)
        .args(["credential", "status", "foo", "--as", "session"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(out).unwrap();
    assert!(
        text.contains("aware credential put foo.session"),
        "the missing hint would provision a different account: {text}"
    );

    // Run the hint verbatim and the handle it was reported against goes present —
    // which is the only thing that makes it a recovery instruction.
    aware(home)
        .args(["credential", "put", "foo.session"])
        .write_stdin("tk_from_the_hint")
        .assert()
        .success();
    assert_eq!(status_of(home, "foo.session"), "present");
    assert_eq!(stored_secret(home, "foo.session"), "tk_from_the_hint");
}

/// A registered OAuth integration is refused, and the message names the command
/// that does own it. Storing one here would be shadowed at run time anyway:
/// `resolve_rest_credential` prefers the refreshed OAuth token for a registered
/// base, so the credential would be written and never read.
#[test]
fn a_registered_integration_is_refused_and_points_at_connect() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();

    aware(home)
        .args(["credential", "put", "google-workspace"])
        .write_stdin("tk")
        .assert()
        .code(3)
        .stderr(predicate::str::contains("aware connect google-workspace"));

    assert!(!cred_file(home, "google-workspace").is_file());
}

/// A handle that would escape `~/.aware/credentials/` is refused at exit 3, and
/// nothing is written anywhere. Exercised through the binary because the failure
/// this guards against is a file appearing outside the store.
#[test]
fn a_traversing_handle_is_refused_and_writes_nothing() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();
    let outside = tmp.path().join("escaped.json");

    aware(home)
        .args(["credential", "put", "../escaped"])
        .write_stdin("tk")
        .assert()
        .code(3);

    assert!(!outside.is_file());
    assert!(!home.join("credentials").join("escaped.json").is_file());
    // The store was never even created — the handle is rejected before any write.
    assert!(
        !home
            .join("credentials")
            .join("..")
            .join("escaped.json")
            .is_file()
    );
}

/// The default input path. An empty stdin is refused rather than stored: a blank
/// credential would report the handle as provisioned while every call it
/// authorizes fails with an empty bearer.
#[test]
fn an_empty_stdin_is_refused_rather_than_stored() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();

    aware(home)
        .args(["credential", "put", "blank-api"])
        .write_stdin("   \n")
        .assert()
        .code(3)
        .stderr(predicate::str::contains("empty credential"));

    assert_eq!(status_of(home, "blank-api"), "missing");
}

/// `--from-env` end to end: the derived variable is read and the value lands in
/// the store. The derivation itself is pinned by a unit test; what this adds is
/// that the flag routing and the store write around it actually run.
#[test]
fn from_env_reads_the_variable_named_for_the_handle() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();

    aware(home)
        .env("AWARE_TOKEN_FLOLESS_WORKSPACE_SESSION", "  tk_from_env  ")
        .args([
            "credential",
            "put",
            "floless-workspace",
            "--as",
            "session",
            "--from-env",
        ])
        .assert()
        .success();

    assert_eq!(
        stored_secret(home, "floless-workspace.session"),
        "tk_from_env"
    );
}

/// Two input sources at once is a validation error, not a silent precedence
/// rule — a caller that set both is wrong about which secret it is storing.
#[test]
fn two_input_sources_at_once_are_refused() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();
    let path = tmp.path().join("secret.txt");
    std::fs::write(&path, "tk_file").unwrap();

    aware(home)
        .env("AWARE_TOKEN_MY_API", "tk_env")
        .args(["credential", "put", "my-api", "--from-env"])
        .arg("--from-file")
        .arg(&path)
        .assert()
        .code(3)
        .stderr(predicate::str::contains("at most one of"));

    assert_eq!(status_of(home, "my-api"), "missing");
}

/// The secret never appears on stdout, on any verb. A credential echoed into a
/// terminal ends up in scrollback, in CI logs, and in whatever captured the
/// command's output.
#[test]
fn no_verb_ever_prints_the_secret() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();
    const SECRET: &str = "tk_must_never_be_echoed";

    let put = aware(home)
        .args(["credential", "put", "my-api"])
        .write_stdin(SECRET)
        .assert()
        .success();
    let status = aware(home)
        .args(["credential", "status", "my-api"])
        .assert()
        .success();
    let delete = aware(home)
        .args(["credential", "delete", "my-api"])
        .assert()
        .success();

    for out in [put.get_output(), status.get_output(), delete.get_output()] {
        let combined = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(
            !combined.contains(SECRET),
            "a credential verb echoed the secret: {combined}"
        );
    }
}
