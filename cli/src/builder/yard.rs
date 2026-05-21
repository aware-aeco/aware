//! YARD-rendered Ruby docs → AWARE agent.
//!
//! YARD (Yay! A Ruby Documentation tool) is the de-facto Ruby docs generator.
//! YARD-rendered HTML follows a stable template — class index at
//! `_index.html`, per-class pages at `<Namespace>/<Class>.html`, methods
//! inside each class page under `<h3 class="signature">`.
//!
//! Use this when a Ruby project publishes its API as YARD HTML but doesn't
//! ship a gem on rubygems.org. Canonical example: SketchUp's Ruby API at
//! <https://ruby.sketchup.com/>.
//!
//! Accepts HTTP(S) URL (e.g. `https://ruby.sketchup.com/`) or a local
//! directory (a YARD output dir on disk).

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::io::Read;
use std::path::{Path, PathBuf};

use scraper::{Html, Selector};

use crate::builder::{GeneratedAgent, GeneratedCommand, GeneratedSkill, Provenance, now_iso};
use crate::error::AwareError;

pub fn build_from_url_or_dir(
    input: &str,
    agent_id: Option<&str>,
) -> Result<GeneratedAgent, AwareError> {
    let source = YardSource::resolve(input)?;
    let index_html = source.fetch("_index.html")?;
    let class_paths = parse_index_classes(&index_html);

    if class_paths.is_empty() {
        return Err(AwareError::Validation(format!(
            "no classes found in YARD index at {input} \u{2014} not a YARD docs site?"
        )));
    }

    let mut commands: BTreeMap<String, GeneratedCommand> = BTreeMap::new();
    let mut skills: Vec<GeneratedSkill> = Vec::new();

    for class_path in &class_paths {
        let class_html = match source.fetch(class_path) {
            Ok(h) => h,
            Err(_) => continue, // skip broken refs rather than fail the whole build
        };
        let parsed = parse_class_page(&class_html);
        for m in &parsed.methods {
            let cmd_name = kebab(&format!("{}-{}", parsed.class_name, m.name));
            commands.insert(
                cmd_name,
                GeneratedCommand {
                    lifecycle: "single".into(),
                    description: m.description.clone(),
                    inputs_yaml: String::new(),
                    outputs_yaml: String::new(),
                },
            );
        }
        // One skill per class — captures the class-level docstring + a roster
        // of its methods so the AI can find the right command.
        let pkg_lower = "yard";
        let skill_stem = kebab(&parsed.class_name);
        let method_list = parsed
            .methods
            .iter()
            .map(|m| format!("- `{}` \u{2014} {}", m.name, m.description))
            .collect::<Vec<_>>()
            .join("\n");
        let body = format!(
            "---\nname: {pkg_lower}-{skill_stem}\ndescription: {} API reference (YARD)\n---\n\n# {} API reference\n\n{}\n\n## Methods\n\n{}\n",
            parsed.class_name, parsed.class_name, parsed.class_doc, method_list
        );
        skills.push(GeneratedSkill {
            filename: format!("{skill_stem}.md"),
            body,
        });
    }
    skills.sort_by(|a, b| a.filename.cmp(&b.filename));

    let id = agent_id.map(String::from).unwrap_or_else(|| {
        let derived = derive_id_from_input(input);
        if derived.is_empty() {
            "yard-agent".into()
        } else {
            derived
        }
    });

    let provenance = Provenance {
        generated_by: "aware-agent-builder".into(),
        generator_version: env!("CARGO_PKG_VERSION").into(),
        source: serde_json::json!({
            "type": "yard",
            "input": input,
            "classes": class_paths.len(),
        }),
        generated_at: now_iso(),
    };

    Ok(GeneratedAgent {
        id,
        version: "0.1.0".into(),
        sdk_target: None,
        description: format!(
            "Generated from YARD docs at {input} ({} classes, {} methods)",
            class_paths.len(),
            commands.len()
        ),
        commands,
        skills,
        provenance,
        stateful: false,
        license: "see-source".into(),
        rest: None,
    })
}

/// `YardSource` abstracts the difference between fetching pages over HTTP
/// vs reading them from a local YARD output directory.
enum YardSource {
    Url(String),
    Dir(PathBuf),
}

impl YardSource {
    fn resolve(input: &str) -> Result<Self, AwareError> {
        if input.starts_with("http://") || input.starts_with("https://") {
            Ok(YardSource::Url(input.trim_end_matches('/').to_string()))
        } else {
            let path = PathBuf::from(input);
            if !path.is_dir() {
                return Err(AwareError::Validation(format!(
                    "--from-yard expects a URL or a directory; {input} is neither"
                )));
            }
            Ok(YardSource::Dir(path))
        }
    }

    fn fetch(&self, relative_path: &str) -> Result<String, AwareError> {
        match self {
            YardSource::Url(base) => {
                let url = format!("{base}/{relative_path}");
                let resp = ureq::get(&url)
                    .call()
                    .map_err(|e| AwareError::Network(format!("GET {url}: {e}")))?;
                let mut body = String::new();
                resp.into_reader()
                    .read_to_string(&mut body)
                    .map_err(|e| AwareError::Network(format!("read {url}: {e}")))?;
                Ok(body)
            }
            YardSource::Dir(root) => {
                let file = root.join(relative_path);
                std::fs::read_to_string(&file)
                    .map_err(|e| AwareError::NotFound(format!("read {}: {e}", file.display())))
            }
        }
    }
}

/// Parse `_index.html` for the YARD class list. Returns the relative paths
/// to each class page (e.g. `Sketchup/Animation.html`).
fn parse_index_classes(html: &str) -> Vec<String> {
    let doc = Html::parse_document(html);
    let sel = Selector::parse("a.object_link, span.object_link a").unwrap();
    let mut out: Vec<String> = Vec::new();
    for a in doc.select(&sel) {
        if let Some(href) = a.value().attr("href")
            && href.ends_with(".html")
            && !href.starts_with("http")
            && !href.starts_with("#")
            && !href.starts_with("file.")
        {
            out.push(href.to_string());
        }
    }
    out.sort();
    out.dedup();
    out
}

struct ParsedClass {
    class_name: String,
    class_doc: String,
    methods: Vec<ParsedMethod>,
}

struct ParsedMethod {
    name: String,
    description: String,
}

/// Parse a single class page. Class name comes from `<h1>`, class docstring
/// from the first `<div class="docstring">`, methods from `<h3 class="signature">`.
fn parse_class_page(html: &str) -> ParsedClass {
    let doc = Html::parse_document(html);

    let h1_sel = Selector::parse("h1").unwrap();
    let class_name = doc
        .select(&h1_sel)
        .next()
        .map(|h| text_of(&h).trim().to_string())
        .unwrap_or_else(|| "Unknown".into());
    // YARD <h1> is always "Class: <Name>" or "Module: <Name>"; strip the prefix.
    let class_name = class_name
        .strip_prefix("Class: ")
        .or_else(|| class_name.strip_prefix("Module: "))
        .map(|s| s.to_string())
        .unwrap_or(class_name);

    let class_doc =
        Selector::parse("#description .docstring .discussion p, .docstring .discussion p")
            .ok()
            .and_then(|sel| doc.select(&sel).next().map(|p| text_of(&p)))
            .unwrap_or_default()
            .trim()
            .to_string();

    let sig_sel = Selector::parse("h3.signature").unwrap();
    let mut methods = Vec::new();
    for h3 in doc.select(&sig_sel) {
        // Method name is inside <strong>...</strong>
        let strong_sel = Selector::parse("strong").unwrap();
        let name = h3
            .select(&strong_sel)
            .next()
            .map(|s| text_of(&s).trim().to_string())
            .unwrap_or_default();
        if name.is_empty() {
            continue;
        }
        // Docstring lives in the sibling <div class="docstring"> following <h3>.
        // scraper doesn't expose sibling-of-element traversal cleanly, so we
        // pull the first <p> inside the parent's docstring block.
        let desc = h3
            .parent()
            .and_then(scraper::ElementRef::wrap)
            .and_then(|parent| {
                let p_sel = Selector::parse(".docstring .discussion p").ok()?;
                parent.select(&p_sel).next().map(|p| text_of(&p))
            })
            .unwrap_or_default()
            .trim()
            .lines()
            .next()
            .unwrap_or("")
            .to_string();
        methods.push(ParsedMethod {
            name,
            description: desc,
        });
    }

    ParsedClass {
        class_name,
        class_doc,
        methods,
    }
}

fn text_of(el: &scraper::ElementRef) -> String {
    el.text().collect::<Vec<_>>().join("").replace('\n', " ")
}

fn kebab(s: &str) -> String {
    let mut out = String::new();
    let mut prev_was_sep = true;
    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            if ch.is_ascii_uppercase() && !prev_was_sep && !out.ends_with('-') {
                out.push('-');
            }
            out.push(ch.to_ascii_lowercase());
            prev_was_sep = false;
        } else if !out.is_empty() && !out.ends_with('-') {
            out.push('-');
            prev_was_sep = true;
        }
    }
    out.trim_matches('-').to_string()
}

fn derive_id_from_input(input: &str) -> String {
    let trimmed = input.trim_end_matches('/');
    if let Some(host_pos) = trimmed.find("://") {
        let host = &trimmed[host_pos + 3..];
        let host = host.split('/').next().unwrap_or(host);
        // strip leading 'www.' and trailing TLDs ad-hoc
        let host = host.trim_start_matches("www.");
        return kebab(host.split('.').next().unwrap_or(host));
    }
    let basename = Path::new(trimmed)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(trimmed);
    kebab(basename)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INDEX: &str = r#"<!DOCTYPE html>
<html><body>
  <ul class="toplevel">
    <li><span class='object_link'><a href="top-level-namespace.html" title="Top Level Namespace (root)">Top Level Namespace</a></span></li>
  </ul>
  <ul>
    <li><span class='object_link'><a href="Sketchup/Animation.html" title="Sketchup::Animation (class)">Animation</a></span></li>
    <li><span class='object_link'><a href="Sketchup/ArcCurve.html" title="Sketchup::ArcCurve (class)">ArcCurve</a></span></li>
    <li><a class="object_link" href="Layout/Document.html">Layout::Document</a></li>
    <li><a href="https://example.com/external.html">External</a></li>
    <li><a href="file.README.html">Readme</a></li>
  </ul>
</body></html>"#;

    const SAMPLE_CLASS_PAGE: &str = r#"<!DOCTYPE html>
<html><body>
<h1>Class: Sketchup::Animation</h1>
<div id="description">
<div class="docstring"><div class="discussion"><p>An Animation is a callback interface used by SketchUp to drive its scene transitions.</p></div></div>
</div>
<div id="instance_method_details" class="method_details_list">
  <div class="method_details first">
    <h3 class="signature first" id="nextFrame-instance_method">
      #<strong>nextFrame</strong>(view)  &#x21d2; <tt>Boolean</tt>
    </h3>
    <div class="docstring"><div class="discussion"><p>Display the next animation frame.</p></div></div>
  </div>
  <div class="method_details">
    <h3 class="signature" id="pause-instance_method">
      #<strong>pause</strong>  &#x21d2; <tt>Object</tt>
    </h3>
    <div class="docstring"><div class="discussion"><p>Pause the animation.</p></div></div>
  </div>
</div>
</body></html>"#;

    #[test]
    fn parse_index_extracts_relative_class_paths_only() {
        let paths = parse_index_classes(SAMPLE_INDEX);
        assert!(paths.contains(&"Sketchup/Animation.html".to_string()));
        assert!(paths.contains(&"Sketchup/ArcCurve.html".to_string()));
        assert!(paths.contains(&"Layout/Document.html".to_string()));
        assert!(paths.contains(&"top-level-namespace.html".to_string()));
        // external URLs + file.* (overview pages) excluded
        assert!(!paths.iter().any(|p| p.starts_with("http")));
        assert!(!paths.iter().any(|p| p.starts_with("file.")));
    }

    #[test]
    fn parse_class_page_extracts_name_doc_methods() {
        let parsed = parse_class_page(SAMPLE_CLASS_PAGE);
        assert!(parsed.class_name.contains("Sketchup::Animation"));
        assert!(
            parsed
                .class_doc
                .contains("Animation is a callback interface")
        );
        let names: Vec<&str> = parsed.methods.iter().map(|m| m.name.as_str()).collect();
        assert_eq!(names, vec!["nextFrame", "pause"]);
        assert!(
            parsed.methods[0]
                .description
                .contains("Display the next animation frame")
        );
        assert!(parsed.methods[1].description.contains("Pause"));
    }

    #[test]
    fn build_from_local_dir_end_to_end() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::write(root.join("_index.html"), SAMPLE_INDEX).unwrap();
        std::fs::create_dir(root.join("Sketchup")).unwrap();
        std::fs::write(root.join("Sketchup/Animation.html"), SAMPLE_CLASS_PAGE).unwrap();
        // The other classes referenced in SAMPLE_INDEX aren't on disk — that's
        // fine, build_from_url_or_dir skips missing pages rather than failing.
        let agent = build_from_url_or_dir(root.to_str().unwrap(), Some("test")).unwrap();
        assert_eq!(agent.id, "test");
        assert!(agent.commands.contains_key("sketchup-animation-next-frame"));
        assert!(agent.commands.contains_key("sketchup-animation-pause"));
    }

    #[test]
    fn build_rejects_non_yard_input() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(
            tmp.path().join("_index.html"),
            "<html><body>not a YARD index</body></html>",
        )
        .unwrap();
        let err = build_from_url_or_dir(tmp.path().to_str().unwrap(), None).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }

    #[test]
    fn build_rejects_missing_directory() {
        let err = build_from_url_or_dir("/no/such/path", None).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }

    #[test]
    fn derive_id_from_url_uses_host_first_label() {
        assert_eq!(derive_id_from_input("https://ruby.sketchup.com/"), "ruby");
        assert_eq!(
            derive_id_from_input("https://docs.example.org/api/"),
            "docs"
        );
        assert_eq!(
            derive_id_from_input("/local/path/sketchup-api"),
            "sketchup-api"
        );
    }

    #[test]
    fn kebab_handles_namespaces_and_camelcase() {
        assert_eq!(kebab("Sketchup::Animation"), "sketchup-animation");
        assert_eq!(kebab("nextFrame"), "next-frame");
        assert_eq!(kebab("ArcCurve"), "arc-curve");
    }
}
