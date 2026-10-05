//! Atomic agent swaps, end to end through the real binary (#627).
//!
//! `aware agent update` is killed in the middle of its swap — after it moved
//! the old copy aside and before it moved the new one in, and again after the
//! new one is in but before it committed — and the test checks what a person
//! would see: `agents/verbot` is never a partial tree, the next command
//! (a run, `aware doctor`) finishes or rolls the swap back on its own, and the
//! approved app runs on its approved bytes throughout or refuses cleanly —
//! never a raw IO error.
//!
//! The kill is made deterministic with `AWARE_TEST_SWAP_PAUSE_AFTER`, a hook
//! that exists only in debug builds (the binary `cargo test` runs): it pauses
//! the swap for two minutes right after it writes the named journal line, and
//! the test kills the process there — exactly a crash at that point.

use assert_cmd::Command;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const V1: &str = "1.0.0";
const V2: &str = "1.1.0";

struct Fixture {
    _tmp: tempfile::TempDir,
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

/// A tarball holding the manifest plus enough skill files that a partial
/// tree would be visible.
fn tarball(path: &Path, manifest: &str, version: &str) {
    let enc = flate2::write::GzEncoder::new(
        std::fs::File::create(path).unwrap(),
        flate2::Compression::default(),
    );
    let mut tar = tar::Builder::new(enc);
    let mut files = vec![("manifest.yaml".to_string(), manifest.as_bytes().to_vec())];
    for n in 0..20 {
        files.push((
            format!("skills/s{n}.md"),
            format!(
                "skill {n} of verbot {version}
"
            )
            .into_bytes(),
        ));
    }
    for (name, body) in files {
        let mut header = tar::Header::new_gnu();
        header.set_size(body.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        tar.append_data(
            &mut header,
            format!("aware-main/20-agents/verbot/{name}"),
            body.as_slice(),
        )
        .unwrap();
    }
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
    tarball(&t1, &manifest(V1, &b1), V1);
    tarball(&t2, &manifest(V2, &b2), V2);
    let index = root.join("registry-index.json");
    std::fs::write(
        &index,
        format!(
            r#"{{
    "version": "1.0",
    "updated-at": "2026-10-05T00:00:00Z",
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
    Some(Fixture {
        aware: root.join("aware"),
        registry: file_url(&index),
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

    /// Run app `id`; return the build that printed its output.
    fn run_says(&self, id: &str) -> String {
        self.ok(&["app", "run", id]);
        printed_by(&self.aware.join("logs").join(id))
    }

    /// Start `aware agent update verbot` that pauses right after `line`, and
    /// return it once it has paused there.
    fn update_paused_after(&self, line: &str) -> std::process::Child {
        let mut child = std::process::Command::new(assert_cmd::cargo::cargo_bin("aware"))
            .args(["agent", "update", "verbot"])
            .env("AWARE_HOME", &self.aware)
            .env("AWARE_REGISTRY", &self.registry)
            .env("AWARE_TEST_SWAP_PAUSE_AFTER", line)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .unwrap();
        let deadline = Instant::now() + Duration::from_secs(90);
        loop {
            if journal_lines(&self.aware).iter().any(|l| l == line) {
                // The line is fsynced before the pause; give the hook a beat.
                std::thread::sleep(Duration::from_millis(200));
                return child;
            }
            if let Ok(Some(status)) = child.try_wait() {
                panic!("the update exited ({status}) before reaching {line:?}");
            }
            assert!(
                Instant::now() < deadline,
                "the update never reached {line:?}"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    fn agent_tree(&self) -> Option<Vec<(String, Vec<u8>)>> {
        tree(&self.aware.join("agents").join("verbot"))
    }
}

/// Every journal line of every swap transaction under `agents/.aware-swap`.
fn journal_lines(aware: &Path) -> Vec<String> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(aware.join("agents").join(".aware-swap"))
        .into_iter()
        .flatten()
        .flatten()
    {
        if let Ok(text) = std::fs::read_to_string(entry.path().join("journal.log")) {
            out.extend(text.lines().map(str::to_string));
        }
    }
    out
}

fn swap_transactions(aware: &Path) -> usize {
    std::fs::read_dir(aware.join("agents").join(".aware-swap"))
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.file_name() != "locks")
        .count()
}

/// Sorted (relative path, bytes) of every file under `dir`, or `None` if absent.
fn tree(dir: &Path) -> Option<Vec<(String, Vec<u8>)>> {
    if !dir.exists() {
        return None;
    }
    let mut out = Vec::new();
    fn walk(base: &Path, dir: &Path, out: &mut Vec<(String, Vec<u8>)>) {
        for entry in std::fs::read_dir(dir).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(base, &path, out);
            } else {
                let rel = path
                    .strip_prefix(base)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/");
                out.push((rel, std::fs::read(&path).unwrap()));
            }
        }
    }
    walk(dir, dir, &mut out);
    out.sort();
    Some(out)
}

fn manifest_version(tree: &[(String, Vec<u8>)]) -> String {
    let manifest = tree
        .iter()
        .find(|(name, _)| name == "manifest.yaml")
        .map(|(_, bytes)| String::from_utf8_lossy(bytes).into_owned())
        .expect("a complete tree has its manifest");
    if manifest.contains("version: 1.1.0") {
        V2.into()
    } else {
        V1.into()
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
    assert_eq!(builds.len(), 1, "expected one build in {body}");
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

/// Killed after the old copy moved aside, before the new one moved in: the
/// directory is ABSENT (never partial); a run started while the update was
/// still alive waits for it, and once the update is killed the run recovers
/// the swap itself (rolling it back), restores the old copy byte for byte, and
/// runs the approved 1.0.0 bytes.
#[test]
fn an_update_killed_mid_swap_rolls_back_and_the_run_recovers_it() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    fx.ok(&["agent", "install", "verbot@1.0.0"]);
    fx.compiled_app("a");
    assert_eq!(fx.run_says("a"), V1);
    let before = fx.agent_tree().unwrap();

    let mut update = fx.update_paused_after("done out verbot");
    assert!(
        fx.agent_tree().is_none(),
        "mid-swap the unlocked view is absent, never a partial tree"
    );

    // A run started now blocks on the swap lock rather than reading a vanished
    // or half-copied tree.
    let run = std::process::Command::new(assert_cmd::cargo::cargo_bin("aware"))
        .args(["app", "run", "a"])
        .env("AWARE_HOME", &fx.aware)
        .env("AWARE_REGISTRY", &fx.registry)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    std::thread::sleep(Duration::from_millis(800));

    update.kill().unwrap();
    update.wait().unwrap();

    let output = run.wait_with_output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "the run must recover and run, not fail: {stderr}"
    );
    assert_eq!(printed_by(&fx.aware.join("logs").join("a")), V1);
    assert_eq!(
        fx.agent_tree().unwrap(),
        before,
        "rolled back: the old copy is back, byte for byte"
    );
    assert_eq!(swap_transactions(&fx.aware), 0, "nothing left behind");
    assert!(!stderr.contains("os error"), "{stderr}");
}

/// Killed after the new copy moved in but before the commit: the directory is
/// the complete NEW tree; `aware doctor` finishes the swap (deleting the old
/// copy it had moved aside), and the approved app still runs its approved
/// 1.0.0 bytes from the store.
#[test]
fn an_update_killed_after_the_move_in_commits_on_recovery() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    fx.ok(&["agent", "install", "verbot@1.0.0"]);
    fx.compiled_app("a");

    let mut update = fx.update_paused_after("done in verbot");
    update.kill().unwrap();
    update.wait().unwrap();

    let now = fx.agent_tree().expect("the new copy is in place");
    assert_eq!(manifest_version(&now), V2);
    assert_eq!(
        now.iter()
            .filter(|(name, _)| name.starts_with("skills/"))
            .count(),
        20,
        "the new copy is complete"
    );
    assert_eq!(swap_transactions(&fx.aware), 1, "the swap awaits recovery");

    let doctor: serde_json::Value = serde_json::from_str(&fx.ok(&["--json", "doctor"])).unwrap();
    let swaps = &doctor["agent_swaps"];
    assert_eq!(swaps["ok"], true, "{swaps}");
    assert_eq!(swaps["transactions"][0]["outcome"], "recovered", "{swaps}");
    assert_eq!(swap_transactions(&fx.aware), 0);
    assert_eq!(manifest_version(&fx.agent_tree().unwrap()), V2);

    // The approved app keeps its approved bytes.
    assert_eq!(fx.run_says("a"), V1);
}
