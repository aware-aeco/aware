//! Bridge protocol compatibility, end to end through the real binary (#632).
//!
//! The fixture agent `verbot` dispatches to the managed bridge `aware-tekla`
//! (a tiny compiled helper planted under `<home>/bridges`, stamped with the
//! protocol each test chooses). Three registry versions declare different
//! protocol ranges, so each run says exactly which manifest it used:
//!
//! - 1.0.0 declares `{ max: 1 }`
//! - 1.1.0 declares `{ min: 2 }`
//! - 1.2.0 declares nothing
//!
//! A mismatch warns, is recorded in the run record, and the run still happens.

use assert_cmd::Command;
use std::io::Write;
use std::path::{Path, PathBuf};

struct Fixture {
    _tmp: tempfile::TempDir,
    root: PathBuf,
    aware: PathBuf,
    registry: String,
    helper: PathBuf,
}

/// Compile a helper that drains stdin and prints `{"version":"bridge-ran"}`.
fn compile_helper(dir: &Path) -> Option<PathBuf> {
    let rustc = which(if cfg!(windows) { "rustc.exe" } else { "rustc" })?;
    let src = dir.join("bridge_helper.rs");
    std::fs::write(
        &src,
        "use std::io::Read;\nfn main() { let mut s = String::new(); let _ = std::io::stdin().read_to_string(&mut s); \
         println!(\"{{\\\"version\\\":\\\"bridge-ran\\\"}}\"); }\n",
    )
    .unwrap();
    let bin = dir.join(if cfg!(windows) {
        "bridge_helper.exe"
    } else {
        "bridge_helper"
    });
    let status = std::process::Command::new(rustc)
        .arg(&src)
        .arg("-o")
        .arg(&bin)
        .status()
        .unwrap();
    assert!(status.success(), "rustc failed for the bridge helper");
    Some(bin)
}

fn which(name: &str) -> Option<PathBuf> {
    std::env::var_os("PATH").and_then(|paths| {
        std::env::split_paths(&paths)
            .map(|dir| dir.join(name))
            .find(|candidate| candidate.is_file())
    })
}

fn manifest(version: &str, declaration: Option<&str>) -> String {
    let decl = declaration
        .map(|d| format!("    bridge-protocol: {d}\n"))
        .unwrap_or_default();
    format!(
        "agent: verbot\nversion: {version}\ndescription: runs through a bridge\nstateful: false\nlicense: MIT\n\
         transport:\n  cli:\n    binary: aware-tekla\n{decl}\
         commands:\n  say:\n    lifecycle: single\n    mode: read\n    description: run the bridge\n    \
         outputs:\n      type: single\n      schema:\n        version: {{ type: string }}\n"
    )
}

fn tarball(path: &Path, manifest: &str) {
    let enc = flate2::write::GzEncoder::new(
        std::fs::File::create(path).unwrap(),
        flate2::Compression::default(),
    );
    let mut tar = tar::Builder::new(enc);
    let mut header = tar::Header::new_gnu();
    header.set_size(manifest.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    tar.append_data(
        &mut header,
        "aware-main/20-agents/verbot/manifest.yaml",
        manifest.as_bytes(),
    )
    .unwrap();
    let mut file = tar.into_inner().unwrap().finish().unwrap();
    file.flush().unwrap();
}

fn file_url(path: &Path) -> String {
    format!("file://{}", path.display().to_string().replace('\\', "/"))
}

fn fixture() -> Option<Fixture> {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().to_path_buf();
    let helper = compile_helper(&root)?;
    let versions = [
        ("1.0.0", Some("{ max: 1 }")),
        ("1.1.0", Some("{ min: 2 }")),
        ("1.2.0", None),
    ];
    let mut entries = Vec::new();
    for (version, declaration) in versions {
        let tar_path = root.join(format!("verbot-{version}.tar.gz"));
        tarball(&tar_path, &manifest(version, declaration));
        entries.push(format!(
            r#""{version}": {{ "tarball": "{}", "subdir": "aware-main/20-agents/verbot", "manifest-agent": "verbot", "manifest-version": "{version}" }}"#,
            file_url(&tar_path)
        ));
    }
    let index = root.join("registry-index.json");
    std::fs::write(
        &index,
        format!(
            r#"{{ "version": "1.0", "updated-at": "2026-10-06T00:00:00Z",
  "agents": {{ "verbot": {{ "versions": {{ {} }} }} }}, "bundles": {{}} }}"#,
            entries.join(",\n")
        ),
    )
    .unwrap();
    let aware = root.join("aware");
    Some(Fixture {
        registry: file_url(&index),
        aware,
        root,
        helper,
        _tmp: tmp,
    })
}

impl Fixture {
    fn aware(&self) -> Command {
        let mut command = Command::cargo_bin("aware").unwrap();
        command
            .env("AWARE_HOME", &self.aware)
            .env("AWARE_REGISTRY", &self.registry);
        command
    }

    fn ok(&self, args: &[&str]) -> String {
        let output = self.aware().args(args).output().unwrap();
        assert!(
            output.status.success(),
            "aware {args:?} failed:\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8_lossy(&output.stdout).into_owned()
    }

    /// Plant the managed bridge, stamped by this CLI, with `protocol` (or none).
    fn plant_bridge(&self, protocol: Option<u32>) {
        self.plant_bridge_stamped_by(protocol, env!("CARGO_PKG_VERSION"));
    }

    /// As [`Self::plant_bridge`], with the `.version` stamp of another release.
    fn plant_bridge_stamped_by(&self, protocol: Option<u32>, version: &str) {
        let bridges = self.aware.join("bridges");
        std::fs::create_dir_all(&bridges).unwrap();
        std::fs::copy(&self.helper, bridges.join("aware-tekla.exe")).unwrap();
        std::fs::write(bridges.join("aware-tekla.version"), version).unwrap();
        let marker = bridges.join("aware-tekla.protocol");
        match protocol {
            Some(n) => std::fs::write(marker, format!("{n}\n")).unwrap(),
            None => {
                let _ = std::fs::remove_file(marker);
            }
        }
    }

    /// Write `apps/<id>/<id>.flo` calling `verbot.say` and compile it.
    fn compiled_app(&self, id: &str) {
        let dir = self.aware.join("apps").join(id);
        std::fs::create_dir_all(&dir).unwrap();
        let source = dir.join(format!("{id}.flo"));
        std::fs::write(
            &source,
            format!(
                "app: {id}\nversion: 0.1.0\ndescription: runs verbot\n\
                 nodes:\n  - id: say\n    agent: verbot\n    command: say\nconnections: []\nrequires: []\n"
            ),
        )
        .unwrap();
        self.ok(&["app", "compile", source.to_str().unwrap()]);
    }

    fn check(&self, id: &str) -> serde_json::Value {
        let out = self.ok(&["--json", "app", "check", id]);
        let envelope: serde_json::Value = serde_json::from_str(out.trim()).unwrap();
        assert_eq!(envelope["ok"], true, "{envelope}");
        envelope["data"].clone()
    }

    /// Run `id`, which must succeed; return (stderr, every trace body joined).
    fn run(&self, id: &str) -> (String, String) {
        let output = self.aware().args(["app", "run", id]).output().unwrap();
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        assert!(
            output.status.success(),
            "a bridge verdict must never stop a run:\nstdout: {}\nstderr: {stderr}",
            String::from_utf8_lossy(&output.stdout)
        );
        let mut traces = Vec::new();
        collect_jsonl(&self.aware.join("logs").join(id), &mut traces);
        let body: String = traces
            .iter()
            .map(|p| std::fs::read_to_string(p).unwrap())
            .collect();
        (stderr, body)
    }
}

fn collect_jsonl(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_jsonl(&path, out);
        } else if path.extension().is_some_and(|e| e == "jsonl") {
            out.push(path);
        }
    }
}

fn row(check: &serde_json::Value, agent: &str, via: Option<&str>) -> serde_json::Value {
    check["agents"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["agent"] == agent && r.get("via").and_then(|v| v.as_str()) == via)
        .cloned()
        .unwrap_or_else(|| panic!("no row for {agent} via {via:?}: {check}"))
}

#[test]
fn a_known_mismatch_warns_is_recorded_and_the_run_still_happens() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    fx.plant_bridge(Some(1));
    fx.ok(&["agent", "install", "verbot@1.1.0"]); // declares min: 2
    fx.compiled_app("a");

    let check = fx.check("a");
    // The bridge is a separate fact: approval is untouched.
    assert_eq!(check["approval-current"], true, "{check}");
    let bridge = row(&check, "verbot", None)["bridge"].clone();
    assert_eq!(bridge["status"], "bridge-too-old", "{bridge}");
    assert_eq!(bridge["installed-protocol"], 1);
    assert_eq!(bridge["assumed"], false);
    assert_eq!(bridge["declared"]["min"], 2);
    assert!(
        bridge["choices"][0]
            .as_str()
            .unwrap()
            .contains("aware sidecar install tekla"),
        "{bridge}"
    );

    let (stderr, trace) = fx.run("a");
    assert!(
        stderr.contains("needs protocol 2 or newer")
            && stderr.contains("speaks protocol 1")
            && stderr.contains("Carry on anyway"),
        "{stderr}"
    );
    assert!(
        trace.contains("bridge-too-old"),
        "recorded in the run record: {trace}"
    );
    assert!(
        trace.contains("bridge-ran"),
        "the bridge really ran: {trace}"
    );
}

#[test]
fn a_matching_declaration_is_quiet() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    fx.plant_bridge(Some(2));
    fx.ok(&["agent", "install", "verbot@1.1.0"]);
    fx.compiled_app("a");
    let bridge = row(&fx.check("a"), "verbot", None)["bridge"].clone();
    assert_eq!(bridge["status"], "fits", "{bridge}");
    assert!(bridge["choices"].as_array().unwrap().is_empty());
    let (stderr, trace) = fx.run("a");
    assert!(!stderr.contains("protocol"), "{stderr}");
    assert!(trace.contains("bridge-ran"));
}

#[test]
fn no_declaration_means_no_claim_no_output_and_the_same_check() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    fx.plant_bridge(Some(1));
    fx.ok(&["agent", "install", "verbot@1.2.0"]);
    fx.compiled_app("a");
    let check = fx.check("a");
    assert!(
        row(&check, "verbot", None).get("bridge").is_none(),
        "{check}"
    );
    let (stderr, trace) = fx.run("a");
    assert!(!stderr.contains("protocol"), "{stderr}");
    assert!(!trace.contains("bridge-too"), "{trace}");
}

#[test]
fn a_bridge_with_no_stamp_is_unknown_not_a_mismatch_and_says_nothing_on_stderr() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    // Installed by a release that stamps protocols, yet carrying none: unknown.
    fx.plant_bridge_stamped_by(None, "0.156.0");
    fx.ok(&["agent", "install", "verbot@1.1.0"]);
    fx.compiled_app("a");
    let bridge = row(&fx.check("a"), "verbot", None)["bridge"].clone();
    assert_eq!(bridge["status"], "unknown", "{bridge}");
    assert_eq!(bridge["installed-protocol"], serde_json::Value::Null);
    let (stderr, _) = fx.run("a");
    assert!(!stderr.contains("needs protocol"), "{stderr}");
}

#[test]
fn a_bridge_installed_before_stamping_existed_is_the_baseline_and_says_so() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    fx.plant_bridge_stamped_by(None, "0.155.0");
    fx.ok(&["agent", "install", "verbot@1.1.0"]); // declares min: 2
    fx.compiled_app("a");
    let bridge = row(&fx.check("a"), "verbot", None)["bridge"].clone();
    assert_eq!(bridge["status"], "bridge-too-old", "{bridge}");
    assert_eq!(bridge["installed-protocol"], 1);
    assert_eq!(
        bridge["assumed"], true,
        "an assumed baseline is labelled as such"
    );
}

#[test]
fn the_verdict_is_judged_on_the_pinned_copy_not_the_installed_one() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    fx.plant_bridge(Some(1));
    fx.ok(&["agent", "install", "verbot@1.0.0"]); // declares max: 1 — fits
    fx.compiled_app("a");
    // The installed copy moves to 1.1.0, which declares min: 2 (too old for the bridge).
    fx.ok(&["agent", "update", "verbot@1.1.0"]);

    let check = fx.check("a");
    let pinned = row(&check, "verbot", None);
    assert_eq!(pinned["pinned-version"], "1.0.0", "{check}");
    assert_eq!(pinned["installed-version"], "1.1.0");
    assert_eq!(pinned["bridge"]["status"], "fits", "{check}");
    let (stderr, trace) = fx.run("a");
    assert!(!stderr.contains("needs protocol"), "{stderr}");
    assert!(trace.contains("\"fits\""), "{trace}");

    // A fresh compile pins the installed 1.1.0, and now the verdict is the mismatch.
    fx.compiled_app("b");
    assert_eq!(
        row(&fx.check("b"), "verbot", None)["bridge"]["status"],
        "bridge-too-old"
    );
}

#[test]
fn a_leaf_behind_a_backing_app_carries_its_own_verdict() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    fx.plant_bridge(Some(1));
    fx.ok(&["agent", "install", "verbot@1.1.0"]);

    let src = fx.root.join("src");
    let inner = src.join("inner");
    std::fs::create_dir_all(&inner).unwrap();
    std::fs::write(
        inner.join("inner.flo"),
        "app: inner\nversion: 0.1.0\ndescription: wraps verbot\nexposes-as-agent: true\n\
         exposed-commands:\n  ask:\n    lifecycle: single\n    outputs:\n      type: single\n\
         nodes:\n  - id: say\n    agent: verbot\n    command: say\nconnections: []\nrequires: []\n",
    )
    .unwrap();
    fx.ok(&["app", "compile", inner.join("inner.flo").to_str().unwrap()]);
    fx.ok(&["app", "install", inner.to_str().unwrap()]);
    let outer = fx.aware.join("apps/outer");
    std::fs::create_dir_all(&outer).unwrap();
    std::fs::write(
        outer.join("outer.flo"),
        "app: outer\nversion: 0.1.0\ndescription: calls inner\n\
         nodes:\n  - id: call\n    agent: inner\n    command: ask\nconnections: []\nrequires: []\n",
    )
    .unwrap();
    fx.ok(&["app", "compile", outer.join("outer.flo").to_str().unwrap()]);

    let check = fx.check("outer");
    let leaf = row(&check, "verbot", Some("inner"));
    assert_eq!(leaf["bridge"]["status"], "bridge-too-old", "{check}");
    assert!(row(&check, "inner", None).get("bridge").is_none());

    let output = fx.aware().args(["app", "run", "outer"]).output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(
        stderr.contains("agent verbot:") && stderr.contains("needs protocol 2"),
        "{stderr}"
    );
}
