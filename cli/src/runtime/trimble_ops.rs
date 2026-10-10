//! Trimble Connect folder / file management (floless.app#2184).
//!
//! The commands here need what the generic REST renderer cannot express: a JSON
//! body assembled from kebab-case inputs into TC's camelCase wire names
//! (`parent-id` → `parentId`), an optional `If-Match` header, a lookup before a
//! write (`create-folder` reuses an existing folder of the same name), and two
//! requests where TC honours only one (`update-*` with both a move and a rename).
//! So, like `upload`/`download` ([`super::trimble_files`]), they are handled
//! in-process and dispatched from [`super::invoker`]'s REST path. Auth (with
//! refresh, #198) and the base URL resolve exactly as the generic REST path does.
//!
//! Every endpoint, method and body shape follows Trimble's published Core API
//! 2.0 definition (`api.swaggerhub.com/apis/Trimble-Connect/tcps/2.0`). Writes
//! fail on a non-2xx with the step, status and TC's own message; the lookups
//! (`find-item`) report "not there" as data, since that is an answer, not a fault.

use serde_json::{Map, Value, json};

use crate::agent_resolution::AgentCatalogue;
use crate::error::AwareError;
use crate::runtime::invoker::percent_encode_path;
use crate::runtime::trimble_files::{
    auth_and_base, get_json, http_agent, json_body, ok_response, post_json, str_arg,
};

/// The trimble-connect commands this module owns.
const COMMANDS: &[&str] = &[
    "create-folder",
    "find-item",
    "list-folder-by-path",
    "update-folder",
    "delete-folder",
    "update-file",
    "delete-file",
    "copy-file",
];

/// Whether `command` is a trimble-connect command handled here.
pub fn handles(command: &str) -> bool {
    COMMANDS.contains(&command)
}

/// Run one of [`COMMANDS`]. Blocking HTTP runs off the reactor.
pub async fn invoke(
    catalogue: impl Into<AgentCatalogue>,
    command: &str,
    args: Value,
) -> Result<Value, AwareError> {
    let catalogue = catalogue.into();
    let command = command.to_string();
    tokio::task::spawn_blocking(move || invoke_blocking(&catalogue, &command, &args))
        .await
        .map_err(|e| AwareError::Internal(format!("trimble-connect task join: {e}")))?
}

fn invoke_blocking(
    catalogue: &AgentCatalogue,
    command: &str,
    args: &Value,
) -> Result<Value, AwareError> {
    // Validate the inputs before touching the credential, so a malformed node is
    // reported as such and never costs a token refresh or a request.
    let op = Op::parse(command, args)?;
    let (token, base) = auth_and_base(catalogue)?;
    let tc = Tc {
        agent: http_agent(),
        token,
        base,
    };
    match op {
        Op::CreateFolder {
            parent_id,
            name,
            reuse,
        } => create_folder(&tc, parent_id, name, reuse),
        Op::FindItem {
            folder_id,
            name,
            kind,
        } => find_item(&tc, folder_id, name, kind),
        Op::ListByPath { project_id, path } => list_folder_by_path(&tc, project_id, path),
        Op::Update {
            kind,
            id,
            name,
            parent_id,
            if_match,
        } => update(&tc, kind, id, name, parent_id, if_match),
        Op::Delete { kind, id, if_match } => delete(&tc, kind, id, if_match),
        Op::CopyFile {
            file_id,
            version_id,
            parent_id,
            merge_existing,
        } => copy_file(&tc, file_id, version_id, parent_id, merge_existing),
    }
}

/// FOLDER or FILE — the two item kinds TC's Files API manages.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Folder,
    File,
}

impl Kind {
    /// The collection segment: `folders/{id}` or `files/{id}`.
    fn collection(self) -> &'static str {
        match self {
            Kind::Folder => "folders",
            Kind::File => "files",
        }
    }
    /// The output key naming the item: `folder-id` or `file-id`.
    fn id_key(self) -> &'static str {
        match self {
            Kind::Folder => "folder-id",
            Kind::File => "file-id",
        }
    }
    fn wire(self) -> &'static str {
        match self {
            Kind::Folder => "FOLDER",
            Kind::File => "FILE",
        }
    }
}

/// A validated command. Parsing is pure, so every refusal of bad input is tested
/// without a server and happens before any credential is read.
#[derive(Debug, PartialEq, Eq)]
enum Op<'a> {
    CreateFolder {
        parent_id: &'a str,
        name: &'a str,
        reuse: bool,
    },
    FindItem {
        folder_id: &'a str,
        name: &'a str,
        kind: Option<Kind>,
    },
    ListByPath {
        project_id: &'a str,
        path: &'a str,
    },
    Update {
        kind: Kind,
        id: &'a str,
        name: Option<&'a str>,
        parent_id: Option<&'a str>,
        if_match: Option<&'a str>,
    },
    Delete {
        kind: Kind,
        id: &'a str,
        if_match: Option<&'a str>,
    },
    CopyFile {
        file_id: &'a str,
        version_id: Option<&'a str>,
        parent_id: &'a str,
        merge_existing: bool,
    },
}

impl<'a> Op<'a> {
    fn parse(command: &str, args: &'a Value) -> Result<Self, AwareError> {
        Ok(match command {
            "create-folder" => Op::CreateFolder {
                parent_id: str_arg(args, "parent-id")?,
                name: item_name(args, "name")?.ok_or_else(|| missing("name"))?,
                reuse: match opt_str(args, "if-exists")? {
                    None | Some("reuse") => true,
                    Some("fail") => false,
                    Some(other) => {
                        return Err(invalid(format!(
                            "`if-exists` must be `reuse` or `fail`, got {other:?}"
                        )));
                    }
                },
            },
            "find-item" => Op::FindItem {
                folder_id: str_arg(args, "folder-id")?,
                name: str_arg(args, "name")?,
                kind: match opt_str(args, "type")? {
                    None => None,
                    Some(t) if t.eq_ignore_ascii_case("folder") => Some(Kind::Folder),
                    Some(t) if t.eq_ignore_ascii_case("file") => Some(Kind::File),
                    Some(other) => {
                        return Err(invalid(format!(
                            "`type` must be `FOLDER` or `FILE`, got {other:?}"
                        )));
                    }
                },
            },
            "list-folder-by-path" => Op::ListByPath {
                project_id: str_arg(args, "project-id")?,
                path: str_arg(args, "path")?,
            },
            "update-folder" | "update-file" => {
                let kind = if command == "update-folder" {
                    Kind::Folder
                } else {
                    Kind::File
                };
                let name = item_name(args, "name")?;
                let parent_id = opt_str(args, "parent-id")?;
                if name.is_none() && parent_id.is_none() {
                    return Err(invalid(format!(
                        "{command} needs `name` (rename), `parent-id` (move), or both"
                    )));
                }
                Op::Update {
                    kind,
                    id: str_arg(args, kind.id_key())?,
                    name,
                    parent_id,
                    if_match: opt_str(args, "if-match")?,
                }
            }
            "delete-folder" | "delete-file" => {
                let kind = if command == "delete-folder" {
                    Kind::Folder
                } else {
                    Kind::File
                };
                Op::Delete {
                    kind,
                    id: str_arg(args, kind.id_key())?,
                    if_match: opt_str(args, "if-match")?,
                }
            }
            "copy-file" => Op::CopyFile {
                file_id: str_arg(args, "file-id")?,
                version_id: opt_str(args, "version-id")?,
                parent_id: str_arg(args, "parent-id")?,
                merge_existing: match args.get("merge-existing") {
                    None | Some(Value::Null) => false,
                    Some(Value::Bool(b)) => *b,
                    Some(other) => {
                        return Err(invalid(format!(
                            "`merge-existing` must be a boolean, got {other}"
                        )));
                    }
                },
            },
            other => {
                return Err(AwareError::Validation(format!(
                    "trimble-connect: {other:?} is not a folder/file management command"
                )));
            }
        })
    }
}

/// The authenticated TC client for one command.
struct Tc {
    agent: ureq::Agent,
    token: String,
    base: String,
}

impl Tc {
    fn url(&self, rel: &str) -> String {
        format!("{}/{rel}", self.base)
    }

    fn get(&self, rel: &str, what: &str) -> Result<Value, AwareError> {
        get_json(&self.agent, &self.url(rel), Some(&self.token), what)
    }

    /// A GET whose 404 is an answer ("no such item"), not a failure.
    fn get_optional(&self, rel: &str, what: &str) -> Result<Option<Value>, AwareError> {
        let res = self
            .agent
            .get(&self.url(rel))
            .set("Authorization", &format!("Bearer {}", self.token))
            .call();
        match res {
            Err(ureq::Error::Status(404, _)) => Ok(None),
            other => json_body(other, what).map(Some),
        }
    }

    fn post(&self, rel: &str, body: &Value, what: &str) -> Result<Value, AwareError> {
        post_json(&self.agent, &self.url(rel), &self.token, body, what)
    }

    fn patch(
        &self,
        rel: &str,
        body: &Value,
        if_match: Option<&str>,
        what: &str,
    ) -> Result<Value, AwareError> {
        let mut req = self
            .agent
            .request("PATCH", &self.url(rel))
            .set("Authorization", &format!("Bearer {}", self.token))
            .set("Content-Type", "application/json");
        if let Some(v) = if_match {
            req = req.set("If-Match", v);
        }
        json_body(req.send_string(&body.to_string()), what)
    }

    fn delete(&self, rel: &str, if_match: Option<&str>, what: &str) -> Result<(), AwareError> {
        let mut req = self
            .agent
            .delete(&self.url(rel))
            .set("Authorization", &format!("Bearer {}", self.token));
        if let Some(v) = if_match {
            req = req.set("If-Match", v);
        }
        ok_response(req.call(), what).map(|_| ())
    }
}

// ── commands ─────────────────────────────────────────────────────────────────

/// `create-folder` — `POST folders {name, parentId}`. With `if-exists: reuse`
/// (the default) a folder of that name already under the parent is returned
/// instead (`created: false`), so a retried or re-run workflow converges on one
/// folder rather than failing or duplicating. `fail` refuses instead.
fn create_folder(tc: &Tc, parent_id: &str, name: &str, reuse: bool) -> Result<Value, AwareError> {
    if let Some(existing) = lookup(tc, parent_id, name, Some(Kind::Folder))? {
        if !reuse {
            return Err(AwareError::Validation(format!(
                "trimble-connect create-folder: a folder named {name:?} already exists \
                 under {parent_id} (id {}); pass `if-exists: reuse` to use it",
                existing.get("id").and_then(Value::as_str).unwrap_or("?")
            )));
        }
        return item_summary(&existing, Kind::Folder, "create-folder lookup")
            .map(|v| with(v, "created", json!(false)));
    }
    let created = tc.post(
        "folders",
        &json!({ "name": name, "parentId": parent_id }),
        "create folder",
    )?;
    item_summary(&created, Kind::Folder, "create folder").map(|v| with(v, "created", json!(true)))
}

/// `find-item` — `GET folders/{id}/item?name=…[&type=…]`. A miss is
/// `{found: false}`; a hit is the item's identifiers plus `found: true`.
fn find_item(
    tc: &Tc,
    folder_id: &str,
    name: &str,
    kind: Option<Kind>,
) -> Result<Value, AwareError> {
    match lookup(tc, folder_id, name, kind)? {
        None => Ok(json!({ "found": false })),
        Some(item) => {
            let kind = match item.get("type").and_then(Value::as_str) {
                Some("FILE") => Kind::File,
                Some("FOLDER") => Kind::Folder,
                other => {
                    return Err(AwareError::Network(format!(
                        "find item: TC returned an item of unknown type {other:?}"
                    )));
                }
            };
            let mut out = item_summary(&item, kind, "find item")?;
            if let Value::Object(map) = &mut out {
                // One shape for both kinds: `id` plus the kind, alongside the
                // kind-specific key, so a consumer need not branch to read it.
                map.insert("found".into(), json!(true));
                map.insert("id".into(), map[kind.id_key()].clone());
                map.insert("type".into(), json!(kind.wire()));
                if let Some(size) = item.get("size") {
                    map.insert("size".into(), size.clone());
                }
            }
            Ok(out)
        }
    }
}

/// `GET folders/{parent}/item?name=…&type=…` — `None` on 404.
fn lookup(
    tc: &Tc,
    parent_id: &str,
    name: &str,
    kind: Option<Kind>,
) -> Result<Option<Value>, AwareError> {
    let mut rel = format!(
        "folders/{}/item?name={}",
        percent_encode_path(parent_id),
        percent_encode_path(name)
    );
    if let Some(k) = kind {
        rel.push_str(&format!("&type={}", k.wire()));
    }
    let found = tc.get_optional(&rel, "item lookup")?;
    // TC filters by `type` itself; checking again costs nothing and keeps a
    // file named like the folder from being "reused" as one if it ever doesn't.
    Ok(found.filter(|item| {
        kind.is_none_or(|k| item.get("type").and_then(Value::as_str) == Some(k.wire()))
    }))
}

/// `list-folder-by-path` — `GET folders/by_path?path=…&projectId=…`.
fn list_folder_by_path(tc: &Tc, project_id: &str, path: &str) -> Result<Value, AwareError> {
    let items = tc.get(
        &format!(
            "folders/by_path?path={}&projectId={}",
            percent_encode_path(path),
            percent_encode_path(project_id)
        ),
        "list folder by path",
    )?;
    if !items.is_array() {
        return Err(AwareError::Network(
            "list folder by path: TC did not return a list of items".into(),
        ));
    }
    Ok(json!({ "items": items }))
}

/// `update-folder` / `update-file` — `PATCH {folders|files}/{id}`. TC applies
/// only the move when a body carries both `parentId` and `name` ("move will take
/// precedence before rename"), so asking for both sends the move, then the
/// rename. `if-match` guards the first request: TC answers 412 when it is not
/// the item's latest version id.
fn update(
    tc: &Tc,
    kind: Kind,
    id: &str,
    name: Option<&str>,
    parent_id: Option<&str>,
    if_match: Option<&str>,
) -> Result<Value, AwareError> {
    let rel = format!("{}/{}", kind.collection(), percent_encode_path(id));
    let what = format!("update {}", kind.collection().trim_end_matches('s'));
    let mut guard = if_match;
    let mut last = None;
    if let Some(parent) = parent_id {
        last = Some(tc.patch(&rel, &json!({ "parentId": parent }), guard, &what)?);
        guard = None;
    }
    if let Some(name) = name {
        last = Some(tc.patch(&rel, &json!({ "name": name }), guard, &what)?);
    }
    let last = last.ok_or_else(|| missing("name` or `parent-id"))?;
    item_summary(&last, kind, &what)
}

/// `delete-folder` / `delete-file` — `DELETE {folders|files}/{id}` (204). TC's
/// delete is addressed by the item id (not its version id); `if-match` makes it
/// conditional on the latest version.
fn delete(tc: &Tc, kind: Kind, id: &str, if_match: Option<&str>) -> Result<Value, AwareError> {
    let what = format!("delete {}", kind.collection().trim_end_matches('s'));
    tc.delete(
        &format!("{}/{}", kind.collection(), percent_encode_path(id)),
        if_match,
        &what,
    )?;
    Ok(json!({ kind.id_key(): id, "deleted": true }))
}

/// `copy-file` — `POST files {parentId, parentType, fromFileVersionId,
/// mergeExisting}`. TC copies a *version*; with no `version-id` the file's
/// latest is resolved first.
fn copy_file(
    tc: &Tc,
    file_id: &str,
    version_id: Option<&str>,
    parent_id: &str,
    merge_existing: bool,
) -> Result<Value, AwareError> {
    let version = match version_id {
        Some(v) => v.to_string(),
        None => {
            let meta = tc.get(
                &format!("files/{}", percent_encode_path(file_id)),
                "copy file: source metadata",
            )?;
            meta.get("versionId")
                .and_then(Value::as_str)
                .filter(|s| !s.is_empty())
                .ok_or_else(|| {
                    AwareError::Network("copy file: source metadata has no versionId".into())
                })?
                .to_string()
        }
    };
    let copy = tc.post(
        "files",
        &json!({
            "parentId": parent_id,
            "parentType": "FOLDER",
            "fromFileVersionId": version,
            "mergeExisting": merge_existing,
        }),
        "copy file",
    )?;
    item_summary(&copy, Kind::File, "copy file")
}

// ── helpers ──────────────────────────────────────────────────────────────────

/// The stable result shape for one item: its id, version id, name, parent and
/// project. `id` and `versionId` are required — they are what a downstream node
/// persists or passes on, so a 2xx without them is a failure, not a blank success.
fn item_summary(item: &Value, kind: Kind, what: &str) -> Result<Value, AwareError> {
    let required = |key: &str| {
        item.get(key)
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty())
            .map(|s| json!(s))
            .ok_or_else(|| AwareError::Network(format!("{what}: TC response has no {key}")))
    };
    let optional = |key: &str| item.get(key).cloned().unwrap_or(Value::Null);
    let mut out = Map::new();
    out.insert(kind.id_key().into(), required("id")?);
    out.insert("version-id".into(), required("versionId")?);
    out.insert("name".into(), optional("name"));
    out.insert("parent-id".into(), optional("parentId"));
    out.insert("project-id".into(), optional("projectId"));
    Ok(Value::Object(out))
}

fn with(mut v: Value, key: &str, val: Value) -> Value {
    if let Value::Object(map) = &mut v {
        map.insert(key.into(), val);
    }
    v
}

/// An optional string input. Absent, `null` and `""` (what `{{ }}` renders an
/// unset input to) all mean "not given"; any other non-string is refused.
fn opt_str<'a>(args: &'a Value, key: &str) -> Result<Option<&'a str>, AwareError> {
    match args.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) if s.is_empty() => Ok(None),
        Some(Value::String(s)) => Ok(Some(s)),
        Some(other) => Err(invalid(format!("`{key}` must be a string, got {other}"))),
    }
}

/// An optional item name: [`opt_str`], plus TC's rule that a name is one path
/// segment — a `/` would name a nested path TC does not create, and a
/// whitespace-only name is not a name.
fn item_name<'a>(args: &'a Value, key: &str) -> Result<Option<&'a str>, AwareError> {
    let Some(name) = opt_str(args, key)? else {
        return Ok(None);
    };
    if name.trim().is_empty() {
        return Err(invalid(format!("`{key}` is blank")));
    }
    if name.contains(['/', '\\']) {
        return Err(invalid(format!(
            "`{key}` {name:?} contains a path separator; create one folder per segment"
        )));
    }
    Ok(Some(name))
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
    use crate::runtime::trimble_files::test_support::*;

    const FOLDER: &str = r#"{"id":"F1","versionId":"FV1","name":"2026-10-10","type":"FOLDER","parentId":"ROOT","projectId":"P1"}"#;

    async fn run(base: &str, command: &str, args: Value) -> Result<Value, AwareError> {
        let agents = mock_agents(base);
        invoke(agents.path().join("agents"), command, args).await
    }

    fn line(req: &str) -> &str {
        req.lines().next().unwrap_or_default()
    }

    /// The JSON body of a captured request (everything after the blank line).
    fn body_of(req: &str) -> Value {
        let body = req.split("\r\n\r\n").nth(1).unwrap_or_default();
        serde_json::from_str(body).unwrap_or_else(|e| panic!("body not JSON ({e}): {req}"))
    }

    #[test]
    fn handles_exactly_the_management_commands() {
        for c in COMMANDS {
            assert!(handles(c), "{c}");
        }
        // upload/download stay with trimble_files; the declarative reads stay REST.
        for c in [
            "upload",
            "download",
            "list-projects",
            "list-folders",
            "get-folder",
        ] {
            assert!(!handles(c), "{c}");
        }
    }

    /// Which commands of a trimble-connect manifest would reach the generic REST
    /// renderer with no `method:` to render — i.e. fail at run with "not an HTTP
    /// method" — and which handled commands the manifest does not declare.
    fn manifest_handler_drift(manifest: &crate::manifest::Agent) -> (Vec<String>, Vec<String>) {
        let unrouted = manifest
            .commands
            .iter()
            .filter(|(name, cmd)| {
                cmd.method.is_none()
                    && !handles(name)
                    && !matches!(name.as_str(), "upload" | "download")
            })
            .map(|(name, _)| name.clone())
            .collect();
        let undeclared = COMMANDS
            .iter()
            .filter(|c| {
                manifest
                    .commands
                    .get(**c)
                    .is_none_or(|cmd| cmd.method.is_some())
            })
            .map(|c| c.to_string())
            .collect();
        (unrouted, undeclared)
    }

    #[test]
    fn the_shipped_manifest_and_this_handler_agree_on_every_command() {
        // A method-less command with no handler fails only at run; a handled
        // command that grew a `method:` would never reach this module (the
        // invoker checks the handler first, but the manifest would then lie).
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("20-agents/aeco/construction/trimble-connect/manifest.yaml");
        let text = std::fs::read_to_string(&path).unwrap();
        let load = || -> crate::manifest::Agent { serde_yaml::from_str(&text).unwrap() };
        let manifest = load();
        assert_eq!(
            manifest_handler_drift(&manifest),
            (vec![], vec![]),
            "manifest ↔ trimble_ops drift"
        );

        // Negative controls: the guard must reject each drift it exists to catch.
        let mut unrouted = load();
        // Rename a handled command: same method-less body, but no handler.
        let stray = unrouted.commands.remove("create-folder").unwrap();
        unrouted.commands.insert("rename-everything".into(), stray);
        assert_eq!(
            manifest_handler_drift(&unrouted).0,
            vec!["rename-everything".to_string()]
        );
        let mut undeclared = load();
        undeclared.commands.remove("delete-file");
        assert_eq!(
            manifest_handler_drift(&undeclared).1,
            vec!["delete-file".to_string()]
        );
        let mut given_a_method = load();
        given_a_method.commands.get_mut("copy-file").unwrap().method = Some("POST".into());
        assert_eq!(
            manifest_handler_drift(&given_a_method).1,
            vec!["copy-file".to_string()]
        );
    }

    #[test]
    fn inputs_are_validated_before_any_request() {
        let cases: &[(&str, Value, &str)] = &[
            ("create-folder", json!({"name": "a"}), "parent-id"),
            (
                "create-folder",
                json!({"parent-id": "p"}),
                "missing required input `name`",
            ),
            (
                "create-folder",
                json!({"parent-id": "p", "name": "  "}),
                "blank",
            ),
            (
                "create-folder",
                json!({"parent-id": "p", "name": "a/b"}),
                "path separator",
            ),
            (
                "create-folder",
                json!({"parent-id": "p", "name": "a", "if-exists": "replace"}),
                "`if-exists` must be",
            ),
            (
                "find-item",
                json!({"folder-id": "f", "name": "a", "type": "LINK"}),
                "`type` must be",
            ),
            ("list-folder-by-path", json!({"path": "a"}), "project-id"),
            ("update-folder", json!({"folder-id": "f"}), "needs `name`"),
            ("update-file", json!({"name": "x.ifc"}), "file-id"),
            (
                "update-file",
                json!({"file-id": "f", "name": 3}),
                "`name` must be a string",
            ),
            ("delete-folder", json!({"folder-id": ""}), "folder-id"),
            ("delete-file", json!({}), "file-id"),
            (
                "copy-file",
                json!({"file-id": "f", "parent-id": "p", "merge-existing": "yes"}),
                "`merge-existing` must be a boolean",
            ),
            ("copy-file", json!({"file-id": "f"}), "parent-id"),
        ];
        for (cmd, args, want) in cases {
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
    fn blank_optionals_mean_not_given() {
        // `{{ inputs.x }}` over an unset input renders "" — that must not become
        // `If-Match: ""` (a version named nothing) or a rename to "".
        let args = json!({"folder-id": "f", "name": "n", "parent-id": "", "if-match": null});
        let op = Op::parse("update-folder", &args).unwrap();
        assert_eq!(
            op,
            Op::Update {
                kind: Kind::Folder,
                id: "f",
                name: Some("n"),
                parent_id: None,
                if_match: None
            }
        );
    }

    #[tokio::test]
    async fn bad_input_is_refused_before_the_credential_is_needed() {
        // No credential stored at all: a validation refusal must still be the
        // error, proving parse runs before auth (no token refresh is spent on a
        // node that could never have run).
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
            agents.path().join("agents"),
            "create-folder",
            json!({"name": "a"}),
        )
        .await
        .unwrap_err();
        assert!(err.to_string().contains("parent-id"), "{err}");
    }

    #[tokio::test]
    async fn create_folder_posts_name_and_parent_when_none_exists() {
        let (base, rx) = mock_routed(3, |_b| {
            vec![
                (
                    "GET /folders/ROOT/item",
                    404,
                    r#"{"errorcode":"NOT_FOUND"}"#.to_string(),
                ),
                ("POST /folders ", 201, FOLDER.to_string()),
            ]
        });
        let out = run(
            &base,
            "create-folder",
            json!({"parent-id": "ROOT", "name": "2026-10-10"}),
        )
        .await
        .unwrap();
        assert_eq!(
            out,
            json!({"folder-id":"F1","version-id":"FV1","name":"2026-10-10","parent-id":"ROOT","project-id":"P1","created":true})
        );
        let lookup = next_request(&rx);
        assert!(
            line(&lookup).contains("/folders/ROOT/item?name=2026-10-10&type=FOLDER"),
            "{lookup}"
        );
        assert!(lookup.contains("Bearer TESTTOKEN"), "{lookup}");
        let post = next_request(&rx);
        assert!(line(&post).starts_with("POST /folders "), "{post}");
        assert!(post.contains("Bearer TESTTOKEN"), "{post}");
        assert_eq!(
            body_of(&post),
            json!({"name":"2026-10-10","parentId":"ROOT"})
        );
    }

    #[tokio::test]
    async fn create_folder_reuses_an_existing_folder_by_default_and_posts_nothing() {
        let (base, rx) = mock_routed(3, |_b| {
            vec![("GET /folders/ROOT/item", 200, FOLDER.to_string())]
        });
        let out = run(
            &base,
            "create-folder",
            json!({"parent-id": "ROOT", "name": "2026-10-10"}),
        )
        .await
        .unwrap();
        assert_eq!(out["folder-id"], "F1");
        assert_eq!(out["created"], json!(false));
        let seen = drain(&rx);
        assert_eq!(seen.len(), 1, "lookup only: {seen:#?}");
    }

    #[tokio::test]
    async fn create_folder_with_if_exists_fail_refuses_an_existing_name() {
        let (base, rx) = mock_routed(3, |_b| {
            vec![("GET /folders/ROOT/item", 200, FOLDER.to_string())]
        });
        let err = run(
            &base,
            "create-folder",
            json!({"parent-id": "ROOT", "name": "2026-10-10", "if-exists": "fail"}),
        )
        .await
        .unwrap_err();
        assert!(err.to_string().contains("already exists"), "{err}");
        assert_eq!(drain(&rx).len(), 1, "no POST after the refusal");
    }

    #[tokio::test]
    async fn create_folder_does_not_reuse_a_file_of_the_same_name() {
        // Defence in depth over TC's own `type=FOLDER` filter.
        let (base, _rx) = mock_routed(3, |_b| {
            vec![
                (
                    "GET /folders/ROOT/item",
                    200,
                    r#"{"id":"X","versionId":"XV","type":"FILE","name":"n"}"#.to_string(),
                ),
                ("POST /folders ", 201, FOLDER.to_string()),
            ]
        });
        let out = run(
            &base,
            "create-folder",
            json!({"parent-id": "ROOT", "name": "n"}),
        )
        .await
        .unwrap();
        assert_eq!(out["folder-id"], "F1");
        assert_eq!(out["created"], json!(true));
    }

    #[tokio::test]
    async fn create_folder_surfaces_tc_refusals_and_incomplete_responses() {
        // A lookup failing for any reason but 404 is not "absent".
        let (base, rx) = mock_routed(3, |_b| {
            vec![(
                "GET /folders/ROOT/item",
                403,
                r#"{"message":"no access"}"#.to_string(),
            )]
        });
        let err = run(
            &base,
            "create-folder",
            json!({"parent-id": "ROOT", "name": "n"}),
        )
        .await
        .unwrap_err();
        assert!(
            err.to_string().contains("403") && err.to_string().contains("no access"),
            "{err}"
        );
        assert_eq!(drain(&rx).len(), 1, "no POST after a failed lookup");

        let (base, _rx) = mock_routed(3, |_b| {
            vec![
                ("GET /folders/ROOT/item", 404, "{}".to_string()),
                (
                    "POST /folders ",
                    400,
                    r#"{"message":"invalid name"}"#.to_string(),
                ),
            ]
        });
        let err = run(
            &base,
            "create-folder",
            json!({"parent-id": "ROOT", "name": "n"}),
        )
        .await
        .unwrap_err();
        let text = err.to_string();
        assert!(
            text.contains("create folder") && text.contains("invalid name"),
            "{text}"
        );

        let (base, _rx) = mock_routed(3, |_b| {
            vec![
                ("GET /folders/ROOT/item", 404, "{}".to_string()),
                ("POST /folders ", 201, r#"{"name":"n"}"#.to_string()),
            ]
        });
        let err = run(
            &base,
            "create-folder",
            json!({"parent-id": "ROOT", "name": "n"}),
        )
        .await
        .unwrap_err();
        assert!(err.to_string().contains("no id"), "{err}");
    }

    /// The output keys the shipped manifest declares for `command`.
    fn declared_output_keys(command: &str) -> std::collections::BTreeSet<String> {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("20-agents/aeco/construction/trimble-connect/manifest.yaml");
        let doc: serde_yaml::Value =
            serde_yaml::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        doc["commands"][command]["outputs"]["schema"]
            .as_mapping()
            .unwrap_or_else(|| panic!("{command}: no outputs.schema"))
            .keys()
            .filter_map(|k| k.as_str().map(str::to_string))
            .collect()
    }

    /// Every key in `out` is one the manifest declares for `command` — the
    /// lockfile compiler checks `{{ node.key }}` references and `--simulate`
    /// builds outputs from that schema, so an undeclared key is unusable.
    fn assert_declared(command: &str, out: &Value) {
        let declared = declared_output_keys(command);
        let undeclared: Vec<&String> = out
            .as_object()
            .unwrap()
            .keys()
            .filter(|k| !declared.contains(*k))
            .collect();
        assert!(
            undeclared.is_empty(),
            "{command} emits keys its manifest schema does not declare: {undeclared:?}"
        );
    }

    #[tokio::test]
    async fn every_key_a_handler_emits_is_declared_in_the_manifest() {
        // find-item, once per kind: each hit carries its kind-specific id key.
        for (ty, body) in [
            (
                "FILE",
                r#"{"id":"D","versionId":"DV","type":"FILE","name":"a","size":3}"#,
            ),
            ("FOLDER", FOLDER),
        ] {
            let owned = body.to_string();
            let (base, _rx) = mock_routed(2, move |_b| vec![("GET /folders/F/item", 200, owned)]);
            let out = run(&base, "find-item", json!({"folder-id": "F", "name": "a"}))
                .await
                .unwrap();
            assert_eq!(out["type"], ty);
            assert_declared("find-item", &out);
        }
        let (base, _rx) = mock_routed(2, |_b| {
            vec![("GET /folders/ROOT/item", 200, FOLDER.to_string())]
        });
        let out = run(
            &base,
            "create-folder",
            json!({"parent-id": "ROOT", "name": "n"}),
        )
        .await
        .unwrap();
        assert_declared("create-folder", &out);
        for (cmd, route, args) in [
            (
                "update-folder",
                "PATCH /folders/F1",
                json!({"folder-id": "F1", "name": "x"}),
            ),
            (
                "update-file",
                "PATCH /files/F1",
                json!({"file-id": "F1", "name": "x"}),
            ),
        ] {
            let (base, _rx) = mock_routed(2, move |_b| vec![(route, 200, FOLDER.to_string())]);
            assert_declared(cmd, &run(&base, cmd, args).await.unwrap());
        }
        for (cmd, route, args) in [
            (
                "delete-folder",
                "DELETE /folders/F1",
                json!({"folder-id": "F1"}),
            ),
            ("delete-file", "DELETE /files/F1", json!({"file-id": "F1"})),
        ] {
            let (base, _rx) = mock_routed(2, move |_b| vec![(route, 204, String::new())]);
            assert_declared(cmd, &run(&base, cmd, args).await.unwrap());
        }
        let (base, _rx) = mock_routed(2, |_b| vec![("POST /files ", 201, FOLDER.to_string())]);
        let out = run(
            &base,
            "copy-file",
            json!({"file-id": "S", "version-id": "SV", "parent-id": "T"}),
        )
        .await
        .unwrap();
        assert_declared("copy-file", &out);
        let (base, _rx) = mock_routed(2, |_b| {
            vec![("GET /folders/by_path", 200, "[]".to_string())]
        });
        let out = run(
            &base,
            "list-folder-by-path",
            json!({"project-id": "P", "path": "R"}),
        )
        .await
        .unwrap();
        assert_declared("list-folder-by-path", &out);
    }

    #[tokio::test]
    async fn find_item_reports_a_miss_as_data_and_a_hit_with_its_kind() {
        let (base, _rx) = mock_routed(2, |_b| vec![("GET /folders/F/item", 404, "{}".to_string())]);
        let out = run(
            &base,
            "find-item",
            json!({"folder-id": "F", "name": "x.ifc"}),
        )
        .await
        .unwrap();
        assert_eq!(out, json!({"found": false}));

        let (base, rx) = mock_routed(2, |_b| {
            vec![(
                "GET /folders/F/item",
                200,
                r#"{"id":"D","versionId":"DV","type":"FILE","name":"x y.ifc","parentId":"F","projectId":"P","size":12}"#
                    .to_string(),
            )]
        });
        let out = run(
            &base,
            "find-item",
            json!({"folder-id": "F", "name": "x y.ifc", "type": "file"}),
        )
        .await
        .unwrap();
        assert_eq!(out["found"], json!(true));
        assert_eq!(out["id"], "D");
        assert_eq!(out["file-id"], "D");
        assert_eq!(out["type"], "FILE");
        assert_eq!(out["size"], json!(12));
        let req = next_request(&rx);
        assert!(line(&req).contains("name=x%20y.ifc&type=FILE"), "{req}");
    }

    #[tokio::test]
    async fn list_folder_by_path_encodes_both_query_values() {
        let (base, rx) = mock_routed(2, |_b| {
            vec![("GET /folders/by_path", 200, r#"[{"id":"a"}]"#.to_string())]
        });
        let out = run(
            &base,
            "list-folder-by-path",
            json!({"project-id": "P 1", "path": "Root/Drawings/MRN"}),
        )
        .await
        .unwrap();
        assert_eq!(out, json!({"items": [{"id": "a"}]}));
        let req = next_request(&rx);
        assert!(
            line(&req).contains("path=Root%2FDrawings%2FMRN&projectId=P%201"),
            "{req}"
        );

        let (base, _rx) = mock_routed(2, |_b| {
            vec![("GET /folders/by_path", 200, r#"{"id":"a"}"#.to_string())]
        });
        let err = run(
            &base,
            "list-folder-by-path",
            json!({"project-id": "P", "path": "R"}),
        )
        .await
        .unwrap_err();
        assert!(err.to_string().contains("did not return a list"), "{err}");
    }

    #[tokio::test]
    async fn update_with_move_and_rename_sends_the_move_then_the_rename() {
        let (base, rx) = mock_routed(3, |_b| {
            vec![(
                "PATCH /folders/F1",
                200,
                r#"{"id":"F1","versionId":"FV9","name":"new","parentId":"P2","projectId":"P"}"#
                    .to_string(),
            )]
        });
        let out = run(
            &base,
            "update-folder",
            json!({"folder-id": "F1", "name": "new", "parent-id": "P2", "if-match": "FV1"}),
        )
        .await
        .unwrap();
        assert_eq!(out["version-id"], "FV9");
        assert_eq!(out["name"], "new");
        let first = next_request(&rx);
        assert_eq!(body_of(&first), json!({"parentId": "P2"}));
        assert!(
            first.contains("If-Match: FV1"),
            "guard on the first request: {first}"
        );
        let second = next_request(&rx);
        assert_eq!(body_of(&second), json!({"name": "new"}));
        assert!(
            !second.contains("If-Match"),
            "the move changed the version: {second}"
        );
        assert!(second.contains("Bearer TESTTOKEN"), "{second}");
    }

    #[tokio::test]
    async fn update_file_rename_only_is_one_patch_and_412_is_reported() {
        let (base, rx) = mock_routed(2, |_b| {
            vec![(
                "PATCH /files/D1",
                412,
                r#"{"errorcode":"INVALID_HEADER","message":"If-Match header value is not latest versionId."}"#
                    .to_string(),
            )]
        });
        let err = run(
            &base,
            "update-file",
            json!({"file-id": "D1", "name": "b.ifc", "if-match": "old"}),
        )
        .await
        .unwrap_err();
        let text = err.to_string();
        assert!(
            text.contains("update file") && text.contains("412"),
            "{text}"
        );
        let seen = drain(&rx);
        assert_eq!(seen.len(), 1, "{seen:#?}");
        assert_eq!(body_of(&seen[0]), json!({"name": "b.ifc"}));
    }

    #[tokio::test]
    async fn delete_addresses_the_item_id_and_carries_an_optional_guard() {
        let (base, rx) = mock_routed(2, |_b| vec![("DELETE /folders/F1 ", 204, String::new())]);
        let out = run(&base, "delete-folder", json!({"folder-id": "F1"}))
            .await
            .unwrap();
        assert_eq!(out, json!({"folder-id": "F1", "deleted": true}));
        let req = next_request(&rx);
        assert!(
            req.contains("Bearer TESTTOKEN") && !req.contains("If-Match"),
            "{req}"
        );

        let (base, rx) = mock_routed(2, |_b| vec![("DELETE /files/D1 ", 204, String::new())]);
        let out = run(
            &base,
            "delete-file",
            json!({"file-id": "D1", "if-match": "DV1"}),
        )
        .await
        .unwrap();
        assert_eq!(out, json!({"file-id": "D1", "deleted": true}));
        assert!(next_request(&rx).contains("If-Match: DV1"));

        let (base, _rx) = mock_routed(2, |_b| {
            vec![(
                "DELETE /files/D1 ",
                404,
                r#"{"message":"gone"}"#.to_string(),
            )]
        });
        let err = run(&base, "delete-file", json!({"file-id": "D1"}))
            .await
            .unwrap_err();
        assert!(
            err.to_string().contains("delete file") && err.to_string().contains("404"),
            "{err}"
        );
    }

    #[tokio::test]
    async fn copy_file_resolves_the_latest_version_when_none_is_given() {
        let copied =
            r#"{"id":"C1","versionId":"CV1","name":"a.ifc","parentId":"T","projectId":"P"}"#;
        let (base, rx) = mock_routed(3, move |_b| {
            vec![
                (
                    "GET /files/S1",
                    200,
                    r#"{"id":"S1","versionId":"SV7"}"#.to_string(),
                ),
                ("POST /files ", 201, copied.to_string()),
            ]
        });
        let out = run(
            &base,
            "copy-file",
            json!({"file-id": "S1", "parent-id": "T"}),
        )
        .await
        .unwrap();
        assert_eq!(out["file-id"], "C1");
        next_request(&rx);
        let post = next_request(&rx);
        assert_eq!(
            body_of(&post),
            json!({"parentId":"T","parentType":"FOLDER","fromFileVersionId":"SV7","mergeExisting":false})
        );

        // A pinned version skips the metadata read.
        let (base, rx) = mock_routed(3, move |_b| vec![("POST /files ", 201, copied.to_string())]);
        run(
            &base,
            "copy-file",
            json!({"file-id": "S1", "version-id": "SV1", "parent-id": "T", "merge-existing": true}),
        )
        .await
        .unwrap();
        let seen = drain(&rx);
        assert_eq!(seen.len(), 1, "{seen:#?}");
        assert_eq!(body_of(&seen[0])["fromFileVersionId"], "SV1");
        assert_eq!(body_of(&seen[0])["mergeExisting"], json!(true));
    }
}
