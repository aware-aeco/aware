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
    /// The backing app's declared permission union (`requires-permissions:`),
    /// carried through so a caller's `aware app explain` surfaces the hosts /
    /// software / secrets a node invoking the exposed app inherits (app-spec
    /// § exposes-as-agent constraints: "the caller approves the full union").
    #[serde(skip_serializing_if = "Option::is_none")]
    requires: Option<Value>,
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
        requires: app.requires_permissions.clone(),
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

/// Coerce + type-validate a caller's routed inputs against an exposed command's
/// declared input types, in place, before they enter the nested app's `inputs:`.
///
/// Templating stringifies whole-`{{ … }}` config leaves, so a routed numeric /
/// boolean / structured value arrives here as a string (e.g. `count: "5"`,
/// `rows: "[…]"`). For each declared input present, this parses such a string
/// back into the declared type so the nested app receives a correctly-typed
/// value — and rejects values that cannot satisfy the declared type
/// (`expected integer, got 1.5`). An unknown declared type (e.g. `enum`) or an
/// input the caller did not supply is left alone; inputs beyond the declared
/// set pass through unchecked.
pub fn validate_exposed_inputs(
    command: &str,
    exposed: &ExposedCommand,
    args: &mut serde_json::Value,
) -> Result<(), AwareError> {
    let Some(declared) = exposed.inputs.as_mapping() else {
        return Ok(());
    };
    // Collect (name, declared_type) first so we can mutate `args` afterwards
    // without holding a borrow of `exposed.inputs`.
    let declared_types: Vec<(String, String)> = declared
        .iter()
        .filter_map(|(name, decl)| {
            let name = name.as_str()?;
            let ty = decl.get("type").and_then(|t| t.as_str())?;
            Some((name.to_string(), ty.to_string()))
        })
        .collect();
    let Some(provided) = args.as_object_mut() else {
        return Ok(());
    };
    for (name, declared_type) in declared_types {
        let Some(value) = provided.get(&name) else {
            continue;
        };
        match coerce_to_declared(value, &declared_type) {
            Some(coerced) => {
                provided.insert(name, coerced);
            }
            None => {
                return Err(AwareError::Validation(format!(
                    "exposed command `{command}` input `{name}`: expected {declared_type}, got {}",
                    json_type_name(value)
                )));
            }
        }
    }
    Ok(())
}

/// Coerce a routed value into a declared input `type`, returning the value to
/// store (possibly unchanged) or `None` if it cannot satisfy the type. Strings
/// produced by templating are parsed into numbers / integers / booleans /
/// objects / arrays as needed; `integer` rejects fractional values. Unknown
/// declared types are permissive (accepted as-is).
fn coerce_to_declared(value: &serde_json::Value, declared: &str) -> Option<serde_json::Value> {
    use serde_json::Value;
    match declared {
        "string" | "str" | "text" => match value {
            Value::String(_) => Some(value.clone()),
            // A scalar (e.g. a number routed into a string input) stringifies.
            Value::Number(_) | Value::Bool(_) => Some(Value::String(value.to_string())),
            _ => None,
        },
        "number" | "num" | "float" | "double" => match value {
            Value::Number(_) => Some(value.clone()),
            Value::String(s) => parse_number(s),
            _ => None,
        },
        "integer" | "int" => match value {
            Value::Number(n) if n.is_i64() || n.is_u64() => Some(value.clone()),
            // Whole-valued floats (1.0) are integers; fractional (1.5) are not.
            Value::Number(n) => n
                .as_f64()
                .filter(|f| f.fract() == 0.0 && f.is_finite())
                .map(|f| Value::Number((f as i64).into())),
            Value::String(s) => s.parse::<i64>().ok().map(|i| Value::Number(i.into())),
            _ => None,
        },
        "boolean" | "bool" => match value {
            Value::Bool(_) => Some(value.clone()),
            Value::String(s) => s.parse::<bool>().ok().map(Value::Bool),
            _ => None,
        },
        "object" | "map" | "mapping" => match value {
            Value::Object(_) => Some(value.clone()),
            Value::String(s) => serde_json::from_str::<Value>(s)
                .ok()
                .filter(Value::is_object),
            _ => None,
        },
        "array" | "list" => match value {
            Value::Array(_) => Some(value.clone()),
            Value::String(s) => serde_json::from_str::<Value>(s)
                .ok()
                .filter(Value::is_array),
            _ => None,
        },
        // Unknown declared type (e.g. `enum`): accept as-is.
        _ => Some(value.clone()),
    }
}

/// Parse a string into a JSON number, preserving integer-ness where possible.
fn parse_number(s: &str) -> Option<serde_json::Value> {
    if let Ok(i) = s.parse::<i64>() {
        return Some(serde_json::Value::Number(i.into()));
    }
    if let Ok(u) = s.parse::<u64>() {
        return Some(serde_json::Value::Number(u.into()));
    }
    s.parse::<f64>()
        .ok()
        .and_then(serde_json::Number::from_f64)
        .map(serde_json::Value::Number)
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
        // The backing app's declared permission union is carried onto the
        // synthesized agent so a caller's `aware app explain` surfaces it.
        let requires = agent.requires.as_ref().expect("requires carried through");
        assert!(
            requires.network.iter().any(|h| h.contains("trimble")),
            "network host should be inherited: {:?}",
            requires.network
        );
        assert!(
            requires.software.iter().any(|s| s.contains("tekla")),
            "software requirement should be inherited: {:?}",
            requires.software
        );
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
        // number declared, non-numeric string provided → error.
        let mut args = serde_json::json!({ "count": "not-a-number" });
        let err = validate_exposed_inputs("run", &cmd, &mut args).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }

    #[test]
    fn matching_and_unknown_and_missing_inputs_pass() {
        let cmd = exposed(
            "lifecycle: single\ninputs:\n  count:\n    type: number\n  flavour:\n    type: enum\n  name:\n    type: string\n",
        );
        // count matches; flavour is an unknown declared type (permissive);
        // name is not provided (left to the nested app). Extra `x` passes through.
        let mut args = serde_json::json!({ "count": 5, "flavour": "vanilla", "x": true });
        validate_exposed_inputs("run", &cmd, &mut args).unwrap();
    }

    #[test]
    fn no_declared_inputs_accepts_anything() {
        let cmd = exposed("lifecycle: single\n");
        let mut args = serde_json::json!({ "anything": [1, 2] });
        validate_exposed_inputs("run", &cmd, &mut args).unwrap();
    }

    #[test]
    fn stringified_routed_values_are_coerced_to_declared_types() {
        // Templating renders whole-`{{ … }}` leaves to strings; coercion must
        // restore the declared types so the nested app receives typed values.
        let cmd = exposed(
            "lifecycle: single\ninputs:\n  count:\n    type: number\n  flag:\n    type: boolean\n  \
             rows:\n    type: array\n  meta:\n    type: object\n",
        );
        let mut args = serde_json::json!({
            "count": "5",
            "flag": "true",
            "rows": "[1, 2, 3]",
            "meta": "{\"a\": 1}",
        });
        validate_exposed_inputs("run", &cmd, &mut args).unwrap();
        assert_eq!(args["count"], serde_json::json!(5));
        assert_eq!(args["flag"], serde_json::json!(true));
        assert_eq!(args["rows"], serde_json::json!([1, 2, 3]));
        assert_eq!(args["meta"], serde_json::json!({ "a": 1 }));
    }

    #[test]
    fn integer_rejects_fractional_values() {
        let cmd = exposed("lifecycle: single\ninputs:\n  n:\n    type: integer\n");
        // 1.5 is not an integer.
        let mut frac = serde_json::json!({ "n": 1.5 });
        assert!(validate_exposed_inputs("run", &cmd, &mut frac).is_err());
        // A stringified whole number coerces to an integer.
        let mut whole = serde_json::json!({ "n": "5" });
        validate_exposed_inputs("run", &cmd, &mut whole).unwrap();
        assert_eq!(whole["n"], serde_json::json!(5));
        // A whole-valued float is accepted as the integer.
        let mut floaty = serde_json::json!({ "n": 4.0 });
        validate_exposed_inputs("run", &cmd, &mut floaty).unwrap();
        assert_eq!(floaty["n"], serde_json::json!(4));
    }
}
