//! Process-scoped working copies: no test mutates the test runner's environment.
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;

struct Fixture {
    _tmp: tempfile::TempDir,
    home: PathBuf,
    overlay: PathBuf,
    source: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let tmp = tempfile::tempdir().unwrap();
        let home = tmp.path().join("home");
        let overlay = tmp.path().join("overlay");
        let source = home.join("apps/demo/demo.flo");
        std::fs::create_dir_all(source.parent().unwrap()).unwrap();
        std::fs::create_dir_all(&overlay).unwrap();
        app(&source, "demo", "local-rest", false);
        Self {
            _tmp: tmp,
            home,
            overlay,
            source,
        }
    }

    fn cmd(&self, overlay: bool) -> Command {
        let mut cmd = Command::cargo_bin("aware").unwrap();
        cmd.env("AWARE_HOME", &self.home)
            .env_remove("AWARE_AGENTS_OVERLAY");
        if overlay {
            cmd.env("AWARE_AGENTS_OVERLAY", &self.overlay);
        }
        cmd
    }

    fn compile(&self, overlay: bool) -> Value {
        self.cmd(overlay)
            .args(["app", "compile"])
            .arg(&self.source)
            .assert()
            .success();
        serde_yaml::from_str(&std::fs::read_to_string(self.source.with_extension("lock")).unwrap())
            .unwrap()
    }

    fn check(&self, overlay: bool) -> Value {
        let output = self
            .cmd(overlay)
            .args(["app", "check", "demo", "--json"])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        serde_json::from_slice::<Value>(&output.stdout).unwrap()["data"].clone()
    }

    fn snapshot(&self, lock: &Value) -> PathBuf {
        let digest = lock["agent-digests"]["local-rest"].as_str().unwrap();
        self.home
            .join("agent-store-v2/local-rest")
            .join(digest.strip_prefix("sha256:").unwrap())
            .join("no-receipt")
    }
}

fn rest(dir: &Path, port: u16, auth: bool) {
    std::fs::create_dir_all(dir).unwrap();
    let auth = if auth {
        "auth:\n  scheme: bearer\n  secret: endpoint-bearer\n"
    } else {
        ""
    };
    std::fs::write(dir.join("manifest.yaml"), format!(
        "agent: local-rest\nversion: 0.1.0\ndescription: endpoint {port}\nstateful: false\nlicense: MIT\ntransport:\n  rest:\n    base: http://127.0.0.1:{port}\n{auth}commands:\n  read:\n    lifecycle: single\n    mode: read\n    description: read endpoint\n    method: GET\n    path: /value\n    outputs:\n      type: single\n      schema:\n        value: int\n"
    )).unwrap();
}

fn app(path: &Path, id: &str, agent: &str, exposed: bool) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let exposed = if exposed {
        "exposes-as-agent: true\nexposed-commands:\n  read:\n    lifecycle: single\n    description: read inner\n"
    } else {
        ""
    };
    std::fs::write(path, format!("app: {id}\nversion: 0.1.0\ndescription: overlay fixture\n{exposed}requires:\n  - {agent}@0.1.0\nnodes:\n  - id: value\n    agent: {agent}\n    command: read\nconnections: []\n")).unwrap();
}

fn serve(listener: std::net::TcpListener, calls: usize) -> std::thread::JoinHandle<()> {
    use std::io::{Read, Write};
    listener.set_nonblocking(true).unwrap();
    std::thread::spawn(move || {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(15);
        let mut received = 0;
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    stream.set_nonblocking(false).unwrap();
                    stream
                        .set_read_timeout(Some(std::time::Duration::from_secs(3)))
                        .unwrap();
                    let mut bytes = [0; 4096];
                    let count = stream.read(&mut bytes).unwrap();
                    assert!(String::from_utf8_lossy(&bytes[..count]).contains("GET /value"));
                    let body = r#"{"value":42}"#;
                    write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
                    received += 1;
                    if received == calls {
                        return;
                    }
                }
                Err(e)
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        && std::time::Instant::now() < deadline =>
                {
                    std::thread::sleep(std::time::Duration::from_millis(10))
                }
                Err(e) => panic!("overlay was not called: {e}"),
            }
        }
    })
}

#[test]
fn overlay_discovery_precedes_home_and_labels_list_and_describe() {
    let f = Fixture::new();
    rest(&f.home.join("agents/local-rest"), 31001, false);
    rest(&f.overlay.join("local-rest"), 31002, false);
    let output = f
        .cmd(true)
        .args(["agent", "list", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let json: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(json["data"]["agents"].as_array().unwrap().len(), 1);
    assert_eq!(json["data"]["agents"][0]["source"], "overlay");
    f.cmd(true)
        .args(["agent", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[overlay]"));
    let output = f
        .cmd(true)
        .args(["agent", "describe", "local-rest", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let json: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(json["data"]["source"], "overlay");
    assert_eq!(json["data"]["description"], "endpoint 31002");
    f.cmd(true)
        .args(["agent", "describe", "local-rest"])
        .assert()
        .success()
        .stdout(predicate::str::contains("source:       overlay"));
}

#[test]
fn overlay_compile_pins_and_stores_distinct_endpoint_bytes() {
    let f = Fixture::new();
    rest(&f.home.join("agents/local-rest"), 31001, false);
    rest(&f.overlay.join("local-rest"), 31002, false);
    let home = f.compile(false);
    let local = f.compile(true);
    assert_ne!(home["agent-digests"], local["agent-digests"]);
    assert!(f.snapshot(&home).is_dir());
    assert!(
        std::fs::read_to_string(f.snapshot(&local).join("manifest.yaml"))
            .unwrap()
            .contains("31002")
    );
    // The resolver must prefer the overlay even when a home copy is installed.
    let check = f.check(true);
    assert_eq!(check["agents"][0]["resolution"], "current");
    assert_eq!(check["agents"][0]["source"], "overlay");
    assert_eq!(check["agents"][0]["rest-base"], "http://127.0.0.1:31002");
    let check = f.check(false);
    assert_eq!(check["agents"][0]["source"], "store");
    assert_eq!(check["agents"][0]["rest-base"], "http://127.0.0.1:31002");
}

#[test]
fn overlay_check_reports_only_auth_handle_and_resolved_endpoint() {
    let f = Fixture::new();
    rest(&f.overlay.join("local-rest"), 31002, true);
    f.compile(true);
    let check = f.check(true);
    assert_eq!(check["agents"][0]["rest-base"], "http://127.0.0.1:31002");
    assert_eq!(check["agents"][0]["auth-secret"], "endpoint-bearer");
    assert!(!f.home.join("credentials").exists());
}

#[test]
fn overlay_run_and_invoke_dispatch_the_overlay_endpoint() {
    let f = Fixture::new();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    rest(&f.home.join("agents/local-rest"), 1, false);
    rest(
        &f.overlay.join("local-rest"),
        listener.local_addr().unwrap().port(),
        false,
    );
    f.compile(true);
    let server = serve(listener, 2);
    let result = f.cmd(true).args(["app", "run", "demo"]).output().unwrap();
    let invoked = f
        .cmd(true)
        .args(["agent", "invoke", "local-rest", "read"])
        .output()
        .unwrap();
    server.join().unwrap();
    assert!(
        invoked.status.success(),
        "{}",
        String::from_utf8_lossy(&invoked.stderr)
    );
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(f.home.join("logs/demo").is_dir());
    assert!(!f.overlay.join("logs").exists());
}

#[test]
fn overlay_absent_at_run_keeps_the_missing_working_directory_rule() {
    let f = Fixture::new();
    rest(&f.overlay.join("local-rest"), 31002, false);
    let lock = f.compile(true);
    assert!(f.snapshot(&lock).exists());
    let check = f.check(false);
    assert_eq!(check["agents"][0]["resolution"], "missing");
    f.cmd(false)
        .args(["app", "run", "demo", "--dry-run"])
        .assert()
        .failure();
}

#[test]
fn invalid_overlay_shadows_discovery_and_compile_but_digest_run_uses_store() {
    let f = Fixture::new();
    rest(&f.home.join("agents/local-rest"), 31001, false);
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    rest(&f.overlay.join("local-rest"), port, false);
    let server = serve(listener, 2);
    f.compile(true);
    std::fs::write(
        f.overlay.join("local-rest/manifest.yaml"),
        "commands: [broken",
    )
    .unwrap();
    let output = f
        .cmd(true)
        .args(["agent", "list", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let json: Value = serde_json::from_slice(&output).unwrap();
    assert!(json["data"]["agents"].as_array().unwrap().is_empty());
    assert_eq!(json["data"]["invalid"][0]["id"], "local-rest");
    f.cmd(true)
        .args(["app", "compile"])
        .arg(&f.source)
        .assert()
        .failure();
    let check = f.check(true);
    assert_eq!(check["agents"][0]["source"], "store");
    assert_eq!(
        check["agents"][0]["rest-base"],
        format!("http://127.0.0.1:{port}")
    );
    f.cmd(true).args(["app", "run", "demo"]).assert().success();
    // A missing manifest in a present overlay directory has the same rule.
    std::fs::remove_file(f.overlay.join("local-rest/manifest.yaml")).unwrap();
    f.cmd(true)
        .args(["app", "compile"])
        .arg(&f.source)
        .assert()
        .failure();
    f.cmd(true).args(["app", "run", "demo"]).assert().success();
    server.join().unwrap();
}

#[test]
fn invalid_overlay_refuses_a_legacy_version_only_lock() {
    let f = Fixture::new();
    rest(&f.home.join("agents/local-rest"), 31001, false);
    rest(&f.overlay.join("local-rest"), 31002, false);
    let mut lock = f.compile(true);
    lock.as_object_mut().unwrap().remove("agent-digests");
    std::fs::write(
        f.source.with_extension("lock"),
        serde_yaml::to_string(&lock).unwrap(),
    )
    .unwrap();
    std::fs::write(
        f.overlay.join("local-rest/manifest.yaml"),
        "commands: [broken",
    )
    .unwrap();
    assert_eq!(
        f.check(true)["agents"][0]["resolution"],
        "legacy-pin-mismatch"
    );
    f.cmd(true)
        .args(["app", "run", "demo", "--dry-run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("E_APP_LOCK_AGENT_PIN_MISMATCH"));
}

#[test]
fn overlay_receipt_cannot_satisfy_verified_agent_requirement() {
    let f = Fixture::new();
    rest(&f.overlay.join("local-rest"), 31002, false);
    std::fs::write(f.overlay.join("local-rest/.aware-install.yaml"), "source: registry\nkey: local-rest\nversion: 0.1.0\nmanifest-agent: local-rest\nmanifest-version: 0.1.0\nofficial-source: true\n").unwrap();
    let lock = f.compile(true);
    assert!(!f.snapshot(&lock).join(".aware-install.yaml").exists());
    f.cmd(true)
        .args([
            "app",
            "run",
            "demo",
            "--dry-run",
            "--require-verified-agents",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("E_APP_AGENT_BUNDLE_UNVERIFIED"));
}

#[test]
fn overlay_nested_app_resolves_leaf_endpoint_and_simulation() {
    let f = Fixture::new();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    rest(&f.overlay.join("local-rest"), port, false);
    let inner = f.home.join("apps/inner/inner.flo");
    app(&inner, "inner", "local-rest", true);
    f.cmd(true)
        .args(["app", "compile"])
        .arg(&inner)
        .assert()
        .success();
    let wrapper = f.overlay.join("inner");
    std::fs::create_dir_all(&wrapper).unwrap();
    std::fs::write(wrapper.join("manifest.yaml"), "agent: inner\nversion: 0.1.0\ndescription: wrapper\nstateful: false\nlicense: MIT\ntransport:\n  app:\n    backed-by: inner\ncommands:\n  read:\n    lifecycle: single\n    description: wrapper\n").unwrap();
    app(&f.source, "demo", "inner", false);
    f.compile(true);
    let check = f.check(true);
    let leaf = check["agents"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["agent"] == "local-rest")
        .unwrap();
    assert_eq!(leaf["via"], "inner");
    assert_eq!(leaf["source"], "overlay");
    assert_eq!(leaf["rest-base"], format!("http://127.0.0.1:{port}"));
    let server = serve(listener, 1);
    f.cmd(true).args(["app", "run", "demo"]).assert().success();
    server.join().unwrap();
    f.cmd(true)
        .args(["app", "run", "demo", "--simulate"])
        .assert()
        .success();
    // Only the backing lock changes: check must report its newly resolved bytes.
    rest(&f.overlay.join("local-rest"), 31003, false);
    f.cmd(true)
        .args(["app", "compile"])
        .arg(&inner)
        .assert()
        .success();
    let check = f.check(true);
    let leaf = check["agents"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["agent"] == "local-rest")
        .unwrap();
    assert_eq!(leaf["rest-base"], "http://127.0.0.1:31003");
}

#[test]
fn overlay_empty_or_missing_directory_preserves_home_and_relative_is_rejected() {
    let f = Fixture::new();
    rest(&f.home.join("agents/local-rest"), 31001, false);
    let original = f.compile(false);
    let empty = f.compile(true);
    assert_eq!(original["agent-digests"], empty["agent-digests"]);
    assert_eq!(f.check(true)["agents"][0]["source"], "home");
    std::fs::remove_dir(&f.overlay).unwrap();
    let missing = f.compile(true);
    assert_eq!(original["agent-digests"], missing["agent-digests"]);
    assert_eq!(f.check(true)["agents"][0]["source"], "home");
    let output = f
        .cmd(false)
        .env("AWARE_AGENTS_OVERLAY", "")
        .args(["agent", "list", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let json: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(json["data"]["agents"][0]["source"], "home");
    f.cmd(false)
        .env("AWARE_AGENTS_OVERLAY", "relative")
        .args(["agent", "list"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("must be an absolute directory"));
}

#[test]
fn overlay_rejects_mismatched_ids_and_nonportable_names() {
    let f = Fixture::new();
    rest(&f.home.join("agents/local-rest"), 31001, false);
    rest(&f.overlay.join("local-rest"), 31002, false);
    let path = f.overlay.join("local-rest/manifest.yaml");
    let manifest = std::fs::read_to_string(&path).unwrap();
    std::fs::write(&path, manifest.replace("agent: local-rest", "agent: other")).unwrap();
    f.cmd(true)
        .args(["app", "compile"])
        .arg(&f.source)
        .assert()
        .failure();
    f.cmd(true)
        .args(["agent", "describe", "local-rest"])
        .assert()
        .failure();
    let bad = f.overlay.join("bad id");
    rest(&bad, 31003, false);
    let manifest = std::fs::read_to_string(bad.join("manifest.yaml"))
        .unwrap()
        .replace("agent: local-rest", "agent: bad id");
    std::fs::write(bad.join("manifest.yaml"), manifest).unwrap();
    let output = f
        .cmd(true)
        .args(["agent", "list", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let json: Value = serde_json::from_slice(&output).unwrap();
    assert!(json["data"]["agents"].as_array().unwrap().is_empty());
    assert_eq!(json["data"]["invalid"].as_array().unwrap().len(), 2);
    f.cmd(true)
        .args(["agent", "invoke", "../local-rest", "read"])
        .assert()
        .failure();
}
