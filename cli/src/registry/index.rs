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

    /// A stable fingerprint of the index's *installable content* — everything that
    /// determines what can be installed and from where (the schema `version`, the
    /// `agents`, and the `bundles`), deliberately EXCLUDING the volatile
    /// `updated-at` timestamp.
    ///
    /// This is the shared tarball cache key (#254). The original key trusted
    /// `updated-at` to rotate on every registry update, but that field is
    /// hand-maintained: it froze (2026-06-10) while `main` advanced, so an agent
    /// added later (#232) was advertised by the catalog yet absent from the cached
    /// `main` archive → `subdir not in tarball`, with no self-recovery. Keying on
    /// the content itself removes the human from the loop: add / remove / repin an
    /// agent and the fingerprint changes, busting the cache so a fresh archive is
    /// fetched. `updated-at` is excluded so a cosmetic timestamp bump alone cannot
    /// needlessly invalidate an otherwise-valid cache.
    ///
    /// Deterministic across runs and platforms: `agents`/`bundles` are `BTreeMap`s,
    /// so their JSON serialization is key-sorted and stable.
    pub fn content_fingerprint(&self) -> String {
        let mut h = Sha256::new();
        h.update(self.version.as_bytes());
        h.update([0]); // domain-separate the fields so they can't run together
        h.update(serde_json::to_vec(&self.agents).unwrap_or_default());
        h.update([0]);
        h.update(serde_json::to_vec(&self.bundles).unwrap_or_default());
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
    fn content_fingerprint_ignores_updated_at_but_tracks_installable_set() {
        // The #254 regression guard. The cache key derives from this fingerprint, so:
        //
        //  (a) a cosmetic `updated-at` change must NOT rotate it (the OLD key did, and
        //      its staleness is exactly what stranded users), and
        //  (b) ANY change to the installable set MUST rotate it (add an agent → fresh
        //      download → the new agent's subdir is present).
        let base = Index::parse(SAMPLE.as_bytes()).unwrap();
        let fp = base.content_fingerprint();

        // (a) Same installable content, different `updated-at` → SAME fingerprint.
        let bumped = Index::parse(
            SAMPLE
                .replace("2026-05-16T00:00:00Z", "2099-01-01T00:00:00Z")
                .as_bytes(),
        )
        .unwrap();
        assert_eq!(
            fp,
            bumped.content_fingerprint(),
            "a pure updated-at bump must not bust the cache (the root cause of #254)"
        );

        // (b1) An added agent (the literal #232 scenario) → DIFFERENT fingerprint.
        let added = Index::parse(KEYED.as_bytes()).unwrap();
        assert_ne!(
            fp,
            added.content_fingerprint(),
            "a changed agent set must bust the cache"
        );

        // (b2) Repinning an existing agent's tarball/subdir → DIFFERENT fingerprint.
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
            repinned.content_fingerprint(),
            "a changed tarball URL must bust the cache"
        );

        // Stable across runs for identical content.
        assert_eq!(fp, base.content_fingerprint());
    }
}
