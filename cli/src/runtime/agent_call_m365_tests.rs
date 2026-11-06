//! Microsoft 365 `list-folder` (#667, #709 slice A): the Graph provider in the
//! reviewed agent-call path. A child of `agent_call_tests`, so it shares that
//! module's fixture. Every request goes to a local fixture.

use super::*;

const MS_TOKEN: &str = "ms-secret-token-709";
const MS_REFRESH: &str = "ms-refresh-secret-709";
const OID: &str = "4f1c2b3a-1111-4222-8333-944455556666";
const UPN: &str = "ada@contoso.onmicrosoft.com";
/// Contoso's directory id — the tenant the work account above lives in.
const TID: &str = "72f988bf-86f1-41af-91ab-2d7cd011db47";
const DRIVE: &str = "b!AbC-_9xYz";
const MS_SCOPE: &str = "openid profile offline_access User.Read Files.Read.All";

// ── helpers ──────────────────────────────────────────────────────────────────

fn ms_shipped_manifest() -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../20-agents/aeco/cross-cutting/microsoft-365/manifest.yaml");
    std::fs::read_to_string(path).unwrap().replace("\r\n", "\n")
}

/// The shipped manifest, its runtime floor lowered to this build so the
/// handler can be exercised before the release that raises the crate version
/// (the floor itself is pinned by `the_shipped_manifest_*`).
fn ms_runnable_manifest() -> String {
    ms_shipped_manifest().replace(
        "minimum-cli-version: 0.175.0",
        &format!(
            "minimum-cli-version: {}",
            crate::validate::CURRENT_CLI_VERSION
        ),
    )
}

fn ms_home() -> tempfile::TempDir {
    let home = tempfile::tempdir().unwrap();
    let dir = home.path().join("agents").join("microsoft-365");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("manifest.yaml"), ms_runnable_manifest()).unwrap();
    home
}

fn ms_token(scope: &str, generation: &str, access: &str) -> StoredToken {
    let now = crate::auth::unix_now_secs().unwrap();
    StoredToken {
        access_token: access.into(),
        refresh_token: None,
        expires_at: now + 3600,
        scope: scope.into(),
        token_type: "Bearer".into(),
        integration: "microsoft-365".into(),
        obtained_at: now,
        generation: Some(generation.to_string()),
        source: crate::auth::keychain::TokenSource::Oauth,
    }
}

fn ms_store(home: &Path, alias: Option<&str>, scope: &str) {
    crate::auth::keychain::store_token(&ms_token(scope, GENERATION, MS_TOKEN), alias, home)
        .unwrap();
}

fn me(id: &str, upn: &str) -> Json {
    json!({"id": id, "userPrincipalName": upn, "displayName": "must-not-leave"})
}

fn children() -> Json {
    json!({
        "@odata.context": "must-not-leave",
        "value": [
            {"id": "01ABCDEF", "name": "Issued", "size": 0, "folder": {"childCount": 3},
             "lastModifiedDateTime": "2026-10-01T10:00:00Z",
             "parentReference": {"driveId": DRIVE, "path": "/drive/root:"},
             "@microsoft.graph.downloadUrl": "https://must-not-leave.example/x"},
            {"id": "01ABC!123", "name": "A-101.pdf", "size": 184320,
             "file": {"mimeType": "application/pdf", "hashes": {"sha1Hash": "x"}},
             "parentReference": {"driveId": DRIVE},
             "createdBy": {"user": {"email": "must-not-leave"}}}
        ]
    })
}

/// `/organization?$select=id` for a work or school account in tenant `tid`.
fn org(tid: &str) -> Reply {
    Reply::json(
        200,
        &json!({"@odata.context": "must-not-leave", "value": [{"id": tid, "displayName": "must-not-leave"}]}),
    )
}

/// A Graph fixture: `/v1.0/me` answers `who`, `/v1.0/organization` the work
/// tenant [`TID`], everything else `data`.
fn graph(who: Json, data: Reply) -> Fixture {
    graph_in(who, org(TID), data)
}

/// [`graph`] with `/v1.0/organization` answering `organization`.
fn graph_in(who: Json, organization: Reply, data: Reply) -> Fixture {
    Fixture::start(Arc::new(move |head: &str| {
        let path = head.split_whitespace().nth(1).unwrap_or("");
        if path == "/v1.0/me" || path.starts_with("/v1.0/me?") {
            Reply::json(200, &who)
        } else if is_organization(path) {
            organization.clone()
        } else {
            data.clone()
        }
    }))
}

fn is_organization(path: &str) -> bool {
    path == "/v1.0/organization" || path.starts_with("/v1.0/organization?")
}

/// A Graph identity request (`/me` or `/organization`), not a data request.
fn is_identity_path(path: &str) -> bool {
    path == "/v1.0/me" || path.starts_with("/v1.0/me?") || is_organization(path)
}

fn ms_endpoints(f: &Fixture) -> Endpoints {
    Endpoints {
        token: TokenPin::MicrosoftLogin,
        identity_url: format!("{}/v1.0/me", f.origin()),
        data_origin: Some(f.origin()),
        allowed_origins: vec![f.origin()],
    }
}

fn ms_installed_sha(home: &Path) -> String {
    crate::runtime::probe::manifest_sha256(
        &std::fs::read(home.join("agents/microsoft-365/manifest.yaml")).unwrap(),
    )
}

fn ms_request(home: &Path, inputs: Json) -> Map<String, Json> {
    let digest = sha256_hex(canonical_string(&inputs).as_bytes());
    let Json::Object(map) = json!({
        "schema": REQUEST_SCHEMA,
        "invocationId": "6f1c2b3a-4d5e-5f60-8a1b-2c3d4e5f6a7b",
        "agent": "microsoft-365",
        "command": "list-folder",
        "expectedAgentVersion": "0.7.0",
        "expectedManifestSha256": ms_installed_sha(home),
        "executionTransport": "rest",
        "operationSchemaVersion": 1,
        "expectedOperationSha256": ms_op().sha256(),
        "integration": "microsoft-365",
        "alias": null,
        "bindingId": binding_id("microsoft-365", "microsoft-365", OID, Some(TID)),
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

fn root_inputs() -> Json {
    json!({"drive-id": DRIVE, "folder-id": "root", "page-size": 10})
}

async fn ms_caps(home: &Path, e: &Endpoints) -> Json {
    capabilities_with(home, ms_op(), None, Duration::from_secs(10), e)
        .await
        .unwrap()
}

fn assert_ms_clean(text: &str) {
    for secret in [MS_TOKEN, MS_REFRESH, "must-not-leave"] {
        assert!(!text.contains(secret), "{secret} leaked: {text}");
    }
}

fn path_of(head: &str) -> String {
    head.split_whitespace().nth(1).unwrap_or("").to_string()
}

// ── digests ──────────────────────────────────────────────────────────────────

/// FloLess reviewed this exact operation for #618; the provider refactor must
/// not move a byte. Captured from `main` (`ac20a229f`) before any edit.
#[test]
fn the_google_operation_digest_is_byte_identical_to_the_reviewed_one() {
    assert_eq!(
        canonical_string(&op().canonical()),
        r#"{"agent":"google-workspace","command":"list-files","effect":"read","fields":"nextPageToken,incompleteSearch,files(id,name,mimeType,size,modifiedTime)","identity":"https://openidconnect.googleapis.com/v1/userinfo","inputs":[{"default":null,"location":"query","maxBytes":2048,"maximum":null,"minimum":null,"name":"q","required":false,"scalar":"string","source":"query"},{"default":100,"location":"query","maxBytes":null,"maximum":100,"minimum":1,"name":"pageSize","required":false,"scalar":"integer","source":"page-size"}],"integration":"google-workspace","method":"GET","schema":"aware.agent-call-operation/v1","scopes":["https://www.googleapis.com/auth/drive.readonly","https://www.googleapis.com/auth/drive.metadata.readonly"],"transport":"rest","url":"https://www.googleapis.com/drive/v3/files"}"#
    );
    assert_eq!(
        op().sha256(),
        "aa42474ff6700da478edd1d1412946a6edbcf471dbda0481328026be1eb6bea9"
    );
}

/// FloLess consumes `list-folder` as shipped in v0.166.0; adding operations
/// must not move a byte of it. Captured from `main` (`9ddba58a6`) before any
/// Slice B edit.
#[test]
fn the_list_folder_digest_is_byte_identical_to_the_shipped_one() {
    assert_eq!(
        canonical_string(&ms_op().canonical()),
        r#"{"adminConsent":false,"agent":"microsoft-365","command":"list-folder","effect":"read","identity":{"query":[["$select","id,userPrincipalName"]],"url":"https://graph.microsoft.com/v1.0/me"},"identityScopes":["User.Read"],"inputs":[{"default":null,"location":"path","maxBytes":256,"maximum":null,"minimum":null,"name":"drive-id","pattern":"graph-opaque-id","required":true,"scalar":"string","source":"drive-id"},{"default":null,"location":"path","maxBytes":256,"maximum":null,"minimum":null,"name":"folder-id","pattern":"graph-opaque-id","required":true,"scalar":"string","source":"folder-id"},{"default":100,"location":"query","maxBytes":null,"maximum":200,"minimum":1,"name":"$top","required":false,"scalar":"integer","source":"page-size"}],"integration":"microsoft-365","method":"GET","origin":"https://graph.microsoft.com","projection":"graph-children/1","provider":"Microsoft","query":[["$select","id,name,size,folder,file,lastModifiedDateTime,parentReference"]],"requestScopes":["User.Read","Files.Read.All"],"routes":{"alternate":{"input":"drive-id","literal":"me","segments":["v1.0","me","drive","items","{folder-id}","children"]},"default":["v1.0","drives","{drive-id}","items","{folder-id}","children"]},"schema":"aware.agent-call-operation/v1","scopes":["Files.Read","Files.Read.All","Files.ReadWrite","Files.ReadWrite.All"],"transport":"rest"}"#
    );
    assert_eq!(
        ms_op().sha256(),
        "8550a5bd95a2927de7de681c6ea9e99d63eff6f6eb09a14ed01745c311a090fe"
    );
}

#[test]
fn the_microsoft_digest_covers_routes_grammar_scopes_and_projection() {
    let canonical = canonical_string(&ms_op().canonical());
    for needle in [
        "\"provider\":\"Microsoft\"",
        "\"origin\":\"https://graph.microsoft.com\"",
        "\"{drive-id}\"",
        "\"literal\":\"me\"",
        "\"pattern\":\"graph-opaque-id\"",
        LIST_FOLDER_SELECT,
        "\"projection\":\"graph-children/1\"",
        "\"identityScopes\":[\"User.Read\"]",
        "\"Files.Read.All\"",
        "\"adminConsent\":false",
        MICROSOFT_ME_URL,
        "id,userPrincipalName",
        "$top",
    ] {
        assert!(canonical.contains(needle), "{needle} not in {canonical}");
    }
    assert_ne!(ms_op().sha256(), op().sha256());
}

// ── path grammar and routes (no network) ─────────────────────────────────────

fn map(inputs: Json) -> Map<String, Json> {
    match inputs {
        Json::Object(m) => m,
        _ => unreachable!(),
    }
}

#[test]
fn path_ids_outside_the_grammar_are_refused_before_any_url_exists() {
    let long = "a".repeat(257);
    for bad in [
        "",
        ".",
        "..",
        "a.b",
        "a/b",
        "a\\b",
        "%2F",
        "a%2e",
        "a b",
        "a:b",
        "a@b",
        "a?b",
        "a#b",
        "\u{e9}",
        long.as_str(),
    ] {
        for field in ["drive-id", "folder-id"] {
            let mut inputs = map(root_inputs());
            inputs.insert(field.into(), json!(bad));
            assert_eq!(
                wire(ms_op(), &inputs).unwrap_err(),
                CallFailure::input(if field == "drive-id" {
                    "drive-id"
                } else {
                    "folder-id"
                }),
                "{field}={bad:?}"
            );
        }
    }
    // The `me` route validates the folder id just the same.
    let mut inputs = map(json!({"drive-id": "me", "folder-id": ".."}));
    assert!(wire(ms_op(), &inputs).is_err());
    // Required: neither path input may be left out, and no unknown key passes.
    for missing in ["drive-id", "folder-id"] {
        inputs = map(root_inputs());
        inputs.remove(missing);
        assert!(wire(ms_op(), &inputs).is_err(), "{missing}");
    }
    inputs = map(root_inputs());
    inputs.insert("drive".into(), json!("x"));
    assert!(wire(ms_op(), &inputs).is_err());
    for bad_size in [0, 201] {
        inputs = map(root_inputs());
        inputs.insert("page-size".into(), json!(bad_size));
        assert!(wire(ms_op(), &inputs).is_err(), "{bad_size}");
    }
}

#[test]
fn routes_are_built_segment_by_segment_from_validated_ids() {
    let e = Endpoints::production(ms_op());
    let w = wire(
        ms_op(),
        &map(json!({"drive-id": DRIVE, "folder-id": "01ABC!123"})),
    )
    .unwrap();
    assert_eq!(
        e.data_url(ms_op(), &w).unwrap(),
        "https://graph.microsoft.com/v1.0/drives/b!AbC-_9xYz/items/01ABC!123/children"
    );
    assert_eq!(
        w.query,
        [
            ("$top".to_string(), "100".to_string()),
            ("$select".to_string(), LIST_FOLDER_SELECT.to_string()),
        ]
    );
    assert_eq!(w.limit, 100);
    let me = wire(
        ms_op(),
        &map(json!({"drive-id": "me", "folder-id": "root"})),
    )
    .unwrap();
    assert_eq!(
        e.data_url(ms_op(), &me).unwrap(),
        "https://graph.microsoft.com/v1.0/me/drive/items/root/children"
    );
    // The Google route is the URL it always was.
    let g = wire(op(), &Map::new()).unwrap();
    assert_eq!(
        Endpoints::production(op()).data_url(op(), &g).unwrap(),
        "https://www.googleapis.com/drive/v3/files"
    );
}

#[tokio::test]
async fn a_refused_path_id_sends_no_request_at_all() {
    let h = ms_home();
    ms_store(h.path(), None, MS_SCOPE);
    let f = graph(me(OID, UPN), Reply::json(200, &children()));
    let req = ms_request(h.path(), json!({"drive-id": "..", "folder-id": "root"}));
    let failure = call_with(h.path(), &parsed(&req), &ms_endpoints(&f))
        .await
        .unwrap_err();
    assert_eq!(failure, CallFailure::input("drive-id"));
    assert!(f.requests().is_empty());
}

// ── capabilities ─────────────────────────────────────────────────────────────

#[tokio::test]
async fn capabilities_verify_the_graph_account_and_publish_permissions() {
    let h = ms_home();
    ms_store(h.path(), None, MS_SCOPE);
    let f = graph(me(OID, UPN), Reply::json(200, &children()));
    let cap = ms_caps(h.path(), &ms_endpoints(&f)).await;
    assert_eq!(cap["available"], json!(true), "{cap}");
    assert_eq!(cap["credential"]["status"], json!("verified"));
    assert_eq!(
        cap["credential"]["principal"],
        json!({"integration": "microsoft-365", "stable_id": OID, "tenant_id": TID})
    );
    assert_eq!(cap["credential"]["presentation_label"], json!(UPN));
    assert_eq!(
        cap["credential"]["binding_id"],
        json!(binding_id("microsoft-365", "microsoft-365", OID, Some(TID)))
    );
    assert_eq!(
        cap["permissions"],
        json!({
            "identityScopes": ["User.Read"],
            "anyOfScopes": ["Files.Read", "Files.Read.All", "Files.ReadWrite", "Files.ReadWrite.All"],
            "requestScopes": ["User.Read", "Files.Read.All"],
            "adminConsent": false,
        })
    );
    let inputs = cap["inputs"].as_array().unwrap();
    assert_eq!(inputs[0]["location"], json!("path"));
    assert_eq!(inputs[0]["required"], json!(true));
    assert_eq!(inputs[2]["name"], json!("$top"));
    // Identity only: no data request from capabilities — `/me` asked for
    // exactly its two fields, then `/organization` for its id alone.
    let heads = f.requests();
    assert_eq!(heads.len(), 2);
    assert!(path_of(&heads[0]).starts_with("/v1.0/me?"));
    assert!(path_of(&heads[0]).contains("id%2CuserPrincipalName"));
    assert!(heads[0].contains(&format!("Bearer {MS_TOKEN}")));
    assert_eq!(path_of(&heads[1]), "/v1.0/organization?%24select=id");
    assert!(heads[1].contains(&format!("Bearer {MS_TOKEN}")));
    assert_ms_clean(&cap.to_string());
}

#[tokio::test]
async fn without_user_read_nothing_is_sent_and_the_hint_names_both_scopes() {
    let h = ms_home();
    ms_store(h.path(), None, "offline_access Files.Read.All");
    let f = graph(me(OID, UPN), Reply::json(200, &children()));
    let cap = ms_caps(h.path(), &ms_endpoints(&f)).await;
    assert_eq!(cap["credential"]["status"], json!("unverified"));
    assert_eq!(cap["credential"]["revision"], json!(0));
    assert_eq!(cap["unavailableCode"], json!("scope-missing"));
    assert_eq!(
        cap["unavailableReason"],
        json!(
            "This Microsoft account was connected without OneDrive read access. Reconnect it with `aware connect microsoft-365 --oauth --scopes User.Read,Files.Read.All`."
        )
    );
    assert!(f.requests().is_empty(), "/me was asked without User.Read");
    let failure = call_with(
        h.path(),
        &parsed(&ms_request(h.path(), root_inputs())),
        &ms_endpoints(&f),
    )
    .await
    .unwrap_err();
    assert_eq!(failure.code, "E_CALL_SCOPE_MISSING");
    assert!(f.requests().is_empty());
}

#[tokio::test]
async fn user_read_alone_is_a_verified_account_without_files_access() {
    let h = ms_home();
    ms_store(h.path(), Some("work"), "offline_access User.Read");
    let f = graph(me(OID, UPN), Reply::json(200, &children()));
    let cap = capabilities_with(
        h.path(),
        ms_op(),
        Some("work"),
        Duration::from_secs(10),
        &ms_endpoints(&f),
    )
    .await
    .unwrap();
    assert_eq!(cap["credential"]["status"], json!("verified"));
    assert_eq!(cap["unavailableCode"], json!("scope-missing"));
    assert!(cap["unavailableReason"].as_str().unwrap().contains(
        "aware connect microsoft-365 --as=work --oauth --scopes User.Read,Files.Read.All"
    ));
}

#[test]
fn graph_scopes_match_in_either_form_and_lookalikes_never_do() {
    let holds = |scope: &str| {
        SlotToken {
            access_token: String::new(),
            generation: String::new(),
            scope: scope.into(),
        }
        .has_operation_scope(ms_op())
    };
    assert!(holds("User.Read Files.Read.All"));
    assert!(holds("https://graph.microsoft.com/Files.Read.All"));
    assert!(holds("files.read"));
    assert!(holds("Files.ReadWrite.All"));
    assert!(!holds("Sites.Read.All User.Read"));
    assert!(!holds("https://graph.microsoft.com.evil/Files.Read.All"));
    assert!(!holds("https://evil.example/Files.Read.All"));
    assert!(!holds("Files.Read.All.Extra"));
    // Google still compares exactly.
    let google = SlotToken {
        access_token: String::new(),
        generation: String::new(),
        scope: DRIVE_READONLY.to_ascii_uppercase(),
    };
    assert!(!google.has_operation_scope(op()));
}

// ── bound call ───────────────────────────────────────────────────────────────

#[tokio::test]
async fn a_bound_call_lists_the_folder_and_returns_only_the_projection() {
    let h = ms_home();
    ms_store(h.path(), None, MS_SCOPE);
    let f = graph(me(OID, UPN), Reply::json(200, &children()));
    let out = call_with(
        h.path(),
        &parsed(&ms_request(h.path(), root_inputs())),
        &ms_endpoints(&f),
    )
    .await
    .unwrap();
    let body = &out["result"]["body"];
    assert_eq!(
        body,
        &json!({
            "items": [
                {"id": "01ABCDEF", "drive-id": DRIVE, "name": "Issued", "is-folder": true,
                 "size": 0, "modified-time": "2026-10-01T10:00:00Z"},
                {"id": "01ABC!123", "drive-id": DRIVE, "name": "A-101.pdf", "is-folder": false,
                 "size": 184320, "mime-type": "application/pdf"}
            ],
            "more-available": false,
            "omitted": 0,
        })
    );
    let data: Vec<String> = f
        .requests()
        .iter()
        .map(|h| path_of(h))
        .filter(|p| p.starts_with("/v1.0/drives/"))
        .collect();
    assert_eq!(data.len(), 1);
    assert!(
        data[0].starts_with("/v1.0/drives/b!AbC-_9xYz/items/root/children?"),
        "{}",
        data[0]
    );
    assert!(data[0].contains("%24top=10"), "{}", data[0]);
    assert_ms_clean(&out.to_string());
}

#[tokio::test]
async fn the_me_route_lists_the_signed_in_persons_own_drive() {
    let h = ms_home();
    ms_store(h.path(), None, MS_SCOPE);
    let f = graph(me(OID, UPN), Reply::json(200, &children()));
    let out = call_with(
        h.path(),
        &parsed(&ms_request(
            h.path(),
            json!({"drive-id": "me", "folder-id": "root"}),
        )),
        &ms_endpoints(&f),
    )
    .await
    .unwrap();
    assert_eq!(out["result"]["body"]["items"].as_array().unwrap().len(), 2);
    assert!(
        f.requests()
            .iter()
            .any(|h| path_of(h).starts_with("/v1.0/me/drive/items/root/children?"))
    );
}

#[tokio::test]
async fn the_binding_follows_the_graph_id_not_the_user_principal_name() {
    let h = ms_home();
    ms_store(h.path(), None, MS_SCOPE);
    // A renamed sign-in name, same directory object: still the bound account.
    let renamed = graph(
        me(OID, "ada.lovelace@contoso.com"),
        Reply::json(200, &children()),
    );
    assert!(
        call_with(
            h.path(),
            &parsed(&ms_request(h.path(), root_inputs())),
            &ms_endpoints(&renamed)
        )
        .await
        .is_ok()
    );
    // Another object under the same name (a guest, `#EXT#`): refused before data.
    let other = graph(
        me("9a8b7c6d-0000-4000-8000-000000000001", UPN),
        Reply::json(200, &children()),
    );
    let failure = call_with(
        h.path(),
        &parsed(&ms_request(h.path(), root_inputs())),
        &ms_endpoints(&other),
    )
    .await
    .unwrap_err();
    assert_eq!(failure.code, "E_BINDING_CHANGED");
    assert_eq!(other.count("/v1.0/drives"), 0);
    // A personal (MSA) account id is opaque, not a GUID — it binds as is.
    let msa = "6e4f1a2b3c4d5e6f";
    let personal = graph(
        me(msa, "ada_live.com#EXT#@contoso.onmicrosoft.com"),
        Reply::json(200, &children()),
    );
    let mut req = ms_request(h.path(), root_inputs());
    req.insert(
        "bindingId".into(),
        json!(binding_id("microsoft-365", "microsoft-365", msa, Some(TID))),
    );
    assert!(
        call_with(h.path(), &parsed(&req), &ms_endpoints(&personal))
            .await
            .is_ok()
    );
    // No id at all: the identity is unconfirmed.
    let no_id = graph(
        json!({"userPrincipalName": UPN}),
        Reply::json(200, &children()),
    );
    let failure = call_with(
        h.path(),
        &parsed(&ms_request(h.path(), root_inputs())),
        &ms_endpoints(&no_id),
    )
    .await
    .unwrap_err();
    assert_eq!(
        failure,
        CallFailure::reason("E_CALL_IDENTITY_FAILED", "no-subject")
    );
}

#[tokio::test]
async fn a_graph_redirect_or_next_link_is_never_followed() {
    let h = ms_home();
    ms_store(h.path(), None, MS_SCOPE);
    let sink = Fixture::start(Arc::new(|_: &str| Reply::json(200, &children())));
    for status in [301, 302, 307, 308] {
        let redirect = Reply {
            status,
            headers: format!("Location: {}/stolen\r\n", sink.origin()),
            body: Vec::new(),
            delay: Duration::ZERO,
        };
        let f = graph(me(OID, UPN), redirect);
        let failure = call_with(
            h.path(),
            &parsed(&ms_request(h.path(), root_inputs())),
            &ms_endpoints(&f),
        )
        .await
        .unwrap_err();
        assert_eq!(failure.code, "E_CALL_FAILED", "{status}");
        assert_ms_clean(&json!(failure.details).to_string());
    }
    // A next page on another host is reported, never fetched.
    let mut paged = children();
    paged["@odata.nextLink"] = json!(format!("{}/v1.0/next", sink.origin()));
    let f = graph(me(OID, UPN), Reply::json(200, &paged));
    let out = call_with(
        h.path(),
        &parsed(&ms_request(h.path(), root_inputs())),
        &ms_endpoints(&f),
    )
    .await
    .unwrap();
    assert_eq!(out["result"]["body"]["more-available"], json!(true));
    assert!(sink.requests().is_empty());
}

// ── token endpoint pin and refresh ───────────────────────────────────────────

#[test]
fn only_microsofts_own_token_endpoint_may_receive_the_refresh_token() {
    for good in [
        "https://login.microsoftonline.com/common/oauth2/v2.0/token",
        "https://login.microsoftonline.com/organizations/oauth2/v2.0/token",
        "https://login.microsoftonline.com/contoso.onmicrosoft.com/oauth2/v2.0/token",
        "https://login.microsoftonline.com/4f1c2b3a-1111-4222-8333-944455556666/oauth2/v2.0/token",
    ] {
        assert!(is_microsoft_token_url(good), "{good}");
    }
    for bad in [
        "https://login.microsoftonline.com/oauth2/v2.0/token",
        "https://login.microsoftonline.com/../oauth2/v2.0/token",
        "https://login.microsoftonline.com/a..b/oauth2/v2.0/token",
        "https://login.microsoftonline.com/.x/oauth2/v2.0/token",
        "https://login.microsoftonline.com/a/b/oauth2/v2.0/token",
        "https://login.microsoftonline.com/a@evil.com/oauth2/v2.0/token",
        "https://login.microsoftonline.com/a%2f/oauth2/v2.0/token",
        "https://login.microsoftonline.com.evil.com/common/oauth2/v2.0/token",
        "http://login.microsoftonline.com/common/oauth2/v2.0/token",
        "https://login.microsoftonline.com/common/oauth2/v2.0/token?x=1",
        "https://evil.example/common/oauth2/v2.0/token",
    ] {
        assert!(!is_microsoft_token_url(bad), "{bad}");
    }
}

fn expired_ms(home: &Path, scope: &str) {
    let mut t = ms_token(scope, GENERATION, "stale-ms-token");
    t.expires_at = 1;
    t.refresh_token = Some(MS_REFRESH.into());
    crate::auth::keychain::store_token(&t, None, home).unwrap();
}

#[tokio::test]
async fn a_profile_token_url_is_never_sent_the_microsoft_refresh_token() {
    let h = ms_home();
    expired_ms(h.path(), MS_SCOPE);
    let sink = Fixture::start(Arc::new(|_: &str| {
        Reply::json(200, &json!({"access_token": "x", "expires_in": 3600}))
    }));
    std::fs::create_dir_all(h.path().join("oauth")).unwrap();
    std::fs::write(
        h.path().join("oauth/microsoft-365.yaml"),
        format!("token_url: {}/token\n", sink.origin()),
    )
    .unwrap();
    let f = graph(me(OID, UPN), Reply::json(200, &children()));
    let cap = ms_caps(h.path(), &ms_endpoints(&f)).await;
    assert_eq!(cap["unavailableCode"], json!("credential-expired"));
    let failure = call_with(
        h.path(),
        &parsed(&ms_request(h.path(), root_inputs())),
        &ms_endpoints(&f),
    )
    .await
    .unwrap_err();
    assert_eq!(
        failure,
        CallFailure::reason("E_CREDENTIAL_EXPIRED", "token-endpoint")
    );
    assert!(
        sink.requests().is_empty(),
        "the refresh token went to the profile URL"
    );
    assert!(f.requests().is_empty());
}

#[tokio::test]
async fn a_tenant_profile_keeps_the_microsoft_endpoint_and_passes_the_pin() {
    let h = ms_home();
    ms_store(h.path(), None, MS_SCOPE);
    std::fs::create_dir_all(h.path().join("oauth")).unwrap();
    std::fs::write(
        h.path().join("oauth/microsoft-365.yaml"),
        "client_id: d4453cc5-0000-4000-8000-000000000000\ntenant: contoso.onmicrosoft.com\n",
    )
    .unwrap();
    let f = graph(me(OID, UPN), Reply::json(200, &children()));
    let cap = ms_caps(h.path(), &ms_endpoints(&f)).await;
    assert_eq!(cap["available"], json!(true), "{cap}");
}

#[tokio::test]
async fn a_redirecting_token_endpoint_sends_nothing_onward() {
    for status in [302, 307, 308] {
        let h = ms_home();
        expired_ms(h.path(), MS_SCOPE);
        let sink = Fixture::start(Arc::new(|_: &str| {
            Reply::json(200, &json!({"access_token": "x", "expires_in": 3600}))
        }));
        let location = format!("Location: {}/stolen\r\n", sink.origin());
        let token_endpoint = Fixture::start(Arc::new(move |_: &str| Reply {
            status,
            headers: location.clone(),
            body: Vec::new(),
            delay: Duration::ZERO,
        }));
        let token_url = format!("{}/token", token_endpoint.origin());
        std::fs::create_dir_all(h.path().join("oauth")).unwrap();
        std::fs::write(
            h.path().join("oauth/microsoft-365.yaml"),
            format!("token_url: {token_url}\n"),
        )
        .unwrap();
        let f = graph(me(OID, UPN), Reply::json(200, &children()));
        let mut e = ms_endpoints(&f);
        e.token = TokenPin::Exact(token_url);
        let failure = call_with(h.path(), &parsed(&ms_request(h.path(), root_inputs())), &e)
            .await
            .unwrap_err();
        assert_eq!(failure.code, "E_CREDENTIAL_EXPIRED", "{status}");
        assert_eq!(token_endpoint.requests().len(), 1, "{status}");
        assert!(
            sink.requests().is_empty(),
            "{status}: the redirect was followed"
        );
        assert!(f.requests().is_empty(), "{status}");
        assert_ms_clean(&json!({"code": failure.code, "d": failure.details}).to_string());
    }
}

/// Run one refresh whose answer reports `scope`, then a bound call.
async fn call_after_refresh(scope: &str) -> Result<Json, CallFailure> {
    let h = ms_home();
    expired_ms(h.path(), MS_SCOPE);
    let reply = json!({"access_token": MS_TOKEN, "expires_in": 3600, "scope": scope,
                       "token_type": "Bearer"});
    let token_endpoint = Fixture::start(Arc::new(move |_: &str| Reply::json(200, &reply)));
    let token_url = format!("{}/token", token_endpoint.origin());
    std::fs::create_dir_all(h.path().join("oauth")).unwrap();
    std::fs::write(
        h.path().join("oauth/microsoft-365.yaml"),
        format!("token_url: {token_url}\n"),
    )
    .unwrap();
    let f = graph(me(OID, UPN), Reply::json(200, &children()));
    let mut e = ms_endpoints(&f);
    e.token = TokenPin::Exact(token_url);
    call_with(h.path(), &parsed(&ms_request(h.path(), root_inputs())), &e).await
}

#[tokio::test]
async fn a_refresh_reporting_the_same_grant_in_url_form_keeps_the_generation() {
    let same = "openid profile offline_access https://graph.microsoft.com/User.Read https://graph.microsoft.com/files.read.all";
    assert!(call_after_refresh(same).await.is_ok());
    // A grant that lost Files.Read.All is a different credential.
    let lost = "openid profile offline_access https://graph.microsoft.com/User.Read";
    assert_eq!(
        call_after_refresh(lost).await.unwrap_err(),
        CallFailure::reason("E_CREDENTIAL_CHANGED", "refreshed")
    );
    // A lookalike resource host is not Graph.
    let lookalike =
        "openid profile offline_access User.Read https://graph.microsoft.com.evil/Files.Read.All";
    assert!(call_after_refresh(lookalike).await.is_err());
}

#[test]
fn same_grant_folds_graph_forms_for_microsoft_only() {
    use crate::auth::keychain::same_grant;
    assert!(same_grant(
        "microsoft-365",
        "User.Read Files.Read.All",
        "https://graph.microsoft.com/files.read.all https://graph.microsoft.com/User.Read"
    ));
    assert!(!same_grant(
        "microsoft-365",
        "User.Read Files.Read.All",
        "User.Read"
    ));
    assert!(!same_grant(
        "microsoft-365",
        "Files.Read.All",
        "https://graph.microsoft.com.evil/Files.Read.All"
    ));
    assert!(!same_grant("google-workspace", "openid", "OPENID"));
    assert!(same_grant("google-workspace", "b a", "a b"));
}

// ── projection ───────────────────────────────────────────────────────────────

#[test]
fn the_children_projection_never_claims_a_complete_listing_it_did_not_return() {
    let three = json!({"value": [{"id": "a"}, {"id": "b"}, {"id": "c"}]});
    let cut = project_children(&three, 2).unwrap();
    assert_eq!(cut["items"].as_array().unwrap().len(), 2);
    assert_eq!(cut["more-available"], json!(true));
    let exact = project_children(&three, 3).unwrap();
    assert_eq!(exact["more-available"], json!(false));
    let malformed = json!({"value": [
        {"id": "ok"},
        {"id": ""},
        {"id": "bad\u{0007}id"},
        {"name": "no id"},
        {"id": "x", "parentReference": {"driveId": 7}},
        {"id": "y", "parentReference": {"driveId": null}},
    ]});
    let out = project_children(&malformed, 100).unwrap();
    let ids: Vec<&str> = out["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|i| i["id"].as_str().unwrap())
        .collect();
    assert_eq!(ids, ["ok", "y"]);
    assert_eq!(out["omitted"], json!(4));
    // Ids are kept byte for byte; display names may be cleaned.
    let kept = project_children(
        &json!({"value": [{"id": " b!x ", "name": "  A\u{0000}B  ", "size": -1}]}),
        5,
    )
    .unwrap();
    assert_eq!(kept["items"][0]["id"], json!(" b!x "));
    assert_eq!(kept["items"][0]["name"], json!("AB"));
    assert!(kept["items"][0].get("size").is_none());
    assert!(project_children(&json!({"items": []}), 5).is_none());
    assert!(project_children(&json!([]), 5).is_none());
}

// ── workflow path ────────────────────────────────────────────────────────────

#[tokio::test]
async fn a_workflow_node_lists_a_folder_without_an_identity_read() {
    let h = ms_home();
    // A workflow has no binding: the operation scope suffices.
    ms_store(h.path(), None, "offline_access Files.Read.All");
    let f = graph(me(OID, UPN), Reply::json(200, &children()));
    let manifest: Agent = serde_yaml::from_str(&ms_runnable_manifest()).unwrap();
    let out = run_for_workflow_with(
        h.path(),
        &manifest,
        "list-folder",
        &json!({"drive-id": "me", "folder-id": "root"}),
        Some(&ms_endpoints(&f)),
        Duration::from_secs(10),
        1 << 20,
    )
    .await
    .unwrap();
    assert_eq!(out["items"].as_array().unwrap().len(), 2);
    assert!(
        f.requests()
            .iter()
            .all(|h| !path_of(h).starts_with("/v1.0/me?"))
    );
    // Without a Files scope nothing is sent.
    ms_store(h.path(), None, "offline_access User.Read");
    let before = f.requests().len();
    let failure = run_for_workflow_with(
        h.path(),
        &manifest,
        "list-folder",
        &json!({"drive-id": "me", "folder-id": "root"}),
        Some(&ms_endpoints(&f)),
        Duration::from_secs(10),
        1 << 20,
    )
    .await
    .unwrap_err();
    assert_eq!(failure.code, "E_CALL_SCOPE_MISSING");
    assert_eq!(f.requests().len(), before);
}

// ── aware agent invoke (#745) ────────────────────────────────────────────────

/// `--as` picks the slot: the token sent is that slot's, and the default slot,
/// connected without a Files scope, is never read for it.
#[tokio::test]
async fn an_invoked_read_uses_the_slot_named_by_as() {
    let h = ms_home();
    ms_store(h.path(), None, "offline_access User.Read");
    crate::auth::keychain::store_token(
        &ms_token(
            "offline_access Files.Read.All",
            GENERATION,
            "work-slot-token",
        ),
        Some("work"),
        h.path(),
    )
    .unwrap();
    let f = graph(me(OID, UPN), Reply::json(200, &children()));
    let args = json!({"drive-id": "me", "folder-id": "root"});
    let run = |alias: Option<&'static str>| {
        let endpoints = ms_endpoints(&f);
        let manifest: Agent = serde_yaml::from_str(&ms_runnable_manifest()).unwrap();
        let args = args.clone();
        let home = h.path().to_path_buf();
        async move {
            run_reviewed(
                &home,
                &manifest,
                "list-folder",
                &args,
                alias,
                Some(&endpoints),
                Duration::from_secs(10),
                1 << 20,
            )
            .await
        }
    };
    let out = run(Some("work")).await.unwrap();
    assert_eq!(out["items"].as_array().unwrap().len(), 2);
    let sent = f.requests();
    assert_eq!(sent.len(), 1, "one data request, no identity read");
    assert!(sent[0].contains("Bearer work-slot-token"), "{}", sent[0]);
    // Without --as it is the manifest's own (default) slot, which lacks the scope.
    let failure = run(None).await.unwrap_err();
    assert_eq!(failure.code, "E_CALL_SCOPE_MISSING");
    // A slot that is not connected, or a malformed alias, sends nothing.
    let missing = run(Some("nobody")).await.unwrap_err();
    assert_eq!(missing.code, "E_CREDENTIAL_MISSING");
    let malformed = run(Some("../work")).await.unwrap_err();
    assert_eq!(malformed.code, "E_CALL_REQUEST_INVALID");
    assert_eq!(f.requests().len(), 1);
}

/// `--as` cannot reach another integration's slots: the manifest must still
/// name this operation's integration, exactly as for a workflow node.
#[tokio::test]
async fn an_invoked_read_refuses_a_manifest_for_another_integration() {
    let h = ms_home();
    crate::auth::keychain::store_token(
        &ms_token(
            "offline_access Files.Read.All",
            GENERATION,
            "work-slot-token",
        ),
        Some("work"),
        h.path(),
    )
    .unwrap();
    let f = graph(me(OID, UPN), Reply::json(200, &children()));
    let manifest: Agent = serde_yaml::from_str(
        &ms_runnable_manifest().replace("secret: microsoft-365", "secret: google-workspace"),
    )
    .unwrap();
    let failure = run_reviewed(
        h.path(),
        &manifest,
        "list-folder",
        &json!({"drive-id": "me", "folder-id": "root"}),
        Some("work"),
        Some(&ms_endpoints(&f)),
        Duration::from_secs(10),
        1 << 20,
    )
    .await
    .unwrap_err();
    assert_eq!(failure.code, "E_CALL_INVALID");
    assert!(f.requests().is_empty());
}

// ── the shipped manifest ─────────────────────────────────────────────────────

#[test]
fn the_shipped_manifest_runs_only_the_reviewed_reads_and_only_on_a_new_enough_cli() {
    let agent: Agent = serde_yaml::from_str(&ms_shipped_manifest()).unwrap();
    assert!(ms_shipped_manifest().contains(
        "
version:      0.7.0
"
    ));
    assert_eq!(agent.status, AgentStatus::RequiresRuntime);
    assert_eq!(agent.minimum_cli_version.as_deref(), Some("0.175.0"));
    // An older CLI has no handler for the Outlook folder reads and would
    // render them generically.
    assert!(crate::validate::runtime_requirement_error(&agent, "0.174.0").is_some());
    assert!(crate::validate::runtime_requirement_error(&agent, "0.175.0").is_none());
    // REST only: the never-shipped `aware-m365` binary is gone.
    assert!(agent.transport.cli.is_none());
    assert_eq!(
        crate::runtime::invoker::dispatch_transport(&agent.transport),
        Some(crate::runtime::invoker::TransportKind::Rest)
    );
    let runnable: Vec<&str> = agent
        .commands
        .iter()
        .filter(|(_, c)| c.status != AgentStatus::Planned)
        .map(|(n, _)| n.as_str())
        .collect();
    let mut runnable = runnable;
    runnable.sort_unstable();
    assert_eq!(runnable, REVIEWED_READS);
    let runnable_manifest: Agent = serde_yaml::from_str(&ms_runnable_manifest()).unwrap();
    for command in REVIEWED_READS {
        // Every runnable command is a reviewed operation, and runs as one.
        let reviewed = operation("microsoft-365", command)
            .unwrap_or_else(|| panic!("{command} has no reviewed operation"));
        assert!(command_runnable(&runnable_manifest, reviewed), "{command}");
        assert!(manifest_slot(&runnable_manifest, reviewed, false).is_ok());
    }
    assert!(
        crate::validate::validate_agent(&runnable_manifest)
            .iter()
            .all(|issue| issue.severity != crate::validate::Severity::Error),
        "{:?}",
        crate::validate::validate_agent(&runnable_manifest)
    );
}

/// The commands the shipped manifest makes runnable, sorted: each a reviewed
/// operation in [`OPERATIONS`].
const REVIEWED_READS: [&str; 18] = [
    "excel.range.read",
    "excel.worksheet.list",
    "get-drive-item",
    "get-shared-item",
    "list-folder",
    "outlook.mail.folder.list",
    "outlook.mail.folder.list-messages",
    "outlook.mail.get-message",
    "outlook.mail.search",
    "planner.plan.list",
    "planner.task.list",
    "sharepoint.list.list",
    "sharepoint.list.read",
    "teams.channel.list",
    "teams.channel.list-messages",
    "teams.chat.list",
    "teams.chat.list-messages",
    "teams.team.list",
];

/// Every command's published permissions, checked one by one against the
/// delegated column of Microsoft's Graph permissions reference (2026-11-01).
/// Only `teams.channel.list-messages` publishes a request set that needs
/// tenant admin consent (`ChannelMessage.Read.All`); some scopes the code also
/// accepts elsewhere, like `Group.Read.All`, do too, and AWARE never asks.
#[test]
fn every_command_publishes_the_delegated_permissions_it_needs() {
    let agent: Agent = serde_yaml::from_str(&ms_shipped_manifest()).unwrap();
    let expected: &[(&str, &[&str])] = &[
        ("get-drive-item", &["User.Read", "Files.Read.All"]),
        ("get-shared-item", &["User.Read", "Files.ReadWrite"]),
        ("download-file", &["User.Read", "Files.Read.All"]),
        ("upload-file", &["User.Read", "Files.ReadWrite.All"]),
        ("list-folder", &["User.Read", "Files.Read.All"]),
        (
            "teams.chat.post-message",
            &["User.Read", "ChatMessage.Send"],
        ),
        ("teams.team.list", &["User.Read", "Team.ReadBasic.All"]),
        (
            "teams.channel.list",
            &["User.Read", "Channel.ReadBasic.All"],
        ),
        (
            "teams.channel.list-messages",
            &["User.Read", "ChannelMessage.Read.All"],
        ),
        ("teams.chat.list", &["User.Read", "Chat.ReadBasic"]),
        ("teams.chat.list-messages", &["User.Read", "Chat.Read"]),
        (
            "teams.channel.post-message",
            &["User.Read", "ChannelMessage.Send"],
        ),
        (
            "teams.channel.post-with-card",
            &["User.Read", "ChannelMessage.Send"],
        ),
        (
            "teams.channel.post-with-screenshot",
            &["User.Read", "ChannelMessage.Send"],
        ),
        ("outlook.mail.send", &["User.Read", "Mail.Send"]),
        (
            "outlook.mail.send-with-attachment",
            &["User.Read", "Mail.Send"],
        ),
        ("outlook.mail.search", &["User.Read", "Mail.Read"]),
        ("outlook.mail.folder.list", &["User.Read", "Mail.Read"]),
        (
            "outlook.mail.folder.list-messages",
            &["User.Read", "Mail.Read"],
        ),
        ("outlook.mail.get-message", &["User.Read", "Mail.Read"]),
        (
            "outlook.calendar.create-event",
            &["User.Read", "Calendars.ReadWrite"],
        ),
        (
            "outlook.calendar.find-availability",
            &["User.Read", "Calendars.Read.Shared"],
        ),
        ("planner.task.create", &["User.Read", "Tasks.ReadWrite"]),
        ("planner.plan.list", &["User.Read", "Tasks.Read"]),
        ("planner.task.list", &["User.Read", "Tasks.Read"]),
        ("planner.task.assign", &["User.Read", "Tasks.ReadWrite"]),
        ("planner.task.complete", &["User.Read", "Tasks.ReadWrite"]),
        ("sharepoint.list.list", &["User.Read", "Sites.Read.All"]),
        ("sharepoint.list.read", &["User.Read", "Sites.Read.All"]),
        (
            "sharepoint.list.append-row",
            &["User.Read", "Sites.ReadWrite.All"],
        ),
        ("excel.range.read", &["User.Read", "Files.ReadWrite"]),
        ("excel.range.write", &["User.Read", "Files.ReadWrite.All"]),
        ("excel.worksheet.list", &["User.Read", "Files.ReadWrite"]),
        (
            "excel.worksheet.create",
            &["User.Read", "Files.ReadWrite.All"],
        ),
        (
            "excel.table.append-row",
            &["User.Read", "Files.ReadWrite.All"],
        ),
    ];
    assert_eq!(agent.commands.len(), expected.len());
    for (name, scopes) in expected {
        let permissions = agent.commands[*name]
            .permissions
            .as_ref()
            .unwrap_or_else(|| panic!("{name} declares no permissions"));
        assert_eq!(permissions.scopes, *scopes, "{name}");
        assert_eq!(
            permissions.admin_consent,
            *name == "teams.channel.list-messages",
            "{name}"
        );
    }
    // Each reviewed operation and its published description agree.
    for command in REVIEWED_READS {
        let reviewed = operation("microsoft-365", command).unwrap();
        let published = agent.commands[command].permissions.as_ref().unwrap();
        assert_eq!(published.scopes, reviewed.request_scopes, "{command}");
        assert_eq!(published.admin_consent, reviewed.admin_consent, "{command}");
    }
}

#[test]
fn a_malformed_permissions_block_is_a_manifest_error() {
    let base = ms_runnable_manifest();
    let swap = |from: &str, to: &str| -> String {
        assert!(base.contains(from), "{from}");
        base.replacen(from, to, 1)
    };
    for bad in [
        swap(
            "      admin-consent: false\n",
            "      admin_consent: false\n",
        ),
        swap(
            "      admin-consent: false\n",
            "      admin-consent: \"no\"\n",
        ),
        swap(
            "      admin-consent: false\n",
            "      admin-consent: false\n      extra: 1\n",
        ),
        swap(
            "      scopes: [User.Read, Files.Read.All]\n",
            "      scopes: User.Read\n",
        ),
    ] {
        assert!(serde_yaml::from_str::<Agent>(&bad).is_err(), "{bad}");
    }
    for bad in [
        swap(
            "      scopes: [User.Read, Files.Read.All]\n",
            "      scopes: []\n",
        ),
        swap(
            "      scopes: [User.Read, Files.Read.All]\n",
            "      scopes: [\"User.Read Files.Read.All\"]\n",
        ),
    ] {
        let agent: Agent = serde_yaml::from_str(&bad).unwrap();
        assert!(
            crate::validate::validate_agent(&agent)
                .iter()
                .any(|issue| issue.code == "E_AGENT_PERMISSIONS_INVALID"),
            "{bad}"
        );
    }
    // Omitted means "not declared".
    let omitted = swap(
        "    permissions:\n      scopes: [User.Read, Files.Read.All]\n      admin-consent: false\n    mode: read\n",
        "    mode: read\n",
    );
    let agent: Agent = serde_yaml::from_str(&omitted).unwrap();
    // The block removed is the first one of that shape (a reviewed read's).
    let undeclared: Vec<&str> = agent
        .commands
        .iter()
        .filter(|(_, c)| c.permissions.is_none())
        .map(|(n, _)| n.as_str())
        .collect();
    assert_eq!(undeclared.len(), 1, "{undeclared:?}");
    assert!(REVIEWED_READS.contains(&undeclared[0]));
}

// ── tenant (#764) ────────────────────────────────────────────────────────────

const CONSUMERS: &str = MICROSOFT_CONSUMER_TENANT;
/// A personal account's Graph id: opaque, not a GUID.
const MSA_ID: &str = "6e4f1a2b3c4d5e6f";
/// Another directory than [`TID`].
const OTHER_TID: &str = "0a1b2c3d-0000-4000-8000-00000000abcd";

fn b64url(text: &str) -> String {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(text)
}

/// A token-endpoint reply naming home tenant `utid` (and, when given, an ID
/// token signed in to `tid`).
fn token_reply(utid: &str, tid: Option<&str>) -> Json {
    let mut reply = json!({
        "access_token": "ms-refreshed-764", "refresh_token": "ms-rotated-764",
        "expires_in": 3600, "token_type": "Bearer",
        "client_info": b64url(&format!(r#"{{"uid":"u","utid":"{utid}"}}"#)),
    });
    if let Some(tid) = tid {
        reply["id_token"] = json!(format!(
            "{}.{}.sig",
            b64url(r#"{"alg":"RS256"}"#),
            b64url(&format!(r#"{{"tid":"{tid}"}}"#))
        ));
    }
    reply
}

/// A home whose slot holds a fresh token with a refresh token, its profile
/// pointing the token endpoint at a local `…/<authority>/oauth2/v2.0/token`
/// that answers `reply`. Returns the home, the token endpoint and its URL.
fn personal_home(authority: &str, reply: Json) -> (tempfile::TempDir, Fixture, String) {
    let h = ms_home();
    let mut t = ms_token(MS_SCOPE, GENERATION, MS_TOKEN);
    t.refresh_token = Some(MS_REFRESH.into());
    crate::auth::keychain::store_token(&t, None, h.path()).unwrap();
    let token_endpoint = Fixture::start(Arc::new(move |_: &str| Reply::json(200, &reply)));
    let token_url = format!("{}/{authority}/oauth2/v2.0/token", token_endpoint.origin());
    std::fs::create_dir_all(h.path().join("oauth")).unwrap();
    std::fs::write(
        h.path().join("oauth/microsoft-365.yaml"),
        format!("token_url: {token_url}\n"),
    )
    .unwrap();
    (h, token_endpoint, token_url)
}

fn pinned(f: &Fixture, token_url: &str) -> Endpoints {
    let mut e = ms_endpoints(f);
    e.token = TokenPin::Exact(token_url.to_string());
    e
}

#[test]
fn the_tenant_is_part_of_the_microsoft_binding() {
    let here = binding_id("microsoft-365", "microsoft-365", OID, Some(TID));
    // The same oid in another directory is another binding.
    let elsewhere = binding_id("microsoft-365", "microsoft-365", OID, Some(OTHER_TID));
    assert_ne!(here, elsewhere);
    assert!(is_uuid(&here) && is_uuid(&elsewhere));
    // And never the tenant-less id an oid alone would hash to.
    assert_ne!(
        here,
        binding_id("microsoft-365", "microsoft-365", OID, None)
    );
    // Part boundaries are kept: moving bytes between oid and tenant changes it.
    assert_ne!(
        binding_id("microsoft-365", "microsoft-365", "ab", Some("c")),
        binding_id("microsoft-365", "microsoft-365", "a", Some("bc"))
    );
}

#[tokio::test]
async fn a_guest_is_bound_to_the_directory_graph_answers_for_its_token() {
    let h = ms_home();
    ms_store(h.path(), None, MS_SCOPE);
    // `/organization` answers the directory the token was issued in, here in
    // upper case; AWARE reports and binds it lowercased.
    let guest = "1b2c3d4e-0000-4000-8000-000000000002";
    let f = graph_in(
        me(guest, "ada_contoso.com#EXT#@fabrikam.onmicrosoft.com"),
        org(&OTHER_TID.to_uppercase()),
        Reply::json(200, &children()),
    );
    let cap = ms_caps(h.path(), &ms_endpoints(&f)).await;
    assert_eq!(cap["available"], json!(true), "{cap}");
    assert_eq!(
        cap["credential"]["principal"]["tenant_id"],
        json!(OTHER_TID)
    );
    assert_eq!(
        cap["credential"]["binding_id"],
        json!(binding_id(
            "microsoft-365",
            "microsoft-365",
            guest,
            Some(OTHER_TID)
        ))
    );
}

#[tokio::test]
async fn an_organization_answer_that_is_not_exactly_one_guid_is_never_a_tenant() {
    for answer in [
        json!({"value": []}),
        json!({"value": [{"id": TID}, {"id": OTHER_TID}]}),
        json!({"value": [{"id": "contoso.onmicrosoft.com"}]}),
        json!({"value": [{"displayName": "no id"}]}),
        json!({"id": TID}),
    ] {
        let h = ms_home();
        ms_store(h.path(), None, MS_SCOPE);
        let f = graph_in(
            me(OID, UPN),
            Reply::json(200, &answer),
            Reply::json(200, &children()),
        );
        // The bundled authority is `common`, but this slot holds no refresh
        // token: the token endpoint cannot be asked, so it stays unverified.
        let cap = ms_caps(h.path(), &ms_endpoints(&f)).await;
        assert_eq!(cap["credential"]["status"], json!("unverified"), "{answer}");
        assert_eq!(
            cap["unavailableCode"],
            json!("identity-unverified"),
            "{answer}"
        );
        assert!(cap["credential"].get("principal").is_none(), "{answer}");
    }
}

#[tokio::test]
async fn a_personal_account_gets_the_consumer_tenant_from_the_token_endpoint() {
    // Graph documents `/organization` as unsupported for a personal account:
    // observed as an empty list and as a 500. Either way the token endpoint
    // decides.
    for organization in [
        Reply::json(200, &json!({"value": []})),
        Reply::json(500, &json!({})),
    ] {
        let (h, token_endpoint, token_url) = personal_home("common", token_reply(CONSUMERS, None));
        let f = graph_in(
            me(MSA_ID, "ada@outlook.com"),
            organization,
            Reply::json(200, &children()),
        );
        let e = pinned(&f, &token_url);
        let cap = ms_caps(h.path(), &e).await;
        assert_eq!(cap["available"], json!(true), "{cap}");
        assert_eq!(
            cap["credential"]["principal"],
            json!({"integration": "microsoft-365", "stable_id": MSA_ID, "tenant_id": CONSUMERS})
        );
        let binding = binding_id("microsoft-365", "microsoft-365", MSA_ID, Some(CONSUMERS));
        assert_eq!(cap["credential"]["binding_id"], json!(binding));
        assert_eq!(
            cap["credential"]["credential_generation"],
            json!(GENERATION)
        );
        assert_eq!(token_endpoint.requests().len(), 1);
        // The refresh was stored back to the same slot (same grant).
        let stored = crate::auth::keychain::load_token("microsoft-365", None, h.path())
            .unwrap()
            .unwrap();
        assert_eq!(stored.refresh_token.as_deref(), Some("ms-rotated-764"));
        assert_eq!(stored.generation.as_deref(), Some(GENERATION));
        assert_ms_clean(&cap.to_string());

        // The bound call verifies the same pair again, then reads.
        let mut req = ms_request(h.path(), root_inputs());
        req.insert("bindingId".into(), json!(binding));
        let out = call_with(h.path(), &parsed(&req), &e).await.unwrap();
        assert_eq!(out["state"], json!("completed"));
        assert_eq!(token_endpoint.requests().len(), 2);
    }
}

#[tokio::test]
async fn the_consumer_tenant_is_accepted_only_when_every_source_says_so() {
    let cases = [
        // An Entra account whose `/organization` read failed: its home tenant is
        // a directory, not the consumer tenant AWARE could vouch for.
        ("common", token_reply(OTHER_TID, None), true),
        // A personal account invited as a guest: home is consumers, but the ID
        // token says it signed in to the inviting directory.
        ("common", token_reply(CONSUMERS, Some(OTHER_TID)), true),
        // No `client_info` at all.
        (
            "common",
            json!({"access_token": "x", "expires_in": 3600}),
            true,
        ),
        // A tenant-specific authority is how a guest signs in: the consumer
        // tenant can never be that token's directory, and nothing is asked.
        (OTHER_TID, token_reply(CONSUMERS, None), false),
        ("organizations", token_reply(CONSUMERS, None), false),
    ];
    for (authority, reply, asked) in cases {
        let (h, token_endpoint, token_url) = personal_home(authority, reply.clone());
        let f = graph_in(
            me(MSA_ID, "ada@outlook.com"),
            Reply::json(500, &json!({})),
            Reply::json(200, &children()),
        );
        let cap = ms_caps(h.path(), &pinned(&f, &token_url)).await;
        assert_eq!(
            cap["credential"]["status"],
            json!("unverified"),
            "{authority} {reply}"
        );
        assert_eq!(cap["unavailableCode"], json!("identity-unverified"));
        assert_eq!(
            token_endpoint.requests().len(),
            usize::from(asked),
            "{authority} {reply}"
        );
        assert_ms_clean(&cap.to_string());
    }
}

#[tokio::test]
async fn a_call_whose_tenant_changed_since_capabilities_is_refused_before_data() {
    let h = ms_home();
    ms_store(h.path(), None, MS_SCOPE);
    let before = graph(me(OID, UPN), Reply::json(200, &children()));
    let cap = ms_caps(h.path(), &ms_endpoints(&before)).await;
    assert_eq!(cap["credential"]["principal"]["tenant_id"], json!(TID));
    let mut req = ms_request(h.path(), root_inputs());
    req.insert("bindingId".into(), cap["credential"]["binding_id"].clone());

    // Same oid, same name, same slot generation — another directory now.
    let after = graph_in(me(OID, UPN), org(OTHER_TID), Reply::json(200, &children()));
    let failure = call_with(h.path(), &parsed(&req), &ms_endpoints(&after))
        .await
        .unwrap_err();
    assert_eq!(failure, CallFailure::new("E_BINDING_CHANGED"));
    assert_eq!(failure.exit_code(), 6);
    assert!(
        after
            .requests()
            .iter()
            .all(|head| is_identity_path(&path_of(head))),
        "a data request was sent"
    );

    // An organization read that no longer names a tenant is not a pass either.
    let unknown = graph_in(
        me(OID, UPN),
        Reply::json(503, &json!({})),
        Reply::json(200, &children()),
    );
    let failure = call_with(h.path(), &parsed(&req), &ms_endpoints(&unknown))
        .await
        .unwrap_err();
    assert_eq!(
        failure,
        CallFailure::reason("E_CALL_IDENTITY_FAILED", "tenant")
    );
    assert!(
        unknown
            .requests()
            .iter()
            .all(|head| is_identity_path(&path_of(head)))
    );
}

/// A Graph fixture whose `/me` and `/organization` answers depend on which
/// access token asks: `before` for the slot's original token, `after` for the
/// one the tenant refresh minted (`ms-refreshed-764`).
fn graph_by_token(before: (Json, Reply), after: (Json, Reply)) -> Fixture {
    Fixture::start(Arc::new(move |head: &str| {
        let path = head.split_whitespace().nth(1).unwrap_or("");
        let (who, organization) = if head.contains("Bearer ms-refreshed-764") {
            &after
        } else {
            &before
        };
        if path == "/v1.0/me" || path.starts_with("/v1.0/me?") {
            Reply::json(200, who)
        } else if is_organization(path) {
            organization.clone()
        } else {
            Reply::json(200, &children())
        }
    }))
}

#[tokio::test]
async fn the_tenant_is_always_that_of_the_token_actually_used() {
    let guest = "1b2c3d4e-0000-4000-8000-000000000002";
    let failed = || Reply::json(500, &json!({}));

    // A guest whose first `/organization` read failed: the refreshed token's
    // read names its directory, and that — not the consumer home tenant
    // `client_info` reports — is the tenant.
    let (h, _token_endpoint, token_url) = personal_home("common", token_reply(CONSUMERS, None));
    let f = graph_by_token((me(guest, UPN), failed()), (me(guest, UPN), org(OTHER_TID)));
    let e = pinned(&f, &token_url);
    let cap = ms_caps(h.path(), &e).await;
    assert_eq!(
        cap["credential"]["principal"]["tenant_id"],
        json!(OTHER_TID),
        "{cap}"
    );
    let binding = binding_id("microsoft-365", "microsoft-365", guest, Some(OTHER_TID));
    assert_eq!(cap["credential"]["binding_id"], json!(binding));

    // The bound call reads data with the token whose tenant was verified.
    let (h, _token_endpoint, token_url) = personal_home("common", token_reply(CONSUMERS, None));
    let f = graph_by_token((me(guest, UPN), failed()), (me(guest, UPN), org(OTHER_TID)));
    let e = pinned(&f, &token_url);
    let mut req = ms_request(h.path(), root_inputs());
    req.insert("bindingId".into(), json!(binding));
    call_with(h.path(), &parsed(&req), &e).await.unwrap();
    let data: Vec<String> = f
        .requests()
        .into_iter()
        .filter(|head| !is_identity_path(&path_of(head)))
        .collect();
    assert_eq!(data.len(), 1);
    assert!(data[0].contains("Bearer ms-refreshed-764"), "{}", data[0]);
    assert!(!data[0].contains(&format!("Bearer {MS_TOKEN}")));

    // The refreshed token answering another account: unverified.
    let (h, _token_endpoint, token_url) = personal_home("common", token_reply(CONSUMERS, None));
    let f = graph_by_token(
        (me(MSA_ID, UPN), failed()),
        (me("7e7e7e7e7e7e7e7e", UPN), failed()),
    );
    let cap = ms_caps(h.path(), &pinned(&f, &token_url)).await;
    assert_eq!(
        cap["unavailableCode"],
        json!("identity-unverified"),
        "{cap}"
    );

    // Both reads failing for a directory object (a GUID id): never consumers,
    // however `client_info` reads.
    let (h, _token_endpoint, token_url) = personal_home("common", token_reply(CONSUMERS, None));
    let f = graph_by_token((me(guest, UPN), failed()), (me(guest, UPN), failed()));
    let cap = ms_caps(h.path(), &pinned(&f, &token_url)).await;
    assert_eq!(
        cap["unavailableCode"],
        json!("identity-unverified"),
        "{cap}"
    );
}

#[path = "agent_call_m365_reads_tests.rs"]
mod reads;

#[path = "agent_call_m365_teams_tests.rs"]
mod teams;

#[path = "agent_call_m365_excel_tests.rs"]
mod excel;

#[path = "agent_call_m365_outlook_tests.rs"]
mod outlook;
