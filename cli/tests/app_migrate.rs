//! `aware app migrate plan|prepare|discard|hold|unhold`, end to end through the
//! real binary (#628 PR2).
//!
//! Same fixture as `agent_store_versions.rs`: agent `verbot` whose 1.0.0 and
//! 1.1.0 builds print different markers, so a run's output proves which bytes
//! were dispatched. The two builds point at different helper binaries, so the
//! executable contract changes between them: the comparison is
//! `not-comparable` and a person must decide.

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

    fn json(&self, args: &[&str]) -> serde_json::Value {
        let mut full = vec!["--json"];
        full.extend_from_slice(args);
        let output = self.aware().args(&full).output().unwrap();
        let text = String::from_utf8_lossy(&output.stdout).into_owned();
        serde_json::from_str(text.trim()).unwrap_or_else(|e| {
            panic!(
                "aware {full:?}: not one JSON envelope ({e}):\nstdout: {text}\nstderr: {}",
                String::from_utf8_lossy(&output.stderr)
            )
        })
    }

    /// `ok: true` data of `aware --json <args>`.
    fn data(&self, args: &[&str]) -> serde_json::Value {
        let envelope = self.json(args);
        assert_eq!(envelope["ok"], true, "aware {args:?}: {envelope}");
        envelope["data"].clone()
    }

    fn plan_row(&self, app: &str) -> serde_json::Value {
        let data = self.data(&["app", "migrate", "plan", "--app", app]);
        data["apps"][0].clone()
    }

    fn lock_state(&self, app: &str) -> (Vec<u8>, std::time::SystemTime) {
        let path = self
            .aware
            .join("apps")
            .join(app)
            .join(format!("{app}.lock"));
        (
            std::fs::read(&path).unwrap(),
            std::fs::metadata(&path).unwrap().modified().unwrap(),
        )
    }
}

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

fn codes(row: &serde_json::Value) -> Vec<String> {
    row["reasons"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["code"].as_str().unwrap().to_string())
        .collect()
}

#[test]
fn prepare_writes_a_candidate_and_never_touches_the_approved_lock_or_the_run() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    fx.ok(&["agent", "install", "verbot@1.0.0"]);
    fx.compiled_app("a");
    assert_eq!(fx.run_says("a"), V1);

    // Nothing newer installed: nothing to carry forward.
    let row = fx.plan_row("a");
    assert_eq!(row["state"], "up-to-date", "{row}");

    fx.ok(&["agent", "update", "verbot"]);
    let row = fx.plan_row("a");
    assert_eq!(row["state"], "needs-person", "{row}");
    assert_eq!(row["no-click-available"], false);
    assert_eq!(row["advisory"], serde_json::Value::Null);
    assert_eq!(row["targets"][0]["agent"], "verbot");
    assert_eq!(row["targets"][0]["from"]["version"], V1);
    assert_eq!(row["targets"][0]["to"]["version"], V2);
    assert_eq!(row["targets"][0]["bump"], "minor");
    assert_eq!(row["effect"], "declared-read-only", "{row}");
    // The two builds run different programs: the contract changed.
    assert_eq!(row["contract"]["unchanged"], false);
    assert_eq!(row["comparison"]["status"], "not-comparable");
    assert_eq!(row["comparison"]["runs"], 0);
    let reasons = codes(&row);
    assert!(
        reasons.contains(&"no-fixed-state-method".to_string()),
        "{reasons:?}"
    );
    for reason in row["reasons"].as_array().unwrap() {
        assert!(!reason["text"].as_str().unwrap().is_empty(), "{reason}");
    }
    assert_eq!(row["candidate"]["present"], false);

    let (lock_bytes, lock_mtime) = fx.lock_state("a");
    let prepared = fx.data(&["app", "migrate", "prepare", "a"]);
    assert_eq!(prepared["prepared"], true, "{prepared}");
    assert_eq!(prepared["lock-unchanged"], true);
    assert_eq!(
        fx.lock_state("a"),
        (lock_bytes.clone(), lock_mtime),
        "the approved lock is byte-identical and untouched"
    );
    let dir = fx.aware.join("apps/a/.aware-migration");
    let candidate = std::fs::read(dir.join("a.candidate.lock")).unwrap();
    let evidence: serde_json::Value =
        serde_json::from_slice(&std::fs::read(dir.join("a.evidence.json")).unwrap()).unwrap();
    assert_eq!(evidence["format"], "aware.migration-evidence/v1");
    assert_eq!(
        evidence["header"]["candidate-digest"],
        prepared["candidate-digest"]
    );
    assert_eq!(evidence["header"]["targets"]["verbot"]["to"]["version"], V2);
    assert!(
        String::from_utf8_lossy(&candidate).contains("verbot: 1.1.0"),
        "{}",
        String::from_utf8_lossy(&candidate)
    );

    // The run reads only <app>.lock: still the approved 1.0.0 bytes.
    assert_eq!(fx.run_says("a"), V1, "a candidate never reaches a run");
    // ...even when the candidate is garbage.
    std::fs::write(dir.join("a.candidate.lock"), "{ this is not a lock").unwrap();
    assert_eq!(fx.run_says("a"), V1);
    std::fs::write(dir.join("a.candidate.lock"), &candidate).unwrap();

    let row = fx.plan_row("a");
    assert_eq!(row["candidate"]["present"], true);
    assert_eq!(row["candidate"]["fresh"], true, "{row}");
    assert_eq!(
        row["candidate"]["candidate-digest"],
        prepared["candidate-digest"]
    );

    let discarded = fx.data(&["app", "migrate", "discard", "a"]);
    assert_eq!(discarded["discarded"], true);
    assert!(
        !dir.exists(),
        "discard removes the candidate directory when empty"
    );
    assert_eq!(fx.plan_row("a")["candidate"]["present"], false);
    assert_eq!(
        fx.data(&["app", "migrate", "discard", "a"])["discarded"],
        false
    );
    assert_eq!(fx.lock_state("a").0, lock_bytes);
}

#[test]
fn a_held_app_is_reported_held_until_a_person_lifts_it() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    fx.ok(&["agent", "install", "verbot@1.0.0"]);
    fx.compiled_app("a");
    fx.ok(&["agent", "update", "verbot"]);

    let refused = fx.json(&["app", "migrate", "hold", "a", "--actor", "  "]);
    assert_eq!(refused["ok"], false);
    assert_eq!(refused["error"]["code"], "E_MIGRATE_NO_ACTOR");

    let held = fx.data(&[
        "app",
        "migrate",
        "hold",
        "a",
        "--actor",
        "pawel",
        "--reason",
        "certified",
    ]);
    assert_eq!(held["held"], true);
    assert!(fx.aware.join("apps/a/.aware-approvals/HOLD").is_file());
    let row = fx.plan_row("a");
    assert_eq!(row["state"], "held", "{row}");
    assert_eq!(row["hold"]["held-by"], "pawel");
    assert_eq!(row["hold"]["reason"], "certified");
    assert_eq!(
        row["targets"][0]["agent"], "verbot",
        "the pending move is still shown"
    );

    let lifted = fx.data(&["app", "migrate", "unhold", "a", "--actor", "pawel"]);
    assert_eq!(lifted["was-held"], true);
    assert_eq!(fx.plan_row("a")["state"], "needs-person");
}

#[test]
fn a_backing_app_is_never_prepared_and_its_callers_are_named() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    fx.ok(&["agent", "install", "verbot@1.0.0"]);
    let inner = fx.root.join("src").join("inner");
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
    fx.ok(&["agent", "update", "verbot"]);
    let (inner_lock, _) = fx.lock_state("inner");

    let refused = fx.json(&["app", "migrate", "prepare", "inner"]);
    assert_eq!(refused["ok"], false, "{refused}");
    assert_eq!(refused["error"]["code"], "E_MIGRATE_BACKING_APP");
    assert_eq!(
        refused["error"]["details"]["callers"],
        serde_json::json!(["outer"])
    );
    assert!(
        !fx.aware.join("apps/inner/.aware-migration").exists(),
        "nothing written"
    );
    assert_eq!(fx.lock_state("inner").0, inner_lock);

    let plan = fx.data(&["app", "migrate", "plan", "--all"]);
    let row = |id: &str| {
        plan["apps"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["app"] == id)
            .cloned()
            .unwrap_or_else(|| panic!("no row for {id}: {plan}"))
    };
    let inner_row = row("inner");
    assert_eq!(inner_row["state"], "needs-person", "{inner_row}");
    assert_eq!(inner_row["backing-app-moved"], true);
    assert!(codes(&inner_row).contains(&"backing-app-moved".to_string()));
    let outer_row = row("outer");
    assert_eq!(outer_row["state"], "needs-person", "{outer_row}");
    assert!(
        codes(&outer_row).contains(&"backing-app-moved".to_string()),
        "{outer_row}"
    );
}

#[test]
fn malformed_or_unusable_targets_are_refused_as_misuse() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    fx.ok(&["agent", "install", "verbot@1.0.0"]);
    fx.compiled_app("a");
    fx.ok(&["agent", "update", "verbot"]);
    for (to, code) in [
        ("verbot", "E_MIGRATE_BAD_TARGET"),
        ("verbot@sha256:XYZ", "E_MIGRATE_BAD_TARGET"),
        ("ghost@1.0.0", "E_MIGRATE_TARGET_UNUSED"),
        ("verbot@9.9.9", "E_MIGRATE_TARGET_NOT_STORED"),
    ] {
        let refused = fx.json(&["app", "migrate", "prepare", "a", "--to", to]);
        assert_eq!(refused["ok"], false, "{to}: {refused}");
        assert_eq!(refused["error"]["code"], code, "{to}: {refused}");
    }
    assert!(!fx.aware.join("apps/a/.aware-migration").exists());
    // An explicit version target that resolves is fine.
    let prepared = fx.data(&["app", "migrate", "prepare", "a", "--to", "verbot@1.1.0"]);
    assert_eq!(prepared["prepared"], true, "{prepared}");
    let unknown = fx.json(&["app", "migrate", "plan", "--app", "nope"]);
    assert_eq!(unknown["ok"], false);
    assert_eq!(unknown["error"]["code"], "E_MIGRATE_APP_NOT_FOUND");
}
