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

/// #593: a deep `AWARE_HOME` put the replacing store writes past the Win32
/// `MAX_PATH`, where the raw `MoveFileExW` publish failed with `os error 3`.
#[test]
fn replacing_store_writes_succeed_under_an_aware_home_beyond_max_path() {
    let temp = tempfile::tempdir().unwrap();
    let home = temp
        .path()
        .join("h".repeat(100))
        .join("o".repeat(100))
        .join("home");
    assert!(home.as_os_str().len() > 200);
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
    for _ in 0..2 {
        aware(&home)
            .args([
                "provider",
                "select",
                "format.synthetic",
                &fixture.manifest_sha256,
            ])
            .assert()
            .success();
    }
    let policy_file = temp.path().join("dependency-policy.json");
    std::fs::write(
        &policy_file,
        r#"{"policyId":"synthetic-policy-v1","roles":[{"affectedDomains":[],"classification":"mandatory","role":"primary"},{"affectedDomains":[],"classification":"optional","role":"catalogue"}],"schemaVersion":"aware.model-dependency-policy-admission/v1"}"#,
    )
    .unwrap();
    for _ in 0..2 {
        let admitted = aware(&home)
            .args(["--json", "provider", "admit-policy"])
            .arg(&fixture.manifest_sha256)
            .args(["capability.synthetic"])
            .arg(&policy_file)
            .assert()
            .success();
        let output: serde_json::Value =
            serde_json::from_slice(&admitted.get_output().stdout).unwrap();
        let fingerprint = output["data"]["policy"]["providerFingerprintSha256"]
            .as_str()
            .unwrap();
        let stored = home.join(format!("providers/policies/{fingerprint}.json"));
        assert!(stored.as_os_str().len() > 260);
        assert!(stored.is_file());
    }
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

    // Re-verification still refuses to list the drifted package as usable; it reports it as
    // unavailable (keeping its selection visible) instead of failing the whole inventory (#589).
    let listed = aware(&home)
        .args(["--json", "provider", "list", "--format", "format.synthetic"])
        .assert()
        .success();
    let listed_json: serde_json::Value =
        serde_json::from_slice(&listed.get_output().stdout).unwrap();
    assert_eq!(listed_json["data"]["packages"], serde_json::json!([]));
    let unavailable = &listed_json["data"]["unavailable"];
    assert_eq!(unavailable.as_array().unwrap().len(), 1);
    assert_eq!(unavailable[0]["manifestSha256"], fixture.manifest_sha256);
    assert_eq!(unavailable[0]["selected"], true);
    assert_eq!(unavailable[0]["reason"], "verification-failed");
}

#[test]
fn listing_survives_a_removed_unselected_package_directory() {
    // #589: package A enrolled, package B enrolled and selected, A's directory removed.
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("home");
    let removed = package_fixture_named(temp.path(), "package-removed");
    let selected = package_fixture_named(temp.path(), "package-selected");
    for fixture in [&removed, &selected] {
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
            "format.synthetic",
            &selected.manifest_sha256,
        ])
        .assert()
        .success();
    std::fs::remove_dir_all(&removed.directory).unwrap();

    for filter in [Some("format.synthetic"), None] {
        let mut command = aware(&home);
        command.args(["--json", "provider", "list"]);
        if let Some(format) = filter {
            command.args(["--format", format]);
        }
        let listed = command.assert().success();
        let listed_json: serde_json::Value =
            serde_json::from_slice(&listed.get_output().stdout).unwrap();
        let packages = listed_json["data"]["packages"].as_array().unwrap();
        assert_eq!(packages.len(), 1, "{filter:?}");
        assert_eq!(packages[0]["manifestSha256"], selected.manifest_sha256);
        assert_eq!(packages[0]["selected"], true);
        assert_eq!(
            listed_json["data"]["unavailable"],
            serde_json::json!([{
                "manifestSha256": removed.manifest_sha256,
                "packageId": "package.synthetic",
                "packageVersion": "1.2.3",
                "formatId": "format.synthetic",
                "selected": false,
                "reason": "package-missing",
            }]),
            "{filter:?}"
        );
        let listing = String::from_utf8(listed.get_output().stdout.clone()).unwrap();
        assert!(!listing.contains(removed.directory.to_string_lossy().as_ref()));
    }

    let text = aware(&home).args(["provider", "list"]).assert().success();
    let text = String::from_utf8(text.get_output().stdout.clone()).unwrap();
    assert!(
        text.contains(&format!("{}  yes", selected.manifest_sha256)),
        "{text}"
    );
    assert!(
        text.contains(&format!(
            "{}  unavailable (package-missing)",
            removed.manifest_sha256
        )),
        "{text}"
    );

    // The selected package still runs its own full re-verification on select.
    aware(&home)
        .args([
            "provider",
            "select",
            "format.synthetic",
            &removed.manifest_sha256,
        ])
        .assert()
        .failure();
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

fn enrolled_and_selected(temp: &std::path::Path) -> (std::path::PathBuf, PackageFixture) {
    let home = temp.join("home");
    let fixture = package_fixture(temp);
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
    (home, fixture)
}

#[test]
fn listing_reports_a_deleted_receipted_file_as_package_missing() {
    let temp = tempfile::tempdir().unwrap();
    let (home, fixture) = enrolled_and_selected(temp.path());
    std::fs::remove_file(fixture.directory.join("provider.bin")).unwrap();

    let listed = aware(&home)
        .args(["--json", "provider", "list"])
        .assert()
        .success();
    let listed_json: serde_json::Value =
        serde_json::from_slice(&listed.get_output().stdout).unwrap();
    assert_eq!(listed_json["data"]["packages"], serde_json::json!([]));
    assert_eq!(
        listed_json["data"]["unavailable"][0]["reason"],
        "package-missing"
    );
    assert_eq!(listed_json["data"]["unavailable"][0]["selected"], true);
}

#[test]
fn listing_still_fails_on_a_corrupt_enrollment_record() {
    let temp = tempfile::tempdir().unwrap();
    let (home, fixture) = enrolled_and_selected(temp.path());
    let record_path = home.join(format!(
        "providers/packages/{}.json",
        fixture.manifest_sha256
    ));
    let mut record: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&record_path).unwrap()).unwrap();
    let original = record.clone();
    record["schemaVersion"] = serde_json::json!("aware.model-provider-enrollment/v0");
    std::fs::write(&record_path, serde_json::to_vec(&record).unwrap()).unwrap();

    aware(&home)
        .args(["provider", "list"])
        .assert()
        .failure()
        .code(3);

    // A still-valid manifest that no longer hashes to the record's digest is corruption too.
    let mut record = original;
    record["manifest"]["packageVersion"] = serde_json::json!("9.9.9");
    std::fs::write(&record_path, serde_json::to_vec(&record).unwrap()).unwrap();
    aware(&home)
        .args(["provider", "list"])
        .assert()
        .failure()
        .code(3);
}

// ---------------------------------------------------------------------------
// #624 — `list` hashes only what a format would run, and superseded enrollments
// can be retired without ever touching a package directory.
// ---------------------------------------------------------------------------

fn trust_and_enroll(home: &std::path::Path, fixture: &PackageFixture) {
    aware(home)
        .args(["provider", "trust-publisher"])
        .arg(&fixture.public_key)
        .args(["--publisher-id", "publisher.synthetic"])
        .assert()
        .success();
    aware(home)
        .args(["provider", "enroll"])
        .arg(&fixture.directory)
        .assert()
        .success();
}

fn select(home: &std::path::Path, fixture: &PackageFixture) {
    aware(home)
        .args([
            "provider",
            "select",
            "format.synthetic",
            &fixture.manifest_sha256,
        ])
        .assert()
        .success();
}

fn json_of(home: &std::path::Path, args: &[&str]) -> serde_json::Value {
    let output = aware(home).arg("--json").args(args).assert().success();
    let envelope: serde_json::Value = serde_json::from_slice(&output.get_output().stdout).unwrap();
    assert_eq!(envelope["ok"], true, "{envelope}");
    envelope["data"].clone()
}

fn listed(data: &serde_json::Value, digest: &str) -> Option<serde_json::Value> {
    data["packages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|package| package["manifestSha256"] == digest)
        .cloned()
}

fn unavailable_reason(data: &serde_json::Value, digest: &str) -> Option<String> {
    data["unavailable"]
        .as_array()
        .unwrap()
        .iter()
        .find(|package| package["manifestSha256"] == digest)
        .map(|package| package["reason"].as_str().unwrap().to_string())
}

/// Same length, different bytes: only a content hash can tell this file changed.
fn flip_launcher_bytes(fixture: &PackageFixture) {
    let path = fixture.directory.join("provider.bin");
    let mut bytes = std::fs::read(&path).unwrap();
    for byte in &mut bytes {
        *byte ^= 0x20;
    }
    std::fs::write(&path, bytes).unwrap();
}

fn record_path(home: &std::path::Path, digest: &str) -> std::path::PathBuf {
    home.join(format!("providers/packages/{digest}.json"))
}

const SYNTHETIC_ADMISSION: &str = r#"{"policyId":"synthetic-policy-v1","roles":[{"affectedDomains":[],"classification":"mandatory","role":"primary"}],"schemaVersion":"aware.model-dependency-policy-admission/v1"}"#;

/// Admit a policy for `digest` and return the file AWARE stored it in.
fn admit_policy(
    home: &std::path::Path,
    temp: &std::path::Path,
    digest: &str,
) -> std::path::PathBuf {
    let policy = temp.join("dependency-policy.json");
    std::fs::write(&policy, SYNTHETIC_ADMISSION).unwrap();
    let data = json_of(
        home,
        &[
            "provider",
            "admit-policy",
            digest,
            "capability.synthetic",
            policy.to_str().unwrap(),
        ],
    );
    let fingerprint = data["policy"]["providerFingerprintSha256"]
        .as_str()
        .unwrap();
    let stored = home.join(format!("providers/policies/{fingerprint}.json"));
    assert!(stored.is_file());
    stored
}

/// The cost `list` was paying per accumulated build (#624) is the content hash. Proven by
/// behaviour rather than a stopwatch: every unselected package carries same-length content
/// drift, which only a re-hash can see. Before #624 each of them was reported
/// `verification-failed`; now none of them is hashed, while the selected package still is.
#[test]
fn listing_hashes_only_the_selected_package() {
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("home");
    let fixtures = (0..6)
        .map(|index| package_fixture_named(temp.path(), &format!("build-{index}")))
        .collect::<Vec<_>>();
    for fixture in &fixtures {
        trust_and_enroll(&home, fixture);
    }
    let (selected, unselected) = fixtures.split_last().unwrap();
    select(&home, selected);
    for fixture in unselected {
        flip_launcher_bytes(fixture);
    }

    for filter in [Some("format.synthetic"), None] {
        let mut args = vec!["provider", "list"];
        if let Some(format) = filter {
            args.extend(["--format", format]);
        }
        let data = json_of(&home, &args);
        assert_eq!(data["unavailable"], serde_json::json!([]), "{filter:?}");
        assert_eq!(data["packages"].as_array().unwrap().len(), 6, "{filter:?}");
        let active = listed(&data, &selected.manifest_sha256).unwrap();
        assert_eq!(active["selected"], true);
        assert_eq!(active["verification"], "complete");
        for fixture in unselected {
            let package = listed(&data, &fixture.manifest_sha256).unwrap();
            assert_eq!(package["selected"], false);
            assert_eq!(package["verification"], "inventory", "{filter:?}");
        }
    }

    // What `list` no longer hashes, every trust decision still does.
    let drifted = &unselected[0];
    aware(&home)
        .args([
            "provider",
            "select",
            "format.synthetic",
            &drifted.manifest_sha256,
        ])
        .assert()
        .failure()
        .code(3);
    let policy = temp.path().join("dependency-policy.json");
    std::fs::write(&policy, SYNTHETIC_ADMISSION).unwrap();
    aware(&home)
        .args(["provider", "admit-policy", &drifted.manifest_sha256])
        .args(["capability.synthetic"])
        .arg(&policy)
        .assert()
        .failure()
        .code(3);
    let selection: serde_json::Value = serde_json::from_slice(
        &std::fs::read(home.join("providers/selections/format.synthetic.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(selection["activeManifestSha256"], selected.manifest_sha256);

    // And the selected package is still hashed by `list` itself.
    flip_launcher_bytes(selected);
    let data = json_of(&home, &["provider", "list"]);
    assert!(listed(&data, &selected.manifest_sha256).is_none());
    assert_eq!(
        unavailable_reason(&data, &selected.manifest_sha256).as_deref(),
        Some("verification-failed")
    );
}

/// The inventory check skips only the content hash. Everything else that can go wrong with an
/// unselected package without reading its bytes is still caught and reported per package.
#[test]
fn listing_still_reports_metadata_drift_of_unselected_packages() {
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("home");
    let names = [
        "selected",
        "resized",
        "extra-file",
        "missing-file",
        "manifest-replaced",
        "publisher-withdrawn",
        "signature-broken",
    ];
    let fixtures = names
        .iter()
        .map(|name| package_fixture_named(temp.path(), name))
        .collect::<Vec<_>>();
    for fixture in &fixtures {
        trust_and_enroll(&home, fixture);
    }
    select(&home, &fixtures[0]);

    let launcher = |index: usize| fixtures[index].directory.join("provider.bin");
    let mut resized = std::fs::read(launcher(1)).unwrap();
    resized.push(b'!');
    std::fs::write(launcher(1), resized).unwrap();
    std::fs::write(fixtures[2].directory.join("unreceipted.bin"), b"x").unwrap();
    std::fs::remove_file(launcher(3)).unwrap();
    // A host that installs each build into one directory: that root now holds another build.
    let other = package_fixture_named(temp.path(), "other-build");
    for name in ["provider-package.json", "provider-package.sig"] {
        std::fs::copy(other.directory.join(name), fixtures[4].directory.join(name)).unwrap();
    }
    let withdrawn_key = std::fs::read_to_string(&fixtures[5].public_key).unwrap();
    let mut withdrawn = 0;
    for entry in std::fs::read_dir(home.join("providers/publishers")).unwrap() {
        let path = entry.unwrap().path();
        let mut record: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        if withdrawn_key.contains(record["publicKeyBase64"].as_str().unwrap()) {
            record["trusted"] = serde_json::json!(false);
            std::fs::write(&path, serde_json::to_vec(&record).unwrap()).unwrap();
            withdrawn += 1;
        }
    }
    assert_eq!(withdrawn, 1);
    let signature = fixtures[6].directory.join("provider-package.sig");
    let text = std::fs::read_to_string(&signature).unwrap();
    std::fs::write(&signature, text.replace("signature: ", "signature: AAAA")).unwrap();

    let data = json_of(&home, &["provider", "list", "--format", "format.synthetic"]);
    let packages = data["packages"].as_array().unwrap();
    assert_eq!(packages.len(), 1, "{data}");
    assert_eq!(packages[0]["manifestSha256"], fixtures[0].manifest_sha256);
    assert_eq!(packages[0]["verification"], "complete");
    for (index, expected) in [
        (1, "verification-failed"),
        (2, "verification-failed"),
        (3, "package-missing"),
        (4, "verification-failed"),
        (5, "verification-failed"),
        (6, "verification-failed"),
    ] {
        assert_eq!(
            unavailable_reason(&data, &fixtures[index].manifest_sha256).as_deref(),
            Some(expected),
            "{}",
            names[index]
        );
    }
}

#[test]
fn prune_retires_only_superseded_enrollments() {
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("home");

    // Enrolled, never selected, and superseded by every selection made after it.
    let stale = package_fixture_named(temp.path(), "stale");
    trust_and_enroll(&home, &stale);
    let stale_policy = admit_policy(&home, temp.path(), &stale.manifest_sha256);
    // Ten selections: the oldest falls out of the eight-deep rollback history.
    let builds = (0..10)
        .map(|index| package_fixture_named(temp.path(), &format!("build-{index}")))
        .collect::<Vec<_>>();
    for build in &builds {
        trust_and_enroll(&home, build);
        select(&home, build);
    }
    // Enrolled after the current selection: a host about to select it must not lose it.
    let fresh = package_fixture_named(temp.path(), "fresh");
    trust_and_enroll(&home, &fresh);

    let retired_digests = |data: &serde_json::Value| {
        let mut digests = data["retired"]
            .as_array()
            .unwrap()
            .iter()
            .map(|entry| entry["manifestSha256"].as_str().unwrap().to_string())
            .collect::<Vec<_>>();
        digests.sort();
        digests
    };
    let kept_reason = |data: &serde_json::Value, digest: &str| {
        data["kept"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["manifestSha256"] == digest)
            .map(|entry| entry["reason"].as_str().unwrap().to_string())
    };
    let mut expected = vec![
        stale.manifest_sha256.clone(),
        builds[0].manifest_sha256.clone(),
    ];
    expected.sort();

    let dry = json_of(
        &home,
        &[
            "provider",
            "prune",
            "--format",
            "format.synthetic",
            "--dry-run",
        ],
    );
    assert_eq!(dry["dryRun"], true);
    assert_eq!(dry["formatId"], "format.synthetic");
    assert_eq!(retired_digests(&dry), expected, "{dry}");
    assert_eq!(dry["kept"].as_array().unwrap().len(), 10);
    assert_eq!(
        kept_reason(&dry, &builds[9].manifest_sha256).as_deref(),
        Some("selected")
    );
    for build in &builds[1..9] {
        assert_eq!(
            kept_reason(&dry, &build.manifest_sha256).as_deref(),
            Some("rollback-history")
        );
    }
    assert_eq!(
        kept_reason(&dry, &fresh.manifest_sha256).as_deref(),
        Some("not-superseded")
    );
    let stale_entry = dry["retired"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["manifestSha256"] == stale.manifest_sha256)
        .unwrap()
        .clone();
    assert_eq!(
        stale_entry,
        serde_json::json!({
            "manifestSha256": stale.manifest_sha256,
            "packageId": "package.synthetic",
            "packageVersion": "1.2.3",
            "formatId": "format.synthetic",
            "dependencyPolicies": 1,
        })
    );
    // A dry run removes nothing.
    assert!(record_path(&home, &stale.manifest_sha256).is_file());
    assert!(record_path(&home, &builds[0].manifest_sha256).is_file());
    assert!(stale_policy.is_file());

    let pruned = json_of(
        &home,
        &["provider", "prune", "--format", "format.synthetic"],
    );
    assert_eq!(pruned["dryRun"], false);
    assert_eq!(retired_digests(&pruned), expected);
    assert!(!record_path(&home, &stale.manifest_sha256).exists());
    assert!(!record_path(&home, &builds[0].manifest_sha256).exists());
    assert!(
        !stale_policy.exists(),
        "the retired package's policy goes with it"
    );
    // AWARE's record is gone; the package directory is not AWARE's to delete.
    for retired in [&stale, &builds[0]] {
        for name in [
            "provider-package.json",
            "provider-package.sig",
            "provider.bin",
        ] {
            assert!(retired.directory.join(name).is_file(), "{name}");
        }
    }
    // Nothing the selection names was touched, and the store still lists cleanly.
    let data = json_of(&home, &["provider", "list"]);
    assert_eq!(data["packages"].as_array().unwrap().len(), 10);
    assert_eq!(data["unavailable"], serde_json::json!([]));
    assert_eq!(
        listed(&data, &builds[9].manifest_sha256).unwrap()["selected"],
        true
    );
    assert!(listed(&data, &fresh.manifest_sha256).is_some());

    // Pruning again finds nothing more to do.
    let again = json_of(
        &home,
        &["provider", "prune", "--format", "format.synthetic"],
    );
    assert_eq!(again["retired"], serde_json::json!([]));

    // A retired package cannot be selected until it is enrolled again — and then it can be.
    aware(&home)
        .args([
            "provider",
            "select",
            "format.synthetic",
            &stale.manifest_sha256,
        ])
        .assert()
        .failure()
        .code(7);
    aware(&home)
        .args(["provider", "enroll"])
        .arg(&stale.directory)
        .assert()
        .success();
    select(&home, &stale);
}

#[test]
fn prune_without_a_selection_retires_nothing() {
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("home");
    let fixture = package_fixture(temp.path());
    trust_and_enroll(&home, &fixture);
    let data = json_of(
        &home,
        &["provider", "prune", "--format", "format.synthetic"],
    );
    assert_eq!(data["retired"], serde_json::json!([]));
    assert_eq!(data["kept"][0]["reason"], "not-superseded");
    assert!(record_path(&home, &fixture.manifest_sha256).is_file());

    // An empty store prunes cleanly too.
    let empty = temp.path().join("empty-home");
    let data = json_of(
        &empty,
        &["provider", "prune", "--format", "format.synthetic"],
    );
    assert_eq!(data["retired"], serde_json::json!([]));
    assert_eq!(data["kept"], serde_json::json!([]));
}

/// The record timestamps decide what is kept, and a tie keeps the package: a filesystem whose
/// clock is too coarse to order an enrollment against a selection must not lose the enrollment.
#[test]
fn prune_keeps_an_enrollment_it_cannot_order_before_the_selection() {
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("home");
    let unselected = package_fixture_named(temp.path(), "unselected");
    let selected = package_fixture_named(temp.path(), "selected");
    trust_and_enroll(&home, &unselected);
    trust_and_enroll(&home, &selected);
    select(&home, &selected);

    let selected_at = std::fs::metadata(home.join("providers/selections/format.synthetic.json"))
        .unwrap()
        .modified()
        .unwrap();
    let set_enrolled_at = |at: std::time::SystemTime| {
        std::fs::OpenOptions::new()
            .write(true)
            .open(record_path(&home, &unselected.manifest_sha256))
            .unwrap()
            .set_modified(at)
            .unwrap();
    };
    set_enrolled_at(selected_at);
    let data = json_of(
        &home,
        &["provider", "prune", "--format", "format.synthetic"],
    );
    assert_eq!(data["retired"], serde_json::json!([]), "{data}");
    assert!(record_path(&home, &unselected.manifest_sha256).is_file());

    set_enrolled_at(selected_at - std::time::Duration::from_secs(1));
    let data = json_of(
        &home,
        &["provider", "prune", "--format", "format.synthetic"],
    );
    assert_eq!(
        data["retired"][0]["manifestSha256"],
        unselected.manifest_sha256
    );
    assert!(!record_path(&home, &unselected.manifest_sha256).exists());
}

#[test]
fn unenroll_refuses_the_selection_and_its_rollback_history() {
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("home");
    let previous = package_fixture_named(temp.path(), "previous");
    let active = package_fixture_named(temp.path(), "active");
    let spare = package_fixture_named(temp.path(), "spare");
    for fixture in [&previous, &active, &spare] {
        trust_and_enroll(&home, fixture);
    }
    select(&home, &previous);
    select(&home, &active);

    for refused in [&active, &previous] {
        aware(&home)
            .args(["provider", "unenroll", &refused.manifest_sha256])
            .assert()
            .failure()
            .code(8);
        assert!(record_path(&home, &refused.manifest_sha256).is_file());
    }

    let policy = admit_policy(&home, temp.path(), &spare.manifest_sha256);
    let retired = json_of(&home, &["provider", "unenroll", &spare.manifest_sha256]);
    assert_eq!(
        retired,
        serde_json::json!({
            "manifestSha256": spare.manifest_sha256,
            "packageId": "package.synthetic",
            "packageVersion": "1.2.3",
            "formatId": "format.synthetic",
            "dependencyPolicies": 1,
        })
    );
    assert!(!record_path(&home, &spare.manifest_sha256).exists());
    assert!(!policy.exists());
    assert!(spare.directory.join("provider.bin").is_file());
    let data = json_of(&home, &["provider", "list"]);
    assert!(listed(&data, &spare.manifest_sha256).is_none());
    assert_eq!(data["packages"].as_array().unwrap().len(), 2);

    aware(&home)
        .args(["provider", "unenroll", &spare.manifest_sha256])
        .assert()
        .failure()
        .code(7);
    aware(&home)
        .args(["provider", "unenroll", "not-a-digest"])
        .assert()
        .failure()
        .code(3);
}

/// `select` verifies before it takes the selection lock, and the retire verbs remove records under
/// that lock. The selection must re-check the record under the lock, or it could publish a
/// selection naming a record that no longer exists — the state that made #589 fatal.
#[test]
fn selection_never_names_a_record_retired_while_it_waited() {
    use fs2::FileExt;

    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("home");
    let current = package_fixture_named(temp.path(), "current");
    let candidate = package_fixture_named(temp.path(), "candidate");
    trust_and_enroll(&home, &current);
    trust_and_enroll(&home, &candidate);
    select(&home, &current);

    let lock = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(home.join("providers/locks/selection-format.synthetic.lock"))
        .unwrap();
    lock.lock_exclusive().unwrap();
    let mut child = std::process::Command::new(assert_cmd::cargo::cargo_bin("aware"))
        .env("AWARE_HOME", &home)
        .args([
            "provider",
            "select",
            "format.synthetic",
            &candidate.manifest_sha256,
        ])
        .spawn()
        .unwrap();
    // Long enough for the child to finish verifying and block on the lock.
    std::thread::sleep(std::time::Duration::from_millis(500));
    assert!(child.try_wait().unwrap().is_none());
    // What a retire verb does while it holds the lock.
    std::fs::remove_file(record_path(&home, &candidate.manifest_sha256)).unwrap();
    fs2::FileExt::unlock(&lock).unwrap();
    assert_eq!(child.wait().unwrap().code(), Some(7));

    let selection: serde_json::Value = serde_json::from_slice(
        &std::fs::read(home.join("providers/selections/format.synthetic.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(selection["activeManifestSha256"], current.manifest_sha256);
    assert_eq!(selection["generation"], 1);
    json_of(&home, &["provider", "list"]);
}
