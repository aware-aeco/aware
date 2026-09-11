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

    /// A well-formed bundle digest that is the digest of nothing here. Used as the value one
    /// side of a comparison carries when the other must disagree with it, so the refusal
    /// lands on the two values differing rather than on a malformed field.
    const WRONG_DIGEST: &str =
        "sha256:1111111111111111111111111111111111111111111111111111111111111111";

    /// A second one, distinct from `WRONG_DIGEST`, for the cases that need the receipt's two
    /// digest fields to disagree with each other as well as with the tree.
    const OTHER_DIGEST: &str =
        "sha256:2222222222222222222222222222222222222222222222222222222222222222";

    /// An agent directory carrying `manifest.yaml` for `probe@1.0.0` and a receipt that is
    /// correct in every respect: official, identity-bound, and digest-bound to the tree as
    /// it stands. Most tests below write it short in exactly one way, so a verdict that
    /// changes is attributable to that one weakening.
    ///
    /// The ordering here is load-bearing and looks accidental: `tree_digest` is measured
    /// BEFORE `write_required` drops the receipt into the same directory. That works only
    /// because `integrity::collect` skips `provenance::FILE` at the bundle root, and it
    /// mirrors the real install, which digests the staging dir, writes the receipt into it,
    /// then renames (`install/registry.rs`). Measure after and every digest here shifts.
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
        // `claims_official` is the cheap gate in front of the expensive one:
        // `app run --require-verified-agents` refuses an agent outright when it is false
        // (commands/app.rs), and `agent describe` skips fetching the official index entirely.
        // Every conjunct is therefore load-bearing, and no unit test covered the individual
        // conjuncts. Each case below writes the receipt short in exactly one way.
        let (complete, _) = official_install(|_| {});
        assert!(
            claims_official(complete.path()),
            "a complete official receipt is the one thing that passes"
        );

        type Weakening = (&'static str, fn(&mut InstallSource));

        let weakenings: &[Weakening] = &[
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

        // A local install and a missing receipt both fail the gate — for DIFFERENT reasons,
        // which is why the local receipt is read back first. Without that, a `write` that
        // silently wrote nothing (it is best-effort by design) would leave an empty directory,
        // and this case would assert exactly what the `empty` case below already asserts.
        let local = tempfile::tempdir().unwrap();
        let local_receipt = InstallSource::Local {
            path: "/work/fork".into(),
        };
        write(local.path(), &local_receipt);
        assert_eq!(
            read(local.path()),
            Some(local_receipt),
            "the fixture must really carry a local receipt"
        );
        assert!(!claims_official(local.path()));
        let empty = tempfile::tempdir().unwrap();
        assert!(!claims_official(empty.path()));
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
        // `assess_against_index` has ten outcomes and `verified` is false in nine, so the
        // REASON is the only thing that distinguishes them — and
        // `app run --require-verified-agents` prints it verbatim under
        // E_APP_AGENT_BUNDLE_UNVERIFIED, which is how an operator finds out what to fix.
        // Each fixture below is written short in exactly one way and is otherwise complete,
        // so the named reason is attributable to that single weakening. For cases (b), (c),
        // (d1), (d2) and (e) the fixture is otherwise VERIFIABLE, so an assessment that
        // stopped performing that particular check would report `verified`.

        // (a) No receipt at all — the default reason, and the likeliest real state of an
        //     unverified agent: one installed before receipts existed, or whose marker was
        //     lost. It is also what a branch that forgot to set `reason` would report, so
        //     leaving it unpinned is what lets "no receipt" quietly become "verified".
        let bare = tempfile::tempdir().unwrap();
        let verdict = assess_against_index(bare.path(), "probe", "1.0.0", None);
        assert!(!verdict.verified);
        assert_eq!(verdict.reason, "missing or legacy installation receipt");
        assert_eq!(verdict.registry_key, None);

        // (a2) Installed from a folder — a receipt, but not a registry one. Read back first,
        //      or a `write` that silently wrote nothing would make this a copy of (a).
        let local = tempfile::tempdir().unwrap();
        let local_receipt = InstallSource::Local {
            path: "/work/fork".into(),
        };
        write(local.path(), &local_receipt);
        assert_eq!(read(local.path()), Some(local_receipt));
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
        //     receipt copied from another agent's directory takes. BOTH halves of the
        //     identity check get a case: the gate is an `||`, so one half can be deleted
        //     while the other still catches the case that exercises it.
        for (what, weaken) in [
            (
                "agent",
                (|s: &mut InstallSource| {
                    if let InstallSource::Registry { manifest_agent, .. } = s {
                        *manifest_agent = Some("other-probe".into());
                    }
                }) as fn(&mut InstallSource),
            ),
            ("version", |s: &mut InstallSource| {
                if let InstallSource::Registry {
                    manifest_version, ..
                } = s
                {
                    *manifest_version = Some("9.9.9".into());
                }
            }),
        ] {
            let (swapped, digest) = official_install(weaken);
            let fresh = index_publishing(&digest, crate::registry::RegistryTrust::FreshOfficial);
            let verdict = assess_against_index(swapped.path(), "probe", "1.0.0", Some(&fresh));
            assert!(!verdict.verified, "bound {what}: {}", verdict.reason);
            assert!(
                verdict.reason.contains("identity does not match"),
                "bound {what}: unexpected reason: {}",
                verdict.reason
            );
        }

        // (d) Official and identity-bound, but carrying no digest to check. Again both
        //     halves: the let-else requires BOTH fields, and either one alone can be dropped
        //     from it without the other's case noticing.
        for (what, weaken) in [
            (
                "entry",
                (|s: &mut InstallSource| {
                    if let InstallSource::Registry { entry_digest, .. } = s {
                        *entry_digest = None;
                    }
                }) as fn(&mut InstallSource),
            ),
            ("installed", |s: &mut InstallSource| {
                if let InstallSource::Registry {
                    installed_digest, ..
                } = s
                {
                    *installed_digest = None;
                }
            }),
        ] {
            let (undigested, digest) = official_install(weaken);
            let fresh = index_publishing(&digest, crate::registry::RegistryTrust::FreshOfficial);
            let verdict = assess_against_index(undigested.path(), "probe", "1.0.0", Some(&fresh));
            assert!(!verdict.verified, "no {what} digest: {}", verdict.reason);
            assert!(
                verdict.reason.contains("no bundle digest"),
                "no {what} digest: unexpected reason: {}",
                verdict.reason
            );
        }

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

        // (f) A receipt whose two digest fields disagree. The final comparison demands the
        //     measured tree equal BOTH of them, and the base fixture sets them to one value,
        //     so without this case either conjunct could be deleted unnoticed. Here the
        //     registry publishes the entry digest and the tree matches it, so only the
        //     `recorded` half can refuse.
        let (mismatched, tree) = official_install(|s| {
            if let InstallSource::Registry {
                installed_digest, ..
            } = s
            {
                *installed_digest = Some(OTHER_DIGEST.into());
            }
        });
        let fresh = index_publishing(&tree, crate::registry::RegistryTrust::FreshOfficial);
        let verdict = assess_against_index(mismatched.path(), "probe", "1.0.0", Some(&fresh));
        assert!(!verdict.verified, "{}", verdict.reason);
        assert!(
            verdict
                .reason
                .contains("does not match its official receipt"),
            "unexpected reason: {}",
            verdict.reason
        );

        // (f2) The mirror, so the other conjunct is covered too: the registry and the receipt
        //      agree on a digest that is NOT the tree's, while the receipt's own
        //      `installed_digest` does match the tree. Only the `expected` half can refuse.
        let (drifted, _) = official_install(|s| {
            if let InstallSource::Registry { entry_digest, .. } = s {
                *entry_digest = Some(WRONG_DIGEST.into());
            }
        });
        let fresh = index_publishing(WRONG_DIGEST, crate::registry::RegistryTrust::FreshOfficial);
        let verdict = assess_against_index(drifted.path(), "probe", "1.0.0", Some(&fresh));
        assert!(!verdict.verified, "{}", verdict.reason);
        assert!(
            verdict
                .reason
                .contains("does not match its official receipt"),
            "unexpected reason: {}",
            verdict.reason
        );

        // (g) A bundle whose digest cannot be MEASURED at all is refused differently from one
        //     that measured cleanly and disagreed. `tree_digest` refuses symlink indirection,
        //     and nothing pinned that the refusal propagates rather than being swallowed.
        #[cfg(unix)]
        {
            let (broken, tree) = official_install(|_| {});
            std::os::unix::fs::symlink("manifest.yaml", broken.path().join("alias.yaml")).unwrap();
            let fresh = index_publishing(&tree, crate::registry::RegistryTrust::FreshOfficial);
            let verdict = assess_against_index(broken.path(), "probe", "1.0.0", Some(&fresh));
            assert!(!verdict.verified, "{}", verdict.reason);
            assert!(
                verdict
                    .reason
                    .starts_with("installed bundle cannot be verified"),
                "unexpected reason: {}",
                verdict.reason
            );
        }

        // (h) The complete receipt against a FRESH index publishing those exact bytes
        //     verifies — without which every assertion above would pass for the wrong reason.
        let fresh = index_publishing(&digest, crate::registry::RegistryTrust::FreshOfficial);
        let verdict = assess_against_index(complete.path(), "probe", "1.0.0", Some(&fresh));
        assert!(verdict.verified, "{}", verdict.reason);
        assert_eq!(verdict.installed_digest.as_deref(), Some(digest.as_str()));
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
        // pin the wrong bytes, which is what the identity check is for.
        let (matching, digest) = official_install(|_| {});
        assert_eq!(
            receipt_digest_pin(matching.path(), "probe", "1.0.0").as_deref(),
            Some(digest.as_str())
        );

        // It is the INSTALLED digest that gets pinned, not the entry digest. The two are equal
        // on a healthy receipt, so only a fixture where they differ can tell the fields apart —
        // and pinning the entry digest would pin what the registry advertised rather than what
        // landed on this disk.
        let (split, _) = official_install(|s| {
            if let InstallSource::Registry { entry_digest, .. } = s {
                *entry_digest = Some(WRONG_DIGEST.into());
            }
        });
        assert_eq!(
            receipt_digest_pin(split.path(), "probe", "1.0.0").as_deref(),
            Some(
                crate::install::integrity::tree_digest(split.path())
                    .unwrap()
                    .as_str()
            ),
            "the pin must come from installed_digest, not entry_digest"
        );

        // Deliberately NOT a trust gate: an unofficial receipt still pins, because the pin only
        // makes post-compile byte changes visible. `app run --require-verified-agents` is the
        // trust gate. Pinned here so a later hardening edit cannot quietly change the contract.
        let (unofficial, digest) = official_install(|s| {
            if let InstallSource::Registry {
                official_source, ..
            } = s
            {
                *official_source = false;
            }
        });
        assert_eq!(
            receipt_digest_pin(unofficial.path(), "probe", "1.0.0").as_deref(),
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

        // A local install and a missing receipt pin nothing either. The local receipt is read
        // back first so this case cannot silently degrade into the empty-directory one below.
        let local = tempfile::tempdir().unwrap();
        let local_receipt = InstallSource::Local {
            path: "/work/fork".into(),
        };
        write(local.path(), &local_receipt);
        assert_eq!(read(local.path()), Some(local_receipt));
        assert_eq!(receipt_digest_pin(local.path(), "probe", "1.0.0"), None);
        let empty = tempfile::tempdir().unwrap();
        assert_eq!(receipt_digest_pin(empty.path(), "probe", "1.0.0"), None);
    }

    #[test]
    fn a_required_receipt_fails_loudly_where_a_best_effort_one_does_not() {
        // The two writers differ in exactly one way, and it is the reason both exist:
        // `write_required` is part of the verified-install transaction (a receipt that did not
        // land must abort the promotion), while `write` is metadata that must never fail an
        // install that already succeeded. Same input, same failure, opposite contracts — which
        // is the whole of what this test pins.
        let missing = Path::new("no/such/directory/anywhere");
        let source = InstallSource::Local { path: "x".into() };
        write(missing, &source);
        assert!(
            !missing.join(FILE).exists(),
            "the write really did fail; otherwise this proves nothing"
        );
        assert!(
            write_required(missing, &source).is_err(),
            "a required receipt that cannot be written must fail the install"
        );
    }
}
