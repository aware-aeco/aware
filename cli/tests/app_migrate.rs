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
    assert!(fx.aware.join("apps/a/.aware-approvals/HOLD.a").is_file());
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
    // Well-formed bytes this machine has never stored.
    let unstored = format!("verbot@sha256:{}", "0".repeat(64));
    for (to, code) in [
        ("verbot", "E_MIGRATE_BAD_TARGET"),
        ("verbot@sha256:XYZ", "E_MIGRATE_BAD_TARGET"),
        ("ghost@1.0.0", "E_MIGRATE_TARGET_UNUSED"),
        ("verbot@9.9.9", "E_MIGRATE_TARGET_NOT_STORED"),
        (unstored.as_str(), "E_MIGRATE_TARGET_NOT_STORED"),
    ] {
        let refused = fx.json(&["app", "migrate", "prepare", "a", "--to", to]);
        assert_eq!(refused["ok"], false, "{to}: {refused}");
        assert_eq!(refused["error"]["code"], code, "{to}: {refused}");
    }
    // `plan` reports the same target as data, in the same words.
    let row =
        fx.data(&["app", "migrate", "plan", "--app", "a", "--to", &unstored])["apps"][0].clone();
    assert_eq!(row["state"], "blocked", "{row}");
    assert_eq!(
        row["reasons"][0]["code"], "migrate-target-not-stored",
        "{row}"
    );
    assert!(
        row["reasons"][0]["text"]
            .as_str()
            .unwrap()
            .contains("no stored copy of agent verbot"),
        "{row}"
    );
    assert!(!fx.aware.join("apps/a/.aware-migration").exists());
    // An explicit version target that resolves is fine.
    let prepared = fx.data(&["app", "migrate", "prepare", "a", "--to", "verbot@1.1.0"]);
    assert_eq!(prepared["prepared"], true, "{prepared}");
    let unknown = fx.json(&["app", "migrate", "plan", "--app", "nope"]);
    assert_eq!(unknown["ok"], false);
    assert_eq!(unknown["error"]["code"], "E_MIGRATE_APP_NOT_FOUND");
}

/// Review #628 PR2 round 1: an installed copy that cannot be hashed is a
/// warning in the plain-text plan too, never a silent "up to date".
#[test]
fn plain_text_plan_prints_the_unhashable_copy_warning() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    fx.ok(&["agent", "install", "verbot@1.0.0"]);
    fx.compiled_app("a");
    let outside = fx.root.join("outside");
    std::fs::create_dir_all(&outside).unwrap();
    let link = fx.aware.join("agents").join("verbot").join("linked");
    #[cfg(windows)]
    {
        let status = std::process::Command::new("cmd.exe")
            .args(["/C", "mklink", "/J"])
            .arg(&link)
            .arg(&outside)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .unwrap();
        assert!(status.success(), "failed to create test junction");
    }
    #[cfg(unix)]
    std::os::unix::fs::symlink(&outside, &link).unwrap();
    let text = fx.ok(&["app", "migrate", "plan", "--app", "a"]);
    assert!(
        text.contains("\u{26a0}") && text.contains("cannot be compared"),
        "{text}"
    );
}

// ── #628 PR3a: reading a carried-forward approval ──────────────────────────
//
// No promote verb exists yet (PR3b), so the promoted lock is built BY HAND from
// a real `migrate prepare` candidate, exactly as plan §5/§12/§13 lay it out.

fn sha(bytes: &[u8]) -> String {
    use sha2::Digest;
    format!("sha256:{:x}", sha2::Sha256::digest(bytes))
}

fn hex(digest: &str) -> &str {
    digest.strip_prefix("sha256:").unwrap()
}

/// Archive `bytes` as `.aware-approvals/<hex>.<ext>` in `dir`; return the digest.
fn archive(dir: &Path, bytes: &[u8], ext: &str) -> String {
    let digest = sha(bytes);
    let approvals = dir.join(".aware-approvals");
    std::fs::create_dir_all(&approvals).unwrap();
    std::fs::write(approvals.join(format!("{}.{ext}", hex(&digest))), bytes).unwrap();
    digest
}

/// The full pin map of a lock's top level, as a link's `from` / `to`.
fn pin_map(lock: &serde_yaml::Value) -> serde_yaml::Value {
    let mut out = serde_yaml::Mapping::new();
    let pins = lock["agent-pins"].as_mapping().unwrap();
    for (id, version) in pins {
        let mut pin = serde_yaml::Mapping::new();
        pin.insert("version".into(), version.clone());
        if let Some(d) = lock.get("agent-digests").and_then(|m| m.get(id)) {
            pin.insert("digest".into(), d.clone());
        }
        if let Some(b) = lock.get("agent-bundle-pins").and_then(|m| m.get(id)) {
            pin.insert("bundle-pin".into(), b.clone());
        }
        out.insert(id.clone(), serde_yaml::Value::Mapping(pin));
    }
    serde_yaml::Value::Mapping(out)
}

struct HandPromotion {
    original_digest: String,
    evidence_digest: String,
    resulting_digest: String,
}

impl Fixture {
    /// Prepare app `id` and promote the candidate by hand, carried forward by
    /// a claimed person approval.
    fn promote_by_hand(&self, id: &str) -> HandPromotion {
        self.data(&["app", "migrate", "prepare", id]);
        let dir = self.aware.join("apps").join(id);
        let lock_path = dir.join(format!("{id}.lock"));
        let base_bytes = std::fs::read(&lock_path).unwrap();
        let base: serde_yaml::Value = serde_yaml::from_slice(&base_bytes).unwrap();
        let candidate_bytes =
            std::fs::read(dir.join(format!(".aware-migration/{id}.candidate.lock"))).unwrap();
        let evidence_bytes =
            std::fs::read(dir.join(format!(".aware-migration/{id}.evidence.json"))).unwrap();
        let evidence: serde_json::Value = serde_json::from_slice(&evidence_bytes).unwrap();
        let plan_digest = evidence["header"]["plan-digest"]
            .as_str()
            .unwrap()
            .to_string();

        let original_digest = archive(&dir, &base_bytes, "lock");
        let resulting_digest = archive(&dir, &candidate_bytes, "lock");
        let evidence_digest = archive(&dir, &evidence_bytes, "json");
        let record = serde_json::json!({
            "format": 1, "kind": "person", "actor": "e2e", "front-door": "floless@test",
            "approval-ref": "batch-1", "candidate-digest": resulting_digest,
            "base-lock-digest": original_digest, "plan-digest": plan_digest,
            "statement-sha256": sha(b"I approve"), "at": "2026-10-05T00:00:00Z",
        });
        let record_digest = archive(&dir, record.to_string().as_bytes(), "json");

        let mut candidate: serde_yaml::Value = serde_yaml::from_slice(&candidate_bytes).unwrap();
        let mut original = serde_yaml::Mapping::new();
        original.insert("lock-digest".into(), original_digest.clone().into());
        original.insert(
            "archive".into(),
            format!(".aware-approvals/{}.lock", hex(&original_digest)).into(),
        );
        for key in [
            "compiled-at",
            "compiler-version",
            "agent-pins",
            "agent-digests",
            "agent-bundle-pins",
        ] {
            if let Some(value) = base.get(key) {
                original.insert(key.into(), value.clone());
            }
        }
        let link = serde_json::json!({
            "seq": 1, "kind": "carried-forward",
            "from-lock-digest": original_digest, "to-plan-digest": plan_digest,
            "resulting-lock-digest": resulting_digest,
            "carried-forward-by": {
                "kind": "person", "actor": "e2e", "approval-ref": "batch-1",
                "front-door": "floless@test", "attested": false,
                "approval-record-digest": record_digest,
            },
            "evidence-digest": evidence_digest,
            "evidence": format!(".aware-approvals/{}.json", hex(&evidence_digest)),
            "front-door": "floless@test", "cli-version": "test", "promoted-at": "2026-10-05T00:00:00Z",
        });
        let mut link: serde_yaml::Value = serde_json::from_value(link).unwrap();
        let fields = link.as_mapping_mut().unwrap();
        fields.insert("from".into(), pin_map(&base));
        fields.insert("to".into(), pin_map(&candidate));
        let mut approval = serde_yaml::Mapping::new();
        approval.insert("format".into(), 1.into());
        approval.insert("original".into(), serde_yaml::Value::Mapping(original));
        approval.insert("successors".into(), serde_yaml::Value::Sequence(vec![link]));
        let map = candidate.as_mapping_mut().unwrap();
        let source_hash = map["source-hash"].as_str().unwrap().to_string();
        map.insert(
            "source-hash".into(),
            format!("successor-v1:{}", hex(&source_hash)).into(),
        );
        map.insert("approval".into(), serde_yaml::Value::Mapping(approval));
        std::fs::write(
            &lock_path,
            format!(
                "# {id}.lock (carried forward by hand, test)\n\n{}",
                serde_yaml::to_string(&candidate).unwrap()
            ),
        )
        .unwrap();
        HandPromotion {
            original_digest,
            evidence_digest,
            resulting_digest,
        }
    }

    /// The `run_config` of app `id`'s latest run, from its trace.
    fn last_run_config(&self, id: &str) -> serde_json::Value {
        let mut traces = Vec::new();
        collect_jsonl(&self.aware.join("logs").join(id), &mut traces);
        traces.sort_by_key(|path| std::fs::metadata(path).unwrap().modified().unwrap());
        let body = std::fs::read_to_string(traces.last().unwrap()).unwrap();
        body.lines()
            .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
            .find_map(|event| {
                event
                    .get("config")
                    .filter(|c| c.get("approval").is_some())
                    .cloned()
            })
            .unwrap_or_else(|| panic!("no run config with an approval in:\n{body}"))
    }
}

#[test]
fn a_carried_forward_lock_runs_its_new_pins_labelled_and_a_tampered_one_is_refused() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    fx.ok(&["agent", "install", "verbot@1.0.0"]);
    fx.compiled_app("a");
    assert_eq!(fx.run_says("a"), V1);
    assert_eq!(fx.last_run_config("a")["approval"]["origin"], "original");
    fx.ok(&["agent", "update", "verbot"]);
    let hand = fx.promote_by_hand("a");

    let check = fx.data(&["app", "check", "a"]);
    assert_eq!(check["approval-current"], true, "{check}");
    assert_eq!(check["approval-origin"], "successor");
    assert_eq!(check["approval-record"], "complete");
    assert_eq!(check["successors"][0]["seq"], 1);
    assert_eq!(check["successors"][0]["by-kind"], "person");
    assert_eq!(check["successors"][0]["attested"], false);
    assert_eq!(check["successors"][0]["from"]["verbot"]["version"], V1);
    assert_eq!(check["successors"][0]["to"]["verbot"]["version"], V2);

    // The run dispatches the TOP-LEVEL (promoted) pins and records the approval.
    assert_eq!(fx.run_says("a"), V2);
    let approval = fx.last_run_config("a")["approval"].clone();
    assert_eq!(approval["origin"], "successor", "{approval}");
    assert_eq!(approval["seq"], 1);
    assert_eq!(approval["by-kind"], "person");
    assert_eq!(approval["attested"], false);
    assert_eq!(approval["record-complete"], true);
    assert_eq!(approval["evidence-digest"], hand.evidence_digest.as_str());
    assert_eq!(approval["to"]["verbot"]["version"], V2);
    let label = approval["label"].as_str().unwrap();
    assert!(
        label.starts_with(
            "approval carried forward from verbot 1.0.0 to 1.1.0 \u{2014} claimed person approval by e2e, recorded by floless@test"
        ),
        "{label}"
    );

    // An archive goes missing: `app check` says incomplete and names it; the
    // run still proceeds, labelled.
    let approvals = fx.aware.join("apps/a/.aware-approvals");
    std::fs::remove_file(approvals.join(format!("{}.lock", hex(&hand.original_digest)))).unwrap();
    let check = fx.data(&["app", "check", "a"]);
    assert_eq!(check["approval-current"], true, "{check}");
    assert_eq!(check["approval-record"], "incomplete");
    assert!(
        check["approval-record-missing"][0]
            .as_str()
            .unwrap()
            .contains(hex(&hand.original_digest)),
        "{check}"
    );
    assert_eq!(fx.run_says("a"), V2);
    let approval = fx.last_run_config("a")["approval"].clone();
    assert_eq!(approval["record-complete"], false, "{approval}");
    assert!(
        approval["label"]
            .as_str()
            .unwrap()
            .contains("the original approval record is missing, provenance cannot be shown")
    );

    // A tampered link is refused before anything runs.
    let lock_path = fx.aware.join("apps/a/a.lock");
    let text = std::fs::read_to_string(&lock_path).unwrap();
    std::fs::write(
        &lock_path,
        text.replacen(
            &hand.resulting_digest,
            &format!("sha256:{}", "0".repeat(64)),
            1,
        ),
    )
    .unwrap();
    let output = fx.aware().args(["app", "run", "a"]).output().unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("E_APP_LOCK_INVALID"), "{stderr}");
    let check = fx.data(&["app", "check", "a"]);
    assert_eq!(check["lock"], "invalid", "{check}");

    // A person's compile writes a fresh original.
    let source = fx.aware.join("apps/a/a.flo");
    fx.ok(&[
        "app",
        "compile",
        source.to_str().unwrap(),
        "--front-door",
        "floless@test",
    ]);
    let text = std::fs::read_to_string(&lock_path).unwrap();
    assert!(
        !text.contains("approval:") && !text.contains("successor-v1:"),
        "{text}"
    );
    assert!(text.contains("front-door: floless@test"), "{text}");
    assert_eq!(
        fx.data(&["app", "check", "a"])["approval-origin"],
        "original"
    );
}

/// Review #628 PR3a round 1: `--simulate` checks the backing app's approval,
/// so its run record carries that approval's origin too — not only a real run.
#[test]
fn a_simulated_run_records_its_backing_app_s_approval_origin() {
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

    for args in [
        &["app", "run", "outer"][..],
        &["app", "run", "outer", "--simulate"],
    ] {
        fx.ok(args);
        let approval = fx.last_run_config("outer")["approval"].clone();
        assert_eq!(approval["origin"], "original", "{args:?}: {approval}");
        let nested = &approval["nested"]["inner"];
        assert_eq!(nested["app"], "inner", "{args:?}: {approval}");
        assert_eq!(nested["origin"], "original", "{args:?}: {approval}");
        assert_eq!(nested["record-complete"], true, "{args:?}: {approval}");
    }
}
