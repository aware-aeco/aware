//! `aware app rename-node` and the structured `E_APP_AMBIGUOUS_NODE_ID` (aware#701),
//! driven through the real binary.

mod common;

use assert_cmd::Command;
use serde_json::Value;

const COLLIDING: &str = "# keep this comment
app: amb
version: 0.0.1
description: x
requires: []
nodes:
  - id: outer
    for-each: '{{ inputs.items }}'
    do:
      - id: think
        inline: { kind: predicate, description: p, code: 'true' }
      - id: use
        inline: { kind: predicate, description: p, code: 'true' }
        inputs:
          v: '{{ think.x }}'
  - id: outer.think
    inline: { kind: predicate, description: p, code: 'true' }
  - id: after
    inline: { kind: predicate, description: p, code: 'true' }
    inputs:
      v: '{{ upstream[\"outer.think\"].y }}'
connections:
  - from: outer.think
    to: after
";

fn aware(home: &std::path::Path) -> Command {
    let mut c = Command::cargo_bin("aware").unwrap();
    c.env("AWARE_HOME", home);
    c
}

fn run_json(home: &std::path::Path, args: &[&str]) -> (Value, i32) {
    let out = aware(home).args(args).arg("--json").output().unwrap();
    let text = String::from_utf8(out.stdout).unwrap();
    let v: Value = serde_json::from_str(text.trim())
        .unwrap_or_else(|e| panic!("stdout is not one JSON envelope ({e}): {text}"));
    (v, out.status.code().unwrap_or(-1))
}

fn setup(src: &str) -> (tempfile::TempDir, std::path::PathBuf, std::path::PathBuf) {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("aware");
    std::fs::create_dir_all(&home).unwrap();
    let file = tmp.path().join("amb.flo");
    std::fs::write(&file, src).unwrap();
    (tmp, home, file)
}

#[test]
fn validate_and_compile_json_carry_the_structured_issue() {
    let (_t, home, file) = setup(COLLIDING);
    let f = file.to_str().unwrap();
    for verb in ["validate", "compile"] {
        let (v, code) = run_json(&home, &["app", verb, f]);
        assert_eq!(code, 3, "{verb}");
        assert_eq!(v["ok"], false, "{verb}");
        let issues = v["error"]["details"]["issues"].as_array().unwrap();
        let issue = issues
            .iter()
            .find(|i| i["code"] == "E_APP_AMBIGUOUS_NODE_ID")
            .unwrap_or_else(|| panic!("{verb}: {v}"));
        let d = &issue["details"];
        assert_eq!(d["scoped-id"], "outer.think");
        let paths: Vec<&str> = d["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|n| n["path"].as_str().unwrap())
            .collect();
        assert_eq!(paths, ["outer/think", "outer.think"]);
        assert_eq!(d["nodes"][0]["location"], "nested");
        assert_eq!(d["nodes"][0]["parent"], "outer");
        assert_eq!(d["nodes"][1]["location"], "top-level");
        assert_eq!(d["suggested-id"], "outer-think");
        let default = d["repairs"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["default"] == true)
            .unwrap();
        assert_eq!(default["node-path"], "outer.think");
        assert!(
            issue["message"]
                .as_str()
                .unwrap()
                .contains("aware app rename-node"),
            "the human message names the repair command"
        );
    }
    assert!(!file.with_file_name("amb.lock").exists(), "no lock written");
}

#[test]
fn dry_run_changes_nothing_then_rename_then_compile_has_distinct_ids() {
    let (_t, home, file) = setup(COLLIDING);
    let f = file.to_str().unwrap();

    let (v, code) = run_json(
        &home,
        &[
            "app",
            "rename-node",
            f,
            "outer.think",
            "outer-think",
            "--dry-run",
        ],
    );
    assert_eq!(code, 0, "{v}");
    assert_eq!(v["data"]["dry-run"], true);
    assert_eq!(v["data"]["applied"], false);
    let kinds: Vec<&str> = v["data"]["edits"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| e["kind"].as_str().unwrap())
        .collect();
    assert_eq!(kinds, ["node-id", "template", "connection"]);
    assert_eq!(
        std::fs::read_to_string(&file).unwrap(),
        COLLIDING,
        "dry-run wrote"
    );

    let (v, code) = run_json(
        &home,
        &["app", "rename-node", f, "outer.think", "outer-think"],
    );
    assert_eq!(code, 0, "{v}");
    assert_eq!(v["data"]["lock"]["touched"], false);
    let after = std::fs::read_to_string(&file).unwrap();
    assert!(
        after.starts_with("# keep this comment\n"),
        "comment preserved"
    );
    assert!(after.contains("- id: outer-think"));
    assert!(after.contains("upstream[\"outer-think\"].y"));
    assert!(after.contains("- from: outer-think"));
    // The nested `think` and its `{{ think.x }}` reference are not the renamed node.
    assert!(after.contains("- id: think\n"));
    assert!(after.contains("'{{ think.x }}'"));

    let (v, code) = run_json(&home, &["app", "validate", f]);
    assert_eq!(code, 0, "{v}");
    let (v, code) = run_json(&home, &["app", "compile", f]);
    assert_eq!(code, 0, "{v}");
    let lock = std::fs::read_to_string(file.with_file_name("amb.lock")).unwrap();
    let ids: Vec<&str> = lock
        .lines()
        .filter_map(|l| l.strip_prefix("- id: "))
        .collect();
    let mut sorted = ids.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(ids.len(), sorted.len(), "lock ids are distinct: {ids:?}");
    assert!(ids.contains(&"outer.think") && ids.contains(&"outer-think"));
}

#[test]
fn rename_never_touches_an_existing_lock() {
    let (_t, home, file) = setup(
        "app: amb\nversion: 0.0.1\ndescription: x\nrequires: []\nnodes:\n  - id: a\n    inline: { kind: predicate, description: p, code: 'true' }\n",
    );
    let f = file.to_str().unwrap();
    let (_, code) = run_json(&home, &["app", "compile", f]);
    assert_eq!(code, 0);
    let lock_path = file.with_file_name("amb.lock");
    let lock_before = std::fs::read(&lock_path).unwrap();
    let (v, code) = run_json(&home, &["app", "rename-node", f, "a", "first"]);
    assert_eq!(code, 0, "{v}");
    assert_eq!(v["data"]["lock"]["present"], true);
    assert_eq!(std::fs::read(&lock_path).unwrap(), lock_before);
    assert!(
        v["data"]["note"]
            .as_str()
            .unwrap()
            .contains("approved again")
    );
}

#[test]
fn a_new_id_that_collides_or_is_invalid_is_refused_with_a_suggestion() {
    let (_t, home, file) = setup(COLLIDING);
    let f = file.to_str().unwrap();
    for bad in ["after", "has.dot", "item", "1abc", "outer"] {
        let (v, code) = run_json(&home, &["app", "rename-node", f, "outer.think", bad]);
        assert_eq!(code, 3, "{bad}: {v}");
        assert_eq!(v["error"]["code"], "E_APP_RENAME_NODE_ID_CONFLICT", "{bad}");
        assert_eq!(
            v["error"]["details"]["suggested-id"], "outer-think",
            "{bad}"
        );
    }
    let (v, code) = run_json(&home, &["app", "rename-node", f, "nope", "x1"]);
    assert_eq!(code, 3);
    assert_eq!(v["error"]["code"], "E_APP_RENAME_NODE_NOT_FOUND");
    assert_eq!(
        std::fs::read_to_string(&file).unwrap(),
        COLLIDING,
        "nothing written"
    );
}

#[test]
fn flow_style_nodes_ask_for_reformat_instead_of_guessing() {
    let (_t, home, file) = setup(
        "app: amb\nversion: 0.0.1\ndescription: x\nrequires: []\nnodes: [{id: a, inline: {kind: predicate, description: p, code: 'true'}}, {id: b, inline: {kind: predicate, description: p, code: 'true'}}]\n",
    );
    let f = file.to_str().unwrap();
    let original = std::fs::read_to_string(&file).unwrap();
    let (v, code) = run_json(&home, &["app", "rename-node", f, "a", "first"]);
    assert_eq!(code, 3, "{v}");
    assert_eq!(v["error"]["code"], "E_APP_RENAME_NODE_NEEDS_REFORMAT");
    assert_eq!(std::fs::read_to_string(&file).unwrap(), original);
    let (v, code) = run_json(
        &home,
        &["app", "rename-node", f, "a", "first", "--reformat"],
    );
    assert_eq!(code, 0, "{v}");
    assert_eq!(v["data"]["edit-mode"], "reformat");
    assert!(
        std::fs::read_to_string(&file)
            .unwrap()
            .contains("id: first")
    );
}

#[test]
fn unprovable_mentions_are_listed_and_need_an_explicit_accept() {
    // `a` is also the name of a node in another scope: a template head could mean either.
    let src = "app: amb\nversion: 0.0.1\ndescription: x\nrequires: []\nnodes:\n  - id: a\n    inline: { kind: predicate, description: p, code: 'true' }\n  - id: loop\n    for-each: '[1]'\n    do:\n      - id: a\n        inline: { kind: predicate, description: p, code: 'true' }\n";
    let (_t, home, file) = setup(src);
    let f = file.to_str().unwrap();
    let (v, code) = run_json(&home, &["app", "rename-node", f, "a", "first"]);
    assert_eq!(code, 3, "{v}");
    assert_eq!(v["error"]["code"], "E_APP_RENAME_NODE_NEEDS_REVIEW");
    assert!(
        !v["error"]["details"]["unverified"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert_eq!(std::fs::read_to_string(&file).unwrap(), src);
    let (_, code) = run_json(
        &home,
        &["app", "rename-node", f, "a", "first", "--accept-unverified"],
    );
    assert_eq!(code, 0);
}
