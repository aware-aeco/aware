//! A run trace never records a credential the vault resolved (#448).
//!
//! `aware credential put` reads the secret from stdin and never from argv,
//! because argv is readable by every process on the machine, and it stores it
//! under `~/.aware/credentials/` at `0600`. A `--dry-run` then rendered the
//! node's params — `{{ secrets… }}` interpolation included — and wrote the
//! result into `~/.aware/logs/<app>/<instance>/<run>.jsonl`, a `0644` file that
//! `aware app logs` prints on request. Both protections were undone by the
//! preview.
//!
//! The pattern is not hypothetical: `20-agents/_core/http` documents it in its
//! own manifest ("Put auth here, e.g. `Authorization: \"Bearer {{ secrets.api
//! }}\"`"), and `post` is write-mode — so the substrate's own escape-hatch
//! agent, used exactly as documented, put the bearer in the trace.
//!
//! These run through the binary because the claim is about a file on disk and
//! what a second command prints out of it, neither of which a unit test over
//! the renderer can see. `AWARE_DISABLE_KEYRING` makes
//! `<AWARE_HOME>/credentials/` the sole store, so the tests never touch the
//! developer's real login keyring (the OS keychain is process-global and not
//! scoped by `AWARE_HOME`).

use assert_cmd::Command;

/// Long and distinctive so a substring search for it cannot match anything the
/// runtime prints for its own reasons.
const SECRET: &str = "sk-live-must-never-reach-a-trace-9f13a7";

fn aware(home: &std::path::Path) -> Command {
    let mut cmd = Command::cargo_bin("aware").unwrap();
    cmd.env("AWARE_HOME", home)
        .env("AWARE_DISABLE_KEYRING", "1");
    cmd
}

fn repo_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn copy_dir(from: &std::path::Path, to: &std::path::Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let dest = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &dest)?;
        } else {
            std::fs::copy(entry.path(), dest)?;
        }
    }
    Ok(())
}

/// A home with the real `http` agent installed, the credential provisioned, and
/// a one-shot app whose single node carries the bearer the way that agent's
/// manifest documents. `mode` is the http command, so the caller chooses
/// write-mode (`post`, which emits `would-write`) or read-mode (`get`).
fn home_with_app(tmp: &std::path::Path, app: &str, command: &str, url: &str) -> std::path::PathBuf {
    let home = tmp.join("aware");
    copy_dir(
        &repo_root().join("20-agents/_core/http"),
        &home.join("agents/http"),
    )
    .unwrap();

    let app_dir = home.join("apps").join(app);
    std::fs::create_dir_all(&app_dir).unwrap();
    // A write-mode node needs a `safety:` block per app-spec § Safety contract;
    // the read-mode one must not carry it. Written out per command rather than
    // always included, so the app is one `aware app validate` would accept.
    let safety = if command == "post" {
        "    safety:\n      transaction-group: redaction-probe\n      snapshot: false\n"
    } else {
        ""
    };
    std::fs::write(
        app_dir.join(format!("{app}.flo")),
        format!(
            r#"app: {app}
version: 0.0.1
description: interpolates a vault credential into a request header
nodes:
  - id: call
    agent: http
    command: {command}
{safety}    config:
      url: "{url}"
      headers:
        Authorization: "Bearer {{{{ secrets['my-api'].access_token }}}}"
connections: []
requires: []
"#
        ),
    )
    .unwrap();

    aware(&home)
        .args(["credential", "put", "my-api"])
        .write_stdin(SECRET)
        .assert()
        .success();

    home
}

/// Every `.jsonl` trace under this home, concatenated.
fn traces(home: &std::path::Path) -> String {
    let mut all = String::new();
    let mut stack = vec![home.join("logs")];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "jsonl") {
                all.push_str(&std::fs::read_to_string(&path).unwrap());
            }
        }
    }
    all
}

/// The whole point: `--dry-run` and `--simulate` write a `would-write` record
/// carrying the node's rendered params, and the credential must not be in it.
///
/// Both modes, because the emit site is shared but reached down two different
/// paths, and a fix applied to one would look complete.
#[test]
fn a_dry_run_trace_never_records_the_vault_credential() {
    let tmp = tempfile::tempdir().unwrap();
    // Unreachable on purpose — neither mode may touch the transport, so the
    // port never has to answer. A run that somehow did reach it would fail here
    // rather than pass on a request nobody looked at.
    let home = home_with_app(
        tmp.path(),
        "redact-probe",
        "post",
        "http://127.0.0.1:1/unused",
    );

    for mode in ["--dry-run", "--simulate"] {
        aware(&home)
            .args(["app", "run", "redact-probe", mode])
            .assert()
            .success();
    }

    let trace = traces(&home);
    assert!(
        !trace.contains(SECRET),
        "a run trace recorded the vault credential:\n{trace}"
    );
    // The header is still previewed, populated, in both traces — a preview that
    // dropped the field entirely would also pass the assertion above while
    // telling the operator nothing about what the write would send.
    assert_eq!(
        trace.matches("Bearer [redacted]").count(),
        2,
        "each mode must preview the header with the value blinded:\n{trace}"
    );
}

/// `aware app logs` is the second half of the exposure: the trace is a `0644`
/// file, and this command prints it. Asserted separately because a fix that
/// redacted only at display time would leave the file itself readable.
#[test]
fn app_logs_never_prints_the_vault_credential() {
    let tmp = tempfile::tempdir().unwrap();
    let home = home_with_app(
        tmp.path(),
        "logs-probe",
        "post",
        "http://127.0.0.1:1/unused",
    );

    aware(&home)
        .args(["app", "run", "logs-probe", "--dry-run"])
        .assert()
        .success();

    let out = aware(&home)
        .args(["app", "logs", "logs-probe"])
        .assert()
        .success()
        .get_output()
        .clone();
    let printed = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        !printed.contains(SECRET),
        "`app logs` printed the vault credential: {printed}"
    );
}

/// The other direction, and the one that makes the fix a redaction rather than
/// a removal: a LIVE run still authenticates with the real secret.
///
/// Blinding is confined to the persisted record. If it ever leaked into the
/// value the transport sends, every credentialed call would start failing
/// authentication — which the tests above cannot see, because nothing in them
/// reaches a server.
#[test]
fn a_live_run_still_sends_the_real_credential() {
    use std::io::{Read, Write};
    use std::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let (tx, rx) = std::sync::mpsc::channel();
    let server = std::thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            stream
                .set_read_timeout(Some(std::time::Duration::from_millis(300)))
                .unwrap();
            let mut data = Vec::new();
            let mut buf = [0u8; 1024];
            loop {
                match stream.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => data.extend_from_slice(&buf[..n]),
                    Err(_) => break,
                }
            }
            let _ = tx.send(String::from_utf8_lossy(&data).to_string());
            let body = r#"{"ok":true}"#;
            let resp = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            let _ = stream.write_all(resp.as_bytes());
        }
    });

    let tmp = tempfile::tempdir().unwrap();
    // Read-mode `get`: a live write-mode node is governed by its safety
    // contract, and what this test needs to observe — the rendered header on
    // the wire — is produced by the same `render_config` call either way.
    let home = home_with_app(
        tmp.path(),
        "live-probe",
        "get",
        &format!("http://127.0.0.1:{port}/ping"),
    );

    aware(&home)
        .args(["app", "run", "live-probe"])
        .assert()
        .success();

    let request = rx.recv().unwrap();
    assert!(
        request.contains(&format!("Bearer {SECRET}")),
        "the live request must carry the real credential, not the redaction: {request}"
    );
    server.join().unwrap();
}

/// A credential whose value is a **number** is blinded too (#450, Codex).
///
/// `aware credential put` always stores a string, so blinding only strings
/// looked complete. But `runtime::context::load_secret` also reads
/// `<AWARE_HOME>/credentials/<id>.json` directly and parses **arbitrary** JSON
/// into the `secrets` namespace — the legacy / hand-written path the loader
/// exists to support. A `pin.json` holding `123456` is a credential whose value
/// is a number, and it was written to the trace verbatim.
///
/// Both routes to one, because they are separate arms of the walk: the whole
/// value being a scalar, and a scalar sitting in a field.
#[test]
fn a_numeric_hand_written_credential_is_blinded_too() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("aware");
    std::fs::create_dir_all(home.join("credentials")).unwrap();
    std::fs::write(home.join("credentials/pin.json"), "123456").unwrap();
    std::fs::write(home.join("credentials/pinobj.json"), r#"{"token":987654}"#).unwrap();
    copy_dir(
        &repo_root().join("20-agents/_core/http"),
        &home.join("agents/http"),
    )
    .unwrap();

    let app_dir = home.join("apps/pin-probe");
    std::fs::create_dir_all(&app_dir).unwrap();
    std::fs::write(
        app_dir.join("pin-probe.flo"),
        r#"app: pin-probe
version: 0.0.1
description: interpolates numeric hand-written credentials
nodes:
  - id: call
    agent: http
    command: post
    safety:
      transaction-group: pin-probe
      snapshot: false
    config:
      url: "http://127.0.0.1:1/unused"
      headers:
        X-Bare: "{{ secrets.pin }}"
        X-Field: "{{ secrets.pinobj.token }}"
connections: []
requires: []
"#,
    )
    .unwrap();

    aware(&home)
        .args(["app", "run", "pin-probe", "--dry-run"])
        .assert()
        .success();

    let trace = traces(&home);
    for pin in ["123456", "987654"] {
        assert!(
            !trace.contains(pin),
            "a numeric credential reached the trace ({pin}):\n{trace}"
        );
    }
    assert_eq!(
        trace.matches("[redacted]").count(),
        2,
        "both numeric credentials must be previewed as blinded, not dropped:\n{trace}"
    );
}

/// A credential whose secret material is in an object **key** is blinded too
/// (#450, Codex).
///
/// `{{ secrets.<id> }}` is a whole-value ref, so `render_config` resolves the
/// object structurally into the record. Blinding only the values left
/// `{"987654":"[redacted]"}` in the trace — the key carrying the secret, and the
/// app template names only the credential id, so nothing else in the record
/// would have revealed it.
///
/// The other half of the assertion is what makes this a redaction rather than a
/// wrecking ball: the addressable field names still survive, which is what keeps
/// `{{ secrets.h.access_token }}` resolving. Collapsing the whole credential to a
/// bare string closes the leak too, and previews the documented header as
/// `Bearer ` — an operator reads that as *no credential at all*.
#[test]
fn a_credential_hiding_in_an_object_key_is_blinded_too() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("aware");
    std::fs::create_dir_all(home.join("credentials")).unwrap();
    std::fs::write(home.join("credentials/pin.json"), r#"{"987654": true}"#).unwrap();
    copy_dir(
        &repo_root().join("20-agents/_core/http"),
        &home.join("agents/http"),
    )
    .unwrap();

    aware(&home)
        .args(["credential", "put", "my-api"])
        .write_stdin(SECRET)
        .assert()
        .success();

    let app_dir = home.join("apps/key-probe");
    std::fs::create_dir_all(&app_dir).unwrap();
    std::fs::write(
        app_dir.join("key-probe.flo"),
        r#"app: key-probe
version: 0.0.1
description: passes a whole hand-written credential, and a field of a real one
nodes:
  - id: call
    agent: http
    command: post
    safety:
      transaction-group: key-probe
      snapshot: false
    config:
      url: "http://127.0.0.1:1/unused"
      headers:
        X-Whole: "{{ secrets.pin }}"
        Authorization: "Bearer {{ secrets['my-api'].access_token }}"
connections: []
requires: []
"#,
    )
    .unwrap();

    aware(&home)
        .args(["app", "run", "key-probe", "--dry-run"])
        .assert()
        .success();

    let trace = traces(&home);
    assert!(
        !trace.contains("987654"),
        "a credential hiding in an object key reached the trace:\n{trace}"
    );
    assert!(
        trace.contains("Bearer [redacted]"),
        "an addressable field must still resolve, or the preview says `Bearer ` \
         and reads as a missing credential:\n{trace}"
    );
}
