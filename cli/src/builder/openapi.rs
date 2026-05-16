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
                            inputs_yaml: String::new(),
                            outputs_yaml: String::new(),
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
        version,
        description,
        commands,
        skills: Vec::new(),
        provenance,
        stateful: false,
        license,
    })
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
    fn kebab_basic_cases() {
        assert_eq!(kebab("camelCase"), "camel-case");
        assert_eq!(kebab("snake_case"), "snake-case");
        assert_eq!(kebab("Title Case"), "title-case");
        assert_eq!(kebab("already-kebab"), "already-kebab");
        assert_eq!(kebab("XMLParser"), "xmlparser"); // acronym handling is loose; acceptable
    }
}
