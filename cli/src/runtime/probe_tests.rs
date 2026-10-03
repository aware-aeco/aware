//! Unit tests for `runtime::probe` (#617). A sibling file so the runtime module
//! stays readable; it is compiled only under `cfg(test)` via `#[path]`.

use super::*;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

const TOKEN: &str = "probe-secret-token-617";

/// A local HTTP fixture that answers every request with one fixed response
/// and records every request it received, raw.
struct Fixture {
    port: u16,
    seen: Arc<Mutex<Vec<String>>>,
}

impl Fixture {
    fn start(status: u16, extra_headers: &str, body: Vec<u8>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let seen = Arc::new(Mutex::new(Vec::new()));
        let record = seen.clone();
        let extra = extra_headers.to_string();
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(mut stream) = stream else { break };
                let _ = stream.set_read_timeout(Some(Duration::from_secs(2)));
                let mut data = Vec::new();
                let mut chunk = [0u8; 1024];
                while !data.windows(4).any(|w| w == b"\r\n\r\n") {
                    match stream.read(&mut chunk) {
                        Ok(0) | Err(_) => break,
                        Ok(n) => data.extend_from_slice(&chunk[..n]),
                    }
                }
                record
                    .lock()
                    .unwrap()
                    .push(String::from_utf8_lossy(&data).into_owned());
                let head = format!(
                    "HTTP/1.1 {status} X\r\nContent-Type: application/json\r\nContent-Length: {}\r\n{extra}Connection: close\r\n\r\n",
                    body.len()
                );
                let _ = stream.write_all(head.as_bytes());
                let _ = stream.write_all(&body);
            }
        });
        Self { port, seen }
    }

    fn origin(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }

    fn requests(&self) -> Vec<String> {
        self.seen.lock().unwrap().clone()
    }
}

fn rest_manifest(agent: &str, secret: &str, origin: &str, kind: &str) -> String {
    format!(
        r#"agent: {agent}
version: 1.0.0
description: test
stateful: false
license: MIT
transport:
  rest:
    base: "{origin}/api/"
auth:
  scheme: bearer
  secret: {secret}
commands:
  whoami:
    lifecycle: single
    description: Reads who is signed in.
    method: GET
    path: "{origin}/v1/userinfo"
probe:
  command: whoami
  describe: Reads who is signed in.
  kind: {kind}
  rest:
    origin: "{origin}"
  reports:
    summary: /body/email
    identity: /body/email
    stable-id: /body/sub
"#
    )
}

fn home_with(agent: &str, manifest: &str) -> tempfile::TempDir {
    let home = tempfile::tempdir().unwrap();
    let dir = home.path().join("agents").join(agent);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("manifest.yaml"), manifest).unwrap();
    home
}

fn store(home: &Path, account: &str, generation: Option<&str>) {
    let token = crate::auth::keychain::StoredToken {
        access_token: TOKEN.into(),
        refresh_token: None,
        expires_at: 0,
        scope: String::new(),
        token_type: "Bearer".into(),
        integration: account.into(),
        obtained_at: 0,
        generation: generation.map(str::to_string),
        source: crate::auth::keychain::TokenSource::Paste,
    };
    crate::auth::keychain::store_token(&token, None, home).unwrap();
}

fn options(alias: Option<&str>, allow_origin: Option<&str>) -> ProbeOptions {
    ProbeOptions {
        alias: alias.map(str::to_string),
        allow_origin: allow_origin.map(str::to_string),
        expect_generation: None,
        expect_manifest: None,
        timeout: Duration::from_secs(10),
    }
}

fn userinfo_body() -> Vec<u8> {
    br#"{"sub":"1234567890","email":"ada@example.com","email_verified":true,"secret_field":"must-not-leave"}"#.to_vec()
}

/// A home holding the fixture-backed custom-handle agent `svc`.
fn custom_home(server: &Fixture) -> tempfile::TempDir {
    home_with(
        "svc",
        &rest_manifest("svc", "my.api.key", &server.origin(), "account"),
    )
}

#[tokio::test]
async fn a_custom_handle_probe_sends_the_token_only_to_the_confirmed_origin() {
    let server = Fixture::start(200, "", userinfo_body());
    let origin = server.origin();
    let manifest = rest_manifest("svc", "my.api.key", &origin, "account");
    let home = home_with("svc", &manifest);
    store(home.path(), "my.api.key", Some("gen-1"));

    let receipt = probe_agent(home.path(), "svc", &options(None, Some(&origin)))
        .await
        .expect("probe succeeds");
    assert_eq!(receipt.schema, SCHEMA);
    assert_eq!(receipt.transport, "rest");
    assert_eq!(receipt.kind, "account");
    assert_eq!(receipt.alias, None);
    assert_eq!(receipt.credential_generation.as_deref(), Some("gen-1"));
    assert!(
        !receipt.reviewed,
        "a locally written agent is never reviewed"
    );
    assert_eq!(
        receipt.reported.identity.as_deref(),
        Some("ada@example.com")
    );
    assert_eq!(receipt.reported.stable_id.as_deref(), Some("1234567890"));
    assert_eq!(
        receipt.manifest_sha256,
        manifest_sha256(manifest.as_bytes())
    );

    let requests = server.requests();
    assert_eq!(requests.len(), 1);
    assert!(
        requests[0].starts_with("GET /v1/userinfo"),
        "{}",
        requests[0]
    );
    assert!(requests[0].contains(&format!("Bearer {TOKEN}")));

    // Nothing raw leaves: not the token, not an undeclared body field.
    let printed = serde_json::to_string(&receipt).unwrap();
    assert!(!printed.contains(TOKEN));
    assert!(!printed.contains("must-not-leave"));
}

#[tokio::test]
async fn a_custom_handle_needs_a_matching_allow_origin_before_any_request() {
    let server = Fixture::start(200, "", userinfo_body());
    let home = custom_home(&server);
    store(home.path(), "my.api.key", Some("gen-1"));

    for (allow, reason) in [
        (None, "allow-origin-required"),
        (Some("http://127.0.0.1:1"), "allow-origin-mismatch"),
    ] {
        let failure = probe_agent(home.path(), "svc", &options(None, allow))
            .await
            .expect_err("must refuse");
        assert_eq!(failure.code, "E_PROBE_ORIGIN_NOT_ALLOWED");
        assert_eq!(failure.details["reason"], reason);
    }
    assert!(
        server.requests().is_empty(),
        "no request may leave before the origin is allowed"
    );
}

#[tokio::test]
async fn a_redirect_is_refused_and_never_followed_with_the_token() {
    let thief = Fixture::start(200, "", userinfo_body());
    let server = Fixture::start(
        302,
        &format!("Location: {}/steal\r\n", thief.origin()),
        Vec::new(),
    );
    let origin = server.origin();
    let home = custom_home(&server);
    store(home.path(), "my.api.key", None);

    let failure = probe_agent(home.path(), "svc", &options(None, Some(&origin)))
        .await
        .expect_err("a 3xx is a failure");
    assert_eq!(failure.code, "E_PROBE_FAILED");
    assert_eq!(failure.details["status"], 302);
    assert_eq!(
        failure.details.len(),
        1,
        "only the status: {:?}",
        failure.details
    );
    assert_eq!(server.requests().len(), 1);
    assert!(
        thief.requests().is_empty(),
        "the redirect target must receive nothing"
    );
}

#[tokio::test]
async fn a_registered_integration_secret_never_goes_to_a_foreign_origin() {
    // The manifest names the google-workspace credential but points the probe at
    // an origin outside that integration's code-owned allowlist. The refusal must
    // come before the credential is even read — so with NO credential stored, the
    // answer is still the origin refusal, never "missing".
    let server = Fixture::start(200, "", userinfo_body());
    let origin = server.origin();
    let home = home_with(
        "svc",
        &rest_manifest("svc", "google-workspace", &origin, "account"),
    );
    let failure = probe_agent(home.path(), "svc", &options(None, Some(&origin)))
        .await
        .expect_err("must refuse");
    assert_eq!(failure.code, "E_PROBE_ORIGIN_NOT_ALLOWED");
    assert_eq!(
        failure.details["reason"],
        "origin-not-in-integration-allowlist"
    );
    assert!(server.requests().is_empty());
}

#[tokio::test]
async fn an_http_error_is_a_failure_carrying_only_its_status() {
    let server = Fixture::start(401, "", br#"{"error":"secret-body-text"}"#.to_vec());
    let origin = server.origin();
    let home = custom_home(&server);
    store(home.path(), "my.api.key", None);
    let failure = probe_agent(home.path(), "svc", &options(None, Some(&origin)))
        .await
        .expect_err("401 fails");
    assert_eq!(failure.code, "E_PROBE_FAILED");
    assert_eq!(failure.details["status"], 401);
    let printed = format!("{failure:?} {}", failure.message());
    assert!(!printed.contains("secret-body-text"));
    assert!(!printed.contains(TOKEN));
}

#[tokio::test]
async fn a_response_over_64_kib_is_refused() {
    let mut body = br#"{"email":""#.to_vec();
    body.extend(std::iter::repeat_n(b'x', OUTPUT_CAP));
    body.extend_from_slice(br#""}"#);
    let server = Fixture::start(200, "", body);
    let origin = server.origin();
    let home = custom_home(&server);
    store(home.path(), "my.api.key", None);
    let failure = probe_agent(home.path(), "svc", &options(None, Some(&origin)))
        .await
        .expect_err("too large");
    assert_eq!(failure.code, "E_PROBE_OUTPUT_TOO_LARGE");
}

#[tokio::test]
async fn an_account_probe_without_an_identity_is_not_verified() {
    let server = Fixture::start(200, "", br#"{"sub":"1"}"#.to_vec());
    let origin = server.origin();
    let home = custom_home(&server);
    store(home.path(), "my.api.key", None);
    let failure = probe_agent(home.path(), "svc", &options(None, Some(&origin)))
        .await
        .expect_err("no identity");
    assert_eq!(failure.code, "E_PROBE_REPORT_MISSING");
}

#[tokio::test]
async fn a_changed_generation_is_refused_before_the_credential_is_sent() {
    let server = Fixture::start(200, "", userinfo_body());
    let origin = server.origin();
    let home = custom_home(&server);
    store(home.path(), "my.api.key", Some("gen-2"));
    let mut opts = options(None, Some(&origin));
    opts.expect_generation = Some("gen-1".into());
    let failure = probe_agent(home.path(), "svc", &opts)
        .await
        .expect_err("changed");
    assert_eq!(failure.code, "E_CREDENTIAL_CHANGED");
    assert!(server.requests().is_empty());

    opts.expect_generation = Some("gen-2".into());
    let receipt = probe_agent(home.path(), "svc", &opts)
        .await
        .expect("matches");
    assert_eq!(receipt.credential_generation.as_deref(), Some("gen-2"));
}

#[tokio::test]
async fn an_alias_resolves_exactly_its_slot_and_never_falls_back() {
    let server = Fixture::start(200, "", userinfo_body());
    let origin = server.origin();
    let home = custom_home(&server);
    // Only the DEFAULT slot holds a credential.
    store(home.path(), "my.api.key", Some("default-gen"));
    let failure = probe_agent(home.path(), "svc", &options(Some("team"), Some(&origin)))
        .await
        .expect_err("the team slot is empty");
    assert_eq!(failure.code, "E_CREDENTIAL_MISSING");
    assert!(
        server.requests().is_empty(),
        "the default slot must not be used instead"
    );

    // Provision the aliased slot: the opaque dotted handle gains `.team`.
    store(home.path(), "my.api.key.team", Some("team-gen"));
    let receipt = probe_agent(home.path(), "svc", &options(Some("team"), Some(&origin)))
        .await
        .expect("team slot");
    assert_eq!(receipt.alias.as_deref(), Some("team"));
    assert_eq!(receipt.credential_generation.as_deref(), Some("team-gen"));
}

#[tokio::test]
async fn a_registered_alias_slot_never_falls_back_to_the_default_account() {
    // The real google-workspace origin is allowlisted, so the probe reaches the
    // credential step — and an empty `.team` slot must stop it there, before any
    // network call, even though the default slot is provisioned.
    let origin = "https://openidconnect.googleapis.com";
    let home = home_with(
        "gw",
        &rest_manifest("gw", "google-workspace", origin, "account"),
    );
    store(home.path(), "google-workspace", Some("default-gen"));
    let failure = probe_agent(home.path(), "gw", &options(Some("team"), None))
        .await
        .expect_err("no team slot");
    assert_eq!(failure.code, "E_CREDENTIAL_MISSING");
}

#[tokio::test]
async fn the_home_directory_changes_only_in_the_credential_store() {
    // A legacy credential with no generation: the resolver writes one (the
    // documented credential maintenance). Nothing else in the home may change;
    // the log line is the command layer's, checked after.
    let server = Fixture::start(200, "", userinfo_body());
    let origin = server.origin();
    let home = custom_home(&server);
    store(home.path(), "my.api.key", None);
    let before = snapshot(home.path());
    let receipt = probe_agent(home.path(), "svc", &options(None, Some(&origin)))
        .await
        .expect("probe succeeds");
    assert!(
        receipt.credential_generation.is_some(),
        "a legacy credential gains a generation"
    );
    let after = snapshot(home.path());
    let changed: Vec<&String> = after
        .iter()
        .filter(|(path, hash)| before.get(*path) != Some(*hash))
        .map(|(path, _)| path)
        .chain(before.keys().filter(|path| !after.contains_key(*path)))
        .collect();
    assert!(!changed.is_empty(), "the generation write must be visible");
    for path in &changed {
        assert!(path.starts_with("credentials/"), "unexpected write: {path}");
    }

    append_log_line(&home.path().join("logs"), "svc", "ok");
    let logged = snapshot(home.path());
    let added: Vec<&String> = logged.keys().filter(|p| !after.contains_key(*p)).collect();
    assert_eq!(added, ["logs/agent-probe.log"]);
    let line = std::fs::read_to_string(home.path().join("logs/agent-probe.log")).unwrap();
    assert!(line.trim_end().ends_with("agent-probe svc ok"), "{line}");
    assert!(!line.contains(TOKEN));
}

fn snapshot(root: &Path) -> std::collections::BTreeMap<String, String> {
    fn walk(root: &Path, dir: &Path, out: &mut std::collections::BTreeMap<String, String>) {
        for entry in std::fs::read_dir(dir).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(root, &path, out);
            } else {
                let rel = path
                    .strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/");
                out.insert(rel, manifest_sha256(&std::fs::read(&path).unwrap()));
            }
        }
    }
    let mut out = std::collections::BTreeMap::new();
    walk(root, root, &mut out);
    out
}

// ── slot rule (pure) ──────────────────────────────────────────────────────────

fn plan(secret: &str, alias: Option<&str>) -> Result<CredentialPlan, ProbeFailure> {
    let base = secret.split('.').next().unwrap_or(secret);
    let origin = if crate::auth::config::for_integration(base).is_ok() {
        "https://openidconnect.googleapis.com"
    } else {
        "https://api.example.com"
    };
    let manifest = rest_manifest("svc", secret, origin, "account");
    let agent: Agent = serde_yaml::from_str(&manifest).unwrap();
    let decl = parse_probe(&agent).unwrap().unwrap();
    plan_credential(&agent, &decl, alias)
}

fn registered(alias: Option<&str>) -> CredentialPlan {
    CredentialPlan::Registered {
        integration: "google-workspace".into(),
        alias: alias.map(str::to_string),
    }
}

#[test]
fn registered_and_custom_handles_resolve_to_exact_slots() {
    assert_eq!(plan("google-workspace", None), Ok(registered(None)));
    assert_eq!(
        plan("google-workspace", Some("team")),
        Ok(registered(Some("team")))
    );
    assert_eq!(
        plan("google-workspace.personal", None),
        Ok(registered(Some("personal")))
    );
    // A custom handle is opaque: never split at its dots.
    assert_eq!(
        plan("my.api.key", None),
        Ok(CredentialPlan::Custom {
            handle: "my.api.key".into(),
            account: "my.api.key".into(),
            alias: None,
        })
    );
    assert_eq!(
        plan("my.api.key", Some("team")),
        Ok(CredentialPlan::Custom {
            handle: "my.api.key".into(),
            account: "my.api.key.team".into(),
            alias: Some("team".into()),
        })
    );
}

#[test]
fn an_alias_conflicting_with_the_manifest_or_malformed_is_refused() {
    let conflict = plan("google-workspace.personal", Some("team")).unwrap_err();
    assert_eq!(conflict.code, "E_PROBE_ALIAS_CONFLICT");
    assert_eq!(conflict.details["reason"], "alias-already-in-manifest");
    for bad in ["../x", "Team", "a.b", ""] {
        let failure = plan("my.api.key", Some(bad)).unwrap_err();
        assert_eq!(failure.code, "E_PROBE_ALIAS_CONFLICT", "{bad}");
        assert_eq!(failure.details["reason"], "alias-invalid");
    }
}

#[test]
fn the_google_workspace_allowlist_is_the_userinfo_origin_only() {
    let config = crate::auth::config::for_integration("google-workspace").unwrap();
    assert_eq!(
        config.probe_origins(),
        ["https://openidconnect.googleapis.com"]
    );
    for other in ["microsoft-365", "trimble-connect"] {
        assert!(
            crate::auth::config::for_integration(other)
                .unwrap()
                .probe_origins()
                .is_empty()
        );
    }
}

// ── cli transport ─────────────────────────────────────────────────────────────

#[tokio::test]
async fn a_missing_bridge_is_host_unavailable() {
    let manifest = "agent: hostx\nversion: 1.0.0\ndescription: x\nstateful: false\nlicense: MIT\n\
        transport: { cli: { binary: aware-no-such-bridge-617 } }\n\
        commands:\n  model-info: { lifecycle: single, description: Reads. }\n\
        probe: { command: model-info, describe: Reads the model., kind: host }\n";
    let home = home_with("hostx", manifest);
    let failure = probe_agent(home.path(), "hostx", &options(None, None))
        .await
        .expect_err("no bridge");
    assert_eq!(failure.code, "E_HOST_UNAVAILABLE");
    assert_eq!(failure.details["reason"], "bridge-missing");

    let failure = probe_agent(home.path(), "hostx", &options(Some("team"), None))
        .await
        .expect_err("--as on a credential-less probe");
    assert_eq!(failure.code, "E_PROBE_ALIAS_CONFLICT");
}

#[tokio::test]
async fn undeclared_invalid_planned_and_absent_agents_have_their_own_codes() {
    let base = "agent: a\nversion: 1.0.0\ndescription: x\nstateful: false\nlicense: MIT\n\
        transport: { cli: { binary: x } }\ncommands:\n  r: { lifecycle: single, description: d }\n  \
        w: { lifecycle: single, description: d, mode: write }\n";
    let cases = [
        (base.to_string(), "E_PROBE_UNDECLARED"),
        (
            format!("{base}probe: {{ command: w, describe: d, kind: host }}\n"),
            "E_PROBE_INVALID",
        ),
        (
            format!("status: planned\n{base}probe: {{ command: r, describe: d, kind: host }}\n"),
            "E_AGENT_PLANNED",
        ),
    ];
    for (manifest, code) in cases {
        let home = home_with("a", &manifest);
        let failure = probe_agent(home.path(), "a", &options(None, None))
            .await
            .unwrap_err();
        assert_eq!(failure.code, code, "{manifest}");
    }
    let empty = tempfile::tempdir().unwrap();
    for id in ["a", "../a"] {
        let failure = probe_agent(empty.path(), id, &options(None, None))
            .await
            .unwrap_err();
        assert_eq!(failure.code, "E_AGENT_NOT_INSTALLED");
    }
}

#[test]
fn a_bridge_failure_surfaces_only_its_structured_code_and_count() {
    let receipt = json!({
        "status": "err",
        "code": "host-ambiguous",
        "instance_count": 2,
        "message": "C:\\secret\\path leaked by a vendor",
    });
    let failure = host_failure(
        Some(&receipt),
        b"Trimble.Remoting: stack trace C:\\secret",
        Some(4),
    );
    assert_eq!(failure.code, "E_HOST_UNAVAILABLE");
    assert_eq!(
        Json::Object(failure.details.clone()),
        json!({"hostCode": "host-ambiguous", "instanceCount": 2, "exitCode": 4})
    );
    // A free-text "code" is not a structured code.
    let receipt = json!({"status": "err", "code": "Tekla is not running at C:\\x"});
    let failure = host_failure(Some(&receipt), b"", Some(1));
    assert!(!failure.details.contains_key("hostCode"));
    // The model-reader envelope on stderr still counts.
    let stderr = br#"{"code":"reference-provider-host-failed","phase":"x"}"#;
    let failure = host_failure(None, stderr, Some(2));
    assert_eq!(
        failure.details["hostCode"],
        "reference-provider-host-failed"
    );
}

#[test]
fn a_receipt_reporting_failure_is_recognised_whatever_its_exit() {
    assert!(receipt_reports_failure(&json!({"ok": false})));
    assert!(receipt_reports_failure(&json!({"status": "err"})));
    assert!(!receipt_reports_failure(&json!({"status": "ok"})));
}

#[test]
fn reports_are_bounded_strings_or_numbers() {
    let result = json!({
        "name": "Model\u{7}\nA",
        "pid": 25068,
        "obj": {"a": 1},
        "long": "x".repeat(MAX_REPORT_CHARS + 1),
        "blank": "  ",
    });
    assert_eq!(
        extract_report(&result, Some("/name")).as_deref(),
        Some("ModelA")
    );
    assert_eq!(
        extract_report(&result, Some("/pid")).as_deref(),
        Some("25068")
    );
    for pointer in ["/obj", "/long", "/blank", "/missing"] {
        assert_eq!(extract_report(&result, Some(pointer)), None, "{pointer}");
    }
    assert_eq!(extract_report(&result, None), None);
}

// ── bounded supervision ───────────────────────────────────────────────────────

#[cfg(windows)]
fn shell(script: String) -> (PathBuf, Vec<String>) {
    (
        PathBuf::from("powershell.exe"),
        vec![
            "-NoProfile".into(),
            "-NonInteractive".into(),
            "-Command".into(),
            script,
        ],
    )
}

#[cfg(unix)]
fn shell(script: String) -> (PathBuf, Vec<String>) {
    (PathBuf::from("sh"), vec!["-c".into(), script])
}

#[cfg(windows)]
fn pid_alive(pid: u32) -> bool {
    let out = std::process::Command::new("tasklist")
        .args(["/FI", &format!("PID eq {pid}"), "/NH"])
        .output()
        .unwrap();
    String::from_utf8_lossy(&out.stdout).contains(&format!(" {pid} "))
}

#[cfg(unix)]
fn pid_alive(pid: u32) -> bool {
    std::process::Command::new("kill")
        .args(["-0", &pid.to_string()])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

async fn run_script(script: String, timeout: Duration) -> Result<BoundedOutput, BoundedFailure> {
    let (program, args) = shell(script);
    let refs: Vec<&str> = args.iter().map(String::as_str).collect();
    run_bounded(&program, &refs, b"{}".to_vec(), timeout, OUTPUT_CAP).await
}

#[tokio::test]
async fn a_timeout_kills_the_whole_tree_including_a_grandchild() {
    let dir = tempfile::tempdir().unwrap();
    let pidfile = dir.path().join("grandchild.pid");
    #[cfg(windows)]
    let script = format!(
        "$p = Start-Process -PassThru -NoNewWindow -FilePath powershell.exe -ArgumentList '-NoProfile','-Command','Start-Sleep 120'; \
         Set-Content -Path '{}' -Value $p.Id; Start-Sleep 120",
        pidfile.display()
    );
    #[cfg(unix)]
    let script = format!("sleep 120 & echo $! > '{}'; sleep 120", pidfile.display());
    let started = Instant::now();
    let outcome = run_script(script, Duration::from_secs(12)).await;
    assert_eq!(outcome.unwrap_err(), BoundedFailure::Timeout);
    assert!(started.elapsed() < Duration::from_secs(40));
    let pid: u32 = std::fs::read_to_string(&pidfile)
        .expect("the child spawned its grandchild before the deadline")
        .trim()
        .parse()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    while pid_alive(pid) && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(200));
    }
    assert!(!pid_alive(pid), "grandchild {pid} survived the timeout");
}

#[tokio::test]
async fn stdout_over_the_cap_is_refused_without_waiting_for_the_deadline() {
    #[cfg(windows)]
    let script = format!(
        "[Console]::Out.Write('x' * {}); [Console]::Out.Flush(); Start-Sleep 120",
        OUTPUT_CAP + 10
    );
    #[cfg(unix)]
    let script = format!(
        "head -c {} /dev/zero | tr '\\0' x; sleep 120",
        OUTPUT_CAP + 10
    );
    let started = Instant::now();
    let outcome = run_script(script, Duration::from_secs(50)).await;
    assert_eq!(outcome.unwrap_err(), BoundedFailure::StdoutTooLarge);
    assert!(started.elapsed() < Duration::from_secs(30));
}

#[tokio::test]
async fn a_normal_exit_returns_stdout_and_status() {
    #[cfg(windows)]
    let script = "[Console]::Out.Write('ok-done')".to_string();
    #[cfg(unix)]
    let script = "printf ok-done".to_string();
    let output = run_script(script, Duration::from_secs(30))
        .await
        .expect("runs");
    assert!(output.status.success());
    assert_eq!(output.stdout, b"ok-done");
}

#[tokio::test]
async fn a_service_that_never_answers_times_out_within_the_deadline() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    std::thread::spawn(move || {
        let held: Vec<_> = listener.incoming().take(4).collect();
        std::thread::sleep(Duration::from_secs(30));
        drop(held);
    });
    let origin = format!("http://127.0.0.1:{port}");
    let home = home_with(
        "svc",
        &rest_manifest("svc", "my.api.key", &origin, "account"),
    );
    store(home.path(), "my.api.key", None);
    let mut opts = options(None, Some(&origin));
    opts.timeout = Duration::from_millis(1500);
    let started = Instant::now();
    let failure = probe_agent(home.path(), "svc", &opts)
        .await
        .expect_err("must time out");
    assert_eq!(failure.code, "E_PROBE_TIMEOUT");
    assert!(
        started.elapsed() < Duration::from_secs(5),
        "the deadline bounds the whole probe: {:?}",
        started.elapsed()
    );
}

#[tokio::test]
async fn a_probe_whose_inputs_fill_the_auth_slot_is_refused_before_sending() {
    // A header input named Authorization would carry a token typed into the
    // manifest instead of the stored credential — the receipt would then name a
    // credential generation that never left AWARE.
    let server = Fixture::start(200, "", userinfo_body());
    let origin = server.origin();
    let manifest = rest_manifest("svc", "my.api.key", &origin, "account")
        .replace(
            "    path: \"",
            "    inputs:\n      Authorization: { type: string, in: header }\n    path: \"",
        )
        .replace(
            "  describe: Reads who is signed in.\n  kind",
            "  inputs: { Authorization: \"Bearer typed-into-the-manifest\" }\n  describe: Reads who is signed in.\n  kind",
        );
    let home = home_with("svc", &manifest);
    store(home.path(), "my.api.key", Some("gen-1"));
    let failure = probe_agent(home.path(), "svc", &options(None, Some(&origin)))
        .await
        .expect_err("the stored credential would not be what authenticates");
    assert_eq!(failure.code, "E_PROBE_INVALID");
    assert_eq!(failure.details["reason"], "auth-not-attached");
    assert!(server.requests().is_empty());
}

#[tokio::test]
async fn a_folder_that_is_not_its_manifests_agent_is_refused() {
    let home = home_with("other", HOST_AGENT_FOR_ID_TEST);
    let failure = probe_agent(home.path(), "other", &options(None, None))
        .await
        .unwrap_err();
    assert_eq!(failure.code, "E_PROBE_INVALID");
    assert_eq!(failure.details["reason"], "agent-id-mismatch");
}

const HOST_AGENT_FOR_ID_TEST: &str = "agent: hostx\nversion: 1.0.0\ndescription: x\nstateful: false\nlicense: MIT\n\
    transport: { cli: { binary: aware-no-such-bridge-617 } }\n\
    commands:\n  model-info: { lifecycle: single, description: Reads. }\n\
    probe: { command: model-info, describe: Reads the model., kind: host }\n";

#[tokio::test]
async fn a_registered_generation_mismatch_is_refused_before_any_refresh() {
    // An EXPIRED google-workspace token with no refresh token: had the refresh
    // run first, the answer would be E_CREDENTIAL_EXPIRED. The expected
    // generation must be compared before the resolver touches the slot.
    let origin = "https://openidconnect.googleapis.com";
    let home = home_with(
        "gw",
        &rest_manifest("gw", "google-workspace", origin, "account"),
    );
    let token = crate::auth::keychain::StoredToken {
        access_token: TOKEN.into(),
        refresh_token: None,
        expires_at: 1,
        scope: "openid".into(),
        token_type: "Bearer".into(),
        integration: "google-workspace".into(),
        obtained_at: 0,
        generation: Some("gen-real".into()),
        source: crate::auth::keychain::TokenSource::Oauth,
    };
    crate::auth::keychain::store_token(&token, None, home.path()).unwrap();
    let mut opts = options(None, None);
    opts.expect_generation = Some("gen-other".into());
    let failure = probe_agent(home.path(), "gw", &opts).await.unwrap_err();
    assert_eq!(failure.code, "E_CREDENTIAL_CHANGED");

    opts.expect_generation = Some("gen-real".into());
    let failure = probe_agent(home.path(), "gw", &opts).await.unwrap_err();
    assert_eq!(failure.code, "E_CREDENTIAL_EXPIRED");
}

#[tokio::test]
async fn abandoning_a_supervised_run_still_kills_the_grandchild() {
    // The probe's outer deadline can drop `run_bounded` before its own deadline
    // fires. Dropping the future must take the whole tree with it.
    let dir = tempfile::tempdir().unwrap();
    let pidfile = dir.path().join("grandchild.pid");
    #[cfg(windows)]
    let script = format!(
        "$p = Start-Process -PassThru -NoNewWindow -FilePath powershell.exe -ArgumentList '-NoProfile','-Command','Start-Sleep 120'; \
         Set-Content -Path '{}' -Value $p.Id; Start-Sleep 120",
        pidfile.display()
    );
    #[cfg(unix)]
    let script = format!("sleep 120 & echo $! > '{}'; sleep 120", pidfile.display());
    let outer = tokio::time::timeout(
        Duration::from_secs(12),
        run_script(script, Duration::from_secs(120)),
    )
    .await;
    assert!(outer.is_err(), "the outer deadline fires first");
    let pid: u32 = std::fs::read_to_string(&pidfile)
        .expect("the grandchild was spawned")
        .trim()
        .parse()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    while pid_alive(pid) && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(200));
    }
    assert!(
        !pid_alive(pid),
        "grandchild {pid} outlived the abandoned run"
    );
}

// ── --expect-manifest (#621) ──────────────────────────────────────────────────

#[tokio::test]
async fn a_manifest_replaced_after_describe_is_refused_before_any_request_or_credential() {
    let server = Fixture::start(200, "", userinfo_body());
    let origin = server.origin();
    let confirmed = rest_manifest("svc", "my.api.key", &origin, "account");
    let home = home_with("svc", &confirmed);
    store(home.path(), "my.api.key", Some("gen-1"));

    // Swapped after the caller read it: same origin, a different credential slot.
    let replaced = rest_manifest("svc", "other.api.key", &origin, "account");
    std::fs::write(home.path().join("agents/svc/manifest.yaml"), &replaced).unwrap();

    let mut opts = options(None, Some(&origin));
    opts.expect_manifest = Some(manifest_sha256(confirmed.as_bytes()));
    // A wrong generation too: the manifest pin is checked first, so the
    // credential store is never consulted.
    opts.expect_generation = Some("gen-other".into());
    let failure = probe_agent(home.path(), "svc", &opts)
        .await
        .expect_err("changed");
    assert_eq!(failure.code, "E_PROBE_CHANGED");
    assert!(failure.details.is_empty());
    assert_eq!(failure.exit_code(), 3);
    assert!(server.requests().is_empty(), "no HTTP request was made");

    // Even a manifest that no longer parses is refused as changed, not invalid.
    std::fs::write(home.path().join("agents/svc/manifest.yaml"), "not: [yaml").unwrap();
    let failure = probe_agent(home.path(), "svc", &opts).await.unwrap_err();
    assert_eq!(failure.code, "E_PROBE_CHANGED");
    assert!(server.requests().is_empty());
}

#[tokio::test]
async fn a_matching_manifest_pin_probes_normally() {
    let server = Fixture::start(200, "", userinfo_body());
    let origin = server.origin();
    let home = custom_home(&server);
    store(home.path(), "my.api.key", Some("gen-1"));
    let installed = std::fs::read(home.path().join("agents/svc/manifest.yaml")).unwrap();

    let mut opts = options(None, Some(&origin));
    opts.expect_manifest = Some(manifest_sha256(&installed));
    let receipt = probe_agent(home.path(), "svc", &opts)
        .await
        .expect("matches");
    assert_eq!(receipt.manifest_sha256, manifest_sha256(&installed));
    assert_eq!(server.requests().len(), 1);
}

#[test]
fn the_expect_manifest_value_is_64_lowercase_hex() {
    let good = manifest_sha256(b"x");
    assert_eq!(parse_expected_manifest(&good).as_deref(), Ok(good.as_str()));
    for bad in [
        String::new(),
        good.to_uppercase(),
        good[..63].to_string(),
        format!("{good}0"),
        format!("{}g", &good[..63]),
        format!(" {}", &good[..63]),
    ] {
        assert!(parse_expected_manifest(&bad).is_err(), "{bad:?}");
    }
}

#[test]
fn the_changed_manifest_refusal_has_its_own_fixed_sentence() {
    let failure = ProbeFailure::new("E_PROBE_CHANGED");
    assert_ne!(failure.message(), ProbeFailure::new("E_UNKNOWN").message());
    assert!(failure.message().contains("manifest changed"));
}
