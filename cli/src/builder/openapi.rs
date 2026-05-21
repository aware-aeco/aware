//! OpenAPI 3.x → AWARE agent.
//!
//! Walks `paths.<path>.<method>` operations and emits one command per operation.
//! Accepts JSON or YAML; HTTP(S) URL or local file path.

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::io::Read;

use serde_json::Value;

use crate::builder::{GeneratedAgent, GeneratedCommand, Provenance, now_iso};
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

    let mut commands = BTreeMap::new();
    if let Some(paths) = spec["paths"].as_object() {
        for (path, methods) in paths {
            if let Some(methods_obj) = methods.as_object() {
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
                    commands.insert(
                        kebab(&raw_op_id),
                        GeneratedCommand {
                            lifecycle: "single".into(),
                            description: format!(
                                "{} {path} \u{2014} {summary}",
                                method.to_uppercase()
                            ),
                            inputs_yaml: build_inputs_yaml(&spec, op),
                            outputs_yaml: build_outputs_yaml(&spec, op),
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
    })
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
fn build_inputs_yaml(spec: &Value, op: &Value) -> String {
    let mut out = String::new();

    if let Some(params) = op.get("parameters").and_then(|v| v.as_array()) {
        for p in params {
            let Some(name) = p.get("name").and_then(|v| v.as_str()) else {
                continue;
            };
            let loc = p.get("in").and_then(|v| v.as_str()).unwrap_or("query");
            let required = p.get("required").and_then(|v| v.as_bool()).unwrap_or(false);
            let empty = Value::Null;
            let ty = schema_type_label(p.get("schema").unwrap_or(&empty));
            let desc = p.get("description").and_then(|v| v.as_str()).unwrap_or("");
            let note = format!(
                "in: {loc}{}{}",
                if required { ", required" } else { "" },
                if desc.is_empty() {
                    String::new()
                } else {
                    format!(" — {}", desc.replace('\n', " "))
                }
            );
            out.push_str(&format!(
                "{name}:\n  type: {ty}\n  description: {}\n",
                yaml_quote(&note)
            ));
        }
    }

    if let Some(content) = op.pointer("/requestBody/content")
        && let Some(schema) = body_schema(content)
    {
        let (resolved, ref_name) = resolve_ref(spec, schema);
        let label = ref_name.unwrap_or_else(|| "request body".into());
        out.push_str(&format!(
            "body:\n  type: {}\n  description: {}\n",
            schema_type_label(resolved),
            yaml_quote(&format!("{label} (request body)"))
        ));
        if let Some(props) = resolved.get("properties").and_then(|v| v.as_object()) {
            out.push_str("  schema:\n");
            for (pname, pschema) in props {
                out.push_str(&format!("    {pname}: {}\n", schema_type_label(pschema)));
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
            out.push_str(&format!("  {pname}: {}\n", schema_type_label(pschema)));
        }
    } else {
        out.push_str(&format!("schema: {}\n", schema_type_label(resolved)));
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
pub(crate) fn kebab(s: &str) -> String {
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
        assert!(get.inputs_yaml.contains("in: path, required"));
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
    fn kebab_basic_cases() {
        assert_eq!(kebab("camelCase"), "camel-case");
        assert_eq!(kebab("snake_case"), "snake-case");
        assert_eq!(kebab("Title Case"), "title-case");
        assert_eq!(kebab("already-kebab"), "already-kebab");
        assert_eq!(kebab("XMLParser"), "xmlparser"); // acronym handling is loose; acceptable
    }
}
