//! `aware agent install <id>@<version> --store-only` (#645): put one exact
//! registry release into the agent store WITHOUT making it the installed copy.
//!
//! An approved app whose pinned version is no longer stored (`aware app check`
//! → `pin-not-installed`, e.g. after `aware agent gc` removed it) can get those
//! bytes back this way while `agents/<id>/` — and so what the next `aware app
//! compile` resolves — stays exactly as it is.
//!
//! The release is fetched and verified by the same code as `agent install`
//! ([`ReleaseCheck`]: release binding, full on-disk validation, the official
//! registry's `bundle-digest`, the install receipt), staged in a private temp
//! directory outside `agents/`, and published by the store's only writer,
//! [`crate::agent_store::snapshot`]. No swap lock, no swap transaction, nothing
//! under `agents/` is read or written. The caller holds the store reference
//! lock shared for the whole operation, so garbage collection (which takes it
//! exclusive) cannot interleave with the fetch, the publish or the stamp.

use crate::agent_store::{RefGuard, StoredPackage};
use crate::error::AwareError;
use crate::install::registry::{ReleaseCheck, stage_agent_from_registry};
use crate::paths::Paths;
use crate::registry::Index;

/// What a store-only fetch did.
#[derive(Debug)]
pub struct StoreOnly {
    /// The verified store package now holding the release.
    pub package: StoredPackage,
    /// The registry key and version the release was fetched as.
    pub registry_key: String,
    pub registry_version: String,
    /// Whether the receipt records a fresh official-registry install.
    pub official_source: bool,
    /// The package was already in the store before this fetch.
    pub already_stored: bool,
    /// The last-needed stamp of a package published by THIS fetch could not
    /// be written: its window still counts from its fresh `snapshotted-at`.
    /// (For an already-stored package a failed stamp is an error.)
    pub stamp_warning: Option<String>,
}

/// Fetch, verify and store `id@version` from the registry. `agents/` is never
/// touched. Requires the store reference lock held shared (`guard`).
pub fn store_agent_from_registry(
    id: &str,
    version: &str,
    paths: &Paths,
    index: &Index,
    guard: &RefGuard,
) -> Result<StoreOnly, AwareError> {
    crate::agent_store::guard::require_home(guard, paths)?;
    let index_entry = index
        .agents
        .get(id)
        .ok_or_else(|| AwareError::NotFound(format!("agent {id} not in registry")))?;
    let (resolved, release) = index.resolve(id, Some(version))?;
    crate::registry::index::validate_release_contract(id, resolved, index_entry, release)
        .map_err(AwareError::Validation)?;
    let (scratch, subdir) = stage_agent_from_registry(id, Some(version), paths, index)?;
    let check = ReleaseCheck {
        trust: index.trust,
        key: id,
        registry_version: resolved,
        index_entry,
        release,
    };
    let agent = check.validated_manifest(&subdir)?;
    // A fresh directory inside this fetch's own scratch: never `agents/`, never
    // the swap area, never shared with another process.
    let staging = scratch.path().join("staged");
    let digest = check.stage_receipted(&subdir, &agent, &staging)?;
    let key = crate::agent_store::receipt_key(&staging)?;
    let already_stored = crate::agent_store::probe(
        &crate::agent_store::digest_container(paths, &agent.agent, &digest)?.join(&key),
    )?
    .is_some();
    let package = crate::agent_store::snapshot(paths, &staging, guard)?;
    if package.digest != digest || package.receipt_key != key {
        return Err(AwareError::Internal(format!(
            "store-only fetch of {id}@{resolved}: staged {digest} / {key} but stored {} / {}",
            package.digest, package.receipt_key
        )));
    }
    // A fresh recovery window from now: an already-stored package keeps its
    // old `snapshotted-at`, so without this GC could remove it again at once
    // when no lock it can find pins it. A package published just now has a
    // fresh `snapshotted-at` already, so for it a failed stamp is a warning;
    // for a reused one it is the whole point of the command, so it fails
    // (the package is stored and verified either way; running it again is safe).
    let stamp_warning = match crate::agent_store::stamps::stamp(
        paths,
        &package.agent,
        &package.digest,
        None,
    ) {
        Ok(()) => None,
        Err(error) if already_stored => {
            return Err(AwareError::Validation(format!(
                "[E_AGENT_STORE_STAMP] {} {} ({}) is stored and verified, but its recovery window could not be restarted ({error}); \
                     garbage collection may remove it again unless a lock it can find pins it - fix that and run the command again",
                package.agent, package.version, package.digest
            )));
        }
        Err(error) => Some(error.to_string()),
    };
    Ok(StoreOnly {
        package,
        registry_key: id.to_string(),
        registry_version: resolved.clone(),
        official_source: index.trust == crate::registry::RegistryTrust::FreshOfficial,
        already_stored,
        stamp_warning,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::{IndexEntry, RegistryTrust, VersionEntry};
    use std::collections::BTreeMap;
    use std::io::Write;
    use std::path::{Path, PathBuf};

    const SUBDIR: &str = "aware-main/20-agents/alpha";

    fn manifest(version: &str) -> String {
        format!(
            "agent: alpha\nversion: {version}\ndescription: x\nstateful: false\nlicense: MIT\n\
             transport:\n  cli:\n    binary: aware-alpha\ncommands:\n  go:\n    lifecycle: single\n    description: x\n"
        )
    }

    fn archive(path: &Path, manifest: &str) {
        let enc = flate2::write::GzEncoder::new(
            std::fs::File::create(path).unwrap(),
            flate2::Compression::default(),
        );
        let mut tar = tar::Builder::new(enc);
        let mut header = tar::Header::new_gnu();
        header.set_size(manifest.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        tar.append_data(
            &mut header,
            format!("{SUBDIR}/manifest.yaml"),
            manifest.as_bytes(),
        )
        .unwrap();
        let mut file = tar.into_inner().unwrap().finish().unwrap();
        file.flush().unwrap();
    }

    struct Fx {
        _tmp: tempfile::TempDir,
        root: PathBuf,
        paths: Paths,
    }

    fn fx() -> Fx {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path().to_path_buf();
        for version in ["1.0.0", "1.1.0"] {
            archive(
                &root.join(format!("alpha-{version}.tar.gz")),
                &manifest(version),
            );
        }
        Fx {
            paths: Paths {
                aware_home: root.join("aware"),
            },
            root,
            _tmp: tmp,
        }
    }

    impl Fx {
        /// An index publishing alpha 1.0.0 and 1.1.0; `declared` overrides the
        /// `manifest-version` binding of 1.0.0, `digest` its `bundle-digest`.
        fn index(&self, declared: &str, digest: Option<String>, trust: RegistryTrust) -> Index {
            let mut versions = BTreeMap::new();
            for (version, bound, digest) in [("1.0.0", declared, digest), ("1.1.0", "1.1.0", None)]
            {
                versions.insert(
                    version.to_string(),
                    VersionEntry {
                        bundle_digest: digest,
                        tarball: format!(
                            "file://{}",
                            self.root.join(format!("alpha-{version}.tar.gz")).display()
                        ),
                        subdir: SUBDIR.into(),
                        manifest_agent: Some("alpha".into()),
                        manifest_version: Some(bound.into()),
                    },
                );
            }
            let mut agents = BTreeMap::new();
            agents.insert(
                "alpha".to_string(),
                IndexEntry {
                    versions,
                    ..Default::default()
                },
            );
            Index {
                trust,
                version: "1.0".into(),
                updated_at: "2026-10-06T00:00:00Z".into(),
                agents,
                bundles: BTreeMap::new(),
            }
        }

        fn plain_index(&self) -> Index {
            self.index("1.0.0", None, RegistryTrust::Unverified)
        }

        fn install(&self, version: &str) {
            crate::install::install_agent_from_registry(
                "alpha",
                Some(version),
                &self.paths,
                &self.plain_index(),
                &crate::agent_store::open(&self.paths).unwrap(),
            )
            .unwrap();
        }

        fn store_only(&self, version: &str, index: &Index) -> Result<StoreOnly, AwareError> {
            store_agent_from_registry(
                "alpha",
                version,
                &self.paths,
                index,
                &crate::agent_store::open(&self.paths).unwrap(),
            )
        }

        fn agents_bytes(&self) -> BTreeMap<String, Vec<u8>> {
            let dir = self.paths.agents_dir();
            if !dir.exists() {
                return BTreeMap::new();
            }
            crate::fs::plain_files_under(&dir, "test tree")
                .unwrap()
                .into_iter()
                .map(|(relative, path)| (relative, std::fs::read(path).unwrap()))
                .collect()
        }

        /// Every verified store package of alpha, as (version, digest).
        fn stored(&self) -> Vec<(String, String)> {
            crate::agent_store::stored_versions(&self.paths, "alpha")
                .stored
                .into_iter()
                .map(|v| (v.version, v.digest))
                .collect()
        }

        fn remove_package(&self, package: &StoredPackage) {
            std::fs::remove_dir_all(&package.root).unwrap();
        }
    }

    /// The package a normal install of `version` leaves in the store.
    fn package_of(fx: &Fx, version: &str) -> StoredPackage {
        let root = fx.paths.agent_store_dir().join("alpha");
        for digest in std::fs::read_dir(&root).unwrap().flatten() {
            for package in std::fs::read_dir(digest.path()).unwrap().flatten() {
                let name = package.file_name().to_string_lossy().into_owned();
                if !crate::agent_store::is_receipt_key(&name) {
                    continue;
                }
                let digest = format!("sha256:{}", digest.file_name().to_string_lossy());
                let verified =
                    crate::agent_store::verify_package(&package.path(), "alpha", &digest, &name)
                        .unwrap();
                if verified.version == version {
                    return verified;
                }
            }
        }
        panic!("no stored alpha {version}");
    }

    #[test]
    fn stores_the_exact_release_and_leaves_the_installed_copy_alone() {
        let fx = fx();
        fx.install("1.0.0");
        let original = package_of(&fx, "1.0.0");
        crate::install::update_agent_from_registry(
            "alpha",
            Some("1.1.0"),
            false,
            &fx.paths,
            &fx.plain_index(),
            &crate::agent_store::open(&fx.paths).unwrap(),
        )
        .unwrap();
        fx.remove_package(&original);
        let before = fx.agents_bytes();

        let stored = fx.store_only("1.0.0", &fx.plain_index()).unwrap();

        assert_eq!(
            fx.agents_bytes(),
            before,
            "agents/ must be byte-identical: the installed copy is still 1.1.0"
        );
        assert!(crate::install::swap::leftover_txn_dirs(&fx.paths).is_empty());
        assert!(!stored.already_stored);
        assert_eq!(stored.package.version, "1.0.0");
        // The same bytes AND the same receipt as the normal install wrote, so
        // the very package the approval was made from is back.
        assert_eq!(stored.package.root, original.root);
        assert_eq!(stored.package.digest, original.digest);
        assert_eq!(stored.package.receipt_key, original.receipt_key);
        assert!(
            crate::agent_store::verify_package(
                &stored.package.root,
                "alpha",
                &stored.package.digest,
                &stored.package.receipt_key
            )
            .is_ok()
        );
        assert_eq!(
            crate::agent_store::receipt_rank(&stored.package.root),
            1,
            "a registry receipt (not official: file registry)"
        );
        assert_eq!(stored.registry_version, "1.0.0");
        assert!(!stored.official_source);
        assert!(
            crate::agent_store::stamps::last_needed(&fx.paths, "alpha", &stored.package.digest)
                .is_some(),
            "a fresh recovery window starts at the fetch"
        );
    }

    #[test]
    fn works_when_the_agent_is_not_installed_and_installs_nothing() {
        let fx = fx();
        let stored = fx.store_only("1.1.0", &fx.plain_index()).unwrap();
        assert_eq!(stored.package.version, "1.1.0");
        assert!(!fx.paths.agents_dir().join("alpha").exists());
        assert!(fx.agents_bytes().is_empty());
        assert_eq!(fx.stored().len(), 1);
    }

    #[test]
    fn an_official_digest_mismatch_stores_nothing() {
        let fx = fx();
        let index = fx.index(
            "1.0.0",
            Some(format!("sha256:{}", "a".repeat(64))),
            RegistryTrust::FreshOfficial,
        );
        let error = fx.store_only("1.0.0", &index).unwrap_err().to_string();
        assert!(error.contains("bundle digest mismatch"), "{error}");
        assert!(fx.stored().is_empty());
        assert!(!fx.paths.agents_dir().join("alpha").exists());
    }

    #[test]
    fn an_official_release_with_its_real_digest_is_stored_as_official() {
        let fx = fx();
        // The digest the payload really has, learned from a plain fetch.
        let probe = fx.store_only("1.0.0", &fx.plain_index()).unwrap();
        fx.remove_package(&probe.package);
        let index = fx.index(
            "1.0.0",
            Some(probe.package.digest.clone()),
            RegistryTrust::FreshOfficial,
        );
        // A normal install from the fresh official index: the package it leaves
        // is what an approval made on this machine pins.
        crate::install::install_agent_from_registry(
            "alpha",
            Some("1.0.0"),
            &fx.paths,
            &index,
            &crate::agent_store::open(&fx.paths).unwrap(),
        )
        .unwrap();
        let installed = package_of(&fx, "1.0.0");
        assert!(crate::install::provenance::claims_official(&installed.root));
        fx.remove_package(&installed);

        let stored = fx.store_only("1.0.0", &index).unwrap();
        assert!(stored.official_source);
        assert_eq!(stored.package.digest, probe.package.digest);
        assert_eq!(
            stored.package.receipt_key, installed.receipt_key,
            "byte-for-byte the receipt an official install writes"
        );
        assert_eq!(stored.package.root, installed.root);
        assert_eq!(crate::agent_store::receipt_rank(&stored.package.root), 0);
        // And it verifies against the fresh index exactly as the install did.
        let assessed = crate::install::provenance::assess_against_index(
            &stored.package.root,
            "alpha",
            "1.0.0",
            Some(&index),
        );
        assert!(assessed.verified, "{assessed:?}");
    }

    // ── the shared release check, driven directly (#645 plan review) ────────
    // `stage_agent_from_registry` checks the binding and the official digest
    // too, so the store-only tests above would pass with the shared helper's
    // own checks deleted. These pin the helper itself.

    fn staged_payload(fx: &Fx, version: &str) -> PathBuf {
        let src = fx.root.join(format!("payload-{version}"));
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("manifest.yaml"), manifest(version)).unwrap();
        src
    }

    #[test]
    fn the_release_check_refuses_a_broken_binding() {
        let fx = fx();
        let index = fx.index("9.9.9", None, RegistryTrust::Unverified);
        let entry = &index.agents["alpha"];
        let check = ReleaseCheck {
            trust: index.trust,
            key: "alpha",
            registry_version: "1.0.0",
            index_entry: entry,
            release: &entry.versions["1.0.0"],
        };
        let error = check
            .validated_manifest(&staged_payload(&fx, "1.0.0"))
            .unwrap_err()
            .to_string();
        assert!(error.contains("payload declares"), "{error}");
    }

    #[test]
    fn the_release_check_requires_the_official_digest() {
        let fx = fx();
        let src = staged_payload(&fx, "1.0.0");
        for digest in [None, Some(format!("sha256:{}", "a".repeat(64)))] {
            let index = fx.index("1.0.0", digest.clone(), RegistryTrust::FreshOfficial);
            let entry = &index.agents["alpha"];
            let check = ReleaseCheck {
                trust: index.trust,
                key: "alpha",
                registry_version: "1.0.0",
                index_entry: entry,
                release: &entry.versions["1.0.0"],
            };
            let agent = check.validated_manifest(&src).unwrap();
            let staging = fx.root.join(format!("staging-{}", digest.is_some()));
            let error = check
                .stage_receipted(&src, &agent, &staging)
                .unwrap_err()
                .to_string();
            let expected = if digest.is_some() {
                "bundle digest mismatch"
            } else {
                "has no bundle-digest"
            };
            assert!(error.contains(expected), "{digest:?}: {error}");
            assert!(
                !staging.join(crate::install::provenance::FILE).exists(),
                "no receipt for an unverified official payload"
            );
        }
    }

    #[test]
    fn a_suffixed_manifest_id_is_stored_under_the_id_it_declares() {
        let fx = fx();
        let suffixed = manifest("1.0.0").replace("agent: alpha\n", "agent: alpha.2024\n");
        let tarball = fx.root.join("alpha-2024.tar.gz");
        archive(&tarball, &suffixed);
        let mut index = fx.plain_index();
        let release = index
            .agents
            .get_mut("alpha")
            .unwrap()
            .versions
            .get_mut("1.0.0")
            .unwrap();
        release.tarball = format!("file://{}", tarball.display());
        release.manifest_agent = Some("alpha.2024".into());

        // The registry KEY is what the person names, as for `agent install`.
        let stored = fx.store_only("1.0.0", &index).unwrap();
        assert_eq!(stored.package.agent, "alpha.2024");
        assert_eq!(stored.registry_key, "alpha");
        assert!(
            crate::agent_store::stored_versions(&fx.paths, "alpha.2024")
                .stored
                .iter()
                .any(|v| v.version == "1.0.0" && v.digest == stored.package.digest)
        );
        assert!(fx.agents_bytes().is_empty());
    }

    /// Make every last-needed stamp of alpha fail: a FILE where its refs
    /// directory must be created.
    fn break_stamps(fx: &Fx) {
        let refs = fx.paths.agent_store_control_dir().join("refs");
        std::fs::create_dir_all(&refs).unwrap();
        let _ = std::fs::remove_dir_all(refs.join("alpha"));
        std::fs::write(refs.join("alpha"), "not a directory").unwrap();
    }

    #[test]
    fn a_failed_stamp_warns_for_a_new_package_and_refuses_for_a_reused_one() {
        let fx = fx();
        break_stamps(&fx);
        // Freshly published: its own `snapshotted-at` starts the window.
        let first = fx.store_only("1.0.0", &fx.plain_index()).unwrap();
        assert!(first.stamp_warning.is_some());
        assert!(!first.already_stored);

        // Reused: restarting the window is the point, so it must not pass silently.
        let error = fx
            .store_only("1.0.0", &fx.plain_index())
            .unwrap_err()
            .to_string();
        assert!(error.contains("E_AGENT_STORE_STAMP"), "{error}");
        assert!(error.contains("is stored and verified"), "{error}");
        assert_eq!(fx.stored().len(), 1, "the package itself is untouched");
    }

    #[test]
    fn nothing_under_agents_is_needed_not_even_the_swap_area() {
        let fx = fx();
        // `agents` is a FILE: any swap staging, swap lock or working-copy write
        // would fail.
        std::fs::create_dir_all(&fx.paths.aware_home).unwrap();
        std::fs::write(fx.paths.agents_dir(), "not a directory").unwrap();
        let stored = fx.store_only("1.0.0", &fx.plain_index()).unwrap();
        assert_eq!(stored.package.version, "1.0.0");
        assert_eq!(
            std::fs::read_to_string(fx.paths.agents_dir()).unwrap(),
            "not a directory"
        );
    }

    #[test]
    fn gc_cannot_take_the_store_while_a_store_only_fetch_publishes() {
        let fx = fx();
        let paths = fx.paths.clone();
        let mut tried = None;
        let probe = std::sync::Arc::new(std::sync::Mutex::new(None));
        let seen = probe.clone();
        crate::agent_store::set_before_copy(Box::new(move || {
            if tried.is_none() {
                let paths = paths.clone();
                // GC's own door, from another thread, while the publish runs.
                let got = std::thread::spawn(move || {
                    crate::agent_store::RefGuard::exclusive_within(
                        &paths,
                        std::time::Duration::from_millis(50),
                    )
                    .unwrap()
                    .is_some()
                })
                .join()
                .unwrap();
                tried = Some(got);
                *seen.lock().unwrap() = Some(got);
            }
        }));
        let result = fx.store_only("1.0.0", &fx.plain_index());
        crate::agent_store::clear_fault();
        result.unwrap();
        assert_eq!(
            *probe.lock().unwrap(),
            Some(false),
            "GC must not get the store lock exclusive mid-publish"
        );
    }

    #[test]
    fn concurrent_fetches_of_one_release_converge_on_one_package() {
        let fx = fx();
        let index = fx.plain_index();
        let results: Vec<StoredPackage> = std::thread::scope(|scope| {
            let handles: Vec<_> = (0..4)
                .map(|_| scope.spawn(|| fx.store_only("1.0.0", &index).unwrap().package))
                .collect();
            handles.into_iter().map(|h| h.join().unwrap()).collect()
        });
        assert!(results.windows(2).all(|w| w[0] == w[1]), "{results:?}");
        assert_eq!(fx.stored().len(), 1);
        let container = results[0].root.parent().unwrap();
        let leftovers: Vec<_> = std::fs::read_dir(container)
            .unwrap()
            .flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| n.starts_with(".tmp-"))
            .collect();
        assert!(leftovers.is_empty(), "{leftovers:?}");
    }

    #[test]
    fn a_payload_that_breaks_the_release_binding_stores_nothing() {
        let fx = fx();
        let index = fx.index("9.9.9", None, RegistryTrust::Unverified);
        let error = fx.store_only("1.0.0", &index).unwrap_err().to_string();
        assert!(error.contains("payload declares"), "{error}");
        assert!(fx.stored().is_empty());
    }

    #[test]
    fn a_version_the_registry_does_not_have_is_not_found() {
        let fx = fx();
        let error = fx.store_only("7.7.7", &fx.plain_index()).unwrap_err();
        assert!(matches!(error, AwareError::NotFound(_)), "{error}");
        assert!(fx.stored().is_empty());
    }

    #[test]
    fn an_already_stored_package_is_reused_and_its_window_restarts() {
        let fx = fx();
        let first = fx.store_only("1.0.0", &fx.plain_index()).unwrap();
        let old = chrono::Utc::now() - chrono::Duration::days(400);
        // Pretend it was last needed long ago: the stamp only moves forward,
        // so write the old time straight into a fresh stamp file.
        let stamp = fx
            .paths
            .agent_store_control_dir()
            .join("refs")
            .join("alpha")
            .join(format!(
                "{}.last-needed",
                crate::agent_store::digest_hex(&first.package.digest).unwrap()
            ));
        std::fs::write(&stamp, old.to_rfc3339()).unwrap();

        let second = fx.store_only("1.0.0", &fx.plain_index()).unwrap();

        assert!(second.already_stored);
        assert_eq!(second.package, first.package);
        let now =
            crate::agent_store::stamps::last_needed(&fx.paths, "alpha", &first.package.digest)
                .unwrap();
        let now = chrono::DateTime::parse_from_rfc3339(&now).unwrap();
        assert!(
            now > old + chrono::Duration::days(399),
            "the stamp moved forward to the fetch"
        );
    }

    #[test]
    fn a_snapshot_failure_at_any_step_stores_nothing() {
        for step in [
            crate::agent_store::FaultStep::Copy,
            crate::agent_store::FaultStep::Recheck,
            crate::agent_store::FaultStep::Rename,
        ] {
            let fx = fx();
            crate::agent_store::inject_fault(0, step);
            let result = fx.store_only("1.0.0", &fx.plain_index());
            crate::agent_store::clear_fault();
            assert!(result.is_err(), "{step:?}");
            assert!(fx.stored().is_empty(), "{step:?}");
            assert!(fx.agents_bytes().is_empty(), "{step:?}");
        }
    }
}
