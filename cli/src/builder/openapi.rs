//! OpenAPI 3.x → AWARE agent.
//!
//! Walks `paths.<path>.<method>` operations and emits one command per operation.
//! Accepts JSON or YAML; HTTP(S) URL or local file path.

use std::collections::BTreeMap;
use std::io::Read;

use serde_json::Value;

use crate::builder::{AuthBlock, GeneratedAgent, GeneratedCommand, Provenance, RestBlock, now_iso};
use crate::error::AwareError;

pub fn build_from_url_or_path(
    input: &str,
    agent_id: Option<&str>,
) -> Result<GeneratedAgent, AwareError> {
    let body = fetch_spec(input)?;

    // Try JSON first, fall back to YAML
    let spec: Value = serde_json::from_str(&body)
        .or_else(|_| serde_yaml::from_str::<Value>(&body))
        .map_err(|e| AwareError::Validation(format!("spec not JSON or YAML: {e}")))?;

    let info = &spec["info"];
    let title = info["title"]
        .as_str()
        .unwrap_or("openapi-agent")
        .to_string();
    let version = info["version"].as_str().unwrap_or("0.1.0").to_string();
    let description = info["description"].as_str().unwrap_or(&title).to_string();
    let license = info["license"]["name"]
        .as_str()
        .unwrap_or("UNKNOWN")
        .to_string();

    let id = agent_id.map(String::from).unwrap_or_else(|| kebab(&title));

    // Declarative auth + whether the API requires it by default (root-level
    // `security`). Public operations (empty effective security) are flagged
    // `no-auth` so the transport skips auth for them — only meaningful when the
    // agent actually has an auth block.
    let auth = build_auth(&spec, &id);
    let agent_has_auth = auth.is_some();
    // The API is secured by default only when root `security` is present,
    // non-empty, and offers NO anonymous alternative (an empty `{}` requirement
    // means auth is optional → effectively public).
    let root_secured = spec
        .get("security")
        .and_then(|v| v.as_array())
        .is_some_and(|reqs| !reqs.is_empty() && !reqs.iter().any(is_anonymous_alt));

    let mut commands = BTreeMap::new();
    if let Some(paths) = spec["paths"].as_object() {
        for (path, methods) in paths {
            if let Some(methods_obj) = methods.as_object() {
                // Path-item-level `parameters` are shared by every operation on
                // this path (OpenAPI); merge them into each command's inputs.
                let path_params = methods_obj.get("parameters");
                for (method, op) in methods_obj {
                    if !["get", "post", "put", "delete", "patch"].contains(&method.as_str()) {
                        continue;
                    }
                    let raw_op_id =
                        op["operationId"]
                            .as_str()
                            .map(String::from)
                            .unwrap_or_else(|| {
                                let cleaned_path =
                                    path.replace('/', "-").trim_matches('-').to_string();
                                format!("{method}-{cleaned_path}")
                            });
                    let summary = op["summary"]
                        .as_str()
                        .or_else(|| op["description"].as_str())
                        .unwrap_or("OpenAPI operation")
                        .to_string();
                    // An operation- or path-level `servers` overrides the root
                    // base for this command: emit an absolute command path so the
                    // runtime targets the override host, not the agent base.
                    let op_path = match first_server_url(op.get("servers"))
                        .or_else(|| first_server_url(methods_obj.get("servers")))
                        .and_then(|s| resolve_server_url(s, input))
                    {
                        Some(base) => {
                            format!(
                                "{}/{}",
                                base.trim_end_matches('/'),
                                path.trim_start_matches('/')
                            )
                        }
                        None => path.clone(),
                    };
                    commands.insert(
                        kebab(&raw_op_id),
                        GeneratedCommand {
                            lifecycle: "single".into(),
                            description: format!(
                                "{} {path} \u{2014} {summary}",
                                method.to_uppercase()
                            ),
                            inputs_yaml: build_inputs_yaml(&spec, op, path_params),
                            outputs_yaml: build_outputs_yaml(&spec, op),
                            method: Some(method.to_uppercase()),
                            path: Some(op_path),
                            // Mutating HTTP methods are write-mode so the safety
                            // contract applies (operationIds like `add-pet` don't
                            // match the name-suffix write convention). #106.
                            mode: matches!(method.as_str(), "post" | "put" | "patch" | "delete")
                                .then(|| "write".to_string()),
                            // A public operation (empty effective security) opts
                            // out of the agent's auth — only when auth exists.
                            no_auth: agent_has_auth && op_effective_public(op, root_secured),
                        },
                    );
                }
            }
        }
    }

    let provenance = Provenance {
        generated_by: "aware-agent-builder".into(),
        generator_version: env!("CARGO_PKG_VERSION").into(),
        source: serde_json::json!({ "type": "openapi", "input": input, "license": license }),
        generated_at: now_iso(),
    };

    // An OpenAPI agent is an HTTP API → `rest` transport (with the spec's base
    // URL), plus declarative `auth` derived from `securitySchemes` (#106).
    let rest = Some(RestBlock {
        base: server_base(&spec, input),
        auth,
    });

    Ok(GeneratedAgent {
        id,
        version: "0.1.0".into(),
        sdk_target: Some(version),
        description,
        commands,
        skills: Vec::new(),
        provenance,
        stateful: false,
        license,
        rest,
    })
}

/// The absolute REST base URL operation commands resolve against. Prefers the
/// first `servers[].url` when absolute; if it's relative or absent, derives the
/// origin from the spec's input URL (so a spec fetched from `https://api.x/...`
/// still yields a usable base). `None` only when neither is available (e.g. a
/// local-file spec with no absolute server).
fn server_base(spec: &Value, input: &str) -> Option<String> {
    match first_server_url(spec.get("servers")) {
        Some(declared) => resolve_server_url(declared, input),
        None => input_origin(input),
    }
}

/// The first `servers[].url` string from a `servers` array, if any.
fn first_server_url(servers: Option<&Value>) -> Option<&str> {
    servers?.as_array()?.first()?.get("url")?.as_str()
}

/// Resolve a (possibly relative) `servers[].url` to an absolute base. An
/// absolute URL is used verbatim; a relative one resolves against the spec
/// document's URL per RFC 3986 (leading-slash → origin-relative; otherwise
/// relative to the document's directory). `None` when neither is possible.
fn resolve_server_url(declared: &str, input: &str) -> Option<String> {
    if declared.starts_with("http://") || declared.starts_with("https://") {
        return Some(declared.to_string());
    }
    let origin = input_origin(input)?;
    if let Some(abs) = declared.strip_prefix('/') {
        Some(format!("{}/{}", origin.trim_end_matches('/'), abs))
    } else {
        let path = &input[origin.len()..];
        let dir = match path.rfind('/') {
            Some(i) => &path[..=i],
            None => "/",
        };
        Some(format!("{origin}{dir}{declared}"))
    }
}

/// The scheme + authority of an http(s) URL (`https://host:port`), or `None`
/// when `input` isn't an http(s) URL (e.g. a local file path).
fn input_origin(input: &str) -> Option<String> {
    let scheme_len = ["https://", "http://"]
        .iter()
        .find_map(|p| input.starts_with(p).then_some(p.len()))?;
    let authority_len = input[scheme_len..]
        .find('/')
        .unwrap_or(input.len() - scheme_len);
    Some(input[..scheme_len + authority_len].to_string())
}

/// An anonymous security alternative — an empty `{}` requirement object, which
/// OpenAPI treats as "no auth required" within a `security` list of alternatives.
fn is_anonymous_alt(req: &Value) -> bool {
    req.as_object().is_some_and(|o| o.is_empty())
}

/// Whether an operation can be called anonymously (a public endpoint). An
/// operation's own `security` overrides the root default: an empty array, or one
/// that lists an empty `{}` alternative, is public; otherwise it's secured. With
/// no operation `security`, the root default (`root_secured`) applies.
fn op_effective_public(op: &Value, root_secured: bool) -> bool {
    match op.get("security").and_then(|v| v.as_array()) {
        Some(reqs) => reqs.is_empty() || reqs.iter().any(is_anonymous_alt),
        None => !root_secured,
    }
}

/// Append every scheme name referenced by a `security` requirement array to
/// `out`. Each requirement is an object whose keys are scheme names.
fn push_security_refs(security: Option<&Value>, out: &mut Vec<String>) {
    if let Some(reqs) = security.and_then(|v| v.as_array()) {
        for r in reqs {
            if let Some(obj) = r.as_object() {
                out.extend(obj.keys().cloned());
            }
        }
    }
}

/// Derive a declarative `auth` block from the spec's `components.security
/// Schemes`. Picks the first scheme referenced by a `security` requirement —
/// root-level or any operation's — and maps apiKey / http-bearer / oauth2 to the
/// AWARE auth model. The credential handle is the agent id (one secret per
/// built agent). Returns `None` when no scheme is required (unauthenticated) or
/// the referenced scheme is unmodeled (e.g. http-basic).
///
/// AWARE auth is **agent-level**: one credential applies to every command. A
/// spec that mixes secured and public operations therefore yields a single
/// agent-wide auth block, so the credential is also sent to its public
/// endpoints (which ignore it). Per-operation auth would need a substrate-level
/// per-command auth model — out of scope here.
fn build_auth(spec: &Value, agent_id: &str) -> Option<AuthBlock> {
    let schemes = spec.pointer("/components/securitySchemes")?.as_object()?;
    if schemes.is_empty() {
        return None;
    }
    // A declared scheme is only *required* when referenced by a `security`
    // requirement. Gather every scheme name referenced at the root OR by any
    // operation (operation-level security overrides an empty root default), and
    // pick the first that resolves. If nothing references a scheme, the API is
    // unauthenticated → no auth block (Codex #106).
    let mut referenced: Vec<String> = Vec::new();
    push_security_refs(spec.get("security"), &mut referenced);
    if let Some(paths) = spec.get("paths").and_then(|v| v.as_object()) {
        for methods in paths.values() {
            if let Some(mobj) = methods.as_object() {
                for (m, op) in mobj {
                    if ["get", "post", "put", "delete", "patch"].contains(&m.as_str()) {
                        push_security_refs(op.get("security"), &mut referenced);
                    }
                }
            }
        }
    }
    // Pick the first referenced scheme that maps to a supported auth model —
    // skip unmappable alternatives (e.g. http-basic) so a mappable alternative
    // such as apiKey/bearer still wins (Codex #106).
    let secret = agent_id.to_string();
    referenced
        .iter()
        .find_map(|name| schemes.get(name).and_then(|s| map_scheme(s, &secret)))
}

/// Map an OpenAPI security scheme object to an AWARE `AuthBlock`, or `None` for
/// an unmodeled scheme (http-basic, mutualTLS, …).
fn map_scheme(scheme: &Value, secret: &str) -> Option<AuthBlock> {
    match scheme.get("type").and_then(|v| v.as_str())? {
        "apiKey" => Some(AuthBlock {
            scheme: "api-key".into(),
            location: Some(
                scheme
                    .get("in")
                    .and_then(|v| v.as_str())
                    .unwrap_or("header")
                    .to_string(),
            ),
            name: Some(
                scheme
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Authorization")
                    .to_string(),
            ),
            secret: secret.to_string(),
        }),
        "http"
            if scheme
                .get("scheme")
                .and_then(|v| v.as_str())
                .is_some_and(|s| s.eq_ignore_ascii_case("bearer")) =>
        {
            Some(AuthBlock {
                scheme: "bearer".into(),
                location: None,
                name: None,
                secret: secret.to_string(),
            })
        }
        "oauth2" | "openIdConnect" => Some(AuthBlock {
            scheme: "oauth2".into(),
            location: None,
            name: None,
            secret: secret.to_string(),
        }),
        _ => None,
    }
}

/// Quote a YAML scalar if it contains characters that would make a plain scalar
/// ambiguous (notably `:` — our parameter descriptions embed `in: query`).
fn yaml_quote(s: &str) -> String {
    let starts_special = s.chars().next().is_some_and(|c| {
        matches!(
            c,
            '-' | '?' | '*' | '&' | '!' | '|' | '>' | '@' | '`' | '%' | '[' | '{'
        )
    });
    if s.contains(':') || s.contains('#') || s.contains('"') || starts_special {
        format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
    } else {
        s.to_string()
    }
}

/// A short type label for an OpenAPI schema: the primitive `type`, `array<item>`,
/// a `$ref` component name, or `object`/`any` as a fallback.
fn schema_type_label(schema: &Value) -> String {
    if let Some(r) = schema.get("$ref").and_then(|v| v.as_str()) {
        return r.rsplit('/').next().unwrap_or("object").to_string();
    }
    match schema.get("type").and_then(|v| v.as_str()) {
        Some("array") => {
            let item = schema
                .get("items")
                .map(schema_type_label)
                .unwrap_or_else(|| "any".into());
            format!("array<{item}>")
        }
        Some(t) => t.to_string(),
        None if schema.get("properties").is_some() => "object".into(),
        None => "any".into(),
    }
}

/// Resolve a one-level `$ref` (`#/components/schemas/X`) against the spec.
/// Returns the resolved schema (or the input unchanged) plus the ref name.
fn resolve_ref<'a>(spec: &'a Value, schema: &'a Value) -> (&'a Value, Option<String>) {
    if let Some(r) = schema.get("$ref").and_then(|v| v.as_str())
        && let Some(name) = r.strip_prefix("#/components/schemas/")
        && let Some(resolved) = spec.pointer(&format!("/components/schemas/{name}"))
    {
        return (resolved, Some(name.to_string()));
    }
    (schema, None)
}

/// Resolve a parameter `$ref` (`#/components/parameters/X`) against the spec;
/// returns the input unchanged when it isn't a reference.
fn resolve_param<'a>(spec: &'a Value, p: &'a Value) -> &'a Value {
    if let Some(r) = p.get("$ref").and_then(|v| v.as_str())
        && let Some(name) = r.strip_prefix("#/components/parameters/")
        && let Some(resolved) = spec.pointer(&format!("/components/parameters/{name}"))
    {
        return resolved;
    }
    p
}

/// Pick a request/response body schema from a `content` map, preferring
/// `application/json`, else the first media type present.
fn body_schema(content: &Value) -> Option<&Value> {
    let obj = content.as_object()?;
    let media = obj
        .get("application/json")
        .or_else(|| obj.values().next())?;
    media.get("schema")
}

/// Build the `inputs:` block (without the header) from an operation's
/// `parameters` (path/query/header) + `requestBody`. Lines are emitted at
/// column 0; the manifest writer indents them under `inputs:`.
fn build_inputs_yaml(spec: &Value, op: &Value, path_params: Option<&Value>) -> String {
    let mut out = String::new();

    // Operation-level parameters first (they override path-item-level ones with
    // the same name + location per OpenAPI), then the shared path-item params.
    let mut seen: std::collections::HashSet<(String, String)> = std::collections::HashSet::new();
    for src in [op.get("parameters"), path_params] {
        let Some(params) = src.and_then(|v| v.as_array()) else {
            continue;
        };
        for p in params {
            // A parameter may be a `$ref` into `components/parameters`.
            let p = resolve_param(spec, p);
            let Some(name) = p.get("name").and_then(|v| v.as_str()) else {
                continue;
            };
            let loc = p.get("in").and_then(|v| v.as_str()).unwrap_or("query");
            if !seen.insert((name.to_string(), loc.to_string())) {
                continue; // already emitted (operation-level wins)
            }
            let required = p.get("required").and_then(|v| v.as_bool()).unwrap_or(false);
            let empty = Value::Null;
            let ty = schema_type_label(p.get("schema").unwrap_or(&empty));
            let desc = p.get("description").and_then(|v| v.as_str()).unwrap_or("");
            // `in:` is structural so the REST transport can route the param to
            // path / query / header at runtime; `required:` and `description:`
            // round out the input schema.
            out.push_str(&format!(
                "{}:\n  type: {}\n  in: {loc}\n",
                yaml_quote(name),
                yaml_quote(&ty),
            ));
            if required {
                out.push_str("  required: true\n");
            }
            if !desc.is_empty() {
                out.push_str(&format!(
                    "  description: {}\n",
                    yaml_quote(&desc.replace('\n', " "))
                ));
            }
        }
    }

    if let Some(content) = op.pointer("/requestBody/content")
        && let Some(schema) = body_schema(content)
    {
        let (resolved, ref_name) = resolve_ref(spec, schema);
        let label = ref_name.unwrap_or_else(|| "request body".into());
        out.push_str(&format!(
            "body:\n  type: {}\n  in: body\n  description: {}\n",
            yaml_quote(&schema_type_label(resolved)),
            yaml_quote(&format!("{label} (request body)"))
        ));
        if let Some(props) = resolved.get("properties").and_then(|v| v.as_object()) {
            out.push_str("  schema:\n");
            for (pname, pschema) in props {
                out.push_str(&format!(
                    "    {}: {}\n",
                    yaml_quote(pname),
                    yaml_quote(&schema_type_label(pschema))
                ));
            }
        }
    }

    out
}

/// Build the `outputs:` block (without the header) from the operation's success
/// response (`200`/`201`/`2xx`/`default`). Mirrors the curated `{ type, schema }`
/// shape so #103's compile-time reference checker has fields to validate against.
fn build_outputs_yaml(spec: &Value, op: &Value) -> String {
    let Some(responses) = op.get("responses").and_then(|v| v.as_object()) else {
        return String::new();
    };
    let resp = ["200", "201", "202", "default"]
        .iter()
        .find_map(|code| responses.get(*code))
        .or_else(|| responses.values().next());
    let Some(content) = resp.and_then(|r| r.get("content")) else {
        return String::new();
    };
    let Some(schema) = body_schema(content) else {
        return String::new();
    };
    let (resolved, _) = resolve_ref(spec, schema);
    let mut out = String::from("type: single\n");
    if let Some(props) = resolved.get("properties").and_then(|v| v.as_object()) {
        out.push_str("schema:\n");
        for (pname, pschema) in props {
            out.push_str(&format!(
                "  {}: {}\n",
                yaml_quote(pname),
                yaml_quote(&schema_type_label(pschema))
            ));
        }
    } else {
        out.push_str(&format!(
            "schema: {}\n",
            yaml_quote(&schema_type_label(resolved))
        ));
    }
    out
}

fn fetch_spec(input: &str) -> Result<String, AwareError> {
    if input.starts_with("http://") || input.starts_with("https://") {
        let resp = ureq::get(input)
            .call()
            .map_err(|e| AwareError::Network(format!("GET {input}: {e}")))?;
        let mut s = String::new();
        resp.into_reader()
            .read_to_string(&mut s)
            .map_err(|e| AwareError::Network(format!("read: {e}")))?;
        Ok(s)
    } else {
        std::fs::read_to_string(input).map_err(AwareError::Io)
    }
}

/// Convert camelCase or snake_case or "Title Case" to kebab-case. Strip non-alphanumeric.
///
/// Deliberately **not** [`crate::builder::kebab_ascii`], despite the shared name and
/// shared purpose. Two behaviours differ, and OpenAPI `operationId`s depend on both:
/// a segment opens only at a lower→upper hump, so `XMLParser` stays `xmlparser`
/// rather than becoming `x-m-l-parser`; and punctuation is *dropped* rather than
/// treated as a separator, so `Pet.Store` is `petstore`, not `pet-store`. It is also
/// Unicode-aware (`char::is_alphanumeric`) where `kebab_ascii` is ASCII-only.
/// Collapsing the two would need a flag at every call site to pick a spelling —
/// which is the tell that they are two functions, not one.
fn kebab(s: &str) -> String {
    let mut out = String::new();
    let mut prev_lower = false;
    for c in s.chars() {
        if c.is_uppercase() {
            if prev_lower && !out.is_empty() {
                out.push('-');
            }
            for lc in c.to_lowercase() {
                out.push(lc);
            }
            prev_lower = false;
        } else if c.is_whitespace() || c == '_' || c == '-' {
            if !out.ends_with('-') && !out.is_empty() {
                out.push('-');
            }
            prev_lower = false;
        } else if c.is_alphanumeric() {
            out.push(c);
            prev_lower = c.is_ascii_lowercase() || c.is_ascii_digit();
        }
    }
    out.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PETSTORE_MINI: &str = r#"{
        "openapi": "3.0.0",
        "info": {
            "title": "Petstore",
            "version": "1.0.0",
            "description": "Tiny petstore",
            "license": { "name": "MIT" }
        },
        "paths": {
            "/pets": {
                "get": { "operationId": "listPets", "summary": "List pets" },
                "post": { "operationId": "createPet", "summary": "Create a pet" }
            },
            "/pets/{petId}": {
                "get": { "operationId": "getPet", "summary": "Get pet by id" }
            }
        }
    }"#;

    #[test]
    fn parses_petstore_to_3_commands() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("petstore.json");
        std::fs::write(&path, PETSTORE_MINI).unwrap();
        let agent = build_from_url_or_path(path.to_str().unwrap(), None).unwrap();
        assert_eq!(agent.id, "petstore");
        assert_eq!(agent.commands.len(), 3);
        assert!(agent.commands.contains_key("list-pets"));
        assert!(agent.commands.contains_key("create-pet"));
        assert!(agent.commands.contains_key("get-pet"));
        assert_eq!(agent.license, "MIT");
    }

    #[test]
    fn override_agent_id() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("p.json");
        std::fs::write(&path, PETSTORE_MINI).unwrap();
        let agent = build_from_url_or_path(path.to_str().unwrap(), Some("custom-id")).unwrap();
        assert_eq!(agent.id, "custom-id");
    }

    #[test]
    fn fallback_command_name_when_no_operation_id() {
        let spec = r#"{
            "openapi": "3.0.0",
            "info": { "title": "noop", "version": "0.0.1" },
            "paths": {
                "/health": { "get": { "summary": "Health check" } }
            }
        }"#;
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("noop.json");
        std::fs::write(&path, spec).unwrap();
        let agent = build_from_url_or_path(path.to_str().unwrap(), None).unwrap();
        assert_eq!(agent.commands.len(), 1);
        assert!(agent.commands.contains_key("get-health"));
    }

    #[test]
    fn yaml_spec_also_works() {
        let yaml = r#"openapi: 3.0.0
info:
  title: yaml-spec
  version: 0.1.0
paths:
  /items:
    get:
      operationId: listItems
      summary: List items
"#;
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("spec.yaml");
        std::fs::write(&path, yaml).unwrap();
        let agent = build_from_url_or_path(path.to_str().unwrap(), None).unwrap();
        assert_eq!(agent.commands.len(), 1);
        assert!(agent.commands.contains_key("list-items"));
    }

    #[test]
    fn captures_params_request_body_and_response_schema() {
        let spec = r##"{
            "openapi": "3.0.0",
            "info": { "title": "shop", "version": "1.0.0" },
            "components": { "schemas": { "Pet": { "type": "object", "properties": {
                "id": { "type": "integer" },
                "name": { "type": "string" },
                "tags": { "type": "array", "items": { "type": "string" } }
            } } } },
            "paths": {
                "/pets/{petId}": { "get": {
                    "operationId": "getPet", "summary": "Get a pet",
                    "parameters": [
                        { "name": "petId", "in": "path", "required": true, "schema": { "type": "integer" }, "description": "id of pet" },
                        { "name": "verbose", "in": "query", "schema": { "type": "boolean" } }
                    ],
                    "responses": { "200": { "content": { "application/json": { "schema": { "$ref": "#/components/schemas/Pet" } } } } }
                } },
                "/pets": { "post": {
                    "operationId": "addPet", "summary": "Add a pet",
                    "requestBody": { "content": { "application/json": { "schema": { "$ref": "#/components/schemas/Pet" } } } },
                    "responses": { "201": { "content": { "application/json": { "schema": { "$ref": "#/components/schemas/Pet" } } } } }
                } }
            }
        }"##;
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("shop.json");
        std::fs::write(&path, spec).unwrap();
        let agent = build_from_url_or_path(path.to_str().unwrap(), None).unwrap();

        // GET: path + query params captured with type + location.
        let get = &agent.commands["get-pet"];
        assert!(
            get.inputs_yaml.contains("petId:"),
            "params: {}",
            get.inputs_yaml
        );
        assert!(get.inputs_yaml.contains("type: integer"));
        assert!(get.inputs_yaml.contains("in: path"));
        assert!(get.inputs_yaml.contains("required: true"));
        assert!(get.inputs_yaml.contains("verbose:"));
        // Response $ref resolved to Pet's fields (incl. array<string>).
        assert!(
            get.outputs_yaml.contains("type: single"),
            "out: {}",
            get.outputs_yaml
        );
        assert!(get.outputs_yaml.contains("name: string"));
        assert!(get.outputs_yaml.contains("tags: array<string>"));

        // POST: requestBody $ref resolved into a body schema.
        let post = &agent.commands["add-pet"];
        assert!(
            post.inputs_yaml.contains("body:"),
            "body: {}",
            post.inputs_yaml
        );
        assert!(post.inputs_yaml.contains("Pet (request body)"));
        assert!(post.inputs_yaml.contains("name: string"));

        // The generated manifest must be valid, parseable YAML that round-trips
        // through the loader (guards the string-built inputs/outputs indentation).
        let out = tempfile::tempdir().unwrap();
        let root = crate::builder::write_agent(&agent, out.path()).unwrap();
        let loaded = crate::manifest::loader::load_agent(&root.join("manifest.yaml")).unwrap();
        let getcmd = loaded.commands.get("get-pet").unwrap();
        assert!(
            getcmd.inputs.get("petId").is_some(),
            "inputs did not round-trip: {:?}",
            getcmd.inputs
        );
        assert!(getcmd.outputs.is_some(), "outputs did not round-trip");
    }

    #[test]
    fn quotes_colon_bearing_names_so_manifest_stays_valid() {
        // OpenAPI parameter/property names are arbitrary strings — a colon must
        // not corrupt the generated YAML (regression guard for the writer).
        let spec = r##"{
            "openapi": "3.0.0",
            "info": { "title": "weird", "version": "1.0.0" },
            "components": { "schemas": { "Odd": { "type": "object", "properties": {
                "a:b": { "type": "string" }
            } } } },
            "paths": { "/x": { "post": {
                "operationId": "doX",
                "parameters": [ { "name": "q:p", "in": "query", "schema": { "type": "string" } } ],
                "requestBody": { "content": { "application/json": { "schema": { "$ref": "#/components/schemas/Odd" } } } },
                "responses": { "200": { "content": { "application/json": { "schema": { "$ref": "#/components/schemas/Odd" } } } } }
            } } }
        }"##;
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("weird.json");
        std::fs::write(&path, spec).unwrap();
        let agent = build_from_url_or_path(path.to_str().unwrap(), None).unwrap();

        // Must write + reload without YAML corruption.
        let out = tempfile::tempdir().unwrap();
        let root = crate::builder::write_agent(&agent, out.path()).unwrap();
        let loaded = crate::manifest::loader::load_agent(&root.join("manifest.yaml")).unwrap();
        let cmd = loaded.commands.get("do-x").unwrap();
        assert!(
            cmd.inputs.get("q:p").is_some(),
            "colon-bearing param key corrupted: {:?}",
            cmd.inputs
        );
    }

    #[test]
    fn captures_security_scheme_as_auth_block() {
        let spec = r##"{
            "openapi": "3.0.0",
            "info": { "title": "secured", "version": "1.0.0" },
            "servers": [ { "url": "https://api.example.com/v1" } ],
            "components": { "securitySchemes": {
                "api_key": { "type": "apiKey", "in": "header", "name": "X-API-Key" }
            } },
            "security": [ { "api_key": [] } ],
            "paths": { "/ping": { "get": { "operationId": "ping", "summary": "Ping" } } }
        }"##;
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("secured.json");
        std::fs::write(&path, spec).unwrap();
        let agent = build_from_url_or_path(path.to_str().unwrap(), Some("secured-demo")).unwrap();

        let rest = agent
            .rest
            .as_ref()
            .expect("an OpenAPI agent uses the rest transport");
        assert_eq!(rest.base.as_deref(), Some("https://api.example.com/v1"));
        let auth = rest
            .auth
            .as_ref()
            .expect("declared securityScheme must yield an auth block");
        assert_eq!(auth.scheme, "api-key");
        assert_eq!(auth.location.as_deref(), Some("header"));
        assert_eq!(auth.name.as_deref(), Some("X-API-Key"));
        assert_eq!(auth.secret, "secured-demo");

        // Round-trip: manifest carries auth: + requires.secrets + rest transport
        // (no cli), and the loader parses it all back.
        let out = tempfile::tempdir().unwrap();
        let root = crate::builder::write_agent(&agent, out.path()).unwrap();
        let loaded = crate::manifest::loader::load_agent(&root.join("manifest.yaml")).unwrap();
        let la = loaded.auth.expect("auth did not round-trip");
        assert_eq!(la.scheme, "api-key");
        assert_eq!(la.name.as_deref(), Some("X-API-Key"));
        assert_eq!(la.secret, "secured-demo");
        assert!(loaded.transport.rest.is_some());
        assert!(loaded.transport.cli.is_none());
        assert!(
            loaded
                .requires
                .expect("requires")
                .secrets
                .contains(&"secured-demo".to_string())
        );
    }

    #[test]
    fn maps_bearer_and_leaves_unauthenticated_specs_without_auth() {
        let tmp = tempfile::tempdir().unwrap();
        let bearer = r##"{ "openapi":"3.0.0","info":{"title":"b","version":"1"},
            "components":{"securitySchemes":{"jwt":{"type":"http","scheme":"bearer"}}},
            "security":[{"jwt":[]}],
            "paths":{} }"##;
        let p = tmp.path().join("b.json");
        std::fs::write(&p, bearer).unwrap();
        let a = build_from_url_or_path(p.to_str().unwrap(), Some("b")).unwrap();
        assert_eq!(a.rest.unwrap().auth.unwrap().scheme, "bearer");

        let unauth = r##"{ "openapi":"3.0.0","info":{"title":"u","version":"1"},"paths":{} }"##;
        let p2 = tmp.path().join("u.json");
        std::fs::write(&p2, unauth).unwrap();
        let a2 = build_from_url_or_path(p2.to_str().unwrap(), Some("u")).unwrap();
        let rest = a2.rest.expect("rest transport even when unauthenticated");
        assert!(rest.auth.is_none(), "unauthenticated spec → no auth block");
    }

    #[test]
    fn merges_path_item_params_and_honors_op_level_security() {
        // Shared path-item `parameters` apply to every operation, and an
        // operation's `security` overrides an empty root `security: []`.
        let spec = r##"{
            "openapi": "3.0.0",
            "info": { "title": "shop", "version": "1.0.0" },
            "components": { "securitySchemes": {
                "api_key": { "type": "apiKey", "in": "header", "name": "X-Key" }
            } },
            "security": [],
            "paths": { "/pets/{petId}": {
                "parameters": [
                    { "name": "petId", "in": "path", "required": true, "schema": { "type": "integer" } }
                ],
                "get": {
                    "operationId": "getPet", "summary": "Get",
                    "security": [ { "api_key": [] } ],
                    "responses": {}
                }
            } }
        }"##;
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("shop.json");
        std::fs::write(&path, spec).unwrap();
        let agent = build_from_url_or_path(path.to_str().unwrap(), Some("shop")).unwrap();

        // Path-item `petId` is merged into the operation's inputs as a path param.
        let get = &agent.commands["get-pet"];
        assert!(
            get.inputs_yaml.contains("petId:"),
            "in: {}",
            get.inputs_yaml
        );
        assert!(
            get.inputs_yaml.contains("in: path"),
            "in: {}",
            get.inputs_yaml
        );
        // Empty root `security: []` but op-level security references api_key.
        let auth = agent
            .rest
            .as_ref()
            .and_then(|r| r.auth.as_ref())
            .expect("op-level security must yield an auth block");
        assert_eq!(auth.scheme, "api-key");
        assert_eq!(auth.name.as_deref(), Some("X-Key"));
    }

    #[test]
    fn mutating_method_is_write_mode_and_param_refs_resolve() {
        // A `$ref` path-item parameter resolves into the inputs, and a mutating
        // method (DELETE) is emitted as `mode: write` so the safety contract
        // applies (Codex #106).
        let spec = r##"{
            "openapi": "3.0.0",
            "info": { "title": "shop", "version": "1.0.0" },
            "components": { "parameters": {
                "PetId": { "name": "petId", "in": "path", "required": true, "schema": { "type": "integer" } }
            } },
            "paths": { "/pets/{petId}": {
                "parameters": [ { "$ref": "#/components/parameters/PetId" } ],
                "delete": { "operationId": "deletePet", "responses": {} }
            } }
        }"##;
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("shop.json");
        std::fs::write(&path, spec).unwrap();
        let agent = build_from_url_or_path(path.to_str().unwrap(), Some("shop")).unwrap();

        let del = &agent.commands["delete-pet"];
        // `$ref` path param resolved into the inputs.
        assert!(
            del.inputs_yaml.contains("petId:"),
            "in: {}",
            del.inputs_yaml
        );
        assert!(
            del.inputs_yaml.contains("in: path"),
            "in: {}",
            del.inputs_yaml
        );
        // DELETE → write mode.
        assert_eq!(del.mode.as_deref(), Some("write"));

        // Round-trip: the loader sees the command as write-mode.
        let out = tempfile::tempdir().unwrap();
        let root = crate::builder::write_agent(&agent, out.path()).unwrap();
        let loaded = crate::manifest::loader::load_agent(&root.join("manifest.yaml")).unwrap();
        let cmd = loaded.commands.get("delete-pet").unwrap();
        assert_eq!(cmd.mode, Some(crate::manifest::agent::Mode::Write));
    }

    #[test]
    fn server_base_derives_from_input_when_servers_relative_or_absent() {
        let url_input = "https://api.example.com/v3/openapi.json";
        // No `servers` → the input URL's origin.
        let no_servers = serde_json::json!({});
        assert_eq!(
            server_base(&no_servers, url_input).as_deref(),
            Some("https://api.example.com")
        );
        // Leading-slash relative server → origin + path.
        let rel = serde_json::json!({ "servers": [ { "url": "/api/v3" } ] });
        assert_eq!(
            server_base(&rel, url_input).as_deref(),
            Some("https://api.example.com/api/v3")
        );
        // No-leading-slash relative → resolved against the document directory.
        let rel_doc = serde_json::json!({ "servers": [ { "url": "v1" } ] });
        assert_eq!(
            server_base(&rel_doc, url_input).as_deref(),
            Some("https://api.example.com/v3/v1")
        );
        // Absolute server → used verbatim.
        let abs = serde_json::json!({ "servers": [ { "url": "https://cdn.example.com/base" } ] });
        assert_eq!(
            server_base(&abs, url_input).as_deref(),
            Some("https://cdn.example.com/base")
        );
        // Local-file input with no absolute server → no base.
        assert_eq!(server_base(&no_servers, "/tmp/spec.json"), None);
    }

    #[test]
    fn public_operations_are_marked_no_auth() {
        // Root security secures the API; an operation with `security: []` is
        // public and must opt out of the agent's auth (Codex #106).
        let spec = r##"{
            "openapi": "3.0.0",
            "info": { "title": "mixed", "version": "1.0.0" },
            "components": { "securitySchemes": {
                "api_key": { "type": "apiKey", "in": "header", "name": "X-Key" }
            } },
            "security": [ { "api_key": [] } ],
            "paths": {
                "/me": { "get": { "operationId": "getMe", "responses": {} } },
                "/health": { "get": { "operationId": "health", "security": [], "responses": {} } }
            }
        }"##;
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("mixed.json");
        std::fs::write(&path, spec).unwrap();
        let agent = build_from_url_or_path(path.to_str().unwrap(), Some("mixed")).unwrap();

        // The agent still carries auth (for the secured operations).
        assert!(agent.rest.as_ref().and_then(|r| r.auth.as_ref()).is_some());
        // `/me` (root security applies) → secured; `/health` (`security: []`) → public.
        assert!(
            !agent.commands["get-me"].no_auth,
            "secured op wrongly marked public"
        );
        assert!(
            agent.commands["health"].no_auth,
            "public op not marked no-auth"
        );
    }

    #[test]
    fn optional_auth_with_anonymous_alternative_is_public() {
        // `security: [{}, {api_key: []}]` means auth is optional (the empty `{}`
        // is an anonymous alternative) → the endpoint is public (Codex #106).
        let spec = r##"{
            "openapi": "3.0.0",
            "info": { "title": "opt", "version": "1.0.0" },
            "components": { "securitySchemes": {
                "api_key": { "type": "apiKey", "in": "header", "name": "X-Key" }
            } },
            "security": [ {}, { "api_key": [] } ],
            "paths": { "/data": { "get": { "operationId": "getData", "responses": {} } } }
        }"##;
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("opt.json");
        std::fs::write(&path, spec).unwrap();
        let agent = build_from_url_or_path(path.to_str().unwrap(), Some("opt")).unwrap();
        assert!(
            agent.commands["get-data"].no_auth,
            "optional-auth (anonymous alternative) endpoint should be public"
        );
    }

    #[test]
    fn auth_skips_unmappable_scheme_for_mappable_alternative() {
        // `security: [{basic}, {api_key}]` — basic is unmodeled, but the apiKey
        // alternative is mappable and must be chosen (Codex #106).
        let spec = r##"{
            "openapi": "3.0.0",
            "info": { "title": "alt", "version": "1.0.0" },
            "components": { "securitySchemes": {
                "basic": { "type": "http", "scheme": "basic" },
                "api_key": { "type": "apiKey", "in": "header", "name": "X-Key" }
            } },
            "security": [ { "basic": [] }, { "api_key": [] } ],
            "paths": { "/x": { "get": { "operationId": "getX", "responses": {} } } }
        }"##;
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("alt.json");
        std::fs::write(&path, spec).unwrap();
        let agent = build_from_url_or_path(path.to_str().unwrap(), Some("alt")).unwrap();
        let auth = agent
            .rest
            .as_ref()
            .and_then(|r| r.auth.as_ref())
            .expect("mappable alternative must yield an auth block");
        assert_eq!(auth.scheme, "api-key");
        assert_eq!(auth.name.as_deref(), Some("X-Key"));
    }

    #[test]
    fn operation_level_server_overrides_base() {
        // An operation's own `servers` overrides the root base → that command
        // gets an absolute path targeting the override host (Codex #106).
        let spec = r##"{
            "openapi": "3.0.0",
            "info": { "title": "multi", "version": "1.0.0" },
            "servers": [ { "url": "https://api.example.com" } ],
            "paths": { "/upload": { "post": {
                "operationId": "upload",
                "servers": [ { "url": "https://upload.example.com" } ],
                "responses": {}
            } } }
        }"##;
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("multi.json");
        std::fs::write(&path, spec).unwrap();
        let agent = build_from_url_or_path(path.to_str().unwrap(), Some("multi")).unwrap();
        // Operation override → absolute command path on the upload host.
        assert_eq!(
            agent.commands["upload"].path.as_deref(),
            Some("https://upload.example.com/upload")
        );
        // Agent base is still the root server.
        assert_eq!(
            agent.rest.as_ref().and_then(|r| r.base.as_deref()),
            Some("https://api.example.com")
        );
    }

    #[test]
    fn kebab_basic_cases() {
        assert_eq!(kebab("camelCase"), "camel-case");
        assert_eq!(kebab("snake_case"), "snake-case");
        assert_eq!(kebab("Title Case"), "title-case");
        assert_eq!(kebab("already-kebab"), "already-kebab");
        assert_eq!(kebab("XMLParser"), "xmlparser"); // acronym handling is loose; acceptable
    }

    // ── helpers for the spec-driven cases below ─────────────────────────────
    //
    // Every builder path runs from a file, so each case writes its spec to a
    // temp dir. `built` keeps that plumbing out of the assertions.

    fn built(spec: &str, id: &str) -> GeneratedAgent {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("spec.json");
        std::fs::write(&path, spec).unwrap();
        build_from_url_or_path(path.to_str().unwrap(), Some(id))
            .unwrap_or_else(|e| panic!("spec did not build: {e}"))
    }

    /// Write the agent out and read it back through the real loader, which is the
    /// only way to observe YAML corruption in the string-built `inputs:` block.
    fn round_tripped(agent: &GeneratedAgent) -> crate::manifest::agent::Agent {
        let out = tempfile::tempdir().unwrap();
        let root = crate::builder::write_agent(agent, out.path()).unwrap();
        crate::manifest::loader::load_agent(&root.join("manifest.yaml"))
            .unwrap_or_else(|e| panic!("generated manifest did not load: {e}"))
    }

    // ── which response describes the output ─────────────────────────────────

    #[test]
    fn the_output_schema_comes_from_the_success_response_not_whichever_is_listed_first() {
        // `responses` is an ordered map and specs routinely list error codes
        // first. `build_outputs_yaml` therefore *prefers* 200/201/202/default and
        // only falls back to "the first one" when none of those exist. Nothing
        // pinned the preference: a build that took `.values().next()` outright
        // would emit the 404 body as the command's declared output, and #103's
        // compile-time reference checker would then validate app expressions
        // against the error shape — silently, since the manifest still parses.
        let spec = r##"{
            "openapi": "3.0.0",
            "info": { "title": "codes", "version": "1.0.0" },
            "components": { "schemas": {
                "Err": { "type": "object", "properties": { "message": { "type": "string" } } },
                "Ok":  { "type": "object", "properties": { "pets": { "type": "integer" } } }
            } },
            "paths": { "/pets": { "get": {
                "operationId": "listPets",
                "responses": {
                    "404": { "content": { "application/json": { "schema": { "$ref": "#/components/schemas/Err" } } } },
                    "200": { "content": { "application/json": { "schema": { "$ref": "#/components/schemas/Ok" } } } }
                }
            } } }
        }"##;
        let agent = built(spec, "codes");
        let out = &agent.commands["list-pets"].outputs_yaml;
        assert!(out.contains("pets: integer"), "success body missing: {out}");
        assert!(
            !out.contains("message:"),
            "error body leaked into the output schema: {out}"
        );
    }

    #[test]
    fn a_response_the_preference_list_does_not_name_is_still_described() {
        // The `.or_else(first)` arm. An operation that documents only `204` or
        // only a `2XX` wildcard has no 200/201/202/default entry; without the
        // fallback its command would ship with no `outputs:` at all and every
        // app reading from it would have nothing to reference.
        let spec = r##"{
            "openapi": "3.0.0",
            "info": { "title": "odd", "version": "1.0.0" },
            "paths": { "/x": { "get": {
                "operationId": "getX",
                "responses": { "2XX": { "content": { "application/json": {
                    "schema": { "type": "object", "properties": { "ok": { "type": "boolean" } } }
                } } } }
            } } }
        }"##;
        let agent = built(spec, "odd");
        let out = &agent.commands["get-x"].outputs_yaml;
        assert!(out.contains("type: single"), "no outputs emitted: {out}");
        assert!(out.contains("ok: boolean"), "schema not described: {out}");
    }

    // ── where a command points ──────────────────────────────────────────────

    #[test]
    fn a_path_item_server_redirects_every_operation_on_that_path() {
        // OpenAPI allows `servers` at three levels. The operation level was
        // covered; the PATH-ITEM level — one override shared by every method on
        // the path — was not, so dropping that fallback would have gone
        // unnoticed and both commands would have been generated against the root
        // host, sending uploads to the wrong service with a valid-looking
        // manifest. The two operations also prove the override is not consumed
        // by the first method that reads it.
        let spec = r##"{
            "openapi": "3.0.0",
            "info": { "title": "multi", "version": "1.0.0" },
            "servers": [ { "url": "https://api.example.com" } ],
            "paths": { "/blob": {
                "servers": [ { "url": "https://storage.example.com" } ],
                "get":  { "operationId": "getBlob",  "responses": {} },
                "post": { "operationId": "putBlob", "responses": {} }
            } }
        }"##;
        let agent = built(spec, "multi");
        assert_eq!(
            agent.commands["get-blob"].path.as_deref(),
            Some("https://storage.example.com/blob")
        );
        assert_eq!(
            agent.commands["put-blob"].path.as_deref(),
            Some("https://storage.example.com/blob")
        );
        // The agent base is untouched — the override is per-command.
        assert_eq!(
            agent.rest.as_ref().and_then(|r| r.base.as_deref()),
            Some("https://api.example.com")
        );
    }

    #[test]
    fn an_operation_server_wins_over_the_path_item_server() {
        // Both levels present at once, which is the only arrangement where the
        // precedence is observable. Reading the path-item override first would
        // silently demote the operation's own, more specific host.
        let spec = r##"{
            "openapi": "3.0.0",
            "info": { "title": "multi", "version": "1.0.0" },
            "servers": [ { "url": "https://api.example.com" } ],
            "paths": { "/blob": {
                "servers": [ { "url": "https://storage.example.com" } ],
                "post": {
                    "operationId": "putBlob",
                    "servers": [ { "url": "https://upload.example.com" } ],
                    "responses": {}
                }
            } }
        }"##;
        let agent = built(spec, "multi");
        assert_eq!(
            agent.commands["put-blob"].path.as_deref(),
            Some("https://upload.example.com/blob")
        );
    }

    #[test]
    fn a_bare_origin_spec_url_still_yields_a_usable_base() {
        // `input_origin` reads the authority by finding the first `/` AFTER the
        // scheme — and a URL with no path at all has none, which is the branch
        // the existing case (a URL with `/v3/openapi.json`) never reaches.
        let no_servers = serde_json::json!({});
        assert_eq!(
            server_base(&no_servers, "https://api.example.com").as_deref(),
            Some("https://api.example.com")
        );
        // A relative server on such a document resolves against the root.
        let rel = serde_json::json!({ "servers": [ { "url": "v1" } ] });
        assert_eq!(
            server_base(&rel, "https://api.example.com").as_deref(),
            Some("https://api.example.com/v1")
        );
        // The port is part of the authority, not the path.
        assert_eq!(
            server_base(&no_servers, "http://localhost:8080/openapi.json").as_deref(),
            Some("http://localhost:8080")
        );
    }

    // ── which keys under a path item are operations ─────────────────────────

    #[test]
    fn only_real_http_operations_become_commands() {
        // A path item carries `summary`, `description`, `parameters` and
        // `servers` as siblings of its operations, and OpenAPI also defines
        // `head` / `options` / `trace`, which this builder does not model.
        // Without the method whitelist each of those keys would be emitted as a
        // callable command — `aware agent describe` would advertise an operation
        // called `parameters`, and the REST transport would try to issue it.
        let spec = r##"{
            "openapi": "3.0.0",
            "info": { "title": "noise", "version": "1.0.0" },
            "paths": { "/thing": {
                "summary": "A thing",
                "description": "Describes a thing",
                "servers": [ { "url": "https://api.example.com" } ],
                "parameters": [ { "name": "q", "in": "query", "schema": { "type": "string" } } ],
                "head":    { "operationId": "headThing", "responses": {} },
                "options": { "operationId": "optionsThing", "responses": {} },
                "trace":   { "operationId": "traceThing", "responses": {} },
                "get":     { "operationId": "getThing", "responses": {} }
            } }
        }"##;
        let agent = built(spec, "noise");
        let names: Vec<&str> = agent.commands.keys().map(String::as_str).collect();
        assert_eq!(names, vec!["get-thing"], "non-operations became commands");
        // The shared `parameters` list is still merged into the one real command.
        assert!(agent.commands["get-thing"].inputs_yaml.contains("q:"));
    }

    // ── which parameter wins ────────────────────────────────────────────────

    #[test]
    fn an_operation_parameter_overrides_the_path_item_one_it_shadows() {
        // OpenAPI: an operation's parameter replaces a path-item parameter with
        // the same name+location. The existing merge case has the two lists
        // disjoint, so it passes whichever order they are read in and whether or
        // not the dedup exists. Here they collide: reading the path-item list
        // first would declare `limit` a string, and dropping the dedup would emit
        // the key twice — a duplicate YAML mapping key, where the loader keeps
        // one silently and the other definition simply vanishes.
        let spec = r##"{
            "openapi": "3.0.0",
            "info": { "title": "shadow", "version": "1.0.0" },
            "paths": { "/pets": {
                "parameters": [
                    { "name": "limit", "in": "query", "schema": { "type": "string" }, "description": "from the path item" }
                ],
                "get": {
                    "operationId": "listPets",
                    "parameters": [
                        { "name": "limit", "in": "query", "required": true, "schema": { "type": "integer" }, "description": "from the operation" }
                    ],
                    "responses": {}
                }
            } }
        }"##;
        let agent = built(spec, "shadow");
        let inputs = &agent.commands["list-pets"].inputs_yaml;
        assert_eq!(
            inputs.matches("limit:").count(),
            1,
            "shadowed parameter emitted twice: {inputs}"
        );
        assert!(
            inputs.contains("type: integer") && !inputs.contains("type: string"),
            "path-item definition won over the operation's: {inputs}"
        );
        // And the operation's own `required` survived rather than the path item's default.
        assert!(inputs.contains("required: true"), "inputs: {inputs}");
        let loaded = round_tripped(&agent);
        let cmd = loaded.commands.get("list-pets").unwrap();
        assert_eq!(cmd.inputs["limit"]["description"], "from the operation");
    }

    // ── which media type describes a body ───────────────────────────────────

    #[test]
    fn a_body_offered_only_under_a_vendor_json_media_type_is_still_described() {
        // `body_schema` prefers `application/json` and otherwise takes the first
        // media type present. Only the preferred arm was covered, so losing the
        // fallback would generate a POST command with no `body` input at all.
        //
        // The media type here is a vendor JSON one deliberately. The builder does
        // NOT record which media type it picked, and the REST sender
        // (`runtime::invoker::build_operation_request` → `RestInvoker`) serializes
        // an object body as JSON under `Content-Type: application/json` unless the
        // app supplies its own header — so the fallback only yields a *callable*
        // command for bodies the transport can serialize that way. Reaching for
        // `application/xml` here would assert that a body input exists while the
        // operation stayed unsatisfiable, which pins a comfort rather than a
        // behaviour (Codex, #434). `+json` suffixed vendor types are also what
        // real specs use far more often than XML.
        let spec = r##"{
            "openapi": "3.0.0",
            "info": { "title": "vendory", "version": "1.0.0" },
            "components": { "schemas": {
                "Pet": { "type": "object", "properties": { "name": { "type": "string" } } }
            } },
            "paths": { "/pets": { "post": {
                "operationId": "addPet",
                "requestBody": { "content": { "application/vnd.petstore.v3+json": {
                    "schema": { "$ref": "#/components/schemas/Pet" }
                } } },
                "responses": {}
            } } }
        }"##;
        let agent = built(spec, "vendory");
        let inputs = &agent.commands["add-pet"].inputs_yaml;
        assert!(inputs.contains("body:"), "no body input: {inputs}");
        assert!(inputs.contains("Pet (request body)"), "inputs: {inputs}");
        assert!(inputs.contains("name: string"), "inputs: {inputs}");
    }

    #[test]
    fn json_wins_when_a_body_is_offered_in_several_media_types() {
        // The preference only shows when JSON is NOT the first entry — with it
        // first, "prefer json" and "take the first" agree and the case proves
        // nothing. `Xml`/`Json` carry different properties so the winner is
        // visible in the generated schema.
        let spec = r##"{
            "openapi": "3.0.0",
            "info": { "title": "both", "version": "1.0.0" },
            "components": { "schemas": {
                "Xml":  { "type": "object", "properties": { "xmlOnly":  { "type": "string" } } },
                "Json": { "type": "object", "properties": { "jsonOnly": { "type": "string" } } }
            } },
            "paths": { "/pets": { "post": {
                "operationId": "addPet",
                "requestBody": { "content": {
                    "application/xml":  { "schema": { "$ref": "#/components/schemas/Xml" } },
                    "application/json": { "schema": { "$ref": "#/components/schemas/Json" } }
                } },
                "responses": {}
            } } }
        }"##;
        let agent = built(spec, "both");
        let inputs = &agent.commands["add-pet"].inputs_yaml;
        assert!(
            inputs.contains("Json (request body)") && inputs.contains("jsonOnly: string"),
            "did not prefer application/json: {inputs}"
        );
        assert!(!inputs.contains("xmlOnly"), "inputs: {inputs}");
    }

    // ── how a property's type is labelled ───────────────────────────────────

    #[test]
    fn every_schema_shape_gets_a_distinguishable_type_label() {
        // `schema_type_label` has four arms and the suite exercised two
        // (primitive and `array<primitive>`). The other two are the ones a
        // reader relies on to tell "we could not determine a type" (`any`) from
        // "a structure we did not expand" (`object`) — collapse them and the
        // manifest stops distinguishing an undeclared field from a nested one.
        // A `$ref` inside a property, and an array of arrays, are both shapes
        // real specs produce and neither had a case.
        let spec = r##"{
            "openapi": "3.0.0",
            "info": { "title": "shapes", "version": "1.0.0" },
            "components": { "schemas": {
                "Pet": { "type": "object", "properties": { "name": { "type": "string" } } },
                "Wrap": { "type": "object", "properties": {
                    "pet":    { "$ref": "#/components/schemas/Pet" },
                    "grid":   { "type": "array", "items": { "type": "array", "items": { "type": "number" } } },
                    "loose":  {},
                    "nested": { "properties": { "x": { "type": "string" } } },
                    "bag":    { "type": "array" }
                } }
            } },
            "paths": { "/w": { "get": {
                "operationId": "getW",
                "responses": { "200": { "content": { "application/json": {
                    "schema": { "$ref": "#/components/schemas/Wrap" }
                } } } }
            } } }
        }"##;
        let agent = built(spec, "shapes");
        let out = &agent.commands["get-w"].outputs_yaml;
        assert!(out.contains("pet: Pet"), "$ref label: {out}");
        assert!(out.contains("grid: array<array<number>>"), "nested: {out}");
        assert!(out.contains("loose: any"), "untyped: {out}");
        assert!(out.contains("nested: object"), "properties-only: {out}");
        assert!(out.contains("bag: array<any>"), "itemless array: {out}");
    }

    // ── keeping the generated YAML honest ───────────────────────────────────

    #[test]
    fn a_hash_in_a_description_is_not_swallowed_as_a_yaml_comment() {
        // ` #` opens a comment in a YAML plain scalar, so an unquoted
        // `description: batch # 3 of 7` loads as `batch` — the manifest parses
        // cleanly and simply carries a description nobody wrote. That silent
        // truncation is why `yaml_quote` tests for `#`, and only the `:` trigger
        // had a case. The assertion has to run through the real loader: the
        // generated string contains the full text either way.
        let spec = r##"{
            "openapi": "3.0.0",
            "info": { "title": "hashy", "version": "1.0.0" },
            "paths": { "/x": { "get": {
                "operationId": "getX",
                "parameters": [
                    { "name": "batch", "in": "query", "schema": { "type": "string" },
                      "description": "batch # 3 of 7" }
                ],
                "responses": {}
            } } }
        }"##;
        let loaded = round_tripped(&built(spec, "hashy"));
        let cmd = loaded.commands.get("get-x").unwrap();
        assert_eq!(cmd.inputs["batch"]["description"], "batch # 3 of 7");
    }

    #[test]
    fn a_description_opening_with_a_yaml_indicator_does_not_break_the_manifest() {
        // The other half of `yaml_quote`'s guard. `*` opens an alias and `%` a
        // directive — unquoted, these are not a truncation but a hard parse
        // error, so the builder would emit a manifest that no longer loads at
        // all. Vendor descriptions really do start with `*` (emphasis) and `%`.
        let spec = r##"{
            "openapi": "3.0.0",
            "info": { "title": "starty", "version": "1.0.0" },
            "paths": { "/x": { "get": {
                "operationId": "getX",
                "parameters": [
                    { "name": "a", "in": "query", "schema": { "type": "string" },
                      "description": "*deprecated* use b" },
                    { "name": "b", "in": "query", "schema": { "type": "string" },
                      "description": "% of total" },
                    { "name": "c", "in": "query", "schema": { "type": "string" },
                      "description": "- leading dash" }
                ],
                "responses": {}
            } } }
        }"##;
        let loaded = round_tripped(&built(spec, "starty"));
        let cmd = loaded.commands.get("get-x").unwrap();
        assert_eq!(cmd.inputs["a"]["description"], "*deprecated* use b");
        assert_eq!(cmd.inputs["b"]["description"], "% of total");
        assert_eq!(cmd.inputs["c"]["description"], "- leading dash");
    }

    #[test]
    fn a_quote_bearing_description_is_escaped_rather_than_terminating_the_scalar() {
        // Once `yaml_quote` decides to quote, the payload must be escaped or the
        // embedded `"` closes the scalar early — turning a description into a
        // parse error or, worse, into trailing tokens the loader reinterprets.
        // The `\` case guards the escape ORDER: escaping quotes before
        // backslashes would double-escape the backslash it just inserted.
        let spec = r##"{
            "openapi": "3.0.0",
            "info": { "title": "quoted", "version": "1.0.0" },
            "paths": { "/x": { "get": {
                "operationId": "getX",
                "parameters": [
                    { "name": "a", "in": "query", "schema": { "type": "string" },
                      "description": "the \"id\": a key" },
                    { "name": "b", "in": "query", "schema": { "type": "string" },
                      "description": "a path: C:\\tmp\\x" }
                ],
                "responses": {}
            } } }
        }"##;
        let loaded = round_tripped(&built(spec, "quoted"));
        let cmd = loaded.commands.get("get-x").unwrap();
        assert_eq!(cmd.inputs["a"]["description"], "the \"id\": a key");
        assert_eq!(cmd.inputs["b"]["description"], "a path: C:\\tmp\\x");
    }
}
