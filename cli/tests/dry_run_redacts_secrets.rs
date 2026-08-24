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

/// A credential whose secret material is in an object **key** is blinded too,
/// and a CUSTOM field the template names still previews as `[redacted]` rather
/// than going missing (#450, Codex).
///
/// `{{ secrets.<id> }}` is a whole-value ref, so `render_config` resolves the
/// object structurally into the record. Blinding only the values left
/// `{"987654":"[redacted]"}` in the trace — the key carrying the secret, and the
/// app template names only the credential id, so nothing else in the record
/// would have revealed it.
///
/// The second half is what keeps this a redaction rather than a wrecking ball.
/// A key the template DOES name is already written in the app file, so keeping
/// it reveals nothing; and it has to be kept, or `{{ secrets.teams.coord }}` —
/// the shape `revit-2026/commands/link.reload-all.md` documents — previews as an
/// empty `channel-id`, which an operator reads as a missing credential.
#[test]
fn a_credential_hiding_in_an_object_key_is_blinded_but_a_named_field_is_not_lost() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("aware");
    std::fs::create_dir_all(home.join("credentials")).unwrap();
    std::fs::write(home.join("credentials/pin.json"), r#"{"987654": true}"#).unwrap();
    std::fs::write(
        home.join("credentials/teams.json"),
        r#"{"coord":"19:abcdef@thread","unnamed":"sk-sibling"}"#,
    )
    .unwrap();
    // A field whose name needs bracket syntax because it carries punctuation.
    std::fs::write(
        home.join("credentials/custom.json"),
        r#"{"api.key":"sk-dotted-secret"}"#,
    )
    .unwrap();
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
description: a whole hand-written credential, a custom field, and a real bearer
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
        X-Channel: "{{ secrets.teams.coord }}"
        X-Dotted: "{{ secrets.custom['api.key'] }}"
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
    for leaked in [
        "987654",
        "19:abcdef@thread",
        "sk-sibling",
        "sk-dotted-secret",
    ] {
        assert!(
            !trace.contains(leaked),
            "credential material reached the trace ({leaked}):\n{trace}"
        );
    }
    assert!(
        trace.contains(r#""X-Channel":"[redacted]""#),
        "a template-named custom field must preview as blinded, not missing — an \
         empty value reads as no credential at all:\n{trace}"
    );
    assert!(
        trace.contains(r#""X-Dotted":"[redacted]""#),
        "a bracket-lookup key names the field verbatim; splitting it on the dot \
         blinds the field the template asked for and previews it empty:\n{trace}"
    );
    assert!(
        trace.contains("Bearer [redacted]"),
        "the documented bearer must still resolve:\n{trace}"
    );
}

/// A credential the runtime itself lifted into a `for-each` binding is blinded
/// (#450, Codex).
///
/// `run_for_each` resolves its collection against the LIVE context and binds each
/// element under `upstream["item"]`, so a `for-each: "{{ secrets.batch }}"` walks
/// the credential out of the namespace the redaction covers. The body node's
/// `{{ item }}` then wrote the real element into the trace.
///
/// This is the one hop past `secrets` that gets covered, and the reason is
/// provenance, not proximity: the runtime created this binding out of the vault,
/// so it knows what it is. A credential a node READ — out of a file, or computed
/// by a `compare` — is that node's output, which the trace records in full
/// regardless of this change.
#[test]
fn a_for_each_binding_drawn_from_the_vault_is_blinded() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("aware");
    std::fs::create_dir_all(home.join("credentials")).unwrap();
    std::fs::write(
        home.join("credentials/batch.json"),
        r#"["sk-batch-one","sk-batch-two"]"#,
    )
    .unwrap();
    // The nested-loop shape: an inner `for-each` selecting out of an element the
    // outer loop already lifted from the vault.
    std::fs::write(
        home.join("credentials/nested.json"),
        r#"[{"tokens":["sk-inner-one","sk-inner-two"]}]"#,
    )
    .unwrap();
    copy_dir(
        &repo_root().join("20-agents/_core/http"),
        &home.join("agents/http"),
    )
    .unwrap();

    let app_dir = home.join("apps/loop-probe");
    std::fs::create_dir_all(&app_dir).unwrap();
    std::fs::write(
        app_dir.join("loop-probe.flo"),
        r#"app: loop-probe
version: 0.0.1
description: iterates the vault and posts each element
nodes:
  - id: loop
    for-each: "{{ secrets.batch }}"
    do:
      - id: send
        agent: http
        command: post
        safety:
          transaction-group: loop-probe
          snapshot: false
        config:
          url: "http://127.0.0.1:1/unused"
          body:
            token: "{{ item }}"
  - id: outer
    for-each: "{{ secrets.nested }}"
    do:
      - id: inner
        for-each: "{{ upstream.item.tokens }}"
        do:
          - id: send-inner
            agent: http
            command: post
            safety:
              transaction-group: loop-probe
              snapshot: false
            config:
              url: "http://127.0.0.1:1/unused"
              body:
                token: "{{ item }}"
connections: []
requires: []
"#,
    )
    .unwrap();

    aware(&home)
        .args(["app", "run", "loop-probe", "--dry-run"])
        .assert()
        .success();

    let trace = traces(&home);
    for leaked in [
        "sk-batch-one",
        "sk-batch-two",
        // A nested loop inherits the outer loop's provenance. The inner one
        // uses the `upstream.item` ALIAS, so this pins both gaps at once:
        // keying on the inner loop's own head called it clean, and reading only
        // the head called the alias a different namespace (#450, Codex).
        "sk-inner-one",
        "sk-inner-two",
    ] {
        assert!(
            !trace.contains(leaked),
            "a for-each element lifted out of the vault reached the trace \
             ({leaked}):\n{trace}"
        );
    }
    assert_eq!(
        trace.matches(r#""token":"[redacted]""#).count(),
        4,
        "every iteration, outer loop and nested alike, must preview the element \
         as blinded:\n{trace}"
    );
}

/// A credential ID is a key too, and a dry-run must not fail because of the
/// preview (#450, Codex).
///
/// Two things this pins, both regressions the redaction itself introduced:
///
/// * `load_secret` takes the credential ID from the **file stem**, so a
///   hand-written `sk-live-….json` puts credential material in the id — and a
///   whole-value `{{ secrets }}` writes the entire vault map, keys included.
/// * Blinding changes a value's TYPE. `{{ secrets.pin + 1 }}` is arithmetic on
///   a numeric credential: it renders fine live, but the record's re-render adds
///   `1` to `"[redacted]"` and minijinja refuses. That aborted the whole
///   dry-run — a preview taking down a run that otherwise works.
#[test]
fn a_preview_never_leaks_the_id_and_never_fails_the_run() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("aware");
    std::fs::create_dir_all(home.join("credentials")).unwrap();
    std::fs::write(home.join("credentials/sk-live-in-the-id.json"), r#""tok""#).unwrap();
    std::fs::write(home.join("credentials/pin.json"), "123456").unwrap();
    copy_dir(
        &repo_root().join("20-agents/_core/http"),
        &home.join("agents/http"),
    )
    .unwrap();

    let app_dir = home.join("apps/preview-probe");
    std::fs::create_dir_all(&app_dir).unwrap();
    std::fs::write(
        app_dir.join("preview-probe.flo"),
        r#"app: preview-probe
version: 0.0.1
description: whole-vault reference and arithmetic on a numeric credential
nodes:
  - id: call
    agent: http
    command: post
    safety:
      transaction-group: preview-probe
      snapshot: false
    config:
      url: "http://127.0.0.1:1/unused"
      headers:
        X-Vault: "{{ secrets }}"
        X-Computed: "{{ secrets.pin + 1 }}"
        X-Plain: "literal stays"
connections: []
requires: []
"#,
    )
    .unwrap();

    // Succeeds at all: before the lenient record render this exited non-zero
    // with `tried to use + operator on unsupported types`.
    aware(&home)
        .args(["app", "run", "preview-probe", "--dry-run"])
        .assert()
        .success();

    let trace = traces(&home);
    for leaked in ["sk-live-in-the-id", "123456", "123457"] {
        assert!(
            !trace.contains(leaked),
            "credential material reached the trace ({leaked}):\n{trace}"
        );
    }
    assert!(
        trace.contains(r#""X-Computed":"[redacted]""#),
        "an unpreviewable leaf becomes the redaction, not an error:\n{trace}"
    );
    assert!(
        trace.contains(r#""X-Plain":"literal stays""#),
        "one unpreviewable leaf must not take its siblings down with it:\n{trace}"
    );
}
