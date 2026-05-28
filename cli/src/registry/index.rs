use std::collections::BTreeMap;
use std::io::Read;

use serde::{Deserialize, Serialize};

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
}
