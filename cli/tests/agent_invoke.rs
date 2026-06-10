//! E2E coverage for `aware agent invoke` (#215): install the builtin `ui`
//! agent from the local checkout, then drive `catalog` / `validate` / `render`
//! through the real binary — plus the builtin-only refusal for a cli-transport
//! agent. Each test gets its own temp AWARE_HOME so nothing touches `~/.aware`.

use assert_cmd::Command;
use predicates::prelude::*;

/// Path to an agent folder inside this checkout.
fn agent_src(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(rel)
}

/// Fresh temp AWARE_HOME with the `ui` agent installed from the local folder.
fn home_with_ui() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["agent", "install"])
        .arg(agent_src("20-agents/_core/ui"))
        .assert()
        .success()
        .stdout(predicate::str::contains("installed ui"));
    tmp
}

const GOOD_DESCRIPTOR_ARGS: &str = r#"{
  "descriptor": {
    "version": 1,
    "panels": [{
      "id": "overview",
      "slot": "dashboard",
      "title": "Overview",
      "blocks": [
        { "type": "stat", "label": "Parts", "value": 128 },
        { "type": "table", "source": "parts" }
      ]
    }]
  }
}"#;

const BAD_DESCRIPTOR_ARGS: &str = r#"{
  "descriptor": {
    "version": 1,
    "panels": [{
      "slot": "dashboard",
      "blocks": [{ "type": "stat", "value": 1 }]
    }]
  }
}"#;

#[test]
fn invoke_ui_catalog_lists_block_contracts() {
    let home = home_with_ui();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .args(["agent", "invoke", "ui", "catalog"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"schema-version\": \"1\""))
        .stdout(predicate::str::contains("\"name\": \"stat\""))
        .stdout(predicate::str::contains("\"name\": \"table\""))
        .stdout(predicate::str::contains("\"name\": \"text\""))
        .stdout(predicate::str::contains("\"name\": \"report\""))
        .stdout(predicate::str::contains("\"name\": \"action\""));
}

#[test]
fn invoke_ui_validate_good_descriptor_is_valid() {
    let home = home_with_ui();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .args([
            "agent",
            "invoke",
            "ui",
            "validate",
            "--inputs",
            GOOD_DESCRIPTOR_ARGS,
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"valid\": true"));
}

#[test]
fn invoke_ui_validate_bad_descriptor_reports_paths() {
    let home = home_with_ui();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .args([
            "agent",
            "invoke",
            "ui",
            "validate",
            "--inputs",
            BAD_DESCRIPTOR_ARGS,
        ])
        .assert()
        .success() // the INVOCATION succeeds; the verdict is in the data
        .stdout(predicate::str::contains("\"valid\": false"))
        .stdout(predicate::str::contains(
            "panels[0].id: missing required field",
        ))
        .stdout(predicate::str::contains("panels[0].blocks[0].label"));
}

#[test]
fn invoke_ui_render_writes_html_artifact() {
    let home = home_with_ui();
    let out_path = home.path().join("panel.html");
    let args = format!(
        r#"{{
          "descriptor": {{
            "version": 1,
            "panels": [{{
              "id": "p", "slot": "dashboard", "title": "T",
              "blocks": [
                {{ "type": "stat", "label": "N", "value": 7 }},
                {{ "type": "table", "source": "rows" }}
              ]
            }}]
          }},
          "data": {{ "rows": [{{ "a": 1, "b": 2 }}] }},
          "output-path": "{}"
        }}"#,
        out_path.display().to_string().replace('\\', "/")
    );
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .args(["agent", "invoke", "ui", "render", "--inputs", &args])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"bytes\""));
    let html = std::fs::read_to_string(&out_path).expect("render wrote the artifact");
    assert!(html.starts_with("<!DOCTYPE html>"));
    assert!(html.contains("data-panel-id=\"p\""));
    assert!(
        html.contains("<th>a</th><th>b</th>"),
        "table resolved from data"
    );
}

#[test]
fn invoke_inputs_from_file() {
    let home = home_with_ui();
    let f = home.path().join("args.json");
    std::fs::write(&f, GOOD_DESCRIPTOR_ARGS).unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .args(["agent", "invoke", "ui", "validate", "--inputs"])
        .arg(format!("@{}", f.display()))
        .assert()
        .success()
        .stdout(predicate::str::contains("\"valid\": true"));
}

#[test]
fn invoke_json_flag_emits_envelope() {
    let home = home_with_ui();
    let out = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .args(["agent", "invoke", "ui", "catalog", "--json"])
        .assert()
        .success();
    let stdout = String::from_utf8(out.get_output().stdout.clone()).unwrap();
    let v: serde_json::Value = serde_json::from_str(stdout.trim()).expect("one JSON envelope");
    assert_eq!(v["ok"], serde_json::json!(true));
    assert_eq!(v["meta"]["command"], serde_json::json!("agent invoke"));
    assert_eq!(v["data"]["schema-version"], serde_json::json!("1"));
}

#[test]
fn invoke_html_report_render_is_host_callable_too() {
    // Stage 0 retroactively makes the existing builtin host-callable (#215).
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["agent", "install"])
        .arg(agent_src("20-agents/_core/html-report"))
        .assert()
        .success();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args([
            "agent",
            "invoke",
            "html-report",
            "render",
            "--inputs",
            r#"{"title":"T","data":[{"a":1}]}"#,
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("<!DOCTYPE html>"));
}

#[test]
fn invoke_non_builtin_agent_is_refused_clearly() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["agent", "install"])
        .arg(agent_src("20-agents/aeco/engineering/tekla"))
        .assert()
        .success();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["agent", "invoke", "tekla", "exec"])
        .assert()
        .failure()
        .code(3) // Validation
        .stderr(predicate::str::contains("builtin-only"));
}

#[test]
fn invoke_unknown_agent_exits_not_found() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["agent", "invoke", "no-such-agent", "go"])
        .assert()
        .failure()
        .code(7) // NotFound
        .stderr(predicate::str::contains("not installed"));
}

#[test]
fn invoke_unknown_command_lists_available() {
    let home = home_with_ui();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .args(["agent", "invoke", "ui", "nope"])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("available:"))
        .stderr(predicate::str::contains("validate"));
}

#[test]
fn invoke_rejects_non_object_inputs() {
    let home = home_with_ui();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .args(["agent", "invoke", "ui", "validate", "--inputs", "[1,2]"])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("must be a JSON object"));
}
