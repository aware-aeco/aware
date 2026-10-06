//! `aware agent install <id>@<version> --store-only`, end to end through the
//! real binary (#645): put an approved tool version back into the agent store
//! without making it the installed copy.
//!
//! Same fixture as `agent_store_versions.rs`: agent `verbot` whose 1.0.0 and
//! 1.1.0 manifests point at helpers printing `printed-by-<version>`, so a run's
//! output says exactly which bytes were dispatched.

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

impl Fixture {
    fn fails(&self, args: &[&str]) -> (i32, String) {
        let output = self.aware().args(args).output().unwrap();
        assert!(
            !output.status.success(),
            "aware {args:?} unexpectedly succeeded"
        );
        (
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stderr).into_owned(),
        )
    }

    fn data(&self, args: &[&str]) -> serde_json::Value {
        let mut all = vec!["--json"];
        all.extend_from_slice(args);
        let out = self.ok(&all);
        let envelope: serde_json::Value = serde_json::from_str(out.trim()).unwrap();
        assert_eq!(envelope["ok"], true, "{envelope}");
        envelope["data"].clone()
    }

    fn installed_version(&self) -> String {
        let text = std::fs::read_to_string(self.aware.join("agents/verbot/manifest.yaml")).unwrap();
        text.lines()
            .find_map(|l| l.strip_prefix("version:"))
            .unwrap()
            .trim()
            .to_string()
    }

    /// Every stored verbot package whose record says `version`.
    fn stored_packages(&self, version: &str) -> Vec<PathBuf> {
        let mut out = Vec::new();
        let root = self.aware.join("agent-store-v2/verbot");
        for digest in std::fs::read_dir(&root).into_iter().flatten().flatten() {
            for package in std::fs::read_dir(digest.path())
                .into_iter()
                .flatten()
                .flatten()
            {
                if package.file_name().to_string_lossy().starts_with('.') {
                    continue;
                }
                let record =
                    std::fs::read_to_string(package.path().join(".aware-package.yaml")).unwrap();
                if record.contains(&format!("version: {version}")) {
                    out.push(package.path());
                }
            }
        }
        out
    }

    /// Install 1.0.0, approve app `a` (in `app_dir`) against it, update to
    /// 1.1.0. Returns the app source path.
    fn approved_then_updated(&self, app_dir: &Path) -> PathBuf {
        self.ok(&["agent", "install", "verbot@1.0.0"]);
        std::fs::create_dir_all(app_dir).unwrap();
        let source = app_dir.join("a.flo");
        std::fs::write(
            &source,
            "app: a\nversion: 0.1.0\ndescription: says which verbot ran\n\
             nodes:\n  - id: say\n    agent: verbot\n    command: say\nconnections: []\nrequires: []\n",
        )
        .unwrap();
        self.ok(&["app", "compile", source.to_str().unwrap()]);
        self.ok(&["agent", "update", "verbot"]);
        assert_eq!(self.installed_version(), V2);
        source
    }
}

#[test]
fn a_removed_approved_version_is_put_back_without_changing_the_installed_one() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    fx.approved_then_updated(&fx.aware.join("apps/a"));
    assert_eq!(fx.run_says("a"), V1, "stored 1.0.0 runs before the removal");

    // The issue's repro: the approved package disappears from the store.
    let removed = fx.stored_packages(V1);
    assert_eq!(removed.len(), 1, "{removed:?}");
    std::fs::remove_dir_all(&removed[0]).unwrap();
    let row = agent_row(&fx.check("a"), "verbot");
    assert_eq!(row["resolution"], "pin-not-installed", "{row}");
    let (_, refusal) = fx.fails(&["app", "run", "a"]);
    assert!(
        refusal.contains("E_APP_LOCK_AGENT_PIN_MISMATCH"),
        "{refusal}"
    );

    // A plain install still refuses: the agent is installed.
    let (code, conflict) = fx.fails(&["agent", "install", "verbot@1.0.0"]);
    assert_eq!(code, 8, "{conflict}");

    let out = fx.ok(&["agent", "install", "verbot@1.0.0", "--store-only"]);
    assert!(out.contains("stored verbot 1.0.0"), "{out}");
    assert!(out.contains("the installed copy was not changed"), "{out}");

    assert_eq!(
        fx.installed_version(),
        V2,
        "the installed copy is still 1.1.0"
    );
    let check = fx.check("a");
    let row = agent_row(&check, "verbot");
    assert_eq!(row["resolution"], "stored", "{check}");
    assert_eq!(row["pinned-version"], V1);
    assert_eq!(row["installed-version"], V2);
    assert_eq!(check["approval-current"], true, "{check}");
    assert_eq!(fx.run_says("a"), V1, "the approved 1.0.0 bytes run again");
    assert_eq!(fx.stored_packages(V1), removed, "the very package is back");

    // A compile still resolves the INSTALLED version.
    fx.compiled_app("b");
    assert_eq!(fx.run_says("b"), V2);
}

#[test]
fn a_version_gc_removed_is_put_back_and_kept_for_the_recovery_window() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    // The app's folder is not a root AWARE searches, so GC cannot see its lock.
    let outside = fx.root.join("outside");
    let source = fx.approved_then_updated(&outside);
    let source = source.to_str().unwrap();

    let gc = fx.data(&["agent", "gc", "--apply", "--recovery-window", "0s"]);
    let removed: Vec<&str> = gc["removed"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["version"].as_str().unwrap())
        .collect();
    assert_eq!(removed, vec![V1], "{gc}");
    let row = agent_row(&fx.check(source), "verbot");
    assert_eq!(row["resolution"], "pin-not-installed", "{row}");
    assert_eq!(row["removed"]["by"], "gc", "{row}");

    let stored = fx.data(&["agent", "install", "verbot@1.0.0", "--store-only"]);
    assert_eq!(stored["store-only"], true, "{stored}");
    assert_eq!(stored["agent"], "verbot");
    assert_eq!(stored["version"], V1);
    assert_eq!(stored["registry-key"], "verbot");
    assert_eq!(stored["registry-version"], V1);
    assert_eq!(stored["already-stored"], false);
    assert_eq!(
        stored["official-source"], false,
        "a file registry is not official"
    );
    assert_eq!(stored["stamped"], true);
    assert!(
        stored["digest"].as_str().unwrap().starts_with("sha256:"),
        "{stored}"
    );
    assert_eq!(fx.installed_version(), V2);

    // The tombstone GC left beside it does not hide the package.
    let row = agent_row(&fx.check(source), "verbot");
    assert_eq!(row["resolution"], "stored", "{row}");
    assert!(row.get("removed").is_none_or(|r| r.is_null()), "{row}");
    let refs = fx.data(&["agent", "refs"]);
    assert_eq!(refs["complete"], true, "{refs}");

    // Nothing GC can see pins it, but the fetch restarted its recovery window:
    // a default GC keeps it, as recently used.
    let gc = fx.data(&["agent", "gc", "--apply"]);
    assert!(gc["removed"].as_array().unwrap().is_empty(), "{gc}");
    assert_eq!(fx.stored_packages(V1).len(), 1);
    assert_eq!(
        agent_row(&fx.check(source), "verbot")["resolution"],
        "stored"
    );

    // Fetching again is idempotent.
    let again = fx.data(&["agent", "install", "verbot@1.0.0", "--store-only"]);
    assert_eq!(again["already-stored"], true, "{again}");
    assert_eq!(again["digest"], stored["digest"]);
    assert_eq!(again["path"], stored["path"]);

    // Registered, the folder's lock keeps it with no window at all.
    fx.ok(&["agent", "refs", "roots", "add", outside.to_str().unwrap()]);
    let gc = fx.data(&["agent", "gc", "--apply", "--recovery-window", "0s"]);
    assert!(gc["removed"].as_array().unwrap().is_empty(), "{gc}");
    assert_eq!(
        agent_row(&fx.check(source), "verbot")["resolution"],
        "stored"
    );
}

#[test]
fn store_only_refuses_anything_but_an_exact_registry_release() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    let (code, error) = fx.fails(&["agent", "install", "verbot", "--store-only"]);
    assert_eq!(code, 3, "{error}");
    assert!(
        error.contains("E_AGENT_STORE_ONLY_NEEDS_VERSION"),
        "{error}"
    );

    let folder = fx.root.join("local-verbot");
    std::fs::create_dir_all(&folder).unwrap();
    let (code, error) = fx.fails(&["agent", "install", folder.to_str().unwrap(), "--store-only"]);
    assert_eq!(code, 3, "{error}");
    assert!(error.contains("E_AGENT_STORE_ONLY_REGISTRY"), "{error}");

    let (code, error) = fx.fails(&["agent", "install", "verbot@9.9.9", "--store-only"]);
    assert_eq!(code, 7, "{error}");
    assert!(fx.stored_packages("9.9.9").is_empty());
    assert!(
        !fx.aware.join("agents/verbot").exists(),
        "nothing installed"
    );
}
