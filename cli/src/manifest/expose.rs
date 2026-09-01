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
                lifecycle: cmd.lifecycle.as_str(),
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
                    crate::json::type_name(value)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::Agent;

    fn welded_to_tc() -> App {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("30-apps/_examples/welded-to-tc.app");
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

    /// The synthesized `mode:` is what decides whether a caller must wrap the
    /// node in a `safety:` block, so both halves of `exposed_mode` are
    /// load-bearing: an omitted `mode:` must land as `read` (an un-annotated
    /// caller stays legal) and an authored `mode: write` must survive
    /// synthesis (a caller is forced to declare the safety contract). Nothing
    /// covered either half — `mode` appeared in no test in the tree.
    #[test]
    fn an_omitted_mode_synthesizes_read_and_an_authored_write_survives() {
        let app: App = serde_yaml::from_str(
            r#"
app: gatekeeper
version: 1.0.0
description: exposes one read and one write command
exposes-as-agent: true
exposed-commands:
  peek:
    lifecycle: single
  push:
    lifecycle: single
    mode: write
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
        let agent: Agent = serde_yaml::from_str(&synthesize_agent_manifest(&app).unwrap()).unwrap();
        assert_eq!(
            agent.commands["peek"].mode,
            Some(Mode::Read),
            "an exposed command with no mode: must present as read"
        );
        assert_eq!(
            agent.commands["push"].mode,
            Some(Mode::Write),
            "an authored mode: write must reach the caller, or the safety gate is lost"
        );
    }

    /// `stateful` is `any`, not `all`: one `lifecycle: start` command makes the
    /// whole synthesized agent stateful, however many single-shot commands sit
    /// beside it. The existing tests pin only the all-start and all-single
    /// apps, which an `any` → `all` slip survives.
    #[test]
    fn one_start_command_among_singles_still_synthesizes_a_stateful_agent() {
        let app: App = serde_yaml::from_str(
            r#"
app: mixed
version: 2.0.0
description: mostly single, one long-running
exposes-as-agent: true
exposed-commands:
  compute:
    lifecycle: single
  serve:
    lifecycle: start
  halt:
    lifecycle: stop
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
        let agent: Agent = serde_yaml::from_str(&synthesize_agent_manifest(&app).unwrap()).unwrap();
        assert!(
            agent.stateful,
            "a single `start` among singles must still make the agent stateful"
        );
        assert_eq!(agent.commands.len(), 3);
        assert_eq!(agent.commands["serve"].lifecycle, Lifecycle::Start);
        assert_eq!(agent.commands["halt"].lifecycle, Lifecycle::Stop);
    }

    /// An agent `description:` is a one-line field; an app's is free prose. The
    /// synthesizer takes the first line and trims it, so a multi-line app
    /// description cannot smuggle a newline into the agent manifest.
    #[test]
    fn the_synthesized_description_is_the_first_line_trimmed() {
        let app: App = serde_yaml::from_str(
            "app: wordy\nversion: 1.0.0\ndescription: \"  headline sentence  \\nbody paragraph that must not travel\"\n\
             exposes-as-agent: true\nexposed-commands:\n  run:\n    lifecycle: single\n\
             nodes:\n  - id: n\n    inline:\n      kind: predicate\n      description: pass\n      code: 'true'\nrequires: []\n",
        )
        .unwrap();
        let agent: Agent = serde_yaml::from_str(&synthesize_agent_manifest(&app).unwrap()).unwrap();
        assert_eq!(agent.description, "headline sentence");
        assert!(
            !agent.description.contains("body paragraph"),
            "only the first line belongs on the agent: {:?}",
            agent.description
        );
    }

    /// When an exposed command names no `description:`, the synthesizer writes a
    /// generated one — the field is required on an agent command, so an empty
    /// or missing description would make the manifest unparseable/invalid.
    #[test]
    fn a_command_without_a_description_gets_a_generated_one_naming_it() {
        let app: App = serde_yaml::from_str(
            "app: terse\nversion: 1.0.0\ndescription: no command descriptions\nexposes-as-agent: true\n\
             exposed-commands:\n  run:\n    lifecycle: single\n  spelt:\n    lifecycle: single\n    description: authored text\n\
             nodes:\n  - id: n\n    inline:\n      kind: predicate\n      description: pass\n      code: 'true'\nrequires: []\n",
        )
        .unwrap();
        let agent: Agent = serde_yaml::from_str(&synthesize_agent_manifest(&app).unwrap()).unwrap();
        let generated = &agent.commands["run"].description;
        assert!(
            generated.contains("run") && generated.contains("terse"),
            "the generated description should name the command and its app: {generated:?}"
        );
        assert_eq!(
            agent.commands["spelt"].description, "authored text",
            "an authored description must not be replaced by the generated one"
        );
    }

    /// Every spelling in `coerce_to_declared`'s alias table has to keep working:
    /// an alias that falls out of its match arm becomes an *unknown* type, which
    /// is permissive — the routed value would then pass through uncoerced and
    /// reach the nested app with the wrong JSON type instead of erroring. Each
    /// case below therefore feeds a value the coercion must *change*, so a
    /// dropped alias shows up as an unchanged value rather than as an error.
    #[test]
    fn every_declared_type_alias_still_coerces_its_routed_value() {
        let cmd = exposed(
            "lifecycle: single\ninputs:\n\
             \x20 as_string: { type: string }\n\x20 as_str: { type: str }\n\x20 as_text: { type: text }\n\
             \x20 as_number: { type: number }\n\x20 as_num: { type: num }\n\x20 as_float: { type: float }\n\x20 as_double: { type: double }\n\
             \x20 as_integer: { type: integer }\n\x20 as_int: { type: int }\n\
             \x20 as_boolean: { type: boolean }\n\x20 as_bool: { type: bool }\n\
             \x20 as_object: { type: object }\n\x20 as_map: { type: map }\n\x20 as_mapping: { type: mapping }\n\
             \x20 as_array: { type: array }\n\x20 as_list: { type: list }\n",
        );
        let mut args = serde_json::json!({
            // A scalar routed into a string input stringifies.
            "as_string": 7, "as_str": 7, "as_text": 7,
            // Templating stringified these; they must come back as numbers.
            "as_number": "2.5", "as_num": "2.5", "as_float": "2.5", "as_double": "2.5",
            "as_integer": "3", "as_int": "3",
            "as_boolean": "true", "as_bool": "true",
            "as_object": "{\"k\": 1}", "as_map": "{\"k\": 1}", "as_mapping": "{\"k\": 1}",
            "as_array": "[1]", "as_list": "[1]",
        });
        validate_exposed_inputs("run", &cmd, &mut args).unwrap();
        for key in ["as_string", "as_str", "as_text"] {
            assert_eq!(args[key], serde_json::json!("7"), "{key} should stringify");
        }
        for key in ["as_number", "as_num", "as_float", "as_double"] {
            assert_eq!(args[key], serde_json::json!(2.5), "{key} should parse");
        }
        for key in ["as_integer", "as_int"] {
            assert_eq!(args[key], serde_json::json!(3), "{key} should parse");
        }
        for key in ["as_boolean", "as_bool"] {
            assert_eq!(args[key], serde_json::json!(true), "{key} should parse");
        }
        for key in ["as_object", "as_map", "as_mapping"] {
            assert_eq!(
                args[key],
                serde_json::json!({ "k": 1 }),
                "{key} should parse"
            );
        }
        for key in ["as_array", "as_list"] {
            assert_eq!(args[key], serde_json::json!([1]), "{key} should parse");
        }
    }

    /// A string input takes scalars, but a container has no sensible string
    /// form — accepting one would hand the nested app the debug spelling of a
    /// structure. Null likewise: it is an absent value, not the empty string.
    #[test]
    fn a_container_or_null_routed_into_a_string_input_is_refused() {
        let cmd = exposed("lifecycle: single\ninputs:\n  s:\n    type: string\n");
        for bad in [
            serde_json::json!([1, 2]),
            serde_json::json!({ "a": 1 }),
            serde_json::Value::Null,
        ] {
            let mut args = serde_json::json!({ "s": bad });
            let err = validate_exposed_inputs("run", &cmd, &mut args).unwrap_err();
            assert!(
                matches!(err, AwareError::Validation(ref m) if m.contains("expected string")),
                "expected a string-type rejection, got {err:?}"
            );
        }
        // A bool is a scalar and does stringify.
        let mut ok = serde_json::json!({ "s": true });
        validate_exposed_inputs("run", &cmd, &mut ok).unwrap();
        assert_eq!(ok["s"], serde_json::json!("true"));
    }

    /// A stringified container must parse into the *declared* container kind.
    /// Both arms parse arbitrary JSON, so without the `is_object` / `is_array`
    /// filter an array string would satisfy an `object` input (and vice versa)
    /// and the nested app would receive a shape its schema does not describe.
    #[test]
    fn a_stringified_container_must_match_the_declared_kind() {
        let obj = exposed("lifecycle: single\ninputs:\n  v:\n    type: object\n");
        let arr = exposed("lifecycle: single\ninputs:\n  v:\n    type: array\n");
        // Right JSON, wrong kind.
        let mut arr_into_obj = serde_json::json!({ "v": "[1, 2]" });
        assert!(validate_exposed_inputs("run", &obj, &mut arr_into_obj).is_err());
        let mut obj_into_arr = serde_json::json!({ "v": "{\"a\": 1}" });
        assert!(validate_exposed_inputs("run", &arr, &mut obj_into_arr).is_err());
        // A bare scalar in a string is JSON too, and is neither.
        let mut scalar_into_obj = serde_json::json!({ "v": "42" });
        assert!(validate_exposed_inputs("run", &obj, &mut scalar_into_obj).is_err());
        // Not JSON at all.
        let mut junk = serde_json::json!({ "v": "{oops" });
        assert!(validate_exposed_inputs("run", &obj, &mut junk).is_err());
    }

    /// String→bool and string→integer parsing is deliberately strict: only the
    /// exact JSON/Rust spellings convert. A near-miss (`"True"`, `"1"`,
    /// `"5.0"`) must error rather than be guessed at, because guessing wrong at
    /// an app boundary silently changes what the nested app runs.
    #[test]
    fn strings_convert_to_booleans_and_integers_only_on_an_exact_spelling() {
        let b = exposed("lifecycle: single\ninputs:\n  v:\n    type: boolean\n");
        for good in ["true", "false"] {
            let mut args = serde_json::json!({ "v": good });
            validate_exposed_inputs("run", &b, &mut args).unwrap();
            assert_eq!(args["v"], serde_json::json!(good == "true"));
        }
        for near_miss in ["True", "TRUE", "1", "yes", "on", ""] {
            let mut args = serde_json::json!({ "v": near_miss });
            assert!(
                validate_exposed_inputs("run", &b, &mut args).is_err(),
                "{near_miss:?} must not be read as a boolean"
            );
        }
        // A number is not a boolean either.
        let mut numeric = serde_json::json!({ "v": 1 });
        assert!(validate_exposed_inputs("run", &b, &mut numeric).is_err());

        let i = exposed("lifecycle: single\ninputs:\n  v:\n    type: integer\n");
        let mut negative = serde_json::json!({ "v": "-5" });
        validate_exposed_inputs("run", &i, &mut negative).unwrap();
        assert_eq!(negative["v"], serde_json::json!(-5));
        // The float 5.0 is accepted (see `integer_rejects_fractional_values`)
        // but the *string* "5.0" is not — `str::parse::<i64>` has no float path.
        for near_miss in ["5.0", "5 ", " 5", "+5.0", "0x10", ""] {
            let mut args = serde_json::json!({ "v": near_miss });
            assert!(
                validate_exposed_inputs("run", &i, &mut args).is_err(),
                "{near_miss:?} must not be read as an integer"
            );
        }
    }

    /// `parse_number` tries i64, then u64, then f64, so a value above
    /// `i64::MAX` stays an exact integer instead of degrading to a lossy float.
    /// The f64 arm goes through `Number::from_f64`, which rejects the
    /// non-finite results `str::parse::<f64>` happily produces — an infinity or
    /// a NaN reaching a nested app's arithmetic is worse than a refusal.
    #[test]
    fn number_parsing_keeps_big_integers_exact_and_refuses_non_finite() {
        let cmd = exposed("lifecycle: single\ninputs:\n  v:\n    type: number\n");

        let mut big = serde_json::json!({ "v": "9223372036854775808" }); // i64::MAX + 1
        validate_exposed_inputs("run", &cmd, &mut big).unwrap();
        assert!(
            big["v"].is_u64() && !big["v"].is_f64(),
            "a value past i64::MAX must stay an exact integer, got {:?}",
            big["v"]
        );
        assert_eq!(big["v"].as_u64(), Some(9_223_372_036_854_775_808));

        let mut small = serde_json::json!({ "v": "-9223372036854775808" }); // i64::MIN
        validate_exposed_inputs("run", &cmd, &mut small).unwrap();
        assert_eq!(small["v"].as_i64(), Some(i64::MIN));

        // A plain integer string stays integral rather than becoming 7.0.
        let mut whole = serde_json::json!({ "v": "7" });
        validate_exposed_inputs("run", &cmd, &mut whole).unwrap();
        assert!(whole["v"].is_i64(), "got {:?}", whole["v"]);

        let mut exponent = serde_json::json!({ "v": "1e3" });
        validate_exposed_inputs("run", &cmd, &mut exponent).unwrap();
        assert_eq!(exponent["v"].as_f64(), Some(1000.0));

        for non_finite in ["inf", "-inf", "NaN", "infinity", "1e400"] {
            let mut args = serde_json::json!({ "v": non_finite });
            assert!(
                validate_exposed_inputs("run", &cmd, &mut args).is_err(),
                "{non_finite:?} must not reach the nested app as a number"
            );
        }
    }

    /// Two documented escape hatches, neither of which had a test. Routed args
    /// that are not a JSON object cannot be indexed by input name, and an input
    /// declared without a `type:` states no type to check — both must be handed
    /// on untouched rather than turned into a validation failure.
    #[test]
    fn untypable_declarations_and_non_object_args_pass_through_untouched() {
        let cmd = exposed("lifecycle: single\ninputs:\n  v:\n    type: integer\n");
        let mut not_a_map = serde_json::json!([1, 2, 3]);
        validate_exposed_inputs("run", &cmd, &mut not_a_map).unwrap();
        assert_eq!(not_a_map, serde_json::json!([1, 2, 3]));

        // Declared, but with no `type:` — nothing to enforce, so anything goes
        // and the value is left exactly as routed.
        let untyped = exposed("lifecycle: single\ninputs:\n  blob:\n    description: freeform\n");
        let mut args = serde_json::json!({ "blob": [1, 2] });
        validate_exposed_inputs("run", &untyped, &mut args).unwrap();
        assert_eq!(args["blob"], serde_json::json!([1, 2]));

        // An input the caller did not supply is not invented, and an input
        // beyond the declared set is neither checked nor rewritten.
        let mut partial = serde_json::json!({ "extra": "9" });
        validate_exposed_inputs("run", &cmd, &mut partial).unwrap();
        assert_eq!(partial, serde_json::json!({ "extra": "9" }));
        assert!(!partial.as_object().unwrap().contains_key("v"));
    }
}
