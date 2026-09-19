//! Closed, format-neutral trust store for signed model-provider packages.

use std::collections::{BTreeMap, BTreeSet};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};

use base64::Engine;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use fs2::FileExt;
use semver::Version;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::AwareError;

const MANIFEST_NAME: &str = "provider-package.json";
const SIGNATURE_NAME: &str = "provider-package.sig";
const MANIFEST_SCHEMA: &str = "aware.model-provider-package/v1";
const PUBLISHER_SCHEMA: &str = "aware.model-provider-publisher/v1";
const PACKAGE_SCHEMA: &str = "aware.model-provider-enrollment/v1";
const SELECTION_SCHEMA: &str = "aware.model-provider-selection/v1";
const POLICY_ADMISSION_SCHEMA: &str = "aware.model-dependency-policy-admission/v1";
const POLICY_SCHEMA: &str = "aware.model-dependency-policy/v1";
const MAX_CONTROL_BYTES: u64 = 1024 * 1024;
const MAX_VERSION_LENGTH: usize = 128;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PackageManifest {
    pub schema_version: String,
    pub package_id: String,
    pub package_version: String,
    pub format_id: String,
    pub launcher: String,
    pub minimum_aware_version: String,
    pub maximum_aware_version: Option<String>,
    pub publisher_fingerprint_sha256: String,
    pub capabilities: Vec<ProviderCapability>,
    pub files: Vec<PackageFile>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ProviderCapability {
    pub capability_id: String,
    pub protocol_version: String,
    pub source_capture_mode: String,
    pub request_schema: String,
    pub result_schema: String,
    pub artifact_root_version: String,
    pub cache_namespace_version: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct DependencyPolicyAdmission {
    schema_version: String,
    policy_id: String,
    roles: Vec<DependencyPolicyRole>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct DependencyPolicyRole {
    role: String,
    classification: String,
    affected_domains: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct DependencyPolicy {
    schema_version: String,
    pub(crate) policy_id: String,
    capability_id: String,
    provider_fingerprint_sha256: String,
    roles: Vec<DependencyPolicyRole>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AdmittedDependencyPolicy {
    pub(crate) policy: DependencyPolicy,
    pub(crate) sha256: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EnrolledProviderFingerprint<'a> {
    schema_version: &'static str,
    execution: &'static str,
    package_manifest_sha256: &'a str,
    launcher_sha256: &'a str,
    publisher_fingerprint_sha256: &'a str,
    format_id: &'a str,
    capability: &'a ProviderCapability,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PackageFile {
    pub path: String,
    pub bytes: u64,
    pub sha256: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PublisherRecord {
    pub schema_version: String,
    pub publisher_id: String,
    pub key_fingerprint_sha256: String,
    pub public_key_base64: String,
    pub trusted: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct PackageRecord {
    pub schema_version: String,
    pub manifest_sha256: String,
    pub package_root: String,
    pub publisher_fingerprint_sha256: String,
    pub manifest: PackageManifest,
    pub enrolled: bool,
    pub revoked: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PackagePublicView<'a> {
    manifest_sha256: &'a str,
    package_id: &'a str,
    package_version: &'a str,
    format_id: &'a str,
    publisher_fingerprint_sha256: &'a str,
    capabilities: &'a [ProviderCapability],
}

impl PackageRecord {
    pub(crate) fn public_view(&self) -> PackagePublicView<'_> {
        PackagePublicView {
            manifest_sha256: &self.manifest_sha256,
            package_id: &self.manifest.package_id,
            package_version: &self.manifest.package_version,
            format_id: &self.manifest.format_id,
            publisher_fingerprint_sha256: &self.publisher_fingerprint_sha256,
            capabilities: &self.manifest.capabilities,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct SelectionRecord {
    pub schema_version: String,
    pub format_id: String,
    pub generation: u64,
    pub active_manifest_sha256: String,
    pub previous_manifest_sha256: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProviderList {
    pub packages: Vec<ListedPackage>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListedPackage {
    pub(crate) manifest_sha256: String,
    pub(crate) package_id: String,
    pub(crate) package_version: String,
    pub(crate) format_id: String,
    publisher_fingerprint_sha256: String,
    capabilities: Vec<ProviderCapability>,
    pub(crate) selected: bool,
}

pub(crate) struct ProviderStore {
    root: PathBuf,
}

impl ProviderStore {
    pub(crate) fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub(crate) fn trust_publisher(
        &self,
        public_key_path: &Path,
        publisher_id: &str,
    ) -> Result<PublisherRecord, AwareError> {
        validate_id(publisher_id, "publisher id")?;
        let public_key = crate::receipt::load_verifying_key(public_key_path)?;
        let public_bytes = public_key.to_bytes();
        let fingerprint = sha256_hex(&public_bytes);
        let record = PublisherRecord {
            schema_version: PUBLISHER_SCHEMA.into(),
            publisher_id: publisher_id.into(),
            key_fingerprint_sha256: fingerprint.clone(),
            public_key_base64: base64::engine::general_purpose::STANDARD.encode(public_bytes),
            trusted: true,
        };
        write_new_json(
            &self
                .root
                .join("publishers")
                .join(format!("{fingerprint}.json")),
            &record,
        )?;
        Ok(record)
    }

    pub(crate) fn enroll(&self, directory: &Path) -> Result<PackageRecord, AwareError> {
        let root = canonical_regular_directory(directory)?;
        let manifest_bytes = read_bounded(&root.join(MANIFEST_NAME), MAX_CONTROL_BYTES)?;
        let manifest: PackageManifest =
            serde_json::from_slice(&manifest_bytes).map_err(|error| {
                AwareError::Validation(format!("invalid provider package manifest: {error}"))
            })?;
        let canonical = canonical_json_bytes(&manifest)?;
        if canonical != manifest_bytes {
            return Err(AwareError::Validation(
                "provider package manifest must use its closed canonical JSON encoding".into(),
            ));
        }
        validate_manifest(&manifest)?;
        verify_compatible(&manifest)?;
        let manifest_sha256 = sha256_hex(&manifest_bytes);
        let publisher = self.read_publisher(&manifest.publisher_fingerprint_sha256)?;
        if !publisher.trusted {
            return Err(AwareError::PermissionDenied(
                "provider package publisher is not trusted".into(),
            ));
        }
        verify_package_signature(&root, &manifest_sha256, &publisher)?;
        verify_package_inventory(&root, &manifest)?;
        let record = PackageRecord {
            schema_version: PACKAGE_SCHEMA.into(),
            manifest_sha256: manifest_sha256.clone(),
            package_root: root
                .to_str()
                .ok_or_else(|| {
                    AwareError::Validation("provider package root must be valid Unicode".into())
                })?
                .into(),
            publisher_fingerprint_sha256: manifest.publisher_fingerprint_sha256.clone(),
            manifest,
            enrolled: true,
            revoked: false,
        };
        write_new_json(
            &self
                .root
                .join("packages")
                .join(format!("{manifest_sha256}.json")),
            &record,
        )?;
        Ok(record)
    }

    pub(crate) fn admit_dependency_policy(
        &self,
        manifest_sha256: &str,
        capability_id: &str,
        policy_file: &Path,
    ) -> Result<AdmittedDependencyPolicy, AwareError> {
        validate_sha256(manifest_sha256, "provider manifest sha256")?;
        validate_id(capability_id, "capability id")?;
        let package = self.verify_enrollment(manifest_sha256)?;
        let capability = package
            .manifest
            .capabilities
            .iter()
            .find(|entry| entry.capability_id == capability_id)
            .ok_or_else(|| AwareError::NotFound("provider capability is not enrolled".into()))?;
        let launcher_sha256 = package
            .manifest
            .files
            .iter()
            .find(|entry| entry.path == package.manifest.launcher)
            .map(|entry| entry.sha256.as_str())
            .ok_or_else(|| AwareError::Validation("provider launcher receipt is missing".into()))?;
        let fingerprint = EnrolledProviderFingerprint {
            schema_version: "aware.enrolled-model-provider-fingerprint/v1",
            execution: "enrolled-local",
            package_manifest_sha256: manifest_sha256,
            launcher_sha256,
            publisher_fingerprint_sha256: &package.publisher_fingerprint_sha256,
            format_id: &package.manifest.format_id,
            capability,
        };
        let provider_fingerprint_sha256 = sha256_hex(&canonical_json_bytes(&fingerprint)?);
        let admission_bytes = read_bounded(policy_file, MAX_CONTROL_BYTES)?;
        let admission: DependencyPolicyAdmission = serde_json::from_slice(&admission_bytes)
            .map_err(|error| {
                AwareError::Validation(format!("invalid dependency policy admission: {error}"))
            })?;
        validate_policy_admission(&admission)?;
        let policy = DependencyPolicy {
            schema_version: POLICY_SCHEMA.into(),
            policy_id: admission.policy_id,
            capability_id: capability_id.into(),
            provider_fingerprint_sha256: provider_fingerprint_sha256.clone(),
            roles: admission.roles,
        };
        let bytes = canonical_json_bytes(&policy)?;
        let digest = sha256_hex(&bytes);
        write_replace_json(
            &self
                .root
                .join("policies")
                .join(format!("{provider_fingerprint_sha256}.json")),
            &policy,
        )?;
        Ok(AdmittedDependencyPolicy {
            policy,
            sha256: digest,
        })
    }

    pub(crate) fn select(
        &self,
        format_id: &str,
        manifest_sha256: &str,
    ) -> Result<SelectionRecord, AwareError> {
        validate_id(format_id, "format id")?;
        validate_sha256(manifest_sha256, "manifest sha256")?;
        let package = self.verify_enrollment(manifest_sha256)?;
        if package.revoked || !package.enrolled || package.manifest.format_id != format_id {
            return Err(AwareError::Validation(
                "provider package is not an active enrollment for that format".into(),
            ));
        }
        verify_compatible(&package.manifest)?;
        let _selection_lock = self.acquire_selection_lock(format_id)?;
        let path = self.selection_path(format_id);
        let prior = read_optional_json::<SelectionRecord>(&path)?;
        if let Some(selection) = &prior {
            validate_selection_record(selection, format_id)?;
        }
        let generation = prior.as_ref().map_or(Ok(1), |selection| {
            selection.generation.checked_add(1).ok_or_else(|| {
                AwareError::Conflict("provider selection generation overflow".into())
            })
        })?;
        let mut previous = prior.map_or_else(Vec::new, |selection| {
            let mut values = vec![selection.active_manifest_sha256];
            values.extend(selection.previous_manifest_sha256);
            values
        });
        previous.retain(|digest| digest != manifest_sha256);
        previous.truncate(8);
        let selection = SelectionRecord {
            schema_version: SELECTION_SCHEMA.into(),
            format_id: format_id.into(),
            generation,
            active_manifest_sha256: manifest_sha256.into(),
            previous_manifest_sha256: previous,
        };
        write_replace_json(&path, &selection)?;
        Ok(selection)
    }

    pub(crate) fn list(&self, format: Option<&str>) -> Result<ProviderList, AwareError> {
        if let Some(value) = format {
            validate_id(value, "format id")?;
        }
        let mut selections = BTreeMap::new();
        let selections_dir = self.root.join("selections");
        if selections_dir.is_dir() {
            for entry in std::fs::read_dir(&selections_dir)? {
                let entry = entry?;
                let file_name = entry.file_name();
                let file_name = file_name.to_str();
                if file_name.is_some_and(|name| name.starts_with(".tmp-")) {
                    continue;
                }
                if let Some(format_id) = format
                    && file_name != Some(format!("{format_id}.json").as_str())
                {
                    continue;
                }
                let selection: SelectionRecord = read_json(&entry.path())?;
                let expected_name = format!("{}.json", selection.format_id);
                if file_name != Some(expected_name.as_str()) {
                    return Err(AwareError::Validation(
                        "provider selection record filename does not match its format".into(),
                    ));
                }
                validate_selection_record(&selection, &selection.format_id)?;
                let selected = self.verify_enrollment(&selection.active_manifest_sha256)?;
                if selected.manifest.format_id != selection.format_id {
                    return Err(AwareError::Validation(
                        "provider selection does not reference a package for its format".into(),
                    ));
                }
                selections.insert(selection.format_id, selection.active_manifest_sha256);
            }
        }
        let mut packages = Vec::new();
        let packages_dir = self.root.join("packages");
        if packages_dir.is_dir() {
            for entry in std::fs::read_dir(packages_dir)? {
                let entry = entry?;
                if entry
                    .file_name()
                    .to_str()
                    .is_some_and(|name| name.starts_with(".tmp-"))
                {
                    continue;
                }
                let file_name = entry.file_name();
                let file_name = file_name.to_str().ok_or_else(|| {
                    AwareError::Validation(
                        "provider enrollment filename must be valid Unicode".into(),
                    )
                })?;
                let digest = file_name.strip_suffix(".json").ok_or_else(|| {
                    AwareError::Validation(
                        "provider enrollment record filename must end in .json".into(),
                    )
                })?;
                validate_sha256(digest, "provider enrollment filename digest")?;
                let package_record = self.read_package(digest)?;
                if format.is_some_and(|id| id != package_record.manifest.format_id) {
                    continue;
                }
                let package = self.verify_enrollment(digest)?;
                if package.revoked || !package.enrolled {
                    continue;
                }
                packages.push(ListedPackage {
                    manifest_sha256: package.manifest_sha256.clone(),
                    package_id: package.manifest.package_id,
                    package_version: package.manifest.package_version,
                    format_id: package.manifest.format_id.clone(),
                    publisher_fingerprint_sha256: package.publisher_fingerprint_sha256,
                    capabilities: package.manifest.capabilities,
                    selected: selections.get(&package.manifest.format_id)
                        == Some(&package.manifest_sha256),
                });
            }
        }
        packages.sort_by(|left, right| {
            (
                &left.format_id,
                &left.package_id,
                &left.package_version,
                &left.manifest_sha256,
            )
                .cmp(&(
                    &right.format_id,
                    &right.package_id,
                    &right.package_version,
                    &right.manifest_sha256,
                ))
        });
        Ok(ProviderList { packages })
    }

    fn read_publisher(&self, fingerprint: &str) -> Result<PublisherRecord, AwareError> {
        validate_sha256(fingerprint, "publisher fingerprint")?;
        let publisher: PublisherRecord = read_json(
            &self
                .root
                .join("publishers")
                .join(format!("{fingerprint}.json")),
        )?;
        let public_bytes = base64::engine::general_purpose::STANDARD
            .decode(&publisher.public_key_base64)
            .map_err(|_| AwareError::Validation("trusted publisher key is malformed".into()))?;
        if publisher.schema_version != PUBLISHER_SCHEMA
            || publisher.key_fingerprint_sha256 != fingerprint
            || public_bytes.len() != 32
            || sha256_hex(&public_bytes) != fingerprint
        {
            return Err(AwareError::Validation(
                "trusted publisher record does not match its key fingerprint".into(),
            ));
        }
        validate_id(&publisher.publisher_id, "publisher id")?;
        Ok(publisher)
    }

    fn read_package(&self, digest: &str) -> Result<PackageRecord, AwareError> {
        read_json(&self.root.join("packages").join(format!("{digest}.json")))
    }

    fn verify_enrollment(&self, digest: &str) -> Result<PackageRecord, AwareError> {
        let package = self.read_package(digest)?;
        if package.schema_version != PACKAGE_SCHEMA
            || package.manifest_sha256 != digest
            || package.publisher_fingerprint_sha256 != package.manifest.publisher_fingerprint_sha256
            || !package.enrolled
            || package.revoked
        {
            return Err(AwareError::Validation(
                "provider enrollment record is invalid or inactive".into(),
            ));
        }
        validate_manifest(&package.manifest)?;
        verify_compatible(&package.manifest)?;
        let root = canonical_regular_directory(Path::new(&package.package_root))?;
        if root != Path::new(&package.package_root) {
            return Err(AwareError::Validation(
                "provider package root no longer matches its enrollment".into(),
            ));
        }
        let manifest_bytes = read_bounded(&root.join(MANIFEST_NAME), MAX_CONTROL_BYTES)?;
        if sha256_hex(&manifest_bytes) != digest {
            return Err(AwareError::Validation(
                "provider package manifest changed after enrollment".into(),
            ));
        }
        let disk_manifest: PackageManifest = serde_json::from_slice(&manifest_bytes)
            .map_err(|_| AwareError::Validation("provider package manifest is invalid".into()))?;
        if canonical_json_bytes(&disk_manifest)? != manifest_bytes
            || canonical_json_bytes(&disk_manifest)? != canonical_json_bytes(&package.manifest)?
        {
            return Err(AwareError::Validation(
                "provider package manifest differs from its enrollment".into(),
            ));
        }
        let publisher = self.read_publisher(&package.publisher_fingerprint_sha256)?;
        if !publisher.trusted {
            return Err(AwareError::PermissionDenied(
                "provider package publisher is not trusted".into(),
            ));
        }
        verify_package_signature(&root, digest, &publisher)?;
        verify_package_inventory(&root, &disk_manifest)?;
        Ok(package)
    }

    fn selection_path(&self, format_id: &str) -> PathBuf {
        self.root
            .join("selections")
            .join(format!("{format_id}.json"))
    }

    fn acquire_selection_lock(&self, format_id: &str) -> Result<std::fs::File, AwareError> {
        let directory = self.root.join("locks");
        std::fs::create_dir_all(&directory)?;
        let path = directory.join(format!("selection-{format_id}.lock"));
        match std::fs::symlink_metadata(&path) {
            Ok(metadata)
                if metadata.file_type().is_symlink()
                    || crate::fs::is_reparse_point(&metadata)
                    || !metadata.is_file() =>
            {
                return Err(AwareError::Validation(
                    "provider selection lock must be a regular file".into(),
                ));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?;
        file.lock_exclusive()?;
        Ok(file)
    }
}

fn validate_selection_record(
    selection: &SelectionRecord,
    expected_format_id: &str,
) -> Result<(), AwareError> {
    let previous = selection
        .previous_manifest_sha256
        .iter()
        .collect::<BTreeSet<_>>();
    if selection.schema_version != SELECTION_SCHEMA
        || selection.format_id != expected_format_id
        || selection.generation == 0
        || validate_id(&selection.format_id, "format id").is_err()
        || validate_sha256(&selection.active_manifest_sha256, "active manifest sha256").is_err()
        || selection.previous_manifest_sha256.len() > 8
        || previous.len() != selection.previous_manifest_sha256.len()
        || previous.contains(&selection.active_manifest_sha256)
        || selection
            .previous_manifest_sha256
            .iter()
            .any(|digest| validate_sha256(digest, "previous manifest sha256").is_err())
    {
        return Err(AwareError::Validation(
            "provider selection record is invalid".into(),
        ));
    }
    Ok(())
}

fn validate_manifest(manifest: &PackageManifest) -> Result<(), AwareError> {
    if manifest.schema_version != MANIFEST_SCHEMA {
        return Err(AwareError::Validation(
            "unsupported provider package manifest schema".into(),
        ));
    }
    for (value, label) in [
        (&manifest.package_id, "package id"),
        (&manifest.format_id, "format id"),
    ] {
        validate_id(value, label)?;
    }
    validate_version(&manifest.package_version, "provider package version")?;
    validate_version(&manifest.minimum_aware_version, "minimum AWARE version")?;
    if let Some(maximum) = &manifest.maximum_aware_version {
        validate_version(maximum, "maximum AWARE version")?;
    }
    validate_sha256(
        &manifest.publisher_fingerprint_sha256,
        "publisher fingerprint",
    )?;
    validate_relative_path(&manifest.launcher)?;
    if manifest.capabilities.is_empty() || manifest.files.is_empty() {
        return Err(AwareError::Validation(
            "provider package capabilities and files cannot be empty".into(),
        ));
    }
    let mut capabilities = BTreeSet::new();
    for capability in &manifest.capabilities {
        validate_id(&capability.capability_id, "capability id")?;
        if !capabilities.insert(&capability.capability_id) || capability.protocol_version != "3" {
            return Err(AwareError::Validation(
                "provider capabilities must be unique protocol v3 entries".into(),
            ));
        }
        for value in [
            &capability.source_capture_mode,
            &capability.request_schema,
            &capability.result_schema,
            &capability.artifact_root_version,
            &capability.cache_namespace_version,
        ] {
            validate_id(value, "capability contract id")?;
        }
    }
    let mut files = BTreeSet::new();
    for file in &manifest.files {
        validate_relative_path(&file.path)?;
        validate_sha256(&file.sha256, "package file sha256")?;
        if !files.insert(&file.path) {
            return Err(AwareError::Validation(
                "provider package file paths must be unique".into(),
            ));
        }
    }
    if !files.contains(&manifest.launcher) {
        return Err(AwareError::Validation(
            "provider launcher must be receipted in package files".into(),
        ));
    }
    Ok(())
}

fn validate_policy_admission(policy: &DependencyPolicyAdmission) -> Result<(), AwareError> {
    if policy.schema_version != POLICY_ADMISSION_SCHEMA
        || policy.roles.is_empty()
        || policy.roles.len() > 10_000
    {
        return Err(AwareError::Validation(
            "dependency policy admission is unsupported or empty".into(),
        ));
    }
    validate_id(&policy.policy_id, "dependency policy id")?;
    let mut roles = BTreeSet::new();
    for entry in &policy.roles {
        validate_id(&entry.role, "dependency role")?;
        if !roles.insert(&entry.role) || entry.affected_domains.len() > 64 {
            return Err(AwareError::Validation(
                "dependency policy roles must be unique and bounded".into(),
            ));
        }
        for domain in &entry.affected_domains {
            validate_id(domain, "affected domain")?;
        }
        let valid = match entry.classification.as_str() {
            "mandatory" | "optional" => entry.affected_domains.is_empty(),
            "degraded" => !entry.affected_domains.is_empty(),
            _ => false,
        };
        if !valid {
            return Err(AwareError::Validation(
                "dependency policy role classification is invalid".into(),
            ));
        }
    }
    Ok(())
}

fn validate_version(value: &str, label: &str) -> Result<(), AwareError> {
    if value.len() > MAX_VERSION_LENGTH {
        return Err(AwareError::Validation(format!(
            "{label} must be at most {MAX_VERSION_LENGTH} characters"
        )));
    }
    Version::parse(value)
        .map(|_| ())
        .map_err(|_| AwareError::Validation(format!("{label} must be semver")))
}

fn verify_compatible(manifest: &PackageManifest) -> Result<(), AwareError> {
    let current = Version::parse(env!("CARGO_PKG_VERSION"))
        .map_err(|error| AwareError::Internal(format!("parse current AWARE version: {error}")))?;
    let minimum = Version::parse(&manifest.minimum_aware_version)
        .map_err(|_| AwareError::Validation("minimum AWARE version must be semver".into()))?;
    let maximum = manifest
        .maximum_aware_version
        .as_deref()
        .map(Version::parse)
        .transpose()
        .map_err(|_| AwareError::Validation("maximum AWARE version must be semver".into()))?;
    if current < minimum || maximum.is_some_and(|version| current > version) {
        return Err(AwareError::Validation(
            "provider package is incompatible with this AWARE version".into(),
        ));
    }
    Ok(())
}

fn verify_package_signature(
    root: &Path,
    manifest_sha256: &str,
    publisher: &PublisherRecord,
) -> Result<(), AwareError> {
    let signature_bytes = read_bounded(&root.join(SIGNATURE_NAME), 64 * 1024)?;
    let text = std::str::from_utf8(&signature_bytes)
        .map_err(|_| AwareError::Validation("provider package signature is not UTF-8".into()))?;
    let mut values = BTreeMap::new();
    let mut lines = text.lines();
    if lines.next() != Some("ed25519-signature-v1") {
        return Err(AwareError::Validation(
            "provider package signature has an unsupported schema".into(),
        ));
    }
    for line in lines {
        let Some((key, value)) = line.split_once(':') else {
            return Err(AwareError::Validation(
                "provider package signature is malformed".into(),
            ));
        };
        if values.insert(key.trim(), value.trim()).is_some() {
            return Err(AwareError::Validation(
                "provider package signature repeats a field".into(),
            ));
        }
    }
    if values.keys().copied().collect::<BTreeSet<_>>()
        != BTreeSet::from(["over-sha256-of", "public-key", "sha256", "signature"])
        || values.get("over-sha256-of") != Some(&MANIFEST_NAME)
        || values.get("sha256") != Some(&manifest_sha256)
        || values.get("public-key") != Some(&publisher.public_key_base64.as_str())
    {
        return Err(AwareError::Validation(
            "provider package signature does not bind the enrolled manifest and publisher".into(),
        ));
    }
    let public_bytes = base64::engine::general_purpose::STANDARD
        .decode(&publisher.public_key_base64)
        .map_err(|_| AwareError::Validation("trusted publisher key is malformed".into()))?;
    let public_array: [u8; 32] = public_bytes
        .try_into()
        .map_err(|_| AwareError::Validation("trusted publisher key must be 32 bytes".into()))?;
    let verifying_key = VerifyingKey::from_bytes(&public_array)
        .map_err(|_| AwareError::Validation("trusted publisher key is invalid".into()))?;
    let signature_bytes = base64::engine::general_purpose::STANDARD
        .decode(values["signature"])
        .map_err(|_| AwareError::Validation("provider package signature is malformed".into()))?;
    let signature = Signature::from_slice(&signature_bytes)
        .map_err(|_| AwareError::Validation("provider package signature is malformed".into()))?;
    let digest = hex_digest_bytes(manifest_sha256)?;
    verifying_key
        .verify(&digest, &signature)
        .map_err(|_| AwareError::Validation("provider package signature does not verify".into()))
}

fn verify_package_inventory(root: &Path, manifest: &PackageManifest) -> Result<(), AwareError> {
    let expected = manifest
        .files
        .iter()
        .map(|file| file.path.clone())
        .chain([MANIFEST_NAME.into(), SIGNATURE_NAME.into()])
        .collect::<BTreeSet<_>>();
    let actual = walk_regular_files(root, root)?;
    if actual != expected {
        return Err(AwareError::Validation(
            "provider package directory is not a closed manifest allowlist".into(),
        ));
    }
    for receipt in &manifest.files {
        let path = root.join(receipt.path.replace('/', std::path::MAIN_SEPARATOR_STR));
        let (bytes, digest) = hash_regular_file(&path)?;
        if bytes != receipt.bytes || digest != receipt.sha256 {
            return Err(AwareError::Validation(format!(
                "provider package file {} does not match its receipt",
                receipt.path
            )));
        }
    }
    Ok(())
}

fn walk_regular_files(root: &Path, directory: &Path) -> Result<BTreeSet<String>, AwareError> {
    let mut files = BTreeSet::new();
    for entry in std::fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = std::fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() || crate::fs::is_reparse_point(&metadata) {
            return Err(AwareError::Validation(
                "provider package cannot contain links or reparse points".into(),
            ));
        }
        if metadata.is_dir() {
            files.extend(walk_regular_files(root, &path)?);
        } else if metadata.is_file() {
            let relative = path
                .strip_prefix(root)
                .map_err(|_| AwareError::Internal("package inventory escaped its root".into()))?;
            files.insert(relative.to_string_lossy().replace('\\', "/"));
        } else {
            return Err(AwareError::Validation(
                "provider package can contain only regular files".into(),
            ));
        }
    }
    Ok(files)
}

fn canonical_regular_directory(path: &Path) -> Result<PathBuf, AwareError> {
    if !path.is_absolute() {
        return Err(AwareError::Validation(
            "provider package directory must be absolute".into(),
        ));
    }
    let metadata = std::fs::symlink_metadata(path)?;
    if !metadata.is_dir()
        || metadata.file_type().is_symlink()
        || crate::fs::is_reparse_point(&metadata)
    {
        return Err(AwareError::Validation(
            "provider package root must be a regular non-link directory".into(),
        ));
    }
    let canonical = std::fs::canonicalize(path)?;
    #[cfg(windows)]
    {
        // `canonicalize` returns verbatim `\\?\` paths on Windows, while the Node reader's
        // `realpath` and executable check return ordinary absolute paths. Keep the resolved path
        // identity but store its ordinary spelling so both halves compare the same location.
        let value = canonical.to_str().ok_or_else(|| {
            AwareError::Validation("provider package root must be valid Unicode".into())
        })?;
        if let Some(rest) = value.strip_prefix(r"\\?\UNC\") {
            return Ok(PathBuf::from(format!(r"\\{rest}")));
        }
        if let Some(rest) = value.strip_prefix(r"\\?\") {
            return Ok(PathBuf::from(rest));
        }
    }
    Ok(canonical)
}

fn validate_id(value: &str, label: &str) -> Result<(), AwareError> {
    if value.is_empty()
        || value.len() > 128
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err(AwareError::Validation(format!(
            "{label} must be an opaque ASCII identifier"
        )));
    }
    Ok(())
}

fn validate_sha256(value: &str, label: &str) -> Result<(), AwareError> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(AwareError::Validation(format!(
            "{label} must be a lowercase SHA-256 digest"
        )));
    }
    Ok(())
}

fn validate_relative_path(value: &str) -> Result<(), AwareError> {
    if value.is_empty()
        || value.len() > 512
        || value.contains('\\')
        || value.contains(':')
        || value.ends_with(['.', ' '])
    {
        return Err(AwareError::Validation(
            "provider package paths must be normalized relative paths".into(),
        ));
    }
    let path = Path::new(value);
    if path.is_absolute()
        || path
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
    {
        return Err(AwareError::Validation(
            "provider package paths must be normalized relative paths".into(),
        ));
    }
    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn hash_regular_file(path: &Path) -> Result<(u64, String), AwareError> {
    let metadata = std::fs::symlink_metadata(path)?;
    if !metadata.is_file()
        || metadata.file_type().is_symlink()
        || crate::fs::is_reparse_point(&metadata)
    {
        return Err(AwareError::Validation(
            "provider package receipt must name a regular non-link file".into(),
        ));
    }
    let mut file = std::fs::File::open(path)?;
    let mut hash = Sha256::new();
    let mut read_bytes = 0_u64;
    let mut chunk = vec![0_u8; 1024 * 1024];
    loop {
        let count = file.read(&mut chunk)?;
        if count == 0 {
            break;
        }
        read_bytes = read_bytes
            .checked_add(count as u64)
            .ok_or_else(|| AwareError::Validation("provider package file is too large".into()))?;
        hash.update(&chunk[..count]);
    }
    if read_bytes != metadata.len() {
        return Err(AwareError::Validation(
            "provider package file changed while it was read".into(),
        ));
    }
    Ok((read_bytes, format!("{:x}", hash.finalize())))
}

fn read_bounded(path: &Path, limit: u64) -> Result<Vec<u8>, AwareError> {
    let metadata = std::fs::symlink_metadata(path)?;
    if !metadata.is_file()
        || metadata.file_type().is_symlink()
        || crate::fs::is_reparse_point(&metadata)
        || metadata.len() > limit
    {
        return Err(AwareError::Validation(format!(
            "provider control file {} is unsafe or too large",
            path.display()
        )));
    }
    let file = std::fs::File::open(path)?;
    let capacity = usize::try_from(metadata.len())
        .map_err(|_| AwareError::Validation("provider control file is too large".into()))?;
    let mut bytes = Vec::with_capacity(capacity);
    file.take(limit + 1).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > limit || bytes.len() as u64 != metadata.len() {
        return Err(AwareError::Validation(
            "provider control file changed or exceeded its byte limit".into(),
        ));
    }
    Ok(bytes)
}

fn canonical_json_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, AwareError> {
    fn ordered(value: serde_json::Value) -> serde_json::Value {
        match value {
            serde_json::Value::Array(values) => {
                serde_json::Value::Array(values.into_iter().map(ordered).collect())
            }
            serde_json::Value::Object(values) => {
                let sorted = values.into_iter().collect::<BTreeMap<_, _>>();
                serde_json::Value::Object(
                    sorted
                        .into_iter()
                        .map(|(key, value)| (key, ordered(value)))
                        .collect(),
                )
            }
            scalar => scalar,
        }
    }
    Ok(serde_json::to_vec(&ordered(serde_json::to_value(value)?))?)
}

fn hex_digest_bytes(value: &str) -> Result<[u8; 32], AwareError> {
    validate_sha256(value, "manifest sha256")?;
    let mut bytes = [0_u8; 32];
    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        let text = std::str::from_utf8(pair)
            .map_err(|_| AwareError::Validation("manifest sha256 is malformed".into()))?;
        bytes[index] = u8::from_str_radix(text, 16)
            .map_err(|_| AwareError::Validation("manifest sha256 is malformed".into()))?;
    }
    Ok(bytes)
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, AwareError> {
    let bytes = read_bounded(path, MAX_CONTROL_BYTES).map_err(|error| match error {
        AwareError::Io(io) if io.kind() == std::io::ErrorKind::NotFound => {
            AwareError::NotFound(format!("{}: {io}", path.display()))
        }
        other => other,
    })?;
    serde_json::from_slice(&bytes).map_err(|error| {
        AwareError::Validation(format!(
            "invalid provider-store record {}: {error}",
            path.display()
        ))
    })
}

fn read_optional_json<T: serde::de::DeserializeOwned>(
    path: &Path,
) -> Result<Option<T>, AwareError> {
    match read_bounded(path, MAX_CONTROL_BYTES) {
        Ok(bytes) => serde_json::from_slice(&bytes).map(Some).map_err(|error| {
            AwareError::Validation(format!(
                "invalid provider-store record {}: {error}",
                path.display()
            ))
        }),
        Err(AwareError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

fn write_new_json<T: Serialize>(path: &Path, value: &T) -> Result<(), AwareError> {
    if path.exists() {
        return Err(AwareError::Conflict(format!(
            "provider-store record already exists: {}",
            path.display()
        )));
    }
    write_json(path, value, false)
}

fn write_replace_json<T: Serialize>(path: &Path, value: &T) -> Result<(), AwareError> {
    write_json(path, value, true)
}

fn write_json<T: Serialize>(path: &Path, value: &T, replace: bool) -> Result<(), AwareError> {
    let Some(parent) = path.parent() else {
        return Err(AwareError::Internal(
            "provider-store record has no parent".into(),
        ));
    };
    std::fs::create_dir_all(parent)?;
    let temporary = parent.join(format!(".tmp-{}", uuid::Uuid::new_v4()));
    let result = (|| -> Result<(), AwareError> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)?;
        file.write_all(&serde_json::to_vec(value)?)?;
        file.sync_all()?;
        atomic_rename(&temporary, path, replace)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&temporary);
    }
    result
}

#[cfg(not(windows))]
fn atomic_rename(source: &Path, destination: &Path, replace: bool) -> std::io::Result<()> {
    if replace {
        return std::fs::rename(source, destination);
    }
    // A same-filesystem hard link publishes the already-fsynced inode only when the immutable
    // destination does not exist. `rename` would silently replace it on Unix after the earlier
    // existence check raced another enrollment.
    std::fs::hard_link(source, destination)?;
    std::fs::remove_file(source)
}

#[cfg(windows)]
fn atomic_rename(source: &Path, destination: &Path, replace: bool) -> std::io::Result<()> {
    if !replace {
        return std::fs::rename(source, destination);
    }
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::{
        MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH, MoveFileExW,
    };
    let source_wide = source
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let destination_wide = destination
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    // SAFETY: both pointers name live, NUL-terminated UTF-16 buffers for the duration of the call.
    let moved = unsafe {
        MoveFileExW(
            source_wide.as_ptr(),
            destination_wide.as_ptr(),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    };
    if moved == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    fn valid_manifest() -> PackageManifest {
        PackageManifest {
            schema_version: MANIFEST_SCHEMA.into(),
            package_id: "package.synthetic".into(),
            package_version: "1.2.3".into(),
            format_id: "format.synthetic".into(),
            launcher: "provider.bin".into(),
            minimum_aware_version: "0.137.0".into(),
            maximum_aware_version: None,
            publisher_fingerprint_sha256: "a".repeat(64),
            capabilities: vec![ProviderCapability {
                capability_id: "capability.synthetic".into(),
                protocol_version: "3".into(),
                source_capture_mode: "capture.synthetic".into(),
                request_schema: "request.synthetic.v1".into(),
                result_schema: "result.synthetic.v1".into(),
                artifact_root_version: "artifact.synthetic.v1".into(),
                cache_namespace_version: "cache.synthetic.v1".into(),
            }],
            files: vec![PackageFile {
                path: "provider.bin".into(),
                bytes: 1,
                sha256: "b".repeat(64),
            }],
        }
    }

    #[test]
    fn manifest_versions_obey_the_schema_length_bound() {
        let boundary = format!("1.0.0+{}", "a".repeat(122));
        assert_eq!(boundary.len(), MAX_VERSION_LENGTH);
        let mut manifest = valid_manifest();
        manifest.package_version = boundary;
        validate_manifest(&manifest).unwrap();

        let too_long = format!("1.0.0+{}", "a".repeat(123));
        for field in ["package", "minimum", "maximum"] {
            let mut manifest = valid_manifest();
            match field {
                "package" => manifest.package_version = too_long.clone(),
                "minimum" => manifest.minimum_aware_version = too_long.clone(),
                "maximum" => manifest.maximum_aware_version = Some(too_long.clone()),
                _ => unreachable!(),
            }
            assert!(validate_manifest(&manifest).is_err(), "{field} version");
        }
    }

    /// SHA-256 of the empty input and of `abc` — the two most widely published
    /// SHA-256 vectors (`abc` is NIST's worked example; the empty-input digest
    /// is the CAVP `SHA256ShortMsg` Len=0 value). Hard-coding them keeps
    /// [`sha256_hex`] and [`hex_digest_bytes`] honest against an answer from
    /// outside this crate rather than against each other.
    const EMPTY_SHA256: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    const ABC_SHA256: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

    fn valid_selection() -> SelectionRecord {
        SelectionRecord {
            schema_version: SELECTION_SCHEMA.into(),
            format_id: "format.synthetic".into(),
            generation: 4,
            active_manifest_sha256: "a".repeat(64),
            previous_manifest_sha256: vec!["b".repeat(64), "c".repeat(64)],
        }
    }

    fn valid_admission() -> DependencyPolicyAdmission {
        DependencyPolicyAdmission {
            schema_version: POLICY_ADMISSION_SCHEMA.into(),
            policy_id: "policy.synthetic".into(),
            roles: vec![
                DependencyPolicyRole {
                    role: "role.required".into(),
                    classification: "mandatory".into(),
                    affected_domains: vec![],
                },
                DependencyPolicyRole {
                    role: "role.partial".into(),
                    classification: "degraded".into(),
                    affected_domains: vec!["domain.geometry".into()],
                },
            ],
        }
    }

    // ---------------------------------------------------------------------
    // validate_relative_path — the guard standing between a package manifest
    // and an arbitrary write target on the host.
    // ---------------------------------------------------------------------

    #[test]
    fn package_paths_refuse_the_known_escape_spellings() {
        // Not "every way out": `validate_relative_path` tests `ends_with(['.',
        // ' '])` against the whole string rather than each component, so
        // `bin./provider` is accepted today and aliases `bin/provider` on
        // Win32 by the same stripping rule the two cases below rely on. That
        // gap is in production code and is reported rather than changed here.
        for escape in [
            "../outside",        // ParentDir component
            "bin/../../outside", // ParentDir after a Normal component
            "./bin/provider",    // CurDir component
            "/etc/passwd",       // RootDir component
            "bin\\provider", // on Windows two Normal components, so the backslash check is the only guard
            "C:bin",         // drive-relative; on Unix the colon check is the only guard
            "provider.bin.", // trailing dot — Win32 strips it, so it aliases another file
            "provider.bin ", // trailing space — same aliasing
            "",              // empty
        ] {
            assert!(
                validate_relative_path(escape).is_err(),
                "{escape:?} must not be accepted as a package-relative path"
            );
        }
    }

    #[test]
    fn package_paths_accept_plain_relative_spellings_up_to_the_length_bound() {
        for ok in ["provider.bin", "bin/provider", "a/b/c/d.so", "lib/.keep"] {
            validate_relative_path(ok)
                .unwrap_or_else(|error| panic!("{ok:?} is a normalized relative path: {error:?}"));
        }
        let boundary = "a".repeat(512);
        validate_relative_path(&boundary).unwrap();
        assert!(validate_relative_path(&"a".repeat(513)).is_err());
    }

    // ---------------------------------------------------------------------
    // validate_sha256 / validate_id — the shapes every record key is built from.
    // ---------------------------------------------------------------------

    #[test]
    fn digests_must_be_exactly_sixty_four_lowercase_hex_characters() {
        validate_sha256(&"0".repeat(64), "digest").unwrap();
        validate_sha256(ABC_SHA256, "digest").unwrap();
        for bad in [
            "a".repeat(63),                 // one short
            "a".repeat(65),                 // one long
            ABC_SHA256.to_uppercase(),      // uppercase hex is a different filename on Unix
            format!("{}g", "a".repeat(63)), // out-of-alphabet
            format!("{} ", "a".repeat(63)), // trailing space
            String::new(),
        ] {
            assert!(
                validate_sha256(&bad, "digest").is_err(),
                "{bad:?} must not pass as a digest"
            );
        }
    }

    #[test]
    fn identifiers_are_bounded_and_rejected_outside_their_alphabet() {
        validate_id("format.synthetic-1_0", "id").unwrap();
        validate_id(&"a".repeat(128), "id").unwrap();
        for bad in [
            "a".repeat(129), // one past the bound
            String::new(),   // empty — would collapse a record filename
            "a/b".into(),    // separator
            "a\\b".into(),   // Windows separator
            "a b".into(),    // whitespace
            "a+b".into(),    // outside the alphabet
            "café".into(),   // non-ASCII
        ] {
            assert!(
                validate_id(&bad, "id").is_err(),
                "{bad:?} must not pass as an id"
            );
        }
    }

    #[test]
    fn the_identifier_alphabet_is_exactly_ascii_alphanumerics_dot_underscore_and_dash() {
        // Pinned exhaustively rather than by sample: every record filename in
        // the store is an id with a suffix pasted on, so widening this alphabet
        // by one byte is how an id would start naming something else.
        let accepted = (0_u8..=127)
            .filter(|byte| validate_id(&(*byte as char).to_string(), "id").is_ok())
            .collect::<BTreeSet<_>>();
        let expected = (b'0'..=b'9')
            .chain(b'A'..=b'Z')
            .chain(b'a'..=b'z')
            .chain([b'.', b'_', b'-'])
            .collect::<BTreeSet<_>>();
        assert_eq!(accepted, expected);
    }

    #[test]
    fn a_validated_identifier_always_stays_one_child_of_its_directory() {
        // `.` is in the alphabet, so `.` and `..` are both accepted ids. That is
        // harmless only because the two paths built from an id paste a suffix
        // on — `selections/{id}.json` and `locks/selection-{id}.lock`, so `..`
        // becomes the filename `...json` — and because no separator can get
        // through. (The publisher, package and policy records are named from a
        // digest instead, and are guarded by `validate_sha256`, not this.)
        // Assert the consequence over the whole accepted alphabet, not the rule.
        for byte in 0_u8..=127 {
            let id = (byte as char).to_string();
            if validate_id(&id, "id").is_err() {
                continue;
            }
            for joined in [
                Path::new("root").join(format!("{id}.json")),
                Path::new("root").join(format!("selection-{id}.lock")),
            ] {
                assert_eq!(
                    joined.components().count(),
                    2,
                    "id {id:?} did not stay a single child of its directory"
                );
            }
        }
        assert_eq!(
            Path::new("root").join("...json").components().count(),
            2,
            "`..` as a format id must land as the filename `...json`"
        );
    }

    // ---------------------------------------------------------------------
    // Digest helpers, against published vectors.
    // ---------------------------------------------------------------------

    #[test]
    fn digest_helpers_agree_with_the_published_sha256_vectors() {
        assert_eq!(sha256_hex(b""), EMPTY_SHA256);
        assert_eq!(sha256_hex(b"abc"), ABC_SHA256);

        // The raw bytes the signature is verified over: a transposition or a
        // wrong radix here would verify a signature against the wrong message.
        assert_eq!(
            hex_digest_bytes(EMPTY_SHA256).unwrap(),
            [
                0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f,
                0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b,
                0x78, 0x52, 0xb8, 0x55,
            ]
        );
        assert!(hex_digest_bytes(&ABC_SHA256.to_uppercase()).is_err());
        assert!(hex_digest_bytes("abc").is_err());
    }

    #[test]
    fn hashing_a_file_reports_its_length_and_digest_and_refuses_indirection() {
        let tmp = tempfile::tempdir().unwrap();
        let file = tmp.path().join("payload");
        std::fs::write(&file, b"abc").unwrap();
        assert_eq!(
            hash_regular_file(&file).unwrap(),
            (3, ABC_SHA256.to_string())
        );

        let empty = tmp.path().join("empty");
        std::fs::write(&empty, b"").unwrap();
        assert_eq!(
            hash_regular_file(&empty).unwrap(),
            (0, EMPTY_SHA256.to_string())
        );

        match hash_regular_file(tmp.path()) {
            Err(AwareError::Validation(_)) => {}
            other => {
                panic!("a directory must be refused as a non-file, not by a read error: {other:?}")
            }
        }
        assert!(matches!(
            hash_regular_file(&tmp.path().join("absent")),
            Err(AwareError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound
        ));

        #[cfg(unix)]
        {
            // Note this does NOT isolate the explicit `is_symlink()` clause:
            // `symlink_metadata` already reports a link as `!is_file()`, so on
            // Unix that clause is redundant and deleting it leaves this green.
            // It is load-bearing only on Windows, via `is_reparse_point`, where
            // `cargo test` never runs — recorded in the pull request.
            let link = tmp.path().join("link");
            std::os::unix::fs::symlink(&file, &link).unwrap();
            assert!(
                hash_regular_file(&link).is_err(),
                "a receipt must not be satisfied through a symlink"
            );
        }
    }

    // ---------------------------------------------------------------------
    // canonical_json_bytes — the encoding a manifest's on-disk bytes must
    // already equal before enrollment will digest them. The digest itself is
    // taken over the raw file bytes (`sha256_hex(&manifest_bytes)`); this
    // function is the equality gate in front of it, so if the sort were wrong
    // the gate would reject canonical manifests and admit the spelling the
    // sort happens to produce. `serde_json` is built here with
    // `preserve_order`, so without the recursive sort the bytes would follow
    // insertion order rather than being sorted.
    // ---------------------------------------------------------------------

    #[test]
    fn canonical_json_sorts_keys_at_every_depth_including_inside_arrays() {
        let value = json!({
            "b": 1,
            "a": {
                "d": 2,
                "c": [{"f": 3, "e": 4}]
            }
        });
        assert_eq!(
            String::from_utf8(canonical_json_bytes(&value).unwrap()).unwrap(),
            r#"{"a":{"c":[{"e":4,"f":3}],"d":2},"b":1}"#
        );
    }

    #[test]
    fn canonical_json_is_stable_under_the_input_key_order() {
        let forward = json!({"alpha": {"x": 1, "y": 2}, "beta": [3, 4]});
        let reversed = json!({"beta": [3, 4], "alpha": {"y": 2, "x": 1}});
        assert_eq!(
            canonical_json_bytes(&forward).unwrap(),
            canonical_json_bytes(&reversed).unwrap()
        );
        // Array order is data, not key order, and must survive untouched.
        assert_ne!(
            canonical_json_bytes(&json!({"a": [1, 2]})).unwrap(),
            canonical_json_bytes(&json!({"a": [2, 1]})).unwrap()
        );
    }

    // ---------------------------------------------------------------------
    // verify_compatible — the version window, read against this build.
    // ---------------------------------------------------------------------

    #[test]
    fn the_version_window_is_inclusive_at_both_ends_and_closed_outside_them() {
        let current = env!("CARGO_PKG_VERSION");
        // Both bounds are built structurally rather than by pasting a suffix
        // onto `current`: `{current}-alpha` is below `current` only while the
        // crate is on a plain X.Y.Z, and sorts ABOVE it once the crate is
        // itself on a prerelease — which `sync_stats.py --bump` accepts and
        // `release.yml` ships. That would have failed this test on the first
        // `--bump X.Y.Z-rc.1`, for a reason unrelated to the window.
        let below = Version::new(0, 0, 0).to_string();
        let above = Version::parse(current).unwrap();
        let above = Version::new(above.major + 1, 0, 0).to_string();

        let mut manifest = valid_manifest();
        manifest.minimum_aware_version = current.into();
        manifest.maximum_aware_version = Some(current.into());
        verify_compatible(&manifest).unwrap();

        let mut manifest = valid_manifest();
        manifest.minimum_aware_version = below.clone();
        manifest.maximum_aware_version = None;
        verify_compatible(&manifest).unwrap();

        let mut manifest = valid_manifest();
        manifest.minimum_aware_version = above.clone();
        assert!(
            verify_compatible(&manifest).is_err(),
            "a package needing {above} must not load on {current}"
        );

        let mut manifest = valid_manifest();
        manifest.minimum_aware_version = below;
        manifest.maximum_aware_version = Some("0.0.1".into());
        assert!(
            verify_compatible(&manifest).is_err(),
            "a package capped at 0.0.1 must not load on {current}"
        );
    }

    // ---------------------------------------------------------------------
    // validate_manifest — the closed shape of a package manifest.
    // ---------------------------------------------------------------------

    #[test]
    fn a_manifest_must_receipt_its_own_launcher() {
        validate_manifest(&valid_manifest()).unwrap();
        let mut manifest = valid_manifest();
        manifest.launcher = "other.bin".into();
        assert!(
            validate_manifest(&manifest).is_err(),
            "an unreceipted launcher would run bytes no digest covers"
        );
    }

    #[test]
    fn a_manifest_refuses_duplicate_files_and_duplicate_capabilities() {
        let mut manifest = valid_manifest();
        manifest.files.push(manifest.files[0].clone());
        assert!(
            validate_manifest(&manifest).is_err(),
            "duplicate file receipts"
        );

        let mut manifest = valid_manifest();
        manifest.capabilities.push(manifest.capabilities[0].clone());
        assert!(
            validate_manifest(&manifest).is_err(),
            "duplicate capability ids"
        );
    }

    #[test]
    fn a_manifest_pins_protocol_v3_and_refuses_empty_inventories() {
        let mut manifest = valid_manifest();
        manifest.capabilities[0].protocol_version = "2".into();
        assert!(validate_manifest(&manifest).is_err(), "protocol v2");

        let mut manifest = valid_manifest();
        manifest.capabilities.clear();
        assert!(validate_manifest(&manifest).is_err(), "no capabilities");

        let mut manifest = valid_manifest();
        manifest.files.clear();
        assert!(validate_manifest(&manifest).is_err(), "no files");

        let mut manifest = valid_manifest();
        manifest.schema_version = "aware.model-provider-package/v2".into();
        assert!(validate_manifest(&manifest).is_err(), "unknown schema");
    }

    #[test]
    fn a_manifest_validates_every_capability_contract_id_not_just_the_first() {
        // `source_capture_mode` through `cache_namespace_version` are checked in
        // a loop; a loop that stopped early would let the later ones through.
        for index in 0..5 {
            let mut manifest = valid_manifest();
            let capability = &mut manifest.capabilities[0];
            let field = match index {
                0 => &mut capability.source_capture_mode,
                1 => &mut capability.request_schema,
                2 => &mut capability.result_schema,
                3 => &mut capability.artifact_root_version,
                _ => &mut capability.cache_namespace_version,
            };
            *field = "not an id".into();
            assert!(
                validate_manifest(&manifest).is_err(),
                "capability contract field {index} must be an opaque id"
            );
        }
    }

    #[test]
    fn a_manifest_refuses_a_launcher_or_receipt_that_escapes_the_package_root() {
        let mut manifest = valid_manifest();
        manifest.launcher = "../provider.bin".into();
        manifest.files[0].path = "../provider.bin".into();
        assert!(validate_manifest(&manifest).is_err(), "traversing launcher");

        let mut manifest = valid_manifest();
        manifest.files.push(PackageFile {
            path: "../sibling.so".into(),
            bytes: 1,
            sha256: "c".repeat(64),
        });
        assert!(validate_manifest(&manifest).is_err(), "traversing receipt");

        let mut manifest = valid_manifest();
        manifest.files[0].sha256 = "not-a-digest".into();
        assert!(
            validate_manifest(&manifest).is_err(),
            "malformed receipt digest"
        );
    }

    // ---------------------------------------------------------------------
    // validate_selection_record — what `list` and `select` will trust on disk.
    // ---------------------------------------------------------------------

    #[test]
    fn a_selection_must_name_the_format_it_is_filed_under() {
        validate_selection_record(&valid_selection(), "format.synthetic").unwrap();
        assert!(
            validate_selection_record(&valid_selection(), "format.other").is_err(),
            "a selection read for one format must not answer for another"
        );

        let mut selection = valid_selection();
        selection.format_id = "not an id".into();
        assert!(validate_selection_record(&selection, "not an id").is_err());
    }

    #[test]
    fn a_selection_generation_is_one_based_so_zero_is_a_forged_record() {
        let mut selection = valid_selection();
        selection.generation = 0;
        assert!(validate_selection_record(&selection, "format.synthetic").is_err());

        selection.generation = 1;
        validate_selection_record(&selection, "format.synthetic").unwrap();
    }

    #[test]
    fn a_selection_history_is_a_bounded_set_that_excludes_the_active_digest() {
        let mut selection = valid_selection();
        selection.previous_manifest_sha256 = (0..8).map(|i| i.to_string().repeat(64)).collect();
        validate_selection_record(&selection, "format.synthetic").unwrap();

        selection.previous_manifest_sha256 = (0..9).map(|i| i.to_string().repeat(64)).collect();
        assert!(
            validate_selection_record(&selection, "format.synthetic").is_err(),
            "history must stay bounded at eight"
        );

        let mut selection = valid_selection();
        let repeated = selection.previous_manifest_sha256[0].clone();
        selection.previous_manifest_sha256.push(repeated);
        assert!(
            validate_selection_record(&selection, "format.synthetic").is_err(),
            "history must not repeat a digest"
        );

        let mut selection = valid_selection();
        selection
            .previous_manifest_sha256
            .push(selection.active_manifest_sha256.clone());
        assert!(
            validate_selection_record(&selection, "format.synthetic").is_err(),
            "the active digest must not also be history"
        );
    }

    #[test]
    fn a_selection_validates_every_digest_it_carries() {
        let mut selection = valid_selection();
        selection.active_manifest_sha256 = "not-a-digest".into();
        assert!(validate_selection_record(&selection, "format.synthetic").is_err());

        // The last history entry, not the first: an `any` that had become a
        // check of `first()` would pass this.
        let mut selection = valid_selection();
        *selection.previous_manifest_sha256.last_mut().unwrap() = "not-a-digest".into();
        assert!(validate_selection_record(&selection, "format.synthetic").is_err());

        let mut selection = valid_selection();
        selection.schema_version = "aware.model-provider-selection/v2".into();
        assert!(validate_selection_record(&selection, "format.synthetic").is_err());
    }

    // ---------------------------------------------------------------------
    // validate_policy_admission — the operator-supplied half of a policy.
    // ---------------------------------------------------------------------

    #[test]
    fn a_policy_role_classification_decides_whether_domains_are_required() {
        validate_policy_admission(&valid_admission()).unwrap();

        for classification in ["mandatory", "optional"] {
            let mut admission = valid_admission();
            admission.roles[0].classification = classification.into();
            admission.roles[0].affected_domains = vec![];
            validate_policy_admission(&admission).unwrap_or_else(|error| {
                panic!("a {classification} role with no affected domain is valid: {error:?}")
            });

            admission.roles[0].affected_domains = vec!["domain.geometry".into()];
            assert!(
                validate_policy_admission(&admission).is_err(),
                "{classification} roles name no affected domain"
            );
        }

        let mut admission = valid_admission();
        admission.roles[1].affected_domains = vec![];
        assert!(
            validate_policy_admission(&admission).is_err(),
            "a degraded role must name what it degrades"
        );

        let mut admission = valid_admission();
        admission.roles[0].classification = "advisory".into();
        assert!(
            validate_policy_admission(&admission).is_err(),
            "the classification vocabulary is closed"
        );
    }

    #[test]
    fn a_policy_admission_is_non_empty_with_unique_bounded_roles() {
        let mut admission = valid_admission();
        admission.roles.clear();
        assert!(validate_policy_admission(&admission).is_err(), "no roles");

        let mut admission = valid_admission();
        admission.roles.push(admission.roles[0].clone());
        assert!(
            validate_policy_admission(&admission).is_err(),
            "repeated role"
        );

        let mut admission = valid_admission();
        admission.roles[1].affected_domains = (0..64).map(|i| format!("domain.d{i}")).collect();
        validate_policy_admission(&admission).unwrap();
        admission.roles[1].affected_domains = (0..65).map(|i| format!("domain.d{i}")).collect();
        assert!(validate_policy_admission(&admission).is_err(), "65 domains");

        let mut admission = valid_admission();
        admission.roles[1].affected_domains = vec!["not a domain".into()];
        assert!(
            validate_policy_admission(&admission).is_err(),
            "domain is not an id"
        );

        let mut admission = valid_admission();
        admission.schema_version = "aware.model-dependency-policy-admission/v2".into();
        assert!(
            validate_policy_admission(&admission).is_err(),
            "unknown schema"
        );

        let mut admission = valid_admission();
        admission.policy_id = "not an id".into();
        assert!(
            validate_policy_admission(&admission).is_err(),
            "policy id is not an id"
        );
    }

    // ---------------------------------------------------------------------
    // read_bounded / walk_regular_files / canonical_regular_directory —
    // everything the store reads before it has verified anything.
    // ---------------------------------------------------------------------

    #[test]
    fn a_control_file_is_read_up_to_its_limit_inclusive_and_no_further() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("control.json");

        std::fs::write(&path, vec![b'x'; 16]).unwrap();
        assert_eq!(read_bounded(&path, 16).unwrap(), vec![b'x'; 16]);

        std::fs::write(&path, vec![b'x'; 17]).unwrap();
        assert!(
            read_bounded(&path, 16).is_err(),
            "one byte past the limit must be refused, not truncated"
        );

        // A directory must be refused by the guard, not by the `EISDIR` that
        // `read_to_end` raises later — `is_err()` alone cannot tell those apart.
        // The limit here is deliberately far above a directory's own reported
        // size: with the small limit a directory trips the byte ceiling instead
        // (4096 > 1024), which hides whether the `!is_file()` clause exists.
        match read_bounded(tmp.path(), 16 * 1024 * 1024) {
            Err(AwareError::Validation(_)) => {}
            other => {
                panic!("a directory must be refused as unsafe, not by a read error: {other:?}")
            }
        }
        assert!(matches!(
            read_bounded(&tmp.path().join("absent"), 1024),
            Err(AwareError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound
        ));
    }

    #[cfg(unix)]
    #[test]
    fn a_control_file_is_never_read_through_a_symlink() {
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("real.json");
        std::fs::write(&target, b"{}").unwrap();
        let link = tmp.path().join("link.json");
        std::os::unix::fs::symlink(&target, &link).unwrap();
        assert!(
            read_bounded(&link, 1024).is_err(),
            "a symlinked control file could point outside the package"
        );
    }

    #[test]
    fn the_inventory_walk_descends_and_reports_slash_separated_relative_paths() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::write(root.join(MANIFEST_NAME), b"{}").unwrap();
        std::fs::create_dir_all(root.join("bin").join("deep")).unwrap();
        std::fs::write(root.join("bin").join("provider"), b"").unwrap();
        std::fs::write(root.join("bin").join("deep").join("lib.so"), b"").unwrap();

        assert_eq!(
            walk_regular_files(root, root).unwrap(),
            BTreeSet::from([
                "bin/deep/lib.so".to_string(),
                "bin/provider".to_string(),
                "provider-package.json".to_string(),
            ])
        );
    }

    #[cfg(unix)]
    #[test]
    fn the_inventory_walk_refuses_a_link_anywhere_beneath_the_root() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir(root.join("bin")).unwrap();
        std::fs::write(root.join("bin").join("provider"), b"").unwrap();
        // Nested, not at the top: a walk that stopped descending would hand
        // back a clean inventory that never saw this entry at all. Note this
        // does NOT isolate the explicit `is_symlink()` guard — with that guard
        // deleted a symlink still reports neither dir nor file on Unix, so the
        // `else` arm ("only regular files") rejects it and this stays green.
        // The guard is load-bearing on Windows, where a junction reports
        // `is_dir()` AND the reparse bit; nothing here covers that.
        std::os::unix::fs::symlink("/etc/passwd", root.join("bin").join("secrets")).unwrap();
        // Matched on the message, not `is_err()`: with the link guard deleted
        // the `else` arm still rejects the entry, but with a different string
        // ("can contain only regular files"), so only this tells the guard
        // being gone apart from the guard doing its job.
        match walk_regular_files(root, root) {
            Err(AwareError::Validation(message)) => assert!(
                message.contains("links or reparse points"),
                "the link guard, not the regular-file fallback, must reject this: {message}"
            ),
            other => panic!("expected the link guard to reject the entry, got {other:?}"),
        }
    }

    #[test]
    fn a_package_root_must_be_an_absolute_real_directory() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        assert_eq!(
            canonical_regular_directory(root).unwrap(),
            std::fs::canonicalize(root).unwrap()
        );

        // Asserted on the variant, not just `is_err`: a relative path that does
        // not exist also fails at `symlink_metadata`, so `is_err()` alone would
        // stay green with the absolute-path guard deleted.
        match canonical_regular_directory(Path::new("relative/dir")) {
            Err(AwareError::Validation(message)) => assert!(
                message.contains("must be absolute"),
                "a relative root must be refused as relative, not as missing: {message}"
            ),
            other => panic!("expected a validation error for a relative root, got {other:?}"),
        }

        let file = root.join("not-a-dir");
        std::fs::write(&file, b"").unwrap();
        assert!(canonical_regular_directory(&file).is_err());
        assert!(matches!(
            canonical_regular_directory(&root.join("absent")),
            Err(AwareError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound
        ));

        #[cfg(unix)]
        {
            let target = root.join("target");
            std::fs::create_dir(&target).unwrap();
            let link = root.join("link");
            std::os::unix::fs::symlink(&target, &link).unwrap();
            assert!(
                canonical_regular_directory(&link).is_err(),
                "a symlinked root lets the tree be swapped after enrollment"
            );
        }
    }

    // ---------------------------------------------------------------------
    // verify_package_signature — the sidecar that binds a manifest digest to a
    // trusted publisher key.
    // ---------------------------------------------------------------------

    /// A deterministic publisher and a `.sig` sidecar that verifies over
    /// `manifest_sha256`. Returns the temp root, the publisher record and the
    /// signature lines, so a test can corrupt exactly one thing.
    fn signature_fixture(
        manifest_sha256: &str,
    ) -> (tempfile::TempDir, PublisherRecord, Vec<String>) {
        use ed25519_dalek::{Signer as _, SigningKey};
        let signing = SigningKey::from_bytes(&[7_u8; 32]);
        let public_bytes = signing.verifying_key().to_bytes();
        let public_key_base64 = base64::engine::general_purpose::STANDARD.encode(public_bytes);
        let signature = signing.sign(&hex_digest_bytes(manifest_sha256).unwrap());
        let lines = vec![
            "ed25519-signature-v1".to_string(),
            format!("over-sha256-of: {MANIFEST_NAME}"),
            format!("sha256: {manifest_sha256}"),
            format!("public-key: {public_key_base64}"),
            format!(
                "signature: {}",
                base64::engine::general_purpose::STANDARD.encode(signature.to_bytes())
            ),
        ];
        let publisher = PublisherRecord {
            schema_version: PUBLISHER_SCHEMA.into(),
            publisher_id: "publisher.synthetic".into(),
            key_fingerprint_sha256: sha256_hex(&public_bytes),
            public_key_base64,
            trusted: true,
        };
        (tempfile::tempdir().unwrap(), publisher, lines)
    }

    fn write_signature(root: &Path, lines: &[String]) {
        std::fs::write(root.join(SIGNATURE_NAME), format!("{}\n", lines.join("\n"))).unwrap();
    }

    #[test]
    fn a_well_formed_signature_over_the_manifest_digest_verifies() {
        let (tmp, publisher, lines) = signature_fixture(ABC_SHA256);
        write_signature(tmp.path(), &lines);
        verify_package_signature(tmp.path(), ABC_SHA256, &publisher).unwrap();
    }

    #[test]
    fn a_signature_only_verifies_for_the_manifest_digest_it_was_made_over() {
        // The whole point of the sidecar: a valid signature lifted from one
        // package must not enroll another.
        let (tmp, publisher, lines) = signature_fixture(ABC_SHA256);
        write_signature(tmp.path(), &lines);
        assert!(
            verify_package_signature(tmp.path(), EMPTY_SHA256, &publisher).is_err(),
            "a signature over {ABC_SHA256} must not pass for {EMPTY_SHA256}"
        );
    }

    #[test]
    fn a_signature_must_carry_the_trusted_publishers_own_key() {
        let (tmp, publisher, lines) = signature_fixture(ABC_SHA256);
        use ed25519_dalek::{Signer as _, SigningKey};
        let other = SigningKey::from_bytes(&[9_u8; 32]);
        let mut forged = lines.clone();
        forged[3] = format!(
            "public-key: {}",
            base64::engine::general_purpose::STANDARD.encode(other.verifying_key().to_bytes())
        );
        forged[4] = format!(
            "signature: {}",
            base64::engine::general_purpose::STANDARD.encode(
                other
                    .sign(&hex_digest_bytes(ABC_SHA256).unwrap())
                    .to_bytes()
            )
        );
        write_signature(tmp.path(), &forged);
        assert!(
            verify_package_signature(tmp.path(), ABC_SHA256, &publisher).is_err(),
            "a self-consistent signature from an untrusted key must be refused"
        );
    }

    #[test]
    fn a_tampered_signature_value_does_not_verify() {
        let (tmp, publisher, lines) = signature_fixture(ABC_SHA256);
        let mut broken = lines.clone();
        broken[4] = format!(
            "signature: {}",
            base64::engine::general_purpose::STANDARD.encode([0_u8; 64])
        );
        write_signature(tmp.path(), &broken);
        assert!(verify_package_signature(tmp.path(), ABC_SHA256, &publisher).is_err());
    }

    #[test]
    fn a_signature_sidecar_must_be_exactly_the_four_expected_fields() {
        let (tmp, publisher, lines) = signature_fixture(ABC_SHA256);

        let mut wrong_schema = lines.clone();
        wrong_schema[0] = "ed25519-signature-v2".into();
        write_signature(tmp.path(), &wrong_schema);
        assert!(
            verify_package_signature(tmp.path(), ABC_SHA256, &publisher).is_err(),
            "schema"
        );

        let mut malformed = lines.clone();
        malformed.push("a line with no separator".into());
        write_signature(tmp.path(), &malformed);
        assert!(
            verify_package_signature(tmp.path(), ABC_SHA256, &publisher).is_err(),
            "no colon"
        );

        let mut repeated = lines.clone();
        repeated.push(lines[2].clone());
        write_signature(tmp.path(), &repeated);
        assert!(
            verify_package_signature(tmp.path(), ABC_SHA256, &publisher).is_err(),
            "a repeated field lets a reader pick either value"
        );

        let mut extra = lines.clone();
        extra.push("note: harmless".into());
        write_signature(tmp.path(), &extra);
        assert!(
            verify_package_signature(tmp.path(), ABC_SHA256, &publisher).is_err(),
            "the field set is closed"
        );

        for index in 1..lines.len() {
            let mut missing = lines.clone();
            missing.remove(index);
            write_signature(tmp.path(), &missing);
            assert!(
                verify_package_signature(tmp.path(), ABC_SHA256, &publisher).is_err(),
                "field at line {index} is required"
            );
        }

        let mut wrong_subject = lines.clone();
        wrong_subject[1] = "over-sha256-of: provider-package.sig".into();
        write_signature(tmp.path(), &wrong_subject);
        assert!(
            verify_package_signature(tmp.path(), ABC_SHA256, &publisher).is_err(),
            "the signature must name the manifest as its subject"
        );
    }

    #[test]
    fn a_missing_or_oversized_signature_sidecar_is_refused_not_ignored() {
        // Both halves assert the specific outcome rather than `is_err()`: a
        // sidecar of random bytes fails its schema line anyway, and a missing
        // one fails at any read, so a bare `is_err()` here would stay green
        // with the byte ceiling deleted.
        let (tmp, publisher, lines) = signature_fixture(ABC_SHA256);
        match verify_package_signature(tmp.path(), ABC_SHA256, &publisher) {
            Err(AwareError::Io(error)) => {
                assert_eq!(error.kind(), std::io::ErrorKind::NotFound, "{error}")
            }
            other => panic!("a missing sidecar must fail closed as not-found, got {other:?}"),
        }

        // Well formed in every other respect, and padded past the 64 KiB
        // ceiling, so only the ceiling can be what rejects it.
        let mut padded = lines.clone();
        padded[4] = format!("{}{}", lines[4], "A".repeat(64 * 1024));
        write_signature(tmp.path(), &padded);
        match verify_package_signature(tmp.path(), ABC_SHA256, &publisher) {
            Err(AwareError::Validation(message)) => assert!(
                message.contains("too large"),
                "the sidecar must be refused by its byte ceiling, not later: {message}"
            ),
            other => panic!("expected the byte ceiling to reject the sidecar, got {other:?}"),
        }
    }

    // ---------------------------------------------------------------------
    // verify_package_inventory — the closed allowlist over the package tree.
    // ---------------------------------------------------------------------

    #[test]
    fn the_inventory_is_a_closed_allowlist_matched_byte_for_byte() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        let mut manifest = valid_manifest();
        manifest.files[0] = PackageFile {
            path: "provider.bin".into(),
            bytes: 3,
            sha256: ABC_SHA256.into(),
        };
        std::fs::write(root.join("provider.bin"), b"abc").unwrap();
        std::fs::write(root.join(MANIFEST_NAME), b"{}").unwrap();
        std::fs::write(root.join(SIGNATURE_NAME), b"").unwrap();
        verify_package_inventory(root, &manifest).unwrap();

        // A file nobody receipted.
        std::fs::write(root.join("extra.so"), b"").unwrap();
        assert!(
            verify_package_inventory(root, &manifest).is_err(),
            "unreceipted file"
        );
        std::fs::remove_file(root.join("extra.so")).unwrap();

        // Right length, different bytes — only the digest catches this.
        std::fs::write(root.join("provider.bin"), b"abd").unwrap();
        assert!(
            verify_package_inventory(root, &manifest).is_err(),
            "content drift"
        );

        // Right digest recorded, wrong length recorded.
        std::fs::write(root.join("provider.bin"), b"abc").unwrap();
        manifest.files[0].bytes = 4;
        assert!(
            verify_package_inventory(root, &manifest).is_err(),
            "length drift"
        );

        // A receipted file that is simply not there.
        manifest.files[0].bytes = 3;
        std::fs::remove_file(root.join("provider.bin")).unwrap();
        assert!(
            verify_package_inventory(root, &manifest).is_err(),
            "missing file"
        );
    }
}
