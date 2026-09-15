//! End-to-end command coverage for the generic provider package trust boundary.

use assert_cmd::Command;
use base64::Engine;
use ed25519_dalek::{Signer, SigningKey};
use rand_core::OsRng;
use sha2::{Digest, Sha256};

fn aware(home: &std::path::Path) -> Command {
    let mut command = Command::cargo_bin("aware").unwrap();
    command.env("AWARE_HOME", home);
    command
}

struct PackageFixture {
    directory: std::path::PathBuf,
    public_key: std::path::PathBuf,
    manifest_sha256: String,
}

fn package_fixture(root: &std::path::Path) -> PackageFixture {
    package_fixture_named(root, "package")
}

fn package_fixture_named(root: &std::path::Path, name: &str) -> PackageFixture {
    package_fixture_named_format(root, name, "format.synthetic")
}

fn package_fixture_named_format(
    root: &std::path::Path,
    name: &str,
    format_id: &str,
) -> PackageFixture {
    let directory = root.join(name);
    std::fs::create_dir_all(&directory).unwrap();
    let launcher = b"synthetic provider image";
    std::fs::write(directory.join("provider.bin"), launcher).unwrap();

    let signing = SigningKey::generate(&mut OsRng);
    let public_bytes = signing.verifying_key().to_bytes();
    let public_base64 = base64::engine::general_purpose::STANDARD.encode(public_bytes);
    let publisher_fingerprint = format!("{:x}", Sha256::digest(public_bytes));
    let launcher_sha256 = format!("{:x}", Sha256::digest(launcher));
    let manifest = format!(
        "{{\"capabilities\":[{{\"artifactRootVersion\":\"artifact.synthetic.v1\",\"cacheNamespaceVersion\":\"cache.synthetic.v1\",\"capabilityId\":\"capability.synthetic\",\"protocolVersion\":\"3\",\"requestSchema\":\"request.synthetic.v1\",\"resultSchema\":\"result.synthetic.v1\",\"sourceCaptureMode\":\"capture.synthetic\"}}],\"files\":[{{\"bytes\":{},\"path\":\"provider.bin\",\"sha256\":\"{launcher_sha256}\"}}],\"formatId\":\"{format_id}\",\"launcher\":\"provider.bin\",\"maximumAwareVersion\":null,\"minimumAwareVersion\":\"0.137.0\",\"packageId\":\"package.synthetic\",\"packageVersion\":\"1.2.3\",\"publisherFingerprintSha256\":\"{publisher_fingerprint}\",\"schemaVersion\":\"aware.model-provider-package/v1\"}}",
        launcher.len()
    );
    let manifest_sha256 = format!("{:x}", Sha256::digest(manifest.as_bytes()));
    let signature = signing.sign(&Sha256::digest(manifest.as_bytes()));
    std::fs::write(directory.join("provider-package.json"), manifest).unwrap();
    std::fs::write(
        directory.join("provider-package.sig"),
        format!(
            "ed25519-signature-v1\nover-sha256-of: provider-package.json\nsha256: {manifest_sha256}\nsignature: {}\npublic-key: {public_base64}\n",
            base64::engine::general_purpose::STANDARD.encode(signature.to_bytes())
        ),
    )
    .unwrap();
    let public_key = root.join(format!("{name}.pub"));
    std::fs::write(
        &public_key,
        format!("ed25519-public-key-v1 {public_base64}\n"),
    )
    .unwrap();
    PackageFixture {
        directory,
        public_key,
        manifest_sha256,
    }
}

#[test]
fn concurrent_selections_serialize_generation_and_history_updates() {
    use fs2::FileExt;

    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("home");
    let first = package_fixture_named(temp.path(), "package-first");
    let second = package_fixture_named(temp.path(), "package-second");
    for fixture in [&first, &second] {
        aware(&home)
            .args(["provider", "trust-publisher"])
            .arg(&fixture.public_key)
            .args(["--publisher-id", "publisher.synthetic"])
            .assert()
            .success();
        aware(&home)
            .args(["provider", "enroll"])
            .arg(&fixture.directory)
            .assert()
            .success();
    }

    let lock_directory = home.join("providers/locks");
    std::fs::create_dir_all(&lock_directory).unwrap();
    let lock = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(lock_directory.join("selection-format.synthetic.lock"))
        .unwrap();
    lock.lock_exclusive().unwrap();

    let executable = assert_cmd::cargo::cargo_bin("aware");
    let mut first_child = std::process::Command::new(&executable)
        .env("AWARE_HOME", &home)
        .args([
            "provider",
            "select",
            "format.synthetic",
            &first.manifest_sha256,
        ])
        .spawn()
        .unwrap();
    let mut second_child = std::process::Command::new(executable)
        .env("AWARE_HOME", &home)
        .args([
            "provider",
            "select",
            "format.synthetic",
            &second.manifest_sha256,
        ])
        .spawn()
        .unwrap();

    std::thread::sleep(std::time::Duration::from_millis(100));
    assert!(first_child.try_wait().unwrap().is_none());
    assert!(second_child.try_wait().unwrap().is_none());
    fs2::FileExt::unlock(&lock).unwrap();
    assert!(first_child.wait().unwrap().success());
    assert!(second_child.wait().unwrap().success());

    let selection: serde_json::Value = serde_json::from_slice(
        &std::fs::read(home.join("providers/selections/format.synthetic.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(selection["generation"], 2);
    let active = selection["activeManifestSha256"].as_str().unwrap();
    let previous = selection["previousManifestSha256"].as_array().unwrap();
    assert_eq!(previous.len(), 1);
    assert!(
        (active == first.manifest_sha256 && previous[0] == second.manifest_sha256)
            || (active == second.manifest_sha256 && previous[0] == first.manifest_sha256)
    );
}

#[test]
fn trust_enroll_select_and_list_are_closed_and_format_neutral() {
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("home");
    let fixture = package_fixture(temp.path());

    aware(&home)
        .args(["provider", "trust-publisher"])
        .arg(&fixture.public_key)
        .args(["--publisher-id", "publisher.synthetic"])
        .assert()
        .success();

    let enrolled = aware(&home)
        .args(["--json", "provider", "enroll"])
        .arg(&fixture.directory)
        .assert()
        .success();
    let enrolled_json: serde_json::Value =
        serde_json::from_slice(&enrolled.get_output().stdout).unwrap();
    assert_eq!(
        enrolled_json["data"]["manifestSha256"],
        fixture.manifest_sha256
    );
    assert_eq!(enrolled_json["data"]["formatId"], "format.synthetic");
    assert!(enrolled_json["data"].get("packageRoot").is_none());
    assert!(enrolled_json["data"].get("launcher").is_none());
    assert!(enrolled_json["data"].get("files").is_none());

    aware(&home)
        .args([
            "provider",
            "select",
            "format.synthetic",
            &fixture.manifest_sha256,
        ])
        .assert()
        .success();

    let listed = aware(&home)
        .args(["--json", "provider", "list", "--format", "format.synthetic"])
        .assert()
        .success();
    let listed_json: serde_json::Value =
        serde_json::from_slice(&listed.get_output().stdout).unwrap();
    assert_eq!(listed_json["data"]["packages"][0]["selected"], true);
    assert_eq!(
        listed_json["data"]["packages"][0]["capabilities"][0]["capabilityId"],
        "capability.synthetic"
    );
    let listing = String::from_utf8(listed.get_output().stdout.clone()).unwrap();
    assert!(!listing.contains(fixture.directory.to_string_lossy().as_ref()));
    assert!(!listing.contains("provider.bin"));
}

#[test]
fn operator_admits_a_closed_dependency_policy_for_the_exact_enrolled_capability() {
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("home");
    let fixture = package_fixture(temp.path());
    aware(&home)
        .args(["provider", "trust-publisher"])
        .arg(&fixture.public_key)
        .args(["--publisher-id", "publisher.synthetic"])
        .assert()
        .success();
    aware(&home)
        .args(["provider", "enroll"])
        .arg(&fixture.directory)
        .assert()
        .success();

    let policy_file = temp.path().join("dependency-policy.json");
    std::fs::write(
        &policy_file,
        r#"{"policyId":"synthetic-policy-v1","roles":[{"affectedDomains":[],"classification":"mandatory","role":"primary"},{"affectedDomains":["geometry","properties"],"classification":"degraded","role":"catalogue"}],"schemaVersion":"aware.model-dependency-policy-admission/v1"}"#,
    )
    .unwrap();
    let admitted = aware(&home)
        .args(["--json", "provider", "admit-policy"])
        .arg(&fixture.manifest_sha256)
        .args(["capability.synthetic"])
        .arg(&policy_file)
        .assert()
        .success();
    let output: serde_json::Value = serde_json::from_slice(&admitted.get_output().stdout).unwrap();
    let fingerprint = output["data"]["policy"]["providerFingerprintSha256"]
        .as_str()
        .unwrap();
    assert_eq!(fingerprint.len(), 64);
    assert_eq!(
        output["data"]["policy"]["capabilityId"],
        "capability.synthetic"
    );
    assert_eq!(output["data"]["sha256"].as_str().unwrap().len(), 64);
    let stored: serde_json::Value = serde_json::from_slice(
        &std::fs::read(home.join(format!("providers/policies/{fingerprint}.json"))).unwrap(),
    )
    .unwrap();
    assert_eq!(stored, output["data"]["policy"]);

    std::fs::write(
        &policy_file,
        r#"{"policyId":"synthetic-policy-v2","roles":[{"affectedDomains":[],"classification":"mandatory","role":"primary"},{"affectedDomains":[],"classification":"optional","role":"catalogue"}],"schemaVersion":"aware.model-dependency-policy-admission/v1"}"#,
    )
    .unwrap();
    let updated = aware(&home)
        .args(["--json", "provider", "admit-policy"])
        .arg(&fixture.manifest_sha256)
        .args(["capability.synthetic"])
        .arg(&policy_file)
        .assert()
        .success();
    let updated_output: serde_json::Value =
        serde_json::from_slice(&updated.get_output().stdout).unwrap();
    assert_eq!(
        updated_output["data"]["policy"]["policyId"],
        "synthetic-policy-v2"
    );
    assert_ne!(updated_output["data"]["sha256"], output["data"]["sha256"]);

    let invalid = temp.path().join("invalid-policy.json");
    std::fs::write(
        &invalid,
        r#"{"policyId":"bad-policy","roles":[{"affectedDomains":[],"classification":"degraded","role":"catalogue"}],"schemaVersion":"aware.model-dependency-policy-admission/v1"}"#,
    )
    .unwrap();
    aware(&home)
        .args(["provider", "admit-policy"])
        .arg(&fixture.manifest_sha256)
        .args(["capability.synthetic"])
        .arg(&invalid)
        .assert()
        .failure();
}

#[test]
fn listing_ignores_interrupted_atomic_write_scratch_files() {
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("home");
    let fixture = package_fixture(temp.path());
    aware(&home)
        .args(["provider", "trust-publisher"])
        .arg(&fixture.public_key)
        .args(["--publisher-id", "publisher.synthetic"])
        .assert()
        .success();
    aware(&home)
        .args(["provider", "enroll"])
        .arg(&fixture.directory)
        .assert()
        .success();
    aware(&home)
        .args([
            "provider",
            "select",
            "format.synthetic",
            &fixture.manifest_sha256,
        ])
        .assert()
        .success();

    std::fs::write(
        home.join("providers/selections/.tmp-interrupted-selection"),
        b"partial",
    )
    .unwrap();
    std::fs::write(
        home.join("providers/packages/.tmp-interrupted-package"),
        b"partial",
    )
    .unwrap();

    let listed = aware(&home)
        .args(["--json", "provider", "list", "--format", "format.synthetic"])
        .assert()
        .success();
    let listed_json: serde_json::Value =
        serde_json::from_slice(&listed.get_output().stdout).unwrap();
    assert_eq!(listed_json["data"]["packages"][0]["selected"], true);
}

#[test]
fn listing_reverifies_enrolled_package_contents() {
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("home");
    let fixture = package_fixture(temp.path());
    aware(&home)
        .args(["provider", "trust-publisher"])
        .arg(&fixture.public_key)
        .args(["--publisher-id", "publisher.synthetic"])
        .assert()
        .success();
    aware(&home)
        .args(["provider", "enroll"])
        .arg(&fixture.directory)
        .assert()
        .success();
    aware(&home)
        .args([
            "provider",
            "select",
            "format.synthetic",
            &fixture.manifest_sha256,
        ])
        .assert()
        .success();

    std::fs::write(fixture.directory.join("provider.bin"), b"changed").unwrap();

    aware(&home)
        .args(["provider", "list", "--format", "format.synthetic"])
        .assert()
        .failure()
        .code(3);
}

#[test]
fn filtered_listing_ignores_drift_in_an_unrelated_format() {
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("home");
    let requested = package_fixture_named_format(temp.path(), "requested", "format.requested");
    let unrelated = package_fixture_named_format(temp.path(), "unrelated", "format.unrelated");
    for fixture in [&requested, &unrelated] {
        aware(&home)
            .args(["provider", "trust-publisher"])
            .arg(&fixture.public_key)
            .args(["--publisher-id", "publisher.synthetic"])
            .assert()
            .success();
        aware(&home)
            .args(["provider", "enroll"])
            .arg(&fixture.directory)
            .assert()
            .success();
    }
    aware(&home)
        .args([
            "provider",
            "select",
            "format.requested",
            &requested.manifest_sha256,
        ])
        .assert()
        .success();
    std::fs::write(
        home.join("providers/selections/format.unrelated.json"),
        b"malformed unrelated selection",
    )
    .unwrap();
    std::fs::write(unrelated.directory.join("provider.bin"), b"changed").unwrap();

    let listed = aware(&home)
        .args(["--json", "provider", "list", "--format", "format.requested"])
        .assert()
        .success();
    let listed_json: serde_json::Value =
        serde_json::from_slice(&listed.get_output().stdout).unwrap();
    assert_eq!(listed_json["data"]["packages"].as_array().unwrap().len(), 1);
    assert_eq!(
        listed_json["data"]["packages"][0]["formatId"],
        "format.requested"
    );
    assert_eq!(listed_json["data"]["packages"][0]["selected"], true);
}

#[test]
fn listing_refuses_a_selection_stored_under_the_wrong_format_filename() {
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("home");
    let fixture = package_fixture(temp.path());
    aware(&home)
        .args(["provider", "trust-publisher"])
        .arg(&fixture.public_key)
        .args(["--publisher-id", "publisher.synthetic"])
        .assert()
        .success();
    aware(&home)
        .args(["provider", "enroll"])
        .arg(&fixture.directory)
        .assert()
        .success();
    aware(&home)
        .args([
            "provider",
            "select",
            "format.synthetic",
            &fixture.manifest_sha256,
        ])
        .assert()
        .success();

    std::fs::rename(
        home.join("providers/selections/format.synthetic.json"),
        home.join("providers/selections/format.other.json"),
    )
    .unwrap();

    aware(&home)
        .args(["provider", "list"])
        .assert()
        .failure()
        .code(3);
}

#[test]
fn enrollment_refuses_an_extra_unreceipted_file() {
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("home");
    let fixture = package_fixture(temp.path());
    aware(&home)
        .args(["provider", "trust-publisher"])
        .arg(&fixture.public_key)
        .args(["--publisher-id", "publisher.synthetic"])
        .assert()
        .success();
    std::fs::write(fixture.directory.join("unlisted.bin"), b"unlisted").unwrap();
    aware(&home)
        .args(["provider", "enroll"])
        .arg(&fixture.directory)
        .assert()
        .failure()
        .code(3);
    assert!(!home.join("providers/packages").exists());
}

#[test]
fn selection_refuses_a_package_under_a_different_format() {
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("home");
    let fixture = package_fixture(temp.path());
    aware(&home)
        .args(["provider", "trust-publisher"])
        .arg(&fixture.public_key)
        .args(["--publisher-id", "publisher.synthetic"])
        .assert()
        .success();
    aware(&home)
        .args(["provider", "enroll"])
        .arg(&fixture.directory)
        .assert()
        .success();
    aware(&home)
        .args([
            "provider",
            "select",
            "format.other",
            &fixture.manifest_sha256,
        ])
        .assert()
        .failure()
        .code(3);
}

#[test]
fn selection_reverifies_the_enrolled_package_before_publishing_it() {
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("home");
    let fixture = package_fixture(temp.path());
    aware(&home)
        .args(["provider", "trust-publisher"])
        .arg(&fixture.public_key)
        .args(["--publisher-id", "publisher.synthetic"])
        .assert()
        .success();
    aware(&home)
        .args(["provider", "enroll"])
        .arg(&fixture.directory)
        .assert()
        .success();
    std::fs::write(fixture.directory.join("provider.bin"), b"changed").unwrap();

    aware(&home)
        .args([
            "provider",
            "select",
            "format.synthetic",
            &fixture.manifest_sha256,
        ])
        .assert()
        .failure()
        .code(3);
    assert!(
        !home
            .join("providers/selections/format.synthetic.json")
            .exists()
    );
}

#[test]
fn selection_refuses_an_unbounded_existing_history() {
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("home");
    let fixture = package_fixture(temp.path());
    aware(&home)
        .args(["provider", "trust-publisher"])
        .arg(&fixture.public_key)
        .args(["--publisher-id", "publisher.synthetic"])
        .assert()
        .success();
    aware(&home)
        .args(["provider", "enroll"])
        .arg(&fixture.directory)
        .assert()
        .success();
    aware(&home)
        .args([
            "provider",
            "select",
            "format.synthetic",
            &fixture.manifest_sha256,
        ])
        .assert()
        .success();

    let selection_path = home.join("providers/selections/format.synthetic.json");
    let mut selection: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&selection_path).unwrap()).unwrap();
    selection["previousManifestSha256"] = serde_json::Value::Array(
        (0..9)
            .map(|index| serde_json::Value::String(format!("{index:064x}")))
            .collect(),
    );
    std::fs::write(&selection_path, serde_json::to_vec(&selection).unwrap()).unwrap();

    aware(&home)
        .args([
            "provider",
            "select",
            "format.synthetic",
            &fixture.manifest_sha256,
        ])
        .assert()
        .failure()
        .code(3);
}
