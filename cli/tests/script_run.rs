//! `script/run` (#777) through the real CLI, with no host agent installed: the version gate on
//! the shipped manifest, the path-map rule at validate and compile, and an end-to-end
//! `aware app run` of `script/run` -> `file/write` in a temporary AWARE_HOME.
//!
//! The agents are installed from the repo's own `20-agents/` copies through `aware agent install`,
//! so a manifest edit that dropped `literal: true` or `path-map: true` turns this file red.

mod common;

use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};

fn repo_agent(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root")
        .join("20-agents/_core")
        .join(rel)
}

fn shipped_minimum() -> semver::Version {
    let text = fs::read_to_string(repo_agent("script/manifest.yaml")).expect("read the manifest");
    let value: serde_yaml::Value = serde_yaml::from_str(&text).expect("parse the manifest");
    semver::Version::parse(
        value["minimum-cli-version"]
            .as_str()
            .expect("the script agent declares a minimum CLI version"),
    )
    .expect("a semver minimum")
}

fn aware(home: &Path) -> Command {
    let mut cmd = Command::cargo_bin("aware").expect("aware test binary");
    cmd.env("AWARE_HOME", home);
    cmd
}

fn output_text(out: &std::process::Output) -> String {
    let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    text
}

/// A copy of the shipped `script` agent whose `minimum-cli-version` is THIS binary's version,
/// so the rest of the file can run on a CLI built before the release that ships it. Everything
/// else is the shipped bytes.
fn script_agent_for_this_cli(scratch: &Path) -> PathBuf {
    let src = repo_agent("script");
    let dst = scratch.join("script");
    for rel in [
        "manifest.yaml",
        "commands/run.md",
        "skills/writing-scripts.md",
    ] {
        let to = dst.join(rel);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(src.join(rel), &to).unwrap();
    }
    let manifest = dst.join("manifest.yaml");
    let text = fs::read_to_string(&manifest).unwrap();
    let pinned = format!("minimum-cli-version: {}", shipped_minimum());
    assert!(text.contains(&pinned), "the manifest pins {pinned}");
    fs::write(
        &manifest,
        text.replace(
            &pinned,
            &format!("minimum-cli-version: {}", env!("CARGO_PKG_VERSION")),
        ),
    )
    .unwrap();
    dst
}

/// A temporary AWARE_HOME with `script` and `file` installed, and nothing else: no host agent.
fn host_free_home() -> (tempfile::TempDir, tempfile::TempDir) {
    let home = tempfile::tempdir().expect("AWARE_HOME");
    let scratch = tempfile::tempdir().expect("scratch");
    for dir in [
        script_agent_for_this_cli(scratch.path()),
        repo_agent("file"),
    ] {
        let out = aware(home.path())
            .args(["agent", "install"])
            .arg(&dir)
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "install {}:\n{}",
            dir.display(),
            output_text(&out)
        );
    }
    let listed = fs::read_dir(home.path().join("agents"))
        .unwrap()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|name| !name.starts_with('.'))
        .collect::<Vec<_>>();
    assert!(
        listed.iter().all(|a| a == "script" || a == "file"),
        "only the two host-free agents are installed: {listed:?}"
    );
    (home, scratch)
}

/// The shipped manifest is gated to the CLI that ships the runner: an older CLI refuses it at
/// install, naming the version, and the CLI it is meant for accepts it.
#[test]
fn the_shipped_manifest_is_gated_to_the_cli_that_ships_the_runner() {
    let home = tempfile::tempdir().unwrap();
    let out = aware(home.path())
        .args(["agent", "install"])
        .arg(repo_agent("script"))
        .output()
        .unwrap();
    let text = output_text(&out);
    let this = semver::Version::parse(env!("CARGO_PKG_VERSION")).unwrap();
    let minimum = shipped_minimum();
    if this < minimum {
        assert!(
            !out.status.success(),
            "an older CLI must refuse it:\n{text}"
        );
        assert!(
            text.contains(&minimum.to_string()),
            "names the version:\n{text}"
        );
    } else {
        assert!(out.status.success(), "the shipping CLI accepts it:\n{text}");
    }
}

fn write_app(dir: &Path, name: &str, body: &str) {
    fs::create_dir_all(dir).unwrap();
    fs::write(dir.join(format!("{name}.flo")), body).unwrap();
}

/// A `script/run` -> `file/write` app; `files` is the YAML for the `files:` mapping entries.
fn compare_app(name: &str, files: &str) -> String {
    format!(
        r#"app: {name}
version: 0.1.0
description: Compares two exports without any host program.
requires:
  - script@0.1.x
  - file@1.x
nodes:
  - id: compare
    agent: script
    command: run
    inputs:
      language: javascript
      args:
        tolerance: 0.5
      files:
{files}      code: |
        // Lists the marks whose weight changed by more than the tolerance.
        const was = new Map(files.before.json.parts.map(p => [p.mark, p.weight]));
        const changed = files.after.json.parts
          .filter(p => was.has(p.mark) && Math.abs(was.get(p.mark) - p.weight) > args.tolerance)
          .map(p => ({{ mark: p.mark, was: was.get(p.mark), now: p.weight }}));
        console.log(`${{changed.length}} changed`);
        return {{ changed, stamp: run.startedAt, before: files.before.sha256 }};
  - id: save
    agent: file
    command: write
    inputs:
      path: '{{{{ inputs.out }}}}'
      bytes: '{{{{ compare.result }}}}'
    safety:
      transaction-group: script-probe
      snapshot: false
connections:
  - from: compare
    to: save
"#
    )
}

fn yaml_path(p: &Path) -> String {
    format!("'{}'", p.display())
}

#[test]
fn validate_and_compile_refuse_a_file_path_that_is_not_a_literal_absolute_path() {
    let (home, _scratch) = host_free_home();
    let exports = tempfile::tempdir().unwrap();
    let after = yaml_path(&exports.path().join("after.json"));
    let shapes = [
        (
            "relative",
            format!("        before: 'exports/before.json'\n        after: {after}\n"),
            "E_APP_FILE_PATH_NOT_LITERAL",
        ),
        (
            "traversal",
            format!(
                "        before: {}\n        after: {after}\n",
                yaml_path(&exports.path().join("..").join("before.json"))
            ),
            "E_APP_FILE_PATH_NOT_LITERAL",
        ),
        (
            "template",
            format!("        before: '{{{{ inputs.before }}}}'\n        after: {after}\n"),
            "E_APP_INPUT_MUST_BE_LITERAL",
        ),
        // A limit is literal too: an upstream value must never move one (#777 review). The
        // shape rides in on the `files:` slot, as a sibling input at the same indentation.
        (
            "templated limit",
            format!(
                "        before: {after}\n        after: {after}\n      \
                 memory-mb: '{{{{ inputs.memory }}}}'\n"
            ),
            "E_APP_INPUT_MUST_BE_LITERAL",
        ),
    ];
    for (shape, files, code) in shapes {
        let dir = tempfile::tempdir().unwrap();
        let name = format!("refuse-{shape}");
        write_app(dir.path(), &name, &compare_app(&name, &files));
        for verb in ["validate", "compile"] {
            let out = aware(home.path())
                .args(["app", verb])
                .arg(dir.path())
                .output()
                .unwrap();
            let text = output_text(&out);
            assert!(
                !out.status.success(),
                "{shape}: {verb} must refuse:\n{text}"
            );
            assert!(text.contains(code), "{shape}: {verb} names {code}:\n{text}");
            assert!(
                text.contains("\"compare\""),
                "{shape}: names the node:\n{text}"
            );
        }
        assert!(
            !dir.path().join(format!("{name}.lock")).exists(),
            "{shape}: a refused compile writes no lock"
        );
    }

    // The same app with literal absolute paths validates and compiles.
    let dir = tempfile::tempdir().unwrap();
    let files = format!(
        "        before: {}\n        after: {after}\n",
        yaml_path(&exports.path().join("before.json"))
    );
    write_app(dir.path(), "accept", &compare_app("accept", &files));
    for verb in ["validate", "compile"] {
        let out = aware(home.path())
            .args(["app", verb])
            .arg(dir.path())
            .output()
            .unwrap();
        assert!(out.status.success(), "{verb}:\n{}", output_text(&out));
    }
    assert!(dir.path().join("accept.lock").is_file());
}

/// The whole point of #777: a workflow whose steps touch no host runs with no host installed.
/// `script/run` reads two exports, compares them, and `file/write` lands the answer. The file's
/// contents reach the script but never the trace: the trace keeps a receipt per file.
#[test]
fn a_host_free_compare_runs_end_to_end_and_lands_its_answer() {
    let (home, _scratch) = host_free_home();
    let exports = tempfile::tempdir().unwrap();
    let marker = "PRIVATE-NOTE-7f3a";
    fs::write(
        exports.path().join("before.json"),
        format!(
            r#"{{"note":"{marker}","parts":[{{"mark":"B1","weight":10.0}},{{"mark":"B2","weight":20.0}}]}}"#
        ),
    )
    .unwrap();
    fs::write(
        exports.path().join("after.json"),
        r#"{"parts":[{"mark":"B1","weight":10.2},{"mark":"B2","weight":25.0}]}"#,
    )
    .unwrap();
    let files = format!(
        "        before: {}\n        after: {}\n",
        yaml_path(&exports.path().join("before.json")),
        yaml_path(&exports.path().join("after.json"))
    );
    write_app(
        &home.path().join("apps/host-free-compare"),
        "host-free-compare",
        &compare_app("host-free-compare", &files),
    );
    common::approve_installed_apps(home.path());

    let out_file = exports.path().join("changed.json");
    let out = aware(home.path())
        .args(["app", "run", "host-free-compare", "--input"])
        .arg(format!("out={}", out_file.display()))
        .output()
        .unwrap();
    let text = output_text(&out);
    assert!(out.status.success(), "the run must succeed:\n{text}");

    let landed: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&out_file).expect("file/write landed it"))
            .expect("JSON");
    assert_eq!(
        landed["changed"],
        serde_json::json!([{ "mark": "B2", "was": 20, "now": 25 }]),
        "{landed}"
    );
    let stamp = landed["stamp"].as_str().expect("the run start");
    assert!(stamp.ends_with('Z') && stamp.len() == 24, "{stamp}");
    let before_sha = landed["before"].as_str().unwrap();
    assert_eq!(before_sha.len(), 64);

    // The trace carries the receipt (name, sha256) and the log line, never the contents.
    let mut trace = String::new();
    for entry in walk(&home.path().join("logs")) {
        trace.push_str(&fs::read_to_string(entry).unwrap_or_default());
    }
    assert!(trace.contains(before_sha), "the trace keeps the receipt");
    assert!(trace.contains("1 changed"), "the trace keeps the log line");
    assert!(
        !trace.contains(marker),
        "a file's contents never reach the trace"
    );
}

fn walk(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.extend(walk(&path));
        } else {
            out.push(path);
        }
    }
    out
}
