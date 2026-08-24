//! The AWARE **agent catalog** — a generated, searchable sidecar to the thin
//! registry index. Where `registry-index.json` carries only `{tarball, subdir}`
//! per agent+version (enough to install), the catalog carries the *searchable*
//! metadata (description, commands, skills) so a user can browse, search, and
//! capability-check *available* agents before installing.
//!
//! The catalog is DERIVED from the index ([`build_catalog`]): it reuses the
//! index's agent+version keys verbatim (so a catalog hit maps 1:1 to an
//! installable `<id>[@version]`) and enriches each from the agent's on-disk
//! `manifest.yaml`. The pure cores ([`build_catalog`], [`score_agent`],
//! [`CatalogAgent::capability_hits`]) are unit-tested without disk or network.

use std::collections::BTreeMap;
use std::io::Read;

use serde::{Deserialize, Serialize};

use crate::error::AwareError;
use crate::manifest::agent::{Agent, AgentStatus, Category, Command, Transport};
use crate::registry::Index;
use crate::runtime::invoker::dispatch_transport;

#[derive(Debug, Serialize, Deserialize)]
pub struct Catalog {
    pub version: String,
    #[serde(rename = "updated-at")]
    pub updated_at: String,
    pub agents: BTreeMap<String, CatalogAgent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogAgent {
    #[serde(
        rename = "display-name",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vendor: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
    pub versions: BTreeMap<String, CatalogVersion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogVersion {
    pub description: String,
    pub status: String, // "available" | "planned"
    /// The agent's own `manifest.version` for this entry. DISTINCT from the entry's
    /// map KEY, which is the registry *index* version (the install spec). A consumer
    /// can compare this against an installed agent's `manifest.version` (what
    /// `agent list` reports) to tell whether an in-place `agent update` would pull a
    /// newer build — the index key lives in a different axis and is not comparable.
    /// `default` so an older sidecar (pre-this-field) still deserializes.
    #[serde(rename = "manifest-version", default)]
    pub manifest_version: String,
    pub stateful: bool,
    #[serde(
        rename = "sdk-target",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sdk_target: Option<String>,
    pub transport: String, // "cli" | "app" | "rest" | "mcp" | "none"
    /// TOTAL command count (curated + reflected). `commands` below lists only the
    /// curated verbs — reflected (auto-generated leaf API) commands are summarized by
    /// this count, not enumerated, to keep the catalog small (some agents reflect
    /// 10k+ API methods). The full reflected surface is available via `describe`/grep
    /// after install.
    #[serde(default)]
    pub command_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<String>,
    /// CURATED commands only (the hand-authored workflow verbs).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub commands: Vec<CatalogCommand>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogCommand {
    pub name: String,
    pub description: String,
    pub lifecycle: String, // "start" | "stop" | "single"
    pub category: String,  // "curated" | "reflected"
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mode: Option<String>, // "read" | "write"
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub path: Option<String>,
}

/// One capability/search match within an agent (a command, a method, or a skill).
#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct Hit {
    pub kind: String, // "command" | "method" | "skill"
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
}

/// A ranked search result for one agent.
#[derive(Debug, Serialize)]
pub struct SearchMatch {
    pub id: String,
    pub score: i32,
    /// Field labels that contributed to the score (for "why it matched" display).
    pub matched: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
}

impl Catalog {
    pub fn parse<R: Read>(mut r: R) -> Result<Self, AwareError> {
        let mut s = String::new();
        r.read_to_string(&mut s)?;
        serde_json::from_str(&s)
            .map_err(|e| AwareError::Validation(format!("registry catalog: {e}")))
    }
}

impl CatalogAgent {
    /// The greatest version by SemVer precedence + its metadata — the same answer the index's
    /// `resolve(None)` gives, through the same comparator.
    ///
    /// Both used to take `next_back()` on a `BTreeMap<String, _>`, which is a STRING comparison:
    /// `1.9.0` outranked `1.10.1`, so this reported the older version as current while the
    /// `versions:` list beside it was correctly ordered — the two contradicting each other on
    /// screen (#371).
    pub fn latest(&self) -> Option<(&String, &CatalogVersion)> {
        self.versions
            .iter()
            .max_by(|a, b| crate::validate::compare_version_keys(a.0, b.0))
    }

    /// Capability hits in the agent's latest version: a command whose name or
    /// `method` contains `cap`, or a skill whose name contains `cap`
    /// (case-insensitive substring). Empty when nothing matches.
    pub fn capability_hits(&self, cap: &str) -> Vec<Hit> {
        let q = cap.trim().to_lowercase();
        let mut hits = Vec::new();
        if q.is_empty() {
            return hits;
        }
        let Some((_, v)) = self.latest() else {
            return hits;
        };
        for c in &v.commands {
            let name_m = c.name.to_lowercase().contains(&q);
            let method_m = c
                .method
                .as_deref()
                .is_some_and(|m| m.to_lowercase().contains(&q));
            if name_m || method_m {
                hits.push(Hit {
                    kind: if method_m && !name_m {
                        "method"
                    } else {
                        "command"
                    }
                    .to_string(),
                    name: c.name.clone(),
                    description: c.description.clone(),
                });
            }
        }
        for s in &v.skills {
            if s.to_lowercase().contains(&q) {
                hits.push(Hit {
                    kind: "skill".to_string(),
                    name: s.clone(),
                    description: String::new(),
                });
            }
        }
        hits
    }
}

fn first_line(s: &str) -> String {
    s.lines().next().unwrap_or("").trim().to_string()
}

fn category_str(c: Category) -> &'static str {
    match c {
        Category::Curated => "curated",
        Category::Reflected => "reflected",
    }
}

fn status_str(s: AgentStatus) -> &'static str {
    match s {
        AgentStatus::Available => "available",
        AgentStatus::Planned => "planned",
    }
}

/// The transport `aware agent catalog` publishes for an agent: **the one it would
/// dispatch on** — and, only when nothing would dispatch, what it declares instead.
///
/// This ordered `app` above `rest` until #366, so a manifest carrying both was
/// published as `app` while dispatch took `rest`. Nothing flagged it, because a
/// hand-written priority list cannot disagree with itself. It now asks
/// [`dispatch_transport`] — the owner of the order — and so cannot drift again.
///
/// The `mcp` arm is the deliberate exception to "what dispatches", and it is not a
/// rank: `mcp` is reported only when it is all there is, and it cannot run. `none`
/// would be the worse answer — it reads as "declares no transport", which is a
/// different and false statement about the manifest. Reachable for any registry
/// payload: the catalogue is generated by `aware agent reindex` off the index and
/// never calls `validate_agent`, so `E_AGENT_MCP_ONLY` gates INSTALLING such an
/// agent, not listing one.
fn transport_str(t: &Transport) -> &'static str {
    match dispatch_transport(t) {
        Some(kind) => kind.as_str(),
        None if t.mcp.is_some() => "mcp",
        None => "none",
    }
}

/// Project a loaded agent manifest into a catalog version entry. Only CURATED
/// commands are enumerated (the meaningful verbs); reflected commands are counted
/// via `command_count` but not listed — they can number in the tens of thousands.
fn version_from_agent(a: &Agent) -> CatalogVersion {
    let commands = a
        .commands
        .iter()
        .filter(|(_, c): &(&String, &Command)| a.category_of(c) == Category::Curated)
        .map(|(name, c): (&String, &Command)| CatalogCommand {
            name: name.clone(),
            description: first_line(&c.description),
            lifecycle: c.lifecycle.as_str().to_string(),
            category: category_str(a.category_of(c)).to_string(),
            mode: c.mode.map(|m| m.as_str().to_string()),
            method: c.method.clone(),
            path: c.path.clone(),
        })
        .collect();
    CatalogVersion {
        description: first_line(&a.description),
        status: status_str(a.status).to_string(),
        manifest_version: a.version.clone(),
        stateful: a.stateful,
        sdk_target: a.sdk_target.clone(),
        transport: transport_str(&a.transport).to_string(),
        command_count: a.command_count(),
        // Skills are referenced by name; strip a `.md` suffix for display consistency.
        skills: a
            .skills
            .iter()
            .map(|s| s.trim_end_matches(".md").to_string())
            .collect(),
        commands,
    }
}

/// Build the catalog from the registry index × each agent's on-disk manifest.
///
/// `load(subdir)` resolves a manifest from an index entry's `subdir`. Keys
/// (agent id + version) come from the INDEX, never from `manifest.version` —
/// so the catalog can't drift from what's installable. A `load` failure for one
/// agent+version is recorded in the returned error list and that entry skipped
/// (one bad manifest must not silently vanish, but must not abort the catalog).
pub fn build_catalog<F>(
    index: &Index,
    updated_at: String,
    mut load: F,
) -> (Catalog, Vec<(String, String)>)
where
    F: FnMut(&str) -> Result<Agent, AwareError>,
{
    let mut agents: BTreeMap<String, CatalogAgent> = BTreeMap::new();
    let mut errors: Vec<(String, String)> = Vec::new();
    for (id, entry) in &index.agents {
        // Two version keys of ONE agent that resolve to the same SUBDIR cannot each be
        // built into a faithful catalog entry (#454), so refuse rather than emit the
        // fiction.
        //
        // The key is the subdir alone, and deliberately so: it is exactly what decides
        // which manifest this build reads. `load` is handed a subdir and nothing else —
        // `agent reindex` resolves it against the local checkout
        // (`repo_root/<subdir>/manifest.yaml`) and never opens `tarball` at all. So two
        // versions sharing a subdir get the SAME manifest whatever their tarballs say,
        // and the older key's entry is stamped with the current build's description,
        // commands and `manifest-version` — a historical version described by files that
        // path no longer holds.
        //
        // Keying this on the installer's payload identity (a `(tarball, subdir)` pair)
        // was wrong in a way worth recording, because it is the natural mistake: it
        // describes when two keys INSTALL alike, which is a different question from when
        // this generator can TELL THEM APART. Under that key, versions in distinct
        // immutable tarballs sharing one subdir passed the guard and were then both
        // described from the one checkout manifest — reindex reporting success while
        // reproducing the exact defect #454 is about. (Codex review, PR #457, rounds 1
        // and 5: round 1 caught the pair being ignored, round 5 caught that honouring it
        // here reopened the hole.)
        //
        // Refusing that shape is the honest answer while this reads one checkout: a
        // remote archive's bytes are not present to describe. Teaching `reindex` to fetch
        // each version's tarball would lift the restriction and is a real option — it is
        // also a new mechanism (network access inside a CI gate, caching, offline
        // behaviour), so it belongs to a maintainer, not to this fix.
        //
        // `agent publish` enforces the SAME rule before it writes, so the registry's
        // producer cannot emit an index its generator refuses.
        //
        // The check is WITHIN one agent id. Two DIFFERENT ids sharing a subdir is the
        // legitimate rename-alias shape (#256: `steel-detailer-aisc` → the `us` subdir),
        // which this never sees, since it only compares an entry's own versions.
        // `manifest-version` differing from the index key is intended and untouched
        // (calendar keys like `tekla@2025.0.1` over a `0.30.0` manifest) — only a shared
        // subdir is the defect, never a differing version.
        let mut seen_subdirs: BTreeMap<String, (&String, &str, String)> = BTreeMap::new();
        for (ver, ve) in &entry.versions {
            // A subdir that is not portably written resolves to different manifests on
            // different platforms, so no key computed from it means anything. Report it
            // and move on rather than guessing which reading was intended.
            if let Err(reason) = crate::registry::check_subdir_portable(&ve.subdir) {
                errors.push((format!("{id}@{ver}"), reason));
                continue;
            }
            // Keyed through the SAME routine `reindex`'s loader resolves with, so the
            // guard and the load cannot disagree about which entries reach one manifest —
            // then case-folded, because the index is one artifact and a case-insensitive
            // checkout (Windows, macOS) resolves `foo` and `Foo` to a single folder.
            let subdir = crate::registry::checkout_relative_subdir(&ve.subdir);
            let key = crate::registry::portable_subdir_key(&ve.subdir);
            if let Some((other, other_tarball, other_subdir)) =
                seen_subdirs.insert(key, (ver, ve.tarball.as_str(), subdir.clone()))
            {
                // Name the newer key as the failing one and the older as the peer, so
                // the message reads the same whichever iteration order surfaced them.
                let (older, newer) = if crate::validate::compare_version_keys(other, ver).is_le() {
                    (other, ver)
                } else {
                    (ver, other)
                };
                // Distinct tarballs are a DIFFERENT mistake from one archive reused, and
                // the remedy differs too, so say which one this is rather than emitting a
                // message that fits neither.
                let detail = if other_subdir != subdir {
                    // Same folder only where the filesystem ignores case — so the index
                    // would mean different things on Linux and on Windows/macOS.
                    "The two differ only in case. `registry-index.json` is one file served \
                     to every platform, and a case-insensitive checkout (Windows, macOS) \
                     resolves both to a single folder, so this index cannot mean the same \
                     thing everywhere. Name one of the folders distinctly."
                } else if other_tarball == ve.tarball {
                    "Both keys name one subdir of one archive, so both are indexed from \
                     that path's single current manifest and the older key would \
                     advertise the newer build's metadata. Give each version its own \
                     subdir, or remove the stale key."
                } else {
                    "The tarballs differ, but `reindex` reads this checkout and never \
                     opens either archive — so both keys are indexed from this one \
                     path's manifest and the older key would advertise the current \
                     build's metadata. Give each version its own subdir."
                };
                errors.push((
                    format!("{id}@{newer}"),
                    format!("shares subdir '{subdir}' with version {older}. {detail}"),
                ));
            }
        }

        let mut ca = CatalogAgent {
            display_name: None,
            vendor: None,
            keywords: Vec::new(),
            versions: BTreeMap::new(),
        };
        // Agent-level metadata comes from the NEWEST version by SemVer precedence —
        // the same version `CatalogAgent::latest` reports and `install` fetches.
        //
        // This took it "last wins" from the iteration until #384, and `entry.versions`
        // is a `BTreeMap<String, _>`, so the last one visited is the lexically-greatest:
        // `1.9.0` came after `1.10.1` and overwrote it. The comment defending that read
        // "agent-level metadata is stable across versions", which is an assumption
        // nothing enforces — a rebrand changes `display-name`, an acquisition changes
        // `vendor`, and `keywords` are edited as an agent grows commands. Those are
        // exactly the fields a maintainer edits in the NEW version, and the catalogue
        // published the old one while `install` fetched the new (#371's fix).
        // "Newest among the versions that LOADED", tracked as a running best. A first
        // cut computed the newest key up front and then bolted on a fallback for the
        // case where that version's manifest failed to load — which is the same answer
        // by a longer route (when the newest loads, it IS the newest that loaded), and
        // it re-invoked `load` for a manifest already read, quietly requiring the
        // caller's closure to be idempotent. A running best needs neither.
        let mut newest: Option<String> = None;
        for (ver, ve) in &entry.versions {
            match load(&ve.subdir) {
                Ok(agent) => {
                    let is_newer = newest.as_ref().is_none_or(|best| {
                        crate::validate::compare_version_keys(ver, best).is_gt()
                    });
                    if is_newer {
                        ca.display_name = agent.display_name.clone();
                        ca.vendor = agent.vendor.clone();
                        ca.keywords = agent.keywords.clone();
                        newest = Some(ver.clone());
                    }
                    ca.versions.insert(ver.clone(), version_from_agent(&agent));
                }
                Err(e) => errors.push((format!("{id}@{ver}"), e.to_string())),
            }
        }
        // A rename alias / deprecated key is still LOADED above — so a broken one is
        // reported by `reindex`/`reindex --check`, since it stays resolvable for
        // `agent update` — but it is kept OUT of the catalog so it isn't listed as a
        // duplicate of the agent it points at (#256).
        if entry.hidden_from_catalog() {
            continue;
        }
        if !ca.versions.is_empty() {
            agents.insert(id.clone(), ca);
        }
    }
    (
        Catalog {
            version: "1.0".to_string(),
            updated_at,
            agents,
        },
        errors,
    )
}

/// Score one agent against a search `query` (whitespace-split, case-insensitive
/// substring terms). Returns `None` when nothing matches. `capability` mode
/// scores ONLY command names/methods + skills (functionality search), ignoring
/// id/name/keywords/description.
pub fn score_agent(
    query: &str,
    id: &str,
    a: &CatalogAgent,
    capability: bool,
) -> Option<SearchMatch> {
    let q = query.to_lowercase();
    let terms: Vec<&str> = q.split_whitespace().filter(|t| !t.is_empty()).collect();
    if terms.is_empty() {
        return None;
    }
    let (_, v) = a.latest()?;
    let any = |hay: &str| {
        let h = hay.to_lowercase();
        terms.iter().any(|t| h.contains(t))
    };

    let mut score = 0i32;
    let mut matched: Vec<String> = Vec::new();
    let mut snippet: Option<String> = None;

    if !capability {
        if any(id) {
            score += 10;
            matched.push("id".to_string());
        }
        if a.display_name.as_deref().is_some_and(any) {
            score += 8;
            matched.push("name".to_string());
        }
        if a.keywords.iter().any(|k| any(k)) {
            score += 6;
            matched.push("keywords".to_string());
        }
        if any(&v.description) {
            score += 5;
            matched.push("description".to_string());
        }
    }

    let cmd_weight = if capability { 10 } else { 4 };
    for c in &v.commands {
        let name_or_method = any(&c.name) || c.method.as_deref().is_some_and(any);
        if name_or_method {
            score += cmd_weight;
            if !matched.iter().any(|m| m == "command") {
                matched.push("command".to_string());
            }
            snippet.get_or_insert_with(|| format!("{} — {}", c.name, c.description));
        } else if !capability && any(&c.description) {
            score += 2;
            if !matched.iter().any(|m| m == "command-desc") {
                matched.push("command-desc".to_string());
            }
            snippet.get_or_insert_with(|| format!("{} — {}", c.name, c.description));
        }
    }

    for s in &v.skills {
        if any(s) {
            score += 3;
            if !matched.iter().any(|m| m == "skill") {
                matched.push("skill".to_string());
            }
            snippet.get_or_insert_with(|| format!("skill: {s}"));
        }
    }

    if score == 0 {
        None
    } else {
        Some(SearchMatch {
            id: id.to_string(),
            score,
            matched,
            snippet,
        })
    }
}

/// Rank every catalog agent against `query`, best first (ties broken by id).
pub fn search(catalog: &Catalog, query: &str, capability: bool) -> Vec<SearchMatch> {
    let mut hits: Vec<SearchMatch> = catalog
        .agents
        .iter()
        .filter_map(|(id, a)| score_agent(query, id, a, capability))
        .collect();
    hits.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.id.cmp(&b.id)));
    hits
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::{IndexEntry, VersionEntry};

    fn agent_from_yaml(id: &str, desc: &str) -> Agent {
        let y = format!(
            "agent: {id}\nversion: 9.9.9\ndescription: {desc}\nstateful: false\nlicense: MIT\n\
             transport:\n  cli:\n    binary: aware-{id}\n\
             commands:\n  do-thing:\n    lifecycle: single\n    description: does a thing\n\
             skills:\n  - some-skill\n"
        );
        serde_yaml::from_str(&y).unwrap()
    }

    fn index_with(entries: &[(&str, &str, &str)]) -> Index {
        let mut agents = BTreeMap::new();
        for (id, ver, subdir) in entries {
            let mut versions = BTreeMap::new();
            versions.insert(
                (*ver).to_string(),
                VersionEntry {
                    tarball: "t".to_string(),
                    subdir: (*subdir).to_string(),
                },
            );
            agents.insert(
                (*id).to_string(),
                IndexEntry {
                    versions,
                    ..Default::default()
                },
            );
        }
        Index {
            version: "1.0".to_string(),
            updated_at: "x".to_string(),
            agents,
            bundles: BTreeMap::new(),
        }
    }

    #[test]
    fn build_catalog_keys_come_from_index_not_manifest() {
        let index = index_with(&[("tekla", "2025.0.1", "aware-main/.../tekla")]);
        let (cat, errs) = build_catalog(&index, "now".to_string(), |subdir| {
            assert!(subdir.contains("tekla"));
            Ok(agent_from_yaml("tekla", "steel detailing"))
        });
        assert!(errs.is_empty());
        let a = cat.agents.get("tekla").unwrap();
        assert!(
            a.versions.contains_key("2025.0.1"),
            "uses the index version key"
        );
        assert!(
            !a.versions.contains_key("9.9.9"),
            "NOT the manifest.version"
        );
        // …but the entry CARRIES the manifest version (9.9.9) as a comparable field,
        // distinct from its index key (2025.0.1), so a UI can detect outdated installs.
        assert_eq!(
            a.versions.get("2025.0.1").unwrap().manifest_version,
            "9.9.9",
            "the entry records manifest.version separately from the index key"
        );
    }

    #[test]
    fn build_catalog_excludes_alias_and_deprecated_entries() {
        // #256: a rename alias (old id → new agent) and a deprecated key must stay
        // OUT of the catalog so a rename leaves no duplicate listing — while the real
        // target is present exactly once.
        let mut index = index_with(&[("steel-detailer-us", "0.1.0", "us")]);
        // `steel-detailer-aisc` is the OLD id, kept as an alias pointing at the new
        // agent's subdir (so `agent update` migrates existing installs) but hidden.
        let mut alias_versions = BTreeMap::new();
        alias_versions.insert(
            "0.1.0".to_string(),
            VersionEntry {
                tarball: "t".to_string(),
                subdir: "us".to_string(),
            },
        );
        index.agents.insert(
            "steel-detailer-aisc".to_string(),
            IndexEntry {
                versions: alias_versions,
                alias_of: Some("steel-detailer-us".to_string()),
                deprecated: false,
            },
        );
        // A separately-retired key with no successor (deprecated alone) is hidden too.
        let mut dep_versions = BTreeMap::new();
        dep_versions.insert(
            "0.1.0".to_string(),
            VersionEntry {
                tarball: "t".to_string(),
                subdir: "sunset".to_string(),
            },
        );
        index.agents.insert(
            "old-sunset".to_string(),
            IndexEntry {
                versions: dep_versions,
                alias_of: None,
                deprecated: true,
            },
        );

        let (cat, errs) = build_catalog(&index, "now".to_string(), |_subdir| {
            // The alias resolves to the new agent's manifest; if `load` were called for
            // a hidden entry this would still succeed, so the assertion below is what
            // proves the entry was skipped (not merely that its manifest loaded).
            Ok(agent_from_yaml("steel-detailer-us", "us steel detailing"))
        });

        assert!(errs.is_empty());
        assert!(
            cat.agents.contains_key("steel-detailer-us"),
            "the rename target is listed"
        );
        assert!(
            !cat.agents.contains_key("steel-detailer-aisc"),
            "the alias key must NOT appear (no duplicate listing)"
        );
        assert!(
            !cat.agents.contains_key("old-sunset"),
            "a deprecated key must NOT appear"
        );
        assert_eq!(cat.agents.len(), 1, "exactly one catalog entry survives");
    }

    #[test]
    fn build_catalog_still_validates_hidden_entries() {
        // #256 review (Codex P2): a hidden alias/deprecated entry stays resolvable for
        // `agent update`, so a broken one must STILL be reported by reindex — it just
        // mustn't appear in the catalog. Excluding it from `agents` must not also
        // exclude it from the error list.
        let mut index = index_with(&[("good", "1.0.0", "good")]);
        let mut bad_versions = BTreeMap::new();
        bad_versions.insert(
            "1.0.0".to_string(),
            VersionEntry {
                tarball: "t".to_string(),
                subdir: "broken".to_string(),
            },
        );
        index.agents.insert(
            "retired".to_string(),
            IndexEntry {
                versions: bad_versions,
                alias_of: None,
                deprecated: true, // hidden, but its broken manifest must still be flagged
            },
        );

        let (cat, errs) = build_catalog(&index, "now".to_string(), |subdir| {
            if subdir == "broken" {
                Err(AwareError::Validation("boom".to_string()))
            } else {
                Ok(agent_from_yaml("good", "fine"))
            }
        });

        assert!(cat.agents.contains_key("good"));
        assert!(
            !cat.agents.contains_key("retired"),
            "the hidden entry is not listed"
        );
        assert_eq!(errs.len(), 1, "but its broken manifest is still reported");
        assert!(
            errs[0].0.contains("retired"),
            "the failure names the hidden entry: {errs:?}"
        );
    }

    /// Build an index with one agent carrying SEVERAL version entries, each named as
    /// `(version, tarball, subdir)` — the multi-version shapes the collision guard has
    /// to tell apart. The tarball is explicit because it is half the payload identity.
    fn index_multi(id: &str, versions: &[(&str, &str, &str)]) -> Index {
        let mut vmap = BTreeMap::new();
        for (ver, tarball, subdir) in versions {
            vmap.insert(
                (*ver).to_string(),
                VersionEntry {
                    tarball: (*tarball).to_string(),
                    subdir: (*subdir).to_string(),
                },
            );
        }
        let mut agents = BTreeMap::new();
        agents.insert(
            id.to_string(),
            IndexEntry {
                versions: vmap,
                ..Default::default()
            },
        );
        Index {
            version: "1.0".to_string(),
            updated_at: "x".to_string(),
            agents,
            bundles: BTreeMap::new(),
        }
    }

    #[test]
    fn build_catalog_refuses_two_versions_sharing_one_payload() {
        // #454: bumping an agent's manifest and adding a new index version that reuses
        // the SAME tarball and subdir makes reindex build both keys from that path's
        // one current manifest, so the older key advertises the newer build. That is a
        // registry defect, reported (so reindex refuses) rather than silently fabricated.
        let index = index_multi(
            "demo",
            &[
                ("0.1.0", "main.tar.gz", "aware-main/demo"),
                ("0.2.0", "main.tar.gz", "aware-main/demo"),
            ],
        );
        let (_cat, errs) = build_catalog(&index, "now".to_string(), |_subdir| {
            // Both keys resolve here; the manifest declares only the current (0.2.0) build.
            Ok(agent_from_yaml("demo", "the current build"))
        });
        assert_eq!(
            errs.len(),
            1,
            "the collision is reported exactly once: {errs:?}"
        );
        assert_eq!(
            errs[0].0, "demo@0.2.0",
            "the NEWER key is named as the offender"
        );
        assert!(
            errs[0].1.contains("shares subdir") && errs[0].1.contains("with version 0.1.0"),
            "the message names the shared subdir and the peer version: {}",
            errs[0].1
        );
    }

    #[test]
    fn build_catalog_refuses_one_subdir_even_across_distinct_tarballs() {
        // Codex review (PR #457, round 5): allowing this shape — which round 3 did, keyed
        // on the installer's `(tarball, subdir)` payload identity — reopened the very hole
        // #454 is about. `load` receives only a subdir and `reindex` resolves it against
        // the local checkout, never opening either tarball, so BOTH keys were described
        // from the one checkout manifest while reindex reported success.
        //
        // The distinction is the point: `payload_id` says when two keys INSTALL alike;
        // this guard needs to know when this generator can TELL THEM APART, and reading
        // one checkout it cannot. Refusing is honest until reindex fetches each archive.
        let index = index_multi(
            "probe-agent",
            &[
                (
                    "1.2.0",
                    "probe-1.2.0.tar.gz",
                    "aware-main/20-agents/probe-agent",
                ),
                (
                    "1.3.0",
                    "probe-1.3.0.tar.gz",
                    "aware-main/20-agents/probe-agent",
                ),
            ],
        );
        let (_cat, errs) = build_catalog(&index, "now".to_string(), |_subdir| {
            Ok(agent_from_yaml("probe-agent", "a probe"))
        });
        assert_eq!(
            errs.len(),
            1,
            "one subdir, one manifest, one refusal: {errs:?}"
        );
        assert_eq!(errs[0].0, "probe-agent@1.3.0");
        assert!(
            errs[0].1.contains("tarballs differ"),
            "the message names the distinct-tarball case specifically, since its remedy \
             differs from the one-archive case: {}",
            errs[0].1
        );
    }

    #[test]
    fn build_catalog_sees_through_a_trailing_slash_spelling() {
        // Codex review (PR #457, P2): `extract_subdir` (`install/registry.rs`) trims
        // trailing slashes, so `foo` and `foo/` are ONE directory to the installer. A
        // raw-string key treated them as two and let the collision through into the
        // very catalog this guard exists to prevent.
        let index = index_multi(
            "demo",
            &[
                ("0.1.0", "main.tar.gz", "aware-main/demo"),
                ("0.2.0", "main.tar.gz", "aware-main/demo/"),
            ],
        );
        let (_cat, errs) = build_catalog(&index, "now".to_string(), |_subdir| {
            Ok(agent_from_yaml("demo", "the current build"))
        });
        assert_eq!(
            errs.len(),
            1,
            "a trailing slash is the same directory, so still one collision: {errs:?}"
        );
        assert_eq!(errs[0].0, "demo@0.2.0");
    }

    #[test]
    fn build_catalog_sees_through_the_archive_root_prefix() {
        // Codex review (PR #457 round 7): `reindex`'s loader strips `aware-main/` before
        // joining onto the checkout, so `aware-main/demo` and `demo` reach ONE
        // `repo_root/demo/manifest.yaml` while normalising to different strings. The guard
        // passed and both versions were stamped from that single manifest — the corruption
        // it exists to prevent. Guard and loader now share one routine.
        let index = index_multi(
            "demo",
            &[
                ("0.1.0", "main.tar.gz", "aware-main/demo"),
                ("0.2.0", "main.tar.gz", "demo"),
            ],
        );
        let (_cat, errs) = build_catalog(&index, "now".to_string(), |_subdir| {
            Ok(agent_from_yaml("demo", "the current build"))
        });
        assert_eq!(
            errs.len(),
            1,
            "both entries load one manifest, so it is one collision: {errs:?}"
        );
        assert_eq!(errs[0].0, "demo@0.2.0");
    }

    #[test]
    fn build_catalog_reports_a_backslash_subdir_instead_of_keying_on_it() {
        // Codex review (PR #457 round 8): `Path::join` treats `\\` as a separator on
        // Windows but not on Linux, so `foo/bar` and `foo\\bar` load ONE manifest there
        // and two here. No key computed from such a subdir means the same thing on both
        // platforms, so it is reported rather than guessed at — and normalising instead
        // would fold two genuinely distinct Linux directories together.
        let index = index_multi(
            "demo",
            &[
                ("0.1.0", "main.tar.gz", "aware-main/demo/one"),
                ("0.2.0", "main.tar.gz", "aware-main/demo\\one"),
            ],
        );
        let (_cat, errs) = build_catalog(&index, "now".to_string(), |_subdir| {
            Ok(agent_from_yaml("demo", "the current build"))
        });
        assert_eq!(errs.len(), 1, "the malformed entry is reported: {errs:?}");
        assert_eq!(errs[0].0, "demo@0.2.0");
        assert!(
            errs[0].1.contains("backslash"),
            "and says why: {}",
            errs[0].1
        );
    }

    #[test]
    fn build_catalog_refuses_subdirs_that_differ_only_in_case() {
        // Codex review (PR #457 round 9): a case-insensitive checkout (Windows, macOS —
        // both default) resolves `demo` and `Demo` to ONE folder, so both entries load one
        // manifest there while Linux sees two. `registry-index.json` is a single file
        // served to every platform, so a pair that resolves differently per platform
        // cannot be a valid registry whichever machine ran `reindex`.
        let index = index_multi(
            "demo",
            &[
                ("0.1.0", "main.tar.gz", "aware-main/demo"),
                ("0.2.0", "main.tar.gz", "aware-main/Demo"),
            ],
        );
        let (_cat, errs) = build_catalog(&index, "now".to_string(), |_subdir| {
            Ok(agent_from_yaml("demo", "the current build"))
        });
        assert_eq!(errs.len(), 1, "the case collision is reported: {errs:?}");
        assert_eq!(errs[0].0, "demo@0.2.0");
        assert!(
            errs[0].1.contains("differ only in case"),
            "and is diagnosed AS a case collision, whose remedy differs from a plain \
             duplicate: {}",
            errs[0].1
        );
    }

    #[test]
    fn build_catalog_accepts_multi_version_with_distinct_subdirs() {
        // The correct same-tarball multi-version shape: each version frozen at its own
        // subdir. The guard must NOT flag it, and every version is listed with the
        // manifest that subdir actually holds.
        let index = index_multi(
            "demo",
            &[
                ("0.1.0", "main.tar.gz", "aware-main/demo-0.1.0"),
                ("0.2.0", "main.tar.gz", "aware-main/demo"),
            ],
        );
        let (cat, errs) = build_catalog(&index, "now".to_string(), |subdir| {
            let desc = if subdir.ends_with("0.1.0") {
                "the frozen 0.1.0 build"
            } else {
                "the current build"
            };
            Ok(agent_from_yaml("demo", desc))
        });
        assert!(errs.is_empty(), "distinct subdirs are legitimate: {errs:?}");
        let demo = cat.agents.get("demo").unwrap();
        assert_eq!(demo.versions.len(), 2, "both versions listed");
        assert_eq!(
            demo.versions.get("0.1.0").unwrap().description,
            "the frozen 0.1.0 build",
            "the 0.1.0 entry reflects its OWN subdir's manifest, not the current one"
        );
        assert_eq!(
            demo.versions.get("0.2.0").unwrap().description,
            "the current build"
        );
    }

    #[test]
    fn build_catalog_skips_bad_manifest_and_signals() {
        let index = index_with(&[("good", "1.0.0", "good"), ("bad", "1.0.0", "bad")]);
        let (cat, errs) = build_catalog(&index, "now".to_string(), |subdir| {
            if subdir == "bad" {
                Err(AwareError::Validation("boom".to_string()))
            } else {
                Ok(agent_from_yaml("good", "fine"))
            }
        });
        assert!(cat.agents.contains_key("good"));
        assert!(!cat.agents.contains_key("bad"), "bad agent dropped");
        assert_eq!(errs.len(), 1);
        assert!(
            errs[0].0.contains("bad"),
            "the failure is reported: {:?}",
            errs
        );
    }

    fn sample_catalog() -> Catalog {
        let json = r#"{"version":"1.0","updated-at":"x","agents":{
          "tekla":{"display-name":"Tekla Structures","vendor":"trimble","keywords":["steel","structural"],
            "versions":{"2025.0.1":{"description":"steel detailing and connections","status":"available","stateful":true,"transport":"cli","command_count":25,
              "skills":["drawing-identity","connection-mapper"],
              "commands":[
                {"name":"run-macro","description":"run a tekla macro","lifecycle":"single","category":"curated","mode":"write"},
                {"name":"list-parts","description":"list parts","lifecycle":"single","category":"curated","method":"GET"}
              ]}}}
        }}"#;
        Catalog::parse(json.as_bytes()).unwrap()
    }

    #[test]
    fn parse_roundtrips_and_latest_picks_greatest() {
        let c = sample_catalog();
        let t = c.agents.get("tekla").unwrap();
        let (ver, v) = t.latest().unwrap();
        assert_eq!(ver, "2025.0.1");
        assert_eq!(v.transport, "cli");
    }

    #[test]
    fn capability_hits_match_command_method_and_skill() {
        let c = sample_catalog();
        let t = c.agents.get("tekla").unwrap();
        assert!(!t.capability_hits("run-macro").is_empty(), "command name");
        assert_eq!(t.capability_hits("GET")[0].kind, "method", "http method");
        assert_eq!(
            t.capability_hits("connection")[0].kind,
            "skill",
            "skill substring"
        );
        assert!(t.capability_hits("nonexistent").is_empty());
        assert!(t.capability_hits("").is_empty());
    }

    #[test]
    fn search_ranks_id_over_description_over_skill() {
        let c = sample_catalog();
        let t = c.agents.get("tekla").unwrap();
        let by_id = score_agent("tekla", "tekla", t, false).unwrap().score;
        let by_desc = score_agent("detailing", "tekla", t, false).unwrap().score;
        let by_skill = score_agent("drawing-identity", "tekla", t, false)
            .unwrap()
            .score;
        assert!(
            by_id > by_desc,
            "id {by_id} should outrank description {by_desc}"
        );
        assert!(
            by_desc > by_skill,
            "description {by_desc} should outrank skill {by_skill}"
        );
        assert!(score_agent("zzzznomatch", "tekla", t, false).is_none());
    }

    #[test]
    fn search_capability_mode_scores_only_functionality() {
        let c = sample_catalog();
        let t = c.agents.get("tekla").unwrap();
        // "macro" is in a command name → found in capability mode
        assert!(score_agent("macro", "tekla", t, true).is_some());
        // "detailing" is only in the description → NOT scored in capability mode
        assert!(score_agent("detailing", "tekla", t, true).is_none());
    }

    #[test]
    fn search_sorts_best_first() {
        let c = sample_catalog();
        let hits = search(&c, "tekla steel", false);
        assert_eq!(hits[0].id, "tekla");
        assert!(hits[0].score > 0);
    }

    /// Build a manifest declaring exactly the named transports, so a test can state a
    /// COMBINATION rather than a single field.
    fn agent_with(transports: &[&str]) -> Agent {
        let blocks: String = transports
            .iter()
            .map(|t| match *t {
                "cli" => "  cli:\n    binary: b\n".to_string(),
                "rest" => "  rest: {}\n".to_string(),
                "app" => "  app:\n    backed-by: some-app\n".to_string(),
                "builtin" => "  builtin: {}\n".to_string(),
                "mcp" => "  mcp:\n    server: s\n".to_string(),
                other => panic!("unknown transport {other}"),
            })
            .collect();
        // `transport: {}` when the list is empty — an empty mapping, not a missing key.
        let transport = if blocks.is_empty() {
            "transport: {}\n".to_string()
        } else {
            format!("transport:\n{blocks}")
        };
        serde_yaml::from_str(&format!(
            "agent: probe\nversion: 1.0.0\ndescription: x\nstateful: false\nlicense: MIT\n\
             {transport}commands:\n  go:\n    lifecycle: single\n    description: x\n"
        ))
        .unwrap()
    }

    #[test]
    fn the_published_transport_is_the_one_dispatch_would_take() {
        // #366: this ordered `app` above `rest` by hand, so an agent carrying both was
        // published as `app` while `aware app run` dispatched on `rest` — the catalogue
        // told the operator the wrong thing about how the agent runs. The pair below is
        // the repro; the rest pin the whole order so a future hand-rewrite is caught in
        // whichever direction it drifts.
        for (declared, expected) in [
            (vec!["rest", "app"], "rest"),
            (vec!["cli", "rest"], "cli"),
            (vec!["cli", "app"], "cli"),
            (vec!["cli", "builtin"], "cli"),
            (vec!["app", "builtin"], "app"),
            (vec!["rest", "builtin"], "rest"),
            (vec!["cli", "rest", "app", "builtin", "mcp"], "cli"),
            (vec!["cli"], "cli"),
            (vec!["rest"], "rest"),
            (vec!["app"], "app"),
            (vec!["builtin"], "builtin"),
        ] {
            let a = agent_with(&declared);
            assert_eq!(
                transport_str(&a.transport),
                expected,
                "catalogue must publish {expected} for a manifest declaring {declared:?}"
            );
            // …and it must be the SAME answer dispatch gets. Both read
            // `dispatch_transport` today; this fails the moment one of them stops.
            assert_eq!(
                transport_str(&a.transport),
                crate::runtime::invoker::effective_transport(&a, "probe")
                    .unwrap()
                    .as_str(),
                "the catalogue and dispatch disagree about {declared:?}"
            );
        }
    }

    #[test]
    fn a_manifest_with_nothing_dispatchable_is_reported_not_hidden() {
        // `mcp` is not in the priority order because the runtime cannot dispatch it —
        // but the catalogue still says what the manifest declares, because "none" would
        // read as "declares no transport", which is a different (and false) statement.
        // `validate_agent` refuses mcp-only outright (E_AGENT_MCP_ONLY), so reaching
        // this arm means a manifest got in without validation.
        assert_eq!(transport_str(&agent_with(&["mcp"]).transport), "mcp");
        assert_eq!(transport_str(&agent_with(&[]).transport), "none");
        // Alongside a runnable transport, `mcp` never wins.
        assert_eq!(
            transport_str(&agent_with(&["mcp", "builtin"]).transport),
            "builtin"
        );
    }

    /// An index publishing one agent at each of `versions`, with a `load` that returns a
    /// manifest whose metadata NAMES its version — so the catalogue's metadata says
    /// outright which manifest it came from.
    fn catalog_of(versions: &[&str]) -> Catalog {
        let entries: String = versions
            .iter()
            .map(|v| {
                format!(r#""{v}": {{ "tarball": "https://x.invalid/x.tar.gz", "subdir": "{v}" }}"#)
            })
            .collect::<Vec<_>>()
            .join(",\n            ");
        let index: Index = serde_json::from_str(&format!(
            r#"{{
    "version": "1.0",
    "updated-at": "x",
    "agents": {{ "probe": {{ "versions": {{
            {entries}
    }} }} }},
    "bundles": {{}}
}}"#
        ))
        .unwrap();
        // `subdir` IS the version here, so the loader can answer per-version.
        let (cat, errors) = build_catalog(&index, "x".into(), |subdir| {
            Ok(serde_yaml::from_str(&format!(
                "agent: probe\nversion: {subdir}\ndescription: probe\nstateful: false\n\
                 license: MIT\ndisplay-name: Name {subdir}\nvendor: vendor-{subdir}\n\
                 keywords: [kw-{subdir}]\ntransport:\n  cli:\n    binary: b\n\
                 commands:\n  go:\n    lifecycle: single\n    description: x\n"
            ))
            .unwrap())
        });
        assert!(errors.is_empty(), "{errors:?}");
        cat
    }

    #[test]
    fn agent_metadata_comes_from_the_newest_version_not_the_lexically_last() {
        // #384. `entry.versions` is a `BTreeMap<String, _>`, so "last wins" landed on the
        // lexically-greatest — `1.9.0` after `1.10.1` — and the catalogue described the
        // OLDER manifest while `install` fetched the newer one.
        let cat = catalog_of(&["1.9.0", "1.10.1"]);
        let a = cat.agents.get("probe").unwrap();
        assert_eq!(a.display_name.as_deref(), Some("Name 1.10.1"));
        assert_eq!(a.vendor.as_deref(), Some("vendor-1.10.1"));
        assert_eq!(a.keywords, ["kw-1.10.1"]);
        // Both versions are still published — this is about which one DESCRIBES the
        // agent, not about dropping any.
        assert_eq!(a.versions.len(), 2);
        // And it agrees with `latest()`, which is the version `install` would fetch:
        // the two disagreeing is the whole defect.
        assert_eq!(a.latest().map(|(v, _)| v.as_str()), Some("1.10.1"));

        // Calendar-shaped keys, which is what this registry actually publishes.
        let cat = catalog_of(&["2025.0.2", "2025.0.10"]);
        let a = cat.agents.get("probe").unwrap();
        assert_eq!(a.vendor.as_deref(), Some("vendor-2025.0.10"));

        // A single version is unaffected, and a prerelease still loses to its release.
        let cat = catalog_of(&["1.0.0"]);
        assert_eq!(cat.agents["probe"].vendor.as_deref(), Some("vendor-1.0.0"));
        let cat = catalog_of(&["1.0.0-rc.1", "1.0.0"]);
        assert_eq!(cat.agents["probe"].vendor.as_deref(), Some("vendor-1.0.0"));
    }

    /// A two-version index for `probe` where `subdir` IS the version, so a test's `load`
    /// closure can answer differently per version.
    fn two_version_index() -> Index {
        serde_json::from_str(
            r#"{
    "version": "1.0",
    "updated-at": "x",
    "agents": { "probe": { "versions": {
            "1.9.0":  { "tarball": "https://x.invalid/x.tar.gz", "subdir": "1.9.0" },
            "1.10.1": { "tarball": "https://x.invalid/x.tar.gz", "subdir": "1.10.1" }
    } } },
    "bundles": {}
}"#,
        )
        .unwrap()
    }

    /// A manifest for `version`, carrying whatever `meta` lines are given. Raw literal so
    /// the YAML in a test reads as YAML.
    fn manifest_with_meta(version: &str, meta: &str) -> Agent {
        let yaml = format!(
            r#"agent: probe
version: {version}
description: probe
stateful: false
license: MIT
{meta}transport:
  cli:
    binary: b
commands:
  go:
    lifecycle: single
    description: x
"#
        );
        serde_yaml::from_str(&yaml).unwrap()
    }

    #[test]
    fn a_newest_version_with_no_metadata_publishes_none_rather_than_reviving_an_older_one() {
        // The case that keeps the fix honest, and the one a "fall back whenever the fields
        // came out empty" formulation gets wrong: an agent that DROPS its metadata in a new
        // version must publish nothing, not resurrect what 1.9.0 said. That is #384 in the
        // other direction — stale metadata presented as current.
        let index = two_version_index();
        let (cat, errors) = build_catalog(&index, "x".into(), |subdir| {
            Ok(manifest_with_meta(
                subdir,
                if subdir == "1.10.1" {
                    ""
                } else {
                    "display-name: Old Name\nvendor: vendor-1.9.0\nkeywords: [kw-1.9.0]\n"
                },
            ))
        });
        assert!(errors.is_empty(), "{errors:?}");
        let a = cat.agents.get("probe").unwrap();
        assert_eq!(a.display_name, None, "1.9.0's name must not come back");
        assert_eq!(a.vendor, None, "1.9.0's vendor must not come back");
        assert!(a.keywords.is_empty(), "1.9.0's keywords must not come back");

        // The PARTIAL case, which an `||` where the rule wants "the newest decides" would
        // also get wrong: the newest sets a display-name but no vendor, the older has one.
        let (cat, _) = build_catalog(&index, "x".into(), |subdir| {
            Ok(manifest_with_meta(
                subdir,
                if subdir == "1.10.1" {
                    "display-name: New Name\n"
                } else {
                    "display-name: Old Name\nvendor: vendor-1.9.0\n"
                },
            ))
        });
        let a = cat.agents.get("probe").unwrap();
        assert_eq!(a.display_name.as_deref(), Some("New Name"));
        assert_eq!(a.vendor, None, "a field the newest omits stays omitted");
    }

    #[test]
    fn a_broken_newest_manifest_degrades_the_metadata_rather_than_erasing_it() {
        // Taking metadata from ONE version means a load failure at that version leaves
        // the agent with none at all — worse than stale, and invisible, since the
        // catalogue would simply omit the fields. It falls back to the newest version
        // that did load.
        let index: Index = serde_json::from_str(
            r#"{
    "version": "1.0",
    "updated-at": "x",
    "agents": { "probe": { "versions": {
            "1.9.0":  { "tarball": "https://x.invalid/x.tar.gz", "subdir": "1.9.0" },
            "1.10.1": { "tarball": "https://x.invalid/x.tar.gz", "subdir": "1.10.1" }
    } } },
    "bundles": {}
}"#,
        )
        .unwrap();
        let (cat, errors) = build_catalog(&index, "x".into(), |subdir| {
            if subdir == "1.10.1" {
                return Err(AwareError::Validation("broken manifest".into()));
            }
            Ok(serde_yaml::from_str(&format!(
                "agent: probe\nversion: {subdir}\ndescription: probe\nstateful: false\n\
                 license: MIT\ndisplay-name: Name {subdir}\nvendor: vendor-{subdir}\n\
                 keywords: [kw-{subdir}]\ntransport:\n  cli:\n    binary: b\n\
                 commands:\n  go:\n    lifecycle: single\n    description: x\n"
            ))
            .unwrap())
        });
        // The failure is still REPORTED — `reindex` must not swallow it.
        assert_eq!(errors.len(), 1, "{errors:?}");
        assert!(errors[0].0.contains("1.10.1"), "{errors:?}");
        let a = cat.agents.get("probe").unwrap();
        assert_eq!(
            a.vendor.as_deref(),
            Some("vendor-1.9.0"),
            "with the newest manifest unloadable, the newest that DID load describes it"
        );
        assert_eq!(a.versions.len(), 1, "the broken version is not published");
    }
}
