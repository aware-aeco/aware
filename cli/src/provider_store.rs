//! Closed, format-neutral trust store for signed model-provider packages.

use std::collections::{BTreeMap, BTreeSet};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};

use base64::Engine;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
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
const MAX_CONTROL_BYTES: u64 = 1024 * 1024;

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
        let path = self.selection_path(format_id);
        let prior = read_optional_json::<SelectionRecord>(&path)?;
        if let Some(selection) = &prior
            && (selection.schema_version != SELECTION_SCHEMA
                || selection.format_id != format_id
                || selection.generation == 0
                || selection
                    .previous_manifest_sha256
                    .iter()
                    .any(|digest| validate_sha256(digest, "previous manifest sha256").is_err()))
        {
            return Err(AwareError::Validation(
                "existing provider selection record is invalid".into(),
            ));
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
                let selection: SelectionRecord = read_json(&entry.path())?;
                selections.insert(selection.format_id, selection.active_manifest_sha256);
            }
        }
        let mut packages = Vec::new();
        let packages_dir = self.root.join("packages");
        if packages_dir.is_dir() {
            for entry in std::fs::read_dir(packages_dir)? {
                let entry = entry?;
                let package: PackageRecord = read_json(&entry.path())?;
                if package.revoked
                    || !package.enrolled
                    || format.is_some_and(|id| id != package.manifest.format_id)
                {
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
    Version::parse(&manifest.package_version)
        .map_err(|_| AwareError::Validation("provider package version must be semver".into()))?;
    Version::parse(&manifest.minimum_aware_version)
        .map_err(|_| AwareError::Validation("minimum AWARE version must be semver".into()))?;
    if let Some(maximum) = &manifest.maximum_aware_version {
        Version::parse(maximum)
            .map_err(|_| AwareError::Validation("maximum AWARE version must be semver".into()))?;
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
