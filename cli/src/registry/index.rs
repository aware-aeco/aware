use std::collections::BTreeMap;
use std::io::Read;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::AwareError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    pub version: String,
    #[serde(rename = "updated-at")]
    pub updated_at: String,
    pub agents: BTreeMap<String, IndexEntry>,
    #[serde(default)]
    pub bundles: BTreeMap<String, BundleEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexEntry {
    pub versions: BTreeMap<String, VersionEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionEntry {
    pub tarball: String,
    pub subdir: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BundleEntry {
    pub description: String,
    pub agents: Vec<String>,
}

impl Index {
    pub fn parse<R: Read>(mut r: R) -> Result<Self, AwareError> {
        let mut s = String::new();
        r.read_to_string(&mut s)?;
        serde_json::from_str(&s).map_err(|e| AwareError::Validation(format!("registry index: {e}")))
    }

    /// A stable fingerprint of the WHOLE index snapshot — every field that could
    /// signal "the registry changed, refetch": the installable content (`version`,
    /// `agents`, `bundles`) AND the declared `updated-at`.
    ///
    /// This is the shared tarball cache key (#254). The original key trusted
    /// `updated-at` *alone* to rotate on every registry update, but that field is
    /// hand-maintained: it froze (2026-06-10) while `main` advanced, so an agent
    /// added later (#232) was advertised by the catalog yet absent from the cached
    /// `main` archive → `subdir not in tarball`, with no self-recovery. Hashing the
    /// installable content removes the human from the loop for the common case: add /
    /// remove / repin an agent and the key rotates automatically, busting the cache.
    ///
    /// `updated-at` is kept in the hash as the registry's *manual refresh lever*: the
    /// tarball is a mutable `main` archive, so an existing agent's files can change
    /// (a skill/manifest fix under the same version + subdir) WITHOUT any index-entry
    /// change. Bumping `updated-at` then forces a refresh that the content hash alone
    /// could not. Including it can only bust the cache *more*, never serve stale —
    /// the safe direction for a bug that was about staleness.
    ///
    /// Deterministic across runs and platforms: `Index` serializes its fields in a
    /// fixed order and `agents`/`bundles` are key-sorted `BTreeMap`s.
    pub fn snapshot_fingerprint(&self) -> String {
        let mut h = Sha256::new();
        h.update(serde_json::to_vec(self).unwrap_or_default());
        format!("{:x}", h.finalize())
    }

    /// Resolve `<id>[@version]` → `(version, &VersionEntry)`. If version is `None`, return
    /// the lexicographically-greatest version (good enough for v0.2; v0.3+ adds semver).
    pub fn resolve(
        &self,
        id: &str,
        version: Option<&str>,
    ) -> Result<(&String, &VersionEntry), AwareError> {
        let entry = self
            .agents
            .get(id)
            .ok_or_else(|| AwareError::NotFound(format!("agent {id} not in registry")))?;
        let (resolved_version, version_entry) = match version {
            Some(v) => entry
                .versions
                .get_key_value(v)
                .ok_or_else(|| AwareError::NotFound(format!("agent {id}@{v} not in registry")))?,
            None => entry
                .versions
                .iter()
                .next_back()
                .ok_or_else(|| AwareError::NotFound(format!("agent {id} has no versions")))?,
        };
        Ok((resolved_version, version_entry))
    }

    /// Resolve an installed agent id to the registry key it was installed from.
    ///
    /// An installed agent's `manifest.agent` can carry a version-ish suffix the
    /// registry key does not (e.g. key `allplan-2024` installs as
    /// `allplan-2024.0`). `agent update` only knows the installed id, so it must
    /// map back to the base-name before calling [`resolve`](Self::resolve).
    ///
    /// An exact key match wins; otherwise the *longest* key `k` for which `id`
    /// is `k` followed by `.<suffix>` is returned. `None` means nothing matches,
    /// letting callers fail *before* mutating the install — a bad id must never
    /// delete an installed agent (#174).
    pub fn resolve_key<'a>(&'a self, id: &str) -> Option<&'a str> {
        if let Some((k, _)) = self.agents.get_key_value(id) {
            return Some(k.as_str());
        }
        self.agents
            .keys()
            .filter(|k| {
                id.strip_prefix(k.as_str())
                    .is_some_and(|rest| rest.starts_with('.'))
            })
            .max_by_key(|k| k.len())
            .map(String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{
        "version": "1.0",
        "updated-at": "2026-05-16T00:00:00Z",
        "agents": {
            "tekla": {
                "versions": {
                    "2025.0.1": { "tarball": "https://example/tekla.tar.gz", "subdir": "tekla" }
                }
            }
        },
        "bundles": {
            "aware-aeco": { "description": "all", "agents": ["tekla@2025.0.1"] }
        }
    }"#;

    #[test]
    fn parses_minimal_index() {
        let idx = Index::parse(SAMPLE.as_bytes()).unwrap();
        assert_eq!(idx.version, "1.0");
        assert!(idx.agents.contains_key("tekla"));
        assert!(idx.bundles.contains_key("aware-aeco"));
    }

    #[test]
    fn resolves_pinned_version() {
        let idx = Index::parse(SAMPLE.as_bytes()).unwrap();
        let (v, e) = idx.resolve("tekla", Some("2025.0.1")).unwrap();
        assert_eq!(v, "2025.0.1");
        assert_eq!(e.tarball, "https://example/tekla.tar.gz");
    }

    #[test]
    fn missing_agent_is_not_found() {
        let idx = Index::parse(SAMPLE.as_bytes()).unwrap();
        assert!(idx.resolve("nope", None).is_err());
    }

    const KEYED: &str = r#"{
        "version": "1.0",
        "updated-at": "2026-05-16T00:00:00Z",
        "agents": {
            "tekla": { "versions": { "1": { "tarball": "t", "subdir": "s" } } },
            "allplan-2024": { "versions": { "1": { "tarball": "t", "subdir": "s" } } },
            "allplan-2025": { "versions": { "1": { "tarball": "t", "subdir": "s" } } }
        },
        "bundles": {}
    }"#;

    #[test]
    fn resolve_key_exact_match_wins() {
        let idx = Index::parse(KEYED.as_bytes()).unwrap();
        assert_eq!(idx.resolve_key("tekla"), Some("tekla"));
        assert_eq!(idx.resolve_key("allplan-2024"), Some("allplan-2024"));
    }

    #[test]
    fn resolve_key_strips_version_suffix_to_base_name() {
        // #174: installed `allplan-2024.0` must map back to registry key `allplan-2024`.
        let idx = Index::parse(KEYED.as_bytes()).unwrap();
        assert_eq!(idx.resolve_key("allplan-2024.0"), Some("allplan-2024"));
        assert_eq!(idx.resolve_key("allplan-2025.0"), Some("allplan-2025"));
        assert_eq!(idx.resolve_key("tekla.0"), Some("tekla"));
    }

    #[test]
    fn resolve_key_none_when_no_match() {
        let idx = Index::parse(KEYED.as_bytes()).unwrap();
        // Unknown agent.
        assert_eq!(idx.resolve_key("revit-2026.0"), None);
        // A bare prefix that is not itself a key and not `<key>.<suffix>`.
        assert_eq!(idx.resolve_key("allplan"), None);
        // `<key>` followed by a non-dot char must not match (`allplan-2024x`).
        assert_eq!(idx.resolve_key("allplan-2024x"), None);
    }

    #[test]
    fn snapshot_fingerprint_busts_on_any_index_change() {
        // The #254 regression guard. The cache key derives from this fingerprint,
        // which must rotate on ANY signal that the registry changed — content OR the
        // hand-bumped `updated-at` (the manual refresh lever for a mutable archive
        // whose content advanced under an unchanged index entry).
        let base = Index::parse(SAMPLE.as_bytes()).unwrap();
        let fp = base.snapshot_fingerprint();

        // (a) An added agent (the literal #232 scenario) → DIFFERENT fingerprint, even
        //     with `updated-at` frozen. This is the bug: the old key, sha256(url +
        //     updated_at), could NOT rotate while the timestamp was stale.
        let added = Index::parse(KEYED.as_bytes()).unwrap();
        assert_ne!(
            fp,
            added.snapshot_fingerprint(),
            "a changed agent set must bust the cache"
        );

        // (b) Repinning an existing agent's tarball/subdir → DIFFERENT fingerprint.
        let repinned = Index::parse(
            SAMPLE
                .replace(
                    "https://example/tekla.tar.gz",
                    "https://example/tekla-v2.tar.gz",
                )
                .as_bytes(),
        )
        .unwrap();
        assert_ne!(
            fp,
            repinned.snapshot_fingerprint(),
            "a changed tarball URL must bust the cache"
        );

        // (c) A bare `updated-at` bump → DIFFERENT fingerprint: the registry's manual
        //     lever to force a refresh when `main` advanced without an index change.
        let bumped = Index::parse(
            SAMPLE
                .replace("2026-05-16T00:00:00Z", "2099-01-01T00:00:00Z")
                .as_bytes(),
        )
        .unwrap();
        assert_ne!(
            fp,
            bumped.snapshot_fingerprint(),
            "an updated-at bump must bust the cache (the manual refresh lever)"
        );

        // Stable across runs for an identical index.
        assert_eq!(fp, base.snapshot_fingerprint());
    }
}
