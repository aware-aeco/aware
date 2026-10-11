//! Trimble Connect BCF 2.1 topics and ToDos (floless.app#2184).
//!
//! Two APIs, one handler:
//!
//! - **BCF 2.1** (topics, comments, viewpoints + snapshots, document
//!   references) is Trimble's Topics API, published as
//!   `api.swaggerhub.com/apis/Trimble-Connect/topic/v2`. It does not live under
//!   the agent's Core base: it is served from a *regional host*
//!   (`open11.connect.trimble.com` for North America, `open21` for Europe, …).
//!   The host is discovered per project — `GET {core}/projects/{id}` gives the
//!   project's `location`, and the public `GET {core}/regions` maps that
//!   location to the region's `topic-api` — and only an exact, known Trimble
//!   Topics origin is ever sent the bearer token ([`prod_topics_host`]).
//!   Redirects are never followed, so a discovered or answering host cannot move
//!   the request somewhere else.
//! - **ToDos** are Core API 2.0 (`/todos`, same base as the folder commands).
//!
//! Like [`super::trimble_ops`], the commands need what the generic REST
//! renderer cannot express: snake_case (BCF) or camelCase (Core) bodies from
//! kebab-case inputs, the region discovery above, a read-modify-write for BCF's
//! whole-topic `PUT`, base64 snapshots in and out, and BCF's `next`-header
//! paging. Inputs are validated before the credential is read. A non-2xx fails
//! the node with the step, the status and TC's own message.
//!
//! What the definition does NOT offer is not invented here: BCF 2.1 on Trimble
//! Connect has no `If-Match`/ETag and no `Range` (it pages with `top` +
//! `skiptoken`), and `/todos` declares no `If-Match`.

use std::io::Read as _;
use std::time::Duration;

use base64::Engine as _;
use serde_json::{Map, Value, json};

use crate::agent_resolution::AgentCatalogue;
use crate::error::AwareError;
use crate::runtime::invoker::percent_encode_path;
use crate::runtime::trimble_files::{auth_and_base, ok_response};

/// The trimble-connect commands this module owns. `get-todo` is declarative
/// REST (a path parameter is all it needs), so it is not here.
pub(super) const COMMANDS: &[&str] = &[
    "list-topics",
    "get-topic",
    "create-topic",
    "update-topic",
    "delete-topic",
    "list-comments",
    "create-comment",
    "list-viewpoints",
    "create-viewpoint",
    "get-viewpoint-snapshot",
    "list-document-references",
    "create-document-reference",
    "list-todos",
    "create-todo",
    "update-todo",
    "delete-todo",
];

/// Whether `command` is a trimble-connect command handled here.
pub(super) fn handles(command: &str) -> bool {
    COMMANDS.contains(&command)
}

/// The Topics API hosts Trimble publishes (one per production region). The
/// bearer token goes to no other BCF host.
const TOPICS_HOSTS: &[&str] = &[
    "open11.connect.trimble.com",
    "open21.connect.trimble.com",
    "open22.connect.trimble.com",
    "open31.connect.trimble.com",
    "open32.connect.trimble.com",
];

/// A snapshot image larger than this is refused, both ways.
const MAX_SNAPSHOT_BYTES: usize = 20 * 1024 * 1024;

/// Where a discovered Topics API may be, and how long one request may take.
/// Production uses [`Policy::PROD`]; the wire-mock tests pass a policy that
/// admits their 127.0.0.1 mock — there is no runtime switch to widen it.
#[derive(Clone, Copy)]
pub(super) struct Policy {
    allow: fn(&url::Url) -> bool,
    timeout: Duration,
    /// The first retry's backoff; the second waits twice this.
    backoff: Duration,
    /// The longest wait between attempts, `Retry-After` included.
    max_wait: Duration,
}

impl Policy {
    const PROD: Policy = Policy {
        allow: prod_topics_host,
        timeout: Duration::from_secs(120),
        backoff: Duration::from_secs(1),
        max_wait: Duration::from_secs(30),
    };
}

/// Attempts per request (the agent's rate-limit contract, skills/rate-limits.md).
const MAX_ATTEMPTS: u32 = 3;

/// `https://<one of TOPICS_HOSTS>` on the default port, with no credentials in
/// the URL.
fn prod_topics_host(url: &url::Url) -> bool {
    url.scheme() == "https"
        && url.port().is_none()
        && url.username().is_empty()
        && url.password().is_none()
        && url.host_str().is_some_and(|h| TOPICS_HOSTS.contains(&h))
}

/// Run one of [`COMMANDS`]. Blocking HTTP runs off the reactor.
pub(super) async fn invoke(
    catalogue: AgentCatalogue,
    command: &str,
    args: Value,
) -> Result<Value, AwareError> {
    invoke_with(catalogue, command, args, Policy::PROD).await
}

async fn invoke_with(
    catalogue: AgentCatalogue,
    command: &str,
    args: Value,
    policy: Policy,
) -> Result<Value, AwareError> {
    let command = command.to_string();
    tokio::task::spawn_blocking(move || {
        // Validate the inputs before touching the credential, so a malformed
        // node is reported as such and never costs a token refresh or a request.
        let op = Op::parse(&command, &args)?;
        let (token, core) = auth_and_base(&catalogue)?;
        let tc = Client {
            agent: client_agent(policy.timeout),
            token,
            core,
            policy,
        };
        run(&tc, op, policy)
    })
    .await
    .map_err(|e| AwareError::Internal(format!("trimble-connect task join: {e}")))?
}

/// No redirects (a 3xx is an error, never followed) and a whole-request
/// deadline on top of the connect / read / write inactivity bounds.
fn client_agent(total: Duration) -> ureq::Agent {
    ureq::AgentBuilder::new()
        .redirects(0)
        .timeout_connect(Duration::from_secs(15))
        .timeout_read(Duration::from_secs(60))
        .timeout_write(Duration::from_secs(60))
        .timeout(total)
        .build()
}

// ── validated commands ───────────────────────────────────────────────────────

/// A validated command. Parsing is pure, so every refusal of bad input is
/// tested without a server and happens before any credential is read.
#[derive(Debug, PartialEq)]
enum Op {
    /// A paged BCF list under the project: `rel` is `topics` or
    /// `topics/{id}/{comments|viewpoints|document_references}`.
    BcfList {
        project: String,
        rel: String,
        top: Option<u64>,
        skiptoken: Option<String>,
    },
    GetTopic {
        project: String,
        topic: String,
    },
    /// A BCF `POST` under the project; the response's `guid` is reported under
    /// `id_key` and the whole object under `object_key`.
    BcfCreate {
        project: String,
        rel: String,
        body: Map<String, Value>,
        id_key: &'static str,
        object_key: &'static str,
    },
    UpdateTopic {
        project: String,
        topic: String,
        /// BCF wire key → new value; `Null` clears.
        changes: Map<String, Value>,
    },
    DeleteTopic {
        project: String,
        topic: String,
    },
    Snapshot {
        project: String,
        topic: String,
        viewpoint: String,
    },
    ListTodos {
        project: String,
        file: Option<String>,
        version: Option<String>,
        range: Option<String>,
    },
    CreateTodo {
        body: Map<String, Value>,
    },
    UpdateTodo {
        todo: String,
        changes: Map<String, Value>,
    },
    DeleteTodo {
        todo: String,
    },
}

/// The fields of BCF's `PUT topic` body (Trimble's UpdateTopic schema), in
/// order. `update-topic` re-sends every one the topic has, so a field the
/// caller did not touch survives the whole-topic replace.
const TOPIC_PUT_KEYS: &[&str] = &[
    "topic_type",
    "topic_status",
    "reference_links",
    "title",
    "priority",
    "index",
    "labels",
    "assigned_to",
    "assigned_to_uuid",
    "assignees",
    "stage",
    "description",
    "due_date",
    "marker",
    "bim_snippet",
];

/// A topic's plain-string fields: kebab input → BCF wire key. Topic types,
/// statuses, priorities and stages are project extensions in BCF, so they are
/// passed through rather than checked against a fixed list.
const TOPIC_STRING_FIELDS: &[(&str, &str)] = &[
    ("title", "title"),
    ("description", "description"),
    ("topic-type", "topic_type"),
    ("topic-status", "topic_status"),
    ("priority", "priority"),
    ("assigned-to", "assigned_to"),
    ("due-date", "due_date"),
    ("stage", "stage"),
];

/// The topic fields `update-topic` may clear with an explicit `null`. Verified
/// live against Trimble Connect: its `PUT topic` ignores a `null` (and an
/// omitted key) rather than clearing, `description: ""` clears, `labels: []`
/// empties, and there is no way to clear `due_date` (`""` is ignored) or
/// `assigned_to` (`""` is refused) — so only these two are clearable, and a
/// `null` is sent as the value that actually clears.
const TOPIC_CLEARABLE: &[&str] = &["description", "labels"];

/// ToDo status / priority values — fixed enums in the Core definition. (ToDo
/// *types* are per project — `GET /todos/types` — so they pass through.)
const TODO_STATUSES: &[&str] = &["NEW", "IN_PROGRESS", "RESOLVED", "BLOCKED", "CLOSED"];
const TODO_PRIORITIES: &[&str] = &["LOW", "NORMAL", "HIGH", "CRITICAL"];

/// A ToDo's plain-string fields: kebab input → Core wire key.
const TODO_STRING_FIELDS: &[(&str, &str)] = &[
    ("title", "title"),
    ("description", "description"),
    ("type", "type"),
    ("label", "label"),
    ("due-date", "dueDate"),
];

/// The ToDo fields `update-todo` may clear with an explicit `null`.
/// (`label` is not: TC answers a `null` label by keeping the auto-assigned one —
/// verified live.)
const TODO_CLEARABLE: &[&str] = &["due-date", "assignees"];

impl Op {
    fn parse(command: &str, args: &Value) -> Result<Self, AwareError> {
        let project = || req_str(args, "project-id");
        let topic = || req_str(args, "topic-id");
        let list = |rel: String| -> Result<Op, AwareError> {
            Ok(Op::BcfList {
                project: project()?,
                rel,
                top: opt_count(args, "top", 1, u64::MAX)?,
                skiptoken: opt_str(args, "skiptoken")?.map(str::to_string),
            })
        };
        Ok(match command {
            "list-topics" => list("topics".into())?,
            "list-comments" => list(format!("topics/{}/comments", enc(&topic()?)))?,
            "list-viewpoints" => list(format!("topics/{}/viewpoints", enc(&topic()?)))?,
            "list-document-references" => {
                list(format!("topics/{}/document_references", enc(&topic()?)))?
            }
            "get-topic" => Op::GetTopic {
                project: project()?,
                topic: topic()?,
            },
            "delete-topic" => Op::DeleteTopic {
                project: project()?,
                topic: topic()?,
            },
            "create-topic" => {
                let mut body = Map::new();
                if let Some(guid) = opt_str(args, "guid")? {
                    body.insert("guid".into(), json!(guid));
                }
                for (input, wire) in TOPIC_STRING_FIELDS {
                    if let Some(v) = opt_str(args, input)? {
                        body.insert((*wire).into(), json!(v));
                    }
                }
                if let Some(labels) = opt_labels(args)? {
                    body.insert("labels".into(), labels);
                }
                match body.get("title").and_then(Value::as_str) {
                    Some(t) if !t.trim().is_empty() => {}
                    Some(_) => return Err(invalid("`title` is blank".into())),
                    None => return Err(missing("title")),
                }
                Op::BcfCreate {
                    project: project()?,
                    rel: "topics".into(),
                    body,
                    id_key: "topic-id",
                    object_key: "topic",
                }
            }
            "update-topic" => {
                if present(args, "guid") {
                    return Err(invalid(
                        "update-topic cannot change a topic's `guid` (it is its identity)".into(),
                    ));
                }
                let mut changes = Map::new();
                for (input, wire) in TOPIC_STRING_FIELDS {
                    match tri_str(args, input)? {
                        Tri::Absent => {}
                        Tri::Null if TOPIC_CLEARABLE.contains(input) => {
                            changes.insert((*wire).into(), json!(""));
                        }
                        Tri::Null => {
                            return Err(invalid(format!("`{input}` cannot be cleared")));
                        }
                        Tri::Value(v) => {
                            if *input == "title" && v.trim().is_empty() {
                                return Err(invalid("`title` is blank".into()));
                            }
                            changes.insert((*wire).into(), json!(v));
                        }
                    }
                }
                if args.get("labels").is_some_and(Value::is_null) {
                    changes.insert("labels".into(), json!([]));
                } else if let Some(labels) = opt_labels(args)? {
                    changes.insert("labels".into(), labels);
                }
                if changes.is_empty() {
                    return Err(invalid(
                        "update-topic needs at least one field to change (title, description, \
                         topic-type, topic-status, priority, labels, assigned-to, due-date, stage)"
                            .into(),
                    ));
                }
                Op::UpdateTopic {
                    project: project()?,
                    topic: topic()?,
                    changes,
                }
            }
            "create-comment" => {
                let comment = req_str(args, "comment")?;
                let mut body = Map::new();
                body.insert("comment".into(), json!(comment));
                if let Some(v) = opt_str(args, "viewpoint-id")? {
                    body.insert("viewpoint_guid".into(), json!(v));
                }
                if let Some(v) = opt_str(args, "reply-to-comment-id")? {
                    body.insert("reply_to_comment_guid".into(), json!(v));
                }
                Op::BcfCreate {
                    project: project()?,
                    rel: format!("topics/{}/comments", enc(&topic()?)),
                    body,
                    id_key: "comment-id",
                    object_key: "comment",
                }
            }
            "create-viewpoint" => Op::BcfCreate {
                project: project()?,
                rel: format!("topics/{}/viewpoints", enc(&topic()?)),
                body: viewpoint_body(args)?,
                id_key: "viewpoint-id",
                object_key: "viewpoint",
            },
            "get-viewpoint-snapshot" => Op::Snapshot {
                project: project()?,
                topic: topic()?,
                viewpoint: req_str(args, "viewpoint-id")?,
            },
            "create-document-reference" => {
                let mut body = Map::new();
                match (opt_str(args, "document-id")?, opt_str(args, "url")?) {
                    (Some(d), None) => {
                        body.insert("document_guid".into(), json!(d));
                    }
                    (None, Some(u)) => {
                        body.insert("url".into(), json!(u));
                    }
                    _ => {
                        return Err(invalid(
                            "create-document-reference needs exactly one of `document-id` \
                             (a project BCF document) or `url`"
                                .into(),
                        ));
                    }
                }
                if let Some(d) = opt_str(args, "description")? {
                    body.insert("description".into(), json!(d));
                }
                Op::BcfCreate {
                    project: project()?,
                    rel: format!("topics/{}/document_references", enc(&topic()?)),
                    body,
                    id_key: "document-reference-id",
                    object_key: "document-reference",
                }
            }
            "list-todos" => {
                let file = opt_str(args, "file-id")?.map(str::to_string);
                let version = opt_str(args, "version-id")?.map(str::to_string);
                if version.is_some() && file.is_none() {
                    return Err(invalid("`version-id` needs `file-id`".into()));
                }
                Op::ListTodos {
                    project: project()?,
                    file,
                    version,
                    range: crate::runtime::trimble_ops::item_range(args)?.map(str::to_string),
                }
            }
            "create-todo" => {
                let mut body = todo_fields(args, false)?;
                match body.get("description") {
                    Some(Value::String(d)) if !d.trim().is_empty() => {}
                    Some(_) => return Err(invalid("`description` is blank".into())),
                    None => return Err(missing("description")),
                }
                body.insert("projectId".into(), json!(project()?));
                Op::CreateTodo { body }
            }
            "update-todo" => {
                if present(args, "project-id") {
                    return Err(invalid(
                        "update-todo cannot move a ToDo to another project".into(),
                    ));
                }
                let changes = todo_fields(args, true)?;
                if changes.is_empty() {
                    return Err(invalid(
                        "update-todo needs at least one field to change (title, description, \
                         type, label, due-date, percent-complete, status, priority, assignees)"
                            .into(),
                    ));
                }
                Op::UpdateTodo {
                    todo: req_str(args, "todo-id")?,
                    changes,
                }
            }
            "delete-todo" => Op::DeleteTodo {
                todo: req_str(args, "todo-id")?,
            },
            other => {
                return Err(AwareError::Validation(format!(
                    "trimble-connect: {other:?} is not a BCF / ToDo command"
                )));
            }
        })
    }
}

/// The ToDo body fields from kebab inputs. On update, an explicit `null`
/// clears a [`TODO_CLEARABLE`] field (`assignees` → `[]`) and is refused on
/// any other.
fn todo_fields(args: &Value, update: bool) -> Result<Map<String, Value>, AwareError> {
    let mut body = Map::new();
    let null_ok = |input: &str| -> Result<(), AwareError> {
        if update && TODO_CLEARABLE.contains(&input) {
            Ok(())
        } else {
            Err(invalid(format!("`{input}` cannot be cleared")))
        }
    };
    for (input, wire) in TODO_STRING_FIELDS {
        match tri_str(args, input)? {
            Tri::Absent => {}
            Tri::Null if !update => {}
            Tri::Null => {
                null_ok(input)?;
                body.insert((*wire).into(), Value::Null);
            }
            Tri::Value(v) => {
                if matches!(*input, "title" | "description") && v.trim().is_empty() {
                    return Err(invalid(format!("`{input}` is blank")));
                }
                if *input == "due-date" {
                    body.insert((*wire).into(), json!(tc_datetime(v)));
                } else {
                    body.insert((*wire).into(), json!(v));
                }
            }
        }
    }
    for input in ["status", "priority", "percent-complete"] {
        if update && args.get(input).is_some_and(Value::is_null) {
            return Err(invalid(format!("`{input}` cannot be cleared")));
        }
    }
    for (input, allowed) in [("status", TODO_STATUSES), ("priority", TODO_PRIORITIES)] {
        if let Some(v) = opt_str(args, input)? {
            let upper = v.to_ascii_uppercase();
            if !allowed.contains(&upper.as_str()) {
                return Err(invalid(format!(
                    "`{input}` must be one of {}, got {v:?}",
                    allowed.join(", ")
                )));
            }
            body.insert(input.into(), json!(upper));
        }
    }
    if let Some(p) = opt_count(args, "percent-complete", 0, 100)? {
        body.insert("percentComplete".into(), json!(p));
    }
    match args.get("assignees") {
        None => {}
        Some(Value::String(s)) if s.is_empty() => {}
        Some(Value::Null) if !update => {}
        Some(Value::Null) => {
            null_ok("assignees")?;
            body.insert("assignees".into(), json!([]));
        }
        Some(v) => {
            body.insert("assignees".into(), assignees(v)?);
        }
    }
    Ok(body)
}

/// An ISO 8601 date-time as the Core API accepts it. TC refuses a `Z` suffix
/// and fractional seconds ("Invalid date format") but takes `+0000` and
/// `±hh:mm` offsets and a bare date — verified live — so `Z` becomes `+0000`
/// and a fraction is dropped. Anything else passes through for TC to judge.
fn tc_datetime(v: &str) -> String {
    let mut t = v.trim().to_string();
    if t.ends_with(['Z', 'z']) {
        t.pop();
        t.push_str("+0000");
    }
    if let Some(dot) = t.find('.') {
        let digits = t[dot + 1..]
            .find(|c: char| !c.is_ascii_digit())
            .map_or(t.len(), |i| dot + 1 + i);
        if digits > dot + 1 {
            t.replace_range(dot..digits, "");
        }
    }
    t
}

/// `assignees`: a list whose items are a user id (→ `{id, type: USER}`) or an
/// `{id, type: USER|GROUP}` object. A JSON-encoded list (what `{{ }}` renders a
/// list to) is accepted too.
fn assignees(v: &Value) -> Result<Value, AwareError> {
    let v = decoded(v, "assignees")?;
    let Value::Array(items) = &v else {
        return Err(invalid("`assignees` must be a list".into()));
    };
    items
        .iter()
        .map(|item| match item {
            Value::String(id) if !id.trim().is_empty() => Ok(json!({"id": id, "type": "USER"})),
            Value::Object(o) => {
                let id = o
                    .get("id")
                    .and_then(Value::as_str)
                    .filter(|s| !s.trim().is_empty())
                    .ok_or_else(|| invalid("each assignee needs an `id`".into()))?;
                let ty = match o.get("type").and_then(Value::as_str) {
                    None => "USER".to_string(),
                    Some(t) if t.eq_ignore_ascii_case("user") => "USER".to_string(),
                    Some(t) if t.eq_ignore_ascii_case("group") => "GROUP".to_string(),
                    Some(t) => {
                        return Err(invalid(format!(
                            "assignee `type` must be USER or GROUP, got {t:?}"
                        )));
                    }
                };
                Ok(json!({"id": id, "type": ty}))
            }
            other => Err(invalid(format!(
                "each assignee must be a user id or an {{id, type}} object, got {other}"
            ))),
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Value::Array)
}

/// `labels`: a list of non-blank strings (or its JSON encoding).
fn opt_labels(args: &Value) -> Result<Option<Value>, AwareError> {
    let Some(raw) = args.get("labels") else {
        return Ok(None);
    };
    if raw.is_null() || raw.as_str() == Some("") {
        return Ok(None);
    }
    let v = decoded(raw, "labels")?;
    let ok = v.as_array().is_some_and(|a| {
        a.iter()
            .all(|l| l.as_str().is_some_and(|s| !s.trim().is_empty()))
    });
    if !ok {
        return Err(invalid(
            "`labels` must be a list of non-blank strings".into(),
        ));
    }
    Ok(Some(v))
}

// ── viewpoint validation ─────────────────────────────────────────────────────

/// The `create-viewpoint` body. Cameras, lines, clipping planes and components
/// are given in BCF's own wire shape (snake_case, `{x, y, z}` points) and
/// checked structurally here, so a malformed viewpoint is refused before any
/// request. A camera or a snapshot is required.
fn viewpoint_body(args: &Value) -> Result<Map<String, Value>, AwareError> {
    let mut body = Map::new();
    let perspective = opt_object(args, "perspective-camera")?;
    let orthogonal = opt_object(args, "orthogonal-camera")?;
    if perspective.is_some() && orthogonal.is_some() {
        return Err(invalid(
            "give `perspective-camera` or `orthogonal-camera`, not both".into(),
        ));
    }
    if let Some(cam) = perspective {
        camera(&cam, "perspective-camera", "field_of_view", |f| {
            f > 0.0 && f < 180.0
        })?;
        body.insert("perspective_camera".into(), cam);
    }
    if let Some(cam) = orthogonal {
        camera(&cam, "orthogonal-camera", "view_to_world_scale", |s| {
            s > 0.0
        })?;
        body.insert("orthogonal_camera".into(), cam);
    }
    if let Some(lines) = opt_list(args, "lines")? {
        for (i, l) in lines.as_array().into_iter().flatten().enumerate() {
            let what = format!("lines[{i}]");
            point(l.get("start_point"), &format!("{what}.start_point"), false)?;
            point(l.get("end_point"), &format!("{what}.end_point"), false)?;
        }
        body.insert("lines".into(), lines);
    }
    if let Some(planes) = opt_list(args, "clipping-planes")? {
        for (i, p) in planes.as_array().into_iter().flatten().enumerate() {
            let what = format!("clipping-planes[{i}]");
            point(p.get("location"), &format!("{what}.location"), false)?;
            point(p.get("direction"), &format!("{what}.direction"), true)?;
        }
        body.insert("clipping_planes".into(), planes);
    }
    if let Some(components) = opt_object(args, "components")? {
        for key in ["selection", "coloring"] {
            if let Some(v) = components.get(key)
                && !v.as_array().is_some_and(|a| a.iter().all(Value::is_object))
            {
                return Err(invalid(format!(
                    "`components.{key}` must be a list of objects"
                )));
            }
        }
        if let Some(v) = components.get("visibility")
            && !v.is_object()
        {
            return Err(invalid("`components.visibility` must be an object".into()));
        }
        body.insert("components".into(), components);
    }
    if let Some(snapshot) = snapshot_input(args)? {
        body.insert("snapshot".into(), snapshot);
    }
    if !body.contains_key("perspective_camera")
        && !body.contains_key("orthogonal_camera")
        && !body.contains_key("snapshot")
    {
        return Err(invalid(
            "create-viewpoint needs a camera (`perspective-camera` / `orthogonal-camera`) \
             or a `snapshot`"
                .into(),
        ));
    }
    Ok(body)
}

/// A BCF camera: view point, non-zero direction and up vectors, and the
/// camera-specific scalar (`field_of_view` / `view_to_world_scale`).
fn camera(cam: &Value, what: &str, scalar: &str, ok: fn(f64) -> bool) -> Result<(), AwareError> {
    point(
        cam.get("camera_view_point"),
        &format!("{what}.camera_view_point"),
        false,
    )?;
    point(
        cam.get("camera_direction"),
        &format!("{what}.camera_direction"),
        true,
    )?;
    point(
        cam.get("camera_up_vector"),
        &format!("{what}.camera_up_vector"),
        true,
    )?;
    match cam.get(scalar).and_then(Value::as_f64) {
        Some(v) if v.is_finite() && ok(v) => Ok(()),
        _ => Err(invalid(format!(
            "`{what}.{scalar}` is missing or out of range"
        ))),
    }
}

/// An `{x, y, z}` of finite numbers; a direction must not be the zero vector.
fn point(v: Option<&Value>, what: &str, direction: bool) -> Result<(), AwareError> {
    let coords: Option<Vec<f64>> = v.and_then(|p| {
        ["x", "y", "z"]
            .iter()
            .map(|k| p.get(*k).and_then(Value::as_f64).filter(|n| n.is_finite()))
            .collect()
    });
    let Some(coords) = coords else {
        return Err(invalid(format!(
            "`{what}` must be an object with numeric x, y and z"
        )));
    };
    if direction && coords.iter().all(|c| *c == 0.0) {
        return Err(invalid(format!("`{what}` is the zero vector")));
    }
    Ok(())
}

/// The optional `snapshot` (base64 PNG or JPEG) → BCF's `{snapshot_type,
/// snapshot_data}`. The size is checked on the encoded string before decoding,
/// the image's structure is checked, and a declared `snapshot-type` must match
/// what the bytes are.
fn snapshot_input(args: &Value) -> Result<Option<Value>, AwareError> {
    let Some(encoded) = opt_str(args, "snapshot")? else {
        if opt_str(args, "snapshot-type")?.is_some() {
            return Err(invalid("`snapshot-type` given without a `snapshot`".into()));
        }
        return Ok(None);
    };
    // Measured on the encoded text, before anything is copied or decoded.
    let encoded_len = encoded.bytes().filter(|b| !b.is_ascii_whitespace()).count();
    if encoded_len > MAX_SNAPSHOT_BYTES.div_ceil(3) * 4 {
        return Err(invalid(format!(
            "`snapshot` is larger than {} MiB",
            MAX_SNAPSHOT_BYTES / 1024 / 1024
        )));
    }
    let cleaned: String = encoded
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect();
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(&cleaned)
        .map_err(|_| invalid("`snapshot` must be a base64 string".into()))?;
    if bytes.len() > MAX_SNAPSHOT_BYTES {
        return Err(invalid(format!(
            "`snapshot` is larger than {} MiB",
            MAX_SNAPSHOT_BYTES / 1024 / 1024
        )));
    }
    let kind = image_kind(&bytes)
        .ok_or_else(|| invalid("`snapshot` is not a complete PNG or JPEG image".into()))?;
    if let Some(declared) = opt_str(args, "snapshot-type")? {
        let declared = match declared.to_ascii_lowercase().as_str() {
            "png" => "png",
            "jpg" | "jpeg" => "jpg",
            other => {
                return Err(invalid(format!(
                    "`snapshot-type` must be png or jpg, got {other:?}"
                )));
            }
        };
        if declared != kind {
            return Err(invalid(format!(
                "`snapshot-type` is {declared} but the snapshot is a {kind}"
            )));
        }
    }
    Ok(Some(json!({
        "snapshot_type": kind,
        "snapshot_data": base64::engine::general_purpose::STANDARD.encode(&bytes),
    })))
}

/// `png` for a complete PNG (see [`png_is_complete`]), `jpg` for a
/// well-formed JPEG (see [`jpeg_is_complete`]); anything else (truncated
/// included) is `None`.
fn image_kind(b: &[u8]) -> Option<&'static str> {
    if png_is_complete(b) {
        return Some("png");
    }
    if jpeg_is_complete(b) {
        return Some("jpg");
    }
    None
}

/// A JPEG whose marker stream is whole: SOI, then length-prefixed segments that
/// stay inside the file, a frame header (SOF, with a non-zero size and at least
/// one component) before the first scan (SOS), entropy-coded data after it, and
/// EOI as the last two bytes. `FF D8 FF D9` — markers with no image — fails.
fn jpeg_is_complete(b: &[u8]) -> bool {
    if b.len() < 4 || b[..2] != [0xFF, 0xD8] || b[b.len() - 2..] != [0xFF, 0xD9] {
        return false;
    }
    let mut i = 2;
    let mut frame = false;
    while i + 4 <= b.len() {
        if b[i] != 0xFF {
            return false;
        }
        let marker = b[i + 1];
        // Fill bytes and standalone markers (TEM, RSTn) carry no length.
        if marker == 0xFF {
            i += 1;
            continue;
        }
        if marker == 0x01 || (0xD0..=0xD7).contains(&marker) {
            i += 2;
            continue;
        }
        let len = usize::from(u16::from_be_bytes([b[i + 2], b[i + 3]]));
        if len < 2 || i + 2 + len > b.len() {
            return false;
        }
        let segment = &b[i + 4..i + 2 + len];
        match marker {
            // SOF0..SOF15, except DHT (C4), JPG (C8) and DAC (CC).
            0xC0..=0xCF if !matches!(marker, 0xC4 | 0xC8 | 0xCC) => {
                // precision, height(2), width(2), component count, 3 bytes each.
                let ok = segment.len() >= 6 && {
                    let height = u16::from_be_bytes([segment[1], segment[2]]);
                    let width = u16::from_be_bytes([segment[3], segment[4]]);
                    let comps = usize::from(segment[5]);
                    height > 0 && width > 0 && comps > 0 && segment.len() == 6 + 3 * comps
                };
                if !ok {
                    return false;
                }
                frame = true;
            }
            // SOS: the scan's coded data follows the header, up to EOI.
            0xDA => {
                let data_start = i + 2 + len;
                // At least one byte of coded data between the scan header and EOI.
                return frame && data_start < b.len() - 2;
            }
            // EOI before any scan: no image.
            0xD9 => return false,
            _ => {}
        }
        i += 2 + len;
    }
    false
}

/// A PNG whose chunk stream is whole: the signature, then chunks each with an
/// intact length and CRC, starting with a 13-byte `IHDR`, carrying at least one
/// `IDAT`, and ending exactly with an empty `IEND`. A header-only or cut-off
/// file fails here rather than reaching TC as a corrupt snapshot.
fn png_is_complete(b: &[u8]) -> bool {
    const SIGNATURE: &[u8] = b"\x89PNG\r\n\x1a\n";
    let Some(mut rest) = b.strip_prefix(SIGNATURE) else {
        return false;
    };
    let (mut first, mut idat) = (true, false);
    while rest.len() >= 12 {
        let len = u32::from_be_bytes([rest[0], rest[1], rest[2], rest[3]]) as usize;
        let Some(end) = len.checked_add(12).filter(|e| *e <= rest.len()) else {
            return false;
        };
        let kind = &rest[4..8];
        let crc = u32::from_be_bytes([rest[end - 4], rest[end - 3], rest[end - 2], rest[end - 1]]);
        if crc32(&rest[4..end - 4]) != crc || (first && (kind != b"IHDR" || len != 13)) {
            return false;
        }
        first = false;
        idat |= kind == b"IDAT";
        if kind == b"IEND" {
            return len == 0 && end == rest.len() && idat;
        }
        rest = &rest[end..];
    }
    false
}

/// CRC-32 (ISO-HDLC, the one PNG uses), bitwise — snapshots are small and
/// checked once.
fn crc32(data: &[u8]) -> u32 {
    let mut crc = !0u32;
    for &byte in data {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            crc = if crc & 1 == 1 {
                (crc >> 1) ^ 0xEDB8_8320
            } else {
                crc >> 1
            };
        }
    }
    !crc
}

// ── the client ───────────────────────────────────────────────────────────────

/// The authenticated client for one command: `core` is the agent's Core API
/// base (`…/tc/api/2.0`).
struct Client {
    agent: ureq::Agent,
    token: String,
    core: String,
    policy: Policy,
}

impl Client {
    fn authed(&self, method: &str, url: &str) -> ureq::Request {
        self.agent
            .request(method, url)
            .set("Authorization", &format!("Bearer {}", self.token))
    }

    /// Send `req` (with a JSON body when given) and insist on a 2xx: redirects
    /// are not followed, so a 3xx is a failure here rather than a silent success.
    ///
    /// Throttling and transient faults are retried — up to [`MAX_ATTEMPTS`],
    /// waiting `backoff` then twice that, or the response's `Retry-After` (both
    /// capped at `max_wait`). A `429` is retried for every method: TC did not
    /// process the request. A `5xx` or a transport error is retried only for
    /// GET / PUT / PATCH, which are safe to repeat; a POST could already have
    /// created a topic or comment, and a repeated DELETE would report the
    /// first one's success as a 404, so those surface the fault instead.
    fn send(
        &self,
        req: ureq::Request,
        body: Option<&Value>,
        what: &str,
    ) -> Result<ureq::Response, AwareError> {
        let repeatable = matches!(req.method(), "GET" | "PUT" | "PATCH");
        let mut attempt = 1;
        let res = loop {
            let res = match body {
                Some(b) => req
                    .clone()
                    .set("Content-Type", "application/json")
                    .send_string(&b.to_string()),
                None => req.clone().call(),
            };
            let retry_after = match &res {
                Err(ureq::Error::Status(429, r)) => Some(retry_after_of(r)),
                Err(ureq::Error::Status(500..=599, r)) if repeatable => Some(retry_after_of(r)),
                Err(ureq::Error::Transport(_)) if repeatable => Some(None),
                _ => None,
            };
            match retry_after {
                Some(after) if attempt < MAX_ATTEMPTS => {
                    let backoff = self.policy.backoff * 2u32.pow(attempt - 1);
                    std::thread::sleep(after.unwrap_or(backoff).min(self.policy.max_wait));
                    attempt += 1;
                }
                _ => break res,
            }
        };
        let what = if attempt > 1 {
            format!("{what} (after {attempt} attempts)")
        } else {
            what.to_string()
        };
        let resp = ok_response(res, &what)?;
        if !(200..300).contains(&resp.status()) {
            return Err(AwareError::Network(format!(
                "{what}: HTTP {} — redirects are not followed (location: {})",
                resp.status(),
                resp.header("Location").unwrap_or("none")
            )));
        }
        Ok(resp)
    }

    fn json_of(
        &self,
        req: ureq::Request,
        body: Option<&Value>,
        what: &str,
    ) -> Result<Value, AwareError> {
        read_json(self.send(req, body, what)?, what)?
            .ok_or_else(|| AwareError::Network(format!("{what}: TC answered with no content")))
    }
}

/// A `Retry-After` given in seconds (TC's form); an HTTP date is not honoured
/// and the backoff applies instead.
fn retry_after_of(resp: &ureq::Response) -> Option<Duration> {
    resp.header("Retry-After")
        .and_then(|v| v.trim().parse::<u64>().ok())
        .map(Duration::from_secs)
}

/// The JSON body of a 2xx; `None` for 204 No Content.
fn read_json(resp: ureq::Response, what: &str) -> Result<Option<Value>, AwareError> {
    if resp.status() == 204 {
        return Ok(None);
    }
    let text = resp
        .into_string()
        .map_err(|e| AwareError::Network(format!("{what}: read body: {e}")))?;
    serde_json::from_str(&text)
        .map(Some)
        .map_err(|e| AwareError::Network(format!("{what}: bad JSON: {e}")))
}

/// The project's BCF 2.1 base, `{topic-api}bcf/2.1/projects/{id}`: the
/// project's `location` (Core `GET projects/{id}`) mapped through the public
/// `GET regions` to that region's `topic-api`, which must pass `policy` before
/// any request carrying the token is made to it.
fn bcf_base(tc: &Client, project: &str, policy: Policy) -> Result<String, AwareError> {
    let what = "BCF region";
    let info = tc.json_of(
        tc.authed("GET", &format!("{}/projects/{}", tc.core, enc(project))),
        None,
        &format!("{what}: project"),
    )?;
    let location = info
        .get("location")
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            AwareError::Network(format!("{what}: project {project} has no `location`"))
        })?;
    // `regions` is public: it is asked without the token.
    let regions = tc.json_of(
        tc.agent.get(&format!("{}/regions", tc.core)),
        None,
        &format!("{what}: regions"),
    )?;
    let topic_api = regions
        .as_array()
        .into_iter()
        .flatten()
        .find(|r| r.get("location").and_then(Value::as_str) == Some(location))
        .and_then(|r| r.get("topic-api").and_then(Value::as_str))
        .ok_or_else(|| {
            AwareError::Network(format!(
                "{what}: TC lists no Topics API for the project's location {location:?}"
            ))
        })?;
    let parsed = url::Url::parse(topic_api)
        .map_err(|e| AwareError::Network(format!("{what}: Topics API URL {topic_api:?}: {e}")))?;
    if !(policy.allow)(&parsed) {
        return Err(AwareError::Network(format!(
            "{what}: TC named {topic_api:?} as the Topics API for {location:?}; that is not a \
             known Trimble Connect Topics host, so the token is not sent there"
        )));
    }
    Ok(format!(
        "{}/bcf/2.1/projects/{}",
        topic_api.trim_end_matches('/'),
        enc(project)
    ))
}

// ── commands ─────────────────────────────────────────────────────────────────

fn run(tc: &Client, op: Op, policy: Policy) -> Result<Value, AwareError> {
    match op {
        Op::BcfList {
            project,
            rel,
            top,
            skiptoken,
        } => {
            let base = bcf_base(tc, &project, policy)?;
            bcf_list(tc, &base, &rel, top, skiptoken.as_deref())
        }
        Op::GetTopic { project, topic } => {
            let base = bcf_base(tc, &project, policy)?;
            let t = get_topic(tc, &base, &topic)?;
            Ok(json!({ "topic-id": guid_of(&t, "get topic")?, "topic": t }))
        }
        Op::BcfCreate {
            project,
            rel,
            body,
            id_key,
            object_key,
        } => {
            let base = bcf_base(tc, &project, policy)?;
            let what = format!("create {}", object_key.replace('-', " "));
            let created = tc.json_of(
                tc.authed("POST", &format!("{base}/{rel}")),
                Some(&Value::Object(body)),
                &what,
            )?;
            Ok(json!({ id_key: guid_of(&created, &what)?, object_key: created }))
        }
        Op::UpdateTopic {
            project,
            topic,
            changes,
        } => {
            let base = bcf_base(tc, &project, policy)?;
            update_topic(tc, &base, &topic, changes)
        }
        Op::DeleteTopic { project, topic } => {
            let base = bcf_base(tc, &project, policy)?;
            tc.send(
                tc.authed("DELETE", &format!("{base}/topics/{}", enc(&topic))),
                None,
                "delete topic",
            )?;
            Ok(json!({ "topic-id": topic, "deleted": true }))
        }
        Op::Snapshot {
            project,
            topic,
            viewpoint,
        } => {
            let base = bcf_base(tc, &project, policy)?;
            snapshot(tc, &base, &topic, &viewpoint)
        }
        Op::ListTodos {
            project,
            file,
            version,
            range,
        } => list_todos(
            tc,
            &project,
            file.as_deref(),
            version.as_deref(),
            range.as_deref(),
        ),
        Op::CreateTodo { body } => {
            let todo = tc.json_of(
                tc.authed("POST", &format!("{}/todos", tc.core)),
                Some(&Value::Object(body)),
                "create todo",
            )?;
            Ok(json!({ "todo-id": id_of(&todo, "create todo")?, "todo": todo }))
        }
        Op::UpdateTodo { todo, changes } => {
            let updated = tc.json_of(
                tc.authed("PATCH", &format!("{}/todos/{}", tc.core, enc(&todo))),
                Some(&Value::Object(changes)),
                "update todo",
            )?;
            Ok(json!({ "todo-id": id_of(&updated, "update todo")?, "todo": updated }))
        }
        Op::DeleteTodo { todo } => {
            tc.send(
                tc.authed("DELETE", &format!("{}/todos/{}", tc.core, enc(&todo))),
                None,
                "delete todo",
            )?;
            Ok(json!({ "todo-id": todo, "deleted": true }))
        }
    }
}

/// One page of a BCF list. BCF 2.1 pages with `top` + `skiptoken`; the
/// response's `next` header is the next page's URL, from which the opaque
/// `skiptoken` is handed back as `next-skiptoken` (null on the last page).
/// A 204 is an empty list.
fn bcf_list(
    tc: &Client,
    base: &str,
    rel: &str,
    top: Option<u64>,
    skiptoken: Option<&str>,
) -> Result<Value, AwareError> {
    let what = format!(
        "list {}",
        rel.rsplit('/').next().unwrap_or(rel).replace('_', " ")
    );
    let mut url = format!("{base}/{rel}");
    let mut sep = '?';
    if let Some(t) = top {
        url.push_str(&format!("{sep}top={t}"));
        sep = '&';
    }
    if let Some(s) = skiptoken {
        url.push_str(&format!("{sep}skiptoken={}", enc(s)));
    }
    let resp = tc.send(tc.authed("GET", &url), None, &what)?;
    let next = resp.header("next").map(str::to_string);
    let items = match read_json(resp, &what)? {
        None => json!([]),
        Some(v @ Value::Array(_)) => v,
        Some(_) => {
            return Err(AwareError::Network(format!(
                "{what}: TC did not return a list"
            )));
        }
    };
    let next_skiptoken = match next.as_deref().map(str::trim).filter(|n| !n.is_empty()) {
        None => None,
        Some(n) => Some(skiptoken_of(n).ok_or_else(|| {
            AwareError::Network(format!(
                "{what}: TC's `next` page link {n:?} carries no skiptoken"
            ))
        })?),
    };
    Ok(json!({ "items": items, "next-skiptoken": next_skiptoken }))
}

/// The `skiptoken` query value of a (possibly relative) next-page URL.
fn skiptoken_of(next: &str) -> Option<String> {
    let base = url::Url::parse("https://next.invalid/").ok()?;
    let url = base.join(next).ok()?;
    url.query_pairs()
        .find(|(k, _)| k.eq_ignore_ascii_case("skiptoken"))
        .map(|(_, v)| v.into_owned())
        .filter(|v| !v.is_empty())
}

fn get_topic(tc: &Client, base: &str, topic: &str) -> Result<Value, AwareError> {
    tc.json_of(
        tc.authed("GET", &format!("{base}/topics/{}", enc(topic))),
        None,
        "get topic",
    )
}

/// `update-topic`. BCF's `PUT topic` is specified to replace the whole topic
/// (TC's in practice keeps a field the body leaves out), so the current topic is
/// read first and every field of the PUT schema it carries is sent back with the
/// caller's changes laid over it — an untouched field survives either way.
/// When the changes leave the topic as it is, nothing is written
/// (`changed: false`). BCF 2.1 on TC has no `If-Match`, so an edit another
/// client makes between the read and the write is overwritten.
fn update_topic(
    tc: &Client,
    base: &str,
    topic: &str,
    changes: Map<String, Value>,
) -> Result<Value, AwareError> {
    let current = get_topic(tc, base, topic)?;
    let mut body = Map::new();
    for key in TOPIC_PUT_KEYS {
        if let Some(v) = current.get(*key) {
            body.insert((*key).into(), v.clone());
        }
    }
    let before = body.clone();
    // TC's `assignees` (and the `assigned_to_uuid` beside it) override
    // `assigned_to` (UpdateTopicDto), so re-sending the old ones with a new email
    // would silently keep the old assignee. A reassignment drops them.
    if let Some(new) = changes.get("assigned_to")
        && current.get("assigned_to") != Some(new)
    {
        body.remove("assignees");
        body.remove("assigned_to_uuid");
    }
    for (k, v) in changes {
        body.insert(k, v);
    }
    if body == before {
        return Ok(json!({
            "topic-id": guid_of(&current, "update topic")?,
            "topic": current,
            "changed": false,
        }));
    }
    let updated = tc.json_of(
        tc.authed("PUT", &format!("{base}/topics/{}", enc(topic))),
        Some(&Value::Object(body)),
        "update topic",
    )?;
    Ok(json!({
        "topic-id": guid_of(&updated, "update topic")?,
        "topic": updated,
        "changed": true,
    }))
}

/// `get-viewpoint-snapshot` — the image bytes, base64, bounded to
/// [`MAX_SNAPSHOT_BYTES`].
fn snapshot(tc: &Client, base: &str, topic: &str, viewpoint: &str) -> Result<Value, AwareError> {
    let what = "get viewpoint snapshot";
    let resp = tc.send(
        tc.authed(
            "GET",
            &format!(
                "{base}/topics/{}/viewpoints/{}/snapshot",
                enc(topic),
                enc(viewpoint)
            ),
        ),
        None,
        what,
    )?;
    let mut bytes = Vec::new();
    resp.into_reader()
        .take(MAX_SNAPSHOT_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| AwareError::Network(format!("{what}: read: {e}")))?;
    if bytes.len() > MAX_SNAPSHOT_BYTES {
        return Err(AwareError::Network(format!(
            "{what}: the snapshot is larger than {} MiB",
            MAX_SNAPSHOT_BYTES / 1024 / 1024
        )));
    }
    if bytes.is_empty() {
        return Err(AwareError::Network(format!("{what}: TC returned no image")));
    }
    // Named from the bytes, which must be a whole PNG or JPEG: TC was seen
    // serving a PNG snapshot as `image/jpeg` (live, 2026-10-11), so its header
    // is not trusted, and a cut-off or corrupt image is a failure, not a result.
    let content_type = match image_kind(&bytes) {
        Some("png") => "image/png",
        Some(_) => "image/jpeg",
        None => {
            return Err(AwareError::Network(format!(
                "{what}: TC returned {} bytes that are not a complete PNG or JPEG",
                bytes.len()
            )));
        }
    };
    Ok(json!({
        "bytes": base64::engine::general_purpose::STANDARD.encode(&bytes),
        "encoding": "base64",
        "content-type": content_type,
        "size-kb": (bytes.len() as f64) / 1024.0,
    }))
}

/// `list-todos` — `GET todos?projectId=…[&fileId=…[&versionId=…]]`. TC pages
/// the list like its other Core lists (a 206 with `Content-Range`), so the
/// result carries `partial` and `content-range`, and `range` asks for a page.
fn list_todos(
    tc: &Client,
    project: &str,
    file: Option<&str>,
    version: Option<&str>,
    range: Option<&str>,
) -> Result<Value, AwareError> {
    let what = "list todos";
    let mut url = format!("{}/todos?projectId={}", tc.core, enc(project));
    if let Some(f) = file {
        url.push_str(&format!("&fileId={}", enc(f)));
    }
    if let Some(v) = version {
        url.push_str(&format!("&versionId={}", enc(v)));
    }
    let mut req = tc.authed("GET", &url);
    if let Some(r) = range {
        req = req.set("Range", r);
    }
    let resp = tc.send(req, None, what)?;
    let status = resp.status();
    let content_range = resp.header("Content-Range").map(str::to_string);
    let items = match read_json(resp, what)? {
        None => json!([]),
        Some(v @ Value::Array(_)) => v,
        Some(_) => {
            return Err(AwareError::Network(format!(
                "{what}: TC did not return a list"
            )));
        }
    };
    Ok(json!({
        "items": items,
        "partial": status == 206,
        "content-range": content_range,
    }))
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn enc(s: &str) -> String {
    percent_encode_path(s)
}

/// The BCF object's `guid` — what a downstream node passes on, so a 2xx
/// without it is a failure.
fn guid_of(v: &Value, what: &str) -> Result<String, AwareError> {
    required_key(v, "guid", what)
}

fn id_of(v: &Value, what: &str) -> Result<String, AwareError> {
    required_key(v, "id", what)
}

fn required_key(v: &Value, key: &str, what: &str) -> Result<String, AwareError> {
    v.get(key)
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .ok_or_else(|| AwareError::Network(format!("{what}: TC response has no {key}")))
}

/// Absent / given-as-null / a value — `update-*` needs all three apart, since
/// `null` clears a field and absence leaves it alone. `""` (what `{{ }}` renders
/// an unset input to) counts as absent.
enum Tri<'a> {
    Absent,
    Null,
    Value(&'a str),
}

fn tri_str<'a>(args: &'a Value, key: &str) -> Result<Tri<'a>, AwareError> {
    match args.get(key) {
        None => Ok(Tri::Absent),
        Some(Value::Null) => Ok(Tri::Null),
        Some(Value::String(s)) if s.is_empty() => Ok(Tri::Absent),
        Some(Value::String(s)) => Ok(Tri::Value(s)),
        Some(other) => Err(invalid(format!("`{key}` must be a string, got {other}"))),
    }
}

/// Whether `key` was given a real value (not absent, null or `""`).
fn present(args: &Value, key: &str) -> bool {
    match args.get(key) {
        None | Some(Value::Null) => false,
        Some(Value::String(s)) => !s.is_empty(),
        Some(_) => true,
    }
}

fn req_str(args: &Value, key: &str) -> Result<String, AwareError> {
    opt_str(args, key)?
        .filter(|s| !s.trim().is_empty())
        .map(str::to_string)
        .ok_or_else(|| missing(key))
}

/// An optional string input: absent, `null` and `""` all mean "not given".
fn opt_str<'a>(args: &'a Value, key: &str) -> Result<Option<&'a str>, AwareError> {
    match tri_str(args, key)? {
        Tri::Value(s) => Ok(Some(s)),
        Tri::Absent | Tri::Null => Ok(None),
    }
}

/// An optional whole number in `min..=max`, given as a number or a decimal
/// string (what `{{ }}` renders a number to).
fn opt_count(args: &Value, key: &str, min: u64, max: u64) -> Result<Option<u64>, AwareError> {
    let n = match args.get(key) {
        None | Some(Value::Null) => return Ok(None),
        Some(Value::String(s)) if s.is_empty() => return Ok(None),
        Some(Value::String(s)) => s.trim().parse::<u64>().ok(),
        Some(Value::Number(n)) => n.as_u64(),
        Some(_) => None,
    };
    match n {
        Some(n) if (min..=max).contains(&n) => Ok(Some(n)),
        _ => Err(invalid(format!(
            "`{key}` must be a whole number from {min}{}",
            if max == u64::MAX {
                " up".to_string()
            } else {
                format!(" to {max}")
            }
        ))),
    }
}

/// An optional JSON input: the value itself, or its JSON encoding as a string.
fn decoded(v: &Value, key: &str) -> Result<Value, AwareError> {
    match v {
        Value::String(s) => serde_json::from_str(s)
            .map_err(|_| invalid(format!("`{key}` is a string that is not JSON"))),
        other => Ok(other.clone()),
    }
}

fn opt_json(args: &Value, key: &str) -> Result<Option<Value>, AwareError> {
    match args.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) if s.is_empty() => Ok(None),
        Some(v) => decoded(v, key).map(Some),
    }
}

fn opt_object(args: &Value, key: &str) -> Result<Option<Value>, AwareError> {
    match opt_json(args, key)? {
        Some(v @ Value::Object(_)) => Ok(Some(v)),
        Some(_) => Err(invalid(format!("`{key}` must be an object"))),
        None => Ok(None),
    }
}

fn opt_list(args: &Value, key: &str) -> Result<Option<Value>, AwareError> {
    match opt_json(args, key)? {
        Some(v @ Value::Array(_)) => Ok(Some(v)),
        Some(_) => Err(invalid(format!("`{key}` must be a list"))),
        None => Ok(None),
    }
}

fn missing(key: &str) -> AwareError {
    AwareError::Validation(format!("trimble-connect: missing required input `{key}`"))
}

fn invalid(msg: String) -> AwareError {
    AwareError::Validation(format!("trimble-connect: {msg}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::trimble_files::test_support::{
        agents_with, dead_base, drain, mock_agents, next_request,
    };
    use crate::runtime::trimble_ops::tests::assert_declared;
    use std::sync::mpsc;

    /// The policy the wire mock needs: plain http on loopback. Production never
    /// sees it — [`invoke`] always passes [`Policy::PROD`].
    const LOOPBACK: Policy = Policy {
        allow: |u| u.scheme() == "http" && u.host_str() == Some("127.0.0.1"),
        timeout: Duration::from_secs(20),
        backoff: Duration::from_millis(1),
        max_wait: Duration::from_millis(5),
    };

    /// A route: request-line substring, status, extra headers, body bytes.
    type Route = (&'static str, u16, Vec<(&'static str, String)>, Vec<u8>);

    /// Like `mock_routed`, but a route can set response headers (`next`,
    /// `Content-Range`, `Location`) and a binary body.
    fn mock<F>(count: usize, routes_fn: F) -> (String, mpsc::Receiver<String>)
    where
        F: FnOnce(&str) -> Vec<Route> + Send + 'static,
    {
        use std::io::{Read as _, Write as _};
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let base = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
        let for_thread = base.clone();
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let routes = routes_fn(&for_thread);
            for _ in 0..count {
                let Ok((mut stream, _)) = listener.accept() else {
                    break;
                };
                stream
                    .set_read_timeout(Some(Duration::from_millis(250)))
                    .ok();
                let mut data = Vec::new();
                let mut tmp = [0u8; 4096];
                while let Ok(n) = stream.read(&mut tmp) {
                    if n == 0 {
                        break;
                    }
                    data.extend_from_slice(&tmp[..n]);
                }
                let req = String::from_utf8_lossy(&data).to_string();
                let line = req.lines().next().unwrap_or("").to_string();
                let (status, headers, body) = routes
                    .iter()
                    .find(|(pat, ..)| line.contains(pat))
                    .map(|(_, s, h, b)| (*s, h.clone(), b.clone()))
                    .unwrap_or((404, vec![], b"{}".to_vec()));
                let mut head = format!("HTTP/1.1 {status} X\r\n");
                if !headers
                    .iter()
                    .any(|(k, _)| k.eq_ignore_ascii_case("content-type"))
                {
                    head.push_str("Content-Type: application/json\r\n");
                }
                for (k, v) in &headers {
                    head.push_str(&format!("{k}: {v}\r\n"));
                }
                head.push_str(&format!(
                    "Content-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                ));
                stream.write_all(head.as_bytes()).ok();
                stream.write_all(&body).ok();
                tx.send(req).ok();
            }
        });
        (base, rx)
    }

    fn json_route(pat: &'static str, status: u16, body: &str) -> Route {
        (pat, status, vec![], body.as_bytes().to_vec())
    }

    /// Project P is in North America; the regions list maps that to the mock.
    fn discovery(base: &str) -> Vec<Route> {
        vec![
            json_route(
                "GET /projects/P ",
                200,
                r#"{"id":"P","name":"Take 5","location":"northAmerica"}"#,
            ),
            json_route(
                "GET /regions",
                200,
                &format!(
                    r#"[{{"location":"europe","topic-api":"https://open21.connect.trimble.com/"}},
                        {{"location":"northAmerica","topic-api":"{base}/"}}]"#
                ),
            ),
        ]
    }

    async fn run(base: &str, command: &str, args: Value) -> Result<Value, AwareError> {
        let agents = mock_agents(base);
        invoke_with(
            AgentCatalogue::from(agents.path().join("agents")),
            command,
            args,
            LOOPBACK,
        )
        .await
    }

    fn line(req: &str) -> &str {
        req.lines().next().unwrap_or_default()
    }

    fn body_of(req: &str) -> Value {
        let body = req.split("\r\n\r\n").nth(1).unwrap_or_default();
        serde_json::from_str(body).unwrap_or_else(|e| panic!("body not JSON ({e}): {req}"))
    }

    /// One PNG chunk: length, type, data, CRC.
    fn chunk(kind: &[u8], data: &[u8]) -> Vec<u8> {
        let mut c = (data.len() as u32).to_be_bytes().to_vec();
        c.extend_from_slice(kind);
        c.extend_from_slice(data);
        let mut crc_input = kind.to_vec();
        crc_input.extend_from_slice(data);
        c.extend_from_slice(&crc32(&crc_input).to_be_bytes());
        c
    }

    /// A complete 1×1 PNG: signature, IHDR, IDAT (a zlib stream of one
    /// filtered RGBA pixel), IEND.
    fn png() -> Vec<u8> {
        let mut b = b"\x89PNG\r\n\x1a\n".to_vec();
        b.extend(chunk(b"IHDR", &[0, 0, 0, 1, 0, 0, 0, 1, 8, 6, 0, 0, 0]));
        b.extend(chunk(
            b"IDAT",
            &[
                0x78, 0x9c, 0x63, 0x60, 0x60, 0x60, 0xf8, 0x0f, 0x00, 0x01, 0x04, 0x01, 0x00,
            ],
        ));
        b.extend(chunk(b"IEND", &[]));
        b
    }

    /// A structurally complete 1×1 greyscale baseline JPEG: SOI, SOF0, SOS, one
    /// byte of coded data, EOI.
    fn jpeg() -> Vec<u8> {
        let mut b = vec![0xFF, 0xD8];
        b.extend([0xFF, 0xC0, 0x00, 0x0B, 8, 0, 1, 0, 1, 1, 1, 0x11, 0]);
        b.extend([0xFF, 0xDA, 0x00, 0x08, 1, 1, 0x00, 0, 63, 0]);
        b.extend([0x00, 0xFF, 0xD9]);
        b
    }

    /// A real JPEG: the thumbnail Trimble Connect generated for a test viewpoint
    /// (live, 2026-10-11) — APP0, DQT, SOF0, DHT and SOS segments as encoders write them.
    const TC_THUMBNAIL: &str = concat!(
        "/9j/4AAQSkZJRgABAgAAAQABAAD/2wBDAAsICAoIBwsKCQoNDAsNERwSEQ8PESIZGhQcKSQrKigk",
        "JyctMkA3LTA9MCcnOEw5PUNFSElIKzZPVU5GVEBHSEX/2wBDAQwNDREPESESEiFFLicuRUVFRUVF",
        "RUVFRUVFRUVFRUVFRUVFRUVFRUVFRUVFRUVFRUVFRUVFRUVFRUVFRUVFRUX/wAARCACgAKADASIA",
        "AhEBAxEB/8QAHwAAAQUBAQEBAQEAAAAAAAAAAAECAwQFBgcICQoL/8QAtRAAAgEDAwIEAwUFBAQA",
        "AAF9AQIDAAQRBRIhMUEGE1FhByJxFDKBkaEII0KxwRVS0fAkM2JyggkKFhcYGRolJicoKSo0NTY3",
        "ODk6Q0RFRkdISUpTVFVWV1hZWmNkZWZnaGlqc3R1dnd4eXqDhIWGh4iJipKTlJWWl5iZmqKjpKWm",
        "p6ipqrKztLW2t7i5usLDxMXGx8jJytLT1NXW19jZ2uHi4+Tl5ufo6erx8vP09fb3+Pn6/8QAHwEA",
        "AwEBAQEBAQEBAQAAAAAAAAECAwQFBgcICQoL/8QAtREAAgECBAQDBAcFBAQAAQJ3AAECAxEEBSEx",
        "BhJBUQdhcRMiMoEIFEKRobHBCSMzUvAVYnLRChYkNOEl8RcYGRomJygpKjU2Nzg5OkNERUZHSElK",
        "U1RVVldYWVpjZGVmZ2hpanN0dXZ3eHl6goOEhYaHiImKkpOUlZaXmJmaoqOkpaanqKmqsrO0tba3",
        "uLm6wsPExcbHyMnK0tPU1dbX2Nna4uPk5ebn6Onq8vP09fb3+Pn6/9oADAMBAAIRAxEAPwDmaKKK",
        "8I/VQooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKK",
        "KKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooo",
        "oAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiig",
        "AooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKAC",
        "iiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKK",
        "KKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooo",
        "oAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiig",
        "D//Z",
    );

    #[test]
    fn only_a_complete_jpeg_is_a_jpeg() {
        assert_eq!(image_kind(&jpeg()), Some("jpg"));
        let real = base64::engine::general_purpose::STANDARD
            .decode(TC_THUMBNAIL)
            .unwrap();
        assert_eq!(image_kind(&real), Some("jpg"), "a real encoder's JPEG");
        assert_eq!(image_kind(&[0xFF, 0xD8, 0xFF, 0xD9]), None, "markers only");
        let whole = jpeg();
        let no_frame: Vec<u8> = [&whole[..2], &whole[15..]].concat();
        assert_eq!(image_kind(&no_frame), None, "a scan with no frame header");
        let no_scan: Vec<u8> = [&whole[..15], &[0xFF, 0xD9][..]].concat();
        assert_eq!(image_kind(&no_scan), None, "a frame with no scan");
        let mut zero_size = whole.clone();
        zero_size[7] = 0; // height 1 → 0
        zero_size[8] = 0;
        assert_eq!(image_kind(&zero_size), None, "zero height");
        let mut overrun = whole.clone();
        overrun[5] = 0x40; // SOF length runs past the end
        assert_eq!(image_kind(&overrun), None, "segment past the end");
        let no_data: Vec<u8> = [&whole[..25], &[0xFF, 0xD9][..]].concat();
        assert_eq!(image_kind(&no_data), None, "no coded data");
        let cut = whole[..whole.len() - 1].to_vec();
        assert_eq!(image_kind(&cut), None, "no EOI");
    }

    #[test]
    fn only_a_complete_png_is_a_png() {
        // Known answer: the CRC every PNG's IEND chunk carries.
        assert_eq!(crc32(b"IEND"), 0xAE42_6082);
        let whole = png();
        assert_eq!(image_kind(&whole), Some("png"));
        let header_only = whole[..33].to_vec();
        assert_eq!(image_kind(&header_only), None, "signature + IHDR only");
        let cut = whole[..whole.len() - 12].to_vec();
        assert_eq!(image_kind(&cut), None, "no IEND");
        let mut bad_crc = whole.clone();
        let n = bad_crc.len();
        bad_crc[n - 20] ^= 0xFF; // inside IDAT's data
        assert_eq!(image_kind(&bad_crc), None, "corrupt chunk");
        let mut trailing = whole.clone();
        trailing.push(0);
        assert_eq!(image_kind(&trailing), None, "bytes after IEND");
        let mut no_idat = b"\x89PNG\r\n\x1a\n".to_vec();
        no_idat.extend(chunk(b"IHDR", &[0, 0, 0, 1, 0, 0, 0, 1, 8, 6, 0, 0, 0]));
        no_idat.extend(chunk(b"IEND", &[]));
        assert_eq!(image_kind(&no_idat), None, "no image data");
    }

    fn b64(b: &[u8]) -> String {
        base64::engine::general_purpose::STANDARD.encode(b)
    }

    fn camera() -> Value {
        json!({"camera_view_point": {"x": 0, "y": 0, "z": 0},
               "camera_direction": {"x": 1, "y": 0, "z": 0},
               "camera_up_vector": {"x": 0, "y": 0, "z": 1},
               "field_of_view": 60})
    }

    const TOPIC: &str = r#"{"guid":"T1","title":"Clash at B4","topic_status":"Open","version":3,
        "creation_date":"2026-10-11T00:00:00Z","creation_author":"a@b.c","creation_author_uuid":"U"}"#;

    #[test]
    fn inputs_are_validated_before_any_request() {
        let cam = camera();
        let mut zero_dir = cam.clone();
        zero_dir["camera_direction"] = json!({"x": 0, "y": 0, "z": 0});
        let mut wide = cam.clone();
        wide["field_of_view"] = json!(180);
        let truncated = png()[..20].to_vec();
        let pt = |extra: Value| {
            let mut a = json!({"project-id": "P", "topic-id": "T"});
            a.as_object_mut()
                .unwrap()
                .extend(extra.as_object().unwrap().clone());
            a
        };
        let cases: Vec<(&str, Value, &str)> = vec![
            ("list-topics", json!({}), "project-id"),
            (
                "list-topics",
                json!({"project-id": "P", "top": 0}),
                "`top` must be",
            ),
            (
                "list-topics",
                json!({"project-id": "P", "top": "ten"}),
                "`top` must be",
            ),
            ("list-comments", json!({"project-id": "P"}), "topic-id"),
            ("get-topic", pt(json!({"topic-id": " "})), "topic-id"),
            ("create-topic", json!({"project-id": "P"}), "`title`"),
            (
                "create-topic",
                json!({"project-id": "P", "title": "  "}),
                "blank",
            ),
            (
                "create-topic",
                json!({"project-id": "P", "title": "t", "labels": "x"}),
                "not JSON",
            ),
            (
                "create-topic",
                json!({"project-id": "P", "title": "t", "labels": [""]}),
                "`labels` must be",
            ),
            ("update-topic", pt(json!({})), "at least one field"),
            (
                "update-topic",
                pt(json!({"description": ""})),
                "at least one field",
            ),
            (
                "update-topic",
                pt(json!({"title": null})),
                "`title` cannot be cleared",
            ),
            (
                "update-topic",
                pt(json!({"topic-status": null})),
                "cannot be cleared",
            ),
            (
                "update-topic",
                pt(json!({"due-date": null})),
                "`due-date` cannot be cleared",
            ),
            (
                "update-topic",
                pt(json!({"assigned-to": null})),
                "`assigned-to` cannot be cleared",
            ),
            (
                "update-topic",
                pt(json!({"guid": "G", "title": "x"})),
                "`guid`",
            ),
            ("create-comment", pt(json!({})), "`comment`"),
            ("create-comment", pt(json!({"comment": " "})), "`comment`"),
            ("create-viewpoint", pt(json!({})), "needs a camera"),
            (
                "create-viewpoint",
                pt(json!({"perspective-camera": cam, "orthogonal-camera": cam})),
                "not both",
            ),
            (
                "create-viewpoint",
                pt(json!({"perspective-camera": {}})),
                "camera_view_point",
            ),
            (
                "create-viewpoint",
                pt(json!({"perspective-camera": zero_dir})),
                "zero vector",
            ),
            (
                "create-viewpoint",
                pt(json!({"perspective-camera": wide})),
                "field_of_view",
            ),
            (
                "create-viewpoint",
                pt(json!({"orthogonal-camera": cam})),
                "view_to_world_scale",
            ),
            (
                "create-viewpoint",
                pt(json!({"perspective-camera": cam, "lines": [{"start_point": {"x": 0}}]})),
                "lines[0].start_point",
            ),
            (
                "create-viewpoint",
                pt(json!({"perspective-camera": cam, "clipping-planes": [{
                    "location": {"x": 0, "y": 0, "z": 0},
                    "direction": {"x": 0, "y": 0, "z": 0}}]})),
                "zero vector",
            ),
            (
                "create-viewpoint",
                pt(json!({"perspective-camera": cam, "components": {"selection": {}}})),
                "components.selection",
            ),
            (
                "create-viewpoint",
                pt(json!({"perspective-camera": cam, "components": {"visibility": []}})),
                "components.visibility",
            ),
            (
                "create-viewpoint",
                pt(json!({"perspective-camera": "{not json"})),
                "not JSON",
            ),
            ("create-viewpoint", pt(json!({"snapshot": "%%%"})), "base64"),
            (
                "create-viewpoint",
                pt(json!({"snapshot": b64(&truncated)})),
                "not a complete PNG",
            ),
            (
                "create-viewpoint",
                pt(json!({"snapshot": b64(&png()), "snapshot-type": "jpg"})),
                "but the snapshot is a png",
            ),
            (
                "create-viewpoint",
                pt(json!({"perspective-camera": cam, "snapshot-type": "png"})),
                "without a `snapshot`",
            ),
            ("get-viewpoint-snapshot", pt(json!({})), "viewpoint-id"),
            ("create-document-reference", pt(json!({})), "exactly one"),
            (
                "create-document-reference",
                pt(json!({"url": "u", "document-id": "d"})),
                "exactly one",
            ),
            ("list-todos", json!({}), "project-id"),
            (
                "list-todos",
                json!({"project-id": "P", "version-id": "V"}),
                "needs `file-id`",
            ),
            (
                "list-todos",
                json!({"project-id": "P", "range": "0-9"}),
                "`range` must look like",
            ),
            ("create-todo", json!({"project-id": "P"}), "`description`"),
            ("create-todo", json!({"description": "d"}), "project-id"),
            (
                "create-todo",
                json!({"project-id": "P", "description": "d", "status": "DONE"}),
                "`status` must be one of",
            ),
            (
                "create-todo",
                json!({"project-id": "P", "description": "d", "priority": "URGENT"}),
                "`priority` must be one of",
            ),
            (
                "create-todo",
                json!({"project-id": "P", "description": "d", "percent-complete": 101}),
                "`percent-complete`",
            ),
            (
                "create-todo",
                json!({"project-id": "P", "description": "d",
                       "assignees": [{"id": "u", "type": "ROBOT"}]}),
                "USER or GROUP",
            ),
            (
                "create-todo",
                json!({"project-id": "P", "description": "d", "assignees": [3]}),
                "each assignee",
            ),
            ("update-todo", json!({"todo-id": "D"}), "at least one field"),
            (
                "update-todo",
                json!({"todo-id": "D", "description": null}),
                "cannot be cleared",
            ),
            (
                "update-todo",
                json!({"todo-id": "D", "label": null}),
                "`label` cannot be cleared",
            ),
            (
                "update-todo",
                json!({"todo-id": "D", "project-id": "P2", "title": "x"}),
                "another project",
            ),
            ("update-todo", json!({"title": "x"}), "todo-id"),
            (
                "update-todo",
                json!({"todo-id": "D", "title": "x", "status": null}),
                "`status` cannot be cleared",
            ),
            (
                "update-todo",
                json!({"todo-id": "D", "title": "x", "priority": null}),
                "`priority` cannot be cleared",
            ),
            (
                "update-todo",
                json!({"todo-id": "D", "title": "x", "percent-complete": null}),
                "`percent-complete` cannot be cleared",
            ),
            ("delete-todo", json!({}), "todo-id"),
        ];
        for (cmd, args, want) in &cases {
            let err = Op::parse(cmd, args).unwrap_err();
            assert!(
                matches!(err, AwareError::Validation(_)),
                "{cmd} {args}: {err}"
            );
            assert!(
                err.to_string().contains(want),
                "{cmd} {args}: want {want:?}, got {err}"
            );
        }
    }

    #[test]
    fn an_oversized_snapshot_is_refused_before_it_is_decoded() {
        // Not valid base64 at all — the length check must fire first.
        let huge = "!".repeat(MAX_SNAPSHOT_BYTES / 3 * 4 + 64);
        let err = Op::parse(
            "create-viewpoint",
            &json!({"project-id": "P", "topic-id": "T", "snapshot": huge}),
        )
        .unwrap_err();
        assert!(err.to_string().contains("larger than 20 MiB"), "{err}");
    }

    #[test]
    fn only_the_five_published_topics_hosts_may_receive_the_token() {
        for ok in TOPICS_HOSTS {
            let url = url::Url::parse(&format!("https://{ok}/")).unwrap();
            assert!(prod_topics_host(&url), "{ok}");
        }
        for bad in [
            "http://open11.connect.trimble.com/",
            "https://open11.connect.trimble.com:8443/",
            "https://evil.connect.trimble.com/",
            "https://open11.connect.trimble.com.evil.example/",
            "https://user:pw@open11.connect.trimble.com/",
            "https://app.connect.trimble.com/",
            "http://127.0.0.1/",
        ] {
            assert!(
                !prod_topics_host(&url::Url::parse(bad).unwrap()),
                "{bad} must not be allowed"
            );
        }
    }

    #[tokio::test]
    async fn bad_input_is_refused_before_the_credential_is_needed() {
        let agents = agents_with(
            &format!(
                "agent: trimble-connect\nversion: 0.1.0\ndescription: x\nstateful: false\n\
                 license: MIT\ntransport:\n  rest:\n    base: {}/\nauth:\n  scheme: oauth2\n  \
                 secret: mock-tc-tok\ncommands: {{}}\n",
                dead_base()
            ),
            None,
        );
        let err = invoke(
            AgentCatalogue::from(agents.path().join("agents")),
            "create-topic",
            json!({"project-id": "P"}),
        )
        .await
        .unwrap_err();
        assert!(err.to_string().contains("`title`"), "{err}");
    }

    #[tokio::test]
    async fn a_list_resolves_the_region_then_pages_with_top_and_skiptoken() {
        let (base, rx) = mock(4, |b| {
            let mut r = discovery(b);
            r.push((
                "GET /bcf/2.1/projects/P/topics",
                200,
                vec![(
                    "next",
                    format!("{b}/bcf/2.1/projects/P/topics?top=2&skiptoken=q1%2Bby"),
                )],
                format!("[{TOPIC}]").into_bytes(),
            ));
            r
        });
        let out = run(
            &base,
            "list-topics",
            json!({"project-id": "P", "top": "2", "skiptoken": "a b"}),
        )
        .await
        .unwrap();
        assert_eq!(out["items"][0]["guid"], "T1");
        assert_eq!(out["next-skiptoken"], "q1+by");
        let project = next_request(&rx);
        assert!(line(&project).starts_with("GET /projects/P "), "{project}");
        assert!(project.contains("Bearer TESTTOKEN"), "{project}");
        let regions = next_request(&rx);
        assert!(line(&regions).starts_with("GET /regions"), "{regions}");
        assert!(
            !regions.contains("Authorization"),
            "regions is public — no token: {regions}"
        );
        let list = next_request(&rx);
        assert!(
            line(&list).contains("/bcf/2.1/projects/P/topics?top=2&skiptoken=a%20b"),
            "{list}"
        );
        assert!(list.contains("Bearer TESTTOKEN"), "{list}");
    }

    #[tokio::test]
    async fn an_empty_list_and_the_last_page_have_no_next_skiptoken() {
        let (base, _rx) = mock(4, |b| {
            let mut r = discovery(b);
            r.push((
                "GET /bcf/2.1/projects/P/topics/T/comments",
                204,
                vec![],
                vec![],
            ));
            r
        });
        let out = run(
            &base,
            "list-comments",
            json!({"project-id": "P", "topic-id": "T"}),
        )
        .await
        .unwrap();
        assert_eq!(out, json!({"items": [], "next-skiptoken": null}));
    }

    #[tokio::test]
    async fn a_next_link_without_a_skiptoken_is_an_error_not_a_last_page() {
        let (base, _rx) = mock(4, |b| {
            let mut r = discovery(b);
            r.push((
                "GET /bcf/2.1/projects/P/topics/T/viewpoints",
                200,
                vec![(
                    "next",
                    "/bcf/2.1/projects/P/topics/T/viewpoints?page=2".into(),
                )],
                b"[]".to_vec(),
            ));
            r
        });
        let err = run(
            &base,
            "list-viewpoints",
            json!({"project-id": "P", "topic-id": "T"}),
        )
        .await
        .unwrap_err();
        assert!(err.to_string().contains("carries no skiptoken"), "{err}");
    }

    #[tokio::test]
    async fn the_token_is_never_sent_to_a_host_outside_the_policy() {
        // The production policy: the regions list names the mock (http,
        // loopback), which is not a published Topics host — refused before any
        // BCF request.
        let (base, rx) = mock(4, discovery);
        let agents = mock_agents(&base);
        let err = invoke(
            AgentCatalogue::from(agents.path().join("agents")),
            "list-topics",
            json!({"project-id": "P"}),
        )
        .await
        .unwrap_err();
        assert!(err.to_string().contains("not a known"), "{err}");
        let seen = drain(&rx);
        assert_eq!(seen.len(), 2, "project + regions only: {seen:#?}");

        // A project whose region is missing from the list is an error too.
        let (base, _rx) = mock(4, |b| {
            let mut r = discovery(b);
            r[0] = json_route("GET /projects/P ", 200, r#"{"id":"P","location":"mars"}"#);
            r
        });
        let err = run(&base, "list-topics", json!({"project-id": "P"}))
            .await
            .unwrap_err();
        assert!(err.to_string().contains("no Topics API"), "{err}");
    }

    #[tokio::test]
    async fn a_redirect_is_never_followed() {
        let (base, rx) = mock(5, |b| {
            let mut r = discovery(b);
            r.push((
                "GET /bcf/2.1/projects/P/topics/T ",
                302,
                vec![("Location", format!("{b}/elsewhere"))],
                vec![],
            ));
            r.push(json_route("GET /elsewhere", 200, TOPIC));
            r
        });
        let err = run(
            &base,
            "get-topic",
            json!({"project-id": "P", "topic-id": "T"}),
        )
        .await
        .unwrap_err();
        assert!(
            err.to_string().contains("redirects are not followed"),
            "{err}"
        );
        let seen = drain(&rx);
        assert_eq!(
            seen.len(),
            3,
            "no request to the redirect target: {seen:#?}"
        );

        // Discovery itself does not follow one either.
        let (base, rx) = mock(4, |b| {
            vec![(
                "GET /projects/P ",
                301,
                vec![("Location", format!("{b}/projects/Q"))],
                vec![],
            )]
        });
        let err = run(&base, "list-topics", json!({"project-id": "P"}))
            .await
            .unwrap_err();
        assert!(
            err.to_string().contains("redirects are not followed"),
            "{err}"
        );
        assert_eq!(drain(&rx).len(), 1);
    }

    #[tokio::test]
    async fn create_topic_posts_snake_case_and_reports_the_guid() {
        let (base, rx) = mock(4, |b| {
            let mut r = discovery(b);
            r.push(json_route("POST /bcf/2.1/projects/P/topics ", 201, TOPIC));
            r
        });
        let out = run(
            &base,
            "create-topic",
            json!({"project-id": "P", "title": "Clash at B4", "topic-type": "Clash",
                   "labels": "[\"steel\"]", "assigned-to": "a@b.c", "due-date": "",
                   "guid": "T1"}),
        )
        .await
        .unwrap();
        assert_eq!(out["topic-id"], "T1");
        let seen = drain(&rx);
        assert_eq!(
            body_of(&seen[2]),
            json!({"guid": "T1", "title": "Clash at B4", "topic_type": "Clash",
                   "assigned_to": "a@b.c", "labels": ["steel"]})
        );
    }

    #[tokio::test]
    async fn update_topic_resends_every_writable_field_with_the_changes_laid_over() {
        let full = r#"{"guid":"T1","version":3,"creation_date":"d","creation_author":"a",
            "creation_author_uuid":"u","modified_date":"m","viewpoint":{"guid":"V"},
            "files":[],"related_topics":[],
            "topic_type":"Clash","topic_status":"Open","reference_links":["r"],"title":"Old",
            "priority":"High","index":7,"labels":["a"],"assigned_to":"x@y.z",
            "assigned_to_uuid":"xu","assignees":[{"id":"x"}],"stage":"S1","description":"D",
            "due_date":"2026-12-01","marker":{"m":1},"bim_snippet":{"s":1}}"#;
        let (base, rx) = mock(5, move |b| {
            let mut r = discovery(b);
            r.push(json_route("GET /bcf/2.1/projects/P/topics/T1 ", 200, full));
            r.push(json_route("PUT /bcf/2.1/projects/P/topics/T1 ", 200, TOPIC));
            r
        });
        let out = run(
            &base,
            "update-topic",
            json!({"project-id": "P", "topic-id": "T1", "title": "New",
                   "description": null, "labels": null, "stage": ""}),
        )
        .await
        .unwrap();
        assert_eq!(out["changed"], json!(true));
        let seen = drain(&rx);
        assert_eq!(seen.len(), 4, "{seen:#?}");
        assert!(line(&seen[3]).starts_with("PUT "), "{}", seen[3]);
        // Every PUT-schema field the topic had, untouched ones included; none of
        // the server-owned fields (guid, version, creation_*, viewpoint, …).
        assert_eq!(
            body_of(&seen[3]),
            json!({"topic_type": "Clash", "topic_status": "Open", "reference_links": ["r"],
                   "title": "New", "priority": "High", "index": 7, "labels": [],
                   "assigned_to": "x@y.z", "assigned_to_uuid": "xu",
                   "assignees": [{"id": "x"}], "stage": "S1", "description": "",
                   "due_date": "2026-12-01", "marker": {"m": 1}, "bim_snippet": {"s": 1}})
        );
    }

    #[tokio::test]
    async fn reassigning_a_topic_drops_the_assignees_that_would_override_it() {
        let assigned = r#"{"guid":"T1","title":"t","assigned_to":"old@x.y",
            "assigned_to_uuid":"old-uuid","assignees":[{"id":"old-uuid","type":"USER"}]}"#;
        let (base, rx) = mock(5, move |b| {
            let mut r = discovery(b);
            r.push(json_route(
                "GET /bcf/2.1/projects/P/topics/T1 ",
                200,
                assigned,
            ));
            r.push(json_route("PUT /bcf/2.1/projects/P/topics/T1 ", 200, TOPIC));
            r
        });
        run(
            &base,
            "update-topic",
            json!({"project-id": "P", "topic-id": "T1", "assigned-to": "new@x.y"}),
        )
        .await
        .unwrap();
        let seen = drain(&rx);
        assert_eq!(
            body_of(&seen[3]),
            json!({"title": "t", "assigned_to": "new@x.y"})
        );

        // Naming the current assignee again is no change: nothing is dropped or written.
        let (base, rx) = mock(5, move |b| {
            let mut r = discovery(b);
            r.push(json_route(
                "GET /bcf/2.1/projects/P/topics/T1 ",
                200,
                assigned,
            ));
            r
        });
        let out = run(
            &base,
            "update-topic",
            json!({"project-id": "P", "topic-id": "T1", "assigned-to": "old@x.y"}),
        )
        .await
        .unwrap();
        assert_eq!(out["changed"], json!(false));
        assert_eq!(drain(&rx).len(), 3, "no PUT");
    }

    #[tokio::test]
    async fn update_topic_writes_nothing_when_nothing_changes() {
        let (base, rx) = mock(5, |b| {
            let mut r = discovery(b);
            r.push(json_route("GET /bcf/2.1/projects/P/topics/T1 ", 200, TOPIC));
            r
        });
        let out = run(
            &base,
            "update-topic",
            json!({"project-id": "P", "topic-id": "T1", "topic-status": "Open"}),
        )
        .await
        .unwrap();
        assert_eq!(out["changed"], json!(false));
        assert_eq!(out["topic-id"], "T1");
        assert_eq!(drain(&rx).len(), 3, "no PUT");
    }

    #[tokio::test]
    async fn a_viewpoint_carries_its_snapshot_and_the_snapshot_comes_back_as_base64() {
        let image = png();
        let cam = camera();
        let (base, rx) = mock(4, |b| {
            let mut r = discovery(b);
            r.push(json_route(
                "POST /bcf/2.1/projects/P/topics/T/viewpoints ",
                201,
                r#"{"guid":"V1"}"#,
            ));
            r
        });
        let out = run(
            &base,
            "create-viewpoint",
            json!({"project-id": "P", "topic-id": "T",
                   "perspective-camera": cam.to_string(), "snapshot": b64(&image)}),
        )
        .await
        .unwrap();
        assert_eq!(out["viewpoint-id"], "V1");
        let seen = drain(&rx);
        assert_eq!(
            body_of(&seen[2]),
            json!({"perspective_camera": cam,
                   "snapshot": {"snapshot_type": "png", "snapshot_data": b64(&image)}})
        );

        let served = image.clone();
        let (base, _rx) = mock(4, move |b| {
            let mut r = discovery(b);
            r.push((
                "GET /bcf/2.1/projects/P/topics/T/viewpoints/V1/snapshot",
                200,
                // Mislabelled, as TC was seen doing: the bytes decide.
                vec![("Content-Type", "image/jpeg".into())],
                served,
            ));
            r
        });
        let out = run(
            &base,
            "get-viewpoint-snapshot",
            json!({"project-id": "P", "topic-id": "T", "viewpoint-id": "V1"}),
        )
        .await
        .unwrap();
        assert_eq!(out["bytes"], b64(&image));
        assert_eq!(out["content-type"], "image/png");

        // A cut-off image is a failure, whatever the header claims.
        let cut = image[..40].to_vec();
        let (base, _rx) = mock(4, move |b| {
            let mut r = discovery(b);
            r.push((
                "GET /bcf/2.1/projects/P/topics/T/viewpoints/V1/snapshot",
                200,
                vec![("Content-Type", "image/png".into())],
                cut,
            ));
            r
        });
        let err = run(
            &base,
            "get-viewpoint-snapshot",
            json!({"project-id": "P", "topic-id": "T", "viewpoint-id": "V1"}),
        )
        .await
        .unwrap_err();
        assert!(
            err.to_string().contains("not a complete PNG or JPEG"),
            "{err}"
        );
    }

    #[tokio::test]
    async fn todos_speak_camel_case_on_the_core_base() {
        let todo = r#"{"id":"D1","title":"t","description":"d","status":"NEW"}"#;
        let (base, rx) = mock(2, move |_b| vec![json_route("POST /todos ", 201, todo)]);
        let out = run(
            &base,
            "create-todo",
            json!({"project-id": "P", "description": "d", "title": "t", "status": "new",
                   "priority": "high", "due-date": "2026-12-01T00:00:00Z",
                   "percent-complete": "40",
                   "assignees": ["u1", {"id": "g1", "type": "group"}]}),
        )
        .await
        .unwrap();
        assert_eq!(out["todo-id"], "D1");
        let req = next_request(&rx);
        assert!(req.contains("Bearer TESTTOKEN"), "{req}");
        assert_eq!(
            body_of(&req),
            json!({"title": "t", "description": "d", "dueDate": "2026-12-01T00:00:00+0000",
                   "status": "NEW", "priority": "HIGH", "percentComplete": 40,
                   "assignees": [{"id": "u1", "type": "USER"}, {"id": "g1", "type": "GROUP"}],
                   "projectId": "P"})
        );

        let (base, rx) = mock(2, move |_b| vec![json_route("PATCH /todos/D1 ", 200, todo)]);
        run(
            &base,
            "update-todo",
            json!({"todo-id": "D1", "status": "RESOLVED", "due-date": null, "assignees": null}),
        )
        .await
        .unwrap();
        assert_eq!(
            body_of(&next_request(&rx)),
            json!({"dueDate": null, "status": "RESOLVED", "assignees": []})
        );

        let (base, rx) = mock(2, |_b| vec![json_route("DELETE /todos/D1 ", 204, "")]);
        let out = run(&base, "delete-todo", json!({"todo-id": "D1"}))
            .await
            .unwrap();
        assert_eq!(out, json!({"todo-id": "D1", "deleted": true}));
        assert!(next_request(&rx).contains("Bearer TESTTOKEN"));
    }

    #[test]
    fn todo_dates_are_sent_in_the_form_tc_accepts() {
        for (given, sent) in [
            ("2026-12-31T00:00:00Z", "2026-12-31T00:00:00+0000"),
            ("2026-12-31T00:00:00.000Z", "2026-12-31T00:00:00+0000"),
            ("2026-12-31T10:00:00.5+01:00", "2026-12-31T10:00:00+01:00"),
            ("2026-12-31T00:00:00+0000", "2026-12-31T00:00:00+0000"),
            ("2026-12-31", "2026-12-31"),
        ] {
            assert_eq!(tc_datetime(given), sent, "{given}");
        }
        let op = Op::parse(
            "update-todo",
            &json!({"todo-id": "D", "due-date": "2026-12-31T00:00:00.000Z"}),
        )
        .unwrap();
        let Op::UpdateTodo { changes, .. } = op else {
            panic!("{op:?}")
        };
        assert_eq!(changes["dueDate"], "2026-12-31T00:00:00+0000");
    }

    #[tokio::test]
    async fn list_todos_filters_by_file_and_pages_with_range() {
        let (base, rx) = mock(2, |_b| {
            vec![(
                "GET /todos?",
                206,
                vec![("Content-Range", "items 0-1/5".into())],
                br#"[{"id":"a"},{"id":"b"}]"#.to_vec(),
            )]
        });
        let out = run(
            &base,
            "list-todos",
            json!({"project-id": "P 1", "file-id": "F", "version-id": "V",
                   "range": "items=0-1"}),
        )
        .await
        .unwrap();
        assert_eq!(out["partial"], json!(true));
        assert_eq!(out["content-range"], "items 0-1/5");
        let req = next_request(&rx);
        assert!(
            line(&req).contains("/todos?projectId=P%201&fileId=F&versionId=V"),
            "{req}"
        );
        assert!(req.contains("Range: items=0-1"), "{req}");
    }

    /// A mock whose route for `pat` answers `first` for the first `times`
    /// requests and `then` after that.
    fn flaky(
        pat: &'static str,
        first: u16,
        times: usize,
        then: (u16, &'static str),
    ) -> (String, mpsc::Receiver<String>) {
        use std::io::{Read as _, Write as _};
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let base = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let mut hits = 0;
            for _ in 0..6 {
                let Ok((mut stream, _)) = listener.accept() else {
                    break;
                };
                stream
                    .set_read_timeout(Some(Duration::from_millis(250)))
                    .ok();
                let mut data = Vec::new();
                let mut tmp = [0u8; 4096];
                while let Ok(n) = stream.read(&mut tmp) {
                    if n == 0 {
                        break;
                    }
                    data.extend_from_slice(&tmp[..n]);
                }
                let req = String::from_utf8_lossy(&data).to_string();
                let (status, body) = if req.lines().next().unwrap_or("").contains(pat) {
                    hits += 1;
                    if hits <= times {
                        (first, "{\"message\":\"slow down\"}")
                    } else {
                        then
                    }
                } else {
                    (404, "{}")
                };
                let resp = format!(
                    "HTTP/1.1 {status} X\r\nContent-Type: application/json\r\nRetry-After: 0\r\n\
                     Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                );
                stream.write_all(resp.as_bytes()).ok();
                tx.send(req).ok();
            }
        });
        (base, rx)
    }

    #[tokio::test]
    async fn throttling_is_retried_and_a_repeatable_read_survives_a_5xx() {
        // 429 twice, then success: three attempts, one result.
        let (base, rx) = flaky(
            "POST /todos ",
            429,
            2,
            (201, r#"{"id":"D1","description":"d"}"#),
        );
        let out = run(
            &base,
            "create-todo",
            json!({"project-id": "P", "description": "d"}),
        )
        .await
        .unwrap();
        assert_eq!(out["todo-id"], "D1");
        assert_eq!(drain(&rx).len(), 3);

        // A read rides out any 5xx — 503, and a proxy's 520 too.
        let (base, rx) = flaky("GET /todos?", 520, 1, (200, "[]"));
        run(&base, "list-todos", json!({"project-id": "P"}))
            .await
            .unwrap();
        assert_eq!(drain(&rx).len(), 2);
        let (base, rx) = flaky("GET /todos?", 503, 1, (200, "[]"));
        run(&base, "list-todos", json!({"project-id": "P"}))
            .await
            .unwrap();
        assert_eq!(drain(&rx).len(), 2);

        // Throttled on every attempt: fails after three, naming the count.
        let (base, rx) = flaky("GET /todos?", 429, 9, (200, "[]"));
        let err = run(&base, "list-todos", json!({"project-id": "P"}))
            .await
            .unwrap_err();
        assert!(
            err.to_string().contains("after 3 attempts") && err.to_string().contains("429"),
            "{err}"
        );
        assert_eq!(drain(&rx).len(), 3);
    }

    #[tokio::test]
    async fn a_write_that_may_have_landed_is_not_repeated() {
        // A 5xx on a POST might follow a created ToDo; it is not sent again.
        let (base, rx) = flaky(
            "POST /todos ",
            502,
            1,
            (201, r#"{"id":"D1","description":"d"}"#),
        );
        let err = run(
            &base,
            "create-todo",
            json!({"project-id": "P", "description": "d"}),
        )
        .await
        .unwrap_err();
        assert!(err.to_string().contains("502"), "{err}");
        assert_eq!(drain(&rx).len(), 1);

        // Nor is a DELETE.
        let (base, rx) = flaky("DELETE /todos/D1", 500, 1, (204, ""));
        run(&base, "delete-todo", json!({"todo-id": "D1"}))
            .await
            .unwrap_err();
        assert_eq!(drain(&rx).len(), 1);
    }

    #[tokio::test]
    async fn tc_refusals_name_the_step_and_status() {
        let (base, _rx) = mock(4, |b| {
            let mut r = discovery(b);
            r.push(json_route(
                "POST /bcf/2.1/projects/P/topics/T/comments",
                403,
                r#"{"message":"no access"}"#,
            ));
            r
        });
        let err = run(
            &base,
            "create-comment",
            json!({"project-id": "P", "topic-id": "T", "comment": "hi"}),
        )
        .await
        .unwrap_err();
        let text = err.to_string();
        assert!(
            text.contains("create comment") && text.contains("403") && text.contains("no access"),
            "{text}"
        );
        // A 2xx without the identifier a downstream node needs is a failure.
        let (base, _rx) = mock(2, |_b| {
            vec![json_route("POST /todos ", 201, r#"{"title":"t"}"#)]
        });
        let err = run(
            &base,
            "create-todo",
            json!({"project-id": "P", "description": "d"}),
        )
        .await
        .unwrap_err();
        assert!(err.to_string().contains("no id"), "{err}");
    }

    /// Every command this module handles, run once against the wire mock, emits
    /// only keys its manifest output schema declares — and none is skipped.
    #[tokio::test]
    async fn every_key_a_handler_emits_is_declared_in_the_manifest() {
        let mut ortho = camera();
        ortho.as_object_mut().unwrap().remove("field_of_view");
        ortho["view_to_world_scale"] = json!(2);
        let pt = |extra: Value| {
            let mut a = json!({"project-id": "P", "topic-id": "T"});
            a.as_object_mut()
                .unwrap()
                .extend(extra.as_object().unwrap().clone());
            a
        };
        let cases: Vec<(&str, Value)> = vec![
            ("list-topics", json!({"project-id": "P"})),
            ("get-topic", pt(json!({}))),
            ("create-topic", json!({"project-id": "P", "title": "t"})),
            ("update-topic", pt(json!({"title": "changed"}))),
            ("delete-topic", pt(json!({}))),
            ("list-comments", pt(json!({}))),
            ("create-comment", pt(json!({"comment": "c"}))),
            ("list-viewpoints", pt(json!({}))),
            ("create-viewpoint", pt(json!({"orthogonal-camera": ortho}))),
            ("get-viewpoint-snapshot", pt(json!({"viewpoint-id": "V"}))),
            ("list-document-references", pt(json!({}))),
            ("create-document-reference", pt(json!({"url": "https://x"}))),
            ("list-todos", json!({"project-id": "P"})),
            (
                "create-todo",
                json!({"project-id": "P", "description": "d"}),
            ),
            ("update-todo", json!({"todo-id": "D", "title": "t"})),
            ("delete-todo", json!({"todo-id": "D"})),
        ];
        let exercised: std::collections::BTreeSet<&str> = cases.iter().map(|(c, _)| *c).collect();
        let owned: std::collections::BTreeSet<&str> = COMMANDS.iter().copied().collect();
        assert_eq!(exercised, owned, "every handled command is exercised here");
        for (cmd, args) in cases {
            let image = png();
            let (base, _rx) = mock(6, move |b| {
                let mut r = discovery(b);
                r.push((
                    "/snapshot",
                    200,
                    vec![("Content-Type", "image/png".into())],
                    image,
                ));
                r.push(json_route("GET /bcf/2.1/projects/P/topics/T ", 200, TOPIC));
                r.push(json_route("PUT /bcf/2.1/projects/P/topics/T ", 200, TOPIC));
                r.push(json_route("DELETE /bcf/", 204, ""));
                r.push(json_route("GET /bcf/", 200, "[]"));
                r.push(json_route("POST /bcf/", 201, r#"{"guid":"G"}"#));
                r.push(json_route("GET /todos?", 200, "[]"));
                r.push(json_route("DELETE /todos/", 204, ""));
                r.push(json_route("/todos", 200, r#"{"id":"D"}"#));
                r
            });
            let out = run(&base, cmd, args)
                .await
                .unwrap_or_else(|e| panic!("{cmd}: {e}"));
            assert_declared(cmd, &out);
        }
    }
}
