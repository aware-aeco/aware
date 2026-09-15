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
    let directory = root.join("package");
    std::fs::create_dir_all(&directory).unwrap();
    let launcher = b"synthetic provider image";
    std::fs::write(directory.join("provider.bin"), launcher).unwrap();

    let signing = SigningKey::generate(&mut OsRng);
    let public_bytes = signing.verifying_key().to_bytes();
    let public_base64 = base64::engine::general_purpose::STANDARD.encode(public_bytes);
    let publisher_fingerprint = format!("{:x}", Sha256::digest(public_bytes));
    let launcher_sha256 = format!("{:x}", Sha256::digest(launcher));
    let manifest = format!(
        "{{\"capabilities\":[{{\"artifactRootVersion\":\"artifact.synthetic.v1\",\"cacheNamespaceVersion\":\"cache.synthetic.v1\",\"capabilityId\":\"capability.synthetic\",\"protocolVersion\":\"3\",\"requestSchema\":\"request.synthetic.v1\",\"resultSchema\":\"result.synthetic.v1\",\"sourceCaptureMode\":\"capture.synthetic\"}}],\"files\":[{{\"bytes\":{},\"path\":\"provider.bin\",\"sha256\":\"{launcher_sha256}\"}}],\"formatId\":\"format.synthetic\",\"launcher\":\"provider.bin\",\"maximumAwareVersion\":null,\"minimumAwareVersion\":\"0.137.0\",\"packageId\":\"package.synthetic\",\"packageVersion\":\"1.2.3\",\"publisherFingerprintSha256\":\"{publisher_fingerprint}\",\"schemaVersion\":\"aware.model-provider-package/v1\"}}",
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
    let public_key = root.join("publisher.pub");
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
