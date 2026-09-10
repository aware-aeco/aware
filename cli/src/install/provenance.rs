//! Where an installed agent came from — the fact `update` needs and never had (#370).
//!
//! `aware agent update <id>` resolves the id in the registry and replaces whatever sits at
//! `agents/<id>/`. That is right for an agent installed FROM the registry and destructive for one
//! installed from a folder: a local fork whose `agent:` id is also a registry key was silently
//! overwritten with the registry's copy, exit 0, and the local work was unrecoverable — the staging
//! directory holds the registry payload, not yours.
//!
//! The existing hijack guard only covered the *suffix-fallback* case (a local `tekla.dev` resolving
//! to registry key `tekla`, #174). An EXACT id collision walked straight past it, because
//! `index.agents.contains_key(id)` is true and the guard's second clause never fires.
//!
//! Nothing recorded the answer, so nothing could ask. The manifest's own `provenance:` block is
//! generator metadata (`generated-by: aware-agent-builder`) — it says how the agent was AUTHORED,
//! not how it got onto this machine. This module records the second question.
//!
//! ## Where it lives, and why
//!
//! A dotfile inside the agent's own directory (`agents/<id>/.aware-install.yaml`) rather than a
//! central receipts file. It then has no lifecycle of its own: `uninstall`'s `remove_dir_all` takes
//! it, and `update`'s atomic staging-then-rename replaces it with the new one. There is no
//! `agent rename` or `agent duplicate` to carry it anywhere — those verbs are APP verbs and copy
//! `apps/`, never an agent directory. A central index would need every one of those paths to
//! remember to update it, which is the class of bug this file exists to close, not to reintroduce.
//!
//! Verified registry receipts are written before promotion and are mandatory. Legacy/local
//! source markers remain best-effort and always assess as unverified.
//!
//! It is dot-prefixed so it cannot be mistaken for agent content: `validate_agent_on_disk` reads
//! `manifest.yaml`, skills and commands by name, and nothing walks for arbitrary files.

use std::path::Path;

use serde::{Deserialize, Serialize};

/// The filename, dot-prefixed so it reads as metadata rather than content.
pub(crate) const FILE: &str = ".aware-install.yaml";

/// How an installed agent got onto this machine.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "source", rename_all = "lowercase")]
pub enum InstallSource {
    /// Fetched from the registry. `key` is the registry entry (which may differ from the agent id —
    /// `allplan-2024` installs as `allplan-2024.0`); `version` is the key that was resolved.
    Registry {
        key: String,
        version: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        manifest_agent: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        manifest_version: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        entry_digest: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installed_digest: Option<String>,
        #[serde(default)]
        official_source: bool,
    },
    /// Copied from a folder on this machine. `path` is where, so a refusal can name it.
    Local { path: String },
}

/// Record where an agent was just installed from. Best-effort by design.
///
/// A failure here must not fail the install: the agent IS on disk and working, and losing the
/// marker degrades to "unknown provenance", which the reader already handles. Failing the install
/// instead would turn a metadata problem into a functional one — which is why this returns nothing
/// to check rather than a `Result` a caller might be tempted to `?`.
pub fn write(agent_dir: &Path, source: &InstallSource) {
    if let Ok(yaml) = serde_yaml::to_string(source) {
        let _ = std::fs::write(agent_dir.join(FILE), yaml);
    }
}

/// Write a receipt whose presence is part of the verified-install transaction.
pub fn write_required(
    agent_dir: &Path,
    source: &InstallSource,
) -> Result<(), crate::error::AwareError> {
    let yaml = serde_yaml::to_string(source)?;
    std::fs::write(agent_dir.join(FILE), yaml)?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct BundleProvenance {
    pub verified: bool,
    pub reason: String,
    pub registry_key: Option<String>,
    pub registry_version: Option<String>,
    pub manifest_agent: String,
    pub manifest_version: String,
    pub entry_digest: Option<String>,
    pub installed_digest: Option<String>,
    pub executable_attested: bool,
}

pub fn assess_against_index(
    agent_dir: &Path,
    manifest_agent: &str,
    manifest_version: &str,
    trusted_index: Option<&crate::registry::Index>,
) -> BundleProvenance {
    let mut result = BundleProvenance {
        verified: false,
        reason: "missing or legacy installation receipt".into(),
        registry_key: None,
        registry_version: None,
        manifest_agent: manifest_agent.into(),
        manifest_version: manifest_version.into(),
        entry_digest: None,
        installed_digest: None,
        // Bundle integrity cannot attest a PATH executable, managed sidecar, or REST service.
        executable_attested: false,
    };
    let Some(InstallSource::Registry {
        key,
        version,
        manifest_agent: bound_agent,
        manifest_version: bound_version,
        entry_digest,
        installed_digest,
        official_source,
    }) = read(agent_dir)
    else {
        result.reason = if matches!(read(agent_dir), Some(InstallSource::Local { .. })) {
            "locally installed bundle".into()
        } else {
            result.reason
        };
        return result;
    };
    result.registry_key = Some(key);
    result.registry_version = Some(version);
    result.entry_digest = entry_digest.clone();
    result.installed_digest = installed_digest.clone();
    if !official_source {
        result.reason =
            "registry source was custom, cached, offline, or otherwise untrusted".into();
        return result;
    }
    if bound_agent.as_deref() != Some(manifest_agent)
        || bound_version.as_deref() != Some(manifest_version)
    {
        result.reason = "receipt manifest identity does not match installed manifest".into();
        return result;
    }
    let (Some(expected), Some(recorded)) = (entry_digest, installed_digest) else {
        result.reason = "official receipt has no bundle digest".into();
        return result;
    };
    let Some(index) =
        trusted_index.filter(|i| i.trust == crate::registry::RegistryTrust::FreshOfficial)
    else {
        result.reason = "no freshly fetched official registry index was available".into();
        return result;
    };
    let registry_expected = result
        .registry_key
        .as_deref()
        .and_then(|key| {
            result
                .registry_version
                .as_deref()
                .and_then(|version| index.resolve(key, Some(version)).ok())
        })
        .and_then(|(_, release)| release.bundle_digest.as_deref());
    if registry_expected != Some(expected.as_str()) {
        result.reason = "receipt digest does not match the fresh official registry release".into();
        return result;
    }
    match super::integrity::tree_digest(agent_dir) {
        Ok(actual) if actual == expected && actual == recorded => {
            result.verified = true;
            result.reason = "official registry bundle digest matches installed content".into();
        }
        Ok(actual) => {
            result.installed_digest = Some(actual);
            result.reason = "installed bundle content does not match its official receipt".into();
        }
        Err(error) => result.reason = format!("installed bundle cannot be verified: {error}"),
    }
    result
}

/// Digest to pin in a compiled app. This does not claim registry verification;
/// it only makes post-compile byte changes visible at run time.
pub fn receipt_digest_pin(
    agent_dir: &Path,
    manifest_agent: &str,
    manifest_version: &str,
) -> Option<String> {
    let InstallSource::Registry {
        manifest_agent: a,
        manifest_version: v,
        installed_digest,
        ..
    } = read(agent_dir)?
    else {
        return None;
    };
    if a.as_deref() == Some(manifest_agent) && v.as_deref() == Some(manifest_version) {
        installed_digest
    } else {
        None
    }
}

pub fn claims_official(agent_dir: &Path) -> bool {
    matches!(read(agent_dir), Some(InstallSource::Registry {
        official_source: true, key, version,
        manifest_agent: Some(_), manifest_version: Some(_),
        entry_digest: Some(entry), installed_digest: Some(installed),
    }) if !key.is_empty() && !version.is_empty()
        && crate::registry::index::is_bundle_digest(&entry)
        && crate::registry::index::is_bundle_digest(&installed))
}

/// Where this agent came from, or `None` when nothing says.
///
/// `None` is not "registry" — it is the honest "this predates the marker, or it was lost". Callers
/// must decide what to do with an unknown rather than assuming the safe-for-them answer; see
/// `update_agent_from_registry`, which falls back to asking the registry whether the installed
/// VERSION is one it publishes.
pub fn read(agent_dir: &Path) -> Option<InstallSource> {
    let text = std::fs::read_to_string(agent_dir.join(FILE)).ok()?;
    serde_yaml::from_str(&text).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn a_written_source_reads_back_as_itself() {
        let tmp = tempfile::tempdir().unwrap();
        for source in [
            InstallSource::Registry {
                key: "tekla".into(),
                version: "2025.0.1".into(),
                manifest_agent: None,
                manifest_version: None,
                entry_digest: None,
                installed_digest: None,
                official_source: false,
            },
            InstallSource::Local {
                path: "C:/work/my-fork".into(),
            },
        ] {
            write(tmp.path(), &source);
            assert_eq!(read(tmp.path()), Some(source));
        }
    }

    #[test]
    fn no_marker_reads_as_unknown_not_as_registry() {
        // The distinction the whole module turns on. An agent installed before this existed has no
        // marker, and calling that "registry" would be exactly the assumption that destroyed local
        // forks — quietly, and for the people most likely to have one.
        let tmp = tempfile::tempdir().unwrap();
        assert_eq!(read(tmp.path()), None);
    }

    #[test]
    fn an_unreadable_marker_reads_as_unknown_rather_than_erroring() {
        // A corrupt marker must not break `update` — it is metadata about an agent that is
        // otherwise fine. Unknown is the same answer as absent, and the caller's fallback covers
        // both.
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join(FILE), "source: [not\n  yaml\n").unwrap();
        assert_eq!(read(tmp.path()), None);
    }

    #[test]
    fn writing_is_best_effort_and_never_panics() {
        // Into a directory that does not exist — the install still succeeded, so this must be a
        // no-op rather than a failure.
        write(
            Path::new("no/such/directory/anywhere"),
            &InstallSource::Local { path: "x".into() },
        );
    }

    #[test]
    fn verification_requires_fresh_index_and_detects_tampering() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(
            tmp.path().join("manifest.yaml"),
            b"agent: probe\nversion: 1.0.0\n",
        )
        .unwrap();
        let digest = crate::install::integrity::tree_digest(tmp.path()).unwrap();
        write_required(
            tmp.path(),
            &InstallSource::Registry {
                key: "probe-release".into(),
                version: "2026.1.0".into(),
                manifest_agent: Some("probe".into()),
                manifest_version: Some("1.0.0".into()),
                entry_digest: Some(digest.clone()),
                installed_digest: Some(digest.clone()),
                official_source: true,
            },
        )
        .unwrap();
        let mut versions = BTreeMap::new();
        versions.insert(
            "2026.1.0".into(),
            crate::registry::VersionEntry {
                tarball: "https://example.invalid/a.tar.gz".into(),
                subdir: "probe".into(),
                bundle_digest: Some(digest),
            },
        );
        let mut agents = BTreeMap::new();
        agents.insert(
            "probe-release".into(),
            crate::registry::IndexEntry {
                versions,
                ..Default::default()
            },
        );
        let index = crate::registry::Index {
            version: "1.0".into(),
            updated_at: "now".into(),
            agents,
            bundles: BTreeMap::new(),
            trust: crate::registry::RegistryTrust::FreshOfficial,
        };
        assert!(
            !assess_against_index(tmp.path(), "probe", "1.0.0", None).verified,
            "receipt alone is not trust"
        );
        assert!(assess_against_index(tmp.path(), "probe", "1.0.0", Some(&index)).verified);
        std::fs::write(
            tmp.path().join("manifest.yaml"),
            b"agent: probe\nversion: 1.0.0\n# tampered\n",
        )
        .unwrap();
        assert!(!assess_against_index(tmp.path(), "probe", "1.0.0", Some(&index)).verified);
    }
}
