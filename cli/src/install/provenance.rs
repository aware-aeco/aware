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

    /// A syntactically valid bundle digest that is not the digest of anything here — the
    /// shape `claims_official` accepts, so a refusal on one of these is a refusal on the
    /// FIELD being wrong, never on the string being malformed.
    const WRONG_DIGEST: &str =
        "sha256:1111111111111111111111111111111111111111111111111111111111111111";

    /// An agent directory carrying `manifest.yaml` for `probe@1.0.0` and a receipt that is
    /// correct in every respect: official, identity-bound, and digest-bound to the tree as
    /// it stands. Every test below weakens exactly ONE thing about it, so a verdict that
    /// changes can only be attributed to that one weakening.
    fn official_install(edit: impl FnOnce(&mut InstallSource)) -> (tempfile::TempDir, String) {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(
            tmp.path().join("manifest.yaml"),
            b"agent: probe\nversion: 1.0.0\n",
        )
        .unwrap();
        let digest = crate::install::integrity::tree_digest(tmp.path()).unwrap();
        let mut source = InstallSource::Registry {
            key: "probe-release".into(),
            version: "2026.1.0".into(),
            manifest_agent: Some("probe".into()),
            manifest_version: Some("1.0.0".into()),
            entry_digest: Some(digest.clone()),
            installed_digest: Some(digest.clone()),
            official_source: true,
        };
        edit(&mut source);
        write_required(tmp.path(), &source).unwrap();
        (tmp, digest)
    }

    /// A one-release index publishing `probe-release@2026.1.0` at `bundle_digest`.
    fn index_publishing(
        bundle_digest: &str,
        trust: crate::registry::RegistryTrust,
    ) -> crate::registry::Index {
        let mut versions = BTreeMap::new();
        versions.insert(
            "2026.1.0".into(),
            crate::registry::VersionEntry {
                tarball: "https://example.invalid/a.tar.gz".into(),
                subdir: "probe".into(),
                bundle_digest: Some(bundle_digest.into()),
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
        crate::registry::Index {
            version: "1.0".into(),
            updated_at: "now".into(),
            agents,
            bundles: BTreeMap::new(),
            trust,
        }
    }

    #[test]
    fn claims_official_accepts_only_a_complete_official_receipt() {
        // `claims_official` is the cheap gate in front of the expensive one: `app run` refuses
        // an agent outright when it is false (commands/app.rs), and `agent describe` skips
        // fetching the official index entirely. Every conjunct is therefore load-bearing, and
        // none of them had a test. Each case below is the full receipt minus ONE field.
        let (complete, digest) = official_install(|_| {});
        assert!(
            claims_official(complete.path()),
            "a complete official receipt is the one thing that passes"
        );

        /// One way of writing an otherwise-complete receipt short, and the name the
        /// assertion reports it by.
        type Weakening = (&'static str, fn(&mut InstallSource));

        let weakenings: [Weakening; 7] = [
            ("not flagged official", |s| {
                if let InstallSource::Registry {
                    official_source, ..
                } = s
                {
                    *official_source = false;
                }
            }),
            ("empty registry key", |s| {
                if let InstallSource::Registry { key, .. } = s {
                    key.clear();
                }
            }),
            ("empty registry version", |s| {
                if let InstallSource::Registry { version, .. } = s {
                    version.clear();
                }
            }),
            ("no manifest agent bound", |s| {
                if let InstallSource::Registry { manifest_agent, .. } = s {
                    *manifest_agent = None;
                }
            }),
            ("no manifest version bound", |s| {
                if let InstallSource::Registry {
                    manifest_version, ..
                } = s
                {
                    *manifest_version = None;
                }
            }),
            ("no entry digest", |s| {
                if let InstallSource::Registry { entry_digest, .. } = s {
                    *entry_digest = None;
                }
            }),
            ("no installed digest", |s| {
                if let InstallSource::Registry {
                    installed_digest, ..
                } = s
                {
                    *installed_digest = None;
                }
            }),
        ];
        for (what, weaken) in weakenings {
            let (dir, _) = official_install(weaken);
            assert!(
                !claims_official(dir.path()),
                "a receipt with {what} must not claim officialness"
            );
        }

        // A digest of the wrong SHAPE is rejected without ever reading the tree: an
        // attacker-supplied receipt is a text file, so the field being present is not the
        // same as the field being a digest.
        for malformed in [
            "sha256:short",
            "sha256:1111111111111111111111111111111111111111111111111111111111111111111",
            "sha256:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
            "sha512:1111111111111111111111111111111111111111111111111111111111111111",
            "SHA256:1111111111111111111111111111111111111111111111111111111111111111",
        ] {
            // Each of these is rejected on SHAPE; the point here is only that both digest
            // fields are shape-checked, not just one. The rules themselves are exercised
            // against `is_bundle_digest` directly, in registry::index.
            let malformed = malformed.to_string();
            let entry_bad = official_install(|s| {
                if let InstallSource::Registry { entry_digest, .. } = s {
                    *entry_digest = Some(malformed.clone());
                }
            });
            assert!(
                !claims_official(entry_bad.0.path()),
                "entry digest {malformed:?} is not a bundle digest"
            );
            let installed_bad = official_install(|s| {
                if let InstallSource::Registry {
                    installed_digest, ..
                } = s
                {
                    *installed_digest = Some(malformed.clone());
                }
            });
            assert!(
                !claims_official(installed_bad.0.path()),
                "installed digest {malformed:?} is not a bundle digest"
            );
        }

        // A local install and a missing receipt both fail the gate.
        let local = tempfile::tempdir().unwrap();
        write(
            local.path(),
            &InstallSource::Local {
                path: "/work/fork".into(),
            },
        );
        assert!(!claims_official(local.path()));
        let empty = tempfile::tempdir().unwrap();
        assert!(!claims_official(empty.path()));

        // Sanity: the fixture's digest really is the tree's, so the passing case above
        // passed on its merits rather than on a digest nobody checked.
        assert_eq!(
            crate::install::integrity::tree_digest(complete.path()).unwrap(),
            digest
        );
    }

    #[test]
    fn a_receipt_cannot_attest_itself_without_the_fresh_official_registry() {
        // The property that makes verification worth anything: the digest in the receipt is
        // checked AGAINST the registry, not merely against the tree. A receipt forged to be
        // internally consistent — entry digest, installed digest and the actual tree content
        // all agreeing — must still be refused, because nothing official vouches for it.
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(
            tmp.path().join("manifest.yaml"),
            b"agent: probe\nversion: 1.0.0\n# forged\n",
        )
        .unwrap();
        let forged = crate::install::integrity::tree_digest(tmp.path()).unwrap();
        write_required(
            tmp.path(),
            &InstallSource::Registry {
                key: "probe-release".into(),
                version: "2026.1.0".into(),
                manifest_agent: Some("probe".into()),
                manifest_version: Some("1.0.0".into()),
                entry_digest: Some(forged.clone()),
                installed_digest: Some(forged.clone()),
                official_source: true,
            },
        )
        .unwrap();
        assert!(
            claims_official(tmp.path()),
            "the forgery is internally consistent, so the cheap gate lets it through"
        );

        // The registry publishes a DIFFERENT digest for that release, so the assessment
        // refuses — and says the receipt, not the content, is what failed.
        let index = index_publishing(WRONG_DIGEST, crate::registry::RegistryTrust::FreshOfficial);
        let verdict = assess_against_index(tmp.path(), "probe", "1.0.0", Some(&index));
        assert!(!verdict.verified, "{}", verdict.reason);
        assert!(
            verdict
                .reason
                .contains("does not match the fresh official registry"),
            "the refusal must name the registry cross-check: {}",
            verdict.reason
        );
    }

    #[test]
    fn every_way_a_receipt_falls_short_is_refused_and_named() {
        // `verified` is false for all of these, so the REASON is the only thing that
        // distinguishes them — and `app run` prints it verbatim under
        // E_APP_AGENT_BUNDLE_UNVERIFIED, which is how an operator finds out what to fix.
        // Each fixture is weakened in exactly one way and is otherwise verifiable, so an
        // assessment that stopped performing that particular check would report `verified`.

        // (a) Installed from a folder — not a registry receipt at all.
        let local = tempfile::tempdir().unwrap();
        write(
            local.path(),
            &InstallSource::Local {
                path: "/work/fork".into(),
            },
        );
        let verdict = assess_against_index(local.path(), "probe", "1.0.0", None);
        assert!(!verdict.verified);
        assert_eq!(verdict.reason, "locally installed bundle");
        assert_eq!(verdict.registry_key, None);

        // (b) A registry receipt that did not come from a fresh official fetch.
        let (unofficial, digest) = official_install(|s| {
            if let InstallSource::Registry {
                official_source, ..
            } = s
            {
                *official_source = false;
            }
        });
        let fresh = index_publishing(&digest, crate::registry::RegistryTrust::FreshOfficial);
        let verdict = assess_against_index(unofficial.path(), "probe", "1.0.0", Some(&fresh));
        assert!(!verdict.verified, "{}", verdict.reason);
        assert!(
            verdict.reason.contains("untrusted"),
            "unexpected reason: {}",
            verdict.reason
        );
        assert_eq!(
            verdict.registry_key.as_deref(),
            Some("probe-release"),
            "the receipt is still reported even when it is not trusted"
        );

        // (c) A receipt bound to a different manifest than the one on disk — the shape a
        //     receipt copied from another agent's directory takes.
        let (swapped, digest) = official_install(|s| {
            if let InstallSource::Registry { manifest_agent, .. } = s {
                *manifest_agent = Some("other-probe".into());
            }
        });
        let fresh = index_publishing(&digest, crate::registry::RegistryTrust::FreshOfficial);
        let verdict = assess_against_index(swapped.path(), "probe", "1.0.0", Some(&fresh));
        assert!(!verdict.verified, "{}", verdict.reason);
        assert!(
            verdict.reason.contains("identity does not match"),
            "unexpected reason: {}",
            verdict.reason
        );

        // (d) Official and identity-bound, but carrying no digest to check.
        let (undigested, digest) = official_install(|s| {
            if let InstallSource::Registry { entry_digest, .. } = s {
                *entry_digest = None;
            }
        });
        let fresh = index_publishing(&digest, crate::registry::RegistryTrust::FreshOfficial);
        let verdict = assess_against_index(undigested.path(), "probe", "1.0.0", Some(&fresh));
        assert!(!verdict.verified, "{}", verdict.reason);
        assert!(
            verdict.reason.contains("no bundle digest"),
            "unexpected reason: {}",
            verdict.reason
        );

        // (e) A complete receipt, a matching index — but the index was served from cache
        //     rather than freshly fetched from the official registry. Trust is a property of
        //     the FETCH, so a stale copy of the right bytes is still not an attestation.
        let (complete, digest) = official_install(|_| {});
        let stale = index_publishing(&digest, crate::registry::RegistryTrust::Unverified);
        let verdict = assess_against_index(complete.path(), "probe", "1.0.0", Some(&stale));
        assert!(!verdict.verified, "{}", verdict.reason);
        assert!(
            verdict
                .reason
                .contains("no freshly fetched official registry"),
            "unexpected reason: {}",
            verdict.reason
        );

        // (f) The same receipt against a FRESH index publishing those exact bytes verifies —
        //     without which every assertion above would pass for the wrong reason.
        let fresh = index_publishing(&digest, crate::registry::RegistryTrust::FreshOfficial);
        let verdict = assess_against_index(complete.path(), "probe", "1.0.0", Some(&fresh));
        assert!(verdict.verified, "{}", verdict.reason);
        assert_eq!(verdict.installed_digest.as_deref(), Some(digest.as_str()));
        assert!(
            !verdict.executable_attested,
            "a bundle digest never attests a PATH executable or sidecar"
        );
    }

    #[test]
    fn a_tampered_bundle_reports_the_digest_it_actually_has() {
        // The refusal has to be actionable: reporting the RECEIPT's digest back would tell an
        // operator only what they already believed. `assess_against_index` overwrites
        // `installed_digest` with what it measured, and that substitution is the difference
        // between "something is wrong" and "here is what is on your disk".
        let (tmp, receipt_digest) = official_install(|_| {});
        let index = index_publishing(
            &receipt_digest,
            crate::registry::RegistryTrust::FreshOfficial,
        );
        std::fs::write(tmp.path().join("skill.md"), b"# added after install\n").unwrap();
        let measured = crate::install::integrity::tree_digest(tmp.path()).unwrap();
        assert_ne!(measured, receipt_digest, "the fixture must really differ");

        let verdict = assess_against_index(tmp.path(), "probe", "1.0.0", Some(&index));
        assert!(!verdict.verified);
        assert!(
            verdict
                .reason
                .contains("does not match its official receipt"),
            "unexpected reason: {}",
            verdict.reason
        );
        assert_eq!(
            verdict.installed_digest.as_deref(),
            Some(measured.as_str()),
            "the reported digest must be the measured one, not the receipt's claim"
        );
    }

    #[test]
    fn a_digest_pin_is_only_issued_for_the_manifest_it_was_bound_to() {
        // `app compile` pins this digest into the lockfile (app_lock.rs), and a run then fails
        // if the agent's bytes moved. Handing back a digest bound to some OTHER manifest would
        // pin the wrong bytes, so the identity check is the whole function.
        let (matching, digest) = official_install(|_| {});
        assert_eq!(
            receipt_digest_pin(matching.path(), "probe", "1.0.0").as_deref(),
            Some(digest.as_str())
        );

        // Same receipt, asked about a different agent id / version than it records.
        assert_eq!(receipt_digest_pin(matching.path(), "other", "1.0.0"), None);
        assert_eq!(receipt_digest_pin(matching.path(), "probe", "9.9.9"), None);

        // A receipt written before identity binding existed carries no agent/version, so it
        // cannot vouch for any manifest and must pin nothing.
        let (legacy, _) = official_install(|s| {
            if let InstallSource::Registry {
                manifest_agent,
                manifest_version,
                ..
            } = s
            {
                *manifest_agent = None;
                *manifest_version = None;
            }
        });
        assert_eq!(receipt_digest_pin(legacy.path(), "probe", "1.0.0"), None);

        // A local install and a missing receipt pin nothing either.
        let local = tempfile::tempdir().unwrap();
        write(
            local.path(),
            &InstallSource::Local {
                path: "/work/fork".into(),
            },
        );
        assert_eq!(receipt_digest_pin(local.path(), "probe", "1.0.0"), None);
        let empty = tempfile::tempdir().unwrap();
        assert_eq!(receipt_digest_pin(empty.path(), "probe", "1.0.0"), None);
    }

    #[test]
    fn a_required_receipt_fails_loudly_where_a_best_effort_one_does_not() {
        // The two writers differ in exactly one way, and it is the reason both exist:
        // `write_required` is part of the verified-install transaction (a receipt that did not
        // land must abort the promotion), while `write` is metadata that must never fail an
        // install that already succeeded.
        let missing = Path::new("no/such/directory/anywhere");
        let source = InstallSource::Local { path: "x".into() };
        write(missing, &source); // no panic, no Result to check
        assert!(
            write_required(missing, &source).is_err(),
            "a required receipt that cannot be written must fail the install"
        );

        let tmp = tempfile::tempdir().unwrap();
        write_required(tmp.path(), &source).unwrap();
        assert_eq!(read(tmp.path()), Some(source));
    }
}
