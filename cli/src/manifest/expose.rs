//! `exposes-as-agent` support: synthesize a callable agent manifest from an
//! app's `exposed-commands:` block, and type-validate a caller's routed inputs
//! against an exposed command's declared input types.
//!
//! When an app sets `exposes-as-agent: true`, install writes the manifest this
//! module produces to `<aware-home>/agents/<app>/manifest.yaml`, so the app
//! resolves and dispatches exactly like a hand-written agent
//! (`agent: <app>, command: <cmd>`). See `10-core/app-spec.md § exposes-as-agent`
//! and `10-core/agent-spec.md § The callable contract`.

use std::collections::BTreeMap;

use serde::Serialize;
use serde_yaml::Value;

use crate::error::AwareError;
use crate::manifest::App;
use crate::manifest::agent::{Lifecycle, Mode};
use crate::manifest::app::ExposedCommand;

/// License recorded on a synthesized agent manifest. Apps carry no `license:`
/// field; the real license is the backing app's. This marker keeps the field
/// (required on every agent manifest) non-empty and self-documenting.
const SYNTH_LICENSE: &str = "app-exposed";

#[derive(Serialize)]
struct SynthManifest {
    agent: String,
    version: String,
    #[serde(rename = "display-name", skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    description: String,
    stateful: bool,
    license: &'static str,
    transport: SynthTransport,
    commands: BTreeMap<String, SynthCommand>,
}

#[derive(Serialize)]
struct SynthTransport {
    app: SynthAppTransport,
}

#[derive(Serialize)]
struct SynthAppTransport {
    #[serde(rename = "backed-by")]
    backed_by: String,
}

#[derive(Serialize)]
struct SynthCommand {
    lifecycle: &'static str,
    description: String,
    mode: &'static str,
    #[serde(skip_serializing_if = "Value::is_null")]
    inputs: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    outputs: Option<Value>,
}

fn lifecycle_str(l: Lifecycle) -> &'static str {
    match l {
        Lifecycle::Start => "start",
        Lifecycle::Stop => "stop",
        Lifecycle::Single => "single",
    }
}

/// The read/write mode a synthesized command presents at the app boundary.
/// Defaults to `read` (a "run and return outputs" call); an author may declare
/// `mode: write` to force callers to wrap the node in a `safety:` block.
fn exposed_mode(cmd: &ExposedCommand) -> Mode {
    cmd.mode.unwrap_or(Mode::Read)
}

/// Build the agent manifest YAML for an `exposes-as-agent: true` app. The
/// synthesized agent is `stateful` iff any exposed command is `lifecycle: start`,
/// and declares an `app` transport naming the backing app.
///
/// Errors when the app does not actually expose itself or declares no commands
/// (callers should gate on validation first; this is a defensive backstop).
pub fn synthesize_agent_manifest(app: &App) -> Result<String, AwareError> {
    if !app.exposes_as_agent {
        return Err(AwareError::Validation(format!(
            "app {} does not set exposes-as-agent: true",
            app.app
        )));
    }
    let exposed = app.exposed_commands.as_ref().filter(|m| !m.is_empty());
    let Some(exposed) = exposed else {
        return Err(AwareError::Validation(format!(
            "app {} exposes-as-agent but declares no exposed-commands",
            app.app
        )));
    };

    let stateful = exposed.values().any(|c| c.lifecycle == Lifecycle::Start);
    let mut commands = BTreeMap::new();
    for (name, cmd) in exposed {
        commands.insert(
            name.clone(),
            SynthCommand {
                lifecycle: lifecycle_str(cmd.lifecycle),
                description: cmd
                    .description
                    .clone()
                    .unwrap_or_else(|| format!("Exposed command `{name}` of app `{}`.", app.app)),
                mode: exposed_mode(cmd).as_str(),
                inputs: cmd.inputs.clone(),
                outputs: cmd.outputs.clone(),
            },
        );
    }

    let manifest = SynthManifest {
        agent: app.app.clone(),
        version: app.version.clone(),
        display_name: app.display_name.clone(),
        description: app
            .description
            .lines()
            .next()
            .unwrap_or(&app.description)
            .trim()
            .to_string(),
        stateful,
        license: SYNTH_LICENSE,
        transport: SynthTransport {
            app: SynthAppTransport {
                backed_by: app.app.clone(),
            },
        },
        commands,
    };

    let yaml = serde_yaml::to_string(&manifest)
        .map_err(|e| AwareError::Internal(format!("synthesize agent manifest: {e}")))?;
    Ok(format!(
        "# Synthesized by `aware app install` from {app}'s exposes-as-agent / exposed-commands.\n\
         # Do not edit by hand — reinstall {app} to regenerate. Source: apps/{app}/.\n{yaml}",
        app = app.app
    ))
}

/// Type-validate a caller's routed inputs against an exposed command's declared
/// input types before they enter the nested app's `inputs:` map. Each declared
/// input whose `type:` is a known scalar/structured type is checked against the
/// provided value; an unknown declared type (e.g. `enum`) or an input the caller
/// did not supply is left alone (the nested app may default it). Inputs the
/// caller supplies beyond the declared set pass through unchecked.
pub fn validate_exposed_inputs(
    command: &str,
    exposed: &ExposedCommand,
    args: &serde_json::Value,
) -> Result<(), AwareError> {
    let Some(declared) = exposed.inputs.as_mapping() else {
        return Ok(());
    };
    let Some(provided) = args.as_object() else {
        return Ok(());
    };
    for (name, decl) in declared {
        let Some(name) = name.as_str() else { continue };
        let Some(declared_type) = decl.get("type").and_then(|t| t.as_str()) else {
            continue;
        };
        let Some(value) = provided.get(name) else {
            continue;
        };
        if !json_matches_type(value, declared_type) {
            return Err(AwareError::Validation(format!(
                "exposed command `{command}` input `{name}`: expected {declared_type}, got {}",
                json_type_name(value)
            )));
        }
    }
    Ok(())
}

/// Whether a JSON value satisfies a declared input `type`. Unknown type names
/// (e.g. `enum`, vendor-specific) are permissive — they return `true` so a
/// reasonable value is never rejected on a type AWARE doesn't model.
fn json_matches_type(value: &serde_json::Value, declared: &str) -> bool {
    match declared {
        "string" | "str" | "text" => value.is_string(),
        "number" | "num" | "integer" | "int" | "float" | "double" => value.is_number(),
        "boolean" | "bool" => value.is_boolean(),
        "object" | "map" | "mapping" => value.is_object(),
        "array" | "list" => value.is_array(),
        _ => true,
    }
}

fn json_type_name(value: &serde_json::Value) -> &'static str {
    match value {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "boolean",
        serde_json::Value::Number(_) => "number",
        serde_json::Value::String(_) => "string",
        serde_json::Value::Array(_) => "array",
        serde_json::Value::Object(_) => "object",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::Agent;

    fn welded_to_tc() -> App {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("30-apps/_examples/welded-to-tc.flo");
        serde_yaml::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }

    #[test]
    fn synthesizes_a_parseable_agent_manifest_from_welded_to_tc() {
        let app = welded_to_tc();
        let yaml = synthesize_agent_manifest(&app).unwrap();
        // Round-trips back into a valid Agent.
        let agent: Agent = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(agent.agent, "welded-to-tc");
        assert_eq!(agent.version, "0.3.1");
        // start command → stateful agent.
        assert!(agent.stateful);
        let start = agent
            .commands
            .get("start")
            .expect("start command synthesized");
        assert_eq!(start.lifecycle, Lifecycle::Start);
        // No fictional cli/rest transport leaked in.
        assert!(agent.transport.cli.is_none());
        assert!(agent.transport.rest.is_none());
        // app transport names the backing app.
        let app_transport = agent.transport.app.as_ref().expect("app transport present");
        assert_eq!(app_transport.backed_by, "welded-to-tc");
        // The synthesized manifest passes agent validation.
        assert!(!crate::validate::has_errors(
            &crate::validate::validate_agent(&agent)
        ));
    }

    #[test]
    fn single_lifecycle_app_synthesizes_stateless_agent() {
        let app: App = serde_yaml::from_str(
            r#"
app: baker
version: 1.0.0
description: bakes a value
exposes-as-agent: true
exposed-commands:
  run:
    lifecycle: single
    inputs:
      phase:
        type: string
        description: which phase
    outputs:
      type: single
      schema:
        result: string
nodes:
  - id: n
    inline:
      kind: predicate
      description: pass
      code: 'true'
requires: []
"#,
        )
        .unwrap();
        let yaml = synthesize_agent_manifest(&app).unwrap();
        let agent: Agent = serde_yaml::from_str(&yaml).unwrap();
        assert!(!agent.stateful, "single-only app → stateless agent");
        assert_eq!(agent.commands.len(), 1);
    }

    #[test]
    fn refuses_to_synthesize_when_not_exposing() {
        let app: App = serde_yaml::from_str(
            "app: x\nversion: 1.0\ndescription: x\nnodes:\n  - id: n\nrequires: []\n",
        )
        .unwrap();
        assert!(synthesize_agent_manifest(&app).is_err());
    }

    fn exposed(yaml: &str) -> ExposedCommand {
        serde_yaml::from_str(yaml).unwrap()
    }

    #[test]
    fn input_type_mismatch_is_rejected() {
        let cmd = exposed(
            "lifecycle: single\ninputs:\n  count:\n    type: number\n  name:\n    type: string\n",
        );
        // number declared, string provided → error.
        let err =
            validate_exposed_inputs("run", &cmd, &serde_json::json!({ "count": "not-a-number" }))
                .unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }

    #[test]
    fn matching_and_unknown_and_missing_inputs_pass() {
        let cmd = exposed(
            "lifecycle: single\ninputs:\n  count:\n    type: number\n  flavour:\n    type: enum\n  name:\n    type: string\n",
        );
        // count matches; flavour is an unknown declared type (permissive);
        // name is not provided (left to the nested app). Extra `x` passes through.
        validate_exposed_inputs(
            "run",
            &cmd,
            &serde_json::json!({ "count": 5, "flavour": "vanilla", "x": true }),
        )
        .unwrap();
    }

    #[test]
    fn no_declared_inputs_accepts_anything() {
        let cmd = exposed("lifecycle: single\n");
        validate_exposed_inputs("run", &cmd, &serde_json::json!({ "anything": [1, 2] })).unwrap();
    }
}
