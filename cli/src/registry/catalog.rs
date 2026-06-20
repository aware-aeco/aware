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
use crate::manifest::agent::{Agent, AgentStatus, Category, Command, Lifecycle, Transport};
use crate::registry::Index;

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
    /// The greatest version key + its metadata (BTreeMap is sorted; matches the
    /// index's `resolve(None)` which also returns the lexicographically-greatest).
    pub fn latest(&self) -> Option<(&String, &CatalogVersion)> {
        self.versions.iter().next_back()
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

fn lifecycle_str(l: Lifecycle) -> &'static str {
    match l {
        Lifecycle::Start => "start",
        Lifecycle::Stop => "stop",
        Lifecycle::Single => "single",
    }
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

fn transport_str(t: &Transport) -> &'static str {
    if t.cli.is_some() {
        "cli"
    } else if t.app.is_some() {
        "app"
    } else if t.rest.is_some() {
        "rest"
    } else if t.mcp.is_some() {
        "mcp"
    } else if t.builtin.is_some() {
        "builtin"
    } else {
        "none"
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
            lifecycle: lifecycle_str(c.lifecycle).to_string(),
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
        // A rename alias / deprecated key stays resolvable for `agent update` (so
        // existing installs migrate) but is excluded here so it isn't listed as a
        // duplicate of the agent it points at (#256).
        if entry.hidden_from_catalog() {
            continue;
        }
        let mut ca = CatalogAgent {
            display_name: None,
            vendor: None,
            keywords: Vec::new(),
            versions: BTreeMap::new(),
        };
        for (ver, ve) in &entry.versions {
            match load(&ve.subdir) {
                Ok(agent) => {
                    // Agent-level metadata is stable across versions; take it from each
                    // loaded manifest (last wins — they describe the same agent).
                    ca.display_name = agent.display_name.clone();
                    ca.vendor = agent.vendor.clone();
                    ca.keywords = agent.keywords.clone();
                    ca.versions.insert(ver.clone(), version_from_agent(&agent));
                }
                Err(e) => errors.push((format!("{id}@{ver}"), e.to_string())),
            }
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
}
