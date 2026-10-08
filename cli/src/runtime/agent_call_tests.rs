//! Unit tests for `runtime::agent_call` (#618). Compiled only under
//! `cfg(test)` via `#[path]`. Every network request goes to a local fixture; the
//! production allowlist is used only where a test proves it refuses.

use super::*;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

const TOKEN: &str = "call-secret-token-618";
const GENERATION: &str = "0b9c3f53-6a3e-4c0f-9d0e-2f3b4a5c6d7e";
const SUB: &str = "109876543210";
const EMAIL: &str = "ada@example.com";

// ── fixture ──────────────────────────────────────────────────────────────────

/// What the fixture answers for one request.
#[derive(Clone)]
struct Reply {
    status: u16,
    headers: String,
    body: Vec<u8>,
    delay: Duration,
}

impl Reply {
    fn json(status: u16, body: &Json) -> Self {
        Self {
            status,
            headers: String::new(),
            body: body.to_string().into_bytes(),
            delay: Duration::ZERO,
        }
    }
}

type Handler = Arc<dyn Fn(&str) -> Reply + Send + Sync>;

/// A local HTTP fixture: answers each request through `handler` (given the raw
/// request head) and records every request head it received.
struct Fixture {
    port: u16,
    seen: Arc<Mutex<Vec<String>>>,
}

impl Fixture {
    fn start(handler: Handler) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let seen = Arc::new(Mutex::new(Vec::new()));
        let record = seen.clone();
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(mut stream) = stream else { break };
                let handler = handler.clone();
                let record = record.clone();
                std::thread::spawn(move || {
                    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
                    let mut data = Vec::new();
                    let mut chunk = [0u8; 1024];
                    while !data.windows(4).any(|w| w == b"\r\n\r\n") {
                        match stream.read(&mut chunk) {
                            Ok(0) | Err(_) => break,
                            Ok(n) => data.extend_from_slice(&chunk[..n]),
                        }
                    }
                    let head = String::from_utf8_lossy(&data).into_owned();
                    record.lock().unwrap().push(head.clone());
                    let reply = handler(&head);
                    std::thread::sleep(reply.delay);
                    let out = format!(
                        "HTTP/1.1 {} X\r\nContent-Type: application/json\r\nContent-Length: {}\r\n{}Connection: close\r\n\r\n",
                        reply.status,
                        reply.body.len(),
                        reply.headers
                    );
                    let _ = stream.write_all(out.as_bytes());
                    let _ = stream.write_all(&reply.body);
                });
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

    fn count(&self, path: &str) -> usize {
        self.requests()
            .iter()
            .filter(|head| {
                head.split_whitespace()
                    .nth(1)
                    .is_some_and(|p| p.starts_with(path))
            })
            .count()
    }
}

fn userinfo(sub: &str) -> Json {
    json!({"sub": sub, "email": EMAIL, "email_verified": true, "name": "must-not-leave"})
}

fn files_body() -> Json {
    json!({
        "files": [
            {"id": "f1", "name": "Plan.pdf", "mimeType": "application/pdf", "size": "1234",
             "modifiedTime": "2026-10-01T10:00:00.000Z", "owners": [{"emailAddress": "x@y"}]},
            {"id": "f2", "name": "Notes", "mimeType": "application/vnd.google-apps.document"}
        ],
        "kind": "drive#fileList"
    })
}

/// The standard Google fixture: userinfo answers `sub`, Drive answers `files`.
fn google(sub: &'static str, files: Reply) -> Fixture {
    Fixture::start(Arc::new(move |head: &str| {
        if head.contains(" /v1/userinfo") {
            Reply::json(200, &userinfo(sub))
        } else {
            files.clone()
        }
    }))
}

fn endpoints(f: &Fixture) -> Endpoints {
    Endpoints {
        token_url: GOOGLE_TOKEN_URL.to_string(),
        identity_url: format!("{}/v1/userinfo", f.origin()),
        files_url: Some(format!("{}/drive/v3/files", f.origin())),
        allowed_origins: vec![f.origin()],
    }
}

// ── homes, tokens, requests ──────────────────────────────────────────────────

fn shipped_manifest() -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../20-agents/aeco/cross-cutting/google-workspace/manifest.yaml");
    // Normalised: a Windows checkout may hold CRLF.
    std::fs::read_to_string(path).unwrap().replace("\r\n", "\n")
}

fn home_with_manifest(manifest: &str) -> tempfile::TempDir {
    let home = tempfile::tempdir().unwrap();
    let dir = home.path().join("agents").join("google-workspace");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("manifest.yaml"), manifest).unwrap();
    home
}

fn home() -> tempfile::TempDir {
    home_with_manifest(&shipped_manifest())
}

fn mail_and_drive() -> String {
    format!(
        "openid https://www.googleapis.com/auth/userinfo.email https://www.googleapis.com/auth/gmail.send {DRIVE_METADATA_READONLY}"
    )
}

fn token(scope: &str, generation: Option<&str>, access: &str) -> StoredToken {
    let now = crate::auth::unix_now_secs().unwrap();
    StoredToken {
        access_token: access.into(),
        refresh_token: None,
        expires_at: now + 3600,
        scope: scope.into(),
        token_type: "Bearer".into(),
        integration: "google-workspace".into(),
        obtained_at: now,
        generation: generation.map(str::to_string),
        source: crate::auth::keychain::TokenSource::Oauth,
    }
}

fn store(home: &Path, alias: Option<&str>, scope: &str) {
    crate::auth::keychain::store_token(&token(scope, Some(GENERATION), TOKEN), alias, home)
        .unwrap();
}

fn op() -> &'static Operation {
    operation("google-workspace", "list-files").unwrap()
}

fn installed_sha(home: &Path) -> String {
    crate::runtime::probe::manifest_sha256(
        &std::fs::read(home.join("agents/google-workspace/manifest.yaml")).unwrap(),
    )
}

/// A valid request for the default slot, as FloLess would write it.
fn request(home: &Path) -> Map<String, Json> {
    let inputs = json!({"page-size": 5, "query": "trashed=false"});
    let digest = sha256_hex(canonical_string(&inputs).as_bytes());
    let Json::Object(map) = json!({
        "schema": REQUEST_SCHEMA,
        "invocationId": "6f1c2b3a-4d5e-5f60-8a1b-2c3d4e5f6a7b",
        "agent": "google-workspace",
        "command": "list-files",
        "expectedAgentVersion": "2.2.0",
        "expectedManifestSha256": installed_sha(home),
        "executionTransport": "rest",
        "operationSchemaVersion": 1,
        "expectedOperationSha256": op().sha256(),
        "integration": "google-workspace",
        "alias": null,
        "bindingId": binding_id("google-workspace", "google-workspace", SUB),
        "bindingRevision": 1,
        "credentialGeneration": GENERATION,
        "inputs": inputs,
        "inputsSha256": digest,
        "owner": {"rootId": "r", "rootGeneration": 1, "scopeKind": "workspace", "scopeId": "s",
                  "conversationId": "c", "turnId": "t", "ownershipGeneration": "g"},
        "connectionRevision": "rev-1",
        "approvalReceiptId": "6f1c2b3a-4d5e-5f60-8a1b-2c3d4e5f6a7c",
        "timeoutMs": 10000,
        "maxOutputBytes": 262144
    }) else {
        unreachable!()
    };
    map
}

fn parsed(map: &Map<String, Json>) -> CallRequest {
    parse_request(Json::Object(map.clone()).to_string().as_bytes()).unwrap()
}

fn set_inputs(map: &mut Map<String, Json>, inputs: Json) {
    let digest = sha256_hex(canonical_string(&inputs).as_bytes());
    map.insert("inputs".into(), inputs);
    map.insert("inputsSha256".into(), json!(digest));
}

async fn caps(home: &Path, alias: Option<&str>, e: &Endpoints) -> Json {
    capabilities_with(home, op(), alias, Duration::from_secs(10), e)
        .await
        .unwrap()
}

/// Assert nothing a failure or a receipt carries holds the token or service text.
fn assert_clean(text: &str) {
    assert!(!text.contains(TOKEN), "token leaked: {text}");
    assert!(
        !text.contains("must-not-leave"),
        "service text leaked: {text}"
    );
}

// ── digests, ids ─────────────────────────────────────────────────────────────

#[test]
fn input_digests_match_floless_canonical_json() {
    // Generated with FloLess's own algorithm (sort keys with localeCompare,
    // JSON.stringify, SHA-256) in node.
    let cases = [
        (
            json!({}),
            "44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a",
        ),
        (
            json!({"page-size": 100}),
            "b966154588879aef6f7a98dcd39a512a7b80e917d52b8f7505005c576a436118",
        ),
        (
            json!({"query": "name contains 'a\u{0001}\"é'", "page-size": 5}),
            "830becfa8dd52bc1d77c738d2c1bb6ac88ef8a09976a61b2cf8df6b05724ac41",
        ),
    ];
    for (inputs, expected) in cases {
        assert_eq!(
            sha256_hex(canonical_string(&inputs).as_bytes()),
            expected,
            "{inputs}"
        );
    }
}

#[test]
fn binding_ids_are_floless_shaped_uuids_tied_to_slot_and_account() {
    let id = binding_id("google-workspace", "google-workspace", SUB);
    // FloLess's UUID regex: version nibble 1-8, RFC 4122 variant.
    let b = id.as_bytes();
    assert_eq!(id.len(), 36);
    assert_eq!(b[14], b'8');
    assert!(matches!(b[19], b'8' | b'9' | b'a' | b'b'));
    assert!(is_uuid(&id));
    assert_eq!(id, binding_id("google-workspace", "google-workspace", SUB));
    assert_ne!(
        id,
        binding_id("google-workspace", "google-workspace", "other")
    );
    assert_ne!(
        id,
        binding_id("google-workspace", "google-workspace.team", SUB)
    );
}

#[test]
fn the_operation_digest_is_stable_and_covers_the_request_shape() {
    let digest = op().sha256();
    assert!(is_sha256_hex(&digest));
    let canonical = canonical_string(&op().canonical());
    for needle in [
        "nextPageToken",
        "pageSize",
        DRIVE_READONLY,
        DRIVE_METADATA_READONLY,
        GOOGLE_USERINFO_URL,
    ] {
        assert!(canonical.contains(needle), "{needle}");
    }
}

#[test]
fn only_list_files_is_a_reviewed_operation() {
    assert!(operation("google-workspace", "list-files").is_some());
    assert!(operation("google-workspace", "gmail.send").is_none());
    assert!(operation("microsoft-365", "list-folder").is_none());
    assert!(handles_workflow("google-workspace", "list-files"));
    assert!(!handles_workflow("google-workspace", "download-file"));
}

#[test]
fn the_production_allowlist_is_drive_and_openid_only() {
    let e = Endpoints::production("google-workspace");
    let mut allowed = e.allowed_origins.clone();
    allowed.sort();
    assert_eq!(
        allowed,
        [
            "https://openidconnect.googleapis.com",
            "https://www.googleapis.com"
        ]
    );
    assert!(e.check_origins(op()).is_ok());
    assert!(
        Endpoints::production("microsoft-365")
            .allowed_origins
            .is_empty()
    );
}

// ── projection ───────────────────────────────────────────────────────────────

#[test]
fn the_projection_keeps_only_allowlisted_bounded_fields() {
    let body = json!({
        "files": [
            {"id": "a", "name": "x\u{0007}y", "mimeType": "m", "size": "42", "modifiedTime": "t",
             "webContentLink": "secret-link", "owners": []},
            {"name": "no id is dropped"},
            {"id": "native", "name": "Doc", "mimeType": "application/vnd.google-apps.document"},
            {"id": "huge", "size": "9007199254740993"},
            {"id": "long", "name": "n".repeat(2000)}
        ],
        "nextPageToken": "next",
        "incompleteSearch": true,
    });
    let out = project_files(&body, 100).unwrap();
    let text = out.to_string();
    assert!(!text.contains("secret-link") && !text.contains("owners"));
    let files = out["files"].as_array().unwrap();
    assert_eq!(files.len(), 4);
    assert_eq!(
        files[0],
        json!({"id": "a", "name": "xy", "mime-type": "m", "size": 42, "modified-time": "t"})
    );
    // A Google-native doc has no size or modifiedTime: still listed.
    assert_eq!(
        files[1],
        json!({"id": "native", "name": "Doc", "mime-type": "application/vnd.google-apps.document"})
    );
    assert!(files[2].get("size").is_none(), "over 2^53 is omitted");
    assert!(
        files[3].get("name").is_none(),
        "an over-long name is omitted"
    );
    assert_eq!(out["more-available"], json!(true));
    assert_eq!(out["incomplete-search"], json!(true));
}

#[test]
fn a_short_page_with_a_continuation_is_more_available_and_the_limit_holds() {
    let body = json!({"files": [{"id": "1"}, {"id": "2"}, {"id": "3"}], "nextPageToken": "t"});
    let out = project_files(&body, 2).unwrap();
    assert_eq!(out["files"].as_array().unwrap().len(), 2);
    assert_eq!(out["more-available"], json!(true));
    let last = project_files(&json!({"files": []}), 5).unwrap();
    assert_eq!(last["more-available"], json!(false));
    assert!(project_files(&json!({"nofiles": 1}), 5).is_none());
    assert!(project_files(&json!([]), 5).is_none());
}

// ── request grammar ──────────────────────────────────────────────────────────

#[test]
fn the_request_grammar_refuses_each_bad_field() {
    let h = home();
    let good = request(h.path());
    assert!(parse_request(Json::Object(good.clone()).to_string().as_bytes()).is_ok());

    let mut unknown = good.clone();
    unknown.insert("authorization".into(), json!("Bearer x"));
    assert_eq!(
        parse_request(Json::Object(unknown).to_string().as_bytes()).unwrap_err(),
        CallFailure::request_reason("unknown-field")
    );
    let bad: &[(&str, Json)] = &[
        ("schema", json!("aware.agent-call/v2")),
        ("invocationId", json!("not-a-uuid")),
        ("agent", json!("../x")),
        ("expectedManifestSha256", json!("ABC")),
        ("expectedOperationSha256", json!("0".repeat(63))),
        ("inputsSha256", json!(null)),
        ("executionTransport", json!("cli")),
        ("operationSchemaVersion", json!(2)),
        ("alias", json!("-team")),
        ("alias", json!("Team")),
        ("bindingId", json!("x")),
        ("bindingRevision", json!(0)),
        ("credentialGeneration", json!("a b")),
        ("inputs", json!({"query": {"nested": 1}})),
        ("inputs", json!({"page-size": 1.5})),
        ("owner", json!("x")),
        ("owner", json!({"pad": "x".repeat(5000)})),
        ("connectionRevision", json!("")),
        ("approvalReceiptId", json!(1)),
        ("timeoutMs", json!(999)),
        ("timeoutMs", json!(60_001)),
        ("maxOutputBytes", json!(1_048_577)),
    ];
    for (field, value) in bad {
        let mut map = good.clone();
        map.insert((*field).into(), value.clone());
        let failure = parse_request(Json::Object(map).to_string().as_bytes()).unwrap_err();
        assert_eq!(failure, CallFailure::field(field), "{field} = {value}");
    }
    for field in ["agent", "bindingId", "owner", "timeoutMs"] {
        let mut map = good.clone();
        map.remove(field);
        assert!(
            parse_request(Json::Object(map).to_string().as_bytes()).is_err(),
            "{field}"
        );
    }
    assert_eq!(
        parse_request(b"[1]").unwrap_err(),
        CallFailure::request_reason("not-an-object")
    );
    assert_eq!(
        parse_request(b"{").unwrap_err(),
        CallFailure::request_reason("not-json")
    );
}

// ── call-capabilities ────────────────────────────────────────────────────────

#[tokio::test]
async fn capabilities_report_a_verified_binding_for_the_default_slot() {
    let h = home();
    store(h.path(), None, &mail_and_drive());
    let f = google(SUB, Reply::json(200, &files_body()));
    let cap = caps(h.path(), None, &endpoints(&f)).await;
    assert_eq!(cap["schema"], json!(CAPABILITY_SCHEMA));
    assert_eq!(cap["agent"], json!("google-workspace"));
    assert_eq!(cap["command"], json!("list-files"));
    assert_eq!(cap["installedVersion"], json!("2.2.0"));
    assert_eq!(cap["manifestSha256"], json!(installed_sha(h.path())));
    assert_eq!(cap["operationSha256"], json!(op().sha256()));
    assert_eq!(cap["transport"], json!("rest"));
    assert_eq!(cap["effect"], json!("read"));
    assert_eq!(cap["alias"], json!(null));
    assert_eq!(cap["available"], json!(true));
    let c = &cap["credential"];
    assert_eq!(c["status"], json!("verified"));
    assert_eq!(
        c["binding_id"],
        json!(binding_id("google-workspace", "google-workspace", SUB))
    );
    assert_eq!(c["revision"], json!(1));
    assert_eq!(c["credential_generation"], json!(GENERATION));
    assert_eq!(
        c["principal"],
        json!({"integration": "google-workspace", "stable_id": SUB})
    );
    assert_eq!(c["presentation_label"], json!(EMAIL));
    assert!(c["verified_at"].as_i64().unwrap() > 0);
    assert_eq!(cap["inputs"].as_array().unwrap().len(), 2);
    assert_clean(&cap.to_string());
    // Exactly one identity read with the token; Drive is never touched.
    assert_eq!(f.count("/v1/userinfo"), 1);
    assert_eq!(f.count("/drive"), 0);
    assert!(f.requests()[0].contains(&format!("Bearer {TOKEN}")));
}

#[tokio::test]
async fn an_empty_alias_slot_is_missing_and_never_falls_back_to_the_default() {
    let h = home();
    store(h.path(), None, &mail_and_drive());
    let f = google(SUB, Reply::json(200, &files_body()));
    let cap = caps(h.path(), Some("team"), &endpoints(&f)).await;
    assert_eq!(
        cap["credential"],
        json!({"status": "missing", "integration": "google-workspace", "alias": "team"})
    );
    assert_eq!(cap["alias"], json!("team"));
    assert_eq!(cap["available"], json!(false));
    assert_eq!(cap["unavailableCode"], json!("credential-missing"));
    assert!(
        f.requests().is_empty(),
        "the default slot's token must not be used"
    );
}

#[tokio::test]
async fn an_aliased_slot_resolves_exactly_that_slot() {
    let h = home();
    // Different tokens in the two slots; only the alias's may be sent.
    crate::auth::keychain::store_token(
        &token(&mail_and_drive(), Some(GENERATION), "default-slot-token"),
        None,
        h.path(),
    )
    .unwrap();
    store(h.path(), Some("team"), &mail_and_drive());
    let f = google(SUB, Reply::json(200, &files_body()));
    let cap = caps(h.path(), Some("team"), &endpoints(&f)).await;
    assert_eq!(cap["credential"]["status"], json!("verified"));
    assert_eq!(
        cap["credential"]["binding_id"],
        json!(binding_id("google-workspace", "google-workspace.team", SUB))
    );
    let seen = f.requests().join("\n");
    assert!(seen.contains(TOKEN) && !seen.contains("default-slot-token"));
}

#[tokio::test]
async fn a_slot_without_the_drive_scope_is_scope_missing_with_the_exact_reconnect() {
    let h = home();
    let mail = "openid https://www.googleapis.com/auth/userinfo.email https://www.googleapis.com/auth/gmail.send";
    store(h.path(), Some("team"), mail);
    let f = google(SUB, Reply::json(200, &files_body()));
    let cap = caps(h.path(), Some("team"), &endpoints(&f)).await;
    assert_eq!(cap["credential"]["status"], json!("verified"));
    assert_eq!(cap["available"], json!(false));
    assert_eq!(cap["unavailableCode"], json!("scope-missing"));
    let reason = cap["unavailableReason"].as_str().unwrap();
    assert!(reason.contains("--as=team"), "{reason}");
    assert!(reason.contains(DRIVE_READONLY), "{reason}");
    // Full `drive` is not on the allowlist (gmail.send would refuse it too).
    let h2 = home();
    store(
        h2.path(),
        None,
        &format!("{mail} https://www.googleapis.com/auth/drive"),
    );
    let cap = caps(h2.path(), None, &endpoints(&f)).await;
    assert_eq!(cap["unavailableCode"], json!("scope-missing"));
}

#[tokio::test]
async fn a_failed_identity_read_is_unverified_with_revision_zero() {
    let h = home();
    store(h.path(), None, &mail_and_drive());
    let f = Fixture::start(Arc::new(|_: &str| {
        Reply::json(401, &json!({"error": TOKEN}))
    }));
    let cap = caps(h.path(), None, &endpoints(&f)).await;
    assert_eq!(
        cap["credential"],
        json!({"status": "unverified", "integration": "google-workspace", "alias": null, "revision": 0})
    );
    assert_eq!(cap["unavailableCode"], json!("identity-unverified"));
    assert_clean(&cap.to_string());
}

#[tokio::test]
async fn a_manifest_with_another_credential_is_auth_invalid() {
    let manifest = shipped_manifest().replace("secret: google-workspace", "secret: my.raw.handle");
    let h = home_with_manifest(&manifest);
    store(h.path(), None, &mail_and_drive());
    let f = google(SUB, Reply::json(200, &files_body()));
    let cap = caps(h.path(), None, &endpoints(&f)).await;
    assert_eq!(cap["available"], json!(false));
    assert_eq!(cap["unavailableCode"], json!("auth-invalid"));
}

#[tokio::test]
async fn a_planned_list_files_is_command_planned() {
    let manifest = shipped_manifest().replace(
        "  list-files:\n    lifecycle: single",
        "  list-files:\n    status: planned\n    lifecycle: single",
    );
    assert_ne!(manifest, shipped_manifest(), "the fixture edit must apply");
    let h = home_with_manifest(&manifest);
    store(h.path(), None, &mail_and_drive());
    let f = google(SUB, Reply::json(200, &files_body()));
    let cap = caps(h.path(), None, &endpoints(&f)).await;
    assert_eq!(cap["available"], json!(false));
    assert_eq!(cap["unavailableCode"], json!("command-planned"));
}

#[tokio::test]
async fn capabilities_hard_errors_only_when_there_is_nothing_to_describe() {
    let empty = tempfile::tempdir().unwrap();
    let t = Duration::from_secs(5);
    assert_eq!(
        capabilities(empty.path(), "google-workspace", "list-files", None, t)
            .await
            .unwrap_err()
            .code,
        "E_AGENT_NOT_INSTALLED"
    );
    assert_eq!(
        capabilities(empty.path(), "microsoft-365", "list-folder", None, t)
            .await
            .unwrap_err()
            .code,
        "E_CALL_UNSUPPORTED"
    );
    let h = home();
    assert_eq!(
        capabilities(h.path(), "google-workspace", "list-files", Some("-x"), t)
            .await
            .unwrap_err()
            .code,
        "E_CALL_ALIAS_INVALID"
    );
}

#[tokio::test]
async fn a_token_endpoint_override_is_never_sent_the_refresh_token() {
    let h = home();
    // An expired token with a refresh token: a refresh would be attempted.
    let mut t = token(&mail_and_drive(), Some(GENERATION), TOKEN);
    t.expires_at = 1;
    t.refresh_token = Some("refresh-secret".into());
    crate::auth::keychain::store_token(&t, None, h.path()).unwrap();
    let sink = Fixture::start(Arc::new(|_: &str| {
        Reply::json(200, &json!({"access_token": "x"}))
    }));
    std::fs::create_dir_all(h.path().join("oauth")).unwrap();
    std::fs::write(
        h.path().join("oauth/google-workspace.yaml"),
        format!("token_url: {}/token\n", sink.origin()),
    )
    .unwrap();
    let f = google(SUB, Reply::json(200, &files_body()));
    let cap = caps(h.path(), None, &endpoints(&f)).await;
    assert_eq!(cap["credential"]["status"], json!("unverified"));
    assert_eq!(cap["unavailableCode"], json!("credential-expired"));
    let mut req = request(h.path());
    req.insert("credentialGeneration".into(), json!(GENERATION));
    let failure = call_with(h.path(), &parsed(&req), &endpoints(&f))
        .await
        .unwrap_err();
    assert_eq!(
        failure,
        CallFailure::reason("E_CREDENTIAL_EXPIRED", "token-endpoint")
    );
    assert!(
        sink.requests().is_empty(),
        "the refresh token went to the override"
    );
    assert!(f.requests().is_empty());
}

/// A refresh token endpoint that, like a second AWARE process, stores its own
/// fresh token (same grant) into the slot before answering — so our
/// compare-and-store loses.
#[tokio::test]
async fn a_concurrent_refresh_of_the_same_grant_is_used_not_refused() {
    let h = home();
    let mut expired = token(&mail_and_drive(), Some(GENERATION), "stale-token");
    expired.expires_at = 1;
    expired.refresh_token = Some("refresh-secret".into());
    crate::auth::keychain::store_token(&expired, None, h.path()).unwrap();
    let home_path = h.path().to_path_buf();
    let token_endpoint = Fixture::start(Arc::new(move |_: &str| {
        crate::auth::keychain::store_token(
            &token(&mail_and_drive(), Some(GENERATION), TOKEN),
            None,
            &home_path,
        )
        .unwrap();
        Reply::json(
            200,
            &json!({"access_token": "loser-token", "expires_in": 3600}),
        )
    }));
    std::fs::create_dir_all(h.path().join("oauth")).unwrap();
    let token_url = format!("{}/token", token_endpoint.origin());
    std::fs::write(
        h.path().join("oauth/google-workspace.yaml"),
        format!("token_url: {token_url}\n"),
    )
    .unwrap();
    let f = google(SUB, Reply::json(200, &files_body()));
    let mut e = endpoints(&f);
    e.token_url = token_url;
    let cap = caps(h.path(), None, &e).await;
    assert_eq!(
        token_endpoint.requests().len(),
        1,
        "a refresh was attempted"
    );
    assert_eq!(cap["credential"]["status"], json!("verified"), "{cap}");
    // The winner's token is the one that went to Google.
    assert!(f.requests()[0].contains(&format!("Bearer {TOKEN}")));
}

#[tokio::test]
async fn two_concurrent_capability_reads_of_one_slot_both_verify() {
    let h = home();
    store(h.path(), None, &mail_and_drive());
    let f = google(SUB, Reply::json(200, &files_body()));
    let e = endpoints(&f);
    let (a, b) = tokio::join!(caps(h.path(), None, &e), caps(h.path(), None, &e));
    assert_eq!(a["credential"]["status"], json!("verified"));
    assert_eq!(b["credential"]["status"], json!("verified"));
}

#[tokio::test]
async fn a_capability_read_never_outlives_its_budget() {
    let h = home();
    store(h.path(), None, &mail_and_drive());
    let f = Fixture::start(Arc::new(|_: &str| Reply {
        delay: Duration::from_secs(20),
        ..Reply::json(200, &userinfo(SUB))
    }));
    // A stalled identity read is bounded below the verb's budget, so the
    // capability still answers — as `unverified` — inside the budget.
    let started = Instant::now();
    let cap = capabilities_with(h.path(), op(), None, Duration::from_secs(2), &endpoints(&f))
        .await
        .unwrap();
    assert!(
        started.elapsed() < Duration::from_secs(2),
        "{:?}",
        started.elapsed()
    );
    assert_eq!(cap["credential"]["status"], json!("unverified"));
    assert_eq!(cap["unavailableCode"], json!("identity-unverified"));
}

/// A token endpoint that never answers: the refresh has its own fixed 30 s
/// deadline, so the verb's budget is what bounds the reply — a hard
/// `E_CALL_TIMEOUT` (FloLess renders a failed capability call as the slot's
/// unavailable reason), never a reply after the budget.
#[tokio::test]
async fn a_stalled_token_endpoint_cannot_hold_a_capability_past_its_budget() {
    let h = home();
    let mut expired = token(&mail_and_drive(), Some(GENERATION), TOKEN);
    expired.expires_at = 1;
    expired.refresh_token = Some("refresh-secret".into());
    crate::auth::keychain::store_token(&expired, None, h.path()).unwrap();
    let stall = Fixture::start(Arc::new(|_: &str| Reply {
        delay: Duration::from_secs(40),
        ..Reply::json(200, &json!({}))
    }));
    let token_url = format!("{}/token", stall.origin());
    std::fs::create_dir_all(h.path().join("oauth")).unwrap();
    std::fs::write(
        h.path().join("oauth/google-workspace.yaml"),
        format!("token_url: {token_url}\n"),
    )
    .unwrap();
    let f = google(SUB, Reply::json(200, &files_body()));
    let mut e = endpoints(&f);
    e.token_url = token_url;
    let started = Instant::now();
    let failure = capabilities_with(h.path(), op(), None, Duration::from_millis(1500), &e)
        .await
        .unwrap_err();
    assert_eq!(failure.code, "E_CALL_TIMEOUT");
    assert!(
        started.elapsed() < Duration::from_millis(2500),
        "{:?}",
        started.elapsed()
    );
    assert!(f.requests().is_empty());
}

// ── call ─────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn a_bound_call_reads_drive_and_returns_only_the_projection() {
    let h = home();
    store(h.path(), None, &mail_and_drive());
    let f = google(SUB, Reply::json(200, &files_body()));
    let req = request(h.path());
    let record = call_with(h.path(), &parsed(&req), &endpoints(&f))
        .await
        .unwrap();
    assert_eq!(record["schema"], json!(RECORD_SCHEMA));
    assert_eq!(record["state"], json!("completed"));
    assert_eq!(record["invocationId"], req["invocationId"]);
    assert_eq!(record["bindingId"], req["bindingId"]);
    assert_eq!(record["credentialGeneration"], json!(GENERATION));
    assert_eq!(record["httpStatus"], json!(200));
    assert!(is_sha256_hex(record["requestDigest"].as_str().unwrap()));
    assert!(is_sha256_hex(record["correlationSha256"].as_str().unwrap()));
    let result = &record["result"];
    assert_eq!(result["schema"], json!(RESULT_SCHEMA));
    assert_eq!(result["outcome"], json!("ok"));
    assert_eq!(
        result["body"],
        json!({"files": [
            {"id": "f1", "name": "Plan.pdf", "mime-type": "application/pdf", "size": 1234,
             "modified-time": "2026-10-01T10:00:00.000Z"},
            {"id": "f2", "name": "Notes", "mime-type": "application/vnd.google-apps.document"}
        ], "more-available": false, "incomplete-search": false})
    );
    assert_clean(&record.to_string());
    assert!(!record.to_string().contains("emailAddress"));
    // Identity first, then exactly one Drive request with the fixed shape.
    let seen = f.requests();
    assert_eq!(seen.len(), 2);
    assert!(seen[0].contains(" /v1/userinfo"));
    let drive = &seen[1];
    assert!(drive.starts_with("GET /drive/v3/files?"), "{drive}");
    assert!(drive.contains("pageSize=5"));
    assert!(drive.contains("q=trashed%3Dfalse"));
    assert!(drive.contains("fields=nextPageToken%2CincompleteSearch%2Cfiles%28id%2Cname%2CmimeType%2Csize%2CmodifiedTime%29"));
    assert!(drive.contains(&format!("Bearer {TOKEN}")));
}

/// Owner ruling 2026-10-08: AWARE requests `drive.readonly`, which could read
/// content. Pin that a slot holding it still gets exactly one `files.list`
/// request with the fixed metadata field mask — no other path, no other fields.
#[tokio::test]
async fn a_drive_readonly_slot_still_only_lists_metadata() {
    for drive_scope in [DRIVE_READONLY, DRIVE_METADATA_READONLY] {
        let h = home();
        let scope = format!(
            "openid https://www.googleapis.com/auth/userinfo.email https://www.googleapis.com/auth/gmail.send {drive_scope}"
        );
        store(h.path(), None, &scope);
        let f = google(SUB, Reply::json(200, &files_body()));
        call_with(h.path(), &parsed(&request(h.path())), &endpoints(&f))
            .await
            .unwrap();
        let drive: Vec<String> = f
            .requests()
            .into_iter()
            .filter(|head| !head.contains(" /v1/userinfo"))
            .collect();
        assert_eq!(drive.len(), 1, "{drive_scope}");
        let line = drive[0].lines().next().unwrap();
        assert!(line.starts_with("GET /drive/v3/files?"), "{line}");
        assert!(
            !line.contains("alt=media") && !line.contains("/export"),
            "{line}"
        );
        let fields = line
            .split(['?', '&', ' '])
            .find_map(|part| part.strip_prefix("fields="))
            .unwrap();
        assert_eq!(
            fields,
            "nextPageToken%2CincompleteSearch%2Cfiles%28id%2Cname%2CmimeType%2Csize%2CmodifiedTime%29"
        );
    }
    assert_eq!(
        LIST_FILES_FIELDS,
        "nextPageToken,incompleteSearch,files(id,name,mimeType,size,modifiedTime)"
    );
    assert_eq!(op().url, "https://www.googleapis.com/drive/v3/files");
}

#[tokio::test]
async fn a_service_error_carries_only_its_status_never_the_echoed_token() {
    let h = home();
    store(h.path(), None, &mail_and_drive());
    let echo = Reply::json(
        403,
        &json!({"error": {"message": format!("token {TOKEN} must-not-leave")}}),
    );
    let f = google(SUB, echo);
    let failure = call_with(h.path(), &parsed(&request(h.path())), &endpoints(&f))
        .await
        .unwrap_err();
    assert_eq!(failure.code, "E_CALL_FAILED");
    assert_eq!(
        failure.details,
        Map::from_iter([("status".to_string(), json!(403))])
    );
    assert_clean(&format!("{failure:?} {}", failure.message()));
    assert_eq!(failure.exit_code(), 4);
}

#[tokio::test]
async fn a_changed_generation_is_refused_before_any_request() {
    let h = home();
    store(h.path(), None, &mail_and_drive());
    let f = google(SUB, Reply::json(200, &files_body()));
    let mut req = request(h.path());
    req.insert(
        "credentialGeneration".into(),
        json!("11111111-2222-4333-8444-555555555555"),
    );
    let failure = call_with(h.path(), &parsed(&req), &endpoints(&f))
        .await
        .unwrap_err();
    assert_eq!(failure, CallFailure::new("E_CREDENTIAL_CHANGED"));
    assert!(f.requests().is_empty());
}

#[tokio::test]
async fn another_google_account_in_the_slot_is_refused_before_drive() {
    let h = home();
    store(h.path(), None, &mail_and_drive());
    // The slot now answers for a different Google account.
    let f = google("someone-else", Reply::json(200, &files_body()));
    let failure = call_with(h.path(), &parsed(&request(h.path())), &endpoints(&f))
        .await
        .unwrap_err();
    assert_eq!(failure, CallFailure::new("E_BINDING_CHANGED"));
    assert_eq!(f.count("/drive"), 0);
    let mut req = request(h.path());
    req.insert("bindingRevision".into(), json!(2));
    let f2 = google(SUB, Reply::json(200, &files_body()));
    assert_eq!(
        call_with(h.path(), &parsed(&req), &endpoints(&f2))
            .await
            .unwrap_err(),
        CallFailure::new("E_BINDING_CHANGED")
    );
}

#[tokio::test]
async fn a_slot_replaced_after_the_identity_read_is_refused_before_drive() {
    let h = home();
    store(h.path(), None, &mail_and_drive());
    let home_path = h.path().to_path_buf();
    let f = Fixture::start(Arc::new(move |head: &str| {
        if head.contains(" /v1/userinfo") {
            // A reconnect lands between the identity read and the Drive read.
            crate::auth::keychain::store_token(
                &token(
                    &mail_and_drive(),
                    Some("99999999-2222-4333-8444-555555555555"),
                    "new",
                ),
                None,
                &home_path,
            )
            .unwrap();
            Reply::json(200, &userinfo(SUB))
        } else {
            Reply::json(200, &files_body())
        }
    }));
    let failure = call_with(h.path(), &parsed(&request(h.path())), &endpoints(&f))
        .await
        .unwrap_err();
    assert_eq!(
        failure,
        CallFailure::reason("E_CREDENTIAL_CHANGED", "replaced")
    );
    assert_eq!(f.count("/drive"), 0);
}

#[tokio::test]
async fn a_disconnected_slot_after_the_identity_read_is_refused_before_drive() {
    let h = home();
    store(h.path(), None, &mail_and_drive());
    let home_path = h.path().to_path_buf();
    let f = Fixture::start(Arc::new(move |head: &str| {
        if head.contains(" /v1/userinfo") {
            let _ = crate::auth::keychain::delete_token("google-workspace", None, &home_path);
            Reply::json(200, &userinfo(SUB))
        } else {
            Reply::json(200, &files_body())
        }
    }));
    let failure = call_with(h.path(), &parsed(&request(h.path())), &endpoints(&f))
        .await
        .unwrap_err();
    assert_eq!(
        failure,
        CallFailure::reason("E_CREDENTIAL_CHANGED", "replaced")
    );
    assert_eq!(f.count("/drive"), 0);
}

#[tokio::test]
async fn the_production_allowlist_refuses_a_foreign_origin_before_any_credential() {
    let h = home();
    store(h.path(), None, &mail_and_drive());
    let f = google(SUB, Reply::json(200, &files_body()));
    let mut e = endpoints(&f);
    e.allowed_origins = Endpoints::production("google-workspace").allowed_origins;
    let failure = call_with(h.path(), &parsed(&request(h.path())), &e)
        .await
        .unwrap_err();
    assert_eq!(failure.code, "E_CALL_ORIGIN_NOT_ALLOWED");
    assert!(f.requests().is_empty());
    let cap = capabilities_with(h.path(), op(), None, Duration::from_secs(5), &e).await;
    assert_eq!(cap.unwrap_err().code, "E_CALL_ORIGIN_NOT_ALLOWED");
    assert!(f.requests().is_empty());
}

#[tokio::test]
async fn a_redirect_is_never_followed_with_the_token() {
    let h = home();
    store(h.path(), None, &mail_and_drive());
    let elsewhere = Fixture::start(Arc::new(|_: &str| Reply::json(200, &files_body())));
    let location = format!("Location: {}/drive/v3/files\r\n", elsewhere.origin());
    let redirect = Reply {
        headers: location,
        ..Reply::json(302, &json!({}))
    };
    let f = google(SUB, redirect);
    let failure = call_with(h.path(), &parsed(&request(h.path())), &endpoints(&f))
        .await
        .unwrap_err();
    assert_eq!(failure.code, "E_CALL_FAILED");
    assert_eq!(failure.details["status"], json!(302));
    assert!(elsewhere.requests().is_empty());
}

#[tokio::test]
async fn a_response_over_the_cap_is_refused() {
    let h = home();
    store(h.path(), None, &mail_and_drive());
    let big = json!({"files": [{"id": "x", "name": "n".repeat(5000)}]});
    let f = google(SUB, Reply::json(200, &big));
    let mut req = request(h.path());
    req.insert("maxOutputBytes".into(), json!(1024));
    let failure = call_with(h.path(), &parsed(&req), &endpoints(&f))
        .await
        .unwrap_err();
    assert_eq!(failure, CallFailure::new("E_CALL_OUTPUT_TOO_LARGE"));
}

#[tokio::test]
async fn a_drive_that_never_answers_times_out_within_the_deadline() {
    let h = home();
    store(h.path(), None, &mail_and_drive());
    let stall = Reply {
        delay: Duration::from_secs(30),
        ..Reply::json(200, &files_body())
    };
    let f = google(SUB, stall);
    let mut req = request(h.path());
    req.insert("timeoutMs".into(), json!(1500));
    let started = Instant::now();
    let failure = call_with(h.path(), &parsed(&req), &endpoints(&f))
        .await
        .unwrap_err();
    assert_eq!(failure.code, "E_CALL_TIMEOUT");
    assert!(
        started.elapsed() < Duration::from_millis(2500),
        "{:?}",
        started.elapsed()
    );
}

/// A budget already spent when the blocking phase starts: no credential is
/// read and no request of any kind is made.
#[tokio::test]
async fn a_spent_budget_starts_no_phase() {
    let h = home();
    store(h.path(), None, &mail_and_drive());
    let f = google(SUB, Reply::json(200, &files_body()));
    let req = parsed(&request(h.path()));
    let long_ago = Instant::now().checked_sub(Duration::from_secs(30)).unwrap();
    let failure = call_within(h.path(), &req, &endpoints(&f), long_ago, 0)
        .await
        .unwrap_err();
    assert_eq!(failure, CallFailure::new("E_CALL_TIMEOUT"));
    assert!(f.requests().is_empty());
    assert!(remaining(Duration::from_secs(1), long_ago).is_err());
    assert!(remaining(Duration::from_secs(10), Instant::now()).is_ok());
}

/// An identity read slower than the budget is cut off by its own deadline and
/// no Drive request follows.
#[tokio::test]
async fn a_slow_identity_read_never_leads_to_a_drive_request() {
    let h = home();
    store(h.path(), None, &mail_and_drive());
    let f = Fixture::start(Arc::new(|head: &str| {
        if head.contains(" /v1/userinfo") {
            Reply {
                delay: Duration::from_millis(1400),
                ..Reply::json(200, &userinfo(SUB))
            }
        } else {
            Reply::json(200, &files_body())
        }
    }));
    let mut req = request(h.path());
    req.insert("timeoutMs".into(), json!(1200));
    let failure = call_with(h.path(), &parsed(&req), &endpoints(&f))
        .await
        .unwrap_err();
    assert!(
        matches!(failure.code, "E_CALL_TIMEOUT" | "E_CALL_IDENTITY_FAILED"),
        "{failure:?}"
    );
    // Give a stray request time to arrive, then prove none did.
    std::thread::sleep(Duration::from_millis(600));
    assert_eq!(f.count("/drive"), 0);
}

#[tokio::test]
async fn a_missing_drive_scope_is_refused_before_any_request() {
    let h = home();
    store(
        h.path(),
        None,
        "openid https://www.googleapis.com/auth/userinfo.email https://www.googleapis.com/auth/gmail.send",
    );
    let f = google(SUB, Reply::json(200, &files_body()));
    let failure = call_with(h.path(), &parsed(&request(h.path())), &endpoints(&f))
        .await
        .unwrap_err();
    assert_eq!(failure, CallFailure::new("E_CALL_SCOPE_MISSING"));
    assert!(f.requests().is_empty());
}

#[tokio::test]
async fn an_unreviewed_manifest_version_or_operation_is_refused_before_any_request() {
    let h = home();
    store(h.path(), None, &mail_and_drive());
    let f = google(SUB, Reply::json(200, &files_body()));
    let e = endpoints(&f);
    let cases: [(&str, Json, CallFailure); 4] = [
        (
            "expectedManifestSha256",
            json!("a".repeat(64)),
            CallFailure::reason("E_CALL_CHANGED", "manifest"),
        ),
        (
            "expectedAgentVersion",
            json!("2.1.0"),
            CallFailure::reason("E_CALL_CHANGED", "version"),
        ),
        (
            "expectedOperationSha256",
            json!("b".repeat(64)),
            CallFailure::reason("E_CALL_CHANGED", "operation"),
        ),
        (
            "integration",
            json!("microsoft-365"),
            CallFailure::new("E_CALL_UNSUPPORTED"),
        ),
    ];
    for (field, value, expected) in cases {
        let mut req = request(h.path());
        req.insert(field.into(), value);
        assert_eq!(
            call_with(h.path(), &parsed(&req), &e).await.unwrap_err(),
            expected,
            "{field}"
        );
    }
    assert!(f.requests().is_empty());
}

#[tokio::test]
async fn inputs_are_mapped_and_bounded_before_the_digest_is_checked() {
    let h = home();
    store(h.path(), None, &mail_and_drive());
    let f = google(SUB, Reply::json(200, &files_body()));
    let e = endpoints(&f);
    // An unknown key is an input refusal even with a WRONG digest: keys first.
    let mut req = request(h.path());
    req.insert("inputs".into(), json!({"Fields": "*"}));
    assert_eq!(
        call_with(h.path(), &parsed(&req), &e)
            .await
            .unwrap_err()
            .code,
        "E_CALL_INPUT_INVALID"
    );
    for (inputs, input) in [
        (json!({"page-size": 0}), "page-size"),
        (json!({"page-size": 101}), "page-size"),
        (json!({"page-size": "5"}), "page-size"),
        (json!({"query": "x".repeat(2049)}), "query"),
        (json!({"query": "a\u{0000}b"}), "query"),
        (json!({"query": true}), "query"),
    ] {
        let mut req = request(h.path());
        set_inputs(&mut req, inputs.clone());
        assert_eq!(
            call_with(h.path(), &parsed(&req), &e).await.unwrap_err(),
            CallFailure::input(input),
            "{inputs}"
        );
    }
    let mut req = request(h.path());
    req.insert("inputsSha256".into(), json!("c".repeat(64)));
    assert_eq!(
        call_with(h.path(), &parsed(&req), &e).await.unwrap_err(),
        CallFailure::field("inputsSha256")
    );
    assert!(f.requests().is_empty());
    // No inputs at all: the default page size goes out.
    let mut req = request(h.path());
    set_inputs(&mut req, json!({}));
    call_with(h.path(), &parsed(&req), &e).await.unwrap();
    assert!(f.requests()[1].contains("pageSize=100") && !f.requests()[1].contains("q="));
}

#[tokio::test]
async fn an_aliased_call_uses_exactly_that_slot() {
    let h = home();
    store(h.path(), None, &mail_and_drive());
    let f = google(SUB, Reply::json(200, &files_body()));
    let mut req = request(h.path());
    req.insert("alias".into(), json!("team"));
    req.insert(
        "bindingId".into(),
        json!(binding_id("google-workspace", "google-workspace.team", SUB)),
    );
    let failure = call_with(h.path(), &parsed(&req), &endpoints(&f))
        .await
        .unwrap_err();
    assert_eq!(failure, CallFailure::new("E_CREDENTIAL_MISSING"));
    assert!(f.requests().is_empty(), "never the default account");
    store(h.path(), Some("team"), &mail_and_drive());
    call_with(h.path(), &parsed(&req), &endpoints(&f))
        .await
        .unwrap();
    // The default slot's binding does not fit the alias slot.
    let mut wrong = req.clone();
    wrong.insert(
        "bindingId".into(),
        json!(binding_id("google-workspace", "google-workspace", SUB)),
    );
    assert_eq!(
        call_with(h.path(), &parsed(&wrong), &endpoints(&f))
            .await
            .unwrap_err(),
        CallFailure::new("E_BINDING_CHANGED")
    );
}

#[tokio::test]
async fn a_call_changes_nothing_in_the_home_directory() {
    let h = home();
    store(h.path(), None, &mail_and_drive());
    // The resolver's own maintenance (scope normalisation of a stored legacy
    // shape) happens on the first read; it is not the call's write.
    crate::auth::keychain::load_token("google-workspace", None, h.path()).unwrap();
    let before = snapshot(h.path());
    let f = google(SUB, Reply::json(200, &files_body()));
    call_with(h.path(), &parsed(&request(h.path())), &endpoints(&f))
        .await
        .unwrap();
    let _ = caps(h.path(), None, &endpoints(&f)).await;
    let after = snapshot(h.path());
    let changed: Vec<&String> = after
        .iter()
        .filter(|(k, v)| before.get(*k) != Some(*v))
        .map(|(k, _)| k)
        .chain(before.keys().filter(|k| !after.contains_key(*k)))
        .collect();
    assert!(changed.is_empty(), "changed: {changed:?}");
}

fn snapshot(root: &Path) -> std::collections::BTreeMap<String, Vec<u8>> {
    fn walk(root: &Path, dir: &Path, out: &mut std::collections::BTreeMap<String, Vec<u8>>) {
        for entry in std::fs::read_dir(dir).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(root, &path, out);
            } else {
                let rel = path.strip_prefix(root).unwrap().display().to_string();
                // Lock files are created and released by every credential read.
                if !rel.ends_with(".lock") {
                    out.insert(rel, std::fs::read(&path).unwrap());
                }
            }
        }
    }
    let mut out = std::collections::BTreeMap::new();
    walk(root, root, &mut out);
    out
}

// ── workflow path ────────────────────────────────────────────────────────────

fn manifest_with_secret(secret: &str) -> Agent {
    let text = shipped_manifest().replace("secret: google-workspace", &format!("secret: {secret}"));
    serde_yaml::from_str(&text).unwrap()
}

#[tokio::test]
async fn a_workflow_node_honours_a_baked_alias_and_refuses_other_handles() {
    let h = home();
    store(h.path(), Some("team"), &mail_and_drive());
    let f = google(SUB, Reply::json(200, &files_body()));
    let e = endpoints(&f);
    let t = Duration::from_secs(10);
    let out = run_for_workflow_with(
        h.path(),
        &manifest_with_secret("google-workspace.team"),
        "list-files",
        &json!({"page-size": 2}),
        Some(&e),
        t,
        1 << 20,
    )
    .await
    .unwrap();
    assert_eq!(out["files"].as_array().unwrap().len(), 2);
    // No identity read in a workflow (no binding to check).
    assert_eq!(f.count("/v1/userinfo"), 0);
    for secret in ["microsoft-365", "my.raw.handle", "google-workspace.Bad"] {
        let failure = run_for_workflow_with(
            h.path(),
            &manifest_with_secret(secret),
            "list-files",
            &json!({}),
            Some(&e),
            t,
            1 << 20,
        )
        .await
        .unwrap_err();
        assert_eq!(
            failure,
            CallFailure::reason("E_CALL_INVALID", "auth"),
            "{secret}"
        );
    }
    assert_eq!(f.count("/drive"), 1);
}

#[tokio::test]
async fn a_workflow_node_is_bounded_and_refuses_a_planned_command() {
    let h = home();
    store(h.path(), None, &mail_and_drive());
    let stall = google(
        SUB,
        Reply {
            delay: Duration::from_secs(30),
            ..Reply::json(200, &files_body())
        },
    );
    let started = Instant::now();
    let failure = run_for_workflow_with(
        h.path(),
        &manifest_with_secret("google-workspace"),
        "list-files",
        &json!({}),
        Some(&endpoints(&stall)),
        Duration::from_millis(1500),
        1 << 20,
    )
    .await
    .unwrap_err();
    assert_eq!(failure.code, "E_CALL_TIMEOUT");
    assert!(started.elapsed() < Duration::from_millis(2500));

    let big = google(
        SUB,
        Reply::json(200, &json!({"files": [{"id": "x".repeat(4096)}]})),
    );
    let failure = run_for_workflow_with(
        h.path(),
        &manifest_with_secret("google-workspace"),
        "list-files",
        &json!({}),
        Some(&endpoints(&big)),
        Duration::from_secs(10),
        1024,
    )
    .await
    .unwrap_err();
    assert_eq!(failure, CallFailure::new("E_CALL_OUTPUT_TOO_LARGE"));

    let planned: Agent = serde_yaml::from_str(&shipped_manifest().replace(
        "  list-files:\n    lifecycle: single",
        "  list-files:\n    status: planned\n    lifecycle: single",
    ))
    .unwrap();
    let failure = run_for_workflow_with(
        h.path(),
        &planned,
        "list-files",
        &json!({}),
        Some(&endpoints(&big)),
        Duration::from_secs(10),
        1 << 20,
    )
    .await
    .unwrap_err();
    assert_eq!(
        failure,
        CallFailure::reason("E_CALL_UNAVAILABLE", "command-planned")
    );
}

#[test]
fn a_credential_without_a_generation_is_never_bound() {
    let without = token(&mail_and_drive(), None, TOKEN);
    assert_eq!(
        slot_generation(&without).unwrap_err(),
        CallFailure::reason("E_CREDENTIAL_CHANGED", "generation-unavailable")
    );
    let with = token(&mail_and_drive(), Some(GENERATION), TOKEN);
    assert_eq!(slot_generation(&with).unwrap(), GENERATION);
}

#[test]
fn every_code_has_a_fixed_sentence_and_an_exit_status() {
    let codes = [
        ("E_AGENT_NOT_INSTALLED", 7),
        ("E_CALL_UNSUPPORTED", 3),
        ("E_CALL_INVALID", 3),
        ("E_CALL_ALIAS_INVALID", 3),
        ("E_CALL_REQUEST_INVALID", 3),
        ("E_CALL_INPUT_INVALID", 3),
        ("E_CALL_CHANGED", 3),
        ("E_CALL_UNAVAILABLE", 3),
        ("E_CALL_ORIGIN_NOT_ALLOWED", 3),
        ("E_CREDENTIAL_MISSING", 6),
        ("E_CREDENTIAL_EXPIRED", 6),
        ("E_CREDENTIAL_CHANGED", 6),
        ("E_CALL_SCOPE_MISSING", 6),
        ("E_BINDING_CHANGED", 6),
        ("E_CALL_IDENTITY_FAILED", 4),
        ("E_CALL_FAILED", 4),
        ("E_CALL_OUTPUT_TOO_LARGE", 4),
        ("E_CALL_TIMEOUT", 4),
    ];
    for (code, exit) in codes {
        let failure = CallFailure::new(code);
        assert_ne!(failure.message(), "The call failed.", "{code}");
        assert_eq!(failure.exit_code(), exit, "{code}");
    }
}
