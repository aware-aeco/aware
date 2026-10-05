//! Versioned agent store, end to end through the real binary (#626, plan tests 7-9).
//!
//! The fixture agent `verbot` has one command, `say`, whose CLI transport is a
//! tiny compiled helper that prints which build it is. Version 1.0.0's manifest
//! points at the helper that prints `printed-by-1.0.0`; 1.1.0's at the one that
//! prints `printed-by-1.1.0`. So a run's output says exactly which manifest
//! bytes were dispatched — not which version string some record claims.

use assert_cmd::Command;
use std::io::Write;
use std::path::{Path, PathBuf};

const V1: &str = "1.0.0";
const V2: &str = "1.1.0";

struct Fixture {
    _tmp: tempfile::TempDir,
    root: PathBuf,
    aware: PathBuf,
    registry: String,
    bins: [PathBuf; 2],
}

/// Compile a helper that drains stdin and prints `{"version":"printed-by-<v>"}`.
fn compile_echo(dir: &Path, version: &str) -> Option<PathBuf> {
    let rustc = which(if cfg!(windows) { "rustc.exe" } else { "rustc" })?;
    let name = format!("echo_{}", version.replace('.', "_"));
    let src = dir.join(format!("{name}.rs"));
    std::fs::write(
        &src,
        format!(
            "use std::io::Read;\nfn main() {{ let mut s = String::new(); let _ = std::io::stdin().read_to_string(&mut s); \
             println!(\"{{{{\\\"version\\\":\\\"printed-by-{version}\\\"}}}}\"); }}\n"
        ),
    )
    .unwrap();
    let bin = dir.join(if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.clone()
    });
    let status = std::process::Command::new(rustc)
        .arg(&src)
        .arg("-o")
        .arg(&bin)
        .status()
        .unwrap();
    assert!(status.success(), "rustc failed for {name}");
    Some(bin)
}

fn which(name: &str) -> Option<PathBuf> {
    std::env::var_os("PATH").and_then(|paths| {
        std::env::split_paths(&paths)
            .map(|dir| dir.join(name))
            .find(|candidate| candidate.is_file())
    })
}

fn manifest(version: &str, binary: &Path) -> String {
    let binary = binary.to_string_lossy().replace('\\', "/");
    format!(
        "agent: verbot\nversion: {version}\ndescription: prints which build it is\nstateful: false\nlicense: MIT\n\
         transport:\n  cli:\n    binary: {binary}\n\
         commands:\n  say:\n    lifecycle: single\n    mode: read\n    description: print the build\n    \
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
    let b1 = compile_echo(&root, V1)?;
    let b2 = compile_echo(&root, V2)?;
    let t1 = root.join("verbot-1.0.0.tar.gz");
    let t2 = root.join("verbot-1.1.0.tar.gz");
    tarball(&t1, &manifest(V1, &b1));
    tarball(&t2, &manifest(V2, &b2));
    let index = root.join("registry-index.json");
    std::fs::write(
        &index,
        format!(
            r#"{{
    "version": "1.0",
    "updated-at": "2026-10-04T00:00:00Z",
    "agents": {{
        "verbot": {{
            "versions": {{
                "1.0.0": {{ "tarball": "{}", "subdir": "aware-main/20-agents/verbot", "manifest-agent": "verbot", "manifest-version": "1.0.0" }},
                "1.1.0": {{ "tarball": "{}", "subdir": "aware-main/20-agents/verbot", "manifest-agent": "verbot", "manifest-version": "1.1.0" }}
            }}
        }}
    }},
    "bundles": {{}}
}}"#,
            file_url(&t1),
            file_url(&t2)
        ),
    )
    .unwrap();
    let aware = root.join("aware");
    Some(Fixture {
        registry: file_url(&index),
        aware,
        root,
        bins: [b1, b2],
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

    /// Write `apps/<id>/<id>.flo` calling `verbot.say` and compile it.
    fn compiled_app(&self, id: &str) {
        let dir = self.aware.join("apps").join(id);
        std::fs::create_dir_all(&dir).unwrap();
        let source = dir.join(format!("{id}.flo"));
        std::fs::write(
            &source,
            format!(
                "app: {id}\nversion: 0.1.0\ndescription: says which verbot ran\n\
                 nodes:\n  - id: say\n    agent: verbot\n    command: say\nconnections: []\nrequires: []\n"
            ),
        )
        .unwrap();
        self.ok(&["app", "compile", source.to_str().unwrap()]);
    }

    /// Run app `id` and return which build printed its output.
    fn run_says(&self, id: &str) -> String {
        self.ok(&["app", "run", id]);
        printed_by(&self.aware.join("logs").join(id))
    }

    fn check(&self, id: &str) -> serde_json::Value {
        let out = self.ok(&["--json", "app", "check", id]);
        let envelope: serde_json::Value = serde_json::from_str(out.trim()).unwrap();
        assert_eq!(envelope["ok"], true, "{envelope}");
        envelope["data"].clone()
    }
}

/// The newest trace under `dir` names exactly one build; return it.
fn printed_by(dir: &Path) -> String {
    let mut traces = Vec::new();
    collect_jsonl(dir, &mut traces);
    traces.sort_by_key(|path| std::fs::metadata(path).unwrap().modified().unwrap());
    let latest = traces.last().expect("no trace written");
    let body = std::fs::read_to_string(latest).unwrap();
    let builds: Vec<&str> = [V1, V2]
        .into_iter()
        .filter(|v| body.contains(&format!("printed-by-{v}")))
        .collect();
    assert_eq!(
        builds.len(),
        1,
        "expected one build in {}:\n{body}",
        latest.display()
    );
    builds[0].to_string()
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

fn agent_row(check: &serde_json::Value, agent: &str) -> serde_json::Value {
    check["agents"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["agent"] == agent && row.get("via").is_none())
        .cloned()
        .unwrap_or_else(|| panic!("no row for {agent}: {check}"))
}

// ── plan test 7 ───────────────────────────────────────────────────────────────

#[test]
fn an_update_does_not_change_the_bytes_an_approved_app_runs() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    fx.ok(&["agent", "install", "verbot@1.0.0"]);
    fx.compiled_app("a");
    assert_eq!(fx.run_says("a"), V1);

    fx.ok(&["agent", "update", "verbot"]);
    let installed = std::fs::read_to_string(fx.aware.join("agents/verbot/manifest.yaml")).unwrap();
    assert!(installed.contains("version: 1.1.0"), "the update landed");

    // The front door's question, answered by the run's own resolver.
    let check = fx.check("a");
    assert_eq!(check["approval-current"], true, "{check}");
    assert_eq!(check["approval-kind"], "bytes", "{check}");
    let row = agent_row(&check, "verbot");
    assert_eq!(row["resolution"], "stored", "{check}");
    assert_eq!(row["pinned-version"], V1);
    assert_eq!(row["installed-version"], V2);

    // No recompile: A still runs the 1.0.0 bytes it approved.
    assert_eq!(
        fx.run_says("a"),
        V1,
        "app A must run on the approved 1.0.0 bytes"
    );

    // A freshly compiled app pins and runs the new version.
    fx.compiled_app("b");
    assert_eq!(fx.run_says("b"), V2);
    assert_eq!(agent_row(&fx.check("b"), "verbot")["resolution"], "current");

    // `agent list --json` shows both stored versions (display only).
    let listed: serde_json::Value =
        serde_json::from_str(fx.ok(&["--json", "agent", "list"]).trim()).unwrap();
    let row = listed["data"]["agents"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["id"] == "verbot")
        .unwrap()
        .clone();
    let stored: Vec<&str> = row["stored"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry["version"].as_str().unwrap())
        .collect();
    assert!(stored.contains(&V1) && stored.contains(&V2), "{row}");

    // Uninstall removes the tool: the run refuses even though the store keeps the bytes.
    fx.ok(&["agent", "uninstall", "verbot"]);
    let refused = fx.aware().args(["app", "run", "a"]).output().unwrap();
    assert!(!refused.status.success());
    assert!(
        String::from_utf8_lossy(&refused.stderr).contains("not installed"),
        "{}",
        String::from_utf8_lossy(&refused.stderr)
    );
    assert_eq!(agent_row(&fx.check("a"), "verbot")["resolution"], "missing");
    assert!(
        fx.aware.join("agent-store-v2/verbot").is_dir(),
        "uninstall leaves the store"
    );
}

// ── plan test 8 ───────────────────────────────────────────────────────────────

#[test]
fn a_nested_app_backed_dispatch_keeps_its_backing_apps_approved_version() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    fx.ok(&["agent", "install", "verbot@1.0.0"]);

    // The backing app, approved against verbot 1.0.0, installed as an agent.
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

    // The outer app calls it.
    let outer = fx.aware.join("apps/outer");
    std::fs::create_dir_all(&outer).unwrap();
    std::fs::write(
        outer.join("outer.flo"),
        "app: outer\nversion: 0.1.0\ndescription: calls inner\n\
         nodes:\n  - id: call\n    agent: inner\n    command: ask\nconnections: []\nrequires: []\n",
    )
    .unwrap();
    fx.ok(&["app", "compile", outer.join("outer.flo").to_str().unwrap()]);

    fx.ok(&["agent", "update", "verbot"]);
    let check = fx.check("outer");
    assert_eq!(check["approval-current"], true, "{check}");
    let leaf = check["agents"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["agent"] == "verbot" && row["via"] == "inner")
        .cloned()
        .unwrap_or_else(|| panic!("no nested verbot row: {check}"));
    assert_eq!(leaf["resolution"], "stored", "{check}");

    fx.ok(&["app", "run", "outer"]);
    assert_eq!(
        printed_by(&fx.aware.join("logs/inner/nested")),
        V1,
        "the nested run dispatched the backing app's approved 1.0.0 bytes"
    );
}

// ── plan test 9 ───────────────────────────────────────────────────────────────

#[test]
fn editing_the_working_copy_after_compile_does_not_reach_an_approved_run() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    fx.ok(&["agent", "install", "verbot@1.0.0"]);
    fx.compiled_app("a");

    // Hand-edit the installed copy: same version string, different bytes — its
    // transport now points at the OTHER build.
    let manifest_path = fx.aware.join("agents/verbot/manifest.yaml");
    let edited = manifest(V1, &fx.bins[1]);
    std::fs::write(&manifest_path, edited).unwrap();

    assert_eq!(
        fx.run_says("a"),
        V1,
        "the run dispatches the snapshot taken at compile, not the edited copy"
    );
    assert_eq!(agent_row(&fx.check("a"), "verbot")["resolution"], "stored");

    // Recompiling approves the edited bytes, and only then do they run.
    fx.compiled_app("a");
    assert_eq!(fx.run_says("a"), V2);
}
