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

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IndexEntry {
    pub versions: BTreeMap<String, VersionEntry>,
    /// A backward-compatible **rename alias** (#256). The key carrying this is the
    /// agent's OLD id; its `versions[].subdir` points at the NEW agent (whose
    /// `manifest.agent` is the new id). The entry stays resolvable for
    /// `agent update` — so an existing install of the old id MIGRATES to the new id
    /// (the folder swap in [`update_agent_from_registry`](crate::install::update_agent_from_registry)
    /// already fires when the fetched manifest's id differs) — but it is EXCLUDED
    /// from the generated catalog, so the old id is NOT listed as a duplicate of the
    /// new one. That decouples "resolvable for update" from "listed in the catalog",
    /// the clean rename path: new users see only the new id; existing installs
    /// migrate automatically; the catalog stays deduplicated. `None` for an ordinary
    /// entry.
    #[serde(rename = "alias-of", default, skip_serializing_if = "Option::is_none")]
    pub alias_of: Option<String>,
    /// Retire this index key **without** a rename target (#256): still resolvable for
    /// `agent update` (existing installs can keep pulling it) but hidden from the
    /// catalog so it is no longer offered to new users — a soft sunset. `alias-of`
    /// already implies hidden; set `deprecated` alone when there is no successor id.
    #[serde(default, skip_serializing_if = "is_false")]
    pub deprecated: bool,
}

/// serde `skip_serializing_if` predicate: omit a `false` boolean from the JSON so an
/// ordinary index entry stays `{ "versions": … }` with no `deprecated` noise.
fn is_false(b: &bool) -> bool {
    !*b
}

impl IndexEntry {
    /// Whether this entry must be omitted from the generated catalog. A rename alias
    /// (`alias-of`) or a deprecated key stays resolvable for `agent update` but must
    /// not surface in `agent catalog`/`search` or a downstream Agent Library —
    /// otherwise a rename leaves a duplicate listing (#256).
    pub fn hidden_from_catalog(&self) -> bool {
        self.alias_of.is_some() || self.deprecated
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionEntry {
    pub tarball: String,
    pub subdir: String,
}

/// The archive's top-level folder, which every substrate-hosted `subdir` is written
/// under (`aware-main/20-agents/...`) because that is the prefix the entry must carry to
/// resolve inside `main.tar.gz`.
pub const SUBSTRATE_ARCHIVE_ROOT: &str = "aware-main/";

/// `Err(reason)` when a `subdir` is not written in the one portable form the registry
/// accepts: a RELATIVE, `/`-separated path that stays inside the archive — no backslash,
/// no absolute or drive-prefixed path, no leading `..`.
///
/// Each rejected form is one a reader resolves to a path the string does not look like,
/// so two entries can name a single manifest while comparing as distinct. Every one of
/// them is REJECTED rather than normalised, because there is no normalisation that is
/// right on every platform, and because none of these is a spelling in real use: `agent
/// publish` emits `aware-main/<repo-relative>` with `/` separators, and tar members are
/// relative and `/`-separated by the POSIX format itself. They are malformed input.
///
/// Take the backslash as the worked example, since normalising it is the tempting move: `Path::join` treats `\` as a separator on Windows, so
/// `foo/bar` and `foo\bar` load one manifest there — but on Linux `\` is an ordinary
/// filename character, so they are two different directories and folding them together
/// would make the guard refuse a registry that is actually fine. There is no
/// normalisation that is right on both; refusing is (Codex review, PR #457 round 8).
///
/// Refusing is also what the value already is: `agent publish` writes
/// `rel.replace('\\', "/")`, and tar member paths are `/`-separated by the POSIX format
/// itself, so a backslash here is malformed input rather than a spelling in use.
pub fn check_subdir_portable(subdir: &str) -> Result<(), String> {
    if subdir.contains('\\') {
        return Err(format!(
            "subdir '{subdir}' contains a backslash. Registry subdirs name a path inside a \
             tar archive and are always '/'-separated; a backslash is a directory name on \
             Linux but a separator on Windows, so the same entry would resolve to different \
             manifests per platform. Write it with '/'."
        ));
    }
    // An ABSOLUTE subdir is not a location inside the archive at all, and `Path::join`
    // DISCARDS its base when given one — so `repo_root.join("/etc/foo")` is `/etc/foo`, and
    // on Windows `repo_root.join("C:/repo/foo")` is `C:/repo/foo`. Two entries can then
    // name one manifest by different-looking strings, or reach outside the checkout
    // entirely (Codex review, PR #457 round 11).
    //
    // Checked on the RAW value, not the normalised one: `normalize_subdir` drops a leading
    // `/` as an empty segment, so `/foo` and `foo` normalise identically and the evidence
    // is gone by then.
    if subdir.starts_with('/') {
        return Err(format!(
            "subdir '{subdir}' is an absolute path. A registry subdir is relative to the \
             archive root; an absolute one replaces the base it is joined to instead of \
             naming a member of the archive. Write it relative, under \
             '{SUBSTRATE_ARCHIVE_ROOT}'."
        ));
    }
    // A drive prefix (`C:`, `C:/repo/foo`) is absolute or drive-relative on Windows and an
    // ordinary directory name on Linux — so the same entry means different things per
    // platform. Only the drive-letter SHAPE is rejected: a colon elsewhere in a name is a
    // legal Linux filename and is left alone.
    let first = subdir.split('/').next().unwrap_or("");
    if first.len() >= 2 && first.as_bytes()[1] == b':' && first.as_bytes()[0].is_ascii_alphabetic()
    {
        return Err(format!(
            "subdir '{subdir}' starts with a drive letter. That is absolute (or \
             drive-relative) on Windows and an ordinary directory name on Linux, so one \
             registry entry would resolve to different manifests per platform. Write it \
             relative, under '{SUBSTRATE_ARCHIVE_ROOT}'."
        ));
    }
    // A normalised subdir that still leads with `..` climbs OUT of its root — out of the
    // archive when the installer matches it, out of `repo_root` when `reindex` joins it.
    // The latter is the trap: `repo_root.join("../aware/foo")` resolves back INSIDE when the
    // checkout is itself named `aware`, so `aware-main/foo` and `../aware/foo` load one
    // manifest while reading as two distinct escaping paths (Codex review, PR #457 r10).
    //
    // The FIRST path component, not a string prefix: a directory literally named `..foo`
    // is legitimate and must not be caught. `normalize_subdir` already popped every
    // interior `..`, so a surviving one can only be leading.
    if normalize_subdir(subdir).split('/').next() == Some("..") {
        return Err(format!(
            "subdir '{subdir}' points above its root with '..'. A registry subdir names a \
             path INSIDE the archive (under '{SUBSTRATE_ARCHIVE_ROOT}'); one that climbs out \
             matches no archive member and, when the catalog is generated, can resolve back \
             into the checkout by a different route than it appears to. Write a path that \
             stays within the archive."
        ));
    }
    Ok(())
}

/// The checkout-relative directory a `subdir` names — [`normalize_subdir`] plus the
/// archive-root prefix a checkout does not have, since the checkout IS that root.
///
/// This is the mapping `agent reindex` applies to reach a manifest, and it must be the
/// SAME routine the collision guard keys on or the two disagree: `aware-main/foo` and
/// `foo` normalize to different strings while resolving to one
/// `repo_root/foo/manifest.yaml`, so the guard passed and both versions were stamped
/// from that single manifest — the corruption it exists to prevent (Codex review, PR
/// #457 round 7). Sharing one function is what makes that class of drift unrepresentable
/// rather than merely fixed.
///
/// Only the *checkout* reader strips this prefix. `extract_subdir`
/// ([`crate::install`]) matches the subdir inside the tarball, where `aware-main/` is a
/// real component of the path — so it is deliberately not folded into
/// [`normalize_subdir`], which both readers share.
pub fn checkout_relative_subdir(subdir: &str) -> String {
    let normalized = normalize_subdir(subdir);
    normalized
        .strip_prefix(SUBSTRATE_ARCHIVE_ROOT)
        .unwrap_or(&normalized)
        .to_string()
}

/// The key two subdirs must differ on to be *portably* different directories:
/// [`checkout_relative_subdir`] ASCII-lowercased.
///
/// Windows and macOS checkouts are case-insensitive by default, so `foo` and `Foo` are
/// one directory there and two on Linux. `registry-index.json` is a SINGLE artifact
/// served to all three, so a pair differing only in case cannot be resolved consistently
/// by the consumers of one registry — whichever platform the author happened to run
/// `reindex` on (Codex review, PR #457 round 9).
///
/// This is the deliberate false-positive direction, and the one place in this guard where
/// that is right: on Linux such a pair IS two directories, so refusing it rejects an index
/// that would work *there*. A registry that only works on the maintainer's filesystem is
/// the worse outcome, and the remedy — name the second version's folder distinctly — costs
/// nothing.
///
/// ASCII-only folding, not [`str::to_lowercase`]: Unicode folding brings in mappings
/// (dotless i, final sigma) whose behaviour differs from any given filesystem's, so it
/// would collapse names no platform actually treats as one. Registry subdirs are ASCII in
/// practice, and under-folding merely misses an exotic case rather than refusing a valid
/// registry.
pub fn portable_subdir_key(subdir: &str) -> String {
    checkout_relative_subdir(subdir).to_ascii_lowercase()
}

/// A version's `subdir` reduced to the directory it actually names, so two spellings of
/// one location compare equal.
///
/// Both consumers resolve a subdir as a path — `extract_subdir` ([`crate::install`])
/// matches it inside the archive after trimming trailing slashes, and `agent reindex`
/// joins it onto the checkout root — so `foo`, `foo/`, `foo/.` and `foo//bar` are the
/// same directory to them and must be one key here. A guard keyed on the raw string
/// misses a collision spelled any other way, which is the hole it exists to close
/// (#454; Codex review, PR #457, rounds 2 and 6, each naming a spelling the previous
/// normalisation still let through).
///
/// `..` pops, matching how both the archive matcher and the filesystem read the path.
/// That is lexical: through a symlinked directory the OS would resolve differently, but
/// these name locations inside a tar archive and a source checkout, where no such link
/// is meaningful, and erring toward treating two paths as EQUAL is the safe direction
/// for a guard whose miss corrupts the catalog.
pub fn normalize_subdir(subdir: &str) -> String {
    let mut parts: Vec<&str> = Vec::new();
    for seg in subdir.split('/') {
        match seg {
            // Empty (a repeated or trailing slash) and `.` name the current directory.
            "" | "." => {}
            ".." => {
                // A leading `..` has nothing to pop; keep it so two different paths that
                // both escape the root do not collapse onto each other.
                if matches!(parts.last(), Some(&last) if last != "..") {
                    parts.pop();
                } else {
                    parts.push(seg);
                }
            }
            other => parts.push(other),
        }
    }
    parts.join("/")
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

    /// Resolve `<id>[@version]` → `(version, &VersionEntry)`.
    ///
    /// With no version, returns the greatest by **SemVer §11 precedence** — not the
    /// lexicographically-greatest, which is what this did until #371 and which put `1.9.0`
    /// ahead of `1.10.1`. A version key that is not strict SemVer ranks below every key that
    /// is, so it can be asked for by name but never resolves as "latest".
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
            // #371: NOT `next_back()`. These keys live in a `BTreeMap<String, _>`, so that is a
            // string comparison — `"1.10.1" < "1.9.0"` — and `aware agent install <id>` fetched
            // 1.9.0 when 1.10.1 existed. This registry publishes calendar-shaped versions
            // (`tekla@2025.0.1`), where a `.10` follows a `.9` routinely.
            None => entry
                .versions
                .iter()
                .max_by(|a, b| crate::validate::compare_version_keys(a.0, b.0))
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

    #[test]
    fn check_subdir_portable_rejects_escaping_and_backslash_but_allows_dotdot_names() {
        assert!(check_subdir_portable("aware-main/20-agents/demo").is_ok());
        // A directory literally named `..foo` is fine — only a `..` COMPONENT escapes.
        assert!(check_subdir_portable("aware-main/..foo/demo").is_ok());
        // Leading parent component escapes the root (#457 round 10).
        assert!(check_subdir_portable("../aware/foo").is_err());
        assert!(check_subdir_portable("aware-main/../../etc").is_err());
        // Backslash resolves differently per platform (#457 round 8).
        assert!(check_subdir_portable("aware-main/foo\\bar").is_err());
        // Absolute and drive-prefixed paths replace the base they are joined to
        // (#457 round 11). Checked on the RAW value: `normalize_subdir` drops the
        // leading `/` as an empty segment, so `/foo` and `foo` normalise alike.
        assert!(check_subdir_portable("/etc/foo").is_err());
        assert!(check_subdir_portable("C:/repo/foo").is_err());
        assert!(check_subdir_portable("c:foo").is_err());
        assert_eq!(normalize_subdir("/foo"), normalize_subdir("foo"));
        // A colon that is NOT a drive prefix is an ordinary Linux filename.
        assert!(check_subdir_portable("aware-main/a:b/demo").is_ok());
        assert!(check_subdir_portable("aware-main/ab:/demo").is_ok());
    }

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

    const ALIASED: &str = r#"{
        "version": "1.0",
        "updated-at": "2026-05-16T00:00:00Z",
        "agents": {
            "steel-detailer-us": { "versions": { "0.1.0": { "tarball": "t", "subdir": "us" } } },
            "steel-detailer-aisc": {
                "alias-of": "steel-detailer-us",
                "versions": { "0.1.0": { "tarball": "t", "subdir": "us" } }
            },
            "old-sunset": {
                "deprecated": true,
                "versions": { "0.1.0": { "tarball": "t", "subdir": "sunset" } }
            }
        },
        "bundles": {}
    }"#;

    #[test]
    fn parses_alias_and_deprecated_flags() {
        // #256: the new optional index-entry flags deserialize, default to absent, and
        // the helper marks exactly the alias / deprecated keys as catalog-hidden.
        let idx = Index::parse(ALIASED.as_bytes()).unwrap();

        let plain = &idx.agents["steel-detailer-us"];
        assert_eq!(plain.alias_of, None);
        assert!(!plain.deprecated);
        assert!(!plain.hidden_from_catalog(), "an ordinary entry is listed");

        let alias = &idx.agents["steel-detailer-aisc"];
        assert_eq!(alias.alias_of.as_deref(), Some("steel-detailer-us"));
        assert!(alias.hidden_from_catalog(), "an alias is catalog-hidden");

        let dep = &idx.agents["old-sunset"];
        assert!(dep.deprecated);
        assert!(
            dep.hidden_from_catalog(),
            "a deprecated key is catalog-hidden"
        );
    }

    #[test]
    fn alias_key_stays_resolvable_for_update() {
        // The whole point of the alias (#256): the OLD id must still resolve so
        // `agent update <old>` can fetch the new payload and migrate the install.
        let idx = Index::parse(ALIASED.as_bytes()).unwrap();
        assert_eq!(
            idx.resolve_key("steel-detailer-aisc"),
            Some("steel-detailer-aisc"),
            "the alias key resolves for update"
        );
        let (ver, entry) = idx.resolve("steel-detailer-aisc", None).unwrap();
        assert_eq!(ver, "0.1.0");
        assert_eq!(
            entry.subdir, "us",
            "the alias points at the NEW agent's payload, so update migrates to it"
        );
    }

    #[test]
    fn ordinary_entry_serializes_without_flag_noise() {
        // skip_serializing_if keeps an ordinary entry as `{"versions":…}` — no
        // `alias-of` / `deprecated` keys leak into a freshly published index.
        let entry = IndexEntry {
            versions: BTreeMap::new(),
            ..Default::default()
        };
        let json = serde_json::to_string(&entry).unwrap();
        assert!(!json.contains("alias-of"), "no alias-of key: {json}");
        assert!(!json.contains("deprecated"), "no deprecated key: {json}");
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
