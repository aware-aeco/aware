//! Python module → AWARE agent.
//!
//! Spawns `python -c "<introspection script>"` and parses the JSON output.

use std::collections::BTreeMap;

use crate::builder::{GeneratedAgent, GeneratedCommand, Provenance, now_iso};
use crate::error::AwareError;

pub fn build_from_python(
    module: &str,
    agent_id: Option<&str>,
) -> Result<GeneratedAgent, AwareError> {
    // Use a semicolon-separated one-liner so indentation is not an issue
    // when the script is passed via -c on different platforms.
    let script = format!(
        "import json,importlib,inspect; \
         m=importlib.import_module('{module}'); \
         out=[(n,(inspect.getdoc(getattr(m,n)) or '').split('\\n')[0]) \
         for n in dir(m) if not n.startswith('_') and callable(getattr(m,n))]; \
         print(json.dumps(out))"
    );
    let output = std::process::Command::new("python")
        .arg("-c")
        .arg(&script)
        .output()
        .map_err(|e| AwareError::Network(format!("spawn python: {e}")))?;
    if !output.status.success() {
        return Err(AwareError::Network(format!(
            "python introspect failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    let entries: Vec<(String, String)> = serde_json::from_slice(&output.stdout)
        .map_err(|e| AwareError::Validation(format!("python introspect output: {e}")))?;

    let mut commands = BTreeMap::new();
    for (name, doc) in entries {
        let kebab_name = name.replace('_', "-");
        commands.insert(
            kebab_name,
            GeneratedCommand {
                lifecycle: "single".into(),
                description: if doc.is_empty() {
                    format!("{module}.{name}()")
                } else {
                    doc
                },
                inputs_yaml: String::new(),
                outputs_yaml: String::new(),
                ..Default::default()
            },
        );
    }

    let id = agent_id
        .map(String::from)
        .unwrap_or_else(|| module.replace('.', "-"));
    let provenance = Provenance {
        generated_by: "aware-agent-builder".into(),
        generator_version: env!("CARGO_PKG_VERSION").into(),
        source: serde_json::json!({ "type": "python", "module": module }),
        generated_at: now_iso(),
    };

    Ok(GeneratedAgent {
        id,
        version: "0.1.0".into(),
        sdk_target: None,
        description: format!("Python module wrapper: {module}"),
        commands,
        skills: Vec::new(),
        provenance,
        stateful: false,
        license: "UNKNOWN".into(),
        rest: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Skip the test if `python` isn't on PATH (common in CI).
    fn python_available() -> bool {
        std::process::Command::new("python")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    #[test]
    fn introspects_json_stdlib_module_when_python_present() {
        if !python_available() {
            eprintln!("python not on PATH; skipping");
            return;
        }
        let agent = build_from_python("json", None).unwrap();
        assert_eq!(agent.id, "json");
        // json stdlib exposes dump, dumps, load, loads as callables
        assert!(agent.commands.contains_key("dumps"));
        assert!(agent.commands.contains_key("loads"));
    }

    #[test]
    fn a_module_python_cannot_import_is_reported_as_a_network_error() {
        // The introspect-failed arm: `python` runs fine but exits non-zero.
        // This is the failure users actually hit (a typo'd or uninstalled
        // module), and it must surface as Network — `aware build` maps that
        // to its own exit code, so a change to Validation here would be a
        // silent behaviour change at the CLI boundary.
        if !python_available() {
            eprintln!("python not on PATH; skipping");
            return;
        }
        let err = build_from_python("definitely_not_a_module_xyz_123", None)
            .expect_err("importing a nonexistent module must fail");
        assert!(
            matches!(&err, AwareError::Network(m) if m.contains("python introspect failed")),
            "wrong error for an unimportable module: {err:?}"
        );
    }

    // The *spawn*-failure arm of `build_from_python` (the `.output()` error,
    // also `AwareError::Network`) stays uncovered: the interpreter name is
    // hardcoded to `python` inside the function, and Rust 2024 makes `set_var`
    // on PATH unsafe and process-global. Reaching it needs an injectable
    // interpreter on the production signature — a change for a PR that wants
    // it, not a placeholder test that runs no code.
}
