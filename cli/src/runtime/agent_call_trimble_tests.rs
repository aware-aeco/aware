//! Trimble Connect in the reviewed, account-bound call (#788). A child of
//! `agent_call_tests`, so it shares that module's fixture. Two local fixtures
//! stand in for a region's Core host (identity, `/regions`, projects, ToDos)
//! and its Topics host (BCF); the production allowlists are used only where a
//! test proves they refuse.

use super::*;

const TC_TOKEN: &str = "tc-secret-token-788";
const USER_ID: &str = "c0ffee00-1111-4222-8333-944455556666";
const USER_EMAIL: &str = "ada@example.com";
const PROJECT: &str = "Pr0ject_ID-1";
const TOPIC: &str = "3f2a1b4c-5d6e-4f70-8a9b-0c1d2e3f4a5b";
const SCOPE: &str = "openid profile TrimbleConnect offline_access";

// ── fixtures ─────────────────────────────────────────────────────────────────

type Routes = Vec<(String, Reply)>;

/// A Trimble region: `core` (identity, `/regions`, projects, ToDos) and
/// `topics` (BCF). Each answers the first route whose `"<METHOD> <path>"`
/// prefix matches the request line, else 404.
struct Tc {
    core: Fixture,
    topics: Fixture,
}

fn route(prefix: &str, reply: Reply) -> (String, Reply) {
    (prefix.to_string(), reply)
}

fn serve(routes: Routes) -> Fixture {
    Fixture::start(Arc::new(move |head: &str| {
        let line = head.lines().next().unwrap_or("");
        routes
            .iter()
            .find(|(prefix, _)| line.starts_with(prefix.as_str()))
            .map(|(_, reply)| reply.clone())
            .unwrap_or_else(|| Reply::json(404, &json!({"message": "must-not-leave"})))
    }))
}

fn me(id: &str) -> Json {
    json!({"id": id, "email": USER_EMAIL, "firstName": "Ada", "lastName": "Lovelace",
           "status": "ACTIVE", "secret": "must-not-leave"})
}

/// `/regions` naming `core` and `topics` as Europe's hosts, as Trimble spells
/// them (a path on the Core URL, the root on the Topics URL).
fn regions(core: &str, topics: &str) -> Json {
    json!([
        {"location": "northAmerica", "tc-api": "https://app.connect.trimble.com/tc/api/2.0/",
         "topic-api": "https://open11.connect.trimble.com/"},
        {"location": "europe", "tc-api": format!("{core}/tc/api/2.0/"),
         "topic-api": format!("{topics}/")}
    ])
}

/// A region whose Core host answers identity, `/regions` and the project check
/// before `core_routes`, and whose Topics host answers `topic_routes`.
fn tc(core_routes: Routes, topic_routes: Routes) -> Tc {
    tc_with(core_routes, topic_routes, |core, topics| {
        regions(core, topics)
    })
}

fn tc_with(
    core_routes: Routes,
    topic_routes: Routes,
    regions_of: impl Fn(&str, &str) -> Json,
) -> Tc {
    let topics = serve(topic_routes);
    // The Core fixture's own origin is known only once it listens, so the
    // `/regions` answer is filled in through a shared cell.
    let regions_body = Arc::new(Mutex::new(Json::Null));
    let mut routes = core_routes;
    routes.push(route(
        "GET /tc/api/2.0/users/me ",
        Reply::json(200, &me(USER_ID)),
    ));
    routes.push(route(
        &format!("GET /tc/api/2.0/projects/{PROJECT} "),
        Reply::json(
            200,
            &json!({"id": PROJECT, "name": "Tower", "location": "europe"}),
        ),
    ));
    let cell = regions_body.clone();
    let core = Fixture::start(Arc::new(move |head: &str| {
        let line = head.lines().next().unwrap_or("");
        if line.starts_with("GET /tc/api/2.0/regions ") {
            return Reply::json(200, &cell.lock().unwrap());
        }
        routes
            .iter()
            .find(|(prefix, _)| line.starts_with(prefix.as_str()))
            .map(|(_, reply)| reply.clone())
            .unwrap_or_else(|| Reply::json(404, &json!({"message": "must-not-leave"})))
    }));
    *regions_body.lock().unwrap() = regions_of(&core.origin(), &topics.origin());
    Tc { core, topics }
}

fn tc_endpoints(t: &Tc) -> Endpoints {
    Endpoints {
        token: TokenPin::Exact(TRIMBLE_TOKEN_URL.to_string()),
        identity_url: format!("{}/tc/api/2.0/users/me", t.core.origin()),
        data_origin: Some(t.core.origin()),
        allowed_origins: vec![t.core.origin()],
        topics_origins: vec![t.topics.origin()],
    }
}

fn with_headers(mut reply: Reply, headers: &str) -> Reply {
    reply.headers = headers.to_string();
    reply
}

// ── homes, tokens, requests ──────────────────────────────────────────────────

fn tc_manifest() -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../20-agents/aeco/construction/trimble-connect/manifest.yaml");
    std::fs::read_to_string(path).unwrap().replace("\r\n", "\n")
}

fn tc_home() -> tempfile::TempDir {
    let home = tempfile::tempdir().unwrap();
    let dir = home.path().join("agents").join("trimble-connect");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("manifest.yaml"), tc_manifest()).unwrap();
    home
}

fn tc_store(home: &Path, alias: Option<&str>, scope: &str) {
    let now = crate::auth::unix_now_secs().unwrap();
    let token = StoredToken {
        access_token: TC_TOKEN.into(),
        refresh_token: None,
        expires_at: now + 3600,
        scope: scope.into(),
        token_type: "Bearer".into(),
        integration: "trimble-connect".into(),
        obtained_at: now,
        generation: Some(GENERATION.to_string()),
        source: crate::auth::keychain::TokenSource::Oauth,
        pkce_verifier: None,
    };
    crate::auth::keychain::store_token(&token, alias, home).unwrap();
}

fn tc_op(command: &str) -> &'static Operation {
    operation("trimble-connect", command).unwrap()
}

fn tc_request(home: &Path, command: &str, inputs: Json) -> Map<String, Json> {
    let digest = sha256_hex(canonical_string(&inputs).as_bytes());
    let sha = crate::runtime::probe::manifest_sha256(
        &std::fs::read(home.join("agents/trimble-connect/manifest.yaml")).unwrap(),
    );
    let Json::Object(map) = json!({
        "schema": REQUEST_SCHEMA,
        "invocationId": "6f1c2b3a-4d5e-5f60-8a1b-2c3d4e5f6a7b",
        "agent": "trimble-connect",
        "command": command,
        "expectedAgentVersion": "0.5.2",
        "expectedManifestSha256": sha,
        "executionTransport": "rest",
        "operationSchemaVersion": 1,
        "expectedOperationSha256": tc_op(command).sha256(),
        "integration": "trimble-connect",
        "alias": null,
        "bindingId": binding_id("trimble-connect", "trimble-connect", USER_ID, None),
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

/// Run `command` against `t` as a person connected with [`SCOPE`].
async fn tc_call(t: &Tc, command: &str, inputs: Json) -> Result<Json, CallFailure> {
    let h = tc_home();
    tc_store(h.path(), None, SCOPE);
    call_with(
        h.path(),
        &parsed(&tc_request(h.path(), command, inputs)),
        &tc_endpoints(t),
    )
    .await
}

fn body(out: Result<Json, CallFailure>) -> Json {
    let record = out.unwrap();
    assert_tc_clean(&record.to_string());
    record["result"]["body"].clone()
}

fn assert_tc_clean(text: &str) {
    for secret in [TC_TOKEN, "must-not-leave"] {
        assert!(!text.contains(secret), "{secret} leaked: {text}");
    }
}

fn line(head: &str) -> &str {
    head.lines().next().unwrap_or("")
}

fn lines(f: &Fixture) -> Vec<String> {
    f.requests().iter().map(|h| line(h).to_string()).collect()
}

fn expect_failure(out: Result<Json, CallFailure>) -> CallFailure {
    let failure = out.unwrap_err();
    assert_tc_clean(&format!("{failure:?}"));
    failure
}

fn phase(failure: &CallFailure) -> Option<&str> {
    failure.details.get("phase").and_then(Json::as_str)
}

fn europe(extra: Json) -> Json {
    let mut inputs = json!({"region": "europe", "project-id": PROJECT});
    if let (Json::Object(base), Json::Object(more)) = (&mut inputs, extra) {
        base.extend(more);
    }
    inputs
}

// ── grammars, digests, table (no network) ────────────────────────────────────

#[test]
fn a_todo_range_is_a_bounded_zero_based_items_range() {
    assert_eq!(trimble::items_range("items=0-99"), Some((0, 99)));
    assert_eq!(trimble::items_range("items=100-199"), Some((100, 199)));
    assert_eq!(trimble::items_range("items=5-5"), Some((5, 5)));
    for bad in [
        "",
        "items=",
        "items=1",
        "items=9-1",
        "items=01-5",
        "items=0-1000",
        "items=-5",
        "bytes=0-99",
        "items=0-99,200-299",
        "items= 0-99",
        "items=0-99 ",
    ] {
        assert_eq!(trimble::items_range(bad), None, "{bad}");
    }
}

#[test]
fn a_content_range_is_parsed_for_its_shape_and_nothing_is_invented() {
    let parse = trimble::content_range;
    assert_eq!(
        parse("items 0-99/240"),
        Some(json!({"unit": "items", "start": 0, "end": 99, "total": 240}))
    );
    assert_eq!(
        parse("items */240"),
        Some(json!({"unit": "items", "start": null, "end": null, "total": 240}))
    );
    assert_eq!(
        parse("items 0-99/*"),
        Some(json!({"unit": "items", "start": 0, "end": 99, "total": null}))
    );
    // Another unit is reported as sent; the caller decides it is not items.
    assert_eq!(
        parse("bytes 0-9/10"),
        Some(json!({"unit": "bytes", "start": 0, "end": 9, "total": 10}))
    );
    for bad in [
        "",
        "items",
        "items 0-99",
        "items 9-1/240",
        "items a-b/1",
        "it3ms 0-1/2",
    ] {
        assert_eq!(parse(bad), None, "{bad}");
    }
}

#[test]
fn trimble_inputs_have_their_own_grammars() {
    assert!(IdKind::TrimbleId.accepts(PROJECT));
    assert!(IdKind::TrimbleId.accepts(TOPIC));
    for bad in [
        "",
        ".",
        "..",
        "a/b",
        "a%2Fb",
        "a b",
        "a.b",
        &"x".repeat(129),
    ] {
        assert!(!IdKind::TrimbleId.accepts(bad), "{bad}");
    }
    for region in trimble::REGIONS {
        assert!(IdKind::TrimbleRegion.accepts(region), "{region}");
    }
    for bad in [
        "Europe",
        "mars",
        "",
        "europe/../x",
        "app21.connect.trimble.com",
    ] {
        assert!(!IdKind::TrimbleRegion.accepts(bad), "{bad}");
    }
    assert!(IdKind::TrimbleSkiptoken.accepts("abc=="));
    for bad in ["", "a b", "a\nb", &"t".repeat(2049)] {
        assert!(!IdKind::TrimbleSkiptoken.accepts(bad), "{bad}");
    }
}

#[test]
fn the_trimble_reads_are_bound_call_operations_only() {
    for command in [
        "list-projects",
        "list-topics",
        "get-topic",
        "list-comments",
        "list-todos",
    ] {
        let op = tc_op(command);
        assert_eq!(op.provider, Provider::Trimble, "{command}");
        // The workflow and `agent invoke` paths keep their own handlers.
        assert!(!handles_workflow("trimble-connect", command), "{command}");
    }
    // Writes, downloads and every other read stay out of the bound call.
    for command in [
        "create-topic",
        "update-topic",
        "delete-topic",
        "create-comment",
        "create-todo",
        "download",
        "download-bcf-document",
        "get-viewpoint-snapshot",
        "get-current-user",
    ] {
        assert!(operation("trimble-connect", command).is_none(), "{command}");
    }
}

#[test]
fn the_trimble_digests_pin_hosts_regions_routes_and_projection() {
    let mut digests = std::collections::BTreeSet::new();
    for command in [
        "list-projects",
        "list-topics",
        "get-topic",
        "list-comments",
        "list-todos",
    ] {
        let op = tc_op(command);
        let canonical = canonical_string(&op.canonical());
        for needle in [
            TRIMBLE_ME_URL,
            "open21.connect.trimble.com",
            "app32.connect.trimble.com",
            "unitedKingdom",
            "\"redirects\":\"refused\"",
            "\"tenant\":null",
            op.projection.id(),
        ] {
            assert!(canonical.contains(needle), "{command}: {needle}");
        }
        assert!(digests.insert(op.sha256()), "{command} shares a digest");
    }
    assert!(canonical_string(&tc_op("list-todos").canonical()).contains("[200,204,206,416]"));
    assert!(canonical_string(&tc_op("get-topic").canonical()).contains("projectCheck"));
    assert!(
        canonical_string(&tc_op("list-projects").canonical()).contains("\"projectCheck\":null")
    );
}

#[test]
fn production_sends_the_token_to_core_and_topics_hosts_only_in_their_roles() {
    let e = Endpoints::production(tc_op("list-topics"));
    let hosts = |list: &[String]| -> Vec<String> {
        list.iter()
            .map(|o| o.trim_start_matches("https://").to_string())
            .collect()
    };
    assert_eq!(
        hosts(&e.allowed_origins),
        crate::runtime::trimble_region::CORE_HOSTS
    );
    assert_eq!(
        hosts(&e.topics_origins),
        crate::runtime::trimble_bcf::TOPICS_HOSTS
    );
    assert_eq!(e.identity_url, TRIMBLE_ME_URL);
    assert!(e.check_origins(tc_op("list-topics")).is_ok());
    assert!(matches!(&e.token, TokenPin::Exact(url) if url == TRIMBLE_TOKEN_URL));
    // Every other provider keeps an empty Topics list.
    assert!(Endpoints::production(op()).topics_origins.is_empty());
}

#[test]
fn the_shipped_manifest_offers_exactly_the_five_bound_reads() {
    let agent: Agent = serde_yaml::from_str(&tc_manifest()).unwrap();
    for op in OPERATIONS
        .iter()
        .filter(|op| op.provider == Provider::Trimble)
    {
        assert!(command_runnable(&agent, op), "{}", op.command);
        assert!(manifest_slot(&agent, op, false).is_ok(), "{}", op.command);
    }
}

// ── identity ─────────────────────────────────────────────────────────────────

#[tokio::test]
async fn capabilities_verify_the_trimble_user_with_a_tenantless_principal() {
    let t = tc(vec![], vec![]);
    let h = tc_home();
    tc_store(h.path(), None, SCOPE);
    let cap = capabilities_with(
        h.path(),
        tc_op("list-topics"),
        None,
        Duration::from_secs(10),
        &tc_endpoints(&t),
    )
    .await
    .unwrap();
    assert_tc_clean(&cap.to_string());
    let credential = &cap["credential"];
    assert_eq!(credential["status"], "verified");
    assert_eq!(credential["integration"], "trimble-connect");
    assert_eq!(credential["credential_generation"], GENERATION);
    assert_eq!(credential["revision"], 1);
    assert_eq!(credential["presentation_label"], USER_EMAIL);
    // The convention: `tenant_id` is present and null; the principal names its
    // namespace; the binding is over (integration, slot, user id) alone.
    let principal = credential["principal"].as_object().unwrap();
    assert_eq!(
        Json::Object(principal.clone()),
        json!({"integration": "trimble-connect", "stable_id": USER_ID, "tenant_id": null,
               "namespace": "trimble-connect-user"})
    );
    assert_eq!(
        credential["binding_id"],
        binding_id("trimble-connect", "trimble-connect", USER_ID, None)
    );
    assert_eq!(cap["available"], true, "{cap}");
    assert_eq!(cap["regions"], json!(trimble::REGIONS));
    assert_eq!(cap["operationSha256"], tc_op("list-topics").sha256());
    // Exactly one request: `/users/me` with the slot's token.
    let seen = t.core.requests();
    assert_eq!(seen.len(), 1, "{seen:?}");
    assert!(line(&seen[0]).starts_with("GET /tc/api/2.0/users/me "));
    assert!(seen[0].contains(&format!("Bearer {TC_TOKEN}")));
    assert!(t.topics.requests().is_empty());
}

#[tokio::test]
async fn the_label_falls_back_to_the_name_and_the_binding_never_uses_it() {
    let topics = serve(vec![]);
    let core = serve(vec![route(
        "GET /tc/api/2.0/users/me ",
        Reply::json(
            200,
            &json!({"id": USER_ID, "firstName": "Ada", "lastName": "Lovelace"}),
        ),
    )]);
    let t = Tc { core, topics };
    let h = tc_home();
    tc_store(h.path(), None, SCOPE);
    let cap = capabilities_with(
        h.path(),
        tc_op("get-topic"),
        None,
        Duration::from_secs(10),
        &tc_endpoints(&t),
    )
    .await
    .unwrap();
    assert_eq!(cap["credential"]["presentation_label"], "Ada Lovelace");
    assert_eq!(
        cap["credential"]["binding_id"],
        binding_id("trimble-connect", "trimble-connect", USER_ID, None)
    );
}

#[tokio::test]
async fn a_slot_without_the_trimble_connect_scope_is_unavailable_and_sends_nothing_more() {
    let t = tc(vec![], vec![]);
    let h = tc_home();
    tc_store(h.path(), None, "openid profile");
    let cap = capabilities_with(
        h.path(),
        tc_op("list-todos"),
        None,
        Duration::from_secs(10),
        &tc_endpoints(&t),
    )
    .await
    .unwrap();
    assert_eq!(cap["available"], false);
    assert_eq!(cap["unavailableCode"], "scope-missing");
    let reason = cap["unavailableReason"].as_str().unwrap();
    assert!(reason.contains("Trimble Connect"), "{reason}");
    assert!(
        reason.contains("--scopes openid,TrimbleConnect,offline_access"),
        "{reason}"
    );
}

#[tokio::test]
async fn a_call_bound_to_another_trimble_user_is_refused_before_any_data_request() {
    let t = tc(vec![], vec![]);
    let h = tc_home();
    tc_store(h.path(), None, SCOPE);
    let mut request = tc_request(h.path(), "list-topics", europe(json!({})));
    request.insert(
        "bindingId".into(),
        json!(binding_id(
            "trimble-connect",
            "trimble-connect",
            "someone-else",
            None
        )),
    );
    let out = call_with(h.path(), &parsed(&request), &tc_endpoints(&t)).await;
    assert_eq!(expect_failure(out).code, "E_BINDING_CHANGED");
    assert_eq!(lines(&t.core), ["GET /tc/api/2.0/users/me HTTP/1.1"]);
    assert!(t.topics.requests().is_empty());
}

#[tokio::test]
async fn an_unlisted_region_is_refused_before_the_credential_or_any_request() {
    let t = tc(vec![], vec![]);
    let out = tc_call(
        &t,
        "list-topics",
        json!({"region": "mars", "project-id": PROJECT}),
    )
    .await;
    let failure = expect_failure(out);
    assert_eq!(failure.code, "E_CALL_INPUT_INVALID");
    assert_eq!(failure.details["input"], "region");
    assert!(t.core.requests().is_empty() && t.topics.requests().is_empty());
}

// ── list-projects ────────────────────────────────────────────────────────────

#[tokio::test]
async fn list_projects_reads_one_region_and_offers_the_next_skip() {
    let projects = json!([
        {"id": "P-1", "name": "Tower", "location": "europe", "rootId": "must-not-leave"},
        {"name": "no id"},
        {"id": "P-3", "name": null, "location": 21}
    ]);
    let t = tc(
        vec![route(
            "GET /tc/api/2.0/projects?",
            Reply::json(200, &projects),
        )],
        vec![],
    );
    let out = body(
        tc_call(
            &t,
            "list-projects",
            json!({"region": "europe", "top": 3, "skip": 6}),
        )
        .await,
    );
    assert_eq!(
        out,
        json!({
            "region": "europe",
            "projects": [
                {"id": "P-1", "name": "Tower", "location": "europe"},
                {"id": "P-3", "name": null, "withheld": ["location"]}
            ],
            // A full page may be followed by more; every returned entry counts.
            "more-available": true,
            "next-skip": 9,
            "omitted": 1,
            "regions": trimble::REGIONS,
        })
    );
    let seen = t.core.requests();
    let order: Vec<&str> = seen.iter().map(|h| line(h)).collect();
    assert!(
        order[1].starts_with("GET /tc/api/2.0/regions "),
        "{order:?}"
    );
    assert!(!seen[1].contains("Authorization"), "regions is public");
    assert_eq!(order[2], "GET /tc/api/2.0/projects?top=3&skip=6 HTTP/1.1");
    assert!(seen[2].contains(&format!("Bearer {TC_TOKEN}")));
    // No project check: there is no project yet.
    assert_eq!(seen.len(), 3, "{order:?}");
}

#[tokio::test]
async fn a_short_project_page_is_the_last() {
    let t = tc(
        vec![route(
            "GET /tc/api/2.0/projects?",
            Reply::json(
                200,
                &json!([{"id": "P-1", "name": "Tower", "location": "europe"}]),
            ),
        )],
        vec![],
    );
    let out = body(tc_call(&t, "list-projects", json!({"region": "europe"})).await);
    assert_eq!(out["more-available"], false);
    assert_eq!(out["next-skip"], Json::Null);
    assert_eq!(out["omitted"], 0);
}

// ── list-topics ──────────────────────────────────────────────────────────────

fn topics_page() -> Json {
    json!([
        {"guid": "T-1", "title": "Clash at B4", "topic_type": "Clash", "topic_status": "Open",
         "priority": "High", "assigned_to": "bob@example.com", "due_date": "2026-11-30",
         "labels": ["Steel", "L2"], "creation_date": "2026-10-01T10:00:00+00:00",
         "modified_date": "2026-11-02T08:30:00.123+01:00", "description": "listed too",
         "bim_snippet": "must-not-leave"},
        // Cleared fields are null; fields Trimble did not send stay absent.
        {"guid": "T-2", "title": "Opening", "assigned_to": null, "labels": null},
        {"title": "no guid is omitted"},
        {"guid": "", "title": "an empty guid is omitted"},
        {"guid": "T-5", "labels": ["ok", 7], "modified_date": 20261102}
    ])
}

#[tokio::test]
async fn list_topics_checks_the_project_then_projects_every_listed_field() {
    let t = tc(
        vec![],
        vec![route(
            &format!("GET /bcf/2.1/projects/{PROJECT}/topics?"),
            with_headers(
                Reply::json(200, &topics_page()),
                &format!(
                    "next: {}/bcf/2.1/projects/{PROJECT}/topics?top=2&skiptoken=PAGE2%3D%3D\r\n",
                    "https://open21.connect.trimble.com"
                ),
            ),
        )],
    );
    let out = body(tc_call(&t, "list-topics", europe(json!({"top": 2}))).await);
    assert_eq!(
        out,
        json!({
            "region": "europe",
            "items": [
                {"guid": "T-1", "title": "Clash at B4", "topic_type": "Clash",
                 "topic_status": "Open", "priority": "High", "assigned_to": "bob@example.com",
                 "due_date": "2026-11-30", "labels": ["Steel", "L2"],
                 "creation_date": "2026-10-01T10:00:00+00:00",
                 "modified_date": "2026-11-02T08:30:00.123+01:00"},
                {"guid": "T-2", "title": "Opening", "assigned_to": null, "labels": null},
                {"guid": "T-5", "withheld": ["labels", "modified_date"]}
            ],
            "next-skiptoken": "PAGE2==",
            "more-available": true,
            "omitted": 2,
        })
    );
    // Identity, regions and the project check on Core, then the page on Topics.
    let core: Vec<String> = lines(&t.core);
    assert_eq!(core.len(), 3, "{core:?}");
    assert!(core[2].starts_with(&format!("GET /tc/api/2.0/projects/{PROJECT} ")));
    let topics = t.topics.requests();
    assert_eq!(topics.len(), 1);
    assert_eq!(
        line(&topics[0]),
        format!("GET /bcf/2.1/projects/{PROJECT}/topics?top=2 HTTP/1.1")
    );
    assert!(topics[0].contains(&format!("Bearer {TC_TOKEN}")));
}

#[tokio::test]
async fn the_last_topic_page_has_no_next_skiptoken_and_a_204_is_empty() {
    let t = tc(
        vec![],
        vec![route(
            &format!("GET /bcf/2.1/projects/{PROJECT}/topics?"),
            Reply::json(204, &Json::Null),
        )],
    );
    let out = body(tc_call(&t, "list-topics", europe(json!({"skiptoken": "PAGE2=="}))).await);
    assert_eq!(
        out,
        json!({"region": "europe", "items": [], "next-skiptoken": null,
               "more-available": false, "omitted": 0})
    );
    let head = &t.topics.requests()[0];
    assert!(line(head).contains("skiptoken=PAGE2%3D%3D"), "{head}");
}

#[tokio::test]
async fn a_next_link_without_a_skiptoken_fails_the_page_instead_of_ending_it() {
    let t = tc(
        vec![],
        vec![route(
            &format!("GET /bcf/2.1/projects/{PROJECT}/topics?"),
            with_headers(
                Reply::json(200, &json!([])),
                "next: https://open21.connect.trimble.com/elsewhere\r\n",
            ),
        )],
    );
    let failure = expect_failure(tc_call(&t, "list-topics", europe(json!({}))).await);
    assert_eq!(failure.code, "E_CALL_FAILED");
    assert_eq!(failure.reason_str(), Some("next-link-unsupported"));
    assert_eq!(phase(&failure), Some("item"));
}

// ── get-topic ────────────────────────────────────────────────────────────────

#[tokio::test]
async fn get_topic_projects_the_id_every_listed_field_and_the_description() {
    let topic = json!({
        "guid": TOPIC, "title": "Clash at B4", "topic_type": "Clash",
        "topic_status": "Closed", "priority": "High",
        // Cleared since the listing: null, never dropped.
        "assigned_to": null, "due_date": null,
        "labels": [], "creation_date": "2026-10-01T10:00:00+00:00",
        "modified_date": "2026-11-03T09:00:00+00:00",
        "description": "Beam B4 clashes with duct D7.\nSee level 2.",
        "viewpoints": "must-not-leave"
    });
    let t = tc(
        vec![],
        vec![route(
            &format!("GET /bcf/2.1/projects/{PROJECT}/topics/{TOPIC} "),
            Reply::json(200, &topic),
        )],
    );
    let out = body(tc_call(&t, "get-topic", europe(json!({"topic-id": TOPIC}))).await);
    assert_eq!(
        out,
        json!({
            "region": "europe",
            "topic": {
                "guid": TOPIC, "title": "Clash at B4", "topic_type": "Clash",
                "topic_status": "Closed", "priority": "High", "assigned_to": null,
                "due_date": null, "labels": [],
                "creation_date": "2026-10-01T10:00:00+00:00",
                "modified_date": "2026-11-03T09:00:00+00:00",
                "description": "Beam B4 clashes with duct D7.\nSee level 2."
            }
        })
    );
}

#[tokio::test]
async fn get_topic_keeps_absent_fields_absent_and_an_absent_description_absent() {
    let t = tc(
        vec![],
        vec![route(
            &format!("GET /bcf/2.1/projects/{PROJECT}/topics/{TOPIC} "),
            Reply::json(200, &json!({"guid": TOPIC.to_uppercase(), "title": "t"})),
        )],
    );
    let out = body(tc_call(&t, "get-topic", europe(json!({"topic-id": TOPIC}))).await);
    // A guid differing only in case is the same topic, kept as Trimble sent it.
    assert_eq!(
        out["topic"],
        json!({"guid": TOPIC.to_uppercase(), "title": "t"})
    );
}

#[tokio::test]
async fn get_topic_answered_with_another_topic_is_refused() {
    let t = tc(
        vec![],
        vec![route(
            &format!("GET /bcf/2.1/projects/{PROJECT}/topics/{TOPIC} "),
            Reply::json(200, &json!({"guid": "another", "title": "t"})),
        )],
    );
    let failure =
        expect_failure(tc_call(&t, "get-topic", europe(json!({"topic-id": TOPIC}))).await);
    assert_eq!(failure.reason_str(), Some("topic-mismatch"));
    assert_eq!(phase(&failure), Some("item"));
}

// ── list-comments ────────────────────────────────────────────────────────────

#[tokio::test]
async fn list_comments_passes_a_modified_date_through_only_when_sent() {
    let comments = json!([
        {"guid": "C-1", "comment": "First\nline", "author": "ada@example.com",
         "date": "2026-11-01T10:00:00+00:00", "viewpoint_guid": "V-1",
         "reply_to_comment_guid": null, "topic_guid": "must-not-leave"},
        {"guid": "C-2", "comment": "Edited", "author": "bob@example.com",
         "date": "2026-11-01T11:00:00+00:00", "modified_date": "2026-11-02T09:00:00+00:00",
         "reply_to_comment_guid": "C-1"},
        {"comment": "no guid"}
    ]);
    let t = tc(
        vec![],
        vec![route(
            &format!("GET /bcf/2.1/projects/{PROJECT}/topics/{TOPIC}/comments?"),
            with_headers(Reply::json(200, &comments), "next: ?skiptoken=C2&top=2\r\n"),
        )],
    );
    let out = body(
        tc_call(
            &t,
            "list-comments",
            europe(json!({"topic-id": TOPIC, "top": 2})),
        )
        .await,
    );
    assert_eq!(
        out,
        json!({
            "region": "europe",
            "items": [
                {"guid": "C-1", "comment": "First\nline", "author": "ada@example.com",
                 "date": "2026-11-01T10:00:00+00:00", "viewpoint_guid": "V-1",
                 "reply_to_comment_guid": null},
                {"guid": "C-2", "comment": "Edited", "author": "bob@example.com",
                 "date": "2026-11-01T11:00:00+00:00",
                 "modified_date": "2026-11-02T09:00:00+00:00",
                 "reply_to_comment_guid": "C-1"}
            ],
            "next-skiptoken": "C2",
            "more-available": true,
            "omitted": 1,
        })
    );
}

// ── list-todos ───────────────────────────────────────────────────────────────

fn todos() -> Json {
    json!([
        {"id": "TD-1", "label": "TD-1", "title": "Fix anchor", "description": "Bolts",
         "status": "IN_PROGRESS", "priority": "HIGH", "dueDate": "2026-12-01T00:00:00+0000",
         "percentComplete": 40,
         "assignees": [{"id": "U-1", "type": "USER", "name": "Ada", "email": "must-not-leave"}],
         "modifiedOn": "2026-11-05T10:00:00+0000", "createdOn": "2026-11-01T10:00:00+0000"},
        // No revision sent: none is invented.
        {"id": "TD-2", "title": "Paint", "dueDate": null, "assignees": []},
        {"title": "no id"}
    ])
}

fn todo_route(reply: Reply) -> Vec<(String, Reply)> {
    vec![route("GET /tc/api/2.0/todos?", reply)]
}

#[tokio::test]
async fn a_206_todo_page_carries_its_status_range_and_revisions() {
    let t = tc(
        todo_route(with_headers(
            Reply::json(206, &todos()),
            "Content-Range: items 100-102/240\r\n",
        )),
        vec![],
    );
    let out = body(tc_call(&t, "list-todos", europe(json!({"range": "items=100-199"}))).await);
    assert_eq!(
        out,
        json!({
            "region": "europe",
            "status": 206,
            "partial": true,
            "range": {"unit": "items", "start": 100, "end": 102, "total": 240},
            "items": [
                {"id": "TD-1", "label": "TD-1", "title": "Fix anchor", "description": "Bolts",
                 "status": "IN_PROGRESS", "priority": "HIGH",
                 "dueDate": "2026-12-01T00:00:00+0000", "percentComplete": 40,
                 "assignees": [{"id": "U-1", "type": "USER", "name": "Ada"}],
                 "createdOn": "2026-11-01T10:00:00+0000",
                 "modifiedOn": "2026-11-05T10:00:00+0000"},
                {"id": "TD-2", "title": "Paint", "dueDate": null, "assignees": []}
            ],
            "omitted": 1,
        })
    );
    let core = t.core.requests();
    let todo = core.last().unwrap();
    assert_eq!(
        line(todo),
        format!("GET /tc/api/2.0/todos?projectId={PROJECT} HTTP/1.1")
    );
    assert!(
        todo.to_ascii_lowercase().contains("range: items=100-199"),
        "{todo}"
    );
    assert!(t.topics.requests().is_empty(), "ToDos are Core only");
}

#[tokio::test]
async fn a_416_is_data_carrying_its_content_range() {
    let t = tc(
        todo_route(with_headers(
            Reply::json(416, &json!({"message": "must-not-leave"})),
            "Content-Range: items */240\r\n",
        )),
        vec![],
    );
    let out = body(tc_call(&t, "list-todos", europe(json!({"range": "items=300-399"}))).await);
    assert_eq!(
        out,
        json!({
            "region": "europe",
            "status": 416,
            "partial": false,
            "range": {"unit": "items", "start": null, "end": null, "total": 240},
            "items": [],
            "omitted": 0,
        })
    );
}

#[tokio::test]
async fn a_200_and_a_204_todo_answer_carry_their_status_and_no_invented_range() {
    let t = tc(todo_route(Reply::json(200, &todos())), vec![]);
    let out = body(tc_call(&t, "list-todos", europe(json!({}))).await);
    assert_eq!(out["status"], 200);
    assert_eq!(out["partial"], false);
    assert_eq!(out["range"], Json::Null);
    assert!(out.get("content-range-unparsed").is_none());
    assert_eq!(out["items"].as_array().unwrap().len(), 2);
    // No `range` input: no `Range` header.
    let todo = t.core.requests().last().unwrap().to_ascii_lowercase();
    assert!(!todo.contains("range:"), "{todo}");

    let t = tc(
        todo_route(with_headers(
            Reply {
                status: 204,
                headers: String::new(),
                body: Vec::new(),
                delay: Duration::ZERO,
            },
            "Content-Range: items */0\r\n",
        )),
        vec![],
    );
    let out = body(tc_call(&t, "list-todos", europe(json!({"range": "items=0-99"}))).await);
    assert_eq!(out["status"], 204);
    assert_eq!(out["items"], json!([]));
    assert_eq!(
        out["range"],
        json!({"unit": "items", "start": null, "end": null, "total": 0})
    );
}

#[tokio::test]
async fn a_content_range_in_another_shape_is_flagged_never_guessed() {
    let t = tc(
        todo_route(with_headers(
            Reply::json(206, &todos()),
            "Content-Range: items 0-99\r\n",
        )),
        vec![],
    );
    let out = body(tc_call(&t, "list-todos", europe(json!({"range": "items=0-99"}))).await);
    assert_eq!(out["range"], Json::Null);
    assert_eq!(out["content-range-unparsed"], true);
}

// ── phase-tagged failures ────────────────────────────────────────────────────

#[tokio::test]
async fn a_project_this_account_cannot_open_fails_in_the_project_phase() {
    // The region's Core host does not know the project: the item endpoint is
    // never asked.
    let t = tc_with(
        vec![route(
            &format!("GET /tc/api/2.0/projects/{PROJECT} "),
            Reply::json(404, &json!({"message": "must-not-leave"})),
        )],
        vec![],
        regions,
    );
    let failure =
        expect_failure(tc_call(&t, "get-topic", europe(json!({"topic-id": TOPIC}))).await);
    assert_eq!(failure.code, "E_CALL_FAILED");
    assert_eq!(failure.details["status"], 404);
    assert_eq!(phase(&failure), Some("project"));
    assert!(t.topics.requests().is_empty());
}

#[tokio::test]
async fn a_topic_that_is_not_found_fails_in_the_item_phase() {
    let t = tc(vec![], vec![]);
    let failure =
        expect_failure(tc_call(&t, "get-topic", europe(json!({"topic-id": TOPIC}))).await);
    assert_eq!(failure.code, "E_CALL_FAILED");
    assert_eq!(failure.details["status"], 404);
    assert_eq!(phase(&failure), Some("item"));
    assert_eq!(t.topics.requests().len(), 1);
}

#[tokio::test]
async fn a_regions_lookup_that_fails_or_does_not_know_the_region_fails_in_the_region_phase() {
    let t = tc_with(vec![], vec![], |_, _| json!([{"location": "asia"}]));
    let failure = expect_failure(tc_call(&t, "list-todos", europe(json!({}))).await);
    assert_eq!(failure.reason_str(), Some("region-unknown"));
    assert_eq!(phase(&failure), Some("region"));

    let t = tc_with(vec![], vec![], |_, _| json!({"not": "a list"}));
    let failure = expect_failure(tc_call(&t, "list-todos", europe(json!({}))).await);
    assert_eq!(failure.reason_str(), Some("response-shape"));
    assert_eq!(phase(&failure), Some("region"));
    // Nothing but identity and the lookup was asked.
    assert_eq!(t.core.requests().len(), 2);
}

#[tokio::test]
async fn an_oversized_answer_names_the_bound_and_the_phase() {
    let long = "x".repeat(4000);
    let t = tc(
        vec![],
        vec![route(
            &format!("GET /bcf/2.1/projects/{PROJECT}/topics/{TOPIC} "),
            Reply::json(200, &json!({"guid": TOPIC, "description": long})),
        )],
    );
    let h = tc_home();
    tc_store(h.path(), None, SCOPE);
    let mut request = tc_request(h.path(), "get-topic", europe(json!({"topic-id": TOPIC})));
    request.insert("maxOutputBytes".into(), json!(1024));
    let failure = expect_failure(call_with(h.path(), &parsed(&request), &tc_endpoints(&t)).await);
    assert_eq!(failure.code, "E_CALL_OUTPUT_TOO_LARGE");
    assert_eq!(failure.reason_str(), Some("output-too-large"));
    assert_eq!(failure.details["limitBytes"], 1024);
    assert_eq!(phase(&failure), Some("item"));
}

// ── hosts and redirects ──────────────────────────────────────────────────────

#[tokio::test]
async fn a_topics_api_outside_the_topics_allowlist_never_gets_the_token() {
    // `/regions` names the Core host as Europe's Topics API: a Core host is
    // not a Topics host, whatever it answers.
    let t = tc_with(vec![], vec![], |core, _| {
        json!([{"location": "europe", "tc-api": format!("{core}/tc/api/2.0/"),
                "topic-api": format!("{core}/")}])
    });
    let failure = expect_failure(tc_call(&t, "list-topics", europe(json!({}))).await);
    assert_eq!(failure.code, "E_CALL_ORIGIN_NOT_ALLOWED");
    assert_eq!(phase(&failure), Some("region"));
    // Neither the project check nor any BCF request was made.
    assert_eq!(t.core.requests().len(), 2);
    assert!(t.topics.requests().is_empty());

    // A host outside every allowlist, and a Core URL on another path.
    for (tc_api, topic_api) in [
        (
            "https://evil.example/tc/api/2.0/",
            "https://open21.connect.trimble.com/",
        ),
        ("{core}/other/", "{topics}/"),
        ("{core}/tc/api/2.0/?x=1", "{topics}/"),
        ("{core}/tc/api/2.0/", "{topics}/bcf/"),
    ] {
        let t = tc_with(vec![], vec![], move |core, topics| {
            json!([{"location": "europe",
                    "tc-api": tc_api.replace("{core}", core).replace("{topics}", topics),
                    "topic-api": topic_api.replace("{core}", core).replace("{topics}", topics)}])
        });
        let failure = expect_failure(tc_call(&t, "list-topics", europe(json!({}))).await);
        assert_eq!(
            failure.code, "E_CALL_ORIGIN_NOT_ALLOWED",
            "{tc_api} {topic_api}"
        );
        assert!(t.topics.requests().is_empty(), "{tc_api} {topic_api}");
        assert_eq!(t.core.requests().len(), 2, "{tc_api} {topic_api}");
    }
}

#[tokio::test]
async fn the_production_allowlist_refuses_a_loopback_host() {
    let t = tc(vec![], vec![]);
    let mut e = Endpoints::production(tc_op("list-topics"));
    e.identity_url = format!("{}/tc/api/2.0/users/me", t.core.origin());
    let h = tc_home();
    tc_store(h.path(), None, SCOPE);
    let failure = expect_failure(
        call_with(
            h.path(),
            &parsed(&tc_request(h.path(), "list-topics", europe(json!({})))),
            &e,
        )
        .await,
    );
    assert_eq!(failure.code, "E_CALL_ORIGIN_NOT_ALLOWED");
    assert!(t.core.requests().is_empty(), "checked before any request");
}

#[tokio::test]
async fn a_redirect_is_refused_and_never_followed() {
    // The Topics host answers the page with a redirect to a path it would
    // also serve: the call must fail and that path never be asked.
    let t = tc(
        vec![],
        vec![
            route(
                &format!("GET /bcf/2.1/projects/{PROJECT}/topics?"),
                with_headers(Reply::json(302, &json!({})), "Location: /stolen\r\n"),
            ),
            route("GET /stolen", Reply::json(200, &json!([]))),
        ],
    );
    let failure = expect_failure(tc_call(&t, "list-topics", europe(json!({}))).await);
    assert_eq!(failure.code, "E_CALL_FAILED");
    assert_eq!(failure.reason_str(), Some("redirect-refused"));
    assert_eq!(failure.details["status"], 302);
    assert_eq!(phase(&failure), Some("item"));
    let asked = lines(&t.topics);
    assert_eq!(asked.len(), 1, "followed: {asked:?}");
    assert!(
        !asked.iter().any(|l| l.starts_with("GET /stolen")),
        "{asked:?}"
    );
}

// ── the slot is rechecked before every request that carries the token ────────

/// What happens to the slot while one Trimble request is pending.
#[derive(Clone, Copy)]
enum SlotChange {
    /// Another grant replaces this one (a reconnect).
    Replaced,
    /// The slot is emptied (a disconnect).
    Disconnected,
}

/// A region whose Core host changes the slot while it answers the request
/// starting with `during` (the `/regions` lookup or the project check), then
/// answers as usual. The Topics host answers the topic page.
fn tc_changing(home: &Path, during: String, change: SlotChange) -> Tc {
    let topics = serve(vec![route(
        &format!("GET /bcf/2.1/projects/{PROJECT}/topics?"),
        Reply::json(200, &json!([])),
    )]);
    let regions_body = Arc::new(Mutex::new(Json::Null));
    let cell = regions_body.clone();
    let home = home.to_path_buf();
    let core = Fixture::start(Arc::new(move |head: &str| {
        let line = head.lines().next().unwrap_or("");
        if line.starts_with(during.as_str()) {
            match change {
                SlotChange::Replaced => {
                    let now = crate::auth::unix_now_secs().unwrap();
                    let other = StoredToken {
                        access_token: "tc-reconnected-token".into(),
                        refresh_token: None,
                        expires_at: now + 3600,
                        scope: SCOPE.into(),
                        token_type: "Bearer".into(),
                        integration: "trimble-connect".into(),
                        obtained_at: now,
                        generation: Some("99999999-2222-4333-8444-555555555555".into()),
                        source: crate::auth::keychain::TokenSource::Oauth,
                        pkce_verifier: None,
                    };
                    crate::auth::keychain::store_token(&other, None, &home).unwrap();
                }
                SlotChange::Disconnected => {
                    let _ = crate::auth::keychain::delete_token("trimble-connect", None, &home);
                }
            }
        }
        if line.starts_with("GET /tc/api/2.0/regions ") {
            return Reply::json(200, &cell.lock().unwrap());
        }
        if line.starts_with("GET /tc/api/2.0/users/me ") {
            return Reply::json(200, &me(USER_ID));
        }
        if line.starts_with(&format!("GET /tc/api/2.0/projects/{PROJECT} ")) {
            return Reply::json(
                200,
                &json!({"id": PROJECT, "name": "Tower", "location": "europe"}),
            );
        }
        if line.starts_with("GET /tc/api/2.0/todos?") {
            return Reply::json(200, &json!([]));
        }
        Reply::json(404, &json!({"message": "must-not-leave"}))
    }));
    *regions_body.lock().unwrap() = regions(&core.origin(), &topics.origin());
    Tc { core, topics }
}

async fn tc_call_in(h: &Path, t: &Tc, command: &str, inputs: Json) -> Result<Json, CallFailure> {
    call_with(
        h,
        &parsed(&tc_request(h, command, inputs)),
        &tc_endpoints(t),
    )
    .await
}

fn project_check() -> String {
    format!("GET /tc/api/2.0/projects/{PROJECT} ")
}

#[tokio::test]
async fn a_slot_replaced_during_the_region_lookup_sends_the_token_nowhere_after() {
    // The region lookup carries no token and can take as long as Trimble
    // takes. A reconnect or disconnect landing in it must stop the project
    // check and the topic read, so the earlier grant is never used again.
    for change in [SlotChange::Replaced, SlotChange::Disconnected] {
        let h = tc_home();
        tc_store(h.path(), None, SCOPE);
        let t = tc_changing(h.path(), "GET /tc/api/2.0/regions ".into(), change);
        let failure =
            expect_failure(tc_call_in(h.path(), &t, "list-topics", europe(json!({}))).await);
        assert_eq!(
            failure,
            CallFailure::reason("E_CREDENTIAL_CHANGED", "replaced")
        );
        assert_eq!(t.core.count("/tc/api/2.0/regions"), 1);
        assert_eq!(t.core.count(&format!("/tc/api/2.0/projects/{PROJECT}")), 0);
        assert!(t.topics.requests().is_empty(), "{:?}", lines(&t.topics));
    }
}

#[tokio::test]
async fn a_slot_replaced_during_the_project_check_stops_the_item_read() {
    // Both item hosts: the BCF read on the Topics host and the ToDo read on
    // the Core host itself.
    for change in [SlotChange::Replaced, SlotChange::Disconnected] {
        let h = tc_home();
        tc_store(h.path(), None, SCOPE);
        let t = tc_changing(h.path(), project_check(), change);
        let failure =
            expect_failure(tc_call_in(h.path(), &t, "list-topics", europe(json!({}))).await);
        assert_eq!(
            failure,
            CallFailure::reason("E_CREDENTIAL_CHANGED", "replaced")
        );
        assert_eq!(t.core.count(&format!("/tc/api/2.0/projects/{PROJECT}")), 1);
        assert!(t.topics.requests().is_empty(), "{:?}", lines(&t.topics));

        let h = tc_home();
        tc_store(h.path(), None, SCOPE);
        let t = tc_changing(h.path(), project_check(), change);
        let failure =
            expect_failure(tc_call_in(h.path(), &t, "list-todos", europe(json!({}))).await);
        assert_eq!(
            failure,
            CallFailure::reason("E_CREDENTIAL_CHANGED", "replaced")
        );
        assert_eq!(t.core.count(&format!("/tc/api/2.0/projects/{PROJECT}")), 1);
        assert_eq!(t.core.count("/tc/api/2.0/todos"), 0);
    }
}

#[tokio::test]
async fn an_unchanged_slot_reads_through_the_same_changing_fixture() {
    // The control: with nothing happening to the slot, the fixture above
    // answers a full read, so the refusals come from the recheck.
    let h = tc_home();
    tc_store(h.path(), None, SCOPE);
    let t = tc_changing(h.path(), "GET /never-asked ".into(), SlotChange::Replaced);
    let out = body(tc_call_in(h.path(), &t, "list-topics", europe(json!({}))).await);
    assert_eq!(out["items"], json!([]));
    assert_eq!(t.topics.requests().len(), 1);
}
