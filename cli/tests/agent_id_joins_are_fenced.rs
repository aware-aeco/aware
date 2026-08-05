//! An agent id must never be joined onto `agents/` by hand to read a manifest (#365).
//!
//! An agent id reaches most callers from a FILE — a node's `agent:`, a manifest's
//! `backed-by:`, an app's `requires:` — and is turned into a path by joining it
//! onto `agents/`. Nothing validates it as a path along the way, so every such
//! join needs `loader::is_safe_segment` in front of it, or a crafted
//! `agent: ../elsewhere` reads a `manifest.yaml` from anywhere on disk.
//!
//! There were seventeen of those joins and only two carried a fence. Guarding
//! them one at a time would leave the next one to be written unfenced, so they
//! all go through `loader::load_agent_by_id` / `loader::agent_manifest_path`,
//! which fence once — and this test fails the build if a new raw one OF THAT
//! SHAPE appears. It is a text scan, so it catches the shape rather than the
//! intent: a caller determined to build the path another way (`PathBuf::push`,
//! `format!`) is beyond it, and beyond any grep.
//!
//! It scans the REAL source tree, not a fixture: a guard that checks a sample of
//! what it guards proves nothing about the rest. And it carries its own
//! self-test, because a scanner whose pattern has rotted passes exactly as
//! quietly as a clean tree.

use std::path::{Path, PathBuf};

/// The shape that skips the fence: a `load_agent(…)` whose path is built by
/// joining an ID onto a directory — `dir.join(id).join("manifest.yaml")`.
///
/// Deliberately narrow on both axes. Joining a LITERAL (`root.join("manifest.yaml")`
/// on an already-resolved directory) is fine and common, so the first `join`
/// argument must be a non-literal. And it must feed `load_agent`: the fixtures
/// that WRITE a manifest into a test tree build the same path and cross no trust
/// boundary.
fn raw_joins(src: &str) -> Vec<String> {
    const NEEDLE: &str = ".join(\"manifest.yaml\")";
    // Collapse whitespace so a rustfmt-wrapped call reads as one string.
    let flat: String = src.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut out = Vec::new();
    let mut from = 0usize;
    while let Some(rel) = flat[from..].find(NEEDLE) {
        let at = from + rel;
        from = at + NEEDLE.len();
        let before = &flat[..at];
        // The join immediately before it — its argument is the id.
        let Some(prev) = before.rfind(".join(") else {
            continue;
        };
        let arg = before[prev + ".join(".len()..].trim_start();
        if arg.starts_with('"') {
            continue; // a fixed directory name, not an id
        }
        // …and the base must be the AGENTS directory. The same two-step shape is
        // used to read a manifest out of the repo's own source tree by registry
        // subdir (`repo_root.join(rel)`), where `rel` is legitimately several
        // segments — `is_safe_segment` would refuse it, so that is a different
        // question and not this guard's business.
        let base_from = prev.saturating_sub(60);
        if !before[base_from..].contains("agents_dir") {
            continue;
        }
        // Only a READ is a finding. The loader call can sit on EITHER side of the
        // join — `load_agent(&dir.join(id).join(…))` puts it before, while
        // `let p = dir.join(id).join(…); load_agent(&p)` puts it after — so look
        // both ways within a bounded distance. Looking only backwards was the
        // first cut, and the self-test below is what caught it.
        let back_from = prev.saturating_sub(160);
        let fwd_to = (from + 160).min(flat.len());
        if !before[back_from..].contains("load_agent(")
            && !flat[from..fwd_to].contains("load_agent")
        {
            continue;
        }
        let snippet_start = prev.saturating_sub(80);
        out.push(flat[snippet_start..from].to_string());
    }
    out
}

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            rust_files(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

/// Where the fenced join lives — the one file allowed to build the path.
const FENCE_FILE: &str = "manifest/loader.rs";

#[test]
fn no_source_file_joins_an_agent_id_onto_agents_by_hand() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    assert!(
        src_root.is_dir(),
        "src/ not found — the scan would be vacuous"
    );
    let mut files = Vec::new();
    rust_files(&src_root, &mut files);
    // A scan that reached nothing passes trivially. Pin the floor so a broken
    // walk (a renamed directory, a changed extension filter) fails loudly
    // instead of reporting a clean tree.
    assert!(
        files.len() > 80,
        "only {} source files scanned — the walk is not reaching the tree",
        files.len()
    );
    assert!(
        files.iter().any(|f| f.ends_with("runtime/invoker.rs")),
        "the walk missed runtime/invoker.rs, which carries most of these joins"
    );

    let mut findings = Vec::new();
    for f in &files {
        let rel = f
            .strip_prefix(&src_root)
            .unwrap_or(f)
            .to_string_lossy()
            .replace('\\', "/");
        if rel == FENCE_FILE {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(f) else {
            continue;
        };
        for snippet in raw_joins(&text) {
            findings.push(format!("{rel}\n    …{snippet}"));
        }
    }

    assert!(
        findings.is_empty(),
        "an agent id is joined onto a directory and read without the \
         `is_safe_segment` fence.\nUse `loader::load_agent_by_id` — or \
         `loader::agent_manifest_path` if you need the path — instead. See #365.\n\n{}",
        findings.join("\n\n")
    );
}

#[test]
fn the_scanner_finds_the_shape_it_claims_to() {
    // The other direction, and the one that keeps the test above honest: a
    // scanner whose pattern stopped matching passes on a dirty tree exactly as
    // quietly as on a clean one.
    let positive = r#"
        let mp = self.agents_dir.join(agent_id).join("manifest.yaml");
        let Ok(m) = crate::manifest::loader::load_agent(&mp) else { return false; };
    "#;
    assert_eq!(raw_joins(positive).len(), 1, "should flag the raw join");

    // Split across lines the way rustfmt leaves it.
    let wrapped = r#"
        let m = crate::manifest::loader::load_agent(
            &self.agents_dir.join(agent).join("manifest.yaml"),
        )?;
    "#;
    assert_eq!(
        raw_joins(wrapped).len(),
        1,
        "should flag a wrapped call too"
    );

    for ok in [
        // An already-resolved directory — there is no preceding `.join(` at all.
        r#"let m = load_agent(&root.join("manifest.yaml"))?;"#,
        // A LITERAL id. This is the case the literal-argument narrowing exists
        // for, and it needs a preceding join to reach that branch at all — the
        // line above is excluded one step earlier, so without this the narrowing
        // was inert and the claim that all three axes are pinned was untrue
        // (#365 review).
        r#"let m = load_agent(&self.agents_dir.join("trimble-connect").join("manifest.yaml"))?;"#,
        // A WRITE, in a fixture. Same path shape, no trust boundary.
        r#"std::fs::write(aware.join("agents").join(a).join("manifest.yaml"), body).unwrap();"#,
        // The fenced call this test exists to push people towards.
        r#"let m = loader::load_agent_by_id(&self.agents_dir, agent)?;"#,
        // A read out of the repo's own source tree by REGISTRY SUBDIR. Same
        // shape, different question: `rel` is several segments by design, so
        // `is_safe_segment` is the wrong check for it.
        r#"let manifest = repo_root.join(rel).join("manifest.yaml");
           let m = load_agent(&manifest);"#,
    ] {
        assert!(
            raw_joins(ok).is_empty(),
            "should not flag: {ok}\n  got: {:?}",
            raw_joins(ok)
        );
    }
}
